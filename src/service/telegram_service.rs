use log::{error, info};
use reqwest::Response;
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
    service::{movie_service, soccer_service},
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
        .branch(dptree::endpoint(handle_unknown_update))
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.set_chat_menu_button()
        .chat_id(msg.chat.id)
        .menu_button(teloxide::types::MenuButton::Commands)
        .await?;
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
                    .map(|service| InlineKeyboardButton::callback(service, service));

                bot.answer_callback_query(&q.id).await?;

                if let Some(message) = q.message {
                    bot.edit_message_text(message.chat().id, message.id(), "Select a service:")
                        .reply_markup(InlineKeyboardMarkup::new([buttons]))
                        .await?;
                } else {
                    bot.send_message(dialogue.chat_id(), "Select a service:")
                        .reply_markup(InlineKeyboardMarkup::new([buttons]))
                        .await?;
                }

                let new_state = match service.as_str() {
                    "Get Live Scores" => State::HandleSoccer { message: service },
                    "Get latest crypto charts" => State::HandleCrypto { message: service },
                    "top trending movies" => State::HandleMovie { message: service },
                    _ => {
                        log::warn!("Unrecognized service: {}", service);
                        State::Start // or some other appropriate state
                    }
                };

                dialogue.update(new_state).await?;
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
        log::info!("this is the message  >>>>>>>> {}", &service);

        match service.as_str() {
            "today event" => {
                match soccer_service::today_events().await {
                    Ok(events) => {
                        // Format and send the events data
                        // let format_events = format_events(events);
                        let message = "hjhbhhhhg".to_owned();
                        bot.send_message(dialogue.chat_id(), message).await?;
                    }
                    Err(err) => {
                        log::error!("Failed to fetch today's events: {}", err);
                        bot.send_message(
                            dialogue.chat_id(),
                            "Sorry, I couldn't fetch today's events. Please try again later.",
                        )
                        .await?;
                    }
                }
            }
            _ => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Sorry, I don't recognize that command. Please choose a valid option.",
                )
                .await?;
            }
        }
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

        match service.as_str() {
            "Top trending Movie" => match movie_service::trending_movie().await {
                Ok(response) => {
                    bot.send_message(dialogue.chat_id(), "Trending Movies".to_owned()).await?;
                    for movie in response {
                        bot.send_message(dialogue.chat_id(), movie).await?;
                    }
                }
                Err(err) => {
                    log::error!("Failed to fetch today's events: {}", err);
                    bot.send_message(
                        dialogue.chat_id(),
                        "Sorry, I couldn't fetch today's events. Please try again later.",
                    )
                    .await?;
                }
            },
            "Popular Movie" => match movie_service::popular_movie().await {
                Ok(response) => {
                    bot.send_message(dialogue.chat_id(), "Popular Movies: ".to_owned()).await?;
                    for movie in response {
                        bot.send_message(dialogue.chat_id(), movie).await?;
                    }
                }
                Err(err) => {
                    log::error!("Failed to fetch today's events: {}", err);
                    bot.send_message(
                        dialogue.chat_id(),
                        "Sorry, I couldn't fetch today's events. Please try again later.",
                    )
                    .await?;
                }
            },
            "Movies in Theatres" => match movie_service::get_movies_in_theatres().await {
                Ok(response) => {
                    bot.send_message(dialogue.chat_id(), "Get a list of movies that are currently in theatres.: ".to_owned()).await?;
                    for movie in response {
                        bot.send_message(dialogue.chat_id(), movie).await?;
                    }
                }
                Err(err) => {
                    log::error!("Failed to fetch today's events: {}", err);
                    bot.send_message(
                        dialogue.chat_id(),
                        "Sorry, I couldn't fetch today's events. Please try again later.",
                    )
                    .await?;
                }
            },
            "Upcoming Movie" => match movie_service::upcoming_movie().await {
                Ok(response) => {
                    bot.send_message(dialogue.chat_id(), "Get a list of movies that will be released soon: ".to_owned()).await?;
                    for movie in response {
                        bot.send_message(dialogue.chat_id(), movie).await?;
                    }
                }
                Err(err) => {
                    log::error!("Failed to fetch today's events: {}", err);
                    bot.send_message(
                        dialogue.chat_id(),
                        "Sorry, I couldn't fetch today's events. Please try again later.",
                    )
                    .await?;
                }
            },
            _ => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Sorry, I don't recognize that command. Please choose a valid option.",
                )
                .await?;
            }
        }
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

async fn handle_unknown_update(update: Update) -> HandlerResult {
    log::warn!("Received unknown update: {:?}", update);
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
