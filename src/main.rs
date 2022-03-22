#![recursion_limit = "256"]
use serde_json::Value;
use std::fs;

macro_rules! json_extract {
    ($keys:expr,$json:expr,$t:ty,$nt:ty, $($counter:expr)?) => {{
        let chain: Vec<&str> = $keys.split(".").collect();
        let mut prev_key: Option<&Value> = None;
        let mut counter: usize = 0;
        let mut res: Option<$t> = None;
        let how_many_arrays = chain.iter().filter(|&n| *n == "$").count();

        if how_many_arrays == 1 {
            
        }
        // next feature
        $(
            if $counter> 0 {
                counter = $counter
            };
            
            
        )*

        fn get_value<'a>(mut _prev_key: Option<&'a Value>, _key: &'a str) -> Option<&'a Value> {
            if let Value::Object(actual_obj) = _prev_key.unwrap() {
                let val: &Value = actual_obj.get(_key).unwrap();
                Some(val)
            } else {
                None
            }
        }

        fn get_final_value<'a>(mut _prev_key: Option<&'a Value>, _key: &'a str) -> Option<$t> {
            if let Value::Object(val_return) = _prev_key.unwrap() {
                let val_return: &Value = val_return.get(_key).unwrap();
                serde_json::from_value(val_return.clone()).unwrap_or_else(|_| None)
            } else {
                None
            }
        }

        fn get_final_value_array<'a>(mut _prev_key: Option<&'a Value>, _key: &'a str) -> Option<$nt> {
            if let Value::Object(val_return) = _prev_key.unwrap() {
                let val_return: &Value = val_return.get(_key).unwrap();
                let r = serde_json::from_value(val_return.clone());
                Some(r.unwrap_or_default())

            } else {
                None
            }
        }
        
        while counter < chain.len() {

            let key: &str = chain[counter];
            if counter == chain.len() - 1 {
                res = get_final_value(prev_key, key);
                break;
            }
            if key == "$" {
                if let Value::Array(val_return) = prev_key.unwrap() {
                    let mut vals = Vec::new();
                    for current_val in val_return {
                        let mut sliced_chain = Vec::new();
                        for _ in 0..counter{
                             sliced_chain.push("#");
                        }
                        
                        if counter+1 == chain.len()-1  {
                            let val = get_final_value_array(Some(current_val), &chain[&counter + 1..][0]);
                            
                            vals.push(val.unwrap());
                        }
                        // else{
                            // let sliced_chain = sliced_chain.join(".");
                            // let _sliced_chain = &chain[&counter + 1..].join(".");
                            // let sliced_chain = format!("{}.{}",sliced_chain,_sliced_chain);

                        //     json_extract!(sliced_chain, $json.clone(), $t,$nt,counter)
                        // }
                    }
                    res = Some(vals);
                    break;
                }
            }
            if prev_key.is_none() {
                prev_key = get_value(Some($json), key);
            } else {
                prev_key = get_value(prev_key, key);
            }
            counter += 1;
        }
      
        res
    }};
}

fn main() {
    let path = "./src/test.json";
    let data = fs::read_to_string(path).unwrap();
    let res: Value = serde_json::from_str(&data).unwrap();
    let x = json_extract!("brand.tesla.models.$.designers", &res, Vec<Vec<String>> ,Vec<String>,);

    println!(" $ {:?}", x.unwrap_or_default());
}
