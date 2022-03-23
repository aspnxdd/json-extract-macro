use serde_json::Value;
use std::fs;
use serde::de::DeserializeOwned;


#[macro_use] mod macros;





fn main() {
    let path = "./src/test.json";
    let data = fs::read_to_string(path).unwrap();
    let json_parsed: Value = serde_json::from_str(&data).unwrap();
    let value = json_extract!("brand.tesla.model.designers", &json_parsed, Vec<String>);

    println!(" $ {:?}", value.unwrap_or_default());
}
