use std::cmp;
use std::collections::VecDeque;

use crate::filter::{
    CancelWalk, HierarchicalIterator, Isomeric, SeparatingFilterInput, Separation, TreeResidue,
};
use crate::token::variance::natural::NaturalRange;
use crate::token::{
    Alternation, BranchKind, Composition, Concatenation, LeafKind, Repetition, Token,
    TokenTopology, TokenTree,
};

type TokenFeed<'i, 't, A> = (TokenEntry<'i, 't, A>, TreeResidue<TokenEntry<'i, 't, A>>);

impl<'i, 't, A> Isomeric for TokenFeed<'i, 't, A> {
    type Substituent<'a>
        = &'a TokenEntry<'i, 't, A>
    where
        Self: 'a;

    fn substituent(separation: &Separation<Self>) -> Self::Substituent<'_> {
        match separation {
            Separation::Filtrate(filtrate) => filtrate.get(),
            Separation::Residue(residue) => residue.get().get(),
        }
    }
}

// Fold APIs are batched: folds operate against a complete and ordered sequence of terms rather
// than an accumulator and a single subsequent term. This incurs a performance penalty, but
// supports aggregations that must consider the complete sequence of terms without the need for
// additional state in `Fold` implementers or `Term`s.
pub trait Fold<'t, A> {
    type Sequencer: Sequencer;
    type Term;

    fn sequencer() -> Self::Sequencer;

    fn initialize(&mut self, _branch: &BranchKind<'t, A>) -> Option<Self::Term> {
        None
    }

    fn fold(&mut self, branch: &BranchKind<'t, A>, terms: Vec<Self::Term>) -> Option<Self::Term>;

    fn finalize(&mut self, _branch: &BranchKind<'t, A>, term: Self::Term) -> Self::Term {
        term
    }

    fn term(&mut self, leaf: &LeafKind<'t>) -> Self::Term;
}

impl<'t, A, F> Fold<'t, A> for &mut F
where
    F: Fold<'t, A>,
{
    type Sequencer = F::Sequencer;
    type Term = F::Term;

    fn sequencer() -> Self::Sequencer {
        F::sequencer()
    }

    fn initialize(&mut self, branch: &BranchKind<'t, A>) -> Option<Self::Term> {
        F::initialize(self, branch)
    }

    fn fold(&mut self, branch: &BranchKind<'t, A>, terms: Vec<Self::Term>) -> Option<Self::Term> {
        F::fold(self, branch, terms)
    }

    fn finalize(&mut self, branch: &BranchKind<'t, A>, term: Self::Term) -> Self::Term {
        F::finalize(self, branch, term)
    }

    fn term(&mut self, leaf: &LeafKind<'t>) -> Self::Term {
        F::term(self, leaf)
    }
}

pub trait FoldMap<'t, 'o, A> {
    type Annotation;

    fn fold(
        &mut self,
        annotation: A,
        branch: BranchFold,
        tokens: Vec<Token<'o, Self::Annotation>>,
    ) -> Option<Token<'o, Self::Annotation>>;

    fn map(&mut self, annotation: A, leaf: LeafKind<'t>) -> Token<'o, Self::Annotation>;
}

impl<'t, A, B, F> FoldMap<'t, 't, A> for F
where
    F: FnMut(A) -> B,
{
    type Annotation = B;

    fn fold(
        &mut self,
        annotation: A,
        branch: BranchFold,
        tokens: Vec<Token<'t, Self::Annotation>>,
    ) -> Option<Token<'t, Self::Annotation>> {
        branch
            .fold(tokens)
            .ok()
            .map(|branch| Token::new(branch, (self)(annotation)))
    }

    fn map(&mut self, annotation: A, leaf: LeafKind<'t>) -> Token<'t, Self::Annotation> {
        Token::new(leaf, (self)(annotation))
    }
}

// The data in each variant must be the same as the associated `BranchComposition::BranchData` type
// for each branch type. However, there is no way to reference this type indepenently of the input
// type parameter `A`. These types must not depend on `A`, but there is no way to enforce this in
// `BranchComposition` as written. `BranchFold` expresses these types with no explicit relationship
// to `BranchComposition`. See `BranchComposition`.
#[derive(Debug)]
pub enum BranchFold {
    Alternation(()),
    Concatenation(()),
    Repetition(NaturalRange),
}

impl BranchFold {
    pub fn fold<'t, A>(self, tokens: Vec<Token<'t, A>>) -> Result<BranchKind<'t, A>, ()> {
        match self {
            BranchFold::Alternation(data) => {
                BranchKind::compose::<Alternation<'t, A>>(data, tokens)
            },
            BranchFold::Concatenation(data) => {
                BranchKind::compose::<Concatenation<'t, A>>(data, tokens)
            },
            BranchFold::Repetition(data) => BranchKind::compose::<Repetition<'t, A>>(data, tokens),
        }
    }
}

