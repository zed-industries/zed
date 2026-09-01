//! The color picker that opens when a document color swatch is clicked.
//!
//! The picker edits the buffer directly: it works out how to rewrite the color
//! from the text at the swatch's range, either through the language's
//! `colors.scm` captures (which know each channel's position and units) or, for
//! a plain literal such as `#ff00aa`, by rewriting the literal in place. A color
//! it cannot rewrite — a Tailwind class name a language server reported, say —
//! is shown but not editable.

use std::{cell::Cell, ops::Range, rc::Rc, time::Duration};

use gpui::{
    App, Bounds, Context, DismissEvent, DragMoveEvent, Empty, EventEmitter, FocusHandle, Focusable,
    Hsla, MouseDownEvent, Pixels, Point, Rgba, Window, div, linear_color_stop, linear_gradient,
    prelude::*, px,
};
use language::{ColorMatch, ColorReplacement, ToOffset as _, parse_color_literal};
use multi_buffer::Anchor;
use ui::{
    ActiveTheme as _, ButtonCommon as _, Clickable as _, IconButton, IconName, IconSize, Label,
    LabelCommon as _, LabelSize, StyledExt as _, Toggleable as _, Tooltip, h_flex, v_flex,
};

use crate::Editor;

/// How often a drag across the picker is allowed to rewrite the buffer.
///
/// The picker itself repaints on every mouse move, but each buffer edit costs a
/// reparse and a full round of language-server refreshes, which is far too much
/// to run at mouse-move rate. Writes are coalesced to this interval, with a
/// trailing write so the buffer always ends up holding the color that was
/// released on.
const WRITE_THROTTLE: Duration = Duration::from_millis(50);

/// Payload for the drag handlers, so that dragging one track does not move
/// another picker's handle.
struct DraggedTrack {
    track: Track,
    picker: gpui::EntityId,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Track {
    /// The two-dimensional saturation (x) and value (y) area.
    SaturationValue,
    Hue,
    Alpha,
}

pub struct ColorPicker {
    editor: gpui::WeakEntity<Editor>,
    /// The construct being edited, as an anchor range so that it survives the
    /// picker's own edits.
    range: Range<Anchor>,
    /// The color in HSV, which is what the saturation/value area is shaped
    /// around. HSL would make the area's corners unreachable.
    hue: f32,
    saturation: f32,
    value: f32,
    alpha: f32,
    supports_alpha: bool,
    /// `false` when the text at `range` is not something we know how to
    /// rewrite, in which case the picker only reports the color.
    editable: bool,
    track_bounds: [Rc<Cell<Bounds<Pixels>>>; 3],
    /// Set while the platform's eyedropper is on screen. It takes focus away
    /// from Zed, which would otherwise dismiss the picker before the user has
    /// picked anything.
    picking_from_screen: bool,
    screen_pick_task: gpui::Task<()>,
    /// The most recent color not yet written to the buffer, and the task that
    /// drains it. See [`WRITE_THROTTLE`].
    pending_write: Option<lsp::Color>,
    writing: bool,
    write_task: gpui::Task<()>,
    focus_handle: FocusHandle,
    _dismiss_on_blur: gpui::Subscription,
}

impl EventEmitter<DismissEvent> for ColorPicker {}

impl Focusable for ColorPicker {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ColorPicker {
    pub fn new(
        editor: gpui::WeakEntity<Editor>,
        range: Range<Anchor>,
        color: lsp::Color,
        rewrite: Option<ColorRewrite>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (hue, saturation, value) = rgb_to_hsv(Rgba {
            r: color.red,
            g: color.green,
            b: color.blue,
            a: color.alpha,
        });
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        // Clicking back into the editor, or anywhere else, closes the picker.
        let dismiss_on_blur = cx.on_focus_out(&focus_handle, window, |this, _, _, cx| {
            if !this.picking_from_screen {
                cx.emit(DismissEvent);
            }
        });
        Self {
            editor,
            range,
            hue,
            saturation,
            value,
            alpha: color.alpha,
            supports_alpha: rewrite.as_ref().is_some_and(|rewrite| rewrite.supports_alpha),
            editable: rewrite.is_some(),
            track_bounds: std::array::from_fn(|_| Rc::new(Cell::new(Bounds::default()))),
            picking_from_screen: false,
            screen_pick_task: gpui::Task::ready(()),
            pending_write: None,
            writing: false,
            write_task: gpui::Task::ready(()),
            focus_handle,
            _dismiss_on_blur: dismiss_on_blur,
        }
    }

