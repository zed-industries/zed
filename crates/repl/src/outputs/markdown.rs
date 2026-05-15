use gpui::{
    App, AppContext, ClipboardItem, Context, Entity, StatefulInteractiveElement, Window, prelude::*,
};
use language::Buffer;
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownOptions, MarkdownStyle};
use ui::div;

use crate::outputs::OutputContent;

pub struct MarkdownView {
    markdown: Entity<Markdown>,
    source: String,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut Context<Self>) -> Self {
        Self::from_render_source(text.clone(), normalize_output_markdown(&text), cx)
    }

    pub fn from_latex(text: String, cx: &mut Context<Self>) -> Self {
        Self::from_render_source(text.clone(), latex_to_display_markdown(&text), cx)
    }

    fn from_render_source(source: String, render_source: String, cx: &mut Context<Self>) -> Self {
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(
                render_source.clone().into(),
                None,
                None,
                output_markdown_options(),
                cx,
            )
        });

        Self { markdown, source }
    }
}

fn output_markdown_options() -> MarkdownOptions {
    MarkdownOptions {
        render_math: true,
        ..Default::default()
    }
}

fn latex_to_display_markdown(text: &str) -> String {
    let trimmed = text.trim();
    let source = trimmed
        .strip_prefix("$$")
        .and_then(|text| text.strip_suffix("$$"))
        .or_else(|| {
            trimmed
                .strip_prefix("\\[")
                .and_then(|text| text.strip_suffix("\\]"))
        })
        .or_else(|| {
            trimmed
                .strip_prefix('$')
                .and_then(|text| text.strip_suffix('$'))
        })
        .or_else(|| {
            trimmed
                .strip_prefix("\\(")
                .and_then(|text| text.strip_suffix("\\)"))
        })
        .unwrap_or(trimmed)
        .trim();

    format!("$${source}$$")
}

fn normalize_output_markdown(source: &str) -> String {
    let mut normalized = String::new();
    let mut in_fenced_code_block = false;

    for line in source.split_inclusive('\n') {
        let (line_without_newline, newline) = line
            .strip_suffix('\n')
            .map_or((line, ""), |line| (line, "\n"));
        let trimmed = line_without_newline.trim_start();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fenced_code_block = !in_fenced_code_block;
            normalized.push_str(line_without_newline);
            normalized.push_str(newline);
            continue;
        }

        if in_fenced_code_block
            || trimmed.starts_with('|')
            || trimmed.starts_with('$')
            || trimmed.starts_with('<')
        {
            normalized.push_str(line_without_newline);
            normalized.push_str(newline);
            continue;
        }

        normalized.push_str(&format_loose_math_text(line_without_newline));
        normalized.push_str(newline);
    }

    normalized
}

fn format_loose_math_text(line: &str) -> String {
    if !line_has_loose_math(line) {
        return line.to_string();
    }

    let mut text = line.to_string();
    for (from, to) in [
        ("<=", "≤"),
        (">=", "≥"),
        ("!=", "≠"),
        (" * ", " ⋅ "),
        ("somme", "∑"),
        ("sqrt", "√"),
        ("alpha", "α"),
        ("beta", "β"),
        ("gamma", "γ"),
        ("delta", "δ"),
        ("Delta", "Δ"),
        ("sigma", "σ"),
        ("Sigma", "Σ"),
    ] {
        text = text.replace(from, to);
    }

    text = format_loose_scripts(&text, '^', superscript_char);
    format_loose_scripts(&text, '_', subscript_char)
}

fn line_has_loose_math(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    let has_operator = trimmed.contains('=')
        || trimmed.contains("<=")
        || trimmed.contains(">=")
        || trimmed.contains("sqrt(")
        || trimmed.contains("somme_");
    let has_script = trimmed
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|window| matches!(window, ['_' | '^', next] if next.is_ascii_alphanumeric() || *next == '('));

    has_operator && has_script
}

