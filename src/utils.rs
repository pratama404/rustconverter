use crate::models::ConversionRecord;
use std::fs::File;
use std::io::{BufReader, Write};

const FILE_PATH: &str = "conversion.json";

pub fn save_history(record: &ConversionRecord) {
    let mut records = load_history();
    records.push(record.clone());

    let json = serde_json::to_string_pretty(&records).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn load_history() -> Vec<ConversionRecord> {
    if let Ok(file) = File::open(FILE_PATH) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