    fn color(&self) -> lsp::Color {
        let rgba = hsv_to_rgb(self.hue, self.saturation, self.value, self.alpha);
        lsp::Color {
            red: rgba.r,
            green: rgba.g,
            blue: rgba.b,
            alpha: rgba.a,
        }
    }

    fn hsla(&self) -> Hsla {
        Hsla::from(hsv_to_rgb(self.hue, self.saturation, self.value, self.alpha))
    }

    fn bounds(&self, track: Track) -> Bounds<Pixels> {
        self.track_bounds[track as usize].get()
    }

    fn set_from_position(&mut self, track: Track, position: Point<Pixels>, cx: &mut Context<Self>) {
        let bounds = self.bounds(track);
        if bounds.size.width <= px(0.) || bounds.size.height <= px(0.) {
            return;
        }
        let fraction = |value: Pixels, origin: Pixels, size: Pixels| {
            (f32::from(value - origin) / f32::from(size)).clamp(0.0, 1.0)
        };
        let x = fraction(position.x, bounds.origin.x, bounds.size.width);
        match track {
            Track::SaturationValue => {
                self.saturation = x;
                self.value = 1.0 - fraction(position.y, bounds.origin.y, bounds.size.height);
            }
            Track::Hue => self.hue = x,
            Track::Alpha => self.alpha = x,
        }
        self.apply(cx);
        cx.notify();
    }

    /// Hand the user the platform's eyedropper and adopt whatever they click.
    fn pick_from_screen(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.picking_from_screen {
            return;
        }
        self.picking_from_screen = true;
        cx.notify();

        let picked = cx.pick_screen_color();
        self.screen_pick_task = cx.spawn_in(window, async move |this, cx| {
            let picked = picked.await;
            this.update_in(cx, |this, window, cx| {
                // The eyedropper took focus; take it back so that the picker
                // keeps behaving like an open popover. The guard stays up until
                // focus is settled, so that returning from the eyedropper does
                // not read as the user clicking away.
                this.focus_handle.focus(window, cx);
                this.picking_from_screen = false;
                match picked {
                    Ok(Ok(Some(color))) => {
                        let (hue, saturation, value) = rgb_to_hsv(color);
                        this.hue = hue;
                        this.saturation = saturation;
                        this.value = value;
                        this.apply(cx);
                    }
                    // The user dismissed the eyedropper.
                    Ok(Ok(None)) => {}
                    Ok(Err(error)) => log::error!("Failed to pick a color from the screen: {error}"),
                    Err(canceled) => {
                        log::error!("Screen color picking did not complete: {canceled}")
                    }
                }
                cx.notify();
            })
            .ok();
        });
    }

    fn apply(&mut self, cx: &mut Context<Self>) {
        if !self.editable {
            return;
        }
        self.pending_write = Some(self.color());
        if self.writing {
            // A drain is already running and will pick this color up.
            return;
        }
        self.writing = true;
        self.write_task = cx.spawn(async move |this, cx| {
            loop {
                let still_writing = this
                    .update(cx, |this, cx| match this.pending_write.take() {
                        Some(color) => {
                            this.write(color, cx);
                            true
                        }
                        None => {
                            this.writing = false;
                            false
                        }
                    })
                    .unwrap_or(false);
                if !still_writing {
                    break;
                }
                cx.background_executor().timer(WRITE_THROTTLE).await;
            }
        });
    }

    /// Write the throttled color straight away, so that releasing the mouse
    /// lands the final color without waiting out the interval.
    fn flush_write(&mut self, cx: &mut Context<Self>) {
        if let Some(color) = self.pending_write.take() {
            self.write(color, cx);
        }
    }

    fn write(&mut self, color: lsp::Color, cx: &mut Context<Self>) {
        let range = self.range.clone();
        self.editor
            .update(cx, |editor, cx| {
                editor.rewrite_color(&range, color, cx);
            })
            .ok();
    }

