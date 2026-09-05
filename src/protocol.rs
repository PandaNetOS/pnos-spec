//! API 路径常量与协议定义

/// API 版本前缀
pub const API_V1: &str = "/api/v1";

// ===== 系统 =====
pub const SYSTEM_INFO: &str = "/system/info";
pub const SYSTEM_STATS: &str = "/system/stats";
pub const SYSTEM_HEALTH: &str = "/system/health";

// ===== 容器 =====
pub const CONTAINERS: &str = "/containers";
pub const CONTAINER_DETAIL: &str = "/containers/:id";
pub const CONTAINER_START: &str = "/containers/:id/start";
pub const CONTAINER_STOP: &str = "/containers/:id/stop";
pub const CONTAINER_RESTART: &str = "/containers/:id/restart";
pub const CONTAINER_LOGS: &str = "/containers/:id/logs";

// ===== 商店 =====
pub const STORE_SOURCES: &str = "/store/sources";
pub const STORE_SOURCE_REFRESH: &str = "/store/sources/:id/refresh";
pub const STORE_APPS: &str = "/store/apps";
pub const STORE_APP_DETAIL: &str = "/store/apps/:id";
pub const STORE_APP_INSTALL: &str = "/store/apps/:id/install";
pub const STORE_APP_UNINSTALL: &str = "/store/apps/:id/uninstall";

// ===== 文件 =====
pub const FILES_LIST: &str = "/files/list";
pub const FILES_DOWNLOAD: &str = "/files/download";
pub const FILES_UPLOAD: &str = "/files/upload";
pub const FILES_MKDIR: &str = "/files/mkdir";
pub const FILES_DELETE: &str = "/files/delete";
pub const FILES_RENAME: &str = "/files/rename";

/// WebSocket 消息类型
pub mod ws {
    pub const APP_INSTALL_PROGRESS: &str = "app_install_progress";
    pub const APP_STATUS_CHANGED: &str = "app_status_changed";
    pub const CONTAINER_LOG: &str = "container_log";
    pub const SYSTEM_STATS: &str = "system_stats";
}
