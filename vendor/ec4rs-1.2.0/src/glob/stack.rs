use super::{Glob, Matcher, Splitter};

/// A stack for unwrapping globs to match them,
/// as might happen with alternation.
#[derive(Clone, Debug)]
pub struct GlobStack<'a>(Vec<&'a [Matcher]>);

impl<'a> GlobStack<'a> {
    pub fn new(starter: &Glob) -> GlobStack<'_> {
        GlobStack(vec![starter.0.as_slice()])
    }

    pub fn add_glob(&mut self, glob: &'a Glob) {
        self.0.push(glob.0.as_slice());
    }
    pub fn add_matcher(&mut self, matcher: &'a Matcher) {
        self.0.push(std::slice::from_ref(matcher));
    }

    pub fn next(&mut self) -> Option<&'a Matcher> {
        // ^ impl Iterator?
        while let Some(front) = self.0.last_mut() {
            if let Some((retval, rest)) = front.split_last() {
                *front = rest;
                return Some(retval);
            }
            self.0.pop();
        }
        None
    }
}

enum SavePoint<'a, 'b> {
    Rewind(Splitter<'a>, GlobStack<'b>, &'b Matcher),
    Alts(Splitter<'a>, GlobStack<'b>, &'b [Glob]),
}

/// A stack for saving and restoring state.
pub struct SaveStack<'a, 'b> {
    globs: GlobStack<'b>,
    stack: Vec<SavePoint<'a, 'b>>,
}

impl<'a, 'b> SaveStack<'a, 'b> {
    pub fn new(_: &Splitter<'a>, glob: &'b Glob) -> SaveStack<'a, 'b> {
        SaveStack {
            globs: GlobStack::new(glob),
            stack: Vec::<SavePoint<'a, 'b>>::new(),
        }
    }
    pub fn globs(&mut self) -> &mut GlobStack<'b> {
        &mut self.globs
    }
    pub fn add_rewind(&mut self, splitter: Splitter<'a>, matcher: &'b Matcher) {
        self.stack
            .push(SavePoint::Rewind(splitter, self.globs.clone(), matcher))
    }
    pub fn add_alts(&mut self, splitter: Splitter<'a>, matcher: &'b [Glob]) {
        if let Some((first, rest)) = matcher.split_first() {
            self.stack
                .push(SavePoint::Alts(splitter, self.globs.clone(), rest));
            self.globs().add_glob(first);
        }
    }

    pub fn restore(&mut self) -> Option<Splitter<'a>> {
        loop {
            // There's a continue in here, don't panic.
            break match self.stack.pop()? {
                SavePoint::Rewind(splitter, globs, matcher) => {
                    self.stack.pop();
                    self.globs = globs;
                    self.globs.add_matcher(matcher);
                    Some(splitter)
                }
                SavePoint::Alts(splitter, globs, alts) => {
                    self.globs = globs;
                    if let Some((glob, rest)) = alts.split_first() {
                        if !rest.is_empty() {
                            self.stack.push(SavePoint::Alts(
                                splitter.clone(),
                                self.globs.clone(),
                                rest,
                            ));
                        }
                        self.globs.add_glob(glob);
                        Some(splitter)
                    } else {
                        continue;
                    }
                }
            };
        }
    }
}
