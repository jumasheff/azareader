// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{time::Duration, io::Read};

use reqwest::{Proxy};

fn fetch_url_content(url: &str) -> String {
    let raw_proxy = format!("127.0.0.1:18080");
    let proxy = Proxy::all(&raw_proxy).unwrap();
    let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .proxy(proxy)
            .build()
            .unwrap();
    let  mut response = client.get(url).send().unwrap();
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    buf
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let content = fetch_url_content(&name);
    content
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
