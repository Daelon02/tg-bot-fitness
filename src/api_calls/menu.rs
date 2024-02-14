use crate::api_calls::diet::{delete_diet, show_diet};
use crate::api_calls::trainings::{delete_training, show_trainings};
use crate::consts::{GYM_STATE, HOME_STATE};
use crate::db::database::Db;
use crate::models::{
    DataCommands, DietCommands, MenuCommands, MyDialogue, State, TrainingsCommands,
};
use crate::utils::make_keyboard;
use std::collections::HashMap;
use std::ops::DerefMut;
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
            MenuCommands::Data => {
                log::info!("User wants to update data {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    DataCommands::UpdateData.to_string(),
                    DataCommands::UpdateSize.to_string(),
                    DataCommands::ShowData.to_string(),
                    DataCommands::ShowStatistics.to_string(),
                    DataCommands::GoBack.to_string(),
                ]);
                bot.send_message(msg.chat.id, MenuCommands::Data.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::Data { phone_number }).await?;
            }
            MenuCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                    MenuCommands::Data.to_string(),
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

async fn show_data(
    bot: Bot,
    db: &mut Db,
    msg: Message,
    phone_number: String,
) -> crate::errors::Result<()> {
    log::info!("User wants to show data {}", msg.chat.id);
    let user = db.get_user(&phone_number).await?;
    let after = db.is_user_have_after_size(user.id).await?;
    if after {
        let size = db.get_after_size(user.id).await?;
        bot.send_message(
            msg.chat.id,
            format!(
                "Ваші дані: \n\n\
                     Вік: {} \n\
                     Зріст: {} \n\
                     Вага: {} \n\
                     Розмір грудей: {} \n\
                     Розмір талії: {} \n\
                     Розмір бедер: {} \n\
                     Розмір біцепсу руки: {} \n\
                     Розмір біцепсу ноги: {} \n\
                     Розмір ікри: {}",
                user.age.unwrap_or_default(),
                user.height.unwrap_or_default(),
                user.weight.unwrap_or_default(),
                size.clone().unwrap_or_default().chest,
                size.clone().unwrap_or_default().waist,
                size.clone().unwrap_or_default().hips,
                size.clone().unwrap_or_default().hand_biceps,
                size.clone().unwrap_or_default().leg_biceps,
                size.unwrap_or_default().calf
            ),
        )
        .await?;
        let keyboard = make_keyboard(vec![
            DataCommands::UpdateData.to_string(),
            DataCommands::UpdateSize.to_string(),
            DataCommands::ShowData.to_string(),
            DataCommands::ShowStatistics.to_string(),
            DataCommands::GoBack.to_string(),
        ]);
        bot.send_message(msg.chat.id, MenuCommands::Data.to_string())
            .reply_markup(keyboard.resize_keyboard(true))
            .await?;
    } else {
        let size = db.get_before_size(user.id).await?;
        if let Some(size) = size {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Ваші дані: \n\n\
                     Вік: {} \n\
                     Зріст: {} \n\
                     Вага: {} \n\
                     Розмір грудей: {} \n\
                     Розмір талії: {} \n\
                     Розмір бедер: {} \n\
                     Розмір біцепсу руки: {} \n\
                     Розмір біцепсу ноги: {} \n\
                     Розмір ікри: {}",
                    user.age.unwrap_or_default(),
                    user.height.unwrap_or_default(),
                    user.weight.unwrap_or_default(),
                    size.chest,
                    size.waist,
                    size.hips,
                    size.hand_biceps,
                    size.leg_biceps,
                    size.calf
                ),
            )
            .await?;
            let keyboard = make_keyboard(vec![
                DataCommands::UpdateData.to_string(),
                DataCommands::UpdateSize.to_string(),
                DataCommands::ShowData.to_string(),
                DataCommands::ShowStatistics.to_string(),
                DataCommands::GoBack.to_string(),
            ]);
            bot.send_message(msg.chat.id, MenuCommands::Data.to_string())
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
        } else {
            bot.send_message(msg.chat.id, "Ви ще не вводили дані!")
                .await?;
            let keyboard = make_keyboard(vec![
                MenuCommands::MyGymTrainings.to_string(),
                MenuCommands::MyHomeTrainings.to_string(),
                MenuCommands::MyDiet.to_string(),
                MenuCommands::Data.to_string(),
            ]);
            bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
        }
    }
    Ok(())
}

