use log::{error, info};
use std::env;
use tokio::task;

use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup},
    utils::command::BotCommands,
};

use crate::service::gpt_service::gpt_quick_reply;
use crate::utils::environment::init_vars;
use crate::{
    models::{
        assets::MessageError,
        orders::{Command as OtherCommand, State},
    },
    service::soccer_service,
    utils::{custom_error_handler::CustomErrorHandler, data::PROMPT_DATA},
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub async fn main() {
    let token = env::var("TELOXIDE_TOKEN").expect("expect teloxide token to be set");
    info!(">>>>>>>>>>>>>>>>>>> starting Bot <<<<<<<<<<<<<<<<<");

    init_vars().expect("Failed to initialize VARS");

    let bot_task = task::spawn(async move {
        let bot = Bot::new(token);
        let handler = dptree::entry().branch(schema()); // Assuming schema() is defined elsewhere

        Dispatcher::builder(bot, handler)
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            //  .error_handler(Arc::new(CustomErrorHandler{}))
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
        .branch(case![State::HandleConversation { message }].endpoint(handle_prompt))
        .branch(case![State::HandleSoccer { message }].endpoint(handle_soccer))
        .branch(case![State::HandleCrypto { message }].endpoint(handle_crypto))
        .branch(case![State::HandleMovie { message }].endpoint(handle_movie));

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
        "unable to handle the message. type /hello to start",
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

fn handle_message(message: &String) -> Result<Vec<&'static str>, MessageError> {
    PROMPT_DATA
        .get(message)
        .map(|result| {
            log::info!("data for {}: {:?}", message, result);
            result.to_vec()
        })
        .ok_or_else(|| MessageError::NoDataFound(message.to_string()))
}

pub async fn handle_prompt(
    bot: Bot,
    dialogue: MyDialogue,
    message: String,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(service) = q.data {
        log::info!("this is the message {}", &service);

        match handle_message(&service) {
            Ok(prompts) => {
                let buttons = prompts
                    .into_iter()
                    .map(|service| KeyboardButton::new(service));

                bot.answer_callback_query(&q.id).await?;
                bot.send_message(dialogue.chat_id(), "select a service:")
                    .reply_markup(KeyboardMarkup::new([buttons]))
                    .await?;

                if service == "Get Live Scores" {
                    dialogue
                        .update(State::HandleSoccer { message: service })
                        .await?;
                } else if service == "Get latest crypto charts" {
                    dialogue
                        .update(State::HandleCrypto { message: service })
                        .await?;
                } else if service == "top trending movies" {
                    dialogue
                        .update(State::HandleMovie { message: service })
                        .await?;
                }
            }
            Err(err) => {
                log::error!(
                    "failed to recognize prompt: {:?}, the error message is {err}",
                    &service
                );

                dialogue.exit().await?;
                // bot.answer_callback_query(&q.id).await?;
            }
        }
    }
    Ok(())
}

#[warn(unused_variables)]
pub async fn handle_soccer(
    bot: Bot,
    dialogue: MyDialogue,
    message: String,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(service) = q.data {
        log::info!("this is the message {}", &service);
    }
    Ok(())
}

#[warn(unused_variables)]
pub async fn handle_movie(
    bot: Bot,
    dialogue: MyDialogue,
    message: String,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(service) = q.data {
        log::info!("this is the message {}", &service);
    }
    Ok(())
}

#[warn(unused_variables)]
pub async fn handle_crypto(
    bot: Bot,
    dialogue: MyDialogue,
    message: String,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(service) = q.data {
        log::info!("this is the message {}", &service);
    }
    Ok(())
}

#[warn(unused_variables)]
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
                bot.send_message(dialogue.chat_id(), conversation_reply)
                    .await?;
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
