use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build();

    // Create a file appender
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build("log/output.log")?;

    // Create the log configuration
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("console")
                .appender("file")
                .build(LevelFilter::Info),
        )?;
    // Initialize the logger
    log4rs::init_config(config)?;

    Ok(())
}
