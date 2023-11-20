use crate::{
    BorrowWindow, Bounds, Element, ElementId, LayoutId, Pixels, RenderOnce, SharedString, Size,
    TextRun, ViewContext, WindowContext, WrappedLine,
};
use anyhow::anyhow;
use parking_lot::{Mutex, MutexGuard};
use smallvec::SmallVec;
use std::{cell::Cell, rc::Rc, sync::Arc};
use util::ResultExt;

impl<V: 'static> Element<V> for &'static str {
    type State = TextState;

    fn layout(
        &mut self,
        _: &mut V,
        _: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        let mut state = TextState::default();
        let layout_id = state.layout(SharedString::from(*self), None, cx);
        (layout_id, state)
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        _: &mut V,
        state: &mut TextState,
        cx: &mut ViewContext<V>,
    ) {
        state.paint(bounds, self, cx)
    }
}

impl<V: 'static> RenderOnce<V> for &'static str {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

impl<V: 'static> Element<V> for SharedString {
    type State = TextState;

    fn layout(
        &mut self,
        _: &mut V,
        _: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        let mut state = TextState::default();
        let layout_id = state.layout(self.clone(), None, cx);
        (layout_id, state)
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        _: &mut V,
        state: &mut TextState,
        cx: &mut ViewContext<V>,
    ) {
        let text_str: &str = self.as_ref();
        state.paint(bounds, text_str, cx)
    }
}

impl<V: 'static> RenderOnce<V> for SharedString {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.clone().into())
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

pub struct StyledText {
    text: SharedString,
    runs: Option<Vec<TextRun>>,
}

impl StyledText {
    /// Renders text with runs of different styles.
    ///
    /// Callers are responsible for setting the correct style for each run.
    /// For text with a uniform style, you can usually avoid calling this constructor
    /// and just pass text directly.
    pub fn new(text: SharedString, runs: Vec<TextRun>) -> Self {
        StyledText {
            text,
            runs: Some(runs),
        }
    }
}

impl<V: 'static> Element<V> for StyledText {
    type State = TextState;

    fn layout(
        &mut self,
        _view: &mut V,
        element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        let element_state = element_state.unwrap_or_default();
        let text_system = cx.text_system().clone();
        let text_style = cx.text_style();
        let font_size = text_style.font_size.to_pixels(cx.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), cx.rem_size());
        let text = self.text.clone();

        let rem_size = cx.rem_size();

        let runs = if let Some(runs) = self.runs.take() {
            runs
        } else {
            vec![text_style.to_run(text.len())]
        };

        let layout_id = cx.request_measured_layout(Default::default(), rem_size, {
            let element_state = element_state.clone();
            move |known_dimensions, available_space| {
                let wrap_width = known_dimensions.width.or(match available_space.width {
                    crate::AvailableSpace::Definite(x) => Some(x),
                    _ => None,
                });

                if let Some(text_state) = element_state.0.lock().as_ref() {
                    if text_state.size.is_some()
                        && (wrap_width.is_none() || wrap_width == text_state.wrap_width)
                    {
                        return text_state.size.unwrap();
                    }
                }

                let Some(lines) = text_system
                    .shape_text(
                        &text,
                        font_size,
                        &runs[..],
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
                    size.width = size.width.max(line_size.width);
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

        (layout_id, element_state)
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        _: &mut V,
        element_state: &mut Self::State,
        cx: &mut ViewContext<V>,
    ) {
        let element_state = element_state.lock();
        let element_state = element_state
            .as_ref()
            .ok_or_else(|| anyhow!("measurement has not been performed on {}", &self.text))
            .unwrap();

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        for line in &element_state.lines {
            line.paint(line_origin, line_height, cx).log_err();
            line_origin.y += line.size(line_height).height;
        }
    }
}

impl<V: 'static> RenderOnce<V> for StyledText {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn render_once(self) -> Self::Element {
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
        let text_system = cx.text_system().clone();
        let text_style = cx.text_style();
        let font_size = text_style.font_size.to_pixels(cx.rem_size());
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), cx.rem_size());
        let text = SharedString::from(text);

        let rem_size = cx.rem_size();

        let runs = if let Some(runs) = runs {
            runs
        } else {
            vec![text_style.to_run(text.len())]
        };

        let layout_id = cx.request_measured_layout(Default::default(), rem_size, {
            let element_state = self.clone();
            move |known_dimensions, _| {
                let Some(lines) = text_system
                    .shape_text(
                        &text,
                        font_size,
                        &runs[..],
                        known_dimensions.width, // Wrap if we know the width.
                    )
                    .log_err()
                else {
                    element_state.lock().replace(TextStateInner {
                        lines: Default::default(),
                        line_height,
                    });
                    return Size::default();
                };

                let mut size: Size<Pixels> = Size::default();
                for line in &lines {
                    let line_size = line.size(line_height);
                    size.height += line_size.height;
                    size.width = size.width.max(line_size.width);
                }

                element_state
                    .lock()
                    .replace(TextStateInner { lines, line_height });

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
}

struct InteractiveText {
    id: ElementId,
    text: StyledText,
}

struct InteractiveTextState {
    text_state: TextState,
    clicked_range_ixs: Rc<Cell<SmallVec<[usize; 1]>>>,
}

impl<V: 'static> Element<V> for InteractiveText {
    type State = InteractiveTextState;

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        if let Some(InteractiveTextState {
            text_state,
            clicked_range_ixs,
        }) = element_state
        {
            let (layout_id, text_state) = self.text.layout(view_state, Some(text_state), cx);
            let element_state = InteractiveTextState {
                text_state,
                clicked_range_ixs,
            };
            (layout_id, element_state)
        } else {
            let (layout_id, text_state) = self.text.layout(view_state, None, cx);
            let element_state = InteractiveTextState {
                text_state,
                clicked_range_ixs: Rc::default(),
            };
            (layout_id, element_state)
        }
    }

    fn paint(
        self,
        bounds: Bounds<Pixels>,
        view_state: &mut V,
        element_state: &mut Self::State,
        cx: &mut ViewContext<V>,
    ) {
        self.text
            .paint(bounds, view_state, &mut element_state.text_state, cx)
    }
}

impl<V: 'static> RenderOnce<V> for InteractiveText {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn render_once(self) -> Self::Element {
        self
    }
}