impl<'t, A> Token<'t, A> {
    pub fn fold<F>(&self, f: F) -> Option<F::Term>
    where
        F: Fold<'t, A>,
    {
        self.fold_with_sequence(F::sequencer(), f)
    }

    fn fold_with_sequence<S, F>(&self, mut sequencer: S, f: F) -> Option<F::Term>
    where
        S: Sequencer,
        F: Fold<'t, A>,
    {
        struct TokenPath<'i, 't, A, F>
        where
            F: Fold<'t, A>,
        {
            branches: Vec<TokenBranch<'i, 't, A, F>>,
            f: F,
        }

        impl<'i, 't, A, F> TokenPath<'i, 't, A, F>
        where
            F: Fold<'t, A>,
        {
            pub fn push(&mut self, branch: &'i BranchKind<'t, A>) {
                let mut terms = VecDeque::with_capacity(branch.tokens().into_inner().len() + 1);
                if let Some(term) = self.f.initialize(branch) {
                    terms.push_front(term);
                }
                self.branches.push(TokenBranch { branch, terms });
            }

            pub fn pop(&mut self, depth: usize) {
                if let Some(n) = self.branches.len().checked_sub(depth) {
                    self.fold_n(n);
                }
            }

            pub fn fold(mut self) -> Option<F::Term> {
                self.fold_n(usize::MAX);
                self.branches
                    .pop()
                    .and_then(|branch| branch.fold(&mut self.f))
            }

            pub fn accumulate(&mut self, leaf: &'i LeafKind<'t>) -> Result<(), F::Term> {
                let term = self.f.term(leaf);
                match self.branches.last_mut() {
                    Some(branch) => {
                        branch.push(term);
                        Ok(())
                    },
                    None => Err(term),
                }
            }

            fn fold_n(&mut self, n: usize) {
                for _ in 0..cmp::min(n, self.branches.len().saturating_sub(1)) {
                    if let Some(term) = self.branches.pop().unwrap().fold(&mut self.f) {
                        self.branches.last_mut().unwrap().push(term);
                    }
                }
            }
        }

        struct TokenBranch<'i, 't, A, F>
        where
            F: Fold<'t, A>,
        {
            branch: &'i BranchKind<'t, A>,
            // A queue is used to preserve the order of terms w.r.t. to the token tree and, more
            // subtly, any initializer terms.
            terms: VecDeque<F::Term>,
        }

        impl<'i, 't, A, F> TokenBranch<'i, 't, A, F>
        where
            F: Fold<'t, A>,
        {
            fn push(&mut self, term: F::Term) {
                self.terms.push_front(term)
            }

            fn fold(self, f: &mut F) -> Option<F::Term> {
                let TokenBranch { branch, terms } = self;
                f.fold(branch, terms.into())
                    .map(|term| f.finalize(branch, term))
            }
        }

        let mut path = TokenPath {
            branches: vec![],
            f,
        };
        let mut tokens = vec![(self, 0usize)];
        while let Some((token, depth)) = tokens.pop() {
            path.pop(depth);
            match token.topology() {
                TokenTopology::Branch(branch) => {
                    tokens.extend(
                        sequencer
                            .enqueue(Parent(branch))
                            .map(|Child(token)| token)
                            .map(|token| (token, depth + 1)),
                    );
                    path.push(branch);
                },
                TokenTopology::Leaf(leaf) => {
                    if let Err(term) = path.accumulate(leaf) {
                        return Some(term);
                    }
                },
            }
        }
        path.fold()
    }

    pub fn fold_map<'o, F>(self, f: F) -> Token<'o, F::Annotation>
    where
        F: FoldMap<'t, 'o, A>,
    {
        struct TokenPath<'t, 'o, A, F>
        where
            F: FoldMap<'t, 'o, A>,
        {
            branches: Vec<TokenBranch<'t, 'o, A, F>>,
            f: F,
        }

        impl<'t, 'o, A, F> TokenPath<'t, 'o, A, F>
        where
            F: FoldMap<'t, 'o, A>,
        {
            pub fn push(&mut self, annotation: A, branch: BranchFold, hint: Option<usize>) {
                self.branches.push(TokenBranch {
                    annotation,
                    branch,
                    tokens: match hint {
                        Some(hint) => Vec::with_capacity(hint),
                        _ => vec![],
                    },
                });
            }

            pub fn pop(&mut self, depth: usize) {
                if let Some(n) = self.branches.len().checked_sub(depth) {
                    self.fold_n(n);
                }
            }

            pub fn fold(mut self) -> Option<Token<'o, F::Annotation>> {
                self.fold_n(usize::MAX);
                self.branches
                    .pop()
                    .and_then(|branch| branch.fold(&mut self.f))
            }

            pub fn map(
                &mut self,
                annotation: A,
                leaf: LeafKind<'t>,
            ) -> Result<(), Token<'o, F::Annotation>> {
                let token = self.f.map(annotation, leaf);
                match self.branches.last_mut() {
                    Some(branch) => {
                        branch.push(token);
                        Ok(())
                    },
                    None => Err(token),
                }
            }

