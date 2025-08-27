use crate::markdown_elements::{
    HeadingLevel, Link, MarkdownParagraph, MarkdownParagraphChunk, ParsedMarkdown,
    ParsedMarkdownBlockQuote, ParsedMarkdownCodeBlock, ParsedMarkdownElement,
    ParsedMarkdownHeading, ParsedMarkdownListItem, ParsedMarkdownListItemType, ParsedMarkdownTable,
    ParsedMarkdownTableAlignment, ParsedMarkdownTableRow,
};
use fs::normalize_path;
use gpui::{
    AbsoluteLength, AnyElement, App, AppContext as _, ClipboardItem, Context, DefiniteLength, Div,
    Element, ElementId, Entity, HighlightStyle, Hsla, ImageSource, InteractiveText, IntoElement,
    Keystroke, Length, Modifiers, ParentElement, Render, Resource, SharedString, Styled,
    StyledText, TextStyle, WeakEntity, Window, div, img, rems,
};
use settings::Settings;
use std::{
    ops::{Mul, Range},
    sync::Arc,
    vec,
};
use theme::{ActiveTheme, SyntaxTheme, ThemeSettings};
use ui::{
    ButtonCommon, Clickable, Color, FluentBuilder, IconButton, IconName, IconSize,
    InteractiveElement, Label, LabelCommon, LabelSize, LinkPreview, Pixels, Rems,
    StatefulInteractiveElement, StyledExt, StyledImage, ToggleState, Tooltip, VisibleOnHover,
    h_flex, relative, tooltip_container, v_flex,
};
use workspace::{OpenOptions, OpenVisible, Workspace};

pub struct CheckboxClickedEvent {
    pub checked: bool,
    pub source_range: Range<usize>,
}

impl CheckboxClickedEvent {
    pub fn source_range(&self) -> Range<usize> {
        self.source_range.clone()
    }

    pub fn checked(&self) -> bool {
        self.checked
    }
}

type CheckboxClickedCallback = Arc<Box<dyn Fn(&CheckboxClickedEvent, &mut Window, &mut App)>>;

#[derive(Clone)]
pub struct RenderContext {
    workspace: Option<WeakEntity<Workspace>>,
    next_id: usize,
    buffer_font_family: SharedString,
    buffer_text_style: TextStyle,
    text_style: TextStyle,
    border_color: Hsla,
    text_color: Hsla,
    window_rem_size: Pixels,
    text_muted_color: Hsla,
    code_block_background_color: Hsla,
    code_span_background_color: Hsla,
    syntax_theme: Arc<SyntaxTheme>,
    indent: usize,
    checkbox_clicked_callback: Option<CheckboxClickedCallback>,
}

impl RenderContext {
    pub fn new(
        workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut App,
    ) -> RenderContext {
        let theme = cx.theme().clone();

        let settings = ThemeSettings::get_global(cx);
        let buffer_font_family = settings.buffer_font.family.clone();
        let mut buffer_text_style = window.text_style();
        buffer_text_style.font_family = buffer_font_family.clone();
        buffer_text_style.font_size = AbsoluteLength::from(settings.buffer_font_size(cx));

        RenderContext {
            workspace,
            next_id: 0,
            indent: 0,
            buffer_font_family,
            buffer_text_style,
            text_style: window.text_style(),
            syntax_theme: theme.syntax().clone(),
            border_color: theme.colors().border,
            text_color: theme.colors().text,
            window_rem_size: window.rem_size(),
            text_muted_color: theme.colors().text_muted,
            code_block_background_color: theme.colors().surface_background,
            code_span_background_color: theme.colors().editor_document_highlight_read_background,
            checkbox_clicked_callback: None,
        }
    }

