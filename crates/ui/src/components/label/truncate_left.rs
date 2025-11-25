use crate::LabelSize;
use crate::prelude::*;
use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, SharedString, TextAlign, TextRun, Window, px,
};
use settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;
use theme::ThemeSettings;

const ELLIPSIS: &str = "…";

pub struct TruncateLeft {
    text: SharedString,
    color: Color,
    size: LabelSize,
    state: Rc<RefCell<TruncateLeftState>>,
}

#[derive(Default)]
struct TruncateLeftState {
    truncated_text: Option<SharedString>,
    text_runs: Option<Vec<TextRun>>,
    line_height: Option<Pixels>,
    font_size: Option<Pixels>,
}

impl TruncateLeft {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            color: Color::Default,
            size: LabelSize::Default,
            state: Rc::new(RefCell::new(TruncateLeftState::default())),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    fn measure_text_width(
        text: &str,
        font_size: Pixels,
        runs: &[TextRun],
        window: &Window,
    ) -> Option<Pixels> {
        let adjusted_runs = vec![TextRun {
            len: text.len(),
            ..runs.first().cloned().unwrap_or_default()
        }];

        let lines = window
            .text_system()
            .shape_text(
                SharedString::from(text.to_string()),
                font_size,
                &adjusted_runs,
                None,
                None,
            )
            .ok()?;

        Some(
            lines
                .iter()
                .map(|line| line.width())
                .fold(px(0.), |a, b| a.max(b)),
        )
    }

    fn compute_truncated_text(
        &self,
        available_width: Pixels,
        font_size: Pixels,
        runs: &[TextRun],
        window: &Window,
    ) -> SharedString {
        let full_width = match Self::measure_text_width(&self.text, font_size, runs, window) {
            Some(w) => w,
            None => return self.text.clone(),
        };

        if full_width <= available_width {
            return self.text.clone();
        }

        let ellipsis_width =
            Self::measure_text_width(ELLIPSIS, font_size, runs, window).unwrap_or(px(10.));

        let available_for_text = available_width - ellipsis_width;

        if available_for_text <= px(0.) {
            return SharedString::from(ELLIPSIS);
        }

        let char_indices: Vec<(usize, char)> = self.text.char_indices().collect();

        if char_indices.is_empty() {
            return self.text.clone();
        }

        let mut best_start_idx = self.text.len();

        for (byte_idx, _) in char_indices.iter().rev() {
            let suffix = &self.text[*byte_idx..];
            if let Some(suffix_width) = Self::measure_text_width(suffix, font_size, runs, window) {
                if suffix_width <= available_for_text {
                    best_start_idx = *byte_idx;
                } else {
                    break;
                }
            }
        }

        if best_start_idx >= self.text.len() {
            SharedString::from(ELLIPSIS)
        } else if best_start_idx == 0 {
            self.text.clone()
        } else {
            SharedString::from(format!("{}{}", ELLIPSIS, &self.text[best_start_idx..]))
        }
    }
}

impl IntoElement for TruncateLeft {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TruncateLeft {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();
        let font_size = match self.size {
            LabelSize::Large => TextSize::Large.rems(cx),
            LabelSize::Default => TextSize::Default.rems(cx),
            LabelSize::Small => TextSize::Small.rems(cx),
            LabelSize::XSmall => TextSize::XSmall.rems(cx),
        };

        let rem_size = window.rem_size();
        let font_size_px = font_size.to_pixels(rem_size);
        let line_height = font_size_px * 1.3;

        let color = self.color.color(cx);

        let runs = vec![TextRun {
            len: self.text.len(),
            font: gpui::Font {
                family: ui_font.family.clone(),
                features: ui_font.features.clone(),
                fallbacks: ui_font.fallbacks.clone(),
                weight: ui_font.weight,
                style: gpui::FontStyle::Normal,
            },
            color,
            underline: None,
            strikethrough: None,
            background_color: None,
        }];

        {
            let mut state = self.state.borrow_mut();
            state.text_runs = Some(runs);
            state.line_height = Some(line_height);
            state.font_size = Some(font_size_px);
        }

        let style = gpui::Style {
            flex_grow: 1.0,
            flex_shrink: 1.0,
            min_size: gpui::Size {
                width: gpui::Length::Definite(gpui::DefiniteLength::Absolute(
                    gpui::AbsoluteLength::Pixels(px(0.)),
                )),
                height: gpui::Length::Auto,
            },
            size: gpui::Size {
                width: gpui::Length::Auto,
                height: gpui::Length::Definite(gpui::DefiniteLength::Absolute(
                    gpui::AbsoluteLength::Pixels(line_height),
                )),
            },
            ..Default::default()
        };

        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let state = self.state.borrow();
        if let (Some(runs), Some(font_size)) = (&state.text_runs, state.font_size) {
            let truncated = self.compute_truncated_text(bounds.size.width, font_size, runs, window);
            drop(state);
            self.state.borrow_mut().truncated_text = Some(truncated);
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let state = self.state.borrow();
        let text = state
            .truncated_text
            .clone()
            .unwrap_or_else(|| self.text.clone());
        let runs = state.text_runs.clone();
        let line_height = state.line_height;
        let font_size = state.font_size;
        drop(state);

        if let (Some(runs), Some(line_height), Some(font_size)) = (runs, line_height, font_size) {
            let adjusted_runs = vec![TextRun {
                len: text.len(),
                ..runs.first().cloned().unwrap_or_default()
            }];

            if let Ok(lines) =
                window
                    .text_system()
                    .shape_text(text, font_size, &adjusted_runs, None, None)
            {
                for line in lines {
                    let _ = line.paint(
                        bounds.origin,
                        line_height,
                        TextAlign::Left,
                        Some(bounds),
                        window,
                        cx,
                    );
                }
            }
        }
    }
}
