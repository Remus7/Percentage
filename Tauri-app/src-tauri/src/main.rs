// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been yeeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn  drink_from_ingredients(Ingredients:Vec<String>)->Result<Vec<String>,CommandError>{
    let url = format!("REQUESTUL").expect(CommandError);
    for i in 0..Ingredients.len(){
    let ingredient = reqwest::get(url);
    .await.unwrap()
    .text()
    .await.unwrap()
}
    let drink_list: Value = serde_json::from_str(&body_of_request).expect("Eroare la deserializare");
    
}
