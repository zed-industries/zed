use anyhow::{anyhow, Context, Result};
use smallvec::SmallVec;
use std::{collections::BTreeMap, ops::Range};

#[derive(Default)]
pub struct Snippet {
    pub text: String,
    pub tabstops: Vec<TabStop>,
}

type TabStop = SmallVec<[Range<usize>; 2]>;

impl Snippet {
    pub fn parse(source: &str) -> Result<Self> {
        let mut text = String::new();
        let mut tabstops = BTreeMap::new();
        parse_snippet(source, false, &mut text, &mut tabstops)
            .context("failed to parse snippet")?;
        Ok(Snippet {
            text,
            tabstops: tabstops.into_values().collect(),
        })
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
            Some('}') => {
                if nested {
                    return Ok(source);
                } else {
                    text.push('}');
                    source = &source[1..];
                }
            }
            Some(_) => {
                let chunk_end = source.find(&['}', '$']).unwrap_or(source.len());
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
    if source.chars().next() == Some('{') {
        let (index, rest) = parse_int(&source[1..])?;
        tabstop_index = index;
        source = rest;

        if source.chars().next() == Some(':') {
            source = parse_snippet(&source[1..], true, text, tabstops)?;
        }

        if source.chars().next() == Some('}') {
            source = &source[1..];
        } else {
            return Err(anyhow!("expected a closing brace"));
        }
    } else {
        let (index, rest) = parse_int(&source)?;
        tabstop_index = index;
        source = rest;
    }

    tabstops
        .entry(tabstop_index)
        .or_default()
        .push(tabstop_start..text.len());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snippet_with_tabstops() {
        let snippet = Snippet::parse("one$1two").unwrap();
        assert_eq!(snippet.text, "onetwo");
        assert_eq!(
            snippet
                .tabstops
                .iter()
                .map(SmallVec::as_slice)
                .collect::<Vec<_>>(),
            &[vec![3..3]]
        );

        // Multi-digit numbers
        let snippet = Snippet::parse("one$123 $99").unwrap();
        assert_eq!(snippet.text, "one ");
        assert_eq!(
            snippet
                .tabstops
                .iter()
                .map(SmallVec::as_slice)
                .collect::<Vec<_>>(),
            &[vec![4..4], vec![3..3]]
        );
    }

    #[test]
    fn test_parse_snippet_with_placeholders() {
        let snippet = Snippet::parse("one${1:two}three${2:four}").unwrap();
        assert_eq!(snippet.text, "onetwothreefour");
        assert_eq!(
            snippet
                .tabstops
                .iter()
                .map(SmallVec::as_slice)
                .collect::<Vec<_>>(),
            &[vec![3..6], vec![11..15]]
        );
    }

    #[test]
    fn test_parse_snippet_with_nested_placeholders() {
        let snippet = Snippet::parse(
            "for (${1:var ${2:i} = 0; ${2:i} < ${3:${4:array}.length}; ${2:i}++}) {$5}",
        )
        .unwrap();
        assert_eq!(snippet.text, "for (var i = 0; i < array.length; i++) {}");
        assert_eq!(
            snippet
                .tabstops
                .iter()
                .map(SmallVec::as_slice)
                .collect::<Vec<_>>(),
            &[
                vec![5..37],
                vec![9..10, 16..17, 34..35],
                vec![20..32],
                vec![20..25],
                vec![40..40],
            ]
        );
    }
}
