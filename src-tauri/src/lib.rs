use openapiv3::OpenAPI;
use serde_json::from_str;
use std::error::Error;

#[tauri::command]
fn parse_openapi(content: &str) -> Result<String, String> {
    match from_str::<OpenAPI>(content) {
        Ok(openapi) => Ok(format!("Parsed OpenAPI: {:?}", openapi)),
        Err(e) => Err(format!("Failed to parse OpenAPI: {}", e)),
    }
}

#[tauri::command]
fn parse_openapi_sample(content: &str) -> String {
        format!("Parsed OpenAPI: {:?}", content)
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![parse_openapi])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
