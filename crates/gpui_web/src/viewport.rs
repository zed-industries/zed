//! Browser viewport and safe-area frame snapshots in canvas coordinates.
//!
//! Browser geometry is sampled before rendering, never by geometry getters.
//! Safe-area padding is resolved by CSS so display cutouts use the browser's
//! own environment values rather than device-specific assumptions.

use gpui::{Bounds, Edges, Pixels, WindowInsets, point, px, size};
use wasm_bindgen::JsCast;

pub(crate) struct WebViewport {
    safe_area_probe: web_sys::HtmlElement,
    pub(crate) visible_bounds: Bounds<Pixels>,
    pub(crate) insets: WindowInsets,
}

impl WebViewport {
    pub(crate) fn new(document: &web_sys::Document) -> anyhow::Result<Self> {
        let probe = document
            .create_element("div")
            .map_err(|error| anyhow::anyhow!("Failed to create safe-area probe: {error:?}"))?
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|error| anyhow::anyhow!("Invalid safe-area probe element: {error:?}"))?;
        probe
            .set_attribute(
                "style",
                "position:fixed;left:0;top:0;width:0;height:0;visibility:hidden;\
                 pointer-events:none;padding-top:env(safe-area-inset-top,0px);\
                 padding-right:env(safe-area-inset-right,0px);\
                 padding-bottom:env(safe-area-inset-bottom,0px);\
                 padding-left:env(safe-area-inset-left,0px)",
            )
            .map_err(|error| anyhow::anyhow!("Failed to style safe-area probe: {error:?}"))?;
        document
            .body()
            .ok_or_else(|| anyhow::anyhow!("Missing document body"))?
            .append_child(&probe)
            .map_err(|error| anyhow::anyhow!("Failed to attach safe-area probe: {error:?}"))?;
        Ok(Self {
            safe_area_probe: probe,
            visible_bounds: Bounds::default(),
            insets: WindowInsets::default(),
        })
    }

    pub(crate) fn update(
        &mut self,
        window: &web_sys::Window,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> anyhow::Result<(bool, bool)> {
        let canvas_bounds = canvas.get_bounding_client_rect();
        let document = window
            .document()
            .ok_or_else(|| anyhow::anyhow!("Missing document"))?;
        let root = document
            .document_element()
            .ok_or_else(|| anyhow::anyhow!("Missing root"))?;
        let layout_width = root.client_width() as f64;
        let layout_height = root.client_height() as f64;
        let (left, top, width, height) = match window.visual_viewport() {
            Some(viewport) => (
                viewport.offset_left(),
                viewport.offset_top(),
                viewport.width(),
                viewport.height(),
            ),
            None => (0., 0., layout_width, layout_height),
        };
        // VisualViewport offsets and getBoundingClientRect share layout CSS
        // coordinates. Multiplying by the pinch scale here would incorrectly
        // expand the visible rectangle back to its unzoomed size.
        let x = (left - canvas_bounds.left()).clamp(0., canvas_bounds.width());
        let y = (top - canvas_bounds.top()).clamp(0., canvas_bounds.height());
        let right = (left + width - canvas_bounds.left()).clamp(x, canvas_bounds.width());
        let bottom = (top + height - canvas_bounds.top()).clamp(y, canvas_bounds.height());
        let visible_bounds = Bounds::new(
            point(px(x as f32), px(y as f32)),
            size(px((right - x) as f32), px((bottom - y) as f32)),
        );

        let style = window
            .get_computed_style(&self.safe_area_probe)
            .map_err(|error| anyhow::anyhow!("Failed to measure safe area: {error:?}"))?
            .ok_or_else(|| anyhow::anyhow!("Missing safe-area style"))?;
        let padding = |property| -> anyhow::Result<f64> {
            let value = style
                .get_property_value(property)
                .map_err(|error| anyhow::anyhow!("Failed to read {property}: {error:?}"))?;
            Ok(value.trim_end_matches("px").parse::<f64>()?)
        };
        let safe_area = Edges {
            top: px((padding("padding-top")? - canvas_bounds.top()).max(0.) as f32),
            right: px(
                (canvas_bounds.right() - layout_width + padding("padding-right")?).max(0.) as f32,
            ),
            bottom: px(
                (canvas_bounds.bottom() - layout_height + padding("padding-bottom")?).max(0.)
                    as f32,
            ),
            left: px((padding("padding-left")? - canvas_bounds.left()).max(0.) as f32),
        };
        let insets = WindowInsets {
            safe_area,
            ..WindowInsets::default()
        };
        let changed = (self.visible_bounds != visible_bounds, self.insets != insets);
        self.visible_bounds = visible_bounds;
        self.insets = insets;
        Ok(changed)
    }
}

impl Drop for WebViewport {
    fn drop(&mut self) {
        self.safe_area_probe.remove();
    }
}
