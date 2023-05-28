// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;
use reqwest::Proxy;
use std::{io::Read, time::Duration};
use tauri::{
    api::process::{Command, CommandEvent},
    Manager, Window,
};

fn replace_resource_links(page_source: &str, base_url: &str) -> String {
    let correct_base_url = if base_url.ends_with('/') {
        base_url.to_string()
    } else {
        format!("{}/", base_url)
    };
    let replacements = [
        (
            r#"href="/(.*?)""#,
            format!("href=\"{}$1\"", correct_base_url),
        ),
        (r#"src="/(.*?)""#, format!("src=\"{}$1\"", correct_base_url)),
    ];
    let mut new_page_source = page_source.to_string();
    for &(pattern, ref replacement) in &replacements {
        let re = Regex::new(pattern).unwrap();
        new_page_source = re
            .replace_all(&new_page_source, replacement.as_str())
            .to_string();
    }
    new_page_source
}

fn fetch_url_content(url: &str) -> String {
    let raw_proxy = format!("127.0.0.1:18080");
    let proxy = Proxy::all(&raw_proxy).unwrap();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .proxy(proxy)
        .build()
        .unwrap();
    let mut response = client.get(url).send().unwrap();
    let mut buf = String::new();
    response
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    buf
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}!", name);
    let content = fetch_url_content(&name);
    let new_content = replace_resource_links(&content, &name);
    new_content
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window: Window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                let (mut rx, mut child) = Command::new_sidecar("opera-proxy")
                    .expect("failed to setup `opera-proxy` sidecar")
                    .spawn()
                    .expect("Failed to spawn packaged node");
                while let Some(event) = rx.recv().await {
                    let line = match event {
                        CommandEvent::Stdout(line) => line,
                        CommandEvent::Stderr(line) => line,
                        _ => continue,
                    };
                    println!("line: {}", line);
                    window
                        .emit("message", Some(format!("'{}'", line)))
                        .expect("failed to emit event");

                    child.write("message from Rust\n".as_bytes()).unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
