use serde_json::Value;

use super::tickets::{push_opt, push_opt_num};
use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

#[derive(Debug, Clone, Default)]
pub struct ListSalesOrdersParams {
    pub page: Page,
    /// One of: id, created_at, last_updated_at.
    pub order_by: Option<String>,
    /// asc or desc.
    pub order_direction: Option<String>,
    pub contact_id: Option<i64>,
    pub channel_id: Option<i64>,
    pub seller_order_id: Option<String>,
    /// One of: OrderReceived, PaymentReceived, PaymentRejected,
    /// PaymentAccepted, OrderShipped, InTransit, Delivered, Canceled,
    /// Returned, Hold.
    pub status: Option<String>,
    pub id_gte: Option<i64>,
    pub id_lte: Option<i64>,
    /// `YYYY-MM-DD`, inclusive.
    pub created_at_gte: Option<String>,
    pub created_at_lte: Option<String>,
    pub last_updated_at_gte: Option<String>,
    pub last_updated_at_lte: Option<String>,
}

impl ListSalesOrdersParams {
    fn to_query(&self) -> Vec<(String, String)> {
        let mut q = Vec::new();
        self.page.apply(&mut q);
        push_opt(&mut q, "order_by", &self.order_by);
        push_opt(&mut q, "order_direction", &self.order_direction);
        push_opt_num(&mut q, "filter_contact_id_equals", self.contact_id);
        push_opt_num(&mut q, "filter_channel_id_equals", self.channel_id);
        push_opt(
            &mut q,
            "filter_seller_order_id_equals",
            &self.seller_order_id,
        );
        push_opt(&mut q, "filter_status_equals", &self.status);
        push_opt_num(&mut q, "filter_id_gte", self.id_gte);
        push_opt_num(&mut q, "filter_id_lte", self.id_lte);
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
        q
    }
}

impl Client {
    pub async fn list_sales_orders(&self, params: &ListSalesOrdersParams) -> Result<ApiResponse> {
        self.get("/sales-orders", &params.to_query()).await
    }

    pub async fn get_sales_order(&self, sales_order_id: i64) -> Result<ApiResponse> {
        self.get(&format!("/sales-orders/{sales_order_id}"), &[])
            .await
    }

    /// Create a sales order. The request body is passed through as JSON: the
    /// upstream schema is a wide `oneOf` (inline contact vs `contact_id`) with
    /// nested order items, addresses and tracking codes, so it is accepted
    /// here untyped and validated server-side.
    pub async fn create_sales_order(&self, body: &Value) -> Result<ApiResponse> {
        self.post("/sales-orders", body).await
    }

    /// Update a sales order (raw JSON body — see [`Client::create_sales_order`]).
    /// Note: on update each order item must carry its existing item `id`.
    pub async fn update_sales_order(
        &self,
        sales_order_id: i64,
        body: &Value,
    ) -> Result<ApiResponse> {
        self.put(&format!("/sales-orders/{sales_order_id}"), body)
            .await
    }

    pub async fn delete_sales_order(&self, sales_order_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/sales-orders/{sales_order_id}"))
            .await
    }
}
