use crate::{Action, KeyDownEvent, PlatformInput};
use crate::{
    AnyWindowHandle, App, Bounds, Element, ElementId, FocusHandle, FocusId, Focusable,
    GlobalElementId, Hitbox, HitboxBehavior, Hsla, InspectorElementId, InteractiveElement,
    IntoElement, LayoutId, PaintNativeView, ParentElement, Pixels, Point, Style, Styled, Window,
    div,
};
use anyhow::{Context as _, Result};
use collections::HashSet;
use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
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
/// Mouse input over the uncovered part of the view goes to the view, and to
/// nothing behind it. The window's own content is still told of a mouse press
/// there, as a press outside every element.
///
/// The view takes keyboard input directly from the platform while its
/// [`FocusHandle`] is focused, which a click on the view does, as can an
/// assistive technology such as VoiceOver moving its cursor into the view. A key the view doesn't use is then dispatched
/// from the view's element like any other key press, so key bindings apply
/// to what the view leaves. Bindings for actions passed to
/// [`reserve_action`](Self::reserve_action) apply before the view sees the key.
///
/// The view stays attached to the window it was created in, hidden when it
/// isn't painted, until the last clone of this handle is dropped. Painted more
/// than once in a frame, it shows only where it was painted first. It shows
/// nothing as an element of any other window, so showing the same platform
/// view in another window takes dropping this handle and creating one there.
#[derive(Clone)]
pub struct NativeView(Rc<NativeViewState>);

pub(crate) struct NativeViewState {
    id: NativeViewId,
    window: AnyWindowHandle,
    focus_handle: FocusHandle,
    background: Cell<Hsla>,
    reserved_actions: RefCell<HashSet<TypeId>>,
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
    ///
    /// GPUI retains the view and puts it inside a view of its own, until the
    /// last clone of the returned handle is dropped. Until then GPUI decides the
    /// view's frame, whether it is hidden, and its order among the window's other
    /// views, so the caller must leave those alone and must not move the view to
    /// another superview.
    pub unsafe fn new(view: NonNull<c_void>, window: &mut Window, cx: &mut App) -> Result<Self> {
        let id = NativeViewId(window.next_native_view_id);
        // SAFETY: The caller upholds the platform's requirements for `view`.
        let attachment = unsafe { window.platform_window.attach_native_view(id, view) }
            .context("this platform cannot show native views")?;
        window.next_native_view_id += 1;
        let state = Rc::new(NativeViewState {
            id,
            window: window.handle,
            focus_handle: cx.focus_handle(),
            background: Cell::new(Hsla::transparent_black()),
            reserved_actions: RefCell::default(),
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
    /// uncovered. A color close to the view's own background hides that. A
    /// window showing several native views uses the color of the one painted
    /// last.
    pub fn set_background(&self, color: impl Into<Hsla>, cx: &mut App) {
        let color = color.into();
        if self.0.background.replace(color) == color {
            return;
        }
        // The color is part of what the element paints.
        let window = self.0.window;
        cx.defer(move |cx| {
            window.update(cx, |_, window, _| window.refresh()).ok();
        });
    }

    /// Makes key bindings for `A` apply before the view sees the key.
    ///
    /// The view otherwise sees every key first and can keep any of them, so
    /// this is for the few bindings that must work whatever the view shows.
    pub fn reserve_action<A: Action>(&self) {
        self.0
            .reserved_actions
            .borrow_mut()
            .insert(TypeId::of::<A>());
    }
}

impl Focusable for NativeView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.0.focus_handle.clone()
    }
}

impl IntoElement for NativeView {
    type Element = crate::Div;

