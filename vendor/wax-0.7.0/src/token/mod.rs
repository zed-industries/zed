mod parse;
mod variance;
pub mod walk;

use itertools::Itertools as _;
use std::borrow::Cow;
use std::cmp;
use std::collections::VecDeque;
use std::iter;
use std::mem;
use std::path::{MAIN_SEPARATOR_STR, PathBuf};
use std::slice;
use std::str;

use crate::diagnostics::{Span, Spanned};
use crate::query::When;
use crate::token::variance::invariant::{
    BoundaryTerm, Finalize, IntoNominalText, IntoStructuralText, InvariantTerm, One, Zero,
};
use crate::token::variance::ops;
use crate::token::variance::{TreeExhaustiveness, TreeVariance, VarianceFold, VarianceTerm};
use crate::token::walk::{BranchFold, Fold, FoldMap, Starting, TokenEntry};
use crate::{PATHS_ARE_CASE_INSENSITIVE, StrExt as _};

pub use crate::token::parse::{ParseError, ROOT_SEPARATOR_EXPRESSION, parse};
pub use crate::token::variance::invariant::{Breadth, Depth, Invariant, Size, Text};
pub use crate::token::variance::natural::{BoundedVariantRange, NaturalRange, VariantRange};
pub use crate::token::variance::{Boundedness, TokenVariance, Variance};

// TODO: Tree representations of expressions are intrusive and only differ in their annotations.
//       This supports some distinctions between tree types, but greatly limits the ability to
//       model syntactic vs. semantic glob representations. Consider an unintrusive tree data
//       structure and more distinct (and disjoint) glob representations. See also
//       `BranchComposition`.

pub type ExpressionMetadata = Span;

pub trait ConcatenationTree<'t> {
    type Annotation;

    fn concatenation(&self) -> &[Token<'t, Self::Annotation>];
}

pub trait TokenTree<'t> {
    type Annotation;

    fn into_token(self) -> Token<'t, Self::Annotation>
    where
        Self: Sized;

    fn as_token(&self) -> &Token<'t, Self::Annotation>;
}

impl<'t, T> ConcatenationTree<'t> for T
where
    T: TokenTree<'t>,
{
    type Annotation = T::Annotation;

    fn concatenation(&self) -> &[Token<'t, Self::Annotation>] {
        self.as_token().concatenation()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Composition<C, D> {
    Conjunctive(C),
    Disjunctive(D),
}

pub type TokenComposition<T> = Composition<T, T>;

impl<C, D> Composition<C, D> {
    pub fn map_conjunctive<T, F>(self, f: F) -> Composition<T, D>
    where
        F: FnOnce(C) -> T,
    {
        match self {
            Composition::Conjunctive(inner) => Composition::Conjunctive(f(inner)),
            Composition::Disjunctive(inner) => Composition::Disjunctive(inner),
        }
    }

    pub fn map_disjunctive<T, F>(self, f: F) -> Composition<C, T>
    where
        F: FnOnce(D) -> T,
    {
        match self {
            Composition::Conjunctive(inner) => Composition::Conjunctive(inner),
            Composition::Disjunctive(inner) => Composition::Disjunctive(f(inner)),
        }
    }

    pub fn conjunctive(self) -> Option<C> {
        match self {
            Composition::Conjunctive(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn disjunctive(self) -> Option<D> {
        match self {
            Composition::Disjunctive(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_ref(&self) -> Composition<&C, &D> {
        match self {
            Composition::Conjunctive(inner) => Composition::Conjunctive(inner),
            Composition::Disjunctive(inner) => Composition::Disjunctive(inner),
        }
    }
}

impl<T> Composition<T, T> {
    pub fn into_inner(self) -> T {
        match self {
            Composition::Conjunctive(inner) | Composition::Disjunctive(inner) => inner,
        }
    }

    pub fn map<U, F>(self, f: F) -> Composition<U, U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Composition::Conjunctive(inner) => Composition::Conjunctive(f(inner)),
            Composition::Disjunctive(inner) => Composition::Disjunctive(f(inner)),
        }
    }

    pub fn get(&self) -> &T {
        AsRef::as_ref(self)
    }

    pub fn get_mut(&mut self) -> &mut T {
        AsMut::as_mut(self)
    }
}

impl<T> AsMut<T> for Composition<T, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Composition::Conjunctive(inner) | Composition::Disjunctive(inner) => inner,
        }
    }
}

impl<T> AsRef<T> for Composition<T, T> {
    fn as_ref(&self) -> &T {
        match self {
            Composition::Conjunctive(inner) | Composition::Disjunctive(inner) => inner,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Boundary {
    Component,
    Separator,
}

#[derive(Clone, Debug)]
pub struct Tokenized<'t, A> {
    expression: Cow<'t, str>,
    token: Token<'t, A>,
}

impl<'t, A> Tokenized<'t, A> {
    pub fn into_owned(self) -> Tokenized<'static, A> {
        let Tokenized { expression, token } = self;
        Tokenized {
            expression: expression.into_owned().into(),
            token: token.into_owned(),
        }
    }

    pub fn expression(&self) -> &Cow<'t, str> {
        &self.expression
    }
}

impl<'t, A> Tokenized<'t, A>
where
    A: Default + Spanned,
{
    // TODO: This function is limited to immediate concatenations and so expressions like
    //       `{.cargo/**/*.crate}` do not partition (into the path `.cargo` and glob
    //       `{**/*.crate}`). This requires a more sophisticated transformation of the token tree,
    //       but is probably worth supporting. Maybe this can be done via `FoldMap`.
    pub fn partition(self) -> (PathBuf, Option<Self>) {
        fn pop_expression_bytes(expression: &str, n: usize) -> &str {
            let n = cmp::min(expression.len(), n);
            str::from_utf8(&expression.as_bytes()[n..])
                .expect("span offset split UTF-8 byte sequence")
        }

        let Tokenized { expression, token } = self;

        let (n, text) = token.invariant_text_prefix();
        let mut unrooted = 0;
        let (popped, token) = token.pop_prefix_tokens_with(n, |first| {
            unrooted = first.unroot_boundary_component();
        });
        let offset = popped
            .into_iter()
            .map(|token| token.annotation().span().1)
            .sum::<usize>()
            + unrooted;

        (
            text.into(),
            token.map(|token| Tokenized {
                token: token.fold_map(|annotation: A| {
                    annotation.map_span(|(start, n)| (start.saturating_sub(offset), n))
                }),
                expression: match expression {
                    Cow::Borrowed(expression) => pop_expression_bytes(expression, offset).into(),
                    Cow::Owned(expression) => {
                        String::from(pop_expression_bytes(&expression, offset)).into()
                    },
                },
            }),
        )
    }
}

impl<'t, A> TokenTree<'t> for Tokenized<'t, A> {
    type Annotation = A;

    fn into_token(self) -> Token<'t, Self::Annotation> {
        let Tokenized { token, .. } = self;
        token
    }

    fn as_token(&self) -> &Token<'t, Self::Annotation> {
        &self.token
    }
}

