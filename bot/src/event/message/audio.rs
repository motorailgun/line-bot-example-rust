use line_bot_sdk::{
    error::AppError,
    models::{message::text::TextMessage, message::MessageObject, webhook_event::Audio},
};

pub fn handler(message: &Audio) -> Result<Option<Vec<MessageObject>>, AppError> {
    Ok(Some(vec![TextMessage::builder()
        .text(&format!(
            "音声を受け取りました！\nメッセージID: {}",
            message.id
        ))
        .build()
        .into()]))
}