    fn into_element(self) -> Self::Element {
        div()
            .track_focus(&self.0.focus_handle)
            .size_full()
            .child(NativeViewElement(self))
    }
}

struct NativeViewElement(NativeView);

impl Element for NativeViewElement {
    type RequestLayoutState = ();
    /// `None` in a window other than the one the view is attached to.
    type PrepaintState = Option<Hitbox>;

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
    ) -> Option<Hitbox> {
        (self.0.0.window == window.handle)
            .then(|| window.insert_hitbox(bounds, HitboxBehavior::BlockMouse))
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Option<Hitbox>,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let Some(hitbox) = hitbox else {
            return;
        };
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

    /// Ends an unfinished composition in the window's own content when keyboard
    /// input is about to move to a native view.
    ///
    /// The platform also asks for this as its own view loses keyboard input, but
    /// that request needs the app, which is already borrowed when focus is moved
    /// from inside an update.
    pub(super) fn end_composition_before_focusing_native_view(
        &mut self,
        focus: FocusId,
        cx: &mut App,
    ) {
        if self.native_view_for_focus(focus).is_none() {
            return;
        }
        if let Some(mut input_handler) = self.platform_window.take_input_handler() {
            input_handler.unmark_text_during_update(self, cx);
            self.platform_window.set_input_handler(input_handler);
        }
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

    /// Dispatches a key press that the focused native view has not seen yet, if
    /// it continues a pending binding or is bound to an action the view reserved.
    pub(super) fn dispatch_key_down_before_native_view(
        &mut self,
        event: KeyDownEvent,
        cx: &mut App,
    ) -> bool {
        let Some(view) = self
            .native_views
            .iter()
            .filter_map(Weak::upgrade)
            .find(|view| Some(view.focus_handle.id) == self.focus)
        else {
            return false;
        };
        if !self.has_pending_keystrokes() {
            let node_id = self.focus_node_id_in_rendered_frame(self.focus);
            let dispatch_path = self.rendered_frame.dispatch_tree.dispatch_path(node_id);
            let (bindings, _, _) = self
                .rendered_frame
                .dispatch_tree
                .bindings_for_input(std::slice::from_ref(&event.keystroke), &dispatch_path);
            let is_reserved = bindings.first().is_some_and(|binding| {
                view.reserved_actions
                    .borrow()
                    .contains(&binding.action.as_any().type_id())
            });
            if !is_reserved {
                return false;
            }
        }
        !self
            .dispatch_event(PlatformInput::KeyDown(event), cx)
            .propagate
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
        Context, FocusHandle, InputHandler, KeyBinding, Keystroke, Modifiers, MouseButton, Render,
        ScaledPixels, TestAppContext, UTF16Selection, VisualTestContext, WindowHandle, canvas,
        point, prelude::*, px, rgb,
    };
    use std::ops::Range;

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
    fn only_reserved_and_pending_bindings_apply_before_a_native_view(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.bind_keys([
                KeyBinding::new("cmd-r", Reserved, None),
                KeyBinding::new("cmd-o", Ordinary, None),
                KeyBinding::new("cmd-k o", Ordinary, None),
            ]);
        });
        let window = open_window(cx);
        let platform_window = cx.test_window(window.into());
        window
            .update(cx, |view, window, cx| {
                view.native_view.reserve_action::<Reserved>();
                window.focus(&view.native_view.focus_handle(cx), cx);
            })
            .unwrap();
        cx.run_until_parked();
        let press = |keystroke: &'static str| KeyDownEvent {
            keystroke: Keystroke::parse(keystroke).unwrap(),
            is_held: false,
            prefer_character_input: false,
        };
        let actions = |cx: &mut TestAppContext| {
            window
                .update(cx, |view, _, _| std::mem::take(&mut view.actions))
                .unwrap()
        };

        assert!(platform_window.simulate_key_down_before_native_view(press("cmd-r")));
        assert_eq!(actions(cx), ["reserved"]);

        // The view gets to see an ordinary binding's key first.
        assert!(!platform_window.simulate_key_down_before_native_view(press("cmd-o")));
        assert_eq!(actions(cx), [] as [&str; 0]);

