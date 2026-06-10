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

/// A file to attach to an order note.
///
/// Uploaded as `multipart/form-data` (`attachments[]` file parts plus an
/// optional `attachmentType` field). The spec also documents a JSON variant
/// with base64-encoded `files`, but the live API accepts it without ever
/// materializing the attachment — verified empirically; only multipart works.
#[derive(Debug, Clone)]
pub struct NoteFile {
    pub name: String,
    /// MIME type, e.g. `image/png`.
    pub mime_type: String,
    pub bytes: Vec<u8>,
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

    /// Attach files to an order note via multipart upload. Returns the
    /// updated order note. `attachment_type` is `Other` or `Invoice` and
    /// applies to all files in the batch (an `attachmentType` per file is not
    /// supported upstream — sending it as an array crashes the server).
    pub async fn create_order_note_attachment(
        &self,
        order_note_id: i64,
        files: Vec<NoteFile>,
        attachment_type: Option<&str>,
    ) -> Result<ApiResponse> {
        use reqwest::multipart::{Form, Part};

        let mut form = Form::new();
        for file in files {
            let part = Part::bytes(file.bytes)
                .file_name(file.name)
                .mime_str(&file.mime_type)
                .map_err(crate::error::Error::Network)?;
            form = form.part("attachments[]", part);
        }
        if let Some(kind) = attachment_type {
            form = form.text("attachmentType", kind.to_string());
        }
        self.post_multipart(&format!("/order-notes/{order_note_id}/attachments"), form)
            .await
    }
}
