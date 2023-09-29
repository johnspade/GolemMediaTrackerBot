cargo_component_bindings::generate!();
use crate::bindings::exports::golem::template::api::*;

use dialog_engine::HasDialogMessage;
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
    EnterText,
    // (Text)
    EnterTitle(String),
    // (Text, Title)
    EnterAuthor(String, String),
    // (Text, Title, Author)
    Completed(String, String, String),
}

#[derive(Debug)]
enum Event {
    Start,
    ProvideText(String),
    ProvideTitle(String),
    ProvideAuthor(String),
}

impl DialogState {
    fn next(self, event: Event) -> Self {
        use DialogState::*;
        use Event::*;

        match (self, event) {
            (Started, Start) => EnterText,
            (EnterText, ProvideText(text)) => EnterTitle(text),
            (EnterTitle(text), ProvideTitle(title)) => EnterAuthor(text, title),
            (EnterAuthor(text, title), ProvideAuthor(author)) => Completed(text, title, author),
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
            EnterText => Some("Enter text".to_string()),
            EnterTitle(_) => Some("Enter title".to_string()),
            EnterAuthor(_, _) => Some("Enter author".to_string()),
            Completed(text, title, author) => Some(format!("Added quote: \"{}\" from {} by {}", text, title, author)),
        }
    }
}

const EMPTY_RESULT: Result<DialogResult, String> = Ok(DialogResult { quote: None });

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
        DialogState::EnterText => {
            if let UpdateContent::Message(ref message) = update.content {
                if let Some(ref text) = message.text {
                    advance_dialog_and_send_message(api, message.chat.id, state, Event::ProvideText(text.clone()));
                }
            }
            EMPTY_RESULT
        }
        DialogState::EnterTitle(_) => {
            if let UpdateContent::Message(ref message) = update.content {
                if let Some(ref text) = message.text {
                    advance_dialog_and_send_message(api, message.chat.id, state, Event::ProvideTitle(text.clone()));
                }
            }
            EMPTY_RESULT
        }
        DialogState::EnterAuthor(_, _) => {
            if let UpdateContent::Message(ref message) = update.content {
                if let Some(ref text) = message.text {
                    let current_state = advance_dialog_state(state, Event::ProvideAuthor(text.clone()));
                    if let DialogState::Completed(ref text, ref title, ref author) = current_state {
                        send_dialog_message(api, message.chat.id, &current_state);
                        return Ok(DialogResult {
                            quote: Some(Quote {
                                text: text.clone(),
                                title: title.clone(),
                                author: author.clone(),
                            })
                        });
                    }
                }
            }
            EMPTY_RESULT
        }
        DialogState::Completed(ref text, ref title, ref author) => {
            Ok(DialogResult {
                quote: Some(Quote {
                    text: text.clone(),
                    title: title.clone(),
                    author: author.clone(),
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

    fn hello() -> String {
        "Hello from Rust!".to_string()
    }

    fn state() -> Result<String, String> {
        with_state(|state| {
            serde_json::to_string(&state.dialog_state)
                .map_err(|err| format!("JSON serialization failed: {}", err))
        })
    }
}
