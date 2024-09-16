use dotenvy::dotenv;

mod models;
mod service;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // env_logger::init_from_env(
    //     env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    // );
    utils::logger::setup_logger()?;

    println!("Starting bot! --->>>>>>>");

    dotenv().expect(".env should be present");


    service::telegram_service::main();

    Ok(())
}