use anyhow::{Context as _, Result};
use rand::Rng as _;
use smallvec::SmallVec;
use std::{collections::BTreeMap, ops::Range};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snippet {
    pub text: String,
    pub tabstops: Vec<TabStop>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TabStop {
    pub ranges: SmallVec<[Range<isize>; 2]>,
    pub choices: Option<Vec<String>>,
}

/// Parsing state for `$NAME` and `${NAME:default}` variables.
struct Variables<'a> {
    resolve: &'a mut dyn FnMut(&str) -> Option<String>,
    /// Variables the resolver did not know, which become placeholders once all
    /// explicit tabstop indices are known.
    unresolved: Vec<(String, Range<isize>)>,
}

impl Snippet {
    /// Parses a snippet, resolving variables that don't depend on the editor
    /// state (date, time, random values).
    pub fn parse(source: &str) -> Result<Self> {
        Self::parse_with_variables(source, resolve_builtin_variable)
    }

    /// Parses a snippet, using `resolve` to look up variable values.
    ///
    /// `None` marks a variable as unknown: its default (or its name) is inserted
    /// as a placeholder. `Some("")` marks it as known but empty: its default, if
    /// any, is inserted.
    pub fn parse_with_variables(
        source: &str,
        mut resolve: impl FnMut(&str) -> Option<String>,
    ) -> Result<Self> {
        let mut text = String::with_capacity(source.len());
        let mut tabstops = BTreeMap::new();
        let mut variables = Variables {
            resolve: &mut resolve,
            unresolved: Vec::new(),
        };
        parse_snippet(source, false, &mut text, &mut tabstops, &mut variables)
            .context("failed to parse snippet")?;

        let first_placeholder_index = tabstops.keys().next_back().map_or(1, |index| index + 1);
        let mut placeholder_names: Vec<String> = Vec::new();
        for (name, range) in variables.unresolved {
            let position = match placeholder_names
                .iter()
                .position(|existing| *existing == name)
            {
                Some(position) => position,
                None => {
                    placeholder_names.push(name);
                    placeholder_names.len() - 1
                }
            };
            tabstops
                .entry(first_placeholder_index + position)
                .or_default()
                .ranges
                .push(range);
        }

        let len = text.len() as isize;
        let final_tabstop = tabstops.remove(&0);
        let mut tabstops = tabstops.into_values().collect::<Vec<_>>();

        if let Some(final_tabstop) = final_tabstop {
            tabstops.push(final_tabstop);
        } else {
            let end_tabstop = TabStop {
                ranges: [len..len].into_iter().collect(),
                choices: None,
            };

            if !tabstops.last().is_some_and(|t| *t == end_tabstop) {
                tabstops.push(end_tabstop);
            }
        }

        Ok(Snippet { text, tabstops })
    }
}

/// Resolves variables that don't depend on editor state.
pub fn resolve_builtin_variable(name: &str) -> Option<String> {
    use chrono::Local;

    let format = |format: &str| Local::now().format(format).to_string();
    let value = match name {
        "CURRENT_YEAR" => format("%Y"),
        "CURRENT_YEAR_SHORT" => format("%y"),
        "CURRENT_MONTH" => format("%m"),
        "CURRENT_MONTH_NAME" => format("%B"),
        "CURRENT_MONTH_NAME_SHORT" => format("%b"),
        "CURRENT_DATE" => format("%d"),
        "CURRENT_DAY_NAME" => format("%A"),
        "CURRENT_DAY_NAME_SHORT" => format("%a"),
        "CURRENT_HOUR" => format("%H"),
        "CURRENT_MINUTE" => format("%M"),
        "CURRENT_SECOND" => format("%S"),
        "CURRENT_MILLISECOND" => format("%3f"),
        "CURRENT_SECONDS_UNIX" => Local::now().timestamp().to_string(),
        "CURRENT_MILLISECONDS_UNIX" => Local::now().timestamp_millis().to_string(),
        "CURRENT_TIMEZONE_OFFSET" => format("%:z"),
        "CURRENT_TIMEZONE_NAME" => iana_time_zone::get_timezone().unwrap_or_default(),
        "RANDOM" => random_digits(10),
        "RANDOM_HEX" => random_digits(16),
        "UUID" => uuid::Uuid::new_v4().to_string(),
        _ => return None,
    };
    Some(value)
}

