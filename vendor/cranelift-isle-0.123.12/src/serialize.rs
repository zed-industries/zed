//! Put "sea of nodes" representation of a `RuleSet` into a sequential order.
//!
//! We're trying to satisfy two key constraints on generated code:
//!
//! First, we must produce the same result as if we tested the left-hand side
//! of every rule in descending priority order and picked the first match.
//! But that would mean a lot of duplicated work since many rules have similar
//! patterns. We want to evaluate in an order that gets the same answer but
//! does as little work as possible.
//!
//! Second, some ISLE patterns can only be implemented in Rust using a `match`
//! expression (or various choices of syntactic sugar). Others can only
//! be implemented as expressions, which can't be evaluated while matching
//! patterns in Rust. So we need to alternate between pattern matching and
//! expression evaluation.
//!
//! To meet both requirements, we repeatedly partition the set of rules for a
//! term and build a tree of Rust control-flow constructs corresponding to each
//! partition. The root of such a tree is a [Block], and [serialize] constructs
//! it.

use crate::disjointsets::DisjointSets;
use crate::lexer::Pos;
use crate::trie_again::{Binding, BindingId, Constraint, Rule, RuleSet};
use std::cmp::Reverse;

/// Decomposes the rule-set into a tree of [Block]s.
pub fn serialize(rules: &RuleSet) -> Block {
    // While building the tree, we need temporary storage to keep track of
    // different subsets of the rules as we partition them into ever smaller
    // sets. As long as we're allowed to re-order the rules, we can ensure
    // that every partition is contiguous; but since we plan to re-order them,
    // we actually just store indexes into the `RuleSet` to minimize data
    // movement. The algorithm in this module never duplicates or discards
    // rules, so the total size of all partitions is exactly the number of
    // rules. For all the above reasons, we can pre-allocate all the space
    // we'll need to hold those partitions up front and share it throughout the
    // tree.
    //
    // As an interesting side effect, when the algorithm finishes, this vector
    // records the order in which rule bodies will be emitted in the generated
    // Rust. We don't care because we could get the same information from the
    // built tree, but it may be helpful to think about the intermediate steps
    // as recursively sorting the rules. It may not be possible to produce the
    // same order using a comparison sort, and the asymptotic complexity is
    // probably worse than the O(n log n) of a comparison sort, but it's still
    // doing sorting of some kind.
    let mut order = Vec::from_iter(0..rules.rules.len());
    Decomposition::new(rules).sort(&mut order)
}

/// A sequence of steps to evaluate in order. Any step may return early, so
/// steps ordered later can assume the negation of the conditions evaluated in
/// earlier steps.
#[derive(Default)]
pub struct Block {
    /// Steps to evaluate.
    pub steps: Vec<EvalStep>,
}

/// A step to evaluate involves possibly let-binding some expressions, then
/// executing some control flow construct.
pub struct EvalStep {
    /// Before evaluating this case, emit let-bindings in this order.
    pub bind_order: Vec<BindingId>,
    /// The control-flow construct to execute at this point.
    pub check: ControlFlow,
}

/// What kind of control-flow structure do we need to emit here?
pub enum ControlFlow {
    /// Test a binding site against one or more mutually-exclusive patterns and
    /// branch to the appropriate block if a pattern matches.
    Match {
        /// Which binding site are we examining at this point?
        source: BindingId,
        /// What patterns do we care about?
        arms: Vec<MatchArm>,
    },
    /// Test whether two binding sites have values which are equal when
    /// evaluated on the current input.
    Equal {
        /// One binding site.
        a: BindingId,
        /// The other binding site. To ensure we always generate the same code
        /// given the same set of ISLE rules, `b` should be strictly greater
        /// than `a`.
        b: BindingId,
        /// If the test succeeds, evaluate this block.
        body: Block,
    },
    /// Evaluate a block once with each value of the given binding site.
    Loop {
        /// A binding site of type [Binding::Iterator]. Its source binding site
        /// must be a multi-extractor or multi-constructor call.
        result: BindingId,
        /// What to evaluate with each binding.
        body: Block,
    },
    /// Return a result from the right-hand side of a rule. If we're building a
    /// multi-constructor then this doesn't actually return, but adds to a list
    /// of results instead. Otherwise this return stops evaluation before any
    /// later steps.
    Return {
        /// Where was the rule defined that had this right-hand side?
        pos: Pos,
        /// What is the result expression which should be returned if this
        /// rule matched?
        result: BindingId,
    },
}

