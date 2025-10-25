use std::path::Path;

use crate::context::Ctx;
use anyhow::{Context, Result};

pub fn slurp(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.as_ref().display())).map(|s| s.trim().to_string())
}

#[allow(dead_code)]
pub fn get_dirs_in_path(ctx: &Ctx, path: &str) -> Vec<String> {
    let mut dirs = Vec::new();
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                dirs.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            if ctx.debug {
                eprintln!("Failed to read directory {}: {}", path, err);
            }
        }
    }
    dirs
}