    fn render_track(
        &self,
        track: Track,
        background: gpui::AnyElement,
        handle: gpui::AnyElement,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        let picker = cx.entity_id();
        let bounds = self.track_bounds[track as usize].clone();
        div()
            .id(match track {
                Track::SaturationValue => "saturation-value",
                Track::Hue => "hue",
                Track::Alpha => "alpha",
            })
            .relative()
            .rounded_sm()
            .overflow_hidden()
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .cursor_crosshair()
            // Records where the track was painted so that a click position can
            // be turned into a channel value. Without an explicit size the
            // canvas lays out at zero and every click resolves to nothing.
            .child(
                gpui::canvas(
                    move |new_bounds, _, _| bounds.set(new_bounds),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .child(background)
            .child(handle)
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                    this.set_from_position(track, event.position, cx);
                }),
            )
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|this, _: &gpui::MouseUpEvent, _, cx| this.flush_write(cx)),
            )
            .on_drag(DraggedTrack { track, picker }, |_, _, _, cx| {
                cx.new(|_| Empty)
            })
            .on_drag_move(cx.listener(
                move |this, event: &DragMoveEvent<DraggedTrack>, _, cx| {
                    let dragged = event.drag(cx);
                    if dragged.picker == picker {
                        this.set_from_position(dragged.track, event.event.position, cx);
                    }
                },
            ))
    }
}

/// How the color at a swatch can be written back to the buffer.
pub struct ColorRewrite {
    pub supports_alpha: bool,
}

/// The six hue stops that make up the rainbow track.
fn hue_stops() -> [Hsla; 7] {
    std::array::from_fn(|index| Hsla {
        h: index as f32 / 6.0,
        s: 1.0,
        l: 0.5,
        a: 1.0,
    })
}

fn rgb_to_hsv(color: Rgba) -> (f32, f32, f32) {
    let max = color.r.max(color.g).max(color.b);
    let min = color.r.min(color.g).min(color.b);
    let delta = max - min;
    let hue = if delta <= f32::EPSILON {
        0.0
    } else if max == color.r {
        ((color.g - color.b) / delta).rem_euclid(6.0) / 6.0
    } else if max == color.g {
        ((color.b - color.r) / delta + 2.0) / 6.0
    } else {
        ((color.r - color.g) / delta + 4.0) / 6.0
    };
    let saturation = if max <= f32::EPSILON { 0.0 } else { delta / max };
    (hue, saturation, max)
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32, alpha: f32) -> Rgba {
    let sector = (hue.rem_euclid(1.0) * 6.0).min(5.999_9);
    let offset = sector - sector.floor();
    let p = value * (1.0 - saturation);
    let q = value * (1.0 - saturation * offset);
    let t = value * (1.0 - saturation * (1.0 - offset));
    let (r, g, b) = match sector as u32 {
        0 => (value, t, p),
        1 => (q, value, p),
        2 => (p, value, t),
        3 => (p, q, value),
        4 => (t, p, value),
        _ => (value, p, q),
    };
    Rgba { r, g, b, a: alpha }
}

fn hex_label(color: lsp::Color, with_alpha: bool) -> String {
    let byte = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    if with_alpha {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            byte(color.red),
            byte(color.green),
            byte(color.blue),
            byte(color.alpha)
        )
    } else {
        format!(
            "#{:02X}{:02X}{:02X}",
            byte(color.red),
            byte(color.green),
            byte(color.blue)
        )
    }
}

/// A checkerboard hint behind translucent colors, so that alpha is visible.
fn transparency_checker(cx: &App) -> impl IntoElement {
    let checker = if cx.theme().appearance().is_light() {
        gpui::black().opacity(0.12)
    } else {
        gpui::white().opacity(0.12)
    };
    div().absolute().inset_0().bg(checker)
}

