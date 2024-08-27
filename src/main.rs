use dotenvy::dotenv;

mod models;
mod service;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    // pretty_env_logger::init();
    // log::info!("Starting purchase bot...");
    println!("Starting bot! --->>>>>>>");

    dotenv().expect(".env should be present");

    service::telegram_service::main();
}