#[derive(Clone, Debug)]
pub struct Token<'t, A> {
    topology: TokenTopology<'t, A>,
    annotation: A,
}

impl<'t, A> Token<'t, A> {
    fn new(topology: impl Into<TokenTopology<'t, A>>, annotation: A) -> Self {
        Token {
            topology: topology.into(),
            annotation,
        }
    }

    pub const fn empty(annotation: A) -> Self {
        Token {
            topology: TokenTopology::Leaf(LeafKind::Literal(Literal::EMPTY)),
            annotation,
        }
    }

    pub fn into_owned(self) -> Token<'static, A> {
        struct IntoOwned;

        impl<'t, A> FoldMap<'t, 'static, A> for IntoOwned {
            type Annotation = A;

            fn fold(
                &mut self,
                annotation: A,
                branch: BranchFold,
                tokens: Vec<Token<'static, Self::Annotation>>,
            ) -> Option<Token<'static, Self::Annotation>> {
                branch
                    .fold(tokens)
                    .ok()
                    .map(|branch| Token::new(branch, annotation))
            }

            fn map(
                &mut self,
                annotation: A,
                leaf: LeafKind<'t>,
            ) -> Token<'static, Self::Annotation> {
                Token::new(leaf.into_owned(), annotation)
            }
        }

        self.fold_map(IntoOwned)
    }

    pub fn into_alternatives(self) -> Vec<Self> {
        let mut alternatives = vec![];
        let mut tokens: VecDeque<Self> = [self.into_non_trivial()].into_iter().collect();
        while let Some(mut token) = tokens.pop_front() {
            match token.as_branch_mut() {
                Some(BranchKind::Alternation(alternation)) => {
                    for token in alternation.0.drain(..).map(Token::into_non_trivial) {
                        if token.is_alternation() {
                            tokens.push_back(token);
                        }
                        else {
                            alternatives.push(token);
                        }
                    }
                },
                _ => {
                    alternatives.push(token);
                },
            }
        }
        alternatives
    }

    fn into_non_trivial(self) -> Self {
        use BranchKind::{
            Alternation as AlternationKind, Concatenation as ConcatenationKind,
            Repetition as RepetitionKind,
        };
        use Topology::Branch;

        // Collapse trivial branch tokens. Branch tokens are trivial if they have no semantic
        // effect.
        let mut token = self;
        loop {
            token = match token.topology {
                // Alternations and concatenations with only one child token are trivial.
                Branch(
                    AlternationKind(Alternation(mut tokens))
                    | ConcatenationKind(Concatenation(mut tokens)),
                ) if tokens.len() == 1 => tokens.drain(..).next().unwrap(),
                // Repetitions that occur exactly once are trivial.
                Branch(RepetitionKind(repetition)) if repetition.variance().is_one() => {
                    *repetition.token
                },
                _ => {
                    return token;
                },
            };
        }
    }

    fn pop_prefix_tokens_with<F>(mut self, n: usize, f: F) -> (Vec<Self>, Option<Self>)
    where
        F: FnOnce(&mut Self),
    {
        if let Some(concatenation) = self.as_concatenation_mut() {
            if n >= concatenation.tokens().len() {
                // Pop the entire concatenation if exhausted. This always pops empty
                // concatenations.
                (vec![self], None)
            }
            else {
                // Pop `n` tokens and forward the first remaining token in the concatenation
                // (postfix) to `f`.
                let tokens = concatenation.0.drain(0..n).collect();
                f(concatenation.0.first_mut().unwrap());
                (tokens, Some(self))
            }
        }
        else if n == 0 {
            // Yield the token as-is if there are no tokens to pop.
            (vec![], Some(self))
        }
        else {
            // Pop the entire token if it is not a concatenation (and `n` is not zero).
            (vec![self], None)
        }
    }

    fn unroot_boundary_component(&mut self) -> usize
    where
        LeafKind<'t>: Unroot<A, Output = usize>,
    {
        let annotation = &mut self.annotation;
        match &mut self.topology {
            TokenTopology::Leaf(leaf) => leaf.unroot(annotation),
            _ => 0,
        }
    }

    pub fn tokens(&self) -> Option<TokenComposition<&[Self]>> {
        self.as_branch().map(BranchKind::tokens)
    }

    pub fn components(&self) -> impl Iterator<Item = Component<'_, 't, A>> {
        self::components(self.concatenation())
    }

    // TODO: This API operates differently than `components`, but appears similar and may be
    //       confusing. Unlike `components`, this function searches the tree of the token rather
    //       than only its concatenation. Consider names or different APIs that make this more
    //       obvious.
    // TODO: This function only finds `LiteralSequence` components rather than **invariant text**
    //       components, and so may miss some interesting text. Consider `[.][.]/a`. The text of
    //       this pattern is invariant, but this function does not find and cannot represent the
    //       `..` component.
    pub fn literals(
        &self,
    ) -> impl Iterator<Item = (Component<'_, 't, A>, LiteralSequence<'_, 't>)> {
        let mut components: VecDeque<_> = self.components().collect();
        iter::from_fn(move || {
            while let Some(component) = components.pop_front() {
                if let Some(literal) = component.literal() {
                    return Some((component, literal));
                }
                components.extend(
                    component
                        .tokens()
                        .iter()
                        .filter_map(Token::as_branch)
                        .flat_map(|branch| self::components(branch.tokens().into_inner())),
                );
            }
            None
        })
    }

    pub fn concatenation(&self) -> &[Self] {
        if let Some(concatenation) = self.as_concatenation() {
            concatenation.tokens()
        }
        else {
            slice::from_ref(self)
        }
    }

    pub fn composition(&self) -> TokenComposition<()> {
        self.as_branch()
            .map_or(Composition::Conjunctive(()), BranchKind::composition)
    }

    pub fn topology(&self) -> &TokenTopology<'t, A> {
        &self.topology
    }

    pub fn annotation(&self) -> &A {
        &self.annotation
    }

    pub fn boundary(&self) -> Option<Boundary> {
        self.as_leaf().and_then(LeafKind::boundary)
    }

    pub fn variance<T>(&self) -> TokenVariance<T>
    where
        TreeVariance<T>: Fold<'t, A, Term = T::Term>,
        T: Invariant,
    {
        T::Term::finalize(
            self.fold(TreeVariance::default())
                .unwrap_or_else(Zero::zero),
        )
    }

    pub fn invariant_text_prefix(&self) -> (usize, String) {
        #[derive(Clone, Debug, Default)]
        struct Prefix {
            index: usize,
            text: String,
        }

        impl Prefix {
            fn into_count_text(self) -> (usize, String) {
                (self.index + 1, self.text)
            }
        }

        fn empty() -> (usize, String) {
            (0, String::new())
        }

        let mut tokens = self.concatenation().iter().peekable();
        if tokens.peek().is_some_and(|token| {
            // This is a very general predicate, but at time of writing amounts to, "Is this a tree
            // wildcard?"
            token.has_root().is_always() && token.variance::<Text>().is_variant()
        }) {
            return (0, String::from(Separator::INVARIANT_TEXT));
        }
        let mut head = None;
        let mut checkpoint = None;
        for (n, token) in tokens.enumerate() {
            match token.variance::<Text>() {
                Variance::Invariant(text) => {
                    let prefix = head.get_or_insert_with(Prefix::default);
                    prefix.index = n;
                    prefix.text.push_str(text.to_string().as_ref());
                    if token.is_boundary() {
                        checkpoint = head.clone();
                    }
                },
                _ => {
                    return match token.boundary() {
                        Some(_) => head,
                        None => checkpoint,
                    }
                    .map_or_else(empty, Prefix::into_count_text);
                },
            }
        }
        head.map_or_else(empty, Prefix::into_count_text)
    }

    pub fn has_root(&self) -> When {
        struct IsRooting;

        impl<'t, A> Fold<'t, A> for IsRooting {
            type Sequencer = Starting;
            type Term = When;

            fn sequencer() -> Self::Sequencer {
                Starting
            }

            fn fold(
                &mut self,
                branch: &BranchKind<'t, A>,
                terms: Vec<Self::Term>,
            ) -> Option<Self::Term> {
                terms
                    .into_iter()
                    .reduce(match branch.composition() {
                        Composition::Conjunctive(_) => When::or,
                        Composition::Disjunctive(_) => When::certainty,
                    })
                    .map(|term| {
                        if let BranchKind::Repetition(repetition) = branch {
                            if repetition.variance().lower().into_bound().is_unbounded() {
                                return term.and(When::Sometimes);
                            }
                        }
                        term
                    })
            }

            fn term(&mut self, leaf: &LeafKind<'t>) -> Self::Term {
                leaf.is_rooting().into()
            }
        }

        self.fold(IsRooting).unwrap_or(When::Never)
    }

    pub fn has_boundary(&self) -> bool {
        walk::forward(self)
            .map(TokenEntry::into_token)
            .any(|token| token.boundary().is_some())
    }

    pub fn is_boundary(&self) -> bool {
        self.boundary().is_some()
    }

    pub fn is_capturing(&self) -> bool {
        match &self.topology {
            TokenTopology::Branch(branch) => branch.is_capturing(),
            TokenTopology::Leaf(leaf) => leaf.is_capturing(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.as_literal() {
            Some(literal) => literal == &Literal::EMPTY,
            _ => false,
        }
    }

    // TODO: There is a distinction between exhaustiveness of a glob and exhaustiveness of a match
    //       (this is also true of other properties). The latter can be important for performance
    //       optimization, but may also be useful in the public API (perhaps as part of
    //       `MatchedText`).
    // NOTE: False positives in this function cause negated pattern to incorrectly discard
    //       directory trees in the `FileIterator::not` combinator.
    pub fn is_exhaustive(&self) -> When {
        self.fold(TreeExhaustiveness)
            .as_ref()
            .map_or(When::Never, BoundaryTerm::is_exhaustive)
    }
}

impl<'t, A> Token<'t, A> {
    pub fn as_branch(&self) -> Option<&BranchKind<'t, A>> {
        match &self.topology {
            TokenTopology::Branch(branch) => Some(branch),
            _ => None,
        }
    }

    fn as_branch_mut(&mut self) -> Option<&mut BranchKind<'t, A>> {
        match &mut self.topology {
            TokenTopology::Branch(branch) => Some(branch),
            _ => None,
        }
    }

    pub fn as_leaf(&self) -> Option<&LeafKind<'t>> {
        match &self.topology {
            TokenTopology::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn as_alternation(&self) -> Option<&Alternation<'t, A>> {
        self.as_branch().and_then(|branch| match branch {
            BranchKind::Alternation(alternation) => Some(alternation),
            _ => None,
        })
    }

    pub fn as_class(&self) -> Option<&Class> {
        self.as_leaf().and_then(|leaf| match leaf {
            LeafKind::Class(class) => Some(class),
            _ => None,
        })
    }

    pub fn as_concatenation(&self) -> Option<&Concatenation<'t, A>> {
        self.as_branch().and_then(|branch| match branch {
            BranchKind::Concatenation(concatenation) => Some(concatenation),
            _ => None,
        })
    }

    fn as_concatenation_mut(&mut self) -> Option<&mut Concatenation<'t, A>> {
        self.as_branch_mut().and_then(|branch| match branch {
            BranchKind::Concatenation(concatenation) => Some(concatenation),
            _ => None,
        })
    }

    pub fn as_literal(&self) -> Option<&Literal<'t>> {
        self.as_leaf().and_then(|leaf| match leaf {
            LeafKind::Literal(literal) => Some(literal),
            _ => None,
        })
    }

    pub fn as_repetition(&self) -> Option<&Repetition<'t, A>> {
        self.as_branch().and_then(|branch| match branch {
            BranchKind::Repetition(repetition) => Some(repetition),
            _ => None,
        })
    }

    pub fn as_separator(&self) -> Option<&Separator> {
        self.as_leaf().and_then(|leaf| match leaf {
            LeafKind::Separator(separator) => Some(separator),
            _ => None,
        })
    }

    pub fn as_wildcard(&self) -> Option<&Wildcard> {
        self.as_leaf().and_then(|leaf| match leaf {
            LeafKind::Wildcard(wildcard) => Some(wildcard),
            _ => None,
        })
    }
}

