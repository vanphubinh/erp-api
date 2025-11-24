use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PageParams {
    #[serde(default = "default_page")]
    #[param(example = 1, minimum = 1)]
    pub page: u64,

    #[serde(default = "default_page_size")]
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub page_size: u64,
}

const fn default_page() -> u64 {
    1
}

const fn default_page_size() -> u64 {
    20
}

impl PageParams {
    pub const fn offset(&self) -> u64 {
        self.page.saturating_sub(1) * self.page_size
    }

    pub const fn limit(&self) -> u64 {
        self.page_size
    }

    pub fn validate(mut self, max_page_size: u64) -> Self {
        self.page = self.page.max(1);
        self.page_size = self.page_size.clamp(1, max_page_size);
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    #[schema(example = 1)]
    pub page: u64,
    #[schema(example = 20)]
    pub page_size: u64,
    #[schema(example = 100)]
    pub total: u64,
    #[schema(example = 5)]
    pub total_pages: u64,
    #[schema(example = true)]
    pub has_next: bool,
    #[schema(example = false)]
    pub has_prev: bool,
}

impl PaginationMeta {
    pub fn new(page: u64, page_size: u64, total: u64) -> Self {
        let total_pages = if total == 0 || page_size == 0 {
            1
        } else {
            (total + page_size - 1) / page_size // Ceiling division
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

    pub fn from_params(params: &PageParams, total: u64) -> Self {
        Self::new(params.page, params.page_size, total)
    }
}
