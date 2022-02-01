use anyhow::{anyhow, Result};
use smallvec::SmallVec;
use std::{collections::BTreeMap, ops::Range};
use tree_sitter::{Parser, TreeCursor};

#[derive(Default)]
pub struct Snippet {
    pub text: String,
    pub tabstops: Vec<SmallVec<[Range<usize>; 2]>>,
}

impl Snippet {
    pub fn parse(source: &str) -> Result<Self> {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_snippet::language())
            .unwrap();

        let tree = parser.parse(source, None).unwrap();
        if tree.root_node().has_error() {
            return Err(anyhow!("invalid snippet"));
        }

        let mut text = String::new();
        let mut tabstops = BTreeMap::new();
        let mut cursor = tree.root_node().walk();
        parse_snippet_node(&mut cursor, &mut text, &mut tabstops, source)?;

        Ok(Snippet {
            text,
            tabstops: tabstops.into_values().collect(),
        })
    }
}

fn parse_snippet_node(
    cursor: &mut TreeCursor,
    text: &mut String,
    tabstops: &mut BTreeMap<usize, SmallVec<[Range<usize>; 2]>>,
    source: &str,
) -> Result<()> {
    cursor.goto_first_child();
    loop {
        let node = cursor.node();
        match node.kind() {
            "text" => text.push_str(&source[node.byte_range()]),
            "tabstop" => {
                if let Some(int_node) = node.named_child(0) {
                    let index = source[int_node.byte_range()].parse::<usize>()?;
                    tabstops
                        .entry(index)
                        .or_insert(SmallVec::new())
                        .push(text.len()..text.len());
                }
            }
            "placeholder" => {
                cursor.goto_first_child();
                cursor.goto_next_sibling();
                let int_node = cursor.node();
                let index = source[int_node.byte_range()].parse::<usize>()?;

                cursor.goto_next_sibling();
                cursor.goto_next_sibling();
                let range_start = text.len();

                parse_snippet_node(cursor, text, tabstops, source)?;
                tabstops
                    .entry(index)
                    .or_insert(SmallVec::new())
                    .push(range_start..text.len());

                cursor.goto_parent();
            }
            _ => {}
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
    cursor.goto_parent();
    Ok(())
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
    }

    #[test]
    fn test_parse_snippet_with_placeholders() {
        let snippet = Snippet::parse("one${1:two}three").unwrap();
        assert_eq!(snippet.text, "onetwothree");
        assert_eq!(
            snippet
                .tabstops
                .iter()
                .map(SmallVec::as_slice)
                .collect::<Vec<_>>(),
            &[vec![3..6]]
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
