use collections::HashMap;
use gpui::{AnyElement, Context, Hsla, ImageSource, RenderImage, Rgba, StyledText, Task, img};
use ratex_layout::{LayoutOptions, layout, to_display_list};
use ratex_parser::parse;
use ratex_svg::{SvgOptions, render_to_svg};
use ratex_types::{color::Color, math_style::MathStyle};
use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::{Arc, OnceLock};
use ui::prelude::*;

use crate::parser::MarkdownEvent;

use super::{Markdown, ParsedMarkdown};

type KatexDiagramCache = HashMap<ParsedMarkdownKatexDiagramContents, Arc<CachedKatexDiagram>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum KatexRenderMode {
    Inline,
    Display,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedMarkdownKatexDiagram {
    pub(crate) source_range: Range<usize>,
    pub(crate) contents: SharedString,
    pub(crate) mode: KatexRenderMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ParsedMarkdownKatexDiagramContents {
    pub(crate) contents: SharedString,
    pub(crate) mode: KatexRenderMode,
    pub(crate) font_size: u32,
    pub(crate) color: [u8; 4],
}

#[derive(Default, Clone)]
pub(crate) struct KatexState {
    pub(crate) cache: KatexDiagramCache,
    occurrences: Vec<KatexOccurrenceState>,
}

#[derive(Clone, Debug)]
struct KatexOccurrenceState {
    source_start: usize,
    contents: SharedString,
    mode: KatexRenderMode,
    rendered_key: Option<ParsedMarkdownKatexDiagramContents>,
}

pub(crate) struct CachedKatexDiagram {
    render_image: Arc<OnceLock<anyhow::Result<Arc<RenderImage>>>>,
    fallback_image: Option<Arc<RenderImage>>,
    _task: Task<()>,
}

impl KatexState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
        self.occurrences.clear();
    }

    pub(crate) fn update(&mut self, parsed: &ParsedMarkdown, _cx: &mut Context<Markdown>) {
        let old_occurrences = std::mem::take(&mut self.occurrences);
        let can_migrate_rendered_keys = old_occurrences.len() == parsed.katex_diagrams.len();

        self.occurrences = parsed
            .katex_diagrams
            .iter()
            .enumerate()
            .map(|(idx, (source_start, katex_diagram))| {
                let rendered_key = can_migrate_rendered_keys
                    .then(|| old_occurrences[idx].rendered_key.clone())
                    .flatten();

                KatexOccurrenceState {
                    source_start: *source_start,
                    contents: katex_diagram.contents.clone(),
                    mode: katex_diagram.mode,
                    rendered_key,
                }
            })
            .collect();

        Self::retain_active_formula_cache_entries(&mut self.cache, &self.occurrences, true);
    }

    fn retain_active_formula_cache_entries(
        cache: &mut KatexDiagramCache,
        active_occurrences: &[KatexOccurrenceState],
        retain_rendered_keys: bool,
    ) {
        cache.retain(|content, _| {
            active_occurrences.iter().any(|occurrence| {
                occurrence.contents == content.contents && occurrence.mode == content.mode
                    || retain_rendered_keys
                        && occurrence
                            .rendered_key
                            .as_ref()
                            .is_some_and(|rendered_key| rendered_key == content)
            })
        });
    }

    fn get_fallback_image_for_source_start(&self, source_start: usize) -> Option<Arc<RenderImage>> {
        let rendered_key = self
            .occurrences
            .iter()
            .find(|occurrence| occurrence.source_start == source_start)?
            .rendered_key
            .as_ref()?;

        Self::cached_fallback_image(&self.cache, rendered_key)
    }

    fn cached_fallback_image(
        cache: &KatexDiagramCache,
        rendered_key: &ParsedMarkdownKatexDiagramContents,
    ) -> Option<Arc<RenderImage>> {
        cache.get(rendered_key).and_then(|cached| {
            cached
                .render_image
                .get()
                .and_then(|result| result.as_ref().ok().cloned())
                .or_else(|| cached.fallback_image.clone())
        })
    }

    fn occurrence_for_source_start_mut(
        &mut self,
        source_start: usize,
    ) -> Option<&mut KatexOccurrenceState> {
        self.occurrences
            .iter_mut()
            .find(|occurrence| occurrence.source_start == source_start)
    }

    pub(crate) fn should_show_source_text_for_contents(
        &self,
        contents: &ParsedMarkdownKatexDiagramContents,
    ) -> bool {
        let Some(cached) = self.cache.get(contents) else {
            return true;
        };

        match cached.render_image.get() {
            Some(Ok(_)) => false,
            Some(Err(_)) => true,
            None => cached.fallback_image.is_none(),
        }
    }

    pub(crate) fn ensure_cached_contents(
        &mut self,
        source_start: usize,
        contents: &ParsedMarkdownKatexDiagramContents,
        cx: &mut Context<Markdown>,
    ) {
        let fallback_image = if self.cache.contains_key(contents) {
            None
        } else {
            self.get_fallback_image_for_source_start(source_start)
        };

        self.cache
            .entry(contents.clone())
            .or_insert_with_key(|contents| {
                Arc::new(CachedKatexDiagram::new(
                    contents.clone(),
                    fallback_image.clone(),
                    cx,
                ))
            });

        if let Some(occurrence) = self.occurrence_for_source_start_mut(source_start) {
            occurrence.rendered_key = Some(contents.clone());
        }
        Self::retain_active_formula_cache_entries(&mut self.cache, &self.occurrences, false);
    }
}