impl<'t, A> Token<'t, A> {
    pub fn is_branch(&self) -> bool {
        self.as_branch().is_some()
    }

    pub fn is_leaf(&self) -> bool {
        self.as_leaf().is_some()
    }

    pub fn is_alternation(&self) -> bool {
        self.as_alternation().is_some()
    }

    pub fn is_concatenation(&self) -> bool {
        self.as_concatenation().is_some()
    }

    pub fn is_literal(&self) -> bool {
        self.as_literal().is_some()
    }
}

impl<'t, A> AsRef<A> for Token<'t, A> {
    fn as_ref(&self) -> &A {
        &self.annotation
    }
}

impl<'t, A> From<TokenTopology<'t, A>> for Token<'t, A>
where
    A: Default,
{
    fn from(topology: TokenTopology<'t, A>) -> Self {
        Token {
            topology,
            annotation: A::default(),
        }
    }
}

impl<'t, A> TokenTree<'t> for Token<'t, A> {
    type Annotation = A;

    fn into_token(self) -> Token<'t, Self::Annotation> {
        self
    }

    fn as_token(&self) -> &Token<'t, Self::Annotation> {
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Topology<B, L> {
    Branch(B),
    Leaf(L),
}

pub type TokenTopology<'t, A> = Topology<BranchKind<'t, A>, LeafKind<'t>>;

