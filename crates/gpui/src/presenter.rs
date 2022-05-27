use crate::{
    app::{AppContext, MutableAppContext, WindowInvalidation},
    elements::Element,
    font_cache::FontCache,
    geometry::rect::RectF,
    json::{self, ToJson},
    keymap::Keystroke,
    platform::{CursorStyle, Event},
    scene::CursorRegion,
    text_layout::TextLayoutCache,
    Action, AnyModelHandle, AnyViewHandle, AnyWeakModelHandle, AssetCache, ElementBox, Entity,
    FontSystem, ModelHandle, MouseRegion, MouseRegionId, ReadModel, ReadView, RenderContext,
    RenderParams, Scene, UpgradeModelHandle, UpgradeViewHandle, View, ViewHandle, WeakModelHandle,
    WeakViewHandle,
};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;
use smallvec::SmallVec;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub struct Presenter {
    window_id: usize,
    pub(crate) rendered_views: HashMap<usize, ElementBox>,
    parents: HashMap<usize, usize>,
    cursor_regions: Vec<CursorRegion>,
    mouse_regions: Vec<(MouseRegion, usize)>,
    font_cache: Arc<FontCache>,
    text_layout_cache: TextLayoutCache,
    asset_cache: Arc<AssetCache>,
    last_mouse_moved_event: Option<Event>,
    hovered_region_ids: HashSet<MouseRegionId>,
    clicked_region: Option<MouseRegion>,
    right_clicked_region: Option<MouseRegion>,
    prev_drag_position: Option<Vector2F>,
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
            cursor_regions: Default::default(),
            mouse_regions: Default::default(),
            font_cache,
            text_layout_cache,
            asset_cache,
            last_mouse_moved_event: None,
            hovered_region_ids: Default::default(),
            clicked_region: None,
            right_clicked_region: None,
            prev_drag_position: None,
            titlebar_height,
        }
    }

    pub fn dispatch_path(&self, app: &AppContext) -> Vec<usize> {
        let mut path = Vec::new();
        if let Some(view_id) = app.focused_view_id(self.window_id) {
            self.compute_dispatch_path_from(view_id, &mut path)
        }
        path
    }

    pub(crate) fn compute_dispatch_path_from(&self, mut view_id: usize, path: &mut Vec<usize>) {
        path.push(view_id);
        while let Some(parent_id) = self.parents.get(&view_id).copied() {
            path.push(parent_id);
            view_id = parent_id;
        }
        path.reverse();
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
                cx.render_view(RenderParams {
                    window_id: self.window_id,
                    view_id: *view_id,
                    titlebar_height: self.titlebar_height,
                    hovered_region_ids: self.hovered_region_ids.clone(),
                    clicked_region_id: self.clicked_region.as_ref().and_then(MouseRegion::id),
                    right_clicked_region_id: self
                        .right_clicked_region
                        .as_ref()
                        .and_then(MouseRegion::id),
                    refreshing: false,
                })
                .unwrap(),
            );
        }
    }

    pub fn refresh(&mut self, invalidation: &mut WindowInvalidation, cx: &mut MutableAppContext) {
        self.invalidate(invalidation, cx);
        for (view_id, view) in &mut self.rendered_views {
            if !invalidation.updated.contains(view_id) {
                *view = cx
                    .render_view(RenderParams {
                        window_id: self.window_id,
                        view_id: *view_id,
                        titlebar_height: self.titlebar_height,
                        hovered_region_ids: self.hovered_region_ids.clone(),
                        clicked_region_id: self.clicked_region.as_ref().and_then(MouseRegion::id),
                        right_clicked_region_id: self
                            .right_clicked_region
                            .as_ref()
                            .and_then(MouseRegion::id),
                        refreshing: true,
                    })
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
            self.cursor_regions = scene.cursor_regions();
            self.mouse_regions = scene.mouse_regions();

            if cx.window_is_active(self.window_id) {
                if let Some(event) = self.last_mouse_moved_event.clone() {
                    self.dispatch_event(event, cx)
                }
            }
        } else {
            log::error!("could not find root_view_id for window {}", self.window_id);
        }

        scene
    }

    fn layout(&mut self, window_size: Vector2F, refreshing: bool, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            self.build_layout_context(window_size, refreshing, cx)
                .layout(root_view_id, SizeConstraint::strict(window_size));
        }
    }

    pub fn build_layout_context<'a>(
        &'a mut self,
        window_size: Vector2F,
        refreshing: bool,
        cx: &'a mut MutableAppContext,
    ) -> LayoutContext<'a> {
        LayoutContext {
            window_id: self.window_id,
            rendered_views: &mut self.rendered_views,
            parents: &mut self.parents,
            font_cache: &self.font_cache,
            font_system: cx.platform().fonts(),
            text_layout_cache: &self.text_layout_cache,
            asset_cache: &self.asset_cache,
            view_stack: Vec::new(),
            refreshing,
            hovered_region_ids: self.hovered_region_ids.clone(),
            clicked_region_id: self.clicked_region.as_ref().and_then(MouseRegion::id),
            right_clicked_region_id: self.right_clicked_region.as_ref().and_then(MouseRegion::id),
            titlebar_height: self.titlebar_height,
            window_size,
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
            view_stack: Vec::new(),
            app: cx,
        }
    }

    pub fn dispatch_event(&mut self, event: Event, cx: &mut MutableAppContext) {
        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            let mut invalidated_views = Vec::new();
            let mut hovered_regions = Vec::new();
            let mut unhovered_regions = Vec::new();
            let mut mouse_down_region = None;
            let mut clicked_region = None;
            let mut right_mouse_down_region = None;
            let mut right_clicked_region = None;
            let mut dragged_region = None;

            match event {
                Event::LeftMouseDown { position, .. } => {
                    for (region, _) in self.mouse_regions.iter().rev() {
                        if region.bounds.contains_point(position) {
                            invalidated_views.push(region.view_id);
                            mouse_down_region = Some((region.clone(), position));
                            self.clicked_region = Some(region.clone());
                            self.prev_drag_position = Some(position);
                            break;
                        }
                    }
                }
                Event::LeftMouseUp {
                    position,
                    click_count,
                    ..
                } => {
                    self.prev_drag_position.take();
                    if let Some(region) = self.clicked_region.take() {
                        invalidated_views.push(region.view_id);
                        if region.bounds.contains_point(position) {
                            clicked_region = Some((region, position, click_count));
                        }
                    }
                }
                Event::RightMouseDown { position, .. } => {
                    for (region, _) in self.mouse_regions.iter().rev() {
                        if region.bounds.contains_point(position) {
                            invalidated_views.push(region.view_id);
                            right_mouse_down_region = Some((region.clone(), position));
                            self.right_clicked_region = Some(region.clone());
                            break;
                        }
                    }
                }
                Event::RightMouseUp {
                    position,
                    click_count,
                    ..
                } => {
                    if let Some(region) = self.right_clicked_region.take() {
                        invalidated_views.push(region.view_id);
                        if region.bounds.contains_point(position) {
                            right_clicked_region = Some((region, position, click_count));
                        }
                    }
                }
                Event::MouseMoved {
                    position,
                    left_mouse_down,
                } => {
                    self.last_mouse_moved_event = Some(event.clone());

                    if !left_mouse_down {
                        let mut style_to_assign = CursorStyle::Arrow;
                        for region in self.cursor_regions.iter().rev() {
                            if region.bounds.contains_point(position) {
                                style_to_assign = region.style;
                                break;
                            }
                        }
                        cx.platform().set_cursor_style(style_to_assign);

                        let mut hover_depth = None;
                        for (region, depth) in self.mouse_regions.iter().rev() {
                            if region.bounds.contains_point(position)
                                && hover_depth.map_or(true, |hover_depth| hover_depth == *depth)
                            {
                                hover_depth = Some(*depth);
                                if let Some(region_id) = region.id() {
                                    if !self.hovered_region_ids.contains(&region_id) {
                                        invalidated_views.push(region.view_id);
                                        hovered_regions.push(region.clone());
                                        self.hovered_region_ids.insert(region_id);
                                    }
                                }
                            } else {
                                if let Some(region_id) = region.id() {
                                    if self.hovered_region_ids.contains(&region_id) {
                                        invalidated_views.push(region.view_id);
                                        unhovered_regions.push(region.clone());
                                        self.hovered_region_ids.remove(&region_id);
                                    }
                                }
                            }
                        }
                    }
                }
                Event::LeftMouseDragged { position } => {
                    if let Some((clicked_region, prev_drag_position)) = self
                        .clicked_region
                        .as_ref()
                        .zip(self.prev_drag_position.as_mut())
                    {
                        dragged_region =
                            Some((clicked_region.clone(), position - *prev_drag_position));
                        *prev_drag_position = position;
                    }

                    self.last_mouse_moved_event = Some(Event::MouseMoved {
                        position,
                        left_mouse_down: true,
                    });
                }
                _ => {}
            }

            let mut event_cx = self.build_event_context(cx);
            let mut handled = false;
            for unhovered_region in unhovered_regions {
                if let Some(hover_callback) = unhovered_region.hover {
                    handled = true;
                    event_cx.with_current_view(unhovered_region.view_id, |event_cx| {
                        hover_callback(false, event_cx);
                    })
                }
            }

            for hovered_region in hovered_regions {
                if let Some(hover_callback) = hovered_region.hover {
                    handled = true;
                    event_cx.with_current_view(hovered_region.view_id, |event_cx| {
                        hover_callback(true, event_cx);
                    })
                }
            }

            if let Some((mouse_down_region, position)) = mouse_down_region {
                if let Some(mouse_down_callback) = mouse_down_region.mouse_down {
                    handled = true;
                    event_cx.with_current_view(mouse_down_region.view_id, |event_cx| {
                        mouse_down_callback(position, event_cx);
                    })
                }
            }

            if let Some((clicked_region, position, click_count)) = clicked_region {
                if let Some(click_callback) = clicked_region.click {
                    handled = true;
                    event_cx.with_current_view(clicked_region.view_id, |event_cx| {
                        click_callback(position, click_count, event_cx);
                    })
                }
            }

            if let Some((right_mouse_down_region, position)) = right_mouse_down_region {
                if let Some(right_mouse_down_callback) = right_mouse_down_region.right_mouse_down {
                    handled = true;
                    event_cx.with_current_view(right_mouse_down_region.view_id, |event_cx| {
                        right_mouse_down_callback(position, event_cx);
                    })
                }
            }

            if let Some((right_clicked_region, position, click_count)) = right_clicked_region {
                if let Some(right_click_callback) = right_clicked_region.right_click {
                    handled = true;
                    event_cx.with_current_view(right_clicked_region.view_id, |event_cx| {
                        right_click_callback(position, click_count, event_cx);
                    })
                }
            }

            if let Some((dragged_region, delta)) = dragged_region {
                if let Some(drag_callback) = dragged_region.drag {
                    handled = true;
                    event_cx.with_current_view(dragged_region.view_id, |event_cx| {
                        drag_callback(delta, event_cx);
                    })
                }
            }

            if !handled {
                event_cx.dispatch_event(root_view_id, &event);
            }

            invalidated_views.extend(event_cx.invalidated_views);
            let dispatch_directives = event_cx.dispatched_actions;

            for view_id in invalidated_views {
                cx.notify_view(self.window_id, view_id);
            }

            let mut dispatch_path = Vec::new();
            for directive in dispatch_directives {
                dispatch_path.clear();
                if let Some(view_id) = directive.dispatcher_view_id {
                    self.compute_dispatch_path_from(view_id, &mut dispatch_path);
                }
                cx.dispatch_action_any(self.window_id, &dispatch_path, directive.action.as_ref());
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
        let view = cx.root_view(self.window_id)?;
        Some(json!({
            "root_view": view.debug_json(cx),
            "root_element": self.rendered_views.get(&view.id())
                .map(|root_element| {
                    root_element.debug(&DebugContext {
                        rendered_views: &self.rendered_views,
                        font_cache: &self.font_cache,
                        app: cx,
                    })
                })
        }))
    }
}

