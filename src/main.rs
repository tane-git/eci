// use eci::{process_args};

mod cli;

fn main() {
    // * easy
    // process_args();

    // * following ripgrep
    // if let Err(err) = Args::parse().and_then(try_main) {
    //     eprintln!("{}", err);
    // }
    // process::exit(2)

    // * following clap docs
    cli::cli();
}

fn try_main() {
    println!("I dont work");
}