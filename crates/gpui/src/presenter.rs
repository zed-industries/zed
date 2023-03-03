use crate::{
    app::{AppContext, MutableAppContext, WindowInvalidation},
    elements::Element,
    font_cache::FontCache,
    geometry::rect::RectF,
    json::{self, ToJson},
    platform::{CursorStyle, Event},
    scene::{
        CursorRegion, MouseClick, MouseDown, MouseDownOut, MouseDrag, MouseEvent, MouseHover,
        MouseMove, MouseMoveOut, MouseScrollWheel, MouseUp, MouseUpOut, Scene,
    },
    text_layout::TextLayoutCache,
    Action, AnyModelHandle, AnyViewHandle, AnyWeakModelHandle, AnyWeakViewHandle, Appearance,
    AssetCache, ElementBox, Entity, FontSystem, ModelHandle, MouseButton, MouseMovedEvent,
    MouseRegion, MouseRegionId, ParentId, ReadModel, ReadView, RenderContext, RenderParams,
    SceneBuilder, UpgradeModelHandle, UpgradeViewHandle, View, ViewHandle, WeakModelHandle,
    WeakViewHandle,
};
use anyhow::bail;
use collections::{HashMap, HashSet};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use serde_json::json;
use smallvec::SmallVec;
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};

pub struct Presenter {
    window_id: usize,
    pub(crate) rendered_views: HashMap<usize, ElementBox>,
    cursor_regions: Vec<CursorRegion>,
    mouse_regions: Vec<(MouseRegion, usize)>,
    font_cache: Arc<FontCache>,
    text_layout_cache: TextLayoutCache,
    asset_cache: Arc<AssetCache>,
    last_mouse_moved_event: Option<Event>,
    hovered_region_ids: HashSet<MouseRegionId>,
    clicked_region_ids: HashSet<MouseRegionId>,
    clicked_button: Option<MouseButton>,
    mouse_position: Vector2F,
    titlebar_height: f32,
    appearance: Appearance,
}

impl Presenter {
    pub fn new(
        window_id: usize,
        titlebar_height: f32,
        appearance: Appearance,
        font_cache: Arc<FontCache>,
        text_layout_cache: TextLayoutCache,
        asset_cache: Arc<AssetCache>,
        cx: &mut MutableAppContext,
    ) -> Self {
        Self {
            window_id,
            rendered_views: cx.render_views(window_id, titlebar_height, appearance),
            cursor_regions: Default::default(),
            mouse_regions: Default::default(),
            font_cache,
            text_layout_cache,
            asset_cache,
            last_mouse_moved_event: None,
            hovered_region_ids: Default::default(),
            clicked_region_ids: Default::default(),
            clicked_button: None,
            mouse_position: vec2f(0., 0.),
            titlebar_height,
            appearance,
        }
    }

    pub fn invalidate(
        &mut self,
        invalidation: &mut WindowInvalidation,
        appearance: Appearance,
        cx: &mut MutableAppContext,
    ) {
        cx.start_frame();
        self.appearance = appearance;
        for view_id in &invalidation.removed {
            invalidation.updated.remove(view_id);
            self.rendered_views.remove(view_id);
        }
        for view_id in &invalidation.updated {
            self.rendered_views.insert(
                *view_id,
                cx.render_view(RenderParams {
                    window_id: self.window_id,
                    view_id: *view_id,
                    titlebar_height: self.titlebar_height,
                    hovered_region_ids: self.hovered_region_ids.clone(),
                    clicked_region_ids: self
                        .clicked_button
                        .map(|button| (self.clicked_region_ids.clone(), button)),
                    refreshing: false,
                    appearance,
                })
                .unwrap(),
            );
        }
    }

