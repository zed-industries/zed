use language::{BufferSnapshot, SyntaxMapMatches};
use std::{cmp::Reverse, ops::Range};

use crate::declaration::Identifier;

// TODO:
//
// * how to handle multiple name captures? for now last one wins
//
// * annotation ranges
//
// * new "signature" capture for outline queries
//
// * Check parent behavior of "int x, y = 0" declarations in a test

pub struct OutlineDeclaration {
    pub parent_index: Option<usize>,
    pub identifier: Identifier,
    pub item_range: Range<usize>,
    pub signature_range: Range<usize>,
}

pub fn declarations_in_buffer(buffer: &BufferSnapshot) -> Vec<OutlineDeclaration> {
    declarations_overlapping_range(0..buffer.len(), buffer)
}

pub fn declarations_overlapping_range(
    range: Range<usize>,
    buffer: &BufferSnapshot,
) -> Vec<OutlineDeclaration> {
    let mut declarations = OutlineIterator::new(range, buffer).collect::<Vec<_>>();
    declarations.sort_unstable_by_key(|item| (item.item_range.start, Reverse(item.item_range.end)));

    let mut parent_stack: Vec<(usize, Range<usize>)> = Vec::new();
    for (index, declaration) in declarations.iter_mut().enumerate() {
        while let Some((top_parent_index, top_parent_range)) = parent_stack.last() {
            if declaration.item_range.start >= top_parent_range.end {
                parent_stack.pop();
            } else {
                declaration.parent_index = Some(*top_parent_index);
                break;
            }
        }
        parent_stack.push((index, declaration.item_range.clone()));
    }
    declarations
}

/// Iterates outline items without being ordered w.r.t. nested items and without populating
/// `parent`.
pub struct OutlineIterator<'a> {
    buffer: &'a BufferSnapshot,
    matches: SyntaxMapMatches<'a>,
}

impl<'a> OutlineIterator<'a> {
    pub fn new(range: Range<usize>, buffer: &'a BufferSnapshot) -> Self {
        let matches = buffer.syntax.matches(range, &buffer.text, |grammar| {
            grammar.outline_config.as_ref().map(|c| &c.query)
        });

        Self { buffer, matches }
    }
}

impl<'a> Iterator for OutlineIterator<'a> {
    type Item = OutlineDeclaration;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mat) = self.matches.peek() {
            let config = self.matches.grammars()[mat.grammar_index]
                .outline_config
                .as_ref()
                .unwrap();

            let mut name_range = None;
            let mut item_range = None;
            let mut signature_start = None;
            let mut signature_end = None;

            let mut add_to_signature = |range: Range<usize>| {
                if signature_start.is_none() {
                    signature_start = Some(range.start);
                }
                signature_end = Some(range.end);
            };

            for capture in mat.captures {
                let range = capture.node.byte_range();
                if capture.index == config.name_capture_ix {
                    name_range = Some(range.clone());
                    add_to_signature(range);
                } else if Some(capture.index) == config.context_capture_ix
                    || Some(capture.index) == config.extra_context_capture_ix
                {
                    add_to_signature(range);
                } else if capture.index == config.item_capture_ix {
                    item_range = Some(range.clone());
                }
            }

            let language_id = mat.language.id();
            self.matches.advance();

            if let Some(name_range) = name_range
                && let Some(item_range) = item_range
                && let Some(signature_start) = signature_start
                && let Some(signature_end) = signature_end
            {
                let name = self
                    .buffer
                    .text_for_range(name_range)
                    .collect::<String>()
                    .into();

                return Some(OutlineDeclaration {
                    identifier: Identifier { name, language_id },
                    item_range: item_range,
                    signature_range: signature_start..signature_end,
                    parent_index: None,
                });
            }
        }
        None
    }
}
