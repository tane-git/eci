// use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::i64;

// #[derive(Serialize, Deserialize)]
// struct rpc_response {
//     result: String,
// }

pub fn parse_data(data: &str) -> Result<()> {
    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    // let p: Person = serde_json::from_str(data)?;

    let v: Value = serde_json::from_str(data)?;

    let string = v["result"].as_str();

    match string {
        Some(string) => {
            let without_prefix = string.trim_start_matches("0x");
            let z = i64::from_str_radix(without_prefix, 16);
            println!("{:?}", z);
        },
        None => println!("No string")
    }


    Ok(())
}