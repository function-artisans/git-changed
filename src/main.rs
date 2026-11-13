use self::cli::Options;
use anyhow::Context;
use anyhow::Result;
use std::{path::Path, process};

mod cli;
mod git;

fn matches(file: &str, root: &Path, options: &Options) -> bool {
    if !options.pattern.matches(file) {
        return false;
    }

    if !options.exist {
        return true;
    }

    root.join(file).exists()
}

fn execute(options: Options) -> Result<()> {
    let repo = git::open_repository()?;
    let root = repo
        .workdir()
        .context("The repository has no working directory")?;
    let changes = git::changes(&repo, &options.base)?;
    let separator = if options.null { '\0' } else { '\n' };

    for file in &changes {
        if matches(file, root, &options) {
            print!("{}{}", file, separator);
        }
    }

    Ok(())
}

fn main() {
    let options = cli::get_options();

    if let Err(error) = execute(options) {
        eprintln!("{error}");
        process::exit(1);
    }
}
