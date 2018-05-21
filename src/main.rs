#[macro_use]
extern crate clap;
extern crate git2;

mod app;
mod args;
mod git_utils;

use std::process;
use std::result;

use git2::{Error, Repository};

use args::Args;

pub type Result<T> = result::Result<T, Error>;

fn main() {
    match args::parse().and_then(run) {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<()> {
    let path = args.path();
    Repository::open(path).and_then(|r| {
        match git_utils::get_head_reference(&r) {
            Ok(head_reference) => {
                println!("{}", head_reference);
                Ok(())
            }
            Err(_) => Ok(())
        }
    })
}
