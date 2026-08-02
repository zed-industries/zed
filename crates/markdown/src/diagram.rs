use std::{hash::Hash, ops::Range, sync::Arc, time::Duration};

use collections::HashMap;
use gpui::{
    Animation, AnimationExt, AnyElement, Entity, ImageSource, RenderImage, StyledText, img,
    pulsating_between,
};
use ui::{CopyButton, TintColor, prelude::*};

use super::{CopyButtonVisibility, Markdown, MarkdownStyle};

#[derive(Clone, Copy)]
pub(super) enum DiagramKind {
    Mermaid,
    PlantUml,
}

impl DiagramKind {
    fn configuration(self) -> DiagramConfiguration {
        match self {
            Self::Mermaid => DiagramConfiguration {
                preview_tab_id: "mermaid-tab-preview",
                code_tab_id: "mermaid-tab-code",
                copy_button_id: "copy-mermaid-code",
                scroll_id: "mermaid-scroll",
                image_error_message: "Failed to Load Mermaid Diagram",
                fallback_animation_id: "mermaid-fallback-pulse",
                loading_animation_id: "mermaid-loading-pulse",
                code_debug_selector: "mermaid-code",
                render_error_debug_selector: None,
            },
            Self::PlantUml => DiagramConfiguration {
                preview_tab_id: "plantuml-tab-preview",
                code_tab_id: "plantuml-tab-code",
                copy_button_id: "copy-plantuml-code",
                scroll_id: "plantuml-scroll",
                image_error_message: "Failed to Load PlantUML Diagram",
                fallback_animation_id: "plantuml-fallback-pulse",
                loading_animation_id: "plantuml-loading-pulse",
                code_debug_selector: "plantuml-code",
                render_error_debug_selector: Some("plantuml-error"),
            },
        }
    }

    fn is_showing_code(self, markdown: &Markdown, source_offset: usize) -> bool {
        match self {
            Self::Mermaid => markdown.is_mermaid_showing_code(source_offset),
            Self::PlantUml => markdown.is_plantuml_showing_code(source_offset),
        }
    }

    fn toggle_code(self, markdown: &mut Markdown, source_offset: usize) {
        match self {
            Self::Mermaid => markdown.toggle_mermaid_tab(source_offset),
            Self::PlantUml => markdown.toggle_plantuml_tab(source_offset),
        }
    }
}

#[derive(Clone, Copy)]
struct DiagramConfiguration {
    preview_tab_id: &'static str,
    code_tab_id: &'static str,
    copy_button_id: &'static str,
    scroll_id: &'static str,
    image_error_message: &'static str,
    fallback_animation_id: &'static str,
    loading_animation_id: &'static str,
    code_debug_selector: &'static str,
    render_error_debug_selector: Option<&'static str>,
}

pub(super) enum DiagramRenderState<'a> {
    Ready(&'a Arc<RenderImage>),
    Failed(&'a anyhow::Error),
    Pending(Option<Arc<RenderImage>>),
}

impl<'a> DiagramRenderState<'a> {
    pub(super) fn from_result(
        render_result: Option<&'a anyhow::Result<Arc<RenderImage>>>,
        fallback_image: impl FnOnce() -> Option<Arc<RenderImage>>,
    ) -> Self {
        match render_result {
            Some(Ok(rendered_image)) => Self::Ready(rendered_image),
            Some(Err(error)) => Self::Failed(error),
            None => Self::Pending(fallback_image()),
        }
    }
}

pub(super) struct DiagramView<'a> {
    pub(super) kind: DiagramKind,
    pub(super) render_state: DiagramRenderState<'a>,
    pub(super) contents: &'a SharedString,
    pub(super) style: &'a MarkdownStyle,
    pub(super) markdown: Entity<Markdown>,
    pub(super) source_offset: usize,
    pub(super) showing_code: bool,
    pub(super) copy_button_visibility: CopyButtonVisibility,
}

