use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

impl Client {
    pub async fn list_channels(&self, page: Page) -> Result<ApiResponse> {
        let mut q = Vec::new();
        page.apply(&mut q);
        self.get("/channels", &q).await
    }
}
