use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    // "id": "316011312549208064",
    pub id: String,
    // "accountId": "285292618571587584",
    pub account_id: String,
    // "billingEmail": "arasureynn@gmail.com",
    pub billing_email: String,
    // "walletId": "285295465711284224",
    pub wallet_id: String,
    // "type": 14000,
    pub transaction_type: String,
    // "subscriptionId": null,
    pub subscription_id: Option<String>,
    // "correlationId": "316011312549208064",
    pub correlation_id: Option<String>,
    // "amount": 80000,
    pub amount: i32,
    // "threeDSecure": 1,
    pub three_d_secure: ThreeDSecureTransaction,
    // "rebill": 0,
    pub rebill: i8,
    // "rebillCorrelationId": null,
    pub rebill_correlation_id: Option<String>,
    // "status": 3,
    pub status: TransactionStatus,
    // "version": 2,
    pub version: i8,
    // "createdAt": 1636837328000,
    pub created_at: i64,
    // "updatedAt": 1636837331000
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionStatus {}

#[derive(Debug, Deserialize, Serialize)]
pub enum ThreeDSecureTransaction {}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTransaction {
    // "accountId": "285292618571587584",
    #[serde(rename = "accountId")]
    pub account_id: String,
    // "amount": 20000,
    pub ammount: String,
    // "correlationId": "315873778649280512",
    pub correlation_id: String,
    // "createdAt": 1636804540000,
    pub created_at: String,
    // "destination": 2,
    pub destination: String,
    // "destinationAmount": 20000,
    pub destination_amount: String,
    // "destinationTax": 0,
    pub destination_tax: i32,
    // "newBalance": 20001,
    pub new_balance: i32,
    // "receiverId": "285292618571587584"
    pub receiver_id: String,
    // "senderId": null,
    pub sender_id: Option<String>,
    // "status": 2,
    pub status: i32,
    // "transactionId": "315873791643230208",
    pub transaction_id: String,
    // "type": 14001,
    pub transaction_typpe: String,
    // "updatedAt": null,
    pub updated_at: String,
    // "walletId": "285295569348341760",
    pub wallet_id: String,
}
