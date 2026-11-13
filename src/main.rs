use self::cli::Options;
use anyhow::Result;
use std::{path::Path, process};

mod cli;
mod git;

fn matches(file: &String, options: &Options) -> bool {
    if !options.pattern.matches(file) {
        return false;
    }

    if !options.exist {
        return true;
    }

    Path::new(file).exists()
}

fn execute(options: Options) -> Result<()> {
    let changes = git::changes(&options.base)?;
    let separator = if options.null { '\0' } else { '\n' };

    for file in &changes {
        if matches(file, &options) {
            print!("{}{}", file, separator);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let options = cli::get_options()?;

    match execute(options) {
        Ok(()) => {
            process::exit(0);
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}
