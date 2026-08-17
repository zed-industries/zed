//! A strongly-normalizing intermediate representation for ISLE rules. This representation is chosen
//! to closely reflect the operations we can implement in Rust, to make code generation easy.
use crate::disjointsets::DisjointSets;
use crate::error::{Error, Span};
use crate::lexer::Pos;
use crate::sema;
use crate::stablemapset::StableSet;
use std::collections::{HashMap, hash_map::Entry};

/// A field index in a tuple or an enum variant.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TupleIndex(u8);
/// A hash-consed identifier for a binding, stored in a [RuleSet].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingId(u16);

impl std::convert::TryFrom<usize> for TupleIndex {
    type Error = <u8 as std::convert::TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(TupleIndex(value.try_into()?))
    }
}

impl std::convert::TryFrom<usize> for BindingId {
    type Error = <u16 as std::convert::TryFrom<usize>>::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(BindingId(value.try_into()?))
    }
}

impl TupleIndex {
    /// Get the index of this field.
    pub fn index(self) -> usize {
        self.0.into()
    }
}

impl BindingId {
    /// Get the index of this id.
    pub fn index(self) -> usize {
        self.0.into()
    }
}

/// Bindings are anything which can be bound to a variable name in Rust. This includes expressions,
/// such as constants or function calls; but it also includes names bound in pattern matches.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Binding {
    /// Evaluates to the given boolean literal.
    ConstBool {
        /// The constant value.
        val: bool,
        /// The constant's type.
        ty: sema::TypeId,
    },
    /// Evaluates to the given integer literal.
    ConstInt {
        /// The constant value.
        val: i128,
        /// The constant's type. Unsigned types preserve the representation of `val`, not its value.
        ty: sema::TypeId,
    },
    /// Evaluates to the given primitive Rust value.
    ConstPrim {
        /// The constant value.
        val: sema::Sym,
    },
    /// One of the arguments to the top-level function.
    Argument {
        /// Which of the function's arguments is this?
        index: TupleIndex,
    },
    /// The result of calling an external extractor.
    Extractor {
        /// Which extractor should be called?
        term: sema::TermId,
        /// What expression should be passed to the extractor?
        parameter: BindingId,
    },
    /// The result of calling an external constructor.
    Constructor {
        /// Which constructor should be called?
        term: sema::TermId,
        /// What expressions should be passed to the constructor?
        parameters: Box<[BindingId]>,
        /// For impure constructors, a unique number for each use of this term. Always 0 for pure
        /// constructors.
        instance: u32,
    },
    /// The result of getting one value from a multi-constructor or multi-extractor.
    Iterator {
        /// Which expression produced the iterator that this consumes?
        source: BindingId,
    },
    /// The result of constructing an enum variant.
    MakeVariant {
        /// Which enum type should be constructed?
        ty: sema::TypeId,
        /// Which variant of that enum should be constructed?
        variant: sema::VariantId,
        /// What expressions should be provided for this variant's fields?
        fields: Box<[BindingId]>,
    },
    /// Pattern-match one of the previous bindings against an enum variant and produce a new binding
    /// from one of its fields. There must be a corresponding [Constraint::Variant] for each
    /// `source`/`variant` pair that appears in some `MatchVariant` binding.
    MatchVariant {
        /// Which binding is being matched?
        source: BindingId,
        /// Which enum variant are we pulling binding sites from? This is somewhat redundant with
        /// information in a corresponding [Constraint]. However, it must be here so that different
        /// enum variants aren't hash-consed into the same binding site.
        variant: sema::VariantId,
        /// Which field of this enum variant are we projecting out? Although ISLE uses named fields,
        /// we track them by index for constant-time comparisons. The [sema::TypeEnv] can be used to
        /// get the field names.
        field: TupleIndex,
    },
    /// The result of constructing an Option::Some variant.
    MakeSome {
        /// Contained expression.
        inner: BindingId,
    },
    /// Pattern-match one of the previous bindings against `Option::Some` and produce a new binding
    /// from its contents. There must be a corresponding [Constraint::Some] for each `source` that
    /// appears in a `MatchSome` binding. (This currently only happens with external extractors.)
    MatchSome {
        /// Which binding is being matched?
        source: BindingId,
    },
    /// Pattern-match one of the previous bindings against a tuple and produce a new binding from
    /// one of its fields. This is an irrefutable pattern match so there is no corresponding
    /// [Constraint]. (This currently only happens with external extractors.)
    MatchTuple {
        /// Which binding is being matched?
        source: BindingId,
        /// Which tuple field are we projecting out?
        field: TupleIndex,
    },
}

