use crate::async_openai::client::OpenAiClient;
use crate::consts::{PROMPT_MSG_DIET_WITHOUT_ARGS, PROMPT_MSG_DIET_WITH_ARGS};
use crate::db::database::Db;
use crate::db::models::Users;
use crate::errors::Result;
use crate::models::{DietCommands, MyDialogue, State};
use crate::utils::{format_prompt, make_keyboard};
use std::ops::DerefMut;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::Bot;
use tokio::sync::Mutex;

pub async fn add_diet(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    open_ai_client: Arc<Mutex<OpenAiClient>>,
    phone_number: String,
) -> Result<()> {
    log::info!("User {} is adding diet", phone_number);
    let mut db = db.lock().await;
    let user = db.get_user(&phone_number).await?;

    let response = process_diet(
        msg.clone(),
        open_ai_client,
        user,
        PROMPT_MSG_DIET_WITHOUT_ARGS,
        PROMPT_MSG_DIET_WITH_ARGS,
        db.deref_mut(),
    )
    .await?;

    log::info!("Getting response for user {}", phone_number);

    let keyboard = make_keyboard(vec![
        DietCommands::AddDiet.to_string(),
        DietCommands::ShowDiet.to_string(),
        DietCommands::DeleteDiet.to_string(),
        DietCommands::GoBack.to_string(),
    ]);

    bot.send_message(
        msg.chat.id,
        format!("Дієта додана! \n\n А ось і воно: \n\n {}", response),
    )
    .reply_markup(keyboard.resize_keyboard(true))
    .await?;

    dialogue.update(State::DietMenu { phone_number }).await?;
    Ok(())
}

pub async fn process_diet(
    msg: Message,
    open_ai_client: Arc<Mutex<OpenAiClient>>,
    user: Users,
    prompt_msg_without_args: &str,
    prompt_msg_with_args: &str,
    db: &mut Db,
) -> Result<String> {
    let response = if let Some(text) = msg.text() {
        let prompt = format_prompt(
            Some(text),
            prompt_msg_with_args,
            prompt_msg_without_args,
            user.clone(),
        );
        log::info!("Start sending prompt for diet {}!", prompt);
        open_ai_client.lock().await.send_message(&prompt).await?
    } else {
        let prompt = format_prompt(
            None,
            prompt_msg_with_args,
            prompt_msg_without_args,
            user.clone(),
        );
        log::info!("Start sending prompt for diet {}!", prompt);
        open_ai_client.lock().await.send_message(&prompt).await?
    };

    let user_id = user.id;
    db.insert_diet_list(user_id, &response).await?;
    Ok(response)
}

pub async fn show_diet(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    phone_number: String,
) -> Result<()> {
    log::info!("User {} is showing diet", phone_number);
    let mut db = db.lock().await;
    let user = db.get_user(&phone_number).await?;

    if let Ok(diet) = db.get_diet_list(user.id).await {
        let keyboard = make_keyboard(vec![
            DietCommands::AddDiet.to_string(),
            DietCommands::ShowDiet.to_string(),
            DietCommands::DeleteDiet.to_string(),
            DietCommands::GoBack.to_string(),
        ]);

        let diet: String = serde_json::from_value(diet.diet_list)?;

        bot.send_message(msg.chat.id, format!("Ось твоя дієта: \n\n {:?}", diet))
            .reply_markup(keyboard.resize_keyboard(true))
            .await?;

        dialogue.update(State::DietMenu { phone_number }).await?;
    } else {
        let keyboard = make_keyboard(vec![
            DietCommands::AddDiet.to_string(),
            DietCommands::ShowDiet.to_string(),
            DietCommands::DeleteDiet.to_string(),
            DietCommands::GoBack.to_string(),
        ]);

        bot.send_message(msg.chat.id, "Ти ще не додав дієту!".to_string())
            .reply_markup(keyboard.resize_keyboard(true))
            .await?;

        dialogue.update(State::DietMenu { phone_number }).await?;
    }
    Ok(())
}

pub async fn delete_diet(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    phone_number: String,
) -> Result<()> {
    log::info!("User {} is deleting diet", phone_number);
    let mut db = db.lock().await;
    let user = db.get_user(&phone_number).await?;

    db.delete_diet_list(user.id).await?;

    let keyboard = make_keyboard(vec![
        DietCommands::AddDiet.to_string(),
        DietCommands::ShowDiet.to_string(),
        DietCommands::DeleteDiet.to_string(),
        DietCommands::GoBack.to_string(),
    ]);

    bot.send_message(msg.chat.id, "Дієта видалена!".to_string())
        .reply_markup(keyboard.resize_keyboard(true))
        .await?;

    dialogue.update(State::DietMenu { phone_number }).await?;
    Ok(())
}
