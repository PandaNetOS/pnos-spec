//! 通用工具函数

/// 格式化字节数为人类可读字符串
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let i = (bytes as f64).log2().floor() as usize / 10;
    let i = i.min(UNITS.len() - 1);
    let value = bytes as f64 / (1024u64.pow(i as u32)) as f64;
    format!("{:.2} {}", value, UNITS[i])
}

/// 解析人类可读字节字符串为 u64
pub fn parse_bytes(s: &str) -> Result<u64, String> {
    let s = s.trim().to_lowercase();
    let multipliers: [(&str, u64); 6] = [
        ("pb", 1024u64.pow(5)),
        ("tb", 1024u64.pow(4)),
        ("gb", 1024u64.pow(3)),
        ("mb", 1024u64.pow(2)),
        ("kb", 1024),
        ("b", 1),
    ];

    for (unit, mult) in &multipliers {
        if let Some(num_str) = s.strip_suffix(unit) {
            let num: f64 = num_str.trim().parse().map_err(|_| format!("无效数字: {}", num_str))?;
            return Ok((num * *mult as f64) as u64);
        }
    }

    // 无单位，按字节
    s.parse::<u64>().map_err(|_| format!("无效字节格式: {}", s))
}

/// 校验 UUID 格式
pub fn is_valid_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let lengths = [8, 4, 4, 4, 12];
    parts.iter().zip(lengths.iter()).all(|(p, &len)| {
        p.len() == len && p.chars().all(|c| c.is_ascii_hexdigit())
    })
}

/// 生成容器名后缀（短 ID）
pub fn short_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string().chars().take(8).collect()
}
