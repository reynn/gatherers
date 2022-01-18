use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpErrors {
    #[error("Status code [{status_code}] is not expected. Response: {resp:?}")]
    BadStatus {
        status_code: surf::StatusCode,
        resp: super::Response,
    },
    #[error("Internal HTTP client library failed. {0:?}")]
    InternalHttpClientError(surf::Error),
    #[error("Malformed JSON? {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid URL {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("The response body was invalid: {0}")]
    InvalidBody(String),
}
