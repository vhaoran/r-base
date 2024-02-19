extern crate chrono;

use anyhow::anyhow;
// use tracing::*;
// use simple_log::LogConfigBuilder;
use chrono::{Datelike, Local, Timelike};
use std::fs::File;
use std::path::PathBuf;
use std::{env, io};

use tracing::subscriber::set_global_default;
use tracing::*;
use tracing_appender::rolling;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{self, fmt, fmt::time::FormatTime};

use super::Config;

// pub fn init(cfg: &Config) -> anyhow::Result<()> {
//     let path: PathBuf = env::current_dir()?;
//     let path = path.join(cfg.path.as_str());
//     if !path.exists() {
//         std::fs::create_dir_all(path.clone())?;
//     }
//     let path = path.join(cfg.file_name.as_str());
//     let s = path.to_str().unwrap().to_string();
//
//     let config = LogConfigBuilder::builder()
//         .path(s)
//         .size(cfg.size)
//         .roll_count(cfg.roll_count)
//         .level(cfg.level.as_str())
//         .output_file()
//         .output_console()
//         .build();
//
//     let _ = simple_log::new(config).map_err(|e| anyhow!("{:?}", e))?;
//     debug!("....log module init ok.....");
//     Ok(())
// }

pub fn init(cfg: &Config) -> anyhow::Result<()> {
    let level = match cfg.level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::DEBUG,
    };
    let _ = self::do_init_log(None, None, None, None, Some(level))?;

    debug!("....log module init ok.....");
    Ok(())
}

struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let dt = Local::now();
        let s = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{}",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            dt.nanosecond() / 1000
        );

        write!(w, "{s}")
    }
}

fn do_init_log(
    dir: Option<&str>,
    filename_prefix: Option<&str>,
    filename_suffix: Option<&str>,
    max_files: Option<usize>,
    level: Option<tracing::Level>,
) -> anyhow::Result<()> {
    let dir = dir.unwrap_or("./logs");
    let filename_prefix = filename_prefix.unwrap_or("log");
    let filename_suffix = filename_suffix.unwrap_or("log");
    let max_files = max_files.unwrap_or(3);
    let level = level.unwrap_or(tracing::Level::TRACE);

    // let warn_file = rolling::daily(dir, "log").with_max_level(level);
    let rotate_file = RollingFileAppender::builder()
        .rotation(Rotation::DAILY) // rotate log files once every hour
        .max_log_files(max_files)
        .filename_prefix(filename_prefix) // log file names will be prefixed with `myapp.`
        .filename_suffix(filename_suffix) // log file names will be suffixed with `.log`
        .build(dir)?;

    let rotate_err = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(max_files)
        .filename_prefix("err")
        .filename_suffix(filename_suffix)
        .build(dir)?
        .with_max_level(Level::ERROR);

    let all_files = rotate_file.and(io::stdout).and(rotate_err);

    tracing_subscriber::fmt()
        // .pretty()
        .with_writer(all_files)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        // .with_thread_names(true)
        .with_timer(LocalTimer)
        .with_max_level(level) //tracing::Level::TRACE
        .init();

    Ok(())
}
