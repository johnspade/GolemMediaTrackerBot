use crate::domain::{Dialog, DialogType, State};
use crate::env::TELEGRAM_TOKEN;
use crate::workers::*;

use frankenstein::Update;
use uuid::Uuid;

use std::fmt::Debug;
use serde::de::DeserializeOwned;

pub fn create_dialog<F>(
    state: &mut State,
    user_id: u64,
    dialog_type: DialogType,
    update: &Update,
    step: F,
) -> Result<(), String>
    where F: Fn(&mut State, u64, Uuid, String, &Update) -> Result<(), String> {
    let dialog_id = Uuid::new_v4();
    let env = vec!("TELEGRAM_TOKEN", TELEGRAM_TOKEN.as_str());
    if let Err(err) = create_worker(dialog_id, dialog_type.template(), vec!(env)) {
        return Err(format!("Failed to create worker: {}", err));
    }

    state.dialogs.insert(user_id, Dialog {
        dialog_type,
        dialog_id,
    });

    let invocation_key = get_invocation_key(dialog_id, dialog_type.template())
        .map_err(|err| format!("Error getting invocation key: {}", err))?;

    step(state, user_id, dialog_id, invocation_key, update)
        .map_err(|err| format!("Error in dialog step: {}", err))
}

pub fn dialog_step<T: DeserializeOwned + Debug>(template: &str, dialog_id: Uuid, invocation_key: String, update: &Update) -> Result<T, String> {
    let update_param = serde_json::to_string(update)
        .map_err(|err| format!("Update serialization error: {}", err))?;

    let response_json = invoke_function(template, dialog_id, invocation_key, "golem%3Atemplate%2Fapi%2Fstep", update_param)
        .map_err(|err| format!("Dialog step failed: {}", err))?;
    println!("{:?}", response_json);

    // Deserialize JSON and handle any error
    let parsed_response = serde_json::from_value::<FunctionResult<T>>(response_json)
        .map_err(|err| format!("JSON deserialization failed: {}", err))?;

    // Handle the Result inside the parsed_response
    let first_result = parsed_response
        .result
        .into_iter()
        .next()
        .ok_or("No result found in dialog step".to_string());
    first_result
}

pub fn dispose_dialog(state: &mut State, user_id: u64, template: &str, dialog_id: Uuid) {
    state.dialogs.remove(&user_id);
    delete_worker(template, dialog_id)
}
