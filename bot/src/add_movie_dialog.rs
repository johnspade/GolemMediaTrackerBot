use crate::domain::{Movie, DialogType, ResultCaseInsensitive, State};
use crate::dialogs::{dialog_step, dispose_dialog};

use frankenstein::Update;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AddMovieDialogResult {
    movie: Option<Movie>
}

pub fn add_movie_dialog_step(state: &mut State, user_id: u64, dialog_id: Uuid, invocation_key: String, update: &Update) -> Result<(), String> {
    let first_result = dialog_step::<ResultCaseInsensitive<AddMovieDialogResult, String>>(
        DialogType::AddMovie.template(), dialog_id, invocation_key, update
    )?;
    println!("{:?}", first_result);
    match first_result {
        ResultCaseInsensitive::Ok(movie_opt) => {
            // If the movie exists, push it to the state and dispose of the dialog
            if let Some(movie) = movie_opt.movie {
                state.movies.entry(user_id).or_insert(vec![]).push(movie);
                dispose_dialog(state, user_id, DialogType::AddMovie.template(), dialog_id);
            }
            Ok(())
        },
        ResultCaseInsensitive::Err(err) => {
            Err(format!("Error in dialog step: {}", err))
        }
    }
}
