//! 统一错误类型与错误码
//!
//! 合并 pnos-spec（应用领域）与 pandanetos（Agent 领域）的错误码，
//! 统一使用 u32 数字编码，按领域分段。

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 错误码枚举（按领域分段）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ErrorCode {
    // ===== 通用 0-999 =====
    Success = 0,
    Unknown = 1,
    InvalidParameter = 2,
    NotFound = 3,
    AlreadyExists = 4,
    Unauthorized = 5,
    Forbidden = 6,
    InternalError = 7,
    ServiceUnavailable = 8,
    RateLimited = 9,
    Timeout = 10,
    Conflict = 11,

    // ===== 应用管理 1000-1999 =====
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

    // ===== 组件注册与发现 2000-2099（统一应用+Agent） =====
    ComponentNotRegistered = 2000,
    ComponentAlreadyRegistered = 2001,
    HeartbeatTimeout = 2002,
    TokenInvalid = 2003,
    TokenExpired = 2004,
    TokenRevoked = 2005,
    ComponentOffline = 2006,
    ComponentVersionTooOld = 2007,
    NoAvailableComponent = 2008,

    // ===== 二进制部署 2100-2199 =====
    BinaryDownloadFailed = 2100,
    BinaryChecksumMismatch = 2101,
    BinaryExtractFailed = 2102,
    BinaryNotFound = 2103,

    // ===== 商店 2200-2299 =====
    StoreSourceUnreachable = 2200,
    StoreAppNotFound = 2201,

    // ===== 文件 3000-3999 =====
    FileNotFound = 3000,
    FileAlreadyExists = 3001,
    PermissionDenied = 3002,
    DirectoryNotEmpty = 3003,
    DiskFull = 3004,

    // ===== 系统 4000-4999 =====
    SystemInfoUnavailable = 4000,

    // ===== 网络 5000-5999 =====
    NetworkError = 5000,
    ProxyError = 5001,
    ConnectionFailed = 5002,

    // ===== 任务调度 6000-6999 =====
    TaskNotFound = 6000,
    TaskAlreadyExists = 6001,
    TaskInvalidState = 6002,
    TaskDownloadFailed = 6003,
    TaskDisabled = 6004,
    TaskCancelled = 6005,

    // ===== 调度记录 6100-6199 =====
    DispatchNotFound = 6100,
    DispatchAlreadyClaimed = 6101,
    DispatchInvalidState = 6102,
    DispatchExpired = 6103,

    // ===== 节点管理 7000-7999 =====
    NodeNotFound = 7000,
    NodeOffline = 7001,
    NodeAlreadyRegistered = 7002,
    NodeHeartbeatTimeout = 7003,
    NodeVersionTooOld = 7004,
    NodeBusy = 7005,

    // ===== 下载 8000-8999 =====
    DownloadUnsupportedProtocol = 8000,
    DownloadConnectionFailed = 8001,
    DownloadTimeout = 8002,
    DownloadChecksumMismatch = 8003,
    DownloadDiskFull = 8004,
    DownloadPartialContent = 8005,
    DownloadRangeNotSatisfied = 8006,

    // ===== 工作流 9000-9999 =====
    WorkflowNotFound = 9000,
    WorkflowAlreadyExists = 9001,
    WorkflowDisabled = 9002,
    WorkflowInvalidCron = 9003,
    WorkflowTriggerFailed = 9004,

    // ===== 配置 10000-10099 =====
    ConfigInvalid = 10000,
    ConfigNotFound = 10001,
    ConfigReadOnly = 10002,
}

impl ErrorCode {
    pub fn code(&self) -> u32 {
        *self as u32
    }

