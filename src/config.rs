//! 配置加载工具（YAML + 环境变量覆盖）

use serde::{Deserialize, Serialize};

/// pnos 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnosConfig {
    /// 数据根目录
    #[serde(default = "default_data_dir")]
    pub data_dir: String,

    /// 媒体目录
    #[serde(default = "default_media_dir")]
    pub media_dir: String,

    /// 应用数据目录（通常为 data_dir/apps）
    #[serde(default = "default_app_data_dir")]
    pub app_data_dir: String,

    /// 监听端口
    #[serde(default = "default_port")]
    pub port: u16,

    /// 默认商店源 URL
    #[serde(default = "default_store_url")]
    pub default_store_url: String,

    /// Docker socket 路径
    #[serde(default = "default_docker_socket")]
    pub docker_socket: String,

    /// pnos 网络名
    #[serde(default = "default_network")]
    pub network: String,

    /// 日志级别
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_data_dir() -> String {
    "/data".to_string()
}
fn default_media_dir() -> String {
    "/media".to_string()
}
fn default_app_data_dir() -> String {
    "/data/apps".to_string()
}
fn default_port() -> u16 {
    80
}
fn default_store_url() -> String {
    "https://raw.githubusercontent.com/PandaNetOS/pnos-store/main/index.json".to_string()
}
fn default_docker_socket() -> String {
    "/var/run/docker.sock".to_string()
}
fn default_network() -> String {
    "pnos-net".to_string()
}
fn default_log_level() -> String {
    "info".to_string()
}

impl Default for PnosConfig {
    fn default() -> Self {
        PnosConfig {
            data_dir: default_data_dir(),
            media_dir: default_media_dir(),
            app_data_dir: default_app_data_dir(),
            port: default_port(),
            default_store_url: default_store_url(),
            docker_socket: default_docker_socket(),
            network: default_network(),
            log_level: default_log_level(),
        }
    }
}

impl PnosConfig {
    /// 从 YAML 文件加载，环境变量覆盖
    pub fn load() -> Result<Self, crate::error::PnosError> {
        let mut config = PnosConfig::default();

        // 尝试从 /etc/pnos/config.yml 加载
        let config_path = std::env::var("PNOS_CONFIG").unwrap_or_else(|_| "/etc/pnos/config.yml".to_string());
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(file_config) = serde_yaml::from_str::<PnosConfig>(&content) {
                config = file_config;
            }
        }

        // 环境变量覆盖
        if let Ok(v) = std::env::var("PNOS_DATA_DIR") {
            config.data_dir = v;
        }
        if let Ok(v) = std::env::var("PNOS_MEDIA_DIR") {
            config.media_dir = v;
        }
        if let Ok(v) = std::env::var("PNOS_PORT") {
            if let Ok(p) = v.parse() {
                config.port = p;
            }
        }
        if let Ok(v) = std::env::var("PNOS_LOG_LEVEL") {
            config.log_level = v;
        }

        Ok(config)
    }

    /// 替换 app.yml 中的变量
    pub fn render_vars(&self, input: &str) -> String {
        input
            .replace("{{app_data}}", &self.app_data_dir)
            .replace("{{media_dir}}", &self.media_dir)
            .replace("{{data_dir}}", &self.data_dir)
            .replace("{{pk_url}}", "http://pk:8080")
    }
}
