use const_format::formatcp;
use itertools::{Itertools as _, Position};
#[cfg(feature = "miette")]
use miette::Diagnostic;
use regex::{Error as RegexError, Regex};
use std::borrow::{Borrow, Cow};
#[cfg(feature = "miette")]
use std::fmt::Display;
use thiserror::Error;

use crate::token::{ConcatenationTree, Token, TokenTopology};

/// A regular expression that never matches.
///
/// This expression is formed from a character class that intersects completely disjoint
/// characters. Unlike an empty regular expression, which always matches, this yields an empty
/// character class, which never matches (even against empty strings).
const NEVER_EXPRESSION: &str = "[a&&b]";

#[cfg(windows)]
const SEPARATOR_CLASS_EXPRESSION: &str = "/\\\\";
#[cfg(unix)]
const SEPARATOR_CLASS_EXPRESSION: &str = "/";

// This only encodes the platform's main separator, so any additional separators will be missed. It
// may be better to have explicit platform support and invoke `compile_error!` on unsupported
// platforms, as this could cause very aberrant behavior. Then again, it seems that platforms using
// more than one separator are rare. GS/OS, OS/2, and Windows are likely the best known examples
// and of those only Windows is a supported Rust target at the time of writing (and is already
// supported by Wax).
#[cfg(not(any(windows, unix)))]
const SEPARATOR_CLASS_EXPRESSION: &str = main_separator_class_expression();

#[cfg(not(any(windows, unix)))]
const fn main_separator_class_expression() -> &'static str {
    use std::path::MAIN_SEPARATOR;

    // TODO: This is based upon `regex_syntax::is_meta_character`, but that function is not
    //       `const`. Perhaps that can be changed upstream.
    const fn escape(x: char) -> &'static str {
        match x {
            '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$'
            | '#' | '&' | '-' | '~' => "\\",
            _ => "",
        }
    }

    formatcp!("{0}{1}", escape(MAIN_SEPARATOR), MAIN_SEPARATOR)
}

macro_rules! sepexpr {
    ($fmt:expr) => {
        formatcp!($fmt, formatcp!("[{0}]", SEPARATOR_CLASS_EXPRESSION))
    };
}

macro_rules! nsepexpr {
    ($fmt:expr) => {
        formatcp!($fmt, formatcp!("[^{0}]", SEPARATOR_CLASS_EXPRESSION))
    };
}

/// Describes errors that occur when compiling a glob expression.
///
/// **This error only occurs when the size of the compiled program is too large.** All other
/// compilation errors are considered internal bugs and will panic.
#[derive(Clone, Debug, Error)]
#[error("failed to compile glob: {kind}")]
pub struct CompileError {
    kind: CompileErrorKind,
}

#[derive(Clone, Copy, Debug, Error)]
#[non_exhaustive]
enum CompileErrorKind {
    #[error("oversized program")]
    OversizedProgram,
}

#[cfg(feature = "miette")]
#[cfg_attr(docsrs, doc(cfg(feature = "miette")))]
impl Diagnostic for CompileError {
    fn code<'a>(&'a self) -> Option<Box<dyn 'a + Display>> {
        Some(Box::new(String::from(match self.kind {
            CompileErrorKind::OversizedProgram => "wax::glob::oversized_program",
        })))
    }
}

trait Escaped {
    fn escaped(&self) -> String;
}

impl Escaped for char {
    fn escaped(&self) -> String {
        regex::escape(&self.to_string())
    }
}

impl Escaped for str {
    fn escaped(&self) -> String {
        regex::escape(self)
    }
}

#[derive(Clone, Copy, Debug)]
enum Grouping {
    Capture,
    NonCapture,
}

impl Grouping {
    pub fn push_str(&self, pattern: &mut String, encoding: &str) {
        self.push_with(pattern, || encoding.into());
    }

    pub fn push_with<'p, F>(&self, pattern: &mut String, f: F)
    where
        F: Fn() -> Cow<'p, str>,
    {
        match self {
            Grouping::Capture => pattern.push('('),
            Grouping::NonCapture => pattern.push_str("(?:"),
        }
        pattern.push_str(f().as_ref());
        pattern.push(')');
    }
}

pub fn case_folded_eq(left: &str, right: &str) -> bool {
    let regex = Regex::new(&format!("(?i){}", regex::escape(left)))
        .expect("failed to compile literal regular expression");
    if let Some(matched) = regex.find(right) {
        matched.start() == 0 && matched.end() == right.len()
    }
    else {
        false
    }
}