fn random_digits(radix: u32) -> String {
    let mut rng = rand::rng();
    (0..6)
        .filter_map(|_| char::from_digit(rng.random_range(0..radix), radix))
        .collect()
}

fn is_variable_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn parse_snippet<'a>(
    mut source: &'a str,
    nested: bool,
    text: &mut String,
    tabstops: &mut BTreeMap<usize, TabStop>,
    variables: &mut Variables,
) -> Result<&'a str> {
    loop {
        match source.chars().next() {
            None => return Ok(""),
            Some('$') => {
                let name_start = source[1..].strip_prefix('{').unwrap_or(&source[1..]);
                if name_start.starts_with(is_variable_start) {
                    source = parse_variable(&source[1..], text, tabstops, variables)?;
                } else {
                    source = parse_tabstop(&source[1..], text, tabstops, variables)?;
                }
            }
            Some('\\') => {
                // As specified in the LSP spec (`Grammar` section),
                // backslashes can escape some characters:
                // https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#snippet_syntax
                source = &source[1..];
                if let Some(c) = source.chars().next() {
                    if c == '$' || c == '\\' || c == '}' {
                        text.push(c);
                        // All escapable characters are 1 byte long:
                        source = &source[1..];
                    } else {
                        text.push('\\');
                    }
                } else {
                    text.push('\\');
                }
            }
            Some('}') => {
                if nested {
                    return Ok(source);
                } else {
                    text.push('}');
                    source = &source[1..];
                }
            }
            Some(_) => {
                let chunk_end = source.find(['}', '$', '\\']).unwrap_or(source.len());
                let (chunk, rest) = source.split_at(chunk_end);
                text.push_str(chunk);
                source = rest;
            }
        }
    }
}

fn parse_variable<'a>(
    mut source: &'a str,
    text: &mut String,
    tabstops: &mut BTreeMap<usize, TabStop>,
    variables: &mut Variables,
) -> Result<&'a str> {
    let braced = source.starts_with('{');
    if braced {
        source = &source[1..];
    }

    let name_end = source
        .find(|c: char| c != '_' && !c.is_ascii_alphanumeric())
        .unwrap_or(source.len());
    let (name, rest) = source.split_at(name_end);
    source = rest;

    let start = text.len();
    // A resolved variable discards any tabstops declared in its default.
    let mut checkpoint = None;
    if braced {
        if source.starts_with(':') {
            checkpoint = Some((tabstops.clone(), variables.unresolved.len()));
            source = parse_snippet(&source[1..], true, text, tabstops, variables)?;
        }

        if source.starts_with('}') {
            source = &source[1..];
        } else {
            anyhow::bail!("expected a closing brace");
        }
    }

    match (variables.resolve)(name) {
        Some(value) if !value.is_empty() => {
            text.truncate(start);
            if let Some((saved_tabstops, unresolved_len)) = checkpoint {
                *tabstops = saved_tabstops;
                variables.unresolved.truncate(unresolved_len);
            }
            text.push_str(&value);
        }
        Some(_) => {}
        None => {
            if text.len() == start {
                text.push_str(name);
            }
            variables
                .unresolved
                .push((name.to_string(), start as isize..text.len() as isize));
        }
    }

    Ok(source)
}

