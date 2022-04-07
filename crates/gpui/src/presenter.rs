use crate::{
    app::{AppContext, MutableAppContext, WindowInvalidation},
    elements::Element,
    font_cache::FontCache,
    geometry::rect::RectF,
    json::{self, ToJson},
    platform::Event,
    text_layout::TextLayoutCache,
    Action, AnyAction, AnyModelHandle, AnyViewHandle, AnyWeakModelHandle, AssetCache, ElementBox,
    ElementStateContext, Entity, FontSystem, ModelHandle, ReadModel, ReadView, Scene,
    UpgradeModelHandle, UpgradeViewHandle, View, ViewHandle, WeakModelHandle, WeakViewHandle,
};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub struct Presenter {
    window_id: usize,
    pub(crate) rendered_views: HashMap<usize, ElementBox>,
    parents: HashMap<usize, usize>,
    font_cache: Arc<FontCache>,
    text_layout_cache: TextLayoutCache,
    asset_cache: Arc<AssetCache>,
    last_mouse_moved_event: Option<Event>,
    titlebar_height: f32,
}

impl Presenter {
    pub fn new(
        window_id: usize,
        titlebar_height: f32,
        font_cache: Arc<FontCache>,
        text_layout_cache: TextLayoutCache,
        asset_cache: Arc<AssetCache>,
        cx: &mut MutableAppContext,
    ) -> Self {
        Self {
            window_id,
            rendered_views: cx.render_views(window_id, titlebar_height),
            parents: HashMap::new(),
            font_cache,
            text_layout_cache,
            asset_cache,
            last_mouse_moved_event: None,
            titlebar_height,
        }
    }

    pub fn dispatch_path(&self, app: &AppContext) -> Vec<usize> {
        let mut path = Vec::new();
        if let Some(mut view_id) = app.focused_view_id(self.window_id) {
            path.push(view_id);
            while let Some(parent_id) = self.parents.get(&view_id).copied() {
                path.push(parent_id);
                view_id = parent_id;
            }
            path.reverse();
        }
        path
    }

    pub fn invalidate(
        &mut self,
        invalidation: &mut WindowInvalidation,
        cx: &mut MutableAppContext,
    ) {
        cx.start_frame();
        for view_id in &invalidation.removed {
            invalidation.updated.remove(&view_id);
            self.rendered_views.remove(&view_id);
            self.parents.remove(&view_id);
        }
        for view_id in &invalidation.updated {
            self.rendered_views.insert(
                *view_id,
                cx.render_view(self.window_id, *view_id, self.titlebar_height, false)
                    .unwrap(),
            );
        }
    }

    pub fn refresh(&mut self, invalidation: &mut WindowInvalidation, cx: &mut MutableAppContext) {
        self.invalidate(invalidation, cx);
        for (view_id, view) in &mut self.rendered_views {
            if !invalidation.updated.contains(view_id) {
                *view = cx
                    .render_view(self.window_id, *view_id, self.titlebar_height, true)
                    .unwrap();
            }
        }
    }

    pub fn build_scene(
        &mut self,
        window_size: Vector2F,
        scale_factor: f32,
        refreshing: bool,
        cx: &mut MutableAppContext,
    ) -> Scene {
        let mut scene = Scene::new(scale_factor);

        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            self.layout(window_size, refreshing, cx);
            let mut paint_cx = self.build_paint_context(&mut scene, cx);
            paint_cx.paint(
                root_view_id,
                Vector2F::zero(),
                RectF::new(Vector2F::zero(), window_size),
            );
            self.text_layout_cache.finish_frame();

            if let Some(event) = self.last_mouse_moved_event.clone() {
                self.dispatch_event(event, cx)
            }
        } else {
            log::error!("could not find root_view_id for window {}", self.window_id);
        }