    pub fn refresh(
        &mut self,
        invalidation: &mut WindowInvalidation,
        appearance: Appearance,
        cx: &mut MutableAppContext,
    ) {
        self.invalidate(invalidation, appearance, cx);
        for (view_id, view) in &mut self.rendered_views {
            if !invalidation.updated.contains(view_id) {
                *view = cx
                    .render_view(RenderParams {
                        window_id: self.window_id,
                        view_id: *view_id,
                        titlebar_height: self.titlebar_height,
                        hovered_region_ids: self.hovered_region_ids.clone(),
                        clicked_region_ids: self
                            .clicked_button
                            .map(|button| (self.clicked_region_ids.clone(), button)),
                        refreshing: true,
                        appearance,
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
        let mut scene_builder = SceneBuilder::new(scale_factor);

        if let Some(root_view_id) = cx.root_view_id(self.window_id) {
            self.layout(window_size, refreshing, cx);
            let mut paint_cx = self.build_paint_context(&mut scene_builder, window_size, cx);
            paint_cx.paint(
                root_view_id,
                Vector2F::zero(),
                RectF::new(Vector2F::zero(), window_size),
            );
            self.text_layout_cache.finish_frame();
            let scene = scene_builder.build();
            self.cursor_regions = scene.cursor_regions();
            self.mouse_regions = scene.mouse_regions();

            // window.is_topmost for the mouse moved event's postion?
            if cx.window_is_active(self.window_id) {
                if let Some(event) = self.last_mouse_moved_event.clone() {
                    self.dispatch_event(event, true, cx);
                }
            }

            scene
        } else {
            log::error!("could not find root_view_id for window {}", self.window_id);
            scene_builder.build()
        }
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
            font_cache: &self.font_cache,
            font_system: cx.platform().fonts(),
            text_layout_cache: &self.text_layout_cache,
            asset_cache: &self.asset_cache,
            view_stack: Vec::new(),
            refreshing,
            hovered_region_ids: self.hovered_region_ids.clone(),
            clicked_region_ids: self
                .clicked_button
                .map(|button| (self.clicked_region_ids.clone(), button)),
            titlebar_height: self.titlebar_height,
            appearance: self.appearance,
            window_size,
            app: cx,
        }
    }

    pub fn build_paint_context<'a>(
        &'a mut self,
        scene: &'a mut SceneBuilder,
        window_size: Vector2F,
        cx: &'a mut MutableAppContext,
    ) -> PaintContext {
        PaintContext {
            scene,
            window_size,
            font_cache: &self.font_cache,
            text_layout_cache: &self.text_layout_cache,
            rendered_views: &mut self.rendered_views,
            view_stack: Vec::new(),
            app: cx,
        }
    }

    pub fn rect_for_text_range(&self, range_utf16: Range<usize>, cx: &AppContext) -> Option<RectF> {
        cx.focused_view_id(self.window_id).and_then(|view_id| {
            let cx = MeasurementContext {
                app: cx,
                rendered_views: &self.rendered_views,
                window_id: self.window_id,
            };
            cx.rect_for_text_range(view_id, range_utf16)
        })
    }

    pub fn dispatch_event(
        &mut self,
        event: Event,
        event_reused: bool,
        cx: &mut MutableAppContext,
    ) -> bool {
        let mut mouse_events = SmallVec::<[_; 2]>::new();
        let mut notified_views: HashSet<usize> = Default::default();

        // 1. Handle platform event. Keyboard events get dispatched immediately, while mouse events
        //    get mapped into the mouse-specific MouseEvent type.
        //  -> These are usually small: [Mouse Down] or [Mouse up, Click] or [Mouse Moved, Mouse Dragged?]
        //  -> Also updates mouse-related state
        match &event {
            Event::KeyDown(e) => return cx.dispatch_key_down(self.window_id, e),

            Event::KeyUp(e) => return cx.dispatch_key_up(self.window_id, e),

            Event::ModifiersChanged(e) => return cx.dispatch_modifiers_changed(self.window_id, e),

            Event::MouseDown(e) => {
                // Click events are weird because they can be fired after a drag event.
                // MDN says that browsers handle this by starting from 'the most
                // specific ancestor element that contained both [positions]'
                // So we need to store the overlapping regions on mouse down.

                // If there is already clicked_button stored, don't replace it.
                if self.clicked_button.is_none() {
                    self.clicked_region_ids = self
                        .mouse_regions
                        .iter()
                        .filter_map(|(region, _)| {
                            if region.bounds.contains_point(e.position) {
                                Some(region.id())
                            } else {
                                None
                            }
                        })
                        .collect();

                    self.clicked_button = Some(e.button);
                }

                mouse_events.push(MouseEvent::Down(MouseDown {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
                mouse_events.push(MouseEvent::DownOut(MouseDownOut {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
            }

            Event::MouseUp(e) => {
                // NOTE: The order of event pushes is important! MouseUp events MUST be fired
                // before click events, and so the MouseUp events need to be pushed before
                // MouseClick events.
                mouse_events.push(MouseEvent::Up(MouseUp {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
                mouse_events.push(MouseEvent::UpOut(MouseUpOut {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
                mouse_events.push(MouseEvent::Click(MouseClick {
                    region: Default::default(),
                    platform_event: e.clone(),
                }));
            }

            Event::MouseMoved(
                e @ MouseMovedEvent {
                    position,
                    pressed_button,
                    ..
                },
            ) => {
                let mut style_to_assign = CursorStyle::Arrow;
                for region in self.cursor_regions.iter().rev() {
                    if region.bounds.contains_point(*position) {
                        style_to_assign = region.style;
                        break;
                    }
                }

                if cx.is_topmost_window_for_position(self.window_id, *position) {
                    cx.platform().set_cursor_style(style_to_assign);
                }

                if !event_reused {
                    if pressed_button.is_some() {
                        mouse_events.push(MouseEvent::Drag(MouseDrag {
                            region: Default::default(),
                            prev_mouse_position: self.mouse_position,
                            platform_event: e.clone(),
                        }));
                    } else if let Some(clicked_button) = self.clicked_button {
                        // Mouse up event happened outside the current window. Simulate mouse up button event
                        let button_event = e.to_button_event(clicked_button);
                        mouse_events.push(MouseEvent::Up(MouseUp {
                            region: Default::default(),
                            platform_event: button_event.clone(),
                        }));
                        mouse_events.push(MouseEvent::UpOut(MouseUpOut {
                            region: Default::default(),
                            platform_event: button_event.clone(),
                        }));
                        mouse_events.push(MouseEvent::Click(MouseClick {
                            region: Default::default(),
                            platform_event: button_event.clone(),
                        }));
                    }

                    mouse_events.push(MouseEvent::Move(MouseMove {
                        region: Default::default(),
                        platform_event: e.clone(),
                    }));
                }

                mouse_events.push(MouseEvent::Hover(MouseHover {
                    region: Default::default(),
                    platform_event: e.clone(),
                    started: false,
                }));
                mouse_events.push(MouseEvent::MoveOut(MouseMoveOut {
                    region: Default::default(),
                }));

                self.last_mouse_moved_event = Some(event.clone());
            }

            Event::MouseExited(event) => {
                // When the platform sends a MouseExited event, synthesize
                // a MouseMoved event whose position is outside the window's
                // bounds so that hover and cursor state can be updated.
                return self.dispatch_event(
                    Event::MouseMoved(MouseMovedEvent {
                        position: event.position,
                        pressed_button: event.pressed_button,
                        modifiers: event.modifiers,
                    }),
                    event_reused,
                    cx,
                );
            }

            Event::ScrollWheel(e) => mouse_events.push(MouseEvent::ScrollWheel(MouseScrollWheel {
                region: Default::default(),
                platform_event: e.clone(),
            })),
        }

        if let Some(position) = event.position() {
            self.mouse_position = position;
        }

        // 2. Dispatch mouse events on regions
        let mut any_event_handled = false;
        for mut mouse_event in mouse_events {
            let mut valid_regions = Vec::new();

            // GPUI elements are arranged by z_index but sibling elements can register overlapping
            // mouse regions. As such, hover events are only fired on overlapping elements which
            // are at the same z-index as the topmost element which overlaps with the mouse.
            match &mouse_event {
                MouseEvent::Hover(_) => {
                    let mut highest_z_index = None;
                    let mouse_position = self.mouse_position.clone();
                    for (region, z_index) in self.mouse_regions.iter().rev() {
                        // Allow mouse regions to appear transparent to hovers
                        if !region.hoverable {
                            continue;
                        }

                        let contains_mouse = region.bounds.contains_point(mouse_position);

                        if contains_mouse && highest_z_index.is_none() {
                            highest_z_index = Some(z_index);
                        }

                        // This unwrap relies on short circuiting boolean expressions
                        // The right side of the && is only executed when contains_mouse
                        // is true, and we know above that when contains_mouse is true
                        // highest_z_index is set.
                        if contains_mouse && z_index == highest_z_index.unwrap() {
                            //Ensure that hover entrance events aren't sent twice
                            if self.hovered_region_ids.insert(region.id()) {
                                valid_regions.push(region.clone());
                                if region.notify_on_hover {
                                    notified_views.insert(region.id().view_id());
                                }
                            }
                        } else {
                            // Ensure that hover exit events aren't sent twice
                            if self.hovered_region_ids.remove(&region.id()) {
                                valid_regions.push(region.clone());
                                if region.notify_on_hover {
                                    notified_views.insert(region.id().view_id());
                                }
                            }
                        }
                    }
                }

                MouseEvent::Down(_) | MouseEvent::Up(_) => {
                    for (region, _) in self.mouse_regions.iter().rev() {
                        if region.bounds.contains_point(self.mouse_position) {
                            valid_regions.push(region.clone());
                            if region.notify_on_click {
                                notified_views.insert(region.id().view_id());
                            }
                        }
                    }
                }

                MouseEvent::Click(e) => {
                    // Only raise click events if the released button is the same as the one stored
                    if self
                        .clicked_button
                        .map(|clicked_button| clicked_button == e.button)
                        .unwrap_or(false)
                    {
                        // Clear clicked regions and clicked button
                        let clicked_region_ids =
                            std::mem::replace(&mut self.clicked_region_ids, Default::default());
                        self.clicked_button = None;

                        // Find regions which still overlap with the mouse since the last MouseDown happened
                        for (mouse_region, _) in self.mouse_regions.iter().rev() {
                            if clicked_region_ids.contains(&mouse_region.id()) {
                                if mouse_region.bounds.contains_point(self.mouse_position) {
                                    valid_regions.push(mouse_region.clone());
                                }
                            }
                        }
                    }
                }

                MouseEvent::Drag(_) => {
                    for (mouse_region, _) in self.mouse_regions.iter().rev() {
                        if self.clicked_region_ids.contains(&mouse_region.id()) {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }

                MouseEvent::MoveOut(_) | MouseEvent::UpOut(_) | MouseEvent::DownOut(_) => {
                    for (mouse_region, _) in self.mouse_regions.iter().rev() {
                        // NOT contains
                        if !mouse_region.bounds.contains_point(self.mouse_position) {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }

                _ => {
                    for (mouse_region, _) in self.mouse_regions.iter().rev() {
                        // Contains
                        if mouse_region.bounds.contains_point(self.mouse_position) {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }
            }

            //3. Fire region events
            let hovered_region_ids = self.hovered_region_ids.clone();
            for valid_region in valid_regions.into_iter() {
                let mut event_cx = self.build_event_context(&mut notified_views, cx);

                mouse_event.set_region(valid_region.bounds);
                if let MouseEvent::Hover(e) = &mut mouse_event {
                    e.started = hovered_region_ids.contains(&valid_region.id())
                }
                // Handle Down events if the MouseRegion has a Click or Drag handler. This makes the api more intuitive as you would
                // not expect a MouseRegion to be transparent to Down events if it also has a Click handler.
                // This behavior can be overridden by adding a Down handler
                if let MouseEvent::Down(e) = &mouse_event {
                    let has_click = valid_region
                        .handlers
                        .contains(MouseEvent::click_disc(), Some(e.button));
                    let has_drag = valid_region
                        .handlers
                        .contains(MouseEvent::drag_disc(), Some(e.button));
                    let has_down = valid_region
                        .handlers
                        .contains(MouseEvent::down_disc(), Some(e.button));
                    if !has_down && (has_click || has_drag) {
                        event_cx.handled = true;
                    }
                }

                // `event_consumed` should only be true if there are any handlers for this event.
                let mut event_consumed = event_cx.handled;
                if let Some(callbacks) = valid_region.handlers.get(&mouse_event.handler_key()) {
                    for callback in callbacks {
                        event_cx.handled = true;
                        event_cx.with_current_view(valid_region.id().view_id(), {
                            let region_event = mouse_event.clone();
                            |cx| callback(region_event, cx)
                        });
                        event_consumed |= event_cx.handled;
                        any_event_handled |= event_cx.handled;
                    }
                }

                any_event_handled |= event_cx.handled;

                // For bubbling events, if the event was handled, don't continue dispatching.
                // This only makes sense for local events which return false from is_capturable.
                if event_consumed && mouse_event.is_capturable() {
                    break;
                }
            }
        }

        for view_id in notified_views {
            cx.notify_view(self.window_id, view_id);
        }

        any_event_handled
    }

    pub fn build_event_context<'a>(
        &'a mut self,
        notified_views: &'a mut HashSet<usize>,
        cx: &'a mut MutableAppContext,
    ) -> EventContext<'a> {
        EventContext {
            font_cache: &self.font_cache,
            text_layout_cache: &self.text_layout_cache,
            view_stack: Default::default(),
            notified_views,
            notify_count: 0,
            handled: false,
            window_id: self.window_id,
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

pub struct LayoutContext<'a> {
    window_id: usize,
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    view_stack: Vec<usize>,
    pub font_cache: &'a Arc<FontCache>,
    pub font_system: Arc<dyn FontSystem>,
    pub text_layout_cache: &'a TextLayoutCache,
    pub asset_cache: &'a AssetCache,
    pub app: &'a mut MutableAppContext,
    pub refreshing: bool,
    pub window_size: Vector2F,
    titlebar_height: f32,
    appearance: Appearance,
    hovered_region_ids: HashSet<MouseRegionId>,
    clicked_region_ids: Option<(HashSet<MouseRegionId>, MouseButton)>,
}

impl<'a> LayoutContext<'a> {
    fn layout(&mut self, view_id: usize, constraint: SizeConstraint) -> Vector2F {
        let print_error = |view_id| {
            format!(
                "{} with id {}",
                self.app.name_for_view(self.window_id, view_id).unwrap(),
                view_id,
            )
        };
        match (
            self.view_stack.last(),
            self.app.parents.get(&(self.window_id, view_id)),
        ) {
            (Some(layout_parent), Some(ParentId::View(app_parent))) => {
                if layout_parent != app_parent {
                    panic!(
                        "View {} was laid out with parent {} when it was constructed with parent {}", 
                        print_error(view_id),
                        print_error(*layout_parent),
                        print_error(*app_parent))
                }
            }
            (None, Some(ParentId::View(app_parent))) => panic!(
                "View {} was laid out without a parent when it was constructed with parent {}",
                print_error(view_id),
                print_error(*app_parent)
            ),
            (Some(layout_parent), Some(ParentId::Root)) => panic!(
                "View {} was laid out with parent {} when it was constructed as a window root",
                print_error(view_id),
                print_error(*layout_parent),
            ),
            (_, None) => panic!(
                "View {} did not have a registered parent in the app context",
                print_error(view_id),
            ),
            _ => {}
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
                clicked_region_ids: self.clicked_region_ids.clone(),
                refreshing: self.refreshing,
                appearance: self.appearance,
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
    pub window_size: Vector2F,
    pub scene: &'a mut SceneBuilder,
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

    #[inline]
    pub fn paint_stacking_context<F>(
        &mut self,
        clip_bounds: Option<RectF>,
        z_index: Option<usize>,
        f: F,
    ) where
        F: FnOnce(&mut Self),
    {
        self.scene.push_stacking_context(clip_bounds, z_index);
        f(self);
        self.scene.pop_stacking_context();
    }

    #[inline]
    pub fn paint_layer<F>(&mut self, clip_bounds: Option<RectF>, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.scene.push_layer(clip_bounds);
        f(self);
        self.scene.pop_layer();
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
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a mut MutableAppContext,
    pub window_id: usize,
    pub notify_count: usize,
    view_stack: Vec<usize>,
    handled: bool,
    notified_views: &'a mut HashSet<usize>,
}

impl<'a> EventContext<'a> {
    fn with_current_view<F, T>(&mut self, view_id: usize, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.view_stack.push(view_id);
        let result = f(self);
        self.view_stack.pop();
        result
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn view_id(&self) -> Option<usize> {
        self.view_stack.last().copied()
    }

    pub fn is_parent_view_focused(&self) -> bool {
        if let Some(parent_view_id) = self.view_stack.last() {
            self.app.focused_view_id(self.window_id) == Some(*parent_view_id)
        } else {
            false
        }
    }

    pub fn focus_parent_view(&mut self) {
        if let Some(parent_view_id) = self.view_stack.last() {
            self.app.focus(self.window_id, Some(*parent_view_id))
        }
    }

    pub fn dispatch_any_action(&mut self, action: Box<dyn Action>) {
        self.app
            .dispatch_any_action_at(self.window_id, *self.view_stack.last().unwrap(), action)
    }

    pub fn dispatch_action<A: Action>(&mut self, action: A) {
        self.dispatch_any_action(Box::new(action));
    }

    pub fn notify(&mut self) {
        self.notify_count += 1;
        if let Some(view_id) = self.view_stack.last() {
            self.notified_views.insert(*view_id);
        }
    }

    pub fn notify_count(&self) -> usize {
        self.notify_count
    }

    pub fn propagate_event(&mut self) {
        self.handled = false;
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

pub struct MeasurementContext<'a> {
    app: &'a AppContext,
    rendered_views: &'a HashMap<usize, ElementBox>,
    pub window_id: usize,
}

impl<'a> Deref for MeasurementContext<'a> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<'a> MeasurementContext<'a> {
    fn rect_for_text_range(&self, view_id: usize, range_utf16: Range<usize>) -> Option<RectF> {
        let element = self.rendered_views.get(&view_id)?;
        element.rect_for_text_range(range_utf16, self)
    }
}

pub struct DebugContext<'a> {
    rendered_views: &'a HashMap<usize, ElementBox>,
    pub font_cache: &'a FontCache,
    pub app: &'a AppContext,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Axis {
    #[default]
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

    pub fn component(&self, point: Vector2F) -> f32 {
        match self {
            Self::Horizontal => point.x(),
            Self::Vertical => point.y(),
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

impl StaticColumnCount for Axis {}
impl Bind for Axis {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        match self {
            Axis::Horizontal => "Horizontal",
            Axis::Vertical => "Vertical",
        }
        .bind(statement, start_index)
    }
}

impl Column for Axis {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(axis_text, next_index)| {
            Ok((
                match axis_text.as_str() {
                    "Horizontal" => Axis::Horizontal,
                    "Vertical" => Axis::Vertical,
                    _ => bail!("Stored serialized item kind is incorrect"),
                },
                next_index,
            ))
        })
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
    view: AnyWeakViewHandle,
    view_name: &'static str,
}

impl ChildView {
    pub fn new(view: impl Into<AnyViewHandle>, cx: &AppContext) -> Self {
        let view = view.into();
        let view_name = cx.view_ui_name(view.window_id(), view.id()).unwrap();
        Self {
            view: view.downgrade(),
            view_name,
        }
    }
}

impl Element for ChildView {
    type LayoutState = bool;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        if cx.rendered_views.contains_key(&self.view.id()) {
            let size = cx.layout(self.view.id(), constraint);
            (size, true)
        } else {
            log::error!(
                "layout called on a ChildView element whose underlying view was dropped (view_id: {}, name: {:?})",
                self.view.id(),
                self.view_name
            );
            (Vector2F::zero(), false)
        }
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        view_is_valid: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) {
        if *view_is_valid {
            cx.paint(self.view.id(), bounds.origin(), visible_bounds);
        } else {
            log::error!(
                "paint called on a ChildView element whose underlying view was dropped (view_id: {}, name: {:?})",
                self.view.id(),
                self.view_name
            );
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        view_is_valid: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        if *view_is_valid {
            cx.rect_for_text_range(self.view.id(), range_utf16)
        } else {
            log::error!(
                "rect_for_text_range called on a ChildView element whose underlying view was dropped (view_id: {}, name: {:?})",
                self.view.id(),
                self.view_name
            );
            None
        }
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
            "view": if let Some(view) = self.view.upgrade(cx.app) {
                view.debug_json(cx.app)
            } else {
                json!(null)
            },
            "child": if let Some(view) = cx.rendered_views.get(&self.view.id()) {
                view.debug(cx)
            } else {
                json!(null)
            }
        })
    }
}
