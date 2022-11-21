use glob::Pattern;

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pattern: Pattern,
    base: String,
    null: bool,
}
