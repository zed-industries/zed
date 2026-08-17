// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Constructs useful in infix operator parsing with the precedence climbing method.

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::Peekable;
use core::ops::BitOr;

use crate::iterators::Pair;
use crate::RuleType;

/// Macro for more convenient const fn definition of `prec_climber::PrecClimber`.
///
/// # Examples
///
/// ```
/// # use pest::prec_climber::{Assoc, PrecClimber};
/// # use pest::prec_climber;
/// # #[allow(non_camel_case_types)]
/// # #[allow(dead_code)]
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule {
/// #     plus,
/// #     minus,
/// #     times,
/// #     divide,
/// #     power
/// # }
/// static CLIMBER: PrecClimber<Rule> = prec_climber![
///     L   plus | minus,
///     L   times | divide,
///     R   power,
/// ];
/// ```
#[cfg(feature = "const_prec_climber")]
#[macro_export]
macro_rules! prec_climber {
    (
        $( $assoc:ident $rule:ident $( | $rules:ident )* ),+ $(,)?
    ) => {{
        prec_climber!(
            @precedences { 1u32 }
            $( [ $rule $( $rules )* ] )*
        );

        $crate::prec_climber::PrecClimber::new_const(
            prec_climber!(
                @array
                $( $assoc $rule $(, $assoc $rules )* ),*
            )
        )
    }};

    ( @assoc L ) => { $crate::prec_climber::Assoc::Left };
    ( @assoc R ) => { $crate::prec_climber::Assoc::Right };

    (
        @array
        $(
            $assoc:ident $rule:ident
        ),*
    ) => {
        &[
            $(
                (
                    Rule::$rule,
                    $rule,
                    prec_climber!( @assoc $assoc ),
                )
            ),*
        ]
    };

    (
        @precedences { $precedence:expr }
    ) => {};

    (
        @precedences { $precedence:expr }
        [ $( $rule:ident )* ]
        $( [ $( $rules:ident )* ] )*
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            const $rule: u32 = $precedence;
        )*
        prec_climber!(
            @precedences { 1u32 + $precedence }
            $( [ $( $rules )* ] )*
        );
    };
}

/// Associativity of an [`Operator`].
///
/// [`Operator`]: struct.Operator.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Assoc {
    /// Left `Operator` associativity
    Left,
    /// Right `Operator` associativity
    Right,
}

/// Infix operator used in [`PrecClimber`].
///
/// [`PrecClimber`]: struct.PrecClimber.html
#[derive(Debug)]
pub struct Operator<R: RuleType> {
    rule: R,
    assoc: Assoc,
    next: Option<Box<Operator<R>>>,
}

impl<R: RuleType> Operator<R> {
    /// Creates a new `Operator` from a `Rule` and `Assoc`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::prec_climber::{Assoc, Operator};
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     plus,
    /// #     minus
    /// # }
    /// Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Right);
    /// ```
    pub fn new(rule: R, assoc: Assoc) -> Operator<R> {
        Operator {
            rule,
            assoc,
            next: None,
        }
    }
}

impl<R: RuleType> BitOr for Operator<R> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self {
        fn assign_next<R: RuleType>(op: &mut Operator<R>, next: Operator<R>) {
            if let Some(ref mut child) = op.next {
                assign_next(child, next);
            } else {
                op.next = Some(Box::new(next));
            }
        }

        assign_next(&mut self, rhs);
        self
    }
}

/// List of operators and precedences, which can perform [precedence climbing][1] on infix
/// expressions contained in a [`Pairs`]. The token pairs contained in the `Pairs` should start
/// with a *primary* pair and then alternate between an *operator* and a *primary*.
///
/// [1]: https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method
/// [`Pairs`]: ../iterators/struct.Pairs.html
#[derive(Debug)]
pub struct PrecClimber<R: Clone + 'static> {
    ops: Cow<'static, [(R, u32, Assoc)]>,
}

#[cfg(feature = "const_prec_climber")]
impl<R: Clone + 'static> PrecClimber<R> {
    /// Creates a new `PrecClimber` directly from a static slice of
    /// `(rule: Rule, precedence: u32, associativity: Assoc)` tuples.
    ///
    /// Precedence starts from `1`.  Entries don't have to be ordered in any way, but it's easier to read when
    /// sorted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::prec_climber::{Assoc, PrecClimber};
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     plus,
    /// #     minus,
    /// #     times,
    /// #     divide,
    /// #     power
    /// # }
    /// static CLIMBER: PrecClimber<Rule> = PrecClimber::new_const(&[
    ///     (Rule::plus, 1, Assoc::Left), (Rule::minus, 1, Assoc::Left),
    ///     (Rule::times, 2, Assoc::Left), (Rule::divide, 2, Assoc::Left),
    ///     (Rule::power, 3, Assoc::Right)
    /// ]);
    /// ```
    pub const fn new_const(ops: &'static [(R, u32, Assoc)]) -> PrecClimber<R> {
        PrecClimber {
            ops: Cow::Borrowed(ops),
        }
    }
}

