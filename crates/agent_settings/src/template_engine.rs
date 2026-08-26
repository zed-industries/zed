//! Shared strict-mode Handlebars engine for the agent's prompt templates.
//!
//! The built-in system prompt (in the `agent` crate) and the per-tool
//! guidance tier both render through [`template_engine`], so every template
//! sees the same strict mode and the same small helper surface. Strict mode
//! turns a typo'd context variable into a render error instead of silent
//! empty output.
//!
//! Helper surface, deliberately small and stable:
//!
//! - `contains`: `{{#if (contains available_tools "grep")}}`
//! - `join`: `{{join available_tools ", "}}`
//! - `array`: builds a list inline — `{{join (array "a" "b") ", "}}` — since
//!   the template syntax has no array literal.
//! - set operations over two lists, each returning a deduplicated list that
//!   preserves the order its elements first appear in its inputs:
//!   - `union`: elements in either list —
//!     `{{join (union (array "a" "b") (array "b" "c")) ", "}}` → `a, b, c`
//!   - `intersect`: elements in both lists —
//!     `{{#if (intersect available_tools (array "grep"))}}`
//!   - `differ`: elements of the first list absent from the second —
//!     `{{join (differ available_tools (array "grep")) ", "}}`
//! - the engine's built-in `and` / `or` / `not` compose as subexpressions:
//!   `{{#if (and sandboxing (or is_linux is_macos))}}`.
//!
//! All helpers return typed values via `call_inner` (rather than writing to
//! the output stream) so they keep their type when used as subexpressions —
//! output-writing helpers degrade to strings there, where the string
//! `"false"` would be truthy.

use std::sync::LazyLock;

use gpui::SharedString;
use handlebars::{Handlebars, JsonRender as _, JsonValue, RenderError, ScopedJson};
use serde::Serialize;

/// The session context available to tool guidance templates.
///
/// Deliberately small and stable: it covers the axes tool guidance actually
/// conditions on — the session's tool set, platform, sandboxing state —
/// without exposing request-builder internals that templates would couple to.
#[derive(Serialize)]
pub struct ToolGuidanceContext<'a> {
    /// Names of the tools enabled for the session.
    pub available_tools: &'a [SharedString],
    pub model_name: Option<&'a str>,
    /// Today's date, `YYYY-MM-DD`.
    pub date: &'a str,
    pub is_linux: bool,
    pub is_windows: bool,
    pub is_macos: bool,
    /// Whether agent terminal commands are sandboxed for this thread's
    /// project — the same gate the built-in system prompt applies to its
    /// sandbox section.
    pub sandboxing: bool,
}

/// A strict-mode Handlebars registry with the shared helpers registered.
pub fn template_engine() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("contains", Box::new(ContainsHelper));
    handlebars.register_helper("join", Box::new(JoinHelper));
    handlebars.register_helper("array", Box::new(ArrayHelper));
    handlebars.register_helper("union", Box::new(SetOpHelper(SetOp::Union)));
    handlebars.register_helper("intersect", Box::new(SetOpHelper(SetOp::Intersect)));
    handlebars.register_helper("differ", Box::new(SetOpHelper(SetOp::Differ)));
    handlebars
}

/// The shared helper-registered engine, built once and reused by
/// [`render_template`], which renders tool guidance per-tool on every
/// completion request and would otherwise rebuild the registry each time.
static TEMPLATE_ENGINE: LazyLock<Handlebars<'static>> = LazyLock::new(template_engine);

/// Renders a template source against the shared engine.
pub fn render_template(source: &str, context: &impl Serialize) -> anyhow::Result<String> {
    Ok(TEMPLATE_ENGINE.render_template(source, context)?)
}

/// Handlebars helper for checking if an item is in a list:
/// `{{#if (contains available_tools "grep")}}`.
#[derive(Clone, Copy)]
pub struct ContainsHelper;

impl handlebars::HelperDef for ContainsHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let list = h
            .param(0)
            .and_then(|value| value.value().as_array())
            .ok_or_else(|| RenderError::new("contains: missing or invalid list parameter"))?;
        let query = h
            .param(1)
            .map(|value| value.value())
            .ok_or_else(|| RenderError::new("contains: missing or invalid query parameter"))?;
        Ok(ScopedJson::Derived(JsonValue::Bool(list.contains(query))))
    }
}

/// Handlebars helper for joining a list into a string with a separator:
/// `{{join available_tools ", "}}`. Elements render like `{{this}}` would
/// render them.
#[derive(Clone, Copy)]
pub struct JoinHelper;

impl handlebars::HelperDef for JoinHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let list = h
            .param(0)
            .and_then(|value| value.value().as_array())
            .ok_or_else(|| RenderError::new("join: missing or invalid list parameter"))?;
        let separator = h
            .param(1)
            .and_then(|value| value.value().as_str())
            .ok_or_else(|| RenderError::new("join: missing or invalid separator parameter"))?;
        let joined = list
            .iter()
            .map(|value| value.render())
            .collect::<Vec<_>>()
            .join(separator);
        Ok(ScopedJson::Derived(JsonValue::String(joined)))
    }
}

/// Handlebars helper that builds a JSON array from its parameters, letting
/// templates construct a list inline: `{{join (array "a" "b") ", "}}` or
/// `{{#if (contains (array "a" "b") "a")}}`. The template syntax has no array
/// literal, so this is the only way to define a list in place.
#[derive(Clone, Copy)]
pub struct ArrayHelper;

impl handlebars::HelperDef for ArrayHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Ok(ScopedJson::Derived(JsonValue::Array(
            h.params()
                .iter()
                .map(|param| param.value().clone())
                .collect(),
        )))
    }
}

