use serde::Serialize;

use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderNoteRequest {
    pub sales_order_id: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateOrderNoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<i64>,
    pub text: String,
}

/// A file attached to an order note, sent inline as base64.
#[derive(Debug, Clone, Serialize)]
pub struct NoteFile {
    pub name: String,
    /// MIME type, e.g. `image/png`.
    #[serde(rename = "type")]
    pub mime_type: String,
    /// Base64-encoded file content (not a URL).
    pub base64: String,
    /// `Other` or `Invoice`.
    #[serde(rename = "attachmentType")]
    pub attachment_type: String,
}

impl Client {
    /// The spec declares no filters for this endpoint; only paging works.
    pub async fn list_order_notes(&self, page: Page) -> Result<ApiResponse> {
        let mut q = Vec::new();
        page.apply(&mut q);
        self.get("/order-notes", &q).await
    }

    pub async fn get_order_note(&self, order_note_id: i64) -> Result<ApiResponse> {
        self.get(&format!("/order-notes/{order_note_id}"), &[])
            .await
    }

    pub async fn create_order_note(&self, req: &CreateOrderNoteRequest) -> Result<ApiResponse> {
        self.post("/order-notes", &serde_json::to_value(req)?).await
    }

    pub async fn update_order_note(
        &self,
        order_note_id: i64,
        req: &UpdateOrderNoteRequest,
    ) -> Result<ApiResponse> {
        self.put(
            &format!("/order-notes/{order_note_id}"),
            &serde_json::to_value(req)?,
        )
        .await
    }

    pub async fn delete_order_note(&self, order_note_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/order-notes/{order_note_id}")).await
    }

    /// Attach files to an order note (JSON variant: base64-encoded content).
    /// Returns the updated order note.
    pub async fn create_order_note_attachment(
        &self,
        order_note_id: i64,
        files: &[NoteFile],
    ) -> Result<ApiResponse> {
        let body = serde_json::json!({ "files": files });
        self.post(&format!("/order-notes/{order_note_id}/attachments"), &body)
            .await
    }
}
