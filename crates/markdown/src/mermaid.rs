use collections::HashMap;
use gpui::{
    Animation, AnimationExt, AnyElement, App, ClipboardItem, Context, Entity, ImageSource,
    ParsedSvg, RenderImage, SMOOTH_SVG_SCALE_FACTOR, ScrollDelta, ScrollHandle, ScrollWheelEvent,
    Size, Stateful, StyledText, Task, Window, img, pulsating_between, size,
};
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use ui::{CopyButton, ScrollAxes, Scrollbars, TintColor, WithScrollbar, prelude::*};

use crate::parser::{CodeBlockKind, MarkdownEvent, MarkdownTag};
use settings::Settings as _;
use theme_settings::ThemeSettings;

use super::{CopyButtonVisibility, Markdown, MarkdownStyle, ParsedMarkdown};

type MermaidDiagramCache = HashMap<ParsedMarkdownMermaidDiagramContents, Arc<CachedMermaidDiagram>>;

/// Per scroll tick, zoom changes by 10 percentage points, regardless of how
/// large a delta the platform reports for the tick.
const MERMAID_ZOOM_STEP: f32 = 0.1;
/// How many pixels of precise (trackpad) scroll make up one zoom tick.
const PIXELS_PER_ZOOM_TICK: f32 = 20.0;

#[derive(Clone, Debug)]
pub(crate) struct ParsedMarkdownMermaidDiagram {
    pub(crate) content_range: Range<usize>,
    pub(crate) contents: ParsedMarkdownMermaidDiagramContents,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ParsedMarkdownMermaidDiagramContents {
    pub(crate) contents: SharedString,
    pub(crate) scale: u32,
}

#[derive(Default, Clone)]
pub(crate) struct MermaidState {
    cache: MermaidDiagramCache,
    order: Vec<ParsedMarkdownMermaidDiagramContents>,
}

struct CachedMermaidDiagram {
    render_image: Arc<OnceLock<anyhow::Result<Arc<RenderImage>>>>,
    parsed_svg: Arc<OnceLock<Arc<ParsedSvg>>>,
    /// The previous raster shown while `render_image` is pending, along with
    /// the scale it was rasterized at, so it can be displayed at the same
    /// logical size as the raster that will replace it.
    fallback_image: Option<(Arc<RenderImage>, f32)>,
    /// The scale, relative to the diagram's natural size, that `render_image`
    /// was (or is being) rasterized at.
    rasterized_scale: f32,
    _task: Task<()>,
}

impl MermaidState {
    pub(crate) fn clear(&mut self, cx: &mut App) {
        for cached in self.cache.values() {
            cached.drop_images(cx);
        }
        self.cache.clear();
        self.order.clear();
    }

    fn get_fallback_image(
        idx: usize,
        old_order: &[ParsedMarkdownMermaidDiagramContents],
        new_order_len: usize,
        cache: &MermaidDiagramCache,
    ) -> Option<(Arc<RenderImage>, f32)> {
        if old_order.len() != new_order_len {
            return None;
        }

        old_order.get(idx).and_then(|old_content| {
            cache.get(old_content).and_then(|old_cached| {
                old_cached
                    .render_image
                    .get()
                    .and_then(|result| result.as_ref().ok().cloned())
                    .map(|image| (image, old_cached.rasterized_scale))
                    .or_else(|| old_cached.fallback_image.clone())
            })
        })
    }

    pub(crate) fn update(
        &mut self,
        parsed: &ParsedMarkdown,
        zoom_for_offset: impl Fn(usize) -> f32,
        cx: &mut Context<Markdown>,
    ) {
        let mut new_order = Vec::new();
        let mut source_offsets = Vec::new();
        for (source_offset, mermaid_diagram) in parsed.mermaid_diagrams.iter() {
            new_order.push(mermaid_diagram.contents.clone());
            source_offsets.push(*source_offset);
        }

        for (idx, new_content) in new_order.iter().enumerate() {
            if !self.cache.contains_key(new_content) {
                let fallback =
                    Self::get_fallback_image(idx, &self.order, new_order.len(), &self.cache);
                // The cache is keyed by contents, so duplicate diagrams share
                // one entry; if duplicates have different zooms, the zoom of
                // the first occurrence wins.
                let zoom = source_offsets
                    .get(idx)
                    .copied()
                    .map_or(1.0, &zoom_for_offset);
                self.cache.insert(
                    new_content.clone(),
                    Arc::new(CachedMermaidDiagram::new(
                        new_content.clone(),
                        zoom,
                        fallback,
                        cx,
                    )),
                );
            }
        }

        let new_order_set: std::collections::HashSet<_> = new_order.iter().cloned().collect();
        self.cache.retain(|content, cached| {
            let keep = new_order_set.contains(content);
            if !keep {
                cached.drop_images(cx);
            }
            keep
        });
        self.order = new_order;
    }

    /// The natural (zoom 1.0) logical size of a diagram, if any raster of it
    /// is available to recover the size from.
    pub(crate) fn natural_size(
        &self,
        contents: &ParsedMarkdownMermaidDiagramContents,
    ) -> Option<Size<Pixels>> {
        let cached = self.cache.get(contents)?;
        let (image, rasterized_scale) = cached
            .render_image
            .get()
            .and_then(|result| result.as_ref().ok())
            .map(|image| (image.clone(), cached.rasterized_scale))
            .or_else(|| cached.fallback_image.clone())?;
        Some(mermaid_base_size(&image, rasterized_scale))
    }

