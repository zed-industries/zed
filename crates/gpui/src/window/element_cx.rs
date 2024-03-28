//! The element context is the main interface for interacting with the frame during a paint.
//!
//! Elements are hierarchical and with a few exceptions the context accumulates state in a stack
//! as it processes all of the elements in the frame. The methods that interact with this stack
//! are generally marked with `with_*`, and take a callback to denote the region of code that
//! should be executed with that state.
//!
//! The other main interface is the `paint_*` family of methods, which push basic drawing commands
//! to the GPU. Everything in a GPUI app is drawn with these methods.
//!
//! There are also several internal methods that GPUI uses, such as [`ElementContext::with_element_state`]
//! to call the paint and layout methods on elements. These have been included as they're often useful
//! for taking manual control of the layouting or painting of specialized elements.

use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut, Cow},
    mem,
    ops::Range,
    rc::Rc,
    sync::Arc,
};

use anyhow::Result;
use collections::FxHashMap;
use derive_more::{Deref, DerefMut};
#[cfg(target_os = "macos")]
use media::core_video::CVImageBuffer;
use smallvec::SmallVec;

use crate::{
    prelude::*, size, AnyElement, AnyTooltip, AppContext, AvailableSpace, Bounds, BoxShadow,
    ContentMask, Corners, CursorStyle, DevicePixels, DispatchNodeId, DispatchPhase, DispatchTree,
    DrawPhase, ElementId, ElementStateBox, EntityId, FocusHandle, FocusId, FontId, GlobalElementId,
    GlyphId, Hsla, ImageData, InputHandler, IsZero, KeyContext, KeyEvent, LayoutId,
    LineLayoutIndex, ModifiersChangedEvent, MonochromeSprite, MouseEvent, PaintQuad, Path, Pixels,
    PlatformInputHandler, Point, PolychromeSprite, Quad, RenderGlyphParams, RenderImageParams,
    RenderSvgParams, Scene, Shadow, SharedString, Size, StrikethroughStyle, Style,
    TextStyleRefinement, TransformationMatrix, Underline, UnderlineStyle, Window, WindowContext,
    SUBPIXEL_VARIANTS,
};

pub(crate) type AnyMouseListener =
    Box<dyn FnMut(&dyn Any, DispatchPhase, &mut ElementContext) + 'static>;

#[derive(Clone)]
pub(crate) struct CursorStyleRequest {
    pub(crate) hitbox_id: HitboxId,
    pub(crate) style: CursorStyle,
}

/// An identifier for a [Hitbox].
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct HitboxId(usize);

impl HitboxId {
    /// Checks if the hitbox with this id is currently hovered.
    pub fn is_hovered(&self, cx: &WindowContext) -> bool {
        cx.window.mouse_hit_test.0.contains(self)
    }
}

/// A rectangular region that potentially blocks hitboxes inserted prior.
/// See [ElementContext::insert_hitbox] for more details.
#[derive(Clone, Debug, Deref)]
pub struct Hitbox {
    /// A unique identifier for the hitbox
    pub id: HitboxId,
    /// The bounds of the hitbox
    #[deref]
    pub bounds: Bounds<Pixels>,
    /// Whether the hitbox occludes other hitboxes inserted prior.
    pub opaque: bool,
}

impl Hitbox {
    /// Checks if the hitbox is currently hovered.
    pub fn is_hovered(&self, cx: &WindowContext) -> bool {
        self.id.is_hovered(cx)
    }
}

#[derive(Default, Eq, PartialEq)]
pub(crate) struct HitTest(SmallVec<[HitboxId; 8]>);

pub(crate) struct DeferredDraw {
    priority: usize,
    parent_node: DispatchNodeId,
    element_id_stack: GlobalElementId,
    text_style_stack: Vec<TextStyleRefinement>,
    element: Option<AnyElement>,
    absolute_offset: Point<Pixels>,
    layout_range: Range<AfterLayoutIndex>,
    paint_range: Range<PaintIndex>,
}

pub(crate) struct Frame {
    pub(crate) focus: Option<FocusId>,
    pub(crate) window_active: bool,
    pub(crate) element_states: FxHashMap<(GlobalElementId, TypeId), ElementStateBox>,
    accessed_element_states: Vec<(GlobalElementId, TypeId)>,
    pub(crate) mouse_listeners: Vec<Option<AnyMouseListener>>,
    pub(crate) dispatch_tree: DispatchTree,
    pub(crate) scene: Scene,
    pub(crate) hitboxes: Vec<Hitbox>,
    pub(crate) deferred_draws: Vec<DeferredDraw>,
    pub(crate) content_mask_stack: Vec<ContentMask<Pixels>>,
    pub(crate) element_offset_stack: Vec<Point<Pixels>>,
    pub(crate) input_handlers: Vec<Option<PlatformInputHandler>>,
    pub(crate) tooltip_requests: Vec<Option<AnyTooltip>>,
    pub(crate) cursor_styles: Vec<CursorStyleRequest>,
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) debug_bounds: FxHashMap<String, Bounds<Pixels>>,
}

#[derive(Clone, Default)]
pub(crate) struct AfterLayoutIndex {
    hitboxes_index: usize,
    tooltips_index: usize,
    deferred_draws_index: usize,
    dispatch_tree_index: usize,
    accessed_element_states_index: usize,
    line_layout_index: LineLayoutIndex,
}

