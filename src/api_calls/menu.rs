use crate::api_calls::diet::{delete_diet, show_diet};
use crate::api_calls::trainings::{delete_training, show_trainings};
use crate::consts::{GYM_STATE, HOME_STATE};
use crate::db::database::Db;
use crate::models::{DietCommands, MenuCommands, MyDialogue, State, TrainingsCommands};
use crate::utils::make_keyboard;
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::Bot;
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn change_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
) -> crate::errors::Result<()> {
    if let Some(menu_button) = msg.text() {
        let menu_button = MenuCommands::from(menu_button.to_string());
        let trainings_buttons = vec![
            TrainingsCommands::AddTraining.to_string(),
            TrainingsCommands::DeleteTraining.to_string(),
            TrainingsCommands::ShowTrainings.to_string(),
            TrainingsCommands::GoBack.to_string(),
        ];

        let diet_buttons = vec![
            DietCommands::AddDiet.to_string(),
            DietCommands::DeleteDiet.to_string(),
            DietCommands::ShowDiet.to_string(),
            DietCommands::GoBack.to_string(),
        ];

        match menu_button {
            MenuCommands::MyHomeTrainings => {
                log::info!("User wants to see home training {}", msg.chat.id);
                let keyboard = make_keyboard(trainings_buttons);
                bot.send_message(msg.chat.id, MenuCommands::MyHomeTrainings.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue
                    .update(State::HomeTrainingMenu { phone_number })
                    .await?;
            }
            MenuCommands::MyGymTrainings => {
                log::info!("User wants to see gym training {}", msg.chat.id);
                let keyboard = make_keyboard(trainings_buttons);
                bot.send_message(msg.chat.id, MenuCommands::MyGymTrainings.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue
                    .update(State::GymTrainingMenu { phone_number })
                    .await?;
            }
            MenuCommands::MyDiet => {
                log::info!("User wants to see diet {}", msg.chat.id);
                let keyboard = make_keyboard(diet_buttons);
                bot.send_message(msg.chat.id, MenuCommands::MyDiet.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::DietMenu { phone_number }).await?;
            }
            MenuCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                    MenuCommands::UpdateData.to_string(),
                ]);
                bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::ChangeMenu { phone_number }).await?;
            }
            MenuCommands::UpdateData => {
                log::info!("User wants to update data {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Оновити дані").await?;
                bot.send_message(
                    msg.chat.id,
                    "Хочете оновити дані? \n\n\
     Добре, тільки скидуйте у такому вигляді: вік: 21, зріст: 185, вага: 112",
                )
                .await?;
                dialogue.update(State::UpdateData { phone_number }).await?;
            }
        }
    }
    Ok(())
}

pub async fn home_training_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    phone_number: String,
) -> crate::errors::Result<()> {
    log::info!("User in home training menu {}", msg.chat.id);
    if let Some(training_button) = msg.text() {
        let training_button = TrainingsCommands::from(training_button.to_string());
        match training_button {
            TrainingsCommands::AddTraining => {
                log::info!("User wants to add training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Додати тренування").await?;
                bot.send_message(
                    msg.chat.id,
                   "Напишить будь-ласка, чи є у вас якісь протипоказання, якщо ні, просто відправте крапку. \n\n\
                 Також, потрібно буде трохи зачекати, генерую для тебе тренування)"
                ).await?;
                dialogue
                    .update(State::AddTraining {
                        phone_number,
                        training_state: HOME_STATE.to_string(),
                    })
                    .await?;
            }
            TrainingsCommands::DeleteTraining => {
                log::info!("User wants to delete training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Видалити тренування").await?;
                delete_training(
                    bot.clone(),
                    dialogue.clone(),
                    msg.clone(),
                    db,
                    (phone_number.clone(), HOME_STATE.to_string()),
                )
                .await?
            }
            TrainingsCommands::ShowTrainings => {
                log::info!("User wants to show training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Показати тренування").await?;
                show_trainings(
                    bot.clone(),
                    dialogue.clone(),
                    msg.clone(),
                    db,
                    (phone_number.clone(), HOME_STATE.to_string()),
                )
                .await?
            }
            TrainingsCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                    MenuCommands::UpdateData.to_string(),
                ]);
                bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::ChangeMenu { phone_number }).await?;
            }
        }
    }
    Ok(())
}

