use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

impl Client {
    /// List the agent users belonging to the account.
    pub async fn list_users(&self, page: Page) -> Result<ApiResponse> {
        let mut q = Vec::new();
        page.apply(&mut q);
        self.get("/users", &q).await
    }
}
