use std::time::Duration;

use reqwest::Method;
use serde_json::Value;

use crate::error::{Error, Result};
use crate::types::ApiResponse;

pub const DEFAULT_BASE_URL: &str = "https://api.edesk.com/v1";
pub const USER_AGENT: &str = concat!("edesk-cli/", env!("CARGO_PKG_VERSION"));

const MAX_ATTEMPTS: u32 = 3;
const BASE_BACKOFF: Duration = Duration::from_millis(400);

/// Authenticated eDesk API client.
///
/// Cheap to clone; the underlying HTTP connection pool is shared.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    token: String,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    token: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Override the API base URL (e.g. for a mock server in tests).
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<Client> {
        let http = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(self.timeout.unwrap_or(Duration::from_secs(30)))
            .build()?;
        Ok(Client {
            http,
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
                .trim_end_matches('/')
                .to_string(),
            token: self.token.unwrap_or_default(),
        })
    }
}

impl Client {
    /// Client against the production API with the given bearer token.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        Self::builder().token(token).build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Perform a request against an API path (e.g. `/tickets`) and decode the
    /// standard `{data, paginator}` envelope.
    ///
    /// Retries up to 2 times with exponential backoff on HTTP 429/5xx and on
    /// connection errors — except for POST, which is only retried when the
    /// request never reached the server (connect errors), since POSTs are not
    /// idempotent.
    pub async fn request(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> Result<ApiResponse> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let mut attempt = 0u32;

        loop {
            attempt += 1;
            let mut req = self
                .http
                .request(method.clone(), &url)
                .bearer_auth(&self.token);
            if !query.is_empty() {
                req = req.query(query);
            }
            if let Some(body) = body {
                req = req.json(body);
            }

            match req.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        let text = resp.text().await?;
                        return serde_json::from_str(&text).map_err(Error::Decode);
                    }
                    let retryable = status.as_u16() == 429 || status.is_server_error();
                    if retryable && attempt < MAX_ATTEMPTS && method != Method::POST {
                        tokio::time::sleep(backoff(attempt)).await;
                        continue;
                    }
                    let text = resp.text().await.unwrap_or_default();
                    return Err(Error::from_response(status.as_u16(), &text));
                }
                Err(err) => {
                    // Safe to retry any method on connect errors: the request
                    // never reached the server.
                    if err.is_connect() && attempt < MAX_ATTEMPTS {
                        tokio::time::sleep(backoff(attempt)).await;
                        continue;
                    }
                    return Err(Error::Network(err));
                }
            }
        }
    }

    pub async fn get(&self, path: &str, query: &[(String, String)]) -> Result<ApiResponse> {
        self.request(Method::GET, path, query, None).await
    }

    pub async fn post(&self, path: &str, body: &Value) -> Result<ApiResponse> {
        self.request(Method::POST, path, &[], Some(body)).await
    }

    pub async fn put(&self, path: &str, body: &Value) -> Result<ApiResponse> {
        self.request(Method::PUT, path, &[], Some(body)).await
    }

    pub async fn delete(&self, path: &str) -> Result<ApiResponse> {
        self.request(Method::DELETE, path, &[], None).await
    }

    /// POST a multipart form (file uploads). Not retried.
    pub async fn post_multipart(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<ApiResponse> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .multipart(form)
            .send()
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        if status.is_success() {
            serde_json::from_str(&text).map_err(Error::Decode)
        } else {
            Err(Error::from_response(status.as_u16(), &text))
        }
    }
}

fn backoff(attempt: u32) -> Duration {
    BASE_BACKOFF * 2u32.saturating_pow(attempt - 1)
}