/// Set operation applied by [`SetOpHelper`].
#[derive(Clone, Copy)]
enum SetOp {
    /// Elements in either list.
    Union,
    /// Elements in both lists.
    Intersect,
    /// Elements of the first list absent from the second.
    Differ,
}

impl SetOp {
    fn name(self) -> &'static str {
        match self {
            SetOp::Union => "union",
            SetOp::Intersect => "intersect",
            SetOp::Differ => "differ",
        }
    }

    fn apply(self, first: &[JsonValue], second: &[JsonValue]) -> Vec<JsonValue> {
        // Deduplicate while preserving first-appearance order, so the result
        // reads naturally when joined back into a prompt list.
        let deduped = |values: &[JsonValue]| -> Vec<JsonValue> {
            let mut result: Vec<JsonValue> = Vec::new();
            for value in values {
                if !result.contains(value) {
                    result.push(value.clone());
                }
            }
            result
        };
        match self {
            SetOp::Union => deduped(
                &first
                    .iter()
                    .chain(second.iter())
                    .cloned()
                    .collect::<Vec<_>>(),
            ),
            SetOp::Intersect => deduped(
                &first
                    .iter()
                    .filter(|value| second.contains(value))
                    .cloned()
                    .collect::<Vec<_>>(),
            ),
            SetOp::Differ => deduped(
                &first
                    .iter()
                    .filter(|value| !second.contains(value))
                    .cloned()
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

/// Handlebars helper for set operations over two lists:
/// `{{join (union available_tools (array "grep")) ", "}}` or
/// `{{#if (intersect available_tools (array "grep"))}}`. Returns a
/// deduplicated list preserving first-appearance order, so it composes as a
/// subexpression anywhere a list does.
#[derive(Clone, Copy)]
pub struct SetOpHelper(SetOp);

impl handlebars::HelperDef for SetOpHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let name = self.0.name();
        let list = |index: usize| -> Result<&Vec<JsonValue>, RenderError> {
            h.param(index)
                .and_then(|value| value.value().as_array())
                .ok_or_else(|| {
                    RenderError::new(format!("{name}: missing or invalid list parameter {index}"))
                })
        };
        Ok(ScopedJson::Derived(JsonValue::Array(
            self.0.apply(list(0)?, list(1)?),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(source: &str) -> anyhow::Result<String> {
        let available_tools = vec![SharedString::from("grep"), SharedString::from("terminal")];
        let context = ToolGuidanceContext {
            available_tools: &available_tools,
            model_name: Some("test-model"),
            date: "2026-01-01",
            is_linux: false,
            is_windows: false,
            is_macos: true,
            sandboxing: true,
        };
        render_template(source, &context)
    }

    #[test]
    fn test_contains_as_subexpression() -> anyhow::Result<()> {
        assert_eq!(
            render("{{#if (contains available_tools \"grep\")}}yes{{else}}no{{/if}}")?,
            "yes"
        );
        assert_eq!(
            render("{{#if (contains available_tools \"fetch\")}}yes{{else}}no{{/if}}")?,
            "no"
        );
        Ok(())
    }

    #[test]
    fn test_join() -> anyhow::Result<()> {
        assert_eq!(render("{{join available_tools \", \"}}")?, "grep, terminal");
        Ok(())
    }

    #[test]
    fn test_array_builds_inline_list() -> anyhow::Result<()> {
        assert_eq!(render("{{join (array \"a\" \"b\") \"-\"}}")?, "a-b");
        assert_eq!(
            render("{{#if (contains (array \"a\" \"b\") \"b\")}}yes{{else}}no{{/if}}")?,
            "yes"
        );
        Ok(())
    }

    #[test]
    fn test_builtin_boolean_helpers_compose() -> anyhow::Result<()> {
        assert_eq!(
            render("{{#if (and sandboxing (or is_linux is_macos))}}yes{{else}}no{{/if}}")?,
            "yes"
        );
        assert_eq!(
            render("{{#if (and sandboxing (not is_macos))}}yes{{else}}no{{/if}}")?,
            "no"
        );
        Ok(())
    }

    #[test]
    fn test_union() -> anyhow::Result<()> {
        assert_eq!(
            render("{{join (union available_tools (array \"terminal\" \"fetch\")) \", \"}}")?,
            "grep, terminal, fetch"
        );
        Ok(())
    }

    #[test]
    fn test_intersect() -> anyhow::Result<()> {
        assert_eq!(
            render("{{join (intersect available_tools (array \"terminal\" \"fetch\")) \", \"}}")?,
            "terminal"
        );
        assert_eq!(
            render("{{#if (intersect available_tools (array \"grep\"))}}yes{{else}}no{{/if}}")?,
            "yes"
        );
        assert_eq!(
            render("{{#if (intersect available_tools (array \"fetch\"))}}yes{{else}}no{{/if}}")?,
            "no"
        );
        Ok(())
    }

    #[test]
    fn test_differ() -> anyhow::Result<()> {
        assert_eq!(
            render("{{join (differ available_tools (array \"grep\")) \", \"}}")?,
            "terminal"
        );
        Ok(())
    }

    #[test]
    fn test_set_ops_deduplicate_preserving_order() -> anyhow::Result<()> {
        assert_eq!(
            render("{{join (union (array \"b\" \"a\" \"b\") (array \"a\" \"c\")) \", \"}}")?,
            "b, a, c"
        );
        Ok(())
    }

    #[test]
    fn test_set_ops_reject_non_list_parameters() {
        assert!(render("{{union available_tools \"grep\"}}").is_err());
        assert!(render("{{differ \"grep\" available_tools}}").is_err());
    }

    #[test]
    fn test_strict_mode_rejects_unknown_variables() {
        assert!(render("{{no_such_variable}}").is_err());
    }
}
