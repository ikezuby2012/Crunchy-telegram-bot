use serde::Deserialize;
use teloxide::utils::command::BotCommands;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    HandleConversation {
        message: String,
    },
    HandlePrompt {
        message: String,
    },
    HandleSoccer {
        message: String,
    },
    HandleMovie {
        message: String,
    },
    HandleCrypto {
        message: String,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    /// Display this text.
    Help,
    /// Start the purchase procedure.
    Start,
    /// Cancel the purchase procedure.
    Cancel,
}
