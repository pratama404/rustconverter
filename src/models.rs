use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversionRecord {
    pub from: String,
    pub to: String,
    pub value: f64,
    pub result: f64,
}

#[derive(Debug)]
pub enum UnitCategory {
    Temperature,
    Length,
    Unknown,
}

pub fn detect_category(unit: &str) -> UnitCategory {
    match unit.to_lowercase().as_str() {
        "celsius" | "fahrenheit" | "kelvin" => UnitCategory::Temperature,
        "cm" | "inch" | "km" | "miles" => UnitCategory::Length,
        _ => UnitCategory::Unknown,
    }
}
