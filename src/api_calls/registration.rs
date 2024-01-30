use crate::db::db::Db;
use crate::models::{MenuCommands, MyDialogue, State};
use crate::utils::{is_valid_email, make_keyboard};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::Bot;
use tokio::sync::Mutex;

pub async fn get_email(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    log::info!("Start getting email!");
    match msg.text() {
        Some(email) => {
            if is_valid_email(email)? {
                // process and send check to storage
                let mut db = db.lock().await;
                db.add_email(&phone_number, email).await?;

                bot.send_message(msg.chat.id, "Дякую за пошту! Тепер ти з нами!")
                    .reply_markup(
                        make_keyboard(vec![
                            MenuCommands::MyGymTrainings.to_string(),
                            MenuCommands::MyHomeTrainings.to_string(),
                            MenuCommands::MyDiet.to_string(),
                        ])
                        .resize_keyboard(true),
                    )
                    .await?;
                dialogue.update(State::ChangeMenu).await?;
            } else {
                bot.send_message(msg.chat.id, "Пошта не валідна!").await?;
            }
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "На жаль, я не зможу зареєструвати тебе без пошти!",
            )
            .await?;
            return Ok(());
        }
    }

    Ok(())
}

pub async fn get_number(
    bot: Bot,
    msg: Message,
    db: Arc<Mutex<Db>>,
    dialogue: MyDialogue,
) -> crate::errors::Result<()> {
    let mut db = db.lock().await;
    // Create a list of buttons and send them.
    if let Some(contact) = msg.contact() {
        log::info!("Got contact: {:?}", contact);
        if db.if_user_exists(&contact.user_id).await? {
            log::info!("User already exists");
            bot.send_message(msg.chat.id, "Ти вже з нами!")
                .reply_markup(
                    make_keyboard(vec![
                        MenuCommands::MyGymTrainings.to_string(),
                        MenuCommands::MyHomeTrainings.to_string(),
                        MenuCommands::MyDiet.to_string(),
                    ])
                    .resize_keyboard(true),
                )
                .await?;
            dialogue.update(State::ChangeMenu).await?;
        } else {
            log::info!("Adding new user to db");
            bot.send_message(
                msg.chat.id,
                "Ти новий користувач! Дякую за номер, тепер віправ свою пошту!",
            )
            .await?;
            db.insert_user(&contact.first_name, &contact.phone_number, &contact.user_id)
                .await?;
            dialogue
                .update(State::GetEmail {
                    phone_number: contact.phone_number.clone(),
                })
                .await?;
        }
    } else {
        log::warn!("No contact in message");
    }
    Ok(())
}
