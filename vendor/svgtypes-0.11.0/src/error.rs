/// List of all errors.
#[derive(Debug)]
pub enum Error {
    /// An input data ended earlier than expected.
    ///
    /// Should only appear on invalid input data.
    /// Errors in a valid XML should be handled by errors below.
    UnexpectedEndOfStream,

    /// An input text contains unknown data.
    UnexpectedData(usize),

    /// A provided string doesn't have a valid data.
    ///
    /// For example, if we try to parse a color form `zzz`
    /// string - we will get this error.
    /// But if we try to parse a number list like `1.2 zzz`,
    /// then we will get `InvalidNumber`, because at least some data is valid.
    InvalidValue,

    /// An invalid/unexpected character.
    ///
    /// The first byte is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidChar(Vec<u8>, usize),

    /// An unexpected character instead of an XML space.
    ///
    /// The first string is an actual one, others - expected.
    ///
    /// We are using a single value to reduce the struct size.
    InvalidString(Vec<String>, usize),

    /// An invalid number.
    InvalidNumber(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::UnexpectedEndOfStream => {
                write!(f, "unexpected end of stream")
            }
            Error::UnexpectedData(pos) => {
                write!(f, "unexpected data at position {}", pos)
            }
            Error::InvalidValue => {
                write!(f, "invalid value")
            }
            Error::InvalidChar(ref chars, pos) => {
                // Vec<u8> -> Vec<String>
                let list: Vec<String> = chars
                    .iter()
                    .skip(1)
                    .map(|c| String::from_utf8(vec![*c]).unwrap())
                    .collect();

                write!(
                    f,
                    "expected '{}' not '{}' at position {}",
                    list.join("', '"),
                    chars[0] as char,
                    pos
                )
            }
            Error::InvalidString(ref strings, pos) => {
                write!(
                    f,
                    "expected '{}' not '{}' at position {}",
                    strings[1..].join("', '"),
                    strings[0],
                    pos
                )
            }
            Error::InvalidNumber(pos) => {
                write!(f, "invalid number at position {}", pos)
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "an SVG data parsing error"
    }
}
