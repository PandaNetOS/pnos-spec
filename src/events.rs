//! WebSocket 事件协议
//!
//! pnos-runtime 提供 WebSocket 端点 /api/v1/ws，应用和前端可以订阅事件。

use serde::{Deserialize, Serialize};

// ===== 事件类型常量 =====

/// 应用安装进度
pub const EVENT_APP_INSTALL_PROGRESS: &str = "app.install_progress";
/// 应用状态变更
pub const EVENT_APP_STATUS_CHANGED: &str = "app.status_changed";
/// 应用日志
pub const EVENT_APP_LOG: &str = "app.log";
/// 系统实时监控数据
pub const EVENT_SYSTEM_STATS: &str = "system.stats";
/// 系统通知
pub const EVENT_SYSTEM_NOTIFICATION: &str = "system.notification";
/// 任务进度
pub const EVENT_TASK_PROGRESS: &str = "task.progress";
/// 任务完成
pub const EVENT_TASK_COMPLETED: &str = "task.completed";

/// WebSocket 消息信封
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    /// 事件类型（如 app.install_progress）
    pub event_type: String,
    /// 来源应用 ID（系统事件为 "system"）
    #[serde(default = "default_source")]
    pub source: String,
    /// 时间戳（RFC3339）
    pub timestamp: String,
    /// 事件数据
    pub payload: serde_json::Value,
}

fn default_source() -> String {
    "system".to_string()
}

impl WsMessage {
    /// 创建事件消息
    pub fn new(event_type: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            event_type: event_type.into(),
            source: default_source(),
            timestamp: crate::time::now_rfc3339(),
            payload,
        }
    }

    /// 设置来源
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }
}

/// 客户端订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsSubscribe {
    /// 动作：subscribe / unsubscribe
    pub action: String,
    /// 订阅的事件类型（支持前缀匹配，如 "app.*"）
    pub event_type: String,
}

/// 应用安装进度事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstallProgress {
    pub app_id: String,
    pub status: String, // downloading / extracting / configuring / running / failed
    pub progress: u8,   // 0-100
    pub message: Option<String>,
}

/// 应用状态变更事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStatusChanged {
    pub app_id: String,
    pub old_status: String,
    pub new_status: String,
}

/// 系统通知事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemNotification {
    pub level: String, // info / warning / error
    pub title: String,
    pub message: String,
}