pub async fn gym_training_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    db: Arc<Mutex<Db>>,
    phone_number: String,
) -> crate::errors::Result<()> {
    if let Some(training_button) = msg.text() {
        let training_button = TrainingsCommands::from(training_button.to_string());
        match training_button {
            TrainingsCommands::AddTraining => {
                log::info!("User wants to add training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Додати тренування").await?;
                bot.send_message(msg.chat.id, "Напишить будь-ласка, чи є у вас якісь протипоказання, якщо ні, просто відправте крапку. \n\n\
                \n Також, потрібно буде трохи зачекати, генерую для тебе тренування)").await?;
                dialogue
                    .update(State::AddTraining {
                        phone_number,
                        training_state: GYM_STATE.to_string(),
                    })
                    .await?;
            }
            TrainingsCommands::DeleteTraining => {
                log::info!("User wants to delete training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Видалити тренування").await?;
                delete_training(
                    bot.clone(),
                    dialogue.clone(),
                    msg.clone(),
                    db,
                    (phone_number.clone(), GYM_STATE.to_string()),
                )
                .await?;
            }
            TrainingsCommands::ShowTrainings => {
                log::info!("User wants to show training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Показати тренування").await?;
                show_trainings(
                    bot.clone(),
                    dialogue.clone(),
                    msg.clone(),
                    db,
                    (phone_number.clone(), GYM_STATE.to_string()),
                )
                .await?;
            }
            TrainingsCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                    MenuCommands::UpdateData.to_string(),
                ]);
                bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::ChangeMenu { phone_number }).await?;
            }
        }
    }
    Ok(())
}

pub async fn diet_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    if let Some(training_button) = msg.text() {
        let training_button = DietCommands::from(training_button.to_string());
        match training_button {
            DietCommands::AddDiet => {
                log::info!("User wants to add training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Додати дієту").await?;
                bot.send_message(msg.chat.id, "Напишить будь-ласка, чи є у вас якісь протипоказання, якщо ні, просто відправте крапку. \n\n \
                Також, потрібно буде трохи зачекати, генерую для тебе дієту)").await?;
                dialogue.update(State::AddDiet { phone_number }).await?;
            }
            DietCommands::DeleteDiet => {
                log::info!("User wants to delete training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Видалити дієту").await?;
                delete_diet(
                    bot.clone(),
                    dialogue.clone(),
                    msg.clone(),
                    db,
                    phone_number.clone(),
                )
                .await?;
            }
            DietCommands::ShowDiet => {
                log::info!("User wants to show diet {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Показати дієту").await?;
                show_diet(
                    bot.clone(),
                    dialogue.clone(),
                    msg.clone(),
                    db,
                    phone_number.clone(),
                )
                .await?;
            }
            DietCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                    MenuCommands::UpdateData.to_string(),
                ]);
                bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::ChangeMenu { phone_number }).await?;
            }
        }
    }
    Ok(())
}

pub async fn update_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    match msg.text() {
        Some(data) => {
            let mut db = db.lock().await;
            let user = db.get_user(&phone_number).await?;
            let data = parse_string(data);
            update_age(
                data.clone(),
                bot.clone(),
                msg.clone(),
                phone_number.clone(),
                &mut db,
                dialogue.clone(),
                user.id,
            )
            .await?;

            update_height(
                data.clone(),
                bot.clone(),
                msg.clone(),
                phone_number.clone(),
                &mut db,
                dialogue.clone(),
                user.id,
            )
            .await?;

            bot.send_message(msg.chat.id, "Дані оновлено!").await?;
            dialogue.update(State::ChangeMenu { phone_number }).await?;
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "На жаль, я не зможу зареєструвати тебе без даних!",
            )
            .await?;
            return Ok(());
        }
    }

    Ok(())
}

async fn update_age(
    data: HashMap<String, String>,
    bot: Bot,
    msg: Message,
    phone_number: String,
    db: &mut Db,
    dialogue: MyDialogue,
    id: Uuid,
) -> crate::errors::Result<()> {
    let age = match data.get("вік") {
        Some(age) => age.parse::<i32>()?,
        None => {
            bot.send_message(msg.chat.id, "Вік не валідний!").await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(());
        }
    };

    db.update_age(id, age).await?;

    Ok(())
}

async fn update_height(
    data: HashMap<String, String>,
    bot: Bot,
    msg: Message,
    phone_number: String,
    db: &mut Db,
    dialogue: MyDialogue,
    id: Uuid,
) -> crate::errors::Result<()> {
    let (height, weight) = match (data.get("зріст"), data.get("вага")) {
        (Some(height), Some(weight)) => (height.parse::<i32>()?, weight.parse::<i32>()?),
        _ => {
            bot.send_message(msg.chat.id, "Висота та вага не валідні!")
                .await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(());
        }
    };

    db.update_height_and_weight(id, height, weight).await?;

    Ok(())
}

fn parse_string(data: &str) -> HashMap<String, String> {
    let vec = data.split(", ").collect::<Vec<&str>>();
    let mut data = HashMap::new();

    // Обработка каждой подстроки
    for vec_str in vec {
        // Разделение подстроки на ключ и значение по двоеточию
        let pare: Vec<&str> = vec_str.split(": ").collect();
        // Получение ключа и значения
        let key = pare[0].to_string();
        let value = pare[1].trim().to_string(); // Удаление пробелов в начале и конце значения
                                                // Добавление данных в хэш-мап
        data.insert(key, value);
    }
    data
}
