use crate::{
    AnyElement, Bounds, Element, IntoAnyElement, LayoutId, Line, Pixels, Size, ViewContext,
};
use parking_lot::Mutex;
use std::{marker::PhantomData, sync::Arc};
use util::{arc_cow::ArcCow, ResultExt};

impl<S: 'static + Send + Sync> IntoAnyElement<S> for ArcCow<'static, str> {
    fn into_any(self) -> AnyElement<S> {
        Text {
            text: self,
            state_type: PhantomData,
        }
        .into_any()
    }
}

impl<V: 'static + Send + Sync> IntoAnyElement<V> for &'static str {
    fn into_any(self) -> AnyElement<V> {
        Text {
            text: ArcCow::from(self),
            state_type: PhantomData,
        }
        .into_any()
    }
}

pub struct Text<S> {
    text: ArcCow<'static, str>,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> Element for Text<S> {
    type ViewState = S;
    type ElementState = Arc<Mutex<Option<TextElementState>>>;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn layout(
        &mut self,
        _view: &mut S,
        _element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<S>,
    ) -> (LayoutId, Self::ElementState) {
        let text_system = cx.text_system().clone();
        let text_style = cx.text_style();
        let font_size = text_style.font_size * cx.rem_size();
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), cx.rem_size());
        let text = self.text.clone();
        let element_state = Arc::new(Mutex::new(None));

        let rem_size = cx.rem_size();
        let layout_id = cx.request_measured_layout(Default::default(), rem_size, {
            let element_state = element_state.clone();
            move |_, _| {
                let Some(line_layout) = text_system
                    .layout_line(
                        text.as_ref(),
                        font_size,
                        &[(text.len(), text_style.to_run())],
                    )
                    .log_err()
                else {
                    return Size::default();
                };

                let size = Size {
                    width: line_layout.width(),
                    height: line_height,
                };

                element_state.lock().replace(TextElementState {
                    line: Arc::new(line_layout),
                    line_height,
                });

                size
            }
        });

        (layout_id, element_state)
    }

    fn paint<'a>(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<S>,
    ) {
        let line;
        let line_height;
        {
            let element_state = element_state.lock();
            let element_state = element_state
                .as_ref()
                .expect("measurement has not been performed");
            line = element_state.line.clone();
            line_height = element_state.line_height;
        }

        line.paint(bounds, bounds, line_height, cx).log_err();
    }
}

pub struct TextElementState {
    line: Arc<Line>,
    line_height: Pixels,
}
