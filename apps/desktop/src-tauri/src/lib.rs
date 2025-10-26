use std::time::Duration;
use serde::Serialize;
use schemars::{schema_for, JsonSchema};
// mod schemas;
// use schemas::get_all_schemas;
// mod ts_generator;
// use ts_generator::write_ts_types;

#[derive(Serialize, JsonSchema)]
struct Transfer {
    time: String,
    from: String,
    to: String,
    token: String,
}

use tauri::{async_runtime, AppHandle, Emitter};
use reqwest::Client;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_listening_sse(app: AppHandle, url: String) -> Result<(), String> {
async_runtime::spawn(async move {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| e.to_string());

        let client = match client {
            Ok(c) => c,
            Err(err) => {
                let _ = app.emit("sse-error", format!("Error in creating client: {}", err));
                return;
            }
        };

        let resp = match client.get(&url).send().await {
            Ok(r) => r,
            Err(err) => {
                let _ = app.emit("sse-error", format!("Error in getting response: {}", err));
                return;
            }
        };

        let body = match resp.text().await {
            Ok(b) => b,
            Err(err) => {
                let _ = app.emit("sse-error", format!("Error in getting body: {}", err));
                return;
            }
        };

        for line in body.lines() {
            if line.starts_with("data: ") {
                let payload = line.trim_start_matches("data: ");
                if let Err(err) = app.emit("sse-update", payload.to_string()) {
                    eprintln!("Error in emitting event: {:?}", err);
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn stop_listening_sse(app: AppHandle) -> Result<(), String> {
    let _ = match app.emit("sse-update", "stop") {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error in stopping listening event: {}", err)),
    };
    Ok(())
}

#[tauri::command]
async fn get_initial_data() -> Result<Vec<Transfer>, String> {

    //TODO : Replace with actual data
    let data = vec![
        Transfer { time: "2025-10-02 10:15".to_string(), from: "0xAlice".to_string(), to: "0xBob".to_string(), token: "USDT".to_string() },
        Transfer { time: "2025-10-02 10:17".to_string(), from: "0xEve".to_string(), to: "0xCarl".to_string(), token: "USDC".to_string() },
        Transfer { time: "2025-10-02 10:19".to_string(), from: "0xDan".to_string(), to: "0xMike".to_string(), token: "DAI".to_string() },
        Transfer { time: "2025-10-02 10:21".to_string(), from: "0xTom".to_string(), to: "0xSue".to_string(), token: "USDT".to_string() },
    ];
    Ok(data)
}

#[tauri::command]
fn get_transfer_schema() -> String {
    let schema = schema_for!(Transfer);
    serde_json::to_string(&schema).unwrap()
}

// #[tauri::command]
// fn schemas() -> serde_json::Value {
//     get_all_schemas()
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ts_generator::write_ts_types("gen").unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_initial_data, get_transfer_schema ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
