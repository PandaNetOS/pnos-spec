//! 应用描述模型（app.yml）

use serde::{Deserialize, Serialize};

/// 应用状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppStatus {
    NotInstalled,
    Installing,
    Running,
    Stopped,
    Error,
}

impl AppStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppStatus::NotInstalled => "not_installed",
            AppStatus::Installing => "installing",
            AppStatus::Running => "running",
            AppStatus::Stopped => "stopped",
            AppStatus::Error => "error",
        }
    }
}

/// 健康检查类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthCheckType {
    Http,
    Tcp,
    Command,
}

/// 健康检查配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    #[serde(rename = "type")]
    pub check_type: HealthCheckType,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default = "default_interval")]
    pub interval: String,
    #[serde(default = "default_timeout")]
    pub timeout: String,
    #[serde(default = "default_retries")]
    pub retries: u32,
}

fn default_interval() -> String {
    "30s".to_string()
}
fn default_timeout() -> String {
    "5s".to_string()
}
fn default_retries() -> u32 {
    3
}

/// 应用描述清单（对应 app.yml）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppManifest {
    /// 唯一标识
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 语义化版本
    pub version: String,
    /// 简短描述
    pub description: String,
    /// 作者
    #[serde(default)]
    pub author: Option<String>,
    /// 主页
    #[serde(default)]
    pub homepage: Option<String>,
    /// 图标文件名
    pub icon: String,
    /// 分类
    #[serde(default)]
    pub categories: Vec<String>,
    /// 标签
    #[serde(default)]
    pub tags: Vec<String>,

    /// Docker 镜像
    pub image: String,

    /// 端口映射
    #[serde(default)]
    pub ports: Vec<crate::container::PortMapping>,

    /// 卷映射
    #[serde(default)]
    pub volumes: Vec<crate::container::VolumeMount>,

    /// 环境变量
    #[serde(default)]
    pub env: Vec<crate::container::EnvVar>,

    /// 设备映射
    #[serde(default)]
    pub devices: Vec<String>,

    /// 网络名
    #[serde(default = "default_network")]
    pub network: String,

    /// 重启策略
    #[serde(default = "default_restart")]
    pub restart: String,

    /// 是否特权模式
    #[serde(default)]
    pub privileged: bool,

    /// 健康检查
    #[serde(default)]
    pub health_check: Option<HealthCheck>,

    /// 依赖的应用 id 列表
    #[serde(default)]
    pub depends_on: Vec<String>,
}

fn default_network() -> String {
    "pnos-net".to_string()
}
fn default_restart() -> String {
    "unless-stopped".to_string()
}

impl AppManifest {
    /// 容器名：pnos-app-{id}
    pub fn container_name(&self) -> String {
        format!("pnos-app-{}", self.id)
    }

    /// 容器标签
    pub fn labels(&self) -> Vec<(String, String)> {
        vec![
            ("io.pnos.managed".to_string(), "true".to_string()),
            ("io.pnos.app.id".to_string(), self.id.clone()),
            ("io.pnos.version".to_string(), self.version.clone()),
        ]
    }
}
