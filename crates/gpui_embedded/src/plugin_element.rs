//! The GPUI view that caches and replays a guest's retained display list, and forwards mouse
//! input back to the guest. See DESIGN.md invariants 1, 5, 6, and 7.

use gpui::{
    App, Bounds, BoxShadow, ContentMask, Context, Corners, Edges, FocusHandle, IntoElement,
    KeyDownEvent, KeyUpEvent, Keystroke, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PaintQuad, Pixels, Point, Render, ScrollDelta, ScrollWheelEvent, Size,
    UnderlineStyle, WeakEntity, Window, canvas, div, point, prelude::*, px,
};

use crate::bindings;
use crate::{PluginHost, PluginImages};

/// Host-side state for a single plugin view. The display list is a retained copy of the
/// guest's most recent scene; the host replays it cheaply every frame without calling into
/// the guest (DESIGN.md invariant 1).
pub struct PluginViewState {
    view_id: u32,
    display_list: Option<bindings::DisplayList>,
    cursor: Option<gpui::CursorStyle>,
    last_size: Size<Pixels>,
    last_origin: Point<Pixels>,
    host: WeakEntity<PluginHost>,
    images: PluginImages,
    focus_handle: FocusHandle,
}

impl PluginViewState {
    pub fn new(
        view_id: u32,
        size: Size<Pixels>,
        host: WeakEntity<PluginHost>,
        images: PluginImages,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            view_id,
            display_list: None,
            cursor: None,
            last_size: size,
            last_origin: Point::default(),
            host,
            images,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_display_list(&mut self, list: bindings::DisplayList) {
        self.display_list = Some(list);
    }

    pub fn set_cursor(&mut self, cursor: gpui::CursorStyle) {
        self.cursor = Some(cursor);
    }

    /// Translate and forward a mouse button press or release to the guest.
    fn emit_button(
        &self,
        pressed: bool,
        button: MouseButton,
        position: Point<Pixels>,
        modifiers: gpui::Modifiers,
        click_count: usize,
        cx: &mut Context<Self>,
    ) {
        let Some(button) = wire_button(button) else {
            return;
        };
        let button_event = bindings::MouseButtonEvent {
            button,
            position: wire_point(position - self.last_origin),
            modifiers: wire_modifiers(modifiers),
            click_count: click_count as u32,
        };
        let event = if pressed {
            bindings::MouseEvent::Down(button_event)
        } else {
            bindings::MouseEvent::Up(button_event)
        };
        self.forward_mouse(event, cx);
    }

    /// Forward a mouse event to the guest. Deferred so it never re-enters wasm while this
    /// view is mid-update (the event listener runs inside the view's own lease), and so the
    /// resulting scene update can safely update this view (DESIGN.md invariants 3 and 7).
    fn forward_mouse(&self, event: bindings::MouseEvent, cx: &mut Context<Self>) {
        let view_id = self.view_id;
        let host = self.host.clone();
        cx.defer(move |cx| {
            if let Some(host) = host.upgrade() {
                host.update(cx, |host, cx| host.handle_mouse(view_id, event, cx));
            }
        });
    }

    /// Forward a keyboard event to the guest; same deferral rules as `forward_mouse`.
    fn forward_key(&self, event: bindings::KeyEvent, cx: &mut Context<Self>) {
        let view_id = self.view_id;
        let host = self.host.clone();
        cx.defer(move |cx| {
            if let Some(host) = host.upgrade() {
                host.update(cx, |host, cx| host.handle_key(view_id, event, cx));
            }
        });
    }

    /// Notify the guest that this view's slot changed size. Deferred so it does not call into
    /// wasm during the host's draw (DESIGN.md invariant 1).
    fn request_resize(&self, size: Size<Pixels>, scale: f32, cx: &mut Context<Self>) {
        let view_id = self.view_id;
        let host = self.host.clone();
        cx.defer(move |cx| {
            if let Some(host) = host.upgrade() {
                host.update(cx, |host, cx| host.resize_view(view_id, size, scale, cx));
            }
        });
    }
}

impl Render for PluginViewState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let prepaint_entity = cx.entity();
        let paint_entity = cx.entity();

