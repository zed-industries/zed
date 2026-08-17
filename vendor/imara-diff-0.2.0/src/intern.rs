use std::hash::{BuildHasher as _, Hash};
use std::ops::Index;

use hashbrown::hash_table::{Entry, HashTable};
use hashbrown::DefaultHashBuilder as RandomState;

/// A token represented as an interned integer.
///
/// A token represents the smallest possible unit of change during a diff.
/// For text this is usually a line, a word or a single character.
/// All [algorithms](crate::Algorithm) operate on interned tokens instead
/// of using the token data directly.
/// This allows for much better performance by amortizing the cost of hashing/equality.
///
/// While you can intern tokens yourself it is strongly recommended to use [`InternedInput`] module.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Token(pub u32);

impl From<u32> for Token {
    fn from(token: u32) -> Self {
        Token(token)
    }
}

impl From<Token> for u32 {
    fn from(token: Token) -> Self {
        token.0
    }
}

pub trait TokenSource {
    type Token: Hash + Eq;
    type Tokenizer: Iterator<Item = Self::Token>;
    fn tokenize(&self) -> Self::Tokenizer;
    fn estimate_tokens(&self) -> u32;
}

/// Two lists of interned [tokens](crate::intern::Token) that a [`Diff`](crate::Diff) can be computed from.
///
/// A token represents the smallest possible unit of change during a diff.
/// For text this is usually a line, a word or a single character.
/// All [algorithms](crate::Algorithm) operate on interned tokens instead
/// of using the token data directly.
/// This allows for much better performance by amortizing the cost of hashing/equality.
///
/// While you can intern tokens yourself it is strongly recommended to use [`InternedInput`] module.
#[derive(Default)]
pub struct InternedInput<T> {
    pub before: Vec<Token>,
    pub after: Vec<Token>,
    pub interner: Interner<T>,
}

impl<T> InternedInput<T> {
    pub fn clear(&mut self) {
        self.before.clear();
        self.after.clear();
        self.interner.clear();
    }
}

impl<T: Eq + Hash> InternedInput<T> {
    pub fn new<I: TokenSource<Token = T>>(before: I, after: I) -> Self {
        let token_estimate_before = before.estimate_tokens() as usize;
        let token_estimate_after = after.estimate_tokens() as usize;
        let mut res = Self {
            before: Vec::with_capacity(token_estimate_before),
            after: Vec::with_capacity(token_estimate_after),
            interner: Interner::new(token_estimate_before + token_estimate_after),
        };
        res.update_before(before.tokenize());
        res.update_after(after.tokenize());
        res
    }

    /// Create an Interner with an intial capacity calculated by calling
    /// [`estimate_tokens`](crate::intern::TokenSource::estimate_tokens) methods of `before` and `after`
    pub fn reserve_for_token_source<S: TokenSource<Token = T> + ?Sized>(
        &mut self,
        before: &S,
        after: &S,
    ) {
        self.reserve(before.estimate_tokens(), after.estimate_tokens())
    }

    pub fn reserve(&mut self, capacity_before: u32, capacity_after: u32) {
        self.before.reserve(capacity_before as usize);
        self.after.reserve(capacity_after as usize);
        self.interner
            .reserve(capacity_before as usize + capacity_after as usize);
    }

    /// replaces `self.before` with the interned Tokens yielded by `input`
    /// Note that this does not erase any tokens from the interner and might therefore be considered
    /// a memory leak. If this function is called often over a long_running process
    /// consider clearing the interner with [`clear`](crate::intern::Interner::clear).
    pub fn update_before(&mut self, input: impl Iterator<Item = T>) {
        self.before.clear();
        self.before
            .extend(input.map(|token| self.interner.intern(token)));
    }

