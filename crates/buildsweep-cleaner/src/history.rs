use buildsweep_core::HistoryEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryRecord {
    pub timestamp: DateTime<Utc>,
    pub moved_bytes: u64,
    pub item_count: u32,
}

pub fn load_history(history_path: &Path) -> Vec<HistoryEntry> {
    let file = match fs::File::open(history_path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if let Ok(record) = serde_json::from_str::<HistoryRecord>(&line) {
            entries.push(HistoryEntry {
                timestamp: record.timestamp,
                moved_bytes: record.moved_bytes,
                item_count: record.item_count,
            });
        }
    }

    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    entries
}

pub fn append_history(
    history_path: &Path,
    moved_bytes: u64,
    item_count: u32,
) -> std::io::Result<()> {
    if let Some(parent) = history_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let record = HistoryRecord {
        timestamp: Utc::now(),
        moved_bytes,
        item_count,
    };
    let line = serde_json::to_string(&record)?;
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(history_path)?
        .write_all(format!("{}\n", line).as_bytes())?;
    Ok(())
}