pub struct DispatchDirective {
    pub dispatcher_view_id: Option<usize>,
    pub action: Box<dyn Action>,
}

pub struct LayoutContext<'a> {
    window_id: usize,
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    parents: &'a mut HashMap<usize, usize>,
    view_stack: Vec<usize>,
    pub font_cache: &'a Arc<FontCache>,
    pub font_system: Arc<dyn FontSystem>,
    pub text_layout_cache: &'a TextLayoutCache,
    pub asset_cache: &'a AssetCache,
    pub app: &'a mut MutableAppContext,
    pub refreshing: bool,
    pub window_size: Vector2F,
    titlebar_height: f32,
    hovered_region_ids: HashSet<MouseRegionId>,
    clicked_region_id: Option<MouseRegionId>,
    right_clicked_region_id: Option<MouseRegionId>,
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn keystrokes_for_action(
        &self,
        action: &dyn Action,
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        self.app
            .keystrokes_for_action(self.window_id, &self.view_stack, action)
    }

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

    pub fn render<F, V, T>(&mut self, handle: &ViewHandle<V>, f: F) -> T
    where
        F: FnOnce(&mut V, &mut RenderContext<V>) -> T,
        V: View,
    {
        handle.update(self.app, |view, cx| {
            let mut render_cx = RenderContext {
                app: cx,
                window_id: handle.window_id(),
                view_id: handle.id(),
                view_type: PhantomData,
                titlebar_height: self.titlebar_height,
                hovered_region_ids: self.hovered_region_ids.clone(),
                clicked_region_id: self.clicked_region_id,
                right_clicked_region_id: self.right_clicked_region_id,
                refreshing: self.refreshing,
            };
            f(view, &mut render_cx)
        })
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

pub struct PaintContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    view_stack: Vec<usize>,
    pub scene: &'a mut Scene,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a AppContext,
}