fn parse_tabstop<'a>(
    mut source: &'a str,
    text: &mut String,
    tabstops: &mut BTreeMap<usize, TabStop>,
    variables: &mut Variables,
) -> Result<&'a str> {
    let tabstop_start = text.len();
    let tabstop_index;
    let mut choices = None;

    if source.starts_with('{') {
        let (index, rest) = parse_int(&source[1..])?;
        tabstop_index = index;
        source = rest;

        if source.starts_with("|") {
            (source, choices) = parse_choices(&source[1..], text)?;
        }

        if source.starts_with(':') {
            source = parse_snippet(&source[1..], true, text, tabstops, variables)?;
        }

        if source.starts_with('}') {
            source = &source[1..];
        } else {
            anyhow::bail!("expected a closing brace");
        }
    } else {
        let (index, rest) = parse_int(source)?;
        tabstop_index = index;
        source = rest;
    }

    tabstops
        .entry(tabstop_index)
        .or_insert_with(|| TabStop {
            ranges: Default::default(),
            choices,
        })
        .ranges
        .push(tabstop_start as isize..text.len() as isize);
    Ok(source)
}

fn parse_int(source: &str) -> Result<(usize, &str)> {
    let len = source
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(source.len());
    anyhow::ensure!(len > 0, "expected an integer");
    let (prefix, suffix) = source.split_at(len);
    Ok((prefix.parse()?, suffix))
}

