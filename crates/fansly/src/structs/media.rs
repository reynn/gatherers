use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub account_id: String,
    #[serde(rename = "accountMediaId")]
    pub account_media_id: String,
    pub r#type: u8,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

impl TryFrom<Media> for gatherer_core::gatherers::Media {
    type Error = String;

    fn try_from(media: Media) -> Result<Self, Self::Error> {
        if let Some(details) = media.details {
            if details.locations.is_empty() {
                return Err(format!("Content not available: {:?}", details));
            }
            let original_file_path = Path::new(&details.file_name);
            // debug!("The original upload file name was {:?}", original_file_path);
            let mut file_name = media.id;
            file_name += &original_file_path
                .extension()
                .map(|ext| format!(".{}", &ext.to_str().unwrap_or_default()))
                .unwrap_or_default();
            // debug!("Media ID filename is {}", file_name);
            Ok(Self {
                file_name,
                url: details.locations[0].location.to_string(),
                mime_type: details.mimetype,
                paid: media.purchased,
            })
        } else {
            Err(format!("Content not available: {:?}", media))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDetails {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: i64,
    pub status: i64,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub mimetype: String,
    #[serde(rename = "filename")]
    pub file_name: String,
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
    pub filename: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    pub locations: Vec<Location>,
}
