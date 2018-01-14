extern crate git2;

use git2::Repository;

fn main() {
    let repo = match Repository::open("/home/balaji/Projects/rust/ripgrep") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    println!("opened the repo: {}", repo.head().unwrap().name().unwrap());
}