        scene
    }

    fn layout(&mut self, size: Vector2F, refreshing: bool, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            self.build_layout_context(refreshing, cx)
                .layout(root_view_id, SizeConstraint::strict(size));
        }
    }

    pub fn build_layout_context<'a>(
        &'a mut self,
        refreshing: bool,
        cx: &'a mut MutableAppContext,
    ) -> LayoutContext<'a> {
        LayoutContext {
            rendered_views: &mut self.rendered_views,
            parents: &mut self.parents,
            refreshing,
            font_cache: &self.font_cache,
            font_system: cx.platform().fonts(),
            text_layout_cache: &self.text_layout_cache,
            asset_cache: &self.asset_cache,
            view_stack: Vec::new(),
            app: cx,
        }
    }

    pub fn build_paint_context<'a>(
        &'a mut self,
        scene: &'a mut Scene,
        cx: &'a mut MutableAppContext,
    ) -> PaintContext {
        PaintContext {
            scene,
            font_cache: &self.font_cache,
            text_layout_cache: &self.text_layout_cache,
            rendered_views: &mut self.rendered_views,
            app: cx,
        }
    }

    pub fn dispatch_event(&mut self, event: Event, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            match event {
                Event::MouseMoved { .. } => {
                    self.last_mouse_moved_event = Some(event.clone());
                }
                Event::LeftMouseDragged { position } => {
                    self.last_mouse_moved_event = Some(Event::MouseMoved {
                        position,
                        left_mouse_down: true,
                    });
                }
                _ => {}
            }

            let mut event_cx = self.build_event_context(cx);
            event_cx.dispatch_event(root_view_id, &event);

            let invalidated_views = event_cx.invalidated_views;
            let dispatch_directives = event_cx.dispatched_actions;

            for view_id in invalidated_views {
                cx.notify_view(self.window_id, view_id);
            }
            for directive in dispatch_directives {
                cx.dispatch_action_any(self.window_id, &directive.path, directive.action.as_ref());
            }
        }
    }

    pub fn build_event_context<'a>(
        &'a mut self,
        cx: &'a mut MutableAppContext,
    ) -> EventContext<'a> {
        EventContext {
            rendered_views: &mut self.rendered_views,
            dispatched_actions: Default::default(),
            font_cache: &self.font_cache,
            text_layout_cache: &self.text_layout_cache,
            view_stack: Default::default(),
            invalidated_views: Default::default(),
            notify_count: 0,
            app: cx,
        }
    }

    pub fn debug_elements(&self, cx: &AppContext) -> Option<json::Value> {
        cx.root_view_id(self.window_id)
            .and_then(|root_view_id| self.rendered_views.get(&root_view_id))
            .map(|root_element| {
                root_element.debug(&DebugContext {
                    rendered_views: &self.rendered_views,
                    font_cache: &self.font_cache,
                    app: cx,
                })
            })
    }
}

pub struct DispatchDirective {
    pub path: Vec<usize>,
    pub action: Box<dyn AnyAction>,
}

pub struct LayoutContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    parents: &'a mut HashMap<usize, usize>,
    view_stack: Vec<usize>,
    pub refreshing: bool,
    pub font_cache: &'a Arc<FontCache>,
    pub font_system: Arc<dyn FontSystem>,
    pub text_layout_cache: &'a TextLayoutCache,
    pub asset_cache: &'a AssetCache,
    pub app: &'a mut MutableAppContext,
}

impl<'a> LayoutContext<'a> {
    fn layout(&mut self, view_id: usize, constraint: SizeConstraint) -> Vector2F {
        if let Some(parent_id) = self.view_stack.last() {
            self.parents.insert(view_id, *parent_id);
        }
        self.view_stack.push(view_id);
        let mut rendered_view = self.rendered_views.remove(&view_id).unwrap();
        let size = rendered_view.layout(constraint, self);
        self.rendered_views.insert(view_id, rendered_view);
        self.view_stack.pop();
        size
    }
}

impl<'a> Deref for LayoutContext<'a> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<'a> DerefMut for LayoutContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.app
    }
}

impl<'a> ReadView for LayoutContext<'a> {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        self.app.read_view(handle)
    }
}

impl<'a> ReadModel for LayoutContext<'a> {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        self.app.read_model(handle)
    }
}

impl<'a> UpgradeModelHandle for LayoutContext<'a> {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        self.app.upgrade_model_handle(handle)
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.app.model_handle_is_upgradable(handle)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        self.app.upgrade_any_model_handle(handle)
    }
}

impl<'a> UpgradeViewHandle for LayoutContext<'a> {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
        self.app.upgrade_view_handle(handle)
    }

    fn upgrade_any_view_handle(&self, handle: &crate::AnyWeakViewHandle) -> Option<AnyViewHandle> {
        self.app.upgrade_any_view_handle(handle)
    }
}

impl<'a> ElementStateContext for LayoutContext<'a> {
    fn current_view_id(&self) -> usize {
        *self.view_stack.last().unwrap()
    }
}

