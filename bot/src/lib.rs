cargo_component_bindings::generate!();

mod add_book_dialog;
mod add_movie_dialog;
mod add_quote_dialog;
mod bot;
mod dialogs;
mod env;
mod domain;
mod workers;


use crate::bindings::exports::golem::template::api::*;

struct Component;

impl Guest for Component {
    fn start_bot() {
        domain::with_state(|state| {
            bot::handle_updates(state);
        })
    }
}