/// Pattern matches which can fail. Some binding sites are the result of successfully matching a
/// constraint. A rule applies constraints to binding sites to determine whether the rule matches.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Constraint {
    /// The value must match this enum variant.
    Variant {
        /// Which enum type is being matched? This is implied by the binding where the constraint is
        /// applied, but recorded here for convenience.
        ty: sema::TypeId,
        /// Which enum variant must this binding site match to satisfy the rule?
        variant: sema::VariantId,
        /// Number of fields in this variant of this enum. This is recorded in the constraint for
        /// convenience, to avoid needing to look up the variant in a [sema::TypeEnv].
        fields: TupleIndex,
    },
    /// The value must equal this boolean literal.
    ConstBool {
        /// The constant value.
        val: bool,
        /// The constant's type.
        ty: sema::TypeId,
    },
    /// The value must equal this integer literal.
    ConstInt {
        /// The constant value.
        val: i128,
        /// The constant's type. Unsigned types preserve the representation of `val`, not its value.
        ty: sema::TypeId,
    },
    /// The value must equal this Rust primitive value.
    ConstPrim {
        /// The constant value.
        val: sema::Sym,
    },
    /// The value must be an `Option::Some`, from a fallible extractor.
    Some,
}

/// A term-rewriting rule. All [BindingId]s are only meaningful in the context of the [RuleSet] that
/// contains this rule.
#[derive(Debug, Default)]
pub struct Rule {
    /// Where was this rule defined?
    pub pos: Pos,
    /// All of these bindings must match the given constraints for this rule to apply. Note that
    /// within a single rule, if a binding site must match two different constraints, then the rule
    /// can never match.
    constraints: HashMap<BindingId, Constraint>,
    /// Sets of bindings which must be equal for this rule to match.
    pub equals: DisjointSets<BindingId>,
    /// These bindings are from multi-terms which need to be evaluated in this rule.
    pub iterators: StableSet<BindingId>,
    /// If other rules apply along with this one, the one with the highest numeric priority is
    /// evaluated. If multiple applicable rules have the same priority, that's an overlap error.
    pub prio: i64,
    /// If this rule applies, these side effects should be evaluated before returning.
    pub impure: Vec<BindingId>,
    /// If this rule applies, the top-level term should evaluate to this expression.
    pub result: BindingId,
}

/// Records whether a given pair of rules can both match on some input.
#[derive(Debug, Eq, PartialEq)]
pub enum Overlap {
    /// There is no input on which this pair of rules can both match.
    No,
    /// There is at least one input on which this pair of rules can both match.
    Yes {
        /// True if every input accepted by one rule is also accepted by the other. This does not
        /// indicate which rule is more general and in fact the rules could match exactly the same
        /// set of inputs. You can work out which by comparing `total_constraints()` in both rules:
        /// The more general rule has fewer constraints.
        subset: bool,
    },
}

/// A collection of [Rule]s, along with hash-consed [Binding]s for all of them.
#[derive(Debug, Default)]
pub struct RuleSet {
    /// The [Rule]s for a single [sema::Term].
    pub rules: Vec<Rule>,
    /// The bindings identified by [BindingId]s within rules.
    pub bindings: Vec<Binding>,
    /// Intern table for de-duplicating [Binding]s.
    binding_map: HashMap<Binding, BindingId>,
}

/// Construct a [RuleSet] for each term in `termenv` that has rules.
pub fn build(termenv: &sema::TermEnv) -> (Vec<(sema::TermId, RuleSet)>, Vec<Error>) {
    let mut errors = Vec::new();
    let mut term = HashMap::new();
    for rule in termenv.rules.iter() {
        term.entry(rule.root_term)
            .or_insert_with(RuleSetBuilder::default)
            .add_rule(rule, termenv, &mut errors);
    }

    // The `term` hash map may return terms in any order. Sort them to ensure that we produce the
    // same output every time when given the same ISLE source. Rules are added to terms in `RuleId`
    // order, so it's not necessary to sort within a `RuleSet`.
    let mut result: Vec<_> = term
        .into_iter()
        .map(|(term, builder)| (term, builder.rules))
        .collect();
    result.sort_unstable_by_key(|(term, _)| *term);

    (result, errors)
}

