use crate::{
    AnyElement, Element, IntoAnyElement, Layout, LayoutId, Line, LineLayout, Pixels, Result, Size,
    ViewContext,
};
use parking_lot::Mutex;
use std::{marker::PhantomData, sync::Arc};
use util::arc_cow::ArcCow;

impl<S: 'static> IntoAnyElement<S> for ArcCow<'static, str> {
    fn into_any(self) -> AnyElement<S> {
        Text {
            text: self,
            state_type: PhantomData,
        }
        .into_any()
    }
}

impl<V: 'static> IntoAnyElement<V> for &'static str {
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

impl<S: 'static> Element for Text<S> {
    type State = S;
    type FrameState = Arc<Mutex<Option<TextLayout>>>;

    fn layout(
        &mut self,
        _view: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        let text_system = cx.text_system().clone();
        let text_style = cx.text_style();
        let line_height = cx.text_system().line_height(text_style.font_size);
        let text = self.text.clone();
        let paint_state = Arc::new(Mutex::new(None));

        let layout_id = cx.request_measured_layout(Default::default(), cx.rem_size(), {
            let frame_state = paint_state.clone();
            move |_, _| {
                let line_layout = text_system.layout_str(
                    text.as_ref(),
                    text_style.font_size,
                    &[(text.len(), text_style.to_run())],
                );

                let size = Size {
                    width: line_layout.width(),
                    height: line_height,
                };

                frame_state.lock().replace(TextLayout {
                    line: Arc::new(line_layout),
                    line_height,
                });

                size
            }
        });

        Ok((layout_id?, paint_state))
    }

    fn paint<'a>(
        &mut self,
        layout: Layout,
        _: &mut Self::State,
        paint_state: &mut Self::FrameState,
        cx: &mut ViewContext<S>,
    ) {
        let bounds = layout.bounds;

        let line;
        let line_height;
        {
            let paint_state = paint_state.lock();
            let paint_state = paint_state
                .as_ref()
                .expect("measurement has not been performed");
            line = paint_state.line.clone();
            line_height = paint_state.line_height;
        }

        let text_style = cx.text_style();

        // todo!("We haven't added visible bounds to the new element system yet, so this is a placeholder.");
        let visible_bounds = bounds;
        line.paint(bounds.origin, visible_bounds, line_height, cx.legacy_cx);
    }
}

pub struct TextLayout {
    line: Arc<Line>,
    line_height: Pixels,
}
