use std::fs::File;
use std::path::Path;
use std::{env, io};
use std::process::Command;
use std::io::Write;

pub fn process_args() {
    let mut args = env::args();

    let name = args.nth(1);

    match name {
        Some(name) => println!("Hello, {name}!"),
        None => println!("Enter your name next time!")
    }
}

// // Asks user for a todo
// pub fn ask_for_todo() {
//     let mut input_text = String::new();
//     print!("Enter todo: ");
//     while input_text.len() < 3 {
//         std::io::stdout().flush().unwrap();
//         io::stdin()
//             .read_line(&mut input_text)
//             .expect("failed to read from stdin");
//         input_text = input_text.trim().to_string();
//         if input_text.len() < 3 {
//             print!("Use more than 2 characters: ");
//             // reset
//             input_text = String::new();
//         }
//     }
//     println!("todo: {input_text}");
//     save_todo(input_text);
// }

// pub fn recursive() {
//     let file_name = "./src/main.rs";
//     let mut file = File::options()
//         .append(true)
//         .open(file_name)
//         .unwrap();
//     file.write(b"hi").unwrap();
// }

// fn save_todo(title: String) {
//     // create an empty text file called the todo
//     let file_name = title + ".text";
//     // let file_folder = String::from("todos/");
//     let file =
//         File::create(
//             Path::new(
//                 format!("{}{}", "files/", file_name).as_str()
//             )
//         );
//     println!("Created {:?}", file);
//     // let path = std::path::Path::new("/home/roger/foo/bar/baz.txt");
//     // let prefix = path.parent().unwrap();
//     // std::fs::create_dir_all(prefix).unwrap();
// }