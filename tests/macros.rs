#[macro_export]
macro_rules! gptest {
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            let (dir, mut cmd) = ::util::setup(stringify!($name));
            cmd.current_dir(&dir.dir);
            $fun(dir, cmd);
        }
    }
}