    pub fn with_checkbox_clicked_callback(
        mut self,
        callback: impl Fn(&CheckboxClickedEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.checkbox_clicked_callback = Some(Arc::new(Box::new(callback)));
        self
    }

    fn next_id(&mut self, span: &Range<usize>) -> ElementId {
        let id = format!("markdown-{}-{}-{}", self.next_id, span.start, span.end);
        self.next_id += 1;
        ElementId::from(SharedString::from(id))
    }

    /// HACK: used to have rems relative to buffer font size, so that things scale appropriately as
    /// buffer font size changes. The callees of this function should be reimplemented to use real
    /// relative sizing once that is implemented in GPUI
    pub fn scaled_rems(&self, rems: f32) -> Rems {
        self.buffer_text_style
            .font_size
            .to_rems(self.window_rem_size)
            .mul(rems)
    }

    /// This ensures that children inside of block quotes
    /// have padding between them.
    ///
    /// For example, for this markdown:
    ///
    /// ```markdown
    /// > This is a block quote.
    /// >
    /// > And this is the next paragraph.
    /// ```
    ///
    /// We give padding between "This is a block quote."
    /// and "And this is the next paragraph."
    fn with_common_p(&self, element: Div) -> Div {
        if self.indent > 0 {
            element.pb(self.scaled_rems(0.75))
        } else {
            element
        }
    }
}

pub fn render_parsed_markdown(
    parsed: &ParsedMarkdown,
    workspace: Option<WeakEntity<Workspace>>,
    window: &mut Window,
    cx: &mut App,
) -> Div {
    let mut cx = RenderContext::new(workspace, window, cx);

    v_flex().gap_3().children(
        parsed
            .children
            .iter()
            .map(|block| render_markdown_block(block, &mut cx)),
    )
}
pub fn render_markdown_block(block: &ParsedMarkdownElement, cx: &mut RenderContext) -> AnyElement {
    use ParsedMarkdownElement::*;
    match block {
        Paragraph(text) => render_markdown_paragraph(text, cx),
        Heading(heading) => render_markdown_heading(heading, cx),
        ListItem(list_item) => render_markdown_list_item(list_item, cx),
        Table(table) => render_markdown_table(table, cx),
        BlockQuote(block_quote) => render_markdown_block_quote(block_quote, cx),
        CodeBlock(code_block) => render_markdown_code_block(code_block, cx),
        HorizontalRule(_) => render_markdown_rule(cx),
    }
}

fn render_markdown_heading(parsed: &ParsedMarkdownHeading, cx: &mut RenderContext) -> AnyElement {
    let size = match parsed.level {
        HeadingLevel::H1 => 2.,
        HeadingLevel::H2 => 1.5,
        HeadingLevel::H3 => 1.25,
        HeadingLevel::H4 => 1.,
        HeadingLevel::H5 => 0.875,
        HeadingLevel::H6 => 0.85,
    };

    let text_size = cx.scaled_rems(size);

    // was `DefiniteLength::from(text_size.mul(1.25))`
    // let line_height = DefiniteLength::from(text_size.mul(1.25));
    let line_height = text_size * 1.25;

    // was `rems(0.15)`
    // let padding_top = cx.scaled_rems(0.15);
    let padding_top = rems(0.15);

    // was `.pb_1()` = `rems(0.25)`
    // let padding_bottom = cx.scaled_rems(0.25);
    let padding_bottom = rems(0.25);

    let color = match parsed.level {
        HeadingLevel::H6 => cx.text_muted_color,
        _ => cx.text_color,
    };
    div()
        .line_height(line_height)
        .text_size(text_size)
        .text_color(color)
        .pt(padding_top)
        .pb(padding_bottom)
        .children(render_markdown_text(&parsed.contents, cx))
        .whitespace_normal()
        .into_any()
}

fn render_markdown_list_item(
    parsed: &ParsedMarkdownListItem,
    cx: &mut RenderContext,
) -> AnyElement {
    use ParsedMarkdownListItemType::*;

    let padding = cx.scaled_rems((parsed.depth - 1) as f32);

    let bullet = match &parsed.item_type {
        Ordered(order) => format!("{}.", order).into_any_element(),
        Unordered => "•".into_any_element(),
        Task(checked, range) => div()
            .id(cx.next_id(range))
            .mt(cx.scaled_rems(3.0 / 16.0))
            .child(
                MarkdownCheckbox::new(
                    "checkbox",
                    if *checked {
                        ToggleState::Selected
                    } else {
                        ToggleState::Unselected
                    },
                    cx.clone(),
                )
                .when_some(
                    cx.checkbox_clicked_callback.clone(),
                    |this, callback| {
                        this.on_click({
                            let range = range.clone();
                            move |selection, window, cx| {
                                let checked = match selection {
                                    ToggleState::Selected => true,
                                    ToggleState::Unselected => false,
                                    _ => return,
                                };

                                if window.modifiers().secondary() {
                                    callback(
                                        &CheckboxClickedEvent {
                                            checked,
                                            source_range: range.clone(),
                                        },
                                        window,
                                        cx,
                                    );
                                }
                            }
                        })
                    },
                ),
            )
            .hover(|s| s.cursor_pointer())
            .tooltip(|_, cx| {
                InteractiveMarkdownElementTooltip::new(None, "toggle checkbox", cx).into()
            })
            .into_any_element(),
    };
    let bullet = div().mr(cx.scaled_rems(0.5)).child(bullet);

    let contents: Vec<AnyElement> = parsed
        .content
        .iter()
        .map(|c| render_markdown_block(c, cx))
        .collect();

    let item = h_flex()
        .pl(DefiniteLength::Absolute(AbsoluteLength::Rems(padding)))
        .items_start()
        .children(vec![
            bullet,
            div().children(contents).pr(cx.scaled_rems(1.0)).w_full(),
        ]);

    cx.with_common_p(item).into_any()
}

/// # MarkdownCheckbox ///
/// HACK: Copied from `ui/src/components/toggle.rs` to deal with scaling issues in markdown preview
/// changes should be integrated into `Checkbox` in `toggle.rs` while making sure checkboxes elsewhere in the
/// app are not visually affected
#[derive(gpui::IntoElement)]
struct MarkdownCheckbox {
    id: ElementId,
    toggle_state: ToggleState,
    disabled: bool,
    placeholder: bool,
    on_click: Option<Box<dyn Fn(&ToggleState, &mut Window, &mut App) + 'static>>,
    filled: bool,
    style: ui::ToggleStyle,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> gpui::AnyView>>,
    label: Option<SharedString>,
    render_cx: RenderContext,
}

impl MarkdownCheckbox {
    /// Creates a new [`Checkbox`].
    fn new(id: impl Into<ElementId>, checked: ToggleState, render_cx: RenderContext) -> Self {
        Self {
            id: id.into(),
            toggle_state: checked,
            disabled: false,
            on_click: None,
            filled: false,
            style: ui::ToggleStyle::default(),
            tooltip: None,
            label: None,
            placeholder: false,
            render_cx,
        }
    }

