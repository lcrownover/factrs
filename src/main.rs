mod components;
mod context;
mod filesystem;

use crate::components::{Component, kernel, memory};
use anyhow::Result;
use context::Ctx;
use serde_json::{Map, Value};
use std::sync::Arc;

fn main() -> Result<()> {
    let debug = true;
    let ctx = Ctx::new(debug);
    let components: Vec<Arc<dyn Component>> = vec![
        Arc::new(kernel::KernelComponent::new()),
        Arc::new(memory::MemoryComponent::new()),
    ];

    let mut root = Map::new();

    for c in &components {
        match c.collect(&ctx) {
            Ok(value) => {
                root.insert(c.name().to_string(), value);
            }
            Err(err) => {
                println!("error: {}", err);
            }
        }
    }

    let j = serde_json::to_string(&Value::Object(root))?;
    println!("{}", j);
    Ok(())
}
