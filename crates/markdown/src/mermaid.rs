use collections::HashMap;
use gpui::{
    Animation, AnimationExt, AnyElement, ClickEvent, ClipboardItem, Context, Entity, ImageSource,
    RenderImage, StyledText, Task, img, pulsating_between,
};
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use ui::CopyButton;
use ui::prelude::*;

use crate::parser::{CodeBlockKind, MarkdownEvent, MarkdownTag};
use settings::Settings as _;
use theme_settings::ThemeSettings;

use super::{CopyButtonVisibility, Markdown, MarkdownStyle, ParsedMarkdown};

type MermaidDiagramCache = HashMap<ParsedMarkdownMermaidDiagramContents, Arc<CachedMermaidDiagram>>;

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
    fallback_image: Option<Arc<RenderImage>>,
    _task: Task<()>,
}

impl MermaidState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    fn get_fallback_image(
        idx: usize,
        old_order: &[ParsedMarkdownMermaidDiagramContents],
        new_order_len: usize,
        cache: &MermaidDiagramCache,
    ) -> Option<Arc<RenderImage>> {
        if old_order.len() != new_order_len {
            return None;
        }

        old_order.get(idx).and_then(|old_content| {
            cache.get(old_content).and_then(|old_cached| {
                old_cached
                    .render_image
                    .get()
                    .and_then(|result| result.as_ref().ok().cloned())
                    .or_else(|| old_cached.fallback_image.clone())
            })
        })
    }

    pub(crate) fn update(&mut self, parsed: &ParsedMarkdown, cx: &mut Context<Markdown>) {
        let mut new_order = Vec::new();
        for mermaid_diagram in parsed.mermaid_diagrams.values() {
            new_order.push(mermaid_diagram.contents.clone());
        }

        for (idx, new_content) in new_order.iter().enumerate() {
            if !self.cache.contains_key(new_content) {
                let fallback =
                    Self::get_fallback_image(idx, &self.order, new_order.len(), &self.cache);
                self.cache.insert(
                    new_content.clone(),
                    Arc::new(CachedMermaidDiagram::new(new_content.clone(), fallback, cx)),
                );
            }
        }

        let new_order_set: std::collections::HashSet<_> = new_order.iter().cloned().collect();
        self.cache
            .retain(|content, _| new_order_set.contains(content));
        self.order = new_order;
    }
}

