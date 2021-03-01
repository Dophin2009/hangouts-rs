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

impl EventData {
    /// Returns `true` if the event is [`Self::ChatMessage`].
    #[inline]
    pub fn is_chat_message(&self) -> bool {
        matches!(self, Self::ChatMessage(..))
    }

    #[inline]
    pub fn as_chat_message(&self) -> Option<&ChatMessage> {
        if let Self::ChatMessage(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the event is [`Self::HangoutEvent`].
    #[inline]
    pub fn is_hangout_event(&self) -> bool {
        matches!(self, Self::HangoutEvent(..))
    }

    #[inline]
    pub fn as_hangout_event(&self) -> Option<&HangoutEvent> {
        if let Self::HangoutEvent(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the event is [`Self::MembershipChange`].
    #[inline]
    pub fn is_membership_change(&self) -> bool {
        matches!(self, Self::MembershipChange(..))
    }

    #[inline]
    pub fn as_membership_change(&self) -> Option<&MembershipChange> {
        if let Self::MembershipChange(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the event is [`Self::ConversationRename`].
    #[inline]
    pub fn is_conversation_rename(&self) -> bool {
        matches!(self, Self::ConversationRename(..))
    }

    #[inline]
    pub fn as_conversation_rename(&self) -> Option<&ConversationRename> {
        if let Self::ConversationRename(v) = self {
            Some(v)
        } else {
            None
        }
    }
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

impl ChatMessage {
    #[inline]
    pub fn contents_as_str(&self) -> String {
        self.contents
            .iter()
            .map(|seg| seg.text())
            .fold(String::new(), |mut acc, s| {
                acc.push_str(s);
                acc
            })
    }
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

impl ChatSegment {
    /// Returns the text string of the segment.
    #[inline]
    pub fn text(&self) -> &str {
        match self {
            Self::Text { text, .. } => &text,
            Self::Link { text, .. } => &text,
            Self::LinkBreak { text, .. } => match text {
                Some(text) => &text,
                None => "\n",
            },
        }
    }

    /// Returns the [`Formatting`] of the segment.
    #[inline]
    pub fn formatting(&self) -> &Formatting {
        match self {
            Self::Text { format, .. } => format,
            Self::Link { format, .. } => format,
            Self::LinkBreak { format, .. } => format,
        }
    }

    /// Returns `true` if the chat_segment is [`Self::Text`].
    #[inline]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    /// Returns `true` if the chat_segment is [`Self::Link`].
    #[inline]
    pub fn is_link(&self) -> bool {
        matches!(self, Self::Link { .. })
    }

    /// Returns `true` if the chat_segment is [`Self::LinkBreak`].
    #[inline]
    pub fn is_link_break(&self) -> bool {
        matches!(self, Self::LinkBreak { .. })
    }
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

impl HangoutEventType {
    /// Returns `true` if the event type is [`Self::Start`].
    #[inline]
    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    /// Returns `true` if the event type is [`Self::End`].
    #[inline]
    pub fn is_end(&self) -> bool {
        matches!(self, Self::End { .. })
    }
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

impl MediaType {
    /// Returns `true` if the media type is [`Self::Audio`].
    #[inline]
    pub fn is_audio(&self) -> bool {
        matches!(self, Self::Audio)
    }

    /// Returns `true` if the media type is [`Self::Video`].
    #[inline]
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video)
    }

    /// Returns `true` if the media type is [`Self::AudioVideo`].
    #[inline]
    pub fn is_audio_video(&self) -> bool {
        matches!(self, Self::AudioVideo)
    }

    /// Returns `true` if the media type is [`Self::Photo`].
    #[inline]
    pub fn is_photo(&self) -> bool {
        matches!(self, Self::Photo)
    }

    /// Returns `true` if the media type is [`Self::AnimatedPhoto`].
    #[inline]
    pub fn is_animated_photo(&self) -> bool {
        matches!(self, Self::AnimatedPhoto)
    }
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

impl MembershipChangeType {
    /// Returns `true` if the change type is [`Self::Join`].
    #[inline]
    pub fn is_join(&self) -> bool {
        matches!(self, Self::Join)
    }

    /// Returns `true` if the change type is [`Self::Leave`].
    #[inline]
    pub fn is_leave(&self) -> bool {
        matches!(self, Self::Leave)
    }
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
