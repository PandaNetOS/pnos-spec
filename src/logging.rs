//! 结构化日志初始化（tracing + env-filter）

use tracing_subscriber::{fmt, EnvFilter};

/// 初始化结构化日志
pub fn init_logging(default_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .json()
        .init();
}

/// 初始化人类可读日志（开发环境）
pub fn init_logging_pretty(default_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}