/// One concrete pattern and the block to evaluate if the pattern matches.
pub struct MatchArm {
    /// The pattern to match.
    pub constraint: Constraint,
    /// If this pattern matches, it brings these bindings into scope. If a
    /// binding is unused in this block, then the corresponding position in the
    /// pattern's bindings may be `None`.
    pub bindings: Vec<Option<BindingId>>,
    /// Steps to evaluate if the pattern matched.
    pub body: Block,
}

/// Given a set of rules that's been partitioned into two groups, move rules
/// from the first partition to the second if there are higher-priority rules
/// in the second group. In the final generated code, we'll check the rules
/// in the first ("selected") group before any in the second ("deferred")
/// group. But we need the result to be _as if_ we checked the rules in strict
/// descending priority order.
///
/// When evaluating the relationship between one rule in the selected set and
/// one rule in the deferred set, there are two cases where we can keep a rule
/// in the selected set:
/// 1. The deferred rule is lower priority than the selected rule; or
/// 2. The two rules don't overlap, so they can't match on the same inputs.
///
/// In either case, if the selected rule matches then we know the deferred rule
/// would not have been the one we wanted anyway; and if it doesn't match then
/// the fall-through semantics of the code we generate will let us go on to
/// check the deferred rule.
///
/// So a rule can stay in the selected set as long as it's in one of the above
/// relationships with every rule in the deferred set.
///
/// Due to the overlap checking pass which occurs before codegen, we know that
/// if two rules have the same priority, they do not overlap. So case 1 above
/// can be expanded to when the deferred rule is lower _or equal_ priority
/// to the selected rule. This much overlap checking is absolutely necessary:
/// There are terms where codegen is impossible if we use only the unmodified
/// case 1 and don't also check case 2.
///
/// Aside from the equal-priority case, though, case 2 does not seem to matter
/// in practice. On the current backends, doing a full overlap check here does
/// not change the generated code at all. So we don't bother.
///
/// Since this function never moves rules from the deferred set to the selected
/// set, the returned partition-point is always less than or equal to the
/// initial partition-point.
fn respect_priority(rules: &RuleSet, order: &mut [usize], partition_point: usize) -> usize {
    let (selected, deferred) = order.split_at_mut(partition_point);

    if let Some(max_deferred_prio) = deferred.iter().map(|&idx| rules.rules[idx].prio).max() {
        partition_in_place(selected, |&idx| rules.rules[idx].prio >= max_deferred_prio)
    } else {
        // If the deferred set is empty, all selected rules are fine where
        // they are.
        partition_point
    }
}

/// A query which can be tested against a [Rule] to see if that rule requires
/// the given kind of control flow around the given binding sites. These
/// choices correspond to the identically-named variants of [ControlFlow].
///
/// The order of these variants is significant, because it's used as a tie-
/// breaker in the heuristic that picks which control flow to generate next.
///
/// - Loops should always be chosen last. If a rule needs to run once for each
///   value from an iterator, but only if some other condition is true, we
///   should check the other condition first.
///
/// - Sorting concrete [HasControlFlow::Match] constraints first has the effect
///   of clustering such constraints together, which is not important but means
///   codegen could theoretically merge the cluster of matches into a single
///   Rust `match` statement.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HasControlFlow {
    /// Find rules which have a concrete pattern constraint on the given
    /// binding site.
    Match(BindingId),

    /// Find rules which require both given binding sites to be in the same
    /// equivalence class.
    Equal(BindingId, BindingId),

    /// Find rules which must loop over the multiple values of the given
    /// binding site.
    Loop(BindingId),
}

struct PartitionResults {
    any_matched: bool,
    valid: usize,
}

impl HasControlFlow {
    /// Identify which rules both satisfy this query, and are safe to evaluate
    /// before all rules that don't satisfy the query, considering rules'
    /// relative priorities like [respect_priority]. Partition matching rules
    /// first in `order`. Return the number of rules which are valid with
    /// respect to priority, as well as whether any rules matched the query at
    /// all. No ordering is guaranteed within either partition, which allows
    /// this function to run in linear time. That's fine because later we'll
    /// recursively sort both partitions.
    fn partition(self, rules: &RuleSet, order: &mut [usize]) -> PartitionResults {
        let matching = partition_in_place(order, |&idx| {
            let rule = &rules.rules[idx];
            match self {
                HasControlFlow::Match(binding_id) => rule.get_constraint(binding_id).is_some(),
                HasControlFlow::Equal(x, y) => rule.equals.in_same_set(x, y),
                HasControlFlow::Loop(binding_id) => rule.iterators.contains(&binding_id),
            }
        });
        PartitionResults {
            any_matched: matching > 0,
            valid: respect_priority(rules, order, matching),
        }
    }
}

