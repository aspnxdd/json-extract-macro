
use serde_json::Value;
use std::fs;

macro_rules! json_extract {
    ($keys:expr,$json:expr,$t:ty, $($counter:expr)?) => {{
        let chain: Vec<&str> = $keys.split(".").collect();
        let mut res: Option<$t> = None;
        
        let mut prev_key: Option<&Value> = None;
        let mut counter: usize = 0;
        // next feature
        $(
            if $counter> 0 {counter = $counter};
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
                let val_return = val_return.get(_key).unwrap();
                serde_json::from_value(val_return.clone()).unwrap_or_else(|_| None)
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
            // if key == "$" {
            //     if let Value::Array(val_return) = prev_key.unwrap() {
            //         let vals = vec::New();
            //         for _ in val_return {
            //             let mut sliced_chain = Vec::new();
            //             for _ in 0..counter{
            //                  sliced_chain.push("#");
            //             }
            //             let sliced_chain = sliced_chain.join(".");
            //             let _sliced_chain = &chain[&counter + 1..].join(".");
            //             let sliced_chain = format!("{}.{}",sliced_chain,_sliced_chain);
            //             println!("x - {:?}", sliced_chain);
                        
            //             let x = json_extract!(&sliced_chain, $json.clone(), $t, counter.clone() + 1);
            //             vals.push(x);
            //         }
            //         break;
                    
            //     }
            // }
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
    let x = json_extract!("brand.tesla.models.designers", &res, Vec<String>,);
    
    println!(" $ {:?}", x.unwrap_or_default());
}
