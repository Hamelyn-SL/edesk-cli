use serde::Serialize;

use crate::error::Result;
use crate::types::ApiResponse;
use crate::Client;

/// A resolved tracking link for a specific order (full URL, no placeholder).
#[derive(Debug, Clone, Serialize)]
pub struct TrackingLink {
    pub tracking_link: String,
    /// Max 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_carrier_name: Option<String>,
}

impl Client {
    /// Add tracking links to a sales order. Returns the full sales order.
    pub async fn create_tracking_links(
        &self,
        sales_order_id: i64,
        links: &[TrackingLink],
    ) -> Result<ApiResponse> {
        let body = serde_json::json!({ "tracking_links": links });
        self.post(
            &format!("/sales-orders-tracking-links/{sales_order_id}"),
            &body,
        )
        .await
    }

    pub async fn get_tracking_links(&self, sales_order_id: i64) -> Result<ApiResponse> {
        self.get(
            &format!("/sales-orders-tracking-links/{sales_order_id}"),
            &[],
        )
        .await
    }

    /// Replace the tracking links of a sales order. Returns the full sales order.
    pub async fn update_tracking_links(
        &self,
        sales_order_id: i64,
        links: &[TrackingLink],
    ) -> Result<ApiResponse> {
        let body = serde_json::json!({ "tracking_links": links });
        self.put(
            &format!("/sales-orders-tracking-links/{sales_order_id}"),
            &body,
        )
        .await
    }

    /// Delete ALL tracking links of a sales order (the API has no per-link delete).
    pub async fn delete_tracking_links(&self, sales_order_id: i64) -> Result<ApiResponse> {
        self.delete(&format!("/sales-orders-tracking-links/{sales_order_id}"))
            .await
    }
}
