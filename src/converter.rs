use crate::models::UnitCategory;

pub fn convert(from: &str, to: &str, value: f64) -> Result<f64, String> {
    let category_from = crate::models::detect_category(from);
    let category_to = crate::models::detect_category(to);

    if let (UnitCategory::Unknown, _) | (_, UnitCategory::Unknown) = (&category_from, &category_to) {
        return Err(format!("[ERROR] Satuan '{}' atau '{}' tidak dikenali.", from, to));
    }

    if std::mem::discriminant(&category_from) != std::mem::discriminant(&category_to) {
        return Err(format!(
            "[ERROR] Tidak dapat mengonversi satuan yang berbeda kategori: {:?} → {:?}",
            from, to
        ));
    }

    match category_from {
        UnitCategory::Temperature => convert_temperature(from, to, value),
        UnitCategory::Length => convert_length(from, to, value),
        _ => Err("[ERROR] Konversi tidak valid.".to_string()),
    }
}

fn convert_temperature(from: &str, to: &str, value: f64) -> Result<f64, String> {
    let result = match (from, to) {
        ("celsius", "fahrenheit") => value * 9.0 / 5.0 + 32.0,
        ("fahrenheit", "celsius") => (value - 32.0) * 5.0 / 9.0,
        ("celsius", "kelvin") => value + 273.15,
        ("kelvin", "celsius") => value - 273.15,
        ("fahrenheit", "kelvin") => (value - 32.0) * 5.0 / 9.0 + 273.15,
        ("kelvin", "fahrenheit") => (value - 273.15) * 9.0 / 5.0 + 32.0,
        _ if from == to => value,
        _ => return Err(format!("[ERROR] Tidak dapat mengonversi dari {} ke {}", from, to)),
    };
    Ok(result)
}

fn convert_length(from: &str, to: &str, value: f64) -> Result<f64, String> {
    let cm = match from {
        "cm" => value,
        "inch" => value * 2.54,
        "km" => value * 100000.0,
        "miles" => value * 160934.0,
        _ => return Err(format!("[ERROR] Satuan panjang '{}' tidak dikenali.", from)),
    };

    let result = match to {
        "cm" => cm,
        "inch" => cm / 2.54,
        "km" => cm / 100000.0,
        "miles" => cm / 160934.0,
        _ => return Err(format!("[ERROR] Satuan panjang '{}' tidak dikenali.", to)),
    };

    Ok(result)
}
