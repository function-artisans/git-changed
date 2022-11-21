extern crate glob;

use glob::Pattern;

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pattern: Pattern,
    base: String,
    null: bool,
}

impl fmt::Debug for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("pattern", &self.pattern)
            .field("base", &self.base)
            .field("null", &self.null)
            .finish()
    }
}