    /// Re-rasterizes a single cached diagram at exactly the scale it is
    /// displayed at (`zoom` times its natural size), reusing the cached
    /// parsed SVG so that neither mermaid layout nor SVG parsing is re-run.
    /// No-ops if the entry is already rasterized at the target scale, or if
    /// it's still mid-render or failed to render.
    pub(crate) fn rerasterize_diagram(
        &mut self,
        contents: &ParsedMarkdownMermaidDiagramContents,
        zoom: f32,
        cx: &mut Context<Markdown>,
    ) {
        let Some(cached) = self.cache.get_mut(contents) else {
            return;
        };
        let Some(parsed_svg) = cached.parsed_svg.get().cloned() else {
            return;
        };
        let target_scale = zoom;
        if (cached.rasterized_scale - target_scale).abs() < 0.001 {
            return;
        }
        // The old render image lives on as the new entry's fallback, so it
        // must not be dropped here; the old fallback is no longer painted.
        // If the old raster is still pending, keep showing the previous
        // fallback so the diagram doesn't disappear mid-zoom.
        let new_fallback = match cached
            .render_image
            .get()
            .and_then(|result| result.as_ref().ok())
        {
            Some(image) => {
                if let Some((old_fallback, _)) = cached.fallback_image.clone() {
                    cx.drop_image(old_fallback, None);
                }
                Some((image.clone(), cached.rasterized_scale))
            }
            None => cached.fallback_image.clone(),
        };
        let scale_factor = contents.scale as f32 / 100.0 * target_scale;
        *cached = Arc::new(CachedMermaidDiagram::new_from_parsed(
            parsed_svg,
            scale_factor,
            target_scale,
            new_fallback,
            cx,
        ));
    }
}

impl CachedMermaidDiagram {
    fn new(
        contents: ParsedMarkdownMermaidDiagramContents,
        zoom: f32,
        fallback_image: Option<(Arc<RenderImage>, f32)>,
        cx: &mut Context<Markdown>,
    ) -> Self {
        let render_image = Arc::new(OnceLock::<anyhow::Result<Arc<RenderImage>>>::new());
        let parsed_svg = Arc::new(OnceLock::<Arc<ParsedSvg>>::new());
        let svg_renderer = cx.svg_renderer();
        let mermaid_theme = build_mermaid_theme(cx);

        let task = cx.spawn({
            let render_image = render_image.clone();
            let parsed_svg = parsed_svg.clone();
            let fallback_image = fallback_image.clone();
            async move |this, cx| {
                let value = cx
                    .background_spawn(async move {
                        let svg_string =
                            mermaid_render::render_to_svg(&contents.contents, &mermaid_theme)?;
                        let tree = svg_renderer
                            .parse_svg(svg_string.as_bytes())
                            .map_err(|error| anyhow::anyhow!("{error}"))?;
                        let tree = Arc::new(tree);
                        let scale = contents.scale as f32 / 100.0 * zoom;
                        let image = svg_renderer
                            .render_parsed(&tree, scale)
                            .map_err(|error| anyhow::anyhow!("{error}"))?;
                        anyhow::Ok((tree, image))
                    })
                    .await;
                let value = match value {
                    Ok((tree, image)) => {
                        parsed_svg.set(tree).ok();
                        Ok(image)
                    }
                    Err(error) => Err(error),
                };
                render_image.set(value).ok();
                Self::on_render_complete(this, fallback_image, cx);
            }
        });

        Self {
            render_image,
            parsed_svg,
            fallback_image,
            rasterized_scale: zoom,
            _task: task,
        }
    }

    fn new_from_parsed(
        parsed_svg: Arc<ParsedSvg>,
        scale_factor: f32,
        rasterized_scale: f32,
        fallback_image: Option<(Arc<RenderImage>, f32)>,
        cx: &mut Context<Markdown>,
    ) -> Self {
        let render_image = Arc::new(OnceLock::<anyhow::Result<Arc<RenderImage>>>::new());
        let parsed_svg_cell = Arc::new(OnceLock::new());
        parsed_svg_cell.set(parsed_svg.clone()).ok();
        let svg_renderer = cx.svg_renderer();

        let task = cx.spawn({
            let render_image = render_image.clone();
            let fallback_image = fallback_image.clone();
            async move |this, cx| {
                let value = cx
                    .background_spawn(async move {
                        svg_renderer
                            .render_parsed(&parsed_svg, scale_factor)
                            .map_err(|error| anyhow::anyhow!("{error}"))
                    })
                    .await;
                render_image.set(value).ok();
                Self::on_render_complete(this, fallback_image, cx);
            }
        });

        Self {
            render_image,
            parsed_svg: parsed_svg_cell,
            fallback_image,
            rasterized_scale,
            _task: task,
        }
    }

    fn on_render_complete(
        this: gpui::WeakEntity<Markdown>,
        fallback_image: Option<(Arc<RenderImage>, f32)>,
        cx: &mut gpui::AsyncApp,
    ) {
        this.update(cx, |_, cx| {
            // The fallback will no longer be painted now that the real render
            // is available, so its GPU texture can be released.
            if let Some((fallback_image, _)) = fallback_image {
                cx.drop_image(fallback_image, None);
            }
            cx.notify();
        })
        .ok();
    }

    fn drop_images(&self, cx: &mut App) {
        if let Some(Ok(render_image)) = self.render_image.get() {
            cx.drop_image(render_image.clone(), None);
        }
        if let Some((fallback_image, _)) = self.fallback_image.clone() {
            cx.drop_image(fallback_image, None);
        }
    }

