pub mod kernel;
pub mod memory;
use anyhow::Result;

use crate::context::Ctx;

pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, ctx: &Ctx) -> Result<serde_json::Value>;
}
