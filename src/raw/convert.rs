use crate::{
    raw, Address, AttachmentSegment, ChatMessage, ChatSegment, Conversation, ConversationRename,
    ConversationStatus, EmbedItem, Event, EventData, Formatting, Geo, HangoutEvent,
    HangoutEventType, Hangouts, InvitationAffinity, InvitationData, InvitationStatus,
    LinkSharingStatus, MediaType, MembershipChange, MembershipChangeType, NotificationLevel,
    Participant, ParticipantId, ParticipantType, Photo, PlaceV2, ReadState, RepresentativeImage,
    SelfEventState, SelfState, ThingV2, Thumbnail, View,
};

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::num::ParseIntError;

use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConversionError {
    ParseInt(#[from] ParseIntError),
}

impl fmt::Display for ConversionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::ParseInt(err) => write!(f, "iota: {}", err),
        }
    }
}

impl TryFrom<raw::Hangouts> for Hangouts {
    type Error = ConversionError;

    #[inline]
    fn try_from(val: raw::Hangouts) -> Result<Self, Self::Error> {
        let conversations = val
            .conversations
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self { conversations })
    }
}

impl TryFrom<raw::Conversation> for Conversation {
    type Error = ConversionError;

    #[inline]
    fn try_from(val: raw::Conversation) -> Result<Self, Self::Error> {
        let conversation_id = val.header.conversation_id.id;
        let id = val.header.details.id.id;

        let name = match val.header.details.typ {
            raw::ConversationType::OneToOne => None,
            raw::ConversationType::Group => val.header.details.name,
        };

        // Convert various status values.
        let self_conversation_state = val.header.details.self_conversation_state;
        let status = self_conversation_state.status.into();
        let notification_level = self_conversation_state.notification_level.into();
        let views = self_conversation_state
            .view
            .into_iter()
            .map(From::from)
            .collect();

        // Convert invitation data.
        let invitation = InvitationData {
            inviter: self_conversation_state.inviter_id.into(),
            timestamp: from_timestamp(self_conversation_state.invite_timestamp.parse()?),
            affinity: self_conversation_state.invite_affinity.into(),
        };

        let self_state = SelfState {
            invitation,
            notification_level,
            status,
            views,
        };

        let sort_timestamp = from_timestamp(self_conversation_state.sort_timestamp.parse()?);
        let group_link_sharing_status = val.header.details.group_link_sharing_status.into();

        // Collect participant read states.
        let mut read_states: HashMap<_, _> = val
            .header
            .details
            .read_state
            .into_iter()
            .map(|rs| -> Result<_, ParseIntError> {
                let timestamp = from_timestamp(rs.latest_read_timestamp.parse()?);
                Ok((rs.participant_id, ReadState { timestamp }))
            })
            .collect::<Result<_, _>>()?;

        // Convert participant data.
        let participants = val
            .header
            .details
            .participant_data
            .into_iter()
            .map(|pd| Participant {
                id: pd.id.clone().into(),
                typ: pd.participant_type.map(From::from),
                fallback_name: pd.fallback_name,
                invitation_status: pd.invitation_status.map(From::from),
                new_invitation_status: pd.new_invitation_status.map(From::from),
                read_state: read_states.remove(&pd.id).unwrap(),
            })
            .collect();

        let events = val
            .events
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            conversation_id,
            id,
            name,
            participants,
            events,
            self_state,
            sort_timestamp,
            group_link_sharing_status,
        })
    }
}

impl TryFrom<raw::Event> for Event {
    type Error = ConversionError;