    #[cfg(test)]
    fn new_for_test(
        render_image: Option<Arc<RenderImage>>,
        fallback_image: Option<Arc<RenderImage>>,
        parsed_svg: Option<Arc<ParsedSvg>>,
    ) -> Self {
        let result = Arc::new(OnceLock::new());
        if let Some(render_image) = render_image {
            let _ = result.set(Ok(render_image));
        }
        let parsed_svg_cell = Arc::new(OnceLock::new());
        if let Some(parsed_svg) = parsed_svg {
            let _ = parsed_svg_cell.set(parsed_svg);
        }
        Self {
            render_image: result,
            parsed_svg: parsed_svg_cell,
            fallback_image: fallback_image.map(|image| (image, 1.0)),
            rasterized_scale: 1.0,
            _task: Task::ready(()),
        }
    }
}

/// Merman has somewhat limited text measurement capabilities.
///
/// When it doesn't have metrics for any of the specified fonts, it chooses a
/// fairly narrow width, which causes visible overflow. Adding `sans-serif`
/// allows it to fall back to a more conservative (i.e. wider) measurement.
///
/// This isn't perfect - very wide fonts will likely still cause overflow. A
/// proper fix would involve somehow piping `resvg`'s actual measurements into
/// `merman`, but that is a lot of work for a fairly uncommon edge case.
fn mermaid_font_family(font_family: &str) -> String {
    let font_family = gpui::font_name_with_fallbacks(font_family, "system-ui");
    if font_family
        .split(',')
        .any(|family| family.trim().eq_ignore_ascii_case("sans-serif"))
    {
        font_family.to_string()
    } else {
        format!("{font_family}, sans-serif")
    }
}

fn build_mermaid_theme(cx: &Context<Markdown>) -> mermaid_render::MermaidTheme {
    let colors = cx.theme().colors();
    let theme_settings = ThemeSettings::get_global(cx);
    let is_dark = !cx.theme().appearance.is_light();

    let players = cx.theme().players();
    let git_branch_colors = std::array::from_fn(|i| players.0[i % players.0.len()].cursor);
    let git_branch_label_colors = git_branch_colors.map(mermaid_render::text_color_for_background);

    mermaid_render::MermaidTheme {
        dark_mode: is_dark,
        font_family: mermaid_font_family(theme_settings.ui_font.family.as_ref()),
        background: colors.editor_background,
        primary_color: colors.surface_background,
        primary_text_color: colors.text,
        primary_border_color: colors.border,
        secondary_color: colors.element_background,
        tertiary_color: colors.ghost_element_hover,
        line_color: colors.border,
        text_color: colors.text,
        edge_label_background: colors.editor_background,
        cluster_background: colors.panel_background,
        cluster_border: colors.border_variant,
        note_background: colors.surface_background,
        note_border: colors.border_variant,
        actor_background: colors.element_background,
        actor_border: colors.border,
        activation_background: colors.ghost_element_hover,
        activation_border: colors.border,
        git_branch_colors,
        git_branch_label_colors,
        er_attr_bg_odd: colors.surface_background,
        er_attr_bg_even: colors.element_background,
        error_color: cx.theme().status().error,
        warning_color: cx.theme().status().warning,
        accent_colors: players
            .0
            .iter()
            .map(|player| mermaid_render::AccentColor {
                foreground: player.cursor,
                background: player.background,
            })
            .collect(),
    }
}

fn parse_mermaid_info(info: &str) -> Option<u32> {
    let mut parts = info.split_whitespace();
    if parts.next()? != "mermaid" {
        return None;
    }

    Some(
        parts
            .next()
            .and_then(|scale| scale.parse().ok())
            .unwrap_or(100)
            .clamp(10, 500),
    )
}

/// We deliberately block rendering of some diagram types, even though `merman`
/// supports them, because we have not yet written custom CSS to ensure text is
/// readable.
fn is_supported_diagram_type(source: &str) -> bool {
    /// If updating this list, also update the system prompt!
    const SUPPORTED_PREFIXES: &[&str] = &[
        "flowchart",
        "graph",
        "sequenceDiagram",
        "classDiagram",
        "stateDiagram",
        "stateDiagram-v2",
        "erDiagram",
        "gantt",
        "pie",
        "gitGraph",
        "mindmap",
        "timeline",
        "quadrantChart",
        "xychart-beta",
        "journey",
    ];
    let first_token = source
        .trim_start()
        .split(|c: char| c.is_whitespace() || c == '\n')
        .next()
        .unwrap_or("");
    SUPPORTED_PREFIXES
        .iter()
        .any(|prefix| first_token.eq_ignore_ascii_case(prefix))
}

pub(crate) fn extract_mermaid_diagrams(
    source: &str,
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMarkdownMermaidDiagram> {
    let mut mermaid_diagrams = BTreeMap::default();

    for (source_range, event) in events {
        let MarkdownEvent::Start(MarkdownTag::CodeBlock { kind, metadata }) = event else {
            continue;
        };
        if !metadata.is_fenced_closed {
            continue;
        }
        let scale = match kind {
            CodeBlockKind::FencedLang(info) => match parse_mermaid_info(info.as_ref()) {
                Some(scale) => scale,
                None => continue,
            },
            CodeBlockKind::FencedSrc(path_range) => {
                let path = Path::new(path_range.path.as_ref());
                match path.extension().and_then(|ext| ext.to_str()) {
                    Some("mermaid" | "mmd") => 100,
                    _ => continue,
                }
            }
            _ => continue,
        };

        let contents = source[metadata.content_range.clone()]
            .strip_suffix('\n')
            .unwrap_or(&source[metadata.content_range.clone()])
            .to_string();
        if !is_supported_diagram_type(&contents) {
            continue;
        }
        mermaid_diagrams.insert(
            source_range.start,
            ParsedMarkdownMermaidDiagram {
                content_range: metadata.content_range.clone(),
                contents: ParsedMarkdownMermaidDiagramContents {
                    contents: contents.into(),
                    scale,
                },
            },
        );
    }

    mermaid_diagrams
}

/// The natural (zoom 1.0, unfitted) logical size of a diagram, recovered from
/// one of its rasters.
///
/// Mermaid rasters always come from `SvgRenderer::render_parsed`, whose images
/// have a device scale of [`SMOOTH_SVG_SCALE_FACTOR`]; dividing by it and by
/// the scale the raster was made at recovers the natural size, regardless of
/// which raster is currently cached.
fn mermaid_base_size(image: &RenderImage, rasterized_scale: f32) -> Size<Pixels> {
    let device_size = image.size(0);
    let device_scale = SMOOTH_SVG_SCALE_FACTOR * rasterized_scale;
    size(
        px(device_size.width.0 as f32 / device_scale),
        px(device_size.height.0 as f32 / device_scale),
    )
}

fn mermaid_display_size(base_size: Size<Pixels>, display_scale: f32) -> Size<Pixels> {
    size(
        base_size.width * display_scale,
        base_size.height * display_scale,
    )
}

/// The number of zoom ticks represented by a scroll-wheel event.
///
/// A discrete wheel notch arrives as one `Lines` event whose magnitude varies
/// by platform and device (1 or 3 lines are both common), so the delta is
/// clamped to a single tick: every notch is exactly one step. Precise (pixel)
/// deltas from trackpads arrive as many small events, each contributing a
/// fractional tick, so a gesture zooms smoothly at the same overall rate.
fn mermaid_zoom_ticks(delta: ScrollDelta) -> f32 {
    match delta {
        ScrollDelta::Lines(lines) => lines.y,
        ScrollDelta::Pixels(pixels) => f32::from(pixels.y) / PIXELS_PER_ZOOM_TICK,
    }
    .clamp(-1.0, 1.0)
}

fn on_mermaid_zoom_scroll(
    markdown: Entity<Markdown>,
    source_offset: usize,
) -> impl Fn(&ScrollWheelEvent, &mut Window, &mut App) + 'static {
    move |event, _window, cx| {
        if !(event.modifiers.control || event.modifiers.platform) {
            return;
        }
        let scroll_ticks = mermaid_zoom_ticks(event.delta);
        if scroll_ticks != 0.0 {
            markdown.update(cx, |markdown, cx| {
                let current_zoom = markdown.mermaid_zoom_level(source_offset);
                let new_zoom = current_zoom + scroll_ticks * MERMAID_ZOOM_STEP;
                markdown.set_mermaid_zoom_level(source_offset, new_zoom, cx);
            });
        }
        cx.stop_propagation();
    }
}

pub(crate) fn render_mermaid_diagram(
    parsed: &ParsedMarkdownMermaidDiagram,
    mermaid_state: &MermaidState,
    style: &MarkdownStyle,
    markdown: Entity<Markdown>,
    source_offset: usize,
    showing_code: bool,
    zoom: f32,
    copy_button_visibility: CopyButtonVisibility,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let cached = mermaid_state.cache.get(&parsed.contents);
    let render_result = cached.and_then(|cached| cached.render_image.get());
    let show_interactive = copy_button_visibility != CopyButtonVisibility::Hidden;

    let code = parsed.contents.contents.clone();

    let mut container = div().group("code_block").relative().w_full().rounded_lg();
    container.style().refine(&style.code_block);

    match render_result {
        Some(Ok(render_image)) => {
            let body = if showing_code {
                render_mermaid_code_view(&parsed.contents.contents)
            } else {
                let rasterized_scale = cached.map_or(1.0, |cached| cached.rasterized_scale);
                let image_element =
                    img(ImageSource::Render(render_image.clone())).with_fallback(|| {
                        Label::new("Failed to Load Mermaid Diagram").into_any_element()
                    });
                let scroll_handle = markdown.update(cx, |markdown, _| {
                    markdown.mermaid_scroll_handle(source_offset)
                });
                let base_size = mermaid_base_size(render_image, rasterized_scale);
                let display_size = mermaid_display_size(base_size, zoom);
                let body =
                    mermaid_scroll_container(markdown.clone(), source_offset, &scroll_handle)
                        .child(image_element.w(display_size.width).h(display_size.height))
                        .into_any_element();
                with_mermaid_horizontal_scrollbar(source_offset, &scroll_handle, body, window, cx)
            };

            container
                .when(show_interactive, |container| {
                    container.child(render_mermaid_tab_header(
                        source_offset,
                        showing_code,
                        markdown.clone(),
                    ))
                })
                .child(body)
                .when(show_interactive, |container| {
                    container.child(render_mermaid_copy_button(
                        source_offset,
                        code.to_string(),
                        markdown,
                    ))
                })
                .into_any_element()
        }
        Some(Err(_)) => {
            // Render failed — show the source code without tabs
            container
                .child(render_mermaid_code_view(&parsed.contents.contents))
                .when(show_interactive, |container| {
                    container.child(render_mermaid_copy_button(
                        source_offset,
                        code.to_string(),
                        markdown,
                    ))
                })
                .into_any_element()
        }
        None => {
            // Still rendering
            if let Some((fallback, fallback_scale)) =
                cached.and_then(|cached| cached.fallback_image.clone())
            {
                let pulse = Animation::new(Duration::from_secs(2))
                    .repeat()
                    .with_easing(pulsating_between(0.6, 1.0));
                let fallback_element =
                    img(ImageSource::Render(fallback.clone())).with_fallback(|| {
                        div()
                            .child(Label::new("Failed to load mermaid diagram"))
                            .into_any_element()
                    });
                let scroll_handle = markdown.update(cx, |markdown, _| {
                    markdown.mermaid_scroll_handle(source_offset)
                });
                let base_size = mermaid_base_size(&fallback, fallback_scale);
                let display_size = mermaid_display_size(base_size, zoom);
                let inner =
                    mermaid_scroll_container(markdown.clone(), source_offset, &scroll_handle)
                        .child(
                            fallback_element
                                .w(display_size.width)
                                .h(display_size.height),
                        )
                        .with_animation("mermaid-fallback-pulse", pulse, |element, delta| {
                            element.opacity(delta)
                        })
                        .into_any_element();
                let body = with_mermaid_horizontal_scrollbar(
                    source_offset,
                    &scroll_handle,
                    inner,
                    window,
                    cx,
                );
                container
                    .child(body)
                    .when(show_interactive, |container| {
                        container.child(render_mermaid_copy_button(
                            source_offset,
                            code.to_string(),
                            markdown,
                        ))
                    })
                    .into_any_element()
            } else {
                // No fallback — show the code so the user has something to look at
                container
                    .child(render_mermaid_code_view(&parsed.contents.contents))
                    .child(
                        div().absolute().top_1().right_2().child(
                            Label::new("Rendering...")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .with_animation(
                                    "mermaid-loading-pulse",
                                    Animation::new(Duration::from_secs(2))
                                        .repeat()
                                        .with_easing(pulsating_between(0.4, 0.8)),
                                    |label, delta| label.alpha(delta),
                                ),
                        ),
                    )
                    .when(show_interactive, |container| {
                        container.child(render_mermaid_copy_button(
                            source_offset,
                            code.to_string(),
                            markdown,
                        ))
                    })
                    .into_any_element()
            }
        }
    }
}

/// The horizontal scroll container wrapping a mermaid raster. The element id
/// and the [`ScrollHandle`] identity are both stable per source offset so the
/// scroll position survives raster swaps and re-rasters.
fn mermaid_scroll_container(
    markdown: Entity<Markdown>,
    source_offset: usize,
    scroll_handle: &ScrollHandle,
) -> Stateful<Div> {
    let mut container = div()
        .id(ElementId::named_usize(
            "mermaid-diagram-body",
            source_offset,
        ))
        .w_full()
        .overflow_x_scroll()
        .track_scroll(scroll_handle)
        .on_scroll_wheel(on_mermaid_zoom_scroll(markdown, source_offset));
    // Without this, gpui maps vertical wheel deltas onto the x axis for
    // x-only scroll containers (see `paint_scroll_listener` in gpui's div),
    // hijacking plain vertical scrolls. Restricting to the actual axis lets
    // vertical deltas propagate to the surrounding view, and means a
    // ctrl+vertical-scroll (zoom gesture) never moves this container either,
    // since its delta.x is zero.
    container.style().restrict_scroll_to_axis = Some(true);
    container
}

fn with_mermaid_horizontal_scrollbar(
    source_offset: usize,
    scroll_handle: &ScrollHandle,
    body: AnyElement,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    // Always show the scrollbar (rather than autohiding) so it's clear a
    // zoomed diagram can be scrolled. It only appears when the content
    // actually overflows, since the thumb isn't drawn when there's nothing to
    // scroll.
    let scrollbars = Scrollbars::always_visible(ScrollAxes::Horizontal)
        .id(("mermaid-diagram-scrollbar", source_offset))
        .tracked_scroll_handle(scroll_handle)
        .with_track_along(
            ScrollAxes::Horizontal,
            cx.theme().colors().editor_background,
        )
        .notify_content();

    div()
        .w_full()
        .custom_scrollbars(scrollbars, window, cx)
        .child(body)
        .into_any_element()
}

fn render_mermaid_tab_header(
    source_offset: usize,
    showing_code: bool,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let preview_markdown = markdown.clone();
    let code_markdown = markdown;

    h_flex()
        .gap_0p5()
        .mb_2p5()
        .child(
            Button::new(
                ElementId::named_usize("mermaid-tab-preview", source_offset),
                "Preview",
            )
            .label_size(LabelSize::Small)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(!showing_code)
            .on_click(move |_event, _window, cx| {
                preview_markdown.update(cx, |md, cx| {
                    if md.is_mermaid_showing_code(source_offset) {
                        md.toggle_mermaid_tab(source_offset);
                        cx.notify();
                    }
                });
            }),
        )
        .child(
            Button::new(
                ElementId::named_usize("mermaid-tab-code", source_offset),
                "Code",
            )
            .label_size(LabelSize::Small)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(showing_code)
            .on_click(move |_event, _window, cx| {
                code_markdown.update(cx, |md, cx| {
                    if !md.is_mermaid_showing_code(source_offset) {
                        md.toggle_mermaid_tab(source_offset);
                        cx.notify();
                    }
                });
            }),
        )
}

fn render_mermaid_copy_button(
    source_offset: usize,
    code: String,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let id = ElementId::named_usize("copy-mermaid-code", source_offset);

    div().absolute().top_1().right_1().justify_end().child(
        CopyButton::new(id.clone(), code.clone())
            .visible_on_hover("code_block")
            .custom_on_click({
                move |_window, cx| {
                    let id = id.clone();
                    markdown.update(cx, |this, cx| {
                        this.copied_code_blocks.insert(id.clone());
                        cx.write_to_clipboard(ClipboardItem::new_string(code.clone()));
                        cx.spawn(async move |this, cx| {
                            cx.background_executor().timer(Duration::from_secs(2)).await;
                            cx.update(|cx| {
                                this.update(cx, |this, cx| {
                                    this.copied_code_blocks.remove(&id);
                                    cx.notify();
                                })
                            })
                            .ok();
                        })
                        .detach();
                    });
                }
            }),
    )
}

fn render_mermaid_code_view(contents: &SharedString) -> AnyElement {
    div()
        .w_full()
        .child(StyledText::new(contents.clone()))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::{
        CachedMermaidDiagram, MermaidDiagramCache, MermaidState, ParsedMarkdownMermaidDiagram,
        ParsedMarkdownMermaidDiagramContents, extract_mermaid_diagrams, parse_mermaid_info,
    };
    use crate::{
        CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownOptions,
        MarkdownStyle, WrapButtonVisibility,
    };
    use collections::HashMap;
    use gpui::{
        Context, Entity, IntoElement, Render, RenderImage, TestAppContext, Window, point, size,
    };
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::Duration;
    use ui::prelude::*;

    fn ensure_theme_initialized(cx: &mut TestAppContext) {
        cx.update(|cx| {
            if !cx.has_global::<settings::SettingsStore>() {
                settings::init(cx);
            }
            if !cx.has_global::<theme::GlobalTheme>() {
                theme_settings::init(theme::LoadThemes::JustBase, cx);
            }
        });
    }

    /// Renders a [`MarkdownElement`] beneath a throwaway view (mirroring how
    /// elements are always rendered in production) and captures the
    /// [`crate::RenderedText`] produced by its layout. Mermaid diagram bodies
    /// are scroll containers, whose paint requires a current view, so the
    /// element can't be drawn bare with `cx.draw`.
    fn draw_markdown_element(
        markdown: Entity<Markdown>,
        cx: &mut gpui::VisualTestContext,
    ) -> crate::RenderedText {
        struct CaptureRenderedText {
            markdown: Entity<Markdown>,
            rendered_text: Rc<RefCell<Option<crate::RenderedText>>>,
        }

        impl Render for CaptureRenderedText {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let element = MarkdownElement::new(self.markdown.clone(), MarkdownStyle::default())
                    .code_block_renderer(CodeBlockRenderer::Default {
                        copy_button_visibility: CopyButtonVisibility::Hidden,
                        wrap_button_visibility: WrapButtonVisibility::Hidden,
                        border: false,
                    })
                    .on_render({
                        let rendered_text = self.rendered_text.clone();
                        move |text| *rendered_text.borrow_mut() = Some(text)
                    });
                div().child(element)
            }
        }

        let rendered_text = Rc::new(RefCell::new(None));
        cx.draw(Default::default(), size(px(600.0), px(600.0)), {
            let rendered_text = rendered_text.clone();
            |_window, cx| {
                cx.new(|_| CaptureRenderedText {
                    markdown,
                    rendered_text,
                })
                .into_any_element()
            }
        });
        let rendered_text = rendered_text.borrow_mut().take();
        rendered_text.expect("markdown element should have been laid out")
    }

