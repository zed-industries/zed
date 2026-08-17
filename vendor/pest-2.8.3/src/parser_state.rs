// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! The core functionality of parsing grammar.
//! State of parser during the process of rules handling.

use alloc::borrow::{Cow, ToOwned};
use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter};
use core::num::NonZeroUsize;
use core::ops::Deref; // used in BorrowedOrRc.as_str
use core::ops::Range;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::error::{Error, ErrorVariant};
use crate::iterators::pairs::new;
use crate::iterators::{pairs, QueueableToken};
use crate::position::Position;
use crate::span::Span;
use crate::stack::Stack;
use crate::RuleType;

/// The current lookahead status of a [`ParserState`].
///
/// [`ParserState`]: struct.ParserState.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lookahead {
    /// The positive predicate, written as an ampersand &,
    /// attempts to match its inner expression.
    /// If the inner expression succeeds, parsing continues,
    /// but at the same position as the predicate —
    /// &foo ~ bar is thus a kind of "AND" statement:
    /// "the input string must match foo AND bar".
    /// If the inner expression fails,
    /// the whole expression fails too.
    Positive,
    /// The negative predicate, written as an exclamation mark !,
    /// attempts to match its inner expression.
    /// If the inner expression fails, the predicate succeeds
    /// and parsing continues at the same position as the predicate.
    /// If the inner expression succeeds, the predicate fails —
    /// !foo ~ bar is thus a kind of "NOT" statement:
    /// "the input string must match bar but NOT foo".
    Negative,
    /// No lookahead (i.e. it will consume input).
    None,
}

/// The current atomicity of a [`ParserState`].
///
/// [`ParserState`]: struct.ParserState.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Atomicity {
    /// prevents implicit whitespace: inside an atomic rule,
    /// the tilde ~ means "immediately followed by",
    /// and repetition operators (asterisk * and plus sign +)
    /// have no implicit separation. In addition, all other rules
    /// called from an atomic rule are also treated as atomic.
    /// (interior matching rules are silent)
    Atomic,
    /// The same as atomic, but inner tokens are produced as normal.
    CompoundAtomic,
    /// implicit whitespace is enabled
    NonAtomic,
}

/// Type alias to simplify specifying the return value of chained closures.
pub type ParseResult<S> = Result<S, S>;

/// Match direction for the stack. Used in `PEEK[a..b]`/`stack_match_peek_slice`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchDir {
    /// from the bottom to the top of the stack
    BottomToTop,
    /// from the top to the bottom of the stack
    TopToBottom,
}

static CALL_LIMIT: AtomicUsize = AtomicUsize::new(0);

/// Sets the maximum call limit for the parser state
/// to prevent stack overflows or excessive execution times
/// in some grammars.
/// If set, the calls are tracked as a running total
/// over all non-terminal rules that can nest closures
/// (which are passed to transform the parser state).
///
/// # Arguments
///
/// * `limit` - The maximum number of calls. If None,
///             the number of calls is unlimited.
pub fn set_call_limit(limit: Option<NonZeroUsize>) {
    CALL_LIMIT.store(limit.map(|f| f.get()).unwrap_or(0), Ordering::Relaxed);
}

static ERROR_DETAIL: AtomicBool = AtomicBool::new(false);

/// Sets whether information for more error details
/// should be collected. This is useful for debugging
/// parser errors (as it leads to more comprehensive
/// error messages), but it has a higher performance cost.
/// (hence, it's off by default)
///
/// # Arguments
///
/// * `enabled` - Whether to enable the collection for
///               more error details.
pub fn set_error_detail(enabled: bool) {
    ERROR_DETAIL.store(enabled, Ordering::Relaxed);
}

#[derive(Debug)]
struct CallLimitTracker {
    current_call_limit: Option<(usize, usize)>,
}

impl Default for CallLimitTracker {
    fn default() -> Self {
        let limit = CALL_LIMIT.load(Ordering::Relaxed);
        let current_call_limit = if limit > 0 { Some((0, limit)) } else { None };
        Self { current_call_limit }
    }
}

impl CallLimitTracker {
    fn limit_reached(&self) -> bool {
        self.current_call_limit
            .map_or(false, |(current, limit)| current >= limit)
    }

    fn increment_depth(&mut self) {
        if let Some((current, _)) = &mut self.current_call_limit {
            *current += 1;
        }
    }
}

/// Number of call stacks that may result from a sequence of rules parsing.
const CALL_STACK_INITIAL_CAPACITY: usize = 20;
/// Max (un)expected number of tokens that we may see on the parsing error position.
const EXPECTED_TOKENS_INITIAL_CAPACITY: usize = 30;
/// Max rule children number for which we'll extend calls stacks.
///
/// In case rule we're working with has too many children rules that failed in parsing,
/// we don't want to store long stacks for all of them. If rule has more than this number
/// of failed children, they all will be collapsed in a parent rule.
const CALL_STACK_CHILDREN_THRESHOLD: usize = 4;

/// Structure tracking errored parsing call (associated with specific `ParserState` function).
#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum ParseAttempt<R> {
    /// Call of `rule` errored.
    Rule(R),
    /// Call of token element (e.g., `match_string` or `match_insensitive`) errored.
    /// Works as indicator of that leaf node is not a rule. In order to get the token value we
    /// can address `ParseAttempts` `(un)expected_tokens`.
    Token,
}

impl<R> ParseAttempt<R> {
    pub fn get_rule(&self) -> Option<&R> {
        match self {
            ParseAttempt::Rule(r) => Some(r),
            ParseAttempt::Token => None,
        }
    }
}

/// Rules call stack.
/// Contains sequence of rule calls that resulted in new parsing attempt.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RulesCallStack<R> {
    /// Deepest rule caused a parsing error (ParseAttempt::Token transformed into a rule).
    pub deepest: ParseAttempt<R>,
    /// Most top rule covering `deepest`.
    pub parent: Option<R>,
}

