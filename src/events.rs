//! WebSocket 事件协议（统一信封格式）
//!
//! pnos-runtime / pk 提供 WebSocket 端点，所有组件和前端订阅事件。
//! 统一使用 WsMessage 信封格式，消灭 tagged enum，通过 event_type 区分事件类型。

use serde::{Deserialize, Serialize};

// ===== 事件类型常量（点分隔命名，统一规范） =====

// --- 应用事件 ---
/// 应用安装进度
pub const EVENT_APP_INSTALL_PROGRESS: &str = "app.install_progress";
/// 应用状态变更
pub const EVENT_APP_STATUS_CHANGED: &str = "app.status_changed";
/// 应用日志
pub const EVENT_APP_LOG: &str = "app.log";

// --- 组件事件（统一应用+Agent） ---
/// 组件注册
pub const EVENT_COMPONENT_REGISTERED: &str = "component.registered";
/// 组件注销
pub const EVENT_COMPONENT_UNREGISTERED: &str = "component.unregistered";
/// 组件状态变更
pub const EVENT_COMPONENT_STATUS_CHANGED: &str = "component.status_changed";
/// 组件心跳超时（离线）
pub const EVENT_COMPONENT_OFFLINE: &str = "component.offline";

// --- 任务事件（Agent 任务调度） ---
/// 新任务下发
pub const EVENT_TASK_NEW: &str = "task.new";
/// 任务进度更新
pub const EVENT_TASK_PROGRESS: &str = "task.progress";
/// 任务完成
pub const EVENT_TASK_COMPLETED: &str = "task.completed";
/// 任务失败
pub const EVENT_TASK_FAILED: &str = "task.failed";
/// 任务取消
pub const EVENT_TASK_CANCELLED: &str = "task.cancelled";
/// 任务被领取
pub const EVENT_TASK_CLAIMED: &str = "task.claimed";

// --- 节点事件（Agent 节点管理） ---
/// 节点状态变更
pub const EVENT_NODE_STATUS: &str = "node.status";
/// 节点被删除
pub const EVENT_NODE_DELETED: &str = "node.deleted";

// --- 服务发现事件 ---
/// 服务变更（注册/注销/健康变化）
pub const EVENT_SERVICE_CHANGED: &str = "service.changed";

// --- 配置事件 ---
/// 配置变更
pub const EVENT_CONFIG_CHANGED: &str = "config.changed";

// --- 系统事件 ---
/// 系统实时监控数据
pub const EVENT_SYSTEM_STATS: &str = "system.stats";
/// 系统通知
pub const EVENT_SYSTEM_NOTIFICATION: &str = "system.notification";

// --- Peer 发现事件（PDC 专用） ---
/// 发现新任务
pub const EVENT_DISCOVER_TASK: &str = "discover.task";
/// 发现启动
pub const EVENT_DISCOVERY_STARTED: &str = "discovery.started";
/// 发现结果
pub const EVENT_DISCOVERY_RESULT: &str = "discovery.result";

/// WebSocket 消息信封（统一格式，所有事件共用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    /// 事件类型（点分隔，如 task.new / component.status_changed）
    pub event_type: String,
    /// 来源组件 ID（系统事件为 "system"）
    #[serde(default = "default_source")]
    pub source: String,
    /// 时间戳（RFC3339）
    pub timestamp: String,
    /// 事件数据（任意 JSON，由 event_type 决定结构）
    pub payload: serde_json::Value,
}

fn default_source() -> String {
    "system".to_string()
}

impl WsMessage {
    /// 创建事件消息（自动填充时间戳）
    pub fn new(event_type: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            event_type: event_type.into(),
            source: default_source(),
            timestamp: crate::time::now_rfc3339(),
            payload,
        }
    }

    /// 设置来源组件 ID
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// 设置自定义时间戳
    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    /// 尝试将 payload 解析为指定类型
    pub fn parse_payload<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.payload.clone())
    }
}

/// 客户端订阅/取消订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsSubscribe {
    /// 动作：subscribe / unsubscribe
    pub action: String,
    /// 订阅的事件类型（支持前缀匹配，如 "task.*" / "component.*"）
    pub event_type: String,
}

impl WsSubscribe {
    /// 构造订阅请求
    pub fn subscribe(event_type: impl Into<String>) -> Self {
        Self {
            action: "subscribe".into(),
            event_type: event_type.into(),
        }
    }

    /// 构造取消订阅请求
    pub fn unsubscribe(event_type: impl Into<String>) -> Self {
        Self {
            action: "unsubscribe".into(),
            event_type: event_type.into(),
        }
    }
}

// ===== 常用事件 payload 结构 =====

/// 组件状态变更事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatusChanged {
    pub component_id: String,
    pub component_type: String,
    pub old_status: String,
    pub new_status: String,
}

/// 任务进度事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgressPayload {
    pub task_id: String,
    pub status: String,
    pub progress: f64,
    pub speed_bps: u64,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub message: Option<String>,
}

/// 任务完成事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletedPayload {
    pub task_id: String,
    pub success: bool,
    pub total_bytes: u64,
    pub elapsed_secs: f64,
    pub error: Option<String>,
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

/// 服务变更事件 payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceChangedPayload {
    pub service_name: String,
    pub action: String, // registered / unregistered / health_changed
    pub address: String,
    pub port: u16,
    pub healthy: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_message_envelope_format() {
        let msg =
            WsMessage::new(EVENT_TASK_NEW, serde_json::json!({"task_id": "123"})).with_source("pk");
        assert_eq!(msg.event_type, "task.new");
        assert_eq!(msg.source, "pk");
        assert!(!msg.timestamp.is_empty());
        assert_eq!(msg.payload["task_id"], "123");
    }

    #[test]
    fn ws_message_parse_payload() {
        let payload = TaskProgressPayload {
            task_id: "t1".into(),
            status: "running".into(),
            progress: 50.0,
            speed_bps: 1000,
            downloaded_bytes: 500,
            total_bytes: 1000,
            message: None,
        };
        let msg = WsMessage::new(EVENT_TASK_PROGRESS, serde_json::to_value(&payload).unwrap());
        let parsed: TaskProgressPayload = msg.parse_payload().unwrap();
        assert_eq!(parsed.task_id, "t1");
        assert_eq!(parsed.progress, 50.0);
    }

    #[test]
    fn ws_subscribe_actions() {
        let sub = WsSubscribe::subscribe("task.*");
        assert_eq!(sub.action, "subscribe");
        assert_eq!(sub.event_type, "task.*");

        let unsub = WsSubscribe::unsubscribe("task.*");
        assert_eq!(unsub.action, "unsubscribe");
    }

    #[test]
    fn event_type_constants_are_dot_separated() {
        assert!(EVENT_TASK_NEW.contains('.'));
        assert!(EVENT_COMPONENT_STATUS_CHANGED.contains('.'));
        assert!(EVENT_SERVICE_CHANGED.contains('.'));
    }
}
