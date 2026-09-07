//! 统一组件注册与心跳
//!
//! 合并应用注册（AppRegisterRequest）与 Agent 节点注册（RegisterReq），
//! 所有接入 pnos 生态的通信实体统一通过 Component 协议注册。

use serde::{Deserialize, Serialize};

use crate::component::{ComponentStatus, ComponentType};
use crate::health::HealthStatus;

/// 统一组件注册请求
///
/// 合并应用注册与 Agent 节点注册的所有字段。
/// 应用侧关注 port/serve_host/health_check_path/web_path/dependencies；
/// Agent 侧关注 hostname/platform/arch/labels/max_concurrent/max_bandwidth_bps/capabilities。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRegisterRequest {
    /// 组件唯一 ID（应用用 app_id，Agent 用 node_id）
    pub id: String,
    /// 组件名称（如 spde、pcdn-keeper、PeerDiscoveryCenter）
    pub name: String,
    /// 语义化版本号
    pub version: String,
    /// 组件类型
    pub component_type: ComponentType,
    /// 组件监听端口
    pub port: u16,
    /// 对外服务主机（可选，默认用注册时的来源 IP）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serve_host: Option<String>,
    /// 对外服务端口（可选，默认等于 port）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serve_port: Option<u16>,
    /// 能力标签列表（Agent 用，如 ["download.http", "discovery.dht"]）
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// 区域标识（可选，用于就近调度）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 主机名（Agent 用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// 操作系统平台（Agent 用，如 linux/windows/macos）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// CPU 架构（Agent 用，如 x86_64/aarch64）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// 自定义标签（Agent 用）
    #[serde(default)]
    pub labels: Vec<String>,
    /// 最大并发任务数（Agent 用，None 用全局默认）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<u32>,
    /// 最大带宽上限 bps（Agent 用，None 用全局默认）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_bandwidth_bps: Option<u64>,
    /// 健康检查路径（默认 /health）
    #[serde(default = "default_health_path")]
    pub health_check_path: String,
    /// Web UI 路径（应用用，如 /web）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_path: Option<String>,
    /// 依赖的其他组件 ID 列表
    #[serde(default)]
    pub dependencies: Vec<String>,
}

fn default_health_path() -> String {
    "/health".to_string()
}

impl ComponentRegisterRequest {
    /// 构造最小注册请求（应用场景）
    pub fn new_app(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        port: u16,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            component_type: ComponentType::App,
            port,
            serve_host: None,
            serve_port: None,
            capabilities: Vec::new(),
            region: None,
            hostname: None,
            platform: None,
            arch: None,
            labels: Vec::new(),
            max_concurrent: None,
            max_bandwidth_bps: None,
            health_check_path: default_health_path(),
            web_path: None,
            dependencies: Vec::new(),
        }
    }

    /// 构造最小注册请求（Agent 场景）
    pub fn new_agent(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        port: u16,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            component_type: ComponentType::Agent,
            port,
            serve_host: None,
            serve_port: None,
            capabilities: Vec::new(),
            region: None,
            hostname: None,
            platform: None,
            arch: None,
            labels: Vec::new(),
            max_concurrent: None,
            max_bandwidth_bps: None,
            health_check_path: default_health_path(),
            web_path: None,
            dependencies: Vec::new(),
        }
    }

    /// 设置对外服务地址
    pub fn with_serve(mut self, host: impl Into<String>, port: u16) -> Self {
        self.serve_host = Some(host.into());
        self.serve_port = Some(port);
        self
    }

    /// 添加能力标签
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// 设置 Agent 主机信息
    pub fn with_host_info(
        mut self,
        hostname: impl Into<String>,
        platform: impl Into<String>,
        arch: impl Into<String>,
    ) -> Self {
        self.hostname = Some(hostname.into());
        self.platform = Some(platform.into());
        self.arch = Some(arch.into());
        self
    }

    /// 设置最大并发与带宽
    pub fn with_limits(mut self, max_concurrent: u32, max_bandwidth_bps: u64) -> Self {
        self.max_concurrent = Some(max_concurrent);
        self.max_bandwidth_bps = Some(max_bandwidth_bps);
        self
    }

    /// 设置 Web UI 路径
    pub fn with_web_path(mut self, path: impl Into<String>) -> Self {
        self.web_path = Some(path.into());
        self
    }

    /// 是否具备指定能力
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

