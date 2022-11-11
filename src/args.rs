use std::env;

use clap;

// * ripgrep (commenting out for now at least):
// pub struct Args(Arc<ArgsImp>);
// struct ArgsImp {

pub struct Args {
    name: String,
    count: u8,
}

impl Args {
    pub fn parse() -> Args {
        let matches = env::args();

        // let matches = clap_matches(env::args_os())?;
        // matches

        let args = Args::parse();
        args
    }
}

// todo: ripgrep?
// impl ArgMatches {

// }

