//! 自描述能力清单（Capability Manifest）
//!
//! 从 pandanetos 迁移。每个构建版本生成自己的能力清单，运行时上报给主控端。
//! 顶层字段：manifest_version / basic / capabilities / configurable_params / api_interfaces / status_report / communication / build_info

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// 能力清单格式版本号
pub const MANIFEST_VERSION: &str = "1.0";

/// 组件角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentRole {
    /// 主控面
    ControlPlane,
    /// 数据面
    DataPlane,
    /// 边车
    Sidecar,
    /// 监控
    Monitor,
}

impl ComponentRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ControlPlane => "control_plane",
            Self::DataPlane => "data_plane",
            Self::Sidecar => "sidecar",
            Self::Monitor => "monitor",
        }
    }
}

/// 基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicInfo {
    /// 程序名称（如 spde、pk、pcdn-keeper）
    pub name: String,
    /// 语义化版本号
    pub version: String,
    /// 程序功能描述
    pub description: String,
    /// 角色
    pub role: String,
    /// 当前运行模式（agent/standalone/cli，可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_mode: Option<String>,
}

impl BasicInfo {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        role: ComponentRole,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            role: role.as_str().to_string(),
            current_mode: None,
        }
    }

    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.current_mode = Some(mode.into());
        self
    }
}

/// 功能特性分组
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Capabilities {
    /// 支持的协议列表
    pub protocols: Vec<String>,
    /// 功能特性
    pub features: BTreeMap<String, bool>,
    /// 任务控制能力
    pub task_control: BTreeMap<String, bool>,
    /// 硬件能力
    pub hardware: BTreeMap<String, serde_json::Value>,
    /// 编译特性
    pub compile_features: Vec<String>,
}

impl Capabilities {
    pub fn supports_protocol(&self, protocol: &str) -> bool {
        self.protocols.iter().any(|p| p == protocol)
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }
}

/// 可配置参数描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurableParam {
    /// 数据类型（u32/u64/f64/bool/string/enum）
    pub r#type: String,
    /// 默认值
    pub default: serde_json::Value,
    /// 最小值（数值类型，可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /// 最大值（数值类型，可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// 枚举可选值（enum 类型，可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,
    /// 单位（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 参数说明
    pub description: String,
}

impl ConfigurableParam {
    pub fn number(
        type_name: &str,
        default: f64,
        min: Option<f64>,
        max: Option<f64>,
        unit: Option<&str>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            r#type: type_name.to_string(),
            default: serde_json::json!(default),
            min,
            max,
            r#enum: None,
            unit: unit.map(String::from),
            description: description.into(),
        }
    }

    pub fn boolean(default: bool, description: impl Into<String>) -> Self {
        Self {
            r#type: "bool".to_string(),
            default: serde_json::json!(default),
            min: None,
            max: None,
            r#enum: None,
            unit: None,
            description: description.into(),
        }
    }

    pub fn string(
        default: impl Into<String>,
        choices: Option<Vec<&str>>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            r#type: "string".to_string(),
            default: serde_json::json!(default.into()),
            min: None,
            max: None,
            r#enum: choices
                .map(|choices| choices.into_iter().map(|c| serde_json::json!(c)).collect()),
            unit: None,
            description: description.into(),
        }
    }
}

/// API 接口描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInterface {
    /// HTTP 方法（GET/POST/PUT/DELETE）
    pub method: String,
    /// API 路径
    pub path: String,
    /// 接口说明
    pub description: String,
    /// 请求体结构（可选）
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub request: BTreeMap<String, String>,
    /// 响应体结构（可选）
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub response: BTreeMap<String, String>,
    /// 是否需要认证
    #[serde(default)]
    pub auth_required: bool,
}

impl ApiInterface {
    pub fn new(
        method: impl Into<String>,
        path: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            description: description.into(),
            request: BTreeMap::new(),
            response: BTreeMap::new(),
            auth_required: false,
        }
    }

    pub fn with_request_field(mut self, name: &str, type_name: &str) -> Self {
        self.request.insert(name.to_string(), type_name.to_string());
        self
    }

    pub fn with_response_field(mut self, name: &str, type_name: &str) -> Self {
        self.response
            .insert(name.to_string(), type_name.to_string());
        self
    }

    pub fn with_auth(mut self) -> Self {
        self.auth_required = true;
        self
    }
}

/// 状态上报字段
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusReport {
    /// 组件级上报字段
    pub component_level: Vec<String>,
    /// 任务级上报字段
    pub task_level: Vec<String>,
}

/// 通信能力
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Communication {
    /// 是否支持 WebSocket
    pub websocket: bool,
    /// 是否支持 HTTP API
    pub http_api: bool,
    /// 是否支持心跳
    pub heartbeat: bool,
    /// 心跳间隔（秒）
    pub heartbeat_interval_secs: u64,
    /// WebSocket 重连间隔（秒）
    pub websocket_reconnect_secs: u64,
}