#[derive(Clone, Default)]
pub(crate) struct PaintIndex {
    scene_index: usize,
    mouse_listeners_index: usize,
    input_handlers_index: usize,
    cursor_styles_index: usize,
    accessed_element_states_index: usize,
    line_layout_index: LineLayoutIndex,
}

impl Frame {
    pub(crate) fn new(dispatch_tree: DispatchTree) -> Self {
        Frame {
            focus: None,
            window_active: false,
            element_states: FxHashMap::default(),
            accessed_element_states: Vec::new(),
            mouse_listeners: Vec::new(),
            dispatch_tree,
            scene: Scene::default(),
            hitboxes: Vec::new(),
            deferred_draws: Vec::new(),
            content_mask_stack: Vec::new(),
            element_offset_stack: Vec::new(),
            input_handlers: Vec::new(),
            tooltip_requests: Vec::new(),
            cursor_styles: Vec::new(),

            #[cfg(any(test, feature = "test-support"))]
            debug_bounds: FxHashMap::default(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.element_states.clear();
        self.accessed_element_states.clear();
        self.mouse_listeners.clear();
        self.dispatch_tree.clear();
        self.scene.clear();
        self.input_handlers.clear();
        self.tooltip_requests.clear();
        self.cursor_styles.clear();
        self.hitboxes.clear();
        self.deferred_draws.clear();
    }

    pub(crate) fn hit_test(&self, position: Point<Pixels>) -> HitTest {
        let mut hit_test = HitTest::default();
        for hitbox in self.hitboxes.iter().rev() {
            if hitbox.bounds.contains(&position) {
                hit_test.0.push(hitbox.id);
                if hitbox.opaque {
                    break;
                }
            }
        }
        hit_test
    }

    pub(crate) fn focus_path(&self) -> SmallVec<[FocusId; 8]> {
        self.focus
            .map(|focus_id| self.dispatch_tree.focus_path(focus_id))
            .unwrap_or_default()
    }

    pub(crate) fn finish(&mut self, prev_frame: &mut Self) {
        for element_state_key in &self.accessed_element_states {
            if let Some(element_state) = prev_frame.element_states.remove(element_state_key) {
                self.element_states
                    .insert(element_state_key.clone(), element_state);
            }
        }

        self.scene.finish();
    }
}

/// This context is used for assisting in the implementation of the element trait
#[derive(Deref, DerefMut)]
pub struct ElementContext<'a> {
    pub(crate) cx: WindowContext<'a>,
}

impl<'a> WindowContext<'a> {
    /// Convert this window context into an ElementContext in this callback.
    /// If you need to use this method, you're probably intermixing the imperative
    /// and declarative APIs, which is not recommended.
    pub fn with_element_context<R>(&mut self, f: impl FnOnce(&mut ElementContext) -> R) -> R {
        f(&mut ElementContext {
            cx: WindowContext::new(self.app, self.window),
        })
    }
}

impl<'a> Borrow<AppContext> for ElementContext<'a> {
    fn borrow(&self) -> &AppContext {
        self.cx.app
    }
}

impl<'a> BorrowMut<AppContext> for ElementContext<'a> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        self.cx.borrow_mut()
    }
}

impl<'a> Borrow<WindowContext<'a>> for ElementContext<'a> {
    fn borrow(&self) -> &WindowContext<'a> {
        &self.cx
    }
}

impl<'a> BorrowMut<WindowContext<'a>> for ElementContext<'a> {
    fn borrow_mut(&mut self) -> &mut WindowContext<'a> {
        &mut self.cx
    }
}

impl<'a> Borrow<Window> for ElementContext<'a> {
    fn borrow(&self) -> &Window {
        self.cx.window
    }
}

impl<'a> BorrowMut<Window> for ElementContext<'a> {
    fn borrow_mut(&mut self) -> &mut Window {
        self.cx.borrow_mut()
    }
}