impl<R> RulesCallStack<R> {
    fn new(deepest: ParseAttempt<R>) -> RulesCallStack<R> {
        RulesCallStack {
            deepest,
            parent: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ParsingToken {
    Sensitive { token: String },
    Insensitive { token: String },
    Range { start: char, end: char },
    BuiltInRule,
}

impl Display for ParsingToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ParsingToken::Sensitive { token } => write!(f, "{token}"),
            ParsingToken::Insensitive { token } => write!(f, "{}", token.to_uppercase()),
            ParsingToken::Range { start, end } => write!(f, "{start}..{end}"),
            ParsingToken::BuiltInRule => write!(f, "BUILTIN_RULE"),
        }
    }
}
/// A helper that provides efficient string handling without unnecessary copying.
/// We use `Arc<String>` instead of `Cow<'i, str>` to avoid copying strings when cloning the `Owned` variant, since
/// `Arc::clone` only increments a reference count. [SpanOrLiteral] needs to be [Send] and [Sync]`, so we use [Arc]
/// instead of [Rc].
///
/// (We need to clone this struct to detach it from the `&self` borrow in [SpanOrLiteral::as_borrowed_or_rc], so that
/// we can then call `self.match_string` (a `mut self` method).
#[derive(Debug, Clone)]
enum BorrowedOrArc<'i> {
    Borrowed(&'i str),
    Owned(Arc<String>),
}

/// A holder for a literal string, for use in `push_literal`. This is typically a `&'static str`, but is an owned
/// `Rc<String>` for the pest vm.
impl<'i> BorrowedOrArc<'i> {
    fn as_str<'a: 'i>(&'a self) -> &'a str {
        match self {
            BorrowedOrArc::Borrowed(s) => s,
            BorrowedOrArc::Owned(s) => s.deref(),
        }
    }
}

impl From<Cow<'static, str>> for BorrowedOrArc<'_> {
    fn from(value: Cow<'static, str>) -> Self {
        match value {
            Cow::Borrowed(s) => Self::Borrowed(s),
            Cow::Owned(s) => Self::Owned(Arc::new(s)),
        }
    }
}

#[derive(Debug, Clone)]
enum SpanOrLiteral<'i> {
    Span(Span<'i>),
    Literal(BorrowedOrArc<'i>),
}

impl<'i> SpanOrLiteral<'i> {
    #[inline]
    fn as_borrowed_or_rc(&self) -> BorrowedOrArc<'i> {
        match self {
            Self::Span(s) => BorrowedOrArc::Borrowed(s.as_str()),
            Self::Literal(s) => s.clone(),
        }
    }
}

/// Structure that tracks all the parsing attempts made on the max position.
/// We want to give an error hint about parsing rules that succeeded
/// at the farthest input position.
/// The intuition is such rules will be most likely the query user initially wanted to write.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParseAttempts<R> {
    /// Indicates whether the parsing attempts are tracked.
    enabled: bool,
    /// Vec of rule calls sequences awaiting tokens at the same `max_position`.
    /// If there are several stacks in vec, it means all those rule stacks are "equal"
    /// because their attempts occurred on the same position.
    pub call_stacks: Vec<RulesCallStack<R>>,
    /// Tokens that could be putted at `max_position`
    /// in order to get a valid grammar query.
    expected_tokens: Vec<ParsingToken>,
    /// Tokens that we've prohibited to be putted at `max_position`
    /// in order to get a valid grammar query.
    unexpected_tokens: Vec<ParsingToken>,
    /// Max position at which we were expecting to see one of `expected_tokens`.
    pub max_position: usize,
}

impl<R: RuleType> ParseAttempts<R> {
    /// Create new `ParseAttempts` instance with `call_stacks` and `expected_tokens`
    /// initialized with capacity.
    pub fn new() -> Self {
        Self {
            call_stacks: Vec::with_capacity(CALL_STACK_INITIAL_CAPACITY),
            expected_tokens: Vec::with_capacity(EXPECTED_TOKENS_INITIAL_CAPACITY),
            unexpected_tokens: Vec::with_capacity(EXPECTED_TOKENS_INITIAL_CAPACITY),
            max_position: 0,
            enabled: ERROR_DETAIL.load(Ordering::Relaxed),
        }
    }

    /// Get number of currently present call stacks.
    fn call_stacks_number(&self) -> usize {
        self.call_stacks.len()
    }

