
//! # Json_extract crate
//!
//! This crate contains a macro to reduce boilerplate when using the ```serde_json::Value``` variants when trying to get into a nested property.
//! 
//! Given a JSON:
//! ```
//! let json_parsed = serde_json::json!({
//!   "brand": {
//!     "tesla": {
//!         "model": {
//!             "designers": ["Mr Bean","Elon Mosk"]
//!            }
//!         }
//!     }
//! });
//! ```
//! 
//! 
//! With the standard way it takes up to 13 lines of code to get the desired value
//! ```
//! if let serde_json::Value::Object(brand) = &json_parsed {
//!     let brand = brand.get("brand").unwrap();
//!     if let serde_json::Value::Object(tesla) = &brand {
//!         let tesla = tesla.get("tesla").unwrap();
//!         if let serde_json::Value::Object(model) = &tesla {
//!             let model = model.get("model").unwrap();
//!             if let serde_json::Value::Object(designers) = &model {
//!                 let res = designers.get("designers");
//!                 let designers = serde_json::from_value::<Vec<String>>(res.unwrap().to_owned()).unwrap();
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! With the macro it takes only 1 line. 
//! ```
//! let designers = json_extract::json_extract!("brand.tesla.model.designers", &json_parsed, Vec<String>).unwrap();
//! ```

 #[macro_use]
mod macros;

