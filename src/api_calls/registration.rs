use crate::db::database::Db;
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

                bot.send_message(msg.chat.id, "Дякую за пошту! Тепер відправ свій вік!")
                    .await?;
                dialogue.update(State::GetAge { phone_number }).await?;
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
            dialogue
                .update(State::ChangeMenu {
                    phone_number: contact.phone_number.clone(),
                })
                .await?;
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

pub async fn get_age(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    let mut db = db.lock().await;
    log::info!("Start getting age!");
    if let Some(age) = msg.text() {
        match age.parse::<i32>() {
            Ok(age) => {
                if age > 0 && age < 100 {
                    db.add_age(&phone_number, age).await?;
                    // process and send check to storage
                    bot.send_message(
                        msg.chat.id,
                        "Дякую за вік! Тепер віправ свій зріст та вагу у форматі: зріст вага! Приклад: 185 90",
                    )
                        .await?;
                    dialogue
                        .update(State::GetWeightAndHeight { phone_number })
                        .await?;
                } else {
                    bot.send_message(msg.chat.id, "Вік не валідний!").await?;
                }
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Вік не валідний!").await?;
            }
        }
    } else {
        bot.send_message(msg.chat.id, "Вік не валідний!").await?;
    }
    Ok(())
}

pub async fn get_height_and_weight(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    log::info!("Start getting height and weight!");
    let mut db = db.lock().await;
    let height_and_weight: Vec<&str> = msg
        .text()
        .expect("Cannot get message value")
        .split(' ')
        .collect();
    if height_and_weight.len() != 2 {
        bot.send_message(msg.chat.id, "Висота та вага не валідні!")
            .await?;
        return Ok(());
    }
    match height_and_weight[0].parse::<i32>() {
        Ok(height) => {
            match height_and_weight[1].parse::<i32>() {
                Ok(weight) => {
                    db.add_height_and_weight(&phone_number, height, weight)
                        .await?;
                    // process and send check to storage
                    bot.send_message(
                        msg.chat.id,
                        "Дякую за висоту та вагу! \
                    Тепер я зможу розрахувати тренування та дієту для тебе! \
                    Також, дякую за реєстрацію)",
                    )
                    .reply_markup(
                        make_keyboard(vec![
                            MenuCommands::MyGymTrainings.to_string(),
                            MenuCommands::MyHomeTrainings.to_string(),
                            MenuCommands::MyDiet.to_string(),
                        ])
                        .resize_keyboard(true),
                    )
                    .await?;
                    dialogue.update(State::ChangeMenu { phone_number }).await?;
                }
                Err(_) => {
                    bot.send_message(msg.chat.id, "Вага не валідна!").await?;
                }
            }
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "Висота не валідна!").await?;
        }
    }

    Ok(())
}
