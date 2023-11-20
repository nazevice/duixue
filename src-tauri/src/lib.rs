use postgrest::Postgrest;
use dotenv::dotenv;
use std::env;
use base64::{Engine as _, engine::{self, general_purpose}};
use serde::{Serialize, Deserialize};
use reqwest::header::HeaderValue;
use supabase_storage::Storage;
use serde_json;
use std::sync::{Arc, Mutex};
use tauri::State;

struct ClientTable {
    client: Postgrest
}

#[derive(Debug, Serialize, Deserialize)]
struct FormCard {
    character: String,
    pinyin: String,
    translation: String,
    file_path: String
}

#[derive(Serialize)]
struct CardDeck {
    name: String,
    description: String,
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
async fn add_card(mut card: FormCard, file_base64: String) -> String {
    let file_data = general_purpose::STANDARD.decode(file_base64.split(',').last().unwrap()).expect("Failed to decode base64");
    let client = reqwest::Client::new();
    card.file_path = "hsk1/".to_owned() + &card.file_path;
    let response = client
    .post("https://gybhoyqqcrvxwbsqfzpi.supabase.co/storage/v1/object/chinese/".to_owned() + &card.file_path)
    .header("Authorization", format!("Bearer {}", env::var("SUPABASE_KEY").unwrap()))
    .header("Content-Type", "application/octet-stream")
    .body(file_data)
    .send()
    .await
    .expect("Failed to send request");
    println!("{:?}", response);
    println!("test {:?}", card);
    let card_json = serde_json::to_string(&card).expect("Failed to serialize card");
    let client_table = Postgrest::new("https://gybhoyqqcrvxwbsqfzpi.supabase.co/rest/v1/")
    .insert_header("apikey", env::var("SUPABASE_KEY").unwrap());
    let response_insert = client_table.from("chinese").insert(card_json).execute().await.expect("Failed to insert card into table");

    format!("Second response: {:?}", response_insert)
}

#[tauri::command]
async fn get_card_decks(state: State<'_,ClientTable>) -> Result<String, ()> {
    let response = state.client
            .from("card_decks")
            .select("*")
            .execute()
            .await
            .expect("Failed to execute query");
    
    println!("test {:?}", response);
    println!("Response: {:?}", response.text().await.unwrap());
    Ok(format!("Data from backend"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();
    let client = Postgrest::new("https://gybhoyqqcrvxwbsqfzpi.supabase.co/rest/v1")
    .insert_header(
        "apikey",
        env::var("SUPABASE_KEY").unwrap());
    tauri::Builder::default()
        .manage(ClientTable{ client: client})
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, add_card, get_card_decks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