impl<'t, A> From<BranchKind<'t, A>> for TokenTopology<'t, A> {
    fn from(kind: BranchKind<'t, A>) -> Self {
        TokenTopology::Branch(kind)
    }
}

impl<'t, A> From<LeafKind<'t>> for TokenTopology<'t, A> {
    fn from(kind: LeafKind<'t>) -> Self {
        TokenTopology::Leaf(kind)
    }
}

// TODO: The use of this trait and `BranchFold` in `Token::fold_map` is a very unfortunate
//       consequence of using an intrusive tree: the data and topology of a token tree cannot be
//       cleanly separated. Remove these APIs if and when the tree is implemented via an
//       unintrusive data structure. See `BranchFold`.
trait BranchComposition<'t>: Sized {
    type Annotation;
    type BranchData;

    fn compose(
        data: Self::BranchData,
        tokens: Vec<Token<'t, Self::Annotation>>,
    ) -> Result<Self, ()>;

    fn decompose(self) -> (Self::BranchData, Vec<Token<'t, Self::Annotation>>);
}

#[derive(Clone, Debug)]
pub enum BranchKind<'t, A> {
    Alternation(Alternation<'t, A>),
    Concatenation(Concatenation<'t, A>),
    Repetition(Repetition<'t, A>),
}

impl<'t, A> BranchKind<'t, A> {
    fn compose<T>(data: T::BranchData, tokens: Vec<Token<'t, A>>) -> Result<Self, ()>
    where
        Self: From<T>,
        T: BranchComposition<'t, Annotation = A>,
    {
        T::compose(data, tokens).map(From::from)
    }

    fn decompose(self) -> (BranchFold, Vec<Token<'t, A>>) {
        match self {
            BranchKind::Alternation(alternation) => {
                let (data, tokens) = alternation.decompose();
                (BranchFold::Alternation(data), tokens)
            },
            BranchKind::Concatenation(concatenation) => {
                let (data, tokens) = concatenation.decompose();
                (BranchFold::Concatenation(data), tokens)
            },
            BranchKind::Repetition(repetition) => {
                let (data, tokens) = repetition.decompose();
                (BranchFold::Repetition(data), tokens)
            },
        }
    }

    pub fn tokens(&self) -> TokenComposition<&[Token<'t, A>]> {
        use BranchKind::{Alternation, Concatenation, Repetition};
        use Composition::{Conjunctive, Disjunctive};

        match self {
            Alternation(alternation) => Disjunctive(alternation.tokens()),
            Concatenation(concatenation) => Conjunctive(concatenation.tokens()),
            Repetition(repetition) => Conjunctive(slice::from_ref(repetition.token())),
        }
    }

    pub fn composition(&self) -> TokenComposition<()> {
        self.tokens().map(|_| ())
    }

    pub fn is_capturing(&self) -> bool {
        matches!(self, BranchKind::Alternation(_) | BranchKind::Repetition(_))
    }
}

impl<'t, A> From<Alternation<'t, A>> for BranchKind<'t, A> {
    fn from(alternation: Alternation<'t, A>) -> Self {
        BranchKind::Alternation(alternation)
    }
}

impl<'t, A> From<Concatenation<'t, A>> for BranchKind<'t, A> {
    fn from(concatenation: Concatenation<'t, A>) -> Self {
        BranchKind::Concatenation(concatenation)
    }
}

impl<'t, A> From<Repetition<'t, A>> for BranchKind<'t, A> {
    fn from(repetition: Repetition<'t, A>) -> Self {
        BranchKind::Repetition(repetition)
    }
}

impl<'t, A, T> VarianceFold<T> for BranchKind<'t, A>
where
    Alternation<'t, A>: VarianceFold<T>,
    Concatenation<'t, A>: VarianceFold<T>,
    Repetition<'t, A>: VarianceFold<T>,
    T: Invariant,
{
    fn fold(&self, terms: Vec<T::Term>) -> Option<T::Term> {
        use BranchKind::{Alternation, Concatenation, Repetition};

        match self {
            Alternation(alternation) => alternation.fold(terms),
            Concatenation(concatenation) => concatenation.fold(terms),
            Repetition(repetition) => repetition.fold(terms),
        }
    }

    fn finalize(&self, term: T::Term) -> T::Term {
        use BranchKind::{Alternation, Concatenation, Repetition};

        match self {
            Alternation(alternation) => alternation.finalize(term),
            Concatenation(concatenation) => concatenation.finalize(term),
            Repetition(repetition) => repetition.finalize(term),
        }
    }
}