/// 统一组件注册响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRegisterResponse {
    /// 访问令牌（后续 HTTP 请求带 X-Pnos-Token，WS 连接带 ?token=）
    pub token: String,
    /// 组件 ID（回显）
    pub component_id: String,
    /// 注册时间（ISO 8601）
    pub registered_at: String,
}

/// 统一心跳请求
///
/// 合并应用心跳与 Agent 节点心跳。
/// 应用侧关注 status/load/message；
/// Agent 侧关注 active_tasks/bytes_downloaded/speed_bps。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {
    /// 组件 ID
    pub id: String,
    /// 当前状态
    pub status: ComponentStatus,
    /// 活跃任务数（Agent 用，应用填 0）
    #[serde(default)]
    pub active_tasks: u32,
    /// 累计下载字节数（Agent 用）
    #[serde(default)]
    pub bytes_downloaded: u64,
    /// 当前下载速度 bps（Agent 用）
    #[serde(default)]
    pub speed_bps: u64,
    /// 系统负载（0.0 - 1.0）
    #[serde(default)]
    pub load: f32,
    /// 附加消息（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl HeartbeatRequest {
    /// 构造应用心跳（仅状态+负载）
    pub fn new_app(id: impl Into<String>, status: ComponentStatus, load: f32) -> Self {
        Self {
            id: id.into(),
            status,
            active_tasks: 0,
            bytes_downloaded: 0,
            speed_bps: 0,
            load,
            message: None,
        }
    }

    /// 构造 Agent 心跳（含任务统计）
    pub fn new_agent(
        id: impl Into<String>,
        status: ComponentStatus,
        active_tasks: u32,
        speed_bps: u64,
    ) -> Self {
        Self {
            id: id.into(),
            status,
            active_tasks,
            bytes_downloaded: 0,
            speed_bps,
            load: 0.0,
            message: None,
        }
    }
}

/// 统一组件信息（注册中心返回的完整组件描述）
///
/// 合并应用信息（AppInfo）与 Agent 节点信息（Node）+ 服务发现信息（ServiceAgentInfo）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    /// 组件 ID
    pub id: String,
    /// 组件名称
    pub name: String,
    /// 版本号
    pub version: String,
    /// 组件类型
    pub component_type: ComponentType,
    /// 组件地址（IP 或主机名）
    pub address: String,
    /// 监听端口
    pub port: u16,
    /// 对外服务主机
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serve_host: Option<String>,
    /// 对外服务端口
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serve_port: Option<u16>,
    /// 能力标签列表
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// 区域标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 主机名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// 操作系统平台
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// CPU 架构
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// 自定义标签
    #[serde(default)]
    pub labels: Vec<String>,
    /// 最大并发任务数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<u32>,
    /// 最大带宽上限 bps
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_bandwidth_bps: Option<u64>,
    /// 当前状态
    pub status: ComponentStatus,
    /// 系统负载
    #[serde(default)]
    pub load: f32,
    /// 活跃任务数
    #[serde(default)]
    pub active_tasks: u32,
    /// 累计下载字节数
    #[serde(default)]
    pub bytes_downloaded: u64,
    /// 健康状态
    pub health: HealthStatus,
    /// 最后心跳时间（ISO 8601）
    pub last_heartbeat: String,
    /// 注册时间（ISO 8601）
    pub registered_at: String,
    /// 基础 URL（http://address:port）
    pub base_url: String,
    /// 对外服务 URL（如有 serve_host/serve_port）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serve_url: Option<String>,
    /// Web UI 路径
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_path: Option<String>,
}

impl ComponentInfo {
    /// 获取实际可访问的 URL（优先 serve_url，其次 base_url）
    pub fn accessible_url(&self) -> &str {
        self.serve_url.as_deref().unwrap_or(&self.base_url)
    }

