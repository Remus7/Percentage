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
use serde_json::from_str;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub enum CommandError {
    Error(String)
}

#[derive(Debug, Deserialize)]
struct Cocktail {
    glass: String,
    ingredients: Vec<String>,
    preparation: String,
    image: String,
}

async fn request_value(url: String) -> Result<HashMap<String,Cocktail>, CommandError>{
    let response = reqwest::get(url).await.map_err(|err| CommandError::Error(format!("{:?}", err)))?.text().await.map_err(|err| CommandError::Error(format!("{:?}", err)))?;
    // let response = std::fs::read_to_string("../src/example.json").map_err(|err| CommandError::Error(format!("{:?}", err)))?;

    let cocktails: HashMap<String, Cocktail> = serde_json::from_str(&response).map_err(|err| CommandError::Error(format!("Bonus: {:?}", err)))?;

    Ok(cocktails)
}
 
#[tauri::command]
async fn drink_from_ingredients(ingredient_vec: Vec<String>) -> Result< Vec<String>, CommandError >{
    let mut freq : HashMap<String, u64> = HashMap::new();
    for i in 0..ingredient_vec.len(){
    println!("muie2");
        let url = format!("http://172.20.50.2/get_drinks/{}", ingredient_vec[i]); 
        let drinks: HashMap<String, Cocktail> = request_value(url).await?;
        println!("muie");
        for (name, _cocktail) in drinks.into_iter(){
            // let ingr = cocktail.ingredients;
            // for k in 0..ingr.len(){
            //     println!("{:?}", ingr[k]);
            // }
            freq.entry(name.clone()).and_modify(|value| *value+=1 ).or_insert(1);
        }
    }

    let mut sorted_pairs: Vec<(String, u64)> = freq.into_iter().collect();
    sorted_pairs.sort_by(|a, b| b.1.cmp(&a.1));

    let sorted_key:Vec<String> = sorted_pairs.into_iter().map(|(k, _)| k).collect();

    return Ok(sorted_key);
}

#[tauri::command]
async fn get_ingredients(drink: String) -> Result<Vec<String>, CommandError> {
    let response: String = reqwest::get(format!("http://172.20.50.2/get_ingredients/{}", drink))
        .await.map_err(|err| CommandError::Error(format!("{:?}", err)))?
        .text()
        .await.map_err(|err| CommandError::Error(format!("{:?}", err)))?;

    let mut details: std::collections::HashMap<String, Cocktail> = serde_json::from_str(&response)
    .map_err(|err| CommandError::Error(format!("{:?}", err))).unwrap();

    if let Some(cocktail) = details.remove(&drink.to_lowercase()) {
        Ok(cocktail.ingredients)
    } else{
        Err(CommandError::Error("No ingredients found!".to_owned()))
    }
}

/*
Request types:
    0 -> image URL
    1 -> glass type
    2 -> preparation
*/
#[tauri::command]
async fn get_details(drink: String, request_type: u32) -> Result<String, CommandError> {
    let url = format!("http://172.20.50.2/get_ingredients/{}", drink);
    let mut details = request_value(url).await?;

    if let Some(cocktail) = details.remove(&drink.to_lowercase()) {
        match request_type{
            0 => Ok(cocktail.image),
            1 => Ok(cocktail.glass),
            2 => Ok(cocktail.preparation),
            _ => Err(CommandError::Error("Unknown request".to_owned())),
        }
    } else{
        Err(CommandError::Error("No image url found!".to_owned()))
    }
}

#[tauri::command]
async fn get_possible_ingredients() -> Vec<String> {
    let url = format!("http://172.20.50.2/get_ingredients");
    let response = reqwest::get(url).await.map_err(|err| CommandError::Error(format!("{:?}", err))).unwrap().text().await.map_err(|err| CommandError::Error(format!("{:?}", err))).unwrap();
    let details: HashMap<u32, String> = serde_json::from_str(&response).map_err(|err| CommandError::Error(format!("Bonus: {:?}", err))).unwrap();

    let mut ingredients = vec![];

    for (_id, name) in details{
        ingredients.push(name);
    }

    return ingredients
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            drink_from_ingredients,
            get_details,
            get_ingredients,
            get_possible_ingredients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