/// As we proceed through sorting a term's rules, the term's binding sites move
/// through this sequence of states. This state machine helps us avoid doing
/// the same thing with a binding site more than once in any subtree.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
enum BindingState {
    /// Initially, all binding sites are unavailable for evaluation except for
    /// top-level arguments, constants, and similar.
    #[default]
    Unavailable,
    /// As more binding sites become available, it becomes possible to evaluate
    /// bindings which depend on those sites.
    Available,
    /// Once we've decided a binding is needed in order to make progress in
    /// matching, we emit a let-binding for it. We shouldn't evaluate it a
    /// second time, if possible.
    Emitted,
    /// We can only match a constraint against a binding site if we can emit it
    /// first. Afterward, we should not try to match a constraint against that
    /// site again in the same subtree.
    Matched,
}

/// A sort key used to order control-flow candidates in `best_control_flow`.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Score {
    // We prefer to match as many rules at once as possible.
    count: usize,
    // Break ties by preferring bindings we've already emitted.
    state: BindingState,
}

impl Score {
    /// Recompute this score. Returns whether this is a valid candidate; if
    /// not, the score may not have been updated and the candidate should
    /// be removed from further consideration. The `partition` callback is
    /// evaluated lazily.
    fn update(
        &mut self,
        state: BindingState,
        partition: impl FnOnce() -> PartitionResults,
    ) -> bool {
        // Candidates which have already been matched in this partition must
        // not be matched again. There's never anything to be gained from
        // matching a binding site when you're in an evaluation path where you
        // already know exactly what pattern that binding site matches. And
        // without this check, we could go into an infinite loop: all rules in
        // the current partition match the same pattern for this binding site,
        // so matching on it doesn't reduce the number of rules to check and it
        // doesn't make more binding sites available.
        //
        // Note that equality constraints never make a binding site `Matched`
        // and are de-duplicated using more complicated equivalence-class
        // checks instead.
        if state == BindingState::Matched {
            return false;
        }
        self.state = state;

        // The score is not based solely on how many rules have this
        // constraint, but on how many such rules can go into the same block
        // without violating rule priority. This number can grow as higher-
        // priority rules are removed from the partition, so we can't drop
        // candidates just because this is zero. If some rule has this
        // constraint, it will become viable in some later partition.
        let partition = partition();
        self.count = partition.valid;

        // Only consider constraints that are present in some rule in the
        // current partition. Note that as we partition the rule set into
        // smaller groups, the number of rules which have a particular kind of
        // constraint can never grow, so a candidate removed here doesn't need
        // to be examined again in this partition.
        partition.any_matched
    }
}

/// A rule filter ([HasControlFlow]), plus temporary storage for the sort
/// key used in `best_control_flow` to order these candidates. Keeping the
/// temporary storage here lets us avoid repeated heap allocations.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Candidate {
    score: Score,
    // Last resort tie-breaker: defer to HasControlFlow order, but prefer
    // control-flow that sorts earlier.
    kind: Reverse<HasControlFlow>,
}

impl Candidate {
    /// Construct a candidate where the score is not set. The score will need
    /// to be reset by [Score::update] before use.
    fn new(kind: HasControlFlow) -> Self {
        Candidate {
            score: Score::default(),
            kind: Reverse(kind),
        }
    }
}

/// A single binding site to check for participation in equality constraints,
/// plus temporary storage for the score used in `best_control_flow` to order
/// these candidates. Keeping the temporary storage here lets us avoid repeated
/// heap allocations.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EqualCandidate {
    score: Score,
    // Last resort tie-breaker: prefer earlier binding sites.
    source: Reverse<BindingId>,
}

impl EqualCandidate {
    /// Construct a candidate where the score is not set. The score will need
    /// to be reset by [Score::update] before use.
    fn new(source: BindingId) -> Self {
        EqualCandidate {
            score: Score::default(),
            source: Reverse(source),
        }
    }
}

