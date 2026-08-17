//! Parser example for INI files.

use std::{
    collections::HashMap,
    env, fmt,
    fs::File,
    io::{self, Read},
};

use combine::{parser::char::space, stream::position, *};

#[cfg(feature = "std")]
use combine::stream::easy;

#[cfg(feature = "std")]
use combine::stream::position::SourcePosition;

enum Error<E> {
    Io(io::Error),
    Parse(E),
}

impl<E> fmt::Display for Error<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Parse(ref err) => write!(f, "{}", err),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Ini {
    pub global: HashMap<String, String>,
    pub sections: HashMap<String, HashMap<String, String>>,
}

fn property<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
{
    (
        many1(satisfy(|c| c != '=' && c != '[' && c != ';')),
        token('='),
        many1(satisfy(|c| c != '\n' && c != ';')),
    )
        .map(|(key, _, value)| (key, value))
        .message("while parsing property")
}

fn whitespace<Input>() -> impl Parser<Input>
where
    Input: Stream<Token = char>,
{
    let comment = (token(';'), skip_many(satisfy(|c| c != '\n'))).map(|_| ());
    // Wrap the `spaces().or(comment)` in `skip_many` so that it skips alternating whitespace and
    // comments
    skip_many(skip_many1(space()).or(comment))
}

fn properties<Input>() -> impl Parser<Input, Output = HashMap<String, String>>
where
    Input: Stream<Token = char>,
{
    // After each property we skip any whitespace that followed it
    many(property().skip(whitespace()))
}

fn section<Input>() -> impl Parser<Input, Output = (String, HashMap<String, String>)>
where
    Input: Stream<Token = char>,
{
    (
        between(token('['), token(']'), many(satisfy(|c| c != ']'))),
        whitespace(),
        properties(),
    )
        .map(|(name, _, properties)| (name, properties))
        .message("while parsing section")
}

fn ini<Input>() -> impl Parser<Input, Output = Ini>
where
    Input: Stream<Token = char>,
{
    (whitespace(), properties(), many(section()))
        .map(|(_, global, sections)| Ini { global, sections })
}

#[test]
fn ini_ok() {
    let text = r#"
language=rust

[section]
name=combine; Comment
type=LL(1)

"#;
    let mut expected = Ini {
        global: HashMap::new(),
        sections: HashMap::new(),
    };
    expected
        .global
        .insert(String::from("language"), String::from("rust"));

    let mut section = HashMap::new();
    section.insert(String::from("name"), String::from("combine"));
    section.insert(String::from("type"), String::from("LL(1)"));
    expected.sections.insert(String::from("section"), section);

    let result = ini().parse(text).map(|t| t.0);
    assert_eq!(result, Ok(expected));
}

#[cfg(feature = "std")]
#[test]
fn ini_error() {
    let text = "[error";
    let result = ini().easy_parse(position::Stream::new(text)).map(|t| t.0);
    assert_eq!(
        result,
        Err(easy::Errors {
            position: SourcePosition { line: 1, column: 7 },
            errors: vec![
                easy::Error::end_of_input(),
                easy::Error::Expected(']'.into()),
                easy::Error::Message("while parsing section".into()),
            ],
        })
    );
}

fn main() {
    let result = match env::args().nth(1) {
        Some(file) => File::open(file).map_err(Error::Io).and_then(main_),
        None => main_(io::stdin()),
    };
    match result {
        Ok(_) => println!("OK"),
        Err(err) => println!("{}", err),
    }
}

#[cfg(feature = "std")]
fn main_<R>(mut read: R) -> Result<(), Error<easy::Errors<char, String, SourcePosition>>>
where
    R: Read,
{
    let mut text = String::new();
    read.read_to_string(&mut text).map_err(Error::Io)?;
    ini()
        .easy_parse(position::Stream::new(&*text))
        .map_err(|err| Error::Parse(err.map_range(|s| s.to_string())))?;
    Ok(())
}

#[cfg(not(feature = "std"))]
fn main_<R>(mut read: R) -> Result<(), Error<::combine::error::StringStreamError>>
where
    R: Read,
{
    let mut text = String::new();
    read.read_to_string(&mut text).map_err(Error::Io)?;
    ini()
        .parse(position::Stream::new(&*text))
        .map_err(Error::Parse)?;
    Ok(())
}
