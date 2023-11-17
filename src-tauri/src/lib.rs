use postgrest::Postgrest;
use dotenv::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
struct FormCard {
    character: String,
    pinyin: String,
    translation: String,
}

let client = Postgrest::new("https://gybhoyqqcrvxwbsqfzpi.supabase.co/rest/v1/")
    .insert_header(
        "apikey",
        env::var("SUPABASE_KEY").unwrap());

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
    
    println!("test {:?}", response.url());
    println!("Response: {:?}", response.text().await.unwrap());
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn add_card(card: FormCard) -> String {
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
