mod cli;
mod git;

use glob::Pattern;

pub struct Options {
    pattern: Pattern,
    base: String,
    null: bool,
}

fn main() {
    let options = cli::options();

    let changes = git::changes(options.base);
    let separator = if options.null { '\0' } else { '\n' };

    for file in &changes {
        if options.pattern.matches(file) {
            print!("{}{}", file, separator);
        }
    }
}
