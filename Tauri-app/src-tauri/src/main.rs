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
use serde_json::from_str;

#[derive(Debug, Serialize)]
pub enum CommandError {
    Error(String)
}

#[derive(Debug, Deserialize)]
struct Cocktail {
    glass: String,
    ingredients: Vec<String>,
    preparation: String,
}

async fn request_value(url: String) -> Result<HashMap<String,Cocktail>, CommandError>{
    let response = reqwest::get(url).await.map_err(|err| CommandError::Error(format!("{:?}", err)))?.text().await.map_err(|err| CommandError::Error(format!("{:?}", err)))?;
    // let response = std::fs::read_to_string("../src/example.json").map_err(|err| CommandError::Error(format!("{:?}", err)))?;

    let cocktails: HashMap<String, Cocktail> = serde_json::from_str(&response).map_err(|err| CommandError::Error(format!("{:?}", err)))?;

    Ok(cocktails)
}
 
#[tauri::command]
async fn drink_from_ingredients(ingredient_vec: Vec<String>) -> Result< Vec<String>, CommandError >{
    let mut freq : HashMap<String, u64> = HashMap::new();
    println!("{:?}",ingredient_vec);
    for i in 0..ingredient_vec.len(){
        let url = format!("http://172.20.50.2/get_drinks/{}", ingredient_vec[i]); 
        let drinks: HashMap<String, Cocktail>= request_value(url).await?;
        for (name, _cocktail) in drinks.into_iter(){
            println!("{}",name);
            // let ingr = cocktail.ingredients;
            // for k in 0..ingr.len(){
            //     println!("{:?}", ingr[k]);
            // }
            freq.entry(name.clone()).and_modify(|value| *value+=1 ).or_insert(1);
        }
    }

    let mut sorted_pairs: Vec<(String, u64)> = freq.into_iter().collect();
    sorted_pairs.sort_by(|a, b| a.1.cmp(&b.1));

    let sorted_key:Vec<String> = sorted_pairs.into_iter().map(|(k, _)| k).collect();

    //let sorted_keys: Vec<&S;
    // // Print the sorted keys
    // for key in sorted_keys {
    //     response.append(key);
    // }

    return Ok(sorted_key);
}

#[tauri::command]
async fn get_details(drink: String) -> Result<Vec<String>, &'static str> {
    let response: String = reqwest::get(format!("http://172.20.50.2/get/{}", drink))
        .await.unwrap()
        .text()
        .await.unwrap();
    let mut details: std::collections::HashMap<String, Cocktail> = serde_json::from_str(&response)
    .map_err(|err| CommandError::Error(format!("{:?}", err))).unwrap();
    if let Some(cocktail) = details.remove(&drink.to_lowercase()) {
        Ok(cocktail.ingredients)
    }else {
        Err("No ingredients found!")
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            drink_from_ingredients,
            get_details
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
