cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;

use dialog_engine::{HasDialogMessage, validate_rating};
use frankenstein::{Update, UpdateContent};
use once_cell::sync::Lazy;
use serde::Serialize;
use telegram_api::*;

use std::env;

struct Component;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    dialog_state: DialogState,
}

/// This holds the state of our application.
/// It is a global variable, which Rust doesn't like, so
/// we use `with_state` to access or update the global variable, so we
/// can avoid `unsafe` noise.
static mut STATE: State = State {
    dialog_state: DialogState::Started
};

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    unsafe { f(&mut STATE) }
}

static TELEGRAM_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("TELEGRAM_TOKEN").unwrap()
});

#[derive(Debug, Clone, Serialize)]
enum DialogState {
    Started,
    EnterTitle,
    EnterAuthor(String),
    // (Title)
    EnterRating(String, String),
    // (Title, Author)
    Completed(String, String, u32), // (Title, Author, Rating)
}

#[derive(Debug)]
enum Event {
    Start,
    ProvideTitle(String),
    ProvideAuthor(String),
    ProvideRating(u32),
}

impl DialogState {
    fn next(self, event: Event) -> Self {
        use DialogState::*;
        use Event::*;

        match (self, event) {
            (Started, Start) => EnterTitle,
            (EnterTitle, ProvideTitle(title)) => EnterAuthor(title),
            (EnterAuthor(title), ProvideAuthor(author)) => EnterRating(title, author),
            (EnterRating(title, author), ProvideRating(rating)) => Completed(title, author, rating),
            (state, event) => {
                println!("Unexpected state transition: {:?} -> {:?}", &state, &event);
                state
            }
        }
    }
}

impl HasDialogMessage for DialogState {
    fn message(&self) -> Option<String> {
        use DialogState::*;

        match self {
            Started => None,
            EnterTitle => Some("Enter title".to_string()),
            EnterAuthor(_) => Some("Enter author".to_string()),
            EnterRating(_, _) => Some("Enter rating".to_string()),
            Completed(title, author, rating) => Some(format!("Added book {} by {} with rating {}", title, author, rating)),
        }
    }
}

const EMPTY_RESULT: Result<DialogResult, String> = Ok(DialogResult { book: None });

fn advance_dialog_state(state: &mut State, event: Event) -> DialogState {
    let new_dialog_state = state.dialog_state.clone().next(event);
    state.dialog_state = new_dialog_state.clone();
    new_dialog_state
}

fn advance_dialog_and_send_message(api: &Api, chat_id: i64, state: &mut State, event: Event) {
    let new_state = advance_dialog_state(state, event);
    send_dialog_message(api, chat_id, &new_state);
}

fn handle_update(state: &mut State, api: &Api, update: &Update) -> Result<DialogResult, String> {
    let dialog_state = &state.dialog_state;
    println!("Dialog state: {:?}", dialog_state);
    match dialog_state {
        DialogState::Started => {
            if let UpdateContent::Message(ref message) = update.content {
                advance_dialog_and_send_message(api, message.chat.id, state, Event::Start);
            }
            EMPTY_RESULT
        }
        DialogState::EnterTitle => {
            if let UpdateContent::Message(ref message) = update.content {
                if let Some(ref text) = message.text {
                    advance_dialog_and_send_message(api, message.chat.id, state, Event::ProvideTitle(text.clone()));
                }
            }
            EMPTY_RESULT
        }
        DialogState::EnterAuthor(_) => {
            if let UpdateContent::Message(ref message) = update.content {
                if let Some(ref text) = message.text {
                    advance_dialog_and_send_message(api, message.chat.id, state, Event::ProvideAuthor(text.clone()));
                }
            }
            EMPTY_RESULT
        }
        DialogState::EnterRating(_, _) => {
            if let UpdateContent::Message(ref message) = update.content {
                if let Some(ref text) = message.text {
                    // rating is a number between 1 and 5, validate and send error message if invalid
                    let rating_result = validate_rating(text);
                    match rating_result {
                        Ok(rating) => {
                            advance_dialog_state(state, Event::ProvideRating(rating));
                        }
                        Err(err) => {
                            send_message(api, message.chat.id, err);
                            return EMPTY_RESULT;
                        }
                    }
                }
                let current_state = &state.dialog_state;
                if let DialogState::Completed(ref title, ref author, ref rating) = current_state {
                    send_dialog_message(api, message.chat.id, &current_state);
                    return Ok(DialogResult {
                        book: Some(Book {
                            title: title.clone(),
                            author: author.clone(),
                            rating: *rating,
                        })
                    });
                }
            }
            EMPTY_RESULT
        }
        DialogState::Completed(ref title, ref author, ref rating) => {
            Ok(DialogResult {
                book: Some(Book {
                    title: title.clone(),
                    author: author.clone(),
                    rating: *rating,
                })
            })
        }
    }
}

fn send_dialog_message(api: &Api, chat_id: i64, dialog_state: &DialogState) {
    if let Some(message) = dialog_state.message() {
        send_message(api, chat_id, &message);
    }
}

impl Guest for Component {
    fn step(update: String) -> Result<DialogResult, String> {
        with_state(|state| {
            let update: Update = serde_json::from_str(&update)
                .map_err(|err| format!("Update JSON deserialization failed: {}", err))?;
            let api = Api::new(TELEGRAM_TOKEN.as_str());
            handle_update(state, &api, &update)
        })
    }
}
