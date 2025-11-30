use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "kebab-case")]
pub struct PageParams {
    #[serde(default = "default_page")]
    #[param(example = 1, minimum = 1)]
    pub page: u32,

    #[serde(default = "default_page_size")]
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub page_size: u32,
}

const fn default_page() -> u32 {
    1
}

const fn default_page_size() -> u32 {
    20
}

impl PageParams {
    pub fn offset(&self) -> u64 {
        u64::from(self.page.saturating_sub(1)) * u64::from(self.page_size)
    }

    pub fn limit(&self) -> u64 {
        u64::from(self.page_size)
    }

    pub fn validate(mut self, max_page_size: u32) -> Self {
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
    pub page: u32,
    #[schema(example = 20)]
    pub page_size: u32,
    #[schema(example = 100)]
    pub total: u32,
    #[schema(example = 5)]
    pub total_pages: u32,
    #[schema(example = true)]
    pub has_next: bool,
    #[schema(example = false)]
    pub has_prev: bool,
}

impl PaginationMeta {
    pub fn new(page: u32, page_size: u32, total: u32) -> Self {
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

    pub fn from_params(params: &PageParams, total: u32) -> Self {
        Self::new(params.page, params.page_size, total)
    }
}
