use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageGroup {
    pub id: String,
    #[serde(rename = "type")]
    pub message_group_type: i64,
    #[serde(rename = "groupFlags")]
    pub group_flags: i64,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    pub users: Vec<MessageUser>,
    pub recipients: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageUser {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub user_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    #[serde(rename = "type")]
    pub message_type: Option<i64>,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub attachments: Vec<super::Attachment>,
    pub likes: Vec<Like>,
    #[serde(rename = "totalTipAmount")]
    pub total_tip_amount: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Like {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "type")]
    pub like_type: i64,
}
