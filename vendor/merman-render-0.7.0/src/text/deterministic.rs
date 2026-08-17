//! Deterministic text measurement and wrapping fallback.

use super::{TextMeasurer, TextMetrics, TextStyle, WrapMode, estimate_line_width_px};

#[derive(Debug, Clone, Default)]
pub struct DeterministicTextMeasurer {
    pub char_width_factor: f64,
    pub line_height_factor: f64,
}

impl DeterministicTextMeasurer {
    fn replace_br_variants(text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut i = 0usize;
        while i < text.len() {
            let Some(rest) = text.get(i..) else {
                break;
            };

            // Mirror Mermaid's `lineBreakRegex = /<br\\s*\\/?>/gi` behavior:
            // - allow ASCII whitespace between `br` and the optional `/` or `>`
            // - do NOT accept extra characters (e.g. `<br \\t/>` should *not* count as a break)
            if rest.starts_with('<') {
                let bytes = text.as_bytes();
                if i + 3 < bytes.len()
                    && matches!(bytes[i + 1], b'b' | b'B')
                    && matches!(bytes[i + 2], b'r' | b'R')
                {
                    let mut j = i + 3;
                    while j < bytes.len() && matches!(bytes[j], b' ' | b'\t' | b'\r' | b'\n') {
                        j += 1;
                    }
                    if j < bytes.len() && bytes[j] == b'/' {
                        j += 1;
                    }
                    if j < bytes.len() && bytes[j] == b'>' {
                        out.push('\n');
                        i = j + 1;
                        continue;
                    }
                }
            }

            let Some(ch) = rest.chars().next() else {
                break;
            };
            out.push(ch);
            i += ch.len_utf8();
        }
        out
    }

    pub fn normalized_text_lines(text: &str) -> Vec<String> {
        let t = Self::replace_br_variants(text);
        let mut out = t.split('\n').map(|s| s.to_string()).collect::<Vec<_>>();

        // Mermaid often produces labels with a trailing newline (e.g. YAML `|` block scalars from
        // FlowDB). The rendered label does not keep an extra blank line at the end, so we trim
        // trailing empty lines to keep height parity.
        while out.len() > 1 && out.last().is_some_and(|s| s.trim().is_empty()) {
            out.pop();
        }

        if out.is_empty() {
            vec!["".to_string()]
        } else {
            out
        }
    }

    pub(crate) fn split_line_to_words(text: &str) -> Vec<String> {
        // Mirrors Mermaid's `splitLineToWords` fallback behavior when `Intl.Segmenter` is absent:
        // split by spaces, then re-add the spaces as separate tokens (preserving multiple spaces).
        let parts = text.split(' ').collect::<Vec<_>>();
        let mut out: Vec<String> = Vec::new();
        for part in parts {
            if !part.is_empty() {
                out.push(part.to_string());
            }
            out.push(" ".to_string());
        }
        while out.last().is_some_and(|s| s == " ") {
            out.pop();
        }
        out
    }

    fn wrap_line(line: &str, max_chars: usize, break_long_words: bool) -> Vec<String> {
        if max_chars == 0 {
            return vec![line.to_string()];
        }

        let mut tokens = std::collections::VecDeque::from(Self::split_line_to_words(line));
        let mut out: Vec<String> = Vec::new();
        let mut cur = String::new();

        while let Some(tok) = tokens.pop_front() {
            if cur.is_empty() && tok == " " {
                continue;
            }

            let candidate = format!("{cur}{tok}");
            if candidate.chars().count() <= max_chars {
                cur = candidate;
                continue;
            }

            if !cur.trim().is_empty() {
                out.push(cur.trim_end().to_string());
                cur.clear();
                tokens.push_front(tok);
                continue;
            }

            // `tok` itself does not fit on an empty line.
            if tok == " " {
                continue;
            }
            if !break_long_words {
                out.push(tok);
            } else {
                // Split it by characters (Mermaid SVG text mode behavior).
                let tok_chars = tok.chars().collect::<Vec<_>>();
                let head: String = tok_chars.iter().take(max_chars.max(1)).collect();
                let tail: String = tok_chars.iter().skip(max_chars.max(1)).collect();
                out.push(head);
                if !tail.is_empty() {
                    tokens.push_front(tail);
                }
            }
        }

        if !cur.trim().is_empty() {
            out.push(cur.trim_end().to_string());
        }

        if out.is_empty() {
            vec!["".to_string()]
        } else {
            out
        }
    }
}

