extern crate chrono;

use std::io::IsTerminal;
use tokio::time::sleep;
use tracing::{debug, error, info, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter, Registry};

// ✅ 核心修改 1: 导入 WorkerGuard
use crate::rlog::Config;
use tracing_appender::non_blocking::WorkerGuard;

//-------------------------------------
use futures::executor::block_on;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::OnceCell;

//
type IType = (Option<WorkerGuard>, Option<WorkerGuard>);

async fn instance() -> &'static Arc<Mutex<IType>> {
    static INSTANCE: OnceCell<Arc<Mutex<IType>>> = OnceCell::const_new();
    INSTANCE
        .get_or_init(|| async {
            let m: (Option<WorkerGuard>, Option<WorkerGuard>) = (None, None);
            Arc::new(Mutex::new(m))
        })
        .await
}

pub async fn push_guard((guard1, guard2): (WorkerGuard, WorkerGuard)) {
    let a = self::instance().await.clone();
    let mut m = a.lock().await;
    *m = (Some(guard1), Some(guard2));
}
//-----------
// --------------------------

pub fn init(cfg: &Config) -> anyhow::Result<()> {
    let guard = init_logging(cfg)?;
    let _ = block_on(self::push_guard((guard.0, guard.1)));

    Ok(())
}

fn init_logging(cfg: &Config) -> anyhow::Result<(WorkerGuard, WorkerGuard)> {
    let level = cfg.level_str();
    let log_dir = cfg.path.as_str();
    let file_name = cfg.file_name.as_str();

    // 1. 普通日志文件 (app.log) - 记录所有日志
    let file_appender_all = tracing_appender::rolling::daily(log_dir, file_name);
    let (non_blocking_all, guard_all) = tracing_appender::non_blocking(file_appender_all);

    // 2. 错误日志文件 (error.log) - 仅记录错误
    let file_appender_err = tracing_appender::rolling::daily(log_dir, "error.log");
    let (non_blocking_err, guard_err) = tracing_appender::non_blocking(file_appender_err);

    // 3. 组装 Subscriber
    let subscriber = Registry::default()
        // 控制台输出
        .with(fmt::layer().with_ansi(std::io::stdout().is_terminal()))
        // 全量文件输出 (Debug 级别以上)
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking_all)
                .with_filter(EnvFilter::new(level.as_str())),
        )
        // 错误文件输出 (仅 Error 级别)
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking_err)
                .with_filter(EnvFilter::new("error")),
        )
        .with(ErrorLayer::default()); // 启用错误追踪

    tracing::subscriber::set_global_default(subscriber)?;

    Ok((guard_all, guard_err))
}
