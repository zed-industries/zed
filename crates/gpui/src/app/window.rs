use crate::{
    elements::AnyRootElement,
    fonts::TextStyle,
    geometry::{rect::RectF, Size},
    json::ToJson,
    keymap_matcher::{Binding, KeymapContext, Keystroke, MatchResult},
    platform::{
        self, Appearance, CursorStyle, Event, KeyDownEvent, KeyUpEvent, ModifiersChangedEvent,
        MouseButton, MouseMovedEvent, PromptLevel, WindowBounds,
    },
    scene::{
        CursorRegion, EventHandler, MouseClick, MouseClickOut, MouseDown, MouseDownOut, MouseDrag,
        MouseEvent, MouseHover, MouseMove, MouseMoveOut, MouseScrollWheel, MouseUp, MouseUpOut,
        Scene,
    },
    text_layout::TextLayoutCache,
    util::post_inc,
    Action, AnyView, AnyViewHandle, AnyWindowHandle, AppContext, BorrowAppContext,
    BorrowWindowContext, Effect, Element, Entity, Handle, LayoutContext, MouseRegion,
    MouseRegionId, PaintContext, SceneBuilder, Subscription, View, ViewContext, ViewHandle,
    WindowInvalidation,
};
use anyhow::{anyhow, bail, Result};
use collections::{HashMap, HashSet};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use postage::oneshot;
use serde_json::json;
use smallvec::SmallVec;
use sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use std::{
    any::{type_name, Any, TypeId},
    mem,
    ops::{Deref, DerefMut, Range, Sub},
};
use taffy::{
    tree::{Measurable, MeasureFunc},
    Taffy,
};
use util::ResultExt;
use uuid::Uuid;

use super::{Reference, ViewMetadata};

pub struct Window {
    layout_engines: Vec<LayoutEngine>,
    pub(crate) root_view: Option<AnyViewHandle>,
    pub(crate) focused_view_id: Option<usize>,
    pub(crate) parents: HashMap<usize, usize>,
    pub(crate) is_active: bool,
    pub(crate) is_fullscreen: bool,
    pub(crate) invalidation: Option<WindowInvalidation>,
    pub(crate) platform_window: Box<dyn platform::Window>,
    pub(crate) rendered_views: HashMap<usize, Box<dyn AnyRootElement>>,
    pub(crate) text_style_stack: Vec<TextStyle>,
    pub(crate) theme_stack: Vec<Box<dyn Any>>,
    pub(crate) new_parents: HashMap<usize, usize>,
    pub(crate) views_to_notify_if_ancestors_change: HashMap<usize, SmallVec<[usize; 2]>>,
    titlebar_height: f32,
    appearance: Appearance,
    cursor_regions: Vec<CursorRegion>,
    mouse_regions: Vec<(MouseRegion, usize)>,
    event_handlers: Vec<EventHandler>,
    last_mouse_moved_event: Option<Event>,
    last_mouse_position: Vector2F,
    pub(crate) hovered_region_ids: Vec<MouseRegionId>,
    pub(crate) clicked_region_ids: Vec<MouseRegionId>,
    pub(crate) clicked_region: Option<(MouseRegionId, MouseButton)>,
    text_layout_cache: TextLayoutCache,
}

impl Window {
    pub fn new<V, F>(
        handle: AnyWindowHandle,
        platform_window: Box<dyn platform::Window>,
        cx: &mut AppContext,
        build_view: F,
    ) -> Self
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        let titlebar_height = platform_window.titlebar_height();
        let appearance = platform_window.appearance();
        let mut window = Self {
            layout_engines: Vec::new(),
            root_view: None,
            focused_view_id: None,
            parents: Default::default(),
            is_active: false,
            invalidation: None,
            is_fullscreen: false,
            platform_window,
            rendered_views: Default::default(),
            text_style_stack: Vec::new(),
            theme_stack: Vec::new(),
            new_parents: HashMap::default(),
            views_to_notify_if_ancestors_change: HashMap::default(),
            cursor_regions: Default::default(),
            mouse_regions: Default::default(),
            event_handlers: Default::default(),
            text_layout_cache: TextLayoutCache::new(cx.font_system.clone()),
            last_mouse_moved_event: None,
            last_mouse_position: Vector2F::zero(),
            hovered_region_ids: Default::default(),
            clicked_region_ids: Default::default(),
            clicked_region: None,
            titlebar_height,
            appearance,
        };

        let mut window_context = WindowContext::mutable(cx, &mut window, handle);
        let root_view = window_context.add_view(|cx| build_view(cx));
        if let Some(invalidation) = window_context.window.invalidation.take() {
            window_context.invalidate(invalidation, appearance);
        }
        window.focused_view_id = Some(root_view.id());
        window.root_view = Some(root_view.into_any());
        window
    }

    pub fn root_view(&self) -> &AnyViewHandle {
        &self
            .root_view
            .as_ref()
            .expect("root_view called during window construction")
    }

    pub fn take_event_handlers(&mut self) -> Vec<EventHandler> {
        mem::take(&mut self.event_handlers)
    }
}