trait Unroot<A> {
    type Output;

    fn unroot(&mut self, annotation: &mut A) -> Self::Output;
}

#[derive(Clone, Debug)]
pub enum LeafKind<'t> {
    Class(Class),
    Literal(Literal<'t>),
    Separator(Separator),
    Wildcard(Wildcard),
}

impl<'t> LeafKind<'t> {
    pub fn into_owned(self) -> LeafKind<'static> {
        match self {
            LeafKind::Class(class) => LeafKind::Class(class),
            LeafKind::Literal(literal) => LeafKind::Literal(literal.into_owned()),
            LeafKind::Separator(separator) => LeafKind::Separator(separator),
            LeafKind::Wildcard(wildcard) => LeafKind::Wildcard(wildcard),
        }
    }

    pub fn boundary(&self) -> Option<Boundary> {
        match self {
            LeafKind::Separator(_) => Some(Boundary::Separator),
            LeafKind::Wildcard(Wildcard::Tree { .. }) => Some(Boundary::Component),
            _ => None,
        }
    }

    pub fn is_rooting(&self) -> bool {
        matches!(
            self,
            LeafKind::Separator(_) | LeafKind::Wildcard(Wildcard::Tree { has_root: true })
        )
    }

    pub fn is_capturing(&self) -> bool {
        matches!(self, LeafKind::Class(_) | LeafKind::Wildcard(_))
    }
}

impl From<Class> for LeafKind<'static> {
    fn from(class: Class) -> Self {
        LeafKind::Class(class)
    }
}

impl<'t> From<Literal<'t>> for LeafKind<'t> {
    fn from(literal: Literal<'t>) -> Self {
        LeafKind::Literal(literal)
    }
}

impl From<Separator> for LeafKind<'static> {
    fn from(separator: Separator) -> Self {
        LeafKind::Separator(separator)
    }
}

impl From<Wildcard> for LeafKind<'static> {
    fn from(wildcard: Wildcard) -> Self {
        LeafKind::Wildcard(wildcard)
    }
}

impl<'t, A> Unroot<A> for LeafKind<'t>
where
    Wildcard: Unroot<A>,
    <Wildcard as Unroot<A>>::Output: Default,
{
    type Output = <Wildcard as Unroot<A>>::Output;

    fn unroot(&mut self, annotation: &mut A) -> Self::Output {
        match self {
            LeafKind::Wildcard(wildcard) => Unroot::unroot(wildcard, annotation),
            _ => Default::default(),
        }
    }
}

impl<'t, T> VarianceTerm<T> for LeafKind<'t>
where
    Class: VarianceTerm<T>,
    Literal<'t>: VarianceTerm<T>,
    Separator: VarianceTerm<T>,
    Wildcard: VarianceTerm<T>,
    T: Invariant,
{
    fn term(&self) -> T::Term {
        use LeafKind::{Class, Literal, Separator, Wildcard};

        match self {
            Class(class) => class.term(),
            Literal(literal) => literal.term(),
            Separator(separator) => separator.term(),
            Wildcard(wildcard) => wildcard.term(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Alternation<'t, A>(Vec<Token<'t, A>>);

impl<'t, A> Alternation<'t, A> {
    pub fn tokens(&self) -> &[Token<'t, A>] {
        &self.0
    }
}

impl<'t, A> BranchComposition<'t> for Alternation<'t, A> {
    type Annotation = A;
    type BranchData = ();

    fn compose(_: Self::BranchData, tokens: Vec<Token<'t, Self::Annotation>>) -> Result<Self, ()> {
        Ok(Alternation(tokens))
    }

    fn decompose(self) -> (Self::BranchData, Vec<Token<'t, Self::Annotation>>) {
        ((), self.0)
    }
}

impl<'t, A> From<Vec<Token<'t, A>>> for Alternation<'t, A> {
    fn from(tokens: Vec<Token<'t, A>>) -> Self {
        Alternation(tokens)
    }
}

impl<'t, A> VarianceFold<Depth> for Alternation<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Depth>>) -> Option<InvariantTerm<Depth>> {
        terms.into_iter().reduce(ops::disjunction)
    }
}

impl<'t, A> VarianceFold<Size> for Alternation<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Size>>) -> Option<InvariantTerm<Size>> {
        terms.into_iter().reduce(ops::disjunction)
    }
}

impl<'t, A> VarianceFold<Text<'t>> for Alternation<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Text<'t>>>) -> Option<InvariantTerm<Text<'t>>> {
        terms.into_iter().reduce(ops::disjunction)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Archetype {
    Character(char),
    // TODO: A range archetype spans Unicode code points. This should be clearly documented and
    //       should elegantly handle Unicode arguments that cannot be represented this way. For
    //       example, what happens if a user specifies a range between two grapheme clusters that
    //       each require more than one 32-bit code point?
    //
    //       It may be reasonable to restrict ranges to ASCII, though that isn't strictly
    //       necessary. Either way, support for predefined classes will be important. For example,
    //       it isn't yet possible to select classes like `{greek}` or `{flag}`. Such support may
    //       relieve the limitations of a code point range.
    Range(char, char),
}

impl From<char> for Archetype {
    fn from(literal: char) -> Self {
        Archetype::Character(literal)
    }
}

impl From<(char, char)> for Archetype {
    fn from(range: (char, char)) -> Self {
        Archetype::Range(range.0, range.1)
    }
}

impl VarianceTerm<Size> for Archetype {
    fn term(&self) -> InvariantTerm<Size> {
        // TODO: Examine the archetype instead of blindly assuming a constant size. This becomes
        //       especially important if character classes gain support for named classes, as these
        //       may contain graphemes that cannot be encoded as a single code point. See related
        //       comments on `Archetype::Range`.
        // This is pessimistic and assumes that the code point will require four bytes when encoded
        // as UTF-8. This is possible, but most commonly only one or two bytes will be required.
        Variance::Invariant(4.into())
    }
}

impl<'t> VarianceTerm<Text<'t>> for Archetype {
    fn term(&self) -> InvariantTerm<Text<'t>> {
        match self {
            Archetype::Character(x) => {
                if PATHS_ARE_CASE_INSENSITIVE {
                    Variance::Variant(Boundedness::BOUNDED)
                }
                else {
                    Variance::Invariant(*x)
                }
            },
            Archetype::Range(a, b) => {
                if (a != b) || PATHS_ARE_CASE_INSENSITIVE {
                    Variance::Variant(Boundedness::BOUNDED)
                }
                else {
                    Variance::Invariant(*a)
                }
            },
        }
        .map_invariant(|invariant| invariant.to_string().into_nominal_text())
    }
}

