use super::{Glob, Splitter};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Matcher {
    End,
    AnySeq(bool),
    AnyChar,
    Sep,
    Suffix(String),
    // TODO: Grapheme clusters?
    CharClass(super::FlatSet<char>, bool),
    Range(isize, isize),
    Any(super::FlatSet<super::Glob>),
}

fn try_match<'a, 'b>(
    splitter: Splitter<'a>,
    matcher: &'b Matcher,
    state: &mut super::stack::SaveStack<'a, 'b>,
) -> Option<Splitter<'a>> {
    match matcher {
        Matcher::End => splitter.match_end(),
        Matcher::Sep => splitter.match_sep(),
        Matcher::AnyChar => splitter.match_any(false),
        Matcher::AnySeq(sep) => {
            if let Some(splitter) = splitter.clone().match_any(*sep) {
                state.add_rewind(splitter, matcher);
            }
            Some(splitter)
        }
        Matcher::Suffix(s) => splitter.match_suffix(s.as_str()),
        Matcher::CharClass(cs, should_have) => {
            let (splitter, c) = splitter.next_char()?;
            if cs.contains(c) != *should_have {
                return None;
            }
            Some(splitter)
        }
        Matcher::Range(lower, upper) => splitter.match_number(*lower, *upper),
        Matcher::Any(options) => {
            state.add_alts(splitter.clone(), options.as_slice());
            Some(splitter)
        }
    }
}

#[must_use]
pub fn matches<'a>(path: &'a std::path::Path, glob: &Glob) -> Option<Splitter<'a>> {
    let mut splitter = super::Splitter::new(path)?;
    let mut state = super::stack::SaveStack::new(&splitter, glob);
    loop {
        if let Some(matcher) = state.globs().next() {
            if let Some(splitter_new) = try_match(splitter, matcher, &mut state) {
                splitter = splitter_new;
            } else if let Some(splitter_new) = state.restore() {
                splitter = splitter_new;
            } else {
                return None;
            }
        } else {
            return Some(splitter);
        }
    }
}
