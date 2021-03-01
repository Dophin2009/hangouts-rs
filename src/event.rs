use chrono::{DateTime, Utc};

use crate::{NotificationLevel, ParticipantId};

/// A single event in a conversation.
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

/// Metadata for the user's state with regards to an event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct SelfEventState {
    /// Client generated ID value for the event.
    pub client_generated_id: Option<String>,
    /// Notification level for the event.
    pub notification_level: Option<NotificationLevel>,
}

/// Data body of an event, which can be one of four types.
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

/// A regular chat message event body.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ChatMessage {
    /// The textual contents of the message.
    pub contents: Vec<ChatSegment>,
    /// List of attachments included in the message.
    pub attachments: Vec<AttachmentSegment>,
}

/// A segment of a chat message.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ChatSegment {
    /// A pure-text message segment.
    Text {
        /// Textual content of the segment.
        text: String,
        /// Formatting of the content.
        format: Formatting,
    },
    /// A link segment.
    Link {
        /// Text displayed as part of the message.
        text: String,
        /// Target location of the url.
        target: String,
        display_url: Option<String>,
        /// Formatting of the text content.
        format: Formatting,
    },
    /// A line-break in the middle of a message.
    LinkBreak {
        /// Raw text of the line-break.
        text: Option<String>,
        /// Format of the text content.
        format: Formatting,
    },
}

/// Formatting data for a text segment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Formatting {
    /// Indicates whether the text is bold or not.
    pub bold: bool,
    /// Indicates whether the text is italicized or not.
    pub italics: bool,
    /// Indicates whether the text is strikethrough or not.
    pub strikethrough: bool,
    /// Indicates whether the text is underlined or not.
    pub underline: bool,
}

/// An attachment segment in a chat message.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct AttachmentSegment {
    /// ID of the attachment.
    pub id: String,
    /// Attachment item data.
    pub item: EmbedItem,
}

/// Data about an attachment item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct EmbedItem {
    /// ID of the embedded item.
    pub id: Option<String>,
    /// Metadata for a photo attachment item, if present.
    pub photo: Option<Photo>,
    /// Metadata for a place_v2 attachment item, if present.
    pub place: Option<PlaceV2>,
    /// Metadata for a thing_v2 attachment item, if present.
    pub thing: Option<ThingV2>,
}

/// Data about a photo or video attachment item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Photo {
    /// The type of media attached.
    pub media_type: MediaType,
    /// Thumbnail data for the media.
    pub thumbnail: Thumbnail,

    /// ID of the photos album the media has been saved to.
    pub album_id: String,
    /// ID the media has been assigned.
    pub photo_id: String,
    pub stream_id: Vec<String>,

    /// URL to the media.
    pub url: String,
    /// URL that can be used to download the media.
    pub download_url: Option<String>,

    pub original_url: String,
    pub owner_obfuscated_id: String,
}

/// Thumbnail data for a media item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Thumbnail {
    /// Height of the thumbnail.
    pub height: u64,
    /// Width of the thumbnail.
    pub width: u64,
    /// URL to the thumbnail.
    pub url: Option<String>,
}

/// Data about a place_v2 item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct PlaceV2 {
    /// Google URL to the place.
    pub url: String,
    /// Optional name of the place.
    pub name: Option<String>,

    /// Full postal address of the place.
    pub address: Address,
    /// Coordinates of the place.
    pub geo: Geo,
    /// Data for a representative image of the place.
    pub representative_image: RepresentativeImage,

    pub place_id: Option<String>,
    pub cluster_id: Option<String>,
    pub reference_id: Option<String>,
}

/// A struct representing a postal address.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Address {
    /// Name of the address.
    pub name: Option<String>,
    /// Street address.
    pub street: Option<String>,
    /// Locality of the address.
    pub locality: Option<String>,
    /// Region of the address.
    pub region: Option<String>,
    /// Country of the address.
    pub country: Option<String>,
    /// Postal code for region of the address.
    pub postal_code: Option<String>,
}

/// A pair of latitude and longitude values.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}

/// Data about a representative image for a place or thing.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct RepresentativeImage {
    /// ID of the image.
    pub id: String,
    /// URL to the image.
    pub url: String,
    /// Optional pixel width of the image.
    pub width: Option<u64>,
    /// Optional pixel height of the image.
    pub height: Option<u64>,
}

/// Data about a thing_v2 attachment item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ThingV2 {
    /// URL to the thing.
    pub url: String,
    /// Optional name of the thing.
    pub name: Option<String>,
    /// Data for the representative image of the thing.
    pub representative_image: RepresentativeImage,
}

/// A Hangout call event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct HangoutEvent {
    /// Type of the event.
    pub typ: HangoutEventType,
    /// The media type of the hangout.
    pub media_type: Option<MediaType>,
    /// List of participants involved in the hangout.
    pub participants: Vec<ParticipantId>,
}

/// The type of a hangout call event, signalling the start or end of the event.
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

/// The type of a media attachment or hangout event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum MediaType {
    /// Audio-only.
    Audio,
    /// Video-only.
    Video,
    /// Both audio and video.
    AudioVideo,
    /// Photo.
    Photo,
    /// Animated photo, such as a gif.
    AnimatedPhoto,
}

/// A conversation participant membership change event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct MembershipChange {
    /// The type of membership change (join or leave).
    pub typ: MembershipChangeType,
    /// List of participants involved in the change.
    pub participants: Vec<ParticipantId>,
}

/// Type of a membership change.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum MembershipChangeType {
    /// Participants joined the conversation.
    Join,
    /// Participants left the conversation.
    Leave,
}

/// A conversation rename event.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub struct ConversationRename {
    /// Old conversation name.
    pub old: String,
    /// New conversation name.
    pub new: String,
}
