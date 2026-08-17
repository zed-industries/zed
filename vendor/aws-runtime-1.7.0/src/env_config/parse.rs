/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Profile file parsing
//!
//! This file implements profile file parsing at a very literal level. Prior to actually being used,
//! profiles must be normalized into a canonical form. Constructions that will eventually be
//! deemed invalid are accepted during parsing such as:
//! - keys that are invalid identifiers: `a b = c`
//! - profiles with invalid names
//! - profile name normalization (`profile foo` => `foo`)

use crate::env_config::source::File;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A set of profiles that still carries a reference to the underlying data
pub(super) type RawProfileSet<'a> = HashMap<&'a str, HashMap<Cow<'a, str>, Cow<'a, str>>>;

/// Characters considered to be whitespace by the spec
///
/// Profile parsing is actually quite strict about what is and is not whitespace, so use this instead
/// of `.is_whitespace()` / `.trim()`
pub(crate) const WHITESPACE: &[char] = &[' ', '\t'];
const COMMENT: &[char] = &['#', ';'];

/// Location for use during error reporting
#[derive(Clone, Debug, Eq, PartialEq)]
struct Location {
    line_number: usize,
    path: String,
}

/// An error encountered while parsing a profile
#[derive(Debug, Clone)]
pub struct EnvConfigParseError {
    /// Location where this error occurred
    location: Location,

    /// Error message
    message: String,
}

impl Display for EnvConfigParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error parsing {} on line {}:\n  {}",
            self.location.path, self.location.line_number, self.message
        )
    }
}

impl Error for EnvConfigParseError {}

/// Validate that a line represents a valid subproperty
///
/// - Sub-properties looks like regular properties (`k=v`) that are nested within an existing property.
/// - Sub-properties must be validated for compatibility with other SDKs, but they are not actually
///   parsed into structured data.
fn validate_subproperty(value: &str, location: Location) -> Result<(), EnvConfigParseError> {
    if value.trim_matches(WHITESPACE).is_empty() {
        Ok(())
    } else {
        parse_property_line(value)
            .map_err(|err| err.into_error("sub-property", location))
            .map(|_| ())
    }
}

fn is_empty_line(line: &str) -> bool {
    line.trim_matches(WHITESPACE).is_empty()
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with(COMMENT)
}

/// Parser for profile files
struct Parser<'a> {
    /// In-progress profile representation
    data: RawProfileSet<'a>,

    /// Parser state
    state: State<'a>,

    /// Parser source location
    ///
    /// Location is tracked to facilitate error reporting
    location: Location,
}

enum State<'a> {
    Starting,
    ReadingProfile {
        profile: &'a str,
        property: Option<Cow<'a, str>>,
        is_subproperty: bool,
    },
}

/// Parse `file` into a `RawProfileSet`
pub(super) fn parse_profile_file(file: &File) -> Result<RawProfileSet<'_>, EnvConfigParseError> {
    let mut parser = Parser {
        data: HashMap::new(),
        state: State::Starting,
        location: Location {
            line_number: 0,
            path: file.path.clone().unwrap_or_default(),
        },
    };
    parser.parse_profile(&file.contents)?;
    Ok(parser.data)
}

impl<'a> Parser<'a> {
    /// Parse `file` containing profile data into `self.data`.
    fn parse_profile(&mut self, file: &'a str) -> Result<(), EnvConfigParseError> {
        for (line_number, line) in file.lines().enumerate() {
            self.location.line_number = line_number + 1; // store a 1-indexed line number
            if is_empty_line(line) || is_comment_line(line) {
                continue;
            }
            if line.starts_with('[') {
                self.read_profile_line(line)?;
            } else if line.starts_with(WHITESPACE) {
                self.read_property_continuation(line)?;
            } else {
                self.read_property_line(line)?;
            }
        }
        Ok(())
    }

