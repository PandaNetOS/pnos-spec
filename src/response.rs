//! 统一响应格式

use serde::{Deserialize, Serialize};

use crate::error::{ErrorCode, PnosError};

/// 统一 API 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: ErrorCode::Success.code(),
            message: ErrorCode::Success.message().to_string(),
            data: Some(data),
            request_id: None,
        }
    }

    pub fn success_with_msg(data: T, message: impl Into<String>) -> Self {
        ApiResponse {
            code: ErrorCode::Success.code(),
            message: message.into(),
            data: Some(data),
            request_id: None,
        }
    }

    pub fn error(err: &PnosError) -> ApiResponse<T> {
        ApiResponse {
            code: err.code().code(),
            message: err.message(),
            data: None,
            request_id: None,
        }
    }

    pub fn error_code(code: ErrorCode, message: impl Into<String>) -> ApiResponse<T> {
        ApiResponse {
            code: code.code(),
            message: message.into(),
            data: None,
            request_id: None,
        }
    }
}

/// 分页查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

impl Default for PageQuery {
    fn default() -> Self {
        PageQuery {
            page: 1,
            page_size: 20,
        }
    }
}

impl PageQuery {
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.page_size
    }
}

/// 分页结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

impl<T> PageResult<T> {
    pub fn new(items: Vec<T>, total: u64, query: &PageQuery) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            (total + query.page_size - 1) / query.page_size
        };
        PageResult {
            items,
            total,
            page: query.page,
            page_size: query.page_size,
            total_pages,
        }
    }
}
