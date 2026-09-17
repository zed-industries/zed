use super::{MacWindowState, get_window_state, is_gpui_window};
use anyhow::{Context as _, Result};
use block2::RcBlock;
use gpui::{Along, Bounds, Pixels, Size, point, popup::*, px, size};
use objc2::{MainThreadMarker, rc::Retained, runtime::AnyObject};
use objc2_app_kit::{
    NSApplication, NSApplicationDidResignActiveNotification, NSEvent, NSEventMask, NSView,
    NSWindow, NSWindowDidMoveNotification, NSWindowDidResizeNotification,
    NSWindowWillCloseNotification,
};
use objc2_foundation::{
    NSNotification, NSNotificationCenter, NSObjectProtocol, NSPoint, NSRect, NSSize,
};
use parking_lot::Mutex;
use std::{
    ptr::NonNull,
    sync::{Arc, Weak},
};

pub(super) struct MacPopup {
    pub options: PopupOptions,
    pub size: Size<Pixels>,
    pub parent: Retained<NSWindow>,
    parent_view: Retained<NSView>,
    observers: Vec<Retained<objc2::runtime::ProtocolObject<dyn NSObjectProtocol>>>,
    mouse_monitor: Option<Retained<AnyObject>>,
}

impl MacPopup {
    pub fn new(
        options: PopupOptions,
        size: Size<Pixels>,
        marker: MainThreadMarker,
    ) -> Result<Self> {
        let application = NSApplication::sharedApplication(marker);
        for parent in application.windows() {
            let pointer = Retained::as_ptr(&parent).cast_mut().cast();
            // SAFETY: NSApplication owns these live windows; only GPUI classes have our ivar.
            if !unsafe { is_gpui_window(pointer) } {
                continue;
            }
            // SAFETY: The class check above guarantees an initialized GPUI window state.
            let state = unsafe { get_window_state(&*pointer) };
            let state = state.lock();
            if state.handle != options.parent
                || state.closed.load(std::sync::atomic::Ordering::Acquire)
            {
                continue;
            }
            anyhow::ensure!(
                !options.grab || state.popup.as_ref().is_none_or(|popup| popup.options.grab),
                "a grabbing popup cannot be parented to a passive popup"
            );
            // SAFETY: The parent owns this live GPUIView. Retaining it keeps the conversion
            // reference valid until the popup is detached during parent destruction.
            let parent_view =
                unsafe { Retained::retain(state.native_view.as_ptr().cast::<NSView>()) }
                    .context("popup parent content view not found")?;
            return Ok(Self {
                options,
                size,
                parent,
                parent_view,
                observers: Vec::new(),
                mouse_monitor: None,
            });
        }
        anyhow::bail!("popup parent window not found")
    }

    pub fn frame(&self) -> Result<NSRect> {
        let screen = self.parent.screen().context("popup parent has no screen")?;
        let view_bounds = self.parent_view.bounds();
        let anchor = self.options.anchor_rect;
        let anchor = NSRect::new(
            NSPoint::new(
                view_bounds.origin.x + anchor.origin.x.to_f64(),
                view_bounds.origin.y
                    + if self.parent_view.isFlipped() {
                        anchor.origin.y.to_f64()
                    } else {
                        view_bounds.size.height - anchor.bottom().to_f64()
                    },
            ),
            NSSize::new(anchor.size.width.to_f64(), anchor.size.height.to_f64()),
        );
        let anchor = self
            .parent
            .convertRectToScreen(self.parent_view.convertRect_toView(anchor, None));
        // Negating Y gives a common top-left coordinate space without assuming that the
        // parent lives on the primary screen or that display origins are nonnegative.
        let bounds = popup_bounds(
            &self.options,
            top_left_bounds(anchor),
            self.size,
            top_left_bounds(screen.visibleFrame()),
        );
        Ok(NSRect::new(
            NSPoint::new(bounds.origin.x.to_f64(), -bounds.bottom().to_f64()),
            NSSize::new(bounds.size.width.to_f64(), bounds.size.height.to_f64()),
        ))
    }

