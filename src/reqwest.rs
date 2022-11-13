// use std::collections::HashMap;
use reqwest::Client;

#[tokio::main]
// async fn reqwest(url: &str) -> Result<(), Box<dyn std::error::Error>> {
pub async fn reqwest(method: &str) -> Result<(), Box<dyn std::error::Error>> {
    let eth_endpoint = "http://localhost:8545";

    let client = Client::new();

    let body = format!(r#"{{
        "jsonrpc":"2.0",
        "method":"{}",
        "params":[],
        "id":1
    }}"#, method);

    let res = client.post(eth_endpoint)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    // println!("{:#?}", res);

    let text = res.text().await?;

    // println!("{:#?}", text);

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