    /// Binds a handler to the [`Checkbox`] that will be called when clicked.
    fn on_click(mut self, handler: impl Fn(&ToggleState, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn bg_color(&self, cx: &App) -> Hsla {
        let style = self.style.clone();
        match (style, self.filled) {
            (ui::ToggleStyle::Ghost, false) => cx.theme().colors().ghost_element_background,
            (ui::ToggleStyle::Ghost, true) => cx.theme().colors().element_background,
            (ui::ToggleStyle::ElevationBased(_), false) => gpui::transparent_black(),
            (ui::ToggleStyle::ElevationBased(elevation), true) => elevation.darker_bg(cx),
            (ui::ToggleStyle::Custom(_), false) => gpui::transparent_black(),
            (ui::ToggleStyle::Custom(color), true) => color.opacity(0.2),
        }
    }

    fn border_color(&self, cx: &App) -> Hsla {
        if self.disabled {
            return cx.theme().colors().border_variant;
        }

        match self.style.clone() {
            ui::ToggleStyle::Ghost => cx.theme().colors().border,
            ui::ToggleStyle::ElevationBased(_) => cx.theme().colors().border,
            ui::ToggleStyle::Custom(color) => color.opacity(0.3),
        }
    }
}

impl gpui::RenderOnce for MarkdownCheckbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let group_id = format!("checkbox_group_{:?}", self.id);
        let color = if self.disabled {
            Color::Disabled
        } else {
            Color::Selected
        };
        let icon_size_small = IconSize::Custom(self.render_cx.scaled_rems(14. / 16.)); // was IconSize::Small
        let icon = match self.toggle_state {
            ToggleState::Selected => {
                if self.placeholder {
                    None
                } else {
                    Some(
                        ui::Icon::new(IconName::Check)
                            .size(icon_size_small)
                            .color(color),
                    )
                }
            }
            ToggleState::Indeterminate => Some(
                ui::Icon::new(IconName::Dash)
                    .size(icon_size_small)
                    .color(color),
            ),
            ToggleState::Unselected => None,
        };

        let bg_color = self.bg_color(cx);
        let border_color = self.border_color(cx);
        let hover_border_color = border_color.alpha(0.7);

        let size = self.render_cx.scaled_rems(1.25); // was Self::container_size(); (20px)

        let checkbox = h_flex()
            .id(self.id.clone())
            .justify_center()
            .items_center()
            .size(size)
            .group(group_id.clone())
            .child(
                div()
                    .flex()
                    .flex_none()
                    .justify_center()
                    .items_center()
                    .m(self.render_cx.scaled_rems(0.25)) // was .m_1
                    .size(self.render_cx.scaled_rems(1.0)) // was .size_4
                    .rounded(self.render_cx.scaled_rems(0.125)) // was .rounded_xs
                    .border_1()
                    .bg(bg_color)
                    .border_color(border_color)
                    .when(self.disabled, |this| this.cursor_not_allowed())
                    .when(self.disabled, |this| {
                        this.bg(cx.theme().colors().element_disabled.opacity(0.6))
                    })
                    .when(!self.disabled, |this| {
                        this.group_hover(group_id.clone(), |el| el.border_color(hover_border_color))
                    })
                    .when(self.placeholder, |this| {
                        this.child(
                            div()
                                .flex_none()
                                .rounded_full()
                                .bg(color.color(cx).alpha(0.5))
                                .size(self.render_cx.scaled_rems(0.25)), // was .size_1
                        )
                    })
                    .children(icon),
            );

        h_flex()
            .id(self.id)
            .gap(ui::DynamicSpacing::Base06.rems(cx))
            .child(checkbox)
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        on_click(&self.toggle_state.inverse(), window, cx)
                    })
                },
            )
            // TODO: Allow label size to be different from default.
            // TODO: Allow label color to be different from muted.
            .when_some(self.label, |this, label| {
                this.child(Label::new(label).color(Color::Muted))
            })
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
    }
}