    /// 是否具备指定能力
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }

    /// 是否可接收任务（状态 running + 未达并发上限）
    pub fn can_accept_task(&self, global_default_max_concurrent: u32) -> bool {
        if self.status != ComponentStatus::Running {
            return false;
        }
        let max = self.max_concurrent.unwrap_or(global_default_max_concurrent);
        self.active_tasks < max
    }
}

/// 注销请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisterRequest {
    /// 组件 ID
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_register_request_minimal() {
        let req = ComponentRegisterRequest::new_app("my-app", "My App", "1.0.0", 8080);
        assert_eq!(req.component_type, ComponentType::App);
        assert_eq!(req.health_check_path, "/health");
        assert!(req.capabilities.is_empty());
    }

    #[test]
    fn agent_register_request_with_limits() {
        let req = ComponentRegisterRequest::new_agent("spde-001", "spde", "0.6.2", 9000)
            .with_host_info("node-1", "linux", "x86_64")
            .with_limits(4, 100_000_000)
            .with_capability("download.http");
        assert_eq!(req.component_type, ComponentType::Agent);
        assert_eq!(req.max_concurrent, Some(4));
        assert!(req.has_capability("download.http"));
    }

    #[test]
    fn heartbeat_app_vs_agent() {
        let app_hb = HeartbeatRequest::new_app("my-app", ComponentStatus::Running, 0.3);
        assert_eq!(app_hb.active_tasks, 0);

        let agent_hb =
            HeartbeatRequest::new_agent("spde-001", ComponentStatus::Busy, 4, 50_000_000);
        assert_eq!(agent_hb.active_tasks, 4);
        assert_eq!(agent_hb.speed_bps, 50_000_000);
    }

    #[test]
    fn component_info_accessible_url() {
        let info = ComponentInfo {
            id: "test".into(),
            name: "test".into(),
            version: "1.0".into(),
            component_type: ComponentType::App,
            address: "127.0.0.1".into(),
            port: 8080,
            serve_host: None,
            serve_port: None,
            capabilities: vec![],
            region: None,
            hostname: None,
            platform: None,
            arch: None,
            labels: vec![],
            max_concurrent: None,
            max_bandwidth_bps: None,
            status: ComponentStatus::Running,
            load: 0.0,
            active_tasks: 0,
            bytes_downloaded: 0,
            health: HealthStatus::Ok,
            last_heartbeat: "2026-01-01T00:00:00Z".into(),
            registered_at: "2026-01-01T00:00:00Z".into(),
            base_url: "http://127.0.0.1:8080".into(),
            serve_url: Some("http://public:80".into()),
            web_path: None,
        };
        assert_eq!(info.accessible_url(), "http://public:80");
    }

    #[test]
    fn component_info_can_accept_task() {
        let mut info = ComponentInfo {
            id: "test".into(),
            name: "test".into(),
            version: "1.0".into(),
            component_type: ComponentType::Agent,
            address: "127.0.0.1".into(),
            port: 9000,
            serve_host: None,
            serve_port: None,
            capabilities: vec![],
            region: None,
            hostname: None,
            platform: None,
            arch: None,
            labels: vec![],
            max_concurrent: Some(4),
            max_bandwidth_bps: None,
            status: ComponentStatus::Running,
            load: 0.0,
            active_tasks: 3,
            bytes_downloaded: 0,
            health: HealthStatus::Ok,
            last_heartbeat: "2026-01-01T00:00:00Z".into(),
            registered_at: "2026-01-01T00:00:00Z".into(),
            base_url: "http://127.0.0.1:9000".into(),
            serve_url: None,
            web_path: None,
        };
        // 活跃 3 < 上限 4 → 可接收
        assert!(info.can_accept_task(8));
        // 活跃 4 = 上限 4 → 不可接收
        info.active_tasks = 4;
        assert!(!info.can_accept_task(8));
        // 状态 busy → 不可接收
        info.status = ComponentStatus::Busy;
        assert!(!info.can_accept_task(8));
    }
}
