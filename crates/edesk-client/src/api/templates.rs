use serde_json::Value;

use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

impl Client {
    /// The spec declares no filters for this endpoint; only paging works.
    pub async fn list_templates(&self, page: Page) -> Result<ApiResponse> {
        let mut q = Vec::new();
        page.apply(&mut q);
        self.get("/templates", &q).await
    }

    pub async fn get_template(&self, template_id: i64) -> Result<ApiResponse> {
        self.get(&format!("/templates/{template_id}"), &[]).await
    }

    /// Create a template. Raw JSON body: the template schema is very wide
    /// (usage/type/query-type/order-status enums, URL attachments, AI
    /// classification rules), so it is passed through and validated
    /// server-side. Required fields: `name`, `body_text`, `template_usage`,
    /// `template_type`, `active`.
    pub async fn create_template(&self, body: &Value) -> Result<ApiResponse> {
        self.post("/templates", body).await
    }

    /// Update a template. Full-replace PUT: the API requires the same fields
    /// as create to be present.
    pub async fn update_template(&self, template_id: i64, body: &Value) -> Result<ApiResponse> {
        self.put(&format!("/templates/{template_id}"), body).await
    }

    pub async fn delete_template(&self, template_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/templates/{template_id}")).await
    }
}