    pub fn observe(&mut self, state: &Arc<Mutex<MacWindowState>>) -> Result<()> {
        let center = NSNotificationCenter::defaultCenter();
        // SAFETY: AppKit's notification names are immutable process-lifetime constants.
        let moved = unsafe { NSWindowDidMoveNotification };
        // SAFETY: AppKit's notification names are immutable process-lifetime constants.
        let resized = unsafe { NSWindowDidResizeNotification };
        for name in [moved, resized] {
            let state = Arc::downgrade(state);
            let callback = RcBlock::new(move |_: NonNull<NSNotification>| reposition(&state, None));
            // SAFETY: AppKit delivers window notifications on the main thread. The block
            // captures only a weak state and the observer is removed before window teardown.
            let observer = unsafe {
                center.addObserverForName_object_queue_usingBlock(
                    Some(name),
                    Some(&self.parent),
                    None,
                    &callback,
                )
            };
            self.observers.push(observer);
        }

        // Hidden popups aren't attached to the parent yet: adding a native child would show it.
        let callback = RcBlock::new({
            let state = Arc::downgrade(state);
            move |_: NonNull<NSNotification>| dismiss(&state)
        });
        // SAFETY: AppKit posts this notification on the main thread. The observer is removed
        // before popup teardown, and the callback holds only a weak state.
        let observer = unsafe {
            center.addObserverForName_object_queue_usingBlock(
                Some(NSWindowWillCloseNotification),
                Some(&self.parent),
                None,
                &callback,
            )
        };
        self.observers.push(observer);

        if self.options.grab {
            let callback = RcBlock::new({
                let state = Arc::downgrade(state);
                move |_: NonNull<NSNotification>| dismiss(&state)
            });
            // SAFETY: AppKit posts application activation notifications on the main thread.
            let observer = unsafe {
                center.addObserverForName_object_queue_usingBlock(
                    Some(NSApplicationDidResignActiveNotification),
                    None,
                    None,
                    &callback,
                )
            };
            self.observers.push(observer);

            // A nonactivating panel can open while its application is already inactive, in
            // which case an outside click does not cause a resign-active notification.
            let state = Arc::downgrade(state);
            let callback = RcBlock::new(move |_: NonNull<NSEvent>| dismiss(&state));
            self.mouse_monitor = Some(
                NSEvent::addGlobalMonitorForEventsMatchingMask_handler(
                    NSEventMask::LeftMouseDown
                        | NSEventMask::RightMouseDown
                        | NSEventMask::OtherMouseDown,
                    &callback,
                )
                .context("could not monitor outside clicks for popup")?,
            );
        }
        Ok(())
    }
}

impl Drop for MacPopup {
    fn drop(&mut self) {
        if let Some(monitor) = self.mouse_monitor.take() {
            // SAFETY: This token came from NSEvent and is removed exactly once.
            unsafe { NSEvent::removeMonitor(&monitor) };
        }
        let center = NSNotificationCenter::defaultCenter();
        for observer in self.observers.drain(..) {
            // SAFETY: These are our observer tokens, removed on the main thread.
            unsafe { center.removeObserver((*observer).as_ref()) };
        }
    }
}

pub(super) fn dismiss(state: &Weak<Mutex<MacWindowState>>) {
    let Some(state) = state.upgrade() else { return };
    let executor = state.lock().foreground_executor.clone();
    executor
        .spawn(async move {
            let state = state.lock();
            if state.closed.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            let native_window = state.native_window;
            drop(state);
            // SAFETY: The state owns a live NSWindow, and the foreground executor is the main thread.
            unsafe { cocoa::appkit::NSWindow::close(native_window) };
        })
        .detach();
}

pub(super) fn dismiss_on_escape(state: &Arc<Mutex<MacWindowState>>) -> bool {
    if state
        .lock()
        .popup
        .as_ref()
        .is_some_and(|popup| popup.options.grab)
    {
        dismiss(&Arc::downgrade(state));
        true
    } else {
        false
    }
}

pub(super) fn reposition(
    state: &Weak<Mutex<MacWindowState>>,
    requested_size: Option<Size<Pixels>>,
) {
    let Some(state) = state.upgrade() else { return };
    let executor = state.lock().foreground_executor.clone();
    executor
        .spawn(async move {
            let mut lock = state.lock();
            if lock.closed.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            let native_window = lock.native_window;
            let Some(popup) = lock.popup.as_mut() else {
                return;
            };
            if let Some(size) = requested_size {
                popup.size = size;
            }
            let frame = popup.frame();
            drop(lock);
            match frame {
                Ok(frame) => {
                    // SAFETY: The closed-state check guarantees a live NSWindow on the main thread.
                    let window = unsafe { &*native_window.cast::<NSWindow>() };
                    window.setFrame_display(frame, true);
                }
                Err(error) => {
                    log::error!("could not reposition popup: {error:#}");
                    dismiss(&Arc::downgrade(&state));
                }
            }
        })
        .detach();
}

