use anyhow::Result;
use clap::builder::Arg;
use clap::builder::ArgAction;
use clap::builder::Command;
use clap::ArgMatches;
use git2::Config;
use glob::Pattern;
use std::fmt;

const CONFIG_PATHS: [&str; 2] = ["changed.defaultBranch", "init.defaultBranch"];
const DEFAULT_BRANCH: &str = "main";

#[derive(Debug, Clone)]
pub struct Options {
    pub pattern: Pattern,
    pub base: String,
    pub null: bool,
    pub exist: bool,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pattern: {}, base: {}, null: {}, exist: {}",
            self.pattern, self.base, self.null, self.exist
        )
    }
}

fn default_branch() -> String {
    let config = Config::open_default().expect("Failed to open global config");

    let branch = CONFIG_PATHS
        .into_iter()
        .find_map(|path| match config.get_entry(path) {
            Ok(entry) => entry.value().map(str::to_string),
            Err(_e) => None,
        });

    branch.unwrap_or(DEFAULT_BRANCH.to_owned())
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

    let base = match matches.get_one::<String>("base") {
        Some(value) => value.to_owned(),
        None => default_branch(),
    };

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

#[cfg(test)]
pub fn get_options_from(cmd: &String) -> Result<Options> {
    let args = cmd.split(' ');

    let matches = command().try_get_matches_from(args)?;

    Ok(matches_to_options(matches))
}

pub fn get_options() -> Result<Options> {
    let matches = command().try_get_matches()?;

    Ok(matches_to_options(matches))
}
