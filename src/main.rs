use crate::db::db::Db;
use crate::errors::Result;
use crate::models::{MenuCommands, MyDialogue, State};
use crate::utils::{init_logging, make_keyboard};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::types::{ButtonRequest, KeyboardButton};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{KeyboardMarkup, Me},
    utils::command::BotCommands,
};

mod db;
mod models;
mod utils;

mod errors;

/// These commands are supported:
#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text
    Help,
    /// Start
    Start,
    /// get user phone number
    GetPhoneNumber,
    GetEmail {
        email: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {

    init_logging();
    log::info!("Starting buttons bot...");

    let bot = Bot::new(dotenv::var("TELOXIDE_TOKEN")?);

    let datavase_url = dotenv::var("DATABASE_URL")?;

    let db = &mut Db::new(&datavase_url);

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(change_menu));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), db])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

/// Parse the text wrote on Telegram and check if that text is a valid command
/// or not, then match the command. If the command is `/start` it writes a
/// markup with the `InlineKeyboardMarkup`.
async fn message_handler(bot: Bot, msg: Message, me: Me, db: &mut Db) -> Result<()> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                // Just send the description of all commands.
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                // Create a list of buttons and send them.
                let button =
                    KeyboardButton::new("Відправити номер").request(ButtonRequest::Contact);
                let markup = KeyboardMarkup::new([[button]])
                    .one_time_keyboard(true)
                    .resize_keyboard(true);
                bot.send_message(msg.chat.id, "Привіт! Я бот для тренувань! \
                Тут ти можешь знайти для себе тренування у залі, \
                тренування удома, та дієту, спеціально підібрану для тебе! \
                Будь-ласка, відправ спочатку свій номер телефона а потім пошту, для реєстрації або перевірки чи ти вже з нами!")
                    .reply_markup(markup)
                    .await?;
            }
            Ok(Command::GetPhoneNumber) => {
                // Create a list of buttons and send them.
                let contact = msg.contact().unwrap();
                if db.if_user_exists(&contact.phone_number).await? {
                    bot.send_message(msg.chat.id, "Ти вже з нами!")
                        .reply_markup(make_keyboard(vec![
                            MenuCommands::MyGymTrainings.to_string(),
                            MenuCommands::MyHomeTrainings.to_string(),
                            MenuCommands::MyDiet.to_string(),
                        ]))
                        .await?;
                } else {
                    bot.send_message(
                        msg.chat.id,
                        "Ти новий! Дякую за номер, тепер віправ свою пошту!",
                    )
                    .await?;
                    db.insert_user(&contact.first_name, &contact.phone_number, &contact.user_id)
                        .await?;
                }
            }
            Ok(Command::GetEmail { email }) => {
                // Create a list of buttons and send them.
                db.add_email(&email, &msg.chat.id.to_string()).await?;
                bot.send_message(msg.chat.id, "Дякую за пошту! Тепер ти з нами!")
                    .reply_markup(make_keyboard(vec![
                        MenuCommands::MyGymTrainings.to_string(),
                        MenuCommands::MyHomeTrainings.to_string(),
                        MenuCommands::MyDiet.to_string(),
                    ]))
                    .await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
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
