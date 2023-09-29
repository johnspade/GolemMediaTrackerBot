use crate::domain::{Book, DialogType, ResultCaseInsensitive, State};
use crate::dialogs::{dialog_step, dispose_dialog};

use frankenstein::Update;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AddBookDialogResult {
    book: Option<Book>,
}

pub fn add_book_dialog_step(state: &mut State, user_id: u64, dialog_id: Uuid, invocation_key: String, update: &Update) -> Result<(), String> {
    let first_result = dialog_step::<ResultCaseInsensitive<AddBookDialogResult, String>>(
        DialogType::AddBook.template(), dialog_id, invocation_key, update,
    )?;
    println!("{:?}", first_result);
    match first_result {
        ResultCaseInsensitive::Ok(book_opt) => {
            // If the book exists, push it to the state and dispose of the dialog
            if let Some(book) = book_opt.book {
                state.books.entry(user_id).or_insert(vec![]).push(book);
                dispose_dialog(state, user_id, DialogType::AddBook.template(), dialog_id);
            }
            Ok(())
        }
        ResultCaseInsensitive::Err(err) => {
            Err(format!("Error in dialog step: {}", err))
        }
    }
}
