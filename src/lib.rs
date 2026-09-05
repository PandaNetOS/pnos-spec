//! pnos 系统级标准库
//!
//! 所有 pnos 应用共同依赖的共享库，提供统一的错误类型、响应格式、
//! 应用描述模型、容器配置、配置加载、结构化日志等基础能力。
//!
//! # 快速开始
//!
//! ```rust
//! use pnos::prelude::*;
//! ```

pub mod app;
pub mod config;
pub mod container;
pub mod error;
pub mod logging;
pub mod protocol;
pub mod response;
pub mod time;
pub mod utils;

/// 一行导入所有常用类型
pub mod prelude {
    pub use crate::app::{AppManifest, AppStatus, HealthCheck, HealthCheckType};
    pub use crate::container::{ContainerConfig, PortMapping, VolumeMount, EnvVar};
    pub use crate::error::{PnosError, ErrorCode};
    pub use crate::response::{ApiResponse, ApiError, PageResult, PageQuery};
    pub use crate::config::PnosConfig;
    pub use crate::time::now_rfc3339;
    pub use crate::utils::{format_bytes, parse_bytes};
}
