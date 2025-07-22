use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentCreatedDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub intent_hash: String,
    pub creator: String,
    pub source_chain_id: u64,
    pub destination_chain_id: u64,
    pub reward_token: String,
    pub reward_amount: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentFundedDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub intent_hash: String,
    pub funder: String,
    pub reward_token: String,
    pub reward_amount: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentPartiallyFundedDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub intent_hash: String,
    pub funder: String,
    pub reward_token: String,
    pub reward_amount: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub intent_hash: String,
    pub claimant: String,
    pub reward_token: String,
    pub reward_amount: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub intent_hash: String,
    pub recipient: String,
    pub reward_token: String,
    pub reward_amount: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentProofChallengedDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub intent_hash: String,
    pub challenger: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub fulfillment_hash: String,
    pub chain_id: u64,
    pub fulfiller: String,
    pub recipient: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderFilledDoc {
    #[serde(rename = "_id")]
    pub id: String,
    pub order_id: String,
    pub solver: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: u64,
}

// MongoDB Collection Names
pub const INTENT_CREATED_COLLECTION: &str = "intent_created";
pub const INTENT_FUNDED_COLLECTION: &str = "intent_funded";
pub const INTENT_PARTIALLY_FUNDED_COLLECTION: &str = "intent_partially_funded";
pub const WITHDRAWAL_COLLECTION: &str = "withdrawal";
pub const REFUND_COLLECTION: &str = "refund";
pub const INTENT_PROOF_CHALLENGED_COLLECTION: &str = "intent_proof_challenged";
pub const FULFILLMENT_COLLECTION: &str = "fulfillment";
pub const ORDER_FILLED_COLLECTION: &str = "order_filled";

// Index specifications for MongoDB
pub struct MongoIndexSpec {
    pub collection: &'static str,
    pub field: &'static str,
    pub unique: bool,
}

pub const MONGO_INDEXES: &[MongoIndexSpec] = &[
    // IntentCreated indexes
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "intent_hash", unique: true },
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "creator", unique: false },
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "source_chain_id", unique: false },
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "destination_chain_id", unique: false },
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "reward_token", unique: false },
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "block_number", unique: false },
    MongoIndexSpec { collection: INTENT_CREATED_COLLECTION, field: "timestamp", unique: false },
    
    // IntentFunded indexes
    MongoIndexSpec { collection: INTENT_FUNDED_COLLECTION, field: "intent_hash", unique: true },
    MongoIndexSpec { collection: INTENT_FUNDED_COLLECTION, field: "funder", unique: false },
    MongoIndexSpec { collection: INTENT_FUNDED_COLLECTION, field: "reward_token", unique: false },
    MongoIndexSpec { collection: INTENT_FUNDED_COLLECTION, field: "block_number", unique: false },
    
    // IntentPartiallyFunded indexes
    MongoIndexSpec { collection: INTENT_PARTIALLY_FUNDED_COLLECTION, field: "intent_hash", unique: false },
    MongoIndexSpec { collection: INTENT_PARTIALLY_FUNDED_COLLECTION, field: "funder", unique: false },
    MongoIndexSpec { collection: INTENT_PARTIALLY_FUNDED_COLLECTION, field: "reward_token", unique: false },
    MongoIndexSpec { collection: INTENT_PARTIALLY_FUNDED_COLLECTION, field: "block_number", unique: false },
    
    // Withdrawal indexes
    MongoIndexSpec { collection: WITHDRAWAL_COLLECTION, field: "intent_hash", unique: true },
    MongoIndexSpec { collection: WITHDRAWAL_COLLECTION, field: "claimant", unique: false },
    MongoIndexSpec { collection: WITHDRAWAL_COLLECTION, field: "reward_token", unique: false },
    MongoIndexSpec { collection: WITHDRAWAL_COLLECTION, field: "block_number", unique: false },
    
    // Refund indexes
    MongoIndexSpec { collection: REFUND_COLLECTION, field: "intent_hash", unique: true },
    MongoIndexSpec { collection: REFUND_COLLECTION, field: "recipient", unique: false },
    MongoIndexSpec { collection: REFUND_COLLECTION, field: "reward_token", unique: false },
    MongoIndexSpec { collection: REFUND_COLLECTION, field: "block_number", unique: false },
    
    // IntentProofChallenged indexes
    MongoIndexSpec { collection: INTENT_PROOF_CHALLENGED_COLLECTION, field: "intent_hash", unique: true },
    MongoIndexSpec { collection: INTENT_PROOF_CHALLENGED_COLLECTION, field: "challenger", unique: false },
    MongoIndexSpec { collection: INTENT_PROOF_CHALLENGED_COLLECTION, field: "block_number", unique: false },
    
    // Fulfillment indexes
    MongoIndexSpec { collection: FULFILLMENT_COLLECTION, field: "fulfillment_hash", unique: true },
    MongoIndexSpec { collection: FULFILLMENT_COLLECTION, field: "fulfiller", unique: false },
    MongoIndexSpec { collection: FULFILLMENT_COLLECTION, field: "recipient", unique: false },
    MongoIndexSpec { collection: FULFILLMENT_COLLECTION, field: "chain_id", unique: false },
    MongoIndexSpec { collection: FULFILLMENT_COLLECTION, field: "block_number", unique: false },
    
    // OrderFilled indexes
    MongoIndexSpec { collection: ORDER_FILLED_COLLECTION, field: "order_id", unique: true },
    MongoIndexSpec { collection: ORDER_FILLED_COLLECTION, field: "solver", unique: false },
    MongoIndexSpec { collection: ORDER_FILLED_COLLECTION, field: "block_number", unique: false },
];