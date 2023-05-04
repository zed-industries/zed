use crate::{
    elements::AnyRootElement,
    geometry::rect::RectF,
    json::ToJson,
    keymap_matcher::{Binding, KeymapContext, Keystroke, MatchResult},
    platform::{
        self, Appearance, CursorStyle, Event, KeyDownEvent, KeyUpEvent, ModifiersChangedEvent,
        MouseButton, MouseMovedEvent, PromptLevel, WindowBounds,
    },
    scene::{
        CursorRegion, MouseClick, MouseDown, MouseDownOut, MouseDrag, MouseEvent, MouseHover,
        MouseMove, MouseMoveOut, MouseScrollWheel, MouseUp, MouseUpOut, Scene,
    },
    text_layout::TextLayoutCache,
    util::post_inc,
    Action, AnyView, AnyViewHandle, AppContext, BorrowAppContext, BorrowWindowContext, Effect,
    Element, Entity, Handle, MouseRegion, MouseRegionId, ParentId, SceneBuilder, Subscription,
    View, ViewContext, ViewHandle, WindowInvalidation,
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
    any::TypeId,
    ops::{Deref, DerefMut, Range},
};
use util::ResultExt;
use uuid::Uuid;

use super::{Reference, ViewMetadata};

pub struct Window {
    pub(crate) root_view: Option<AnyViewHandle>,
    pub(crate) focused_view_id: Option<usize>,
    pub(crate) is_active: bool,
    pub(crate) is_fullscreen: bool,
    pub(crate) invalidation: Option<WindowInvalidation>,
    pub(crate) platform_window: Box<dyn platform::Window>,
    pub(crate) rendered_views: HashMap<usize, Box<dyn AnyRootElement>>,
    titlebar_height: f32,
    appearance: Appearance,
    cursor_regions: Vec<CursorRegion>,
    mouse_regions: Vec<(MouseRegion, usize)>,
    last_mouse_moved_event: Option<Event>,
    pub(crate) hovered_region_ids: HashSet<MouseRegionId>,
    pub(crate) clicked_region_ids: HashSet<MouseRegionId>,
    pub(crate) clicked_button: Option<MouseButton>,
    mouse_position: Vector2F,
    text_layout_cache: TextLayoutCache,
}

impl Window {
    pub fn new<V, F>(
        window_id: usize,
        platform_window: Box<dyn platform::Window>,
        cx: &mut AppContext,
        build_view: F,
    ) -> Self
    where
        F: FnOnce(&mut ViewContext<V>) -> V,
        V: View,
    {
        let titlebar_height = platform_window.titlebar_height();
        let appearance = platform_window.appearance();
        let mut window = Self {
            root_view: None,
            focused_view_id: None,
            is_active: false,
            invalidation: None,
            is_fullscreen: false,
            platform_window,
            rendered_views: Default::default(),
            cursor_regions: Default::default(),
            mouse_regions: Default::default(),
            text_layout_cache: TextLayoutCache::new(cx.font_system.clone()),
            last_mouse_moved_event: None,
            hovered_region_ids: Default::default(),
            clicked_region_ids: Default::default(),
            clicked_button: None,
            mouse_position: vec2f(0., 0.),
            titlebar_height,
            appearance,
        };

        let mut window_context = WindowContext::mutable(cx, &mut window, window_id);
        let root_view = window_context
            .build_and_insert_view(ParentId::Root, |cx| Some(build_view(cx)))
            .unwrap();
        if let Some(mut invalidation) = window_context.window.invalidation.take() {
            window_context.invalidate(&mut invalidation, appearance);
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
}

pub struct WindowContext<'a> {
    pub(crate) app_context: Reference<'a, AppContext>,
    pub(crate) window: Reference<'a, Window>,
    pub(crate) window_id: usize,
    pub(crate) refreshing: bool,
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
    fn read_with<T, F: FnOnce(&WindowContext) -> T>(&self, window_id: usize, f: F) -> T {
        if self.window_id == window_id {
            f(self)
        } else {
            panic!("read_with called with id of window that does not belong to this context")
        }
    }

    fn update<T, F: FnOnce(&mut WindowContext) -> T>(&mut self, window_id: usize, f: F) -> T {
        if self.window_id == window_id {
            f(self)
        } else {
            panic!("update called with id of window that does not belong to this context")
        }
    }
}

impl<'a> WindowContext<'a> {
    pub fn mutable(
        app_context: &'a mut AppContext,
        window: &'a mut Window,
        window_id: usize,
    ) -> Self {
        Self {
            app_context: Reference::Mutable(app_context),
            window: Reference::Mutable(window),
            window_id,
            refreshing: false,
            removed: false,
        }
    }

