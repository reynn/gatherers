use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub purchased: bool,
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    pub access: bool,
}