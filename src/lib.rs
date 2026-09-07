//! pnos 统一通信标准库
//!
//! PandaNetOS 生态唯一的统一通信标准库，所有组件（应用/Agent/运行时/主控）共同依赖。
//! 定义统一的通信标准：组件注册发现、服务发现、事件协议、任务调度、能力自描述、
//! 统一错误码与响应格式、健康检查、系统信息等。

pub mod app;
pub mod capability;
pub mod component;
pub mod config;
pub mod discovery;
pub mod error;
pub mod events;
pub mod health;
pub mod logging;
pub mod protocol;
pub mod proxy;
pub mod registry;
pub mod response;
pub mod system;
pub mod task;
pub mod time;
pub mod utils;

/// 一行导入所有常用类型
pub mod prelude {
    // 应用模型（商店安装用）
    pub use crate::app::{
        AppManifest, AppStatus, BinaryConfig, EnvVar, HealthCheck, HealthCheckType, RunConfig,
        WebConfig, WebUiType,
    };
    // 能力自描述
    pub use crate::capability::{
        ApiInterface, BasicInfo, BuildInfo, Capabilities, CapabilityManifest, Communication,
        ComponentRole, ConfigurableParam, StatusReport, MANIFEST_VERSION,
    };
    // 组件类型与状态
    pub use crate::component::{ComponentStatus, ComponentType};
    // 配置
    pub use crate::config::PnosConfig;
    // 服务发现
    pub use crate::discovery::{ComponentDiscoverResponse, ComponentListResponse, ComponentQuery};
    // 错误码
    pub use crate::error::{ErrorCode, PnosError};
    // 事件协议
    pub use crate::events::{
        AppInstallProgress, AppStatusChanged, ComponentStatusChanged, ServiceChangedPayload,
        SystemNotification, TaskCompletedPayload, TaskProgressPayload, WsMessage, WsSubscribe,
    };
    // 健康检查
    pub use crate::health::{HealthResponse, HealthStatus};
    // 协议路径
    pub use crate::protocol;
    // 代理规则
    pub use crate::proxy::ProxyRule;
    // 组件注册与心跳
    pub use crate::registry::{
        ComponentInfo, ComponentRegisterRequest, ComponentRegisterResponse, HeartbeatRequest,
        UnregisterRequest,
    };
    // 统一响应
    pub use crate::response::{ApiResponse, PageQuery, PageResult};
    // 系统信息
    pub use crate::system::{DiskInfo, NetworkStats, SystemInfo, SystemStats};
    // 任务调度
    pub use crate::task::{
        ClaimTaskRequest, ClaimTaskResponse, CreateTaskRequest, Dispatch, DispatchConfig,
        PendingTask, Task, TaskProgress, TaskQuery, TaskReport, TaskStatus,
    };
    // 工具
    pub use crate::time::now_rfc3339;
    pub use crate::utils::{format_bytes, parse_bytes};
}
