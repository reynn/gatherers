use thiserror::Error;

#[derive(Debug, Error)]
pub enum GathererErrors {
    #[error("No posts found for {username}, using the {gatherer} gatherer")]
    NoPostsFound { gatherer: String, username: String },
    #[error("No messages found for {username}, using the {gatherer} gatherer")]
    NoMessagesFound { gatherer: String, username: String },
    #[error("No stories found for {username}, using the {gatherer} gatherer")]
    NoStoriesFound { gatherer: String, username: String },
    #[error("No subscriptions found using the {gatherer} gatherer")]
    NoSubscriptionsFound { gatherer: String },
    #[error("The {gatherer_name} gatherer does not support the {feature} feature")]
    NotSupportedByGatherer {
        gatherer_name: String,
        feature: String,
    },
    #[error("Failed to initialize the {0} gatherer, error received: {1}")]
    FailedToInitialize(String, String),
    #[error("Gatherer, {gatherer_name}, is enabled but there is no value provided for {option}")]
    OptionNotProvided {
        gatherer_name: String,
        option: String,
    },
    #[error("The {name} gatherer is not enabled")]
    NotEnabled { name: String },
    #[error("Invalid credentials for the {name} gatherer. {msg}")]
    InvalidCredentials { name: String, msg: String },
    #[error("API Error: Status Code {status}")]
    HttpError { status: reqwest::StatusCode, response_body: Option<String> },

    // Sub errors from other modules
    #[error("Failed to parse URL, details: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("Unable to add header to reqwest: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Unable to add header to reqwest: {0}")]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error("Internal `reqwest` error, {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Malformed JSON? {0}")]
    JsonError(#[from] serde_json::Error),
}
