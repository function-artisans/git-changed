use self::cli::Options;
use anyhow::Context;
use anyhow::Result;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

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
    let changes = git::changes(&repo, options.base.as_deref())?;
    let separator = if options.null { '\0' } else { '\n' };

    let mut stdout = io::stdout().lock();
    for file in changes.iter().filter(|file| matches(file, root, &options)) {
        write!(stdout, "{file}{separator}")?;
    }

    stdout.flush()?;

    Ok(())
}

fn is_broken_pipe(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<io::Error>()
        .is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe)
}

fn main() {
    let options = cli::get_options();

    if let Err(error) = execute(options) {
        if is_broken_pipe(&error) {
            return;
        }

        eprintln!("{error}");
        process::exit(1);
    }
}