fn format_loose_scripts(source: &str, marker: char, convert: fn(char) -> Option<char>) -> String {
    let mut output = String::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != marker {
            output.push(ch);
            continue;
        }

        if chars.peek() == Some(&'(') {
            chars.next();
            let mut converted = String::new();
            let mut supported = true;
            for next in chars.by_ref() {
                if next == ')' {
                    break;
                }
                if let Some(converted_char) = convert(next) {
                    converted.push(converted_char);
                } else {
                    supported = false;
                    converted.push(next);
                }
            }
            if supported {
                output.push_str(&converted);
            } else {
                output.push(marker);
                output.push('(');
                output.push_str(&converted);
                output.push(')');
            }
            continue;
        }

        let mut consumed = String::new();
        let mut converted = String::new();
        let mut supported = true;

        while let Some(next) = chars.peek().copied() {
            if !next.is_ascii_alphanumeric() {
                break;
            }
            chars.next();
            consumed.push(next);
            if let Some(converted_char) = convert(next) {
                converted.push(converted_char);
            } else {
                supported = false;
            }
        }

        if consumed.is_empty() {
            output.push(marker);
        } else if supported {
            output.push_str(&converted);
        } else {
            output.push(marker);
            output.push_str(&consumed);
        }
    }

    output
}

fn superscript_char(ch: char) -> Option<char> {
    Some(match ch {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '+' => '⁺',
        '-' => '⁻',
        '=' => '⁼',
        '(' => '⁽',
        ')' => '⁾',
        'i' => 'ⁱ',
        'j' => 'ʲ',
        'k' => 'ᵏ',
        'n' => 'ⁿ',
        _ => return None,
    })
}

fn subscript_char(ch: char) -> Option<char> {
    Some(match ch {
        '0' => '₀',
        '1' => '₁',
        '2' => '₂',
        '3' => '₃',
        '4' => '₄',
        '5' => '₅',
        '6' => '₆',
        '7' => '₇',
        '8' => '₈',
        '9' => '₉',
        '+' => '₊',
        '-' => '₋',
        '=' => '₌',
        '(' => '₍',
        ')' => '₎',
        'a' => 'ₐ',
        'e' => 'ₑ',
        'h' => 'ₕ',
        'i' => 'ᵢ',
        'j' => 'ⱼ',
        'k' => 'ₖ',
        'l' => 'ₗ',
        'm' => 'ₘ',
        'n' => 'ₙ',
        'o' => 'ₒ',
        'p' => 'ₚ',
        'r' => 'ᵣ',
        's' => 'ₛ',
        't' => 'ₜ',
        'u' => 'ᵤ',
        'v' => 'ᵥ',
        'x' => 'ₓ',
        _ => return None,
    })
}

impl OutputContent for MarkdownView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.source.clone()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        let source = self.source.clone();
        let buffer = cx.new(|cx| {
            let mut buffer =
                Buffer::local(source.clone(), cx).with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}

impl Render for MarkdownView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = markdown_style(window, cx);
        div()
            .id(("markdown-output", cx.entity_id()))
            .w_full()
            .overflow_x_scroll()
            .child(MarkdownElement::new(self.markdown.clone(), style))
    }
}

fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let mut style = MarkdownStyle::themed(MarkdownFont::Editor, window, cx);
    style.table_columns_min_size = true;
    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_markdown_enables_math() {
        assert!(output_markdown_options().render_math);
    }

    #[test]
    fn latex_output_is_wrapped_as_display_math() {
        assert_eq!(latex_to_display_markdown("\\sum_i x_i"), "$$\\sum_i x_i$$");
        assert_eq!(
            latex_to_display_markdown("$\\sum_i x_i$"),
            "$$\\sum_i x_i$$"
        );
        assert_eq!(
            latex_to_display_markdown("\\[\\sum_i x_i\\]"),
            "$$\\sum_i x_i$$"
        );
    }

    #[test]
    fn loose_math_output_tokens_are_normalized() {
        let source = "min C_max = min(max_j L_j) avec L_j = somme_i t_ij * x_ij";

        assert_eq!(
            normalize_output_markdown(source),
            "min Cₘₐₓ = min(maxⱼ Lⱼ) avec Lⱼ = ∑ᵢ tᵢⱼ ⋅ xᵢⱼ"
        );
    }

    #[test]
    fn loose_math_normalization_skips_tables_and_code_blocks() {
        let source = "```text\nx_ij = 1\n```\n| x_ij | value |\n| --- | --- |\n";

        assert_eq!(normalize_output_markdown(source), source);
    }
}
