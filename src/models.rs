use std::fmt::Display;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

pub enum MenuCommands {
    MyHomeTrainings,
    MyGymTrainings,
    MyDiet,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    GetPhoneNumber,
    GetEmail {
        phone_number: String,
    },
    CheckMyGymTrainings,
    CheckMyHomeTrainings,
    CheckMyDiet,
}

impl Display for MenuCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuCommands::MyHomeTrainings => write!(f, "Мої домашні тренування"),
            MenuCommands::MyGymTrainings => write!(f, "Мої тренування"),
            MenuCommands::MyDiet => write!(f, "Моє харчування"),
        }
    }
}

impl From<String> for MenuCommands {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Мої домашні тренування" => MenuCommands::MyHomeTrainings,
            "Мої тренування" => MenuCommands::MyGymTrainings,
            "Моє харчування" => MenuCommands::MyDiet,
            _ => panic!("Unknown command"),
        }
    }
}

pub enum TrainingsCommands {
    AddTraining,
    DeleteTraining,
    ShowTrainings,
}

impl Display for TrainingsCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainingsCommands::AddTraining => write!(f, "Додати тренування"),
            TrainingsCommands::DeleteTraining => write!(f, "Видалити тренування"),
            TrainingsCommands::ShowTrainings => write!(f, "Показати тренування"),
        }
    }
}

impl From<String> for TrainingsCommands {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Додати тренування" => TrainingsCommands::AddTraining,
            "Видалити тренування" => TrainingsCommands::DeleteTraining,
            "Показати тренування" => TrainingsCommands::ShowTrainings,
            _ => panic!("Unknown command"),
        }
    }
}

pub enum DietCommands {
    AddDiet,
    DeleteDiet,
    ShowDiet,
}

impl Display for DietCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DietCommands::AddDiet => write!(f, "Додати дієту"),
            DietCommands::DeleteDiet => write!(f, "Видалити дієту"),
            DietCommands::ShowDiet => write!(f, "Показати дієту"),
        }
    }
}

impl From<String> for DietCommands {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Додати дієту" => DietCommands::AddDiet,
            "Видалити дієту" => DietCommands::DeleteDiet,
            "Показати дієту" => DietCommands::ShowDiet,
            _ => panic!("Unknown command"),
        }
    }
}