    pub fn immutable(app_context: &'a AppContext, window: &'a Window, window_id: usize) -> Self {
        Self {
            app_context: Reference::Immutable(app_context),
            window: Reference::Immutable(window),
            window_id,
            refreshing: false,
            removed: false,
        }
    }

    pub fn remove_window(&mut self) {
        self.removed = true;
    }

    pub fn window_id(&self) -> usize {
        self.window_id
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

    pub fn text_layout_cache(&self) -> &TextLayoutCache {
        &self.window.text_layout_cache
    }

    pub(crate) fn update_any_view<F, T>(&mut self, view_id: usize, f: F) -> Option<T>
    where
        F: FnOnce(&mut dyn AnyView, &mut Self) -> T,
    {
        let window_id = self.window_id;
        let mut view = self.views.remove(&(window_id, view_id))?;
        let result = f(view.as_mut(), self);
        self.views.insert((window_id, view_id), view);
        Some(result)
    }

    pub(crate) fn update_view<T, S>(
        &mut self,
        handle: &ViewHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
    ) -> S
    where
        T: View,
    {
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
        let window_id = self.window_id;
        self.app_context.defer(move |cx| {
            cx.update_window(window_id, |cx| callback(cx));
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
        let window_id = self.window_id;
        self.app_context
            .subscribe_internal(handle, move |emitter, event, cx| {
                cx.update_window(window_id, |cx| callback(emitter, event, cx))
                    .unwrap_or(false)
            })
    }

    pub(crate) fn observe_window_activation<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(bool, &mut WindowContext) -> bool,
    {
        let window_id = self.window_id;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.pending_effects
            .push_back(Effect::WindowActivationObservation {
                window_id,
                subscription_id,
                callback: Box::new(callback),
            });
        Subscription::WindowActivationObservation(
            self.window_activation_observations
                .subscribe(window_id, subscription_id),
        )
    }

    pub(crate) fn observe_fullscreen<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(bool, &mut WindowContext) -> bool,
    {
        let window_id = self.window_id;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.pending_effects
            .push_back(Effect::WindowFullscreenObservation {
                window_id,
                subscription_id,
                callback: Box::new(callback),
            });
        Subscription::WindowActivationObservation(
            self.window_activation_observations
                .subscribe(window_id, subscription_id),
        )
    }

    pub(crate) fn observe_window_bounds<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(WindowBounds, Uuid, &mut WindowContext) -> bool,
    {
        let window_id = self.window_id;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.pending_effects
            .push_back(Effect::WindowBoundsObservation {
                window_id,
                subscription_id,
                callback: Box::new(callback),
            });
        Subscription::WindowBoundsObservation(
            self.window_bounds_observations
                .subscribe(window_id, subscription_id),
        )
    }

    pub fn observe_keystrokes<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static
            + FnMut(&Keystroke, &MatchResult, Option<&Box<dyn Action>>, &mut WindowContext) -> bool,
    {
        let window_id = self.window_id;
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.keystroke_observations
            .add_callback(window_id, subscription_id, Box::new(callback));
        Subscription::KeystrokeObservation(
            self.keystroke_observations
                .subscribe(window_id, subscription_id),
        )
    }

    /// Return keystrokes that would dispatch the given action on the given view.
    pub(crate) fn keystrokes_for_action(
        &mut self,
        view_id: usize,
        action: &dyn Action,
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        let window_id = self.window_id;
        let mut contexts = Vec::new();
        let mut handler_depth = None;
        for (i, view_id) in self.ancestors(view_id).enumerate() {
            if let Some(view_metadata) = self.views_metadata.get(&(window_id, view_id)) {
                if let Some(actions) = self.actions.get(&view_metadata.type_id) {
                    if actions.contains_key(&action.as_any().type_id()) {
                        handler_depth = Some(i);
                    }
                }
                contexts.push(view_metadata.keymap_context.clone());
            }
        }

        if self.global_actions.contains_key(&action.as_any().type_id()) {
            handler_depth = Some(contexts.len())
        }

        self.keystroke_matcher
            .bindings_for_action_type(action.as_any().type_id())
            .find_map(|b| {
                handler_depth
                    .map(|highest_handler| {
                        if (0..=highest_handler).any(|depth| b.match_context(&contexts[depth..])) {
                            Some(b.keystrokes().into())
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
    }

    pub fn available_actions(
        &self,
        view_id: usize,
    ) -> impl Iterator<Item = (&'static str, Box<dyn Action>, SmallVec<[&Binding; 1]>)> {
        let window_id = self.window_id;
        let mut contexts = Vec::new();
        let mut handler_depths_by_action_type = HashMap::<TypeId, usize>::default();
        for (depth, view_id) in self.ancestors(view_id).enumerate() {
            if let Some(view_metadata) = self.views_metadata.get(&(window_id, view_id)) {
                contexts.push(view_metadata.keymap_context.clone());
                if let Some(actions) = self.actions.get(&view_metadata.type_id) {
                    handler_depths_by_action_type.extend(
                        actions
                            .keys()
                            .copied()
                            .map(|action_type| (action_type, depth)),
                    );
                }
            } else {
                log::error!(
                    "view {} not found when computing available actions",
                    view_id
                );
            }
        }

        handler_depths_by_action_type.extend(
            self.global_actions
                .keys()
                .copied()
                .map(|action_type| (action_type, contexts.len())),
        );

        self.action_deserializers
            .iter()
            .filter_map(move |(name, (type_id, deserialize))| {
                if let Some(action_depth) = handler_depths_by_action_type.get(type_id).copied() {
                    Some((
                        *name,
                        deserialize("{}").ok()?,
                        self.keystroke_matcher
                            .bindings_for_action_type(*type_id)
                            .filter(|b| {
                                (0..=action_depth).any(|depth| b.match_context(&contexts[depth..]))
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            })
    }

    pub fn dispatch_keystroke(&mut self, keystroke: &Keystroke) -> bool {
        let window_id = self.window_id;
        if let Some(focused_view_id) = self.focused_view_id() {
            let dispatch_path = self
                .ancestors(focused_view_id)
                .filter_map(|view_id| {
                    self.views_metadata
                        .get(&(window_id, view_id))
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
                        if self.handle_dispatch_action_from_effect(Some(*view_id), action.as_ref())
                        {
                            self.keystroke_matcher.clear_pending();
                            handled_by = Some(action.boxed_clone());
                            break;
                        }
                    }
                    handled_by.is_some()
                }
            };

            self.keystroke(
                window_id,
                keystroke.clone(),
                handled_by,
                match_result.clone(),
            );
            keystroke_handled
        } else {
            self.keystroke(window_id, keystroke.clone(), None, MatchResult::None);
            false
        }
    }

    pub fn dispatch_event(&mut self, event: Event, event_reused: bool) -> bool {
        let mut mouse_events = SmallVec::<[_; 2]>::new();
        let mut notified_views: HashSet<usize> = Default::default();
        let window_id = self.window_id;

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

                // If there is already clicked_button stored, don't replace it.
                if self.window.clicked_button.is_none() {
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

                    self.window.clicked_button = Some(e.button);
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
                for region in self.window.cursor_regions.iter().rev() {
                    if region.bounds.contains_point(*position) {
                        style_to_assign = region.style;
                        break;
                    }
                }

                if self
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
                            prev_mouse_position: self.window.mouse_position,
                            platform_event: e.clone(),
                        }));
                    } else if let Some(clicked_button) = self.window.clicked_button {
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
            self.window.mouse_position = position;
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
                    let mouse_position = self.window.mouse_position.clone();
                    let window = &mut *self.window;
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
                            if window.hovered_region_ids.insert(region.id()) {
                                valid_regions.push(region.clone());
                                if region.notify_on_hover {
                                    notified_views.insert(region.id().view_id());
                                }
                            }
                        } else {
                            // Ensure that hover exit events aren't sent twice
                            if window.hovered_region_ids.remove(&region.id()) {
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
                        if region.bounds.contains_point(self.window.mouse_position) {
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
                        .clicked_button
                        .map(|clicked_button| clicked_button == e.button)
                        .unwrap_or(false)
                    {
                        // Clear clicked regions and clicked button
                        let clicked_region_ids = std::mem::replace(
                            &mut self.window.clicked_region_ids,
                            Default::default(),
                        );
                        self.window.clicked_button = None;

                        // Find regions which still overlap with the mouse since the last MouseDown happened
                        for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                            if clicked_region_ids.contains(&mouse_region.id()) {
                                if mouse_region
                                    .bounds
                                    .contains_point(self.window.mouse_position)
                                {
                                    valid_regions.push(mouse_region.clone());
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

                MouseEvent::MoveOut(_) | MouseEvent::UpOut(_) | MouseEvent::DownOut(_) => {
                    for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                        // NOT contains
                        if !mouse_region
                            .bounds
                            .contains_point(self.window.mouse_position)
                        {
                            valid_regions.push(mouse_region.clone());
                        }
                    }
                }

                _ => {
                    for (mouse_region, _) in self.window.mouse_regions.iter().rev() {
                        // Contains
                        if mouse_region
                            .bounds
                            .contains_point(self.window.mouse_position)
                        {
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
            self.notify_view(window_id, view_id);
        }

        any_event_handled
    }

    pub fn dispatch_key_down(&mut self, event: &KeyDownEvent) -> bool {
        let window_id = self.window_id;
        if let Some(focused_view_id) = self.window.focused_view_id {
            for view_id in self.ancestors(focused_view_id).collect::<Vec<_>>() {
                if let Some(mut view) = self.views.remove(&(window_id, view_id)) {
                    let handled = view.key_down(event, self, view_id);
                    self.views.insert((window_id, view_id), view);
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

    pub fn dispatch_key_up(&mut self, event: &KeyUpEvent) -> bool {
        let window_id = self.window_id;
        if let Some(focused_view_id) = self.window.focused_view_id {
            for view_id in self.ancestors(focused_view_id).collect::<Vec<_>>() {
                if let Some(mut view) = self.views.remove(&(window_id, view_id)) {
                    let handled = view.key_up(event, self, view_id);
                    self.views.insert((window_id, view_id), view);
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

    pub fn dispatch_modifiers_changed(&mut self, event: &ModifiersChangedEvent) -> bool {
        let window_id = self.window_id;
        if let Some(focused_view_id) = self.window.focused_view_id {
            for view_id in self.ancestors(focused_view_id).collect::<Vec<_>>() {
                if let Some(mut view) = self.views.remove(&(window_id, view_id)) {
                    let handled = view.modifiers_changed(event, self, view_id);
                    self.views.insert((window_id, view_id), view);
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

    pub fn invalidate(&mut self, invalidation: &mut WindowInvalidation, appearance: Appearance) {
        self.start_frame();
        self.window.appearance = appearance;
        for view_id in &invalidation.removed {
            invalidation.updated.remove(view_id);
            self.window.rendered_views.remove(view_id);
        }
        for view_id in &invalidation.updated {
            let titlebar_height = self.window.titlebar_height;
            let hovered_region_ids = self.window.hovered_region_ids.clone();
            let clicked_region_ids = self
                .window
                .clicked_button
                .map(|button| (self.window.clicked_region_ids.clone(), button));

            let element = self
                .render_view(RenderParams {
                    view_id: *view_id,
                    titlebar_height,
                    hovered_region_ids,
                    clicked_region_ids,
                    refreshing: false,
                    appearance,
                })
                .unwrap();
            self.window.rendered_views.insert(*view_id, element);
        }
    }

    pub fn render_view(&mut self, params: RenderParams) -> Result<Box<dyn AnyRootElement>> {
        let window_id = self.window_id;
        let view_id = params.view_id;
        let mut view = self
            .views
            .remove(&(window_id, view_id))
            .ok_or_else(|| anyhow!("view not found"))?;
        let element = view.render(self, view_id);
        self.views.insert((window_id, view_id), view);
        Ok(element)
    }

    pub fn build_scene(&mut self) -> Result<Scene> {
        let window_size = self.window.platform_window.content_size();
        let scale_factor = self.window.platform_window.scale_factor();

        let root_view_id = self.window.root_view().id();
        let mut rendered_root = self.window.rendered_views.remove(&root_view_id).unwrap();
        rendered_root.layout(SizeConstraint::strict(window_size), self)?;

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
        let scene = scene_builder.build();
        self.window.cursor_regions = scene.cursor_regions();
        self.window.mouse_regions = scene.mouse_regions();

        if self.window_is_active() {
            if let Some(event) = self.window.last_mouse_moved_event.clone() {
                self.dispatch_event(event, true);
            }
        }

        Ok(scene)
    }

    pub fn rect_for_text_range(&self, range_utf16: Range<usize>) -> Option<RectF> {
        let root_view_id = self.window.root_view().id();
        self.window
            .rendered_views
            .get(&root_view_id)?
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

    pub(crate) fn handle_dispatch_action_from_effect(
        &mut self,
        view_id: Option<usize>,
        action: &dyn Action,
    ) -> bool {
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
                if let Some(ParentId::View(parent_id)) =
                    self.parents.get(&(self.window_id, view_id))
                {
                    view_id = *parent_id;
                    Some(view_id)
                } else {
                    None
                }
            }))
    }

    /// Returns the id of the parent of the given view, or none if the given
    /// view is the root.
    pub(crate) fn parent(&self, view_id: usize) -> Option<usize> {
        if let Some(ParentId::View(view_id)) = self.parents.get(&(self.window_id, view_id)) {
            Some(*view_id)
        } else {
            None
        }
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

    pub fn is_child_focused(&self, view: &AnyViewHandle) -> bool {
        if let Some(focused_view_id) = self.focused_view_id() {
            self.ancestors(focused_view_id)
                .skip(1) // Skip self id
                .any(|parent| parent == view.view_id)
        } else {
            false
        }
    }

    pub fn window_bounds(&self) -> WindowBounds {
        self.window.platform_window.bounds()
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

    pub fn replace_root_view<V, F>(&mut self, build_root_view: F) -> ViewHandle<V>
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        let root_view = self
            .build_and_insert_view(ParentId::Root, |cx| Some(build_root_view(cx)))
            .unwrap();
        self.window.root_view = Some(root_view.clone().into_any());
        self.window.focused_view_id = Some(root_view.id());
        root_view
    }

    pub(crate) fn build_and_insert_view<T, F>(
        &mut self,
        parent_id: ParentId,
        build_view: F,
    ) -> Option<ViewHandle<T>>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> Option<T>,
    {
        let window_id = self.window_id;
        let view_id = post_inc(&mut self.next_entity_id);
        // Make sure we can tell child views about their parentu
        self.parents.insert((window_id, view_id), parent_id);
        let mut cx = ViewContext::mutable(self, view_id);
        let handle = if let Some(view) = build_view(&mut cx) {
            let mut keymap_context = KeymapContext::default();
            view.update_keymap_context(&mut keymap_context, cx.app_context());
            self.views_metadata.insert(
                (window_id, view_id),
                ViewMetadata {
                    type_id: TypeId::of::<T>(),
                    keymap_context,
                },
            );
            self.views.insert((window_id, view_id), Box::new(view));
            self.window
                .invalidation
                .get_or_insert_with(Default::default)
                .updated
                .insert(view_id);
            Some(ViewHandle::new(window_id, view_id, &self.ref_counts))
        } else {
            self.parents.remove(&(window_id, view_id));
            None
        };
        handle
    }
}

pub struct RenderParams {
    pub view_id: usize,
    pub titlebar_height: f32,
    pub hovered_region_ids: HashSet<MouseRegionId>,
    pub clicked_region_ids: Option<(HashSet<MouseRegionId>, MouseButton)>,
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
    view_id: usize,
    view_name: &'static str,
}

impl ChildView {
    pub fn new(view: &AnyViewHandle, cx: &AppContext) -> Self {
        let view_name = cx.view_ui_name(view.window_id(), view.id()).unwrap();
        Self {
            view_id: view.id(),
            view_name,
        }
    }
}

impl<V: View> Element<V> for ChildView {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        _: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        if let Some(mut rendered_view) = cx.window.rendered_views.remove(&self.view_id) {
            let size = rendered_view
                .layout(constraint, cx)
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
        cx: &mut ViewContext<V>,
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
