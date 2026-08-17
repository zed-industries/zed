#![cfg(feature = "miette")]
#![cfg_attr(docsrs, doc(cfg(feature = "miette")))]

use miette::{Diagnostic, SourceSpan};
use std::borrow::Cow;
use tardar::{
    BoxedDiagnostic, DiagnosticResult, DiagnosticResultExt as _, IteratorExt as _, ResultExt as _,
};
use thiserror::Error;

use crate::Glob;
use crate::diagnostics::{SpanExt, Spanned};
use crate::rule::{self, Checked};
use crate::token::{self, Boundary, ExpressionMetadata, TokenTree, Tokenized};

/// APIs for diagnosing globs.
impl<'t> Glob<'t> {
    /// Constructs a [`Glob`] from a glob expression with diagnostics.
    ///
    /// This function is the same as [`Glob::new`], but additionally returns detailed diagnostics
    /// on both success and failure.
    ///
    /// See [`Glob::diagnose`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tardar::DiagnosticResultExt as _;
    /// use wax::Glob;
    ///
    /// let result = Glob::diagnosed("(?i)readme.{md,mkd,markdown}");
    /// for diagnostic in result.diagnostics() {
    ///     eprintln!("{}", diagnostic);
    /// }
    /// if let Some(glob) = result.ok_output() { /* ... */ }
    /// ```
    ///
    /// [`Glob`]: crate::Glob
    /// [`Glob::diagnose`]: crate::Glob::diagnose
    /// [`Glob::new`]: crate::Glob::new
    pub fn diagnosed(expression: &'t str) -> DiagnosticResult<'t, Self> {
        parse_and_diagnose(expression).and_then_diagnose(|tree| {
            Glob::compile::<Tokenized<_>>(tree.as_ref())
                .into_error_diagnostic()
                .map_output(|program| Glob { tree, program })
        })
    }

    /// Gets **non-error** [`Diagnostic`]s.
    ///
    /// This function requires a receiving [`Glob`] and so does not report error-level
    /// [`Diagnostic`]s. It can be used to get non-error diagnostics after constructing or
    /// [partitioning][`Glob::partition`] a [`Glob`].
    ///
    /// See [`Glob::diagnosed`].
    ///
    /// [`Diagnostic`]: miette::Diagnostic
    /// [`Glob`]: crate::Glob
    /// [`Glob::diagnosed`]: crate::Glob::diagnosed
    /// [`Glob::partition`]: crate::Glob::partition
    pub fn diagnose(&self) -> impl Iterator<Item = Box<dyn Diagnostic + '_>> {
        diagnose(self.tree.as_ref())
    }
}

#[derive(Clone, Debug, Diagnostic, Error)]
#[diagnostic(code(wax::glob::semantic_literal), severity(warning))]
#[error("`{literal}` has been interpreted as a literal with no semantics")]
pub struct SemanticLiteralWarning<'t> {
    #[source_code]
    expression: Cow<'t, str>,
    literal: Cow<'t, str>,
    #[label("here")]
    span: SourceSpan,
}

#[derive(Clone, Debug, Diagnostic, Error)]
#[diagnostic(code(wax::glob::terminating_separator), severity(warning))]
#[error("terminating separator may discard matches")]
pub struct TerminatingSeparatorWarning<'t> {
    #[source_code]
    expression: Cow<'t, str>,
    #[label("here")]
    span: SourceSpan,
}

fn parse_and_diagnose(
    expression: &str,
) -> DiagnosticResult<'_, Checked<Tokenized<'_, ExpressionMetadata>>> {
    token::parse(expression)
        .into_error_diagnostic()
        .and_then_diagnose(|tree| rule::check(tree).into_error_diagnostic())
        .and_then_diagnose(|checked| {
            // TODO: This should accept `&Checked`.
            diagnose(checked.as_ref())
                .into_non_error_diagnostic()
                .map_output(|_| checked)
        })
}

fn diagnose<'i, 't, A>(tree: &'i Tokenized<'t, A>) -> impl 'i + Iterator<Item = BoxedDiagnostic<'t>>
where
    A: Spanned,
{
    let token = tree.as_token();
    None.into_iter()
        .chain(
            token
                .literals()
                .filter(|(_, literal)| literal.is_semantic_literal())
                .map(|(component, literal)| {
                    Box::new(SemanticLiteralWarning {
                        expression: tree.expression().clone(),
                        literal: literal.text().clone(),
                        span: component
                            .tokens()
                            .iter()
                            .map(|token| token.annotation().span())
                            .copied()
                            .reduce(SpanExt::union)
                            .map(SourceSpan::from)
                            .expect("no tokens in component"),
                    }) as BoxedDiagnostic
                }),
        )
        .chain(
            token
                .concatenation()
                .last()
                .into_iter()
                .filter(|token| matches!(token.boundary(), Some(Boundary::Separator)))
                .map(|token| {
                    Box::new(TerminatingSeparatorWarning {
                        expression: tree.expression().clone(),
                        span: (*token.annotation().span()).into(),
                    }) as BoxedDiagnostic
                }),
        )
}

#[cfg(test)]
pub mod harness {
    use expect_macro::expect;
    use tardar::{BoxedDiagnostic, DiagnosticResult, DiagnosticResultExt as _};

    use crate::Glob;

    // It is non-trivial to downcast `&dyn Diagnostic`, so diagnostics are identified in tests by
    // code.
    pub const CODE_SEMANTIC_LITERAL: &str = "wax::glob::semantic_literal";
    pub const CODE_TERMINATING_SEPARATOR: &str = "wax::glob::terminating_separator";

    pub fn assert_diagnosed_glob_is_ok(expression: &str) -> (Glob<'_>, Vec<BoxedDiagnostic<'_>>) {
        expect!(
            Glob::diagnosed(expression),
            "`Glob::diagnosed` is `Err`, but expected `Ok`: expression: `{}`",
            expression,
        )
    }

    // TODO: This function does not compose well: the glob expression may not be available, so the
    //       message is less clear.
    pub fn assert_diagnosed_glob_has_code<'t>(
        result: DiagnosticResult<'t, Glob<'t>>,
        expected: &str,
    ) -> DiagnosticResult<'t, Glob<'t>> {
        let diagnostics: Vec<_> = result.diagnostics().iter().collect();
        assert!(
            diagnostics.iter().any(|diagnostic| diagnostic
                .code()
                .is_some_and(|code| code.to_string() == expected)),
            "expected diagnostic code `{}`, but not found: diagnostics: `{:?}`",
            expected,
            diagnostics,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::diagnostics::miette::harness;

    #[cfg(any(unix, windows))]
    #[rstest]
    #[case("../a")]
    #[case("a/..")]
    #[case("a/../b")]
    #[case("{a/,../,b/}")]
    fn diagnosed_glob_has_semantic_literal_warning(#[case] expression: &str) {
        let _ = harness::assert_diagnosed_glob_has_code(
            Ok(harness::assert_diagnosed_glob_is_ok(expression)),
            harness::CODE_SEMANTIC_LITERAL,
        );
    }

    #[rstest]
    #[case("a/b/c/")]
    #[case("**/a/")]
    fn diagnosed_glob_has_terminating_separator_warning(#[case] expression: &str) {
        let _ = harness::assert_diagnosed_glob_has_code(
            Ok(harness::assert_diagnosed_glob_is_ok(expression)),
            harness::CODE_TERMINATING_SEPARATOR,
        );
    }
}