    /// replaces `self.before` with the interned Tokens yielded by `input`
    /// Note that this does not erase any tokens from the interner and might therefore be considered
    /// a memory leak. If this function is called often over a long_running process
    /// consider clearing the interner with [`clear`](crate::intern::Interner::clear) or
    /// [`erase_tokens_after`](crate::intern::Interner::erase_tokens_after).
    pub fn update_after(&mut self, input: impl Iterator<Item = T>) {
        self.after.clear();
        self.after
            .extend(input.map(|token| self.interner.intern(token)));
    }
}

/// An interner that allows for fast access of tokens produced by a [`TokenSource`].
#[derive(Default)]
pub struct Interner<T> {
    tokens: Vec<T>,
    table: HashTable<Token>,
    hasher: RandomState,
}

impl<T> Interner<T> {
    /// Create an Interner with an initial capacity calculated by summing the results of calling
    /// [`estimate_tokens`](crate::intern::TokenSource::estimate_tokens) methods of `before` and `after`.
    pub fn new_for_token_source<S: TokenSource<Token = T>>(before: &S, after: &S) -> Self {
        Self::new(before.estimate_tokens() as usize + after.estimate_tokens() as usize)
    }

    /// Create an Interner with initial capacity `capacity`.
    pub fn new(capacity: usize) -> Interner<T> {
        Interner {
            tokens: Vec::with_capacity(capacity),
            table: HashTable::with_capacity(capacity),
            hasher: RandomState::default(),
        }
    }

    /// Remove all interned tokens.
    pub fn clear(&mut self) {
        self.table.clear();
        self.tokens.clear();
    }

    /// Returns to total number of **distinct** tokens currently interned.
    pub fn num_tokens(&self) -> u32 {
        self.tokens.len() as u32
    }
}

impl<T: Hash + Eq> Interner<T> {
    /// Create an Interner with an intial capacity calculated by calling
    /// [`estimate_tokens`](crate::intern::TokenSource::estimate_tokens) methods of `before` and `after`
    pub fn reserve_for_token_source<S: TokenSource<Token = T>>(&mut self, before: &S, after: &S) {
        self.reserve(before.estimate_tokens() as usize + after.estimate_tokens() as usize)
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.table.reserve(capacity, |&token| {
            self.hasher.hash_one(&self.tokens[token.0 as usize])
        });
        self.tokens.reserve(capacity);
    }

    /// Intern `token` and return a the interned integer.
    pub fn intern(&mut self, token: T) -> Token {
        let hash = self.hasher.hash_one(&token);
        match self.table.entry(
            hash,
            |&it| self.tokens[it.0 as usize] == token,
            |&token| self.hasher.hash_one(&self.tokens[token.0 as usize]),
        ) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let interned = Token(self.tokens.len() as u32);
                entry.insert(interned);
                self.tokens.push(token);
                interned
            }
        }
    }

    /// Erases `first_erased_token` and any tokens interned afterward from the interner.
    pub fn erase_tokens_after(&mut self, first_erased_token: Token) {
        assert!(first_erased_token.0 <= self.tokens.len() as u32);
        let retained = first_erased_token.0 as usize;
        let erased = self.tokens.len() - retained;
        if retained <= erased {
            self.table.clear();
            for (i, token) in self.tokens[0..retained].iter().enumerate() {
                let hash = self.hasher.hash_one(token);
                self.table.insert_unique(hash, Token(i as u32), |&token| {
                    self.hasher.hash_one(&self.tokens[token.0 as usize])
                });
            }
        } else {
            for (i, token) in self.tokens[retained..].iter().enumerate() {
                let hash = self.hasher.hash_one(token);
                match self
                    .table
                    .find_entry(hash, |token| token.0 == (retained + i) as u32)
                {
                    Ok(occupied) => drop(occupied.remove()),
                    Err(_absent) => unreachable!(),
                }
            }
        }
        self.tokens.truncate(first_erased_token.0 as usize);
    }
}

impl<T> Index<Token> for Interner<T> {
    type Output = T;
    fn index(&self, index: Token) -> &Self::Output {
        &self.tokens[index.0 as usize]
    }
}