fn paragraph_len(paragraphs: &MarkdownParagraph) -> usize {
    paragraphs
        .iter()
        .map(|paragraph| match paragraph {
            MarkdownParagraphChunk::Text(text) => text.contents.len(),
            // TODO: Scale column width based on image size
            MarkdownParagraphChunk::Image(_) => 1,
        })
        .sum()
}

fn render_markdown_table(parsed: &ParsedMarkdownTable, cx: &mut RenderContext) -> AnyElement {
    let mut max_lengths: Vec<usize> = vec![0; parsed.header.children.len()];

    for (index, cell) in parsed.header.children.iter().enumerate() {
        let length = paragraph_len(cell);
        max_lengths[index] = length;
    }

    for row in &parsed.body {
        for (index, cell) in row.children.iter().enumerate() {
            let length = paragraph_len(cell);

            if length > max_lengths[index] {
                max_lengths[index] = length;
            }
        }
    }

    let total_max_length: usize = max_lengths.iter().sum();
    let max_column_widths: Vec<f32> = max_lengths
        .iter()
        .map(|&length| length as f32 / total_max_length as f32)
        .collect();

    let header = render_markdown_table_row(
        &parsed.header,
        &parsed.column_alignments,
        &max_column_widths,
        true,
        cx,
    );

    let body: Vec<AnyElement> = parsed
        .body
        .iter()
        .map(|row| {
            render_markdown_table_row(
                row,
                &parsed.column_alignments,
                &max_column_widths,
                false,
                cx,
            )
        })
        .collect();

    cx.with_common_p(v_flex())
        .w_full()
        .child(header)
        .children(body)
        .into_any()
}

fn render_markdown_table_row(
    parsed: &ParsedMarkdownTableRow,
    alignments: &Vec<ParsedMarkdownTableAlignment>,
    max_column_widths: &Vec<f32>,
    is_header: bool,
    cx: &mut RenderContext,
) -> AnyElement {
    let mut items = vec![];

    for (index, cell) in parsed.children.iter().enumerate() {
        let alignment = alignments
            .get(index)
            .copied()
            .unwrap_or(ParsedMarkdownTableAlignment::None);

        let contents = render_markdown_text(cell, cx);

        let container = match alignment {
            ParsedMarkdownTableAlignment::Left | ParsedMarkdownTableAlignment::None => div(),
            ParsedMarkdownTableAlignment::Center => v_flex().items_center(),
            ParsedMarkdownTableAlignment::Right => v_flex().items_end(),
        };

        let max_width = max_column_widths.get(index).unwrap_or(&0.0);
        let mut cell = container
            .w(Length::Definite(relative(*max_width)))
            .h_full()
            .children(contents)
            .px_2()
            .py_1()
            .border_color(cx.border_color);

        if is_header {
            cell = cell.border_2()
        } else {
            cell = cell.border_1()
        }

        items.push(cell);
    }

    h_flex().children(items).into_any_element()
}

