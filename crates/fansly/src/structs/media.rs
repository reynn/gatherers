use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(rename = "previewId")]
    pub preview_id: Option<String>,
    pub price: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<i64>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(rename = "media")]
    pub details: Option<MediaDetails>,
    #[serde(default)]
    pub purchased: bool,
    #[serde(default)]
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags", default)]
    pub account_permission_flags: i64,
    #[serde(default)]
    pub access: bool,
    #[serde(rename = "likeCount")]
    pub like_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchasedMedia {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    pub r#type: u8,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDetails {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: i64,
    pub status: i64,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub mimetype: String,
    pub file_name: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub metadata: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub variants: Vec<Variant>,
    pub locations: Vec<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "locationId")]
    pub location_id: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub id: String,
    #[serde(rename = "type")]
    pub variant_type: i64,
    pub status: i64,
    pub mimetype: String,
    pub filename: Option<String>,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    pub locations: Vec<Location>,
}
