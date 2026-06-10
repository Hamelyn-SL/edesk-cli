use serde::Serialize;

use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

/// Shared by create and update.
///
/// The spec marks `name` and `tag_group_id` as required, but the live API
/// treats PUT as a partial update AND rejects re-sending an unchanged `name`
/// with validation code 4003 (must be unique) — verified empirically. So all
/// fields are optional here; create-side requirements are enforced by the
/// server (and by the CLI's required flags).
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_group_id: Option<i64>,
    /// Hex color WITHOUT the leading `#`, from the fixed eDesk palette
    /// (e.g. `F44336`, `2196F3`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Icon name from the fixed eDesk set (e.g. `flag`, `star`, `truck`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl Client {
    pub async fn list_tags(&self, page: Page) -> Result<ApiResponse> {
        let mut q = Vec::new();
        page.apply(&mut q);
        self.get("/tags", &q).await
    }

    pub async fn get_tag(&self, tag_id: i64) -> Result<ApiResponse> {
        self.get(&format!("/tags/{tag_id}"), &[]).await
    }

    pub async fn create_tag(&self, req: &TagRequest) -> Result<ApiResponse> {
        self.post("/tags", &serde_json::to_value(req)?).await
    }

    pub async fn update_tag(&self, tag_id: i64, req: &TagRequest) -> Result<ApiResponse> {
        self.put(&format!("/tags/{tag_id}"), &serde_json::to_value(req)?)
            .await
    }

    pub async fn delete_tag(&self, tag_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/tags/{tag_id}")).await
    }
}
