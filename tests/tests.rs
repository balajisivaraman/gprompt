use std::fmt;
use std::process::Command;
use util::{Dir, TestCommand};

// Macros useful for testing.
#[macro_use]
mod macros;
// Utilities for making tests nicer to read and easier to write.
mod util;

gptest!(retrieve_branch, |dir: Dir, mut cmd: TestCommand| {
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("init")
            .output()
            .unwrap();
    Command::new("touch")
            .current_dir(&dir.dir)
            .arg("foo")
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("add")
            .arg(".")
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("ci")
            .arg("--author")
            .arg("Alice <alice@bob.com>")
            .arg("--no-gpg-sign")
            .arg("-m")
            .arg("test commit")
            .output()
            .unwrap();
    assert_eq!(cmd.stdout().trim(), "[master]");
});

gptest!(retrieve_tag, |dir: Dir, mut cmd: TestCommand| {
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("init")
            .arg(".")
            .output()
            .unwrap();
    Command::new("touch")
            .current_dir(&dir.dir)
            .arg("foo")
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("add")
            .arg(".")
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("ci")
            .arg("--author")
            .arg("Alice <alice@bob.com>")
            .arg("--no-gpg-sign")
            .arg("-m")
            .arg("test commit")
            .output()
            .unwrap();
    let tag_name = "test_tag";
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("tag")
            .arg(tag_name)
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("checkout")
            .arg(tag_name)
            .output()
            .unwrap();
    assert_eq!(cmd.stdout().trim(), "[detached@test_tag]");
});

gptest!(retrieve_detached_head, |dir: Dir, mut cmd: TestCommand| {
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("init")
            .arg(".")
            .output()
            .unwrap();
    Command::new("touch")
            .current_dir(&dir.dir)
            .arg("foo")
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("add")
            .arg(".")
            .output()
            .unwrap();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("ci")
            .arg("--author")
            .arg("Alice <alice@bob.com>")
            .arg("--no-gpg-sign")
            .arg("-m")
            .arg("test commit")
            .output()
            .unwrap();
    let output = Command::new("git")
            .current_dir(&dir.dir)
            .arg("rev-parse")
            .arg("--short")
            .arg("HEAD")
            .output()
            .unwrap();
    let sha = String::from_utf8_lossy(&output.stdout).trim().to_owned().to_string();
    Command::new("git")
            .current_dir(&dir.dir)
            .arg("checkout")
            .arg(sha.clone())
            .output()
            .unwrap();
    assert_eq!(cmd.stdout().trim(), fmt::format(format_args!("[detached@{}]", sha.clone())));
});
