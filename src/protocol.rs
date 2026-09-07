//! API 路径常量与协议定义（统一组件路径）
//!
//! 统一应用与 Agent 的 API 路径，所有通信实体都是 Component。
//! 主路径使用 /components/*，同时保留 /apps/* 作为兼容别名。

/// API 版本前缀
pub const API_V1: &str = "/api/v1";

// ===== 系统 =====
pub const SYSTEM_INFO: &str = "/system/info";
pub const SYSTEM_STATS: &str = "/system/stats";
pub const SYSTEM_HEALTH: &str = "/system/health";

// ===== 组件注册与发现（统一路径，主路径） =====
pub const COMPONENTS_REGISTER: &str = "/components/register";
pub const COMPONENTS_UNREGISTER: &str = "/components/unregister";
pub const COMPONENTS_HEARTBEAT: &str = "/components/heartbeat";
pub const COMPONENTS_LIST: &str = "/components";
pub const COMPONENT_DETAIL: &str = "/components/:id";
pub const COMPONENT_DISCOVER: &str = "/components/:id/discover";

// ===== 应用注册与发现（兼容路径，已废弃，推荐用 /components/*） =====
pub const REGISTER: &str = "/apps/register";
pub const UNREGISTER: &str = "/apps/unregister";
pub const HEARTBEAT: &str = "/apps/heartbeat";
pub const APPS_LIST: &str = "/apps";
pub const APP_DETAIL: &str = "/apps/:id";
pub const APP_DISCOVER: &str = "/apps/:id/discover";

// ===== 任务调度（pk ↔ Agent） =====
pub const TASKS: &str = "/tasks";
pub const TASK_DETAIL: &str = "/tasks/:id";
pub const TASK_PROGRESS: &str = "/tasks/:id/progress";
pub const TASK_REPORT: &str = "/tasks/:id/report";
pub const TASK_CANCEL: &str = "/tasks/:id/cancel";

// ===== 调度记录 =====
pub const DISPATCHES_PENDING: &str = "/dispatches/pending";
pub const DISPATCH_CLAIM: &str = "/dispatches/claim";

// ===== 节点管理（兼容路径，推荐用 /components/*） =====
pub const NODES: &str = "/nodes";
pub const NODE_DETAIL: &str = "/nodes/:id";

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
/// 组件代理前缀：/component/{id}/*（统一路径）
pub const COMPONENT_PROXY_PREFIX: &str = "/component";

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_paths_are_defined() {
        assert_eq!(COMPONENTS_REGISTER, "/components/register");
        assert_eq!(COMPONENTS_UNREGISTER, "/components/unregister");
        assert_eq!(COMPONENTS_HEARTBEAT, "/components/heartbeat");
        assert_eq!(COMPONENTS_LIST, "/components");
        assert_eq!(COMPONENT_DETAIL, "/components/:id");
        assert_eq!(COMPONENT_DISCOVER, "/components/:id/discover");
    }

    #[test]
    fn task_paths_are_defined() {
        assert_eq!(TASKS, "/tasks");
        assert_eq!(TASK_DETAIL, "/tasks/:id");
        assert_eq!(DISPATCHES_PENDING, "/dispatches/pending");
        assert_eq!(DISPATCH_CLAIM, "/dispatches/claim");
    }

    #[test]
    fn legacy_app_paths_are_preserved() {
        assert_eq!(REGISTER, "/apps/register");
        assert_eq!(HEARTBEAT, "/apps/heartbeat");
        assert_eq!(APPS_LIST, "/apps");
    }
}