impl TextMeasurer for DeterministicTextMeasurer {
    fn measure(&self, text: &str, style: &TextStyle) -> TextMetrics {
        self.measure_wrapped(text, style, None, WrapMode::SvgLike)
    }

    fn measure_wrapped(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> TextMetrics {
        self.measure_wrapped_impl(text, style, max_width, wrap_mode, true)
            .0
    }

    fn measure_wrapped_with_raw_width(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> (TextMetrics, Option<f64>) {
        self.measure_wrapped_impl(text, style, max_width, wrap_mode, true)
    }

    fn measure_svg_simple_text_bbox_height_px(&self, text: &str, style: &TextStyle) -> f64 {
        let t = text.trim_end();
        if t.is_empty() {
            return 0.0;
        }
        (style.font_size.max(1.0) * 1.1).max(0.0)
    }
}

impl DeterministicTextMeasurer {
    fn measure_wrapped_impl(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
        clamp_html_width: bool,
    ) -> (TextMetrics, Option<f64>) {
        let uses_heuristic_widths = self.char_width_factor == 0.0;
        let char_width_factor = if uses_heuristic_widths {
            match wrap_mode {
                WrapMode::SvgLike | WrapMode::SvgLikeSingleRun => 0.6,
                WrapMode::HtmlLike => 0.5,
            }
        } else {
            self.char_width_factor
        };
        let default_line_height_factor = match wrap_mode {
            WrapMode::SvgLike | WrapMode::SvgLikeSingleRun => 1.1,
            WrapMode::HtmlLike => 1.5,
        };
        let line_height_factor = if self.line_height_factor == 0.0 {
            default_line_height_factor
        } else {
            self.line_height_factor
        };

        let font_size = style.font_size.max(1.0);
        let max_width = max_width.filter(|w| w.is_finite() && *w > 0.0);
        let break_long_words = matches!(wrap_mode, WrapMode::SvgLike | WrapMode::SvgLikeSingleRun);

        let raw_lines = Self::normalized_text_lines(text);
        let mut raw_width: f64 = 0.0;
        for line in &raw_lines {
            let w = if uses_heuristic_widths {
                estimate_line_width_px(line, font_size)
            } else {
                line.chars().count() as f64 * font_size * char_width_factor
            };
            raw_width = raw_width.max(w);
        }
        let needs_wrap =
            wrap_mode == WrapMode::HtmlLike && max_width.is_some_and(|w| raw_width > w);

        let mut lines = Vec::new();
        for line in raw_lines {
            if let Some(w) = max_width {
                let char_px = font_size * char_width_factor;
                let max_chars = ((w / char_px).floor() as isize).max(1) as usize;
                lines.extend(Self::wrap_line(&line, max_chars, break_long_words));
            } else {
                lines.push(line);
            }
        }

        let mut width: f64 = 0.0;
        for line in &lines {
            let w = if uses_heuristic_widths {
                estimate_line_width_px(line, font_size)
            } else {
                line.chars().count() as f64 * font_size * char_width_factor
            };
            width = width.max(w);
        }
        // Mermaid HTML labels use `max-width` and can visually overflow for long words, but their
        // layout width is effectively clamped to the max width. Mirror this to avoid explosive
        // headless widths when `htmlLabels=true`.
        if clamp_html_width && wrap_mode == WrapMode::HtmlLike {
            if let Some(w) = max_width {
                if needs_wrap {
                    width = w;
                } else {
                    width = width.min(w);
                }
            }
        }
        let height = lines.len() as f64 * font_size * line_height_factor;
        let metrics = TextMetrics {
            width,
            height,
            line_count: lines.len(),
        };
        let raw_width_px = if wrap_mode == WrapMode::HtmlLike {
            Some(raw_width)
        } else {
            None
        };
        (metrics, raw_width_px)
    }
}
