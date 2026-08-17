use std::convert::TryFrom;

use lazy_static::lazy_static;
use regex_syntax::hir::{Dot, Hir, HirKind};
use regex_syntax::ParserBuilder;

pub use regex_syntax::hir::{Class, ClassUnicode, Literal};

use crate::error::{Error, Result};

lazy_static! {
    static ref DOT_UTF8: Hir = Hir::dot(Dot::AnyChar);
    static ref DOT_BYTES: Hir = Hir::dot(Dot::AnyByte);
}

/// Middle Intermediate Representation of the regex, built from
/// `regex_syntax`'s `Hir`. The goal here is to strip and canonicalize
/// the tree, so that we don't have to do transformations later on the
/// graph, with the potential of running into looping references.
#[derive(Clone, Debug)]
pub enum Mir {
    Empty,
    Loop(Box<Mir>),
    Maybe(Box<Mir>),
    Concat(Vec<Mir>),
    Alternation(Vec<Mir>),
    Class(Class),
    Literal(Literal),
}

impl Mir {
    pub fn utf8(source: &str) -> Result<Mir> {
        Mir::try_from(ParserBuilder::new().build().parse(source)?)
    }

    pub fn utf8_ignore_case(source: &str) -> Result<Mir> {
        Mir::try_from(
            ParserBuilder::new()
                .case_insensitive(true)
                .build()
                .parse(source)?,
        )
    }

    pub fn binary(source: &str) -> Result<Mir> {
        Mir::try_from(
            ParserBuilder::new()
                .utf8(false)
                .unicode(false)
                .build()
                .parse(source)?,
        )
    }

    pub fn binary_ignore_case(source: &str) -> Result<Mir> {
        Mir::try_from(
            ParserBuilder::new()
                .utf8(false)
                .unicode(false)
                .case_insensitive(true)
                .build()
                .parse(source)?,
        )
    }

    pub fn priority(&self) -> usize {
        match self {
            Mir::Empty | Mir::Loop(_) | Mir::Maybe(_) => 0,
            Mir::Concat(concat) => concat.iter().map(Mir::priority).sum(),
            Mir::Alternation(alt) => alt.iter().map(Mir::priority).min().unwrap_or(0),
            Mir::Class(_) => 2,
            Mir::Literal(lit) => match std::str::from_utf8(&lit.0) {
                Ok(s) => 2 * s.chars().count(),
                Err(_) => 2 * lit.0.len(),
            },
        }
    }
}

impl TryFrom<Hir> for Mir {
    type Error = Error;

    fn try_from(hir: Hir) -> Result<Mir> {
        match hir.into_kind() {
            HirKind::Empty => Ok(Mir::Empty),
            HirKind::Concat(concat) => {
                let mut out = Vec::with_capacity(concat.len());

                fn extend(mir: Mir, out: &mut Vec<Mir>) {
                    match mir {
                        Mir::Concat(nested) => {
                            for child in nested {
                                extend(child, out);
                            }
                        }
                        mir => out.push(mir),
                    }
                }

                for hir in concat {
                    extend(Mir::try_from(hir)?, &mut out);
                }

                Ok(Mir::Concat(out))
            }
            HirKind::Alternation(alternation) => {
                let alternation = alternation
                    .into_iter()
                    .map(Mir::try_from)
                    .collect::<Result<_>>()?;

                Ok(Mir::Alternation(alternation))
            }
            HirKind::Literal(literal) => Ok(Mir::Literal(literal)),
            HirKind::Class(class) => Ok(Mir::Class(class)),
            HirKind::Repetition(repetition) => {
                if !repetition.greedy {
                    return Err("#[regex]: non-greedy parsing is currently unsupported.".into());
                }

                let is_dot = if repetition.sub.properties().is_utf8() {
                    *repetition.sub == *DOT_UTF8
                } else {
                    *repetition.sub == *DOT_BYTES
                };
                let mir = Mir::try_from(*repetition.sub)?;

                match (repetition.min, repetition.max) {
                    (0..=1, None) if is_dot => {
                        Err(
                            "#[regex]: \".+\" and \".*\" patterns will greedily consume \
                            the entire source till the end as Logos does not allow \
                            backtracking. If you are looking to match everything until \
                            a specific character, you should use a negative character \
                            class. E.g., use regex r\"'[^']*'\" to match anything in \
                            between two quotes. Read more about that here: \
                            https://github.com/maciejhirsz/logos/issues/302#issuecomment-1521342541."
                            .into()
                        )
                    }
                    // 0 or 1
                    (0, Some(1)) => Ok(Mir::Maybe(Box::new(mir))),
                    // 0 or more
                    (0, None) => Ok(Mir::Loop(Box::new(mir))),
                    // 1 or more
                    (1, None) => {
                        Ok(Mir::Concat(vec![mir.clone(), Mir::Loop(Box::new(mir))]))
                    }
                    // Exact {n}
                    (n, Some(m)) if m == n => {
                        let mut out = Vec::with_capacity(n as usize);
                        for _ in 0..n {
                            out.push(mir.clone());
                        }
                        Ok(Mir::Concat(out))
                    }
                    // At least {n,}
                    (n, None) => {
                        let mut out = Vec::with_capacity(n as usize);
                        for _ in 0..n {
                            out.push(mir.clone());
                        }
                        out.push(Mir::Loop(Box::new(mir)));
                        Ok(Mir::Concat(out))
                    }
                    // Bounded {n, m}
                    (n, Some(m)) => {
                        let mut out = Vec::with_capacity(m as usize);
                        for _ in 0..n {
                            out.push(mir.clone());
                        }
                        for _ in n..m {
                            out.push(Mir::Maybe(Box::new(mir.clone())));
                        }
                        Ok(Mir::Concat(out))
                    }
                }
            }
            HirKind::Capture(capture) => Mir::try_from(*capture.sub),
            HirKind::Look(_) => {
                Err("#[regex]: look-around assertions are currently unsupported.".into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mir;

    #[test]
    fn priorities() {
        let regexes = [
            ("a", 2),
            ("à", 2),
            ("京", 2),
            ("Eté", 6),
            ("Été", 6),
            ("[a-z]+", 2),
            ("a|b", 2),
            ("a|[b-z]", 2),
            ("(foo)+", 6),
            ("foobar", 12),
            ("(fooz|bar)+qux", 12),
        ];

        for (regex, expected) in regexes.iter() {
            let mir = Mir::utf8(regex).unwrap();
            assert_eq!(mir.priority(), *expected, "Failed for regex \"{}\"", regex);
        }
    }

    #[test]
    fn equivalent_patterns() {
        let regexes = [
            ("a|b", "[a-b]"),
            ("1|2|3", "[1-3]"),
            ("1+", "[1]+"),
            ("c*", "[c]*"),
            ("aaa", "a{3}"),
            ("a[a]{2}", "a{3}"),
        ];

        for (regex_left, regex_right) in regexes.iter() {
            let mir_left = Mir::utf8(regex_left).unwrap();
            let mir_right = Mir::utf8(regex_right).unwrap();
            assert_eq!(
                mir_left.priority(),
                mir_right.priority(),
                "Regexes \"{regex_left}\" and \"{regex_right}\" \
                are equivalent but have different priorities"
            );
        }
    }
}
