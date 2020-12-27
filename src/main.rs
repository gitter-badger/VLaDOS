extern crate clap;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate thiserror;

mod ast;
mod cmdline;
mod error;
mod lexer;
mod scanner;
mod utils;

use crate::cmdline::get_cmdline;
use crate::scanner::Scanner;
use clap::ArgMatches;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_cmdline();
    if let Some(sub_matches) = matches.subcommand_matches("build") {
        build(sub_matches)?;
    }
    Ok(())
}

fn build(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let top_file = matches.value_of("top_file").unwrap();
    let source = read_to_string(top_file)?;
    for token in Scanner::new(source).scan_tokens()? {
        println!("{:?}", token);
    }
    Ok(())
}