    fn render_markdown_with_options(
        markdown: &str,
        options: MarkdownOptions,
        cx: &mut TestAppContext,
    ) -> crate::RenderedText {
        struct TestWindow;

        impl Render for TestWindow {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }
        }

        ensure_theme_initialized(cx);

        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(markdown.to_string().into(), None, None, options, cx)
        });
        cx.run_until_parked();
        draw_markdown_element(markdown, cx)
    }

    fn mock_render_image(cx: &mut TestAppContext) -> Arc<RenderImage> {
        cx.update(|cx| {
            cx.svg_renderer()
                .render_single_frame(
                    br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#,
                    1.0,
                )
                .unwrap()
        })
    }

    fn mermaid_contents(contents: &str) -> ParsedMarkdownMermaidDiagramContents {
        ParsedMarkdownMermaidDiagramContents {
            contents: contents.to_string().into(),
            scale: 100,
        }
    }

    fn mermaid_sequence(diagrams: &[&str]) -> Vec<ParsedMarkdownMermaidDiagramContents> {
        diagrams
            .iter()
            .map(|diagram| mermaid_contents(diagram))
            .collect()
    }

    fn mermaid_fallback(
        new_diagram: &str,
        new_full_order: &[ParsedMarkdownMermaidDiagramContents],
        old_full_order: &[ParsedMarkdownMermaidDiagramContents],
        cache: &MermaidDiagramCache,
    ) -> Option<Arc<RenderImage>> {
        let new_content = mermaid_contents(new_diagram);
        let idx = new_full_order
            .iter()
            .position(|diagram| diagram == &new_content)?;
        MermaidState::get_fallback_image(idx, old_full_order, new_full_order.len(), cache)
            .map(|(image, _)| image)
    }

    #[test]
    fn test_mermaid_font_family_resolves_zed_virtual_fonts() {
        assert_eq!(
            super::mermaid_font_family(".ZedSans"),
            "IBM Plex Sans, sans-serif"
        );
        assert_eq!(
            super::mermaid_font_family("Zed Plex Sans"),
            "IBM Plex Sans, sans-serif"
        );
        assert_eq!(super::mermaid_font_family(".ZedMono"), "Lilex, sans-serif");
        assert_eq!(
            super::mermaid_font_family(".SystemUIFont"),
            "system-ui, sans-serif"
        );
        assert_eq!(
            super::mermaid_font_family("Custom Font"),
            "Custom Font, sans-serif"
        );
        assert_eq!(
            super::mermaid_font_family("Custom Font, sans-serif"),
            "Custom Font, sans-serif"
        );
    }

    #[test]
    fn test_parse_mermaid_info() {
        assert_eq!(parse_mermaid_info("mermaid"), Some(100));
        assert_eq!(parse_mermaid_info("mermaid 150"), Some(150));
        assert_eq!(parse_mermaid_info("mermaid 5"), Some(10));
        assert_eq!(parse_mermaid_info("mermaid 999"), Some(500));
        assert_eq!(parse_mermaid_info("rust"), None);
    }

    #[test]
    fn test_extract_mermaid_diagrams_parses_scale() {
        let markdown = "```mermaid 150\ngraph TD;\n```\n\n```rust\nfn main() {}\n```";
        let events =
            crate::parser::parse_markdown_with_options(markdown, false, false, false).events;
        let diagrams = extract_mermaid_diagrams(markdown, &events);

        assert_eq!(diagrams.len(), 1);
        let diagram = diagrams.values().next().unwrap();
        assert_eq!(diagram.contents.contents, "graph TD;");
        assert_eq!(diagram.contents.scale, 150);
    }

    #[test]
    fn test_unsupported_diagram_types_are_skipped() {
        let markdown = concat!(
            "```mermaid\nsankey-beta\n```\n\n",
            "```mermaid\nblock-beta\n```\n\n",
            "```mermaid\nflowchart TD\n    A --> B\n```",
        );
        let events =
            crate::parser::parse_markdown_with_options(markdown, false, false, false).events;
        let diagrams = extract_mermaid_diagrams(markdown, &events);
        assert_eq!(
            diagrams.len(),
            1,
            "Only the flowchart should be extracted; sankey and block should be skipped"
        );
        let diagram = diagrams.values().next().unwrap();
        assert!(
            diagram.contents.contents.contains("flowchart"),
            "The extracted diagram should be the flowchart"
        );
    }

    #[gpui::test]
    fn test_mermaid_fallback_on_edit(cx: &mut TestAppContext) {
        let old_full_order = mermaid_sequence(&["graph A", "graph B", "graph C"]);
        let new_full_order = mermaid_sequence(&["graph A", "graph B modified", "graph C"]);

        let svg_b = mock_render_image(cx);

        let mut cache: MermaidDiagramCache = HashMap::default();
        cache.insert(
            mermaid_contents("graph A"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph B"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(svg_b.clone()),
                None,
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph C"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );

        let fallback =
            mermaid_fallback("graph B modified", &new_full_order, &old_full_order, &cache);

        assert_eq!(fallback.as_ref().map(|image| image.id), Some(svg_b.id));
    }

    #[gpui::test]
    fn test_mermaid_no_fallback_on_add_in_middle(cx: &mut TestAppContext) {
        let old_full_order = mermaid_sequence(&["graph A", "graph C"]);
        let new_full_order = mermaid_sequence(&["graph A", "graph NEW", "graph C"]);

        let mut cache: MermaidDiagramCache = HashMap::default();
        cache.insert(
            mermaid_contents("graph A"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph C"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );

        let fallback = mermaid_fallback("graph NEW", &new_full_order, &old_full_order, &cache);

        assert!(fallback.is_none());
    }

    #[gpui::test]
    fn test_mermaid_fallback_chains_on_rapid_edits(cx: &mut TestAppContext) {
        let old_full_order = mermaid_sequence(&["graph A", "graph B modified", "graph C"]);
        let new_full_order = mermaid_sequence(&["graph A", "graph B modified again", "graph C"]);

        let original_svg = mock_render_image(cx);

        let mut cache: MermaidDiagramCache = HashMap::default();
        cache.insert(
            mermaid_contents("graph A"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph B modified"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                None,
                Some(original_svg.clone()),
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph C"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );

        let fallback = mermaid_fallback(
            "graph B modified again",
            &new_full_order,
            &old_full_order,
            &cache,
        );

        assert_eq!(
            fallback.as_ref().map(|image| image.id),
            Some(original_svg.id)
        );
    }

    #[gpui::test]
    fn test_mermaid_fallback_with_duplicate_blocks_edit_second(cx: &mut TestAppContext) {
        let old_full_order = mermaid_sequence(&["graph A", "graph A", "graph B"]);
        let new_full_order = mermaid_sequence(&["graph A", "graph A edited", "graph B"]);

        let svg_a = mock_render_image(cx);

        let mut cache: MermaidDiagramCache = HashMap::default();
        cache.insert(
            mermaid_contents("graph A"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(svg_a.clone()),
                None,
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph B"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
                None,
            )),
        );

        let fallback = mermaid_fallback("graph A edited", &new_full_order, &old_full_order, &cache);

        assert_eq!(fallback.as_ref().map(|image| image.id), Some(svg_a.id));
    }

    #[gpui::test]
    fn test_mermaid_rerasterize_reuses_parsed_svg(cx: &mut TestAppContext) {
        ensure_theme_initialized(cx);

        let markdown = cx.new(|cx| Markdown::new("".into(), None, None, cx));

        let (parsed_svg, original_image) = markdown.update(cx, |_, cx| {
            let svg_renderer = cx.svg_renderer();
            let parsed_svg = Arc::new(
                svg_renderer
                    .parse_svg(
                        br#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"></svg>"#,
                    )
                    .unwrap(),
            );
            let original_image = svg_renderer.render_parsed(&parsed_svg, 1.0).unwrap();
            (parsed_svg, original_image)
        });

        let contents = mermaid_contents("graph A");
        let mut state = MermaidState::default();
        state.cache.insert(
            contents.clone(),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(original_image.clone()),
                None,
                Some(parsed_svg),
            )),
        );

        markdown.update(cx, |_, cx| state.rerasterize_diagram(&contents, 2.0, cx));

        let cached = state.cache.get(&contents).unwrap();
        assert!(
            cached.render_image.get().is_none(),
            "the new raster should still be pending"
        );
        assert_eq!(cached.rasterized_scale, 2.0);
        assert_eq!(
            cached
                .fallback_image
                .as_ref()
                .map(|(image, zoom)| (image.id, *zoom)),
            Some((original_image.id, 1.0)),
            "the old raster should keep being displayed while the new one is pending"
        );

        cx.run_until_parked();

        let cached = state.cache.get(&contents).unwrap();
        let new_image = cached
            .render_image
            .get()
            .expect("render should have completed")
            .as_ref()
            .expect("render should have succeeded");
        let original_size = original_image.size(0);
        let new_size = new_image.size(0);
        assert_eq!(new_size.width.0, original_size.width.0 * 2);
        assert_eq!(new_size.height.0, original_size.height.0 * 2);

        // Re-rasterizing at the zoom the entry is already rasterized at is a
        // no-op.
        let entry_before = Arc::as_ptr(state.cache.get(&contents).unwrap());
        markdown.update(cx, |_, cx| state.rerasterize_diagram(&contents, 2.0, cx));
        assert_eq!(
            Arc::as_ptr(state.cache.get(&contents).unwrap()),
            entry_before
        );
    }

    #[gpui::test]
    fn test_mermaid_zoom_rerasterize_is_debounced(cx: &mut TestAppContext) {
        ensure_theme_initialized(cx);

        let markdown = cx.new(|cx| Markdown::new("".into(), None, None, cx));

        let (parsed_svg, original_image) = markdown.update(cx, |_, cx| {
            let svg_renderer = cx.svg_renderer();
            let parsed_svg = Arc::new(
                svg_renderer
                    .parse_svg(
                        br#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"></svg>"#,
                    )
                    .unwrap(),
            );
            let original_image = svg_renderer.render_parsed(&parsed_svg, 1.0).unwrap();
            (parsed_svg, original_image)
        });

        let contents = mermaid_contents("graph A");
        let source_offset = 0;
        markdown.update(cx, |markdown, _| {
            markdown.parsed_markdown.mermaid_diagrams.insert(
                source_offset,
                ParsedMarkdownMermaidDiagram {
                    content_range: 0..contents.contents.len(),
                    contents: contents.clone(),
                },
            );
            markdown.mermaid_state.cache.insert(
                contents.clone(),
                Arc::new(CachedMermaidDiagram::new_for_test(
                    Some(original_image.clone()),
                    None,
                    Some(parsed_svg),
                )),
            );
        });

        markdown.update(cx, |markdown, cx| {
            markdown.set_mermaid_zoom_level(source_offset, 1.5, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(500));
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            let cached = markdown.mermaid_state.cache.get(&contents).unwrap();
            assert_eq!(
                cached.rasterized_scale, 1.0,
                "no re-raster before the debounce elapses"
            );
        });

        // A second zoom change within the debounce window restarts the timer.
        markdown.update(cx, |markdown, cx| {
            markdown.set_mermaid_zoom_level(source_offset, 2.0, cx)
        });
        cx.executor().advance_clock(Duration::from_millis(700));
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            let cached = markdown.mermaid_state.cache.get(&contents).unwrap();
            assert_eq!(
                cached.rasterized_scale, 1.0,
                "the second zoom change should restart the debounce timer"
            );
        });

        cx.executor().advance_clock(Duration::from_millis(300));
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            let cached = markdown.mermaid_state.cache.get(&contents).unwrap();
            assert_eq!(cached.rasterized_scale, 2.0);
            let new_image = cached
                .render_image
                .get()
                .expect("render should have completed")
                .as_ref()
                .expect("render should have succeeded");
            let original_size = original_image.size(0);
            let new_size = new_image.size(0);
            assert_eq!(new_size.width.0, original_size.width.0 * 2);
            assert_eq!(new_size.height.0, original_size.height.0 * 2);
        });
    }

    #[test]
    fn test_mermaid_zoom_ticks_is_one_step_per_notch() {
        use gpui::{ScrollDelta, point, px};

        use super::mermaid_zoom_ticks;

        // Discrete wheel notches report varying line counts per platform and
        // device; all must count as exactly one tick.
        for lines in [1.0, 3.0, 5.0] {
            assert_eq!(
                mermaid_zoom_ticks(ScrollDelta::Lines(point(0.0, lines))),
                1.0
            );
            assert_eq!(
                mermaid_zoom_ticks(ScrollDelta::Lines(point(0.0, -lines))),
                -1.0
            );
        }

        // Precise deltas contribute fractional ticks, capped at one tick.
        let half_tick = mermaid_zoom_ticks(ScrollDelta::Pixels(point(px(0.0), px(10.0))));
        assert!(half_tick > 0.0 && half_tick < 1.0);
        assert_eq!(
            mermaid_zoom_ticks(ScrollDelta::Pixels(point(px(0.0), px(500.0)))),
            1.0
        );

        // Horizontal-only scrolling must not zoom.
        assert_eq!(mermaid_zoom_ticks(ScrollDelta::Lines(point(2.0, 0.0))), 0.0);
    }

    #[gpui::test]
    fn test_mermaid_zoom_snap_and_clamp(cx: &mut TestAppContext) {
        let markdown = cx.new(|cx| Markdown::new("".into(), None, None, cx));

        markdown.update(cx, |markdown, cx| {
            // With no raster or layout to compute a fit-to-width scale from,
            // the minimum zoom is 1.0.
            markdown.set_mermaid_zoom_level(0, 0.05, cx);
            assert_eq!(markdown.mermaid_zoom_level(0), 1.0);

            markdown.set_mermaid_zoom_level(0, 10.0, cx);
            assert_eq!(markdown.mermaid_zoom_level(0), 2.0);

            // Values close to 1.0 snap back to the default.
            markdown.set_mermaid_zoom_level(0, 1.04, cx);
            assert_eq!(markdown.mermaid_zoom_level(0), 1.0);

            markdown.set_mermaid_zoom_level(0, 2.0, cx);
            markdown.set_mermaid_zoom_level(0, 0.96, cx);
            assert_eq!(markdown.mermaid_zoom_level(0), 1.0);

            markdown.set_mermaid_zoom_level(0, 1.06, cx);
            assert_eq!(markdown.mermaid_zoom_level(0), 1.06);
        });
    }

    #[gpui::test]
    fn test_mermaid_zoom_retained_across_reparse(cx: &mut TestAppContext) {
        ensure_theme_initialized(cx);

        let source = "```mermaid\ngraph TD;\n```";
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(
                source.into(),
                None,
                None,
                MarkdownOptions {
                    render_mermaid_diagrams: true,
                    ..Default::default()
                },
                cx,
            )
        });
        cx.run_until_parked();

        let source_offset = markdown.read_with(cx, |markdown, _| {
            *markdown
                .parsed_markdown
                .mermaid_diagrams
                .keys()
                .next()
                .expect("the mermaid diagram should have been parsed")
        });
        markdown.update(cx, |markdown, cx| {
            markdown.set_mermaid_zoom_level(source_offset, 2.0, cx)
        });

        // Appending after the diagram keeps its offset, so the zoom is
        // retained across the reparse.
        markdown.update(cx, |markdown, cx| {
            markdown.replace(format!("{source}\n\nmore text"), cx);
        });
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            assert_eq!(markdown.mermaid_zoom_level(source_offset), 2.0);
        });

        // Removing the diagram drops its zoom state.
        markdown.update(cx, |markdown, cx| {
            markdown.replace("plain text", cx);
        });
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            assert_eq!(markdown.mermaid_zoom_level(source_offset), 1.0);
            assert!(markdown.mermaid_views.is_empty());
        });
    }

    #[gpui::test]
    fn test_mermaid_scroll_handle_retained_across_reparse(cx: &mut TestAppContext) {
        ensure_theme_initialized(cx);

        let source = "```mermaid\ngraph TD;\n```";
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(
                source.into(),
                None,
                None,
                MarkdownOptions {
                    render_mermaid_diagrams: true,
                    ..Default::default()
                },
                cx,
            )
        });
        cx.run_until_parked();

        let source_offset = markdown.read_with(cx, |markdown, _| {
            *markdown
                .parsed_markdown
                .mermaid_diagrams
                .keys()
                .next()
                .expect("the mermaid diagram should have been parsed")
        });
        let scroll_handle = markdown.update(cx, |markdown, _| {
            markdown.mermaid_scroll_handle(source_offset)
        });
        scroll_handle.set_offset(point(px(-42.0), px(0.0)));

        // Appending after the diagram keeps its offset, so the same scroll
        // handle (and thus the scroll position) is retained across the
        // reparse.
        markdown.update(cx, |markdown, cx| {
            markdown.replace(format!("{source}\n\nmore text"), cx);
        });
        cx.run_until_parked();
        let retained_handle = markdown.update(cx, |markdown, _| {
            markdown.mermaid_scroll_handle(source_offset)
        });
        assert_eq!(retained_handle.offset(), point(px(-42.0), px(0.0)));

        // Removing the diagram drops its scroll state.
        markdown.update(cx, |markdown, cx| {
            markdown.replace("plain text", cx);
        });
        cx.run_until_parked();
        markdown.read_with(cx, |markdown, _| {
            assert!(markdown.mermaid_views.is_empty());
        });
    }

    #[gpui::test]
    fn test_mermaid_rendering_replaces_code_block_text(cx: &mut TestAppContext) {
        let rendered = render_markdown_with_options(
            "```mermaid\ngraph TD;\n```",
            MarkdownOptions {
                render_mermaid_diagrams: true,
                ..Default::default()
            },
            cx,
        );

        let text = rendered
            .lines
            .iter()
            .map(|line| line.layout.wrapped_text())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!text.contains("graph TD;"));
    }

    #[gpui::test]
    fn test_mermaid_source_anchor_maps_inside_block(cx: &mut TestAppContext) {
        struct TestWindow;

        impl Render for TestWindow {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }
        }

        ensure_theme_initialized(cx);

        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| {
            Markdown::new_with_options(
                "```mermaid\ngraph TD;\n```".into(),
                None,
                None,
                MarkdownOptions {
                    render_mermaid_diagrams: true,
                    ..Default::default()
                },
                cx,
            )
        });
        cx.run_until_parked();
        let render_image = mock_render_image(cx);
        markdown.update(cx, |markdown, _| {
            let contents = markdown
                .parsed_markdown
                .mermaid_diagrams
                .values()
                .next()
                .unwrap()
                .contents
                .clone();
            markdown.mermaid_state.cache.insert(
                contents.clone(),
                Arc::new(CachedMermaidDiagram::new_for_test(
                    Some(render_image),
                    None,
                    None,
                )),
            );
            markdown.mermaid_state.order = vec![contents];
        });

        let rendered_text = draw_markdown_element(markdown.clone(), cx);

        let mermaid_diagram = markdown.update(cx, |markdown, _| {
            markdown
                .parsed_markdown
                .mermaid_diagrams
                .values()
                .next()
                .unwrap()
                .clone()
        });
        assert!(
            rendered_text
                .position_for_source_index(mermaid_diagram.content_range.start)
                .is_some()
        );
        assert!(
            rendered_text
                .position_for_source_index(mermaid_diagram.content_range.end.saturating_sub(1))
                .is_some()
        );
    }
}
