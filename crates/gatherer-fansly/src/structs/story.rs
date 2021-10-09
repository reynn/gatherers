use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "contentType")]
    pub content_type: i64,
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}
