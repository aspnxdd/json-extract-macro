# json-extract-macro
## _Access nested JSON values in 1 line of code_

This macro reduces boilerplate when using ```serde_json::Value``` variants when trying to get into a nested property.

```
let json_parsed = serde_json::json!({
   "brand": {
      "tesla": {
         "model": {
             "designers": "Mr Bean"
            }
         }
     }
 });

```
```
let designer: Option<String> = json_extract!("brand.tesla.model.designer", &json_parsed, String);

println!("Who tf is this designer? {}",designer.unwrap_or_default());
```
or...

```
 if let serde_json::Value::Object(brand) = &json_parsed {
     let brand = brand.get("brand").unwrap();
     if let serde_json::Value::Object(tesla) = &brand {
         let tesla = tesla.get("tesla").unwrap();
         if let serde_json::Value::Object(model) = &tesla {
             let model = model.get("model").unwrap();
             if let serde_json::Value::Object(designers) = &model {
                 let res = designer.get("designer");
                 let designer = serde_json::from_value::<String>(res.unwrap().to_owned()).unwrap();
             }
         }
     }
 }

println!("Who tf is this designer? {}",designer.unwrap_or_default());
```
## Macro args

The macro accepts 3 arguments:

1. A &str containg the path, separated by "."
2. The serde_json::Value variable to read.
3. The type of the property we want to get.
 
## Types supported
json_serde::Value has the following variants:

- Array
- Bool
- Null
- Number
- Object
- String

The third parameter to pass in the macro is a Rust type, so, things we can pass if we want to get data from some variants:

| Value variant | Rust types |
| ------ | ------ |
| Array | ``` Vec<String>, Vec<bool>, Vec<f64>, Vec<Value> ``` ... |
| Bool | ``` bool ``` |
| Number | ``` u32, i32, i64, f32, usize ``` ... |
| Object | ``` Value ``` |
| String | ``` String ``` |
| Null | not supported |
