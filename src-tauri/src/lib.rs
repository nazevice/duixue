use postgrest::Postgrest;
use dotenv::dotenv;
use std::env;
use base64::{Engine as _, engine::{self, general_purpose}};
use serde::Deserialize;
use reqwest::header::HeaderValue;
use supabase_storage::Storage;

#[derive(Debug, serde::Deserialize)]
struct FormCard {
    character: String,
    pinyin: String,
    translation: String,
    file_base64: String
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: String) -> String {
    let client = Postgrest::new("https://gybhoyqqcrvxwbsqfzpi.supabase.co/rest/v1/")
    .insert_header(
        "apikey",
        env::var("SUPABASE_KEY").unwrap());
        let response = client
            .from("duixue")
            .select("*")
            .execute()
            .await
            .expect("Failed to execute query");
    
    println!("test {:?}", response);
    println!("Response: {:?}", response.text().await.unwrap());
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn add_card(card: FormCard) -> String {
    let file_data = general_purpose::STANDARD.decode(card.file_base64.split(',').last().unwrap()).expect("Failed to decode base64");
    let client = reqwest::Client::new();
    let response = client
    .post("https://gybhoyqqcrvxwbsqfzpi.supabase.co/storage/v1/object/chinese/hsk1/test.jpg")
    .header("Authorization", format!("Bearer {}", env::var("SUPABASE_KEY").unwrap()))
    .header("Content-Type", "application/octet-stream")
    .body(file_data)
    .send()
    .await
    .expect("Failed to send request");

    println!("{:?}", response);
    println!("test {:?}", card);

    format!("Received: {:?}", card)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, add_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
