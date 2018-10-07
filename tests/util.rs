use std::env;
use std::error;
use std::fs::{self};
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

static TEST_DIR: &'static str = "gprompt-tests";
static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;

/// Setup an empty work directory and return a command pointing to the gprompt
/// executable whose CWD is set to the work directory.
///
/// The name given will be used to create the directory. Generally, it should
/// correspond to the test name.
pub fn setup(test_name: &str) -> (Dir, TestCommand) {
    let dir = Dir::new(test_name);
    let cmd = dir.command();
    (dir, cmd)
}

/// Dir represents a directory in which tests should be run.
///
/// Directories are created from a global atomic counter to avoid duplicates.
#[derive(Clone, Debug)]
pub struct Dir {
    /// The directory in which this test executable is running.
    root: PathBuf,
    /// The directory in which the test should run. If a test needs to create
    /// files, they should go in here. This directory is also used as the CWD
    /// for any processes created by the test.
    pub dir: PathBuf
}

impl Dir {
    /// Create a new test working directory with the given name. The name
    /// does not need to be distinct for each invocation, but should correspond
    /// to a logical grouping of tests.
    pub fn new(name: &str) -> Dir {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let root = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf();
        let dir = env::temp_dir()
            .join(TEST_DIR)
            .join(name)
            .join(&format!("{}", id));
        nice_err(&dir, repeat(|| fs::create_dir_all(&dir)));
        Dir {
            root: root,
            dir: dir
        }
    }

    /// Creates a new command that is set to use the ripgrep executable in
    /// this working directory.
    pub fn command(&self) -> TestCommand {
        let cmd = process::Command::new(&self.bin());
        TestCommand { dir: self.clone(), cmd: cmd }
    }

    /// Returns the path to the gprompt executable.
    pub fn bin(&self) -> PathBuf {
        if cfg!(windows) {
            self.root.join("../gprompt.exe")
        } else {
            self.root.join("../gprompt")
        }
    }
}

/// A simple wrapper around a process::Command with some conveniences.
#[derive(Debug)]
pub struct TestCommand {
    /// The dir used to launched this command.
    dir: Dir,
    /// The actual command we use to control the process.
    cmd: Command,
}

impl TestCommand {
    /// Set the working directory for this command.
    ///
    /// Note that this does not need to be called normally, since the creation
    /// of this TestCommand causes its working directory to be set to the
    /// test's directory automatically.
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut TestCommand {
        self.cmd.current_dir(dir);
        self
    }

    /// Runs and captures the stdout of the given command.
    pub fn stdout(&mut self) -> String {
        let o = self.output();
        let stdout = String::from_utf8_lossy(&o.stdout);
        match stdout.parse() {
            Ok(t) => t,
            Err(err) => {
                panic!(
                    "could not convert from string: {:?}\n\n{}",
                    err,
                    stdout
                );
            }
        }
    }

    /// Gets the output of a command. If the command failed, then this panics.
    pub fn output(&mut self) -> process::Output {
        let output = self.cmd.output().unwrap();
        self.expect_success(output)
    }

    fn expect_success(&self, o: process::Output) -> process::Output {
        if !o.status.success() {
            let suggest =
                if o.stderr.is_empty() {
                    "\n\nDid your search end up with no results?".to_string()
                } else {
                    "".to_string()
                };

            panic!("\n\n==========\n\
                    command failed but expected success!\
                    {}\
                    \n\ncommand: {:?}\
                    \ncwd: {}\
                    \n\nstatus: {}\
                    \n\nstdout: {}\
                    \n\nstderr: {}\
                    \n\n==========\n",
                   suggest, self.cmd, self.dir.dir.display(), o.status,
                   String::from_utf8_lossy(&o.stdout),
                   String::from_utf8_lossy(&o.stderr));
        }
        o
    }
}

fn nice_err<T, E: error::Error>(
    path: &Path,
    res: Result<T, E>,
) -> T {
    match res {
        Ok(t) => t,
        Err(err) => panic!("{}: {:?}", path.display(), err),
    }
}

fn repeat<F: FnMut() -> io::Result<()>>(mut f: F) -> io::Result<()> {
    let mut last_err = None;
    for _ in 0..10 {
        if let Err(err) = f() {
            last_err = Some(err);
            thread::sleep(Duration::from_millis(500));
        } else {
            return Ok(());
        }
    }
    Err(last_err.unwrap())
}
