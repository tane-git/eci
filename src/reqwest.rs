// use std::collections::HashMap;
use reqwest::Client;

#[tokio::main]
// async fn reqwest(url: &str) -> Result<(), Box<dyn std::error::Error>> {
pub async fn reqwest() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let body = r#"{
        "jsonrpc":"2.0",
        "method":"eth_blockNumber",
        "params":[],
        "id":1
    }"#;

    let res = client.post("http://localhost:8545")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    println!("{:#?}", res);

    let text = res.text().await?;

    println!("{:#?}", text);

    crate::json::parse_data(&text)?;

    Ok(())
}

    // * other examples that have worked:

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
