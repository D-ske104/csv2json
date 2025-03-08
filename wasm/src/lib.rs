use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use serde_json::json;

#[wasm_bindgen]
pub fn csv_to_json(csv_input: &str) -> String {
    let mut reader = ReaderBuilder::new()
        .has_headers(true) // CSV の 1 行目をヘッダーとして扱う
        .from_reader(csv_input.as_bytes());

    let headers = reader.headers().unwrap().clone();
    let mut records = vec![];

    for result in reader.records() {
        let record = result.unwrap();
        let mut json_record = json!({});
        
        for (i, field) in record.iter().enumerate() {
            json_record[headers.get(i).unwrap()] = field.into();
        }

        records.push(json_record);
    }

    serde_json::to_string(&records).unwrap()
}
