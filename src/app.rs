use clap::App;

const ABOUT: &str = "
gprompt displays information about the Git repository in the current
folder. By default, it displays pertinent info such as branch name,
working directory status, and commits ahead/behind of remote tracking
branch.

Project home page: https://github.com/balajisivaraman/gprompt";

const USAGE: &str = "
    gprompt [PATH]
";

const TEMPLATE: &str = "\
{bin} {version}
{author}
{about}

USAGE:{usage}";

pub fn app() -> App<'static, 'static> {
    let app = App::new("gprompt")
        .author(crate_authors!())
        .version(crate_version!())
        .long_version(crate_version!())
        .about(ABOUT)
        .max_term_width(100)
        .usage(USAGE)
        .template(TEMPLATE);
    app
}
