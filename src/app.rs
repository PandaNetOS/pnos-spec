//! 应用描述模型（app.yml）— 二进制部署格式

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

/// UI 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WebUiType {
    /// iframe 嵌入
    Iframe,
    /// pnos-web 原生组件
    Native,
}

/// Web UI 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    /// 是否启用 Web UI
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// UI 类型
    #[serde(default = "default_web_type")]
    pub r#type: WebUiType,
    /// UI 根路径
    #[serde(default = "default_path")]
    pub path: String,
    /// 原生 UI 组件包地址（type=native 时必填）
    #[serde(default)]
    pub ui_package: Option<String>,
}

fn default_true() -> bool {
    true
}
fn default_web_type() -> WebUiType {
    WebUiType::Iframe
}
fn default_path() -> String {
    "/".to_string()
}

/// 二进制下载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryConfig {
    /// 下载地址
    pub download_url: String,
    /// SHA256 校验（可选）
    #[serde(default)]
    pub sha256: Option<String>,
    /// 解压后的可执行文件名
    pub binary_name: String,
    /// 安装目录（支持变量 {{app_data}}）
    #[serde(default = "default_install_dir")]
    pub install_dir: String,
}

fn default_install_dir() -> String {
    "{{app_data}}/bin".to_string()
}

/// 运行配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunConfig {
    /// 启动参数
    #[serde(default)]
    pub args: Vec<String>,
    /// 环境变量
    #[serde(default)]
    pub env: Vec<EnvVar>,
    /// 监听端口
    pub port: u16,
    /// 工作目录（支持变量）
    #[serde(default = "default_working_dir")]
    pub working_dir: String,
    /// 重启策略：always / unless-stopped / no
    #[serde(default = "default_restart")]
    pub restart: String,
}

fn default_working_dir() -> String {
    "{{app_data}}".to_string()
}
fn default_restart() -> String {
    "unless-stopped".to_string()
}

/// 环境变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
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
    /// 二进制下载配置
    pub binary: BinaryConfig,
    /// 运行配置
    pub run: RunConfig,
    /// Web UI 配置
    #[serde(default)]
    pub web: Option<WebConfig>,
    /// 健康检查
    #[serde(default)]
    pub health_check: Option<HealthCheck>,
    /// 依赖的应用 id 列表
    #[serde(default)]
    pub depends_on: Vec<String>,
}

impl AppManifest {
    /// 进程名：pnos-app-{id}
    pub fn process_name(&self) -> String {
        format!("pnos-app-{}", self.id)
    }

    /// 数据目录：{{app_data}}/{id}
    pub fn data_dir(&self) -> String {
        format!("{{{{app_data}}}}/{}", self.id)
    }
}
