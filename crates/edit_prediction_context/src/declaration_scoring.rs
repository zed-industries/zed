use cloud_llm_client::predict_edits_v3::DeclarationScoreComponents;
use collections::HashMap;
use itertools::Itertools as _;
use language::BufferSnapshot;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{cmp::Reverse, ops::Range};
use strum::EnumIter;
use text::{Point, ToPoint};

use crate::{
    Declaration, EditPredictionExcerpt, Identifier,
    imports::Imports,
    reference::{Reference, ReferenceRegion},
    syntax_index::SyntaxIndexState,
    text_similarity::{Occurrences, jaccard_similarity, weighted_overlap_coefficient},
};

const MAX_IDENTIFIER_DECLARATION_COUNT: usize = 16;

#[derive(Clone, Debug)]
pub struct ScoredDeclaration {
    pub identifier: Identifier,
    pub declaration: Declaration,
    pub components: DeclarationScoreComponents,
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DeclarationStyle {
    Signature,
    Declaration,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct DeclarationScores {
    pub signature: f32,
    pub declaration: f32,
    pub retrieval: f32,
}

impl ScoredDeclaration {
    /// Returns the score for this declaration with the specified style.
    pub fn score(&self, style: DeclarationStyle) -> f32 {
        // TODO: handle truncation

        // Score related to how likely this is the correct declaration, range 0 to 1
        let retrieval = self.retrieval_score();

        // Score related to the distance between the reference and cursor, range 0 to 1
        let distance_score = if self.components.is_referenced_nearby {
            1.0 / (1.0 + self.components.reference_line_distance as f32 / 10.0).powf(2.0)
        } else {
            // same score as ~14 lines away, rationale is to not overly penalize references from parent signatures
            0.5
        };

        // For now instead of linear combination, the scores are just multiplied together.
        let combined_score = 10.0 * retrieval * distance_score;

        match style {
            DeclarationStyle::Signature => {
                combined_score * self.components.excerpt_vs_signature_weighted_overlap
            }
            DeclarationStyle::Declaration => {
                2.0 * combined_score * self.components.excerpt_vs_item_weighted_overlap
            }
        }
    }

    pub fn retrieval_score(&self) -> f32 {
        if self.components.is_same_file {
            // TODO: use declaration_line_distance_rank
            2.0 / self.components.same_file_declaration_count as f32
        } else if self.components.normalized_import_similarity > 0.0 {
            self.components.normalized_import_similarity / self.components.declaration_count as f32
        } else {
            0.5 * self.components.normalized_wildcard_import_similarity
                / self.components.declaration_count as f32
        }
    }

    pub fn size(&self, style: DeclarationStyle) -> usize {
        match &self.declaration {
            Declaration::File { declaration, .. } => match style {
                DeclarationStyle::Signature => declaration.signature_range.len(),
                DeclarationStyle::Declaration => declaration.text.len(),
            },
            Declaration::Buffer { declaration, .. } => match style {
                DeclarationStyle::Signature => declaration.signature_range.len(),
                DeclarationStyle::Declaration => declaration.item_range.len(),
            },
        }
    }

    pub fn score_density(&self, style: DeclarationStyle) -> f32 {
        self.score(style) / self.size(style) as f32
    }
}

pub fn scored_declarations(
    index: &SyntaxIndexState,
    excerpt: &EditPredictionExcerpt,
    excerpt_occurrences: &Occurrences,
    adjacent_occurrences: &Occurrences,
    imports: Option<&Imports>,
    identifier_to_references: HashMap<Identifier, Vec<Reference>>,
    cursor_offset: usize,
    current_buffer: &BufferSnapshot,
) -> Vec<ScoredDeclaration> {
    let cursor_point = cursor_offset.to_point(&current_buffer);

    let wildcard_imports_ocurrences = imports
        .iter()
        .flat_map(|imports| {
            imports
                .wildcard_namespaces
                .iter()
                .map(|namespace| Occurrences::from_identifiers(&namespace.0))
        })
        .collect::<Vec<_>>();

    let mut declarations = identifier_to_references
        .into_iter()
        .flat_map(|(identifier, references)| {
            let declarations =
                index.declarations_for_identifier::<MAX_IDENTIFIER_DECLARATION_COUNT>(&identifier);
            let declaration_count = declarations.len();
            let import_namespace_occurrences = imports
                .map(|imports| imports.identifier_namespaces.get(&identifier))
                .unwrap_or_default()
                .into_iter()
                .flat_map(|import_namespaces| {
                    import_namespaces
                        .iter()
                        .map(|namespace| Occurrences::from_identifiers(&namespace.0))
                })
                .collect::<Vec<_>>();

            let mut max_import_similarity = 0.0;
            let mut max_wildcard_import_similarity = 0.0;

            let mut scored_declarations_for_identifier = declarations
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

                        let declaration = score_declaration(
                            &identifier,
                            &references,
                            declaration.clone(),
                            is_same_file,
                            declaration_line_distance,
                            declaration_line_distance_rank,
                            same_file_declaration_count,
                            declaration_count,
                            &excerpt_occurrences,
                            &adjacent_occurrences,
                            &import_namespace_occurrences,
                            &wildcard_imports_ocurrences,
                            cursor_point,
                            current_buffer,
                        );

                        if declaration.components.import_similarity > max_import_similarity {
                            max_import_similarity = declaration.components.import_similarity;
                        }

                        if declaration.components.wildcard_import_similarity
                            > max_wildcard_import_similarity
                        {
                            max_wildcard_import_similarity =
                                declaration.components.wildcard_import_similarity;
                        }

                        declaration
                    },
                )
                .collect::<Vec<_>>();

            if max_import_similarity > 0.0 || max_wildcard_import_similarity > 0.0 {
                for declaration in scored_declarations_for_identifier.iter_mut() {
                    if max_import_similarity > 0.0 {
                        declaration.components.normalized_import_similarity =
                            declaration.components.import_similarity / max_import_similarity;
                    }
                    if max_wildcard_import_similarity > 0.0 {
                        declaration.components.normalized_wildcard_import_similarity =
                            declaration.components.wildcard_import_similarity
                                / max_wildcard_import_similarity;
                    }
                }
            }

            scored_declarations_for_identifier
        })
        .collect::<Vec<_>>();

