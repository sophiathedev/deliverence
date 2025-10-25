use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::AppHandle;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CsvSenderRecord {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ReceiverValue {
    Number(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CsvReceiverRecord {
    pub email: String,

    #[serde(flatten)]
    pub replacement_data: HashMap<String, ReceiverValue>,
}

pub struct CsvLoader {
    file_path: String,
}

impl CsvLoader {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn load_into<T>(self: Self) -> Result<Vec<T>, String>
    where
        T: for<'de> Deserialize<'de>,
    {
        let file_location = Path::new(&self.file_path);
        if !file_location.exists() {
            return Err("File is removed or temporary deleted".to_string());
        }

        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::Headers)
            .flexible(true)
            .from_path(file_location)
            .map_err(|e| e.to_string())?;

        let original_headers = reader.headers().map_err(|e| e.to_string())?.clone();
        let lowercase_headers: csv::StringRecord = original_headers
            .iter()
            .map(|field| field.to_lowercase())
            .collect();

        reader.set_headers(lowercase_headers);

        let records = reader
            .deserialize()
            .collect::<Result<Vec<T>, csv::Error>>()
            .map_err(|e| e.to_string())?;

        Ok(records)
    }
}

#[tauri::command]
pub async fn read_csv(_app: AppHandle, file_path: String) -> Result<Vec<CsvSenderRecord>, String> {
    CsvLoader::new(file_path).load_into::<CsvSenderRecord>()
}

#[tauri::command]
pub async fn read_receiver_csv_list(_app: AppHandle, file_path: String) -> Result<Vec<CsvReceiverRecord>, String> {
    CsvLoader::new(file_path).load_into::<CsvReceiverRecord>()
}
