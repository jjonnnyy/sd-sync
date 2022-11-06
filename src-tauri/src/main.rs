#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;

use crate::utils::{is_supported_file, History};
use chrono::{DateTime, Utc};
use std::path::Path;
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

const EVENT_COPY: &str = "copy";
const EVENT_SKIP: &str = "skip";

#[derive(Clone, serde::Serialize)]
struct CountPayload {
    count: u32,
}

fn copy_file(entry: &DirEntry, dest: &Path) -> io::Result<()> {
    let file_name = entry
        .file_name()
        .to_str()
        .expect("Unable to convert filename to string");
    let created = entry.metadata()?.created()?;

    let created_date: DateTime<Utc> = created.into();
    let dest_dir = dest.join(created_date.format("%Y_%m_%d").to_string());

    if !dest_dir.exists() {
        fs::create_dir(&dest_dir)?;
    }
    fs::copy(entry.path(), dest_dir.join(&file_name))?;
    Ok(())
}

#[tauri::command(async)]
fn start_copy(window: tauri::Window, source: String, destination: String) -> Result<(), String> {
    let src = Path::new(&source);
    let dest = Path::new(&destination);

    // Load seen files data
    let mut history = History::new(&src);

    let mut copied = 0;
    let mut skipped = 0;

    let entries = WalkDir::new(&src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_supported_file(e.file_name().to_str().unwrap()));

    for entry in entries {
        let name = entry.file_name().to_str().unwrap();
        let created = entry.metadata().unwrap().created().unwrap();

        if history.seen_before(name, &created) {
            skipped += 1;
            window
                .emit(EVENT_SKIP, CountPayload { count: skipped })
                .expect("Failed to emit skip event.");
            continue;
        }

        match copy_file(&entry, &dest) {
            Ok(_) => {
                history.add_file(&name, &created);
                copied += 1;
                window
                    .emit(EVENT_COPY, CountPayload { count: copied })
                    .expect("Failed to emit copy event.");
            }
            Err(e) => println!("{}", e.to_string()),
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_copy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
