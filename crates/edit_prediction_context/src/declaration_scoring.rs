use cloud_llm_client::predict_edits_v3::ScoreComponents;
use itertools::Itertools as _;
use language::BufferSnapshot;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{cmp::Reverse, collections::HashMap, ops::Range};
use strum::EnumIter;
use text::{Point, ToPoint};

use crate::{
    Declaration, EditPredictionExcerpt, EditPredictionExcerptText, Identifier,
    reference::{Reference, ReferenceRegion},
    syntax_index::SyntaxIndexState,
    text_similarity::{IdentifierOccurrences, jaccard_similarity, weighted_overlap_coefficient},
};

const MAX_IDENTIFIER_DECLARATION_COUNT: usize = 16;

#[derive(Clone, Debug)]
pub struct ScoredSnippet {
    pub identifier: Identifier,
    pub declaration: Declaration,
    pub score_components: ScoreComponents,
    pub scores: Scores,
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SnippetStyle {
    Signature,
    Declaration,
}

impl ScoredSnippet {
    /// Returns the score for this snippet with the specified style.
    pub fn score(&self, style: SnippetStyle) -> f32 {
        match style {
            SnippetStyle::Signature => self.scores.signature,
            SnippetStyle::Declaration => self.scores.declaration,
        }
    }

    pub fn size(&self, style: SnippetStyle) -> usize {
        match &self.declaration {
            Declaration::File { declaration, .. } => match style {
                SnippetStyle::Signature => declaration.signature_range.len(),
                SnippetStyle::Declaration => declaration.text.len(),
            },
            Declaration::Buffer { declaration, .. } => match style {
                SnippetStyle::Signature => declaration.signature_range.len(),
                SnippetStyle::Declaration => declaration.item_range.len(),
            },
        }
    }

