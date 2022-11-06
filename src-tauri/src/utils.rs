use bincode::{deserialize_from, serialize_into};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::SystemTime;

const SUPPORTED_FILES: &[&str] = &[".ARW", ".CR3", ".MP4"];

pub fn is_supported_file(name: &str) -> bool {
    for suffix in SUPPORTED_FILES {
        if name.ends_with(suffix) {
            return true;
        }
    }
    false
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct File(String, u64);

pub struct History<'a> {
    dir: &'a Path,
    seen_files: Vec<File>,
}

const HISTORY_FILENAME: &str = ".sync_history";

impl History<'_> {
    pub fn new(dir_path: &Path) -> History {
        // Attempt to load history from file system
        let seen_files = match fs::File::open(dir_path.join(HISTORY_FILENAME)) {
            Ok(f) => {
                let reader = BufReader::new(f);
                deserialize_from(reader).unwrap_or_default()
            }
            Err(_) => Vec::new(),
        };
        History {
            dir: dir_path,
            seen_files,
        }
    }

    pub fn seen_before(&self, name: &str, created: &SystemTime) -> bool {
        // Convert creation time to unix timestamp
        let created_time = created
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to convert timestamp")
            .as_secs();

        self.seen_files
            .iter()
            .any(|f| *f == File(name.into(), created_time))
    }

    pub fn add_file(&mut self, name: &str, created: &SystemTime) {
        // Convert creation time to unix timestamp
        let created_time = created
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Unable to convert timestamp")
            .as_secs();

        self.seen_files.push(File(name.into(), created_time));
    }
}

impl Drop for History<'_> {
    fn drop(&mut self) {
        // Write seen files back to file
        if let Ok(f) = fs::File::create(self.dir.join(HISTORY_FILENAME)) {
            let mut f = BufWriter::new(f);
            let _ = serialize_into(&mut f, &self.seen_files);
        }
    }
}
