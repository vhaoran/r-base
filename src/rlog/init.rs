extern crate chrono;

use anyhow::anyhow;
use log::debug;
use simple_log::LogConfigBuilder;
use std::env;
use std::fs::File;
use std::path::PathBuf;

// use tracing::field::debug;
// use log::*;
// use simple_log::LogConfigBuilder;

use super::Config;

pub fn init(cfg: &Config) -> anyhow::Result<()> {
    let path: PathBuf = env::current_dir()?;
    let path = path.join(cfg.path.as_str());
    if !path.exists() {
        std::fs::create_dir_all(path.clone())?;
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

    // let level = match cfg.level.as_str() {
    //     "trace" => tracing::Level::TRACE,
    //     "debug" => tracing::Level::DEBUG,
    //     "info" => tracing::Level::INFO,
    //     "warn" => tracing::Level::WARN,
    //     "error" => tracing::Level::ERROR,
    //     _ => tracing::Level::DEBUG,
    // };

    // let f = tracing_appender::rolling::daily(&cfg.path, &cfg.file_name);
    // let (wr, _guard) = tracing_appender::non_blocking(f);
    // tracing_subscriber::fmt()
    //     .with_level(true)
    //     .with_max_level(level)
    //     .with_writer(wr)
    //     // .with_writer(std::io::stdout)
    //     .init();
    //
    debug!("....log module init ok.....");
    Ok(())
}