fn top_left_bounds(rect: NSRect) -> Bounds<Pixels> {
    Bounds::new(
        point(
            px(rect.origin.x as f32),
            px(-(rect.origin.y + rect.size.height) as f32),
        ),
        size(px(rect.size.width as f32), px(rect.size.height as f32)),
    )
}

fn popup_bounds(
    options: &PopupOptions,
    anchor_rect: Bounds<Pixels>,
    size: Size<Pixels>,
    available: Bounds<Pixels>,
) -> Bounds<Pixels> {
    let anchor = match options.anchor {
        PopupAnchor::Center => point(0.5, 0.5),
        PopupAnchor::Top => point(0.5, 0.),
        PopupAnchor::Bottom => point(0.5, 1.),
        PopupAnchor::Left => point(0., 0.5),
        PopupAnchor::Right => point(1., 0.5),
        PopupAnchor::TopLeft => point(0., 0.),
        PopupAnchor::BottomLeft => point(0., 1.),
        PopupAnchor::TopRight => point(1., 0.),
        PopupAnchor::BottomRight => point(1., 1.),
    };
    let gravity = match options.gravity {
        PopupGravity::Center => point(0.5, 0.5),
        PopupGravity::Top => point(0.5, 0.),
        PopupGravity::Bottom => point(0.5, 1.),
        PopupGravity::Left => point(0., 0.5),
        PopupGravity::Right => point(1., 0.5),
        PopupGravity::TopLeft => point(0., 0.),
        PopupGravity::BottomLeft => point(0., 1.),
        PopupGravity::TopRight => point(1., 0.),
        PopupGravity::BottomRight => point(1., 1.),
    };
    let mut result = Bounds::new(point(px(0.), px(0.)), size);
    for axis in [gpui::Axis::Horizontal, gpui::Axis::Vertical] {
        let (flip, slide, resize) = match axis {
            gpui::Axis::Horizontal => (
                PopupConstraintAdjustment::FLIP_X,
                PopupConstraintAdjustment::SLIDE_X,
                PopupConstraintAdjustment::RESIZE_X,
            ),
            gpui::Axis::Vertical => (
                PopupConstraintAdjustment::FLIP_Y,
                PopupConstraintAdjustment::SLIDE_Y,
                PopupConstraintAdjustment::RESIZE_Y,
            ),
        };
        let anchor_origin = anchor_rect.origin.along(axis);
        let anchor_size = anchor_rect.size.along(axis);
        let mut length = size.along(axis).max(px(1.));
        let start = available.origin.along(axis);
        let end = start + available.size.along(axis);
        let anchor = anchor.along(axis);
        let gravity = gravity.along(axis);
        let offset = options.offset.along(axis);
        let mut origin = anchor_origin + anchor_size * anchor - length * (1. - gravity) + offset;
        let constrained = |origin: Pixels| origin < start || origin + length > end;
        if constrained(origin) && options.constraint_adjustment.contains(flip) {
            let flipped = anchor_origin + anchor_size * (1. - anchor) - length * gravity + offset;
            if !constrained(flipped) {
                origin = flipped;
            }
        }
        if options.constraint_adjustment.contains(slide) {
            // Oversized popups leave the edge in the direction of gravity constrained,
            // so a subsequent resize preserves the opposite edge.
            origin = if gravity < 0.5 {
                origin.max(start).min(end - length)
            } else {
                origin.min(end - length).max(start)
            };
        }
        if options.constraint_adjustment.contains(resize) {
            let bottom_right = (origin + length).min(end);
            origin = origin.max(start).min(end - px(1.));
            length = (bottom_right - origin).max(px(1.));
        }
        result.origin = result.origin.apply_along(axis, |_| origin);
        result.size = result.size.apply_along(axis, |_| length);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    fn options(cx: &mut TestAppContext) -> PopupOptions {
        PopupOptions {
            parent: cx
                .add_empty_window()
                .update(|window, _| window.window_handle()),
            anchor_rect: Bounds::new(point(px(140.), px(210.)), size(px(80.), px(30.))),
            anchor: PopupAnchor::BottomLeft,
            gravity: PopupGravity::BottomRight,
            constraint_adjustment: PopupConstraintAdjustment::empty(),
            offset: point(px(7.), px(11.)),
            grab: false,
        }
    }

    #[gpui::test]
    fn dropdown_extends_below_anchor_with_offset(cx: &mut TestAppContext) {
        let options = options(cx);
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(70.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(147.), px(251.)), size(px(130.), px(70.)))
        );
    }

    #[gpui::test]
    fn centered_popup_uses_both_rectangle_sizes(cx: &mut TestAppContext) {
        let options = PopupOptions {
            anchor: PopupAnchor::Center,
            gravity: PopupGravity::Center,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(70.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(bounds.origin, point(px(122.), px(201.)));
    }

    #[gpui::test]
    fn bottom_edge_flips_before_sliding_and_preserves_offset(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::FLIP_Y
                | PopupConstraintAdjustment::SLIDE_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(170.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(147.), px(51.)), size(px(130.), px(170.)))
        );
    }

    #[gpui::test]
    fn horizontal_flip_does_not_flip_vertical_placement(cx: &mut TestAppContext) {
        let options = PopupOptions {
            anchor: PopupAnchor::BottomRight,
            constraint_adjustment: PopupConstraintAdjustment::FLIP_X,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(70.)),
            Bounds::new(point(px(0.), px(0.)), size(px(300.), px(400.))),
        );
        assert_eq!(bounds.origin, point(px(17.), px(251.)));
    }

    #[gpui::test]
    fn unsuccessful_flip_keeps_original_placement(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::FLIP_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(300.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(bounds.origin, point(px(147.), px(251.)));
    }

    #[gpui::test]
    fn failed_flip_can_slide_without_resizing(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::FLIP_Y
                | PopupConstraintAdjustment::SLIDE_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(300.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(147.), px(100.)), size(px(130.), px(300.)))
        );
    }

    #[gpui::test]
    fn resize_clips_only_the_permitted_axis(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::RESIZE_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(430.), px(200.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(147.), px(251.)), size(px(430.), px(149.)))
        );
    }

    #[gpui::test]
    fn resizing_at_top_left_preserves_bottom_right_edges(cx: &mut TestAppContext) {
        let options = PopupOptions {
            anchor: PopupAnchor::TopLeft,
            gravity: PopupGravity::TopLeft,
            constraint_adjustment: PopupConstraintAdjustment::RESIZE_X
                | PopupConstraintAdjustment::RESIZE_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(190.), px(260.)),
            Bounds::new(point(px(20.), px(40.)), size(px(480.), px(350.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(20.), px(40.)), size(px(127.), px(181.)))
        );
    }

    #[gpui::test]
    fn oversized_upward_popup_slides_without_shrinking(cx: &mut TestAppContext) {
        let options = PopupOptions {
            anchor: PopupAnchor::TopLeft,
            gravity: PopupGravity::TopLeft,
            constraint_adjustment: PopupConstraintAdjustment::SLIDE_X
                | PopupConstraintAdjustment::SLIDE_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(630.), px(700.)),
            Bounds::new(point(px(20.), px(40.)), size(px(480.), px(350.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(-130.), px(-310.)), size(px(630.), px(700.)))
        );
    }

    #[gpui::test]
    fn oversized_popup_slides_then_resizes_to_visible_area(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::all(),
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(630.), px(700.)),
            Bounds::new(point(px(20.), px(40.)), size(px(480.), px(350.))),
        );
        assert_eq!(
            bounds,
            Bounds::new(point(px(20.), px(40.)), size(px(480.), px(350.)))
        );
    }

    #[gpui::test]
    fn negative_display_origins_are_not_clamped_to_zero(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::SLIDE_X
                | PopupConstraintAdjustment::SLIDE_Y,
            ..options(cx)
        };
        let anchor = Bounds::new(point(px(-210.), px(-160.)), size(px(80.), px(30.)));
        let bounds = popup_bounds(
            &options,
            anchor,
            size(px(260.), px(190.)),
            Bounds::new(point(px(-800.), px(-600.)), size(px(800.), px(600.))),
        );
        assert_eq!(bounds.origin, point(px(-260.), px(-190.)));
    }

    #[gpui::test]
    fn exact_screen_edge_does_not_trigger_flip(cx: &mut TestAppContext) {
        let options = PopupOptions {
            constraint_adjustment: PopupConstraintAdjustment::FLIP_Y,
            ..options(cx)
        };
        let bounds = popup_bounds(
            &options,
            options.anchor_rect,
            size(px(130.), px(149.)),
            Bounds::new(point(px(0.), px(0.)), size(px(500.), px(400.))),
        );
        assert_eq!(bounds.origin, point(px(147.), px(251.)));
    }

    #[test]
    fn appkit_rect_conversion_preserves_secondary_display_origin() {
        let bounds = top_left_bounds(NSRect::new(
            NSPoint::new(-1200., 300.),
            NSSize::new(450., 280.),
        ));
        assert_eq!(
            bounds,
            Bounds::new(point(px(-1200.), px(-580.)), size(px(450.), px(280.)))
        );
    }
}