    /// Parse a property line like `a = b`
    ///
    /// A property line is only valid when we're within a profile definition, `[profile foo]`
    fn read_property_line(&mut self, line: &'a str) -> Result<(), EnvConfigParseError> {
        let location = &self.location;
        let (current_profile, name) = match &self.state {
            State::Starting => return Err(self.make_error("Expected a profile definition")),
            State::ReadingProfile { profile, .. } => (
                self.data.get_mut(*profile).expect("profile must exist"),
                profile,
            ),
        };
        let (k, v) = parse_property_line(line)
            .map_err(|err| err.into_error("property", location.clone()))?;
        self.state = State::ReadingProfile {
            profile: name,
            property: Some(k.clone()),
            is_subproperty: v.is_empty(),
        };
        current_profile.insert(k, v.into());
        Ok(())
    }

    /// Create a location-tagged error message
    fn make_error(&self, message: &str) -> EnvConfigParseError {
        EnvConfigParseError {
            location: self.location.clone(),
            message: message.into(),
        }
    }

    /// Parse the lines of a property after the first line.
    ///
    /// This is triggered by lines that start with whitespace.
    fn read_property_continuation(&mut self, line: &'a str) -> Result<(), EnvConfigParseError> {
        let current_property = match &self.state {
            State::Starting => return Err(self.make_error("Expected a profile definition")),
            State::ReadingProfile {
                profile,
                property: Some(property),
                is_subproperty,
            } => {
                if *is_subproperty {
                    validate_subproperty(line, self.location.clone())?;
                }
                self.data
                    .get_mut(*profile)
                    .expect("profile must exist")
                    .get_mut(property.as_ref())
                    .expect("property must exist")
            }
            State::ReadingProfile {
                profile: _,
                property: None,
                ..
            } => return Err(self.make_error("Expected a property definition, found continuation")),
        };
        let line = line.trim_matches(WHITESPACE);
        let current_property = current_property.to_mut();
        current_property.push('\n');
        current_property.push_str(line);
        Ok(())
    }

    fn read_profile_line(&mut self, line: &'a str) -> Result<(), EnvConfigParseError> {
        let line = prepare_line(line, false);
        let profile_name = line
            .strip_prefix('[')
            .ok_or_else(|| self.make_error("Profile definition must start with '['"))?
            .strip_suffix(']')
            .ok_or_else(|| self.make_error("Profile definition must end with ']'"))?;
        if !self.data.contains_key(profile_name) {
            self.data.insert(profile_name, Default::default());
        }
        self.state = State::ReadingProfile {
            profile: profile_name,
            property: None,
            is_subproperty: false,
        };
        Ok(())
    }
}

/// Error encountered while parsing a property
#[derive(Debug, Eq, PartialEq)]
enum PropertyError {
    NoEquals,
    NoName,
}

impl PropertyError {
    fn into_error(self, ctx: &str, location: Location) -> EnvConfigParseError {
        let mut ctx = ctx.to_string();
        match self {
            PropertyError::NoName => {
                ctx.get_mut(0..1).unwrap().make_ascii_uppercase();
                EnvConfigParseError {
                    location,
                    message: format!("{ctx} did not have a name"),
                }
            }
            PropertyError::NoEquals => EnvConfigParseError {
                location,
                message: format!("Expected an '=' sign defining a {ctx}"),
            },
        }
    }
}

/// Parse a property line into a key-value pair
fn parse_property_line(line: &str) -> Result<(Cow<'_, str>, &str), PropertyError> {
    let line = prepare_line(line, true);
    let (k, v) = line.split_once('=').ok_or(PropertyError::NoEquals)?;
    let k = k.trim_matches(WHITESPACE);
    let v = v.trim_matches(WHITESPACE);
    if k.is_empty() {
        return Err(PropertyError::NoName);
    }
    // We don't want to blindly use `alloc::str::to_ascii_lowercase` because it
    // always allocates. Instead, we check for uppercase ascii letters. Then,
    // we only allocate in the case that there ARE letters that need to be
    // lower-cased.
    Ok((to_ascii_lowercase(k), v))
}

