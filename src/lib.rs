/// Raw Hangouts.json models.
#[cfg(feature = "raw")]
pub mod raw;

pub use chrono;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Hangouts {
    /// List of conversations.
    pub conversations: Vec<Conversation>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Conversation {
    pub conversation_id: String,
    pub id: String,

    pub typ: ConversationType,
    pub participants: Vec<Participant>,

    // pub read_state: ReadState,
    pub status: ConversationStatus,
    pub notification_level: NotificationLevel,
    // TODO: Better name for this property
    pub views: Vec<View>,
    pub invitation: InvitationData,

    pub sort_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Participant {}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ConversationType {
    Group { name: String },
    OneOnOne,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct InvitationData {
    pub inviter: ParticipantId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ConversationStatus {
    Active,
    Invited,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum NotificationLevel {
    Quiet,
    Ring,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ParticipantId {
    pub gaia_id: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ReadState {
    pub participant_id: ParticipantId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum View {
    Inbox,
    Archived,
}