impl CachedMermaidDiagram {
    fn new(
        contents: ParsedMarkdownMermaidDiagramContents,
        fallback_image: Option<Arc<RenderImage>>,
        cx: &mut Context<Markdown>,
    ) -> Self {
        let render_image = Arc::new(OnceLock::<anyhow::Result<Arc<RenderImage>>>::new());
        let render_image_clone = render_image.clone();
        let svg_renderer = cx.svg_renderer();
        let mermaid_theme = build_mermaid_theme(cx);

        let task = cx.spawn(async move |this, cx| {
            let value = cx
                .background_spawn(async move {
                    let svg_string =
                        mermaid_render::render_to_svg(&contents.contents, &mermaid_theme)?;
                    let scale = contents.scale as f32 / 100.0;
                    svg_renderer
                        .render_single_frame(svg_string.as_bytes(), scale)
                        .map_err(|error| anyhow::anyhow!("{error}"))
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

pub(crate) fn render_mermaid_diagram(
    parsed: &ParsedMarkdownMermaidDiagram,
    mermaid_state: &MermaidState,
    style: &MarkdownStyle,
    markdown: Entity<Markdown>,
    source_offset: usize,
    showing_code: bool,
    copy_button_visibility: CopyButtonVisibility,
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
                div()
                    .w_full()
                    .child(
                        img(ImageSource::Render(render_image.clone()))
                            .max_w_full()
                            .with_fallback(|| {
                                div()
                                    .child(Label::new("Failed to load mermaid diagram"))
                                    .into_any_element()
                            }),
                    )
                    .into_any_element()
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
            if let Some(fallback) = cached.and_then(|cached| cached.fallback_image.as_ref()) {
                container
                    .child(
                        div()
                            .w_full()
                            .child(
                                img(ImageSource::Render(fallback.clone()))
                                    .max_w_full()
                                    .with_fallback(|| {
                                        div()
                                            .child(Label::new("Failed to load mermaid diagram"))
                                            .into_any_element()
                                    }),
                            )
                            .with_animation(
                                "mermaid-fallback-pulse",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.6, 1.0)),
                                |element, delta| element.opacity(delta),
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

fn render_mermaid_tab_header(
    source_offset: usize,
    showing_code: bool,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let preview_markdown = markdown.clone();
    let code_markdown = markdown;

    h_flex()
        .gap_0p5()
        .p_0p5()
        .mb_1()
        .child(render_mermaid_tab_button(
            "Preview",
            source_offset,
            !showing_code,
            move |_event, _window, cx| {
                preview_markdown.update(cx, |md, cx| {
                    if md.is_mermaid_showing_code(source_offset) {
                        md.toggle_mermaid_tab(source_offset);
                        cx.notify();
                    }
                });
            },
        ))
        .child(render_mermaid_tab_button(
            "Code",
            source_offset,
            showing_code,
            move |_event, _window, cx| {
                code_markdown.update(cx, |md, cx| {
                    if !md.is_mermaid_showing_code(source_offset) {
                        md.toggle_mermaid_tab(source_offset);
                        cx.notify();
                    }
                });
            },
        ))
}

fn render_mermaid_tab_button(
    label: &'static str,
    source_offset: usize,
    is_selected: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(ElementId::named_usize(
            SharedString::from(format!("mermaid-tab-{label}")),
            source_offset,
        ))
        .cursor_pointer()
        .px_1p5()
        .py_0p5()
        .rounded_md()
        .text_size(rems(0.75))
        .when(is_selected, |this| this.bg(gpui::hsla(0., 0., 0.5, 0.15)))
        .when(!is_selected, |this| {
            this.hover(|this| this.bg(gpui::hsla(0., 0., 0.5, 0.08)))
        })
        .child(label)
        .on_click(on_click)
}

fn render_mermaid_copy_button(
    source_offset: usize,
    code: String,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let id = ElementId::named_usize("copy-mermaid-code", source_offset);

    h_flex()
        .w_4()
        .absolute()
        .top_0()
        .right_0()
        .justify_end()
        .visible_on_hover("code_block")
        .child(CopyButton::new(id.clone(), code.clone()).custom_on_click({
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
        }))
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
        CachedMermaidDiagram, MermaidDiagramCache, MermaidState,
        ParsedMarkdownMermaidDiagramContents, extract_mermaid_diagrams, parse_mermaid_info,
    };
    use crate::{
        CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownOptions,
        MarkdownStyle, WrapButtonVisibility,
    };
    use collections::HashMap;
    use gpui::{Context, IntoElement, Render, RenderImage, TestAppContext, Window, size};
    use std::sync::Arc;
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
        let (rendered, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |_window, _cx| {
                MarkdownElement::new(markdown, MarkdownStyle::default()).code_block_renderer(
                    CodeBlockRenderer::Default {
                        copy_button_visibility: CopyButtonVisibility::Hidden,
                        wrap_button_visibility: WrapButtonVisibility::Hidden,
                        border: false,
                    },
                )
            },
        );
        rendered.text
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
        let events = crate::parser::parse_markdown_with_options(markdown, false, false).events;
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
        let events = crate::parser::parse_markdown_with_options(markdown, false, false).events;
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
            )),
        );
        cache.insert(
            mermaid_contents("graph B"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(svg_b.clone()),
                None,
            )),
        );
        cache.insert(
            mermaid_contents("graph C"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
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
            )),
        );
        cache.insert(
            mermaid_contents("graph C"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
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
            )),
        );
        cache.insert(
            mermaid_contents("graph B modified"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                None,
                Some(original_svg.clone()),
            )),
        );
        cache.insert(
            mermaid_contents("graph C"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
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
            )),
        );
        cache.insert(
            mermaid_contents("graph B"),
            Arc::new(CachedMermaidDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
            )),
        );

        let fallback = mermaid_fallback("graph A edited", &new_full_order, &old_full_order, &cache);

        assert_eq!(fallback.as_ref().map(|image| image.id), Some(svg_a.id));
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
                Arc::new(CachedMermaidDiagram::new_for_test(Some(render_image), None)),
            );
            markdown.mermaid_state.order = vec![contents];
        });

        let (rendered, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |_window, _cx| {
                MarkdownElement::new(markdown.clone(), MarkdownStyle::default())
                    .code_block_renderer(CodeBlockRenderer::Default {
                        copy_button_visibility: CopyButtonVisibility::Hidden,
                        wrap_button_visibility: WrapButtonVisibility::Hidden,
                        border: false,
                    })
            },
        );

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
            rendered
                .text
                .position_for_source_index(mermaid_diagram.content_range.start)
                .is_some()
        );
        assert!(
            rendered
                .text
                .position_for_source_index(mermaid_diagram.content_range.end.saturating_sub(1))
                .is_some()
        );
    }
}
