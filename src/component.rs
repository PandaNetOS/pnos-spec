//! 组件类型与状态
//!
//! 统一应用（App）与 Agent 的组件模型，所有通信实体都是 Component。
//! 通过 `component_type` 区分应用/Agent/运行时/主控，通过 `capabilities` 声明能力。

use serde::{Deserialize, Serialize};

/// 组件类型
///
/// 统一应用与 Agent 的分类，所有接入 pnos 生态的通信实体都是 Component。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    /// 第三方应用（如 pcdn-keeper、web 前端等）
    App,
    /// 下载/发现 Agent（如 spde、PeerDiscoveryCenter）
    Agent,
    /// 运行时（pnos-runtime，注册中心/反向代理/商店）
    Runtime,
    /// 主控台（pk，Agent 管理/任务调度/工作流）
    Pk,
}

impl ComponentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Agent => "agent",
            Self::Runtime => "runtime",
            Self::Pk => "pk",
        }
    }
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 组件状态
///
/// 统一应用状态与节点状态，覆盖运行/忙碌/离线/错误/未安装/安装中/已停止。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStatus {
    /// 运行中，可接收请求/任务
    Running,
    /// 忙碌，活跃任务达到上限
    Busy,
    /// 离线，心跳超时
    Offline,
    /// 错误，运行异常
    Error,
    /// 未安装（商店应用专用）
    NotInstalled,
    /// 安装中（商店应用专用）
    Installing,
    /// 已停止（商店应用专用）
    Stopped,
}

impl ComponentStatus {
    /// 字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Busy => "busy",
            Self::Offline => "offline",
            Self::Error => "error",
            Self::NotInstalled => "not_installed",
            Self::Installing => "installing",
            Self::Stopped => "stopped",
        }
    }

    /// 是否可接收请求/任务
    pub fn is_available(&self) -> bool {
        matches!(self, Self::Running)
    }

    /// 是否为离线/终止状态
    pub fn is_offline(&self) -> bool {
        matches!(self, Self::Offline | Self::Error | Self::Stopped)
    }
}

impl Default for ComponentStatus {
    fn default() -> Self {
        Self::Running
    }
}

impl std::fmt::Display for ComponentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Running => "running",
            Self::Busy => "busy",
            Self::Offline => "offline",
            Self::Error => "error",
            Self::NotInstalled => "not_installed",
            Self::Installing => "installing",
            Self::Stopped => "stopped",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ComponentType::App).unwrap(),
            "\"app\""
        );
        assert_eq!(
            serde_json::to_string(&ComponentType::Agent).unwrap(),
            "\"agent\""
        );
        assert_eq!(
            serde_json::to_string(&ComponentType::Runtime).unwrap(),
            "\"runtime\""
        );
        assert_eq!(serde_json::to_string(&ComponentType::Pk).unwrap(), "\"pk\"");
    }

    #[test]
    fn component_status_availability() {
        assert!(ComponentStatus::Running.is_available());
        assert!(!ComponentStatus::Busy.is_available());
        assert!(!ComponentStatus::Offline.is_available());
    }

    #[test]
    fn component_status_offline_detection() {
        assert!(ComponentStatus::Offline.is_offline());
        assert!(ComponentStatus::Error.is_offline());
        assert!(ComponentStatus::Stopped.is_offline());
        assert!(!ComponentStatus::Running.is_offline());
    }
}