impl RuleSet {
    /// Returns the [BindingId] corresponding to the given [Binding] within this rule-set, if any.
    pub fn find_binding(&self, binding: &Binding) -> Option<BindingId> {
        self.binding_map.get(binding).copied()
    }
}

impl Binding {
    /// Returns the binding sites which must be evaluated before this binding.
    pub fn sources(&self) -> &[BindingId] {
        match self {
            Binding::ConstBool { .. } => &[][..],
            Binding::ConstInt { .. } => &[][..],
            Binding::ConstPrim { .. } => &[][..],
            Binding::Argument { .. } => &[][..],
            Binding::Extractor { parameter, .. } => std::slice::from_ref(parameter),
            Binding::Constructor { parameters, .. } => &parameters[..],
            Binding::Iterator { source } => std::slice::from_ref(source),
            Binding::MakeVariant { fields, .. } => &fields[..],
            Binding::MatchVariant { source, .. } => std::slice::from_ref(source),
            Binding::MakeSome { inner } => std::slice::from_ref(inner),
            Binding::MatchSome { source } => std::slice::from_ref(source),
            Binding::MatchTuple { source, .. } => std::slice::from_ref(source),
        }
    }
}

impl Constraint {
    /// Return the nested [Binding]s from matching the given [Constraint] against the given [BindingId].
    pub fn bindings_for(self, source: BindingId) -> Vec<Binding> {
        match self {
            // These constraints never introduce any bindings.
            Constraint::ConstBool { .. }
            | Constraint::ConstInt { .. }
            | Constraint::ConstPrim { .. } => vec![],
            Constraint::Some => vec![Binding::MatchSome { source }],
            Constraint::Variant {
                variant, fields, ..
            } => (0..fields.0)
                .map(TupleIndex)
                .map(|field| Binding::MatchVariant {
                    source,
                    variant,
                    field,
                })
                .collect(),
        }
    }
}

impl Rule {
    /// Returns whether a given pair of rules can both match on some input, and if so, whether
    /// either matches a subset of the other's inputs. If this function returns `No`, then the two
    /// rules definitely do not overlap. However, it may return `Yes` in cases where the rules can't
    /// overlap in practice, or where this analysis is not yet precise enough to decide.
    pub fn may_overlap(&self, other: &Rule) -> Overlap {
        // Two rules can't overlap if, for some binding site in the intersection of their
        // constraints, the rules have different constraints: an input can't possibly match both
        // rules then. If the rules do overlap, and one has a subset of the constraints of the
        // other, then the less-constrained rule matches every input that the more-constrained rule
        // matches, and possibly more. We test for both conditions at once, with the observation
        // that if the intersection of two sets is equal to the smaller set, then it's a subset. So
        // the outer loop needs to go over the rule with fewer constraints in order to correctly
        // identify if it's a subset of the other rule. Also, that way around is faster.
        let (small, big) = if self.constraints.len() <= other.constraints.len() {
            (self, other)
        } else {
            (other, self)
        };

        // TODO: nonlinear constraints complicate the subset check
        // For the purpose of overlap checking, equality constraints act like other constraints, in
        // that they can cause rules to not overlap. However, because we don't have a concrete
        // pattern to compare, the analysis to prove that is complicated. For now, we approximate
        // the result. If either rule has nonlinear constraints, conservatively report that neither
        // is a subset of the other. Note that this does not disagree with the doc comment for
        // `Overlap::Yes { subset }` which says to use `total_constraints` to disambiguate, since if
        // we return `subset: true` here, `equals` is empty for both rules, so `total_constraints()`
        // equals `constraints.len()`.
        let mut subset = small.equals.is_empty() && big.equals.is_empty();

        for (binding, a) in small.constraints.iter() {
            if let Some(b) = big.constraints.get(binding) {
                if a != b {
                    // If any binding site is constrained differently by both rules then there is
                    // no input where both rules can match.
                    return Overlap::No;
                }
                // Otherwise both are constrained in the same way at this binding site. That doesn't
                // rule out any possibilities for what inputs the rules accept.
            } else {
                // The `big` rule's inputs are a subset of the `small` rule's inputs if every
                // constraint in `small` is exactly matched in `big`. But we found a counterexample.
                subset = false;
            }
        }
        Overlap::Yes { subset }
    }

