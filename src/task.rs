//! 任务调度协议
//!
//! 从 pandanetos 迁移，统一 pk ↔ Agent 之间的任务调度协议。
//! 覆盖任务模型、状态枚举、调度记录、进度上报、任务领取等。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 待下发
    Pending,
    /// 已确认领取
    Acked,
    /// 执行中
    Running,
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

impl TaskStatus {
    /// 是否为终止状态
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Success | Self::Failed | Self::Cancelled)
    }

    /// 是否可被调度
    pub fn is_schedulable(&self) -> bool {
        matches!(self, Self::Pending)
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pending => "pending",
            Self::Acked => "acked",
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        };
        f.write_str(s)
    }
}

/// 任务模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务 ID
    pub task_id: Uuid,
    /// 任务名称
    pub name: String,
    /// 文件名
    pub filename: String,
    /// 下载 URL
    pub url: String,
    /// 是否启用
    pub enabled: bool,
    /// 文件大小（字节）
    pub file_size_bytes: u64,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 备注（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// 标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 当前状态
    pub status: TaskStatus,
}

/// 调度记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatch {
    /// 调度 ID
    pub dispatch_id: Uuid,
    /// 任务 ID
    pub task_id: Uuid,
    /// 节点 ID（未领取时为 None）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<Uuid>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 领取时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<DateTime<Utc>>,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
    /// 当前状态
    pub status: String,
}

/// 调度配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchConfig {
    /// 调度超时（秒）
    pub timeout_secs: u64,
    /// 最大重试次数
    pub max_retries: u32,
    /// 全局默认最大并发任务数
    pub default_max_concurrent: u32,
    /// 心跳超时阈值（秒）
    pub heartbeat_timeout_secs: u64,
}

impl Default for DispatchConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 300,
            max_retries: 3,
            default_max_concurrent: 4,
            heartbeat_timeout_secs: 30,
        }
    }
}

/// 任务进度上报
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    /// 任务 ID
    pub task_id: Uuid,
    /// 已下载字节数
    pub downloaded_bytes: u64,
    /// 总字节数
    pub total_bytes: u64,
    /// 当前速度 bps
    pub speed_bps: u64,
    /// 活跃连接数
    pub active_connections: u32,
    /// 下载进度百分比（0.0 - 100.0）
    pub percent: f64,
    /// 已用时间（秒）
    pub elapsed_secs: f64,
}

/// 任务结果上报
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReport {
    /// 任务 ID
    pub task_id: Uuid,
    /// 是否成功
    pub success: bool,
    /// 总字节数
    pub total_bytes: u64,
    /// 已下载字节数
    pub downloaded_bytes: u64,
    /// 平均速度（字节/秒）
    pub avg_speed_bps: u64,
    /// 耗时（秒）
    pub elapsed_secs: f64,
    /// 错误信息（成功时为 None）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
}

/// 待领取任务响应（pk → Agent）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTask {
    /// 调度 ID
    pub dispatch_id: Uuid,
    /// 任务 ID
    pub task_id: Uuid,
    /// 任务名称
    pub name: String,
    /// 下载 URL
    pub url: String,
    /// 文件大小（字节）
    pub file_size_bytes: u64,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
}

/// 领取任务请求（Agent → pk）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimTaskRequest {
    /// 调度 ID
    pub dispatch_id: Uuid,
    /// 节点 ID
    pub node_id: Uuid,
}

/// 领取任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimTaskResponse {
    /// 是否领取成功
    pub claimed: bool,
    /// 任务详情（成功时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
    /// 失败原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 创建任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    /// 任务名称
    pub name: String,
    /// 下载 URL
    pub url: String,
    /// 备注（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// 标签
    #[serde(default)]
    pub tags: Vec<String>,
}

/// 任务查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQuery {
    /// 按状态筛选
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 按标签筛选
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 页码（从 1 开始）
    #[serde(default = "default_page")]
    pub page: u64,
    /// 每页大小
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}

impl Default for TaskQuery {
    fn default() -> Self {
        Self {
            status: None,
            tag: None,
            page: 1,
            page_size: 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_status_lifecycle() {
        assert!(TaskStatus::Pending.is_schedulable());
        assert!(!TaskStatus::Running.is_schedulable());
        assert!(TaskStatus::Success.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());
    }

    #[test]
    fn task_status_display() {
        assert_eq!(TaskStatus::Pending.to_string(), "pending");
        assert_eq!(TaskStatus::Running.to_string(), "running");
        assert_eq!(TaskStatus::Success.to_string(), "success");
    }

    #[test]
    fn dispatch_config_defaults() {
        let cfg = DispatchConfig::default();
        assert_eq!(cfg.timeout_secs, 300);
        assert_eq!(cfg.max_retries, 3);
        assert_eq!(cfg.default_max_concurrent, 4);
        assert_eq!(cfg.heartbeat_timeout_secs, 30);
    }

    #[test]
    fn task_query_defaults() {
        let q = TaskQuery::default();
        assert_eq!(q.page, 1);
        assert_eq!(q.page_size, 20);
    }

    #[test]
    fn claim_task_request_serialization() {
        let req = ClaimTaskRequest {
            dispatch_id: Uuid::new_v4(),
            node_id: Uuid::new_v4(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("dispatch_id"));
        assert!(json.contains("node_id"));
    }
}