        div()
            .size_full()
            .id(("plugin-view", self.view_id))
            .track_focus(&self.focus_handle)
            .when_some(self.cursor, |this, cursor| this.cursor(cursor))
            .on_any_mouse_down(cx.listener(|this, event: &MouseDownEvent, window, cx| {
                window.focus(&this.focus_handle, cx);
                this.emit_button(
                    true,
                    event.button,
                    event.position,
                    event.modifiers,
                    event.click_count,
                    cx,
                );
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                this.forward_key(
                    bindings::KeyEvent::Down(bindings::KeyDownEvent {
                        keystroke: wire_keystroke(&event.keystroke),
                        is_held: event.is_held,
                    }),
                    cx,
                );
            }))
            .on_key_up(cx.listener(|this, event: &KeyUpEvent, _window, cx| {
                this.forward_key(
                    bindings::KeyEvent::Up(bindings::KeyUpEvent {
                        keystroke: wire_keystroke(&event.keystroke),
                    }),
                    cx,
                );
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event: &MouseUpEvent, _window, cx| {
                    this.emit_button(false, event.button, event.position, event.modifiers, event.click_count, cx);
                }),
            )
            .on_mouse_up(
                MouseButton::Right,
                cx.listener(|this, event: &MouseUpEvent, _window, cx| {
                    this.emit_button(false, event.button, event.position, event.modifiers, event.click_count, cx);
                }),
            )
            .on_mouse_up(
                MouseButton::Middle,
                cx.listener(|this, event: &MouseUpEvent, _window, cx| {
                    this.emit_button(false, event.button, event.position, event.modifiers, event.click_count, cx);
                }),
            )
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                let position = wire_point(event.position - this.last_origin);
                this.forward_mouse(
                    bindings::MouseEvent::Move(bindings::MouseMoveEvent {
                        position,
                        pressed_button: event.pressed_button.and_then(wire_button),
                        modifiers: wire_modifiers(event.modifiers),
                    }),
                    cx,
                );
            }))
            .on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, _window, cx| {
                let position = wire_point(event.position - this.last_origin);
                let (delta_x, delta_y, precise) = match event.delta {
                    ScrollDelta::Pixels(delta) => (f32::from(delta.x), f32::from(delta.y), true),
                    ScrollDelta::Lines(delta) => (delta.x, delta.y, false),
                };
                this.forward_mouse(
                    bindings::MouseEvent::Scroll(bindings::ScrollWheelEvent {
                        position,
                        delta_x,
                        delta_y,
                        precise,
                        modifiers: wire_modifiers(event.modifiers),
                    }),
                    cx,
                );
            }))
            .child(
                canvas(
                    move |bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App| {
                        let scale = window.scale_factor();
                        prepaint_entity.update(cx, |this, cx| {
                            this.last_origin = bounds.origin;
                            if bounds.size != this.last_size {
                                this.last_size = bounds.size;
                                this.request_resize(bounds.size, scale, cx);
                            }
                        });
                        bounds
                    },
                    move |bounds: Bounds<Pixels>, _: Bounds<Pixels>, window: &mut Window, cx: &mut App| {
                        let view = paint_entity.read(cx);
                        if let Some(list) = view.display_list.as_ref() {
                            let images = view.images.borrow();
                            replay(list, bounds, &images, window);
                        }
                    },
                )
                .size_full(),
            )
    }
}

/// Replay a guest display list into the host window. Coordinates on the wire are logical
/// pixels relative to the view's slot; the host adds the slot origin and paints through the
/// public `Window::paint_*` APIs (DESIGN.md invariant 5). Primitives are grouped by ascending
/// `order` and each group is painted inside its own `paint_layer` so guest stacking is
/// preserved (invariant 6).
fn replay(
    list: &bindings::DisplayList,
    slot: Bounds<Pixels>,
    images: &std::collections::HashMap<u64, std::sync::Arc<gpui::RenderImage>>,
    window: &mut Window,
) {
    let mut indices: Vec<usize> = (0..list.primitives.len()).collect();
    indices.sort_by_key(|&index| list.primitives[index].order);

    let mut cursor = 0;
    while cursor < indices.len() {
        let order = list.primitives[indices[cursor]].order;
        let mut end = cursor + 1;
        while end < indices.len() && list.primitives[indices[end]].order == order {
            end += 1;
        }
        let layer = &indices[cursor..end];
        window.paint_layer(slot, |window| {
            for &index in layer {
                paint_primitive(&list.primitives[index].prim, slot, images, window);
            }
        });
        cursor = end;
    }
}

