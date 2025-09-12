use editor::Editor;
use gpui::{
    Context, Element, EventEmitter, Focusable, FontWeight, IntoElement, ParentElement, Render,
    StyledText, Subscription, Window,
};
use itertools::Itertools;
use settings::Settings;
use std::cmp;
use theme::ActiveTheme;
use ui::{ButtonLike, ButtonStyle, Label, Tooltip, prelude::*};
use workspace::{
    TabBarSettings, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    item::{BreadcrumbText, ItemEvent, ItemHandle},
};

pub struct Breadcrumbs {
    pane_focused: bool,
    active_item: Option<Box<dyn ItemHandle>>,
    subscription: Option<Subscription>,
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

impl Breadcrumbs {
    pub fn new() -> Self {
        Self {
            pane_focused: false,
            active_item: Default::default(),
            subscription: Default::default(),
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for Breadcrumbs {}

impl Render for Breadcrumbs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const MAX_SEGMENTS: usize = 12;

        let element = h_flex()
            .id("breadcrumb-container")
            .flex_grow()
            .overflow_x_scroll()
            .text_ui(cx);

        let Some(active_item) = self.active_item.as_ref() else {
            return element;
        };

        let Some(mut segments) = active_item.breadcrumbs(cx.theme(), cx) else {
            return element;
        };

        let prefix_end_ix = cmp::min(segments.len(), MAX_SEGMENTS / 2);
        let suffix_start_ix = cmp::max(
            prefix_end_ix,
            segments.len().saturating_sub(MAX_SEGMENTS / 2),
        );

        if suffix_start_ix > prefix_end_ix {
            segments.splice(
                prefix_end_ix..suffix_start_ix,
                Some(BreadcrumbText {
                    text: "⋯".into(),
                    highlights: None,
                    font: None,
                }),
            );
        }

        let highlighted_segments = segments.into_iter().enumerate().map(|(index, segment)| {
            let mut text_style = window.text_style();
            if let Some(ref font) = segment.font {
                text_style.font_family = font.family.clone();
                text_style.font_features = font.features.clone();
                text_style.font_style = font.style;
                text_style.font_weight = font.weight;
            }
            text_style.color = Color::Muted.color(cx);

            if index == 0
                && !TabBarSettings::get_global(cx).show
                && active_item.is_dirty(cx)
                && let Some(styled_element) = apply_dirty_filename_style(&segment, &text_style, cx)
            {
                return styled_element;
            }

            StyledText::new(segment.text.replace('\n', "⏎"))
                .with_default_highlights(&text_style, segment.highlights.unwrap_or_default())
                .into_any()
        });
        let breadcrumbs = Itertools::intersperse_with(highlighted_segments, || {
            Label::new("›").color(Color::Placeholder).into_any_element()
        });

        let breadcrumbs_stack = h_flex().gap_1().children(breadcrumbs);

        match active_item
            .downcast::<Editor>()
            .map(|editor| editor.downgrade())
        {
            Some(editor) => element.child(
                ButtonLike::new("toggle outline view")
                    .child(breadcrumbs_stack)
                    .style(ButtonStyle::Transparent)
                    .on_click({
                        let editor = editor.clone();
                        move |_, window, cx| {
                            if let Some((editor, callback)) = editor
                                .upgrade()
                                .zip(zed_actions::outline::TOGGLE_OUTLINE.get())
                            {
                                callback(editor.to_any(), window, cx);
                            }
                        }
                    })
                    .tooltip(move |window, cx| {
                        if let Some(editor) = editor.upgrade() {
                            let focus_handle = editor.read(cx).focus_handle(cx);
                            Tooltip::for_action_in(
                                "Show Symbol Outline",
                                &zed_actions::outline::ToggleOutline,
                                &focus_handle,
                                window,
                                cx,
                            )
                        } else {
                            Tooltip::for_action(
                                "Show Symbol Outline",
                                &zed_actions::outline::ToggleOutline,
                                window,
                                cx,
                            )
                        }
                    }),
            ),
            None => element
                // Match the height and padding of the `ButtonLike` in the other arm.
                .h(rems_from_px(22.))
                .pl_1()
                .child(breadcrumbs_stack),
        }
    }
}

impl ToolbarItemView for Breadcrumbs {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_item = None;

        let Some(item) = active_pane_item else {
            return ToolbarItemLocation::Hidden;
        };

        let this = cx.entity().downgrade();
        self.subscription = Some(item.subscribe_to_item_events(
            window,
            cx,
            Box::new(move |event, _, cx| {
                if let ItemEvent::UpdateBreadcrumbs = event {
                    this.update(cx, |this, cx| {
                        cx.notify();
                        if let Some(active_item) = this.active_item.as_ref() {
                            cx.emit(ToolbarItemEvent::ChangeLocation(
                                active_item.breadcrumb_location(cx),
                            ))
                        }
                    })
                    .ok();
                }
            }),
        ));
        self.active_item = Some(item.boxed_clone());
        item.breadcrumb_location(cx)
    }

    fn pane_focus_update(
        &mut self,
        pane_focused: bool,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        self.pane_focused = pane_focused;
    }
}

fn apply_dirty_filename_style(
    segment: &BreadcrumbText,
    text_style: &gpui::TextStyle,
    cx: &mut Context<Breadcrumbs>,
) -> Option<gpui::AnyElement> {
    let text = segment.text.replace('\n', "⏎");

    let filename_position = std::path::Path::new(&segment.text)
        .file_name()
        .and_then(|f| {
            let filename_str = f.to_string_lossy();
            segment.text.rfind(filename_str.as_ref())
        })?;

    let bold_weight = FontWeight::BOLD;
    let default_color = Color::Default.color(cx);

    if filename_position == 0 {
        let mut filename_style = text_style.clone();
        filename_style.font_weight = bold_weight;
        filename_style.color = default_color;

        return Some(
            StyledText::new(text)
                .with_default_highlights(&filename_style, [])
                .into_any(),
        );
    }

    let highlight_style = gpui::HighlightStyle {
        font_weight: Some(bold_weight),
        color: Some(default_color),
        ..Default::default()
    };

    let highlight = vec![(filename_position..text.len(), highlight_style)];
    Some(
        StyledText::new(text)
            .with_default_highlights(text_style, highlight)
            .into_any(),
    )
}
