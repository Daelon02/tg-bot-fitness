use crate::async_openai::client::OpenAiClient;
use crate::consts::{
    GYM_STATE, HOME_STATE, PROMPT_MSG_GYM_TRAINING_WITHOUT_ARGS, PROMPT_MSG_GYM_TRAINING_WITH_ARGS,
    PROMPT_MSG_HOME_TRAINING_WITHOUT_ARGS, PROMPT_MSG_HOME_TRAINING_WITH_ARGS,
};
use crate::db::database::Db;
use crate::db::models::Users;
use crate::errors::Result;
use crate::models::{MyDialogue, State, TrainingsCommands};
use crate::utils::{format_prompt, make_keyboard};
use std::ops::DerefMut;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::Bot;
use tokio::sync::Mutex;

pub async fn add_training(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    open_ai_client: Arc<Mutex<OpenAiClient>>,
    (phone_number, training_state): (String, String),
) -> Result<()> {
    log::info!("User {} is adding training", phone_number);
    let mut db = db.lock().await;
    let user = db.get_user(&phone_number).await?;

    let response = if training_state == HOME_STATE {
        process_training(
            msg.clone(),
            open_ai_client,
            user,
            PROMPT_MSG_HOME_TRAINING_WITHOUT_ARGS,
            PROMPT_MSG_HOME_TRAINING_WITH_ARGS,
            db.deref_mut(),
            HOME_STATE.to_string(),
        )
        .await?
    } else {
        process_training(
            msg.clone(),
            open_ai_client,
            user,
            PROMPT_MSG_GYM_TRAINING_WITHOUT_ARGS,
            PROMPT_MSG_GYM_TRAINING_WITH_ARGS,
            db.deref_mut(),
            GYM_STATE.to_string(),
        )
        .await?
    };

    log::info!("Getting response for user {}", phone_number);

    let keyboard = make_keyboard(vec![
        TrainingsCommands::AddTraining.to_string(),
        TrainingsCommands::DeleteTraining.to_string(),
        TrainingsCommands::ShowTrainings.to_string(),
        TrainingsCommands::GoBack.to_string(),
    ]);

    bot.send_message(
        msg.chat.id,
        format!("Тренування додано! \n\n А ось і воно: \n\n {}", response),
    )
    .reply_markup(keyboard.resize_keyboard(true))
    .await?;

    if training_state == HOME_STATE {
        dialogue
            .update(State::HomeTrainingMenu { phone_number })
            .await?;
    } else {
        dialogue
            .update(State::GymTrainingMenu { phone_number })
            .await?;
    }

    Ok(())
}

pub async fn show_trainings(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    (phone_number, training_state): (String, String),
) -> Result<()> {
    log::info!("User {} is showing training", phone_number);
    let mut db = db.lock().await;
    let user = db.get_user(&phone_number).await?;
    let user_id = user.id;
    let trainings = if training_state == HOME_STATE {
        db.get_training(user_id, HOME_STATE.to_string()).await
    } else {
        db.get_training(user_id, GYM_STATE.to_string()).await
    };

    match trainings {
        Ok(trainings) => {
            let keyboard = make_keyboard(vec![
                TrainingsCommands::AddTraining.to_string(),
                TrainingsCommands::DeleteTraining.to_string(),
                TrainingsCommands::ShowTrainings.to_string(),
                TrainingsCommands::GoBack.to_string(),
            ]);

            let trainings: String = serde_json::from_value(trainings.trainings)?;

            bot.send_message(
                msg.chat.id,
                format!("Ось твоє тренування: \n\n {}", trainings),
            )
            .reply_markup(keyboard.resize_keyboard(true))
            .await?;
            dialogue
                .update(State::HomeTrainingMenu { phone_number })
                .await?;
        }
        Err(_) => {
            let keyboard = make_keyboard(vec![
                TrainingsCommands::AddTraining.to_string(),
                TrainingsCommands::DeleteTraining.to_string(),
                TrainingsCommands::ShowTrainings.to_string(),
                TrainingsCommands::GoBack.to_string(),
            ]);

            bot.send_message(msg.chat.id, "Тренування відсутнє!".to_string())
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
            dialogue
                .update(State::HomeTrainingMenu { phone_number })
                .await?;
        }
    }
    Ok(())
}

pub async fn delete_training(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    (phone_number, training_status): (String, String),
) -> Result<()> {
    log::info!("User {} is deleting training", phone_number);
    let mut db = db.lock().await;
    let user = db.get_user(&phone_number).await?;
    let user_id = user.id;

    let result = if training_status == HOME_STATE {
        db.delete_training(user_id, HOME_STATE.to_string()).await
    } else {
        db.delete_training(user_id, GYM_STATE.to_string()).await
    };

    match result {
        Ok(_) => {
            let keyboard = make_keyboard(vec![
                TrainingsCommands::AddTraining.to_string(),
                TrainingsCommands::DeleteTraining.to_string(),
                TrainingsCommands::ShowTrainings.to_string(),
                TrainingsCommands::GoBack.to_string(),
            ]);

            bot.send_message(msg.chat.id, "Тренування видалено!")
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
            dialogue
                .update(State::HomeTrainingMenu { phone_number })
                .await?;
        }
        Err(_) => {
            let keyboard = make_keyboard(vec![
                TrainingsCommands::AddTraining.to_string(),
                TrainingsCommands::DeleteTraining.to_string(),
                TrainingsCommands::ShowTrainings.to_string(),
                TrainingsCommands::GoBack.to_string(),
            ]);

            bot.send_message(msg.chat.id, "Тренування вже відсутнє!")
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
            dialogue
                .update(State::HomeTrainingMenu { phone_number })
                .await?;
        }
    }
    Ok(())
}

pub async fn process_training(
    msg: Message,
    open_ai_client: Arc<Mutex<OpenAiClient>>,
    user: Users,
    const1: &str,
    const2: &str,
    db: &mut Db,
    status: String,
) -> Result<String> {
    let response = if let Some(text) = msg.text() {
        let prompt = format_prompt(Some(text), const1, const2, user.clone());
        log::info!("Start sending prompt for training {}!", prompt);
        open_ai_client.lock().await.send_message(&prompt).await?
    } else {
        let prompt = format_prompt(None, const1, const2, user.clone());
        log::info!("Start sending prompt for training {}!", prompt);
        open_ai_client.lock().await.send_message(&prompt).await?
    };

    let user_id = user.id;
    db.insert_training(user_id, &response, status).await?;
    Ok(response)
}
