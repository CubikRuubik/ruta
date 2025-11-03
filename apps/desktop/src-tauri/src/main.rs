// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::process::Command;

fn main() {
    #[cfg(target_os = "macos")]
    let path: PathBuf = {
        let mut p = std::env::current_exe().expect("failed to get current exe path");
        p.pop();
        p.pop();
        p.push("Resources/indexer");
        p
    };

    #[cfg(target_os = "windows")]
    let path: PathBuf = {
        let mut p = std::env::current_exe().expect("failed to get current exe path");
        p.pop(); // target/release/
        p.push("indexer");
        p
    };

    std::thread::spawn(move || {
        Command::new(path)
            .env("DATABASE_URL", "postgres://postgres:password@127.0.0.1:5432/indexer_db")
            .env("CONTRACT_ADDRESSES", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48,0xdAC17F958D2ee523a2206206994597C13D831ec7")
            .spawn()
            .expect("failed to start indexer");
    });

    tauri_app_lib::run()
}
