// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{Duration, Instant};
use jwalk::WalkDir;
use serde::Serialize;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct FileEntry {
    is_dir: bool,
    path: String,
    dir: String,
    children: Vec<FileEntry>,
    parent_path: String,
}

fn measure_execution_time<F, R>(function: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
{
    let start_time = Instant::now();
    let result = function();
    let end_time = Instant::now();
    let duration = end_time - start_time;
    (result, duration)
}

#[tauri::command(async)]
fn recursive_walk(path: String) -> Vec<FileEntry> {
    let (result, duration) = measure_execution_time(|| _recursive_walk(path));
    println!("Execution time: {:?}", duration);
    result
}

fn _recursive_walk(path: String) -> Vec<FileEntry> {

    let mut entries = Vec::new();

    for entry in WalkDir::new(path) {
        match entry {
            Err(err) => {
                eprintln!("Error: {}", err);
            }
            Ok(entry) => {
                let is_dir = entry.file_type().is_dir();
                entries.push(FileEntry {
                    path: entry.path().display().to_string(),
                    is_dir: is_dir,
                    dir: entry.path().parent().unwrap().display().to_string(),
                    children: Vec::new(),
                    parent_path: entry.parent_path().display().to_string(),
                });
            }
        }
    }
    entries
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_extra::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![recursive_walk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
