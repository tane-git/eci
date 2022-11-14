use crate::reqwest;
use crate::methods::{Methods};
use std::{io::{self, Write}};
use std::str::FromStr;
use strum::IntoEnumIterator;

pub fn use_method(method: &str, params: &Option<String>) {

    let method = get_method(method);
    println!("calling method: {:#?}", method);

    let params: String = handle_params(params);

    let res = reqwest::reqwest(&method.to_string(), params);

    res.unwrap_or_else(|e| {
        println!("Error: {}", e);
    });
}

fn get_method(method: &str) -> Methods {
    let method = match Methods::from_str(method) {
        Ok(method) => method,
        Err(_) => try_again(),
    };

    return method
}

fn try_again() -> Methods {
    print!(r#"Enter a valid method (or "ls"): "#);
    std::io::stdout().flush().unwrap();

    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    if input_text.trim() == String::from("ls") {
        list_available_methods();
        try_again()
    } else {
        get_method(&input_text.trim())
    }
}

fn list_available_methods() {
    // todo: color this output???
    println!("Available methods:");

    for method in Methods::iter() {
        println!("{}", method.to_string());
    }

    println!("");
}

fn handle_params(params: &Option<String>) -> String {
    let params: String = match params {
        Some(params) => params.to_string(),
        None => String::from(""),
    };

    return params
}