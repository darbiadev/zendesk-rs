use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketWrapper {
    pub ticket: Ticket,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub url: String,
    pub id: i64,
    pub external_id: Option<String>,
    pub via: Via,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub subject: String,
    pub raw_subject: String,
    pub description: String,
    pub priority: Option<String>,
    pub status: String,
    pub recipient: String,
    pub requester_id: i64,
    pub submitter_id: i64,
    pub assignee_id: i64,
    pub organization_id: Option<i64>,
    pub group_id: i64,
    pub collaborator_ids: Vec<i64>,
    pub follower_ids: Vec<i64>,
    pub email_cc_ids: Vec<i64>,
    pub forum_topic_id: Option<String>,
    pub problem_id: Option<i64>,
    pub has_incidents: bool,
    pub is_public: bool,
    pub due_at: Option<String>,
    pub tags: Vec<String>,
    pub custom_fields: Vec<String>,
    pub satisfaction_rating: Option<SatisfactionRating>,
    pub sharing_agreement_ids: Vec<i64>,
    pub fields: Vec<String>,
    pub followup_ids: Vec<i64>,
    pub brand_id: i64,
    pub allow_channelback: bool,
    pub allow_attachments: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Via {
    pub channel: String,
    pub source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub from: Address,
    pub to: Address,
    pub rel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SatisfactionRating {
    pub id: i64,
    pub score: String,
    pub comment: String,
}
