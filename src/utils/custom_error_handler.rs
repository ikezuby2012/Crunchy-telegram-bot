use std::fmt::Debug;
use teloxide::error_handlers::ErrorHandler;

use crate::utils::environment;

pub struct CustomErrorHandler {}

impl<E> ErrorHandler<E> for CustomErrorHandler
where
    E: Debug,
{
    fn handle_error(
        self: std::sync::Arc<Self>,
        error: E,
    ) -> futures::future::BoxFuture<'static, ()> {
        let text = format!("main::handle::error: {:?}", error);
        log::error!("{}", text);

        let fut = async move {
            if environment::log(&text).await.is_err() {
                log::info!("main::Unable to send message to the service chat");
            }
        };

        Box::pin(fut)
    }
}
