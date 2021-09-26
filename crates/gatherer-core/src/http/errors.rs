use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpErrors {
    #[error("Status code [{status_code}] is not expected. Response: {body}")]
    BadStatus {
        status_code: surf::StatusCode,
        body: String,
    },
    #[error("Internal HTTP client library failed. {0:?}")]
    InternalHttpClientError(surf::Error),
    #[error("Malformed JSON? {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid URL {0}")]
    InvalidUrl(#[from] url::ParseError),
}
