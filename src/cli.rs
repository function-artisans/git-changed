use clap::builder::Arg;
use clap::builder::ArgAction;
use clap::builder::Command;
use clap::ArgMatches;
use glob::Pattern;

#[derive(Debug)]
pub struct Options {
    pub glob: Pattern,
    pub base: Option<String>,
    pub null: bool,
    pub exist: bool,
}

pub fn get_options() -> Options {
    matches_to_options(command().get_matches())
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
                .help("Show only the files that still exist")
                .action(ArgAction::SetTrue),
            Arg::new("glob")
                .default_value("**/*")
                .value_parser(Pattern::new)
                .index(1)
                .help("Use this glob to filter files"),
        ])
}

fn matches_to_options(matches: ArgMatches) -> Options {
    Options {
        glob: matches
            .get_one::<Pattern>("glob")
            .cloned()
            .expect("The glob has a default value"),
        base: matches.get_one::<String>("base").cloned(),
        null: matches.get_flag("null"),
        exist: matches.get_flag("exist"),
    }
}