    pub fn expected_tokens(&self) -> Vec<ParsingToken> {
        self.expected_tokens
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn unexpected_tokens(&self) -> Vec<ParsingToken> {
        self.unexpected_tokens
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Retrieve call stacks.
    pub fn call_stacks(&self) -> Vec<RulesCallStack<R>> {
        self.call_stacks
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// In case we've tried to parse a rule, which start position is bigger than previous
    /// `max_position` it means that we've advanced in our parsing and found better candidate.
    ///
    /// `start_index` is:
    /// * Number of call stacks present in state at the moment current `rule` was called. The idea
    ///   is that we'd like to update only those stacks that originated from the current `rule` and
    ///   not from those that were called previously.
    /// * 0 in case we've successfully parsed some token since the moment `rule` was called.
    fn try_add_new_stack_rule(&mut self, rule: R, start_index: usize) {
        let mut non_token_call_stacks = Vec::new();
        let mut token_call_stack_met = false;
        for call_stack in self.call_stacks.iter().skip(start_index) {
            if matches!(call_stack.deepest, ParseAttempt::Token) {
                token_call_stack_met = true;
            } else {
                non_token_call_stacks.push(call_stack.clone())
            }
        }
        if token_call_stack_met && non_token_call_stacks.is_empty() {
            // If `non_token_call_stacks` is not empty we wouldn't like to add a new standalone
            // `RulesCallStack::new(ParseAttempt::Token)` (that will later be transformed into a
            // rule) as soon as it doesn't give us any useful additional info.
            non_token_call_stacks.push(RulesCallStack::new(ParseAttempt::Token));
        }
        self.call_stacks
            .splice(start_index.., non_token_call_stacks);

        let children_number_over_threshold =
            self.call_stacks_number() - start_index >= CALL_STACK_CHILDREN_THRESHOLD;
        if children_number_over_threshold {
            self.call_stacks.truncate(start_index);
            self.call_stacks
                .push(RulesCallStack::new(ParseAttempt::Rule(rule)));
        } else {
            for call_stack in self.call_stacks.iter_mut().skip(start_index) {
                if matches!(call_stack.deepest, ParseAttempt::Token) {
                    call_stack.deepest = ParseAttempt::Rule(rule);
                } else {
                    call_stack.parent = Some(rule);
                }
            }
        }
    }

    /// If `expected` flag is set to false, it means we've successfully parsed token being in the
    /// state of negative lookahead and want to track `token` in the `unexpected_tokens`. Otherwise,
    /// we want to track it the `expected_tokens`. Let's call chosen vec a `target_vec`.
    ///
    /// In case `position` is:
    /// * Equal to `max_position`, add `token` to `target_vec`,
    /// * Bigger than `max_position`, set `token` as the only new element of `target_vec`.
    #[allow(clippy::comparison_chain)]
    fn try_add_new_token(
        &mut self,
        token: ParsingToken,
        start_position: usize,
        position: usize,
        negative_lookahead: bool,
    ) {
        let target_vec_push_token = |attempts: &mut ParseAttempts<R>| {
            let target_vec = if negative_lookahead {
                &mut attempts.unexpected_tokens
            } else {
                &mut attempts.expected_tokens
            };
            target_vec.push(token);
        };

        if position > self.max_position {
            if negative_lookahead && start_position > self.max_position {
                // We encountered a sequence under negative lookahead.
                // We would like to track only first failed token in this sequence (which
                // `start_position` should be equal to `self.max_position`).
                return;
            }
            target_vec_push_token(self);

            if negative_lookahead {
                // In case of successful parsing of token under negative lookahead the only
                // thing we'd like to do is to track the token in the `unexpected_tokens`.
                return;
            }
            self.max_position = position;
            self.expected_tokens.clear();
            self.unexpected_tokens.clear();
            self.call_stacks.clear();
            self.call_stacks
                .push(RulesCallStack::new(ParseAttempt::Token));
        } else if position == self.max_position {
            target_vec_push_token(self);
            self.call_stacks
                .push(RulesCallStack::new(ParseAttempt::Token));
        }
    }

    /// Reset state in case we've successfully parsed some token in
    /// `match_string` or `match_insensetive`.
    fn nullify_expected_tokens(&mut self, new_max_position: usize) {
        self.call_stacks.clear();
        self.expected_tokens.clear();
        self.unexpected_tokens.clear();
        self.max_position = new_max_position;
    }
}

impl<R: RuleType> Default for ParseAttempts<R> {
    fn default() -> Self {
        Self::new()
    }
}

/// The complete state of a [`Parser`].
///
/// [`Parser`]: trait.Parser.html
#[derive(Debug)]
pub struct ParserState<'i, R: RuleType> {
    /// Current position from which we try to apply some parser function.
    /// Initially is 0.
    /// E.g., we are parsing `create user 'Bobby'` query, we parsed "create" via `match_insensitive`
    /// and switched our `position` from 0 to the length of "create".
    ///
    /// E.g., see `match_string` -> `self.position.match_string(string)` which updates `self.pos`.
    position: Position<'i>,
    /// Queue representing rules partially (`QueueableToken::Start`) and
    /// totally (`QueueableToken::End`) parsed. When entering rule we put it in the queue in a state
    /// of `Start` and after all its sublogic (subrules or strings) are parsed, we change it to
    /// `End` state.
    queue: Vec<QueueableToken<'i, R>>,
    /// Status set in case specific lookahead logic is used in grammar.
    /// See `Lookahead` for more information.
    lookahead: Lookahead,
    /// Rules that we HAVE expected, tried to parse, but failed.
    pos_attempts: Vec<R>,
    /// Rules that we have NOT expected, tried to parse, but failed.
    neg_attempts: Vec<R>,
    /// Max position in the query from which we've tried to parse some rule but failed.
    attempt_pos: usize,
    /// Current atomicity status. For more information see `Atomicity`.
    atomicity: Atomicity,
    /// Helper structure tracking `Stack` status (used in case grammar contains stack PUSH/POP
    /// invocations).
    stack: Stack<SpanOrLiteral<'i>>,
    /// Used for setting max parser calls limit.
    call_tracker: CallLimitTracker,
    /// Together with tracking of `pos_attempts` and `attempt_pos`
    /// as a pair of (list of rules that we've tried to parse but failed, max parsed position)
    /// we track those rules (which we've tried to parse at the same max pos) at this helper struct.
    ///
    /// Note, that we may try to parse several rules on different positions. We want to track only
    /// those rules, which attempt position is bigger, because we consider that it's nearer to the
    /// query that user really wanted to pass.
    ///
    /// E.g. we have a query `create user "Bobby"` and two root rules:
    /// * CreateUser  = { "create" ~ "user"  ~ Name }
    /// * CreateTable = { "create" ~ "table" ~ Name }
    /// * Name = { SOME_DEFINITION }
    ///
    /// While parsing the query we'll update tracker position to the start of "Bobby", because we'd
    /// successfully parse "create" + "user" (and not "table").
    parse_attempts: ParseAttempts<R>,
}

/// Creates a `ParserState` from a `&str`, supplying it to a closure `f`.
///
/// # Examples
///
/// ```
/// # use pest;
/// let input = "";
/// pest::state::<(), _>(input, |s| Ok(s)).unwrap();
/// ```
#[allow(clippy::perf)]
pub fn state<'i, R: RuleType, F>(input: &'i str, f: F) -> Result<pairs::Pairs<'i, R>, Error<R>>
where
    F: FnOnce(Box<ParserState<'i, R>>) -> ParseResult<Box<ParserState<'i, R>>>,
{
    let state = ParserState::new(input);

    match f(state) {
        Ok(state) => {
            let len = state.queue.len();
            Ok(new(Rc::new(state.queue), input, None, 0, len))
        }
        Err(mut state) => {
            let variant = if state.reached_call_limit() {
                ErrorVariant::CustomError {
                    message: "call limit reached".to_owned(),
                }
            } else {
                state.pos_attempts.sort();
                state.pos_attempts.dedup();
                state.neg_attempts.sort();
                state.neg_attempts.dedup();
                ErrorVariant::ParsingError {
                    positives: state.pos_attempts.clone(),
                    negatives: state.neg_attempts.clone(),
                }
            };

            if state.parse_attempts.enabled {
                Err(Error::new_from_pos_with_parsing_attempts(
                    variant,
                    Position::new_internal(input, state.attempt_pos),
                    state.parse_attempts.clone(),
                ))
            } else {
                Err(Error::new_from_pos(
                    variant,
                    Position::new_internal(input, state.attempt_pos),
                ))
            }
        }
    }
}

impl<'i, R: RuleType> ParserState<'i, R> {
    /// Allocates a fresh `ParserState` object to the heap and returns the owned `Box`. This `Box`
    /// will be passed from closure to closure based on the needs of the specified `Parser`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// let input = "";
    /// let state: Box<pest::ParserState<&str>> = pest::ParserState::new(input);
    /// ```
    pub fn new(input: &'i str) -> Box<Self> {
        Box::new(ParserState {
            position: Position::from_start(input),
            queue: vec![],
            lookahead: Lookahead::None,
            pos_attempts: vec![],
            neg_attempts: vec![],
            attempt_pos: 0,
            atomicity: Atomicity::NonAtomic,
            stack: Stack::new(),
            call_tracker: Default::default(),
            parse_attempts: ParseAttempts::new(),
        })
    }

    /// Get all parse attempts after process of parsing is finished.
    pub fn get_parse_attempts(&self) -> &ParseAttempts<R> {
        &self.parse_attempts
    }

    /// Returns a reference to the current `Position` of the `ParserState`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let position = state.position();
    /// assert_eq!(position.pos(), 0);
    /// ```
    pub fn position(&self) -> &Position<'i> {
        &self.position
    }

    /// Returns the current atomicity of the `ParserState`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # use pest::Atomicity;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let atomicity = state.atomicity();
    /// assert_eq!(atomicity, Atomicity::NonAtomic);
    /// ```
    pub fn atomicity(&self) -> Atomicity {
        self.atomicity
    }

    #[inline]
    fn inc_call_check_limit(mut self: Box<Self>) -> ParseResult<Box<Self>> {
        if self.call_tracker.limit_reached() {
            return Err(self);
        }
        self.call_tracker.increment_depth();
        Ok(self)
    }

    #[inline]
    fn reached_call_limit(&self) -> bool {
        self.call_tracker.limit_reached()
    }

    /// Wrapper needed to generate tokens. This will associate the `R` type rule to the closure
    /// meant to match the rule.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "a";
    /// let pairs: Vec<_> = pest::state(input, |state| {
    ///     state.rule(Rule::a, |s| Ok(s))
    /// }).unwrap().collect();
    ///
    /// assert_eq!(pairs.len(), 1);
    /// ```
    #[inline]
    pub fn rule<F>(mut self: Box<Self>, rule: R, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        // Position from which this `rule` starts parsing.
        let actual_pos = self.position.pos();
        // Remember index of the `self.queue` element that will be associated with this `rule`.
        let index = self.queue.len();

        let (pos_attempts_index, neg_attempts_index) = if actual_pos == self.attempt_pos {
            (self.pos_attempts.len(), self.neg_attempts.len())
        } else {
            // Attempts have not been cleared yet since the attempt_pos is older.
            (0, 0)
        };

        if self.lookahead == Lookahead::None && self.atomicity != Atomicity::Atomic {
            // Pair's position will only be known after running the closure.
            self.queue.push(QueueableToken::Start {
                end_token_index: 0,
                input_pos: actual_pos,
            });
        }

        // Remember attempts number before `f` call.
        // In `track` using this variable we can say, how many attempts were added
        // during children rules traversal.
        let attempts = self.attempts_at(actual_pos);
        // Number of call stacks present in `self.parse_attempts` before `f` call.
        // We need to remember this number only in case there wasn't found any farther attempt.
        // E.g. we are handling rule, on start position of which may be tested two
        // children rules. At the moment we'll return from `f` call below,
        // there will be two more children rules in `self.parse_attempts` that we'll
        // consider to be the children of current `rule`.
        let mut remember_call_stacks_number = self.parse_attempts.call_stacks_number();
        // Max parsing attempt position at the moment of `rule` handling.
        // It case it's raised during children rules handling, it means
        // we've made a parsing progress.
        let remember_max_position = self.parse_attempts.max_position;

        let result = f(self);

        let mut try_add_rule_to_stack = |new_state: &mut Box<ParserState<'_, R>>| {
            if new_state.parse_attempts.max_position > remember_max_position {
                // It means that one of `match_string` or e.g. `match_insensetive` function calls
                // have already erased `self.parse_attempts.call_stacks` and that previously
                // remembered values are not valid anymore.
                remember_call_stacks_number = 0;
            }
            if !matches!(new_state.atomicity, Atomicity::Atomic) {
                new_state
                    .parse_attempts
                    .try_add_new_stack_rule(rule, remember_call_stacks_number);
            }
        };

        match result {
            Ok(mut new_state) => {
                if new_state.lookahead == Lookahead::Negative {
                    new_state.track(
                        rule,
                        actual_pos,
                        pos_attempts_index,
                        neg_attempts_index,
                        attempts,
                    );
                }

                if new_state.lookahead == Lookahead::None
                    && new_state.atomicity != Atomicity::Atomic
                {
                    // Index of `QueueableToken::End` token added below
                    // that corresponds to previously added `QueueableToken::Start` token.
                    let new_index = new_state.queue.len();
                    match new_state.queue[index] {
                        QueueableToken::Start {
                            ref mut end_token_index,
                            ..
                        } => *end_token_index = new_index,
                        _ => unreachable!(),
                    };

                    let new_pos = new_state.position.pos();

                    new_state.queue.push(QueueableToken::End {
                        start_token_index: index,
                        rule,
                        tag: None,
                        input_pos: new_pos,
                    });
                }

                // Note, that we need to count positive parsing results too, because we can fail in
                // optional rule call inside which may lie the farthest
                // parsed token.
                if new_state.parse_attempts.enabled {
                    try_add_rule_to_stack(&mut new_state);
                }
                Ok(new_state)
            }
            Err(mut new_state) => {
                if new_state.lookahead != Lookahead::Negative {
                    new_state.track(
                        rule,
                        actual_pos,
                        pos_attempts_index,
                        neg_attempts_index,
                        attempts,
                    );
                    if new_state.parse_attempts.enabled {
                        try_add_rule_to_stack(&mut new_state);
                    }
                }

                if new_state.lookahead == Lookahead::None
                    && new_state.atomicity != Atomicity::Atomic
                {
                    new_state.queue.truncate(index);
                }

                Err(new_state)
            }
        }
    }

