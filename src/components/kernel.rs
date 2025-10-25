use crate::{context::Ctx, filesystem::slurp};

use crate::Collector;
use anyhow::Result;
use serde::Serialize;
use serde_json::to_value;

#[derive(Serialize, Debug)]
pub struct KernelFacts {
    pub ostype: String,
    pub arch: String,
    pub release: String,
    pub version: String,
    pub majorversion: String,
}

pub struct KernelComponent;

impl KernelComponent {
    pub fn new() -> Self {
        Self
    }
}
impl Collector for KernelComponent {
    fn name(&self) -> &'static str {
        return "kernel";
    }

    fn collect(&self, ctx: &Ctx) -> Result<serde_json::Value> {
        let fname = "/proc/sys/kernel/ostype";
        let ostype = slurp(ctx, fname);

        let fname = "/proc/sys/kernel/arch";
        let arch = slurp(ctx, fname);

        let fname = "/proc/sys/kernel/osrelease";
        let release = slurp(ctx, fname);

        let version = match release.split("-").nth(0) {
            Some(t) => t.to_string(),
            None => release.clone(),
        };
        let majorversion = release.split(".").take(2).collect::<Vec<_>>().join(".");

        let kf = KernelFacts {
            ostype,
            arch,
            release,
            majorversion,
            version,
        };
        let j = to_value(kf)?;
        Ok(j)
    }
}
