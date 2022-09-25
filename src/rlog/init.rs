extern crate chrono;

use anyhow::anyhow;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use log::*;
use simple_log::LogConfigBuilder;

use super::Config;

pub fn init(cfg: &Config) -> anyhow::Result<()> {
    let path: PathBuf = env::current_dir()?;
    let path = path.join(cfg.path.as_str());
    if !path.exists() {
        std::fs::create_dir(path.clone())?;
    }
    let path = path.join(cfg.file_name.as_str());
    let s = path.to_str().unwrap().to_string();

    let config = LogConfigBuilder::builder()
        .path(s)
        .size(cfg.size)
        .roll_count(cfg.roll_count)
        .level(cfg.level.as_str())
        .output_file()
        .output_console()
        .build();

    let _ = simple_log::new(config).map_err(|e| anyhow!("{:?}", e))?;

    debug!("..log..module init ok.....");
    Ok(())
}