pub struct WindowContext<'a> {
    pub(crate) app_context: Reference<'a, AppContext>,
    pub(crate) window: Reference<'a, Window>,
    pub(crate) window_handle: AnyWindowHandle,
    pub(crate) removed: bool,
}

impl Deref for WindowContext<'_> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        &self.app_context
    }
}

impl DerefMut for WindowContext<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app_context
    }
}

impl BorrowAppContext for WindowContext<'_> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        self.app_context.read_with(f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        self.app_context.update(f)
    }
}

impl BorrowWindowContext for WindowContext<'_> {
    type Result<T> = T;

    fn read_window<T, F: FnOnce(&WindowContext) -> T>(&self, handle: AnyWindowHandle, f: F) -> T {
        if self.window_handle == handle {
            f(self)
        } else {
            panic!("read_with called with id of window that does not belong to this context")
        }
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        BorrowWindowContext::read_window(self, window, f)
    }

    fn update_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        handle: AnyWindowHandle,
        f: F,
    ) -> T {
        if self.window_handle == handle {
            f(self)
        } else {
            panic!("update called with id of window that does not belong to this context")
        }
    }

    fn update_window_optional<T, F>(&mut self, handle: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        BorrowWindowContext::update_window(self, handle, f)
    }
}

impl<'a> WindowContext<'a> {
    pub fn mutable(
        app_context: &'a mut AppContext,
        window: &'a mut Window,
        handle: AnyWindowHandle,
    ) -> Self {
        Self {
            app_context: Reference::Mutable(app_context),
            window: Reference::Mutable(window),
            window_handle: handle,
            removed: false,
        }
    }

    pub fn immutable(
        app_context: &'a AppContext,
        window: &'a Window,
        handle: AnyWindowHandle,
    ) -> Self {
        Self {
            app_context: Reference::Immutable(app_context),
            window: Reference::Immutable(window),
            window_handle: handle,
            removed: false,
        }
    }

    pub fn repaint(&mut self) {
        let window = self.window();
        self.pending_effects
            .push_back(Effect::RepaintWindow { window });
    }

    pub fn layout_engine(&mut self) -> Option<&mut LayoutEngine> {
        self.window.layout_engines.last_mut()
    }

    pub fn push_layout_engine(&mut self, engine: LayoutEngine) {
        self.window.layout_engines.push(engine);
    }

    pub fn pop_layout_engine(&mut self) -> Option<LayoutEngine> {
        self.window.layout_engines.pop()
    }

    pub fn remove_window(&mut self) {
        self.removed = true;
    }

    pub fn window(&self) -> AnyWindowHandle {
        self.window_handle
    }

    pub fn app_context(&mut self) -> &mut AppContext {
        &mut self.app_context
    }

    pub fn root_view(&self) -> &AnyViewHandle {
        self.window.root_view()
    }

    pub fn window_size(&self) -> Vector2F {
        self.window.platform_window.content_size()
    }

    pub fn mouse_position(&self) -> Vector2F {
        self.window.platform_window.mouse_position()
    }

    pub fn text_layout_cache(&self) -> &TextLayoutCache {
        &self.window.text_layout_cache
    }

    pub(crate) fn update_any_view<F, T>(&mut self, view_id: usize, f: F) -> Option<T>
    where
        F: FnOnce(&mut dyn AnyView, &mut Self) -> T,
    {
        let handle = self.window_handle;
        let mut view = self.views.remove(&(handle, view_id))?;
        let result = f(view.as_mut(), self);
        self.views.insert((handle, view_id), view);
        Some(result)
    }