/// State for a [Decomposition] that needs to be cloned when entering a nested
/// scope, so that changes in that scope don't affect this one.
#[derive(Clone, Default)]
struct ScopedState {
    /// The state of all binding sites at this point in the tree, indexed by
    /// [BindingId]. Bindings which become available in nested scopes don't
    /// magically become available in outer scopes too.
    ready: Vec<BindingState>,
    /// The current set of candidates for control flow to add at this point in
    /// the tree. We can't rely on any match results that might be computed in
    /// a nested scope, so if we still care about a candidate in the fallback
    /// case then we need to emit the correct control flow for it again.
    candidates: Vec<Candidate>,
    /// The current set of binding sites which participate in equality
    /// constraints at this point in the tree. We can't rely on any match
    /// results that might be computed in a nested scope, so if we still care
    /// about a candidate in the fallback case then we need to emit the correct
    /// control flow for it again.
    equal_candidates: Vec<EqualCandidate>,
    /// Equivalence classes that we've established on the current path from
    /// the root.
    equal: DisjointSets<BindingId>,
}

/// Builder for one [Block] in the tree.
struct Decomposition<'a> {
    /// The complete RuleSet, shared across the whole tree.
    rules: &'a RuleSet,
    /// Decomposition state that is scoped to the current subtree.
    scope: ScopedState,
    /// Accumulator for bindings that should be emitted before the next
    /// control-flow construct.
    bind_order: Vec<BindingId>,
    /// Accumulator for the final Block that we'll return as this subtree.
    block: Block,
}

impl<'a> Decomposition<'a> {
    /// Create a builder for the root [Block].
    fn new(rules: &'a RuleSet) -> Decomposition<'a> {
        let mut scope = ScopedState::default();
        scope.ready.resize(rules.bindings.len(), Default::default());
        let mut result = Decomposition {
            rules,
            scope,
            bind_order: Default::default(),
            block: Default::default(),
        };
        result.add_bindings();
        result
    }

    /// Create a builder for a nested [Block].
    fn new_block(&mut self) -> Decomposition<'_> {
        Decomposition {
            rules: self.rules,
            scope: self.scope.clone(),
            bind_order: Default::default(),
            block: Default::default(),
        }
    }

    /// Ensure that every binding site's state reflects its dependencies'
    /// states. This takes time linear in the number of bindings. Because
    /// `trie_again` only hash-conses a binding after all its dependencies have
    /// already been hash-consed, a single in-order pass visits a binding's
    /// dependencies before visiting the binding itself.
    fn add_bindings(&mut self) {
        for (idx, binding) in self.rules.bindings.iter().enumerate() {
            // We only add these bindings when matching a corresponding
            // type of control flow, in `make_control_flow`.
            if matches!(
                binding,
                Binding::Iterator { .. } | Binding::MatchVariant { .. } | Binding::MatchSome { .. }
            ) {
                continue;
            }

            // TODO: proactively put some bindings in `Emitted` state
            // That makes them visible to the best-binding heuristic, which
            // prefers to match on already-emitted bindings first. This helps
            // to sort cheap computations before expensive ones.

            let idx: BindingId = idx.try_into().unwrap();
            if self.scope.ready[idx.index()] < BindingState::Available {
                if binding
                    .sources()
                    .iter()
                    .all(|&source| self.scope.ready[source.index()] >= BindingState::Available)
                {
                    self.set_ready(idx, BindingState::Available);
                }
            }
        }
    }

