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
    /// Link sharing is disabled.
    Off,
    /// Link sharing is enabled.
    On,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Event {
    /// ID of the event.
    pub id: String,
    /// Sender / initiatior of the event.
    pub sender: ParticipantId,
    /// Time of event.
    pub timestamp: DateTime<Utc>,

    /// Event-specific data.
    pub data: EventData,

    /// User's state with regards to the event.
    pub self_state: SelfEventState,
    /// Flag that indicates whether the event advances the sort timestamp of the conversation.
    pub advances_sort_timestamp: bool,

    pub version: u64,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct SelfEventState {
    /// Client generated ID value for the event.
    pub client_generated_id: Option<String>,
    /// Notification level for the event.
    pub notification_level: Option<NotificationLevel>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum EventData {
    /// A regular chat message.
    ChatMessage(ChatMessage),
    /// A hangout event.
    HangoutEvent(HangoutEvent),
    /// Conversation participant membership change.
    MembershipChange(MembershipChange),
    /// Conversation name change.
    ConversationRename(ConversationRename),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ChatMessage {
    pub contents: Vec<ChatSegment>,
    pub attachments: Vec<AttachmentSegment>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ChatSegment {
    Text {
        text: String,
        format: Formatting,
    },
    Link {
        text: String,
        target: String,
        display_url: Option<String>,
        format: Formatting,
    },
    LinkBreak {
        text: Option<String>,
        format: Formatting,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Formatting {
    pub bold: bool,
    pub italics: bool,
    pub strikethrough: bool,
    pub underline: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct AttachmentSegment {
    pub id: String,
    pub item: EmbedItem,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct EmbedItem {
    pub id: Option<String>,
    pub photo: Option<Photo>,
    pub place: Option<PlaceV2>,
    pub thing: Option<ThingV2>,

    // TODO: Not needed?
    pub types: Vec<EmbedItemType>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum EmbedItemType {
    Photo,
    Place,
    Thing,
    ThingV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Photo {
    pub media_type: MediaType,
    pub thumbnail: Thumbnail,

    pub album_id: String,
    pub photo_id: String,
    pub stream_id: Vec<String>,

    pub url: String,
    pub download_url: String,

    pub original_url: String,
    pub owner_obfuscated_id: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Thumbnail {
    pub height_px: u64,
    pub width_px: u64,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct PlaceV2 {
    pub url: String,
    pub name: Option<String>,

    pub address: Address,
    pub geo: Geo,
    pub representative_image: RepresentativeImage,

    pub place_id: Option<String>,
    pub cluster_id: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Address {
    pub name: Option<String>,
    pub street: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct RepresentativeImage {
    pub id: String,
    pub url: String,
    pub width: Option<u64>,
    pub height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ThingV2 {
    pub url: String,
    pub name: Option<String>,
    pub representative_image: RepresentativeImage,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct HangoutEvent {
    /// Type of the event.
    pub typ: HangoutEventType,
    pub media_type: Option<MediaType>,
    pub participants: Vec<ParticipantId>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum HangoutEventType {
    /// Signals the start of a hangout event.
    Start,
    /// Signals the end of a hangout event.
    End {
        /// Duration of hangout in seconds.
        duration: u64,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum MediaType {
    Audio,
    Video,
    AudioVideo,
    Photo,
    AnimatedPhoto,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct MembershipChange {
    pub typ: MembershipChangeType,
    pub participants: Vec<ParticipantId>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum MembershipChangeType {
    Join,
    Leave,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ConversationRename {
    /// Old conversation name.
    old: String,
    /// New conversation name.
    new: String,
}
