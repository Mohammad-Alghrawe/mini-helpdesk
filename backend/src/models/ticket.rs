use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
    pub status: TicketStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl Default for TicketPriority {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for TicketStatus {
    fn default() -> Self {
        Self::Open
    }
}