#[derive(Clone, Debug)]
pub struct Class {
    is_negated: bool,
    archetypes: Vec<Archetype>,
}

impl Class {
    pub fn archetypes(&self) -> &[Archetype] {
        &self.archetypes
    }

    pub fn is_negated(&self) -> bool {
        self.is_negated
    }

    fn fold<T, F>(&self, f: F) -> TokenVariance<T>
    where
        Archetype: VarianceTerm<T>,
        T: Invariant,
        F: FnMut(T::Term, T::Term) -> T::Term,
    {
        T::Term::finalize(
            self.archetypes()
                .iter()
                .map(Archetype::term)
                .reduce(f)
                .unwrap_or_else(Zero::zero),
        )
    }
}

impl VarianceTerm<Breadth> for Class {
    fn term(&self) -> InvariantTerm<Breadth> {
        Variance::zero()
    }
}

impl VarianceTerm<Depth> for Class {
    fn term(&self) -> InvariantTerm<Depth> {
        Zero::zero()
    }
}

impl VarianceTerm<Size> for Class {
    fn term(&self) -> InvariantTerm<Size> {
        self.fold(ops::disjunction)
    }
}

impl<'t> VarianceTerm<Text<'t>> for Class {
    fn term(&self) -> InvariantTerm<Text<'t>> {
        if self.is_negated {
            // It is not feasible to encode a character class that matches all UTF-8 text and
            // therefore nothing when negated, and so a character class must be variant if it is
            // negated and is furthermore assumed to be bounded.
            Variance::Variant(Boundedness::BOUNDED)
        }
        else {
            // TODO: This ignores casing groups, such as in the pattern `[aA]`.
            self.fold(ops::disjunction)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Concatenation<'t, A>(Vec<Token<'t, A>>);

impl<'t, A> Concatenation<'t, A> {
    pub fn tokens(&self) -> &[Token<'t, A>] {
        &self.0
    }
}

impl<'t, A> BranchComposition<'t> for Concatenation<'t, A> {
    type Annotation = A;
    type BranchData = ();

    fn compose(_: Self::BranchData, tokens: Vec<Token<'t, Self::Annotation>>) -> Result<Self, ()> {
        Ok(Concatenation(tokens))
    }

    fn decompose(self) -> (Self::BranchData, Vec<Token<'t, Self::Annotation>>) {
        ((), self.0)
    }
}

impl<'t, A> From<Vec<Token<'t, A>>> for Concatenation<'t, A> {
    fn from(tokens: Vec<Token<'t, A>>) -> Self {
        Concatenation(tokens)
    }
}

impl<'t, A> VarianceFold<Depth> for Concatenation<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Depth>>) -> Option<InvariantTerm<Depth>> {
        terms.into_iter().reduce(ops::conjunction)
    }
}

impl<'t, A> VarianceFold<Size> for Concatenation<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Size>>) -> Option<InvariantTerm<Size>> {
        terms.into_iter().reduce(ops::conjunction)
    }
}

impl<'t, A> VarianceFold<Text<'t>> for Concatenation<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Text<'t>>>) -> Option<InvariantTerm<Text<'t>>> {
        terms.into_iter().reduce(ops::conjunction)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Evaluation {
    Eager,
    Lazy,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal<'t> {
    text: Cow<'t, str>,
    is_case_insensitive: bool,
}

impl Literal<'static> {
    const EMPTY: Self = Literal {
        text: Cow::Borrowed(""),
        is_case_insensitive: false,
    };
}

impl<'t> Literal<'t> {
    pub fn into_owned(self) -> Literal<'static> {
        let Literal {
            text,
            is_case_insensitive,
        } = self;
        Literal {
            text: text.into_owned().into(),
            is_case_insensitive,
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    fn variance(&self) -> Variance<&Cow<'t, str>> {
        if self.has_variant_casing() {
            Variance::Variant(Boundedness::BOUNDED)
        }
        else {
            Variance::Invariant(&self.text)
        }
    }

    pub fn is_case_insensitive(&self) -> bool {
        self.is_case_insensitive
    }

    pub fn has_variant_casing(&self) -> bool {
        // If path case sensitivity agrees with the literal case sensitivity, then the literal is
        // not variant. Otherwise, the literal is variant if it contains characters with casing.
        (PATHS_ARE_CASE_INSENSITIVE != self.is_case_insensitive) && self.text.has_casing()
    }
}

impl<'t> VarianceTerm<Breadth> for Literal<'t> {
    fn term(&self) -> InvariantTerm<Breadth> {
        Variance::zero()
    }
}

impl<'t> VarianceTerm<Depth> for Literal<'t> {
    fn term(&self) -> InvariantTerm<Depth> {
        Zero::zero()
    }
}

impl<'t> VarianceTerm<Size> for Literal<'t> {
    fn term(&self) -> InvariantTerm<Size> {
        // TODO: This assumes that the size of graphemes in a casing set are the same. Is that
        //       correct?
        Variance::Invariant(self.text.len().into())
    }
}

impl<'t> VarianceTerm<Text<'t>> for Literal<'t> {
    fn term(&self) -> InvariantTerm<Text<'t>> {
        self.variance()
            .map_invariant(|invariant| invariant.clone().into_nominal_text())
    }
}

