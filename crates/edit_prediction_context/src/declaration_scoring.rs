use cloud_llm_client::predict_edits_v3::DeclarationScoreComponents;
use collections::HashMap;
use language::BufferSnapshot;
use ordered_float::OrderedFloat;
use project::ProjectEntryId;
use serde::Serialize;
use std::{cmp::Reverse, ops::Range, path::Path, sync::Arc};
use strum::EnumIter;
use text::{Point, ToPoint};
use util::RangeExt as _;

use crate::{
    CachedDeclarationPath, Declaration, EditPredictionExcerpt, Identifier,
    imports::{Import, Imports, Module},
    reference::{Reference, ReferenceRegion},
    syntax_index::SyntaxIndexState,
    text_similarity::{Occurrences, jaccard_similarity, weighted_overlap_coefficient},
};

const MAX_IDENTIFIER_DECLARATION_COUNT: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditPredictionScoreOptions {
    pub omit_excerpt_overlaps: bool,
}

#[derive(Clone, Debug)]
pub struct ScoredDeclaration {
    /// identifier used by the local reference
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
        let mut score = if self.components.is_same_file {
            10.0 / self.components.same_file_declaration_count as f32
        } else if self.components.path_import_match_count > 0 {
            3.0
        } else if self.components.wildcard_path_import_match_count > 0 {
            1.0
        } else if self.components.normalized_import_similarity > 0.0 {
            self.components.normalized_import_similarity
        } else if self.components.normalized_wildcard_import_similarity > 0.0 {
            0.5 * self.components.normalized_wildcard_import_similarity
        } else {
            1.0 / self.components.declaration_count as f32
        };
        score *= 1. + self.components.included_by_others as f32 / 2.;
        score *= 1. + self.components.includes_others as f32 / 4.;
        score
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
    options: &EditPredictionScoreOptions,
    index: &SyntaxIndexState,
    excerpt: &EditPredictionExcerpt,
    excerpt_occurrences: &Occurrences,
    adjacent_occurrences: &Occurrences,
    imports: &Imports,
    identifier_to_references: HashMap<Identifier, Vec<Reference>>,
    cursor_offset: usize,
    current_buffer: &BufferSnapshot,
) -> Vec<ScoredDeclaration> {
    let cursor_point = cursor_offset.to_point(&current_buffer);

    let mut wildcard_import_occurrences = Vec::new();
    let mut wildcard_import_paths = Vec::new();
    for wildcard_import in imports.wildcard_modules.iter() {
        match wildcard_import {
            Module::Namespace(namespace) => {
                wildcard_import_occurrences.push(namespace.occurrences())
            }
            Module::SourceExact(path) => wildcard_import_paths.push(path),
            Module::SourceFuzzy(path) => {
                wildcard_import_occurrences.push(Occurrences::from_path(&path))
            }
        }
    }

    let mut scored_declarations = Vec::new();
    let mut project_entry_id_to_outline_ranges: HashMap<ProjectEntryId, Vec<Range<usize>>> =
        HashMap::default();
    for (identifier, references) in identifier_to_references {
        let mut import_occurrences = Vec::new();
        let mut import_paths = Vec::new();
        let mut found_external_identifier: Option<&Identifier> = None;

        if let Some(imports) = imports.identifier_to_imports.get(&identifier) {
            // only use alias when it's the only import, could be generalized if some language
            // has overlapping aliases
            //
            // TODO: when an aliased declaration is included in the prompt, should include the
            // aliasing in the prompt.
            //
            // TODO: For SourceFuzzy consider having componentwise comparison that pays
            // attention to ordering.
            if let [
                Import::Alias {
                    module,
                    external_identifier,
                },
            ] = imports.as_slice()
            {
                match module {
                    Module::Namespace(namespace) => {
                        import_occurrences.push(namespace.occurrences())
                    }
                    Module::SourceExact(path) => import_paths.push(path),
                    Module::SourceFuzzy(path) => {
                        import_occurrences.push(Occurrences::from_path(&path))
                    }
                }
                found_external_identifier = Some(&external_identifier);
            } else {
                for import in imports {
                    match import {
                        Import::Direct { module } => match module {
                            Module::Namespace(namespace) => {
                                import_occurrences.push(namespace.occurrences())
                            }
                            Module::SourceExact(path) => import_paths.push(path),
                            Module::SourceFuzzy(path) => {
                                import_occurrences.push(Occurrences::from_path(&path))
                            }
                        },
                        Import::Alias { .. } => {}
                    }
                }
            }
        }

        let identifier_to_lookup = found_external_identifier.unwrap_or(&identifier);
        // TODO: update this to be able to return more declarations? Especially if there is the
        // ability to quickly filter a large list (based on imports)
        let identifier_declarations = index
            .declarations_for_identifier::<MAX_IDENTIFIER_DECLARATION_COUNT>(&identifier_to_lookup);
        let declaration_count = identifier_declarations.len();

        if declaration_count == 0 {
            continue;
        }

        // TODO: option to filter out other candidates when same file / import match
        let mut checked_declarations = Vec::with_capacity(declaration_count);
        for (declaration_id, declaration) in identifier_declarations {
            match declaration {
                Declaration::Buffer {
                    buffer_id,
                    declaration: buffer_declaration,
                    ..
                } => {
                    if buffer_id == &current_buffer.remote_id() {
                        let already_included_in_prompt =
                            range_intersection(&buffer_declaration.item_range, &excerpt.range)
                                .is_some()
                                || excerpt
                                    .parent_declarations
                                    .iter()
                                    .any(|(excerpt_parent, _)| excerpt_parent == &declaration_id);
                        if !options.omit_excerpt_overlaps || !already_included_in_prompt {
                            let declaration_line = buffer_declaration
                                .item_range
                                .start
                                .to_point(current_buffer)
                                .row;
                            let declaration_line_distance =
                                (cursor_point.row as i32 - declaration_line as i32).unsigned_abs();
                            checked_declarations.push(CheckedDeclaration {
                                declaration,
                                same_file_line_distance: Some(declaration_line_distance),
                                path_import_match_count: 0,
                                wildcard_path_import_match_count: 0,
                            });
                        }
                        continue;
                    } else {
                    }
                }
                Declaration::File { .. } => {}
            }
            let declaration_path = declaration.cached_path();
            let path_import_match_count = import_paths
                .iter()
                .filter(|import_path| {
                    declaration_path_matches_import(&declaration_path, import_path)
                })
                .count();
            let wildcard_path_import_match_count = wildcard_import_paths
                .iter()
                .filter(|import_path| {
                    declaration_path_matches_import(&declaration_path, import_path)
                })
                .count();
            checked_declarations.push(CheckedDeclaration {
                declaration,
                same_file_line_distance: None,
                path_import_match_count,
                wildcard_path_import_match_count,
            });
        }

        let mut max_import_similarity = 0.0;
        let mut max_wildcard_import_similarity = 0.0;

        let mut scored_declarations_for_identifier = Vec::with_capacity(checked_declarations.len());
        for checked_declaration in checked_declarations {
            let same_file_declaration_count =
                index.file_declaration_count(checked_declaration.declaration);

            let declaration = score_declaration(
                &identifier,
                &references,
                checked_declaration,
                same_file_declaration_count,
                declaration_count,
                &excerpt_occurrences,
                &adjacent_occurrences,
                &import_occurrences,
                &wildcard_import_occurrences,
                cursor_point,
                current_buffer,
            );

            if declaration.components.import_similarity > max_import_similarity {
                max_import_similarity = declaration.components.import_similarity;
            }

            if declaration.components.wildcard_import_similarity > max_wildcard_import_similarity {
                max_wildcard_import_similarity = declaration.components.wildcard_import_similarity;
            }

            project_entry_id_to_outline_ranges
                .entry(declaration.declaration.project_entry_id())
                .or_default()
                .push(declaration.declaration.item_range());
            scored_declarations_for_identifier.push(declaration);
        }

        if max_import_similarity > 0.0 || max_wildcard_import_similarity > 0.0 {
            for declaration in scored_declarations_for_identifier.iter_mut() {
                if max_import_similarity > 0.0 {
                    declaration.components.max_import_similarity = max_import_similarity;
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

        scored_declarations.extend(scored_declarations_for_identifier);
    }

    // TODO: Inform this via import / retrieval scores of outline items
    // TODO: Consider using a sweepline
    for scored_declaration in scored_declarations.iter_mut() {
        let project_entry_id = scored_declaration.declaration.project_entry_id();
        let Some(ranges) = project_entry_id_to_outline_ranges.get(&project_entry_id) else {
            continue;
        };
        for range in ranges {
            if range.contains_inclusive(&scored_declaration.declaration.item_range()) {
                scored_declaration.components.included_by_others += 1
            } else if scored_declaration
                .declaration
                .item_range()
                .contains_inclusive(range)
            {
                scored_declaration.components.includes_others += 1
            }
        }
    }

    scored_declarations.sort_unstable_by_key(|declaration| {
        Reverse(OrderedFloat(
            declaration.score(DeclarationStyle::Declaration),
        ))
    });

    scored_declarations
}

struct CheckedDeclaration<'a> {
    declaration: &'a Declaration,
    same_file_line_distance: Option<u32>,
    path_import_match_count: usize,
    wildcard_path_import_match_count: usize,
}

fn declaration_path_matches_import(
    declaration_path: &CachedDeclarationPath,
    import_path: &Arc<Path>,
) -> bool {
    if import_path.is_absolute() {
        declaration_path.equals_absolute_path(import_path)
    } else {
        declaration_path.ends_with_posix_path(import_path)
    }
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
    checked_declaration: CheckedDeclaration,
    same_file_declaration_count: usize,
    declaration_count: usize,
    excerpt_occurrences: &Occurrences,
    adjacent_occurrences: &Occurrences,
    import_occurrences: &[Occurrences],
    wildcard_import_occurrences: &[Occurrences],
    cursor: Point,
    current_buffer: &BufferSnapshot,
) -> ScoredDeclaration {
    let CheckedDeclaration {
        declaration,
        same_file_line_distance,
        path_import_match_count,
        wildcard_path_import_match_count,
    } = checked_declaration;

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

    let is_same_file = same_file_line_distance.is_some();
    let declaration_line_distance = same_file_line_distance.unwrap_or(u32::MAX);

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

    let mut import_similarity = 0f32;
    let mut wildcard_import_similarity = 0f32;
    if !import_occurrences.is_empty() || !wildcard_import_occurrences.is_empty() {
        let cached_path = declaration.cached_path();
        let path_occurrences = Occurrences::from_worktree_path(
            cached_path
                .worktree_abs_path
                .file_name()
                .map(|f| f.to_string_lossy()),
            &cached_path.rel_path,
        );
        import_similarity = import_occurrences
            .iter()
            .map(|namespace_occurrences| {
                OrderedFloat(jaccard_similarity(namespace_occurrences, &path_occurrences))
            })
            .max()
            .map(|similarity| similarity.into_inner())
            .unwrap_or_default();

        // TODO: Consider something other than max
        wildcard_import_similarity = wildcard_import_occurrences
            .iter()
            .map(|namespace_occurrences| {
                OrderedFloat(jaccard_similarity(namespace_occurrences, &path_occurrences))
            })
            .max()
            .map(|similarity| similarity.into_inner())
            .unwrap_or_default();
    }

    // TODO: Consider adding declaration_file_count
    let score_components = DeclarationScoreComponents {
        is_same_file,
        is_referenced_nearby,
        is_referenced_in_breadcrumb,
        reference_line_distance,
        declaration_line_distance,
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
        path_import_match_count,
        wildcard_path_import_match_count,
        import_similarity,
        max_import_similarity: 0.0,
        normalized_import_similarity: 0.0,
        wildcard_import_similarity,
        normalized_wildcard_import_similarity: 0.0,
        included_by_others: 0,
        includes_others: 0,
    };

    ScoredDeclaration {
        identifier: identifier.clone(),
        declaration: declaration.clone(),
        components: score_components,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_declaration_path_matches() {
        let declaration_path =
            CachedDeclarationPath::new_for_test("/home/user/project", "src/maths.ts");

        assert!(declaration_path_matches_import(
            &declaration_path,
            &Path::new("maths.ts").into()
        ));

        assert!(declaration_path_matches_import(
            &declaration_path,
            &Path::new("project/src/maths.ts").into()
        ));

        assert!(declaration_path_matches_import(
            &declaration_path,
            &Path::new("user/project/src/maths.ts").into()
        ));

        assert!(declaration_path_matches_import(
            &declaration_path,
            &Path::new("/home/user/project/src/maths.ts").into()
        ));

        assert!(!declaration_path_matches_import(
            &declaration_path,
            &Path::new("other.ts").into()
        ));

        assert!(!declaration_path_matches_import(
            &declaration_path,
            &Path::new("/home/user/project/src/other.ts").into()
        ));
    }
}
