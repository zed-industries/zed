/// A stream of tokens
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenStream(pub String);

impl From<String> for TokenStream {
    fn from(tokens: String) -> Self {
        Self(tokens)
    }
}

impl From<&String> for TokenStream {
    fn from(tokens: &String) -> Self {
        Self(tokens.to_string())
    }
}

impl From<&str> for TokenStream {
    fn from(tokens: &str) -> Self {
        Self(tokens.to_string())
    }
}

impl TokenStream {
    pub fn new() -> Self {
        Self(String::new())
    }

    /// Appends another stream to the stream
    ///
    /// note: a space will be inserted before the other stream
    pub fn combine<T: AsRef<TokenStream>>(&mut self, other: T) {
        self.push_space();
        self.0.push_str(&other.as_ref().0)
    }

    #[must_use]
    pub fn join(&self, value: &str) -> Self {
        Self(format!("{}{value}", self.0))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// View the stream as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert the stream into a `String`
    pub fn into_string(self) -> String {
        self.0
    }

    /// Parse the token stream as something
    ///
    /// Mostly used with `proc_macro2::TokenStream` or `proc_macro::TokenStream`
    pub fn parse<T: core::str::FromStr>(self) -> Result<T, T::Err> {
        self.into_string().parse()
    }

    pub(crate) fn push_space(&mut self) {
        match self.last_char() {
            None | Some(' ') => {}
            _ => self.0.push(' '),
        }
    }

    pub fn push(&mut self, c: char) {
        self.0.push(c)
    }

    pub fn push_str(&mut self, str: &str) {
        self.0.push_str(str)
    }

    fn last_char(&self) -> Option<char> {
        self.0.chars().last()
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(iter: I) -> Self {
        iter.into_iter()
            .fold(None, |accum: Option<TokenStream>, n| {
                let mut ts = accum.unwrap_or_default();
                ts.combine(&n);
                Some(ts)
            })
            .unwrap_or_else(TokenStream::new)
    }
}

impl AsRef<TokenStream> for TokenStream {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<[u8]> for TokenStream {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// A delimiter around a block of code
#[derive(Copy, Clone)]
pub enum Delimiter {
    /// `[]`
    Bracket,
    /// `{}`
    Brace,
    /// `()`
    Parenthesis,
}

impl Delimiter {
    /// The opening delimiter
    pub fn open(self) -> char {
        match self {
            Delimiter::Bracket => '[',
            Delimiter::Brace => '{',
            Delimiter::Parenthesis => '(',
        }
    }

    /// The closing delimiter
    pub fn close(self) -> char {
        match self {
            Delimiter::Bracket => ']',
            Delimiter::Brace => '}',
            Delimiter::Parenthesis => ')',
        }
    }
}

/// A literal of some sort
pub struct Literal {
    inner: String,
}

macro_rules! unsuffixed {
    ($ty:ty => $name:ident) => {
        pub fn $name(n: $ty) -> Self {
            Self {
                inner: n.to_string(),
            }
        }
    };
}

impl Literal {
    unsuffixed!(i64 => i64_unsuffixed);
    unsuffixed!(usize => usize_unsuffixed);
    unsuffixed!(u32 => u32_unsuffixed);
    unsuffixed!(u16 => u16_unsuffixed);
    unsuffixed!(u8 => u8_unsuffixed);

    pub fn byte_string(s: &str) -> Self {
        Self {
            inner: format!("b\"{s}\""),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl core::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
