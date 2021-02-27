use crate::raw;
use crate::{Conversation, ConversationDetails, Hangouts, SelfConversationState};

use std::convert::TryFrom;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConversionError {}

impl TryFrom<raw::Hangouts> for Hangouts {
    type Error = ConversionError;

    #[inline]
    fn try_from(val: raw::Hangouts) -> Result<Self, Self::Error> {
        Ok(Self {
            conversations: val.conversations.into_iter().map(From::from).collect()?,
        })
    }
}

impl TryFrom<raw::Conversation> for Conversation {
    type Error = ConversionError;

    #[inline]
    fn try_from(val: raw::Conversation) -> Result<Self, Self::Error> {
        let conversation_id = val.header.conversation_id.id;
        let id = val.header.details.id.id;

        let details = match val.header.details.typ {
            raw::ConversationType::OneToOne => ConversationDetails::OneOnOne {},
            raw::ConversationType::Group => ConversationDetails::Group {
                name: val.header.details.name.unwrap(),
            },
        };

        let raw_self_conversation_state = val.header.details.self_conversation_state;

        let self_conversation_state = SelfConversationState {
            read_state: raw_self_conversation_state.self_read_state,
            status: raw_self_conversation_state.status,
            notification_level: raw_self_conversation_state.notification_level,
            views: raw_self_conversation_state.view,
            inviter: raw_self_conversation_state.inviter_id,
            invite_timestamp: raw_self_conversation_state.invite_timestamp,
        };

        Ok(Self {
            conversation_id,
            id,
            details,
            self_conversation_state,
        })
    }
}
