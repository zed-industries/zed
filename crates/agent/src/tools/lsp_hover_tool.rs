use crate::tools::lsp_tool_utils::{
    format_display_symbol, open_buffer_for_path, resolve_position, validate_lsp_tool_input,
};
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, Entity, Task};
use project::{HoverBlockKind, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::Arc;
use ui::SharedString;
use util::markdown::MarkdownInlineCode;

/// Get type information, documentation, and deprecation notices for a symbol — instantly,
/// without reading the file. This is the fastest way to answer "what type is this?",
/// "what parameters does this method take?", "what does it return?", or "what are the
/// cases of this enum?".
///
/// Use this **before** reaching for `grep` or `read_file` when you need to understand what
/// a symbol is. A single hover call replaces the `grep` → `read_file` → scan-for-the-thing
/// round-trip.
///
/// **When to use hover vs other tools:**
/// - You see `$order->status` and want to know the type → `lsp_hover` (don't read the whole model file)
/// - You see a method call and want its parameters, return type, or full signature → `lsp_hover` (don't find and read the class)
/// - You see an enum value and want all cases → `lsp_hover` on the enum type name (don't find and read the enum file)
/// - You see a magic/virtual property (e.g. Eloquent casts, accessors) and want its type → `lsp_hover`
///   (grep cannot find these — there is no explicit property declaration to match)
/// - You want to read the full implementation → use `goto_definition` then `read_file` instead
/// - You want to find all usages of a symbol → use `find_references` or `grep`
/// - You are looking at a function/class/method **definition** and want to understand it → just `read_file`
///   the surrounding code. Do NOT hover on definition sites — the language server often returns no
///   information or just repeats the signature you can already see.
///
/// **Practical pattern:** When exploring an unfamiliar method, hover on every distinct symbol in
/// parallel — the parameters, each method call, each caught exception, each property access. This
/// builds a complete type map of the code in a few batches of parallel calls, far faster than
/// finding and reading each source file individually.
///
/// Hovering on local variables may return minimal info if the language server cannot infer the type.
/// In that case, fall back to `grep` or `read_file`.
///
/// <example>
/// To get hover information for a symbol by name:
/// {
///     "path": "my_project/src/main.rs",
///     "line": 42,
///     "symbol": "some_function"
/// }
/// </example>
///
/// <example>
/// When the same symbol appears multiple times on a line, provide both `symbol` and `column`
/// to disambiguate. The column does NOT need to be exact — it is fuzzy. The tool picks the
/// occurrence of the symbol nearest to the column you provide, so even a rough estimate works.
/// For example, on `$result->token => $e->paymentToken->token->value`, using `column: 0` would
/// hover over the first `token`, while `column: 99` would hover over the second `token`:
/// {
///     "path": "my_project/src/main.rs",
///     "line": 42,
///     "symbol": "token",
///     "column": 50
/// }
/// </example>
///
/// <guidelines>
/// - Prefer `symbol` over `column` — it is more robust because the tool searches nearby lines
///   if the exact line doesn't match.
/// - When the same symbol appears multiple times on a line, provide both `symbol` and `column`
///   to pick the right occurrence. The `column` is fuzzy — it selects the nearest match, so an
///   approximate value (e.g. "roughly in the second half of the line") is good enough.
/// - If the language server is not available or doesn't support hover, the tool will return an error.
/// - Do NOT use hover on a symbol at its definition site (e.g., hovering on `Payment` in
///   `class Payment { ... }` or on `capturePayment` in `function capturePayment(...)`). The language
///   server returns little or no information for definitions — you are already looking at the definition.
///   Hover is designed for **usages/references** of a symbol, where you want to understand its type or
///   documentation without navigating to the definition.
/// </guidelines>
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LspHoverToolInput {
    /// The relative path of the file containing the symbol.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - lorem
    /// - ipsum
    ///
    /// If you want to hover a symbol in `ipsum/src/dolor.txt`, you should use the path `ipsum/src/dolor.txt`.
    ///
    /// Given an absolute path like `/home/user/code/monorepo/packages/ipsum/src/dolor.txt`:
    /// ✅ Correct: `ipsum/src/dolor.txt`
    /// ❌ Wrong: `packages/ipsum/src/dolor.txt` (includes parent directories above the root)
    /// ❌ Wrong: `src/dolor.txt` (missing the root directory)
    /// </example>
    pub path: String,

    /// Line number (1-based) where the symbol appears.
    pub line: u32,

    /// The symbol name to look up on that line.
    /// Either `symbol` or `column` must be provided.
    #[serde(default)]
    pub symbol: Option<String>,

    /// Approximate column position (0-based) on the line to hover at.
    /// This is fuzzy — the tool picks the occurrence of `symbol` nearest to this column, so even
    /// a rough estimate is useful to disambiguate when the same symbol appears multiple times on
    /// a line. Either `symbol` or `column` must be provided.
    #[serde(default)]
    pub column: Option<u32>,
}

pub struct LspHoverTool {
    project: Entity<Project>,
}

impl LspHoverTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for LspHoverTool {
    type Input = LspHoverToolInput;
    type Output = String;

    const NAME: &'static str = "lsp_hover";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            let target = format_display_symbol(input.symbol.as_deref(), input.column);
            format!(
                "Hover {} in {}",
                MarkdownInlineCode(&target),
                MarkdownInlineCode(&input.path)
            )
            .into()
        } else {
            "LSP Hover".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|e| format!("Failed to receive tool input: {e}"))?;

            validate_lsp_tool_input(
                &input.path,
                input.line,
                input.symbol.as_deref(),
                input.column,
            )?;

            let display_symbol = format_display_symbol(input.symbol.as_deref(), input.column);

            // Find the project path and open the buffer
            let open_buffer_task = project.update(cx, |project, cx| {
                open_buffer_for_path(project, &input.path, cx)
            })?;

            let buffer = futures::select! {
                result = open_buffer_task.fuse() => result.map_err(|e| e.to_string())?,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Hover cancelled by user".to_string());
                }
            };

            // Convert 1-based line to 0-based row
            let row = input.line - 1;

            // Resolve the hover position
            let position = buffer.read_with(cx, |buffer, _cx| {
                let snapshot = buffer.snapshot();
                resolve_position(
                    &snapshot,
                    row,
                    input.line,
                    input.column,
                    input.symbol.as_deref(),
                )
            })?;

            let resolved_line = position.row + 1;

            // Request hover from the LSP
            let hover_task = project.update(cx, |project, cx| project.hover(&buffer, position, cx));

            let hovers = futures::select! {
                result = hover_task.fuse() => result,
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err("Hover cancelled by user".to_string());
                }
            };

            // Format the hover results
            let resolved_line_hint = if resolved_line != input.line {
                format!(
                    " Note: `{display_symbol}` was not found on the requested line {}, \
                     but was found on nearby line {resolved_line}.",
                    input.line,
                )
            } else {
                String::new()
            };

            let Some(hovers) = hovers else {
                return Err(format!(
                    "The language server returned no hover information for `{display_symbol}` \
                     on line {resolved_line}. The symbol was found in the source text, but the \
                     language server could not resolve type or documentation information for it. \
                     This can happen with dynamic/magic properties, unresolved types, or symbols \
                     the language server doesn't understand.{resolved_line_hint}",
                ));
            };

            if hovers.is_empty() {
                return Err(format!(
                    "The language server returned no hover information for `{display_symbol}` \
                     on line {resolved_line}. The symbol was found in the source text, but the \
                     language server could not resolve type or documentation information for it. \
                     This can happen with dynamic/magic properties, unresolved types, or symbols \
                     the language server doesn't understand.{resolved_line_hint}",
                ));
            }

            let mut output = String::new();
            for hover in &hovers {
                if hover.is_empty() {
                    continue;
                }
                for block in &hover.contents {
                    if block.text.is_empty() {
                        continue;
                    }
                    match &block.kind {
                        HoverBlockKind::PlainText => {
                            writeln!(output, "{}", block.text).ok();
                        }
                        HoverBlockKind::Markdown => {
                            writeln!(output, "{}", block.text).ok();
                        }
                        HoverBlockKind::Code { language } => {
                            writeln!(output, "```{}", language).ok();
                            writeln!(output, "{}", block.text).ok();
                            writeln!(output, "```").ok();
                        }
                    }
                }
                if hovers.len() > 1 {
                    writeln!(output, "---").ok();
                }
            }

            let output = output.trim().to_string();
            if output.is_empty() {
                return Err(format!(
                    "The language server returned empty hover content for `{display_symbol}` \
                     on line {resolved_line}. The symbol was found in the source text, but the \
                     language server did not provide any type or documentation \
                     information.{resolved_line_hint}",
                ));
            }

            Ok(output)
        })
    }
}
