/// Raw Hangouts.json models.
#[cfg(feature = "raw")]
pub mod raw;

mod event;

use std::collections::HashMap;

pub use crate::event::*;
pub use chrono;

use chrono::{DateTime, Utc};

/// Top-level struct for Hangouts data.
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

    /// Name of the conversation. Typically [`None`] if the conversation is one-on-one.
    pub name: Option<String>,

    /// List of current participants in the conversation.
    pub current_participants: Vec<ParticipantId>,
    /// List of all past and present participants in the conversation.
    pub participants: HashMap<ParticipantId, Participant>,
    /// List of conversation events.
    pub events: Vec<Event>,

    /// User state with regards to the conversation.
    pub self_state: SelfState,
    /// Timestamp used to sort conversations.
    pub sort_timestamp: DateTime<Utc>,

    /// Currently set group link sharing mode.
    pub group_link_sharing_status: LinkSharingStatus,
}

/// A participant in a conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Participant {
    /// Fallback name.
    pub fallback_name: Option<String>,
    /// Type of the participant.
    pub typ: Option<ParticipantType>,

    /// Invitation status for the participant.
    pub invitation_status: Option<InvitationStatus>,
    pub new_invitation_status: Option<InvitationStatus>,

    /// Read state for the participant.
    pub read_state: ReadState,
}

/// Composite ID for a participant user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ParticipantId {
    /// Gaia ID component.
    pub gaia_id: String,
    /// Chat ID component.
    pub chat_id: String,
}

/// Type of a given participant.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ParticipantType {
    Gaia,
    OffNetworkPhone,
}

/// Metadata regarding the user's status in a conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct SelfState {
    /// The user's involvement status.
    pub status: ConversationStatus,
    /// Currently set notification level.
    pub notification_level: NotificationLevel,
    /// Data about the invitation to the user for the conversation.
    pub invitation: InvitationData,
    /// Views the conversation are present in.
    //  TODO: Better name for this property
    pub views: Vec<View>,
}

/// Status of a participant's involvement in a conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ConversationStatus {
    /// The participant is active in the conversation.
    Active,
    /// The participant has been invited, but is not active in the conversation.
    Invited,
}

/// Invitation status of a participant.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum InvitationStatus {
    /// The invitation is pending.
    Pending,
    /// The invitation has been accepted.
    Accepted,
}

/// Metadata for the invitation to the user to join a conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct InvitationData {
    /// ID of the participant who created the invitation.
    pub inviter: ParticipantId,
    /// Time of invitation.
    pub timestamp: DateTime<Utc>,
    pub affinity: InvitationAffinity,
}

/// Affinity of the invitation.
//  TODO: not really sure what this does.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum InvitationAffinity {
    /// No invitation affinity specified.
    None,
    /// Low invitation affinity.
    Low,
    /// High invitation affinity.
    High,
}

/// Notification ring level for a participant in a conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum NotificationLevel {
    /// Notifications have been set to quiet.
    Quiet,
    /// Notifications have been set to ring.
    Ring,
}

/// Last-read information in a conversation for a participant.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ReadState {
    /// Last time of read.
    pub timestamp: DateTime<Utc>,
}

/// A Hangouts view collection.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum View {
    /// The hangout is visible in the inbox.
    Inbox,
    /// The hangout has been archived.
    Archived,
}

/// Link sharing enabled/disabled setting for a conversation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum LinkSharingStatus {
    /// Link sharing is disabled.
    Off,
    /// Link sharing is enabled.
    On,
}
