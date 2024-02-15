use crate::*;

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum SlackStarsItem {
    Message {
        message: SlackHistoryMessage,
        channel: SlackChannelId,
        date_create: SlackDateTime,
    },
    File {
        file: SlackFile,
        channel: SlackChannelId,
        date_create: SlackDateTime,
    },
    #[serde(rename = "file_comment")]
    FileComment {
        file: SlackFile,
        comment: String,
        channel: SlackChannelId,
        date_create: SlackDateTime,
    },
    Channel {
        channel: SlackChannelId,
        date_create: SlackDateTime,
    },
    Im {
        channel: SlackChannelId,
        date_create: SlackDateTime,
    },
    Group {
        group: SlackChannelId,
        date_create: SlackDateTime,
    },
}
