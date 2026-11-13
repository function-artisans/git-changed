use self::cli::Options;
use anyhow::Context;
use anyhow::Result;
use std::env;
use std::io;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
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
    let directory = env::current_dir().context("Cannot read the current directory")?;
    let repo = git::open_repository(&directory)?;
    let subdirectory = git::subdirectory(&repo, &directory)?;

    let changes = git::changes(&repo, options.base.as_deref())?;
    let selected = changes
        .iter()
        .map(|file| relative_path(Path::new(file), &subdirectory))
        .filter(|file| is_selected(file, &options));

    print_files(selected, options.null)?;

    Ok(())
}

fn relative_path(file: &Path, directory: &Path) -> PathBuf {
    let mut file_parts = file.components().peekable();
    let mut directory_parts = directory.components().peekable();

    while file_parts.peek().is_some() && file_parts.peek() == directory_parts.peek() {
        file_parts.next();
        directory_parts.next();
    }

    let parents = directory_parts.map(|_| Component::ParentDir);

    parents.chain(file_parts).collect()
}

fn is_selected(file: &Path, options: &Options) -> bool {
    if !options.glob.matches_path(file) {
        return false;
    }

    if !options.exist {
        return true;
    }

    file.exists()
}

fn print_files(files: impl Iterator<Item = PathBuf>, null: bool) -> io::Result<()> {
    let separator = if null { '\0' } else { '\n' };
    let mut stdout = io::stdout().lock();

    for file in files {
        write!(stdout, "{}{separator}", file.display())?;
    }

    stdout.flush()
}
