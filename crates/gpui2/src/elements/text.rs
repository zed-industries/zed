use crate::{
    AnyElement, BorrowWindow, Bounds, Component, Element, LayoutId, Line, Pixels, SharedString,
    Size, TextRun, ViewContext,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{marker::PhantomData, sync::Arc};
use util::ResultExt;

impl<V: 'static> Component<V> for SharedString {
    fn render(self) -> AnyElement<V> {
        Text {
            text: self,
            runs: None,
            state_type: PhantomData,
        }
        .render()
    }
}

impl<V: 'static> Component<V> for &'static str {
    fn render(self) -> AnyElement<V> {
        Text {
            text: self.into(),
            runs: None,
            state_type: PhantomData,
        }
        .render()
    }
}

// TODO: Figure out how to pass `String` to `child` without this.
// This impl doesn't exist in the `gpui2` crate.
impl<V: 'static> Component<V> for String {
    fn render(self) -> AnyElement<V> {
        Text {
            text: self.into(),
            runs: None,
            state_type: PhantomData,
        }
        .render()
    }
}

pub struct Text<V> {
    text: SharedString,
    runs: Option<Vec<TextRun>>,
    state_type: PhantomData<V>,
}

impl<V: 'static> Text<V> {
    /// styled renders text that has different runs of different styles.
    /// callers are responsible for setting the correct style for each run.
    ////
    /// For uniform text you can usually just pass a string as a child, and
    /// cx.text_style() will be used automatically.
    pub fn styled(text: SharedString, runs: Vec<TextRun>) -> Self {
        Text {
            text,
            runs: Some(runs),
            state_type: Default::default(),
        }
    }
}

impl<V: 'static> Component<V> for Text<V> {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: 'static> Element<V> for Text<V> {
    type ElementState = Arc<Mutex<Option<TextElementState>>>;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn initialize(
        &mut self,
        _view_state: &mut V,
        element_state: Option<Self::ElementState>,
        _cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        element_state.unwrap_or_default()
    }

    fn layout(
        &mut self,
        _view: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
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
            move |known_dimensions, _| {
                let Some(lines) = text_system
                    .layout_text(
                        &text,
                        font_size,
                        &runs[..],
                        known_dimensions.width, // Wrap if we know the width.
                    )
                    .log_err()
                else {
                    element_state.lock().replace(TextElementState {
                        lines: Default::default(),
                        line_height,
                    });
                    return Size::default();
                };

                let line_count = lines
                    .iter()
                    .map(|line| line.wrap_count() + 1)
                    .sum::<usize>();
                let size = Size {
                    width: lines
                        .iter()
                        .map(|line| line.layout.width)
                        .max()
                        .unwrap()
                        .ceil(),
                    height: line_height * line_count,
                };

                element_state
                    .lock()
                    .replace(TextElementState { lines, line_height });

                size
            }
        });

        layout_id
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        let element_state = element_state.lock();
        let element_state = element_state
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("measurement has not been performed on {}", &self.text))
            .unwrap();

        let line_height = element_state.line_height;
        let mut line_origin = bounds.origin;
        for line in &element_state.lines {
            line.paint(line_origin, line_height, cx).log_err();
            line_origin.y += line.size(line_height).height;
        }
    }
}

pub struct TextElementState {
    lines: SmallVec<[Line; 1]>,
    line_height: Pixels,
}