            fn fold_n(&mut self, n: usize) {
                for _ in 0..cmp::min(n, self.branches.len().saturating_sub(1)) {
                    if let Some(token) = self.branches.pop().unwrap().fold(&mut self.f) {
                        self.branches.last_mut().unwrap().push(token);
                    }
                }
            }
        }

        struct TokenBranch<'t, 'o, A, F>
        where
            F: FoldMap<'t, 'o, A>,
        {
            annotation: A,
            branch: BranchFold,
            tokens: Vec<Token<'o, F::Annotation>>,
        }

        impl<'t, 'o, A, F> TokenBranch<'t, 'o, A, F>
        where
            F: FoldMap<'t, 'o, A>,
        {
            fn push(&mut self, token: Token<'o, F::Annotation>) {
                self.tokens.push(token)
            }

            fn fold(self, f: &mut F) -> Option<Token<'o, F::Annotation>> {
                let TokenBranch {
                    annotation,
                    branch,
                    tokens,
                } = self;
                f.fold(annotation, branch, tokens)
            }
        }

        let mut path = TokenPath {
            branches: vec![],
            f,
        };
        let mut tokens = vec![(self, 0usize)];
        while let Some((token, depth)) = tokens.pop() {
            path.pop(depth);
            match token.topology {
                TokenTopology::Branch(branch) => {
                    let (branch, children) = branch.decompose();
                    let n = children.len();
                    tokens.extend(children.into_iter().rev().map(|token| (token, depth + 1)));
                    path.push(token.annotation, branch, Some(n));
                },
                TokenTopology::Leaf(leaf) => {
                    if let Err(token) = path.map(token.annotation, leaf) {
                        return token;
                    }
                },
            }
        }
        path.fold().expect("no tokens in non-empty tree path")
    }
}

// Together with `Child`, this type prevents `BranchSequencer` implementers from enqueuing
// arbitrary tokens that are not associated with the given branch.
#[derive(Debug)]
#[repr(transparent)]
pub struct Parent<T>(T);

pub type ParentToken<'i, 't, A> = Parent<&'i BranchKind<'t, A>>;

