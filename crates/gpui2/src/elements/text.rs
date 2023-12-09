use crate::{
    Bounds, DispatchPhase, Element, ElementId, HighlightStyle, IntoElement, LayoutId,
    MouseDownEvent, MouseUpEvent, Pixels, Point, SharedString, Size, TextRun, TextStyle,
    WhiteSpace, WindowContext, WrappedLine,
};
use anyhow::anyhow;
use parking_lot::{Mutex, MutexGuard};
use smallvec::SmallVec;
use std::{cell::Cell, ops::Range, rc::Rc, sync::Arc};
use util::ResultExt;

impl Element for &'static str {
    type State = TextState;

    fn layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let mut state = TextState::default();
        let layout_id = state.layout(SharedString::from(*self), None, cx);
        (layout_id, state)
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut TextState, cx: &mut WindowContext) {
        state.paint(bounds, self, cx)
    }
}

impl IntoElement for &'static str {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SharedString {
    type State = TextState;

    fn layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let mut state = TextState::default();
        let layout_id = state.layout(self.clone(), None, cx);
        (layout_id, state)
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut TextState, cx: &mut WindowContext) {
        let text_str: &str = self.as_ref();
        state.paint(bounds, text_str, cx)
    }
}

impl IntoElement for SharedString {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

/// Renders text with runs of different styles.
///
/// Callers are responsible for setting the correct style for each run.
/// For text with a uniform style, you can usually avoid calling this constructor
/// and just pass text directly.
pub struct StyledText {
    text: SharedString,
    runs: Option<Vec<TextRun>>,
}

impl StyledText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        StyledText {
            text: text.into(),
            runs: None,
        }
    }

    pub fn with_highlights(
        mut self,
        default_style: &TextStyle,
        highlights: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
    ) -> Self {
        let mut runs = Vec::new();
        let mut ix = 0;
        for (range, highlight) in highlights {
            if ix < range.start {
                runs.push(default_style.clone().to_run(range.start - ix));
            }
            runs.push(
                default_style
                    .clone()
                    .highlight(highlight)
                    .to_run(range.len()),
            );
            ix = range.end;
        }
        if ix < self.text.len() {
            runs.push(default_style.to_run(self.text.len() - ix));
        }
        self.runs = Some(runs);
        self
    }
}

impl Element for StyledText {
    type State = TextState;

    fn layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let mut state = TextState::default();
        let layout_id = state.layout(self.text.clone(), self.runs.take(), cx);
        (layout_id, state)
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        state.paint(bounds, &self.text, cx)
    }
}

impl IntoElement for StyledText {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Default, Clone)]
pub struct TextState(Arc<Mutex<Option<TextStateInner>>>);

struct TextStateInner {
    lines: SmallVec<[WrappedLine; 1]>,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    size: Option<Size<Pixels>>,
}

impl TextState {
    fn lock(&self) -> MutexGuard<Option<TextStateInner>> {
        self.0.lock()
    }

    fn layout(
        &mut self,
        text: SharedString,
        runs: Option<Vec<TextRun>>,
        cx: &mut WindowContext,
    ) -> LayoutId {
        let text_style = cx.text_style();
        let font_size = text_style.font_size.to_pixels(cx.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), cx.rem_size());
        let text = SharedString::from(text);

        let runs = if let Some(runs) = runs {
            runs
        } else {
            vec![text_style.to_run(text.len())]
        };

        let layout_id = cx.request_measured_layout(Default::default(), {
            let element_state = self.clone();

            move |known_dimensions, available_space, cx| {
                let wrap_width = if text_style.white_space == WhiteSpace::Normal {
                    known_dimensions.width.or(match available_space.width {
                        crate::AvailableSpace::Definite(x) => Some(x),
                        _ => None,
                    })
                } else {
                    None
                };

                if let Some(text_state) = element_state.0.lock().as_ref() {
                    if text_state.size.is_some()
                        && (wrap_width.is_none() || wrap_width == text_state.wrap_width)
                    {
                        return text_state.size.unwrap();
                    }
                }

                let Some(lines) = cx
                    .text_system()
                    .shape_text(
                        &text, font_size, &runs, wrap_width, // Wrap if we know the width.
                    )
                    .log_err()
                else {
                    element_state.lock().replace(TextStateInner {
                        lines: Default::default(),
                        line_height,
                        wrap_width,
                        size: Some(Size::default()),
                    });
                    return Size::default();
                };

                let mut size: Size<Pixels> = Size::default();
                for line in &lines {
                    let line_size = line.size(line_height);
                    size.height += line_size.height;
                    size.width = size.width.max(line_size.width).ceil();
                }

                element_state.lock().replace(TextStateInner {
                    lines,
                    line_height,
                    wrap_width,
                    size: Some(size),
                });

                size
            }
        });

