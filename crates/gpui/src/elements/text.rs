use crate::{
    ActiveTooltip, AnyTooltip, AnyView, Bounds, DispatchPhase, Element, ElementContext, ElementId,
    HighlightStyle, Hitbox, IntoElement, LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point, SharedString, Size, TextRun, TextStyle, WhiteSpace, WindowContext, WrappedLine,
    TOOLTIP_DELAY,
};
use anyhow::anyhow;
use parking_lot::{Mutex, MutexGuard};
use smallvec::SmallVec;
use std::{
    cell::{Cell, RefCell},
    mem,
    ops::Range,
    rc::Rc,
    sync::Arc,
};
use util::ResultExt;

impl Element for &'static str {
    type BeforeLayout = TextState;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let mut state = TextState::default();
        let layout_id = state.layout(SharedString::from(*self), None, cx);
        (layout_id, state)
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        _text_state: &mut Self::BeforeLayout,
        _cx: &mut ElementContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        text_state: &mut TextState,
        _: &mut (),
        cx: &mut ElementContext,
    ) {
        text_state.paint(bounds, self, cx)
    }
}

impl IntoElement for &'static str {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl IntoElement for String {
    type Element = SharedString;

    fn into_element(self) -> Self::Element {
        self.into()
    }
}

impl Element for SharedString {
    type BeforeLayout = TextState;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let mut state = TextState::default();
        let layout_id = state.layout(self.clone(), None, cx);
        (layout_id, state)
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        _text_state: &mut Self::BeforeLayout,
        _cx: &mut ElementContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        text_state: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        let text_str: &str = self.as_ref();
        text_state.paint(bounds, text_str, cx)
    }
}

impl IntoElement for SharedString {
    type Element = Self;

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
    /// Construct a new styled text element from the given string.
    pub fn new(text: impl Into<SharedString>) -> Self {
        StyledText {
            text: text.into(),
            runs: None,
        }
    }

    /// Set the styling attributes for the given text, as well as
    /// as any ranges of text that have had their style customized.
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
    type BeforeLayout = TextState;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let mut state = TextState::default();
        let layout_id = state.layout(self.text.clone(), self.runs.take(), cx);
        (layout_id, state)
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::BeforeLayout,
        _cx: &mut ElementContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        text_state: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        text_state.paint(bounds, &self.text, cx)
    }
}