impl<'a> PaintContext<'a> {
    fn paint(&mut self, view_id: usize, origin: Vector2F, visible_bounds: RectF) {
        if let Some(mut tree) = self.rendered_views.remove(&view_id) {
            self.view_stack.push(view_id);
            tree.paint(origin, visible_bounds, self);
            self.rendered_views.insert(view_id, tree);
            self.view_stack.pop();
        }
    }

    pub fn current_view_id(&self) -> usize {
        *self.view_stack.last().unwrap()
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
            let result =
                self.with_current_view(view_id, |this| element.dispatch_event(event, this));
            self.rendered_views.insert(view_id, element);
            result
        } else {
            false
        }
    }

    fn with_current_view<F, T>(&mut self, view_id: usize, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.view_stack.push(view_id);
        let result = f(self);
        self.view_stack.pop();
        result
    }

    pub fn dispatch_any_action(&mut self, action: Box<dyn Action>) {
        self.dispatched_actions.push(DispatchDirective {
            dispatcher_view_id: self.view_stack.last().copied(),
            action,
        });
    }

    pub fn dispatch_action<A: Action>(&mut self, action: A) {
        self.dispatch_any_action(Box::new(action));
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

impl Default for SizeConstraint {
    fn default() -> Self {
        SizeConstraint {
            min: Vector2F::zero(),
            max: Vector2F::splat(f32::INFINITY),
        }
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
            "view": self.view.debug_json(cx.app),
            "child": if let Some(view) = cx.rendered_views.get(&self.view.id()) {
                view.debug(cx)
            } else {
                json!(null)
            }
        })
    }
}
