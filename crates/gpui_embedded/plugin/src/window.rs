use crate::platform::PluginDisplay;
use crate::text_system::{PluginAtlas, PluginTextSystem, TileContent};
use crate::wit;
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Bounds, Capslock, DispatchEventResult, GpuSpecs, Modifiers, Pixels, PlatformAtlas,
    PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptButton,
    PromptLevel, RequestFrameOptions, ScaledPixels, Scene, Size, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, point,
};
use raw_window_handle as rwh;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Once};

type RequestFrameCallback = Box<dyn FnMut(RequestFrameOptions)>;
type InputCallback = Box<dyn FnMut(PlatformInput) -> DispatchEventResult>;
type ResizeCallback = Box<dyn FnMut(Size<Pixels>, f32)>;

#[derive(Default)]
struct Callbacks {
    request_frame: Option<RequestFrameCallback>,
    input: Option<InputCallback>,
    resize: Option<ResizeCallback>,
}

/// Shared state for one plugin "window" (a view slot in the host application).
pub struct PluginWindowState {
    view_id: u32,
    size: Cell<Size<Pixels>>,
    scale_factor: Cell<f32>,
    mouse_position: Cell<Point<Pixels>>,
    atlas: Arc<PluginAtlas>,
    callbacks: RefCell<Callbacks>,
    input_handler: RefCell<Option<PlatformInputHandler>>,
}

impl PluginWindowState {
    pub fn new(
        view_id: u32,
        size: Size<Pixels>,
        scale_factor: f32,
        text_system: Arc<PluginTextSystem>,
    ) -> Self {
        Self {
            view_id,
            size: Cell::new(size),
            scale_factor: Cell::new(scale_factor),
            mouse_position: Cell::new(Point::default()),
            atlas: Arc::new(PluginAtlas::new(text_system)),
            callbacks: RefCell::new(Callbacks::default()),
            input_handler: RefCell::new(None),
        }
    }

    /// Give GPUI a chance to redraw this window. GPUI's registered frame callback checks the
    /// window's dirty bit itself, so calling this on a clean window is cheap.
    ///
    /// The callback is temporarily moved out so that it can freely re-enter this window's
    /// other methods without hitting the `callbacks` RefCell.
    pub fn pump_frame(&self) {
        let callback = self.callbacks.borrow_mut().request_frame.take();
        if let Some(mut callback) = callback {
            callback(RequestFrameOptions {
                require_presentation: false,
                force_render: false,
            });
            self.callbacks.borrow_mut().request_frame = Some(callback);
        }
    }

    /// Dispatch a host-forwarded input event through GPUI's input pipeline.
    ///
    /// Unhandled printable key-downs fall through to the focused input handler, the same way
    /// GPUI's Linux backends synthesize text input from key events (there is no OS IME on
    /// this side of the wasm boundary).
    pub fn dispatch_input(&self, input: PlatformInput) {
        match &input {
            PlatformInput::MouseDown(event) => self.mouse_position.set(event.position),
            PlatformInput::MouseUp(event) => self.mouse_position.set(event.position),
            PlatformInput::MouseMove(event) => self.mouse_position.set(event.position),
            PlatformInput::ScrollWheel(event) => self.mouse_position.set(event.position),
            _ => {}
        }
        let callback = self.callbacks.borrow_mut().input.take();
        let Some(mut callback) = callback else {
            return;
        };
        let result = callback(input.clone());
        self.callbacks.borrow_mut().input = Some(callback);

        if let PlatformInput::KeyDown(event) = input
            && result.propagate
            && !result.default_prevented
            && event.keystroke.modifiers.is_subset_of(&Modifiers::shift())
            && let Some(key_char) = &event.keystroke.key_char
            && let Some(mut input_handler) = self.input_handler.take()
        {
            input_handler.replace_text_in_range(None, key_char);
            self.input_handler.replace(Some(input_handler));
        }
    }

    /// Apply a slot size or scale factor change coming from the host.
    pub fn resized(&self, size: Size<Pixels>, scale_factor: f32) {
        self.size.set(size);
        self.scale_factor.set(scale_factor);
        let callback = self.callbacks.borrow_mut().resize.take();
        if let Some(mut callback) = callback {
            callback(size, scale_factor);
            self.callbacks.borrow_mut().resize = Some(callback);
        }
    }
}

/// The `PlatformWindow` handed to GPUI. GPUI owns this box; the platform and the exports
/// reach the same state through the `Rc` kept in `PluginPlatform::windows`.
pub struct PluginWindow {
    state: Rc<PluginWindowState>,
    display: Rc<PluginDisplay>,
}