impl<'a> Context for ElementContext<'a> {
    type Result<T> = <WindowContext<'a> as Context>::Result<T>;

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut crate::ModelContext<'_, T>) -> T,
    ) -> Self::Result<crate::Model<T>> {
        self.cx.new_model(build_model)
    }

    fn update_model<T, R>(
        &mut self,
        handle: &crate::Model<T>,
        update: impl FnOnce(&mut T, &mut crate::ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.cx.update_model(handle, update)
    }

    fn read_model<T, R>(
        &self,
        handle: &crate::Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.cx.read_model(handle, read)
    }

    fn update_window<T, F>(&mut self, window: crate::AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(crate::AnyView, &mut WindowContext<'_>) -> T,
    {
        self.cx.update_window(window, f)
    }

    fn read_window<T, R>(
        &self,
        window: &crate::WindowHandle<T>,
        read: impl FnOnce(crate::View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.cx.read_window(window, read)
    }
}

impl<'a> VisualContext for ElementContext<'a> {
    fn new_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut crate::ViewContext<'_, V>) -> V,
    ) -> Self::Result<crate::View<V>>
    where
        V: 'static + Render,
    {
        self.cx.new_view(build_view)
    }

    fn update_view<V: 'static, R>(
        &mut self,
        view: &crate::View<V>,
        update: impl FnOnce(&mut V, &mut crate::ViewContext<'_, V>) -> R,
    ) -> Self::Result<R> {
        self.cx.update_view(view, update)
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut crate::ViewContext<'_, V>) -> V,
    ) -> Self::Result<crate::View<V>>
    where
        V: 'static + Render,
    {
        self.cx.replace_root_view(build_view)
    }

    fn focus_view<V>(&mut self, view: &crate::View<V>) -> Self::Result<()>
    where
        V: crate::FocusableView,
    {
        self.cx.focus_view(view)
    }

    fn dismiss_view<V>(&mut self, view: &crate::View<V>) -> Self::Result<()>
    where
        V: crate::ManagedView,
    {
        self.cx.dismiss_view(view)
    }
}

impl<'a> ElementContext<'a> {
    pub(crate) fn draw_roots(&mut self) {
        self.window.draw_phase = DrawPhase::Layout;

        // Layout all root elements.
        let mut root_element = self.window.root_view.as_ref().unwrap().clone().into_any();
        root_element.layout(Point::default(), self.window.viewport_size.into(), self);

        let mut sorted_deferred_draws =
            (0..self.window.next_frame.deferred_draws.len()).collect::<SmallVec<[_; 8]>>();
        sorted_deferred_draws.sort_by_key(|ix| self.window.next_frame.deferred_draws[*ix].priority);
        self.layout_deferred_draws(&sorted_deferred_draws);

        let mut prompt_element = None;
        let mut active_drag_element = None;
        let mut tooltip_element = None;
        if let Some(prompt) = self.window.prompt.take() {
            let mut element = prompt.view.any_view().into_any();
            element.layout(Point::default(), self.window.viewport_size.into(), self);
            prompt_element = Some(element);
            self.window.prompt = Some(prompt);
        } else if let Some(active_drag) = self.app.active_drag.take() {
            let mut element = active_drag.view.clone().into_any();
            let offset = self.mouse_position() - active_drag.cursor_offset;
            element.layout(offset, AvailableSpace::min_size(), self);
            active_drag_element = Some(element);
            self.app.active_drag = Some(active_drag);
        } else if let Some(tooltip_request) =
            self.window.next_frame.tooltip_requests.last().cloned()
        {
            let tooltip_request = tooltip_request.unwrap();
            let mut element = tooltip_request.view.clone().into_any();
            let offset = tooltip_request.cursor_offset;
            element.layout(offset, AvailableSpace::min_size(), self);
            tooltip_element = Some(element);
        }

        self.window.mouse_hit_test = self.window.next_frame.hit_test(self.window.mouse_position);

        // Now actually paint the elements.
        self.window.draw_phase = DrawPhase::Paint;
        root_element.paint(self);

        self.paint_deferred_draws(&sorted_deferred_draws);

        if let Some(mut prompt_element) = prompt_element {
            prompt_element.paint(self)
        } else if let Some(mut drag_element) = active_drag_element {
            drag_element.paint(self);
        } else if let Some(mut tooltip_element) = tooltip_element {
            tooltip_element.paint(self);
        }
    }

    fn layout_deferred_draws(&mut self, deferred_draw_indices: &[usize]) {
        assert_eq!(self.window.element_id_stack.len(), 0);

        let mut deferred_draws = mem::take(&mut self.window.next_frame.deferred_draws);
        for deferred_draw_ix in deferred_draw_indices {
            let deferred_draw = &mut deferred_draws[*deferred_draw_ix];
            self.window.element_id_stack = deferred_draw.element_id_stack.clone();
            self.window.text_style_stack = deferred_draw.text_style_stack.clone();
            self.window
                .next_frame
                .dispatch_tree
                .set_active_node(deferred_draw.parent_node);

            let layout_start = self.after_layout_index();
            if let Some(element) = deferred_draw.element.as_mut() {
                self.with_absolute_element_offset(deferred_draw.absolute_offset, |cx| {
                    element.after_layout(cx)
                });
            } else {
                self.reuse_after_layout(deferred_draw.layout_range.clone());
            }
            let layout_end = self.after_layout_index();
            deferred_draw.layout_range = layout_start..layout_end;
        }
        assert_eq!(
            self.window.next_frame.deferred_draws.len(),
            0,
            "cannot call defer_draw during deferred drawing"
        );
        self.window.next_frame.deferred_draws = deferred_draws;
        self.window.element_id_stack.clear();
        self.window.text_style_stack.clear();
    }

    fn paint_deferred_draws(&mut self, deferred_draw_indices: &[usize]) {
        assert_eq!(self.window.element_id_stack.len(), 0);

        let mut deferred_draws = mem::take(&mut self.window.next_frame.deferred_draws);
        for deferred_draw_ix in deferred_draw_indices {
            let mut deferred_draw = &mut deferred_draws[*deferred_draw_ix];
            self.window.element_id_stack = deferred_draw.element_id_stack.clone();
            self.window
                .next_frame
                .dispatch_tree
                .set_active_node(deferred_draw.parent_node);

            let paint_start = self.paint_index();
            if let Some(element) = deferred_draw.element.as_mut() {
                element.paint(self);
            } else {
                self.reuse_paint(deferred_draw.paint_range.clone());
            }
            let paint_end = self.paint_index();
            deferred_draw.paint_range = paint_start..paint_end;
        }
        self.window.next_frame.deferred_draws = deferred_draws;
        self.window.element_id_stack.clear();
    }

    pub(crate) fn after_layout_index(&self) -> AfterLayoutIndex {
        AfterLayoutIndex {
            hitboxes_index: self.window.next_frame.hitboxes.len(),
            tooltips_index: self.window.next_frame.tooltip_requests.len(),
            deferred_draws_index: self.window.next_frame.deferred_draws.len(),
            dispatch_tree_index: self.window.next_frame.dispatch_tree.len(),
            accessed_element_states_index: self.window.next_frame.accessed_element_states.len(),
            line_layout_index: self.window.text_system.layout_index(),
        }
    }

    pub(crate) fn reuse_after_layout(&mut self, range: Range<AfterLayoutIndex>) {
        let window = &mut self.window;
        window.next_frame.hitboxes.extend(
            window.rendered_frame.hitboxes[range.start.hitboxes_index..range.end.hitboxes_index]
                .iter()
                .cloned(),
        );
        window.next_frame.tooltip_requests.extend(
            window.rendered_frame.tooltip_requests
                [range.start.tooltips_index..range.end.tooltips_index]
                .iter_mut()
                .map(|request| request.take()),
        );
        window.next_frame.accessed_element_states.extend(
            window.rendered_frame.accessed_element_states[range.start.accessed_element_states_index
                ..range.end.accessed_element_states_index]
                .iter()
                .cloned(),
        );
        window
            .text_system
            .reuse_layouts(range.start.line_layout_index..range.end.line_layout_index);

        let reused_subtree = window.next_frame.dispatch_tree.reuse_subtree(
            range.start.dispatch_tree_index..range.end.dispatch_tree_index,
            &mut window.rendered_frame.dispatch_tree,
        );
        window.next_frame.deferred_draws.extend(
            window.rendered_frame.deferred_draws
                [range.start.deferred_draws_index..range.end.deferred_draws_index]
                .iter()
                .map(|deferred_draw| DeferredDraw {
                    parent_node: reused_subtree.refresh_node_id(deferred_draw.parent_node),
                    element_id_stack: deferred_draw.element_id_stack.clone(),
                    text_style_stack: deferred_draw.text_style_stack.clone(),
                    priority: deferred_draw.priority,
                    element: None,
                    absolute_offset: deferred_draw.absolute_offset,
                    layout_range: deferred_draw.layout_range.clone(),
                    paint_range: deferred_draw.paint_range.clone(),
                }),
        );
    }

    pub(crate) fn paint_index(&self) -> PaintIndex {
        PaintIndex {
            scene_index: self.window.next_frame.scene.len(),
            mouse_listeners_index: self.window.next_frame.mouse_listeners.len(),
            input_handlers_index: self.window.next_frame.input_handlers.len(),
            cursor_styles_index: self.window.next_frame.cursor_styles.len(),
            accessed_element_states_index: self.window.next_frame.accessed_element_states.len(),
            line_layout_index: self.window.text_system.layout_index(),
        }
    }

    pub(crate) fn reuse_paint(&mut self, range: Range<PaintIndex>) {
        let window = &mut self.cx.window;

        window.next_frame.cursor_styles.extend(
            window.rendered_frame.cursor_styles
                [range.start.cursor_styles_index..range.end.cursor_styles_index]
                .iter()
                .cloned(),
        );
        window.next_frame.input_handlers.extend(
            window.rendered_frame.input_handlers
                [range.start.input_handlers_index..range.end.input_handlers_index]
                .iter_mut()
                .map(|handler| handler.take()),
        );
        window.next_frame.mouse_listeners.extend(
            window.rendered_frame.mouse_listeners
                [range.start.mouse_listeners_index..range.end.mouse_listeners_index]
                .iter_mut()
                .map(|listener| listener.take()),
        );
        window.next_frame.accessed_element_states.extend(
            window.rendered_frame.accessed_element_states[range.start.accessed_element_states_index
                ..range.end.accessed_element_states_index]
                .iter()
                .cloned(),
        );
        window
            .text_system
            .reuse_layouts(range.start.line_layout_index..range.end.line_layout_index);
        window.next_frame.scene.replay(
            range.start.scene_index..range.end.scene_index,
            &window.rendered_frame.scene,
        );
    }

    /// Push a text style onto the stack, and call a function with that style active.
    /// Use [`AppContext::text_style`] to get the current, combined text style.
    pub fn with_text_style<F, R>(&mut self, style: Option<TextStyleRefinement>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        if let Some(style) = style {
            self.window.text_style_stack.push(style);
            let result = f(self);
            self.window.text_style_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Updates the cursor style at the platform level.
    pub fn set_cursor_style(&mut self, style: CursorStyle, hitbox: &Hitbox) {
        self.window
            .next_frame
            .cursor_styles
            .push(CursorStyleRequest {
                hitbox_id: hitbox.id,
                style,
            });
    }

    /// Sets a tooltip to be rendered for the upcoming frame
    pub fn set_tooltip(&mut self, tooltip: AnyTooltip) {
        self.window.next_frame.tooltip_requests.push(Some(tooltip));
    }

    /// Pushes the given element id onto the global stack and invokes the given closure
    /// with a `GlobalElementId`, which disambiguates the given id in the context of its ancestor
    /// ids. Because elements are discarded and recreated on each frame, the `GlobalElementId` is
    /// used to associate state with identified elements across separate frames.
    pub fn with_element_id<R>(
        &mut self,
        id: Option<impl Into<ElementId>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(id) = id.map(Into::into) {
            let window = self.window_mut();
            window.element_id_stack.push(id);
            let result = f(self);
            let window: &mut Window = self.borrow_mut();
            window.element_id_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Invoke the given function with the given content mask after intersecting it
    /// with the current mask.
    pub fn with_content_mask<R>(
        &mut self,
        mask: Option<ContentMask<Pixels>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(mask) = mask {
            let mask = mask.intersect(&self.content_mask());
            self.window_mut().next_frame.content_mask_stack.push(mask);
            let result = f(self);
            self.window_mut().next_frame.content_mask_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Updates the global element offset relative to the current offset. This is used to implement
    /// scrolling.
    pub fn with_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if offset.is_zero() {
            return f(self);
        };

        let abs_offset = self.element_offset() + offset;
        self.with_absolute_element_offset(abs_offset, f)
    }

    /// Updates the global element offset based on the given offset. This is used to implement
    /// drag handles and other manual painting of elements.
    pub fn with_absolute_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.window_mut()
            .next_frame
            .element_offset_stack
            .push(offset);
        let result = f(self);
        self.window_mut().next_frame.element_offset_stack.pop();
        result
    }

    /// Obtain the current element offset.
    pub fn element_offset(&self) -> Point<Pixels> {
        self.window()
            .next_frame
            .element_offset_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    /// Obtain the current content mask.
    pub fn content_mask(&self) -> ContentMask<Pixels> {
        self.window()
            .next_frame
            .content_mask_stack
            .last()
            .cloned()
            .unwrap_or_else(|| ContentMask {
                bounds: Bounds {
                    origin: Point::default(),
                    size: self.window().viewport_size,
                },
            })
    }

    /// The size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    pub fn rem_size(&self) -> Pixels {
        self.window().rem_size
    }

    /// Updates or initializes state for an element with the given id that lives across multiple
    /// frames. If an element with this ID existed in the rendered frame, its state will be passed
    /// to the given closure. The state returned by the closure will be stored so it can be referenced
    /// when drawing the next frame.
    pub fn with_element_state<S, R>(
        &mut self,
        element_id: Option<ElementId>,
        f: impl FnOnce(Option<Option<S>>, &mut Self) -> (R, Option<S>),
    ) -> R
    where
        S: 'static,
    {
        let id_is_none = element_id.is_none();
        self.with_element_id(element_id, |cx| {
            if id_is_none {
                let (result, state) = f(None, cx);
                debug_assert!(state.is_none(), "you must not return an element state when passing None for the element id");
                result
            } else {
                let global_id = cx.window().element_id_stack.clone();
                let key = (global_id, TypeId::of::<S>());
                cx.window.next_frame.accessed_element_states.push(key.clone());

                if let Some(any) = cx
                    .window_mut()
                    .next_frame
                    .element_states
                    .remove(&key)
                    .or_else(|| {
                        cx.window_mut()
                            .rendered_frame
                            .element_states
                            .remove(&key)
                    })
                {
                    let ElementStateBox {
                        inner,
                        #[cfg(debug_assertions)]
                        type_name
                    } = any;
                    // Using the extra inner option to avoid needing to reallocate a new box.
                    let mut state_box = inner
                        .downcast::<Option<S>>()
                        .map_err(|_| {
                            #[cfg(debug_assertions)]
                            {
                                anyhow::anyhow!(
                                    "invalid element state type for id, requested_type {:?}, actual type: {:?}",
                                    std::any::type_name::<S>(),
                                    type_name
                                )
                            }

                            #[cfg(not(debug_assertions))]
                            {
                                anyhow::anyhow!(
                                    "invalid element state type for id, requested_type {:?}",
                                    std::any::type_name::<S>(),
                                )
                            }
                        })
                        .unwrap();

                    // Actual: Option<AnyElement> <- View
                    // Requested: () <- AnyElement
                    let state = state_box
                        .take()
                        .expect("reentrant call to with_element_state for the same state type and element id");
                    let (result, state) = f(Some(Some(state)), cx);
                    state_box.replace(state.expect("you must return "));
                    cx.window_mut()
                        .next_frame
                        .element_states
                        .insert(key, ElementStateBox {
                            inner: state_box,
                            #[cfg(debug_assertions)]
                            type_name
                        });
                    result
                } else {
                    let (result, state) = f(Some(None), cx);
                    cx.window_mut()
                        .next_frame
                        .element_states
                        .insert(key,
                            ElementStateBox {
                                inner: Box::new(Some(state.expect("you must return Some<State> when you pass some element id"))),
                                #[cfg(debug_assertions)]
                                type_name: std::any::type_name::<S>()
                            }

                        );
                    result
                }
            }
        })
    }

    /// Defers the drawing of the given element, scheduling it to be painted on top of the currently-drawn tree
    /// at a later time. The `priority` parameter determines the drawing order relative to other deferred elements,
    /// with higher values being drawn on top.
    pub fn defer_draw(
        &mut self,
        element: AnyElement,
        absolute_offset: Point<Pixels>,
        priority: usize,
    ) {
        let window = &mut self.cx.window;
        assert_eq!(
            window.draw_phase,
            DrawPhase::Layout,
            "defer_draw can only be called during before_layout or after_layout"
        );
        let parent_node = window.next_frame.dispatch_tree.active_node_id().unwrap();
        window.next_frame.deferred_draws.push(DeferredDraw {
            parent_node,
            element_id_stack: window.element_id_stack.clone(),
            text_style_stack: window.text_style_stack.clone(),
            priority,
            element: Some(element),
            absolute_offset,
            layout_range: AfterLayoutIndex::default()..AfterLayoutIndex::default(),
            paint_range: PaintIndex::default()..PaintIndex::default(),
        });
    }

    /// Creates a new painting layer for the specified bounds. A "layer" is a batch
    /// of geometry that are non-overlapping and have the same draw order. This is typically used
    /// for performance reasons.
    pub fn paint_layer<R>(&mut self, bounds: Bounds<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let clipped_bounds = bounds.intersect(&content_mask.bounds);
        if !clipped_bounds.is_empty() {
            self.window
                .next_frame
                .scene
                .push_layer(clipped_bounds.scale(scale_factor));
        }

        let result = f(self);

        if !clipped_bounds.is_empty() {
            self.window.next_frame.scene.pop_layer();
        }

        result
    }

    /// Paint one or more drop shadows into the scene for the next frame at the current z-index.
    pub fn paint_shadows(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        shadows: &[BoxShadow],
    ) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        for shadow in shadows {
            let mut shadow_bounds = bounds;
            shadow_bounds.origin += shadow.offset;
            shadow_bounds.dilate(shadow.spread_radius);
            self.window.next_frame.scene.insert_primitive(Shadow {
                order: 0,
                blur_radius: shadow.blur_radius.scale(scale_factor),
                bounds: shadow_bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                corner_radii: corner_radii.scale(scale_factor),
                color: shadow.color,
            });
        }
    }

    /// Paint one or more quads into the scene for the next frame at the current stacking context.
    /// Quads are colored rectangular regions with an optional background, border, and corner radius.
    /// see [`fill`](crate::fill), [`outline`](crate::outline), and [`quad`](crate::quad) to construct this type.
    pub fn paint_quad(&mut self, quad: PaintQuad) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        self.window.next_frame.scene.insert_primitive(Quad {
            order: 0,
            pad: 0,
            bounds: quad.bounds.scale(scale_factor),
            content_mask: content_mask.scale(scale_factor),
            background: quad.background,
            border_color: quad.border_color,
            corner_radii: quad.corner_radii.scale(scale_factor),
            border_widths: quad.border_widths.scale(scale_factor),
        });
    }

    /// Paint the given `Path` into the scene for the next frame at the current z-index.
    pub fn paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Hsla>) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        path.content_mask = content_mask;
        path.color = color.into();
        self.window
            .next_frame
            .scene
            .insert_primitive(path.scale(scale_factor));
    }

    /// Paint an underline into the scene for the next frame at the current z-index.
    pub fn paint_underline(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &UnderlineStyle,
    ) {
        let scale_factor = self.scale_factor();
        let height = if style.wavy {
            style.thickness * 3.
        } else {
            style.thickness
        };
        let bounds = Bounds {
            origin,
            size: size(width, height),
        };
        let content_mask = self.content_mask();

        self.window.next_frame.scene.insert_primitive(Underline {
            order: 0,
            pad: 0,
            bounds: bounds.scale(scale_factor),
            content_mask: content_mask.scale(scale_factor),
            color: style.color.unwrap_or_default(),
            thickness: style.thickness.scale(scale_factor),
            wavy: style.wavy,
        });
    }

    /// Paint a strikethrough into the scene for the next frame at the current z-index.
    pub fn paint_strikethrough(
        &mut self,
        origin: Point<Pixels>,
        width: Pixels,
        style: &StrikethroughStyle,
    ) {
        let scale_factor = self.scale_factor();
        let height = style.thickness;
        let bounds = Bounds {
            origin,
            size: size(width, height),
        };
        let content_mask = self.content_mask();

        self.window.next_frame.scene.insert_primitive(Underline {
            order: 0,
            pad: 0,
            bounds: bounds.scale(scale_factor),
            content_mask: content_mask.scale(scale_factor),
            thickness: style.thickness.scale(scale_factor),
            color: style.color.unwrap_or_default(),
            wavy: false,
        });
    }

    /// Paints a monochrome (non-emoji) glyph into the scene for the next frame at the current z-index.
    ///
    /// The y component of the origin is the baseline of the glyph.
    /// You should generally prefer to use the [`ShapedLine::paint`](crate::ShapedLine::paint) or
    /// [`WrappedLine::paint`](crate::WrappedLine::paint) methods in the [`TextSystem`](crate::TextSystem).
    /// This method is only useful if you need to paint a single glyph that has already been shaped.
    pub fn paint_glyph(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
        color: Hsla,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let glyph_origin = origin.scale(scale_factor);
        let subpixel_variant = Point {
            x: (glyph_origin.x.0.fract() * SUBPIXEL_VARIANTS as f32).floor() as u8,
            y: (glyph_origin.y.0.fract() * SUBPIXEL_VARIANTS as f32).floor() as u8,
        };
        let params = RenderGlyphParams {
            font_id,
            glyph_id,
            font_size,
            subpixel_variant,
            scale_factor,
            is_emoji: false,
        };

        let raster_bounds = self.text_system().raster_bounds(&params)?;
        if !raster_bounds.is_zero() {
            let tile =
                self.window
                    .sprite_atlas
                    .get_or_insert_with(&params.clone().into(), &mut || {
                        let (size, bytes) = self.text_system().rasterize_glyph(&params)?;
                        Ok((size, Cow::Owned(bytes)))
                    })?;
            let bounds = Bounds {
                origin: glyph_origin.map(|px| px.floor()) + raster_bounds.origin.map(Into::into),
                size: tile.bounds.size.map(Into::into),
            };
            let content_mask = self.content_mask().scale(scale_factor);
            self.window
                .next_frame
                .scene
                .insert_primitive(MonochromeSprite {
                    order: 0,
                    pad: 0,
                    bounds,
                    content_mask,
                    color,
                    tile,
                    transformation: TransformationMatrix::unit(),
                });
        }
        Ok(())
    }

    /// Paints an emoji glyph into the scene for the next frame at the current z-index.
    ///
    /// The y component of the origin is the baseline of the glyph.
    /// You should generally prefer to use the [`ShapedLine::paint`](crate::ShapedLine::paint) or
    /// [`WrappedLine::paint`](crate::WrappedLine::paint) methods in the [`TextSystem`](crate::TextSystem).
    /// This method is only useful if you need to paint a single emoji that has already been shaped.
    pub fn paint_emoji(
        &mut self,
        origin: Point<Pixels>,
        font_id: FontId,
        glyph_id: GlyphId,
        font_size: Pixels,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let glyph_origin = origin.scale(scale_factor);
        let params = RenderGlyphParams {
            font_id,
            glyph_id,
            font_size,
            // We don't render emojis with subpixel variants.
            subpixel_variant: Default::default(),
            scale_factor,
            is_emoji: true,
        };

        let raster_bounds = self.text_system().raster_bounds(&params)?;
        if !raster_bounds.is_zero() {
            let tile =
                self.window
                    .sprite_atlas
                    .get_or_insert_with(&params.clone().into(), &mut || {
                        let (size, bytes) = self.text_system().rasterize_glyph(&params)?;
                        Ok((size, Cow::Owned(bytes)))
                    })?;
            let bounds = Bounds {
                origin: glyph_origin.map(|px| px.floor()) + raster_bounds.origin.map(Into::into),
                size: tile.bounds.size.map(Into::into),
            };
            let content_mask = self.content_mask().scale(scale_factor);

            self.window
                .next_frame
                .scene
                .insert_primitive(PolychromeSprite {
                    order: 0,
                    grayscale: false,
                    bounds,
                    corner_radii: Default::default(),
                    content_mask,
                    tile,
                });
        }
        Ok(())
    }

    /// Paint a monochrome SVG into the scene for the next frame at the current stacking context.
    pub fn paint_svg(
        &mut self,
        bounds: Bounds<Pixels>,
        path: SharedString,
        transformation: TransformationMatrix,
        color: Hsla,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        // Render the SVG at twice the size to get a higher quality result.
        let params = RenderSvgParams {
            path,
            size: bounds
                .size
                .map(|pixels| DevicePixels::from((pixels.0 * 2.).ceil() as i32)),
        };

        let tile =
            self.window
                .sprite_atlas
                .get_or_insert_with(&params.clone().into(), &mut || {
                    let bytes = self.svg_renderer.render(&params)?;
                    Ok((params.size, Cow::Owned(bytes)))
                })?;
        let content_mask = self.content_mask().scale(scale_factor);

        self.window
            .next_frame
            .scene
            .insert_primitive(MonochromeSprite {
                order: 0,
                pad: 0,
                bounds,
                content_mask,
                color,
                tile,
                transformation,
            });

        Ok(())
    }

    /// Paint an image into the scene for the next frame at the current z-index.
    pub fn paint_image(
        &mut self,
        bounds: Bounds<Pixels>,
        corner_radii: Corners<Pixels>,
        data: Arc<ImageData>,
        grayscale: bool,
    ) -> Result<()> {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let params = RenderImageParams { image_id: data.id };

        let tile = self
            .window
            .sprite_atlas
            .get_or_insert_with(&params.clone().into(), &mut || {
                Ok((data.size(), Cow::Borrowed(data.as_bytes())))
            })?;
        let content_mask = self.content_mask().scale(scale_factor);
        let corner_radii = corner_radii.scale(scale_factor);

        self.window
            .next_frame
            .scene
            .insert_primitive(PolychromeSprite {
                order: 0,
                grayscale,
                bounds,
                content_mask,
                corner_radii,
                tile,
            });
        Ok(())
    }

    /// Paint a surface into the scene for the next frame at the current z-index.
    #[cfg(target_os = "macos")]
    pub fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVImageBuffer) {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let content_mask = self.content_mask().scale(scale_factor);
        self.window
            .next_frame
            .scene
            .insert_primitive(crate::Surface {
                order: 0,
                bounds,
                content_mask,
                image_buffer,
            });
    }

    #[must_use]
    /// Add a node to the layout tree for the current frame. Takes the `Style` of the element for which
    /// layout is being requested, along with the layout ids of any children. This method is called during
    /// calls to the `Element::layout` trait method and enables any element to participate in layout.
    pub fn request_layout(
        &mut self,
        style: &Style,
        children: impl IntoIterator<Item = LayoutId>,
    ) -> LayoutId {
        self.app.layout_id_buffer.clear();
        self.app.layout_id_buffer.extend(children);
        let rem_size = self.rem_size();

        self.cx
            .window
            .layout_engine
            .as_mut()
            .unwrap()
            .before_layout(style, rem_size, &self.cx.app.layout_id_buffer)
    }

    /// Add a node to the layout tree for the current frame. Instead of taking a `Style` and children,
    /// this variant takes a function that is invoked during layout so you can use arbitrary logic to
    /// determine the element's size. One place this is used internally is when measuring text.
    ///
    /// The given closure is invoked at layout time with the known dimensions and available space and
    /// returns a `Size`.
    pub fn request_measured_layout<
        F: FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut WindowContext) -> Size<Pixels>
            + 'static,
    >(
        &mut self,
        style: Style,
        measure: F,
    ) -> LayoutId {
        let rem_size = self.rem_size();
        self.window
            .layout_engine
            .as_mut()
            .unwrap()
            .request_measured_layout(style, rem_size, measure)
    }

    /// Compute the layout for the given id within the given available space.
    /// This method is called for its side effect, typically by the framework prior to painting.
    /// After calling it, you can request the bounds of the given layout node id or any descendant.
    pub fn compute_layout(&mut self, layout_id: LayoutId, available_space: Size<AvailableSpace>) {
        let mut layout_engine = self.window.layout_engine.take().unwrap();
        layout_engine.compute_layout(layout_id, available_space, self);
        self.window.layout_engine = Some(layout_engine);
    }

    /// Obtain the bounds computed for the given LayoutId relative to the window. This method will usually be invoked by
    /// GPUI itself automatically in order to pass your element its `Bounds` automatically.
    pub fn layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels> {
        let mut bounds = self
            .window
            .layout_engine
            .as_mut()
            .unwrap()
            .layout_bounds(layout_id)
            .map(Into::into);
        bounds.origin += self.element_offset();
        bounds
    }

    /// This method should be called during `after_layout`. You can use
    /// the returned [Hitbox] during `paint` or in an event handler
    /// to determine whether the inserted hitbox was the topmost.
    pub fn insert_hitbox(&mut self, bounds: Bounds<Pixels>, opaque: bool) -> Hitbox {
        let content_mask = self.content_mask();
        let window = &mut self.window;
        let id = window.next_hitbox_id;
        window.next_hitbox_id.0 += 1;
        let hitbox = Hitbox {
            id,
            bounds: bounds.intersect(&content_mask.bounds),
            opaque,
        };
        window.next_frame.hitboxes.push(hitbox.clone());
        hitbox
    }

    /// Sets the key context for the current element. This context will be used to translate
    /// keybindings into actions.
    pub fn set_key_context(&mut self, context: KeyContext) {
        self.window
            .next_frame
            .dispatch_tree
            .set_key_context(context);
    }

    /// Sets the focus handle for the current element. This handle will be used to manage focus state
    /// and keyboard event dispatch for the element.
    pub fn set_focus_handle(&mut self, focus_handle: &FocusHandle) {
        self.window
            .next_frame
            .dispatch_tree
            .set_focus_id(focus_handle.id);
    }

    /// Sets the view id for the current element, which will be used to manage view caching.
    pub fn set_view_id(&mut self, view_id: EntityId) {
        self.window.next_frame.dispatch_tree.set_view_id(view_id);
    }

    /// Get the last view id for the current element
    pub fn parent_view_id(&mut self) -> Option<EntityId> {
        self.window.next_frame.dispatch_tree.parent_view_id()
    }

    /// Sets an input handler, such as [`ElementInputHandler`][element_input_handler], which interfaces with the
    /// platform to receive textual input with proper integration with concerns such
    /// as IME interactions. This handler will be active for the upcoming frame until the following frame is
    /// rendered.
    ///
    /// [element_input_handler]: crate::ElementInputHandler
    pub fn handle_input(&mut self, focus_handle: &FocusHandle, input_handler: impl InputHandler) {
        if focus_handle.is_focused(self) {
            let cx = self.to_async();
            self.window
                .next_frame
                .input_handlers
                .push(Some(PlatformInputHandler::new(cx, Box::new(input_handler))));
        }
    }

    /// Register a mouse event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    pub fn on_mouse_event<Event: MouseEvent>(
        &mut self,
        mut handler: impl FnMut(&Event, DispatchPhase, &mut ElementContext) + 'static,
    ) {
        self.window.next_frame.mouse_listeners.push(Some(Box::new(
            move |event: &dyn Any, phase: DispatchPhase, cx: &mut ElementContext<'_>| {
                if let Some(event) = event.downcast_ref() {
                    handler(event, phase, cx)
                }
            },
        )));
    }

    /// Register a key event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_key_event<Event: KeyEvent>(
        &mut self,
        listener: impl Fn(&Event, DispatchPhase, &mut ElementContext) + 'static,
    ) {
        self.window.next_frame.dispatch_tree.on_key_event(Rc::new(
            move |event: &dyn Any, phase, cx: &mut ElementContext<'_>| {
                if let Some(event) = event.downcast_ref::<Event>() {
                    listener(event, phase, cx)
                }
            },
        ));
    }

    /// Register a modifiers changed event listener on the window for the next frame.
    ///
    /// This is a fairly low-level method, so prefer using event handlers on elements unless you have
    /// a specific need to register a global listener.
    pub fn on_modifiers_changed(
        &mut self,
        listener: impl Fn(&ModifiersChangedEvent, &mut ElementContext) + 'static,
    ) {
        self.window
            .next_frame
            .dispatch_tree
            .on_modifiers_changed(Rc::new(
                move |event: &ModifiersChangedEvent, cx: &mut ElementContext<'_>| {
                    listener(event, cx)
                },
            ));
    }
}
