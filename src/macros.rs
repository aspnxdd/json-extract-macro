


/// This macro reduces boilerplate when using serde_json::Value variants when trying to get into a nested property.
/// 
///
/// ```
/// use std::fs;
/// // Include the crates  serde_json and serde
/// use serde_json;
/// use serde;
/// 
/// 
/// let json_parsed = serde_json::json!({
///   "brand": {
///     "tesla": {
///         "model": {
///             "designers": ["Mr Bean","Elon Mosk"]
///            }
///         }
///     }
/// });
/// 
/// let mut a: Vec<String> = vec![];
/// let mut b: Vec<String>= vec![];
/// 
/// // Standard way
/// if let serde_json::Value::Object(brand) = &json_parsed {
///     let brand = brand.get("brand").unwrap();
///     if let serde_json::Value::Object(tesla) = &brand {
///         let tesla = tesla.get("tesla").unwrap();
///         if let serde_json::Value::Object(model) = &tesla {
///             let model = model.get("model").unwrap();
///             if let serde_json::Value::Object(designers) = &model {
///                 let res = designers.get("designers");
///                 a = serde_json::from_value::<Vec<String>>(res.unwrap().to_owned()).unwrap();
///             }
///         }
///     }
/// }
///
/// // With the macro
/// b = json_extract::json_extract!("brand.tesla.model.designers", &json_parsed, Vec<String>).unwrap();
///
///
/// assert_eq!(a,b);
/// ```
/// ## Macro args
///
/// The macro accepts 3 arguments:
///
/// 1. A &str containg the path, separated by "."
/// 2. The serde_json::Value variable to read.
/// 3. The type of the property we want to get.
///
/// ## Types supported
/// `json_serde::Value` has the following variants:
///
/// - Array
/// - Bool
/// - Null
/// - Number
/// - Object
/// - String
///
/// The third parameter to pass in the macro is a Rust type, so, things we can pass if we want to get data from some variants:
///
/// | Value variant | Rust types |
/// | ------ | ------ |
/// | Array | ``` Vec<String>, Vec<bool>, Vec<f64>, Vec<Value> ``` ... |
/// | Bool | ``` bool ``` |
/// | Number | ``` u32, i32, i64, f32, usize ``` ... |
/// | Object | ``` Value ``` |
/// | String | ``` String ``` |
/// | Null | not supported |

#[macro_export]
macro_rules! json_extract {
    ($keys:expr,$json:expr,$t:ty) => {{
        fn get_value<'a>(prev_key: Option<&'a serde_json::Value>, key: &'a str) -> Option<&'a serde_json::Value> {
            if let Some(serde_json::Value::Object(actual_obj)) = prev_key {
                let val: Option<&serde_json::Value> = actual_obj.get(key);
                if val.is_some() {
                    return val;
                }
                None
            } else {
                None
            }
        }

        fn get_final_value<T>(prev_key: Option<&serde_json::Value>, key: &str) -> Option<T>
        where
            T: serde::de::DeserializeOwned + std::fmt::Debug,
        {
            if let Some(serde_json::Value::Object(val_return)) = prev_key {
                let val_return: serde_json::Value = val_return.get(key).unwrap().clone();
                let val: Result<T, serde_json::Error> = serde_json::from_value(val_return);
                if val.is_ok() {
                    return Some(val.unwrap());
                }
                None
            } else {
                None
            }
        }

        fn json_loop<'a, T>(chain: Vec<&'a str>, json: &'a serde_json::Value) -> Option<T>
        where
            T: serde::de::DeserializeOwned + std::fmt::Debug,
        {
            let mut prev_key: Option<&'a serde_json::Value> = None;
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
