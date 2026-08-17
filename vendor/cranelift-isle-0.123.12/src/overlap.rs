//! Overlap detection for rules in ISLE.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::error::{Error, Span};
use crate::lexer::Pos;
use crate::sema::{TermEnv, TermId, TermKind};
use crate::trie_again;

/// Check for overlap.
pub fn check(termenv: &TermEnv) -> Result<Vec<(TermId, trie_again::RuleSet)>, Vec<Error>> {
    let (terms, mut errors) = trie_again::build(termenv);
    errors.append(&mut check_overlaps(&terms, termenv).report());

    if errors.is_empty() {
        Ok(terms)
    } else {
        Err(errors)
    }
}

/// A graph of rules that overlap in the ISLE source. The edges are undirected.
#[derive(Default)]
struct Errors {
    /// Edges between rules indicating overlap.
    nodes: HashMap<Pos, HashSet<Pos>>,
    /// For each (mask, shadowed) pair, every rule in `shadowed` is unmatchable because `mask` will
    /// always match first.
    shadowed: HashMap<Pos, Vec<Pos>>,
}

impl Errors {
    /// Condense the overlap information down into individual errors. We iteratively remove the
    /// nodes from the graph with the highest degree, reporting errors for them and their direct
    /// connections. The goal with reporting errors this way is to prefer reporting rules that
    /// overlap with many others first, and then report other more targeted overlaps later.
    fn report(mut self) -> Vec<Error> {
        let mut errors = Vec::new();

        while let Some((&pos, _)) = self
            .nodes
            .iter()
            .max_by_key(|(pos, edges)| (edges.len(), *pos))
        {
            let node = self.nodes.remove(&pos).unwrap();
            for other in node.iter() {
                if let Entry::Occupied(mut entry) = self.nodes.entry(*other) {
                    let back_edges = entry.get_mut();
                    back_edges.remove(&pos);
                    if back_edges.is_empty() {
                        entry.remove();
                    }
                }
            }

            // build the real error
            let mut rules = vec![Span::new_single(pos)];

            rules.extend(node.into_iter().map(Span::new_single));

            errors.push(Error::OverlapError {
                msg: String::from("rules are overlapping"),
                rules,
            });
        }

        errors.extend(
            self.shadowed
                .into_iter()
                .map(|(mask, shadowed)| Error::ShadowedError {
                    shadowed: shadowed.into_iter().map(Span::new_single).collect(),
                    mask: Span::new_single(mask),
                }),
        );

        errors.sort_by_key(|err| match err {
            Error::ShadowedError { mask, .. } => mask.from,
            Error::OverlapError { rules, .. } => rules[0].from,
            _ => Pos::default(),
        });
        errors
    }

    fn check_pair(&mut self, a: &trie_again::Rule, b: &trie_again::Rule) {
        if let trie_again::Overlap::Yes { subset } = a.may_overlap(b) {
            if a.prio == b.prio {
                // edges are undirected
                self.nodes.entry(a.pos).or_default().insert(b.pos);
                self.nodes.entry(b.pos).or_default().insert(a.pos);
            } else if subset {
                // One rule's constraints are a subset of the other's, or they're equal.
                // This is fine as long as the higher-priority rule has more constraints.
                let (lo, hi) = if a.prio < b.prio { (a, b) } else { (b, a) };
                if hi.total_constraints() <= lo.total_constraints() {
                    // Otherwise, the lower-priority rule can never match.
                    self.shadowed.entry(hi.pos).or_default().push(lo.pos);
                }
            }
        }
    }
}

/// Determine if any rules overlap in the input that they accept. This checks every unique pair of
/// rules, as checking rules in aggregate tends to suffer from exponential explosion in the
/// presence of wildcard patterns.
fn check_overlaps(terms: &[(TermId, trie_again::RuleSet)], env: &TermEnv) -> Errors {
    let mut errs = Errors::default();
    for (tid, ruleset) in terms {
        let is_multi_ctor = match &env.terms[tid.index()].kind {
            TermKind::Decl { flags, .. } => flags.multi,
            _ => false,
        };
        if is_multi_ctor {
            // Rules for multi-constructors are not checked for
            // overlap: the ctor returns *every* match, not just
            // the first or highest-priority one, so overlap does
            // not actually affect the results.
            continue;
        }

        let mut cursor = ruleset.rules.iter();
        while let Some(left) = cursor.next() {
            for right in cursor.as_slice() {
                errs.check_pair(left, right);
            }
        }
    }
    errs
}