impl DiagramView<'_> {
    pub(super) fn render(self) -> AnyElement {
        let show_interactive = self.copy_button_visibility != CopyButtonVisibility::Hidden;
        let allow_overflow_x = self.style.code_block_overflow_x_scroll;
        let configuration = self.kind.configuration();

        let (show_tabs, body) = match self.render_state {
            DiagramRenderState::Ready(rendered_image) => {
                let body = if self.showing_code {
                    render_code(configuration, self.contents)
                } else {
                    render_image(
                        configuration,
                        rendered_image.clone(),
                        allow_overflow_x,
                        self.source_offset,
                    )
                };
                (true, body)
            }
            DiagramRenderState::Failed(error) => {
                (false, render_error(configuration, self.contents, error))
            }
            DiagramRenderState::Pending(fallback_image) => {
                let body = match fallback_image {
                    Some(fallback_image) => div()
                        .child(render_image(
                            configuration,
                            fallback_image,
                            allow_overflow_x,
                            self.source_offset,
                        ))
                        .with_animation(
                            configuration.fallback_animation_id,
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.6, 1.0)),
                            |element, delta| element.opacity(delta),
                        )
                        .into_any_element(),
                    None => render_loading(configuration, self.contents),
                };
                (false, body)
            }
        };

        let mut container = div().group("code_block").relative().w_full().rounded_lg();
        container.style().refine(&self.style.code_block);

        container
            .when(show_interactive && show_tabs, |container| {
                container.child(render_tab_header(
                    self.kind,
                    configuration,
                    self.source_offset,
                    self.showing_code,
                    self.markdown.clone(),
                ))
            })
            .child(body)
            .when(show_interactive, |container| {
                container.child(render_copy_button(
                    configuration,
                    self.source_offset,
                    self.contents.clone(),
                    self.markdown,
                ))
            })
            .into_any_element()
    }
}

pub(super) fn update_diagram_cache<Key, Cached>(
    cache: &mut HashMap<Key, Arc<Cached>>,
    order: &mut Vec<Key>,
    new_order: Vec<Key>,
    fallback_for_cached: impl Fn(&Cached) -> Option<Arc<RenderImage>>,
    mut create_cached: impl FnMut(Key, Option<Arc<RenderImage>>) -> Cached,
) where
    Key: Clone + Eq + Hash,
{
    for (index, contents) in new_order.iter().enumerate() {
        if cache.contains_key(contents) {
            continue;
        }

        let fallback_image =
            fallback_image_for_edit(index, order, new_order.len(), cache, &fallback_for_cached);
        cache.insert(
            contents.clone(),
            Arc::new(create_cached(contents.clone(), fallback_image)),
        );
    }

    let new_order_set = new_order
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    cache.retain(|contents, _| new_order_set.contains(contents));
    *order = new_order;
}

pub(super) fn fallback_image_for_edit<Key, Cached>(
    index: usize,
    old_order: &[Key],
    new_order_length: usize,
    cache: &HashMap<Key, Arc<Cached>>,
    fallback_image: impl Fn(&Cached) -> Option<Arc<RenderImage>>,
) -> Option<Arc<RenderImage>>
where
    Key: Eq + Hash,
{
    if old_order.len() != new_order_length {
        return None;
    }

    old_order
        .get(index)
        .and_then(|old_contents| cache.get(old_contents))
        .and_then(|cached| fallback_image(cached))
}

pub(super) fn truncate_message(message: &str, max_characters: usize) -> String {
    let mut characters = message.chars();
    let truncated = characters.by_ref().take(max_characters).collect::<String>();
    if characters.next().is_some() {
        format!("{truncated}…")
    } else {
        truncated
    }
}

pub(super) fn fenced_code_block_contents(
    source: &str,
    content_range: Range<usize>,
) -> Option<SharedString> {
    let contents = source.get(content_range)?;
    let contents = contents.strip_suffix('\n').unwrap_or(contents);
    let contents = contents.strip_suffix('\r').unwrap_or(contents);
    (!contents.trim().is_empty()).then(|| contents.to_string().into())
}

fn render_image(
    configuration: DiagramConfiguration,
    render_image: Arc<RenderImage>,
    allow_overflow_x: bool,
    source_offset: usize,
) -> AnyElement {
    let image = img(ImageSource::Render(render_image))
        .with_fallback(move || Label::new(configuration.image_error_message).into_any_element());

    if allow_overflow_x {
        div()
            .id((configuration.scroll_id, source_offset))
            .w_full()
            .map(|mut container| {
                container.style().restrict_scroll_to_axis = Some(true);
                container.overflow_x_scroll()
            })
            .child(image)
            .into_any_element()
    } else {
        div().w_full().child(image.max_w_full()).into_any_element()
    }
}