fn paint_primitive(
    primitive: &bindings::Primitive,
    slot: Bounds<Pixels>,
    images: &std::collections::HashMap<u64, std::sync::Arc<gpui::RenderImage>>,
    window: &mut Window,
) {
    match primitive {
        bindings::Primitive::Quad(quad) => {
            let bounds = to_bounds(&quad.bounds, slot.origin);
            let mask = to_bounds(&quad.content_mask, slot.origin).intersect(&slot);
            window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                window.paint_quad(PaintQuad {
                    bounds,
                    corner_radii: to_corners(&quad.corner_radii),
                    background: to_hsla(&quad.background).into(),
                    border_widths: to_edges(&quad.border_widths),
                    border_color: to_hsla(&quad.border_color),
                    border_style: to_border_style(quad.border_style),
                });
            });
        }
        bindings::Primitive::Shadow(shadow) => {
            let bounds = to_bounds(&shadow.bounds, slot.origin);
            let mask = to_bounds(&shadow.content_mask, slot.origin).intersect(&slot);
            let corner_radii = to_corners(&shadow.corner_radii);
            let box_shadow = BoxShadow {
                color: to_hsla(&shadow.color),
                offset: point(px(shadow.offset.x), px(shadow.offset.y)),
                blur_radius: px(shadow.blur_radius),
                spread_radius: px(shadow.spread_radius),
                inset: false,
            };
            window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                // HOST-INTEGRATION: gpui's public API is `paint_drop_shadows`, not the
                // `paint_shadows` named in the task; drop shadows are the wire's only kind.
                window.paint_drop_shadows(bounds, corner_radii, &[box_shadow]);
            });
        }
        bindings::Primitive::Underline(underline) => {
            let origin = to_point(&underline.origin, slot.origin);
            let mask = to_bounds(&underline.content_mask, slot.origin).intersect(&slot);
            let style = UnderlineStyle {
                color: Some(to_hsla(&underline.color)),
                thickness: px(underline.thickness),
                wavy: underline.wavy,
            };
            window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                window.paint_underline(origin, px(underline.width), &style);
            });
        }
        bindings::Primitive::Glyph(glyph) => {
            let origin = to_point(&glyph.origin, slot.origin);
            let mask = to_bounds(&glyph.content_mask, slot.origin).intersect(&slot);
            let font_id = gpui::FontId(glyph.font_id as usize);
            let glyph_id = gpui::GlyphId(glyph.glyph_id);
            let font_size = px(glyph.font_size);
            let color = to_hsla(&glyph.color);
            let is_emoji = glyph.is_emoji;
            let wire_glyph_id = glyph.glyph_id;
            window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                let result = if is_emoji {
                    window.paint_emoji(origin, font_id, glyph_id, font_size)
                } else {
                    window.paint_glyph(origin, font_id, glyph_id, font_size, color)
                };
                if let Err(error) = result {
                    log::warn!("gpui_embedded: failed to paint glyph {wire_glyph_id}: {error:#}");
                }
            });
        }
        bindings::Primitive::Path(path) => {
            if path.vertices.len() < 3 {
                return;
            }
            let mask = to_bounds(&path.content_mask, slot.origin).intersect(&slot);
            let color = to_hsla(&path.color);
            let mut rebuilt = gpui::Path::new(to_point(&path.vertices[0].xy, slot.origin));
            for triangle in path.vertices.chunks_exact(3) {
                rebuilt.push_triangle(
                    (
                        to_point(&triangle[0].xy, slot.origin),
                        to_point(&triangle[1].xy, slot.origin),
                        to_point(&triangle[2].xy, slot.origin),
                    ),
                    (
                        point(triangle[0].st.x, triangle[0].st.y),
                        point(triangle[1].st.x, triangle[1].st.y),
                        point(triangle[2].st.x, triangle[2].st.y),
                    ),
                );
            }
            window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                window.paint_path(rebuilt, color);
            });
        }
        bindings::Primitive::Image(image) => {
            static MISSING_IMAGE_WARNED: std::sync::Once = std::sync::Once::new();
            static OPACITY_WARNED: std::sync::Once = std::sync::Once::new();
            let Some(render_image) = images.get(&image.image_id) else {
                MISSING_IMAGE_WARNED.call_once(|| {
                    log::warn!(
                        "gpui_embedded: display list references image {} before its payload arrived",
                        image.image_id
                    );
                });
                return;
            };
            if image.opacity != 1.0 {
                OPACITY_WARNED.call_once(|| {
                    log::warn!(
                        "gpui_embedded: image opacity is not supported by the public paint API; \
                         painting fully opaque"
                    );
                });
            }
            let bounds = to_bounds(&image.bounds, slot.origin);
            let mask = to_bounds(&image.content_mask, slot.origin).intersect(&slot);
            let corner_radii = to_corners(&image.corner_radii);
            let render_image = render_image.clone();
            let grayscale = image.grayscale;
            window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                if let Err(error) =
                    window.paint_image(bounds, corner_radii, render_image, 0, grayscale)
                {
                    log::warn!("gpui_embedded: failed to paint image: {error:#}");
                }
            });
        }
    }
}