    /// Determines the final evaluation order for the given subset of rules, and
    /// builds a [Block] representing that order.
    fn sort(mut self, mut order: &mut [usize]) -> Block {
        while let Some(best) = self.best_control_flow(order) {
            // Peel off all rules that have this particular control flow, and
            // save the rest for the next iteration of the loop.
            let partition_point = best.partition(self.rules, order).valid;
            debug_assert!(partition_point > 0);
            let (this, rest) = order.split_at_mut(partition_point);
            order = rest;

            // Recursively build the control-flow tree for these rules.
            let check = self.make_control_flow(best, this);
            // Note that `make_control_flow` may have added more let-bindings.
            let bind_order = std::mem::take(&mut self.bind_order);
            self.block.steps.push(EvalStep { bind_order, check });
        }

        // At this point, `best_control_flow` says the remaining rules don't
        // have any control flow left to emit. That could be because there are
        // no unhandled rules left, or because every candidate for control flow
        // for the remaining rules has already been matched by some ancestor in
        // the tree.
        debug_assert_eq!(self.scope.candidates.len(), 0);
        // TODO: assert something about self.equal_candidates?

        // If we're building a multi-constructor, then there could be multiple
        // rules with the same left-hand side. We'll evaluate them all, but
        // to keep the output consistent, first sort by descending priority
        // and break ties with the order the rules were declared. In non-multi
        // constructors, there should be at most one rule remaining here.
        order.sort_unstable_by_key(|&idx| (Reverse(self.rules.rules[idx].prio), idx));
        for &idx in order.iter() {
            let &Rule {
                pos,
                result,
                ref impure,
                ..
            } = &self.rules.rules[idx];

            // Ensure that any impure constructors are called, even if their
            // results aren't used.
            for &impure in impure.iter() {
                self.use_expr(impure);
            }
            self.use_expr(result);

            let check = ControlFlow::Return { pos, result };
            let bind_order = std::mem::take(&mut self.bind_order);
            self.block.steps.push(EvalStep { bind_order, check });
        }

        self.block
    }

    /// Let-bind this binding site and all its dependencies, skipping any
    /// which are already let-bound. Also skip let-bindings for certain trivial
    /// expressions which are safe and cheap to evaluate multiple times,
    /// because that reduces clutter in the generated code.
    fn use_expr(&mut self, name: BindingId) {
        if self.scope.ready[name.index()] < BindingState::Emitted {
            self.set_ready(name, BindingState::Emitted);
            let binding = &self.rules.bindings[name.index()];
            for &source in binding.sources() {
                self.use_expr(source);
            }

            let should_let_bind = match binding {
                Binding::ConstInt { .. } => false,
                Binding::ConstPrim { .. } => false,
                Binding::Argument { .. } => false,
                Binding::MatchTuple { .. } => false,

                // Only let-bind variant constructors if they have some fields.
                // Building a variant with no fields is cheap, but don't
                // duplicate more complex expressions.
                Binding::MakeVariant { fields, .. } => !fields.is_empty(),

                // By default, do let-bind: that's always safe.
                _ => true,
            };
            if should_let_bind {
                self.bind_order.push(name);
            }
        }
    }

    /// Build one control-flow construct and its subtree for the specified rules.
    /// The rules in `order` must all have the kind of control-flow named in `best`.
    fn make_control_flow(&mut self, best: HasControlFlow, order: &mut [usize]) -> ControlFlow {
        match best {
            HasControlFlow::Match(source) => {
                self.use_expr(source);
                self.add_bindings();
                let mut arms = Vec::new();

                let get_constraint =
                    |idx: usize| self.rules.rules[idx].get_constraint(source).unwrap();

                // Ensure that identical constraints are grouped together, then
                // loop over each group.
                order.sort_unstable_by_key(|&idx| get_constraint(idx));
                for g in group_by_mut(order, |&a, &b| get_constraint(a) == get_constraint(b)) {
                    // Applying a constraint moves the discriminant from
                    // Emitted to Matched, but only within the constraint's
                    // match arm; later fallthrough cases may need to match
                    // this discriminant again. Since `source` is in the
                    // `Emitted` state in the parent due to the above call
                    // to `use_expr`, calling `add_bindings` again after this
                    // wouldn't change anything.
                    let mut child = self.new_block();
                    child.set_ready(source, BindingState::Matched);

                    // Get the constraint for this group, and all of the
                    // binding sites that it introduces.
                    let constraint = get_constraint(g[0]);
                    let bindings = Vec::from_iter(
                        constraint
                            .bindings_for(source)
                            .into_iter()
                            .map(|b| child.rules.find_binding(&b)),
                    );

                    let mut changed = false;
                    for &binding in bindings.iter() {
                        if let Some(binding) = binding {
                            // Matching a pattern makes its bindings
                            // available, and also emits code to bind
                            // them.
                            child.set_ready(binding, BindingState::Emitted);
                            changed = true;
                        }
                    }

                    // As an optimization, only propagate availability
                    // if we changed any binding's readiness.
                    if changed {
                        child.add_bindings();
                    }

                    // Recursively construct a Block for this group of rules.
                    let body = child.sort(g);
                    arms.push(MatchArm {
                        constraint,
                        bindings,
                        body,
                    });
                }

                ControlFlow::Match { source, arms }
            }

            HasControlFlow::Equal(a, b) => {
                // Both sides of the equality test must be evaluated before
                // the condition can be tested. Go ahead and let-bind them
                // so they're available without re-evaluation in fall-through
                // cases.
                self.use_expr(a);
                self.use_expr(b);
                self.add_bindings();

                let mut child = self.new_block();
                // Never mark binding sites used in equality constraints as
                // "matched", because either might need to be used again in
                // a later equality check. Instead record that they're in the
                // same equivalence class on this path.
                child.scope.equal.merge(a, b);
                let body = child.sort(order);
                ControlFlow::Equal { a, b, body }
            }

            HasControlFlow::Loop(source) => {
                // Consuming a multi-term involves two binding sites:
                // calling the multi-term to get an iterator (the `source`),
                // and looping over the iterator to get a binding for each
                // `result`.
                let result = self
                    .rules
                    .find_binding(&Binding::Iterator { source })
                    .unwrap();

                // We must not let-bind the iterator until we're ready to
                // consume it, because it can only be consumed once. This also
                // means that the let-binding for `source` is not actually
                // reusable after this point, so even though we need to emit
                // its let-binding here, we pretend we haven't.
                let base_state = self.scope.ready[source.index()];
                debug_assert_eq!(base_state, BindingState::Available);
                self.use_expr(source);
                self.scope.ready[source.index()] = base_state;
                self.add_bindings();

                let mut child = self.new_block();
                child.set_ready(source, BindingState::Matched);
                child.set_ready(result, BindingState::Emitted);
                child.add_bindings();
                let body = child.sort(order);
                ControlFlow::Loop { result, body }
            }
        }
    }