impl Render for ColorPicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color = self.color();
        let hsla = self.hsla();
        let pure_hue = Hsla {
            h: self.hue,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };
        let border_variant = cx.theme().colors().border_variant;

        let saturation_value = self.render_track(
            Track::SaturationValue,
            div()
                .size_full()
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(pure_hue, 1.0),
                    linear_color_stop(gpui::white(), 0.0),
                ))
                .child(
                    div().absolute().inset_0().bg(linear_gradient(
                        180.,
                        linear_color_stop(gpui::black(), 1.0),
                        linear_color_stop(gpui::black().opacity(0.), 0.0),
                    )),
                )
                .into_any_element(),
            div()
                .absolute()
                .left(gpui::relative(self.saturation))
                .top(gpui::relative(1.0 - self.value))
                .child(
                    div()
                        .size_3()
                        .ml(px(-6.))
                        .mt(px(-6.))
                        .rounded_full()
                        .border_2()
                        .border_color(gpui::white())
                        .shadow_sm(),
                )
                .into_any_element(),
            cx,
        );

        let stops = hue_stops();
        let hue = self.render_track(
            Track::Hue,
            h_flex()
                .size_full()
                .children((0..6).map(|index| {
                    div().flex_1().h_full().bg(linear_gradient(
                        90.,
                        linear_color_stop(stops[index], 0.0),
                        linear_color_stop(stops[index + 1], 1.0),
                    ))
                }))
                .into_any_element(),
            div()
                .absolute()
                .left(gpui::relative(self.hue))
                .h_full()
                .child(
                    div()
                        .w_1()
                        .ml(px(-2.))
                        .h_full()
                        .rounded_sm()
                        .border_2()
                        .border_color(gpui::white())
                        .shadow_sm(),
                )
                .into_any_element(),
            cx,
        );

        let opaque = Hsla { a: 1.0, ..hsla };
        let alpha = self.render_track(
            Track::Alpha,
            div()
                .size_full()
                .child(transparency_checker(cx))
                .child(
                    div().absolute().inset_0().bg(linear_gradient(
                        90.,
                        linear_color_stop(opaque.opacity(0.), 0.0),
                        linear_color_stop(opaque, 1.0),
                    )),
                )
                .into_any_element(),
            div()
                .absolute()
                .left(gpui::relative(self.alpha))
                .h_full()
                .child(
                    div()
                        .w_1()
                        .ml(px(-2.))
                        .h_full()
                        .rounded_sm()
                        .border_2()
                        .border_color(gpui::white())
                        .shadow_sm(),
                )
                .into_any_element(),
            cx,
        );

        v_flex()
            .key_context("ColorPicker")
            .track_focus(&self.focus_handle)
            // Without this, a click on the picker's padding falls through to the
            // editor, which focuses it and dismisses the picker mid-use.
            .occlude()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .w(px(232.))
            .p_2()
            .gap_2()
            .elevation_2(cx)
            .child(saturation_value.h(px(140.)).w_full())
            .child(hue.h(px(12.)).w_full())
            .when(self.supports_alpha, |this| {
                this.child(alpha.h(px(12.)).w_full())
            })
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .relative()
                            .size_5()
                            .rounded_sm()
                            .overflow_hidden()
                            .border_1()
                            .border_color(border_variant)
                            .child(transparency_checker(cx))
                            .child(div().absolute().inset_0().bg(hsla)),
                    )
                    .child(
                        Label::new(hex_label(color, self.supports_alpha && self.alpha < 1.0))
                            .size(LabelSize::Small),
                    )
                    .when(self.editable && cx.is_screen_color_picking_supported(), |this| {
                        this.child(div().flex_1()).child(
                            IconButton::new("pick-from-screen", IconName::Crosshair)
                                .icon_size(IconSize::Small)
                                .toggle_state(self.picking_from_screen)
                                .tooltip(Tooltip::text("Pick a color from anywhere on screen"))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.pick_from_screen(window, cx);
                                })),
                        )
                    }),
            )
            .when(!self.editable, |this| {
                this.child(
                    Label::new("Read-only: this color's syntax can't be rewritten")
                        .size(LabelSize::XSmall)
                        .color(ui::Color::Muted),
                )
            })
    }
}

impl Editor {
    /// Open the color picker for the swatch belonging to `inlay_id`.
    pub(crate) fn open_color_picker(
        &mut self,
        inlay_id: project::InlayId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((range, color)) = self
            .colors
            .as_ref()
            .and_then(|colors| colors.color_for_inlay(inlay_id))
        else {
            return;
        };

        let rewrite = self
            .color_rewrite_at(&range, cx)
            .map(|(color_match, _)| ColorRewrite {
                supports_alpha: color_match.supports_alpha(),
            });

        let editor = cx.weak_entity();
        let picker = cx.new(|cx| ColorPicker::new(editor, range.clone(), color, rewrite, window, cx));
        let dismiss = cx.subscribe(&picker, |editor, _, _: &DismissEvent, cx| {
            editor.color_picker = None;
            cx.notify();
        });
        self.color_picker = Some(ColorPickerState {
            position: range.start,
            picker,
            _dismiss_subscription: dismiss,
        });
        cx.notify();
    }

