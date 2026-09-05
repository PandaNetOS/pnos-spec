//! API 路径常量与协议定义

/// API 版本前缀
pub const API_V1: &str = "/api/v1";

// ===== 系统 =====
pub const SYSTEM_INFO: &str = "/system/info";
pub const SYSTEM_STATS: &str = "/system/stats";
pub const SYSTEM_HEALTH: &str = "/system/health";

// ===== 应用注册与发现 =====
pub const REGISTER: &str = "/apps/register";
pub const UNREGISTER: &str = "/apps/unregister";
pub const HEARTBEAT: &str = "/apps/heartbeat";
pub const APPS_LIST: &str = "/apps";
pub const APP_DETAIL: &str = "/apps/:id";
pub const APP_DISCOVER: &str = "/apps/:id/discover";

// ===== 应用管理（商店安装的应用） =====
pub const STORE_APPS: &str = "/store/apps";
pub const STORE_APP_DETAIL: &str = "/store/apps/:id";
pub const STORE_APP_INSTALL: &str = "/store/apps/:id/install";
pub const STORE_APP_UNINSTALL: &str = "/store/apps/:id/uninstall";
pub const STORE_APP_START: &str = "/store/apps/:id/start";
pub const STORE_APP_STOP: &str = "/store/apps/:id/stop";
pub const STORE_APP_RESTART: &str = "/store/apps/:id/restart";
pub const STORE_APP_LOGS: &str = "/store/apps/:id/logs";
pub const STORE_SOURCES: &str = "/store/sources";
pub const STORE_SOURCE_REFRESH: &str = "/store/sources/:id/refresh";

// ===== 反向代理 =====
/// 应用代理前缀：/app/{id}/*
pub const APP_PROXY_PREFIX: &str = "/app";

// ===== 文件 =====
pub const FILES_LIST: &str = "/files/list";
pub const FILES_DOWNLOAD: &str = "/files/download";
pub const FILES_UPLOAD: &str = "/files/upload";
pub const FILES_MKDIR: &str = "/files/mkdir";
pub const FILES_DELETE: &str = "/files/delete";
pub const FILES_RENAME: &str = "/files/rename";

// ===== WebSocket =====
pub const WS: &str = "/ws";

/// WebSocket 消息类型（已迁移到 events.rs，此处保留兼容引用）
pub mod ws {
    pub use crate::events::*;
}
