use serde_json::Value;
use std::fs;

macro_rules! json_extract {
    
       ($($keys:expr,$json:expr,$t:ty),*)=>{
            {
                let chain: Vec<&str>;
                let json:Value;
                $(
                    chain = $keys.split(".").collect();
                    json =  $json.clone();
                    let mut res: Option<$t> = None;
                )*
                let mut prev_key: Option<&Value>= None;
                let mut counter:usize = 0;

                while counter < chain.len() {
                    let key:&str = chain[counter];
                    if counter == chain.len()-1 {
                        if let Value::Object(val_return) = &prev_key.unwrap() {
                            let val_return = val_return.get(key).unwrap();
                            res = serde_json::from_value(val_return.clone()).unwrap_or_else(|_| None);
                            break;
                        }
                    }
                    if prev_key.is_none() {
                        if let Value::Object(actual_obj) = &json {
                            let val:&Value = actual_obj.get(key).unwrap();
                            prev_key = Some(val);
                        }
                    }
                    else{
                        if let Value::Object(actual_obj) = &prev_key.unwrap() {
                            let val:&Value = actual_obj.get(key).unwrap();
                            prev_key = Some(val);
                        }
                    }
                    counter+=1;
                }
                res
        }
       };
   }

fn main() {
    let path = "./src/test.json";
    let data = fs::read_to_string(path).unwrap();
    let res: Value = serde_json::from_str(&data).unwrap();

    let x = json_extract!("brand.tesla.model.designers", &res, Vec<String>);
    println!("######### {:?}", x.unwrap_or_default());
}
