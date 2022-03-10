use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaBundle {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(rename = "previewId")]
    pub preview_id: Option<String>,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: i64,
    pub price: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<i64>,
    pub deleted: bool,
    #[serde(rename = "accountMediaIds")]
    pub account_media_ids: Vec<String>,
    #[serde(rename = "bundleContent")]
    pub bundle_content: Vec<BundleContent>,
    pub purchased: bool,
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    pub access: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleContent {
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    pub pos: i64,
}
