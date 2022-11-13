use crate::reqwest;
use strum_macros::{EnumString, Display};
use std::str::FromStr;

#[derive(EnumString, Debug, Display)]
enum Methods {
    BlockNumber,
    GetBlockByNumber,
}

pub fn use_method(method: &str) {
    let method = Methods::from_str(method).unwrap();
    println!("{:#?}", method);

    let res = match method {
        Methods::BlockNumber => {
            reqwest::reqwest("eth_blockNumber")
        }
        Methods::GetBlockByNumber => {
            reqwest::reqwest("eth_getBlockByNumber")
        }
    };

//     res.unwrap_or_else(|e| {
//         println!("Error: {}", e);
//     });
}

fn send_method(method: Methods) {
    reqwest::reqwest(method.into()).unwrap_or_else(|e| {
        println!("Error: {}", e);
    });
}