impl<R: RuleType> PrecClimber<R> {
    // find matching operator by `rule`
    fn get(&self, rule: &R) -> Option<(u32, Assoc)> {
        self.ops
            .iter()
            .find(|(r, _, _)| r == rule)
            .map(|(_, precedence, assoc)| (*precedence, *assoc))
    }

    /// Creates a new `PrecClimber` from the `Operator`s contained in `ops`. Every entry in the
    /// `Vec` has precedence *index + 1*. In order to have operators with same precedence, they need
    /// to be chained with `|` between them.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::prec_climber::{Assoc, Operator, PrecClimber};
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     plus,
    /// #     minus,
    /// #     times,
    /// #     divide,
    /// #     power
    /// # }
    /// PrecClimber::new(vec![
    ///     Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
    ///     Operator::new(Rule::times, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
    ///     Operator::new(Rule::power, Assoc::Right)
    /// ]);
    /// ```
    pub fn new(ops: Vec<Operator<R>>) -> PrecClimber<R> {
        let ops = ops
            .into_iter()
            .zip(1..)
            .fold(Vec::new(), |mut vec, (op, prec)| {
                let mut next = Some(op);

                while let Some(op) = next.take() {
                    let Operator {
                        rule,
                        assoc,
                        next: op_next,
                    } = op;

                    vec.push((rule, prec, assoc));
                    next = op_next.map(|op| *op);
                }

                vec
            });

        PrecClimber {
            ops: Cow::Owned(ops),
        }
    }

    /// Performs the precedence climbing algorithm on the `pairs` in a similar manner to map-reduce.
    /// *Primary* pairs are mapped with `primary` and then reduced to one single result with
    /// `infix`.
    ///
    /// # Panics
    ///
    /// Panics will occur when `pairs` is empty or when the alternating *primary*, *operator*,
    /// *primary* order is not respected.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let primary = |pair| {
    ///     consume(pair, climber)
    /// };
    /// let infix = |lhs: i32, op: Pair<Rule>, rhs: i32| {
    ///     match op.rule() {
    ///         Rule::plus => lhs + rhs,
    ///         Rule::minus => lhs - rhs,
    ///         Rule::times => lhs * rhs,
    ///         Rule::divide => lhs / rhs,
    ///         Rule::power => lhs.pow(rhs as u32),
    ///         _ => unreachable!()
    ///     }
    /// };
    ///
    /// let result = climber.climb(pairs, primary, infix);
    /// ```
    pub fn climb<'i, P, F, G, T>(&self, mut pairs: P, mut primary: F, mut infix: G) -> T
    where
        P: Iterator<Item = Pair<'i, R>>,
        F: FnMut(Pair<'i, R>) -> T,
        G: FnMut(T, Pair<'i, R>, T) -> T,
    {
        let lhs = primary(
            pairs
                .next()
                .expect("precedence climbing requires a non-empty Pairs"),
        );
        self.climb_rec(lhs, 0, &mut pairs.peekable(), &mut primary, &mut infix)
    }

    fn climb_rec<'i, P, F, G, T>(
        &self,
        mut lhs: T,
        min_prec: u32,
        pairs: &mut Peekable<P>,
        primary: &mut F,
        infix: &mut G,
    ) -> T
    where
        P: Iterator<Item = Pair<'i, R>>,
        F: FnMut(Pair<'i, R>) -> T,
        G: FnMut(T, Pair<'i, R>, T) -> T,
    {
        while pairs.peek().is_some() {
            let rule = pairs.peek().unwrap().as_rule();
            if let Some((prec, _)) = self.get(&rule) {
                if prec >= min_prec {
                    let op = pairs.next().unwrap();
                    let mut rhs = primary(pairs.next().expect(
                        "infix operator must be followed by \
                         a primary expression",
                    ));

                    while pairs.peek().is_some() {
                        let rule = pairs.peek().unwrap().as_rule();
                        if let Some((new_prec, assoc)) = self.get(&rule) {
                            if new_prec > prec || assoc == Assoc::Right && new_prec == prec {
                                rhs = self.climb_rec(rhs, new_prec, pairs, primary, infix);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    lhs = infix(lhs, op, rhs);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        lhs
    }
}