fn render_tab_header(
    kind: DiagramKind,
    configuration: DiagramConfiguration,
    source_offset: usize,
    showing_code: bool,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let preview_id = ElementId::NamedChild(
        Arc::new(ElementId::from((
            configuration.preview_tab_id,
            markdown.entity_id(),
        ))),
        source_offset.to_string().into(),
    );
    let code_id = ElementId::NamedChild(
        Arc::new(ElementId::from((
            configuration.code_tab_id,
            markdown.entity_id(),
        ))),
        source_offset.to_string().into(),
    );

    h_flex()
        .gap_0p5()
        .mb_2p5()
        .child(
            Button::new(preview_id, "Preview")
                .label_size(LabelSize::Small)
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .toggle_state(!showing_code)
                .on_click({
                    let markdown = markdown.clone();
                    move |_event, _window, cx| {
                        markdown.update(cx, |markdown, cx| {
                            if kind.is_showing_code(markdown, source_offset) {
                                kind.toggle_code(markdown, source_offset);
                                cx.notify();
                            }
                        });
                    }
                }),
        )
        .child(
            Button::new(code_id, "Code")
                .label_size(LabelSize::Small)
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .toggle_state(showing_code)
                .on_click(move |_event, _window, cx| {
                    markdown.update(cx, |markdown, cx| {
                        if !kind.is_showing_code(markdown, source_offset) {
                            kind.toggle_code(markdown, source_offset);
                            cx.notify();
                        }
                    });
                }),
        )
}

fn render_copy_button(
    configuration: DiagramConfiguration,
    source_offset: usize,
    contents: SharedString,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let id = ElementId::NamedChild(
        Arc::new(ElementId::from((
            configuration.copy_button_id,
            markdown.entity_id(),
        ))),
        source_offset.to_string().into(),
    );

    div()
        .absolute()
        .top_1()
        .right_1()
        .justify_end()
        .child(CopyButton::new(id, contents).visible_on_hover("code_block"))
}

fn render_code(configuration: DiagramConfiguration, contents: &SharedString) -> AnyElement {
    div()
        .w_full()
        .debug_selector(move || configuration.code_debug_selector.into())
        .child(StyledText::new(contents.clone()))
        .into_any_element()
}

fn render_error(
    configuration: DiagramConfiguration,
    contents: &SharedString,
    error: &anyhow::Error,
) -> AnyElement {
    div()
        .child(render_code(configuration, contents))
        .when_some(
            configuration.render_error_debug_selector,
            |container, debug_selector| {
                container.child(
                    div()
                        .mt_2()
                        .debug_selector(move || debug_selector.into())
                        .child(
                            Label::new(truncate_message(&error.to_string(), 200))
                                .size(LabelSize::XSmall)
                                .color(Color::Error),
                        ),
                )
            },
        )
        .into_any_element()
}

fn render_loading(configuration: DiagramConfiguration, contents: &SharedString) -> AnyElement {
    div()
        .child(
            div().mb_2().child(
                Label::new("Rendering...")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .with_animation(
                        configuration.loading_animation_id,
                        Animation::new(Duration::from_secs(2))
                            .repeat()
                            .with_easing(pulsating_between(0.4, 0.8)),
                        |label, delta| label.alpha(delta),
                    ),
            ),
        )
        .child(render_code(configuration, contents))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::{DiagramRenderState, fenced_code_block_contents};

    #[test]
    fn test_fenced_code_block_contents_is_safe_and_normalizes_line_endings() {
        assert_eq!(
            fenced_code_block_contents("diagram\r\n", 0..9).as_deref(),
            Some("diagram")
        );
        assert!(fenced_code_block_contents("diagram", 0..100).is_none());
        assert!(fenced_code_block_contents("é", 0..1).is_none());
        assert!(fenced_code_block_contents("  \r\n", 0..4).is_none());
    }

    #[test]
    fn test_render_state_reads_fallback_only_while_pending() {
        let fallback_was_read = Cell::new(false);
        let render_result = Err(anyhow::anyhow!("render failed"));
        let state = DiagramRenderState::from_result(Some(&render_result), || {
            fallback_was_read.set(true);
            None
        });
        assert!(matches!(state, DiagramRenderState::Failed(_)));
        assert!(!fallback_was_read.get());

        let state = DiagramRenderState::from_result(None, || {
            fallback_was_read.set(true);
            None
        });
        assert!(matches!(state, DiagramRenderState::Pending(None)));
        assert!(fallback_was_read.get());
    }
}
