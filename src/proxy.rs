//! 反向代理规则
//!
//! pnos-runtime 作为反向代理，将 /app/{id}/* 转发到对应应用的端口。

use serde::{Deserialize, Serialize};

/// 反向代理规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRule {
    /// 应用 ID
    pub app_id: String,
    /// 匹配路径前缀（如 /app/pk）
    pub path_prefix: String,
    /// 目标地址（如 http://127.0.0.1:18080）
    pub target: String,
    /// 是否剥离前缀（/app/pk/api → /api）
    #[serde(default = "default_strip_prefix")]
    pub strip_prefix: bool,
    /// 是否启用 WebSocket 代理
    #[serde(default)]
    pub websocket: bool,
}

fn default_strip_prefix() -> bool {
    true
}

impl ProxyRule {
    /// 创建标准代理规则
    pub fn new(app_id: &str, port: u16) -> Self {
        Self {
            app_id: app_id.to_string(),
            path_prefix: format!("/app/{}", app_id),
            target: format!("http://127.0.0.1:{}", port),
            strip_prefix: true,
            websocket: true,
        }
    }

    /// 生成完整的目标 URL
    pub fn target_url(&self, path: &str) -> String {
        let path = if self.strip_prefix {
            path.strip_prefix(&self.path_prefix)
                .unwrap_or(path)
        } else {
            path
        };
        format!("{}{}", self.target.trim_end_matches('/'), path)
    }
}
