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
    held_layout: Option<HeldLayout>,
}

/// The layout viewport as it was before a software keyboard could open.
///
/// Browsers disagree on what a keyboard resizes: Safari shrinks only the
/// visual viewport, while Chrome on iOS and `interactive-widget=
/// resizes-content` shrink the layout viewport with it, and a canvas sized
/// to the page along with that. Holding the canvas at its keyboard-free
/// height makes every browser report the keyboard the same way: as the part
/// of the canvas the visual viewport no longer shows.
///
/// The page gives no other signal, so anything else that shortens the layout
/// at the same width while held, such as entering split-screen, also reads as
/// a keyboard until the hold ends.
struct HeldLayout {
    width: f64,
    height: f64,
    /// The inline canvas height to restore on release.
    canvas_height: String,
    /// When set, the hold ends once the layout is back at its held height,
    /// or at this `Date.now()` time if it never gets there.
    release_deadline: Option<f64>,
}

/// How long a released hold waits for the closing keyboard to give the
/// layout its height back. Only reached when something other than the
/// keyboard shortened the layout, which would otherwise hold forever.
const KEYBOARD_CLOSE_TIMEOUT_MS: f64 = 1000.;

impl WebViewport {
    pub(crate) fn new(document: &web_sys::Document) -> anyhow::Result<Self> {
        let [top, right, bottom, left] =
            ["top", "right", "bottom", "left"].map(|edge| safe_area_probe(document, edge));
        let viewport = Self {
            safe_area_probes: [top?, right?, bottom?, left?],
            visible_bounds: Bounds::default(),
            insets: WindowInsets::default(),
            held_layout: None,
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
        let document = window
            .document()
            .ok_or_else(|| anyhow::anyhow!("Missing document"))?;
        let root = document
            .document_element()
            .ok_or_else(|| anyhow::anyhow!("Missing root"))?;
        let layout_width = root.client_width() as f64;
        let mut layout_height = root.client_height() as f64;
        if let Some(held) = &self.held_layout {
            // A keyboard only ever makes the layout shorter at the same
            // width. Anything else is a rotation or a window resize, or a
            // hold taken while a keyboard was still up: measure again.
            let releasing = held.release_deadline.is_some();
            if held.width != layout_width || layout_height > held.height {
                self.restore_canvas_height(canvas);
                if !releasing {
                    self.hold_canvas_height(window, canvas);
                }
            } else if held.release_deadline.is_some_and(|deadline| {
                layout_height >= held.height || js_sys::Date::now() >= deadline
            }) {
                self.restore_canvas_height(canvas);
            } else {
                layout_height = held.height;
            }
        }
        let canvas_bounds = canvas.get_bounding_client_rect();
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

    /// Keeps the canvas at its current height while a software keyboard may
    /// be open, until [`Self::release_canvas_height`].
    pub(crate) fn hold_canvas_height(
        &mut self,
        window: &web_sys::Window,
        canvas: &web_sys::HtmlCanvasElement,
    ) {
        if let Some(held) = &mut self.held_layout {
            match held.release_deadline.take() {
                Some(deadline) if js_sys::Date::now() >= deadline => {
                    self.restore_canvas_height(canvas);
                }
                // Still held, or the keyboard is still closing: the held
                // height predates it, where a fresh measurement would not.
                _ => return,
            }
        }
        let Some(root) = window
            .document()
            .and_then(|document| document.document_element())
        else {
            return;
        };
        let style = canvas.style();
        let canvas_height = style.get_property_value("height").unwrap_or_default();
        let height = canvas.get_bounding_client_rect().height();
        if let Err(error) = style.set_property("height", &format!("{height}px")) {
            log::warn!("Failed to hold the canvas height: {error:?}");
            return;
        }
        self.held_layout = Some(HeldLayout {
            width: root.client_width() as f64,
            height: root.client_height() as f64,
            canvas_height,
            release_deadline: None,
        });
    }

    /// Ends the hold once the keyboard has closed.
    ///
    /// Releasing at once would resize the canvas to the page a still-closing
    /// keyboard has shrunk, and a hold retaken before it finished closing
    /// would measure that page as keyboard-free.
    pub(crate) fn release_canvas_height(&mut self) {
        if let Some(held) = &mut self.held_layout {
            held.release_deadline
                .get_or_insert_with(|| js_sys::Date::now() + KEYBOARD_CLOSE_TIMEOUT_MS);
        }
    }

    fn restore_canvas_height(&mut self, canvas: &web_sys::HtmlCanvasElement) {
        let Some(held) = self.held_layout.take() else {
            return;
        };
        if let Err(error) = canvas.style().set_property("height", &held.canvas_height) {
            log::warn!("Failed to release the canvas height: {error:?}");
        }
    }

    /// Sets the canvas's own height, which a hold in progress restores to.
    pub(crate) fn set_canvas_height(&mut self, canvas: &web_sys::HtmlCanvasElement, height: &str) {
        match &mut self.held_layout {
            Some(held) => held.canvas_height = height.to_string(),
            None => {
                canvas.style().set_property("height", height).ok();
            }
        }
    }

    /// The height a docked software keyboard takes from the page.
    ///
    /// Floating and split keyboards overlay the page and read as zero.
    /// Returns `None` when a pinch zoom makes the visual viewport say nothing
    /// about a keyboard.
    pub(crate) fn keyboard_height(&self, window: &web_sys::Window) -> Option<f64> {
        let viewport = window.visual_viewport()?;
        if (viewport.scale() - 1.).abs() >= 0.001 {
            return None;
        }
        let layout_height = match &self.held_layout {
            Some(held) => held.height,
            None => window.document()?.document_element()?.client_height() as f64,
        };
        Some((layout_height - viewport.height()).max(0.))
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
    let height = format!("env(safe-area-inset-{edge},0px)");
    // A CSP that blocks inline styles still allows per-property CSSOM writes.
    let style = probe.style();
    for (property, value) in [
        ("position", "fixed"),
        ("left", "0"),
        ("top", "0"),
        ("width", "0"),
        ("padding", "0"),
        ("border", "0"),
        ("visibility", "hidden"),
        ("pointer-events", "none"),
        ("height", height.as_str()),
    ] {
        style.set_property(property, value).map_err(|error| {
            anyhow::anyhow!("Failed to set safe-area probe {property} style: {error:?}")
        })?;
    }
    Ok(probe)
}