fn parse_choices<'a>(
    mut source: &'a str,
    text: &mut String,
) -> Result<(&'a str, Option<Vec<String>>)> {
    let mut found_default_choice = false;
    let mut current_choice = String::new();
    let mut choices = Vec::new();

    loop {
        match source.chars().next() {
            None => return Ok(("", Some(choices))),
            Some('\\') => {
                source = &source[1..];

                if let Some(c) = source.chars().next() {
                    current_choice.push(c);
                    if !found_default_choice {
                        text.push(c);
                    }
                    source = &source[c.len_utf8()..];
                }
            }
            Some(',') => {
                found_default_choice = true;
                source = &source[1..];
                choices.push(current_choice);
                current_choice = String::new();
            }
            Some('|') => {
                source = &source[1..];
                choices.push(current_choice);
                return Ok((source, Some(choices)));
            }
            Some(_) => {
                let chunk_end = source.find([',', '|', '\\']);

                anyhow::ensure!(
                    chunk_end.is_some(),
                    "Placeholder choice doesn't contain closing pipe-character '|'"
                );

                let (chunk, rest) = source.split_at(chunk_end.unwrap());

                if !found_default_choice {
                    text.push_str(chunk);
                }

                current_choice.push_str(chunk);
                source = rest;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snippet_without_tabstops() {
        let snippet = Snippet::parse("one-two-three").unwrap();
        assert_eq!(snippet.text, "one-two-three");
        assert_eq!(tabstops(&snippet), &[vec![13..13]]);
    }

    #[test]
    fn test_snippet_with_tabstops() {
        let snippet = Snippet::parse("one$1two").unwrap();
        assert_eq!(snippet.text, "onetwo");
        assert_eq!(tabstops(&snippet), &[vec![3..3], vec![6..6]]);
        assert_eq!(tabstop_choices(&snippet), &[&None, &None]);

        // Multi-digit numbers
        let snippet = Snippet::parse("one$123-$99-two").unwrap();
        assert_eq!(snippet.text, "one--two");
        assert_eq!(tabstops(&snippet), &[vec![4..4], vec![3..3], vec![8..8]]);
        assert_eq!(tabstop_choices(&snippet), &[&None, &None, &None]);
    }

    #[test]
    fn test_snippet_with_last_tabstop_at_end() {
        let snippet = Snippet::parse(r#"foo.$1"#).unwrap();

        // If the final tabstop is already at the end of the text, don't insert
        // an additional tabstop at the end.
        assert_eq!(snippet.text, r#"foo."#);
        assert_eq!(tabstops(&snippet), &[vec![4..4]]);
        assert_eq!(tabstop_choices(&snippet), &[&None]);
    }

    #[test]
    fn test_snippet_with_explicit_final_tabstop() {
        let snippet = Snippet::parse(r#"<div class="$1">$0</div>"#).unwrap();

        // If the final tabstop is explicitly specified via '$0', then
        // don't insert an additional tabstop at the end.
        assert_eq!(snippet.text, r#"<div class=""></div>"#);
        assert_eq!(tabstops(&snippet), &[vec![12..12], vec![14..14]]);
        assert_eq!(tabstop_choices(&snippet), &[&None, &None]);
    }

    #[test]
    fn test_snippet_with_placeholders() {
        let snippet = Snippet::parse("one${1:two}three${2:four}").unwrap();
        assert_eq!(snippet.text, "onetwothreefour");
        assert_eq!(
            tabstops(&snippet),
            &[vec![3..6], vec![11..15], vec![15..15]]
        );
        assert_eq!(tabstop_choices(&snippet), &[&None, &None, &None]);
    }

    #[test]
    fn test_snippet_with_choice_placeholders() {
        let snippet = Snippet::parse("type ${1|i32, u32|} = $2")
            .expect("Should be able to unpack choice placeholders");

        assert_eq!(snippet.text, "type i32 = ");
        assert_eq!(tabstops(&snippet), &[vec![5..8], vec![11..11],]);
        assert_eq!(
            tabstop_choices(&snippet),
            &[&Some(vec!["i32".to_string(), " u32".to_string()]), &None]
        );

        let snippet = Snippet::parse(r"${1|\$\{1\|one\,two\,tree\|\}|}")
            .expect("Should be able to parse choice with escape characters");

        assert_eq!(snippet.text, "${1|one,two,tree|}");
        assert_eq!(tabstops(&snippet), &[vec![0..18], vec![18..18]]);
        assert_eq!(
            tabstop_choices(&snippet),
            &[&Some(vec!["${1|one,two,tree|}".to_string(),]), &None]
        );
    }

    #[test]
    fn test_snippet_with_escaped_chars_in_non_default_choices() {
        let snippet = Snippet::parse(r"${1|a,b\,c|}").unwrap();
        assert_eq!(snippet.text, "a");
        assert_eq!(tabstops(&snippet), &[vec![0..1], vec![1..1]]);
        assert_eq!(
            tabstop_choices(&snippet),
            &[&Some(vec!["a".to_string(), "b,c".to_string()]), &None]
        );

        let snippet = Snippet::parse(r"${1|one,two\|2,three\\3|}").unwrap();
        assert_eq!(snippet.text, "one");
        assert_eq!(tabstops(&snippet), &[vec![0..3], vec![3..3]]);
        assert_eq!(
            tabstop_choices(&snippet),
            &[
                &Some(vec![
                    "one".to_string(),
                    "two|2".to_string(),
                    r"three\3".to_string()
                ]),
                &None
            ]
        );
    }

    #[test]
    fn test_snippet_with_nested_placeholders() {
        let snippet = Snippet::parse(
            "for (${1:var ${2:i} = 0; ${2:i} < ${3:${4:array}.length}; ${2:i}++}) {$0}",
        )
        .unwrap();
        assert_eq!(snippet.text, "for (var i = 0; i < array.length; i++) {}");
        assert_eq!(
            tabstops(&snippet),
            &[
                vec![5..37],
                vec![9..10, 16..17, 34..35],
                vec![20..32],
                vec![20..25],
                vec![40..40],
            ]
        );
        assert_eq!(
            tabstop_choices(&snippet),
            &[&None, &None, &None, &None, &None]
        );
    }

    #[test]
    fn test_snippet_parsing_with_escaped_chars() {
        let snippet = Snippet::parse("\"\\$schema\": $1").unwrap();
        assert_eq!(snippet.text, "\"$schema\": ");
        assert_eq!(tabstops(&snippet), &[vec![11..11]]);
        assert_eq!(tabstop_choices(&snippet), &[&None]);

        let snippet = Snippet::parse("{a\\}").unwrap();
        assert_eq!(snippet.text, "{a}");
        assert_eq!(tabstops(&snippet), &[vec![3..3]]);
        assert_eq!(tabstop_choices(&snippet), &[&None]);

        // backslash not functioning as an escape
        let snippet = Snippet::parse("a\\b").unwrap();
        assert_eq!(snippet.text, "a\\b");
        assert_eq!(tabstops(&snippet), &[vec![3..3]]);

        // first backslash cancelling escaping that would
        // have happened with second backslash
        let snippet = Snippet::parse("one\\\\$1two").unwrap();
        assert_eq!(snippet.text, "one\\two");
        assert_eq!(tabstops(&snippet), &[vec![4..4], vec![7..7]]);
    }

    fn resolve_test_variable(name: &str) -> Option<String> {
        match name {
            "TM_FILENAME" => Some("main.rs".to_string()),
            "TM_SELECTED_TEXT" => Some(String::new()),
            _ => None,
        }
    }

    fn parse_with_test_variables(source: &str) -> Snippet {
        Snippet::parse_with_variables(source, resolve_test_variable).unwrap()
    }

    #[test]
    fn test_snippet_with_variables() {
        let snippet = parse_with_test_variables("// $TM_FILENAME and ${TM_FILENAME}!");
        assert_eq!(snippet.text, "// main.rs and main.rs!");
        assert_eq!(tabstops(&snippet), &[vec![23..23]]);

        // Variables are delimited by non-identifier characters.
        let snippet = parse_with_test_variables("$TM_FILENAME.bak-$TM_FILENAME_$1");
        assert_eq!(snippet.text, "main.rs.bak-TM_FILENAME_");
        assert_eq!(tabstops(&snippet)[0], vec![24..24]);
    }

    #[test]
    fn test_snippet_with_variable_defaults() {
        // A resolved, non-empty variable ignores its default and the default's tabstops.
        let snippet = parse_with_test_variables("${TM_FILENAME:default ${1:one}}$2");
        assert_eq!(snippet.text, "main.rs");
        assert_eq!(tabstops(&snippet), &[vec![7..7]]);

        // A known but empty variable falls back to its default.
        let snippet = parse_with_test_variables("${TM_SELECTED_TEXT:fallback ${1:one}}");
        assert_eq!(snippet.text, "fallback one");
        assert_eq!(tabstops(&snippet), &[vec![9..12], vec![12..12]]);

        let snippet = parse_with_test_variables("a${TM_SELECTED_TEXT}b");
        assert_eq!(snippet.text, "ab");
    }

    #[test]
    fn test_snippet_with_unknown_variables() {
        // Unknown variables become placeholders after the explicit tabstops.
        let snippet = parse_with_test_variables("$foo ${2:bar} ${foo} ${baz:qux} $0");
        assert_eq!(snippet.text, "foo bar foo qux ");
        assert_eq!(
            tabstops(&snippet),
            &[vec![4..7], vec![0..3, 8..11], vec![12..15], vec![16..16]]
        );
    }

    #[test]
    fn test_snippet_with_builtin_variables() {
        let snippet =
            Snippet::parse("$CURRENT_YEAR-$CURRENT_MONTH $RANDOM $RANDOM_HEX $UUID").unwrap();
        let parts = snippet.text.split(' ').collect::<Vec<_>>();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].len(), 7);
        assert_eq!(parts[1].len(), 6);
        assert!(parts[1].chars().all(|c| c.is_ascii_digit()));
        assert_eq!(parts[2].len(), 6);
        assert!(parts[2].chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(parts[3].len(), 36);
        assert_eq!(snippet.tabstops.len(), 1);
    }

    #[test]
    fn test_snippet_with_unclosed_variable() {
        assert!(Snippet::parse("${TM_FILENAME").is_err());
        assert!(Snippet::parse("${TM_FILENAME:default").is_err());
    }

    fn tabstops(snippet: &Snippet) -> Vec<Vec<Range<isize>>> {
        snippet.tabstops.iter().map(|t| t.ranges.to_vec()).collect()
    }

    fn tabstop_choices(snippet: &Snippet) -> Vec<&Option<Vec<String>>> {
        snippet.tabstops.iter().map(|t| &t.choices).collect()
    }
}