// TODO: The `VarianceFold` implementations for `Repetition` reimplement concatenation in `fold`.
//       Moreover, `Repetition`s have only one token, so `fold` can simply forward its term. Can
//       `VarianceFold` be decomposed in a way that avoids this redundancy? Note too that no other
//       token (at time of writing) has an interesting `finalize` implementation.
#[derive(Clone, Debug)]
pub struct Repetition<'t, A> {
    token: Box<Token<'t, A>>,
    lower: usize,
    // This representation is not ideal, as it does not statically enforce the invariant that the
    // upper bound is greater than or equal to the lower bound. For example, this field could
    // instead be a summand. However, tokens must closely resemble their glob expression
    // representations so that errors in expressions can be deferred and presented more clearly.
    // Failures in the parser are difficult to describe.
    upper: Option<usize>,
}

impl<'t, A> Repetition<'t, A> {
    pub fn token(&self) -> &Token<'t, A> {
        &self.token
    }

    pub(crate) fn bound_specification(&self) -> (usize, Option<usize>) {
        (self.lower, self.upper)
    }

    pub fn variance(&self) -> NaturalRange {
        self.bound_specification().into()
    }
}

impl<'t, A> BranchComposition<'t> for Repetition<'t, A> {
    type Annotation = A;
    type BranchData = NaturalRange;

    fn compose(
        variance: Self::BranchData,
        tokens: Vec<Token<'t, Self::Annotation>>,
    ) -> Result<Self, ()> {
        tokens.into_iter().next().ok_or(()).map(|token| Repetition {
            token: Box::new(token),
            lower: variance.lower().into_usize(),
            upper: variance.upper().into_usize(),
        })
    }

    fn decompose(self) -> (Self::BranchData, Vec<Token<'t, Self::Annotation>>) {
        let variance = self.variance();
        (variance, vec![*self.token])
    }
}

impl<'t, A> VarianceFold<Depth> for Repetition<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Depth>>) -> Option<InvariantTerm<Depth>> {
        terms.into_iter().reduce(ops::conjunction)
    }

    fn finalize(&self, term: InvariantTerm<Depth>) -> InvariantTerm<Depth> {
        ops::product(term, self.variance())
    }
}

impl<'t, A> VarianceFold<Size> for Repetition<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Size>>) -> Option<InvariantTerm<Size>> {
        terms.into_iter().reduce(ops::conjunction)
    }

    fn finalize(&self, term: InvariantTerm<Size>) -> InvariantTerm<Size> {
        ops::product(term, self.variance())
    }
}

impl<'t, A> VarianceFold<Text<'t>> for Repetition<'t, A> {
    fn fold(&self, terms: Vec<InvariantTerm<Text<'t>>>) -> Option<InvariantTerm<Text<'t>>> {
        terms.into_iter().reduce(ops::conjunction)
    }

    fn finalize(&self, term: InvariantTerm<Text<'t>>) -> InvariantTerm<Text<'t>> {
        ops::product(term, self.variance())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Separator;

impl Separator {
    const INVARIANT_TEXT: &'static str = MAIN_SEPARATOR_STR;
}

impl VarianceTerm<Breadth> for Separator {
    fn term(&self) -> InvariantTerm<Breadth> {
        Variance::zero()
    }
}

impl VarianceTerm<Depth> for Separator {
    fn term(&self) -> InvariantTerm<Depth> {
        One::one()
    }
}

impl VarianceTerm<Size> for Separator {
    fn term(&self) -> InvariantTerm<Size> {
        // TODO: This is incorrect. The compiled regular expression may ignore a terminating
        //       separator, in which case the size is a bounded range.
        Variance::Invariant(Separator::INVARIANT_TEXT.len().into())
    }
}

impl<'t> VarianceTerm<Text<'t>> for Separator {
    fn term(&self) -> InvariantTerm<Text<'t>> {
        Variance::Invariant(Separator::INVARIANT_TEXT.into_structural_text())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Wildcard {
    One,
    ZeroOrMore(Evaluation),
    Tree { has_root: bool },
}

impl Wildcard {
    fn unroot(&mut self) -> bool {
        match self {
            Wildcard::Tree { has_root } => mem::replace(has_root, false),
            _ => false,
        }
    }
}

impl<A> Unroot<A> for Wildcard
where
    A: Spanned,
{
    type Output = usize;

    fn unroot(&mut self, annotation: &mut A) -> Self::Output {
        if Wildcard::unroot(self) {
            // Move the beginning of the span in the annotation forward to dissociate the token
            // from any separator in the expression.
            let n = ROOT_SEPARATOR_EXPRESSION.len();
            let span = annotation.span_mut();
            span.0 = span.0.saturating_add(n);
            span.1 = span.1.saturating_sub(n);
            n
        }
        else {
            0
        }
    }
}

impl VarianceTerm<Breadth> for Wildcard {
    fn term(&self) -> InvariantTerm<Breadth> {
        match self {
            Wildcard::One => Variance::zero(),
            _ => Variance::unbounded(),
        }
    }
}

impl VarianceTerm<Depth> for Wildcard {
    fn term(&self) -> InvariantTerm<Depth> {
        match self {
            Wildcard::Tree { .. } => BoundaryTerm::unbounded(),
            _ => Zero::zero(),
        }
    }
}

impl VarianceTerm<Size> for Wildcard {
    fn term(&self) -> InvariantTerm<Size> {
        match self {
            // TODO: This is a bit pessimistic and, more importantly, incorrect! Clarity is needed
            //       on what exactly an exactly-one wildcard matches in text. If it is a grapheme,
            //       then size can vary wildly (though there is probably an upper bound that
            //       depends on the version of Unicode). Use an appropriate range here.
            //Wildcard::One => Variance::Variant(Size::from(1).into_lower_bound()),
            Wildcard::One => Variance::Invariant(Size::from(4)),
            _ => Variance::unbounded(),
        }
    }
}

impl<'t> VarianceTerm<Text<'t>> for Wildcard {
    fn term(&self) -> TokenVariance<Text<'t>> {
        Variance::unbounded()
    }
}

#[derive(Clone, Debug)]
pub struct LiteralSequence<'i, 't>(Vec<&'i Literal<'t>>);

impl<'i, 't> LiteralSequence<'i, 't> {
    pub fn literals(&self) -> &[&'i Literal<'t>] {
        self.0.as_ref()
    }

    pub fn text(&self) -> Cow<'t, str> {
        if self.literals().len() == 1 {
            self.literals().first().unwrap().text.clone()
        }
        else {
            self.literals()
                .iter()
                .map(|literal| &literal.text)
                .join("")
                .into()
        }
    }

    #[cfg(any(unix, windows))]
    pub fn is_semantic_literal(&self) -> bool {
        matches!(self.text().as_ref(), "." | "..")
    }

    #[cfg(not(any(unix, windows)))]
    pub fn is_semantic_literal(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Component<'i, 't, A>(&'i [Token<'t, A>]);

impl<'i, 't, A> Component<'i, 't, A> {
    pub fn tokens(&self) -> &'i [Token<'t, A>] {
        self.0
    }

    pub fn literal(&self) -> Option<LiteralSequence<'i, 't>> {
        if self.tokens().is_empty() {
            None
        }
        else {
            self.tokens().iter().all(Token::is_literal).then(|| {
                LiteralSequence(self.tokens().iter().filter_map(Token::as_literal).collect())
            })
        }
    }
}

impl<'i, 't, A> Clone for Component<'i, 't, A> {
    fn clone(&self) -> Self {
        Component(self.0)
    }
}

impl<'i, 't, A> ConcatenationTree<'t> for Component<'i, 't, A> {
    type Annotation = A;

    fn concatenation(&self) -> &[Token<'t, Self::Annotation>] {
        self.tokens()
    }
}

pub fn any<'t, I>(trees: I) -> Token<'t, ()>
where
    I: IntoIterator,
    I::Item: TokenTree<'t>,
{
    Token {
        topology: BranchKind::from(Alternation(
            trees
                .into_iter()
                .map(|tree| tree.into_token().fold_map(|_| ()))
                .collect(),
        ))
        .into(),
        annotation: (),
    }
}

fn components<'i, 't, A>(tokens: &'i [Token<'t, A>]) -> impl Iterator<Item = Component<'i, 't, A>> {
    tokens.iter().enumerate().peekable().batching(|batch| {
        let mut first = batch.next();
        while matches!(
            first.and_then(|(_, token)| token.boundary()),
            Some(Boundary::Separator)
        ) {
            first = batch.next();
        }
        first.map(|(start, token)| match token.as_wildcard() {
            Some(Wildcard::Tree { .. }) => Component(slice::from_ref(token)),
            _ => {
                let end = if let Some((end, _)) = batch
                    .peeking_take_while(|(_, token)| token.boundary().is_none())
                    .last()
                {
                    end
                }
                else {
                    start
                };
                Component(&tokens[start..=end])
            },
        })
    })
}