    /// Returns the total number of binding sites which this rule constrains, with either a concrete
    /// pattern or an equality constraint.
    pub fn total_constraints(&self) -> usize {
        // Because of `normalize_equivalence_classes`, these two sets don't overlap, so the size of
        // the union is the sum of their sizes.
        self.constraints.len() + self.equals.len()
    }

    /// Returns the constraint that the given binding site must satisfy for this rule to match, if
    /// there is one.
    pub fn get_constraint(&self, source: BindingId) -> Option<Constraint> {
        self.constraints.get(&source).copied()
    }

    fn set_constraint(
        &mut self,
        source: BindingId,
        constraint: Constraint,
    ) -> Result<(), UnreachableError> {
        match self.constraints.entry(source) {
            Entry::Occupied(entry) => {
                if entry.get() != &constraint {
                    return Err(UnreachableError {
                        pos: self.pos,
                        constraint_a: *entry.get(),
                        constraint_b: constraint,
                    });
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(constraint);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct UnreachableError {
    pos: Pos,
    constraint_a: Constraint,
    constraint_b: Constraint,
}

#[derive(Debug, Default)]
struct RuleSetBuilder {
    current_rule: Rule,
    impure_instance: u32,
    unreachable: Vec<UnreachableError>,
    rules: RuleSet,
}

impl RuleSetBuilder {
    fn add_rule(&mut self, rule: &sema::Rule, termenv: &sema::TermEnv, errors: &mut Vec<Error>) {
        self.impure_instance = 0;
        self.current_rule.pos = rule.pos;
        self.current_rule.prio = rule.prio;
        self.current_rule.result = rule.visit(self, termenv);
        if termenv.terms[rule.root_term.index()].is_partial() {
            self.current_rule.result = self.dedup_binding(Binding::MakeSome {
                inner: self.current_rule.result,
            });
        }
        self.normalize_equivalence_classes();
        let rule = std::mem::take(&mut self.current_rule);

        if self.unreachable.is_empty() {
            self.rules.rules.push(rule);
        } else {
            // If this rule can never match, drop it so it doesn't affect overlap checking.
            errors.extend(
                self.unreachable
                    .drain(..)
                    .map(|err| Error::UnreachableError {
                        msg: format!(
                            "rule requires binding to match both {:?} and {:?}",
                            err.constraint_a, err.constraint_b
                        ),
                        span: Span::new_single(err.pos),
                    }),
            )
        }
    }

    /// Establish the invariant that a binding site can have a concrete constraint in `constraints`,
    /// or an equality constraint in `equals`, but not both. This is useful because overlap checking
    /// is most effective on concrete constraints, and also because it exposes more rule structure
    /// for codegen.
    ///
    /// If a binding site is constrained and also required to be equal to another binding site, then
    /// copy the constraint and push the equality inside it. For example:
    /// - `(term x @ 2 x)` is rewritten to `(term 2 2)`
    /// - `(term x @ (T.A _ _) x)` is rewritten to `(term (T.A y z) (T.A y z))`
    ///
    /// In the latter case, note that every field of `T.A` has been replaced with a fresh variable
    /// and each of the copies are set equal.
    ///
    /// If several binding sites are supposed to be equal but they each have conflicting constraints
    /// then this rule is unreachable. For example, `(term x @ 2 (and x 3))` requires both arguments
    /// to be equal but also requires them to match both 2 and 3, which can't happen for any input.
    ///
    /// We could do this incrementally, while building the rule. The implementation is nearly
    /// identical but, having tried both ways, it's slightly easier to think about this as a
    /// separate pass. Also, batching up this work should be slightly faster if there are multiple
    /// binding sites set equal to each other.
    fn normalize_equivalence_classes(&mut self) {
        // First, find all the constraints that need to be copied to other binding sites in their
        // respective equivalence classes. Note: do not remove these constraints here! Yes, we'll
        // put them back later, but we rely on still having them around so that
        // `set_constraint` can detect conflicting constraints.
        let mut deferred_constraints = Vec::new();
        for (&binding, &constraint) in self.current_rule.constraints.iter() {
            if let Some(root) = self.current_rule.equals.find_mut(binding) {
                deferred_constraints.push((root, constraint));
            }
        }

        // Pick one constraint and propagate it through its equivalence class. If there are no
        // errors then it doesn't matter what order we do this in, because that means that any
        // redundant constraints on an equivalence class were equal. We can write equal values into
        // the constraint map in any order and get the same result. If there were errors, we aren't
        // going to generate code from this rule, so order only affects how conflicts are reported.
        while let Some((current, constraint)) = deferred_constraints.pop() {
            // Remove the entire equivalence class and instead add copies of this constraint to
            // every binding site in the class. If there are constraints on other binding sites in
            // this class, then when we try to copy this constraint to those binding sites,
            // `set_constraint` will check that the constraints are equal and record an appropriate
            // error otherwise.
            //
            // Later, we'll re-visit those other binding sites because they're still in
            // `deferred_constraints`, but `set` will be empty because we already deleted the
            // equivalence class the first time we encountered it.
            let set = self.current_rule.equals.remove_set_of(current);
            if let Some((&base, rest)) = set.split_first() {
                let mut defer = |this: &Self, binding| {
                    // We're adding equality constraints to binding sites that may not have had
                    // one already. If that binding site already had a concrete constraint, then
                    // we need to "recursively" propagate that constraint through the new
                    // equivalence class too.
                    if let Some(constraint) = this.current_rule.get_constraint(binding) {
                        deferred_constraints.push((binding, constraint));
                    }
                };

                // If this constraint introduces nested binding sites, make the fields of those
                // binding sites equal instead. Arbitrarily pick one member of `set` to set all the
                // others equal to. If there are existing constraints on the new binding sites, copy
                // those around the new equivalence classes too.
                let base_fields = self.set_constraint(base, constraint);
                base_fields.iter().for_each(|&x| defer(self, x));
                for &b in rest {
                    for (&x, y) in base_fields.iter().zip(self.set_constraint(b, constraint)) {
                        defer(self, y);
                        self.current_rule.equals.merge(x, y);
                    }
                }
            }
        }
    }

    fn dedup_binding(&mut self, binding: Binding) -> BindingId {
        if let Some(binding) = self.rules.binding_map.get(&binding) {
            *binding
        } else {
            let id = BindingId(self.rules.bindings.len().try_into().unwrap());
            self.rules.bindings.push(binding.clone());
            self.rules.binding_map.insert(binding, id);
            id
        }
    }

    fn set_constraint(&mut self, input: BindingId, constraint: Constraint) -> Vec<BindingId> {
        if let Err(e) = self.current_rule.set_constraint(input, constraint) {
            self.unreachable.push(e);
        }
        constraint
            .bindings_for(input)
            .into_iter()
            .map(|binding| self.dedup_binding(binding))
            .collect()
    }
}

impl sema::PatternVisitor for RuleSetBuilder {
    type PatternId = BindingId;

    fn add_match_equal(&mut self, a: BindingId, b: BindingId, _ty: sema::TypeId) {
        // If both bindings represent the same binding site, they're implicitly equal.
        if a != b {
            self.current_rule.equals.merge(a, b);
        }
    }

    fn add_match_bool(&mut self, input: BindingId, ty: sema::TypeId, val: bool) {
        let bindings = self.set_constraint(input, Constraint::ConstBool { val, ty });
        debug_assert_eq!(bindings, &[]);
    }

    fn add_match_int(&mut self, input: BindingId, ty: sema::TypeId, val: i128) {
        let bindings = self.set_constraint(input, Constraint::ConstInt { val, ty });
        debug_assert_eq!(bindings, &[]);
    }

    fn add_match_prim(&mut self, input: BindingId, _ty: sema::TypeId, val: sema::Sym) {
        let bindings = self.set_constraint(input, Constraint::ConstPrim { val });
        debug_assert_eq!(bindings, &[]);
    }

    fn add_match_variant(
        &mut self,
        input: BindingId,
        input_ty: sema::TypeId,
        arg_tys: &[sema::TypeId],
        variant: sema::VariantId,
    ) -> Vec<BindingId> {
        let fields = TupleIndex(arg_tys.len().try_into().unwrap());
        self.set_constraint(
            input,
            Constraint::Variant {
                fields,
                ty: input_ty,
                variant,
            },
        )
    }

    fn add_extract(
        &mut self,
        input: BindingId,
        _input_ty: sema::TypeId,
        output_tys: Vec<sema::TypeId>,
        term: sema::TermId,
        infallible: bool,
        multi: bool,
    ) -> Vec<BindingId> {
        let source = self.dedup_binding(Binding::Extractor {
            term,
            parameter: input,
        });

        // If the extractor is fallible, build a pattern and constraint for `Some`
        let source = if multi {
            self.current_rule.iterators.insert(source);
            self.dedup_binding(Binding::Iterator { source })
        } else if infallible {
            source
        } else {
            let bindings = self.set_constraint(source, Constraint::Some);
            debug_assert_eq!(bindings.len(), 1);
            bindings[0]
        };

        // If the extractor has multiple outputs, create a separate binding for each
        match output_tys.len().try_into().unwrap() {
            0 => vec![],
            1 => vec![source],
            outputs => (0..outputs)
                .map(TupleIndex)
                .map(|field| self.dedup_binding(Binding::MatchTuple { source, field }))
                .collect(),
        }
    }
}

impl sema::ExprVisitor for RuleSetBuilder {
    type ExprId = BindingId;

    fn add_const_bool(&mut self, ty: sema::TypeId, val: bool) -> BindingId {
        self.dedup_binding(Binding::ConstBool { val, ty })
    }

    fn add_const_int(&mut self, ty: sema::TypeId, val: i128) -> BindingId {
        self.dedup_binding(Binding::ConstInt { val, ty })
    }

    fn add_const_prim(&mut self, _ty: sema::TypeId, val: sema::Sym) -> BindingId {
        self.dedup_binding(Binding::ConstPrim { val })
    }

    fn add_create_variant(
        &mut self,
        inputs: Vec<(BindingId, sema::TypeId)>,
        ty: sema::TypeId,
        variant: sema::VariantId,
    ) -> BindingId {
        self.dedup_binding(Binding::MakeVariant {
            ty,
            variant,
            fields: inputs.into_iter().map(|(expr, _)| expr).collect(),
        })
    }

    fn add_construct(
        &mut self,
        inputs: Vec<(BindingId, sema::TypeId)>,
        _ty: sema::TypeId,
        term: sema::TermId,
        pure: bool,
        infallible: bool,
        multi: bool,
    ) -> BindingId {
        let instance = if pure {
            0
        } else {
            self.impure_instance += 1;
            self.impure_instance
        };
        let source = self.dedup_binding(Binding::Constructor {
            term,
            parameters: inputs.into_iter().map(|(expr, _)| expr).collect(),
            instance,
        });

        // If the constructor is fallible, build a pattern for `Some`, but not a constraint. If the
        // constructor is on the right-hand side of a rule then its failure is not considered when
        // deciding which rule to evaluate. Corresponding constraints are only added if this
        // expression is subsequently used as a pattern; see `expr_as_pattern`.
        let source = if multi {
            self.current_rule.iterators.insert(source);
            self.dedup_binding(Binding::Iterator { source })
        } else if infallible {
            source
        } else {
            self.dedup_binding(Binding::MatchSome { source })
        };

        if !pure {
            self.current_rule.impure.push(source);
        }

        source
    }
}

impl sema::RuleVisitor for RuleSetBuilder {
    type PatternVisitor = Self;
    type ExprVisitor = Self;
    type Expr = BindingId;

    fn add_arg(&mut self, index: usize, _ty: sema::TypeId) -> BindingId {
        let index = TupleIndex(index.try_into().unwrap());
        self.dedup_binding(Binding::Argument { index })
    }

    fn add_pattern<F: FnOnce(&mut Self)>(&mut self, visitor: F) {
        visitor(self)
    }

    fn add_expr<F>(&mut self, visitor: F) -> BindingId
    where
        F: FnOnce(&mut Self) -> sema::VisitedExpr<Self>,
    {
        visitor(self).value
    }

    fn expr_as_pattern(&mut self, expr: BindingId) -> BindingId {
        let mut todo = vec![expr];
        while let Some(expr) = todo.pop() {
            let expr = &self.rules.bindings[expr.index()];
            todo.extend_from_slice(expr.sources());
            if let &Binding::MatchSome { source } = expr {
                let _ = self.set_constraint(source, Constraint::Some);
            }
        }
        expr
    }

    fn pattern_as_expr(&mut self, pattern: BindingId) -> BindingId {
        pattern
    }
}