    /// Advance the given binding to a new state. The new state usually should
    /// be greater than the existing state; but at the least it must never
    /// go backward.
    fn set_ready(&mut self, source: BindingId, state: BindingState) {
        let old = &mut self.scope.ready[source.index()];
        debug_assert!(*old <= state);

        // Add candidates for this binding, but only when it first becomes
        // available.
        if let BindingState::Unavailable = old {
            // A binding site can't have all of these kinds of constraint,
            // and many have none. But `best_control_flow` has to check all
            // candidates anyway, so let it figure out which (if any) of these
            // are applicable. It will only check false candidates once on any
            // partition, removing them from this list immediately.
            self.scope.candidates.extend([
                Candidate::new(HasControlFlow::Match(source)),
                Candidate::new(HasControlFlow::Loop(source)),
            ]);
            self.scope
                .equal_candidates
                .push(EqualCandidate::new(source));
        }

        *old = state;
    }

    /// For the specified set of rules, heuristically choose which control-flow
    /// will minimize redundant work when the generated code is running.
    fn best_control_flow(&mut self, order: &mut [usize]) -> Option<HasControlFlow> {
        // If there are no rules left, none of the candidates will match
        // anything in the `retain_mut` call below, so short-circuit it.
        if order.is_empty() {
            // This is only read in a debug-assert but it's fast so just do it
            self.scope.candidates.clear();
            return None;
        }

        // Remove false candidates, and recompute the candidate score for the
        // current set of rules in `order`.
        self.scope.candidates.retain_mut(|candidate| {
            let kind = candidate.kind.0;
            let source = match kind {
                HasControlFlow::Match(source) => source,
                HasControlFlow::Loop(source) => source,
                HasControlFlow::Equal(..) => unreachable!(),
            };
            let state = self.scope.ready[source.index()];
            candidate
                .score
                .update(state, || kind.partition(self.rules, order))
        });

        // Find the best normal candidate.
        let mut best = self.scope.candidates.iter().max().cloned();

        // Equality constraints are more complicated. We need to identify
        // some pair of binding sites which are constrained to be equal in at
        // least one rule in the current partition. We do this in two steps.
        // First, find each single binding site which participates in any
        // equality constraint in some rule. We compute the best-case `Score`
        // we could get, if there were another binding site where all the rules
        // constraining this binding site require it to be equal to that one.
        self.scope.equal_candidates.retain_mut(|candidate| {
            let source = candidate.source.0;
            let state = self.scope.ready[source.index()];
            candidate.score.update(state, || {
                let matching = partition_in_place(order, |&idx| {
                    self.rules.rules[idx].equals.find(source).is_some()
                });
                PartitionResults {
                    any_matched: matching > 0,
                    valid: respect_priority(self.rules, order, matching),
                }
            })
        });

        // Now that we know which single binding sites participate in any
        // equality constraints, we need to find the best pair of binding
        // sites. Rules that require binding sites `x` and `y` to be equal are
        // a subset of the intersection of rules constraining `x` and those
        // constraining `y`. So the upper bound on the number of matching rules
        // is whichever candidate is smaller.
        //
        // Do an O(n log n) sort to put the best single binding sites first.
        // Then the O(n^2) all-pairs loop can do branch-and-bound style
        // pruning, breaking out of a loop as soon as the remaining candidates
        // must all produce worse results than our current best candidate.
        //
        // Note that `x` and `y` are reversed, to sort in descending order.
        self.scope
            .equal_candidates
            .sort_unstable_by(|x, y| y.cmp(x));

        let mut equals = self.scope.equal_candidates.iter();
        while let Some(x) = equals.next() {
            if Some(&x.score) < best.as_ref().map(|best| &best.score) {
                break;
            }
            let x_id = x.source.0;
            for y in equals.as_slice().iter() {
                if Some(&y.score) < best.as_ref().map(|best| &best.score) {
                    break;
                }
                let y_id = y.source.0;
                // If x and y are already in the same path-scoped equivalence
                // class, then skip this pair because we already emitted this
                // check or a combination of equivalent checks on this path.
                if !self.scope.equal.in_same_set(x_id, y_id) {
                    // Sort arguments for consistency.
                    let kind = if x_id < y_id {
                        HasControlFlow::Equal(x_id, y_id)
                    } else {
                        HasControlFlow::Equal(y_id, x_id)
                    };
                    let pair = Candidate {
                        kind: Reverse(kind),
                        score: Score {
                            count: kind.partition(self.rules, order).valid,
                            // Only treat this as already-emitted if
                            // both bindings are.
                            state: x.score.state.min(y.score.state),
                        },
                    };
                    if best.as_ref() < Some(&pair) {
                        best = Some(pair);
                    }
                }
            }
        }

        best.filter(|candidate| candidate.score.count > 0)
            .map(|candidate| candidate.kind.0)
    }
}

