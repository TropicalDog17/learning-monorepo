use std::{
    collections::HashMap,
    env::temp_dir,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::Result;
type LogPointer = SimpleLogPointer;
/// Definition of a kvstore
pub struct KvStore {
    inner: HashMap<String, LogPointer>,
    log_path: PathBuf,
}

// Definition of a LogPointer
// pub struct LogPointer {
//     offset: u64,
//     length: u64,
// }

type SimpleLogPointer = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub operation: String,
    pub key: String,
    pub value: Option<String>,
}

impl KvStore {
    /// Open a file
    pub fn open(path: &std::path::Path) -> Result<Self> {
        let log_path = path.join("foo.txt");
        let index = HashMap::new();
        let mut instance = Self {
            inner: index,
            log_path: log_path.clone(),
        };
        // Check if file exists before trying to read
        if instance.log_path.exists() {
            instance.update_indexes()?;
        }
        Ok(instance)
    }

    /// Set a key value
    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
        // Write to log file first
        self.write_log_entry(&LogEntry {
            operation: "set".to_string(),
            key: key.clone(),
            value: Some(value.clone()),
        })?;

        if self.inner.len() >= 1000 {
            self.compaction()?;
        }

        Ok(Some("".into()))
    }

    /// Get a value
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // Get the current log pointer
        let Some(&log_pointer) = self.inner.get(&key) else {
            return Ok(None);
        };

        let log_entry = self.read_log_entry(log_pointer)?;
        if log_entry.operation.as_str() == "set" {
            return Ok(log_entry.value);
        } else if log_entry.operation.as_str() == "remove" {
            return Ok(None);
        }

        Err(anyhow!("Key not found"))
    }

    /// Remove a value
    pub fn remove(&mut self, key: String) -> Result<String> {
        // Check if key exists first
        if !self.inner.contains_key(&key) {
            return Err(anyhow::anyhow!("Key not found"));
        }

        // Write to log file first
        self.write_log_entry(&LogEntry {
            operation: "remove".to_string(),
            key: key.clone(),
            value: None,
        })?;

        Ok("".into())
    }

    // Helper method to write log entries consistently
    fn write_log_entry(&mut self, entry: &LogEntry) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let offset = file.seek(SeekFrom::End(0))?;

        // Write JSON line
        let json_string = serde_json::to_string(entry)?;
        writeln!(file, "{}", json_string)?;
        file.flush()?; // Ensure it's written

        // Store actual byte offset in the index
        self.inner.insert(entry.key.clone(), offset);

        Ok(())
    }

    fn compaction(&mut self) -> Result<()> {
        let log_pointers = self.inner.values().collect::<Vec<_>>();
        for (key, &log_pointer) in &self.inner {
            // Test each pointer individually
            match self.read_log_entry(log_pointer) {
                Ok(entry) => println!("  ✓ Valid entry"),
                Err(e) => println!("  ✗ FAILED: {}", e),
            }
        }

        let new_contents = log_pointers
            .iter()
            .map(|&&log_pointer| {
                self.read_log_entry(log_pointer)
                    .expect("Panic when compacting")
            })
            .collect::<Vec<_>>();

        // Create a temporary file
        let temp_path = format!("{}.tmp", self.log_path.display());
        let mut temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_path)?;

        for entry in new_contents {
            let json_string = serde_json::to_string(&entry)?;
            writeln!(temp_file, "{}", json_string)?;
        }

        // Ensure data is written to disk
        temp_file.sync_all()?;
        drop(temp_file);

        // Replace the original file
        std::fs::rename(&temp_path, &self.log_path)?;
        self.update_indexes()?;
        Ok(())
    }
    fn read_log_entry(&self, log_pointer: LogPointer) -> Result<LogEntry> {
        let mut line = String::new();
        let mut reader = BufReader::new(File::open(&self.log_path)?);
        reader.seek(SeekFrom::Start(log_pointer))?;
        reader.read_line(&mut line)?;

        Ok(serde_json::from_str::<LogEntry>(&line)?)
    }

    /// Update the in-mem index based on the log
    fn update_indexes(&mut self) -> Result<()> {
        let file = File::open(&self.log_path)?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut offset: u64 = reader.stream_position()?;
        let mut inner: HashMap<String, LogPointer> = HashMap::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break; // EOF
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                offset = reader.stream_position()?;
                continue;
            }

            match serde_json::from_str::<LogEntry>(trimmed) {
                Ok(command) => {
                    inner.insert(command.key.clone(), offset);
                }
                Err(e) => {
                    eprintln!(
                        "Failed to parse line at offset {}: '{}', error: {}",
                        offset, trimmed, e
                    );
                }
            }

            offset = reader.stream_position()?; // update offset for next line
        }
        self.inner = inner;
        Ok(())
    }
}
