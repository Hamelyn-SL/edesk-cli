use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Pagination metadata returned by every list endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Paginator {
    #[serde(rename = "currentPage")]
    pub current_page: u64,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: u64,
    #[serde(rename = "totalItemsCount")]
    pub total_items_count: u64,
}

impl Paginator {
    /// Whether there are more pages after the current one.
    pub fn has_more(&self) -> bool {
        self.current_page * self.items_per_page < self.total_items_count
    }
}

/// The `{ "data": ..., "paginator": ... }` envelope every eDesk response uses.
///
/// `data` is kept as raw JSON on purpose — see the crate-level docs.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse {
    #[serde(default)]
    pub data: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Paginator>,
}

/// Page selection for list endpoints.
///
/// Neither parameter is documented in the eDesk OpenAPI specs, but both are
/// honored by the live API (verified empirically): `page` is 1-based and
/// `itemsPerPage` defaults to 20 server-side.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Page {
    pub page: Option<u64>,
    pub items_per_page: Option<u64>,
}

impl Page {
    pub(crate) fn apply(&self, query: &mut Vec<(String, String)>) {
        if let Some(page) = self.page {
            query.push(("page".into(), page.to_string()));
        }
        if let Some(per_page) = self.items_per_page {
            query.push(("itemsPerPage".into(), per_page.to_string()));
        }
    }
}
