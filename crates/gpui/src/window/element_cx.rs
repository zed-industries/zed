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
    rc::Rc,
    sync::Arc,
};

use anyhow::Result;
use collections::{FxHashMap, FxHashSet};
use derive_more::{Deref, DerefMut};
#[cfg(target_os = "macos")]
use media::core_video::CVImageBuffer;
use smallvec::SmallVec;
use util::post_inc;

use crate::{
    prelude::*, size, AnyTooltip, AppContext, AvailableSpace, Bounds, BoxShadow, ContentMask,
    Corners, CursorStyle, DevicePixels, DispatchPhase, DispatchTree, ElementId, ElementStateBox,
    EntityId, FocusHandle, FocusId, FontId, GlobalElementId, GlyphId, Hsla, ImageData,
    InputHandler, IsZero, KeyContext, KeyEvent, LayoutId, MonochromeSprite, MouseEvent, PaintQuad,
    Path, Pixels, PlatformInputHandler, Point, PolychromeSprite, Quad, RenderGlyphParams,
    RenderImageParams, RenderSvgParams, Scene, Shadow, SharedString, Size, StackingContext,
    StackingOrder, StrikethroughStyle, Style, TextStyleRefinement, Underline, UnderlineStyle,
    Window, WindowContext, SUBPIXEL_VARIANTS,
};

type AnyMouseListener = Box<dyn FnMut(&dyn Any, DispatchPhase, &mut ElementContext) + 'static>;

pub(crate) struct RequestedInputHandler {
    pub(crate) view_id: EntityId,
    pub(crate) handler: Option<PlatformInputHandler>,
}

pub(crate) struct TooltipRequest {
    pub(crate) view_id: EntityId,
    pub(crate) tooltip: AnyTooltip,
}

#[derive(Clone)]
pub(crate) struct CursorStyleRequest {
    pub(crate) style: CursorStyle,
    stacking_order: StackingOrder,
}

pub(crate) struct Frame {
    pub(crate) focus: Option<FocusId>,
    pub(crate) window_active: bool,
    pub(crate) element_states: FxHashMap<GlobalElementId, ElementStateBox>,
    pub(crate) mouse_listeners: FxHashMap<TypeId, Vec<(StackingOrder, EntityId, AnyMouseListener)>>,
    pub(crate) dispatch_tree: DispatchTree,
    pub(crate) scene: Scene,
    pub(crate) depth_map: Vec<(StackingOrder, EntityId, Bounds<Pixels>)>,
    pub(crate) z_index_stack: StackingOrder,
    pub(crate) next_stacking_order_ids: Vec<u16>,
    pub(crate) next_root_z_index: u16,
    pub(crate) content_mask_stack: Vec<ContentMask<Pixels>>,
    pub(crate) element_offset_stack: Vec<Point<Pixels>>,
    pub(crate) requested_input_handler: Option<RequestedInputHandler>,
    pub(crate) tooltip_request: Option<TooltipRequest>,
    pub(crate) cursor_styles: FxHashMap<EntityId, CursorStyleRequest>,
    pub(crate) requested_cursor_style: Option<CursorStyleRequest>,
    pub(crate) view_stack: Vec<EntityId>,
    pub(crate) reused_views: FxHashSet<EntityId>,

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) debug_bounds: FxHashMap<String, Bounds<Pixels>>,
}

