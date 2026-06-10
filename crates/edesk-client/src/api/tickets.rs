use serde::Serialize;
use serde_json::Value;

use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

/// A custom field name/value pair. The value must match the type configured
/// for the field in eDesk (API validation code 4018 otherwise).
#[derive(Debug, Clone, Serialize)]
pub struct CustomField {
    pub name: String,
    pub value: Value,
}

/// Inline contact for ticket/order creation. `email` is required by the API.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ContactRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub email: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTicketRequest {
    pub subject: String,
    pub channel_id: i64,
    /// One of: Scheduled, Spam, Archived, Open, Pending, Closed.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<i64>,
    /// Format: `YYYY-MM-DD HH:MM:SS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactRequest>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTicketRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    /// Update accepts the wide status set: Scheduled, Spam, Unread, Read,
    /// Unpriority, Priority, Archived, Open, Pending, Closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Note: the request field is `tag_ids` while the ticket model exposes
    /// `tags_ids` — an inconsistency in the upstream API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct ListTicketsParams {
    pub page: Page,
    /// One of: id, created_at, last_updated_at.
    pub order_by: Option<String>,
    /// asc or desc.
    pub order_direction: Option<String>,
    pub contact_id: Option<i64>,
    pub channel_id: Option<i64>,
    /// Wide status set (includes Unread, Read, Priority, ...).
    pub status: Option<String>,
    pub ticket_type: Option<String>,
    pub sales_order_id: Option<i64>,
    /// `YYYY-MM-DD`, inclusive.
    pub created_at_gte: Option<String>,
    pub created_at_lte: Option<String>,
    pub last_updated_at_gte: Option<String>,
    pub last_updated_at_lte: Option<String>,
    pub owner_user_id: Option<i64>,
    pub seller_order_id: Option<String>,
}

impl ListTicketsParams {
    fn to_query(&self) -> Vec<(String, String)> {
        let mut q = Vec::new();
        self.page.apply(&mut q);
        push_opt(&mut q, "order_by", &self.order_by);
        push_opt(&mut q, "order_direction", &self.order_direction);
        push_opt_num(&mut q, "filter_contact_id_equals", self.contact_id);
        push_opt_num(&mut q, "filter_channel_id_equals", self.channel_id);
        push_opt(&mut q, "filter_status_equals", &self.status);
        push_opt(&mut q, "filter_type_equals", &self.ticket_type);
        push_opt_num(&mut q, "filter_sales_order_id_equals", self.sales_order_id);
        push_opt(&mut q, "filter_created_at_gte", &self.created_at_gte);
        push_opt(&mut q, "filter_created_at_lte", &self.created_at_lte);
        push_opt(
            &mut q,
            "filter_last_updated_at_gte",
            &self.last_updated_at_gte,
        );
        push_opt(
            &mut q,
            "filter_last_updated_at_lte",
            &self.last_updated_at_lte,
        );
        push_opt_num(&mut q, "filter_owner_user_id_equals", self.owner_user_id);
        push_opt(
            &mut q,
            "filter_seller_order_id_equals",
            &self.seller_order_id,
        );
        q
    }
}

pub(crate) fn push_opt(q: &mut Vec<(String, String)>, key: &str, value: &Option<String>) {
    if let Some(v) = value {
        q.push((key.to_string(), v.clone()));
    }
}

pub(crate) fn push_opt_num(q: &mut Vec<(String, String)>, key: &str, value: Option<i64>) {
    if let Some(v) = value {
        q.push((key.to_string(), v.to_string()));
    }
}

impl Client {
    pub async fn list_tickets(&self, params: &ListTicketsParams) -> Result<ApiResponse> {
        self.get("/tickets", &params.to_query()).await
    }

    pub async fn get_ticket(&self, ticket_id: i64) -> Result<ApiResponse> {
        self.get(&format!("/tickets/{ticket_id}"), &[]).await
    }

    pub async fn create_ticket(&self, req: &CreateTicketRequest) -> Result<ApiResponse> {
        self.post("/tickets", &serde_json::to_value(req)?).await
    }

    pub async fn update_ticket(
        &self,
        ticket_id: i64,
        req: &UpdateTicketRequest,
    ) -> Result<ApiResponse> {
        self.put(
            &format!("/tickets/{ticket_id}"),
            &serde_json::to_value(req)?,
        )
        .await
    }

    /// Update only the ticket's custom fields (`PUT /tickets/{id}/data`).
    pub async fn update_ticket_data(
        &self,
        ticket_id: i64,
        custom_fields: &[CustomField],
    ) -> Result<ApiResponse> {
        let body = serde_json::json!({ "custom_fields": custom_fields });
        self.put(&format!("/tickets/{ticket_id}/data"), &body).await
    }

    pub async fn delete_ticket(&self, ticket_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/tickets/{ticket_id}")).await
    }
}