    pub(crate) fn update_view<V: 'static, S>(
        &mut self,
        handle: &ViewHandle<V>,
        update: &mut dyn FnMut(&mut V, &mut ViewContext<V>) -> S,
    ) -> S {
        self.update_any_view(handle.view_id, |view, cx| {
            let mut cx = ViewContext::mutable(cx, handle.view_id);
            update(
                view.as_any_mut()
                    .downcast_mut()
                    .expect("downcast is type safe"),
                &mut cx,
            )
        })
        .expect("view is already on the stack")
    }

    pub fn defer(&mut self, callback: impl 'static + FnOnce(&mut WindowContext)) {
        let handle = self.window_handle;
        self.app_context.defer(move |cx| {
            cx.update_window(handle, |cx| callback(cx));
        })
    }

    pub fn update_global<T, F, U>(&mut self, update: F) -> U
    where
        T: 'static,
        F: FnOnce(&mut T, &mut Self) -> U,
    {
        AppContext::update_global_internal(self, |global, cx| update(global, cx))
    }

    pub fn update_default_global<T, F, U>(&mut self, update: F) -> U
    where
        T: 'static + Default,
        F: FnOnce(&mut T, &mut Self) -> U,
    {
        AppContext::update_default_global_internal(self, |global, cx| update(global, cx))
    }

    pub fn subscribe<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(H, &E::Event, &mut WindowContext),
    {
        self.subscribe_internal(handle, move |emitter, event, cx| {
            callback(emitter, event, cx);
            true
        })
    }

    pub fn subscribe_internal<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(H, &E::Event, &mut WindowContext) -> bool,
    {
        let window_handle = self.window_handle;
        self.app_context
            .subscribe_internal(handle, move |emitter, event, cx| {
                cx.update_window(window_handle, |cx| callback(emitter, event, cx))
                    .unwrap_or(false)
            })
    }

    pub(crate) fn observe_window_activation<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(bool, &mut WindowContext) -> bool,
    {
        let handle = self.window_handle;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.pending_effects
            .push_back(Effect::WindowActivationObservation {
                window: handle,
                subscription_id,
                callback: Box::new(callback),
            });
        Subscription::WindowActivationObservation(
            self.window_activation_observations
                .subscribe(handle, subscription_id),
        )
    }

    pub(crate) fn observe_fullscreen<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(bool, &mut WindowContext) -> bool,
    {
        let window = self.window_handle;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.pending_effects
            .push_back(Effect::WindowFullscreenObservation {
                window,
                subscription_id,
                callback: Box::new(callback),
            });
        Subscription::WindowActivationObservation(
            self.window_activation_observations
                .subscribe(window, subscription_id),
        )
    }

    pub(crate) fn observe_window_bounds<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(WindowBounds, Uuid, &mut WindowContext) -> bool,
    {
        let window = self.window_handle;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.pending_effects
            .push_back(Effect::WindowBoundsObservation {
                window,
                subscription_id,
                callback: Box::new(callback),
            });
        Subscription::WindowBoundsObservation(
            self.window_bounds_observations
                .subscribe(window, subscription_id),
        )
    }

    pub fn observe_keystrokes<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static
            + FnMut(&Keystroke, &MatchResult, Option<&Box<dyn Action>>, &mut WindowContext) -> bool,
    {
        let window = self.window_handle;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.keystroke_observations
            .add_callback(window, subscription_id, Box::new(callback));
        Subscription::KeystrokeObservation(
            self.keystroke_observations
                .subscribe(window, subscription_id),
        )
    }

    pub(crate) fn available_actions(
        &self,
        view_id: usize,
    ) -> Vec<(&'static str, Box<dyn Action>, SmallVec<[Binding; 1]>)> {
        let handle = self.window_handle;
        let mut contexts = Vec::new();
        let mut handler_depths_by_action_id = HashMap::<TypeId, usize>::default();
        for (depth, view_id) in self.ancestors(view_id).enumerate() {
            if let Some(view_metadata) = self.views_metadata.get(&(handle, view_id)) {
                contexts.push(view_metadata.keymap_context.clone());
                if let Some(actions) = self.actions.get(&view_metadata.type_id) {
                    handler_depths_by_action_id
                        .extend(actions.keys().copied().map(|action_id| (action_id, depth)));
                }
            } else {
                log::error!(
                    "view {} not found when computing available actions",
                    view_id
                );
            }
        }

        handler_depths_by_action_id.extend(
            self.global_actions
                .keys()
                .copied()
                .map(|action_id| (action_id, contexts.len())),
        );

        self.action_deserializers
            .iter()
            .filter_map(move |(name, (action_id, deserialize))| {
                if let Some(action_depth) = handler_depths_by_action_id.get(action_id).copied() {
                    let action = deserialize(serde_json::Value::Object(Default::default())).ok()?;
                    let bindings = self
                        .keystroke_matcher
                        .bindings_for_action(*action_id)
                        .filter(|b| {
                            action.eq(b.action())
                                && (0..=action_depth)
                                    .any(|depth| b.match_context(&contexts[depth..]))
                        })
                        .cloned()
                        .collect();
                    Some((*name, action, bindings))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn dispatch_keystroke(&mut self, keystroke: &Keystroke) -> bool {
        let handle = self.window_handle;
        if let Some(focused_view_id) = self.focused_view_id() {
            let dispatch_path = self
                .ancestors(focused_view_id)
                .filter_map(|view_id| {
                    self.views_metadata
                        .get(&(handle, view_id))
                        .map(|view| (view_id, view.keymap_context.clone()))
                })
                .collect();

            let match_result = self
                .keystroke_matcher
                .push_keystroke(keystroke.clone(), dispatch_path);
            let mut handled_by = None;

            let keystroke_handled = match &match_result {
                MatchResult::None => false,
                MatchResult::Pending => true,
                MatchResult::Matches(matches) => {
                    for (view_id, action) in matches {
                        if self.dispatch_action(Some(*view_id), action.as_ref()) {
                            self.keystroke_matcher.clear_pending();
                            handled_by = Some(action.boxed_clone());
                            break;
                        }
                    }
                    handled_by.is_some()
                }
            };

            self.keystroke(handle, keystroke.clone(), handled_by, match_result.clone());
            keystroke_handled
        } else {
            self.keystroke(handle, keystroke.clone(), None, MatchResult::None);
            false
        }
    }

    pub(crate) fn dispatch_event(&mut self, event: Event, event_reused: bool) -> bool {
        if !event_reused {
            self.dispatch_to_new_event_handlers(&event);
        }

        let mut mouse_events = SmallVec::<[_; 2]>::new();
        let mut notified_views: HashSet<usize> = Default::default();
        let handle = self.window_handle;

        // 1. Handle platform event. Keyboard events get dispatched immediately, while mouse events
        //    get mapped into the mouse-specific MouseEvent type.
        //  -> These are usually small: [Mouse Down] or [Mouse up, Click] or [Mouse Moved, Mouse Dragged?]
        //  -> Also updates mouse-related state
        match &event {
            Event::KeyDown(e) => return self.dispatch_key_down(e),

            Event::KeyUp(e) => return self.dispatch_key_up(e),

            Event::ModifiersChanged(e) => return self.dispatch_modifiers_changed(e),

            Event::MouseDown(e) => {
                // Click events are weird because they can be fired after a drag event.
                // MDN says that browsers handle this by starting from 'the most
                // specific ancestor element that contained both [positions]'
                // So we need to store the overlapping regions on mouse down.

                // If there is already region being clicked, don't replace it.
                if self.window.clicked_region.is_none() {
                    self.window.clicked_region_ids = self
                        .window
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

                    let mut highest_z_index = 0;
                    let mut clicked_region_id = None;
                    for (region, z_index) in self.window.mouse_regions.iter() {
                        if region.bounds.contains_point(e.position) && *z_index >= highest_z_index {
                            highest_z_index = *z_index;
                            clicked_region_id = Some(region.id());
                        }
                    }

                    self.window.clicked_region =
                        clicked_region_id.map(|region_id| (region_id, e.button));
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

                // Synthesize one last drag event to end the drag
                mouse_events.push(MouseEvent::Drag(MouseDrag {
                    region: Default::default(),
                    prev_mouse_position: self.window.last_mouse_position,
                    platform_event: MouseMovedEvent {
                        position: e.position,
                        pressed_button: Some(e.button),
                        modifiers: e.modifiers,
                    },
                    end: true,
                }));
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
                mouse_events.push(MouseEvent::ClickOut(MouseClickOut {
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
                for region in self.window.cursor_regions.iter().rev() {
                    if region.bounds.contains_point(*position) {
                        style_to_assign = region.style;
                        break;
                    }
                }

                if pressed_button.is_none()
                    && self
                        .window
                        .platform_window
                        .is_topmost_for_position(*position)
                {
                    self.platform().set_cursor_style(style_to_assign);
                }

                if !event_reused {
                    if pressed_button.is_some() {
                        mouse_events.push(MouseEvent::Drag(MouseDrag {
                            region: Default::default(),
                            prev_mouse_position: self.window.last_mouse_position,
                            platform_event: e.clone(),
                            end: false,
                        }));
                    } else if let Some((_, clicked_button)) = self.window.clicked_region {
                        mouse_events.push(MouseEvent::Drag(MouseDrag {
                            region: Default::default(),
                            prev_mouse_position: self.window.last_mouse_position,
                            platform_event: e.clone(),
                            end: true,
                        }));

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

                self.window.last_mouse_moved_event = Some(event.clone());
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
                );
            }

            Event::ScrollWheel(e) => mouse_events.push(MouseEvent::ScrollWheel(MouseScrollWheel {
                region: Default::default(),
                platform_event: e.clone(),
            })),
        }

        if let Some(position) = event.position() {
            self.window.last_mouse_position = position;
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
                    let mouse_position = self.mouse_position();
                    let window = &mut *self.window;
                    let prev_hovered_regions = mem::take(&mut window.hovered_region_ids);
                    for (region, z_index) in window.mouse_regions.iter().rev() {
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
                            if let Err(ix) = window.hovered_region_ids.binary_search(&region.id()) {
                                window.hovered_region_ids.insert(ix, region.id());
                            }
                            // window.hovered_region_ids.insert(region.id());
                            if !prev_hovered_regions.contains(&region.id()) {
                                valid_regions.push(region.clone());
                                if region.notify_on_hover {
                                    notified_views.insert(region.id().view_id());
                                }
                            }
                        } else {
                            // Ensure that hover exit events aren't sent twice
                            if prev_hovered_regions.contains(&region.id()) {
                                valid_regions.push(region.clone());
                                if region.notify_on_hover {
                                    notified_views.insert(region.id().view_id());
                                }
                            }
                        }
                    }
                }

                MouseEvent::Down(_) | MouseEvent::Up(_) => {
                    for (region, _) in self.window.mouse_regions.iter().rev() {
                        if region.bounds.contains_point(self.mouse_position()) {
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
                        .window
                        .clicked_region
                        .map(|(_, clicked_button)| clicked_button == e.button)
                        .unwrap_or(false)
                    {
                        // Clear clicked regions and clicked button
                        let clicked_region_ids = std::mem::replace(
                            &mut self.window.clicked_region_ids,
                            Default::default(),
                        );
                        self.window.clicked_region = None;

                        // Find regions which still overlap with the mouse since the last MouseDown happened
                        for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                            if clicked_region_ids.contains(&mouse_region.id()) {
                                if mouse_region.bounds.contains_point(self.mouse_position()) {
                                    valid_regions.push(mouse_region.clone());
                                } else {
                                    // Let the view know that it hasn't been clicked anymore
                                    if mouse_region.notify_on_click {
                                        notified_views.insert(mouse_region.id().view_id());
                                    }
                                }
                            }
                        }
                    }
                }

                MouseEvent::Drag(_) => {
                    for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                        if self.window.clicked_region_ids.contains(&mouse_region.id()) {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }

                MouseEvent::MoveOut(_)
                | MouseEvent::UpOut(_)
                | MouseEvent::DownOut(_)
                | MouseEvent::ClickOut(_) => {
                    for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                        // NOT contains
                        if !mouse_region.bounds.contains_point(self.mouse_position()) {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }

                _ => {
                    for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                        // Contains
                        if mouse_region.bounds.contains_point(self.mouse_position()) {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }
            }

            //3. Fire region events
            let hovered_region_ids = self.window.hovered_region_ids.clone();
            for valid_region in valid_regions.into_iter() {
                let mut handled = false;
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
                        handled = true;
                    }
                }

                // `event_consumed` should only be true if there are any handlers for this event.
                let mut event_consumed = handled;
                if let Some(callbacks) = valid_region.handlers.get(&mouse_event.handler_key()) {
                    for callback in callbacks {
                        handled = true;
                        let view_id = valid_region.id().view_id();
                        self.update_any_view(view_id, |view, cx| {
                            handled = callback(mouse_event.clone(), view.as_any_mut(), cx, view_id);
                        });
                        event_consumed |= handled;
                        any_event_handled |= handled;
                    }
                }

                any_event_handled |= handled;

                // For bubbling events, if the event was handled, don't continue dispatching.
                // This only makes sense for local events which return false from is_capturable.
                if event_consumed && mouse_event.is_capturable() {
                    break;
                }
            }
        }

        for view_id in notified_views {
            self.notify_view(handle, view_id);
        }

        any_event_handled
    }

    fn dispatch_to_new_event_handlers(&mut self, event: &Event) {
        if let Some(mouse_event) = event.mouse_event() {
            let event_handlers = self.window.take_event_handlers();
            for event_handler in event_handlers.iter().rev() {
                if event_handler.event_type == mouse_event.type_id() {
                    (event_handler.handler)(mouse_event, self);
                }
            }
            self.window.event_handlers = event_handlers;
        }
    }

    pub(crate) fn dispatch_key_down(&mut self, event: &KeyDownEvent) -> bool {
        let handle = self.window_handle;
        if let Some(focused_view_id) = self.window.focused_view_id {
            for view_id in self.ancestors(focused_view_id).collect::<Vec<_>>() {
                if let Some(mut view) = self.views.remove(&(handle, view_id)) {
                    let handled = view.key_down(event, self, view_id);
                    self.views.insert((handle, view_id), view);
                    if handled {
                        return true;
                    }
                } else {
                    log::error!("view {} does not exist", view_id)
                }
            }
        }

        false
    }

    pub(crate) fn dispatch_key_up(&mut self, event: &KeyUpEvent) -> bool {
        let handle = self.window_handle;
        if let Some(focused_view_id) = self.window.focused_view_id {
            for view_id in self.ancestors(focused_view_id).collect::<Vec<_>>() {
                if let Some(mut view) = self.views.remove(&(handle, view_id)) {
                    let handled = view.key_up(event, self, view_id);
                    self.views.insert((handle, view_id), view);
                    if handled {
                        return true;
                    }
                } else {
                    log::error!("view {} does not exist", view_id)
                }
            }
        }

        false
    }

    pub(crate) fn dispatch_modifiers_changed(&mut self, event: &ModifiersChangedEvent) -> bool {
        let handle = self.window_handle;
        if let Some(focused_view_id) = self.window.focused_view_id {
            for view_id in self.ancestors(focused_view_id).collect::<Vec<_>>() {
                if let Some(mut view) = self.views.remove(&(handle, view_id)) {
                    let handled = view.modifiers_changed(event, self, view_id);
                    self.views.insert((handle, view_id), view);
                    if handled {
                        return true;
                    }
                } else {
                    log::error!("view {} does not exist", view_id)
                }
            }
        }

        false
    }

    pub fn invalidate(&mut self, mut invalidation: WindowInvalidation, appearance: Appearance) {
        self.start_frame();
        self.window.appearance = appearance;
        for view_id in &invalidation.removed {
            invalidation.updated.remove(view_id);
            self.window.rendered_views.remove(view_id);
        }
        for view_id in &invalidation.updated {
            let titlebar_height = self.window.titlebar_height;
            let element = self
                .render_view(RenderParams {
                    view_id: *view_id,
                    titlebar_height,
                    refreshing: false,
                    appearance,
                })
                .unwrap();
            self.window.rendered_views.insert(*view_id, element);
        }
    }

    pub fn render_view(&mut self, params: RenderParams) -> Result<Box<dyn AnyRootElement>> {
        let handle = self.window_handle;
        let view_id = params.view_id;
        let mut view = self
            .views
            .remove(&(handle, view_id))
            .ok_or_else(|| anyhow!("view not found"))?;
        let element = view.render(self, view_id);
        self.views.insert((handle, view_id), view);
        Ok(element)
    }

    pub fn layout(&mut self, refreshing: bool) -> Result<HashMap<usize, usize>> {
        let window_size = self.window.platform_window.content_size();
        let root_view_id = self.window.root_view().id();

        let mut rendered_root = self.window.rendered_views.remove(&root_view_id).unwrap();

        rendered_root.layout(SizeConstraint::strict(window_size), refreshing, self)?;

        let views_to_notify_if_ancestors_change =
            mem::take(&mut self.window.views_to_notify_if_ancestors_change);
        for (view_id, view_ids_to_notify) in views_to_notify_if_ancestors_change {
            let mut current_view_id = view_id;
            loop {
                let old_parent_id = self.window.parents.get(&current_view_id);
                let new_parent_id = self.window.new_parents.get(&current_view_id);
                if old_parent_id.is_none() && new_parent_id.is_none() {
                    break;
                } else if old_parent_id == new_parent_id {
                    current_view_id = *old_parent_id.unwrap();
                } else {
                    let handle = self.window_handle;
                    for view_id_to_notify in view_ids_to_notify {
                        self.notify_view(handle, view_id_to_notify);
                    }
                    break;
                }
            }
        }

        let new_parents = mem::take(&mut self.window.new_parents);
        let old_parents = mem::replace(&mut self.window.parents, new_parents);
        self.window
            .rendered_views
            .insert(root_view_id, rendered_root);
        Ok(old_parents)
    }

    pub fn paint(&mut self) -> Result<Scene> {
        let window_size = self.window.platform_window.content_size();
        let scale_factor = self.window.platform_window.scale_factor();

        let root_view_id = self.window.root_view().id();
        let mut rendered_root = self.window.rendered_views.remove(&root_view_id).unwrap();

        let mut scene_builder = SceneBuilder::new(scale_factor);
        rendered_root.paint(
            &mut scene_builder,
            Vector2F::zero(),
            RectF::from_points(Vector2F::zero(), window_size),
            self,
        )?;
        self.window
            .rendered_views
            .insert(root_view_id, rendered_root);

        self.window.text_layout_cache.finish_frame();
        let mut scene = scene_builder.build();
        self.window.cursor_regions = scene.cursor_regions();
        self.window.mouse_regions = scene.mouse_regions();
        self.window.event_handlers = scene.take_event_handlers();

        if self.window_is_active() {
            if let Some(event) = self.window.last_mouse_moved_event.clone() {
                self.dispatch_event(event, true);
            }
        }

        Ok(scene)
    }

    pub fn root_element(&self) -> &Box<dyn AnyRootElement> {
        let view_id = self.window.root_view().id();
        self.window.rendered_views.get(&view_id).unwrap()
    }

    pub fn rect_for_text_range(&self, range_utf16: Range<usize>) -> Option<RectF> {
        let focused_view_id = self.window.focused_view_id?;
        self.window
            .rendered_views
            .get(&focused_view_id)?
            .rect_for_text_range(range_utf16, self)
            .log_err()
            .flatten()
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.window.platform_window.set_title(title);
    }

    pub fn set_window_edited(&mut self, edited: bool) {
        self.window.platform_window.set_edited(edited);
    }

    pub fn is_topmost_window_for_position(&self, position: Vector2F) -> bool {
        self.window
            .platform_window
            .is_topmost_for_position(position)
    }

    pub fn activate_window(&self) {
        self.window.platform_window.activate();
    }

    pub fn window_is_active(&self) -> bool {
        self.window.is_active
    }

    pub fn window_is_fullscreen(&self) -> bool {
        self.window.is_fullscreen
    }

    pub(crate) fn dispatch_action(&mut self, view_id: Option<usize>, action: &dyn Action) -> bool {
        if let Some(view_id) = view_id {
            self.halt_action_dispatch = false;
            self.visit_dispatch_path(view_id, |view_id, capture_phase, cx| {
                cx.update_any_view(view_id, |view, cx| {
                    let type_id = view.as_any().type_id();
                    if let Some((name, mut handlers)) = cx
                        .actions_mut(capture_phase)
                        .get_mut(&type_id)
                        .and_then(|h| h.remove_entry(&action.id()))
                    {
                        for handler in handlers.iter_mut().rev() {
                            cx.halt_action_dispatch = true;
                            handler(view, action, cx, view_id);
                            if cx.halt_action_dispatch {
                                break;
                            }
                        }
                        cx.actions_mut(capture_phase)
                            .get_mut(&type_id)
                            .unwrap()
                            .insert(name, handlers);
                    }
                });

                !cx.halt_action_dispatch
            });
        }

        if !self.halt_action_dispatch {
            self.halt_action_dispatch = self.dispatch_global_action_any(action);
        }

        self.pending_effects
            .push_back(Effect::ActionDispatchNotification {
                action_id: action.id(),
            });
        self.halt_action_dispatch
    }

    /// Returns an iterator over all of the view ids from the passed view up to the root of the window
    /// Includes the passed view itself
    pub(crate) fn ancestors(&self, mut view_id: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(view_id)
            .into_iter()
            .chain(std::iter::from_fn(move || {
                if let Some(parent_id) = self.window.parents.get(&view_id) {
                    view_id = *parent_id;
                    Some(view_id)
                } else {
                    None
                }
            }))
    }

    // Traverses the parent tree. Walks down the tree toward the passed
    // view calling visit with true. Then walks back up the tree calling visit with false.
    // If `visit` returns false this function will immediately return.
    fn visit_dispatch_path(
        &mut self,
        view_id: usize,
        mut visit: impl FnMut(usize, bool, &mut WindowContext) -> bool,
    ) {
        // List of view ids from the leaf to the root of the window
        let path = self.ancestors(view_id).collect::<Vec<_>>();

        // Walk down from the root to the leaf calling visit with capture_phase = true
        for view_id in path.iter().rev() {
            if !visit(*view_id, true, self) {
                return;
            }
        }

        // Walk up from the leaf to the root calling visit with capture_phase = false
        for view_id in path.iter() {
            if !visit(*view_id, false, self) {
                return;
            }
        }
    }

    pub fn focused_view_id(&self) -> Option<usize> {
        self.window.focused_view_id
    }

    pub fn focus(&mut self, view_id: Option<usize>) {
        self.app_context.focus(self.window_handle, view_id);
    }

    pub fn window_bounds(&self) -> WindowBounds {
        self.window.platform_window.bounds()
    }

    pub fn titlebar_height(&self) -> f32 {
        self.window.titlebar_height
    }

    pub fn window_appearance(&self) -> Appearance {
        self.window.appearance
    }

    pub fn window_display_uuid(&self) -> Option<Uuid> {
        self.window.platform_window.screen().display_uuid()
    }

    pub fn show_character_palette(&self) {
        self.window.platform_window.show_character_palette();
    }

    pub fn minimize_window(&self) {
        self.window.platform_window.minimize();
    }

    pub fn zoom_window(&self) {
        self.window.platform_window.zoom();
    }

    pub fn toggle_full_screen(&self) {
        self.window.platform_window.toggle_full_screen();
    }

    pub fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        self.window.platform_window.prompt(level, msg, answers)
    }

    pub fn add_view<T, F>(&mut self, build_view: F) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.add_option_view(|cx| Some(build_view(cx))).unwrap()
    }

    pub fn add_option_view<T, F>(&mut self, build_view: F) -> Option<ViewHandle<T>>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> Option<T>,
    {
        let handle = self.window_handle;
        let view_id = post_inc(&mut self.next_id);
        let mut cx = ViewContext::mutable(self, view_id);
        let handle = if let Some(view) = build_view(&mut cx) {
            let mut keymap_context = KeymapContext::default();
            view.update_keymap_context(&mut keymap_context, cx.app_context());
            self.views_metadata.insert(
                (handle, view_id),
                ViewMetadata {
                    type_id: TypeId::of::<T>(),
                    keymap_context,
                },
            );
            self.views.insert((handle, view_id), Box::new(view));
            self.window
                .invalidation
                .get_or_insert_with(Default::default)
                .updated
                .insert(view_id);
            Some(ViewHandle::new(handle, view_id, &self.ref_counts))
        } else {
            None
        };
        handle
    }

    pub fn text_style(&self) -> TextStyle {
        self.window
            .text_style_stack
            .last()
            .cloned()
            .unwrap_or(TextStyle::default(&self.font_cache))
    }

    pub fn push_text_style(&mut self, style: TextStyle) {
        self.window.text_style_stack.push(style);
    }

    pub fn pop_text_style(&mut self) {
        self.window.text_style_stack.pop();
    }

    pub fn theme<T: 'static>(&self) -> &T {
        self.window
            .theme_stack
            .iter()
            .rev()
            .find_map(|theme| theme.downcast_ref())
            .ok_or_else(|| anyhow!("no theme provided of type {}", type_name::<T>()))
            .unwrap()
    }

    pub fn push_theme<T: 'static>(&mut self, theme: T) {
        self.window.theme_stack.push(Box::new(theme));
    }

    pub fn pop_theme(&mut self) {
        self.window.theme_stack.pop();
    }
}

#[derive(Default)]
pub struct LayoutEngine(Taffy);
pub use taffy::style::Style as LayoutStyle;

impl LayoutEngine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node<C>(&mut self, style: LayoutStyle, children: C) -> Result<LayoutId>
    where
        C: IntoIterator<Item = LayoutId>,
    {
        let children = children.into_iter().collect::<Vec<_>>();
        if children.is_empty() {
            Ok(self.0.new_leaf(style)?)
        } else {
            Ok(self.0.new_with_children(style, &children)?)
        }
    }

    pub fn add_measured_node<F>(&mut self, style: LayoutStyle, measure: F) -> Result<LayoutId>
    where
        F: Fn(MeasureParams) -> Size<f32> + Sync + Send + 'static,
    {
        Ok(self
            .0
            .new_leaf_with_measure(style, MeasureFunc::Boxed(Box::new(MeasureFn(measure))))?)
    }

    pub fn compute_layout(&mut self, root: LayoutId, available_space: Vector2F) -> Result<()> {
        self.0.compute_layout(
            root,
            taffy::geometry::Size {
                width: available_space.x().into(),
                height: available_space.y().into(),
            },
        )?;
        Ok(())
    }

    pub fn computed_layout(&mut self, node: LayoutId) -> Result<Layout> {
        Ok(Layout::from(self.0.layout(node)?))
    }
}

pub struct MeasureFn<F>(F);

impl<F: Send + Sync> Measurable for MeasureFn<F>
where
    F: Fn(MeasureParams) -> Size<f32>,
{
    fn measure(
        &self,
        known_dimensions: taffy::prelude::Size<Option<f32>>,
        available_space: taffy::prelude::Size<taffy::style::AvailableSpace>,
    ) -> taffy::prelude::Size<f32> {
        (self.0)(MeasureParams {
            known_dimensions: known_dimensions.into(),
            available_space: available_space.into(),
        })
        .into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Layout {
    pub bounds: RectF,
    pub order: u32,
}

pub struct MeasureParams {
    pub known_dimensions: Size<Option<f32>>,
    pub available_space: Size<AvailableSpace>,
}

#[derive(Clone)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Pixels(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl Default for AvailableSpace {
    fn default() -> Self {
        Self::Pixels(0.)
    }
}

impl From<taffy::prelude::AvailableSpace> for AvailableSpace {
    fn from(value: taffy::prelude::AvailableSpace) -> Self {
        match value {
            taffy::prelude::AvailableSpace::Definite(pixels) => Self::Pixels(pixels),
            taffy::prelude::AvailableSpace::MinContent => Self::MinContent,
            taffy::prelude::AvailableSpace::MaxContent => Self::MaxContent,
        }
    }
}

impl From<&taffy::tree::Layout> for Layout {
    fn from(value: &taffy::tree::Layout) -> Self {
        Self {
            bounds: RectF::new(
                vec2f(value.location.x, value.location.y),
                vec2f(value.size.width, value.size.height),
            ),
            order: value.order,
        }
    }
}

pub type LayoutId = taffy::prelude::NodeId;

pub struct RenderParams {
    pub view_id: usize,
    pub titlebar_height: f32,
    pub refreshing: bool,
    pub appearance: Appearance,
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

pub trait RectFExt {
    fn length_along(self, axis: Axis) -> f32;
}

impl RectFExt for RectF {
    fn length_along(self, axis: Axis) -> f32 {
        match axis {
            Axis::Horizontal => self.width(),
            Axis::Vertical => self.height(),
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
    pub fn loose(max: Vector2F) -> Self {
        Self {
            min: Vector2F::zero(),
            max,
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

impl Sub<Vector2F> for SizeConstraint {
    type Output = SizeConstraint;

    fn sub(self, rhs: Vector2F) -> SizeConstraint {
        SizeConstraint {
            min: self.min - rhs,
            max: self.max - rhs,
        }
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

#[derive(Clone)]
pub struct ChildView {
    view_id: usize,
    view_name: &'static str,
}

impl ChildView {
    pub fn new(view: &AnyViewHandle, cx: &AppContext) -> Self {
        let view_name = cx.view_ui_name(view.window, view.id()).unwrap();
        Self {
            view_id: view.id(),
            view_name,
        }
    }
}

impl<V: 'static> Element<V> for ChildView {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        if let Some(mut rendered_view) = cx.window.rendered_views.remove(&self.view_id) {
            let parent_id = cx.view_id();
            cx.window.new_parents.insert(self.view_id, parent_id);
            let size = rendered_view
                .layout(constraint, cx.refreshing, cx.view_context)
                .log_err()
                .unwrap_or(Vector2F::zero());
            cx.window.rendered_views.insert(self.view_id, rendered_view);
            (size, ())
        } else {
            log::error!(
                "layout called on a ChildView element whose underlying view was dropped (view_id: {}, name: {:?})",
                self.view_id,
                self.view_name
            );
            (Vector2F::zero(), ())
        }
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut V,
        cx: &mut PaintContext<V>,
    ) {
        if let Some(mut rendered_view) = cx.window.rendered_views.remove(&self.view_id) {
            rendered_view
                .paint(scene, bounds.origin(), visible_bounds, cx)
                .log_err();
            cx.window.rendered_views.insert(self.view_id, rendered_view);
        } else {
            log::error!(
                "paint called on a ChildView element whose underlying view was dropped (view_id: {}, name: {:?})",
                self.view_id,
                self.view_name
            );
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        if let Some(rendered_view) = cx.window.rendered_views.get(&self.view_id) {
            rendered_view
                .rect_for_text_range(range_utf16, &cx.window_context)
                .log_err()
                .flatten()
        } else {
            log::error!(
                "rect_for_text_range called on a ChildView element whose underlying view was dropped (view_id: {}, name: {:?})",
                self.view_id,
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
        _: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        json!({
            "type": "ChildView",
            "bounds": bounds.to_json(),
            "child": if let Some(element) = cx.window.rendered_views.get(&self.view_id) {
                element.debug(&cx.window_context).log_err().unwrap_or_else(|| json!(null))
            } else {
                json!(null)
            }
        })
    }
}