    pub fn message(&self) -> &'static str {
        match self {
            // 通用
            Self::Success => "成功",
            Self::Unknown => "未知错误",
            Self::InvalidParameter => "参数错误",
            Self::NotFound => "资源不存在",
            Self::AlreadyExists => "资源已存在",
            Self::Unauthorized => "未授权",
            Self::Forbidden => "禁止访问",
            Self::InternalError => "内部错误",
            Self::ServiceUnavailable => "服务不可用",
            Self::RateLimited => "请求过于频繁",
            Self::Timeout => "操作超时",
            Self::Conflict => "状态冲突",
            // 应用管理
            Self::AppNotFound => "应用不存在",
            Self::AppAlreadyInstalled => "应用已安装",
            Self::AppNotInstalled => "应用未安装",
            Self::AppAlreadyRunning => "应用已在运行",
            Self::AppNotRunning => "应用未运行",
            Self::AppStartFailed => "应用启动失败",
            Self::AppStopFailed => "应用停止失败",
            Self::AppManifestInvalid => "应用描述文件无效",
            Self::AppDependencyMissing => "应用依赖缺失",
            Self::PortConflict => "端口冲突",
            // 组件注册与发现
            Self::ComponentNotRegistered => "组件未注册",
            Self::ComponentAlreadyRegistered => "组件已注册",
            Self::HeartbeatTimeout => "心跳超时",
            Self::TokenInvalid => "Token 无效",
            Self::TokenExpired => "Token 已过期",
            Self::TokenRevoked => "Token 已撤销",
            Self::ComponentOffline => "组件离线",
            Self::ComponentVersionTooOld => "组件版本过低，不兼容",
            Self::NoAvailableComponent => "没有可用组件",
            // 二进制部署
            Self::BinaryDownloadFailed => "二进制下载失败",
            Self::BinaryChecksumMismatch => "校验和不匹配",
            Self::BinaryExtractFailed => "解压失败",
            Self::BinaryNotFound => "二进制文件不存在",
            // 商店
            Self::StoreSourceUnreachable => "商店源不可达",
            Self::StoreAppNotFound => "商店应用不存在",
            // 文件
            Self::FileNotFound => "文件不存在",
            Self::FileAlreadyExists => "文件已存在",
            Self::PermissionDenied => "权限不足",
            Self::DirectoryNotEmpty => "目录非空",
            Self::DiskFull => "磁盘空间不足",
            // 系统
            Self::SystemInfoUnavailable => "系统信息不可用",
            // 网络
            Self::NetworkError => "网络错误",
            Self::ProxyError => "代理错误",
            Self::ConnectionFailed => "连接失败",
            // 任务调度
            Self::TaskNotFound => "任务不存在",
            Self::TaskAlreadyExists => "任务已存在",
            Self::TaskInvalidState => "任务状态非法",
            Self::TaskDownloadFailed => "任务下载失败",
            Self::TaskDisabled => "任务已禁用",
            Self::TaskCancelled => "任务已取消",
            // 调度记录
            Self::DispatchNotFound => "调度记录不存在",
            Self::DispatchAlreadyClaimed => "任务已被其他节点领取",
            Self::DispatchInvalidState => "调度状态非法",
            Self::DispatchExpired => "调度已过期",
            // 节点管理
            Self::NodeNotFound => "节点不存在",
            Self::NodeOffline => "节点离线",
            Self::NodeAlreadyRegistered => "节点已注册",
            Self::NodeHeartbeatTimeout => "节点心跳超时",
            Self::NodeVersionTooOld => "节点版本过低，不兼容",
            Self::NodeBusy => "节点忙碌",
            // 下载
            Self::DownloadUnsupportedProtocol => "不支持的下载协议",
            Self::DownloadConnectionFailed => "下载连接失败",
            Self::DownloadTimeout => "下载超时",
            Self::DownloadChecksumMismatch => "下载校验和不匹配",
            Self::DownloadDiskFull => "下载磁盘空间不足",
            Self::DownloadPartialContent => "部分内容，断点续传",
            Self::DownloadRangeNotSatisfied => "请求范围不满足",
            // 工作流
            Self::WorkflowNotFound => "工作流不存在",
            Self::WorkflowAlreadyExists => "工作流已存在",
            Self::WorkflowDisabled => "工作流已禁用",
            Self::WorkflowInvalidCron => "Cron 表达式无效",
            Self::WorkflowTriggerFailed => "工作流触发失败",
            // 配置
            Self::ConfigInvalid => "配置无效",
            Self::ConfigNotFound => "配置项不存在",
            Self::ConfigReadOnly => "配置只读，不可修改",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::Success => 200,
            Self::InvalidParameter
            | Self::AppManifestInvalid
            | Self::DownloadUnsupportedProtocol
            | Self::WorkflowInvalidCron
            | Self::ConfigInvalid
            | Self::DownloadRangeNotSatisfied => 400,
            Self::Unauthorized | Self::TokenInvalid | Self::TokenExpired | Self::TokenRevoked => {
                401
            }
            Self::Forbidden | Self::PermissionDenied => 403,
            Self::NotFound
            | Self::AppNotFound
            | Self::StoreAppNotFound
            | Self::FileNotFound
            | Self::ComponentNotRegistered
            | Self::BinaryNotFound
            | Self::TaskNotFound
            | Self::DispatchNotFound
            | Self::NodeNotFound
            | Self::WorkflowNotFound
            | Self::ConfigNotFound => 404,
            Self::NodeHeartbeatTimeout => 408,
            Self::AlreadyExists
            | Self::Conflict
            | Self::AppAlreadyInstalled
            | Self::ComponentAlreadyRegistered
            | Self::FileAlreadyExists
            | Self::AppAlreadyRunning
            | Self::PortConflict
            | Self::AppNotRunning
            | Self::AppNotInstalled
            | Self::TaskAlreadyExists
            | Self::TaskInvalidState
            | Self::TaskDisabled
            | Self::DispatchAlreadyClaimed
            | Self::DispatchInvalidState
            | Self::DispatchExpired
            | Self::NodeAlreadyRegistered
            | Self::NodeOffline
            | Self::NodeVersionTooOld
            | Self::NodeBusy
            | Self::DownloadChecksumMismatch
            | Self::WorkflowAlreadyExists
            | Self::WorkflowDisabled
            | Self::ConfigReadOnly => 409,
            Self::RateLimited => 429,
            Self::InternalError | Self::TaskDownloadFailed | Self::WorkflowTriggerFailed => 500,
            Self::DownloadConnectionFailed | Self::ConnectionFailed => 502,
            Self::ServiceUnavailable
            | Self::StoreSourceUnreachable
            | Self::HeartbeatTimeout
            | Self::ComponentOffline
            | Self::NoAvailableComponent
            | Self::SystemInfoUnavailable => 503,
            Self::Timeout | Self::DownloadTimeout => 504,
            Self::DownloadDiskFull | Self::DiskFull => 507,
            Self::DownloadPartialContent => 206,
            _ => 500,
        }
    }
}

