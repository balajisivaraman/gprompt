use std::path::{Path, PathBuf};

use clap;

use app;

use Result;

#[derive(Debug)]
pub struct Args {
    pub path: PathBuf
}

struct ArgMatches<'a>(clap::ArgMatches<'a>);

impl <'a> ArgMatches<'a> {
    fn to_args(&self) -> Result<Args> {
        let path = self.path();
        let args = Args {
            path
        };
        Ok(args)
    }

    fn path(&self) -> PathBuf {
        self.default_path()
    }

    fn default_path(&self) -> PathBuf {
        Path::new("./").to_path_buf()
    }

}

pub fn parse() -> Result<Args> {
    ArgMatches(app::app().get_matches()).to_args()
}