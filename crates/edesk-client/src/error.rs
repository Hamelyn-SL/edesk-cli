use serde_json::Value;

pub type Result<T> = std::result::Result<T, Error>;

/// A field-level validation failure from a 400 response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    pub field: String,
    pub code: i64,
}

impl FieldError {
    /// Human-readable meaning of the eDesk validation error code (4001-4022).
    pub fn reason(&self) -> &'static str {
        match self.code {
            4001 => "missing required field",
            4002 => "object not found",
            4003 => "must be a unique value",
            4004 => "no access to this object",
            4005 => "must be a numeric value",
            4006 => "must be an array",
            4007 => "must be one of the allowed values",
            4008 => "must be a string",
            4009 => "must be a boolean",
            4010 => "must be a valid date",
            4011 => "must be a valid file URL",
            4012 => "must be a valid image URL",
            4013 => "related channel type is not supported",
            4014 => "mismatch between sales order and channel",
            4015 => "mismatch between client and channels",
            4016 => "templates limit exceeded",
            4017 => "mismatch between sales order and sales order item",
            4018 => "custom field value must match the field type",
            4019 => "each attachment must match the attachment format",
            4020 => "exceeds the maximum length",
            4021 => "must be an email",
            4022 => "message items limit reached",
            _ => "unknown validation error",
        }
    }
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} (code {})", self.field, self.reason(), self.code)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// HTTP 401/403 — bad or missing token.
    #[error("authentication failed (HTTP {status}): {message}")]
    Auth { status: u16, message: String },

    /// HTTP 400 with per-field validation details.
    #[error("validation failed: {}", format_field_errors(.field_errors, .message))]
    Validation {
        message: String,
        field_errors: Vec<FieldError>,
    },

    /// Any other non-2xx API response.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        status: u16,
        message: String,
        details: Option<Value>,
    },

    #[error("failed to decode API response: {0}")]
    Decode(#[from] serde_json::Error),
}

impl Error {
    /// Build the right error variant from a non-2xx response body.
    pub(crate) fn from_response(status: u16, body: &str) -> Self {
        let parsed: Option<Value> = serde_json::from_str(body).ok();
        let error_obj = parsed.as_ref().and_then(|v| v.get("error"));
        let message = error_obj
            .and_then(|e| e.get("message"))
            .and_then(Value::as_str)
            .unwrap_or_else(|| non_empty_or(body, "unknown error"))
            .to_string();
        let details = error_obj.and_then(|e| e.get("details")).cloned();

        if status == 401 || status == 403 {
            return Error::Auth { status, message };
        }

        if status == 400 {
            if let Some(Value::Object(map)) = &details {
                let field_errors: Vec<FieldError> = map
                    .iter()
                    .filter_map(|(field, v)| {
                        // The live API returns errorCode as a string ("4003")
                        // even though the spec declares an integer.
                        let code = v.get("errorCode").and_then(|c| {
                            c.as_i64()
                                .or_else(|| c.as_str().and_then(|s| s.parse().ok()))
                        })?;
                        Some(FieldError {
                            field: field.clone(),
                            code,
                        })
                    })
                    .collect();
                if !field_errors.is_empty() {
                    return Error::Validation {
                        message,
                        field_errors,
                    };
                }
            }
        }

        Error::Api {
            status,
            message,
            details,
        }
    }
}

fn non_empty_or<'a>(s: &'a str, fallback: &'a str) -> &'a str {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    }
}

fn format_field_errors(errors: &[FieldError], message: &str) -> String {
    if errors.is_empty() {
        return message.to_string();
    }
    let fields: Vec<String> = errors.iter().map(ToString::to_string).collect();
    format!("{} [{}]", message, fields.join("; "))
}
