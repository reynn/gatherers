use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "previewId")]
    pub preview_id: Option<String>,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: i64,
    pub price: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,
    pub deleted: bool,
    pub permissions: Permissions,
    pub details: MediaDetails,
    pub purchased: bool,
    pub whitelisted: bool,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: i64,
    pub access: bool,
    pub preview: Option<MediaDetails>,
    pub whitelist: Option<Vec<Whitelist>>,
    #[serde(rename = "likeCount")]
    pub like_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaDetails {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: i64,
    pub status: i64,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub mimetype: String,
    pub filename: String,
    pub width: i64,
    pub height: i64,
    pub metadata: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub variants: Vec<Variant>,
    #[serde(rename = "variantHash")]
    pub variant_hash: VariantHash,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "locationId")]
    pub location_id: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantHash {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub id: String,
    #[serde(rename = "type")]
    pub variant_type: i64,
    pub status: i64,
    pub mimetype: String,
    pub filename: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(rename = "permissionFlags")]
    pub permission_flags: Vec<PermissionFlag>,
    #[serde(rename = "accountPermissionFlags")]
    pub account_permission_flags: AccountPermissionFlags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPermissionFlags {
    pub flags: i64,
    pub metadata: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionFlag {
    pub id: String,
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    #[serde(rename = "type")]
    pub permission_flag_type: i64,
    pub flags: i64,
    pub price: i64,
    pub metadata: String,
    #[serde(rename = "validAfter")]
    pub valid_after: Option<i64>,
    #[serde(rename = "validBefore")]
    pub valid_before: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Whitelist {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "permissionFlags")]
    pub permission_flags: i64,
}