/// 统一错误类型
#[derive(Debug, Error)]
pub enum PnosError {
    #[error("{code:?}: {message}")]
    Business { code: ErrorCode, message: String },
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

    pub fn http_status(&self) -> u16 {
        self.code().http_status()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_values_are_stable() {
        assert_eq!(ErrorCode::Success.code(), 0);
        assert_eq!(ErrorCode::InternalError.code(), 7);
        assert_eq!(ErrorCode::AppNotFound.code(), 1000);
        assert_eq!(ErrorCode::ComponentNotRegistered.code(), 2000);
        assert_eq!(ErrorCode::TaskNotFound.code(), 6000);
        assert_eq!(ErrorCode::NodeNotFound.code(), 7000);
        assert_eq!(ErrorCode::DownloadTimeout.code(), 8002);
        assert_eq!(ErrorCode::WorkflowNotFound.code(), 9000);
    }

    #[test]
    fn error_code_http_status_mapping() {
        assert_eq!(ErrorCode::Success.http_status(), 200);
        assert_eq!(ErrorCode::InvalidParameter.http_status(), 400);
        assert_eq!(ErrorCode::Unauthorized.http_status(), 401);
        assert_eq!(ErrorCode::Forbidden.http_status(), 403);
        assert_eq!(ErrorCode::NotFound.http_status(), 404);
        assert_eq!(ErrorCode::AlreadyExists.http_status(), 409);
        assert_eq!(ErrorCode::RateLimited.http_status(), 429);
        assert_eq!(ErrorCode::InternalError.http_status(), 500);
        assert_eq!(ErrorCode::ServiceUnavailable.http_status(), 503);
        assert_eq!(ErrorCode::Timeout.http_status(), 504);
        assert_eq!(ErrorCode::TaskNotFound.http_status(), 404);
        assert_eq!(ErrorCode::DispatchAlreadyClaimed.http_status(), 409);
        assert_eq!(ErrorCode::NodeHeartbeatTimeout.http_status(), 408);
        assert_eq!(ErrorCode::DownloadDiskFull.http_status(), 507);
    }

    #[test]
    fn pnos_error_from_error_code() {
        let err: PnosError = ErrorCode::TaskNotFound.into();
        assert_eq!(err.code(), ErrorCode::TaskNotFound);
        assert!(err.message().contains("任务不存在"));
        assert_eq!(err.http_status(), 404);
    }

    #[test]
    fn pnos_error_business_constructor() {
        let err = PnosError::new(ErrorCode::NodeBusy, "节点忙碌中");
        assert_eq!(err.code(), ErrorCode::NodeBusy);
        assert_eq!(err.message(), "节点忙碌中");
    }
}