    pub fn score_density(&self, style: SnippetStyle) -> f32 {
        self.score(style) / (self.size(style)) as f32
    }
}

pub fn scored_snippets(
    index: &SyntaxIndexState,
    excerpt: &EditPredictionExcerpt,
    excerpt_text: &EditPredictionExcerptText,
    identifier_to_references: HashMap<Identifier, Vec<Reference>>,
    cursor_offset: usize,
    current_buffer: &BufferSnapshot,
) -> Vec<ScoredSnippet> {
    let containing_range_identifier_occurrences =
        IdentifierOccurrences::within_string(&excerpt_text.body);
    let cursor_point = cursor_offset.to_point(&current_buffer);

    let start_point = Point::new(cursor_point.row.saturating_sub(2), 0);
    let end_point = Point::new(cursor_point.row + 1, 0);
    let adjacent_identifier_occurrences = IdentifierOccurrences::within_string(
        &current_buffer
            .text_for_range(start_point..end_point)
            .collect::<String>(),
    );

    let mut snippets = identifier_to_references
        .into_iter()
        .flat_map(|(identifier, references)| {
            let declarations =
                index.declarations_for_identifier::<MAX_IDENTIFIER_DECLARATION_COUNT>(&identifier);
            let declaration_count = declarations.len();

            declarations
                .into_iter()
                .filter_map(|(declaration_id, declaration)| match declaration {
                    Declaration::Buffer {
                        buffer_id,
                        declaration: buffer_declaration,
                        ..
                    } => {
                        let is_same_file = buffer_id == &current_buffer.remote_id();

                        if is_same_file {
                            let overlaps_excerpt =
                                range_intersection(&buffer_declaration.item_range, &excerpt.range)
                                    .is_some();
                            if overlaps_excerpt
                                || excerpt
                                    .parent_declarations
                                    .iter()
                                    .any(|(excerpt_parent, _)| excerpt_parent == &declaration_id)
                            {
                                None
                            } else {
                                let declaration_line = buffer_declaration
                                    .item_range
                                    .start
                                    .to_point(current_buffer)
                                    .row;
                                Some((
                                    true,
                                    (cursor_point.row as i32 - declaration_line as i32)
                                        .unsigned_abs(),
                                    declaration,
                                ))
                            }
                        } else {
                            Some((false, u32::MAX, declaration))
                        }
                    }
                    Declaration::File { .. } => {
                        // We can assume that a file declaration is in a different file,
                        // because the current one must be open
                        Some((false, u32::MAX, declaration))
                    }
                })
                .sorted_by_key(|&(_, distance, _)| distance)
                .enumerate()
                .map(
                    |(
                        declaration_line_distance_rank,
                        (is_same_file, declaration_line_distance, declaration),
                    )| {
                        let same_file_declaration_count = index.file_declaration_count(declaration);

                        score_snippet(
                            &identifier,
                            &references,
                            declaration.clone(),
                            is_same_file,
                            declaration_line_distance,
                            declaration_line_distance_rank,
                            same_file_declaration_count,
                            declaration_count,
                            &containing_range_identifier_occurrences,
                            &adjacent_identifier_occurrences,
                            cursor_point,
                            current_buffer,
                        )
                    },
                )
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    snippets.sort_unstable_by_key(|snippet| {
        let score_density = snippet
            .score_density(SnippetStyle::Declaration)
            .max(snippet.score_density(SnippetStyle::Signature));
        Reverse(OrderedFloat(score_density))
    });

    snippets
}

fn range_intersection<T: Ord + Clone>(a: &Range<T>, b: &Range<T>) -> Option<Range<T>> {
    let start = a.start.clone().max(b.start.clone());
    let end = a.end.clone().min(b.end.clone());
    if start < end {
        Some(Range { start, end })
    } else {
        None
    }
}

fn score_snippet(
    identifier: &Identifier,
    references: &[Reference],
    declaration: Declaration,
    is_same_file: bool,
    declaration_line_distance: u32,
    declaration_line_distance_rank: usize,
    same_file_declaration_count: usize,
    declaration_count: usize,
    containing_range_identifier_occurrences: &IdentifierOccurrences,
    adjacent_identifier_occurrences: &IdentifierOccurrences,
    cursor: Point,
    current_buffer: &BufferSnapshot,
) -> Option<ScoredSnippet> {
    let is_referenced_nearby = references
        .iter()
        .any(|r| r.region == ReferenceRegion::Nearby);
    let is_referenced_in_breadcrumb = references
        .iter()
        .any(|r| r.region == ReferenceRegion::Breadcrumb);
    let reference_count = references.len();
    let reference_line_distance = references
        .iter()
        .map(|r| {
            let reference_line = r.range.start.to_point(current_buffer).row as i32;
            (cursor.row as i32 - reference_line).unsigned_abs()
        })
        .min()
        .unwrap();

    let item_source_occurrences = IdentifierOccurrences::within_string(&declaration.item_text().0);
    let item_signature_occurrences =
        IdentifierOccurrences::within_string(&declaration.signature_text().0);
    let containing_range_vs_item_jaccard = jaccard_similarity(
        containing_range_identifier_occurrences,
        &item_source_occurrences,
    );
    let containing_range_vs_signature_jaccard = jaccard_similarity(
        containing_range_identifier_occurrences,
        &item_signature_occurrences,
    );
    let adjacent_vs_item_jaccard =
        jaccard_similarity(adjacent_identifier_occurrences, &item_source_occurrences);
    let adjacent_vs_signature_jaccard =
        jaccard_similarity(adjacent_identifier_occurrences, &item_signature_occurrences);

    let containing_range_vs_item_weighted_overlap = weighted_overlap_coefficient(
        containing_range_identifier_occurrences,
        &item_source_occurrences,
    );
    let containing_range_vs_signature_weighted_overlap = weighted_overlap_coefficient(
        containing_range_identifier_occurrences,
        &item_signature_occurrences,
    );
    let adjacent_vs_item_weighted_overlap =
        weighted_overlap_coefficient(adjacent_identifier_occurrences, &item_source_occurrences);
    let adjacent_vs_signature_weighted_overlap =
        weighted_overlap_coefficient(adjacent_identifier_occurrences, &item_signature_occurrences);

    // TODO: Consider adding declaration_file_count
    let score_components = ScoreComponents {
        is_same_file,
        is_referenced_nearby,
        is_referenced_in_breadcrumb,
        reference_line_distance,
        declaration_line_distance,
        declaration_line_distance_rank,
        reference_count,
        same_file_declaration_count,
        declaration_count,
        containing_range_vs_item_jaccard,
        containing_range_vs_signature_jaccard,
        adjacent_vs_item_jaccard,
        adjacent_vs_signature_jaccard,
        containing_range_vs_item_weighted_overlap,
        containing_range_vs_signature_weighted_overlap,
        adjacent_vs_item_weighted_overlap,
        adjacent_vs_signature_weighted_overlap,
    };

    Some(ScoredSnippet {
        identifier: identifier.clone(),
        declaration: declaration,
        scores: Scores::score(&score_components),
        score_components,
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct Scores {
    pub signature: f32,
    pub declaration: f32,
}

impl Scores {
    fn score(components: &ScoreComponents) -> Scores {
        // TODO: handle truncation

        // Score related to how likely this is the correct declaration, range 0 to 1
        let accuracy_score = if components.is_same_file {
            // TODO: use declaration_line_distance_rank
            1.0 / components.same_file_declaration_count as f32
        } else {
            1.0 / components.declaration_count as f32
        };

        // Score related to the distance between the reference and cursor, range 0 to 1
        let distance_score = if components.is_referenced_nearby {
            1.0 / (1.0 + components.reference_line_distance as f32 / 10.0).powf(2.0)
        } else {
            // same score as ~14 lines away, rationale is to not overly penalize references from parent signatures
            0.5
        };

        // For now instead of linear combination, the scores are just multiplied together.
        let combined_score = 10.0 * accuracy_score * distance_score;

        Scores {
            signature: combined_score * components.containing_range_vs_signature_weighted_overlap,
            // declaration score gets boosted both by being multiplied by 2 and by there being more
            // weighted overlap.
            declaration: 2.0
                * combined_score
                * components.containing_range_vs_item_weighted_overlap,
        }
    }
}
