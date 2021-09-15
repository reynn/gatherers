use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpErrors {
    #[error("Status code [{status_code}] is not expected. Response: {body}")]
    BadStatus {
        status_code: u16,
        body: String,
    },
    #[error("Internal HTTP client library failed. {0}")]
    InternalHttpClientError(#[from] reqwest::Error),
    #[error("Unable to add header to reqwest (Header Value Invalid): Value[{value}]: {source}")]
    InvalidHeaderValue {
        source: reqwest::header::InvalidHeaderValue,
        value: String,
    },
    #[error("Unable to add header to reqwest (Header Name Invalid): Value[{value}]: {source}")]
    InvalidHeaderName {
        source: reqwest::header::InvalidHeaderName,
        value: String,
    },
    #[error("Malformed JSON? {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid URL {0}")]
    InvalidUrl(#[from] url::ParseError),
}