fn render_markdown_block_quote(
    parsed: &ParsedMarkdownBlockQuote,
    cx: &mut RenderContext,
) -> AnyElement {
    cx.indent += 1;

    let children: Vec<AnyElement> = parsed
        .children
        .iter()
        .map(|child| render_markdown_block(child, cx))
        .collect();

    cx.indent -= 1;

    cx.with_common_p(div())
        .child(
            div()
                .border_l_4()
                .border_color(cx.border_color)
                .pl_3()
                .children(children),
        )
        .into_any()
}

fn render_markdown_code_block(
    parsed: &ParsedMarkdownCodeBlock,
    cx: &mut RenderContext,
) -> AnyElement {
    let body = if let Some(highlights) = parsed.highlights.as_ref() {
        StyledText::new(parsed.contents.clone()).with_default_highlights(
            &cx.buffer_text_style,
            highlights.iter().filter_map(|(range, highlight_id)| {
                highlight_id
                    .style(cx.syntax_theme.as_ref())
                    .map(|style| (range.clone(), style))
            }),
        )
    } else {
        StyledText::new(parsed.contents.clone())
    };

    let copy_block_button = IconButton::new("copy-code", IconName::Copy)
        .icon_size(IconSize::Small)
        .on_click({
            let contents = parsed.contents.clone();
            move |_, _window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(contents.to_string()));
            }
        })
        .tooltip(Tooltip::text("Copy code block"))
        .visible_on_hover("markdown-block");

    cx.with_common_p(div())
        .font_family(cx.buffer_font_family.clone())
        .px_3()
        .py_3()
        .bg(cx.code_block_background_color)
        .rounded_sm()
        .child(body)
        .child(
            div()
                .h_flex()
                .absolute()
                .right_1()
                .top_1()
                .child(copy_block_button),
        )
        .into_any()
}

fn render_markdown_paragraph(parsed: &MarkdownParagraph, cx: &mut RenderContext) -> AnyElement {
    cx.with_common_p(div())
        .children(render_markdown_text(parsed, cx))
        .flex()
        .flex_col()
        .into_any_element()
}

