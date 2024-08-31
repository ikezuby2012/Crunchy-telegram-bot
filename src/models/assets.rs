use teloxide::{types::{ChatId, Recipient}, Bot};
use thiserror::Error;

// For send info to service chat
#[derive(Clone)]
struct ServiceChat {
    recipient: Recipient,
    bot: Bot,
}

pub struct Vars {
    // Service chat
    chat: Option<ChatId>,
}

impl Vars {
    pub fn new() -> Self {
        Self { chat: None }
    }
    pub fn set_chat(&mut self, chat: ChatId) {
        self.chat = Some(chat);
    }

    pub fn get_chat(&self) -> Option<ChatId> {
        self.chat
    }
}

#[derive(Error, Debug)]
pub enum LogError {
    #[error("VARS is not initialized")]
    VarsNotInitialized,
    #[error("Chat is not set")]
    ChatNotSet,
    #[error("Failed to send message: {0}")]
    SendError(#[from] teloxide::RequestError),
}

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("No data found for message: {0}")]
    NoDataFound(String),
}