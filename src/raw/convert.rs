use crate::raw;
use crate::{
    Conversation, ConversationStatus, Hangouts, InvitationAffinity, InvitationData,
    LinkSharingStatus, NotificationLevel, ParticipantId, SelfState, View,
};

use std::convert::TryFrom;
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

        let participants = Vec::new();
        let events = Vec::new();

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

impl From<raw::ParticipantId> for ParticipantId {
    #[inline]
    fn from(val: raw::ParticipantId) -> Self {
        Self {
            gaia_id: val.gaia_id,
            chat_id: val.chat_id,
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

#[inline]
fn from_timestamp(nano: i64) -> DateTime<Utc> {
    let secs = nano / 10i64.pow(9);
    let rem = nano % 10i64.pow(9);
    let naive = NaiveDateTime::from_timestamp(secs, rem as u32);
    DateTime::from_utc(naive, Utc)
}
