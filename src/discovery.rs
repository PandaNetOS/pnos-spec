//! 统一服务发现
//!
//! 合并应用发现与 Agent 服务发现，支持按组件类型/能力/健康状态/区域筛选。

use serde::{Deserialize, Serialize};

use crate::component::{ComponentStatus, ComponentType};

/// 服务发现响应（精简版组件信息）
///
/// 用于组件间互相发现，只包含通信必需的字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDiscoverResponse {
    /// 组件 ID
    pub id: String,
    /// 组件名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 组件类型
    pub component_type: ComponentType,
    /// 组件地址
    pub address: String,
    /// 监听端口
    pub port: u16,
    /// 能力标签列表
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// 区域标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 当前状态
    pub status: ComponentStatus,
    /// 基础 URL
    pub base_url: String,
    /// 对外服务 URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serve_url: Option<String>,
}

impl ComponentDiscoverResponse {
    /// 获取实际可访问的 URL
    pub fn accessible_url(&self) -> &str {
        self.serve_url.as_deref().unwrap_or(&self.base_url)
    }

    /// 是否具备指定能力
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

/// 组件查询参数
///
/// 支持按组件类型/能力/健康状态/区域筛选，分页返回。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentQuery {
    /// 按组件类型筛选（app/agent/runtime/pk）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// 按能力标签筛选（如 download.http）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<String>,
    /// 按健康状态筛选（healthy/unhealthy/unknown）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// 按区域筛选
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 按状态筛选（running/busy/offline/error）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 页码（从 1 开始，默认 1）
    #[serde(default = "default_page")]
    pub page: u64,
    /// 每页大小（默认 50，最大 200）
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    50
}

impl Default for ComponentQuery {
    fn default() -> Self {
        Self {
            component_type: None,
            capability: None,
            health: None,
            region: None,
            status: None,
            page: 1,
            page_size: 50,
        }
    }
}

impl ComponentQuery {
    /// 构造空查询
    pub fn new() -> Self {
        Self::default()
    }

    /// 按组件类型筛选
    pub fn with_type(mut self, component_type: ComponentType) -> Self {
        self.component_type = Some(component_type.as_str().to_string());
        self
    }

    /// 按能力筛选
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capability = Some(capability.into());
        self
    }

    /// 按区域筛选
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// 只看运行中的组件
    pub fn running_only(mut self) -> Self {
        self.status = Some(ComponentStatus::Running.as_str().to_string());
        self
    }

    /// 计算分页偏移
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.page_size
    }

    /// 规范化每页大小（0→50，>200→200）
    pub fn effective_page_size(&self) -> u64 {
        match self.page_size {
            0 => 50,
            n if n > 200 => 200,
            n => n,
        }
    }
}

/// 批量发现响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentListResponse {
    /// 组件列表
    pub components: Vec<ComponentDiscoverResponse>,
    /// 总数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_query_defaults() {
        let q = ComponentQuery::new();
        assert_eq!(q.page, 1);
        assert_eq!(q.page_size, 50);
        assert_eq!(q.offset(), 0);
    }

    #[test]
    fn component_query_filters() {
        let q = ComponentQuery::new()
            .with_type(ComponentType::Agent)
            .with_capability("download.http")
            .with_region("cn-hangzhou")
            .running_only();
        assert_eq!(q.component_type.as_deref(), Some("agent"));
        assert_eq!(q.capability.as_deref(), Some("download.http"));
        assert_eq!(q.region.as_deref(), Some("cn-hangzhou"));
        assert_eq!(q.status.as_deref(), Some("running"));
    }

    #[test]
    fn component_query_page_size_normalization() {
        let q = ComponentQuery {
            page_size: 0,
            ..Default::default()
        };
        assert_eq!(q.effective_page_size(), 50);

        let q = ComponentQuery {
            page_size: 500,
            ..Default::default()
        };
        assert_eq!(q.effective_page_size(), 200);
    }

    #[test]
    fn discover_response_accessible_url() {
        let resp = ComponentDiscoverResponse {
            id: "test".into(),
            name: "test".into(),
            version: "1.0".into(),
            component_type: ComponentType::App,
            address: "127.0.0.1".into(),
            port: 8080,
            capabilities: vec![],
            region: None,
            status: ComponentStatus::Running,
            base_url: "http://127.0.0.1:8080".into(),
            serve_url: None,
        };
        assert_eq!(resp.accessible_url(), "http://127.0.0.1:8080");
        assert!(!resp.has_capability("x"));
    }
}
