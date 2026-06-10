use crate::error::Result;
use crate::types::ApiResponse;
use crate::Client;

impl Client {
    /// Identity of the API token's caller (`data.user.{id,name,email,...}`).
    /// Useful to validate a token.
    pub async fn whoami(&self) -> Result<ApiResponse> {
        self.get("/whoami", &[]).await
    }
}