pub struct PaintContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    pub scene: &'a mut Scene,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a AppContext,
}

impl<'a> PaintContext<'a> {
    fn paint(&mut self, view_id: usize, origin: Vector2F, visible_bounds: RectF) {
        if let Some(mut tree) = self.rendered_views.remove(&view_id) {
            tree.paint(origin, visible_bounds, self);
            self.rendered_views.insert(view_id, tree);
        }
    }
}

impl<'a> Deref for PaintContext<'a> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

pub struct EventContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    dispatched_actions: Vec<DispatchDirective>,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a mut MutableAppContext,
    pub notify_count: usize,
    view_stack: Vec<usize>,
    invalidated_views: HashSet<usize>,
}

impl<'a> EventContext<'a> {
    fn dispatch_event(&mut self, view_id: usize, event: &Event) -> bool {
        if let Some(mut element) = self.rendered_views.remove(&view_id) {
            self.view_stack.push(view_id);
            let result = element.dispatch_event(event, self);
            self.view_stack.pop();
            self.rendered_views.insert(view_id, element);
            result
        } else {
            false
        }
    }

    pub fn dispatch_action<A: Action>(&mut self, action: A) {
        self.dispatched_actions.push(DispatchDirective {
            path: self.view_stack.clone(),
            action: Box::new(action),
        });
    }

    pub fn notify(&mut self) {
        self.notify_count += 1;
        if let Some(view_id) = self.view_stack.last() {
            self.invalidated_views.insert(*view_id);
        }
    }

    pub fn notify_count(&self) -> usize {
        self.notify_count
    }
}

impl<'a> Deref for EventContext<'a> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<'a> DerefMut for EventContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.app
    }
}

pub struct DebugContext<'a> {
    rendered_views: &'a HashMap<usize, ElementBox>,
    pub font_cache: &'a FontCache,
    pub app: &'a AppContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn invert(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

impl ToJson for Axis {
    fn to_json(&self) -> serde_json::Value {
        match self {
            Axis::Horizontal => json!("horizontal"),
            Axis::Vertical => json!("vertical"),
        }
    }
}

pub trait Vector2FExt {
    fn along(self, axis: Axis) -> f32;
}

impl Vector2FExt for Vector2F {
    fn along(self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.x(),
            Axis::Vertical => self.y(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SizeConstraint {
    pub min: Vector2F,
    pub max: Vector2F,
}

impl SizeConstraint {
    pub fn new(min: Vector2F, max: Vector2F) -> Self {
        Self { min, max }
    }

    pub fn strict(size: Vector2F) -> Self {
        Self {
            min: size,
            max: size,
        }
    }

    pub fn strict_along(axis: Axis, max: f32) -> Self {
        match axis {
            Axis::Horizontal => Self {
                min: vec2f(max, 0.0),
                max: vec2f(max, f32::INFINITY),
            },
            Axis::Vertical => Self {
                min: vec2f(0.0, max),
                max: vec2f(f32::INFINITY, max),
            },
        }
    }

    pub fn max_along(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.max.x(),
            Axis::Vertical => self.max.y(),
        }
    }

    pub fn min_along(&self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.min.x(),
            Axis::Vertical => self.min.y(),
        }
    }

    pub fn constrain(&self, size: Vector2F) -> Vector2F {
        vec2f(
            size.x().min(self.max.x()).max(self.min.x()),
            size.y().min(self.max.y()).max(self.min.y()),
        )
    }
}

impl ToJson for SizeConstraint {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "min": self.min.to_json(),
            "max": self.max.to_json(),
        })
    }
}

pub struct ChildView {
    view: AnyViewHandle,
}

impl ChildView {
    pub fn new(view: impl Into<AnyViewHandle>) -> Self {
        Self { view: view.into() }
    }
}

impl Element for ChildView {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let size = cx.layout(self.view.id(), constraint);
        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        cx.paint(self.view.id(), bounds.origin(), visible_bounds);
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        cx.dispatch_event(self.view.id(), event)
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> serde_json::Value {
        json!({
            "type": "ChildView",
            "view_id": self.view.id(),
            "bounds": bounds.to_json(),
            "child": if let Some(view) = cx.rendered_views.get(&self.view.id()) {
                view.debug(cx)
            } else {
                json!(null)
            }
        })
    }
}
