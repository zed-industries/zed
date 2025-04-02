use anyhow::{Context as _, Result, anyhow};
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

impl Snippet {
    pub fn parse(source: &str) -> Result<Self> {
        let mut text = String::with_capacity(source.len());
        let mut tabstops = BTreeMap::new();
        parse_snippet(source, false, &mut text, &mut tabstops)
            .context("failed to parse snippet")?;

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

            if !tabstops.last().map_or(false, |t| *t == end_tabstop) {
                tabstops.push(end_tabstop);
            }
        }

        Ok(Snippet { text, tabstops })
    }
}

fn parse_snippet<'a>(
    mut source: &'a str,
    nested: bool,
    text: &mut String,
    tabstops: &mut BTreeMap<usize, TabStop>,
) -> Result<&'a str> {
    loop {
        match source.chars().next() {
            None => return Ok(""),
            Some('$') => {
                source = parse_tabstop(&source[1..], text, tabstops)?;
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

fn parse_tabstop<'a>(
    mut source: &'a str,
    text: &mut String,
    tabstops: &mut BTreeMap<usize, TabStop>,
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
            source = parse_snippet(&source[1..], true, text, tabstops)?;
        }

        if source.starts_with('}') {
            source = &source[1..];
        } else {
            return Err(anyhow!("expected a closing brace"));
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
    if len == 0 {
        return Err(anyhow!("expected an integer"));
    }
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
                    if !found_default_choice {
                        current_choice.push(c);
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

                if chunk_end.is_none() {
                    return Err(anyhow!(
                        "Placeholder choice doesn't contain closing pipe-character '|'"
                    ));
                }

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

    fn tabstops(snippet: &Snippet) -> Vec<Vec<Range<isize>>> {
        snippet.tabstops.iter().map(|t| t.ranges.to_vec()).collect()
    }

    fn tabstop_choices(snippet: &Snippet) -> Vec<&Option<Vec<String>>> {
        snippet.tabstops.iter().map(|t| &t.choices).collect()
    }
}
