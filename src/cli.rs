use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use crate::eth;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Use Ethereum RPCs", long_about = None)]
pub struct Cli {

    // /// Optional name to operate on
    // pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// RPC method
    method: String,

    /// params
    params: Option<Vec<String>>,

    #[command(subcommand)]
    pub command: Option<Commands>,

}

// #[allow(non_camel_case_types)]
// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// enum Method {
//     eth_blockNumber,
//     eth_getBalance,
//     eth_call,
//     eth_getBlockByNumber,
//     eth_getTransactionReceipt,
// }

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    /// RPC methods 
    Method {

        // /// use an rpc method
        // #[arg(short, long)]
        // method: String,

        /// parameters
        #[arg(short, long)]
        params: Vec<String>,
    }
}



// * This function does all the clap stuff
pub fn cli() {
    let cli = Cli::parse();

    handle(&cli);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Method { params }) => {
            // eth::use_method(method, number, full_not_hash);
        }
        None => {}
    }


    // * just testing here:
    // let params = Some(Vec::<String>::new());
    // eth::use_method(&cli.method, &params);

    eth::use_method(&cli.method, &cli.params);

    // Continued program logic goes here...
}

fn handle(cli: &Cli) {
    // name(&cli);
    config(&cli);
    debug(&cli);
}

// fn name(cli: &Cli) {
//     // You can check the value provided by positional arguments, or option arguments
//     if let Some(name) = cli.name.as_deref() {
//         println!("Value for name: {}", name);
//     }
// }

fn config(cli: &Cli) {
    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }
}

fn debug(cli: &Cli) {
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
}