use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "billingEmail")]
    pub billing_email: Option<String>,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    #[serde(rename = "type")]
    pub datum_type: i64,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: Option<String>,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: i64,
    #[serde(rename = "threeDSecure")]
    pub three_d_secure: i64,
    pub rebill: i64,
    #[serde(rename = "rebillCorrelationId")]
    pub rebill_correlation_id: Option<String>,
    pub status: i64,
    pub version: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTransaction {
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(rename = "type")]
    pub datum_type: i64,
    pub destination: i64,
    pub amount: i64,
    #[serde(rename = "destinationTax")]
    pub destination_tax: i64,
    #[serde(rename = "destinationAmount")]
    pub destination_amount: Option<i64>,
    #[serde(rename = "newBalance")]
    pub new_balance: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    pub status: i64,
    #[serde(rename = "senderId")]
    pub sender_id: Option<String>,
    #[serde(rename = "receiverId")]
    pub receiver_id: Option<String>,
}
