use crate::raw;
use crate::{
    Conversation, ConversationStatus, ConversationType, Hangouts, InvitationData,
    NotificationLevel, ParticipantId, View,
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

        let typ = match val.header.details.typ {
            raw::ConversationType::OneToOne => ConversationType::OneOnOne {},
            raw::ConversationType::Group => ConversationType::Group {
                name: val.header.details.name.unwrap(),
            },
        };

        let self_conversation_state = val.header.details.self_conversation_state;
        let status = self_conversation_state.status.into();
        let notification_level = self_conversation_state.notification_level.into();
        let views = self_conversation_state
            .view
            .into_iter()
            .map(From::from)
            .collect();

        let invitation = InvitationData {
            inviter: self_conversation_state.inviter_id.into(),
            timestamp: from_timestamp(self_conversation_state.invite_timestamp.parse()?),
        };

        let sort_timestamp = from_timestamp(self_conversation_state.sort_timestamp.parse()?);

        Ok(Self {
            conversation_id,
            id,
            typ,
            status,
            notification_level,
            views,
            invitation,
            sort_timestamp,
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

#[inline]
fn from_timestamp(secs: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(secs, 0);
    DateTime::from_utc(naive, Utc)
}
