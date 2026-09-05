//! 健康检查协议
//!
//! 所有 pnos 应用必须实现 GET /health 端点，返回统一格式。

use serde::{Deserialize, Serialize};

/// 健康状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// 正常
    Ok,
    /// 降级（部分依赖不可用）
    Degraded,
    /// 不可用
    Down,
}

impl HealthStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthStatus::Ok => "ok",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Down => "down",
        }
    }
}

/// 健康检查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// 整体健康状态
    pub status: HealthStatus,
    /// 应用版本
    pub version: String,
    /// 运行时间（秒）
    pub uptime: u64,
    /// 依赖项健康状态
    #[serde(default)]
    pub dependencies: Vec<DependencyHealth>,
}

/// 依赖项健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyHealth {
    /// 依赖名称（应用 ID 或组件名）
    pub name: String,
    /// 状态
    pub status: HealthStatus,
    /// 额外信息
    #[serde(default)]
    pub message: Option<String>,
}

impl HealthResponse {
    /// 创建健康响应
    pub fn new(version: impl Into<String>, uptime: u64) -> Self {
        Self {
            status: HealthStatus::Ok,
            version: version.into(),
            uptime,
            dependencies: Vec::new(),
        }
    }

    /// 添加依赖项
    pub fn dependency(mut self, name: impl Into<String>, status: HealthStatus) -> Self {
        self.dependencies.push(DependencyHealth {
            name: name.into(),
            status,
            message: None,
        });
        self
    }

    /// 判断是否健康
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Ok)
    }
}