pub fn compile<'t, T>(tree: impl Borrow<T>) -> Result<Regex, CompileError>
where
    T: ConcatenationTree<'t>,
{
    let mut pattern = String::new();
    pattern.push('^');
    encode(Grouping::Capture, None, &mut pattern, tree);
    pattern.push('$');
    Regex::new(&pattern).map_err(|error| match error {
        RegexError::CompiledTooBig(_) => CompileError {
            kind: CompileErrorKind::OversizedProgram,
        },
        _ => panic!("failed to compile glob"),
    })
}

// TODO: Implement this iteratively.
// TODO: Encode expressions using the HIR in `regex-syntax` rather than text.
// TODO: Some versions of `const_format` in `^0.2.0` fail this lint in `formatcp`. See
//       https://github.com/rodrimati1992/const_format_crates/issues/38
#[allow(clippy::double_parens)]
fn encode<'t, T>(
    grouping: Grouping,
    superposition: Option<Position>,
    pattern: &mut String,
    tree: impl Borrow<T>,
) where
    T: ConcatenationTree<'t>,
{
    use itertools::Position::{First, Last, Middle, Only};

    use crate::token::Archetype::{Character, Range};
    use crate::token::BranchKind::{Alternation, Concatenation, Repetition};
    use crate::token::Evaluation::{Eager, Lazy};
    use crate::token::LeafKind::{Class, Literal, Separator, Wildcard};
    use crate::token::Wildcard::{One, Tree, ZeroOrMore};

    fn encode_intermediate_tree(grouping: Grouping, pattern: &mut String) {
        pattern.push_str(sepexpr!("(?:{0}|{0}"));
        grouping.push_str(pattern, sepexpr!(".*{0}"));
        pattern.push(')');
    }

    // TODO: Use `Grouping` everywhere a group is encoded.
    for (position, token) in tree.borrow().concatenation().iter().with_position() {
        match token.topology() {
            TokenTopology::Leaf(leaf) => match (position, leaf) {
                (_, Literal(literal)) => {
                    // TODO: Only encode changes to casing flags.
                    // TODO: Should Unicode support also be toggled by casing flags?
                    if literal.is_case_insensitive() {
                        pattern.push_str("(?i)");
                    }
                    else {
                        pattern.push_str("(?-i)");
                    }
                    pattern.push_str(&literal.text().escaped());
                },
                (_, Separator(_)) => pattern.push_str(sepexpr!("{0}")),
                (_, Class(class)) => {
                    grouping.push_with(pattern, || {
                        use crate::token::Class as ClassToken;

                        fn encode_class_archetypes(class: &ClassToken, pattern: &mut String) {
                            for archetype in class.archetypes() {
                                match archetype {
                                    Character(literal) => pattern.push_str(&literal.escaped()),
                                    Range(left, right) => {
                                        pattern.push_str(&left.escaped());
                                        pattern.push('-');
                                        pattern.push_str(&right.escaped());
                                    },
                                }
                            }
                        }

                        let mut pattern = String::new();
                        pattern.push('[');
                        if class.is_negated() {
                            pattern.push('^');
                            encode_class_archetypes(class, &mut pattern);
                            pattern.push_str(SEPARATOR_CLASS_EXPRESSION);
                        }
                        else {
                            encode_class_archetypes(class, &mut pattern);
                            pattern.push_str(nsepexpr!("&&{0}"));
                        }
                        pattern.push(']');
                        // TODO: The compiled `Regex` is discarded. Is there a way to check the
                        //       correctness of the expression but do less work (i.e., don't build a
                        //       complete `Regex`)?
                        // Compile the character class sub-expression. This may fail if the subtraction
                        // of the separator pattern yields an empty character class (meaning that the
                        // glob expression matches only separator characters on the target platform).
                        if Regex::new(&pattern).is_ok() {
                            pattern.into()
                        }
                        else {
                            // If compilation fails, then use `NEVER_EXPRESSION`, which matches
                            // nothing.
                            NEVER_EXPRESSION.into()
                        }
                    });
                },
                (_, Wildcard(One)) => grouping.push_str(pattern, nsepexpr!("{0}")),
                (_, Wildcard(ZeroOrMore(Eager))) => grouping.push_str(pattern, nsepexpr!("{0}*")),
                (_, Wildcard(ZeroOrMore(Lazy))) => grouping.push_str(pattern, nsepexpr!("{0}*?")),
                (First, Wildcard(Tree { has_root })) => {
                    if let Some(Middle | Last) = superposition {
                        encode_intermediate_tree(grouping, pattern);
                    }
                    else if *has_root {
                        grouping.push_str(pattern, sepexpr!("{0}.*{0}?"));
                    }
                    else {
                        pattern.push_str(sepexpr!("(?:{0}?|"));
                        grouping.push_str(pattern, sepexpr!(".*{0}"));
                        pattern.push(')');
                    }
                },
                (Middle, Wildcard(Tree { .. })) => {
                    encode_intermediate_tree(grouping, pattern);
                },
                (Last, Wildcard(Tree { .. })) => {
                    if let Some(First | Middle) = superposition {
                        encode_intermediate_tree(grouping, pattern);
                    }
                    else {
                        pattern.push_str(sepexpr!("(?:{0}?|{0}"));
                        grouping.push_str(pattern, ".*");
                        pattern.push(')');
                    }
                },
                (Only, Wildcard(Tree { .. })) => grouping.push_str(pattern, ".*"),
            },
            TokenTopology::Branch(branch) => match branch {
                Alternation(alternation) => {
                    let encodings: Vec<_> = alternation
                        .tokens()
                        .iter()
                        .map(|token| {
                            let mut pattern = String::new();
                            pattern.push_str("(?:");
                            encode::<Token<_>>(
                                Grouping::NonCapture,
                                superposition.or(Some(position)),
                                &mut pattern,
                                token,
                            );
                            pattern.push(')');
                            pattern
                        })
                        .collect();
                    grouping.push_str(pattern, &encodings.join("|"));
                },
                Concatenation(_) => unreachable!(),
                Repetition(repetition) => {
                    let encoding = {
                        let variance = repetition.variance();
                        let mut pattern = String::new();
                        pattern.push_str("(?:");
                        encode::<Token<_>>(
                            Grouping::NonCapture,
                            superposition.or(Some(position)),
                            &mut pattern,
                            repetition.token(),
                        );
                        pattern.push_str(&if let Some(upper) = variance.upper().into_usize() {
                            format!("){{{},{}}}", variance.lower().into_usize(), upper)
                        }
                        else {
                            format!("){{{},}}", variance.lower().into_usize())
                        });
                        pattern
                    };
                    grouping.push_str(pattern, &encoding);
                },
            },
        }
    }
}

