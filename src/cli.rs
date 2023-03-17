use clap::builder::Arg;
use clap::builder::ArgAction;
use clap::builder::Command;
use clap::ArgMatches;
use git2::Config;
use glob::Pattern;

use crate::Options;

const CONFIG_PATHS: [&str; 2] = ["changed.defaultBranch", "init.defaultBranch"];
const DEFAULT_BRANCH: &str = "main";

fn arg_matches() -> ArgMatches {
    Command::new("git-changes")
        .about("Get the recently changed files since a branch started")
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
            Arg::new("glob")
                .default_value("**/*")
                .index(1)
                .help("Use this glob to filter files"),
        ])
        .get_matches()
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

pub fn options() -> Options {
    let m = arg_matches();

    let glob = m
        .get_one::<String>("glob")
        .expect("Default value is missing for glob");

    let pattern = Pattern::new(glob).expect("Valid glob required");

    let base = match m.get_one::<String>("base") {
        Some(value) => value.to_owned(),
        None => default_branch(),
    };

    let null = match m.get_one::<bool>("null") {
        Some(value) => value.to_owned(),
        None => false,
    };

    Options {
        pattern,
        base,
        null,
    }
}