impl CachedKatexDiagram {
    fn new(
        contents: ParsedMarkdownKatexDiagramContents,
        fallback_image: Option<Arc<RenderImage>>,
        cx: &mut Context<Markdown>,
    ) -> Self {
        let render_image = Arc::new(OnceLock::<anyhow::Result<Arc<RenderImage>>>::new());
        let render_image_clone = render_image.clone();
        let svg_renderer = cx.svg_renderer();

        let task = cx.spawn(async move |this, cx| {
            let value = cx
                .background_spawn(async move {
                    let ast = parse(contents.contents.as_ref())
                        .map_err(|error| anyhow::anyhow!("Failed to parse katex: {error}"))?;
                    let layout_options = LayoutOptions::default()
                        .with_style(math_style(contents.mode))
                        .with_color(rgba_bytes_to_ratex_color(contents.color));
                    let layout_box = layout(&ast, &layout_options);
                    let display_list = to_display_list(&layout_box);
                    let svg = render_to_svg(
                        &display_list,
                        &SvgOptions {
                            font_size: contents.font_size as f64 / 100.0,
                            padding: f64::from(padding_for_mode(contents.mode)),
                            stroke_width: 1.0,
                            embed_glyphs: true,
                            ..SvgOptions::default()
                        },
                    );

                    svg_renderer
                        .render_single_frame(svg.as_bytes(), 1.0)
                        .map_err(|error| anyhow::anyhow!("Failed to decode katex render: {error}"))
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
            fallback_image,
            _task: task,
        }
    }

    #[cfg(test)]
    fn new_for_test(
        render_image: Option<Arc<RenderImage>>,
        fallback_image: Option<Arc<RenderImage>>,
    ) -> Self {
        let result = Arc::new(OnceLock::new());
        if let Some(render_image) = render_image {
            let _ = result.set(Ok(render_image));
        }
        Self {
            render_image: result,
            fallback_image,
            _task: Task::ready(()),
        }
    }
}

pub(crate) fn extract_katex_diagrams(
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMarkdownKatexDiagram> {
    let mut katex_diagrams = BTreeMap::default();

    for (source_range, event) in events {
        match event {
            MarkdownEvent::InlineMath(contents) => {
                katex_diagrams.insert(
                    source_range.start,
                    ParsedMarkdownKatexDiagram {
                        source_range: source_range.clone(),
                        contents: contents.clone(),
                        mode: KatexRenderMode::Inline,
                    },
                );
            }
            MarkdownEvent::DisplayMath(contents) => {
                katex_diagrams.insert(
                    source_range.start,
                    ParsedMarkdownKatexDiagram {
                        source_range: source_range.clone(),
                        contents: contents.clone(),
                        mode: KatexRenderMode::Display,
                    },
                );
            }
            _ => {}
        }
    }

    katex_diagrams
}

pub(crate) fn render_display_katex_diagram(
    katex_state: &KatexState,
    contents: &ParsedMarkdownKatexDiagramContents,
    source: &str,
) -> AnyElement {
    div()
        .flex()
        .w_full()
        .items_center()
        .justify_center()
        .py_2()
        .child(render_katex_content(katex_state, contents, source))
        .into_any_element()
}

pub(crate) fn render_inline_katex_diagram(
    katex_state: &KatexState,
    contents: &ParsedMarkdownKatexDiagramContents,
    source: &str,
) -> AnyElement {
    render_katex_content(katex_state, contents, source)
}

fn render_katex_content(
    katex_state: &KatexState,
    contents: &ParsedMarkdownKatexDiagramContents,
    source: &str,
) -> AnyElement {
    let cached = katex_state.cache.get(contents);
    let render_result = cached.and_then(|cached| cached.render_image.get());

    match render_result {
        Some(Ok(image)) => render_katex_image(image.clone(), Some(contents)),
        Some(Err(_)) => render_katex_source_view(source),
        None => cached
            .and_then(|cached| cached.fallback_image.as_ref().cloned())
            .map(|image| render_katex_image(image, Some(contents)))
            .unwrap_or_else(|| render_katex_source_view(source)),
    }
}

fn render_katex_image(
    image: Arc<RenderImage>,
    contents: Option<&ParsedMarkdownKatexDiagramContents>,
) -> AnyElement {
    let image = img(ImageSource::Render(image));
    match contents.map(|contents| contents.mode) {
        Some(KatexRenderMode::Inline) => image.flex_none().into_any_element(),
        _ => image.max_w_full().into_any_element(),
    }
}

fn render_katex_source_view(contents: &str) -> AnyElement {
    StyledText::new(contents.to_string()).into_any()
}

pub(crate) fn cache_contents_for_rendered_formula(
    formula: &ParsedMarkdownKatexDiagram,
    font_size: f32,
    color: Hsla,
) -> ParsedMarkdownKatexDiagramContents {
    let effective_font_size = match formula.mode {
        KatexRenderMode::Inline => font_size * 0.83,
        KatexRenderMode::Display => font_size,
    };
    cache_contents(formula, effective_font_size, color)
}

fn cache_contents(
    formula: &ParsedMarkdownKatexDiagram,
    font_size: f32,
    color: Hsla,
) -> ParsedMarkdownKatexDiagramContents {
    ParsedMarkdownKatexDiagramContents {
        contents: formula.contents.clone(),
        mode: formula.mode,
        font_size: (font_size * 100.0).round() as u32,
        color: hsla_to_rgba_bytes(color),
    }
}

fn math_style(mode: KatexRenderMode) -> MathStyle {
    match mode {
        KatexRenderMode::Inline => MathStyle::Text,
        KatexRenderMode::Display => MathStyle::Display,
    }
}

fn padding_for_mode(mode: KatexRenderMode) -> f32 {
    match mode {
        KatexRenderMode::Inline => 0.5,
        KatexRenderMode::Display => 3.0,
    }
}

fn hsla_to_rgba_bytes(color: Hsla) -> [u8; 4] {
    let rgba: Rgba = color.to_rgb();
    [
        (rgba.r * 255.0).round() as u8,
        (rgba.g * 255.0).round() as u8,
        (rgba.b * 255.0).round() as u8,
        (rgba.a * 255.0).round() as u8,
    ]
}

fn rgba_bytes_to_ratex_color(color: [u8; 4]) -> Color {
    Color::new(
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
        color[3] as f32 / 255.0,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        CachedKatexDiagram, KatexDiagramCache, KatexRenderMode, KatexState,
        ParsedMarkdownKatexDiagram, ParsedMarkdownKatexDiagramContents, extract_katex_diagrams,
    };
    use collections::HashMap;
    use gpui::{Context, IntoElement, Render, RenderImage, TestAppContext, Window};
    use std::sync::Arc;
    use ui::prelude::*;

    use crate::{Markdown, MarkdownOptions, ParsedMarkdown};

    fn with_katex_state(
        source: &str,
        cx: &mut TestAppContext,
        f: impl FnOnce(&mut KatexState, &mut Context<Markdown>),
    ) {
        struct TestWindow;

        impl Render for TestWindow {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }
        }

        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(
                source.into(),
                None,
                None,
                MarkdownOptions {
                    render_embedded_diagrams: true,
                    ..Default::default()
                },
                cx,
            )
        });
        cx.run_until_parked();
        markdown.update(cx, |markdown, cx| {
            f(&mut markdown.katex_state, cx);
        });
    }

    fn parsed_markdown(source: &str) -> ParsedMarkdown {
        let events = crate::parser::parse_markdown_with_options(source, false, false).events;
        ParsedMarkdown {
            source: source.into(),
            katex_diagrams: extract_katex_diagrams(&events),
            events: Arc::from(events),
            ..Default::default()
        }
    }

    fn mock_render_image(cx: &mut TestAppContext) -> Arc<RenderImage> {
        cx.update(|cx| {
            cx.svg_renderer()
                .render_single_frame(
                    br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#,
                    1.0,
                )
                .expect("test svg should render")
        })
    }

    fn katex_diagram(
        contents: &str,
        mode: KatexRenderMode,
        source_start: usize,
    ) -> ParsedMarkdownKatexDiagram {
        ParsedMarkdownKatexDiagram {
            source_range: source_start..source_start,
            contents: contents.into(),
            mode,
        }
    }

    fn katex_contents(
        contents: &str,
        mode: KatexRenderMode,
        font_size: u32,
    ) -> ParsedMarkdownKatexDiagramContents {
        ParsedMarkdownKatexDiagramContents {
            contents: contents.into(),
            mode,
            font_size,
            color: [255, 255, 255, 255],
        }
    }

    fn cache_with_contents(
        contents: impl IntoIterator<Item = ParsedMarkdownKatexDiagramContents>,
    ) -> KatexDiagramCache {
        let mut cache = HashMap::default();
        for contents in contents {
            cache.insert(
                contents,
                Arc::new(CachedKatexDiagram::new_for_test(None, None)),
            );
        }
        cache
    }

    fn retain_cache(
        cache: &mut KatexDiagramCache,
        active_diagrams: impl IntoIterator<Item = ParsedMarkdownKatexDiagram>,
    ) {
        let active_diagrams = active_diagrams.into_iter().collect::<Vec<_>>();
        let active_occurrences = active_diagrams
            .iter()
            .map(|diagram| super::KatexOccurrenceState {
                source_start: diagram.source_range.start,
                contents: diagram.contents.clone(),
                mode: diagram.mode,
                rendered_key: None,
            })
            .collect::<Vec<_>>();
        KatexState::retain_active_formula_cache_entries(cache, &active_occurrences, false);
    }

    fn seed_rendered_cache(
        katex_state: &mut KatexState,
        source_start: usize,
        contents: ParsedMarkdownKatexDiagramContents,
        image: Arc<RenderImage>,
    ) {
        katex_state.cache.insert(
            contents.clone(),
            Arc::new(CachedKatexDiagram::new_for_test(Some(image), None)),
        );
        let occurrence = katex_state
            .occurrence_for_source_start_mut(source_start)
            .expect("occurrence should exist");
        occurrence.rendered_key = Some(contents);
    }

    #[test]
    fn test_extract_katex_diagrams() {
        let markdown = "Inline $x^2$\n\n$$y$$";
        let events = crate::parser::parse_markdown_with_options(markdown, false, false).events;
        let diagrams = extract_katex_diagrams(&events);

        assert_eq!(diagrams.len(), 2);
        let inline = diagrams.values().next().unwrap();
        assert_eq!(inline.contents, "x^2");
        assert_eq!(&markdown[inline.source_range.clone()], "$x^2$");
        assert_eq!(inline.mode, KatexRenderMode::Inline);

        let display = diagrams
            .values()
            .find(|diagram| diagram.mode == KatexRenderMode::Display)
            .unwrap();
        assert_eq!(display.contents, "y");
        assert_eq!(&markdown[display.source_range.clone()], "$$y$$");
    }

    #[gpui::test]
    fn test_katex_fallback_on_edit(cx: &mut TestAppContext) {
        let old_y_start = "$x$ ".len();
        let new_y_start = old_y_start;
        let old_y = katex_contents("y", KatexRenderMode::Inline, 1400);
        let new_y = katex_contents("y^2", KatexRenderMode::Inline, 1400);
        let image_y = mock_render_image(cx);
        let parsed = parsed_markdown("$x$ $y^2$ $z$");

        with_katex_state("$x$ $y$ $z$", cx, |katex_state, cx| {
            seed_rendered_cache(katex_state, old_y_start, old_y, image_y.clone());
            katex_state.update(&parsed, cx);
            katex_state.ensure_cached_contents(new_y_start, &new_y, cx);

            assert_eq!(
                katex_state
                    .cache
                    .get(&new_y)
                    .and_then(|cached| cached.fallback_image.as_ref())
                    .map(|image| image.id),
                Some(image_y.id)
            );
        });
    }

    #[gpui::test]
    fn test_katex_no_fallback_on_add_in_middle(cx: &mut TestAppContext) {
        let y_start = "$x$ ".len();
        let old_z_start = "$x$ ".len();
        let old_z = katex_contents("z", KatexRenderMode::Inline, 1400);
        let new_y = katex_contents("y", KatexRenderMode::Inline, 1400);
        let parsed = parsed_markdown("$x$ $y$ $z$");

        let image = mock_render_image(cx);
        with_katex_state("$x$ $z$", cx, |katex_state, cx| {
            seed_rendered_cache(katex_state, old_z_start, old_z, image);
            katex_state.update(&parsed, cx);
            katex_state.ensure_cached_contents(y_start, &new_y, cx);

            assert_eq!(
                katex_state
                    .cache
                    .get(&new_y)
                    .and_then(|cached| cached.fallback_image.as_ref())
                    .map(|image| image.id),
                None
            );
        });
    }

    #[gpui::test]
    fn test_katex_fallback_chains_on_rapid_edits(cx: &mut TestAppContext) {
        let y_start = "$x$ ".len();
        let old_y = katex_contents("y^2", KatexRenderMode::Inline, 1400);
        let new_y = katex_contents("y^3", KatexRenderMode::Inline, 1400);
        let original = mock_render_image(cx);
        let parsed = parsed_markdown("$x$ $y^3$ $z$");

        with_katex_state("$x$ $y^2$ $z$", cx, |katex_state, cx| {
            seed_rendered_cache(katex_state, y_start, old_y, original.clone());
            katex_state.update(&parsed, cx);
            katex_state.ensure_cached_contents(y_start, &new_y, cx);

            assert_eq!(
                katex_state
                    .cache
                    .get(&new_y)
                    .and_then(|cached| cached.fallback_image.as_ref())
                    .map(|image| image.id),
                Some(original.id)
            );
        });
    }

    #[gpui::test]
    fn test_katex_update_does_not_create_cache_entries(cx: &mut TestAppContext) {
        with_katex_state("$x$", cx, |katex_state, _| {
            assert!(
                katex_state.cache.is_empty(),
                "parse/update should track occurrences but must not create render cache entries"
            );
        });
    }

    #[test]
    fn test_katex_cache_retain_preserves_active_formula_render_variants() {
        let heading_x = katex_contents("x", KatexRenderMode::Inline, 3000);
        let heading_y = katex_contents("y", KatexRenderMode::Inline, 3000);
        let body_x = katex_contents("x", KatexRenderMode::Inline, 1400);
        let mut cache = cache_with_contents([heading_x.clone(), heading_y.clone(), body_x.clone()]);

        retain_cache(
            &mut cache,
            [
                katex_diagram("x", KatexRenderMode::Inline, 0),
                katex_diagram("y", KatexRenderMode::Inline, 4),
            ],
        );

        assert!(cache.contains_key(&heading_x));
        assert!(cache.contains_key(&heading_y));
        assert!(cache.contains_key(&body_x));
    }

    #[test]
    fn test_katex_cache_retain_removes_edited_formula_variants() {
        let old_heading_x = katex_contents("x", KatexRenderMode::Inline, 3000);
        let old_body_x = katex_contents("x", KatexRenderMode::Inline, 1400);
        let unchanged_heading_y = katex_contents("y", KatexRenderMode::Inline, 3000);
        let mut cache = cache_with_contents([
            old_heading_x.clone(),
            old_body_x.clone(),
            unchanged_heading_y.clone(),
        ]);

        retain_cache(
            &mut cache,
            [
                katex_diagram("x^2", KatexRenderMode::Inline, 0),
                katex_diagram("y", KatexRenderMode::Inline, 6),
            ],
        );

        assert!(!cache.contains_key(&old_heading_x));
        assert!(!cache.contains_key(&old_body_x));
        assert!(cache.contains_key(&unchanged_heading_y));
    }

    #[test]
    fn test_katex_cache_retain_removes_stale_formula_variants() {
        let active_x = katex_contents("x", KatexRenderMode::Inline, 1400);
        let stale_heading_z = katex_contents("z", KatexRenderMode::Inline, 3000);
        let stale_body_z = katex_contents("z", KatexRenderMode::Inline, 1400);
        let mut cache = cache_with_contents([
            active_x.clone(),
            stale_heading_z.clone(),
            stale_body_z.clone(),
        ]);

        retain_cache(&mut cache, [katex_diagram("x", KatexRenderMode::Inline, 0)]);

        assert!(cache.contains_key(&active_x));
        assert!(!cache.contains_key(&stale_heading_z));
        assert!(!cache.contains_key(&stale_body_z));
    }
}
