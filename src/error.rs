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

    // 容器 1000-1999
    ContainerNotFound = 1000,
    ContainerAlreadyRunning = 1001,
    ContainerNotRunning = 1002,
    ImagePullFailed = 1003,
    PortConflict = 1004,

    // 应用商店 2000-2999
    AppNotFound = 2000,
    AppAlreadyInstalled = 2001,
    AppNotInstalled = 2002,
    StoreSourceUnreachable = 2003,
    AppManifestInvalid = 2004,

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
            ErrorCode::ContainerNotFound => "容器不存在",
            ErrorCode::ContainerAlreadyRunning => "容器已在运行",
            ErrorCode::ContainerNotRunning => "容器未运行",
            ErrorCode::ImagePullFailed => "镜像拉取失败",
            ErrorCode::PortConflict => "端口冲突",
            ErrorCode::AppNotFound => "应用不存在",
            ErrorCode::AppAlreadyInstalled => "应用已安装",
            ErrorCode::AppNotInstalled => "应用未安装",
            ErrorCode::StoreSourceUnreachable => "商店源不可达",
            ErrorCode::AppManifestInvalid => "应用描述文件无效",
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
            ErrorCode::Unauthorized => 401,
            ErrorCode::Forbidden | ErrorCode::PermissionDenied => 403,
            ErrorCode::NotFound
            | ErrorCode::ContainerNotFound
            | ErrorCode::AppNotFound
            | ErrorCode::FileNotFound => 404,
            ErrorCode::AlreadyExists
            | ErrorCode::AppAlreadyInstalled
            | ErrorCode::FileAlreadyExists
            | ErrorCode::ContainerAlreadyRunning
            | ErrorCode::PortConflict => 409,
            ErrorCode::ContainerNotRunning | ErrorCode::AppNotInstalled => 409,
            ErrorCode::ServiceUnavailable | ErrorCode::StoreSourceUnreachable => 503,
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