    /// Tag current node
    ///
    /// # Examples
    ///
    /// Try to recognize the one specified in a set of characters
    ///
    /// ```
    /// use pest::{state, ParseResult, ParserState, iterators::Pair};
    /// #[allow(non_camel_case_types)]
    /// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     character,
    /// }
    /// fn mark_c(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
    ///     state.sequence(|state| {
    ///         character(state)
    ///             .and_then(|state| character(state))
    ///             .and_then(|state| character(state))
    ///             .and_then(|state| state.tag_node("c"))
    ///             .and_then(|state| character(state))
    ///     })
    /// }
    /// fn character(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
    ///     state.rule(Rule::character, |state| state.match_range('a'..'z'))
    /// }
    ///
    /// let input = "abcd";
    /// let pairs = state(input, mark_c).unwrap();
    /// // find all node tag as `c`
    /// let find: Vec<Pair<Rule>> = pairs.filter(|s| s.as_node_tag() == Some("c")).collect();
    /// assert_eq!(find[0].as_str(), "c")
    /// ```
    #[inline]
    pub fn tag_node(mut self: Box<Self>, tag: &'i str) -> ParseResult<Box<Self>> {
        if let Some(QueueableToken::End { tag: old, .. }) = self.queue.last_mut() {
            *old = Some(tag)
        }
        Ok(self)
    }

