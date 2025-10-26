mod components;
mod filesystem;

use crate::components::{
    Collector, kernel, memory,
    network::{self, get_all_ip_devices, parse_ip_devices_output},
};
use anyhow::Result;
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::sync::Arc;

fn main() -> Result<()> {
    let debug = true;

    // Register all the components here. Each component
    // implements the Component trait
    let components: Vec<Arc<dyn Collector>> = vec![
        Arc::new(kernel::KernelComponent::new()),
        Arc::new(memory::MemoryComponent::new()),
        Arc::new(network::NetworkComponent::new()),
    ];

    // Build all the components in parallel into pairs of information
    let pairs: Vec<(String, Value)> = components
        .par_iter()
        .filter_map(|c| {
            let name = c.name().to_string();
            match c.collect() {
                Ok(v) => Some((name, v)),
                Err(e) => {
                    if debug {
                        eprintln!("[{}] {:#}", name, e);
                    }
                    None
                }
            }
        })
        .collect();

    // Collect all the pairs into the main facts structure
    let facts: Map<String, Value> = pairs.into_iter().collect();

    let _j = serde_json::to_string(&Value::Object(facts))?;
    // println!("{}", j);

    let output = get_all_ip_devices()?;
    println!("{:?}", parse_ip_devices_output(&output)?);
    Ok(())
}
