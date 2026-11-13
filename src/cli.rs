use clap::builder::Arg;
use clap::builder::ArgAction;
use clap::builder::Command;
use clap::ArgMatches;
use glob::Pattern;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Options {
    pub pattern: Pattern,
    pub base: Option<String>,
    pub null: bool,
    pub exist: bool,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pattern: {}, base: {:?}, null: {}, exist: {}",
            self.pattern, self.base, self.null, self.exist
        )
    }
}

fn command() -> Command {
    Command::new("git-changed")
        .about("Lists files modified since branching off")
        .args([
            Arg::new("null")
                .short('0')
                .long("print0")
                .help("Use NUL characters as separators (character code 0)")
                .action(ArgAction::SetTrue),
            Arg::new("base")
                .short('b')
                .long("base")
                .help("Use this commit instead of the default branch"),
            Arg::new("exist")
                .short('e')
                .long("exist")
                .help("Filter files that still exist")
                .action(ArgAction::SetTrue),
            Arg::new("glob")
                .default_value("**/*")
                .index(1)
                .help("Use this glob to filter files"),
        ])
}

fn matches_to_options(matches: ArgMatches) -> Options {
    let glob = matches
        .get_one::<String>("glob")
        .expect("Default value is missing for glob");

    let pattern = Pattern::new(glob).expect("Valid glob required");

    let base = matches.get_one::<String>("base").cloned();

    let null = match matches.get_one::<bool>("null") {
        Some(value) => value.to_owned(),
        None => false,
    };

    let exist = match matches.get_one::<bool>("exist") {
        Some(value) => value.to_owned(),
        None => false,
    };

    Options {
        pattern,
        base,
        null,
        exist,
    }
}

pub fn get_options() -> Options {
    matches_to_options(command().get_matches())
}
