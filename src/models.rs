use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Inbox {
    pub id: u64,
    pub name: String,
    pub source: String,
    pub summary: Option<String>,
    pub payload: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateInboxRequest {
    pub inbox: CreateInboxBody,
}

#[derive(Debug, Serialize)]
pub struct CreateInboxBody {
    pub name: String,
    pub source: String,
    pub summary: String,
    pub payload: serde_json::Value,
    pub metadata: serde_json::Value,
}
