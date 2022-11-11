use std::collections::HashMap;

#[tokio::main]
// async fn reqwest(url: &str) -> Result<(), Box<dyn std::error::Error>> {
pub async fn reqwest() -> Result<(), Box<dyn std::error::Error>> {

    // * send this:
    // {
    //     "jsonrpc":"2.0",
    //     "method":"eth_getBlockByNumber",
    //     "params":["0x1",true],
    //     "id":1
    // }

    let client = reqwest::Client::new();

    let body = 

    let res = client.post("http://localhost:8545")
        // .body("{
        // "jsonrpc":"2.0",
        // "method":"eth_getBlockByNumber",
        // "params":["0x1",true],
        // "id":1
        // }")
        .body(body)
        .send()
        .await?;

    println!("{:#?}", res);

    Ok(())
}

    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    // Ok(())

    // let body = reqwest::get("https://www.rust-lang.org")
    //     .await?
    //     .text()
    //     .await?;
    // println!("{:#?}", body);
    // Ok(())

    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    // Ok(())
