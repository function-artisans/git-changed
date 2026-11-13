use self::cli::Options;
use anyhow::Context;
use anyhow::Result;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

mod cli;
mod git;

fn main() {
    let options = cli::get_options();

    if let Err(error) = execute(options) {
        if is_broken_pipe(&error) {
            return;
        }

        eprintln!("{error:#}");
        process::exit(1);
    }
}

fn is_broken_pipe(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<io::Error>()
        .is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe)
}

fn execute(options: Options) -> Result<()> {
    let repo = git::open_repository()?;
    let root = repo
        .workdir()
        .context("The repository has no working directory")?;

    let changes = git::changes(&repo, options.base.as_deref())?;
    let selected = changes
        .iter()
        .filter(|file| is_selected(file, root, &options));

    print_files(selected, options.null)?;

    Ok(())
}

fn is_selected(file: &str, root: &Path, options: &Options) -> bool {
    if !options.pattern.matches(file) {
        return false;
    }

    if !options.exist {
        return true;
    }

    root.join(file).exists()
}

fn print_files<'a>(files: impl Iterator<Item = &'a String>, null: bool) -> io::Result<()> {
    let separator = if null { '\0' } else { '\n' };
    let mut stdout = io::stdout().lock();

    for file in files {
        write!(stdout, "{file}{separator}")?;
    }

    stdout.flush()
}
