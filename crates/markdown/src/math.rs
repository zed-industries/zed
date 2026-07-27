use collections::HashMap;
use gpui::{AnyElement, Context, Entity, RenderImage, Task, StyledText, div};
use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::{Arc, OnceLock};
use ui::prelude::*;

use crate::parser::MarkdownEvent;
use super::{Markdown, MarkdownStyle, ParsedMarkdown};

/// A math expression extracted from parsed markdown events.
#[derive(Clone, Debug)]
pub(crate) struct ParsedMarkdownMathExpression {
    /// Byte range of the full math expression in the source (including delimiters).
    pub(crate) source_range: Range<usize>,
    /// The raw LaTeX content (without delimiters).
    pub(crate) latex: SharedString,
    /// Whether this is a display math expression (`$$...$$` / `\[...\]`).
    pub(crate) is_display: bool,
}

type MathExpressionKey = SharedString;
type MathExpressionCache = HashMap<MathExpressionKey, Arc<CachedMathExpression>>;

#[derive(Default, Clone)]
pub(crate) struct MathState {
    cache: MathExpressionCache,
    order: Vec<MathExpressionKey>,
}

struct CachedMathExpression {
    render_image: Arc<OnceLock<anyhow::Result<Arc<RenderImage>>>>,
    _task: Task<()>,
}

impl MathState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    pub(crate) fn update(&mut self, parsed: &ParsedMarkdown, cx: &mut Context<Markdown>) {
        let mut new_order = Vec::new();
        for expr in parsed.math_expressions.values() {
            new_order.push(expr.latex.clone());
        }

        for latex in &new_order {
            if !self.cache.contains_key(latex) {
                self.cache.insert(
                    latex.clone(),
                    Arc::new(CachedMathExpression::new(latex.clone(), cx)),
                );
            }
        }

        let new_order_set: std::collections::HashSet<_> = new_order.iter().cloned().collect();
        self.cache.retain(|latex, _| new_order_set.contains(latex));
        self.order = new_order;
    }
}

impl CachedMathExpression {
    fn new(latex: SharedString, cx: &mut Context<Markdown>) -> Self {
        let render_image = Arc::new(OnceLock::<anyhow::Result<Arc<RenderImage>>>::new());
        let render_image_clone = render_image.clone();

        // TODO: Replace with actual RaTeX rendering:
        //   let svg_string = ratex_svg::render(latex.as_ref(), font_size, color)?;
        //   svg_renderer.render_single_frame(svg_string.as_bytes(), 1.0)
        //
        // For now, we produce an error result so the fallback path renders the raw LaTeX.
        // This is intentional: the placeholder rendering in MarkdownElement::request_layout
        // handles the fallback display. Once ratex-render is added as a dependency,
        // this task will produce actual rendered images.
        let task = cx.spawn(async move |this, cx| {
            let value = cx
                .background_spawn(async move {
                    Err(anyhow::anyhow!(
                        "LaTeX rendering not yet wired — pending ratex-render integration"
                    ))
                })
                .await;
            let _ = render_image_clone.set(value);
            this.update(cx, |_, cx| {
                cx.notify();
            })
            .ok();
        });

        Self {
            render_image,
            _task: task,
        }
    }
}

/// Renders a math expression as a GPUI element.
///
/// Returns the rendered image if available, or a fallback element showing
/// the raw LaTeX source styled as inline code.
pub(crate) fn render_math_expression(
    parsed: &ParsedMarkdownMathExpression,
    math_state: &MathState,
    style: &MarkdownStyle,
) -> AnyElement {
    let cached = math_state.cache.get(&parsed.latex);
    let render_result = cached.and_then(|cached| cached.render_image.get());

    match render_result {
        Some(Ok(render_image)) => {
            div().child(gpui::img(render_image.clone()).max_h_40()).into_any_element()
        }
        Some(Err(_)) | None => {
            let label = if parsed.is_display {
                format!("$${}$$", parsed.latex)
            } else {
                format!("${}$", parsed.latex)
            };
            div().child(
                StyledText::new(label).with_text_style(style.inline_code.clone()),
            ).into_any_element()
        }
    }
}

/// Extracts math expressions from parsed markdown events.
///
/// Walks the event list looking for `MarkdownEvent::InlineMath` and
/// `MarkdownEvent::DisplayMath` events, collecting them into a map keyed
/// by source offset for O(log n) lookup during rendering.
pub(crate) fn extract_math_expressions(
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMarkdownMathExpression> {
    let mut expressions = BTreeMap::default();

    for (source_range, event) in events {
        match event {
            MarkdownEvent::InlineMath(latex) => {
                expressions.insert(
                    source_range.start,
                    ParsedMarkdownMathExpression {
                        source_range: source_range.clone(),
                        latex: SharedString::from(latex.as_str()),
                        is_display: false,
                    },
                );
            }
            MarkdownEvent::DisplayMath(latex) => {
                expressions.insert(
                    source_range.start,
                    ParsedMarkdownMathExpression {
                        source_range: source_range.clone(),
                        latex: SharedString::from(latex.as_str()),
                        is_display: true,
                    },
                );
            }
            _ => {}
        }
    }

    expressions
}
