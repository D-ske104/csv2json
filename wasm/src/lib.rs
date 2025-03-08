use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use serde_json::{json, Value};
use std::io::Cursor;

#[wasm_bindgen]
pub fn csv_to_json(csv_data: &str) -> String {
    // エラーをキャッチして文字列として返す関数
    let result = process_csv(csv_data);
    match result {
        Ok(json_str) => json_str,
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

// 実際の処理を行う関数（エラーハンドリングを改善）
fn process_csv(csv_data: &str) -> Result<String, String> {
    // CSVリーダーの設定
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)  // 行ごとのフィールド数の違いを許容
        .trim(csv::Trim::All)  // 空白を削除
        .double_quote(true)  // 引用符内の引用符をエスケープ
        .from_reader(Cursor::new(csv_data));
    
    // ヘッダーの取得
    let headers = reader.headers()
        .map_err(|e| format!("Cannot read headers: {}", e))?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    // レコードの処理
    let mut records: Vec<Value> = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| format!("Record parsing error: {}", e))?;
        
        let mut json_record = json!({});
        
        // 各フィールドをJSONオブジェクトに変換
        for (i, field) in record.iter().enumerate() {
            if i < headers.len() {
                // 空のフィールドはnullとして扱う
                let field_value = if field.is_empty() { 
                    Value::Null 
                } else { 
                    Value::String(field.to_string()) 
                };
                json_record[&headers[i]] = field_value;
            }
        }
        
        records.push(json_record);
    }
    
    // JSON文字列に変換
    serde_json::to_string(&records)
        .map_err(|e| format!("JSON conversion error: {}", e))
}
