use crate::thread::ThreadBound;

pub(crate) trait RegexEngine: Sized + ThreadBound {
    type Error: RegexError;
    fn is_match(&self, text: &str) -> Result<bool, Self::Error>;

    fn pattern(&self) -> &str;
}

impl RegexEngine for fancy_regex::Regex {
    type Error = fancy_regex::Error;

    fn is_match(&self, text: &str) -> Result<bool, Self::Error> {
        fancy_regex::Regex::is_match(self, text)
    }

    fn pattern(&self) -> &str {
        self.as_str()
    }
}

impl RegexEngine for regex::Regex {
    type Error = regex::Error;

    fn is_match(&self, text: &str) -> Result<bool, Self::Error> {
        Ok(regex::Regex::is_match(self, text))
    }

    fn pattern(&self) -> &str {
        self.as_str()
    }
}

pub(crate) trait RegexError {
    fn into_backtrack_error(self) -> Option<fancy_regex::Error>;
}

impl RegexError for fancy_regex::Error {
    fn into_backtrack_error(self) -> Option<fancy_regex::Error> {
        Some(self)
    }
}

impl RegexError for regex::Error {
    fn into_backtrack_error(self) -> Option<fancy_regex::Error> {
        None
    }
}
