// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
#[allow(unused_imports)]
use reqwest::get;
#[allow(unused_imports)]
use serde_json::{Value, Map};
#[allow(unused_imports)]
use std::{thread, time};
#[allow(unused_imports)]
use tauri::Window;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum CommandError{
    Error(String)
}

async fn request_value(url: String) -> Result<Value, CommandError>{
    let response = reqwest::get(url).await.map_err(|err| CommandError::Error(format!("{:?}", err)))?.text().await.map_err(|err| CommandError::Error(format!("{:?}", err)))?;

    Ok( serde_json::from_str(&response).map_err(|_err| CommandError::Error("Unable to unwrap to Value".to_owned()))? )
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn drink_from_ingredients(ingredients: Vec<String>) -> Result< Vec<String>, CommandError >{
    for i in 0..ingredients.len(){
        let url = format!("REQUESTUL");
        let drinks = request_value(url).await?;
        println!("{}", drinks[i]);
        // for j in 0..drinks{
        //     println!(ingredient[i]);
        // }
    }
    
    Ok(vec!["Drink1".to_owned(), "Drink2".to_owned(), "Drink3".to_owned()])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            drink_from_ingredients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
