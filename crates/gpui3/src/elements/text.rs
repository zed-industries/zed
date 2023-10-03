use crate::{
    AnyElement, Element, IntoAnyElement, Layout, LayoutId, Line, Pixels, Result, Size, ViewContext,
};
use parking_lot::Mutex;
use std::{marker::PhantomData, sync::Arc};
use util::{arc_cow::ArcCow, ResultExt};

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
    type FrameState = Arc<Mutex<Option<TextFrameState>>>;

    fn layout(
        &mut self,
        _view: &mut S,
        cx: &mut ViewContext<S>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        dbg!("layout text");

        let text_system = cx.text_system().clone();
        let text_style = cx.text_style();
        let font_size = text_style.font_size * cx.rem_size();
        let line_height = text_style
            .line_height
            .to_pixels(font_size.into(), cx.rem_size());
        let text = self.text.clone();
        let paint_state = Arc::new(Mutex::new(None));

        let rem_size = cx.rem_size();
        let layout_id = cx.request_measured_layout(Default::default(), rem_size, {
            let frame_state = paint_state.clone();
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

                frame_state.lock().replace(TextFrameState {
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
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<S>,
    ) -> Result<()> {
        let line;
        let line_height;
        {
            let frame_state = frame_state.lock();
            let frame_state = frame_state
                .as_ref()
                .expect("measurement has not been performed");
            line = frame_state.line.clone();
            line_height = frame_state.line_height;
        }

        // todo!("We haven't added visible bounds to the new element system yet, so this is a placeholder.");
        let visible_bounds = layout.bounds;
        line.paint(&layout, visible_bounds, line_height, cx)?;

        Ok(())
    }
}

pub struct TextFrameState {
    line: Arc<Line>,
    line_height: Pixels,
}