#[cfg(test)]
pub mod harness {
    use crate::encode;

    pub fn assert_case_folded_eq_eq(lhs: &str, rhs: &str, expected: bool) -> bool {
        let eq = encode::case_folded_eq(lhs, rhs);
        assert!(
            eq == expected,
            "`encode::case_folded_eq` is `{}`, but expected `{}`",
            eq,
            expected,
        );
        eq
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::encode::harness;

    #[rstest]
    #[case::ascii_with_no_casing("", "")]
    #[case::ascii_with_no_casing("0", "0")]
    #[case::ascii_with_no_casing("_", "_")]
    #[case::ascii_with_casing("a", "a")]
    #[case::ascii_with_casing("a", "A")]
    #[case::ascii_with_casing("aa", "aa")]
    #[case::ascii_with_casing("aa", "aA")]
    #[case::ascii_with_casing("aa", "Aa")]
    #[case::ascii_with_casing("aa", "AA")]
    #[case::ascii_with_casing("z", "z")]
    #[case::ascii_with_casing("z", "Z")]
    #[case::cjk("愛", "愛")]
    #[case::cjk("グロブ", "グロブ")]
    fn case_folded_eq_eq(#[case] lhs: &str, #[case] rhs: &str) {
        harness::assert_case_folded_eq_eq(lhs, rhs, true);
    }

    #[rstest]
    #[case::whitespace("", " ")]
    #[case::whitespace(" ", "\t")]
    #[case::whitespace(" ", "  ")]
    #[case::length("a", "aa")]
    #[case::ascii_with_no_casing("0", "1")]
    #[case::ascii_with_casing("a", "b")]
    #[case::ascii_with_casing("a", "B")]
    #[case::cjk("金", "銀")]
    #[case::cjk("もー", "もぉ")]
    fn case_folded_eq_not_eq(#[case] lhs: &str, #[case] rhs: &str) {
        harness::assert_case_folded_eq_eq(lhs, rhs, false);
    }
}