    /// Get number of allowed rules attempts + prohibited rules attempts.
    fn attempts_at(&self, pos: usize) -> usize {
        if self.attempt_pos == pos {
            self.pos_attempts.len() + self.neg_attempts.len()
        } else {
            0
        }
    }

    fn track(
        &mut self,
        rule: R,
        pos: usize,
        pos_attempts_index: usize,
        neg_attempts_index: usize,
        prev_attempts: usize,
    ) {
        if self.atomicity == Atomicity::Atomic {
            return;
        }

        // If nested rules made no progress, there is no use to report them; it's only useful to
        // track the current rule, the exception being when only one attempt has been made during
        // the children rules.
        let curr_attempts = self.attempts_at(pos);
        if curr_attempts > prev_attempts && curr_attempts - prev_attempts == 1 {
            return;
        }

        if pos == self.attempt_pos {
            self.pos_attempts.truncate(pos_attempts_index);
            self.neg_attempts.truncate(neg_attempts_index);
        }

        if pos > self.attempt_pos {
            self.pos_attempts.clear();
            self.neg_attempts.clear();
            self.attempt_pos = pos;
        }

        let attempts = if self.lookahead != Lookahead::Negative {
            &mut self.pos_attempts
        } else {
            &mut self.neg_attempts
        };

        if pos == self.attempt_pos {
            attempts.push(rule);
        }
    }

    /// Starts a sequence of transformations provided by `f` from the `Box<ParserState>`. Returns
    /// the same `Result` returned by `f` in the case of an `Ok`, or `Err` with the current
    /// `Box<ParserState>` otherwise.
    ///
    /// This method is useful to parse sequences that only match together which usually come in the
    /// form of chained `Result`s with
    /// [`Result::and_then`](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then).
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "a";
    /// let pairs: Vec<_> = pest::state(input, |state| {
    ///     state.sequence(|s| {
    ///         s.rule(Rule::a, |s| Ok(s)).and_then(|s| {
    ///             s.match_string("b")
    ///         })
    ///     }).or_else(|s| {
    ///         Ok(s)
    ///     })
    /// }).unwrap().collect();
    ///
    /// assert_eq!(pairs.len(), 0);
    /// ```
    #[inline]
    pub fn sequence<F>(mut self: Box<Self>, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        let token_index = self.queue.len();
        let initial_pos = self.position;

        let result = f(self);

        match result {
            Ok(new_state) => Ok(new_state),
            Err(mut new_state) => {
                // Restore the initial position and truncate the token queue.
                new_state.position = initial_pos;
                new_state.queue.truncate(token_index);
                Err(new_state)
            }
        }
    }

