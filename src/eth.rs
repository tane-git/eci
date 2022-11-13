use crate::reqwest;

// pub enum Methods {
//     BlockNumber,
//     getBlockByNumber
// }

// an enum for the different methods as strings
// pub enum Methods {
//     BlockNumber,
//     GetBlockByNumber,
// }

// type methods = "BlockNumber" | "GetBlockByNumber";

pub fn use_method(method: &String) {
    let methods = vec!["BlockNumber", "GetBlockByNumber"];

    // let methods = {

    // }

    if methods.contains(&method.as_str()) {
        println!("Method: {}", method);
        reqwest::reqwest(method).await;
    } else {
        println!("Method not found");
    }
}

#[allow(non_snake_case)]
pub fn blockNumber() {
    println!("fetching blockNumber...");
    match reqwest::reqwest("eth_blockNumber") {
        Ok(_) => println!("ok"),
        Err(err) => println!("{err}")
    }
}

#[allow(non_snake_case)]
pub fn getBlockNumber() {
    println!("fetching blockNumber...");
    match reqwest::reqwest("eth_getBlockNumber") {
        Ok(_) => println!("ok"),
        Err(err) => println!("{err}")
    }
}
