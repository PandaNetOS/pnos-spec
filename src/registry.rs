//! 应用注册与发现协议
//!
//! 应用启动后向 pnos-runtime 注册自己，定期心跳，退出时注销。
//! pnos-runtime 维护应用注册表，提供应用发现服务。

use serde::{Deserialize, Serialize};

use crate::app::AppStatus;

/// 应用注册请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRegisterRequest {
    /// 应用唯一标识
    pub id: String,
    /// 应用显示名称
    pub name: String,
    /// 应用版本
    pub version: String,
    /// 应用监听端口
    pub port: u16,
    /// 健康检查路径（如 /health）
    #[serde(default = "default_health_path")]
    pub health_check_path: String,
    /// Web UI 根路径（如 /，无 UI 则为 None）
    #[serde(default)]
    pub web_path: Option<String>,
    /// 依赖的应用 id 列表
    #[serde(default)]
    pub dependencies: Vec<String>,
}

fn default_health_path() -> String {
    "/health".to_string()
}

/// 应用注册响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRegisterResponse {
    /// 分配的应用 Token
    pub token: String,
    /// 应用 ID
    pub app_id: String,
    /// 注册时间
    pub registered_at: String,
}

/// 心跳请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {
    /// 应用 ID
    pub id: String,
    /// 当前状态
    pub status: AppStatus,
    /// 可选：额外状态信息
    #[serde(default)]
    pub message: Option<String>,
}

/// 应用信息（注册表中的记录）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// 应用 ID
    pub id: String,
    /// 应用名称
    pub name: String,
    /// 应用版本
    pub version: String,
    /// 应用地址（通常为 127.0.0.1）
    pub address: String,
    /// 应用监听端口
    pub port: u16,
    /// 应用状态
    pub status: AppStatus,
    /// 健康检查路径
    pub health_check_path: String,
    /// Web UI 路径
    #[serde(default)]
    pub web_path: Option<String>,
    /// 依赖列表
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// 注册时间（RFC3339）
    pub registered_at: String,
    /// 最后心跳时间（RFC3339）
    pub last_heartbeat: String,
}

/// 应用发现响应（查询单个应用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDiscoverResponse {
    /// 应用 ID
    pub id: String,
    /// 应用地址
    pub address: String,
    /// 应用端口
    pub port: u16,
    /// 应用状态
    pub status: AppStatus,
    /// 完整的基础 URL（如 http://127.0.0.1:18080）
    pub base_url: String,
}

impl AppDiscoverResponse {
    /// 从 AppInfo 构建
    pub fn from_info(info: &AppInfo) -> Self {
        Self {
            id: info.id.clone(),
            address: info.address.clone(),
            port: info.port,
            status: info.status,
            base_url: format!("http://{}:{}", info.address, info.port),
        }
    }
}
