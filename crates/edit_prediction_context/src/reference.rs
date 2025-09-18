use language::BufferSnapshot;
use std::collections::HashMap;
use std::ops::Range;

use crate::{
    excerpt::{EditPredictionExcerpt, EditPredictionExcerptText},
    outline::Identifier,
};

#[derive(Debug)]
pub struct Reference {
    pub identifier: Identifier,
    pub range: Range<usize>,
    pub region: ReferenceRegion,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReferenceRegion {
    Breadcrumb,
    Nearby,
}

pub fn references_in_excerpt(
    excerpt: &EditPredictionExcerpt,
    excerpt_text: &EditPredictionExcerptText,
    snapshot: &BufferSnapshot,
) -> HashMap<Identifier, Vec<Reference>> {
    let mut references = identifiers_in_range(
        excerpt.range.clone(),
        excerpt_text.body.as_str(),
        ReferenceRegion::Nearby,
        snapshot,
    );

    for (range, text) in excerpt
        .parent_signature_ranges
        .iter()
        .zip(excerpt_text.parent_signatures.iter())
    {
        references.extend(identifiers_in_range(
            range.clone(),
            text.as_str(),
            ReferenceRegion::Breadcrumb,
            snapshot,
        ));
    }

    let mut identifier_to_references: HashMap<Identifier, Vec<Reference>> = HashMap::new();
    for reference in references {
        identifier_to_references
            .entry(reference.identifier.clone())
            .or_insert_with(Vec::new)
            .push(reference);
    }
    identifier_to_references
}

/// Finds all nodes which have a "variable" match from the highlights query within the offset range.
pub fn identifiers_in_range(
    range: Range<usize>,
    range_text: &str,
    reference_region: ReferenceRegion,
    buffer: &BufferSnapshot,
) -> Vec<Reference> {
    let mut matches = buffer
        .syntax
        .matches(range.clone(), &buffer.text, |grammar| {
            grammar
                .highlights_config
                .as_ref()
                .map(|config| &config.query)
        });

    let mut references = Vec::new();
    let mut last_added_range = None;
    while let Some(mat) = matches.peek() {
        let config = matches.grammars()[mat.grammar_index]
            .highlights_config
            .as_ref();

        for capture in mat.captures {
            if let Some(config) = config {
                if config.identifier_capture_indices.contains(&capture.index) {
                    let node_range = capture.node.byte_range();

                    // sometimes multiple highlight queries match - this deduplicates them
                    if Some(node_range.clone()) == last_added_range {
                        continue;
                    }

                    let identifier_text =
                        &range_text[node_range.start - range.start..node_range.end - range.start];
                    references.push(Reference {
                        identifier: Identifier {
                            name: identifier_text.into(),
                            language_id: mat.language.id(),
                        },
                        range: node_range.clone(),
                        region: reference_region,
                    });
                    last_added_range = Some(node_range);
                }
            }
        }

        matches.advance();
    }
    references
}
