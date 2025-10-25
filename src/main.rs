mod components;
mod context;
mod filesystem;

use crate::components::{Collector, kernel, memory};
use anyhow::Result;
use context::Ctx;
use serde_json::{Map, Value};
use std::sync::Arc;

fn main() -> Result<()> {
    let debug = true;
    let ctx = Ctx::new(debug);

    let components: Vec<Arc<dyn Collector>> = vec![
        Arc::new(kernel::KernelComponent::new()),
        Arc::new(memory::MemoryComponent::new()),
    ];

    // Build all the components into a huge Map,
    // ignoring errors (we'll log those to stderr with debug)
    let facts: Map<String, Value> = components
        .iter()
        .filter_map(|c| c.collect(&ctx).ok().map(|v| (c.name().to_string(), v)))
        .collect();

    let j = serde_json::to_string(&Value::Object(facts))?;
    println!("{}", j);
    Ok(())
}
