use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, Message, MessageObject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImageMessage {
    #[serde(rename = "type")]
    #[builder(default = "image".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub original_content_url: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub preview_image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<ImageMessage> for MessageObject {
    fn from(message: ImageMessage) -> Self {
        MessageObject::Image(message)
    }
}

/* impl Message<'_> for ImageMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl ImageMessage {
    pub fn new(original_content_url: String, preview_image_url: String) -> Self {
        ImageMessage {
            type_field: "image".to_string(),
            original_content_url,
            preview_image_url,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn build(self) -> super::MessageObject {
        super::MessageObject::Image(self)
    }
} */
