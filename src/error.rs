//! 统一错误类型与错误码

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 错误码枚举（7 大领域）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ErrorCode {
    // 通用 0-999
    Success = 0,
    Unknown = 1,
    InvalidParameter = 2,
    NotFound = 3,
    AlreadyExists = 4,
    Unauthorized = 5,
    Forbidden = 6,
    InternalError = 7,
    ServiceUnavailable = 8,

    // 应用管理 1000-1999
    AppNotFound = 1000,
    AppAlreadyInstalled = 1001,
    AppNotInstalled = 1002,
    AppAlreadyRunning = 1003,
    AppNotRunning = 1004,
    AppStartFailed = 1005,
    AppStopFailed = 1006,
    AppManifestInvalid = 1007,
    AppDependencyMissing = 1008,
    PortConflict = 1009,

    // 应用注册与发现 2000-2099
    AppNotRegistered = 2000,
    AppAlreadyRegistered = 2001,
    HeartbeatTimeout = 2002,
    TokenInvalid = 2003,
    TokenExpired = 2004,

    // 二进制部署 2100-2199
    BinaryDownloadFailed = 2100,
    BinaryChecksumMismatch = 2101,
    BinaryExtractFailed = 2102,
    BinaryNotFound = 2103,

    // 商店 2200-2299
    StoreSourceUnreachable = 2200,
    StoreAppNotFound = 2201,

    // 文件 3000-3999
    FileNotFound = 3000,
    FileAlreadyExists = 3001,
    PermissionDenied = 3002,
    DirectoryNotEmpty = 3003,

    // 系统 4000-4999
    SystemInfoUnavailable = 4000,

    // 网络 5000-5999
    NetworkError = 5000,
    ProxyError = 5001,
}

impl ErrorCode {
    pub fn code(&self) -> u32 {
        *self as u32
    }

    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::Success => "成功",
            ErrorCode::Unknown => "未知错误",
            ErrorCode::InvalidParameter => "参数错误",
            ErrorCode::NotFound => "资源不存在",
            ErrorCode::AlreadyExists => "资源已存在",
            ErrorCode::Unauthorized => "未授权",
            ErrorCode::Forbidden => "禁止访问",
            ErrorCode::InternalError => "内部错误",
            ErrorCode::ServiceUnavailable => "服务不可用",
            ErrorCode::AppNotFound => "应用不存在",
            ErrorCode::AppAlreadyInstalled => "应用已安装",
            ErrorCode::AppNotInstalled => "应用未安装",
            ErrorCode::AppAlreadyRunning => "应用已在运行",
            ErrorCode::AppNotRunning => "应用未运行",
            ErrorCode::AppStartFailed => "应用启动失败",
            ErrorCode::AppStopFailed => "应用停止失败",
            ErrorCode::AppManifestInvalid => "应用描述文件无效",
            ErrorCode::AppDependencyMissing => "应用依赖缺失",
            ErrorCode::PortConflict => "端口冲突",
            ErrorCode::AppNotRegistered => "应用未注册",
            ErrorCode::AppAlreadyRegistered => "应用已注册",
            ErrorCode::HeartbeatTimeout => "心跳超时",
            ErrorCode::TokenInvalid => "Token 无效",
            ErrorCode::TokenExpired => "Token 已过期",
            ErrorCode::BinaryDownloadFailed => "二进制下载失败",
            ErrorCode::BinaryChecksumMismatch => "校验和不匹配",
            ErrorCode::BinaryExtractFailed => "解压失败",
            ErrorCode::BinaryNotFound => "二进制文件不存在",
            ErrorCode::StoreSourceUnreachable => "商店源不可达",
            ErrorCode::StoreAppNotFound => "商店应用不存在",
            ErrorCode::FileNotFound => "文件不存在",
            ErrorCode::FileAlreadyExists => "文件已存在",
            ErrorCode::PermissionDenied => "权限不足",
            ErrorCode::DirectoryNotEmpty => "目录非空",
            ErrorCode::SystemInfoUnavailable => "系统信息不可用",
            ErrorCode::NetworkError => "网络错误",
            ErrorCode::ProxyError => "代理错误",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            ErrorCode::Success => 200,
            ErrorCode::InvalidParameter | ErrorCode::AppManifestInvalid => 400,
            ErrorCode::Unauthorized | ErrorCode::TokenInvalid | ErrorCode::TokenExpired => 401,
            ErrorCode::Forbidden | ErrorCode::PermissionDenied => 403,
            ErrorCode::NotFound
            | ErrorCode::AppNotFound
            | ErrorCode::StoreAppNotFound
            | ErrorCode::FileNotFound
            | ErrorCode::AppNotRegistered
            | ErrorCode::BinaryNotFound => 404,
            ErrorCode::AlreadyExists
            | ErrorCode::AppAlreadyInstalled
            | ErrorCode::AppAlreadyRegistered
            | ErrorCode::FileAlreadyExists
            | ErrorCode::AppAlreadyRunning
            | ErrorCode::PortConflict => 409,
            ErrorCode::AppNotRunning | ErrorCode::AppNotInstalled => 409,
            ErrorCode::ServiceUnavailable
            | ErrorCode::StoreSourceUnreachable
            | ErrorCode::HeartbeatTimeout => 503,
            _ => 500,
        }
    }
}

/// 统一错误类型
#[derive(Debug, Error)]
pub enum PnosError {
    #[error("{code:?}: {message}")]
    Business {
        code: ErrorCode,
        message: String,
    },
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON 序列化错误: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML 序列化错误: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("配置错误: {0}")]
    Config(String),
    #[error("外部错误: {0}")]
    External(String),
}

impl PnosError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        PnosError::Business {
            code,
            message: message.into(),
        }
    }

    pub fn code(&self) -> ErrorCode {
        match self {
            PnosError::Business { code, .. } => *code,
            _ => ErrorCode::InternalError,
        }
    }

    pub fn message(&self) -> String {
        match self {
            PnosError::Business { message, .. } => message.clone(),
            PnosError::Io(e) => e.to_string(),
            PnosError::Json(e) => e.to_string(),
            PnosError::Yaml(e) => e.to_string(),
            PnosError::Config(e) => e.clone(),
            PnosError::External(e) => e.clone(),
        }
    }
}

impl From<ErrorCode> for PnosError {
    fn from(code: ErrorCode) -> Self {
        PnosError::Business {
            code,
            message: code.message().to_string(),
        }
    }
}