    /// Repeatedly applies the transformation provided by `f` from the `Box<ParserState>`. Returns
    /// `Ok` with the updated `Box<ParserState>` returned by `f` wrapped up in an `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "aab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.repeat(|s| {
    ///     s.match_string("a")
    /// });
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 2);
    ///
    /// state = pest::ParserState::new(input);
    /// result = state.repeat(|s| {
    ///     s.match_string("b")
    /// });
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 0);
    /// ```
    #[inline]
    pub fn repeat<F>(mut self: Box<Self>, mut f: F) -> ParseResult<Box<Self>>
    where
        F: FnMut(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        let mut result = f(self);

        loop {
            match result {
                Ok(state) => result = f(state),
                Err(state) => return Ok(state),
            };
        }
    }

    /// Optionally applies the transformation provided by `f` from the `Box<ParserState>`. Returns
    /// `Ok` with the updated `Box<ParserState>` returned by `f` regardless of the `Result`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let result = state.optional(|s| {
    ///     s.match_string("ab")
    /// });
    /// assert!(result.is_ok());
    ///
    /// state = pest::ParserState::new(input);
    /// let result = state.optional(|s| {
    ///     s.match_string("ac")
    /// });
    /// assert!(result.is_ok());
    /// ```
    #[inline]
    pub fn optional<F>(mut self: Box<Self>, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        match f(self) {
            Ok(state) | Err(state) => Ok(state),
        }
    }

    /// Generic function to handle result of char/string/range parsing
    /// in order to track (un)expected tokens.
    fn handle_token_parse_result(
        &mut self,
        start_position: usize,
        token: ParsingToken,
        parse_succeeded: bool,
    ) {
        // New position after tracked parsed element for case of `parse_succeded` is true.
        // Position of parsing failure otherwise.
        let current_pos = self.position.pos();

        if parse_succeeded {
            if self.lookahead == Lookahead::Negative {
                self.parse_attempts
                    .try_add_new_token(token, start_position, current_pos, true);
            } else if current_pos > self.parse_attempts.max_position {
                self.parse_attempts.nullify_expected_tokens(current_pos);
            }
        } else if self.lookahead != Lookahead::Negative {
            self.parse_attempts
                .try_add_new_token(token, start_position, current_pos, false);
        }
    }

    /// Attempts to match a single character based on a filter function. Returns `Ok` with the
    /// updated `Box<ParserState>` if successful, or `Err` with the updated `Box<ParserState>`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let result = state.match_char_by(|c| c.is_ascii());
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 1);
    ///
    /// let input = "❤";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let result = state.match_char_by(|c| c.is_ascii());
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().position().pos(), 0);
    /// ```
    #[inline]
    pub fn match_char_by<F>(mut self: Box<Self>, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(char) -> bool,
    {
        let start_position = self.position.pos();
        let succeeded = self.position.match_char_by(f);
        if self.parse_attempts.enabled {
            let token = ParsingToken::BuiltInRule;
            self.handle_token_parse_result(start_position, token, succeeded);
        }
        if succeeded {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Attempts to match the given string. Returns `Ok` with the updated `Box<ParserState>` if
    /// successful, or `Err` with the updated `Box<ParserState>` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.match_string("ab");
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 2);
    ///
    /// state = pest::ParserState::new(input);
    /// result = state.match_string("ac");
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().position().pos(), 0);
    /// ```
    #[inline]
    pub fn match_string(mut self: Box<Self>, string: &str) -> ParseResult<Box<Self>> {
        let start_position = self.position.pos();
        let succeeded = self.position.match_string(string);
        if self.parse_attempts.enabled {
            let token = ParsingToken::Sensitive {
                token: String::from(string),
            };
            self.handle_token_parse_result(start_position, token, succeeded);
        }
        if succeeded {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Pushes the given literal to the stack, and always returns `Ok(Box<ParserState>)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    ///
    /// let mut result = state.stack_push_literal("a");
    /// assert!(result.is_ok());
    /// assert_eq!(result.as_ref().unwrap().position().pos(), 0);
    ///
    /// let mut result = result.unwrap().stack_pop();
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 1);
    /// ```
    #[inline]
    pub fn stack_push_literal(
        mut self: Box<Self>,
        string: impl Into<Cow<'static, str>>,
    ) -> ParseResult<Box<Self>> {
        // convert string into a Literal, and then into a BorrowedOrRc
        self.stack
            .push(SpanOrLiteral::Literal(string.into().into()));
        Ok(self)
    }

    /// Attempts to case-insensitively match the given string. Returns `Ok` with the updated
    /// `Box<ParserState>` if successful, or `Err` with the updated `Box<ParserState>` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.match_insensitive("AB");
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 2);
    ///
    /// state = pest::ParserState::new(input);
    /// result = state.match_insensitive("AC");
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().position().pos(), 0);
    /// ```
    #[inline]
    pub fn match_insensitive(mut self: Box<Self>, string: &str) -> ParseResult<Box<Self>> {
        let start_position: usize = self.position().pos();
        let succeeded = self.position.match_insensitive(string);
        if self.parse_attempts.enabled {
            let token = ParsingToken::Insensitive {
                token: String::from(string),
            };
            self.handle_token_parse_result(start_position, token, succeeded);
        }
        if succeeded {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Attempts to match a single character from the given range. Returns `Ok` with the updated
    /// `Box<ParserState>` if successful, or `Err` with the updated `Box<ParserState>` otherwise.
    ///
    /// # Caution
    /// The provided `range` is interpreted as inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.match_range('a'..'z');
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 1);
    ///
    /// state = pest::ParserState::new(input);
    /// result = state.match_range('A'..'Z');
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().position().pos(), 0);
    /// ```
    #[inline]
    pub fn match_range(mut self: Box<Self>, range: Range<char>) -> ParseResult<Box<Self>> {
        let start_position = self.position().pos();
        let token = ParsingToken::Range {
            start: range.start,
            end: range.end,
        };
        let succeeded = self.position.match_range(range);
        if self.parse_attempts.enabled {
            self.handle_token_parse_result(start_position, token, succeeded);
        }
        if succeeded {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Attempts to skip `n` characters forward. Returns `Ok` with the updated `Box<ParserState>`
    /// if successful, or `Err` with the updated `Box<ParserState>` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.skip(1);
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 1);
    ///
    /// state = pest::ParserState::new(input);
    /// result = state.skip(3);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().position().pos(), 0);
    /// ```
    #[inline]
    pub fn skip(mut self: Box<Self>, n: usize) -> ParseResult<Box<Self>> {
        if self.position.skip(n) {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Attempts to skip forward until one of the given strings is found. Returns `Ok` with the
    /// updated `Box<ParserState>` whether or not one of the strings is found.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "abcd";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.skip_until(&["c", "d"]);
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 2);
    /// ```
    #[inline]
    pub fn skip_until(mut self: Box<Self>, strings: &[&str]) -> ParseResult<Box<Self>> {
        self.position.skip_until(strings);
        Ok(self)
    }

    /// Attempts to match the start of the input. Returns `Ok` with the current `Box<ParserState>`
    /// if the parser has not yet advanced, or `Err` with the current `Box<ParserState>` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.start_of_input();
    /// assert!(result.is_ok());
    ///
    /// state = pest::ParserState::new(input);
    /// state = state.match_string("ab").unwrap();
    /// result = state.start_of_input();
    /// assert!(result.is_err());
    /// ```
    #[inline]
    pub fn start_of_input(self: Box<Self>) -> ParseResult<Box<Self>> {
        if self.position.at_start() {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Attempts to match the end of the input. Returns `Ok` with the current `Box<ParserState>` if
    /// there is no input remaining, or `Err` with the current `Box<ParserState>` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.end_of_input();
    /// assert!(result.is_err());
    ///
    /// state = pest::ParserState::new(input);
    /// state = state.match_string("ab").unwrap();
    /// result = state.end_of_input();
    /// assert!(result.is_ok());
    /// ```
    #[inline]
    pub fn end_of_input(self: Box<Self>) -> ParseResult<Box<Self>> {
        if self.position.at_end() {
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Starts a lookahead transformation provided by `f` from the `Box<ParserState>`. It returns
    /// `Ok` with the current `Box<ParserState>` if `f` also returns an `Ok`, or `Err` with the current
    /// `Box<ParserState>` otherwise. If `is_positive` is `false`, it swaps the `Ok` and `Err`
    /// together, negating the `Result`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "a";
    /// let pairs: Vec<_> = pest::state(input, |state| {
    ///     state.lookahead(true, |state| {
    ///         state.rule(Rule::a, |s| Ok(s))
    ///     })
    /// }).unwrap().collect();
    ///
    /// assert_eq!(pairs.len(), 0);
    /// ```
    #[inline]
    pub fn lookahead<F>(mut self: Box<Self>, is_positive: bool, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        let initial_lookahead = self.lookahead;

        self.lookahead = if is_positive {
            match initial_lookahead {
                Lookahead::None | Lookahead::Positive => Lookahead::Positive,
                Lookahead::Negative => Lookahead::Negative,
            }
        } else {
            match initial_lookahead {
                Lookahead::None | Lookahead::Positive => Lookahead::Negative,
                Lookahead::Negative => Lookahead::Positive,
            }
        };

        let initial_pos = self.position;

        let result = f(self.checkpoint());

        let result_state = match result {
            Ok(mut new_state) => {
                new_state.position = initial_pos;
                new_state.lookahead = initial_lookahead;
                Ok(new_state.restore())
            }
            Err(mut new_state) => {
                new_state.position = initial_pos;
                new_state.lookahead = initial_lookahead;
                Err(new_state.restore())
            }
        };

        if is_positive {
            result_state
        } else {
            match result_state {
                Ok(state) => Err(state),
                Err(state) => Ok(state),
            }
        }
    }

    /// Transformation which stops `Token`s from being generated according to `is_atomic`.
    /// Used as wrapper over `rule` (or even another `atomic`) call.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::{self, Atomicity};
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "a";
    /// let pairs: Vec<_> = pest::state(input, |state| {
    ///     state.atomic(Atomicity::Atomic, |s| {
    ///         s.rule(Rule::a, |s| Ok(s))
    ///     })
    /// }).unwrap().collect();
    ///
    /// assert_eq!(pairs.len(), 0);
    /// ```
    #[inline]
    pub fn atomic<F>(mut self: Box<Self>, atomicity: Atomicity, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        // In case child parsing call is another `atomic` it will have its own atomicity status.
        let initial_atomicity = self.atomicity;
        // In case child atomicity is the same as we've demanded, we shouldn't do nothing.
        // E.g. we have the following rules:
        // * RootRule = @{ InnerRule }
        // * InnerRule = @{ ... }
        let should_toggle = self.atomicity != atomicity;

        // Note that we take atomicity of the top rule and not of the leaf (inner).
        if should_toggle {
            self.atomicity = atomicity;
        }

        let result = f(self);

        match result {
            Ok(mut new_state) => {
                if should_toggle {
                    new_state.atomicity = initial_atomicity;
                }
                Ok(new_state)
            }
            Err(mut new_state) => {
                if should_toggle {
                    new_state.atomicity = initial_atomicity;
                }
                Err(new_state)
            }
        }
    }

    /// Evaluates the result of closure `f` and pushes the span of the input consumed from before
    /// `f` is called to after `f` is called to the stack. Returns `Ok(Box<ParserState>)` if `f` is
    /// called successfully, or `Err(Box<ParserState>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.stack_push(|state| state.match_string("a"));
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 1);
    /// ```
    #[inline]
    pub fn stack_push<F>(mut self: Box<Self>, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        self = self.inc_call_check_limit()?;
        let start = self.position;

        let result = f(self);

        match result {
            Ok(mut state) => {
                let end = state.position;
                state.stack.push(SpanOrLiteral::Span(start.span(&end)));
                Ok(state)
            }
            Err(state) => Err(state),
        }
    }

    /// Peeks the top of the stack and attempts to match the string. Returns `Ok(Box<ParserState>)`
    /// if the string is matched successfully, or `Err(Box<ParserState>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "aa";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.stack_push(|state| state.match_string("a")).and_then(
    ///     |state| state.stack_peek()
    /// );
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 2);
    /// ```
    #[inline]
    pub fn stack_peek(self: Box<Self>) -> ParseResult<Box<Self>> {
        let string = self
            .stack
            .peek()
            .expect("peek was called on empty stack")
            .as_borrowed_or_rc();
        self.match_string(string.as_str())
    }

    /// Pops the top of the stack and attempts to match the string. Returns `Ok(Box<ParserState>)`
    /// if the string is matched successfully, or `Err(Box<ParserState>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "aa";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.stack_push(|state| state.match_string("a")).and_then(
    ///     |state| state.stack_pop()
    /// );
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 2);
    /// ```
    #[inline]
    pub fn stack_pop(mut self: Box<Self>) -> ParseResult<Box<Self>> {
        let string = self
            .stack
            .pop()
            .expect("pop was called on empty stack")
            .as_borrowed_or_rc();
        self.match_string(string.as_str())
    }

    /// Matches part of the state of the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::{self, MatchDir};
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "abcd cd cb";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state
    ///     .stack_push(|state| state.match_string("a"))
    ///     .and_then(|state| state.stack_push(|state| state.match_string("b")))
    ///     .and_then(|state| state.stack_push(|state| state.match_string("c")))
    ///     .and_then(|state| state.stack_push(|state| state.match_string("d")))
    ///     .and_then(|state| state.match_string(" "))
    ///     .and_then(|state| state.stack_match_peek_slice(2, None, MatchDir::BottomToTop))
    ///     .and_then(|state| state.match_string(" "))
    ///     .and_then(|state| state.stack_match_peek_slice(1, Some(-1), MatchDir::TopToBottom));
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 10);
    /// ```
    #[inline]
    pub fn stack_match_peek_slice(
        mut self: Box<Self>,
        start: i32,
        end: Option<i32>,
        match_dir: MatchDir,
    ) -> ParseResult<Box<Self>> {
        let range = match constrain_idxs(start, end, self.stack.len()) {
            Some(r) => r,
            None => return Err(self),
        };
        // return true if an empty sequence is requested
        if range.end <= range.start {
            return Ok(self);
        }

        let mut position = self.position;
        let result = {
            let mut iter_b2t = self.stack[range].iter();
            let matcher =
                |span: &SpanOrLiteral<'_>| position.match_string(span.as_borrowed_or_rc().as_str());
            match match_dir {
                MatchDir::BottomToTop => iter_b2t.all(matcher),
                MatchDir::TopToBottom => iter_b2t.rev().all(matcher),
            }
        };
        if result {
            self.position = position;
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Matches the full state of the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "abba";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state
    ///     .stack_push(|state| state.match_string("a"))
    ///     .and_then(|state| { state.stack_push(|state| state.match_string("b")) })
    ///     .and_then(|state| state.stack_match_peek());
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 4);
    /// ```
    #[inline]
    pub fn stack_match_peek(self: Box<Self>) -> ParseResult<Box<Self>> {
        self.stack_match_peek_slice(0, None, MatchDir::TopToBottom)
    }

    /// Matches the full state of the stack. This method will clear the stack as it evaluates.
    ///
    /// # Examples
    ///
    /// ```
    /// /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "aaaa";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.stack_push(|state| state.match_string("a")).and_then(|state| {
    ///     state.stack_push(|state| state.match_string("a"))
    /// }).and_then(|state| state.stack_match_peek());
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 4);
    /// ```
    #[inline]
    pub fn stack_match_pop(mut self: Box<Self>) -> ParseResult<Box<Self>> {
        let mut position = self.position;
        let mut result = true;
        while let Some(span) = self.stack.pop() {
            result = position.match_string(span.as_borrowed_or_rc().as_str());
            if !result {
                break;
            }
        }

        if result {
            self.position = position;
            Ok(self)
        } else {
            Err(self)
        }
    }

    /// Drops the top of the stack. Returns `Ok(Box<ParserState>)` if there was a value to drop, or
    /// `Err(Box<ParserState>)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "aa";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.stack_push(|state| state.match_string("a")).and_then(
    ///     |state| state.stack_drop()
    /// );
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().position().pos(), 1);
    /// ```
    #[inline]
    pub fn stack_drop(mut self: Box<Self>) -> ParseResult<Box<Self>> {
        match self.stack.pop() {
            Some(_) => Ok(self),
            None => Err(self),
        }
    }

    /// Restores the original state of the `ParserState` when `f` returns an `Err`. Currently,
    /// this method only restores the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "ab";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input);
    /// let mut result = state.restore_on_err(|state| state.stack_push(|state|
    ///     state.match_string("a")).and_then(|state| state.match_string("a"))
    /// );
    ///
    /// assert!(result.is_err());
    ///
    /// // Since the the rule doesn't match, the "a" pushed to the stack will be removed.
    /// let catch_panic = std::panic::catch_unwind(|| result.unwrap_err().stack_pop());
    /// assert!(catch_panic.is_err());
    /// ```
    #[inline]
    pub fn restore_on_err<F>(self: Box<Self>, f: F) -> ParseResult<Box<Self>>
    where
        F: FnOnce(Box<Self>) -> ParseResult<Box<Self>>,
    {
        match f(self.checkpoint()) {
            Ok(state) => Ok(state.checkpoint_ok()),
            Err(state) => Err(state.restore()),
        }
    }

    // Mark the current state as a checkpoint and return the `Box`.
    #[inline]
    pub(crate) fn checkpoint(mut self: Box<Self>) -> Box<Self> {
        self.stack.snapshot();
        self
    }

    // The checkpoint was cleared successfully
    // so remove it without touching other stack state.
    #[inline]
    pub(crate) fn checkpoint_ok(mut self: Box<Self>) -> Box<Self> {
        self.stack.clear_snapshot();
        self
    }

    // Restore the current state to the most recent checkpoint.
    #[inline]
    pub(crate) fn restore(mut self: Box<Self>) -> Box<Self> {
        self.stack.restore();
        self
    }
}

/// Helper function used only in case stack operations (PUSH/POP) are used in grammar.
fn constrain_idxs(start: i32, end: Option<i32>, len: usize) -> Option<Range<usize>> {
    let start_norm = normalize_index(start, len)?;
    let end_norm = end.map_or(Some(len), |e| normalize_index(e, len))?;
    Some(start_norm..end_norm)
}

/// `constrain_idxs` helper function.
/// Normalizes the index using its sequence’s length.
/// Returns `None` if the normalized index is OOB.
fn normalize_index(i: i32, len: usize) -> Option<usize> {
    if i > len as i32 {
        None
    } else if i >= 0 {
        Some(i as usize)
    } else {
        let real_i = len as i32 + i;
        if real_i >= 0 {
            Some(real_i as usize)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalize_index_pos() {
        assert_eq!(normalize_index(4, 6), Some(4));
        assert_eq!(normalize_index(5, 5), Some(5));
        assert_eq!(normalize_index(6, 3), None);
    }

    #[test]
    fn normalize_index_neg() {
        assert_eq!(normalize_index(-4, 6), Some(2));
        assert_eq!(normalize_index(-5, 5), Some(0));
        assert_eq!(normalize_index(-6, 3), None);
    }
}
