use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliErrors {
  #[error("Provided content type, {provided} is not supported. {valid_options:?}")]
    InvalidContentType {
        provided: String,
        valid_options: Vec<String>,
    },
}
