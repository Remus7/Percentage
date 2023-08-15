// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
#[allow(unused_imports)]
use reqwest::get;
#[allow(unused_imports)]
use serde_json::{Map, Value};
#[allow(unused_imports)]
use std::{thread, time};
#[allow(unused_imports)]
use tauri::Window;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub enum CommandError {
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub struct Drink {
    glass: String,
    ingredients: Vec<String>,
    preparation: String,
}

async fn request_value(_url: String) -> Result<HashMap<String, Drink>, CommandError> {
    // let response = reqwest::get(url).await.map_err(|err| CommandError::Error(format!("{:?}", err)))?.text().await.map_err(|err| CommandError::Error(format!("{:?}", err)))?;
    let response = std::fs::read_to_string("../src/example.json")
        .map_err(|err| CommandError::Error(format!("{:?}", err)))?;
    let drinks: HashMap<String, Drink> = serde_json::from_str(&response).unwrap();
    // Ok( serde_json::from_str(&response).map_err(|_err| CommandError::Error("Unable to unwrap to Value".to_owned()))? )
    Ok(drinks)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn drink_from_ingredients(ingredient_vec: Vec<String>) -> Result<Vec<String>, CommandError> {
    println!("{:?}", ingredient_vec);
    for i in 0..ingredient_vec.len() {
        let url = format!("REQUESTUL");
        let drinks = request_value(url).await?;
        for (name, drink) in drinks {
            let ingredients = drink.ingredients;
            for ingredient in ingredients {
                println!("{:?}", ingredient);
            }
        }
    }

    Ok(vec![
        "Drink1".to_owned(),
        "Drink2".to_owned(),
        "Drink3".to_owned(),
    ])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![drink_from_ingredients])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