impl IntoElement for StyledText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[doc(hidden)]
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
        cx: &mut ElementContext,
    ) -> LayoutId {
        let text_style = cx.text_style();
        let font_size = text_style.font_size.to_pixels(cx.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), cx.rem_size());

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
                        text.clone(),
                        font_size,
                        &runs,
                        wrap_width, // Wrap if we know the width.
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

    fn paint(&mut self, bounds: Bounds<Pixels>, text: &str, cx: &mut ElementContext) {
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

/// A text element that can be interacted with.
pub struct InteractiveText {
    element_id: ElementId,
    text: StyledText,
    click_listener:
        Option<Box<dyn Fn(&[Range<usize>], InteractiveTextClickEvent, &mut WindowContext<'_>)>>,
    hover_listener: Option<Box<dyn Fn(Option<usize>, MouseMoveEvent, &mut WindowContext<'_>)>>,
    tooltip_builder: Option<Rc<dyn Fn(usize, &mut WindowContext<'_>) -> Option<AnyView>>>,
    clickable_ranges: Vec<Range<usize>>,
}

struct InteractiveTextClickEvent {
    mouse_down_index: usize,
    mouse_up_index: usize,
}

#[doc(hidden)]
#[derive(Default)]
pub struct InteractiveTextState {
    mouse_down_index: Rc<Cell<Option<usize>>>,
    hovered_index: Rc<Cell<Option<usize>>>,
    active_tooltip: Rc<RefCell<Option<ActiveTooltip>>>,
}

/// InteractiveTest is a wrapper around StyledText that adds mouse interactions.
impl InteractiveText {
    /// Creates a new InteractiveText from the given text.
    pub fn new(id: impl Into<ElementId>, text: StyledText) -> Self {
        Self {
            element_id: id.into(),
            text,
            click_listener: None,
            hover_listener: None,
            tooltip_builder: None,
            clickable_ranges: Vec::new(),
        }
    }

    /// on_click is called when the user clicks on one of the given ranges, passing the index of
    /// the clicked range.
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

    /// on_hover is called when the mouse moves over a character within the text, passing the
    /// index of the hovered character, or None if the mouse leaves the text.
    pub fn on_hover(
        mut self,
        listener: impl Fn(Option<usize>, MouseMoveEvent, &mut WindowContext<'_>) + 'static,
    ) -> Self {
        self.hover_listener = Some(Box::new(listener));
        self
    }

    /// tooltip lets you specify a tooltip for a given character index in the string.
    pub fn tooltip(
        mut self,
        builder: impl Fn(usize, &mut WindowContext<'_>) -> Option<AnyView> + 'static,
    ) -> Self {
        self.tooltip_builder = Some(Rc::new(builder));
        self
    }
}

impl Element for InteractiveText {
    type BeforeLayout = TextState;
    type AfterLayout = Hitbox;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        self.text.before_layout(cx)
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Hitbox {
        self.text.after_layout(bounds, state, cx);
        cx.insert_hitbox(bounds, false)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        text_state: &mut Self::BeforeLayout,
        hitbox: &mut Hitbox,
        cx: &mut ElementContext,
    ) {
        cx.with_element_state::<InteractiveTextState, _>(
            Some(self.element_id.clone()),
            |interactive_state, cx| {
                let mut interactive_state = interactive_state.unwrap().unwrap_or_default();
                if let Some(click_listener) = self.click_listener.take() {
                    let mouse_position = cx.mouse_position();
                    if let Some(ix) = text_state.index_for_position(bounds, mouse_position) {
                        if self
                            .clickable_ranges
                            .iter()
                            .any(|range| range.contains(&ix))
                        {
                            cx.set_cursor_style(crate::CursorStyle::PointingHand, hitbox)
                        }
                    }

                    let text_state = text_state.clone();
                    let mouse_down = interactive_state.mouse_down_index.clone();
                    if let Some(mouse_down_index) = mouse_down.get() {
                        let hitbox = hitbox.clone();
                        let clickable_ranges = mem::take(&mut self.clickable_ranges);
                        cx.on_mouse_event(move |event: &MouseUpEvent, phase, cx| {
                            if phase == DispatchPhase::Bubble && hitbox.is_hovered(cx) {
                                if let Some(mouse_up_index) =
                                    text_state.index_for_position(bounds, event.position)
                                {
                                    click_listener(
                                        &clickable_ranges,
                                        InteractiveTextClickEvent {
                                            mouse_down_index,
                                            mouse_up_index,
                                        },
                                        cx,
                                    )
                                }

                                mouse_down.take();
                                cx.refresh();
                            }
                        });
                    } else {
                        let hitbox = hitbox.clone();
                        cx.on_mouse_event(move |event: &MouseDownEvent, phase, cx| {
                            if phase == DispatchPhase::Bubble && hitbox.is_hovered(cx) {
                                if let Some(mouse_down_index) =
                                    text_state.index_for_position(bounds, event.position)
                                {
                                    mouse_down.set(Some(mouse_down_index));
                                    cx.refresh();
                                }
                            }
                        });
                    }
                }

                cx.on_mouse_event({
                    let mut hover_listener = self.hover_listener.take();
                    let hitbox = hitbox.clone();
                    let text_state = text_state.clone();
                    let hovered_index = interactive_state.hovered_index.clone();
                    move |event: &MouseMoveEvent, phase, cx| {
                        if phase == DispatchPhase::Bubble && hitbox.is_hovered(cx) {
                            let current = hovered_index.get();
                            let updated = text_state.index_for_position(bounds, event.position);
                            if current != updated {
                                hovered_index.set(updated);
                                if let Some(hover_listener) = hover_listener.as_ref() {
                                    hover_listener(updated, event.clone(), cx);
                                }
                                cx.refresh();
                            }
                        }
                    }
                });

                if let Some(tooltip_builder) = self.tooltip_builder.clone() {
                    let hitbox = hitbox.clone();
                    let active_tooltip = interactive_state.active_tooltip.clone();
                    let pending_mouse_down = interactive_state.mouse_down_index.clone();
                    let text_state = text_state.clone();

                    cx.on_mouse_event(move |event: &MouseMoveEvent, phase, cx| {
                        let position = text_state.index_for_position(bounds, event.position);
                        let is_hovered = position.is_some()
                            && hitbox.is_hovered(cx)
                            && pending_mouse_down.get().is_none();
                        if !is_hovered {
                            active_tooltip.take();
                            return;
                        }
                        let position = position.unwrap();

                        if phase != DispatchPhase::Bubble {
                            return;
                        }

                        if active_tooltip.borrow().is_none() {
                            let task = cx.spawn({
                                let active_tooltip = active_tooltip.clone();
                                let tooltip_builder = tooltip_builder.clone();

                                move |mut cx| async move {
                                    cx.background_executor().timer(TOOLTIP_DELAY).await;
                                    cx.update(|cx| {
                                        let new_tooltip =
                                            tooltip_builder(position, cx).map(|tooltip| {
                                                ActiveTooltip {
                                                    tooltip: Some(AnyTooltip {
                                                        view: tooltip,
                                                        cursor_offset: cx.mouse_position(),
                                                    }),
                                                    _task: None,
                                                }
                                            });
                                        *active_tooltip.borrow_mut() = new_tooltip;
                                        cx.refresh();
                                    })
                                    .ok();
                                }
                            });
                            *active_tooltip.borrow_mut() = Some(ActiveTooltip {
                                tooltip: None,
                                _task: Some(task),
                            });
                        }
                    });

                    let active_tooltip = interactive_state.active_tooltip.clone();
                    cx.on_mouse_event(move |_: &MouseDownEvent, _, _| {
                        active_tooltip.take();
                    });

                    if let Some(tooltip) = interactive_state
                        .active_tooltip
                        .clone()
                        .borrow()
                        .as_ref()
                        .and_then(|at| at.tooltip.clone())
                    {
                        cx.set_tooltip(tooltip);
                    }
                }

                self.text.paint(bounds, text_state, &mut (), cx);

                ((), Some(interactive_state))
            },
        );
    }
}

impl IntoElement for InteractiveText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
