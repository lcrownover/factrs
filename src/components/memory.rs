// "memory": { // Parsed from /proc/meminfo
//   "swap": {
//     "available": "1.14 GiB",
//     "available_bytes": 1221046272,
//     "capacity": "45.07%",
//     "total": "2.07 GiB",
//     "total_bytes": 2222977024,
//     "used": "955.52 MiB",
//     "used_bytes": 1001930752
//   },
//   "system": {
//     "available": "343.66 MiB",
//     "available_bytes": 360353792,
//     "capacity": "80.32%",
//     "total": "1.71 GiB",
//     "total_bytes": 1831251968,
//     "used": "1.37 GiB",
//     "used_bytes": 1470898176
//   }
// },

use crate::{context::Ctx, filesystem::get_file_contents_or_empty_string};

use super::Component;
use anyhow::Result;
use serde::Serialize;
use serde_json::to_value;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Debug)]
pub struct MemoryType {
    pub available: String,
    pub available_bytes: u64,
    pub capacity: String,
    pub total: String,
    pub total_bytes: u64,
    pub used: String,
    pub used_bytes: u64,
}

#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct MemoryFacts(pub BTreeMap<String, MemoryType>);

pub struct MemoryComponent;

impl MemoryComponent {
    pub fn new() -> Self {
        Self
    }
}
impl Component for MemoryComponent {
    fn name(&self) -> &'static str {
        "memory"
    }

    fn collect(&self, ctx: &Ctx) -> Result<serde_json::Value> {
        let contents = get_file_contents_or_empty_string(ctx, "/proc/meminfo");
        let snapshot = parse_meminfo(&contents);
        let facts = build_memory_facts(snapshot);
        let j = to_value(facts)?;
        Ok(j)
    }
}

fn parse_meminfo(contents: &str) -> HashMap<String, u64> {
    contents
        .lines()
        .filter_map(|line| {
            let (label, rest) = line.split_once(':')?;
            let mut parts = rest.split_whitespace();
            let value = parts.next()?.parse::<u64>().ok()?;
            let multiplier = match parts.next() {
                Some("kB") => 1024,
                Some("mB") => 1_000_000,
                Some("MB") => 1_000_000,
                Some("B") | None => 1,
                _ => 1,
            };
            Some((label.to_string(), value * multiplier))
        })
        .collect()
}

fn build_memory_facts(snapshot: HashMap<String, u64>) -> MemoryFacts {
    let mut sections: BTreeMap<String, MemoryType> = BTreeMap::new();

    for (key, total_bytes) in snapshot.iter().filter(|(k, _)| has_total_suffix(k)) {
        if *total_bytes == 0 {
            continue;
        }

        let base = strip_total_suffix(key);
        let available_bytes = find_available_bytes(&snapshot, base);

        let available_bytes = match available_bytes {
            Some(value) => {
                // guard against noisy files exposing larger available than total
                match value.cmp(total_bytes) {
                    Ordering::Greater => *total_bytes,
                    _ => value,
                }
            }
            None => continue,
        };

        if available_bytes > *total_bytes {
            continue;
        }

        let used_bytes = total_bytes.saturating_sub(available_bytes);
        let capacity = if *total_bytes == 0 {
            "0.00%".to_string()
        } else {
            format!("{:.2}%", (used_bytes as f64 / *total_bytes as f64) * 100.0)
        };

        let section_name = normalize_section_name(base);
        let mem_type = MemoryType {
            total: format_bytes(*total_bytes),
            total_bytes: *total_bytes,
            available: format_bytes(available_bytes),
            available_bytes,
            used: format_bytes(used_bytes),
            used_bytes,
            capacity,
        };

        sections.insert(section_name, mem_type);
    }

    MemoryFacts(sections)
}

fn has_total_suffix(key: &str) -> bool {
    key.ends_with("Total") || key.ends_with("_Total")
}

fn strip_total_suffix(key: &str) -> &str {
    key.strip_suffix("Total")
        .or_else(|| key.strip_suffix("_Total"))
        .map(|trimmed| trimmed.trim_end_matches('_'))
        .unwrap_or(key)
}

fn find_available_bytes(snapshot: &HashMap<String, u64>, base: &str) -> Option<u64> {
    let candidate_keys = [
        format!("{base}Available"),
        format!("{base}_Available"),
        format!("{base}Free"),
        format!("{base}_Free"),
    ];

    for key in candidate_keys {
        if let Some(value) = snapshot.get(&key) {
            return Some(*value);
        }
    }

    None
}

fn normalize_section_name(base: &str) -> String {
    match base {
        "Mem" => "system".to_string(),
        "Swap" => "swap".to_string(),
        other => other.to_lowercase(),
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let idx = (bytes as f64).log2() / 10.0;
    let idx = idx.floor().clamp(0.0, (UNITS.len() - 1) as f64) as usize;
    let unit = UNITS[idx];
    let scaled = bytes as f64 / (1u64 << (idx * 10)) as f64;
    if scaled >= 100.0 || unit == "B" {
        format!("{:.0} {}", scaled, unit)
    } else if scaled >= 10.0 {
        format!("{:.1} {}", scaled, unit)
    } else {
        format!("{:.2} {}", scaled, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_memory_facts_from_meminfo_snapshot() {
        let meminfo = "\
MemTotal:       2048000 kB
MemAvailable:   1024000 kB
SwapTotal:      1024000 kB
SwapFree:        512000 kB
";

        let snapshot = parse_meminfo(meminfo);
        let MemoryFacts(map) = build_memory_facts(snapshot);

        assert_eq!(map.len(), 2);

        let system = map.get("system").expect("system memory entry missing");
        assert_eq!(system.total_bytes, 2_097_152_000);
        assert_eq!(system.available_bytes, 1_048_576_000);
        assert_eq!(system.used_bytes, 1_048_576_000);
        assert_eq!(system.capacity, "50.00%");
        assert_eq!(system.total, "1.95 GiB");
        assert_eq!(system.available, "1000 MiB");
        assert_eq!(system.used, "1000 MiB");

        let swap = map.get("swap").expect("swap memory entry missing");
        assert_eq!(swap.total_bytes, 1_048_576_000);
        assert_eq!(swap.available_bytes, 524_288_000);
        assert_eq!(swap.used_bytes, 524_288_000);
        assert_eq!(swap.capacity, "50.00%");
        assert_eq!(swap.total, "1000 MiB");
        assert_eq!(swap.available, "500 MiB");
        assert_eq!(swap.used, "500 MiB");
    }
}