fn wire_keystroke(keystroke: &Keystroke) -> bindings::Keystroke {
    bindings::Keystroke {
        modifiers: wire_modifiers(keystroke.modifiers),
        key: keystroke.key.clone(),
        key_char: keystroke.key_char.clone(),
    }
}

fn to_point(point: &bindings::Point, offset: Point<Pixels>) -> Point<Pixels> {
    gpui::point(px(point.x) + offset.x, px(point.y) + offset.y)
}

fn to_bounds(bounds: &bindings::Bounds, offset: Point<Pixels>) -> Bounds<Pixels> {
    Bounds {
        origin: to_point(&bounds.origin, offset),
        size: gpui::size(px(bounds.size.width), px(bounds.size.height)),
    }
}

fn to_corners(corners: &bindings::Corners) -> Corners<Pixels> {
    Corners {
        top_left: px(corners.top_left),
        top_right: px(corners.top_right),
        bottom_right: px(corners.bottom_right),
        bottom_left: px(corners.bottom_left),
    }
}

fn to_edges(edges: &bindings::Edges) -> Edges<Pixels> {
    Edges {
        top: px(edges.top),
        right: px(edges.right),
        bottom: px(edges.bottom),
        left: px(edges.left),
    }
}

fn to_hsla(color: &bindings::Hsla) -> gpui::Hsla {
    gpui::hsla(color.h, color.s, color.l, color.a)
}

fn to_border_style(style: bindings::BorderStyle) -> gpui::BorderStyle {
    match style {
        bindings::BorderStyle::Solid => gpui::BorderStyle::Solid,
        bindings::BorderStyle::Dashed => gpui::BorderStyle::Dashed,
    }
}

fn wire_button(button: MouseButton) -> Option<bindings::MouseButton> {
    match button {
        MouseButton::Left => Some(bindings::MouseButton::Left),
        MouseButton::Right => Some(bindings::MouseButton::Right),
        MouseButton::Middle => Some(bindings::MouseButton::Middle),
        _ => None,
    }
}

fn wire_modifiers(modifiers: gpui::Modifiers) -> bindings::Modifiers {
    bindings::Modifiers {
        control: modifiers.control,
        alt: modifiers.alt,
        shift: modifiers.shift,
        platform: modifiers.platform,
    }
}

fn wire_point(position: Point<Pixels>) -> bindings::Point {
    bindings::Point {
        x: f32::from(position.x),
        y: f32::from(position.y),
    }
}
