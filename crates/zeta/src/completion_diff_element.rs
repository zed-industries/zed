use std::cmp;

use crate::InlineCompletion;
use gpui::{
    AnyElement, App, BorderStyle, Bounds, Corners, Edges, HighlightStyle, Hsla, StyledText,
    TextLayout, TextStyle, point, prelude::*, quad, size,
};
use language::OffsetRangeExt;
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;

pub struct CompletionDiffElement {
    element: AnyElement,
    text_layout: TextLayout,
    cursor_offset: usize,
}

impl CompletionDiffElement {
    pub fn new(completion: &InlineCompletion, cx: &App) -> Self {
        let mut diff = completion
            .snapshot
            .text_for_range(completion.excerpt_range.clone())
            .collect::<String>();

        let mut cursor_offset_in_diff = None;
        let mut delta = 0;
        let mut diff_highlights = Vec::new();
        for (old_range, new_text) in completion.edits.iter() {
            let old_range = old_range.to_offset(&completion.snapshot);

            if cursor_offset_in_diff.is_none() && completion.cursor_offset <= old_range.end {
                cursor_offset_in_diff =
                    Some(completion.cursor_offset - completion.excerpt_range.start + delta);
            }

            let old_start_in_diff = old_range.start - completion.excerpt_range.start + delta;
            let old_end_in_diff = old_range.end - completion.excerpt_range.start + delta;
            if old_start_in_diff < old_end_in_diff {
                diff_highlights.push((
                    old_start_in_diff..old_end_in_diff,
                    HighlightStyle {
                        background_color: Some(cx.theme().status().deleted_background),
                        strikethrough: Some(gpui::StrikethroughStyle {
                            thickness: px(1.),
                            color: Some(cx.theme().colors().text_muted),
                        }),
                        ..Default::default()
                    },
                ));
            }

            if !new_text.is_empty() {
                diff.insert_str(old_end_in_diff, new_text);
                diff_highlights.push((
                    old_end_in_diff..old_end_in_diff + new_text.len(),
                    HighlightStyle {
                        background_color: Some(cx.theme().status().created_background),
                        ..Default::default()
                    },
                ));
                delta += new_text.len();
            }
        }

        let cursor_offset_in_diff = cursor_offset_in_diff
            .unwrap_or_else(|| completion.cursor_offset - completion.excerpt_range.start + delta);

        let settings = ThemeSettings::get_global(cx).clone();
        let text_style = TextStyle {
            color: cx.theme().colors().editor_foreground,
            font_size: settings.buffer_font_size(cx).into(),
            font_family: settings.buffer_font.family,
            font_features: settings.buffer_font.features,
            font_fallbacks: settings.buffer_font.fallbacks,
            line_height: relative(settings.buffer_line_height.value()),
            font_weight: settings.buffer_font.weight,
            font_style: settings.buffer_font.style,
            ..Default::default()
        };
        let element = StyledText::new(diff).with_default_highlights(&text_style, diff_highlights);
        let text_layout = element.layout().clone();

        CompletionDiffElement {
            element: element.into_any_element(),
            text_layout,
            cursor_offset: cursor_offset_in_diff,
        }
    }
}

impl IntoElement for CompletionDiffElement {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for CompletionDiffElement {
    type RequestLayoutState = ();
    type PrepaintState = ();
    type DebugState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        (self.element.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _bounds: gpui::Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(position) = self.text_layout.position_for_index(self.cursor_offset) {
            let bounds = self.text_layout.bounds();
            let line_height = self.text_layout.line_height();
            let line_width = self
                .text_layout
                .line_layout_for_index(self.cursor_offset)
                .map_or(bounds.size.width, |layout| layout.width());
            window.paint_quad(quad(
                Bounds::new(
                    point(bounds.origin.x, position.y),
                    size(cmp::max(bounds.size.width, line_width), line_height),
                ),
                Corners::default(),
                cx.theme().colors().editor_active_line_background,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));
            self.element.paint(window, cx);
            window.paint_quad(quad(
                Bounds::new(position, size(px(2.), line_height)),
                Corners::default(),
                cx.theme().players().local().cursor,
                Edges::default(),
                Hsla::transparent_black(),
                BorderStyle::default(),
            ));
        }
    }
}
