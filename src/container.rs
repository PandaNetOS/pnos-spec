//! 容器配置模型

use serde::{Deserialize, Serialize};

/// 端口映射
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    /// 容器内端口
    pub container: u16,
    /// 宿主端口，0 表示自动分配
    #[serde(default)]
    pub host: u16,
    /// 协议 tcp/udp
    #[serde(default = "default_protocol")]
    pub protocol: String,
}

fn default_protocol() -> String {
    "tcp".to_string()
}

/// 卷映射
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    /// 容器内路径
    pub container: String,
    /// 宿主路径（支持变量 {{app_data}} {{media_dir}} 等）
    pub host: String,
    /// 是否只读
    #[serde(default)]
    pub read_only: bool,
}

/// 环境变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

/// 容器运行配置（解析变量后的最终配置）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub name: String,
    pub image: String,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub env: Vec<EnvVar>,
    pub network: String,
    pub restart: String,
    pub privileged: bool,
    pub labels: Vec<(String, String)>,
}
