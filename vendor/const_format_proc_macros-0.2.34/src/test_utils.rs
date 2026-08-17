pub trait StrExt {
    fn as_str(&self) -> &str;

    /// Checks that these needles exist consequtively in self.
    ///
    /// Example: `"hello world".consecutive_in_set(&["he", "wor"])` returns `true`.
    /// Example: `"hello world".consecutive_in_set(&["wor", "he"])` returns `false`.
    fn consecutive_in_self(&self, needles: &[&str]) -> bool {
        let mut rem = self.as_str();
        for needle in needles {
            rem = match rem.find(needle) {
                Some(next) => &rem[next + needle.len()..],
                None => return false,
            };
        }
        true
    }
}

impl StrExt for str {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }
}

impl StrExt for String {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }
}
