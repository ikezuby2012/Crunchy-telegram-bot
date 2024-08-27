use log::{error, info};
use rust_decimal::prelude::*;
use std::env;
use std::{path::Path, process::Command};
use tokio::{task, time};

use teloxide::{
    dispatching::{
        dialogue::{self, GetChatId, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use crate::models::{assets, orders::Command as OtherCommand, orders::State};
use crate::service::gpt_service::gpt_quick_reply;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn main() {
    let token = env::var("TELOXIDE_TOKEN").expect("expect teloxide token to be set");
    info!(">>>>>>>>>>>>>>>>>>> starting Bot <<<<<<<<<<<<<<<<<");
    
    let bot_task = task::spawn(async move {
        let bot = Bot::new(token);
        let handler = dptree::entry()
            .branch(schema());  // Assuming schema() is defined elsewhere

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await
    });

    match bot_task.await {
        Ok(_) => info!("Bot task completed successfully"),
        Err(e) => error!("Bot task error: {}", e),
    }
}


pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<OtherCommand, _>().branch(
        case![State::Start]
            .branch(case![OtherCommand::Help].endpoint(help))
            .branch(case![OtherCommand::Start].endpoint(start))
            .branch(case![OtherCommand::Cancel].endpoint(cancel)),
    );

    let message_handler = Update::filter_message().branch(command_handler).branch(
        case![State::ReceiveFullName]
            .endpoint(receive_full_name)
            .branch(dptree::endpoint(invalid_state)),
    );

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::HandleConversation { message }].endpoint(handle_conversation));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "let's start! what's your name")
        .await?;
    dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

pub async fn help(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, OtherCommand::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "unable to handle the message. type /helo to start",
    )
    .await?;
    Ok(())
}

pub async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(full_name) => {
            // do something

            let service = [
                "Get Live Scores",
                "Get latest crypto charts",
                "top trending movies",
            ]
            .map(|service| InlineKeyboardButton::callback(service, service));

            bot.send_message(msg.chat.id, "select a service:")
                .reply_markup(InlineKeyboardMarkup::new([service]))
                .await?;

            dialogue
                .update(State::HandleConversation { message: full_name })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "please send me your full name")
                .await?;
        }
    }
    Ok(())
}

pub async fn handle_conversation(
    bot: Bot,
    dialogue: MyDialogue,
    message: String,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(service) = q.data {
        match gpt_quick_reply(&service).await {
            Ok(conversation_reply) => {
                bot.answer_callback_query(&q.id).await?;
                bot.send_message(dialogue.chat_id(), conversation_reply).await?;
            }
            Err(e) => {
                dialogue.exit().await?;
                bot.answer_callback_query(&q.id).await?;
                eprintln!("something went wrong : {}", e);
            }
        }

    }
    Ok(())
}
