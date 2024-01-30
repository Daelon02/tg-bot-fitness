use crate::models::Command;
use crate::models::{MyDialogue, State};
use teloxide::prelude::*;
use teloxide::requests::Requester;
use teloxide::types::{ButtonRequest, KeyboardButton, KeyboardMarkup};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

pub async fn help(bot: Bot, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
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

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> crate::errors::Result<()> {
    bot.send_message(
        msg.chat.id,
        "Я тебе не розумію, подивись будь-ласка на команду /help",
    )
    .await?;
    Ok(())
}