        layout_id
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, text: &str, cx: &mut WindowContext) {
        let element_state = self.lock();
        let element_state = element_state
            .as_ref()
            .ok_or_else(|| anyhow!("measurement has not been performed on {}", text))
            .unwrap();

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        for line in &element_state.lines {
            line.paint(line_origin, line_height, cx).log_err();
            line_origin.y += line.size(line_height).height;
        }
    }

    fn index_for_position(&self, bounds: Bounds<Pixels>, position: Point<Pixels>) -> Option<usize> {
        if !bounds.contains(&position) {
            return None;
        }

        let element_state = self.lock();
        let element_state = element_state
            .as_ref()
            .expect("measurement has not been performed");

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        let mut line_start_ix = 0;
        for line in &element_state.lines {
            let line_bottom = line_origin.y + line.size(line_height).height;
            if position.y > line_bottom {
                line_origin.y = line_bottom;
                line_start_ix += line.len() + 1;
            } else {
                let position_within_line = position - line_origin;
                let index_within_line =
                    line.index_for_position(position_within_line, line_height)?;
                return Some(line_start_ix + index_within_line);
            }
        }

        None
    }
}

pub struct InteractiveText {
    element_id: ElementId,
    text: StyledText,
    click_listener:
        Option<Box<dyn Fn(&[Range<usize>], InteractiveTextClickEvent, &mut WindowContext<'_>)>>,
    clickable_ranges: Vec<Range<usize>>,
}

struct InteractiveTextClickEvent {
    mouse_down_index: usize,
    mouse_up_index: usize,
}

pub struct InteractiveTextState {
    text_state: TextState,
    mouse_down_index: Rc<Cell<Option<usize>>>,
}

impl InteractiveText {
    pub fn new(id: impl Into<ElementId>, text: StyledText) -> Self {
        Self {
            element_id: id.into(),
            text,
            click_listener: None,
            clickable_ranges: Vec::new(),
        }
    }

    pub fn on_click(
        mut self,
        ranges: Vec<Range<usize>>,
        listener: impl Fn(usize, &mut WindowContext<'_>) + 'static,
    ) -> Self {
        self.click_listener = Some(Box::new(move |ranges, event, cx| {
            for (range_ix, range) in ranges.iter().enumerate() {
                if range.contains(&event.mouse_down_index) && range.contains(&event.mouse_up_index)
                {
                    listener(range_ix, cx);
                }
            }
        }));
        self.clickable_ranges = ranges;
        self
    }
}

impl Element for InteractiveText {
    type State = InteractiveTextState;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        if let Some(InteractiveTextState {
            mouse_down_index, ..
        }) = state
        {
            let (layout_id, text_state) = self.text.layout(None, cx);
            let element_state = InteractiveTextState {
                text_state,
                mouse_down_index,
            };
            (layout_id, element_state)
        } else {
            let (layout_id, text_state) = self.text.layout(None, cx);
            let element_state = InteractiveTextState {
                text_state,
                mouse_down_index: Rc::default(),
            };
            (layout_id, element_state)
        }
    }

    fn paint(self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        if let Some(click_listener) = self.click_listener {
            if let Some(ix) = state
                .text_state
                .index_for_position(bounds, cx.mouse_position())
            {
                if self
                    .clickable_ranges
                    .iter()
                    .any(|range| range.contains(&ix))
                {
                    cx.set_cursor_style(crate::CursorStyle::PointingHand)
                }
            }

            let text_state = state.text_state.clone();
            let mouse_down = state.mouse_down_index.clone();
            if let Some(mouse_down_index) = mouse_down.get() {
                cx.on_mouse_event(move |event: &MouseUpEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble {
                        if let Some(mouse_up_index) =
                            text_state.index_for_position(bounds, event.position)
                        {
                            click_listener(
                                &self.clickable_ranges,
                                InteractiveTextClickEvent {
                                    mouse_down_index,
                                    mouse_up_index,
                                },
                                cx,
                            )
                        }

                        mouse_down.take();
                        cx.notify();
                    }
                });
            } else {
                cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                    if phase == DispatchPhase::Bubble {
                        if let Some(mouse_down_index) =
                            text_state.index_for_position(bounds, event.position)
                        {
                            mouse_down.set(Some(mouse_down_index));
                            cx.notify();
                        }
                    }
                });
            }
        }

        self.text.paint(bounds, &mut state.text_state, cx)
    }
}

impl IntoElement for InteractiveText {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.element_id.clone())
    }

    fn into_element(self) -> Self::Element {
        self
    }
}
