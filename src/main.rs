use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fs;

macro_rules! json_extract {
    ($keys:expr,$json:expr,$t:ty) => {{
        fn get_value<'a>(prev_key: Option<&'a Value>, key: &'a str) -> Option<&'a Value> {
            if let Some(Value::Object(actual_obj)) = prev_key {
                let val: Option<&Value> = actual_obj.get(key);
                if val.is_some() {
                    return val;
                }
                None
            } else {
                None
            }
        }

        fn get_final_value<T>(prev_key: Option<&Value>, key: &str) -> Option<T>
        where
            T: DeserializeOwned + std::fmt::Debug,
        {
            if let Some(Value::Object(val_return)) = prev_key {
                let val_return: Value = val_return.get(key).unwrap().clone();
                let val: Result<T, serde_json::Error> = serde_json::from_value(val_return);
                if val.is_ok() {
                    return Some(val.unwrap());
                }
                None
            } else {
                None
            }
        }

        fn json_loop<'a, T>(chain: Vec<&'a str>, json: &'a Value) -> Option<T>
        where
            T: DeserializeOwned + std::fmt::Debug,
        {
            let mut prev_key: Option<&'a Value> = None;
            let mut counter: usize = 0;
            let mut res: Option<T> = None;
            while counter < chain.len() {
                let key: &str = chain[counter];
                if counter == chain.len() - 1 {
                    res = get_final_value::<T>(prev_key, key);
                    break;
                }
                if prev_key.is_none() {
                    prev_key = get_value(Some(json), key);
                } else {
                    prev_key = get_value(prev_key, key);
                }
                counter += 1;
            }
            res
        }

        let chain: Vec<&str> = $keys.split(".").collect();

        json_loop::<$t>(chain, $json)
    }};
}

fn main() {
    let path = "./src/test.json";
    let data = fs::read_to_string(path).unwrap();
    let json_parsed: Value = serde_json::from_str(&data).unwrap();
    let value = json_extract!("brand.tesla.model.designers", &json_parsed, Vec<String>);

    println!(" $ {:?}", value.unwrap_or_default());
}
