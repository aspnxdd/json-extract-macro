<<<<<<< Updated upstream
=======
#![recursion_limit = "256"]
use serde::de::DeserializeOwned;
>>>>>>> Stashed changes
use serde_json::Value;
use std::fs;

macro_rules! json_extract {
    ($keys:expr,$json:expr,$t:ty, $($counter:expr)?) => {{
<<<<<<< Updated upstream
        let chain: Vec<&str> = $keys.split(".").collect();
        let mut res: Option<$t> = None;
        let mut prev_key: Option<&Value> = None;
        let mut counter: usize = 0;

        $(
            if $counter> 0 {counter = $counter};
=======

        let chain: Vec<&str> = $keys.split(".").collect();
        let prev_key: Option<&Value> = None;
        let counter: usize = 0;

        let how_many_arrays: usize = chain.iter().filter(|&n| *n == "$").count();

        // if how_many_arrays == 1 {
        //     let x = json_loop::<Vec<$t>>(counter: usize, chain: Vec<&str>, prev_key: Option<&Value>);
        // }else{
        //     let x = json_loop::<$t>(counter: usize, chain: Vec<&str>, prev_key: Option<&Value>);
        // }
        // next feature
        $(
            if $counter> 0 {
                counter = $counter
            };
           
>>>>>>> Stashed changes
        )*

        fn get_value<'a>(mut _prev_key: Option<&'a Value>, _key: &'a str) -> Option<&'a Value> {
            if let Value::Object(actual_obj) = _prev_key.unwrap() {
                let val: &Value = actual_obj.get(_key).unwrap();
                Some(val)
            } else {
                None
            }
        }

        fn get_final_value<T>(mut _prev_key: Option<&Value>, _key: &str) -> Option<T>
            where T: DeserializeOwned + std::fmt::Debug
        {
            if let Value::Object(val_return) = _prev_key.unwrap() {
<<<<<<< Updated upstream
                let val_return = val_return.get(_key).unwrap();
                serde_json::from_value(val_return.clone()).unwrap_or_else(|_| None)
=======
                
                let val_return: Value = val_return.get(_key).unwrap().clone();
                println!("val_return - {:?}",val_return);
                let s = serde_json::from_value(val_return);
                println!("t - {:?}",&s.as_ref().unwrap());

                Some(s.unwrap())
>>>>>>> Stashed changes
            } else {
                None
            }
        }

<<<<<<< Updated upstream
        while counter < chain.len() {
            let key: &str = chain[counter];
            if counter == chain.len() - 1 {
                res = get_final_value(prev_key, key);
                break;
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
=======

        fn json_loop<'a,T>(mut counter: usize, chain: Vec<&'a str>,mut prev_key:Option<&'a Value>, json: &'a Value ) -> Option<T>
            where T: DeserializeOwned + std::fmt::Debug
        {
            let mut r: Option<T> = None;
            while counter < chain.len() {

                let key: &str = chain[counter];
                if counter == chain.len() - 1 {
                    // r = get_final_value::<T>(prev_key, key);
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
                                let val = get_final_value::<T>(Some(current_val), &chain[&counter + 1..][0]);
                                println!(" val - {:?}",val);

                                vals.push(val.unwrap());
                            }
                            // else{
                                // let sliced_chain = sliced_chain.join(".");
                                // let _sliced_chain = &chain[&counter + 1..].join(".");
                                // let sliced_chain = format!("{}.{}",sliced_chain,_sliced_chain);

                            //     json_extract!(sliced_chain, $json.clone(), $t,$nt,counter)
                            // }
                        }
                        r = Some(vals);
                        println!(" vals - {:?}",vals);
                        break;
                    }
                }
                if prev_key.is_none() {
                    prev_key = get_value(Some(json), key);
                } else {
                    prev_key = get_value(prev_key, key);
                }
                counter += 1;
            }
            r
        }
        let json = $json.clone();

        let x = json_loop::<Vec<$t>>(counter, chain, prev_key,&json );
        println!("x - {:?}",x);
        x
    }
};
>>>>>>> Stashed changes
}

fn main() {
    let path = "./src/test.json";
    let data = fs::read_to_string(path).unwrap();
<<<<<<< Updated upstream
    let res: Value = serde_json::from_str(&data).unwrap();
    let x = json_extract!("brand.tesla.model.designers", &res, Vec<String>,);
=======
    let json_parsed: Value = serde_json::from_str(&data).unwrap();
  


    let result = json_extract!("brand.tesla.models.$.designers", &json_parsed, String,);
>>>>>>> Stashed changes

    println!(" $ {:?}", result.unwrap_or_default());
}
