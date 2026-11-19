use crate::{
    App, Bounds, Element, ElementId, FocusHandle, FocusId, Focusable, GlobalElementId, Hitbox,
    HitboxBehavior, Hsla, InspectorElementId, InteractiveElement, IntoElement, LayoutId,
    PaintNativeView, ParentElement, Pixels, Point, Stateful, Style, Styled, Window, div,
};
use anyhow::{Context as _, Result};
use std::{
    any::Any,
    cell::Cell,
    ffi::c_void,
    ptr::NonNull,
    rc::{Rc, Weak},
};

/// Identifies a [`NativeView`] within its window.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NativeViewId(u64);

/// A view owned by the platform's UI toolkit, shown inside a GPUI window.
///
/// The view is composited beneath the window's GPUI content and shows through
/// wherever this handle is used as an element, so elements painted after it
/// cover it like they would cover any other element. The view is treated as
/// opaque, and is not affected by element opacity or transformations.
///
/// The view takes keyboard input directly from the platform while its
/// [`FocusHandle`] is focused, and stays attached to the window, hidden when
/// it isn't painted, until the last clone of this handle is dropped.
#[derive(Clone)]
pub struct NativeView(Rc<NativeViewState>);

pub(crate) struct NativeViewState {
    id: NativeViewId,
    focus_handle: FocusHandle,
    background: Cell<Hsla>,
    _attachment: Box<dyn Any>,
}

impl NativeView {
    /// Attaches a platform view to the window.
    ///
    /// # Errors
    ///
    /// Fails on platforms that cannot composite native views.
    ///
    /// # Safety
    ///
    /// `view` must point to a live view of the kind the window's platform
    /// embeds, which is an `NSView` on macOS, and that view must not be in
    /// another view hierarchy.
    pub unsafe fn new(view: NonNull<c_void>, window: &mut Window, cx: &mut App) -> Result<Self> {
        let id = NativeViewId(window.next_native_view_id);
        // SAFETY: The caller upholds the platform's requirements for `view`.
        let attachment = unsafe { window.platform_window.attach_native_view(id, view) }
            .context("this platform cannot show native views")?;
        window.next_native_view_id += 1;
        let state = Rc::new(NativeViewState {
            id,
            focus_handle: cx.focus_handle(),
            background: Cell::new(Hsla::transparent_black()),
            _attachment: attachment,
        });
        window.native_views.push(Rc::downgrade(&state));
        Ok(Self(state))
    }
}

impl NativeView {
    /// Sets the color shown where the view has not caught up with its element.
    ///
    /// The platform moves the view through its own compositor, so for a frame
    /// or two after its element moves or resizes, part of the element can be
    /// uncovered. A color close to the view's own background hides that. It
    /// takes effect the next time the element is painted, and a window showing
    /// several native views uses the color of the one painted last.
    pub fn set_background(&self, color: impl Into<Hsla>) {
        self.0.background.set(color.into());
    }
}

impl Focusable for NativeView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.0.focus_handle.clone()
    }
}

impl IntoElement for NativeView {
    type Element = Stateful<crate::Div>;

    fn into_element(self) -> Self::Element {
        div()
            .id(("native-view", self.0.id.0))
            .track_focus(&self.0.focus_handle)
            .size_full()
            .child(NativeViewElement(self))
    }
}

struct NativeViewElement(NativeView);

impl Element for NativeViewElement {
    type RequestLayoutState = ();
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = crate::relative(1.).into();
        style.size.height = crate::relative(1.).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Hitbox {
        window.insert_hitbox(bounds, HitboxBehavior::Normal)
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Hitbox,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let id = self.0.0.id;
        let bounds = window.snap_bounds(hitbox.bounds);
        let content_mask = window.snapped_content_mask();
        window.next_frame.scene.insert_primitive(PaintNativeView {
            order: 0,
            id,
            hitbox_id: hitbox.id,
            bounds,
            content_mask,
            background: self.0.0.background.get(),
        });
    }
}

impl Window {
    /// Tells the platform which native view, if any, the focused handle belongs to.
    pub(super) fn sync_native_view_focus(&mut self) {
        self.native_views.retain(|view| view.strong_count() > 0);
        if self.native_views.is_empty() {
            return;
        }
        let focused = self
            .focus
            .and_then(|focus| self.native_view_for_focus(focus));
        self.platform_window.set_focused_native_view(focused);
    }

    fn native_view_for_focus(&self, focus: FocusId) -> Option<NativeViewId> {
        self.native_views
            .iter()
            .filter_map(Weak::upgrade)
            .find(|view| view.focus_handle.id == focus)
            .map(|view| view.id)
    }

    /// The native view that a mouse event at this position reaches, if no
    /// GPUI hitbox is above it there.
    pub(super) fn hit_test_native_view(&self, position: Point<Pixels>) -> Option<NativeViewId> {
        let hit_test = self.rendered_frame.hit_test(position);
        let topmost = hit_test.ids.first()?;
        self.rendered_frame
            .scene
            .native_views
            .iter()
            .find(|native_view| native_view.hitbox_id == *topmost)
            .map(|native_view| native_view.id)
    }

