//! Browser viewport and safe-area frame snapshots in canvas coordinates.
//!
//! Browser geometry is sampled before rendering, never by geometry getters.
//! Safe-area padding is resolved by CSS so display cutouts use the browser's
//! own environment values rather than device-specific assumptions.

use gpui::{Bounds, Edges, Pixels, WindowInsets, point, px, size};
use wasm_bindgen::JsCast;

pub(crate) struct WebViewport {
    safe_area_probes: [web_sys::HtmlElement; 4],
    pub(crate) visible_bounds: Bounds<Pixels>,
    pub(crate) insets: WindowInsets,
}

impl WebViewport {
    pub(crate) fn new(document: &web_sys::Document) -> anyhow::Result<Self> {
        let [top, right, bottom, left] =
            ["top", "right", "bottom", "left"].map(|edge| safe_area_probe(document, edge));
        let viewport = Self {
            safe_area_probes: [top?, right?, bottom?, left?],
            visible_bounds: Bounds::default(),
            insets: WindowInsets::default(),
        };
        let body = document
            .body()
            .ok_or_else(|| anyhow::anyhow!("Missing document body"))?;
        for probe in &viewport.safe_area_probes {
            body.append_child(probe)
                .map_err(|error| anyhow::anyhow!("Failed to attach safe-area probe: {error:?}"))?;
        }
        Ok(viewport)
    }

    pub(crate) fn observe_safe_area(&self, observer: &web_sys::ResizeObserver) {
        // Separate boxes detect redistribution between edges even when the
        // total inset, and therefore a single combined probe's size, is unchanged.
        for probe in &self.safe_area_probes {
            observer.observe(probe);
        }
    }

    pub(crate) fn update(
        &mut self,
        window: &web_sys::Window,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> anyhow::Result<bool> {
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

        let [top, right, bottom, left] = self
            .safe_area_probes
            .each_ref()
            .map(|probe| probe.get_bounding_client_rect().height());
        let safe_area = Edges {
            top: px((top - canvas_bounds.top()).max(0.) as f32),
            right: px((canvas_bounds.right() - layout_width + right).max(0.) as f32),
            bottom: px((canvas_bounds.bottom() - layout_height + bottom).max(0.) as f32),
            left: px((left - canvas_bounds.left()).max(0.) as f32),
        };
        let insets = WindowInsets {
            safe_area,
            ..WindowInsets::default()
        };
        let changed = self.visible_bounds != visible_bounds || self.insets != insets;
        self.visible_bounds = visible_bounds;
        self.insets = insets;
        Ok(changed)
    }
}

impl Drop for WebViewport {
    fn drop(&mut self) {
        for probe in &self.safe_area_probes {
            probe.remove();
        }
    }
}

fn safe_area_probe(
    document: &web_sys::Document,
    edge: &str,
) -> anyhow::Result<web_sys::HtmlElement> {
    let probe = document
        .create_element("div")
        .map_err(|error| anyhow::anyhow!("Failed to create safe-area probe: {error:?}"))?
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|error| anyhow::anyhow!("Invalid safe-area probe element: {error:?}"))?;
    probe
        .set_attribute(
            "style",
            &format!(
                "position:fixed;left:0;top:0;width:0;padding:0;border:0;visibility:hidden;\
                 pointer-events:none;height:env(safe-area-inset-{edge},0px)"
            ),
        )
        .map_err(|error| anyhow::anyhow!("Failed to style safe-area probe: {error:?}"))?;
    Ok(probe)
}
