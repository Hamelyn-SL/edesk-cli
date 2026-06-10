use crate::error::Result;
use crate::types::{ApiResponse, Page};
use crate::Client;

impl Client {
    /// Tags reference groups via `tag_group_id`, so call this before
    /// [`Client::create_tag`].
    pub async fn list_tag_groups(&self, page: Page) -> Result<ApiResponse> {
        let mut q = Vec::new();
        page.apply(&mut q);
        self.get("/tag-groups", &q).await
    }
}
