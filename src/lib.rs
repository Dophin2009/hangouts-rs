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

impl Conversation {
    /// Get the data for all the current participants in the conversation.
    #[inline]
    pub fn current_participants(&self) -> Vec<&Participant> {
        self.current_participants
            .iter()
            .map(|id| self.participants.get(id).unwrap())
            .collect()
    }

    /// Sort the events by timestamp, from oldest to newest.
    #[inline]
    pub fn sort_events_by_time(&mut self) {
        self.events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
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

impl Participant {
    /// Get the name of the participant, if present.
    #[inline]
    pub fn name(&self) -> Option<&String> {
        self.fallback_name.as_ref()
    }
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

impl ParticipantType {
    /// Returns `true` if the participant type is [`Self::Gaia`].
    #[inline]
    pub fn is_gaia(&self) -> bool {
        matches!(self, Self::Gaia)
    }

    /// Returns `true` if the participant type is [`Self::OffNetworkPhone`].
    #[inline]
    pub fn is_off_network_phone(&self) -> bool {
        matches!(self, Self::OffNetworkPhone)
    }
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

impl ConversationStatus {
    /// Returns `true` if the status is [`Self::Active`].
    #[inline]
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if the status is [`Self::Invited`].
    #[inline]
    pub fn is_invited(&self) -> bool {
        matches!(self, Self::Invited)
    }
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

impl InvitationStatus {
    /// Returns `true` if the invitation status is [`Self::Pending`].
    #[inline]
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the invitation status is [`Self::Accepted`].
    #[inline]
    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }
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

impl InvitationAffinity {
    /// Returns `true` if the affinity is [`Self::None`].
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the affinity is [`Self::Low`].
    #[inline]
    pub fn is_low(&self) -> bool {
        matches!(self, Self::Low)
    }

    /// Returns `true` if the affinity is [`Self::High`].
    #[inline]
    pub fn is_high(&self) -> bool {
        matches!(self, Self::High)
    }
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

impl NotificationLevel {
    /// Returns `true` if the notification level is [`Self::Quiet`].
    #[inline]
    pub fn is_quiet(&self) -> bool {
        matches!(self, Self::Quiet)
    }

    /// Returns `true` if the notification level is [`Self::Ring`].
    #[inline]
    pub fn is_ring(&self) -> bool {
        matches!(self, Self::Ring)
    }
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

impl View {
    /// Returns `true` if the view is [`Self::Inbox`].
    #[inline]
    pub fn is_inbox(&self) -> bool {
        matches!(self, Self::Inbox)
    }

    /// Returns `true` if the view is [`Self::Archived`].
    #[inline]
    pub fn is_archived(&self) -> bool {
        matches!(self, Self::Archived)
    }
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

impl LinkSharingStatus {
    /// Returns `true` if the link_sharing_status is [`Self::Off`].
    #[inline]
    pub fn is_off(&self) -> bool {
        matches!(self, Self::Off)
    }

    /// Returns `true` if the link_sharing_status is [`Self::On`].
    #[inline]
    pub fn is_on(&self) -> bool {
        matches!(self, Self::On)
    }
}
