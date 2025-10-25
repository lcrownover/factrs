use std::path::Path;

use crate::{context::Ctx, filesystem::slurp};

use crate::Collector;
use anyhow::{Context, Result};
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

    fn collect(&self) -> Result<serde_json::Value> {
        let ostype = slurp("/proc/sys/kernel/ostype")?;
        let arch = slurp("/proc/sys/kernel/arch")?;
        let release = slurp("/proc/sys/kernel/osrelease")?;

        let version = release
            .split_once("-")
            .map(|(v, _)| v)
            .unwrap_or(&release)
            .to_string();

        let majorversion = release.split(".").take(2).collect::<Vec<_>>().join(".");

        let kf = KernelFacts {
            ostype,
            arch,
            release,
            majorversion,
            version,
        };
        Ok(to_value(kf)?)
    }
}
