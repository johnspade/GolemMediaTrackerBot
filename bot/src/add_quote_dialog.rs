use crate::domain::{Quote, DialogType, ResultCaseInsensitive, State};
use crate::dialogs::{dialog_step, dispose_dialog};

use frankenstein::Update;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AddQuoteDialogResult {
    quote: Option<Quote>,
}

pub fn add_quote_dialog_step(state: &mut State, user_id: u64, dialog_id: Uuid, invocation_key: String, update: &Update) -> Result<(), String> {
    let first_result = dialog_step::<ResultCaseInsensitive<AddQuoteDialogResult, String>>(
        DialogType::AddQuote.template(), dialog_id, invocation_key, update,
    )?;
    println!("{:?}", first_result);
    match first_result {
        ResultCaseInsensitive::Ok(quote_opt) => {
            // If the quote exists, push it to the state and dispose of the dialog
            if let Some(quote) = quote_opt.quote {
                state.quotes.entry(user_id).or_insert(vec![]).push(quote);
                dispose_dialog(state, user_id, DialogType::AddQuote.template(), dialog_id);
            }
            Ok(())
        }
        ResultCaseInsensitive::Err(err) => {
            Err(format!("Error in dialog step: {}", err))
        }
    }
}
