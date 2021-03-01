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

/// A single conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Conversation {
    pub conversation_id: String,
    pub id: String,
    pub name: Option<String>,

    /// List of participants.
    pub participants: Vec<Participant>,

    /// List of conversation events.
    pub events: Vec<Event>,

    pub self_state: SelfState,
    pub sort_timestamp: DateTime<Utc>,
    pub group_link_sharing_status: LinkSharingStatus,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Participant {
    pub id: ParticipantId,

    pub fallback_name: Option<String>,

    pub typ: Option<ParticipantType>,
    pub invitation_status: Option<InvitationStatus>,
    pub new_invitation_status: Option<InvitationStatus>,

    /// Read state for the participant.
    pub read_state: ReadState,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct SelfState {
    pub status: ConversationStatus,
    pub notification_level: NotificationLevel,
    pub invitation: InvitationData,
    // TODO: Better name for this property
    pub views: Vec<View>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum InvitationStatus {
    Pending,
    Accepted,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct InvitationData {
    /// ID of the participant who created the invitation.
    pub inviter: ParticipantId,
    /// Time of invitation.
    pub timestamp: DateTime<Utc>,
    pub affinity: InvitationAffinity,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum InvitationAffinity {
    None,
    Low,
    High,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ParticipantId {
    pub gaia_id: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ParticipantType {
    Gaia,
    OffNetworkPhone,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ReadState {
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum View {
    Inbox,
    Archived,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum LinkSharingStatus {
    Off,
    On,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Event {}
