use serde::Serialize;

use crate::error::Result;
use crate::types::ApiResponse;
use crate::Client;

/// URL-based attachment for a message. The API fetches the file from `url`.
#[derive(Debug, Clone, Serialize)]
pub struct MessageAttachment {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMessageRequest {
    pub ticket_id: i64,
    pub body: String,
    /// On create only `Message` or `Note` are accepted.
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<MessageAttachment>>,
    /// Format: `YYYY-MM-DD HH:MM:SS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_response: Option<bool>,
    /// Incoming or Outgoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// When true the message is actually sent to the customer via email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The update enum only accepts `Note`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

impl Client {
    /// Create a message in an existing ticket. There is no list-messages
    /// endpoint — enumerate IDs via the ticket's `messages_ids` field.
    pub async fn create_message(&self, req: &CreateMessageRequest) -> Result<ApiResponse> {
        self.post("/messages", &serde_json::to_value(req)?).await
    }

    pub async fn get_message(&self, message_id: i64) -> Result<ApiResponse> {
        self.get(&format!("/messages/{message_id}"), &[]).await
    }

    pub async fn update_message(
        &self,
        message_id: i64,
        req: &UpdateMessageRequest,
    ) -> Result<ApiResponse> {
        self.put(
            &format!("/messages/{message_id}"),
            &serde_json::to_value(req)?,
        )
        .await
    }

    pub async fn delete_message(&self, message_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/messages/{message_id}")).await
    }
}