    /// The color literal at `range` and the buffer it lives in, if the text
    /// there is something we know how to rewrite.
    fn color_rewrite_at(
        &self,
        range: &Range<Anchor>,
        cx: &App,
    ) -> Option<(ColorMatch, language::BufferSnapshot)> {
        let buffer_id = range.start.buffer_id()?;
        let buffer = self.buffer.read(cx).buffer(buffer_id)?;
        let snapshot = buffer.read(cx).snapshot();
        let start = range.start.text_anchor_in(&snapshot).to_offset(&snapshot);
        let end = range.end.text_anchor_in(&snapshot).to_offset(&snapshot);
        if end <= start {
            return None;
        }

        // Prefer the language's own understanding of the construct, which knows
        // where each channel sits; fall back to reading the text as a literal,
        // which covers colors that only a language server reported.
        let query_match = snapshot
            .color_matches(start..end)
            .find(|color_match| color_match.range == (start..end));
        if let Some(color_match) = query_match {
            return Some((color_match, snapshot));
        }

        let text = snapshot.text_for_range(start..end).collect::<String>();
        let (span, color, notation) = parse_color_literal(&text)?;
        Some((
            ColorMatch {
                buffer_id,
                range: start..end,
                color,
                replacement: ColorReplacement::Literal {
                    range: start + span.start..start + span.end,
                    notation,
                },
            },
            snapshot,
        ))
    }

    /// Rewrite the color literal at `range` to `new_color`, keeping the syntax
    /// the author used.
    pub(crate) fn rewrite_color(
        &mut self,
        range: &Range<Anchor>,
        new_color: lsp::Color,
        cx: &mut Context<Self>,
    ) {
        let Some((color_match, buffer_snapshot)) = self.color_rewrite_at(range, cx) else {
            return;
        };
        let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);

        let edits = color_match
            .edits(new_color)
            .into_iter()
            .filter_map(|(edit_range, text)| {
                let buffer_range = buffer_snapshot.anchor_before(edit_range.start)
                    ..buffer_snapshot.anchor_after(edit_range.end);
                let range = multi_buffer_snapshot.buffer_anchor_range_to_anchor_range(buffer_range)?;
                Some((range, text))
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            return;
        }

        // Wrapped in a transaction so that a drag across the picker collapses
        // into a single undo step rather than one per mouse move.
        self.buffer.update(cx, |buffer, cx| {
            buffer.start_transaction(cx);
            buffer.edit(edits, None, cx);
            buffer.end_transaction(cx);
        });
    }

}

pub(crate) struct ColorPickerState {
    /// Anchored to the start of the color construct, so the picker follows the
    /// text as the buffer scrolls or changes above it.
    pub(crate) position: Anchor,
    pub(crate) picker: gpui::Entity<ColorPicker>,
    _dismiss_subscription: gpui::Subscription,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_close(actual: Rgba, expected: [f32; 4]) {
        let actual = [actual.r, actual.g, actual.b, actual.a];
        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert!(
                (actual - expected).abs() < 0.001,
                "expected {expected:?}, got {actual:?}"
            );
        }
    }

    #[test]
    fn test_hsv_round_trip() {
        for color in [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [0.2, 0.9, 0.4, 1.0],
            [0.5, 0.5, 0.5, 0.5],
            [0.0, 0.0, 0.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.4, 0.2, 0.6, 0.25],
        ] {
            let rgba = Rgba {
                r: color[0],
                g: color[1],
                b: color[2],
                a: color[3],
            };
            let (hue, saturation, value) = rgb_to_hsv(rgba);
            assert_close(hsv_to_rgb(hue, saturation, value, rgba.a), color);
        }
    }

    #[test]
    fn test_hsv_corners_of_the_saturation_value_area() {
        // The top-left of the area is white and the bottom edge is black
        // whatever the hue, which is what makes HSV the right model for it.
        assert_close(hsv_to_rgb(0.5, 0.0, 1.0, 1.0), [1.0, 1.0, 1.0, 1.0]);
        assert_close(hsv_to_rgb(0.5, 1.0, 0.0, 1.0), [0.0, 0.0, 0.0, 1.0]);
        // The top-right is the fully saturated hue itself.
        assert_close(hsv_to_rgb(1.0 / 3.0, 1.0, 1.0, 1.0), [0.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_hex_label() {
        let color = lsp::Color {
            red: 1.0,
            green: 0.0,
            blue: 0.667,
            alpha: 0.5,
        };
        assert_eq!(hex_label(color, false), "#FF00AA");
        assert_eq!(hex_label(color, true), "#FF00AA80");
    }
}
