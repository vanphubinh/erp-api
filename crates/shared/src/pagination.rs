use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Query parameters for pagination
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PageParams {
    /// Current page number (1-based)
    #[serde(default = "default_page")]
    #[param(example = 1, minimum = 1)]
    pub page: u64,

    /// Number of items per page
    #[serde(default = "default_page_size")]
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

impl PageParams {
    /// Calculate database offset based on current page
    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * self.page_size
    }

    /// Get limit for database query
    pub fn limit(&self) -> u64 {
        self.page_size
    }

    /// Validate and clamp values to acceptable ranges
    pub fn validate(mut self, max_page_size: u64) -> Self {
        if self.page == 0 {
            self.page = 1;
        }
        if self.page_size == 0 {
            self.page_size = default_page_size();
        }
        if self.page_size > max_page_size {
            self.page_size = max_page_size;
        }
        self
    }
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

/// Pagination metadata for responses
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    /// Current page number (1-based)
    #[schema(example = 1)]
    pub page: u64,

    /// Number of items per page
    #[schema(example = 20)]
    pub page_size: u64,

    /// Total number of items in database
    #[schema(example = 100)]
    pub total: u64,

    /// Total number of pages
    #[schema(example = 5)]
    pub total_pages: u64,

    /// Whether there is a next page
    #[schema(example = true)]
    pub has_next: bool,

    /// Whether there is a previous page
    #[schema(example = false)]
    pub has_prev: bool,
}

impl PaginationMeta {
    /// Create pagination metadata from page, page_size, and total count
    pub fn new(page: u64, page_size: u64, total: u64) -> Self {
        let total_pages = if total == 0 || page_size == 0 {
            1
        } else {
            // Ceiling division: (total + page_size - 1) / page_size
            (total + page_size - 1) / page_size
        };

        Self {
            page,
            page_size,
            total,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }

    /// Create pagination metadata from PageParams and total count
    pub fn from_params(params: &PageParams, total: u64) -> Self {
        Self::new(params.page, params.page_size, total)
    }
}
