// use std::collections::HashMap;
use reqwest::Client;

#[tokio::main]
// async fn reqwest(url: &str) -> Result<(), Box<dyn std::error::Error>> {
pub async fn reqwest(method: &String, params: &Option<String>, more_params: &Option<bool>)
-> Result<(), Box<dyn std::error::Error>> {
    let eth_endpoint = "http://localhost:8545";

    let client = Client::new();

    enum Thing<'a, 'b> {
        ParamsRap(&'a String),
        MoreParamsRap(&'b bool),
    }

    let mut things: Vec<Thing> = vec![];

    match params {
        Some(params) => {
            things.push(Thing::ParamsRap(params));
        }
        None => {}
    }

    match more_params {
        Some(more_params) => {
            things.push(Thing::MoreParamsRap(more_params));
        }
        None => {}
    }

    // convert Things from a vec to a string
    let mut things_string = String::new();

    for thing in things {
        match thing {
            Thing::ParamsRap(params) => {
                let add = format!(r#""{params}", "#);
                things_string.push_str(add.as_str());
            }
            Thing::MoreParamsRap(more_params) => {
                let add = format!(r#"{more_params}, "#);
                things_string.push_str(add.as_str());
            }
        }
    }

    // remove the last 2 characters of a string (to remove illegal ", " at the end)
    if things_string.len() > 2 {
        things_string.truncate(things_string.len() - 2);
    }
    // let things_string = &things_string[..things_string.len() - 2];

    println!("things_string: {:#?}", things_string);

    let body = format!(r#"{{
        "jsonrpc":"2.0",
        "method":"{method}",
        "params":[{things_string}],
        "id":1
    }}"#);

    println!("body: {:#?}", body);

    let res = client.post(eth_endpoint)
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
