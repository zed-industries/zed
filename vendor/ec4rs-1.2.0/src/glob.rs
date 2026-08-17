// TODO: All of this glob stuff should be extracted to its own crate.

mod flatset;
mod matcher;
mod parser;
mod splitter;
mod stack;

pub use matcher::Matcher;

use flatset::FlatSet;
use splitter::Splitter;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Glob(pub(super) Vec<Matcher>);

impl Glob {
    pub fn new(pattern: &str) -> Glob {
        parser::parse(pattern)
    }

    #[must_use]
    pub fn matches(&self, path: &std::path::Path) -> bool {
        matcher::matches(path, self).is_some()
    }

    pub(super) fn append_char(&mut self, c: char) {
        if c == '/' {
            self.append(Matcher::Sep);
        } else if let Some(Matcher::Suffix(string)) = self.0.last_mut() {
            string.push(c);
        } else {
            // Since we know the Matcher::Suffix case in append() will always be false,
            // we can just save the optimizer the trouble.
            self.0.push(Matcher::Suffix(c.to_string()));
        }
    }

    #[inline]
    pub(super) fn append(&mut self, matcher: Matcher) {
        // Optimizations, fusing certain kinds of matchers together.
        let push = !match &matcher {
            Matcher::Sep => {
                matches!(&self.0.last(), Some(Matcher::Sep))
            }
            Matcher::Suffix(suffix) => {
                if let Some(Matcher::Suffix(prefix)) = self.0.last_mut() {
                    prefix.push_str(suffix);
                    true
                } else {
                    false
                }
            }
            Matcher::AnySeq(true) => {
                matches!(&self.0.last(), Some(Matcher::AnySeq(false)))
            }
            _ => false,
        };
        if push {
            self.0.push(matcher);
        }
    }

    pub fn append_glob(&mut self, glob: Glob) {
        for matcher in glob.0 {
            self.append(matcher)
        }
    }
}
