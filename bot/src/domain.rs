use once_cell::sync::Lazy;
use serde::Deserialize;
use uuid::Uuid;

use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum DialogType {
    AddBook,
    AddMovie,
    AddQuote
}

impl DialogType {
    pub fn template(&self) -> &str {
        match self {
            DialogType::AddBook => "d6e1ea5b-40aa-4f9c-92e2-9db58c02b45f",
            DialogType::AddMovie => "9ac53019-2336-468c-916d-cd46c63bc24b",
            DialogType::AddQuote => "c386feb3-fdfb-4e6a-a24c-cae39cd393f0",
        }
    }
}

#[derive(Clone)]
pub struct Dialog {
    pub dialog_type: DialogType,
    pub dialog_id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub rating: u32,
}

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub text: String,
    pub title: String,
    pub author: String,
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub title: String,
    pub year: u32,
    pub rating: u32,
}

#[derive(Deserialize, Debug)]
pub enum ResultCaseInsensitive<T, E> {
    #[serde(alias = "Ok", alias = "ok")]
    Ok(T),
    #[serde(alias = "Err", alias = "err")]
    Err(E),
}

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
pub struct State {
    pub dialogs: Lazy<HashMap<u64, Dialog>>,
    pub books: Lazy<HashMap<u64, Vec<Book>>>,
    pub movies: Lazy<HashMap<u64, Vec<Movie>>>,
    pub quotes: Lazy<HashMap<u64, Vec<Quote>>>,
}

/// This holds the state of our application.
/// It is a global variable, which Rust doesn't like, so
/// we use `with_state` to access or update the global variable, so we
/// can avoid `unsafe` noise.
pub static mut STATE: State = State {
    dialogs: Lazy::new(|| HashMap::new()),
    books: Lazy::new(|| HashMap::new()),
    movies: Lazy::new(|| HashMap::new()),
    quotes: Lazy::new(|| HashMap::new()),
};

pub fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    unsafe { f(&mut STATE) }
}
