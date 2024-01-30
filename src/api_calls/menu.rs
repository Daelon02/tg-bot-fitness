use crate::models::{DietCommands, MenuCommands, MyDialogue, State, TrainingsCommands};
use crate::utils::make_keyboard;
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn change_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
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
                let keyboard = make_keyboard(trainings_buttons);
                bot.send_message(msg.chat.id, MenuCommands::MyHomeTrainings.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::HomeTrainingMenu).await?;
            }
            MenuCommands::MyGymTrainings => {
                let keyboard = make_keyboard(trainings_buttons);
                bot.send_message(msg.chat.id, MenuCommands::MyGymTrainings.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::GymTrainingMenu).await?;
            }
            MenuCommands::MyDiet => {
                let keyboard = make_keyboard(diet_buttons);
                bot.send_message(msg.chat.id, MenuCommands::MyDiet.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::DietMenu).await?;
            }
            MenuCommands::GoBack => {
                let keyboard = make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                ]);
                bot.send_message(msg.chat.id, MenuCommands::GoBack.to_string())
                    .reply_markup(keyboard.resize_keyboard(true))
                    .await?;
                dialogue.update(State::ChangeMenu).await?;
            }
        }
    }
    Ok(())
}

pub async fn home_training_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> crate::errors::Result<()> {
    if let Some(training_button) = msg.text() {
        let training_button = TrainingsCommands::from(training_button.to_string());
        match training_button {
            TrainingsCommands::AddTraining => {
                bot.send_message(msg.chat.id, "Додати тренування").await?;
            }
            TrainingsCommands::DeleteTraining => {
                bot.send_message(msg.chat.id, "Видалити тренування").await?;
            }
            TrainingsCommands::ShowTrainings => {
                bot.send_message(msg.chat.id, "Показати тренування").await?;
            }
            TrainingsCommands::GoBack => {
                bot.send_message(msg.chat.id, "На головну").await?;
                dialogue.update(State::ChangeMenu).await?;
            }
        }
    }
    Ok(())
}

pub async fn gym_training_menu(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> crate::errors::Result<()> {
    if let Some(training_button) = msg.text() {
        let training_button = TrainingsCommands::from(training_button.to_string());
        match training_button {
            TrainingsCommands::AddTraining => {
                bot.send_message(msg.chat.id, "Додати тренування").await?;
            }
            TrainingsCommands::DeleteTraining => {
                bot.send_message(msg.chat.id, "Видалити тренування").await?;
            }
            TrainingsCommands::ShowTrainings => {
                bot.send_message(msg.chat.id, "Показати тренування").await?;
            }
            TrainingsCommands::GoBack => {
                bot.send_message(msg.chat.id, "На головну").await?;
                dialogue.update(State::ChangeMenu).await?;
            }
        }
    }
    Ok(())
}

pub async fn diet_menu(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
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
                bot.send_message(msg.chat.id, "На головну").await?;
                dialogue.update(State::ChangeMenu).await?;
            }
        }
    }
    Ok(())
}