#[cfg(test)]
pub mod harness {
    use std::path::Path;

    use crate::token::{TokenTree, Tokenized};

    pub fn assert_tokenized_invariant_path_prefix_eq<A>(
        tokenized: Tokenized<'_, A>,
        expected: impl AsRef<Path>,
    ) -> Tokenized<'_, A> {
        let (_, text) = tokenized.as_token().invariant_text_prefix();
        let text = Path::new(&text);
        let expected = expected.as_ref();
        assert!(
            text == expected,
            "`Token::invariant_text_prefix` as path is `{}`, but expected `{}`: in `Tokenized`: `{}`",
            text.display(),
            expected.display(),
            tokenized.expression(),
        );
        tokenized
    }
}

// TODO: Test also against token trees constructed directly from tokens rather than the parser.
#[cfg(test)]
mod tests {
    use expect_macro::expect;
    use rstest::{fixture, rstest};
    use std::path::Path;

    use crate::token::{ExpressionMetadata, Token, TokenTree, Tokenized, harness, parse};

    // This fixture constructs a `Tokenized` with the following tokens in its (filtered)
    // concatenation:
    //
    // | Index | `Literal` | `Wildcard` |
    // |-------|-----------|------------|
    // |     0 | `..`      | `**`       |
    // |     1 | `foo`     | `**`       |
    // |     2 | `bar`     | `*`        |
    // |     3 | `baz`     |            |
    // |     4 | `qux`     |            |
    #[fixture]
    fn flag_literal_wildcard_concatenation() -> Tokenized<'static, ExpressionMetadata> {
        parse::harness::assert_parse_expression_is_ok("(?-i)../foo/(?i)**/bar/**(?-i)/baz/*(?i)qux")
    }

    #[rstest]
    #[case(0, false)] // `..`
    #[case(1, false)] // `foo`
    #[case(2, true)] // `bar`
    #[case(3, false)] // `baz`
    #[case(4, true)] // `qux`
    fn parse_expression_literal_case_insensitivity_eq(
        flag_literal_wildcard_concatenation: Tokenized<'static, ExpressionMetadata>,
        #[case] index: usize,
        #[case] expected: bool,
    ) {
        let literal = expect!(
            flag_literal_wildcard_concatenation
                .as_token()
                .concatenation()
                .iter()
                .filter_map(Token::as_literal)
                .nth(index),
            "`Literal` at index {} is `None`, but expected `Some`",
            index,
        );
        let is_case_insensitive = literal.is_case_insensitive;
        assert!(
            is_case_insensitive == expected,
            "case insensitivity of literal `{}` at index {} is `{}`, but expected `{}`:\
            expression `{}`",
            literal.text.as_ref(),
            index,
            is_case_insensitive,
            expected,
            flag_literal_wildcard_concatenation.expression(),
        );
    }

    #[rstest]
    #[case("/a/b", "/a/b")]
    #[case("a/b", "a/b")]
    #[case("a/*", "a/")]
    #[case("a/*b", "a/")]
    #[case("a/b*", "a/")]
    #[case("a/b/*/c", "a/b/")]
    #[case("**", "")]
    #[case("a*", "")]
    #[case("*/b", "")]
    #[case("a?/b", "")]
    #[cfg_attr(unix, case("../foo/(?i)bar/(?-i)baz", "../foo/"))]
    #[cfg_attr(windows, case("../foo/(?i)bar/(?-i)baz", "../foo/bar/"))]
    fn parse_expression_invariant_path_prefix_eq(
        #[case] expression: &str,
        #[case] expected: impl AsRef<Path>,
    ) {
        harness::assert_tokenized_invariant_path_prefix_eq(
            parse::harness::assert_parse_expression_is_ok(expression),
            expected,
        );
    }
}