fn render_markdown_text(parsed_new: &MarkdownParagraph, cx: &mut RenderContext) -> Vec<AnyElement> {
    let mut any_element = vec![];
    // these values are cloned in-order satisfy borrow checker
    let syntax_theme = cx.syntax_theme.clone();
    let workspace_clone = cx.workspace.clone();
    let code_span_bg_color = cx.code_span_background_color;
    let text_style = cx.text_style.clone();

    for parsed_region in parsed_new {
        match parsed_region {
            MarkdownParagraphChunk::Text(parsed) => {
                let element_id = cx.next_id(&parsed.source_range);

                let highlights = gpui::combine_highlights(
                    parsed.highlights.iter().filter_map(|(range, highlight)| {
                        highlight
                            .to_highlight_style(&syntax_theme)
                            .map(|style| (range.clone(), style))
                    }),
                    parsed.regions.iter().zip(&parsed.region_ranges).filter_map(
                        |(region, range)| {
                            if region.code {
                                Some((
                                    range.clone(),
                                    HighlightStyle {
                                        background_color: Some(code_span_bg_color),
                                        ..Default::default()
                                    },
                                ))
                            } else {
                                None
                            }
                        },
                    ),
                );
                let mut links = Vec::new();
                let mut link_ranges = Vec::new();
                for (range, region) in parsed.region_ranges.iter().zip(&parsed.regions) {
                    if let Some(link) = region.link.clone() {
                        links.push(link);
                        link_ranges.push(range.clone());
                    }
                }
                let workspace = workspace_clone.clone();
                let element = div()
                    .child(
                        InteractiveText::new(
                            element_id,
                            StyledText::new(parsed.contents.clone())
                                .with_default_highlights(&text_style, highlights),
                        )
                        .tooltip({
                            let links = links.clone();
                            let link_ranges = link_ranges.clone();
                            move |idx, _, cx| {
                                for (ix, range) in link_ranges.iter().enumerate() {
                                    if range.contains(&idx) {
                                        return Some(LinkPreview::new(&links[ix].to_string(), cx));
                                    }
                                }
                                None
                            }
                        })
                        .on_click(
                            link_ranges,
                            move |clicked_range_ix, window, cx| match &links[clicked_range_ix] {
                                Link::Web { url } => cx.open_url(url),
                                Link::Path { path, .. } => {
                                    if let Some(workspace) = &workspace {
                                        _ = workspace.update(cx, |workspace, cx| {
                                            workspace
                                                .open_abs_path(
                                                    normalize_path(path.clone().as_path()),
                                                    OpenOptions {
                                                        visible: Some(OpenVisible::None),
                                                        ..Default::default()
                                                    },
                                                    window,
                                                    cx,
                                                )
                                                .detach();
                                        });
                                    }
                                }
                            },
                        ),
                    )
                    .into_any();
                any_element.push(element);
            }

            MarkdownParagraphChunk::Image(image) => {
                let image_resource = match image.link.clone() {
                    Link::Web { url } => Resource::Uri(url.into()),
                    Link::Path { path, .. } => Resource::Path(Arc::from(path)),
                };

                let element_id = cx.next_id(&image.source_range);

                let image_element = div()
                    .id(element_id)
                    .cursor_pointer()
                    .child(
                        img(ImageSource::Resource(image_resource))
                            .max_w_full()
                            .with_fallback({
                                let alt_text = image.alt_text.clone();
                                move || div().children(alt_text.clone()).into_any_element()
                            }),
                    )
                    .tooltip({
                        let link = image.link.clone();
                        move |_, cx| {
                            InteractiveMarkdownElementTooltip::new(
                                Some(link.to_string()),
                                "open image",
                                cx,
                            )
                            .into()
                        }
                    })
                    .on_click({
                        let workspace = workspace_clone.clone();
                        let link = image.link.clone();
                        move |_, window, cx| {
                            if window.modifiers().secondary() {
                                match &link {
                                    Link::Web { url } => cx.open_url(url),
                                    Link::Path { path, .. } => {
                                        if let Some(workspace) = &workspace {
                                            _ = workspace.update(cx, |workspace, cx| {
                                                workspace
                                                    .open_abs_path(
                                                        path.clone(),
                                                        OpenOptions {
                                                            visible: Some(OpenVisible::None),
                                                            ..Default::default()
                                                        },
                                                        window,
                                                        cx,
                                                    )
                                                    .detach();
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    })
                    .into_any();
                any_element.push(image_element);
            }
        }
    }

    any_element
}

fn render_markdown_rule(cx: &mut RenderContext) -> AnyElement {
    let rule = div().w_full().h(cx.scaled_rems(0.125)).bg(cx.border_color);
    div().py(cx.scaled_rems(0.5)).child(rule).into_any()
}

struct InteractiveMarkdownElementTooltip {
    tooltip_text: Option<SharedString>,
    action_text: String,
}

impl InteractiveMarkdownElementTooltip {
    pub fn new(tooltip_text: Option<String>, action_text: &str, cx: &mut App) -> Entity<Self> {
        let tooltip_text = tooltip_text.map(|t| util::truncate_and_trailoff(&t, 50).into());

        cx.new(|_cx| Self {
            tooltip_text,
            action_text: action_text.to_string(),
        })
    }
}

impl Render for InteractiveMarkdownElementTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(window, cx, |el, _, _| {
            let secondary_modifier = Keystroke {
                modifiers: Modifiers::secondary_key(),
                ..Default::default()
            };

            el.child(
                v_flex()
                    .gap_1()
                    .when_some(self.tooltip_text.clone(), |this, text| {
                        this.child(Label::new(text).size(LabelSize::Small))
                    })
                    .child(
                        Label::new(format!(
                            "{}-click to {}",
                            secondary_modifier, self.action_text
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    ),
            )
        })
    }
}
