use crate::db::db::Db;
use crate::errors::Result;
use crate::models::{MenuCommands, MyDialogue, State};
use crate::utils::{init_logging, make_keyboard};
use dotenv::dotenv;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, DpHandlerDescription, UpdateHandler};
use teloxide::dptree::case;
use teloxide::types::{ButtonRequest, KeyboardButton};
use teloxide::{
    payloads::SendMessageSetters, prelude::*, types::KeyboardMarkup, utils::command::BotCommands,
};
use tokio::sync::Mutex;

mod db;
mod models;
mod utils;

mod errors;

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the purchase procedure.")]
    Start,
    #[command(description = "cancel the purchase procedure.")]
    Cancel,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_logging()?;
    log::info!("Starting fitness bot...");

    let bot = Bot::new(dotenv::var("TELOXIDE_TOKEN")?);

    let database_url = dotenv::var("DATABASE_URL")?;

    let db = Arc::new(Mutex::new(Db::new(&database_url)));
    let state = Arc::new(State::Start);

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            Arc::clone(&db),
            Arc::clone(&state)
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

fn schema<'a>() -> Handler<'static, DependencyMap, Result<()>, DpHandlerDescription> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Start].endpoint(start)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::GetPhoneNumber].endpoint(get_number))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_message()
        .branch(case![State::GetEmail { phone_number }].endpoint(get_email));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn help(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    let button = KeyboardButton::new("Відправити номер").request(ButtonRequest::Contact);
    let markup = KeyboardMarkup::new([[button]])
        .resize_keyboard(true)
        .one_time_keyboard(true);
    bot.send_message(msg.chat.id, "Привіт! Я бот для тренувань! \
                Тут ти можешь зробити для себе тренування у залі, \
                тренування удома, та дієту, спеціально підібрану для тебе! \
                Будь-ласка, відправ спочатку свій номер телефона а потім пошту, для реєстрації або перевірки чи ти вже з нами!")
        .reply_markup(markup)
        .allow_sending_without_reply(true)
        .await?;

    dialogue.update(State::GetPhoneNumber).await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(
        msg.chat.id,
        "Я тебе не розумію, подивись будь-ласка на команду /help",
    )
    .await?;
    Ok(())
}

async fn get_email(
    bot: Bot,
    _dialogue: MyDialogue,
    msg: Message,
    phone_number: String,
    db: Arc<Mutex<Db>>,
) -> Result<()> {
    println!("Start getting email!");
    match msg.text() {
        Some(email) => {
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
        }
        None => {
            bot.send_message(
                msg.chat.id,
                "На жаль, я не зможу зареєструвати тебе без пошти!",
            )
            .await?;
        }
    }

    Ok(())
}

async fn get_number(
    bot: Bot,
    msg: Message,
    db: Arc<Mutex<Db>>,
    dialogue: MyDialogue,
) -> Result<()> {
    let mut db = db.lock().await;
    // Create a list of buttons and send them.
    let contact = msg.contact().unwrap();
    if db.if_user_exists(&contact.user_id).await? {
        log::info!("User already exists");
        bot.send_message(msg.chat.id, "Ти вже з нами!")
            .reply_markup(
                make_keyboard(vec![
                    MenuCommands::MyGymTrainings.to_string(),
                    MenuCommands::MyHomeTrainings.to_string(),
                    MenuCommands::MyDiet.to_string(),
                ])
                .one_time_keyboard(true),
            )
            .await?;
        dialogue
            .update(State::GetEmail {
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
    Ok(())
}

async fn change_menu(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    if let Some(menu_button) = msg.text() {
        let menu_button = MenuCommands::from(menu_button.to_string());
        let trainings_buttons = vec![
            MenuCommands::MyGymTrainings.to_string(),
            MenuCommands::MyHomeTrainings.to_string(),
            MenuCommands::MyDiet.to_string(),
        ];

        let diet_buttons = vec![
            MenuCommands::MyGymTrainings.to_string(),
            MenuCommands::MyHomeTrainings.to_string(),
            MenuCommands::MyDiet.to_string(),
        ];

        match menu_button {
            MenuCommands::MyHomeTrainings => {
                log::info!("Ви вибрали: {}", menu_button);
                let keyboard = make_keyboard(trainings_buttons);
                bot.send_message(msg.chat.id, "Меню:")
                    .reply_markup(keyboard)
                    .await?;
            }
            MenuCommands::MyGymTrainings => {
                log::info!("Ви вибрали: {}", menu_button);
                let keyboard = make_keyboard(trainings_buttons);
                bot.send_message(msg.chat.id, "Меню:")
                    .reply_markup(keyboard)
                    .await?;
            }
            MenuCommands::MyDiet => {
                let keyboard = make_keyboard(diet_buttons);
                bot.send_message(msg.chat.id, "Меню:")
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    }
    Ok(())
}
