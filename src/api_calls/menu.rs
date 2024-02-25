use crate::api_calls::diet::{delete_diet, show_diet};
use crate::api_calls::trainings::{delete_training, show_trainings};
use crate::consts::{GYM_STATE, HOME_STATE};
use crate::db::database::Db;
use crate::models::{
    DataCommands, DietCommands, MenuCommands, MyDialogue, State, TrainingsCommands,
};
use crate::utils::make_keyboard;
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::element::{Circle, Text};
use plotters::prelude::{IntoDrawingArea, LogScalable, RED, WHITE};
use plotters::series::LineSeries;
use plotters::style::{IntoFont, ShapeStyle, BLACK, BLUE, CYAN, GREEN, MAGENTA, YELLOW};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::InputFile;
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
    let size = db.get_size_by_user(user.id).await?;
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
    let sizes_list = db.get_sizes_by_user(user.id).await?;

    if let Some(sizes_list) = sizes_list {
        let path = Path::new("plots");
        if !path.exists() {
            std::fs::create_dir(path)?;
        }

        let path = &format!("plots/stats_plot_{}.png", user.id);
        {
            let root = BitMapBackend::new(path, (1280, 720)).into_drawing_area();
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption("Статистика розмірів", ("sans-serif", 40).into_font())
                .x_label_area_size(40.0)
                .y_label_area_size(40.0)
                .build_cartesian_2d(0.0..7.0, 0.0..150.0)?;

            chart.configure_mesh().draw()?;

            for (i, sizes) in sizes_list.iter().enumerate() {
                for (j, field_value) in [
                    &sizes.chest,
                    &sizes.waist,
                    &sizes.hips,
                    &sizes.hand_biceps,
                    &sizes.leg_biceps,
                    &sizes.calf,
                ]
                .iter()
                .enumerate()
                {
                    let color = match j {
                        0 => BLUE,    // груди
                        1 => GREEN,   // талія
                        2 => RED,     // стегна
                        3 => YELLOW,  // біцепс руки
                        4 => MAGENTA, // біцепс ноги
                        5 => CYAN,    // ікали
                        _ => BLACK,   // default
                    };

                    let value = field_value.as_f64();
                    chart.draw_series(std::iter::once(Circle::new(
                        (i as f64, value),
                        3,
                        ShapeStyle::from(color).filled(),
                    )))?;

                    // Додавання тексту з числами
                    chart.draw_series(std::iter::once(Text::new(
                        format!("{}", field_value),
                        ((i as f64) + 0.05, value + 2.0),
                        ("sans-serif", 15.0).into_font(),
                    )))?;

                    // Додавання рисок до наступної точки, крім останньої
                    if i < sizes_list.len() - 1 {
                        let next_sizes = &sizes_list[i + 1];
                        let next_value = match j {
                            0 => next_sizes.chest,
                            1 => next_sizes.waist,
                            2 => next_sizes.hips,
                            3 => next_sizes.hand_biceps,
                            4 => next_sizes.leg_biceps,
                            5 => next_sizes.calf,
                            _ => 0, // default
                        };
                        let next_value = next_value.as_f64();
                        chart.draw_series(LineSeries::new(
                            vec![(i as f64, value), ((i + 1) as f64, next_value)],
                            &BLACK,
                        ))?;
                    } else {
                        // Додаємо пояснення поруч із останньою крапкою
                        let explanation = match j {
                            0 => "Груди",
                            1 => "Талія",
                            2 => "Стегна",
                            3 => "Біцепс руки",
                            4 => "Біцепс ноги",
                            5 => "Ікри",
                            _ => "",
                        };
                        chart.draw_series(std::iter::once(Text::new(
                            explanation,
                            (i as f64 + 0.18, value + 2.0), // Збільшуємо другий аргумент, щоб текст був ближче до крапки
                            ("sans-serif", 15.0).into_font().color(&BLACK),
                        )))?;
                    }
                }
            }
        }

        let file = InputFile::file(format!("plots/stats_plot_{}.png", user.id));

        let keyboard = make_keyboard(vec![
            DataCommands::UpdateData.to_string(),
            DataCommands::UpdateSize.to_string(),
            DataCommands::ShowData.to_string(),
            DataCommands::ShowStatistics.to_string(),
            DataCommands::GoBack.to_string(),
        ]);

        bot.send_photo(msg.chat.id, file)
            .reply_markup(keyboard.resize_keyboard(true))
            .await?;

        std::fs::remove_file(format!("plots/stats_plot_{}.png", user.id))?;
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
            db.update_size(user.id, data).await?;

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
