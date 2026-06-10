use super::tickets::{push_opt, push_opt_num};
use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

#[derive(Debug, Clone, Default)]
pub struct ListContactsParams {
    pub page: Page,
    /// Full-text search across contact fields.
    pub query: Option<String>,
    pub consumer_id: Option<i64>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
}

impl ListContactsParams {
    fn to_query(&self) -> Vec<(String, String)> {
        let mut q = Vec::new();
        self.page.apply(&mut q);
        push_opt(&mut q, "fsf_query", &self.query);
        push_opt_num(&mut q, "consumer_id", self.consumer_id);
        push_opt(&mut q, "email", &self.email);
        push_opt(&mut q, "name", &self.name);
        push_opt(&mut q, "phone_number", &self.phone_number);
        q
    }
}

impl Client {
    pub async fn list_contacts(&self, params: &ListContactsParams) -> Result<ApiResponse> {
        self.get("/contacts", &params.to_query()).await
    }
}
