use crate::async_openai::client::OpenAiClient;
use crate::db::database::Db;
use crate::errors::Result;
use crate::models::State;
use crate::utils::{init_logging, schema};
use dotenv::dotenv;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use tokio::sync::Mutex;

mod db;
mod models;
mod utils;

mod api_calls;
mod async_openai;
mod consts;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_logging()?;
    log::info!("Starting fitness bot...");

    let bot = Bot::new(dotenv::var("TELOXIDE_TOKEN")?);

    let database_url = dotenv::var("DATABASE_URL")?;

    let open_ai_token = dotenv::var("OPENAI_API_KEY")?;

    let open_ai_client = Arc::new(Mutex::new(OpenAiClient::new(open_ai_token)));
    let db = Arc::new(Mutex::new(Db::new(&database_url)));
    let state = Arc::new(State::Start);

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            Arc::clone(&db),
            Arc::clone(&state),
            Arc::clone(&open_ai_client)
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