        // Once the view has left the first key of a binding to GPUI, the rest
        // of that binding's keys are GPUI's.
        assert!(!platform_window.simulate_key_down_before_native_view(press("cmd-k")));
        cx.simulate_keystrokes(window.into(), "cmd-k");
        assert!(platform_window.simulate_key_down_before_native_view(press("o")));
        assert_eq!(actions(cx), ["ordinary"]);

        // Nothing applies ahead of the window's own content.
        window
            .update(cx, |view, window, cx| window.focus(&view.other_focus, cx))
            .unwrap();
        assert!(!platform_window.simulate_key_down_before_native_view(press("cmd-r")));
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
    fn mouse_press_on_a_native_view_reaches_nothing_behind_it(cx: &mut TestAppContext) {
        let window = open_window(cx);
        cx.run_until_parked();
        let mut cx = VisualTestContext::from_window(window.into(), cx);

        // The platform passes on a press that it gives to the view.
        cx.simulate_mouse_down(
            point(px(75.), px(75.)),
            MouseButton::Left,
            Modifiers::default(),
        );

        let presses = window
            .update(&mut cx, |view, _, _| std::mem::take(&mut view.presses))
            .unwrap();
        assert_eq!(presses, ["outside cover"]);
    }

    #[gpui::test]
    fn changing_the_background_repaints_the_native_view(cx: &mut TestAppContext) {
        let window = open_window(cx);
        cx.run_until_parked();

        window
            .update(cx, |view, _, cx| {
                view.native_view.set_background(rgb(0x00ff00), cx);
            })
            .unwrap();
        cx.run_until_parked();

        window
            .update(cx, |_, window, _| {
                let backgrounds = window
                    .rendered_frame
                    .scene
                    .native_views
                    .iter()
                    .map(|native_view| native_view.background)
                    .collect::<Vec<_>>();
                assert_eq!(backgrounds, [Hsla::from(rgb(0x00ff00))]);
            })
            .unwrap();
    }