impl Default for Communication {
    fn default() -> Self {
        Self {
            websocket: false,
            http_api: false,
            heartbeat: false,
            heartbeat_interval_secs: 10,
            websocket_reconnect_secs: 3,
        }
    }
}

/// 构建信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildInfo {
    pub rust_version: String,
    pub build_profile: String,
    pub build_time: String,
    pub git_commit: String,
    pub git_branch: String,
    pub target_triple: String,
}

impl Default for BuildInfo {
    fn default() -> Self {
        Self {
            rust_version: "unknown".to_string(),
            build_profile: "unknown".to_string(),
            build_time: "unknown".to_string(),
            git_commit: "unknown".to_string(),
            git_branch: "unknown".to_string(),
            target_triple: "unknown".to_string(),
        }
    }
}

/// 能力清单（完整结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CapabilityManifest {
    /// 清单格式版本号
    pub manifest_version: String,
    /// 基本信息
    pub basic: BasicInfo,
    /// 能力清单
    #[serde(default)]
    pub capabilities: Capabilities,
    /// 可配置参数
    #[serde(default)]
    pub configurable_params: BTreeMap<String, ConfigurableParam>,
    /// API 接口定义
    #[serde(default)]
    pub api_interfaces: BTreeMap<String, ApiInterface>,
    /// 状态上报字段
    #[serde(default)]
    pub status_report: StatusReport,
    /// 通信能力
    #[serde(default)]
    pub communication: Communication,
    /// 构建信息
    #[serde(default)]
    pub build_info: BuildInfo,
}

impl CapabilityManifest {
    pub fn new(basic: BasicInfo) -> Self {
        Self {
            manifest_version: MANIFEST_VERSION.to_string(),
            basic,
            capabilities: Capabilities::default(),
            configurable_params: BTreeMap::new(),
            api_interfaces: BTreeMap::new(),
            status_report: StatusReport::default(),
            communication: Communication::default(),
            build_info: BuildInfo::default(),
        }
    }

    pub fn with_capabilities(mut self, capabilities: Capabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_configurable_param(mut self, name: &str, param: ConfigurableParam) -> Self {
        self.configurable_params.insert(name.to_string(), param);
        self
    }

    pub fn with_api_interface(mut self, name: &str, interface: ApiInterface) -> Self {
        self.api_interfaces.insert(name.to_string(), interface);
        self
    }

    pub fn with_status_report(mut self, report: StatusReport) -> Self {
        self.status_report = report;
        self
    }

    pub fn with_communication(mut self, communication: Communication) -> Self {
        self.communication = communication;
        self
    }

    pub fn is_data_plane(&self) -> bool {
        self.basic.role == ComponentRole::DataPlane.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> CapabilityManifest {
        CapabilityManifest::new(BasicInfo::new(
            "spde",
            "0.6.2",
            "多协议下载节点",
            ComponentRole::DataPlane,
        ))
        .with_capabilities(Capabilities {
            protocols: vec!["http".into(), "https".into()],
            features: BTreeMap::from([("resume".into(), true)]),
            task_control: BTreeMap::from([("pause".into(), true)]),
            hardware: BTreeMap::new(),
            compile_features: vec![],
        })
        .with_configurable_param(
            "max_concurrent",
            ConfigurableParam::number(
                "u32",
                4.0,
                Some(1.0),
                Some(256.0),
                Some("tasks"),
                "最大并发任务数",
            ),
        )
    }

    #[test]
    fn manifest_serializes_to_documented_json() {
        let m = sample_manifest();
        let json = serde_json::to_value(&m).unwrap();
        assert_eq!(json["manifest_version"], "1.0");
        assert_eq!(json["basic"]["name"], "spde");
        assert_eq!(json["basic"]["role"], "data_plane");
        assert_eq!(json["capabilities"]["protocols"][0], "http");
        assert_eq!(json["configurable_params"]["max_concurrent"]["max"], 256.0);
    }

    #[test]
    fn supports_protocol_and_feature() {
        let m = sample_manifest();
        assert!(m.capabilities.supports_protocol("http"));
        assert!(!m.capabilities.supports_protocol("torrent"));
        assert!(m.capabilities.has_feature("resume"));
        assert!(!m.capabilities.has_feature("dry_run"));
    }

    #[test]
    fn role_detection() {
        let m = sample_manifest();
        assert!(m.is_data_plane());
    }

    #[test]
    fn configurable_param_variants() {
        let bool_param = ConfigurableParam::boolean(true, "是否启用重试");
        assert_eq!(bool_param.r#type, "bool");

        let str_param = ConfigurableParam::string("auto", Some(vec!["auto", "manual"]), "调度模式");
        assert_eq!(str_param.r#type, "string");
        assert!(str_param.r#enum.is_some());
    }

    #[test]
    fn communication_defaults() {
        let c = Communication::default();
        assert!(!c.websocket);
        assert_eq!(c.heartbeat_interval_secs, 10);
        assert_eq!(c.websocket_reconnect_secs, 3);
    }
}
