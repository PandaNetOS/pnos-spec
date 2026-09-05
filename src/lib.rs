//! pnos 系统级标准库
//!
//! 所有 pnos 应用共同依赖的共享库，定义统一的通信标准：
//! 响应格式、错误码、应用模型、注册发现、健康检查、事件协议等。

pub mod app;
pub mod config;
pub mod error;
pub mod events;
pub mod health;
pub mod logging;
pub mod protocol;
pub mod proxy;
pub mod registry;
pub mod response;
pub mod system;
pub mod time;
pub mod utils;

/// 一行导入所有常用类型
pub mod prelude {
    pub use crate::app::{
        AppManifest, AppStatus, BinaryConfig, EnvVar, HealthCheck, HealthCheckType, RunConfig,
        WebConfig, WebUiType,
    };
    pub use crate::config::PnosConfig;
    pub use crate::error::{ErrorCode, PnosError};
    pub use crate::events::{WsMessage, WsSubscribe};
    pub use crate::health::{HealthResponse, HealthStatus};
    pub use crate::protocol;
    pub use crate::proxy::ProxyRule;
    pub use crate::registry::{
        AppDiscoverResponse, AppInfo, AppRegisterRequest, AppRegisterResponse, HeartbeatRequest,
    };
    pub use crate::response::{ApiResponse, PageQuery, PageResult};
    pub use crate::system::{DiskInfo, NetworkStats, SystemInfo, SystemStats};
    pub use crate::time::now_rfc3339;
    pub use crate::utils::{format_bytes, parse_bytes};
}
