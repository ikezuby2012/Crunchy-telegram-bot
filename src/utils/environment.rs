use std::{cell::OnceCell, env, sync::OnceLock};

use teloxide::{payloads::SendMessageSetters, prelude::Requester, types::MessageId, Bot};

use crate::models::assets::{LogError, Vars};
pub static VARS: OnceLock<Vars> = OnceLock::new();

pub fn init_vars() -> Result<(), &'static str> {
    VARS.set(Vars::new())
        .map_err(|_| "Failed to initialize VARS")
}

// Send message to service chat without notification
pub async fn log(text: &str) -> Result<(), LogError> {
    let vars = VARS.get().ok_or(LogError::VarsNotInitialized)?;
    let chat = vars.get_chat().ok_or(LogError::ChatNotSet)?;

    let token = env::var("TELOXIDE_TOKEN").expect("expect teloxide token to be set");
    let bot = Bot::new(token);

    bot.send_message(chat, text)
        .disable_notification(true) // Assuming you want to disable notifications
        .await
        .map_err(|e| LogError::SendError(e))?; // Mapping the error to your custom LogError

    Ok(())
}