    #[gpui::test]
    fn native_view_painted_twice_shows_where_it_was_painted_first(cx: &mut TestAppContext) {
        let window = open_window(cx);
        window
            .update(cx, |view, _, cx| {
                view.paints_native_view_twice = true;
                cx.notify();
            })
            .unwrap();
        cx.run_until_parked();

        window
            .update(cx, |_, window, _| {
                let bounds = window
                    .rendered_frame
                    .scene
                    .native_views
                    .iter()
                    .map(|native_view| native_view.bounds.origin.x)
                    .collect::<Vec<_>>();
                assert_eq!(bounds, [ScaledPixels::from(0.)]);
            })
            .unwrap();
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

    #[gpui::test]
    fn native_view_shows_nothing_in_another_window(cx: &mut TestAppContext) {
        let window = open_window(cx);
        let native_view = window
            .update(cx, |view, _, _| view.native_view.clone())
            .unwrap();
        let other_window = cx.add_window(|_, cx| TestView {
            native_view,
            other_focus: cx.focus_handle(),
            actions: Vec::new(),
            presses: Vec::new(),
            paints_native_view_twice: false,
            unmark_count: Rc::default(),
        });
        cx.run_until_parked();

        other_window
            .update(cx, |_, window, _| {
                assert!(window.rendered_frame.scene.native_views.is_empty());
            })
            .unwrap();
        window
            .update(cx, |_, window, _| {
                assert_eq!(window.rendered_frame.scene.native_views.len(), 1);
            })
            .unwrap();
    }

    #[gpui::test]
    fn focusing_a_native_view_ends_a_composition(cx: &mut TestAppContext) {
        let window = open_window(cx);
        window
            .update(cx, |view, window, cx| window.focus(&view.other_focus, cx))
            .unwrap();
        cx.run_until_parked();
        let unmark_count = window
            .update(cx, |view, _, _| view.unmark_count.clone())
            .unwrap();

        window
            .update(cx, |view, window, cx| {
                window.focus(&view.native_view.focus_handle(cx), cx);
            })
            .unwrap();
        assert_eq!(unmark_count.get(), 1);
    }

    actions!(native_view_tests, [Reserved, Ordinary]);

    struct TestView {
        native_view: NativeView,
        other_focus: FocusHandle,
        actions: Vec<&'static str>,
        presses: Vec<&'static str>,
        paints_native_view_twice: bool,
        /// How many times the input handler of the "cover" element was told to
        /// end its composition.
        unmark_count: Rc<Cell<usize>>,
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .on_action(cx.listener(|this, _: &Reserved, _, _| this.actions.push("reserved")))
                .on_action(cx.listener(|this, _: &Ordinary, _, _| this.actions.push("ordinary")))
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size(px(100.))
                        .bg(rgb(0xff0000))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, _| this.presses.push("behind view")),
                        )
                        .child(self.native_view.clone()),
                )
                .when(self.paints_native_view_twice, |this| {
                    this.child(
                        div()
                            .absolute()
                            .top_0()
                            .left(px(200.))
                            .size(px(100.))
                            .child(self.native_view.clone()),
                    )
                })
                .child(
                    div()
                        .id("cover")
                        .track_focus(&self.other_focus)
                        .absolute()
                        .top_0()
                        .left_0()
                        .size(px(50.))
                        .occlude()
                        .bg(rgb(0x0000ff))
                        .on_mouse_down_out(
                            cx.listener(|this, _, _, _| this.presses.push("outside cover")),
                        )
                        .child({
                            let focus = self.other_focus.clone();
                            let unmark_count = self.unmark_count.clone();
                            canvas(
                                |_, _, _| {},
                                move |_, _, window, cx| {
                                    window.handle_input(&focus, TestInput { unmark_count }, cx);
                                },
                            )
                            .size_full()
                        }),
                )
        }
    }

    struct TestInput {
        unmark_count: Rc<Cell<usize>>,
    }

    impl InputHandler for TestInput {
        fn selected_text_range(
            &mut self,
            _: bool,
            _: &mut Window,
            _: &mut App,
        ) -> Option<UTF16Selection> {
            None
        }

        fn marked_text_range(&mut self, _: &mut Window, _: &mut App) -> Option<Range<usize>> {
            None
        }

        fn text_for_range(
            &mut self,
            _: Range<usize>,
            _: &mut Option<Range<usize>>,
            _: &mut Window,
            _: &mut App,
        ) -> Option<String> {
            None
        }

        fn replace_text_in_range(
            &mut self,
            _: Option<Range<usize>>,
            _: &str,
            _: &mut Window,
            _: &mut App,
        ) {
        }

        fn replace_and_mark_text_in_range(
            &mut self,
            _: Option<Range<usize>>,
            _: &str,
            _: Option<Range<usize>>,
            _: &mut Window,
            _: &mut App,
        ) {
        }

        fn unmark_text(&mut self, _: &mut Window, _: &mut App) {
            self.unmark_count.set(self.unmark_count.get() + 1);
        }

        fn bounds_for_range(
            &mut self,
            _: Range<usize>,
            _: &mut Window,
            _: &mut App,
        ) -> Option<Bounds<Pixels>> {
            None
        }

        fn character_index_for_point(
            &mut self,
            _: Point<Pixels>,
            _: &mut Window,
            _: &mut App,
        ) -> Option<usize> {
            None
        }
    }

    fn open_window(cx: &mut TestAppContext) -> WindowHandle<TestView> {
        cx.add_window(|window, cx| TestView {
            native_view: new_native_view(window, cx),
            other_focus: cx.focus_handle(),
            actions: Vec::new(),
            presses: Vec::new(),
            paints_native_view_twice: false,
            unmark_count: Rc::default(),
        })
    }

    fn new_native_view(window: &mut Window, cx: &mut App) -> NativeView {
        // SAFETY: The test platform never reads the view it is given.
        unsafe { NativeView::new(NonNull::dangling(), window, cx) }
            .expect("the test platform attaches native views")
    }
}