    /// Focuses the handle of a native view that the platform gave keyboard input to.
    pub(super) fn focus_native_view(&mut self, id: NativeViewId, cx: &mut App) {
        let view = self
            .native_views
            .iter()
            .filter_map(Weak::upgrade)
            .find(|view| view.id == id);
        if let Some(view) = view {
            self.focus(&view.focus_handle, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Context, FocusHandle, Render, TestAppContext, WindowHandle, point, prelude::*, px, rgb,
    };

    #[gpui::test]
    fn focusing_a_native_view_gives_it_keyboard_input(cx: &mut TestAppContext) {
        let window = open_window(cx);
        let platform_window = cx.test_window(window.into());

        let id = window
            .update(cx, |view, window, cx| {
                window.focus(&view.native_view.focus_handle(cx), cx);
                view.native_view.0.id
            })
            .unwrap();
        assert_eq!(platform_window.focused_native_view(), Some(id));

        window
            .update(cx, |view, window, cx| window.focus(&view.other_focus, cx))
            .unwrap();
        assert_eq!(platform_window.focused_native_view(), None);

        window
            .update(cx, |view, window, cx| {
                window.focus(&view.native_view.focus_handle(cx), cx);
                window.blur(cx);
            })
            .unwrap();
        assert_eq!(platform_window.focused_native_view(), None);
    }

    #[gpui::test]
    fn platform_focus_of_a_native_view_focuses_its_handle(cx: &mut TestAppContext) {
        let window = open_window(cx);
        let platform_window = cx.test_window(window.into());
        let id = window
            .update(cx, |view, window, cx| {
                window.focus(&view.other_focus, cx);
                view.native_view.0.id
            })
            .unwrap();

        platform_window.simulate_native_view_focused(id);

        window
            .update(cx, |view, window, cx| {
                assert!(view.native_view.focus_handle(cx).is_focused(window));
            })
            .unwrap();
        assert_eq!(platform_window.focused_native_view(), Some(id));
    }

    #[gpui::test]
    fn mouse_reaches_a_native_view_only_where_nothing_covers_it(cx: &mut TestAppContext) {
        let window = open_window(cx);
        let platform_window = cx.test_window(window.into());
        let id = window
            .update(cx, |view, _, _| view.native_view.0.id)
            .unwrap();
        cx.run_until_parked();

        let uncovered = point(px(75.), px(75.));
        let covered = point(px(25.), px(25.));
        let outside = point(px(150.), px(150.));
        assert_eq!(platform_window.hit_test_native_view(uncovered), Some(id));
        assert_eq!(platform_window.hit_test_native_view(covered), None);
        assert_eq!(platform_window.hit_test_native_view(outside), None);
    }

    #[gpui::test]
    fn native_view_is_drawn_in_paint_order(cx: &mut TestAppContext) {
        let window = open_window(cx);
        cx.run_until_parked();

        // The view shows through its container's background, and the element
        // painted after it covers it.
        window
            .update(cx, |_, window, _| {
                let labels = window
                    .rendered_frame
                    .scene
                    .batches()
                    .map(|batch| batch.label())
                    .collect::<Vec<_>>();
                assert_eq!(labels, ["quads (1)", "native views (1)", "quads (1)"]);
            })
            .unwrap();
    }

    #[gpui::test]
    fn native_view_stays_attached_until_its_last_handle_is_dropped(cx: &mut TestAppContext) {
        let window = open_window(cx);
        let platform_window = cx.test_window(window.into());
        let other_handle = window
            .update(cx, |view, _, _| view.native_view.clone())
            .unwrap();
        assert_eq!(platform_window.attached_native_view_count(), 1);

        window
            .update(cx, |view, window, cx| {
                view.native_view = new_native_view(window, cx);
            })
            .unwrap();
        assert_eq!(platform_window.attached_native_view_count(), 2);

        drop(other_handle);
        assert_eq!(platform_window.attached_native_view_count(), 1);
    }

    struct TestView {
        native_view: NativeView,
        other_focus: FocusHandle,
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size(px(100.))
                        .bg(rgb(0xff0000))
                        .child(self.native_view.clone()),
                )
                .child(
                    div()
                        .id("cover")
                        .track_focus(&self.other_focus)
                        .absolute()
                        .top_0()
                        .left_0()
                        .size(px(50.))
                        .occlude()
                        .bg(rgb(0x0000ff)),
                )
        }
    }

    fn open_window(cx: &mut TestAppContext) -> WindowHandle<TestView> {
        cx.add_window(|window, cx| TestView {
            native_view: new_native_view(window, cx),
            other_focus: cx.focus_handle(),
        })
    }

    fn new_native_view(window: &mut Window, cx: &mut App) -> NativeView {
        // SAFETY: The test platform never reads the view it is given.
        unsafe { NativeView::new(NonNull::dangling(), window, cx) }
            .expect("the test platform attaches native views")
    }
}