/// Places all elements which satisfy the predicate at the beginning of the
/// slice, and all elements which don't at the end. Returns the number of
/// elements in the first partition.
///
/// This function runs in time linear in the number of elements, and calls
/// the predicate exactly once per element. If either partition is empty, no
/// writes will occur in the slice, so it's okay to call this frequently with
/// predicates that we expect won't match anything.
fn partition_in_place<T>(xs: &mut [T], mut pred: impl FnMut(&T) -> bool) -> usize {
    let mut iter = xs.iter_mut();
    let mut partition_point = 0;
    while let Some(a) = iter.next() {
        if pred(a) {
            partition_point += 1;
        } else {
            // `a` belongs in the partition at the end. If there's some later
            // element `b` that belongs in the partition at the beginning,
            // swap them. Working backwards from the end establishes the loop
            // invariant that both ends of the array are partitioned correctly,
            // and only the middle needs to be checked.
            while let Some(b) = iter.next_back() {
                if pred(b) {
                    std::mem::swap(a, b);
                    partition_point += 1;
                    break;
                }
            }
        }
    }
    partition_point
}

fn group_by_mut<T: Eq>(
    mut xs: &mut [T],
    mut pred: impl FnMut(&T, &T) -> bool,
) -> impl Iterator<Item = &mut [T]> {
    std::iter::from_fn(move || {
        if xs.is_empty() {
            None
        } else {
            let mid = xs
                .windows(2)
                .position(|w| !pred(&w[0], &w[1]))
                .map_or(xs.len(), |x| x + 1);
            let slice = std::mem::take(&mut xs);
            let (group, rest) = slice.split_at_mut(mid);
            xs = rest;
            Some(group)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_mut() {
        let slice = &mut [1, 1, 1, 3, 3, 2, 2, 2];
        let mut iter = group_by_mut(slice, |a, b| a == b);
        assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
        assert_eq!(iter.next(), Some(&mut [3, 3][..]));
        assert_eq!(iter.next(), Some(&mut [2, 2, 2][..]));
        assert_eq!(iter.next(), None);
    }
}
