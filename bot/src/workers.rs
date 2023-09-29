use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use std::env;

static GOLEM_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("GOLEM_TOKEN").unwrap()
});
const API_ROOT: &str = "https://release.api.golem.cloud/v1";

#[derive(Deserialize, Debug)]
pub struct FunctionResult<T> {
    pub result: Vec<T>
}

pub fn create_worker(worker_id: Uuid, template: &str, env: Vec<Vec<&str>>) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("{}/templates/{}/workers", API_ROOT, template);
    // example body: {"name":"46c2db15-f9d3-4a0c-9f12-ef3116391c8c","env":[],"args":[]}
    let body = serde_json::json!({
        "name": worker_id.to_string(),
        "env": env,
        "args": []
    });
    let response = client.post(&url)
        .json(&body)
        .bearer_auth(GOLEM_TOKEN.as_str())
        .send()
        .map_err(|err| format!("Request error: {}", err))?;
    println!("create worker response: {:?}", response);

    // Check if the HTTP request was successful
    if response.status() != StatusCode::OK {
        return Err(format!("Create worker: Received non-OK HTTP status: {}", response.status()));
    }

    // Parse JSON and handle JSON errors
    let json_value = response.json::<serde_json::Value>()
        .map_err(|err| format!("JSON error: {}", err))?;

    // example response body: {"workerId":{"rawTemplateId":"753b8b37-83ab-4752-8829-3e057d89a74b","workerName":"46c2db15-f9d3-4a0c-9f12-ef3116391c8c"},"templateVersionUsed":0}
    // Check if the workerName in the response matches the given worker_id
    if let Some(response_worker_id) = json_value
        .get("workerId")
        .and_then(|worker_id| worker_id.get("workerName"))
        .and_then(|worker_name| worker_name.as_str())
    {
        return if worker_id.to_string() == response_worker_id {
            println!("Created worker with ID {}", worker_id);
            Ok(())
        } else {
            Err(format!("Create worker: Mismatched worker IDs: expected {}, got {}", worker_id, response_worker_id))
        }
    }

    Err("Create worker: Unexpected JSON response".to_string())
}

pub fn get_invocation_key(worker_id: Uuid, template: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/templates/{}/workers/{}/key", API_ROOT, template, worker_id);

    // Send request and handle request errors
    let response = client.post(&url)
        .bearer_auth(GOLEM_TOKEN.as_str())
        .send()
        .map_err(|err| format!("Request error: {}", err))?;

    // Parse JSON and handle JSON errors
    let json_value = response.json::<serde_json::Value>()
        .map_err(|err| format!("JSON error: {}", err))?;

    // Get the key from the JSON value field
    let key = json_value.get("value")
        .ok_or_else(|| "Missing field in JSON response".to_string())?
        .as_str()
        .ok_or_else(|| "Missing field in JSON response".to_string())?;

    Ok(key.to_string())
}

pub fn delete_worker(template: &str, worker_id: Uuid) {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/templates/{}/workers/{}",
        API_ROOT,
        template,
        worker_id
    );
    let response = client.delete(&url).bearer_auth(GOLEM_TOKEN.as_str()).send();
    println!("delete worker response: {:?}", response);
}

pub fn invoke_function(template: &str, worker_id: Uuid, invocation_key: String, function: &str, params: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/templates/{}/workers/{}/invoke-and-await?invocation-key={}&function={}",
        API_ROOT,
        template,
        worker_id,
        invocation_key,
        function
    );
    let body = serde_json::json!({
        "params": [params]
    });
    println!("{:?}", body);
    let response = client.post(&url)
        .json(&body)
        .bearer_auth(GOLEM_TOKEN.as_str())
        .send()
        .map_err(|err| format!("Invoke function: Request error: {}", err))?;

    // Check if the HTTP request was successful
    if response.status() != StatusCode::OK {
        return Err(format!("Invoke function: Received non-OK HTTP status: {}", response.status()));
    }

    // Parse JSON and handle JSON errors
    let json_value = response.json::<serde_json::Value>()
        .map_err(|err| format!("Invoke function: JSON error: {}", err))?;

    Ok(json_value)
}
