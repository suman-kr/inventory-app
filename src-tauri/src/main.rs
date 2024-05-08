// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use db::{get_connection, create_users_table};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let connection = get_connection();
    create_users_table(connection.as_ref().clone());
    // connection.unwrap().execute(
    //     "INSERT INTO users (name, password) VALUES (?1, ?2)",
    //     ["Suman", "hello"],
    // ).unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
