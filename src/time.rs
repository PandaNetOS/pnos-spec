//! 时间工具（RFC3339 UTC 统一格式）

use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};

/// 当前时间的 RFC3339 UTC 字符串
pub fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

/// 解析 RFC3339 字符串
pub fn parse_rfc3339(s: &str) -> Result<OffsetDateTime, String> {
    OffsetDateTime::parse(s, &Rfc3339).map_err(|e| e.to_string())
}

/// 转换为 UTC OffsetDateTime
pub fn to_utc(dt: OffsetDateTime) -> OffsetDateTime {
    dt.to_offset(UtcOffset::UTC)
}
