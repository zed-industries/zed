/**
 * This WebView Element was demonstrated in a proof of concept by @vultix in this comment:
 * https://github.com/zed-industries/zed/issues/9778#issuecomment-2075713935
 */
use crate::{Bounds, Element, Pixels, Style, WindowContext};

use std::sync::Arc;

use wry::dpi::LogicalSize;
use wry::{dpi, Rect, WebView as WryWebView};

struct WebView {
    view: Arc<WryWebView>,
}
impl IntoElement for WebView {
    type Element = WebView;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for WebView {
    type RequestLayoutState = Style;
    type PrepaintState = ();

    fn before_layout(&mut self, cx: &mut WindowContext) -> (LayoutId, Self::BeforeLayout) {
        let mut style = Style::default();
        style.flex_grow = 1.0;
        style.size = Size::full();
        let id = cx.request_layout(&style, []);
        (id, ())
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut WindowContext,
    ) -> Self::AfterLayout {
        // TODO: Find better way of detecting view visibility
        if bounds.top() > cx.viewport_size().height || bounds.bottom() < Pixels::ZERO {
            self.view.set_visible(false).unwrap();
        } else {
            self.view.set_visible(true).unwrap();

            self.view
                .set_bounds(Rect {
                    size: dpi::Size::Logical(LogicalSize {
                        width: (bounds.size.width.0 - 50.0).into(),
                        height: (bounds.size.height.0 / 2.0).into(),
                    }),
                    position: dpi::Position::Logical(dpi::LogicalPosition::new(
                        bounds.origin.x.into(),
                        bounds.origin.y.into(),
                    )),
                })
                .unwrap();
        }
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        after_layout: &mut Self::AfterLayout,
        cx: &mut WindowContext,
    ) {
        // Do nothing?
    }
}
