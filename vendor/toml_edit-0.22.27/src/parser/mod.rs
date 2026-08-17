#![allow(clippy::type_complexity)]

use std::cell::RefCell;
pub(crate) mod array;
pub(crate) mod datetime;
pub(crate) mod document;
pub(crate) mod error;
pub(crate) mod inline_table;
pub(crate) mod key;
pub(crate) mod numbers;
pub(crate) mod state;
pub(crate) mod strings;
pub(crate) mod table;
pub(crate) mod trivia;
pub(crate) mod value;

pub(crate) use crate::error::TomlError;

pub(crate) fn parse_document<S: AsRef<str>>(raw: S) -> Result<crate::ImDocument<S>, TomlError> {
    use prelude::*;

    let b = new_input(raw.as_ref());
    let state = RefCell::new(state::ParseState::new());
    let state_ref = &state;
    document::document(state_ref)
        .parse(b.clone())
        .map_err(|e| TomlError::new(e, b))?;
    let doc = state
        .into_inner()
        .into_document(raw)
        .map_err(|e| TomlError::custom(e.to_string(), None))?;
    Ok(doc)
}

pub(crate) fn parse_key(raw: &str) -> Result<crate::Key, TomlError> {
    use prelude::*;

    let b = new_input(raw);
    let result = key::simple_key.parse(b.clone());
    match result {
        Ok((raw, key)) => {
            Ok(crate::Key::new(key).with_repr_unchecked(crate::Repr::new_unchecked(raw)))
        }
        Err(e) => Err(TomlError::new(e, b)),
    }
}

pub(crate) fn parse_key_path(raw: &str) -> Result<Vec<crate::Key>, TomlError> {
    use prelude::*;

    let b = new_input(raw);
    let result = key::key.parse(b.clone());
    match result {
        Ok(keys) => Ok(keys),
        Err(e) => Err(TomlError::new(e, b)),
    }
}

pub(crate) fn parse_value(raw: &str) -> Result<crate::Value, TomlError> {
    use prelude::*;

    let b = new_input(raw);
    let parsed = value::value.parse(b.clone());
    match parsed {
        Ok(value) => Ok(value),
        Err(e) => Err(TomlError::new(e, b)),
    }
}

pub(crate) mod prelude {
    pub(crate) use winnow::combinator::dispatch;
    pub(crate) use winnow::error::ContextError;
    pub(crate) use winnow::error::FromExternalError;
    pub(crate) use winnow::error::StrContext;
    pub(crate) use winnow::error::StrContextValue;
    pub(crate) use winnow::ModalParser;
    pub(crate) use winnow::ModalResult;
    pub(crate) use winnow::Parser as _;

    pub(crate) type Input<'b> =
        winnow::Stateful<winnow::LocatingSlice<&'b winnow::BStr>, RecursionCheck>;

    pub(crate) fn new_input(s: &str) -> Input<'_> {
        winnow::Stateful {
            input: winnow::LocatingSlice::new(winnow::BStr::new(s)),
            state: Default::default(),
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub(crate) struct RecursionCheck {
        #[cfg(not(feature = "unbounded"))]
        current: usize,
    }

    #[cfg(not(feature = "unbounded"))]
    const LIMIT: usize = 80;

    impl RecursionCheck {
        pub(crate) fn check_depth(_depth: usize) -> Result<(), super::error::CustomError> {
            #[cfg(not(feature = "unbounded"))]
            if LIMIT <= _depth {
                return Err(super::error::CustomError::RecursionLimitExceeded);
            }

            Ok(())
        }

        fn enter(&mut self) -> Result<(), super::error::CustomError> {
            #[cfg(not(feature = "unbounded"))]
            {
                self.current += 1;
                if LIMIT <= self.current {
                    return Err(super::error::CustomError::RecursionLimitExceeded);
                }
            }
            Ok(())
        }

        fn exit(&mut self) {
            #[cfg(not(feature = "unbounded"))]
            {
                self.current -= 1;
            }
        }
    }

    pub(crate) fn check_recursion<'b, O>(
        mut parser: impl ModalParser<Input<'b>, O, ContextError>,
    ) -> impl ModalParser<Input<'b>, O, ContextError> {
        move |input: &mut Input<'b>| {
            input
                .state
                .enter()
                .map_err(|err| winnow::error::ErrMode::from_external_error(input, err).cut())?;
            let result = parser.parse_next(input);
            input.state.exit();
            result
        }
    }
}

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;
    use snapbox::assert_data_eq;
    use snapbox::prelude::*;

    #[test]
    fn documents() {
        let documents = [
            "",
            r#"
# This is a TOML document.

title = "TOML Example"

    [owner]
    name = "Tom Preston-Werner"
    dob = 1979-05-27T07:32:00-08:00 # First class dates

    [database]
    server = "192.168.1.1"
    ports = [ 8001, 8001, 8002 ]
    connection_max = 5000
    enabled = true

    [servers]

    # Indentation (tabs and/or spaces) is allowed but not required
[servers.alpha]
    ip = "10.0.0.1"
    dc = "eqdc10"

    [servers.beta]
    ip = "10.0.0.2"
    dc = "eqdc10"

    [clients]
    data = [ ["gamma", "delta"], [1, 2] ]

    # Line breaks are OK when inside arrays
hosts = [
    "alpha",
    "omega"
]

   'some.weird .stuff'   =  """
                         like
                         that
                      #   """ # this broke my syntax highlighting
   " also. like " = '''
that
'''
   double = 2e39 # this number looks familiar
# trailing comment"#,
            r#""#,
            r#"  "#,
            r#" hello = 'darkness' # my old friend
"#,
            r#"[parent . child]
key = "value"
"#,
            r#"hello.world = "a"
"#,
            r#"foo = 1979-05-27 # Comment
"#,
        ];
        for input in documents {
            dbg!(input);
            let parsed = parse_document(input).map(|d| d.into_mut());
            let doc = match parsed {
                Ok(doc) => doc,
                Err(err) => {
                    panic!("Parse error: {err:?}\nFailed to parse:\n```\n{input}\n```")
                }
            };

            assert_data_eq!(doc.to_string(), input.raw());
        }
    }

    #[test]
    fn documents_parse_only() {
        let parse_only = ["\u{FEFF}
[package]
name = \"foo\"
version = \"0.0.1\"
authors = []
"];
        for input in parse_only {
            dbg!(input);
            let parsed = parse_document(input).map(|d| d.into_mut());
            match parsed {
                Ok(_) => (),
                Err(err) => {
                    panic!("Parse error: {err:?}\nFailed to parse:\n```\n{input}\n```")
                }
            }
        }
    }

    #[test]
    fn invalid_documents() {
        let invalid_inputs = [r#" hello = 'darkness' # my old friend
$"#];
        for input in invalid_inputs {
            dbg!(input);
            let parsed = parse_document(input).map(|d| d.into_mut());
            assert!(parsed.is_err(), "Input: {input:?}");
        }
    }
}
