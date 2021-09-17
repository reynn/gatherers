use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "previewId")]
    pub preview_id: Option<String>,
    pub price: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,
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

impl TryFrom<MediaDetails> for gatherer_core::gatherers::Media {
    type Error = String;

    fn try_from(details: MediaDetails) -> Result<Self, Self::Error> {
        if details.locations.is_empty() {
            return Err(format!("Content not available: {:?}", details));
        }

        Ok(Self {
            filename: details.filename.to_string(),
            url: details.locations[0].location.to_string(),
            mime_type: details.mimetype,
        })
    }
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
    pub locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "locationId")]
    pub location_id: String,
    pub location: String,
}

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
