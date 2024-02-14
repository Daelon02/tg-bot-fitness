use std::fmt::Display;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use teloxide::utils::command::BotCommands;

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the purchase procedure.")]
    Start,
    #[command(description = "cancel the purchase procedure.")]
    Cancel,
}

pub enum MenuCommands {
    MyHomeTrainings,
    MyGymTrainings,
    MyDiet,
    UpdateData,
    GoBack,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    GetPhoneNumber,
    GetEmail {
        phone_number: String,
    },
    GetAge {
        phone_number: String,
    },
    GetWeightAndHeight {
        phone_number: String,
    },
    ChangeMenu {
        phone_number: String,
    },
    HomeTrainingMenu {
        phone_number: String,
    },
    AddTraining {
        phone_number: String,
        training_state: String,
    },
    GymTrainingMenu {
        phone_number: String,
    },
    DietMenu {
        phone_number: String,
    },
    AddDiet {
        phone_number: String,
    },
    UpdateData {
        phone_number: String,
    },
}

impl Display for MenuCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuCommands::MyHomeTrainings => write!(f, "Мої домашні тренування"),
            MenuCommands::MyGymTrainings => write!(f, "Мої тренування"),
            MenuCommands::MyDiet => write!(f, "Моє харчування"),
            MenuCommands::UpdateData => write!(f, "Оновити дані"),
            MenuCommands::GoBack => write!(f, "На головну"),
        }
    }
}

impl From<String> for MenuCommands {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Мої домашні тренування" => MenuCommands::MyHomeTrainings,
            "Мої тренування" => MenuCommands::MyGymTrainings,
            "Моє харчування" => MenuCommands::MyDiet,
            "Оновити дані" => MenuCommands::UpdateData,
            "На головну" => MenuCommands::GoBack,
            _ => MenuCommands::GoBack,
        }
    }
}

pub enum TrainingsCommands {
    AddTraining,
    DeleteTraining,
    ShowTrainings,
    GoBack,
}

impl Display for TrainingsCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainingsCommands::AddTraining => write!(f, "Додати тренування"),
            TrainingsCommands::DeleteTraining => write!(f, "Видалити тренування"),
            TrainingsCommands::ShowTrainings => write!(f, "Показати тренування"),
            TrainingsCommands::GoBack => write!(f, "На головну"),
        }
    }
}

impl From<String> for TrainingsCommands {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Додати тренування" => TrainingsCommands::AddTraining,
            "Видалити тренування" => TrainingsCommands::DeleteTraining,
            "Показати тренування" => TrainingsCommands::ShowTrainings,
            "На головну" => TrainingsCommands::GoBack,

            _ => TrainingsCommands::GoBack,
        }
    }
}

pub enum DietCommands {
    AddDiet,
    DeleteDiet,
    ShowDiet,
    GoBack,
}

impl Display for DietCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DietCommands::AddDiet => write!(f, "Додати дієту"),
            DietCommands::DeleteDiet => write!(f, "Видалити дієту"),
            DietCommands::ShowDiet => write!(f, "Показати дієту"),
            DietCommands::GoBack => write!(f, "На головну"),
        }
    }
}

impl From<String> for DietCommands {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Додати дієту" => DietCommands::AddDiet,
            "Видалити дієту" => DietCommands::DeleteDiet,
            "Показати дієту" => DietCommands::ShowDiet,
            "На головну" => DietCommands::GoBack,
            _ => DietCommands::GoBack,
        }
    }
}
