use crate::{context::Ctx, filesystem::get_file_contents_or_empty_string};

use super::Component;
use anyhow::Result;
use serde::Serialize;
use serde_json::to_value;
use std::fs;

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
impl Component for KernelComponent {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn collect(&self, ctx: &Ctx) -> Result<serde_json::Value> {
        let fname = "/proc/sys/kernel/ostype";
        let ostype = get_file_contents_or_empty_string(ctx, fname);

        let fname = "/proc/sys/kernel/arch";
        let arch = get_file_contents_or_empty_string(ctx, fname);

        let fname = "/proc/sys/kernel/osrelease";
        let release = get_file_contents_or_empty_string(ctx, fname);

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
