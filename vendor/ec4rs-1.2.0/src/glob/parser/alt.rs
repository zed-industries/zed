use crate::glob::{Glob, Matcher};

pub struct AltStack(Vec<AltBuilder>);

impl AltStack {
    pub const fn new() -> AltStack {
        AltStack(vec![])
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, glob: Glob) {
        self.0.push(AltBuilder::new(glob));
    }

    /// Adds a glob to the top builder of the stack.
    ///
    /// Returns the glob if there is no builder on the stack.
    #[must_use]
    pub fn add_alt(&mut self, glob: Glob) -> Option<Glob> {
        if let Some(ab) = self.0.last_mut() {
            ab.add(glob);
            None
        } else {
            Some(glob)
        }
    }

    pub fn join_and_pop(&mut self, glob: Glob) -> (Glob, bool) {
        if let Some(mut builder) = self.0.pop() {
            builder.add(glob);
            (builder.join(), self.is_empty())
        } else {
            (glob, true)
        }
    }

    pub fn add_alt_and_pop(&mut self, glob: Glob) -> (Glob, bool) {
        if let Some(mut builder) = self.0.pop() {
            builder.add(glob);
            (builder.build(), false)
        } else {
            (glob, true)
        }
    }
}

pub struct AltBuilder {
    glob: Glob,
    options: Vec<Glob>,
}

impl AltBuilder {
    pub const fn new(glob: Glob) -> AltBuilder {
        AltBuilder {
            glob,
            options: vec![],
        }
    }
    pub fn add(&mut self, glob: Glob) {
        self.options.push(glob);
    }
    pub fn build(mut self) -> Glob {
        match self.options.len() {
            0 => {
                self.glob.append_char('{');
                self.glob.append_char('}');
                self.glob
            }
            1 => {
                self.glob.append_char('{');
                for matcher in self.options.pop().unwrap().0 {
                    self.glob.append(matcher);
                }
                self.glob.append_char('}');
                self.glob
            }
            _ => {
                self.options
                    .sort_by(|a, b| (!a.0.is_empty()).cmp(&!b.0.is_empty()));
                self.options.dedup();
                self.glob.append(Matcher::Any(self.options.into()));
                self.glob
            }
        }
    }
    pub fn join(mut self) -> Glob {
        let mut first = true;
        self.glob.append_char('{');
        for option in self.options {
            if first {
                first = false;
            } else {
                self.glob.append_char(',')
            }
            self.glob.append_glob(option);
        }
        self.glob
    }
}
