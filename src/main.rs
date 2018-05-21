#[macro_use]
extern crate clap;

mod app;
mod args;

use std::result;
use std::error::Error;

pub type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    args::parse().map(|_a| ())
}
