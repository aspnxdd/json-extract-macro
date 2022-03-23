//! # Json_extract crate
//!
//! This macro reduces boilerplate when using serde_json::Value variants when trying to get into a nested property.

/// This macro reduces boilerplate when using serde_json::Value variants when trying to get into a nested property.
/// 
/// ```
/// // json example
/// {"brand": {
///     "tesla": {
///         "model": {
///             "designers": ["Mr Bean","Elon Mosk"]
///            }
///         }
///     }
/// }
/// 
/// ```
///
/// ```
/// let designer: Option<String> = json_extract!("brand.tesla.model.designers", &res, String);
///
/// println!("Who tf are these designers? {}",designer.unwrap_or_default());
/// ```
/// or
///
/// ```
/// if let Value::Object(brand) = json_file {
///         let brand = brand.get("brand").unwrap();
///         if let Value::Object(tesla) = brand {
///             let tesla = tesla.get("tesla").unwrap();
///             if let Value::Object(model) = tesla {
///                 let model = model.get("model").unwrap();
///                 if let Value::String("designers") = model {
///                     println!("Who tf are these designers? {}",designer.to_owned());
///                 }
///             }
///         }
///     }
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