impl Frame {
    pub(crate) fn new(dispatch_tree: DispatchTree) -> Self {
        Frame {
            focus: None,
            window_active: false,
            element_states: FxHashMap::default(),
            mouse_listeners: FxHashMap::default(),
            dispatch_tree,
            scene: Scene::default(),
            depth_map: Vec::new(),
            z_index_stack: StackingOrder::default(),
            next_stacking_order_ids: vec![0],
            next_root_z_index: 0,
            content_mask_stack: Vec::new(),
            element_offset_stack: Vec::new(),
            requested_input_handler: None,
            tooltip_request: None,
            cursor_styles: FxHashMap::default(),
            requested_cursor_style: None,
            view_stack: Vec::new(),
            reused_views: FxHashSet::default(),

            #[cfg(any(test, feature = "test-support"))]
            debug_bounds: FxHashMap::default(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.element_states.clear();
        self.mouse_listeners.values_mut().for_each(Vec::clear);
        self.dispatch_tree.clear();
        self.depth_map.clear();
        self.next_stacking_order_ids = vec![0];
        self.next_root_z_index = 0;
        self.reused_views.clear();
        self.scene.clear();
        self.requested_input_handler.take();
        self.tooltip_request.take();
        self.cursor_styles.clear();
        self.requested_cursor_style.take();
        debug_assert_eq!(self.view_stack.len(), 0);
    }

    pub(crate) fn focus_path(&self) -> SmallVec<[FocusId; 8]> {
        self.focus
            .map(|focus_id| self.dispatch_tree.focus_path(focus_id))
            .unwrap_or_default()
    }

    pub(crate) fn finish(&mut self, prev_frame: &mut Self) {
        // Reuse mouse listeners that didn't change since the last frame.
        for (type_id, listeners) in &mut prev_frame.mouse_listeners {
            let next_listeners = self.mouse_listeners.entry(*type_id).or_default();
            for (order, view_id, listener) in listeners.drain(..) {
                if self.reused_views.contains(&view_id) {
                    next_listeners.push((order, view_id, listener));
                }
            }
        }

        // Reuse entries in the depth map that didn't change since the last frame.
        for (order, view_id, bounds) in prev_frame.depth_map.drain(..) {
            if self.reused_views.contains(&view_id) {
                match self
                    .depth_map
                    .binary_search_by(|(level, _, _)| order.cmp(level))
                {
                    Ok(i) | Err(i) => self.depth_map.insert(i, (order, view_id, bounds)),
                }
            }
        }

        // Retain element states for views that didn't change since the last frame.
        for (element_id, state) in prev_frame.element_states.drain() {
            if self.reused_views.contains(&state.parent_view_id) {
                self.element_states.entry(element_id).or_insert(state);
            }
        }

        // Reuse geometry that didn't change since the last frame.
        self.scene
            .reuse_views(&self.reused_views, &mut prev_frame.scene);
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
    pub(crate) fn reuse_view(&mut self, next_stacking_order_id: u16) {
        let view_id = self.parent_view_id();
        let grafted_view_ids = self
            .cx
            .window
            .next_frame
            .dispatch_tree
            .reuse_view(view_id, &mut self.cx.window.rendered_frame.dispatch_tree);
        for view_id in grafted_view_ids {
            assert!(self.window.next_frame.reused_views.insert(view_id));

            // Reuse the previous input handler requested during painting of the reused view.
            if self
                .window
                .rendered_frame
                .requested_input_handler
                .as_ref()
                .map_or(false, |requested| requested.view_id == view_id)
            {
                self.window.next_frame.requested_input_handler =
                    self.window.rendered_frame.requested_input_handler.take();
            }

            // Reuse the tooltip previously requested during painting of the reused view.
            if self
                .window
                .rendered_frame
                .tooltip_request
                .as_ref()
                .map_or(false, |requested| requested.view_id == view_id)
            {
                self.window.next_frame.tooltip_request =
                    self.window.rendered_frame.tooltip_request.take();
            }

            // Reuse the cursor styles previously requested during painting of the reused view.
            if let Some(cursor_style_request) =
                self.window.rendered_frame.cursor_styles.remove(&view_id)
            {
                self.set_cursor_style(
                    cursor_style_request.style,
                    cursor_style_request.stacking_order,
                );
            }
        }

        debug_assert!(
            next_stacking_order_id
                >= self
                    .window
                    .next_frame
                    .next_stacking_order_ids
                    .last()
                    .copied()
                    .unwrap()
        );
        *self
            .window
            .next_frame
            .next_stacking_order_ids
            .last_mut()
            .unwrap() = next_stacking_order_id;
    }

    /// Push a text style onto the stack, and call a function with that style active.
    /// Use [`AppContext::text_style`] to get the current, combined text style.
    pub fn with_text_style<F, R>(&mut self, style: Option<TextStyleRefinement>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        if let Some(style) = style {
            self.push_text_style(style);
            let result = f(self);
            self.pop_text_style();
            result
        } else {
            f(self)
        }
    }

    /// Updates the cursor style at the platform level.
    pub fn set_cursor_style(&mut self, style: CursorStyle, stacking_order: StackingOrder) {
        let view_id = self.parent_view_id();
        let style_request = CursorStyleRequest {
            style,
            stacking_order,
        };
        if self
            .window
            .next_frame
            .requested_cursor_style
            .as_ref()
            .map_or(true, |prev_style_request| {
                style_request.stacking_order >= prev_style_request.stacking_order
            })
        {
            self.window.next_frame.requested_cursor_style = Some(style_request.clone());
        }
        self.window
            .next_frame
            .cursor_styles
            .insert(view_id, style_request);
    }

    /// Sets a tooltip to be rendered for the upcoming frame
    pub fn set_tooltip(&mut self, tooltip: AnyTooltip) {
        let view_id = self.parent_view_id();
        self.window.next_frame.tooltip_request = Some(TooltipRequest { view_id, tooltip });
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

    /// Invoke the given function with the content mask reset to that
    /// of the window.
    pub fn break_content_mask<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let mask = ContentMask {
            bounds: Bounds {
                origin: Point::default(),
                size: self.window().viewport_size,
            },
        };

        let new_root_z_index = post_inc(&mut self.window_mut().next_frame.next_root_z_index);
        let new_stacking_order_id = post_inc(
            self.window_mut()
                .next_frame
                .next_stacking_order_ids
                .last_mut()
                .unwrap(),
        );
        let new_context = StackingContext {
            z_index: new_root_z_index,
            id: new_stacking_order_id,
        };

        let old_stacking_order = mem::take(&mut self.window_mut().next_frame.z_index_stack);

        self.window_mut().next_frame.z_index_stack.push(new_context);
        self.window_mut().next_frame.content_mask_stack.push(mask);
        let result = f(self);
        self.window_mut().next_frame.content_mask_stack.pop();
        self.window_mut().next_frame.z_index_stack = old_stacking_order;

        result
    }

    /// Called during painting to invoke the given closure in a new stacking context. The given
    /// z-index is interpreted relative to the previous call to `stack`.
    pub fn with_z_index<R>(&mut self, z_index: u16, f: impl FnOnce(&mut Self) -> R) -> R {
        let new_stacking_order_id = post_inc(
            self.window_mut()
                .next_frame
                .next_stacking_order_ids
                .last_mut()
                .unwrap(),
        );
        self.window_mut().next_frame.next_stacking_order_ids.push(0);
        let new_context = StackingContext {
            z_index,
            id: new_stacking_order_id,
        };

        self.window_mut().next_frame.z_index_stack.push(new_context);
        let result = f(self);
        self.window_mut().next_frame.z_index_stack.pop();

        self.window_mut().next_frame.next_stacking_order_ids.pop();

        result
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
        id: ElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static,
    {
        self.with_element_id(Some(id), |cx| {
                let global_id = cx.window().element_id_stack.clone();

                if let Some(any) = cx
                    .window_mut()
                    .next_frame
                    .element_states
                    .remove(&global_id)
                    .or_else(|| {
                        cx.window_mut()
                            .rendered_frame
                            .element_states
                            .remove(&global_id)
                    })
                {
                    let ElementStateBox {
                        inner,
                        parent_view_id,
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
                        .expect("element state is already on the stack");
                    let (result, state) = f(Some(state), cx);
                    state_box.replace(state);
                    cx.window_mut()
                        .next_frame
                        .element_states
                        .insert(global_id, ElementStateBox {
                            inner: state_box,
                            parent_view_id,
                            #[cfg(debug_assertions)]
                            type_name
                        });
                    result
                } else {
                    let (result, state) = f(None, cx);
                    let parent_view_id = cx.parent_view_id();
                    cx.window_mut()
                        .next_frame
                        .element_states
                        .insert(global_id,
                            ElementStateBox {
                                inner: Box::new(Some(state)),
                                parent_view_id,
                                #[cfg(debug_assertions)]
                                type_name: std::any::type_name::<S>()
                            }

                        );
                    result
                }
            })
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
        let view_id = self.parent_view_id();
        let window = &mut *self.window;
        for shadow in shadows {
            let mut shadow_bounds = bounds;
            shadow_bounds.origin += shadow.offset;
            shadow_bounds.dilate(shadow.spread_radius);
            window.next_frame.scene.insert(
                &window.next_frame.z_index_stack,
                Shadow {
                    view_id: view_id.into(),
                    layer_id: 0,
                    order: 0,
                    bounds: shadow_bounds.scale(scale_factor),
                    content_mask: content_mask.scale(scale_factor),
                    corner_radii: corner_radii.scale(scale_factor),
                    color: shadow.color,
                    blur_radius: shadow.blur_radius.scale(scale_factor),
                    pad: 0,
                },
            );
        }
    }

    /// Paint one or more quads into the scene for the next frame at the current stacking context.
    /// Quads are colored rectangular regions with an optional background, border, and corner radius.
    /// see [`fill`](crate::fill), [`outline`](crate::outline), and [`quad`](crate::quad) to construct this type.
    pub fn paint_quad(&mut self, quad: PaintQuad) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            Quad {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds: quad.bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                background: quad.background,
                border_color: quad.border_color,
                corner_radii: quad.corner_radii.scale(scale_factor),
                border_widths: quad.border_widths.scale(scale_factor),
            },
        );
    }

    /// Paint the given `Path` into the scene for the next frame at the current z-index.
    pub fn paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Hsla>) {
        let scale_factor = self.scale_factor();
        let content_mask = self.content_mask();
        let view_id = self.parent_view_id();

        path.content_mask = content_mask;
        path.color = color.into();
        path.view_id = view_id.into();
        let window = &mut *self.window;
        window
            .next_frame
            .scene
            .insert(&window.next_frame.z_index_stack, path.scale(scale_factor));
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
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            Underline {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds: bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                color: style.color.unwrap_or_default(),
                thickness: style.thickness.scale(scale_factor),
                wavy: style.wavy,
            },
        );
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
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            Underline {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds: bounds.scale(scale_factor),
                content_mask: content_mask.scale(scale_factor),
                thickness: style.thickness.scale(scale_factor),
                color: style.color.unwrap_or_default(),
                wavy: false,
            },
        );
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
            let view_id = self.parent_view_id();
            let window = &mut *self.window;
            window.next_frame.scene.insert(
                &window.next_frame.z_index_stack,
                MonochromeSprite {
                    view_id: view_id.into(),
                    layer_id: 0,
                    order: 0,
                    bounds,
                    content_mask,
                    color,
                    tile,
                },
            );
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
            let view_id = self.parent_view_id();
            let window = &mut *self.window;

            window.next_frame.scene.insert(
                &window.next_frame.z_index_stack,
                PolychromeSprite {
                    view_id: view_id.into(),
                    layer_id: 0,
                    order: 0,
                    bounds,
                    corner_radii: Default::default(),
                    content_mask,
                    tile,
                    grayscale: false,
                    pad: 0,
                },
            );
        }
        Ok(())
    }

    /// Paint a monochrome SVG into the scene for the next frame at the current stacking context.
    pub fn paint_svg(
        &mut self,
        bounds: Bounds<Pixels>,
        path: SharedString,
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
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            MonochromeSprite {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds,
                content_mask,
                color,
                tile,
            },
        );

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
        let view_id = self.parent_view_id();

        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            PolychromeSprite {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds,
                content_mask,
                corner_radii,
                tile,
                grayscale,
                pad: 0,
            },
        );
        Ok(())
    }

    /// Paint a surface into the scene for the next frame at the current z-index.
    #[cfg(target_os = "macos")]
    pub fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVImageBuffer) {
        let scale_factor = self.scale_factor();
        let bounds = bounds.scale(scale_factor);
        let content_mask = self.content_mask().scale(scale_factor);
        let view_id = self.parent_view_id();
        let window = &mut *self.window;
        window.next_frame.scene.insert(
            &window.next_frame.z_index_stack,
            crate::Surface {
                view_id: view_id.into(),
                layer_id: 0,
                order: 0,
                bounds,
                content_mask,
                image_buffer,
            },
        );
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
            .request_layout(style, rem_size, &self.cx.app.layout_id_buffer)
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

    pub(crate) fn layout_style(&self, layout_id: LayoutId) -> Option<&Style> {
        self.window
            .layout_engine
            .as_ref()
            .unwrap()
            .requested_style(layout_id)
    }

    /// Called during painting to track which z-index is on top at each pixel position
    pub fn add_opaque_layer(&mut self, bounds: Bounds<Pixels>) {
        let stacking_order = self.window.next_frame.z_index_stack.clone();
        let view_id = self.parent_view_id();
        let depth_map = &mut self.window.next_frame.depth_map;
        match depth_map.binary_search_by(|(level, _, _)| stacking_order.cmp(level)) {
            Ok(i) | Err(i) => depth_map.insert(i, (stacking_order, view_id, bounds)),
        }
    }

    /// Invoke the given function with the given focus handle present on the key dispatch stack.
    /// If you want an element to participate in key dispatch, use this method to push its key context and focus handle into the stack during paint.
    pub fn with_key_dispatch<R>(
        &mut self,
        context: Option<KeyContext>,
        focus_handle: Option<FocusHandle>,
        f: impl FnOnce(Option<FocusHandle>, &mut Self) -> R,
    ) -> R {
        let window = &mut self.window;
        let focus_id = focus_handle.as_ref().map(|handle| handle.id);
        window
            .next_frame
            .dispatch_tree
            .push_node(context.clone(), focus_id, None);

        let result = f(focus_handle, self);

        self.window.next_frame.dispatch_tree.pop_node();

        result
    }

    /// Invoke the given function with the given view id present on the view stack.
    /// This is a fairly low-level method used to layout views.
    pub fn with_view_id<R>(&mut self, view_id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R {
        let text_system = self.text_system().clone();
        text_system.with_view(view_id, || {
            if self.window.next_frame.view_stack.last() == Some(&view_id) {
                f(self)
            } else {
                self.window.next_frame.view_stack.push(view_id);
                let result = f(self);
                self.window.next_frame.view_stack.pop();
                result
            }
        })
    }

    /// Invoke the given function with the given view id present on the view stack.
    /// This is a fairly low-level method used to paint views.
    pub fn paint_view<R>(&mut self, view_id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R {
        let text_system = self.text_system().clone();
        text_system.with_view(view_id, || {
            if self.window.next_frame.view_stack.last() == Some(&view_id) {
                f(self)
            } else {
                self.window.next_frame.view_stack.push(view_id);
                self.window
                    .next_frame
                    .dispatch_tree
                    .push_node(None, None, Some(view_id));
                let result = f(self);
                self.window.next_frame.dispatch_tree.pop_node();
                self.window.next_frame.view_stack.pop();
                result
            }
        })
    }

    /// Sets an input handler, such as [`ElementInputHandler`][element_input_handler], which interfaces with the
    /// platform to receive textual input with proper integration with concerns such
    /// as IME interactions. This handler will be active for the upcoming frame until the following frame is
    /// rendered.
    ///
    /// [element_input_handler]: crate::ElementInputHandler
    pub fn handle_input(&mut self, focus_handle: &FocusHandle, input_handler: impl InputHandler) {
        if focus_handle.is_focused(self) {
            let view_id = self.parent_view_id();
            self.window.next_frame.requested_input_handler = Some(RequestedInputHandler {
                view_id,
                handler: Some(PlatformInputHandler::new(
                    self.to_async(),
                    Box::new(input_handler),
                )),
            })
        }
    }

    /// Register a mouse event listener on the window for the next frame. The type of event
    /// is determined by the first parameter of the given listener. When the next frame is rendered
    /// the listener will be cleared.
    pub fn on_mouse_event<Event: MouseEvent>(
        &mut self,
        mut handler: impl FnMut(&Event, DispatchPhase, &mut ElementContext) + 'static,
    ) {
        let view_id = self.parent_view_id();
        let order = self.window.next_frame.z_index_stack.clone();
        self.window
            .next_frame
            .mouse_listeners
            .entry(TypeId::of::<Event>())
            .or_default()
            .push((
                order,
                view_id,
                Box::new(
                    move |event: &dyn Any, phase: DispatchPhase, cx: &mut ElementContext<'_>| {
                        handler(event.downcast_ref().unwrap(), phase, cx)
                    },
                ),
            ))
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
}
