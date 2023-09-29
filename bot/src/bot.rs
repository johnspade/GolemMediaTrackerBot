use crate::add_book_dialog::*;
use crate::add_movie_dialog::*;
use crate::add_quote_dialog::*;
use crate::domain::{Dialog, DialogType, State};
use crate::dialogs::{create_dialog, dispose_dialog};
use crate::env::TELEGRAM_TOKEN;
use crate::workers::get_invocation_key;

use frankenstein::{AllowedUpdate, CallbackQuery, GetUpdatesParams, Message, Update, UpdateContent};
use telegram_api::*;

pub fn handle_updates(state: &mut State) {
    let api = Api::new(TELEGRAM_TOKEN.as_str());
    let mut update_params = GetUpdatesParams {
        offset: None,
        limit: None,
        timeout: Some(10u32),
        allowed_updates: Some(vec![
            AllowedUpdate::Message,
            AllowedUpdate::CallbackQuery,
        ]),
    };
    loop {
        let result = &api.get_updates(&update_params);
        match result {
            Ok(updates) => {
                for update in &updates.result {
                    update_params.offset = Some((update.update_id + 1).into());
                    match update.content {
                        UpdateContent::Message(ref message) => {
                            on_message(state, &api, message, &update);
                        }
                        UpdateContent::CallbackQuery(ref callback_query) => {
                            on_callback_query(state, callback_query, &update);
                        }
                        _ => (),
                    };
                }
            }
            Err(e) => {
                println!("Error while receiving updates: {:?}", e);
            }
        }
    };
}

fn on_message(state: &mut State, api: &Api, message: &Message, update: &Update) -> () {
    // if user is in dialog state, send message to dialog worker
    // else if user is not in dialog state, handle message
    if let Some(user) = &message.from.as_ref() {
        let user_id = user.id;
        if let Some(dialog) = state.dialogs.get(&user_id).cloned() {
            if let Some(text) = &message.text {
                if text.starts_with("/reset") {
                    dispose_dialog(state, user_id, dialog.dialog_type.template(), dialog.dialog_id);
                    send_message(api, message.chat.id, "Dialog reset");
                    return;
                }
            }
            dispatch_dialog(state, update, user_id, dialog);
            return;  // Early return to avoid going to the next block
        }
        // Fallback if we didn't return early
        handle_commands(state, api, message, update, user_id);
    }

}

fn on_callback_query(state: &mut State, cb: &CallbackQuery, update: &Update) -> () {
    let user_id = cb.from.id;
    if let Some(dialog) = state.dialogs.get(&user_id).cloned() {
        dispatch_dialog(state, update, user_id, dialog);
    }
}

fn dispatch_dialog(state: &mut State, update: &Update, user_id: u64, dialog: Dialog) {
    let invocation_key_result = get_invocation_key(dialog.dialog_id, dialog.dialog_type.template());

    let invocation_key = match invocation_key_result {
        Ok(key) => key,
        Err(err) => {
            println!("Error getting invocation key: {}", err);
            return;
        }
    };

    match dialog.dialog_type {
        DialogType::AddBook => {
            let result = add_book_dialog_step(state, user_id, dialog.dialog_id, invocation_key, update);
            if let Err(err) = result {
                println!("Error in add_book_dialog_step: {}", err);
            };
        },
        DialogType::AddMovie => {
            let result = add_movie_dialog_step(state, user_id, dialog.dialog_id, invocation_key, update);
            if let Err(err) = result {
                println!("Error in add_movie_dialog_step: {}", err);
            };
        },
        DialogType::AddQuote => {
            let result = add_quote_dialog_step(state, user_id, dialog.dialog_id, invocation_key, update);
            if let Err(err) = result {
                println!("Error in add_quote_dialog_step: {}", err);
            };
        }
    }
}


fn handle_commands(state: &mut State, api: &Api, message: &Message, update: &Update, user_id: u64) {
    if let Some(text) = &message.text {
        let chat_id = message.chat.id;
        if text.starts_with("/start") {
            let text = "Use /add_book, /add_movie or /add_quote to add a new item. Use /books, /movies or /quotes to list your items.";
            send_message(api,  chat_id, &text);
        } else if text.starts_with("/add_book") {
            let result = create_dialog(state, user_id, DialogType::AddBook, update, add_book_dialog_step);
            if let Err(err) = result {
                println!("Error starting add book dialog: {}", err);
            };
        } else if text.starts_with("/add_movie") {
            let result = create_dialog(state, user_id, DialogType::AddMovie, update, add_movie_dialog_step);
            if let Err(err) = result {
                println!("Error starting add movie dialog: {}", err);
            };
        } else if text.starts_with("/add_quote") {
            let result = create_dialog(state, user_id, DialogType::AddQuote, update, add_quote_dialog_step);
            if let Err(err) = result {
                println!("Error starting add quote dialog: {}", err);
            };
        } else if text.starts_with("/books") {
            if let Some(books) = state.books.get(&user_id) {
                let mut text = "Your books:\n".to_string();
                for book in books {
                    text.push_str(&format!("{} by {} (rating: {})\n", book.title, book.author, book.rating));
                }
                send_message(api, chat_id, &text);
            } else {
                send_message(api, chat_id, "You have no books");
            }
        } else if text.starts_with("/movies") {
            if let Some(movies) = state.movies.get(&user_id) {
                let mut text = "Your movies:\n".to_string();
                for movie in movies {
                    text.push_str(&format!("{} ({}) (rating: {})\n", movie.title, movie.year, movie.rating));
                }
                send_message(api, chat_id, &text);
            } else {
                send_message(api, chat_id, "You have no movies");
            }
        } else if text.starts_with("/quotes") {
            if let Some(quotes) = state.quotes.get(&user_id) {
                let mut text = "Your quotes:\n".to_string();
                for quote in quotes {
                    text.push_str(&format!("\"{}\" from {} by {}", quote.text, quote.title, quote.author));
                }
                send_message(api, chat_id, &text);
            } else {
                send_message(api, chat_id, "You have no quotes");
            }
        }
    }
}
