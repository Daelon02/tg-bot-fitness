use crate::consts::{GYM_STATE, HOME_STATE};
use crate::models::{DietCommands, MenuCommands, MyDialogue, State, TrainingsCommands};
use crate::utils::make_keyboard;
use teloxide::prelude::*;
use teloxide::Bot;

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

pub async fn home_training_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
) -> crate::errors::Result<()> {
    log::info!("User in home training menu {}", msg.chat.id);
    if let Some(training_button) = msg.text() {
        let training_button = TrainingsCommands::from(training_button.to_string());
        match training_button {
            TrainingsCommands::AddTraining => {
                log::info!("User wants to add training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Додати тренування").await?;
                bot.send_message(msg.chat.id, "Напишить будь-ласка, чи є у вас якісь протипоказання, якщо ні, просто відправте крапку.").await?;
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
                dialogue
                    .update(State::DeleteTraining {
                        phone_number,
                        training_state: HOME_STATE.to_string(),
                    })
                    .await?;
            }
            TrainingsCommands::ShowTrainings => {
                log::info!("User wants to show training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Показати тренування").await?;
                dialogue
                    .update(State::ShowTrainings {
                        phone_number,
                        training_state: HOME_STATE.to_string(),
                    })
                    .await?;
            }
            TrainingsCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
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
    phone_number: String,
) -> crate::errors::Result<()> {
    if let Some(training_button) = msg.text() {
        let training_button = TrainingsCommands::from(training_button.to_string());
        match training_button {
            TrainingsCommands::AddTraining => {
                log::info!("User wants to add training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Додати тренування").await?;
                bot.send_message(msg.chat.id, "Напишить будь-ласка, чи є у вас якісь протипоказання, якщо ні, просто відправте крапку.").await?;
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
                dialogue
                    .update(State::DeleteTraining {
                        phone_number,
                        training_state: GYM_STATE.to_string(),
                    })
                    .await?;
            }
            TrainingsCommands::ShowTrainings => {
                log::info!("User wants to show training {}", msg.chat.id);
                bot.send_message(msg.chat.id, "Показати тренування").await?;
                dialogue
                    .update(State::ShowTrainings {
                        phone_number,
                        training_state: GYM_STATE.to_string(),
                    })
                    .await?;
            }
            TrainingsCommands::GoBack => {
                log::info!("User wants to go back {}", msg.chat.id);
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
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
) -> crate::errors::Result<()> {
    if let Some(diet_button) = msg.text() {
        let diet_button = DietCommands::from(diet_button.to_string());
        match diet_button {
            DietCommands::AddDiet => {
                bot.send_message(msg.chat.id, "Додати дієту").await?;
            }
            DietCommands::DeleteDiet => {
                bot.send_message(msg.chat.id, "Видалити дієту").await?;
            }
            DietCommands::ShowDiet => {
                bot.send_message(msg.chat.id, "Показати дієту").await?;
            }
            DietCommands::GoBack => {
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
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
