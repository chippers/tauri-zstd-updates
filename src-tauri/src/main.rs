// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;

use serde::Serialize;

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let msg = match self {
            Error::Io(e) => e.to_string(),
        };

        serializer.serialize_str(&msg)
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn decompress(bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    let cursor = Cursor::new(bytes);
    zstd::stream::copy_decode(cursor, &mut buf)?;
    Ok(buf)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, decompress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
