use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tauri::{async_runtime, AppHandle, Emitter, State};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transfer {
    id: i64,
    block_number: i64,
    transaction_hash: String,
    log_index: i32,
    from_address: String,
    to_address: String,
    amount: String,
    contract_address: String,
    created_at: Option<String>,
}

struct SseState {
    running: bool,
}

#[tauri::command]
async fn start_listening_sse(
    app: AppHandle,
    state: State<'_, Arc<Mutex<SseState>>>,
) -> Result<(), String> {
    let state_clone = state.inner().clone();
    let app_clone = app.clone();

    {
        let mut sse_state = state_clone.lock().unwrap();
        if sse_state.running {
            return Ok(());
        }
        sse_state.running = true;
    }

    println!("Starting listening SSE");

    async_runtime::spawn(async move {
        let url = "http://localhost:3000/transfers/stream";

        let mut retries = 10;

        while retries > 0 {
            let client = match reqwest::Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
            {
                Ok(c) => c,
                Err(err) => {
                    let _ = app_clone.emit("sse-error", format!("Error creating client: {}", err));
                    return;
                }
            };

            let resp = match client.get(url).send().await {
                Ok(r) => r,
                Err(err) => {
                    let _ = app_clone.emit("sse-error", format!("Error sending request: {}", err));
                    retries -= 1;
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            };

            let mut stream = resp.bytes_stream();
            let mut buffer = Vec::new();
            let mut seen = HashSet::new();

            while let Some(item) = stream.next().await {
                println!("Received item: {item:?}");

                match item {
                    Ok(chunk) => {
                        buffer.extend_from_slice(&chunk);

                        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                            let line = buffer.drain(..=pos).collect::<Vec<u8>>();

                            if buffer.len() > 1024 * 1024 {
                                buffer.clear();
                                println!("Buffer cleared due to size limit");
                            }

                            if let Ok(line_str) = String::from_utf8(line) {
                                let line_str = line_str.trim();
                                if !line_str.is_empty() && !line_str.contains(": keep-alive") {
                                    let data = line_str.strip_prefix("data: ").unwrap_or(line_str);
                                    if seen.insert(data.to_string()) {
                                        if let Ok(transfer) = serde_json::from_str::<Transfer>(data)
                                        {
                                            let _ = app_clone.emit("sse-update", transfer);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        let _ = app_clone.emit("sse-error", format!("Stream error: {}", err));
                        retries -= 1;
                        sleep(Duration::from_secs(2)).await;
                        break;
                    }
                }
            }
        }

        let mut sse_state = state_clone.lock().unwrap();
        sse_state.running = false;
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
    let url = "http://localhost:3000/transfers";

    println!("Sending GET request to {}", url);

    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Error fetching data: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Error fetching data by server: {}",
            response.status()
        ));
    }

    let text = response
        .text()
        .await
        .map_err(|e| format!("Error reading response text: {}", e))?;

    let transfers: Vec<Transfer> =
        serde_json::from_str(&text).map_err(|e| format!("Error parsing JSON: {}", e))?;

    Ok(transfers)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use std::sync::Arc;

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(SseState { running: false })))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_initial_data,
            start_listening_sse,
            stop_listening_sse
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use futures_util::stream;
    use httpmock::prelude::*;
    use tokio;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_get_initial_data_success() {
        let server = MockServer::start();

        let transfers_json = r#"[{
            "id": 1,
            "block_number": 12345,
            "transaction_hash": "0xabc",
            "log_index": 1,
            "from_address": "0xfrom",
            "to_address": "0xto",
            "amount": "1000",
            "contract_address": "0xcontract",
            "created_at": null
        }]"#;

        server.mock(|when, then| {
            when.method(GET).path("/transfers");
            then.status(200)
                .header("content-type", "application/json")
                .body(transfers_json);
        });

        let url = format!("{}/transfers", server.base_url());
        let response = reqwest::get(&url).await.unwrap();
        let text = response.text().await.unwrap();
        let transfers: Vec<Transfer> = serde_json::from_str(&text).unwrap();

        assert_eq!(transfers.len(), 1);
        assert_eq!(transfers[0].from_address, "0xfrom");
    }

    #[test]
    fn test_sse_parsing() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let json_1 = "data: {\"id\":1,\"block_number\":10,\"transaction_hash\":\"abc\",\"log_index\":0,   \"from_address\":\"A\",\"to_address\":\"B\",\"amount\":\"100\",\"contract_address\":\"X\"}\n";
            let json_2 = "data: {\"id\":2,\"block_number\":11,\"transaction_hash\":\"def\",\"log_index\":1,\"from_address\":\"C\",\"to_address\":\"D\",\"amount\":\"200\",\"contract_address\":\"Y\"}\n";

            let fake_stream = stream::iter(vec![
                Ok::<_, reqwest::Error>(Bytes::from(json_1)),
                Ok(Bytes::from(json_2)),
            ]);

            let mut buffer = Vec::new();
            let mut seen = std::collections::HashSet::new();

            let mut count = 0;
            tokio::pin!(fake_stream);

            while let Some(item) = fake_stream.next().await {
                buffer.extend_from_slice(&item.unwrap());

                while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                    let line = buffer.drain(..=pos).collect::<Vec<u8>>();
                    let line_str = String::from_utf8(line).unwrap();
                    if let Some(data) = line_str.strip_prefix("data: ") {
                        if seen.insert(data.to_string()) {
                            if let Ok(_transfer) = serde_json::from_str::<Transfer>(data) {
                                count += 1;
                            }
                        }
                    }
                }
            }

            assert_eq!(count, 2);
        });
    }
}