impl PluginWindow {
    pub fn new(state: Rc<PluginWindowState>, display: Rc<PluginDisplay>) -> Self {
        Self { state, display }
    }

    fn bounds_px(&self) -> Bounds<Pixels> {
        Bounds {
            origin: Point::default(),
            size: self.state.size.get(),
        }
    }
}

impl rwh::HasWindowHandle for PluginWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        // A synthetic handle: nothing consumes it, but the trait requires one.
        let raw = rwh::WebWindowHandle::new(self.state.view_id);
        Ok(unsafe { rwh::WindowHandle::borrow_raw(rwh::RawWindowHandle::Web(raw)) })
    }
}

impl rwh::HasDisplayHandle for PluginWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        let raw = rwh::WebDisplayHandle::new();
        Ok(unsafe { rwh::DisplayHandle::borrow_raw(rwh::RawDisplayHandle::Web(raw)) })
    }
}

impl PlatformWindow for PluginWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds_px()
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds_px())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.state.size.get()
    }

    fn resize(&mut self, _size: Size<Pixels>) {
        // The host owns the slot geometry; guest-initiated resizes are meaningless.
    }

    fn scale_factor(&self) -> f32 {
        self.state.scale_factor.get()
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.state.mouse_position.get()
    }

    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }

    fn capslock(&self) -> Capslock {
        Capslock::default()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.state.input_handler.replace(Some(input_handler));
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.state.input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {}

    fn is_active(&self) -> bool {
        true
    }

    fn is_hovered(&self) -> bool {
        true
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Transparent
    }

    fn set_title(&mut self, _title: &str) {}

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        false
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.state.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.state.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_hover_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.state.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, _callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_hit_test_window_control(
        &self,
        _callback: Box<dyn FnMut() -> Option<WindowControlArea>>,
    ) {
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn draw(&self, scene: &Scene) {
        let list = serialize_scene(scene, self.state.scale_factor.get(), &self.state.atlas);
        wit::update_scene(self.state.view_id, &list);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.state.atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}
}

fn warn_once(warned: &'static Once, message: &'static str) {
    warned.call_once(|| log::warn!("{message}"));
}

/// Convert a painted GPUI scene into the wire display list. Everything crossing the boundary
/// is in logical pixels (divided by the scale factor); glyph sprites are mapped back to the
/// symbolic parameters remembered by the atlas so the host can rasterize them itself.
fn serialize_scene(scene: &Scene, scale_factor: f32, atlas: &PluginAtlas) -> wit::DisplayList {
    static GRADIENT_WARNED: Once = Once::new();
    static SURFACE_WARNED: Once = Once::new();
    static SUBPIXEL_WARNED: Once = Once::new();
    static INSET_SHADOW_WARNED: Once = Once::new();
    static UNKNOWN_TILE_WARNED: Once = Once::new();
    static TRANSFORM_WARNED: Once = Once::new();

    let inverse_scale = 1.0 / scale_factor;
    let mut primitives = Vec::new();

    for quad in &scene.quads {
        let background = quad.background.as_solid().unwrap_or_else(|| {
            warn_once(
                &GRADIENT_WARNED,
                "gpui_plugin: gradient backgrounds are not supported; painting transparent",
            );
            gpui::transparent_black()
        });
        primitives.push(wit::PlacedPrimitive {
            order: quad.order,
            prim: wit::Primitive::Quad(wit::Quad {
                bounds: wire_bounds(quad.bounds, inverse_scale),
                content_mask: wire_bounds(quad.content_mask.bounds, inverse_scale),
                background: wire_hsla(background),
                border_color: wire_hsla(quad.border_color),
                corner_radii: wit::Corners {
                    top_left: quad.corner_radii.top_left.0 * inverse_scale,
                    top_right: quad.corner_radii.top_right.0 * inverse_scale,
                    bottom_right: quad.corner_radii.bottom_right.0 * inverse_scale,
                    bottom_left: quad.corner_radii.bottom_left.0 * inverse_scale,
                },
                border_widths: wit::Edges {
                    top: quad.border_widths.top.0 * inverse_scale,
                    right: quad.border_widths.right.0 * inverse_scale,
                    bottom: quad.border_widths.bottom.0 * inverse_scale,
                    left: quad.border_widths.left.0 * inverse_scale,
                },
                border_style: match quad.border_style {
                    gpui::BorderStyle::Solid => wit::BorderStyle::Solid,
                    gpui::BorderStyle::Dashed => wit::BorderStyle::Dashed,
                },
            }),
        });
    }

    for shadow in &scene.shadows {
        if shadow.inset != 0 {
            warn_once(
                &INSET_SHADOW_WARNED,
                "gpui_plugin: inset shadows are not supported; skipping",
            );
            continue;
        }
        let offset_x = (shadow.bounds.center().x.0 - shadow.element_bounds.center().x.0)
            * inverse_scale;
        let offset_y = (shadow.bounds.center().y.0 - shadow.element_bounds.center().y.0)
            * inverse_scale;
        let spread = ((shadow.bounds.size.width.0 - shadow.element_bounds.size.width.0) / 2.0
            - shadow.blur_radius.0)
            * inverse_scale;
        primitives.push(wit::PlacedPrimitive {
            order: shadow.order,
            prim: wit::Primitive::Shadow(wit::Shadow {
                bounds: wire_bounds(shadow.element_bounds, inverse_scale),
                content_mask: wire_bounds(shadow.content_mask.bounds, inverse_scale),
                corner_radii: wit::Corners {
                    top_left: shadow.element_corner_radii.top_left.0 * inverse_scale,
                    top_right: shadow.element_corner_radii.top_right.0 * inverse_scale,
                    bottom_right: shadow.element_corner_radii.bottom_right.0 * inverse_scale,
                    bottom_left: shadow.element_corner_radii.bottom_left.0 * inverse_scale,
                },
                color: wire_hsla(shadow.color),
                blur_radius: shadow.blur_radius.0 * inverse_scale,
                spread_radius: spread,
                offset: wit::Point {
                    x: offset_x,
                    y: offset_y,
                },
            }),
        });
    }

    for underline in &scene.underlines {
        primitives.push(wit::PlacedPrimitive {
            order: underline.order,
            prim: wit::Primitive::Underline(wit::Underline {
                origin: wire_point(underline.bounds.origin, inverse_scale),
                width: underline.bounds.size.width.0 * inverse_scale,
                content_mask: wire_bounds(underline.content_mask.bounds, inverse_scale),
                color: wire_hsla(underline.color),
                thickness: underline.thickness.0 * inverse_scale,
                wavy: underline.wavy != 0,
            }),
        });
    }

    for sprite in &scene.monochrome_sprites {
        if sprite.transformation != gpui::TransformationMatrix::unit() {
            warn_once(
                &TRANSFORM_WARNED,
                "gpui_plugin: sprite transformations are not supported; painting untransformed",
            );
        }
        match atlas.tile_content(sprite.tile.tile_id.0) {
            Some(TileContent::Glyph(params, raster_origin)) => {
                primitives.push(wit::PlacedPrimitive {
                    order: sprite.order,
                    prim: wit::Primitive::Glyph(wire_glyph(
                        &params,
                        raster_origin,
                        sprite.bounds,
                        sprite.content_mask.bounds,
                        sprite.color,
                        inverse_scale,
                    )),
                });
            }
            // A monochrome non-glyph sprite is a guest-rasterized SVG alpha mask: bake the
            // tint color in and ship it as an image.
            Some(TileContent::AlphaMask) => {
                if let Some(payload_id) =
                    atlas.tinted_payload(sprite.tile.tile_id.0, sprite.color)
                {
                    primitives.push(wit::PlacedPrimitive {
                        order: sprite.order,
                        prim: wit::Primitive::Image(wit::Image {
                            image_id: payload_id,
                            bounds: wire_bounds(sprite.bounds, inverse_scale),
                            content_mask: wire_bounds(sprite.content_mask.bounds, inverse_scale),
                            corner_radii: wit::Corners {
                                top_left: 0.0,
                                top_right: 0.0,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                            grayscale: false,
                            opacity: 1.0,
                        }),
                    });
                }
            }
            _ => warn_once(
                &UNKNOWN_TILE_WARNED,
                "gpui_plugin: sprite refers to an unknown atlas tile; skipping",
            ),
        }
    }

    for sprite in &scene.polychrome_sprites {
        match atlas.tile_content(sprite.tile.tile_id.0) {
            Some(TileContent::Glyph(params, raster_origin)) => {
                primitives.push(wit::PlacedPrimitive {
                    order: sprite.order,
                    prim: wit::Primitive::Glyph(wire_glyph(
                        &params,
                        raster_origin,
                        sprite.bounds,
                        sprite.content_mask.bounds,
                        gpui::white(),
                        inverse_scale,
                    )),
                });
            }
            Some(TileContent::Bitmap) => {
                if let Some(payload_id) = atlas.bitmap_payload(sprite.tile.tile_id.0) {
                    primitives.push(wit::PlacedPrimitive {
                        order: sprite.order,
                        prim: wit::Primitive::Image(wit::Image {
                            image_id: payload_id,
                            bounds: wire_bounds(sprite.bounds, inverse_scale),
                            content_mask: wire_bounds(sprite.content_mask.bounds, inverse_scale),
                            corner_radii: wit::Corners {
                                top_left: sprite.corner_radii.top_left.0 * inverse_scale,
                                top_right: sprite.corner_radii.top_right.0 * inverse_scale,
                                bottom_right: sprite.corner_radii.bottom_right.0 * inverse_scale,
                                bottom_left: sprite.corner_radii.bottom_left.0 * inverse_scale,
                            },
                            grayscale: sprite.grayscale,
                            opacity: sprite.opacity,
                        }),
                    });
                }
            }
            _ => warn_once(
                &UNKNOWN_TILE_WARNED,
                "gpui_plugin: sprite refers to an unknown atlas tile; skipping",
            ),
        }
    }

    for path in &scene.paths {
        let color = path.color.as_solid().unwrap_or_else(|| {
            warn_once(
                &GRADIENT_WARNED,
                "gpui_plugin: gradient backgrounds are not supported; painting transparent",
            );
            gpui::transparent_black()
        });
        primitives.push(wit::PlacedPrimitive {
            order: path.order,
            prim: wit::Primitive::Path(wit::Path {
                content_mask: wire_bounds(path.content_mask.bounds, inverse_scale),
                color: wire_hsla(color),
                vertices: path
                    .vertices
                    .iter()
                    .map(|vertex| wit::PathVertex {
                        xy: wire_point(vertex.xy_position, inverse_scale),
                        st: wit::Point {
                            x: vertex.st_position.x,
                            y: vertex.st_position.y,
                        },
                    })
                    .collect(),
            }),
        });
    }

    if !scene.surfaces.is_empty() {
        warn_once(
            &SURFACE_WARNED,
            "gpui_plugin: surface primitives are not supported; skipping",
        );
    }
    if !scene.subpixel_sprites.is_empty() {
        warn_once(
            &SUBPIXEL_WARNED,
            "gpui_plugin: subpixel sprites are not supported; skipping",
        );
    }

    wit::DisplayList {
        primitives,
        new_images: atlas.take_pending_payloads(),
    }
}

/// Reconstruct the symbolic glyph for a fabricated atlas tile. The baseline origin is the
/// sprite origin minus the raster-bounds offset that `Window::paint_glyph` added, plus the
/// subpixel variant's fractional offset, so the host re-derives the same variant and the
/// glyph lands where native rendering would put it.
fn wire_glyph(
    params: &gpui::RenderGlyphParams,
    raster_origin: gpui::Point<gpui::DevicePixels>,
    bounds: Bounds<ScaledPixels>,
    content_mask: Bounds<ScaledPixels>,
    color: gpui::Hsla,
    inverse_scale: f32,
) -> wit::Glyph {
    let baseline = point(
        (bounds.origin.x.0 - raster_origin.x.0 as f32
            + params.subpixel_variant.x as f32 / gpui::SUBPIXEL_VARIANTS_X as f32)
            * inverse_scale,
        (bounds.origin.y.0 - raster_origin.y.0 as f32
            + params.subpixel_variant.y as f32 / gpui::SUBPIXEL_VARIANTS_Y as f32)
            * inverse_scale,
    );
    wit::Glyph {
        font_id: params.font_id.0 as u32,
        glyph_id: params.glyph_id.0,
        origin: wit::Point {
            x: baseline.x,
            y: baseline.y,
        },
        font_size: f32::from(params.font_size),
        color: wire_hsla(color),
        content_mask: wire_bounds(content_mask, inverse_scale),
        is_emoji: params.is_emoji,
    }
}

fn wire_point(value: Point<ScaledPixels>, inverse_scale: f32) -> wit::Point {
    wit::Point {
        x: value.x.0 * inverse_scale,
        y: value.y.0 * inverse_scale,
    }
}

fn wire_bounds(value: Bounds<ScaledPixels>, inverse_scale: f32) -> wit::Bounds {
    wit::Bounds {
        origin: wire_point(value.origin, inverse_scale),
        size: wit::Extent {
            width: value.size.width.0 * inverse_scale,
            height: value.size.height.0 * inverse_scale,
        },
    }
}

fn wire_hsla(color: gpui::Hsla) -> wit::Hsla {
    wit::Hsla {
        h: color.h,
        s: color.s,
        l: color.l,
        a: color.a,
    }
}
