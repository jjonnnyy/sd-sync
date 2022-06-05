#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;

use crate::utils::{is_supported_file, History};
use chrono::{DateTime, Utc};
use std::path::Path;
use std::{fs, io};

const EVENT_COPY: &str = "copy";
const EVENT_SKIP: &str = "skip";

#[derive(Clone, serde::Serialize)]
struct CountPayload {
    count: u32,
}

fn recursive_copy(
    window: &tauri::Window,
    source: &Path,
    destination: &Path,
    history: &mut History,
    copied: &mut u32,
    skipped: &mut u32,
) -> io::Result<()> {
    match fs::read_dir(source) {
        Ok(dir_contents) => {
            for entry in dir_contents {
                let entry = entry?;
                let file_name = entry
                    .file_name()
                    .into_string()
                    .expect("Unable to convert filename to string");
                let metadata = entry.metadata()?;
                let created = metadata.created()?;

                // Recurse directories
                if metadata.is_dir() {
                    recursive_copy(
                        window,
                        entry.path().as_path(),
                        destination,
                        history,
                        copied,
                        skipped,
                    )?;
                    continue;
                }

                // Don't do anything with non-image/video files
                if !is_supported_file(&file_name) {
                    continue;
                }

                // Check if file has previously been copied
                if history.seen_before(&file_name, &created) {
                    *skipped += 1;
                    window
                        .emit(EVENT_SKIP, CountPayload { count: *skipped })
                        .expect("Failed to emit skip event.");
                    continue;
                }

                // Copy to destination
                let created_date: DateTime<Utc> = created.into();
                let dest_dir = destination.join(created_date.format("%Y_%m_%d").to_string());

                if !dest_dir.exists() {
                    fs::create_dir(&dest_dir)?;
                }
                fs::copy(entry.path(), dest_dir.join(&file_name))?;

                history.add_file(&file_name, &created);
                *copied += 1;
                window
                    .emit(EVENT_COPY, CountPayload { count: *copied })
                    .expect("Failed to emit copy event.");
            }
            Ok(())
        },
        Err(_) => Ok(()) // Ignore errors whilst opening directories
    }
}

#[tauri::command(async)]
fn start_copy(window: tauri::Window, source: String, destination: String) -> Result<(), String> {
    let src = Path::new(&source);
    let dest = Path::new(&destination);

    // Load seen files data
    let mut history = History::new(&src);

    // Copy files
    match recursive_copy(&window, src, dest, &mut history, &mut 0, &mut 0) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_copy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