async fn show_statistic(
    bot: Bot,
    db: &mut Db,
    msg: Message,
    phone_number: String,
) -> crate::errors::Result<()> {
    log::info!("User wants to show statistic {}", msg.chat.id);
    let user = db.get_user(&phone_number).await?;
    let after = db.get_after_size(user.id).await?;
    let before = db.get_before_size(user.id).await?;

    if let Some(before) = before {
        if let Some(after) = after {
            let chest = after.chest - before.chest;
            let waist = after.waist - before.waist;
            let hips = after.hips - before.hips;
            let hand_biceps = after.hand_biceps - before.hand_biceps;
            let leg_biceps = after.leg_biceps - before.leg_biceps;
            let calf = after.calf - before.calf;

            bot.send_message(
                msg.chat.id,
                format!(
                    "Це скільки ви набрали/скинули після останнього оновлення данних\n\n\
                Ваші дані: \n\n\
                     Різниця до/після грудей: {} \n\
                     Різниця до/після талії: {} \n\
                     Різниця до/після бедер: {} \n\
                     Різниця до/після біцепсу руки: {} \n\
                     Різниця до/після біцепсу ноги: {} \n\
                     Різниця до/після ікри: {}",
                    chest, waist, hips, hand_biceps, leg_biceps, calf
                ),
            )
            .await?;
            let keyboard = make_keyboard(vec![
                DataCommands::UpdateData.to_string(),
                DataCommands::UpdateSize.to_string(),
                DataCommands::ShowData.to_string(),
                DataCommands::ShowStatistics.to_string(),
                DataCommands::GoBack.to_string(),
            ]);
            bot.send_message(msg.chat.id, "")
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
        } else {
            bot.send_message(msg.chat.id, "Ви ще не вводили нові дані!")
                .await?;
            let keyboard = make_keyboard(vec![
                DataCommands::UpdateData.to_string(),
                DataCommands::UpdateSize.to_string(),
                DataCommands::ShowData.to_string(),
                DataCommands::ShowStatistics.to_string(),
                DataCommands::GoBack.to_string(),
            ]);
            bot.send_message(msg.chat.id, "")
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
        }
    } else {
        bot.send_message(msg.chat.id, "Ви ще не вводили дані!")
            .await?;
        let keyboard = make_keyboard(vec![
            DataCommands::UpdateData.to_string(),
            DataCommands::UpdateSize.to_string(),
            DataCommands::ShowData.to_string(),
            DataCommands::ShowStatistics.to_string(),
            DataCommands::GoBack.to_string(),
        ]);
        bot.send_message(msg.chat.id, MenuCommands::Data.to_string())
            .reply_markup(keyboard.resize_keyboard(true))
            .await?;
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
                    MenuCommands::Data.to_string(),
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
                    MenuCommands::Data.to_string(),
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
                    MenuCommands::Data.to_string(),
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
    let mut db = db.lock().await;
    match msg.text() {
        Some(data) => {
            let menu = DataCommands::from(data.to_string());

            match menu {
                DataCommands::UpdateData => {
                    bot.send_message(msg.chat.id, "Оновити раніше записані дані")
                        .await?;
                    bot.send_message(
                        msg.chat.id,
                        "Хочете оновити дані? \n\n\
     Добре, тільки скидуйте у такому вигляді: вік: 21, зріст: 185, вага: 112",
                    )
                    .await?;
                    dialogue.update(State::UpdateData { phone_number }).await?;
                }
                DataCommands::UpdateSize => {
                    bot.send_message(msg.chat.id, "Оновити розмір м'язів")
                        .await?;
                    bot.send_message(
                        msg.chat.id,
                        "Хочете оновити розмір м'язів? \n\n\
     Добре, тільки скидуйте у такому вигляді: 108 - груди, 105 - талія, 123 - бедра, 39 - біцепс руки, 72 - біцепс ноги, 45 - ікра",
                    )
                        .await?;
                    dialogue.update(State::UpdateSize { phone_number }).await?;
                }
                DataCommands::ShowData => {
                    show_data(
                        bot.clone(),
                        db.deref_mut(),
                        msg.clone(),
                        phone_number.clone(),
                    )
                    .await?;
                }
                DataCommands::ShowStatistics => {
                    show_statistic(
                        bot.clone(),
                        db.deref_mut(),
                        msg.clone(),
                        phone_number.clone(),
                    )
                    .await?;
                }
                DataCommands::GoBack => {
                    let keyboard = make_keyboard(vec![
                        MenuCommands::MyGymTrainings.to_string(),
                        MenuCommands::MyHomeTrainings.to_string(),
                        MenuCommands::MyDiet.to_string(),
                        MenuCommands::Data.to_string(),
                    ]);
                    bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                        .reply_markup(keyboard.resize_keyboard(true))
                        .await?;
                    dialogue.update(State::ChangeMenu { phone_number }).await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "На жаль, я не розумію тебе!")
                .await?;
            return Ok(());
        }
    }

    Ok(())
}

pub async fn update_data_data(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    let mut db = db.lock().await;
    match msg.text() {
        Some(data) => {
            let user = db.get_user(&phone_number).await?;
            let data = parse_string(data);
            update_age(
                data.clone(),
                bot.clone(),
                msg.clone(),
                phone_number.clone(),
                db.deref_mut(),
                dialogue.clone(),
                user.id,
            )
            .await?;

            update_height(
                data.clone(),
                bot.clone(),
                msg.clone(),
                phone_number.clone(),
                db.deref_mut(),
                dialogue.clone(),
                user.id,
            )
            .await?;

            let keyboard = make_keyboard(vec![
                DataCommands::UpdateData.to_string(),
                DataCommands::UpdateSize.to_string(),
                DataCommands::ShowData.to_string(),
                DataCommands::ShowStatistics.to_string(),
                DataCommands::GoBack.to_string(),
            ]);
            bot.send_message(msg.chat.id, "Дані оновлено!")
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
            dialogue.update(State::Data { phone_number }).await?;
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

pub async fn update_size(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> crate::errors::Result<()> {
    let mut db = db.lock().await;
    match msg.text() {
        Some(data) => {
            let user = db.get_user(&phone_number).await?;
            let data = parse_string_size(
                data,
                bot.clone(),
                msg.clone(),
                dialogue.clone(),
                phone_number.clone(),
            )
            .await?;
            let is_after = db.is_user_have_after_size(user.id).await?;
            db.update_size(user.id, data, is_after).await?;

            let keyboard = make_keyboard(vec![
                DataCommands::UpdateData.to_string(),
                DataCommands::UpdateSize.to_string(),
                DataCommands::ShowData.to_string(),
                DataCommands::ShowStatistics.to_string(),
                DataCommands::GoBack.to_string(),
            ]);
            bot.send_message(msg.chat.id, "Розмір м'язів оновлено!")
                .reply_markup(keyboard.resize_keyboard(true))
                .await?;
            dialogue.update(State::Data { phone_number }).await?;
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

async fn parse_string_size(
    data: &str,
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    phone_number: String,
) -> crate::errors::Result<Vec<String>> {
    let mut result_hashmap = HashMap::new();
    let mut result_vec = Vec::new();

    for pair in data.split(", ") {
        let parts: Vec<&str> = pair.split(" - ").collect();
        if parts.len() == 2 {
            if let Ok(value) = parts[0].trim().parse::<i32>() {
                result_hashmap.insert(parts[1].trim().to_string(), value);
            }
        }
    }

    let chest = match result_hashmap.get("груди") {
        Some(chest) => chest,
        None => {
            bot.send_message(msg.chat.id, "Груди не валідні!").await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(Vec::new());
        }
    };

    let waist = match result_hashmap.get("талія") {
        Some(waist) => waist,
        None => {
            bot.send_message(msg.chat.id, "Талія не валідні!").await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(Vec::new());
        }
    };

    let hips = match result_hashmap.get("бедра") {
        Some(hips) => hips,
        None => {
            bot.send_message(msg.chat.id, "Бедра не валідні!").await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(Vec::new());
        }
    };

    let biceps_arm = match result_hashmap.get("біцепс руки") {
        Some(biceps_arm) => biceps_arm,
        None => {
            bot.send_message(msg.chat.id, "Біцепс руки не валідні!")
                .await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(Vec::new());
        }
    };

    let biceps_leg = match result_hashmap.get("біцепс ноги") {
        Some(biceps_leg) => biceps_leg,
        None => {
            bot.send_message(msg.chat.id, "Біцепс ноги не валідні!")
                .await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(Vec::new());
        }
    };

    let calf = match result_hashmap.get("ікра") {
        Some(calf) => calf,
        None => {
            bot.send_message(msg.chat.id, "Ікра не валідні!").await?;
            dialogue
                .update(State::UpdateData {
                    phone_number: phone_number.clone(),
                })
                .await?;
            return Ok(Vec::new());
        }
    };

    result_vec.push(chest.to_string());
    result_vec.push(waist.to_string());
    result_vec.push(hips.to_string());
    result_vec.push(biceps_arm.to_string());
    result_vec.push(biceps_leg.to_string());
    result_vec.push(calf.to_string());

    Ok(result_vec)
}
