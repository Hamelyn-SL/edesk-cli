//! Rust client for the [eDesk REST API](https://developers.edesk.com/).
//!
//! Requests are strongly typed where the API surface is stable; responses are
//! exposed as raw [`serde_json::Value`] wrapped in an [`ApiResponse`] envelope.
//! This is deliberate: the eDesk API mixes types across endpoints (e.g.
//! `created_at` is a Unix epoch number on tickets but a datetime string on
//! order notes, and booleans are sometimes returned as `0`/`1`), so a CLI or
//! integration should not fail to decode a response because of a field it
//! never reads.
//!
//! ```no_run
//! # async fn run() -> Result<(), edesk_client::Error> {
//! let client = edesk_client::Client::new("your-api-token")?;
//! let me = client.whoami().await?;
//! println!("{}", me.data["user"]["email"]);
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
mod types;

pub mod api;

pub use client::{Client, ClientBuilder, DEFAULT_BASE_URL, USER_AGENT};
pub use error::{Error, FieldError, Result};
pub use types::{ApiResponse, Page, Paginator};

// Re-export so downstream code can name `reqwest::Method` without pinning
// its own copy of reqwest.
pub use reqwest;