    fn try_from(val: raw::Event) -> Result<Self, Self::Error> {
        let id = val.header.event_id;
        let sender = val.header.sender_id.into();
        let timestamp = from_timestamp(val.header.timestamp.parse()?);

        let data = match val.data {
            raw::EventData::ChatMessage {
                message_content,
                annotation: _annotation,
            } => EventData::ChatMessage(ChatMessage {
                contents: message_content
                    .segments
                    .into_iter()
                    .map(From::from)
                    .collect(),
                attachments: message_content
                    .attachments
                    .into_iter()
                    .map(From::from)
                    .collect(),
            }),
            raw::EventData::HangoutEvent {
                data,
                media_type,
                participant_id,
            } => EventData::HangoutEvent(HangoutEvent {
                typ: data.try_into()?,
                media_type: media_type.map(From::from),
                participants: participant_id.into_iter().map(From::from).collect(),
            }),
            raw::EventData::MembershipChange {
                typ,
                participant_id,
            } => EventData::MembershipChange(MembershipChange {
                typ: typ.into(),
                participants: participant_id.into_iter().map(From::from).collect(),
            }),
            raw::EventData::ConversationRename {
                new_name: new,
                old_name: old,
            } => EventData::ConversationRename(ConversationRename { new, old }),
        };

        let self_state = SelfEventState {
            client_generated_id: val.header.self_event_state.client_generated_id,
            notification_level: val
                .header
                .self_event_state
                .notification_level
                .map(From::from),
        };
        let advances_sort_timestamp = val.header.advances_sort_timestamp;
        let version = val.header.event_version.parse()?;

        Ok(Self {
            id,
            sender,
            timestamp,
            data,
            self_state,
            advances_sort_timestamp,
            version,
        })
    }
}

impl From<raw::ParticipantId> for ParticipantId {
    #[inline]
    fn from(val: raw::ParticipantId) -> Self {
        Self {
            gaia_id: val.gaia_id,
            chat_id: val.chat_id,
        }
    }
}

impl From<raw::ParticipantType> for ParticipantType {
    #[inline]
    fn from(val: raw::ParticipantType) -> Self {
        match val {
            raw::ParticipantType::Gaia => Self::Gaia,
            raw::ParticipantType::OffNetworkPhone => Self::OffNetworkPhone,
        }
    }
}

impl From<raw::ConversationStatus> for ConversationStatus {
    #[inline]
    fn from(val: raw::ConversationStatus) -> Self {
        match val {
            raw::ConversationStatus::Active => Self::Active,
            raw::ConversationStatus::Invited => Self::Invited,
        }
    }
}

impl From<raw::NotificationLevel> for NotificationLevel {
    #[inline]
    fn from(val: raw::NotificationLevel) -> Self {
        match val {
            raw::NotificationLevel::Quiet => Self::Quiet,
            raw::NotificationLevel::Ring => Self::Ring,
        }
    }
}

impl From<raw::View> for View {
    #[inline]
    fn from(val: raw::View) -> Self {
        match val {
            raw::View::Inbox => Self::Inbox,
            raw::View::Archived => Self::Archived,
        }
    }
}

impl From<raw::InvitationStatus> for InvitationStatus {
    #[inline]
    fn from(val: raw::InvitationStatus) -> Self {
        match val {
            raw::InvitationStatus::Pending => Self::Pending,
            raw::InvitationStatus::Accepted => Self::Accepted,
        }
    }
}

impl From<Option<raw::InvitationAffinity>> for InvitationAffinity {
    #[inline]
    fn from(val: Option<raw::InvitationAffinity>) -> Self {
        match val {
            Some(val) => match val {
                raw::InvitationAffinity::Low => Self::Low,
                raw::InvitationAffinity::High => Self::High,
            },
            None => Self::None,
        }
    }
}

impl From<raw::LinkSharingStatus> for LinkSharingStatus {
    #[inline]
    fn from(val: raw::LinkSharingStatus) -> Self {
        match val {
            raw::LinkSharingStatus::Off => Self::Off,
            raw::LinkSharingStatus::On => Self::On,
        }
    }
}

impl From<raw::ChatSegment> for ChatSegment {
    #[inline]
    fn from(val: raw::ChatSegment) -> Self {
        match val {
            raw::ChatSegment::Text { text, formatting } => Self::Text {
                text,
                format: formatting.into(),
            },
            raw::ChatSegment::Link {
                text,
                link_data,
                formatting,
            } => Self::Link {
                text,
                target: link_data.link_target,
                display_url: link_data.display_url,
                format: formatting.into(),
            },
            raw::ChatSegment::LineBreak { text, formatting } => Self::LinkBreak {
                text,
                format: formatting.into(),
            },
        }
    }
}

impl From<raw::AttachmentSegment> for AttachmentSegment {
    #[inline]
    fn from(val: raw::AttachmentSegment) -> Self {
        Self {
            id: val.id,
            item: val.embed_item.into(),
        }
    }
}

impl From<raw::EmbedItem> for EmbedItem {
    #[inline]
    fn from(val: raw::EmbedItem) -> Self {
        Self {
            id: val.id,
            photo: val.plus_photo.map(From::from),
            place: val.place_v2.map(From::from),
            thing: val.thing_v2.map(From::from),
        }
    }
}

