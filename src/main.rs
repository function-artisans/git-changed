use self::cli::Options;
use anyhow::Result;
use std::process;

mod cli;
mod git;

fn execute(options: Options) -> Result<()> {
    let changes = git::changes(&options.base)?;
    let separator = if options.null { '\0' } else { '\n' };

    for file in &changes {
        if options.pattern.matches(file) {
            print!("{}{}", file, separator);
        }
    }

    Ok(())
}

#[cfg(test)]
pub fn from(cmd: &String) -> Result<()> {
    let options = cli::get_options_from(cmd)?;

    execute(options)
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
