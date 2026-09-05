//! 系统信息与监控数据格式

use serde::{Deserialize, Serialize};

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// 主机名
    pub hostname: String,
    /// 操作系统名称
    pub os: String,
    /// 操作系统版本
    pub os_version: String,
    /// 内核版本
    pub kernel: String,
    /// 架构（x86_64 / aarch64）
    pub arch: String,
    /// CPU 型号
    pub cpu_model: String,
    /// CPU 核心数
    pub cpu_cores: u32,
    /// 总内存（字节）
    pub memory_total: u64,
    /// 运行时间（秒）
    pub uptime: u64,
    /// pnos 版本
    pub pnos_version: String,
}

/// 系统实时监控数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    /// CPU 使用率（0-100）
    pub cpu_usage: f32,
    /// 各核心使用率
    #[serde(default)]
    pub cpu_per_core: Vec<f32>,
    /// 内存总量（字节）
    pub memory_total: u64,
    /// 内存已用（字节）
    pub memory_used: u64,
    /// 内存使用率（0-100）
    pub memory_usage: f32,
    /// Swap 总量（字节）
    #[serde(default)]
    pub swap_total: u64,
    /// Swap 已用（字节）
    #[serde(default)]
    pub swap_used: u64,
    /// 磁盘信息
    #[serde(default)]
    pub disks: Vec<DiskInfo>,
    /// 网络统计
    #[serde(default)]
    pub network: NetworkStats,
    /// 系统负载（1/5/15 分钟）
    #[serde(default)]
    pub load_average: [f32; 3],
    /// 运行中的进程数
    #[serde(default)]
    pub process_count: u32,
}

/// 磁盘信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// 设备名（如 /dev/sda1）
    pub device: String,
    /// 挂载点
    pub mount_point: String,
    /// 文件系统类型
    pub fs_type: String,
    /// 总容量（字节）
    pub total: u64,
    /// 已用（字节）
    pub used: u64,
    /// 可用（字节）
    pub available: u64,
    /// 使用率（0-100）
    pub usage: f32,
}

/// 网络统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    /// 网络接口名
    pub interface: String,
    /// 接收字节速率（B/s）
    pub rx_bytes_per_sec: u64,
    /// 发送字节速率（B/s）
    pub tx_bytes_per_sec: u64,
    /// 总接收字节
    pub rx_total: u64,
    /// 总发送字节
    pub tx_total: u64,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            interface: "eth0".to_string(),
            rx_bytes_per_sec: 0,
            tx_bytes_per_sec: 0,
            rx_total: 0,
            tx_total: 0,
        }
    }
}