impl From<raw::PlusPhoto> for Photo {
    #[inline]
    fn from(val: raw::PlusPhoto) -> Self {
        Self {
            media_type: val.media_type.into(),
            thumbnail: val.thumbnail.into(),
            album_id: val.album_id,
            photo_id: val.photo_id,
            stream_id: val.stream_id,
            url: val.url,
            download_url: val.download_url,
            original_url: val.original_content_url,
            owner_obfuscated_id: val.owner_obfuscated_id,
        }
    }
}

impl From<raw::Thumbnail> for Thumbnail {
    #[inline]
    fn from(val: raw::Thumbnail) -> Self {
        Self {
            url: val.url,
            height: val.height_px,
            width: val.width_px,
        }
    }
}

impl From<raw::PlaceV2> for PlaceV2 {
    #[inline]
    fn from(val: raw::PlaceV2) -> Self {
        Self {
            url: val.url,
            name: val.name,
            address: val.address.into(),
            geo: val.geo.into(),
            place_id: val.place_id,
            cluster_id: val.cluster_id,
            reference_id: val.reference_id,
            representative_image: val.representative_image.into(),
        }
    }
}

impl From<raw::Address> for Address {
    #[inline]
    fn from(val: raw::Address) -> Self {
        let postal = val.postal_address_v2;
        Self {
            name: postal.name,
            street: postal.street_address,
            locality: postal.address_locality,
            region: postal.address_region,
            country: postal.address_country,
            postal_code: postal.postal_code,
        }
    }
}

impl From<raw::Geo> for Geo {
    #[inline]
    fn from(val: raw::Geo) -> Self {
        Self {
            latitude: val.geo_coordinates_v2.latitude,
            longitude: val.geo_coordinates_v2.longitude,
        }
    }
}

impl From<raw::RepresentativeImage> for RepresentativeImage {
    #[inline]
    fn from(val: raw::RepresentativeImage) -> Self {
        let obj = val.image_object_v2;
        Self {
            id: val.id,
            url: obj.url,
            // TODO: Better error handling
            width: obj.width.map(|v| v.parse().unwrap()),
            height: obj.height.map(|v| v.parse().unwrap()),
        }
    }
}

impl From<raw::ThingV2> for ThingV2 {
    #[inline]
    fn from(val: raw::ThingV2) -> Self {
        Self {
            url: val.url,
            name: val.name,
            representative_image: val.representative_image.into(),
        }
    }
}

impl From<raw::Formatting> for Formatting {
    #[inline]
    fn from(val: raw::Formatting) -> Self {
        Self {
            bold: val.bold,
            italics: val.italics,
            strikethrough: val.strikethrough,
            underline: val.underline,
        }
    }
}

impl TryFrom<raw::HangoutEvent> for HangoutEventType {
    type Error = ParseIntError;
    #[inline]
    fn try_from(val: raw::HangoutEvent) -> Result<Self, Self::Error> {
        let typ = match val {
            raw::HangoutEvent::StartHangout => HangoutEventType::Start,
            raw::HangoutEvent::EndHangout {
                hangout_duration_secs,
            } => HangoutEventType::End {
                duration: hangout_duration_secs.parse()?,
            },
        };
        Ok(typ)
    }
}

impl From<raw::MediaType> for MediaType {
    #[inline]
    fn from(val: raw::MediaType) -> Self {
        match val {
            raw::MediaType::AudioOnly => Self::Audio,
            raw::MediaType::Video => Self::Video,
            raw::MediaType::AudioVideo => Self::AudioVideo,
            raw::MediaType::Photo => Self::Photo,
            raw::MediaType::AnimatedPhoto => Self::AnimatedPhoto,
        }
    }
}

impl From<raw::MembershipChangeType> for MembershipChangeType {
    #[inline]
    fn from(val: raw::MembershipChangeType) -> Self {
        match val {
            raw::MembershipChangeType::Join => Self::Join,
            raw::MembershipChangeType::Leave => Self::Leave,
        }
    }
}

#[inline]
fn from_timestamp(millisecs: i64) -> DateTime<Utc> {
    const MILLI: i64 = 10i64.pow(6);
    let secs = millisecs / MILLI;
    let milli = millisecs % MILLI;
    let naive = NaiveDateTime::from_timestamp(secs, (milli * 1000) as u32);
    DateTime::from_utc(naive, Utc)
}