    declarations.sort_unstable_by_key(|declaration| {
        let score_density = declaration
            .score_density(DeclarationStyle::Declaration)
            .max(declaration.score_density(DeclarationStyle::Signature));
        Reverse(OrderedFloat(score_density))
    });

    declarations
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

fn score_declaration(
    identifier: &Identifier,
    references: &[Reference],
    declaration: Declaration,
    is_same_file: bool,
    declaration_line_distance: u32,
    declaration_line_distance_rank: usize,
    same_file_declaration_count: usize,
    declaration_count: usize,
    excerpt_occurrences: &Occurrences,
    adjacent_occurrences: &Occurrences,
    import_namespace_occurrences: &[Occurrences],
    wildcard_imports_occurrences: &[Occurrences],
    cursor: Point,
    current_buffer: &BufferSnapshot,
) -> ScoredDeclaration {
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

    let item_source_occurrences = Occurrences::within_string(&declaration.item_text().0);
    let item_signature_occurrences = Occurrences::within_string(&declaration.signature_text().0);
    let excerpt_vs_item_jaccard = jaccard_similarity(excerpt_occurrences, &item_source_occurrences);
    let excerpt_vs_signature_jaccard =
        jaccard_similarity(excerpt_occurrences, &item_signature_occurrences);
    let adjacent_vs_item_jaccard =
        jaccard_similarity(adjacent_occurrences, &item_source_occurrences);
    let adjacent_vs_signature_jaccard =
        jaccard_similarity(adjacent_occurrences, &item_signature_occurrences);

    let excerpt_vs_item_weighted_overlap =
        weighted_overlap_coefficient(excerpt_occurrences, &item_source_occurrences);
    let excerpt_vs_signature_weighted_overlap =
        weighted_overlap_coefficient(excerpt_occurrences, &item_signature_occurrences);
    let adjacent_vs_item_weighted_overlap =
        weighted_overlap_coefficient(adjacent_occurrences, &item_source_occurrences);
    let adjacent_vs_signature_weighted_overlap =
        weighted_overlap_coefficient(adjacent_occurrences, &item_signature_occurrences);

    // TODO: Consider directly caching this instead
    //
    // TODO: Handle special cases like lib.rs as the last component
    let declaration_path = declaration.cached_full_path();
    let last_component = declaration_path
        .file_stem()
        .map(|stem| stem.to_string_lossy());
    let mut path_components = declaration_path.components();
    path_components.next_back();
    let path_occurrences = Occurrences::from_identifiers(
        path_components
            .map(|component| component.as_os_str().to_string_lossy())
            .chain(last_component),
    );

    let import_similarity = import_namespace_occurrences
        .iter()
        .map(|namespace_occurrences| {
            OrderedFloat(jaccard_similarity(namespace_occurrences, &path_occurrences))
        })
        .max()
        .map(|similarity| similarity.into_inner())
        .unwrap_or_default();

    // TODO: Consider skipping if import_similarity is high
    //
    // TODO: Consider something other than max
    let wildcard_import_similarity = wildcard_imports_occurrences
        .iter()
        .map(|namespace_occurrences| {
            OrderedFloat(jaccard_similarity(namespace_occurrences, &path_occurrences))
        })
        .max()
        .map(|similarity| similarity.into_inner())
        .unwrap_or_default();

    // TODO: Consider adding declaration_file_count
    let score_components = DeclarationScoreComponents {
        is_same_file,
        is_referenced_nearby,
        is_referenced_in_breadcrumb,
        reference_line_distance,
        declaration_line_distance,
        declaration_line_distance_rank,
        reference_count,
        same_file_declaration_count,
        declaration_count,
        excerpt_vs_item_jaccard,
        excerpt_vs_signature_jaccard,
        adjacent_vs_item_jaccard,
        adjacent_vs_signature_jaccard,
        excerpt_vs_item_weighted_overlap,
        excerpt_vs_signature_weighted_overlap,
        adjacent_vs_item_weighted_overlap,
        adjacent_vs_signature_weighted_overlap,
        import_similarity,
        normalized_import_similarity: 0.0,
        wildcard_import_similarity,
        normalized_wildcard_import_similarity: 0.0,
    };

    ScoredDeclaration {
        identifier: identifier.clone(),
        declaration: declaration,
        components: score_components,
    }
}