impl<T> AsRef<T> for Parent<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<'i, 't, A> ParentToken<'i, 't, A> {
    // It may be fragile or unclear what `Item` is without specifying `Iterator`. It is also not
    // clear which common supertrait ought to specify this type.
    #[allow(clippy::implied_bounds_in_impls)]
    pub fn into_tokens(
        self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator + Iterator<Item = ChildToken<'i, 't, A>> {
        self.0.tokens().into_inner().iter().map(Child)
    }
}

// Together with `Parent`, this type prevents `BranchSequencer` implementers from enqueuing
// arbitrary tokens that are not associated with the given branch.
#[derive(Debug)]
#[repr(transparent)]
pub struct Child<T>(T);

pub type ChildToken<'i, 't, A> = Child<&'i Token<'t, A>>;

impl<T> AsRef<T> for Child<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

pub trait Sequencer {
    fn enqueue<'i, 't, A>(
        &mut self,
        parent: ParentToken<'i, 't, A>,
    ) -> impl Iterator<Item = ChildToken<'i, 't, A>>;
}

#[derive(Default)]
pub struct Forward;

impl Sequencer for Forward {
    fn enqueue<'i, 't, A>(
        &mut self,
        parent: ParentToken<'i, 't, A>,
    ) -> impl Iterator<Item = ChildToken<'i, 't, A>> {
        parent.into_tokens()
    }
}

#[derive(Default)]
pub struct Starting;

impl Sequencer for Starting {
    fn enqueue<'i, 't, A>(
        &mut self,
        parent: ParentToken<'i, 't, A>,
    ) -> impl Iterator<Item = Child<&'i Token<'t, A>>> {
        let composition = parent.as_ref().composition();
        let tokens = parent.into_tokens();
        let n = match composition {
            Composition::Conjunctive(_) => 1,
            Composition::Disjunctive(_) => tokens.len(),
        };
        tokens.take(n)
    }
}

#[derive(Default)]
pub struct Ending;

impl Sequencer for Ending {
    fn enqueue<'i, 't, A>(
        &mut self,
        parent: ParentToken<'i, 't, A>,
    ) -> impl Iterator<Item = Child<&'i Token<'t, A>>> {
        let composition = parent.as_ref().composition();
        let tokens = parent.into_tokens();
        let n = match composition {
            Composition::Conjunctive(_) => tokens.len().saturating_sub(1),
            Composition::Disjunctive(_) => 0,
        };
        tokens.skip(n)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Depth {
    pub conjunctive: usize,
    pub disjunctive: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Position {
    pub depth: Depth,
    pub branch: usize,
}

impl Position {
    // This may appear to operate in place.
    #[must_use]
    fn conjunction(self) -> Self {
        let Position {
            depth: Depth {
                conjunctive,
                disjunctive,
            },
            branch,
        } = self;
        Position {
            depth: Depth {
                conjunctive: conjunctive + 1,
                disjunctive,
            },
            branch,
        }
    }

    // This may appear to operate in place.
    #[must_use]
    fn disjunction(self, branch: usize) -> Self {
        let Position {
            depth: Depth {
                conjunctive,
                disjunctive,
            },
            ..
        } = self;
        Position {
            depth: Depth {
                conjunctive,
                disjunctive: disjunctive + 1,
            },
            branch,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WalkEntry<T> {
    pub position: Position,
    pub token: T,
}

impl<T> WalkEntry<T> {
    pub fn into_token(self) -> T {
        self.token
    }

    fn map<U, F>(self, f: F) -> WalkEntry<U>
    where
        F: FnOnce(T) -> U,
    {
        let WalkEntry { position, token } = self;
        WalkEntry {
            position,
            token: f(token),
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

type BranchEntry<'i, 't, A> = WalkEntry<&'i BranchKind<'t, A>>;

pub type TokenEntry<'i, 't, A> = WalkEntry<&'i Token<'t, A>>;

#[derive(Clone, Debug)]
struct Walk<'i, 't, A, S> {
    branch: Option<BranchEntry<'i, 't, A>>,
    tokens: VecDeque<TokenEntry<'i, 't, A>>,
    sequencer: S,
}

impl<'i, 't, A, S> CancelWalk for Walk<'i, 't, A, S> {
    fn cancel_walk_tree(&mut self) {
        self.branch.take();
    }
}

impl<'i, 't, A, S> Iterator for Walk<'i, 't, A, S>
where
    S: Sequencer,
{
    type Item = TokenEntry<'i, 't, A>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(BranchEntry { position, token }) = self.branch.take() {
            match token
                .composition()
                .map(|_| self.sequencer.enqueue(Parent(token)))
            {
                Composition::Conjunctive(tokens) => {
                    self.tokens
                        .extend(tokens.map(|child| child.0).map(|token| TokenEntry {
                            position: position.conjunction(),
                            token,
                        }))
                },
                Composition::Disjunctive(tokens) => {
                    self.tokens
                        .extend(
                            tokens
                                .map(|child| child.0)
                                .enumerate()
                                .map(|(branch, token)| TokenEntry {
                                    position: position.disjunction(branch),
                                    token,
                                }),
                        )
                },
            };
        }
        if let Some(entry) = self.tokens.pop_front() {
            if let Some(branch) = entry.token.as_branch() {
                self.branch = Some(entry.map(|_| branch));
            }
            Some(entry)
        }
        else {
            None
        }
    }
}

impl<'i, 't, A, S> SeparatingFilterInput for Walk<'i, 't, A, S>
where
    S: Sequencer,
{
    type Feed = TokenFeed<'i, 't, A>;
}

// NOTE: Tokens discarded by `sequencer` are **not** present in the feed of the output.
pub fn with_sequence<'i, 't, T, S>(
    tree: &'i T,
    sequencer: S,
) -> impl 'i + HierarchicalIterator<Feed = TokenFeed<'i, 't, T::Annotation>>
where
    't: 'i,
    T: TokenTree<'t>,
    S: 'i + Sequencer,
{
    Walk {
        branch: None,
        tokens: Some(TokenEntry {
            position: Position::default(),
            token: tree.as_token(),
        })
        .into_iter()
        .collect(),
        sequencer,
    }
}

pub fn forward<'i, 't, T>(
    tree: &'i T,
) -> impl 'i + HierarchicalIterator<Feed = TokenFeed<'i, 't, T::Annotation>>
where
    't: 'i,
    T: TokenTree<'t>,
{
    self::with_sequence(tree, Forward)
}

pub fn starting<'i, 't, T>(
    tree: &'i T,
) -> impl 'i + HierarchicalIterator<Feed = TokenFeed<'i, 't, T::Annotation>>
where
    't: 'i,
    T: 't + TokenTree<'t>,
{
    self::with_sequence(tree, Starting)
}

pub fn ending<'i, 't, T>(
    tree: &'i T,
) -> impl 'i + HierarchicalIterator<Feed = TokenFeed<'i, 't, T::Annotation>>
where
    't: 'i,
    T: 't + TokenTree<'t>,
{
    self::with_sequence(tree, Ending)
}