pub(crate) fn to_ascii_lowercase(s: &str) -> Cow<'_, str> {
    if s.bytes().any(|b| b.is_ascii_uppercase()) {
        Cow::Owned(s.to_ascii_lowercase())
    } else {
        Cow::Borrowed(s)
    }
}

/// Prepare a line for parsing
///
/// Because leading whitespace is significant, this method should only be called after determining
/// whether a line represents a property (no whitespace) or a sub-property (whitespace).
/// This function preprocesses a line to simplify parsing:
/// 1. Strip leading and trailing whitespace
/// 2. Remove trailing comments
///
/// Depending on context, comment characters may need to be preceded by whitespace to be considered
/// comments.
fn prepare_line(line: &str, comments_need_whitespace: bool) -> &str {
    let line = line.trim_matches(WHITESPACE);
    let mut prev_char_whitespace = false;
    let mut comment_idx = None;
    for (idx, chr) in line.char_indices() {
        if (COMMENT.contains(&chr)) && (prev_char_whitespace || !comments_need_whitespace) {
            comment_idx = Some(idx);
            break;
        }
        prev_char_whitespace = chr.is_whitespace();
    }
    comment_idx
        .map(|idx| &line[..idx])
        .unwrap_or(line)
        // trimming the comment might result in more whitespace that needs to be handled
        .trim_matches(WHITESPACE)
}

#[cfg(test)]
mod test {
    use super::{parse_profile_file, prepare_line, Location};
    use crate::env_config::file::EnvConfigFileKind;
    use crate::env_config::parse::{parse_property_line, PropertyError};
    use crate::env_config::source::File;
    use std::borrow::Cow;

    // most test cases covered by the JSON test suite

    #[test]
    fn property_parsing() {
        fn ok<'a>(key: &'a str, value: &'a str) -> Result<(Cow<'a, str>, &'a str), PropertyError> {
            Ok((Cow::Borrowed(key), value))
        }

        assert_eq!(parse_property_line("a = b"), ok("a", "b"));
        assert_eq!(parse_property_line("a=b"), ok("a", "b"));
        assert_eq!(parse_property_line("a = b "), ok("a", "b"));
        assert_eq!(parse_property_line(" a = b "), ok("a", "b"));
        assert_eq!(parse_property_line(" a = b 🐱 "), ok("a", "b 🐱"));
        assert_eq!(parse_property_line("a b"), Err(PropertyError::NoEquals));
        assert_eq!(parse_property_line("= b"), Err(PropertyError::NoName));
        assert_eq!(parse_property_line("a =    "), ok("a", ""));
        assert_eq!(
            parse_property_line("something_base64=aGVsbG8gZW50aHVzaWFzdGljIHJlYWRlcg=="),
            ok("something_base64", "aGVsbG8gZW50aHVzaWFzdGljIHJlYWRlcg==")
        );

        assert_eq!(parse_property_line("ABc = DEF"), ok("abc", "DEF"));
    }

    #[test]
    fn prepare_line_strips_comments() {
        assert_eq!(
            prepare_line("name = value # Comment with # sign", true),
            "name = value"
        );

        assert_eq!(
            prepare_line("name = value#Comment # sign", true),
            "name = value#Comment"
        );

        assert_eq!(
            prepare_line("name = value#Comment # sign", false),
            "name = value"
        );
    }

    #[test]
    fn error_line_numbers() {
        let file = File {
            kind: EnvConfigFileKind::Config,
            path: Some("~/.aws/config".into()),
            contents: "[default\nk=v".into(),
        };
        let err = parse_profile_file(&file).expect_err("parsing should fail");
        assert_eq!(err.message, "Profile definition must end with ']'");
        assert_eq!(
            err.location,
            Location {
                path: "~/.aws/config".into(),
                line_number: 1
            }
        )
    }
}
