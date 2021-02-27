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

    pub details: ConversationDetails,
    pub self_conversation_state: SelfConversationState,
}

impl Conversation {
    /// Return the type of the conversation.
    #[inline]
    pub fn typ(&self) -> ConversationType {
        match &self.details {
            ConversationDetails::OneOnOne { .. } => ConversationType::OneToOne,
            ConversationDetails::Group { .. } => ConversationType::Group,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ConversationDetails {
    OneOnOne {},
    Group { name: String },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct GroupConversation {}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct SelfConversationState {
    pub read_state: ReadState,
    pub status: ConversationStatus,
    pub notification_level: NotificationLevel,
    pub views: Vec<View>,

    pub inviter: ParticipantId,
    pub invite_timestamp: DateTime<Utc>,
    pub sort_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ConversationStatus {
    Active,
    Invited,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ConversationType {
    Group,
    OneToOne,
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
    pub latest_read_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum View {
    Inbox,
    Archived,
}
