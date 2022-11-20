// use std::collections::HashMap;
use reqwest::Client;

#[tokio::main]
// async fn reqwest(url: &str) -> Result<(), Box<dyn std::error::Error>> {
pub async fn reqwest(method: &String, params: &Option<Vec<String>>)
-> Result<(), Box<dyn std::error::Error>> {
    let eth_endpoint = "http://localhost:8545"; // move this somewhere? env variables maybe

    let client = Client::new();

    let parsed_params = parse_params(&params);

    let body = format!(r#"{{
        "jsonrpc":"2.0",
        "method":"{method}",
        "params":[{parsed_params}],
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

    println!("text: {:#?}", text);

    crate::json::parse_data(&text)?;

    Ok(())
}

fn parse_params(params: &Option<Vec<String>>) -> String {
    let mut parsed_params = String::new();

    match params {
        Some(params) => {
            // for loop through params, if last param, don't add comma
            for (i, param) in params.iter().enumerate() {
                // push param into parsed_params with "" around it
                if check_implied_type(param) {
                    parsed_params.push_str(param);
                } else {
                    parsed_params.push_str(&format!("\"{}\"", param));
                }

                if i < params.len() - 1 {
                    parsed_params.push_str(",");
                }
            }
        },
        None => (),
    }

    return parsed_params;
}


fn check_implied_type(param: &String) -> bool {
    let not_strings = vec![
        "true",
        "false",
        // "null",
        // "0x",
    ];

    // return true if param is non_strings
    for not_string in not_strings {
        println!("param: {:#?}", param);
        println!("not_string: {:#?}", not_string);

        if param == not_string {
            return true;
        }

        println!("no match")
    }

    return false
}