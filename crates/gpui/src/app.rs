pub mod action;
mod callback_collection;
mod menu;
pub(crate) mod ref_counts;
#[cfg(any(test, feature = "test-support"))]
pub mod test_app_context;
mod window_input_handler;

use std::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
    pin::Pin,
    rc::{self, Rc},
    sync::{Arc, Weak},
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use parking_lot::Mutex;
use pathfinder_geometry::vector::Vector2F;
use postage::oneshot;
use smallvec::SmallVec;
use smol::prelude::*;
use uuid::Uuid;

pub use action::*;
use callback_collection::CallbackCollection;
use collections::{hash_map::Entry, BTreeMap, HashMap, HashSet, VecDeque};
pub use menu::*;
use platform::Event;
#[cfg(any(test, feature = "test-support"))]
use ref_counts::LeakDetector;
#[cfg(any(test, feature = "test-support"))]
pub use test_app_context::{ContextHandle, TestAppContext};
use window_input_handler::WindowInputHandler;

use crate::{
    elements::ElementBox,
    executor::{self, Task},
    keymap_matcher::{self, Binding, KeymapContext, KeymapMatcher, Keystroke, MatchResult},
    platform::{self, KeyDownEvent, Platform, PromptLevel, WindowOptions},
    presenter::Presenter,
    util::post_inc,
    Appearance, AssetCache, AssetSource, ClipboardItem, FontCache, KeyUpEvent,
    ModifiersChangedEvent, MouseButton, MouseRegionId, PathPromptOptions, TextLayoutCache,
    WindowBounds,
};

use self::ref_counts::RefCounts;

pub trait Entity: 'static {
    type Event;

    fn release(&mut self, _: &mut MutableAppContext) {}
    fn app_will_quit(
        &mut self,
        _: &mut MutableAppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>> {
        None
    }
}

pub trait View: Entity + Sized {
    fn ui_name() -> &'static str;
    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox;
    fn focus_in(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {}
    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {}
    fn key_down(&mut self, _: &KeyDownEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn key_up(&mut self, _: &KeyUpEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn modifiers_changed(&mut self, _: &ModifiersChangedEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }

    fn keymap_context(&self, _: &AppContext) -> keymap_matcher::KeymapContext {
        Self::default_keymap_context()
    }
    fn default_keymap_context() -> keymap_matcher::KeymapContext {
        let mut cx = keymap_matcher::KeymapContext::default();
        cx.add_identifier(Self::ui_name());
        cx
    }
    fn debug_json(&self, _: &AppContext) -> serde_json::Value {
        serde_json::Value::Null
    }

    fn text_for_range(&self, _: Range<usize>, _: &AppContext) -> Option<String> {
        None
    }
    fn selected_text_range(&self, _: &AppContext) -> Option<Range<usize>> {
        None
    }
    fn marked_text_range(&self, _: &AppContext) -> Option<Range<usize>> {
        None
    }
    fn unmark_text(&mut self, _: &mut ViewContext<Self>) {}
    fn replace_text_in_range(
        &mut self,
        _: Option<Range<usize>>,
        _: &str,
        _: &mut ViewContext<Self>,
    ) {
    }
    fn replace_and_mark_text_in_range(
        &mut self,
        _: Option<Range<usize>>,
        _: &str,
        _: Option<Range<usize>>,
        _: &mut ViewContext<Self>,
    ) {
    }
}

pub trait ReadModel {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T;
}

pub trait ReadModelWith {
    fn read_model_with<E: Entity, T>(
        &self,
        handle: &ModelHandle<E>,
        read: &mut dyn FnMut(&E, &AppContext) -> T,
    ) -> T;
}

pub trait UpdateModel {
    fn update_model<T: Entity, O>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> O,
    ) -> O;
}

pub trait UpgradeModelHandle {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>>;

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool;

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle>;
}

pub trait UpgradeViewHandle {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>>;

    fn upgrade_any_view_handle(&self, handle: &AnyWeakViewHandle) -> Option<AnyViewHandle>;
}

pub trait ReadView {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T;
}

pub trait ReadViewWith {
    fn read_view_with<V, T>(
        &self,
        handle: &ViewHandle<V>,
        read: &mut dyn FnMut(&V, &AppContext) -> T,
    ) -> T
    where
        V: View;
}

pub trait UpdateView {
    fn update_view<T, S>(
        &mut self,
        handle: &ViewHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
    ) -> S
    where
        T: View;
}

#[derive(Clone)]
pub struct App(Rc<RefCell<MutableAppContext>>);

#[derive(Clone)]
pub struct AsyncAppContext(Rc<RefCell<MutableAppContext>>);

impl App {
    pub fn new(asset_source: impl AssetSource) -> Result<Self> {
        let platform = platform::current::platform();
        let foreground = Rc::new(executor::Foreground::platform(platform.dispatcher())?);
        let foreground_platform = platform::current::foreground_platform(foreground.clone());
        let app = Self(Rc::new(RefCell::new(MutableAppContext::new(
            foreground,
            Arc::new(executor::Background::new()),
            platform.clone(),
            foreground_platform.clone(),
            Arc::new(FontCache::new(platform.fonts())),
            Default::default(),
            asset_source,
        ))));

        foreground_platform.on_quit(Box::new({
            let cx = app.0.clone();
            move || {
                cx.borrow_mut().quit();
            }
        }));
        setup_menu_handlers(foreground_platform.as_ref(), &app);

        app.0.borrow_mut().weak_self = Some(Rc::downgrade(&app.0));
        Ok(app)
    }

    pub fn background(&self) -> Arc<executor::Background> {
        self.0.borrow().background().clone()
    }

    pub fn on_become_active<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(&mut MutableAppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_become_active(Box::new(move || callback(&mut *cx.borrow_mut())));
        self
    }

    pub fn on_resign_active<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(&mut MutableAppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_resign_active(Box::new(move || callback(&mut *cx.borrow_mut())));
        self
    }

    pub fn on_quit<F>(&mut self, mut callback: F) -> &mut Self
    where
        F: 'static + FnMut(&mut MutableAppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_quit(Box::new(move || callback(&mut *cx.borrow_mut())));
        self
    }

    pub fn on_event<F>(&mut self, mut callback: F) -> &mut Self
    where
        F: 'static + FnMut(Event, &mut MutableAppContext) -> bool,
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_event(Box::new(move |event| {
                callback(event, &mut *cx.borrow_mut())
            }));
        self
    }

    pub fn on_open_urls<F>(&mut self, mut callback: F) -> &mut Self
    where
        F: 'static + FnMut(Vec<String>, &mut MutableAppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_open_urls(Box::new(move |paths| {
                callback(paths, &mut *cx.borrow_mut())
            }));
        self
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut MutableAppContext),
    {
        let platform = self.0.borrow().foreground_platform.clone();
        platform.run(Box::new(move || {
            let mut cx = self.0.borrow_mut();
            let cx = &mut *cx;
            crate::views::init(cx);
            on_finish_launching(cx);
        }))
    }

    pub fn platform(&self) -> Arc<dyn Platform> {
        self.0.borrow().platform()
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.0.borrow().cx.font_cache.clone()
    }

    fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.0.borrow_mut();
        let result = state.update(callback);
        state.pending_notifications.clear();
        result
    }
}

impl AsyncAppContext {
    pub fn spawn<F, Fut, T>(&self, f: F) -> Task<T>
    where
        F: FnOnce(AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = T>,
        T: 'static,
    {
        self.0.borrow().foreground.spawn(f(self.clone()))
    }

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&self, callback: F) -> T {
        callback(self.0.borrow().as_ref())
    }

    pub fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        self.0.borrow_mut().update(callback)
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.update(|cx| cx.add_model(build_model))
    }

    pub fn add_window<T, F>(
        &mut self,
        window_options: WindowOptions,
        build_root_view: F,
    ) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.update(|cx| cx.add_window(window_options, build_root_view))
    }

    pub fn remove_window(&mut self, window_id: usize) {
        self.update(|cx| cx.remove_window(window_id))
    }

    pub fn activate_window(&mut self, window_id: usize) {
        self.update(|cx| cx.activate_window(window_id))
    }

    pub fn prompt(
        &mut self,
        window_id: usize,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        self.update(|cx| cx.prompt(window_id, level, msg, answers))
    }

    pub fn platform(&self) -> Arc<dyn Platform> {
        self.0.borrow().platform()
    }

    pub fn foreground(&self) -> Rc<executor::Foreground> {
        self.0.borrow().foreground.clone()
    }

    pub fn background(&self) -> Arc<executor::Background> {
        self.0.borrow().cx.background.clone()
    }
}

impl UpdateModel for AsyncAppContext {
    fn update_model<E: Entity, O>(
        &mut self,
        handle: &ModelHandle<E>,
        update: &mut dyn FnMut(&mut E, &mut ModelContext<E>) -> O,
    ) -> O {
        self.0.borrow_mut().update_model(handle, update)
    }
}

impl UpgradeModelHandle for AsyncAppContext {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        self.0.borrow().upgrade_model_handle(handle)
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.0.borrow().model_handle_is_upgradable(handle)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        self.0.borrow().upgrade_any_model_handle(handle)
    }
}

impl UpgradeViewHandle for AsyncAppContext {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
        self.0.borrow_mut().upgrade_view_handle(handle)
    }

    fn upgrade_any_view_handle(&self, handle: &AnyWeakViewHandle) -> Option<AnyViewHandle> {
        self.0.borrow_mut().upgrade_any_view_handle(handle)
    }
}

impl ReadModelWith for AsyncAppContext {
    fn read_model_with<E: Entity, T>(
        &self,
        handle: &ModelHandle<E>,
        read: &mut dyn FnMut(&E, &AppContext) -> T,
    ) -> T {
        let cx = self.0.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

impl UpdateView for AsyncAppContext {
    fn update_view<T, S>(
        &mut self,
        handle: &ViewHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
    ) -> S
    where
        T: View,
    {
        self.0.borrow_mut().update_view(handle, update)
    }
}

impl ReadViewWith for AsyncAppContext {
    fn read_view_with<V, T>(
        &self,
        handle: &ViewHandle<V>,
        read: &mut dyn FnMut(&V, &AppContext) -> T,
    ) -> T
    where
        V: View,
    {
        let cx = self.0.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

type ActionCallback =
    dyn FnMut(&mut dyn AnyView, &dyn Action, &mut MutableAppContext, usize, usize);
type GlobalActionCallback = dyn FnMut(&dyn Action, &mut MutableAppContext);

type SubscriptionCallback = Box<dyn FnMut(&dyn Any, &mut MutableAppContext) -> bool>;
type GlobalSubscriptionCallback = Box<dyn FnMut(&dyn Any, &mut MutableAppContext)>;
type ObservationCallback = Box<dyn FnMut(&mut MutableAppContext) -> bool>;
type GlobalObservationCallback = Box<dyn FnMut(&mut MutableAppContext)>;
type FocusObservationCallback = Box<dyn FnMut(bool, &mut MutableAppContext) -> bool>;
type ReleaseObservationCallback = Box<dyn FnMut(&dyn Any, &mut MutableAppContext)>;
type ActionObservationCallback = Box<dyn FnMut(TypeId, &mut MutableAppContext)>;
type WindowActivationCallback = Box<dyn FnMut(bool, &mut MutableAppContext) -> bool>;
type WindowFullscreenCallback = Box<dyn FnMut(bool, &mut MutableAppContext) -> bool>;
type WindowBoundsCallback = Box<dyn FnMut(WindowBounds, Uuid, &mut MutableAppContext) -> bool>;
type KeystrokeCallback = Box<
    dyn FnMut(&Keystroke, &MatchResult, Option<&Box<dyn Action>>, &mut MutableAppContext) -> bool,
>;
type ActiveLabeledTasksCallback = Box<dyn FnMut(&mut MutableAppContext) -> bool>;
type DeserializeActionCallback = fn(json: &str) -> anyhow::Result<Box<dyn Action>>;
type WindowShouldCloseSubscriptionCallback = Box<dyn FnMut(&mut MutableAppContext) -> bool>;

pub struct MutableAppContext {
    weak_self: Option<rc::Weak<RefCell<Self>>>,
    foreground_platform: Rc<dyn platform::ForegroundPlatform>,
    assets: Arc<AssetCache>,
    cx: AppContext,
    action_deserializers: HashMap<&'static str, (TypeId, DeserializeActionCallback)>,
    capture_actions: HashMap<TypeId, HashMap<TypeId, Vec<Box<ActionCallback>>>>,
    // Entity Types -> { Action Types -> Action Handlers }
    actions: HashMap<TypeId, HashMap<TypeId, Vec<Box<ActionCallback>>>>,
    // Action Types -> Action Handlers
    global_actions: HashMap<TypeId, Box<GlobalActionCallback>>,
    keystroke_matcher: KeymapMatcher,
    next_entity_id: usize,
    next_window_id: usize,
    next_subscription_id: usize,
    frame_count: usize,

    subscriptions: CallbackCollection<usize, SubscriptionCallback>,
    global_subscriptions: CallbackCollection<TypeId, GlobalSubscriptionCallback>,
    observations: CallbackCollection<usize, ObservationCallback>,
    global_observations: CallbackCollection<TypeId, GlobalObservationCallback>,
    focus_observations: CallbackCollection<usize, FocusObservationCallback>,
    release_observations: CallbackCollection<usize, ReleaseObservationCallback>,
    action_dispatch_observations: CallbackCollection<(), ActionObservationCallback>,
    window_activation_observations: CallbackCollection<usize, WindowActivationCallback>,
    window_fullscreen_observations: CallbackCollection<usize, WindowFullscreenCallback>,
    window_bounds_observations: CallbackCollection<usize, WindowBoundsCallback>,
    keystroke_observations: CallbackCollection<usize, KeystrokeCallback>,
    active_labeled_task_observations: CallbackCollection<(), ActiveLabeledTasksCallback>,

    #[allow(clippy::type_complexity)]
    presenters_and_platform_windows:
        HashMap<usize, (Rc<RefCell<Presenter>>, Box<dyn platform::Window>)>,
    foreground: Rc<executor::Foreground>,
    pending_effects: VecDeque<Effect>,
    pending_notifications: HashSet<usize>,
    pending_global_notifications: HashSet<TypeId>,
    pending_flushes: usize,
    flushing_effects: bool,
    halt_action_dispatch: bool,
    next_labeled_task_id: usize,
    active_labeled_tasks: BTreeMap<usize, &'static str>,
}

impl MutableAppContext {
    fn new(
        foreground: Rc<executor::Foreground>,
        background: Arc<executor::Background>,
        platform: Arc<dyn platform::Platform>,
        foreground_platform: Rc<dyn platform::ForegroundPlatform>,
        font_cache: Arc<FontCache>,
        ref_counts: RefCounts,
        asset_source: impl AssetSource,
    ) -> Self {
        Self {
            weak_self: None,
            foreground_platform,
            assets: Arc::new(AssetCache::new(asset_source)),
            cx: AppContext {
                models: Default::default(),
                views: Default::default(),
                parents: Default::default(),
                windows: Default::default(),
                globals: Default::default(),
                element_states: Default::default(),
                ref_counts: Arc::new(Mutex::new(ref_counts)),
                background,
                font_cache,
                platform,
            },
            action_deserializers: Default::default(),
            capture_actions: Default::default(),
            actions: Default::default(),
            global_actions: Default::default(),
            keystroke_matcher: KeymapMatcher::default(),
            next_entity_id: 0,
            next_window_id: 0,
            next_subscription_id: 0,
            frame_count: 0,
            subscriptions: Default::default(),
            global_subscriptions: Default::default(),
            observations: Default::default(),
            focus_observations: Default::default(),
            release_observations: Default::default(),
            global_observations: Default::default(),
            window_activation_observations: Default::default(),
            window_fullscreen_observations: Default::default(),
            window_bounds_observations: Default::default(),
            keystroke_observations: Default::default(),
            action_dispatch_observations: Default::default(),
            active_labeled_task_observations: Default::default(),
            presenters_and_platform_windows: Default::default(),
            foreground,
            pending_effects: VecDeque::new(),
            pending_notifications: Default::default(),
            pending_global_notifications: Default::default(),
            pending_flushes: 0,
            flushing_effects: false,
            halt_action_dispatch: false,
            next_labeled_task_id: 0,
            active_labeled_tasks: Default::default(),
        }
    }

    pub fn upgrade(&self) -> App {
        App(self.weak_self.as_ref().unwrap().upgrade().unwrap())
    }

    pub fn quit(&mut self) {
        let mut futures = Vec::new();
        for model_id in self.cx.models.keys().copied().collect::<Vec<_>>() {
            let mut model = self.cx.models.remove(&model_id).unwrap();
            futures.extend(model.app_will_quit(self));
            self.cx.models.insert(model_id, model);
        }

        for view_id in self.cx.views.keys().copied().collect::<Vec<_>>() {
            let mut view = self.cx.views.remove(&view_id).unwrap();
            futures.extend(view.app_will_quit(self));
            self.cx.views.insert(view_id, view);
        }

        self.remove_all_windows();

        let futures = futures::future::join_all(futures);
        if self
            .background
            .block_with_timeout(Duration::from_millis(100), futures)
            .is_err()
        {
            log::error!("timed out waiting on app_will_quit");
        }
    }

    pub fn remove_all_windows(&mut self) {
        for (window_id, _) in self.cx.windows.drain() {
            self.presenters_and_platform_windows.remove(&window_id);
        }
        self.flush_effects();
    }

    pub fn platform(&self) -> Arc<dyn platform::Platform> {
        self.cx.platform.clone()
    }

    pub fn font_cache(&self) -> &Arc<FontCache> {
        &self.cx.font_cache
    }

    pub fn foreground(&self) -> &Rc<executor::Foreground> {
        &self.foreground
    }

    pub fn background(&self) -> &Arc<executor::Background> {
        &self.cx.background
    }

    pub fn debug_elements(&self, window_id: usize) -> Option<crate::json::Value> {
        self.presenters_and_platform_windows
            .get(&window_id)
            .and_then(|(presenter, _)| presenter.borrow().debug_elements(self))
    }

    pub fn deserialize_action(
        &self,
        name: &str,
        argument: Option<&str>,
    ) -> Result<Box<dyn Action>> {
        let callback = self
            .action_deserializers
            .get(name)
            .ok_or_else(|| anyhow!("unknown action {}", name))?
            .1;
        callback(argument.unwrap_or("{}"))
            .with_context(|| format!("invalid data for action {}", name))
    }

    pub fn add_action<A, V, F, R>(&mut self, handler: F)
    where
        A: Action,
        V: View,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>) -> R,
    {
        self.add_action_internal(handler, false)
    }

    pub fn capture_action<A, V, F>(&mut self, handler: F)
    where
        A: Action,
        V: View,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>),
    {
        self.add_action_internal(handler, true)
    }

    fn add_action_internal<A, V, F, R>(&mut self, mut handler: F, capture: bool)
    where
        A: Action,
        V: View,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>) -> R,
    {
        let handler = Box::new(
            move |view: &mut dyn AnyView,
                  action: &dyn Action,
                  cx: &mut MutableAppContext,
                  window_id: usize,
                  view_id: usize| {
                let action = action.as_any().downcast_ref().unwrap();
                let mut cx = ViewContext::new(cx, window_id, view_id);
                handler(
                    view.as_any_mut()
                        .downcast_mut()
                        .expect("downcast is type safe"),
                    action,
                    &mut cx,
                );
            },
        );

        self.action_deserializers
            .entry(A::qualified_name())
            .or_insert((TypeId::of::<A>(), A::from_json_str));

        let actions = if capture {
            &mut self.capture_actions
        } else {
            &mut self.actions
        };

        actions
            .entry(TypeId::of::<V>())
            .or_default()
            .entry(TypeId::of::<A>())
            .or_default()
            .push(handler);
    }

    pub fn add_async_action<A, V, F>(&mut self, mut handler: F)
    where
        A: Action,
        V: View,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>) -> Option<Task<Result<()>>>,
    {
        self.add_action(move |view, action, cx| {
            if let Some(task) = handler(view, action, cx) {
                task.detach_and_log_err(cx);
            }
        })
    }

    pub fn add_global_action<A, F>(&mut self, mut handler: F)
    where
        A: Action,
        F: 'static + FnMut(&A, &mut MutableAppContext),
    {
        let handler = Box::new(move |action: &dyn Action, cx: &mut MutableAppContext| {
            let action = action.as_any().downcast_ref().unwrap();
            handler(action, cx);
        });

        self.action_deserializers
            .entry(A::qualified_name())
            .or_insert((TypeId::of::<A>(), A::from_json_str));

        if self
            .global_actions
            .insert(TypeId::of::<A>(), handler)
            .is_some()
        {
            panic!(
                "registered multiple global handlers for {}",
                type_name::<A>()
            );
        }
    }

    pub fn is_topmost_window_for_position(&self, window_id: usize, position: Vector2F) -> bool {
        self.presenters_and_platform_windows
            .get(&window_id)
            .map_or(false, |(_, window)| {
                window.is_topmost_for_position(position)
            })
    }

    pub fn window_ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.cx.windows.keys().copied()
    }

    pub fn activate_window(&self, window_id: usize) {
        if let Some((_, window)) = self.presenters_and_platform_windows.get(&window_id) {
            window.activate()
        }
    }

    pub fn root_view<T: View>(&self, window_id: usize) -> Option<ViewHandle<T>> {
        self.cx
            .windows
            .get(&window_id)
            .and_then(|window| window.root_view.clone().downcast::<T>())
    }

    pub fn window_is_active(&self, window_id: usize) -> bool {
        self.cx
            .windows
            .get(&window_id)
            .map_or(false, |window| window.is_active)
    }

    pub fn window_is_fullscreen(&self, window_id: usize) -> bool {
        self.cx
            .windows
            .get(&window_id)
            .map_or(false, |window| window.is_fullscreen)
    }

    pub fn window_bounds(&self, window_id: usize) -> Option<WindowBounds> {
        let (_, window) = self.presenters_and_platform_windows.get(&window_id)?;
        Some(window.bounds())
    }

    pub fn window_display_uuid(&self, window_id: usize) -> Option<Uuid> {
        let (_, window) = self.presenters_and_platform_windows.get(&window_id)?;
        window.screen().display_uuid()
    }

    pub fn active_labeled_tasks<'a>(
        &'a self,
    ) -> impl DoubleEndedIterator<Item = &'static str> + 'a {
        self.active_labeled_tasks.values().cloned()
    }

    pub fn render_view(&mut self, params: RenderParams) -> Result<ElementBox> {
        let window_id = params.window_id;
        let view_id = params.view_id;
        let mut view = self
            .cx
            .views
            .remove(&(window_id, view_id))
            .ok_or_else(|| anyhow!("view not found"))?;
        let element = view.render(params, self);
        self.cx.views.insert((window_id, view_id), view);
        Ok(element)
    }

    pub fn render_views(
        &mut self,
        window_id: usize,
        titlebar_height: f32,
        appearance: Appearance,
    ) -> HashMap<usize, ElementBox> {
        self.start_frame();
        #[allow(clippy::needless_collect)]
        let view_ids = self
            .views
            .keys()
            .filter_map(|(win_id, view_id)| {
                if *win_id == window_id {
                    Some(*view_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        view_ids
            .into_iter()
            .map(|view_id| {
                (
                    view_id,
                    self.render_view(RenderParams {
                        window_id,
                        view_id,
                        titlebar_height,
                        hovered_region_ids: Default::default(),
                        clicked_region_ids: None,
                        refreshing: false,
                        appearance,
                    })
                    .unwrap(),
                )
            })
            .collect()
    }

    pub(crate) fn start_frame(&mut self) {
        self.frame_count += 1;
    }

    pub fn update<T, F: FnOnce(&mut Self) -> T>(&mut self, callback: F) -> T {
        self.pending_flushes += 1;
        let result = callback(self);
        self.flush_effects();
        result
    }

    fn show_character_palette(&self, window_id: usize) {
        let (_, window) = &self.presenters_and_platform_windows[&window_id];
        window.show_character_palette();
    }

    pub fn minimize_window(&self, window_id: usize) {
        let (_, window) = &self.presenters_and_platform_windows[&window_id];
        window.minimize();
    }

    pub fn zoom_window(&self, window_id: usize) {
        let (_, window) = &self.presenters_and_platform_windows[&window_id];
        window.zoom();
    }

    pub fn toggle_window_full_screen(&self, window_id: usize) {
        let (_, window) = &self.presenters_and_platform_windows[&window_id];
        window.toggle_full_screen();
    }

    pub fn prompt(
        &self,
        window_id: usize,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        let (_, window) = &self.presenters_and_platform_windows[&window_id];
        window.prompt(level, msg, answers)
    }

    pub fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        self.foreground_platform.prompt_for_paths(options)
    }

    pub fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        self.foreground_platform.prompt_for_new_path(directory)
    }

    pub fn reveal_path(&self, path: &Path) {
        self.foreground_platform.reveal_path(path)
    }

    pub fn emit_global<E: Any>(&mut self, payload: E) {
        self.pending_effects.push_back(Effect::GlobalEvent {
            payload: Box::new(payload),
        });
    }

    pub fn subscribe<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(H, &E::Event, &mut Self),
    {
        self.subscribe_internal(handle, move |handle, event, cx| {
            callback(handle, event, cx);
            true
        })
    }

    pub fn subscribe_global<E, F>(&mut self, mut callback: F) -> Subscription
    where
        E: Any,
        F: 'static + FnMut(&E, &mut Self),
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        let type_id = TypeId::of::<E>();
        self.pending_effects.push_back(Effect::GlobalSubscription {
            type_id,
            subscription_id,
            callback: Box::new(move |payload, cx| {
                let payload = payload.downcast_ref().expect("downcast is type safe");
                callback(payload, cx)
            }),
        });
        Subscription::GlobalSubscription(
            self.global_subscriptions
                .subscribe(type_id, subscription_id),
        )
    }

    pub fn observe<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(H, &mut Self),
    {
        self.observe_internal(handle, move |handle, cx| {
            callback(handle, cx);
            true
        })
    }

    pub fn subscribe_internal<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(H, &E::Event, &mut Self) -> bool,
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        let emitter = handle.downgrade();
        self.pending_effects.push_back(Effect::Subscription {
            entity_id: handle.id(),
            subscription_id,
            callback: Box::new(move |payload, cx| {
                if let Some(emitter) = H::upgrade_from(&emitter, cx.as_ref()) {
                    let payload = payload.downcast_ref().expect("downcast is type safe");
                    callback(emitter, payload, cx)
                } else {
                    false
                }
            }),
        });
        Subscription::Subscription(self.subscriptions.subscribe(handle.id(), subscription_id))
    }

    fn observe_internal<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(H, &mut Self) -> bool,
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        let observed = handle.downgrade();
        let entity_id = handle.id();
        self.pending_effects.push_back(Effect::Observation {
            entity_id,
            subscription_id,
            callback: Box::new(move |cx| {
                if let Some(observed) = H::upgrade_from(&observed, cx) {
                    callback(observed, cx)
                } else {
                    false
                }
            }),
        });
        Subscription::Observation(self.observations.subscribe(entity_id, subscription_id))
    }

    fn observe_focus<F, V>(&mut self, handle: &ViewHandle<V>, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(ViewHandle<V>, bool, &mut MutableAppContext) -> bool,
        V: View,
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        let observed = handle.downgrade();
        let view_id = handle.id();

        self.pending_effects.push_back(Effect::FocusObservation {
            view_id,
            subscription_id,
            callback: Box::new(move |focused, cx| {
                if let Some(observed) = observed.upgrade(cx) {
                    callback(observed, focused, cx)
                } else {
                    false
                }
            }),
        });
        Subscription::FocusObservation(self.focus_observations.subscribe(view_id, subscription_id))
    }

    pub fn observe_global<G, F>(&mut self, mut observe: F) -> Subscription
    where
        G: Any,
        F: 'static + FnMut(&mut MutableAppContext),
    {
        let type_id = TypeId::of::<G>();
        let id = post_inc(&mut self.next_subscription_id);

        self.global_observations.add_callback(
            type_id,
            id,
            Box::new(move |cx: &mut MutableAppContext| observe(cx)),
        );
        Subscription::GlobalObservation(self.global_observations.subscribe(type_id, id))
    }

    pub fn observe_default_global<G, F>(&mut self, observe: F) -> Subscription
    where
        G: Any + Default,
        F: 'static + FnMut(&mut MutableAppContext),
    {
        if !self.has_global::<G>() {
            self.set_global(G::default());
        }
        self.observe_global::<G, F>(observe)
    }

    pub fn observe_release<E, H, F>(&mut self, handle: &H, callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnOnce(&E, &mut Self),
    {
        let id = post_inc(&mut self.next_subscription_id);
        let mut callback = Some(callback);
        self.release_observations.add_callback(
            handle.id(),
            id,
            Box::new(move |released, cx| {
                let released = released.downcast_ref().unwrap();
                if let Some(callback) = callback.take() {
                    callback(released, cx)
                }
            }),
        );
        Subscription::ReleaseObservation(self.release_observations.subscribe(handle.id(), id))
    }

    pub fn observe_actions<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(TypeId, &mut MutableAppContext),
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.action_dispatch_observations
            .add_callback((), subscription_id, Box::new(callback));
        Subscription::ActionObservation(
            self.action_dispatch_observations
                .subscribe((), subscription_id),
        )
    }

    fn observe_window_activation<F>(&mut self, window_id: usize, callback: F) -> Subscription
    where
        F: 'static + FnMut(bool, &mut MutableAppContext) -> bool,
    {
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

    fn observe_fullscreen<F>(&mut self, window_id: usize, callback: F) -> Subscription
    where
        F: 'static + FnMut(bool, &mut MutableAppContext) -> bool,
    {
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

    fn observe_window_bounds<F>(&mut self, window_id: usize, callback: F) -> Subscription
    where
        F: 'static + FnMut(WindowBounds, Uuid, &mut MutableAppContext) -> bool,
    {
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

    pub fn observe_keystrokes<F>(&mut self, window_id: usize, callback: F) -> Subscription
    where
        F: 'static
            + FnMut(
                &Keystroke,
                &MatchResult,
                Option<&Box<dyn Action>>,
                &mut MutableAppContext,
            ) -> bool,
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.keystroke_observations
            .add_callback(window_id, subscription_id, Box::new(callback));
        Subscription::KeystrokeObservation(
            self.keystroke_observations
                .subscribe(window_id, subscription_id),
        )
    }

    pub fn observe_active_labeled_tasks<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut MutableAppContext) -> bool,
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.active_labeled_task_observations
            .add_callback((), subscription_id, Box::new(callback));
        Subscription::ActiveLabeledTasksObservation(
            self.active_labeled_task_observations
                .subscribe((), subscription_id),
        )
    }

    pub fn defer(&mut self, callback: impl 'static + FnOnce(&mut MutableAppContext)) {
        self.pending_effects.push_back(Effect::Deferred {
            callback: Box::new(callback),
            after_window_update: false,
        })
    }

    pub fn after_window_update(&mut self, callback: impl 'static + FnOnce(&mut MutableAppContext)) {
        self.pending_effects.push_back(Effect::Deferred {
            callback: Box::new(callback),
            after_window_update: true,
        })
    }

    pub(crate) fn notify_model(&mut self, model_id: usize) {
        if self.pending_notifications.insert(model_id) {
            self.pending_effects
                .push_back(Effect::ModelNotification { model_id });
        }
    }

    pub(crate) fn notify_view(&mut self, window_id: usize, view_id: usize) {
        if self.pending_notifications.insert(view_id) {
            self.pending_effects
                .push_back(Effect::ViewNotification { window_id, view_id });
        }
    }

    pub(crate) fn notify_global(&mut self, type_id: TypeId) {
        if self.pending_global_notifications.insert(type_id) {
            self.pending_effects
                .push_back(Effect::GlobalNotification { type_id });
        }
    }

    pub(crate) fn name_for_view(&self, window_id: usize, view_id: usize) -> Option<&str> {
        self.views
            .get(&(window_id, view_id))
            .map(|view| view.ui_name())
    }

    pub fn all_action_names<'a>(&'a self) -> impl Iterator<Item = &'static str> + 'a {
        self.action_deserializers.keys().copied()
    }

    /// Return keystrokes that would dispatch the given action on the given view.
    pub(crate) fn keystrokes_for_action(
        &mut self,
        window_id: usize,
        view_id: usize,
        action: &dyn Action,
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        let mut contexts = Vec::new();
        let mut handler_depth = None;
        for (i, view_id) in self.ancestors(window_id, view_id).enumerate() {
            if let Some(view) = self.views.get(&(window_id, view_id)) {
                if let Some(actions) = self.actions.get(&view.as_any().type_id()) {
                    if actions.contains_key(&action.as_any().type_id()) {
                        handler_depth = Some(i);
                    }
                }
                contexts.push(view.keymap_context(self));
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
        window_id: usize,
        view_id: usize,
    ) -> impl Iterator<Item = (&'static str, Box<dyn Action>, SmallVec<[&Binding; 1]>)> {
        let mut contexts = Vec::new();
        let mut handler_depths_by_action_type = HashMap::<TypeId, usize>::default();
        for (depth, view_id) in self.ancestors(window_id, view_id).enumerate() {
            if let Some(view) = self.views.get(&(window_id, view_id)) {
                contexts.push(view.keymap_context(self));
                let view_type = view.as_any().type_id();
                if let Some(actions) = self.actions.get(&view_type) {
                    handler_depths_by_action_type.extend(
                        actions
                            .keys()
                            .copied()
                            .map(|action_type| (action_type, depth)),
                    );
                }
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

    pub fn is_action_available(&self, action: &dyn Action) -> bool {
        let action_type = action.as_any().type_id();
        if let Some(window_id) = self.cx.platform.key_window_id() {
            if let Some(focused_view_id) = self.focused_view_id(window_id) {
                for view_id in self.ancestors(window_id, focused_view_id) {
                    if let Some(view) = self.views.get(&(window_id, view_id)) {
                        let view_type = view.as_any().type_id();
                        if let Some(actions) = self.actions.get(&view_type) {
                            if actions.contains_key(&action_type) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        self.global_actions.contains_key(&action_type)
    }

    // Traverses the parent tree. Walks down the tree toward the passed
    // view calling visit with true. Then walks back up the tree calling visit with false.
    // If `visit` returns false this function will immediately return.
    // Returns a bool indicating if the traversal was completed early.
    fn visit_dispatch_path(
        &mut self,
        window_id: usize,
        view_id: usize,
        mut visit: impl FnMut(usize, bool, &mut MutableAppContext) -> bool,
    ) -> bool {
        // List of view ids from the leaf to the root of the window
        let path = self.ancestors(window_id, view_id).collect::<Vec<_>>();

        // Walk down from the root to the leaf calling visit with capture_phase = true
        for view_id in path.iter().rev() {
            if !visit(*view_id, true, self) {
                return false;
            }
        }

        // Walk up from the leaf to the root calling visit with capture_phase = false
        for view_id in path.iter() {
            if !visit(*view_id, false, self) {
                return false;
            }
        }

        true
    }

    fn actions_mut(
        &mut self,
        capture_phase: bool,
    ) -> &mut HashMap<TypeId, HashMap<TypeId, Vec<Box<ActionCallback>>>> {
        if capture_phase {
            &mut self.capture_actions
        } else {
            &mut self.actions
        }
    }

    pub fn dispatch_global_action<A: Action>(&mut self, action: A) {
        self.dispatch_global_action_any(&action);
    }

    fn dispatch_global_action_any(&mut self, action: &dyn Action) -> bool {
        self.update(|this| {
            if let Some((name, mut handler)) = this.global_actions.remove_entry(&action.id()) {
                handler(action, this);
                this.global_actions.insert(name, handler);
                true
            } else {
                false
            }
        })
    }

    pub fn add_bindings<T: IntoIterator<Item = Binding>>(&mut self, bindings: T) {
        self.keystroke_matcher.add_bindings(bindings);
    }

    pub fn clear_bindings(&mut self) {
        self.keystroke_matcher.clear_bindings();
    }

    pub fn dispatch_key_down(&mut self, window_id: usize, event: &KeyDownEvent) -> bool {
        if let Some(focused_view_id) = self.focused_view_id(window_id) {
            for view_id in self
                .ancestors(window_id, focused_view_id)
                .collect::<Vec<_>>()
            {
                if let Some(mut view) = self.cx.views.remove(&(window_id, view_id)) {
                    let handled = view.key_down(event, self, window_id, view_id);
                    self.cx.views.insert((window_id, view_id), view);
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

    pub fn dispatch_key_up(&mut self, window_id: usize, event: &KeyUpEvent) -> bool {
        if let Some(focused_view_id) = self.focused_view_id(window_id) {
            for view_id in self
                .ancestors(window_id, focused_view_id)
                .collect::<Vec<_>>()
            {
                if let Some(mut view) = self.cx.views.remove(&(window_id, view_id)) {
                    let handled = view.key_up(event, self, window_id, view_id);
                    self.cx.views.insert((window_id, view_id), view);
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

    pub fn dispatch_modifiers_changed(
        &mut self,
        window_id: usize,
        event: &ModifiersChangedEvent,
    ) -> bool {
        if let Some(focused_view_id) = self.focused_view_id(window_id) {
            for view_id in self
                .ancestors(window_id, focused_view_id)
                .collect::<Vec<_>>()
            {
                if let Some(mut view) = self.cx.views.remove(&(window_id, view_id)) {
                    let handled = view.modifiers_changed(event, self, window_id, view_id);
                    self.cx.views.insert((window_id, view_id), view);
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

    pub fn dispatch_keystroke(&mut self, window_id: usize, keystroke: &Keystroke) -> bool {
        if let Some(focused_view_id) = self.focused_view_id(window_id) {
            let dispatch_path = self
                .ancestors(window_id, focused_view_id)
                .map(|view_id| {
                    (
                        view_id,
                        self.cx
                            .views
                            .get(&(window_id, view_id))
                            .unwrap()
                            .keymap_context(self.as_ref()),
                    )
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
                        if self.handle_dispatch_action_from_effect(
                            window_id,
                            Some(*view_id),
                            action.as_ref(),
                        ) {
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

    pub fn default_global<T: 'static + Default>(&mut self) -> &T {
        let type_id = TypeId::of::<T>();
        self.update(|this| {
            if let Entry::Vacant(entry) = this.cx.globals.entry(type_id) {
                entry.insert(Box::new(T::default()));
                this.notify_global(type_id);
            }
        });
        self.globals.get(&type_id).unwrap().downcast_ref().unwrap()
    }

    pub fn set_global<T: 'static>(&mut self, state: T) {
        self.update(|this| {
            let type_id = TypeId::of::<T>();
            this.cx.globals.insert(type_id, Box::new(state));
            this.notify_global(type_id);
        });
    }

    pub fn update_default_global<T, F, U>(&mut self, update: F) -> U
    where
        T: 'static + Default,
        F: FnOnce(&mut T, &mut MutableAppContext) -> U,
    {
        self.update(|this| {
            let type_id = TypeId::of::<T>();
            let mut state = this
                .cx
                .globals
                .remove(&type_id)
                .unwrap_or_else(|| Box::new(T::default()));
            let result = update(state.downcast_mut().unwrap(), this);
            this.cx.globals.insert(type_id, state);
            this.notify_global(type_id);
            result
        })
    }

    pub fn update_global<T, F, U>(&mut self, update: F) -> U
    where
        T: 'static,
        F: FnOnce(&mut T, &mut MutableAppContext) -> U,
    {
        self.update(|this| {
            let type_id = TypeId::of::<T>();
            if let Some(mut state) = this.cx.globals.remove(&type_id) {
                let result = update(state.downcast_mut().unwrap(), this);
                this.cx.globals.insert(type_id, state);
                this.notify_global(type_id);
                result
            } else {
                panic!("No global added for {}", std::any::type_name::<T>());
            }
        })
    }

    pub fn clear_globals(&mut self) {
        self.cx.globals.clear();
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.update(|this| {
            let model_id = post_inc(&mut this.next_entity_id);
            let handle = ModelHandle::new(model_id, &this.cx.ref_counts);
            let mut cx = ModelContext::new(this, model_id);
            let model = build_model(&mut cx);
            this.cx.models.insert(model_id, Box::new(model));
            handle
        })
    }

    pub fn add_window<T, F>(
        &mut self,
        window_options: WindowOptions,
        build_root_view: F,
    ) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.update(|this| {
            let window_id = post_inc(&mut this.next_window_id);
            let root_view = this
                .build_and_insert_view(window_id, ParentId::Root, |cx| Some(build_root_view(cx)))
                .unwrap();
            this.cx.windows.insert(
                window_id,
                Window {
                    root_view: root_view.clone().into(),
                    focused_view_id: Some(root_view.id()),
                    is_active: false,
                    invalidation: None,
                    is_fullscreen: false,
                },
            );
            root_view.update(this, |view, cx| view.focus_in(cx.handle().into(), cx));

            let window =
                this.cx
                    .platform
                    .open_window(window_id, window_options, this.foreground.clone());
            this.register_platform_window(window_id, window);

            (window_id, root_view)
        })
    }

    pub fn add_status_bar_item<T, F>(&mut self, build_root_view: F) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.update(|this| {
            let window_id = post_inc(&mut this.next_window_id);
            let root_view = this
                .build_and_insert_view(window_id, ParentId::Root, |cx| Some(build_root_view(cx)))
                .unwrap();
            this.cx.windows.insert(
                window_id,
                Window {
                    root_view: root_view.clone().into(),
                    focused_view_id: Some(root_view.id()),
                    is_active: false,
                    invalidation: None,
                    is_fullscreen: false,
                },
            );
            root_view.update(this, |view, cx| view.focus_in(cx.handle().into(), cx));

            let status_item = this.cx.platform.add_status_item();
            this.register_platform_window(window_id, status_item);

            (window_id, root_view)
        })
    }

    pub fn remove_status_bar_item(&mut self, id: usize) {
        self.remove_window(id);
    }

    fn register_platform_window(
        &mut self,
        window_id: usize,
        mut window: Box<dyn platform::Window>,
    ) {
        let presenter = Rc::new(RefCell::new(self.build_presenter(
            window_id,
            window.titlebar_height(),
            window.appearance(),
        )));

        {
            let mut app = self.upgrade();
            let presenter = Rc::downgrade(&presenter);

            window.on_event(Box::new(move |event| {
                app.update(|cx| {
                    if let Some(presenter) = presenter.upgrade() {
                        if let Event::KeyDown(KeyDownEvent { keystroke, .. }) = &event {
                            if cx.dispatch_keystroke(window_id, keystroke) {
                                return true;
                            }
                        }

                        presenter.borrow_mut().dispatch_event(event, false, cx)
                    } else {
                        false
                    }
                })
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_active_status_change(Box::new(move |is_active| {
                app.update(|cx| cx.window_changed_active_status(window_id, is_active))
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_resize(Box::new(move || {
                app.update(|cx| cx.window_was_resized(window_id))
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_moved(Box::new(move || {
                app.update(|cx| cx.window_was_moved(window_id))
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_fullscreen(Box::new(move |is_fullscreen| {
                app.update(|cx| cx.window_was_fullscreen_changed(window_id, is_fullscreen))
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_close(Box::new(move || {
                app.update(|cx| cx.remove_window(window_id));
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_appearance_changed(Box::new(move || app.update(|cx| cx.refresh_windows())));
        }

        window.set_input_handler(Box::new(WindowInputHandler {
            app: self.upgrade().0,
            window_id,
        }));

        let scene = presenter.borrow_mut().build_scene(
            window.content_size(),
            window.scale_factor(),
            false,
            self,
        );
        window.present_scene(scene);
        self.presenters_and_platform_windows
            .insert(window_id, (presenter.clone(), window));
    }

    pub fn replace_root_view<T, F>(&mut self, window_id: usize, build_root_view: F) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.update(|this| {
            let root_view = this
                .build_and_insert_view(window_id, ParentId::Root, |cx| Some(build_root_view(cx)))
                .unwrap();
            let window = this.cx.windows.get_mut(&window_id).unwrap();
            window.root_view = root_view.clone().into();
            window.focused_view_id = Some(root_view.id());
            root_view
        })
    }

    pub fn remove_window(&mut self, window_id: usize) {
        self.cx.windows.remove(&window_id);
        self.presenters_and_platform_windows.remove(&window_id);
        self.flush_effects();
    }

    pub fn build_presenter(
        &mut self,
        window_id: usize,
        titlebar_height: f32,
        appearance: Appearance,
    ) -> Presenter {
        Presenter::new(
            window_id,
            titlebar_height,
            appearance,
            self.cx.font_cache.clone(),
            TextLayoutCache::new(self.cx.platform.fonts()),
            self.assets.clone(),
            self,
        )
    }

    pub fn add_view<T, F>(
        &mut self,
        parent_handle: impl Into<AnyViewHandle>,
        build_view: F,
    ) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        let parent_handle = parent_handle.into();
        self.build_and_insert_view(
            parent_handle.window_id,
            ParentId::View(parent_handle.view_id),
            |cx| Some(build_view(cx)),
        )
        .unwrap()
    }

    pub fn add_option_view<T, F>(
        &mut self,
        parent_handle: impl Into<AnyViewHandle>,
        build_view: F,
    ) -> Option<ViewHandle<T>>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> Option<T>,
    {
        let parent_handle = parent_handle.into();
        self.build_and_insert_view(
            parent_handle.window_id,
            ParentId::View(parent_handle.view_id),
            build_view,
        )
    }

    pub(crate) fn build_and_insert_view<T, F>(
        &mut self,
        window_id: usize,
        parent_id: ParentId,
        build_view: F,
    ) -> Option<ViewHandle<T>>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> Option<T>,
    {
        self.update(|this| {
            let view_id = post_inc(&mut this.next_entity_id);
            // Make sure we can tell child views about their parent
            this.cx.parents.insert((window_id, view_id), parent_id);
            let mut cx = ViewContext::new(this, window_id, view_id);
            let handle = if let Some(view) = build_view(&mut cx) {
                this.cx.views.insert((window_id, view_id), Box::new(view));
                if let Some(window) = this.cx.windows.get_mut(&window_id) {
                    window
                        .invalidation
                        .get_or_insert_with(Default::default)
                        .updated
                        .insert(view_id);
                }
                Some(ViewHandle::new(window_id, view_id, &this.cx.ref_counts))
            } else {
                this.cx.parents.remove(&(window_id, view_id));
                None
            };
            handle
        })
    }

    fn remove_dropped_entities(&mut self) {
        loop {
            let (dropped_models, dropped_views, dropped_element_states) =
                self.cx.ref_counts.lock().take_dropped();
            if dropped_models.is_empty()
                && dropped_views.is_empty()
                && dropped_element_states.is_empty()
            {
                break;
            }

            for model_id in dropped_models {
                self.subscriptions.remove(model_id);
                self.observations.remove(model_id);
                let mut model = self.cx.models.remove(&model_id).unwrap();
                model.release(self);
                self.pending_effects
                    .push_back(Effect::ModelRelease { model_id, model });
            }

            for (window_id, view_id) in dropped_views {
                self.subscriptions.remove(view_id);
                self.observations.remove(view_id);
                let mut view = self.cx.views.remove(&(window_id, view_id)).unwrap();
                view.release(self);
                let change_focus_to = self.cx.windows.get_mut(&window_id).and_then(|window| {
                    window
                        .invalidation
                        .get_or_insert_with(Default::default)
                        .removed
                        .push(view_id);
                    if window.focused_view_id == Some(view_id) {
                        Some(window.root_view.id())
                    } else {
                        None
                    }
                });
                self.cx.parents.remove(&(window_id, view_id));

                if let Some(view_id) = change_focus_to {
                    self.handle_focus_effect(window_id, Some(view_id));
                }

                self.pending_effects
                    .push_back(Effect::ViewRelease { view_id, view });
            }

            for key in dropped_element_states {
                self.cx.element_states.remove(&key);
            }
        }
    }

    fn flush_effects(&mut self) {
        self.pending_flushes = self.pending_flushes.saturating_sub(1);
        let mut after_window_update_callbacks = Vec::new();

        if !self.flushing_effects && self.pending_flushes == 0 {
            self.flushing_effects = true;

            let mut refreshing = false;
            loop {
                if let Some(effect) = self.pending_effects.pop_front() {
                    match effect {
                        Effect::Subscription {
                            entity_id,
                            subscription_id,
                            callback,
                        } => self
                            .subscriptions
                            .add_callback(entity_id, subscription_id, callback),

                        Effect::Event { entity_id, payload } => {
                            let mut subscriptions = self.subscriptions.clone();
                            subscriptions.emit(entity_id, self, |callback, this| {
                                callback(payload.as_ref(), this)
                            })
                        }

                        Effect::GlobalSubscription {
                            type_id,
                            subscription_id,
                            callback,
                        } => self.global_subscriptions.add_callback(
                            type_id,
                            subscription_id,
                            callback,
                        ),

                        Effect::GlobalEvent { payload } => self.emit_global_event(payload),

                        Effect::Observation {
                            entity_id,
                            subscription_id,
                            callback,
                        } => self
                            .observations
                            .add_callback(entity_id, subscription_id, callback),

                        Effect::ModelNotification { model_id } => {
                            let mut observations = self.observations.clone();
                            observations.emit(model_id, self, |callback, this| callback(this));
                        }

                        Effect::ViewNotification { window_id, view_id } => {
                            self.handle_view_notification_effect(window_id, view_id)
                        }

                        Effect::GlobalNotification { type_id } => {
                            let mut subscriptions = self.global_observations.clone();
                            subscriptions.emit(type_id, self, |callback, this| {
                                callback(this);
                                true
                            });
                        }

                        Effect::Deferred {
                            callback,
                            after_window_update,
                        } => {
                            if after_window_update {
                                after_window_update_callbacks.push(callback);
                            } else {
                                callback(self)
                            }
                        }

                        Effect::ModelRelease { model_id, model } => {
                            self.handle_entity_release_effect(model_id, model.as_any())
                        }

                        Effect::ViewRelease { view_id, view } => {
                            self.handle_entity_release_effect(view_id, view.as_any())
                        }

                        Effect::Focus { window_id, view_id } => {
                            self.handle_focus_effect(window_id, view_id);
                        }

                        Effect::FocusObservation {
                            view_id,
                            subscription_id,
                            callback,
                        } => {
                            self.focus_observations.add_callback(
                                view_id,
                                subscription_id,
                                callback,
                            );
                        }

                        Effect::ResizeWindow { window_id } => {
                            if let Some(window) = self.cx.windows.get_mut(&window_id) {
                                window
                                    .invalidation
                                    .get_or_insert(WindowInvalidation::default());
                            }
                            self.handle_window_moved(window_id);
                        }

                        Effect::MoveWindow { window_id } => {
                            self.handle_window_moved(window_id);
                        }

                        Effect::WindowActivationObservation {
                            window_id,
                            subscription_id,
                            callback,
                        } => self.window_activation_observations.add_callback(
                            window_id,
                            subscription_id,
                            callback,
                        ),

                        Effect::ActivateWindow {
                            window_id,
                            is_active,
                        } => self.handle_window_activation_effect(window_id, is_active),

                        Effect::WindowFullscreenObservation {
                            window_id,
                            subscription_id,
                            callback,
                        } => self.window_fullscreen_observations.add_callback(
                            window_id,
                            subscription_id,
                            callback,
                        ),

                        Effect::FullscreenWindow {
                            window_id,
                            is_fullscreen,
                        } => self.handle_fullscreen_effect(window_id, is_fullscreen),

                        Effect::WindowBoundsObservation {
                            window_id,
                            subscription_id,
                            callback,
                        } => self.window_bounds_observations.add_callback(
                            window_id,
                            subscription_id,
                            callback,
                        ),

                        Effect::RefreshWindows => {
                            refreshing = true;
                        }
                        Effect::DispatchActionFrom {
                            window_id,
                            view_id,
                            action,
                        } => {
                            self.handle_dispatch_action_from_effect(
                                window_id,
                                Some(view_id),
                                action.as_ref(),
                            );
                        }
                        Effect::ActionDispatchNotification { action_id } => {
                            self.handle_action_dispatch_notification_effect(action_id)
                        }
                        Effect::WindowShouldCloseSubscription {
                            window_id,
                            callback,
                        } => {
                            self.handle_window_should_close_subscription_effect(window_id, callback)
                        }
                        Effect::Keystroke {
                            window_id,
                            keystroke,
                            handled_by,
                            result,
                        } => self.handle_keystroke_effect(window_id, keystroke, handled_by, result),
                        Effect::ActiveLabeledTasksChanged => {
                            self.handle_active_labeled_tasks_changed_effect()
                        }
                        Effect::ActiveLabeledTasksObservation {
                            subscription_id,
                            callback,
                        } => self.active_labeled_task_observations.add_callback(
                            (),
                            subscription_id,
                            callback,
                        ),
                    }
                    self.pending_notifications.clear();
                    self.remove_dropped_entities();
                } else {
                    self.remove_dropped_entities();

                    if refreshing {
                        self.perform_window_refresh();
                    } else {
                        self.update_windows();
                    }

                    if self.pending_effects.is_empty() {
                        for callback in after_window_update_callbacks.drain(..) {
                            callback(self);
                        }

                        if self.pending_effects.is_empty() {
                            self.flushing_effects = false;
                            self.pending_notifications.clear();
                            self.pending_global_notifications.clear();
                            break;
                        }
                    }

                    refreshing = false;
                }
            }
        }
    }

    fn update_windows(&mut self) {
        let mut invalidations: HashMap<_, _> = Default::default();
        for (window_id, window) in &mut self.cx.windows {
            if let Some(invalidation) = window.invalidation.take() {
                invalidations.insert(*window_id, invalidation);
            }
        }

        for (window_id, mut invalidation) in invalidations {
            if let Some((presenter, mut window)) =
                self.presenters_and_platform_windows.remove(&window_id)
            {
                {
                    let mut presenter = presenter.borrow_mut();
                    presenter.invalidate(&mut invalidation, window.appearance(), self);
                    let scene = presenter.build_scene(
                        window.content_size(),
                        window.scale_factor(),
                        false,
                        self,
                    );
                    window.present_scene(scene);
                }
                self.presenters_and_platform_windows
                    .insert(window_id, (presenter, window));
            }
        }
    }

    fn window_was_resized(&mut self, window_id: usize) {
        self.pending_effects
            .push_back(Effect::ResizeWindow { window_id });
    }

    fn window_was_moved(&mut self, window_id: usize) {
        self.pending_effects
            .push_back(Effect::MoveWindow { window_id });
    }

    fn window_was_fullscreen_changed(&mut self, window_id: usize, is_fullscreen: bool) {
        self.pending_effects.push_back(Effect::FullscreenWindow {
            window_id,
            is_fullscreen,
        });
    }

    fn window_changed_active_status(&mut self, window_id: usize, is_active: bool) {
        self.pending_effects.push_back(Effect::ActivateWindow {
            window_id,
            is_active,
        });
    }

    fn keystroke(
        &mut self,
        window_id: usize,
        keystroke: Keystroke,
        handled_by: Option<Box<dyn Action>>,
        result: MatchResult,
    ) {
        self.pending_effects.push_back(Effect::Keystroke {
            window_id,
            keystroke,
            handled_by,
            result,
        });
    }

    pub fn refresh_windows(&mut self) {
        self.pending_effects.push_back(Effect::RefreshWindows);
    }

    pub fn dispatch_action_at(&mut self, window_id: usize, view_id: usize, action: impl Action) {
        self.dispatch_any_action_at(window_id, view_id, Box::new(action));
    }

    pub fn dispatch_any_action_at(
        &mut self,
        window_id: usize,
        view_id: usize,
        action: Box<dyn Action>,
    ) {
        self.pending_effects.push_back(Effect::DispatchActionFrom {
            window_id,
            view_id,
            action,
        });
    }

    fn perform_window_refresh(&mut self) {
        let mut presenters = mem::take(&mut self.presenters_and_platform_windows);
        for (window_id, (presenter, window)) in &mut presenters {
            let mut invalidation = self
                .cx
                .windows
                .get_mut(window_id)
                .unwrap()
                .invalidation
                .take();
            let mut presenter = presenter.borrow_mut();
            presenter.refresh(
                invalidation.as_mut().unwrap_or(&mut Default::default()),
                window.appearance(),
                self,
            );
            let scene =
                presenter.build_scene(window.content_size(), window.scale_factor(), true, self);
            window.present_scene(scene);
        }
        self.presenters_and_platform_windows = presenters;
    }

    fn emit_global_event(&mut self, payload: Box<dyn Any>) {
        let type_id = (&*payload).type_id();

        let mut subscriptions = self.global_subscriptions.clone();
        subscriptions.emit(type_id, self, |callback, this| {
            callback(payload.as_ref(), this);
            true //Always alive
        });
    }

    fn handle_view_notification_effect(
        &mut self,
        observed_window_id: usize,
        observed_view_id: usize,
    ) {
        if self
            .cx
            .views
            .contains_key(&(observed_window_id, observed_view_id))
        {
            if let Some(window) = self.cx.windows.get_mut(&observed_window_id) {
                window
                    .invalidation
                    .get_or_insert_with(Default::default)
                    .updated
                    .insert(observed_view_id);
            }

            let mut observations = self.observations.clone();
            observations.emit(observed_view_id, self, |callback, this| callback(this));
        }
    }

    fn handle_entity_release_effect(&mut self, entity_id: usize, entity: &dyn Any) {
        self.release_observations
            .clone()
            .emit(entity_id, self, |callback, this| {
                callback(entity, this);
                // Release observations happen one time. So clear the callback by returning false
                false
            })
    }

    fn handle_fullscreen_effect(&mut self, window_id: usize, is_fullscreen: bool) {
        //Short circuit evaluation if we're already g2g
        if self
            .cx
            .windows
            .get(&window_id)
            .map(|w| w.is_fullscreen == is_fullscreen)
            .unwrap_or(false)
        {
            return;
        }

        self.update(|this| {
            let window = this.cx.windows.get_mut(&window_id)?;
            window.is_fullscreen = is_fullscreen;

            let mut fullscreen_observations = this.window_fullscreen_observations.clone();
            fullscreen_observations.emit(window_id, this, |callback, this| {
                callback(is_fullscreen, this)
            });

            if let Some((uuid, bounds)) = this
                .window_display_uuid(window_id)
                .zip(this.window_bounds(window_id))
            {
                let mut bounds_observations = this.window_bounds_observations.clone();
                bounds_observations.emit(window_id, this, |callback, this| {
                    callback(bounds, uuid, this)
                });
            }

            Some(())
        });
    }

    fn handle_keystroke_effect(
        &mut self,
        window_id: usize,
        keystroke: Keystroke,
        handled_by: Option<Box<dyn Action>>,
        result: MatchResult,
    ) {
        self.update(|this| {
            let mut observations = this.keystroke_observations.clone();
            observations.emit(window_id, this, {
                move |callback, this| callback(&keystroke, &result, handled_by.as_ref(), this)
            });
        });
    }

    fn handle_window_activation_effect(&mut self, window_id: usize, active: bool) {
        //Short circuit evaluation if we're already g2g
        if self
            .cx
            .windows
            .get(&window_id)
            .map(|w| w.is_active == active)
            .unwrap_or(false)
        {
            return;
        }

        self.update(|this| {
            let window = this.cx.windows.get_mut(&window_id)?;
            window.is_active = active;

            //Handle focus
            let focused_id = window.focused_view_id?;
            for view_id in this.ancestors(window_id, focused_id).collect::<Vec<_>>() {
                if let Some(mut view) = this.cx.views.remove(&(window_id, view_id)) {
                    if active {
                        view.focus_in(this, window_id, view_id, focused_id);
                    } else {
                        view.focus_out(this, window_id, view_id, focused_id);
                    }
                    this.cx.views.insert((window_id, view_id), view);
                }
            }

            let mut observations = this.window_activation_observations.clone();
            observations.emit(window_id, this, |callback, this| callback(active, this));

            Some(())
        });
    }

    fn handle_focus_effect(&mut self, window_id: usize, focused_id: Option<usize>) {
        if self
            .cx
            .windows
            .get(&window_id)
            .map(|w| w.focused_view_id)
            .map_or(false, |cur_focused| cur_focused == focused_id)
        {
            return;
        }

        self.update(|this| {
            let blurred_id = this.cx.windows.get_mut(&window_id).and_then(|window| {
                let blurred_id = window.focused_view_id;
                window.focused_view_id = focused_id;
                blurred_id
            });

            let blurred_parents = blurred_id
                .map(|blurred_id| this.ancestors(window_id, blurred_id).collect::<Vec<_>>())
                .unwrap_or_default();
            let focused_parents = focused_id
                .map(|focused_id| this.ancestors(window_id, focused_id).collect::<Vec<_>>())
                .unwrap_or_default();

            if let Some(blurred_id) = blurred_id {
                for view_id in blurred_parents.iter().copied() {
                    if let Some(mut view) = this.cx.views.remove(&(window_id, view_id)) {
                        view.focus_out(this, window_id, view_id, blurred_id);
                        this.cx.views.insert((window_id, view_id), view);
                    }
                }

                let mut subscriptions = this.focus_observations.clone();
                subscriptions.emit(blurred_id, this, |callback, this| callback(false, this));
            }

            if let Some(focused_id) = focused_id {
                for view_id in focused_parents {
                    if let Some(mut view) = this.cx.views.remove(&(window_id, view_id)) {
                        view.focus_in(this, window_id, view_id, focused_id);
                        this.cx.views.insert((window_id, view_id), view);
                    }
                }

                let mut subscriptions = this.focus_observations.clone();
                subscriptions.emit(focused_id, this, |callback, this| callback(true, this));
            }
        })
    }

    fn handle_dispatch_action_from_effect(
        &mut self,
        window_id: usize,
        view_id: Option<usize>,
        action: &dyn Action,
    ) -> bool {
        self.update(|this| {
            if let Some(view_id) = view_id {
                this.halt_action_dispatch = false;
                this.visit_dispatch_path(window_id, view_id, |view_id, capture_phase, this| {
                    if let Some(mut view) = this.cx.views.remove(&(window_id, view_id)) {
                        let type_id = view.as_any().type_id();

                        if let Some((name, mut handlers)) = this
                            .actions_mut(capture_phase)
                            .get_mut(&type_id)
                            .and_then(|h| h.remove_entry(&action.id()))
                        {
                            for handler in handlers.iter_mut().rev() {
                                this.halt_action_dispatch = true;
                                handler(view.as_mut(), action, this, window_id, view_id);
                                if this.halt_action_dispatch {
                                    break;
                                }
                            }
                            this.actions_mut(capture_phase)
                                .get_mut(&type_id)
                                .unwrap()
                                .insert(name, handlers);
                        }

                        this.cx.views.insert((window_id, view_id), view);
                    }

                    !this.halt_action_dispatch
                });
            }

            if !this.halt_action_dispatch {
                this.halt_action_dispatch = this.dispatch_global_action_any(action);
            }

            this.pending_effects
                .push_back(Effect::ActionDispatchNotification {
                    action_id: action.id(),
                });
            this.halt_action_dispatch
        })
    }

    fn handle_action_dispatch_notification_effect(&mut self, action_id: TypeId) {
        self.action_dispatch_observations
            .clone()
            .emit((), self, |callback, this| {
                callback(action_id, this);
                true
            });
    }

    fn handle_window_should_close_subscription_effect(
        &mut self,
        window_id: usize,
        mut callback: WindowShouldCloseSubscriptionCallback,
    ) {
        let mut app = self.upgrade();
        if let Some((_, window)) = self.presenters_and_platform_windows.get_mut(&window_id) {
            window.on_should_close(Box::new(move || app.update(|cx| callback(cx))))
        }
    }

    fn handle_window_moved(&mut self, window_id: usize) {
        if let Some((display, bounds)) = self
            .window_display_uuid(window_id)
            .zip(self.window_bounds(window_id))
        {
            self.window_bounds_observations
                .clone()
                .emit(window_id, self, move |callback, this| {
                    callback(bounds, display, this);
                    true
                });
        }
    }

    fn handle_active_labeled_tasks_changed_effect(&mut self) {
        self.active_labeled_task_observations
            .clone()
            .emit((), self, move |callback, this| {
                callback(this);
                true
            });
    }

    pub fn focus(&mut self, window_id: usize, view_id: Option<usize>) {
        self.pending_effects
            .push_back(Effect::Focus { window_id, view_id });
    }

    fn spawn_internal<F, Fut, T>(&mut self, task_name: Option<&'static str>, f: F) -> Task<T>
    where
        F: FnOnce(AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = T>,
        T: 'static,
    {
        let label_id = task_name.map(|task_name| {
            let id = post_inc(&mut self.next_labeled_task_id);
            self.active_labeled_tasks.insert(id, task_name);
            self.pending_effects
                .push_back(Effect::ActiveLabeledTasksChanged);
            id
        });

        let future = f(self.to_async());
        let cx = self.to_async();
        self.foreground.spawn(async move {
            let result = future.await;
            let mut cx = cx.0.borrow_mut();

            if let Some(completed_label_id) = label_id {
                cx.active_labeled_tasks.remove(&completed_label_id);
                cx.pending_effects
                    .push_back(Effect::ActiveLabeledTasksChanged);
            }
            cx.flush_effects();
            result
        })
    }

    pub fn spawn_labeled<F, Fut, T>(&mut self, task_name: &'static str, f: F) -> Task<T>
    where
        F: FnOnce(AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = T>,
        T: 'static,
    {
        self.spawn_internal(Some(task_name), f)
    }

    pub fn spawn<F, Fut, T>(&mut self, f: F) -> Task<T>
    where
        F: FnOnce(AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = T>,
        T: 'static,
    {
        self.spawn_internal(None, f)
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext(self.weak_self.as_ref().unwrap().upgrade().unwrap())
    }

    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.cx.platform.write_to_clipboard(item);
    }

    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.cx.platform.read_from_clipboard()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn leak_detector(&self) -> Arc<Mutex<LeakDetector>> {
        self.cx.ref_counts.lock().leak_detector.clone()
    }
}

impl ReadModel for MutableAppContext {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        if let Some(model) = self.cx.models.get(&handle.model_id) {
            model
                .as_any()
                .downcast_ref()
                .expect("downcast is type safe")
        } else {
            panic!("circular model reference");
        }
    }
}

impl UpdateModel for MutableAppContext {
    fn update_model<T: Entity, V>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> V,
    ) -> V {
        if let Some(mut model) = self.cx.models.remove(&handle.model_id) {
            self.update(|this| {
                let mut cx = ModelContext::new(this, handle.model_id);
                let result = update(
                    model
                        .as_any_mut()
                        .downcast_mut()
                        .expect("downcast is type safe"),
                    &mut cx,
                );
                this.cx.models.insert(handle.model_id, model);
                result
            })
        } else {
            panic!("circular model update");
        }
    }
}

impl UpgradeModelHandle for MutableAppContext {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        self.cx.upgrade_model_handle(handle)
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.cx.model_handle_is_upgradable(handle)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        self.cx.upgrade_any_model_handle(handle)
    }
}

impl UpgradeViewHandle for MutableAppContext {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
        self.cx.upgrade_view_handle(handle)
    }

    fn upgrade_any_view_handle(&self, handle: &AnyWeakViewHandle) -> Option<AnyViewHandle> {
        self.cx.upgrade_any_view_handle(handle)
    }
}

impl ReadView for MutableAppContext {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        if let Some(view) = self.cx.views.get(&(handle.window_id, handle.view_id)) {
            view.as_any().downcast_ref().expect("downcast is type safe")
        } else {
            panic!("circular view reference for type {}", type_name::<T>());
        }
    }
}

impl UpdateView for MutableAppContext {
    fn update_view<T, S>(
        &mut self,
        handle: &ViewHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
    ) -> S
    where
        T: View,
    {
        self.update(|this| {
            let mut view = this
                .cx
                .views
                .remove(&(handle.window_id, handle.view_id))
                .expect("circular view update");

            let mut cx = ViewContext::new(this, handle.window_id, handle.view_id);
            let result = update(
                view.as_any_mut()
                    .downcast_mut()
                    .expect("downcast is type safe"),
                &mut cx,
            );
            this.cx
                .views
                .insert((handle.window_id, handle.view_id), view);
            result
        })
    }
}

impl AsRef<AppContext> for MutableAppContext {
    fn as_ref(&self) -> &AppContext {
        &self.cx
    }
}

impl Deref for MutableAppContext {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

#[derive(Debug)]
pub enum ParentId {
    View(usize),
    Root,
}

pub struct AppContext {
    models: HashMap<usize, Box<dyn AnyModel>>,
    views: HashMap<(usize, usize), Box<dyn AnyView>>,
    pub(crate) parents: HashMap<(usize, usize), ParentId>,
    windows: HashMap<usize, Window>,
    globals: HashMap<TypeId, Box<dyn Any>>,
    element_states: HashMap<ElementStateId, Box<dyn Any>>,
    background: Arc<executor::Background>,
    ref_counts: Arc<Mutex<RefCounts>>,
    font_cache: Arc<FontCache>,
    platform: Arc<dyn Platform>,
}

impl AppContext {
    pub(crate) fn root_view(&self, window_id: usize) -> Option<AnyViewHandle> {
        self.windows
            .get(&window_id)
            .map(|window| window.root_view.clone())
    }

    pub fn root_view_id(&self, window_id: usize) -> Option<usize> {
        self.windows
            .get(&window_id)
            .map(|window| window.root_view.id())
    }

    pub fn focused_view_id(&self, window_id: usize) -> Option<usize> {
        self.windows
            .get(&window_id)
            .and_then(|window| window.focused_view_id)
    }

    pub fn view_ui_name(&self, window_id: usize, view_id: usize) -> Option<&'static str> {
        Some(self.views.get(&(window_id, view_id))?.ui_name())
    }

    pub fn background(&self) -> &Arc<executor::Background> {
        &self.background
    }

    pub fn font_cache(&self) -> &Arc<FontCache> {
        &self.font_cache
    }

    pub fn platform(&self) -> &Arc<dyn Platform> {
        &self.platform
    }

    pub fn has_global<T: 'static>(&self) -> bool {
        self.globals.contains_key(&TypeId::of::<T>())
    }

    pub fn global<T: 'static>(&self) -> &T {
        if let Some(global) = self.globals.get(&TypeId::of::<T>()) {
            global.downcast_ref().unwrap()
        } else {
            panic!("no global has been added for {}", type_name::<T>());
        }
    }

    /// Returns an iterator over all of the view ids from the passed view up to the root of the window
    /// Includes the passed view itself
    fn ancestors(&self, window_id: usize, mut view_id: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(view_id)
            .into_iter()
            .chain(std::iter::from_fn(move || {
                if let Some(ParentId::View(parent_id)) = self.parents.get(&(window_id, view_id)) {
                    view_id = *parent_id;
                    Some(view_id)
                } else {
                    None
                }
            }))
    }

    /// Returns the id of the parent of the given view, or none if the given
    /// view is the root.
    fn parent(&self, window_id: usize, view_id: usize) -> Option<usize> {
        if let Some(ParentId::View(view_id)) = self.parents.get(&(window_id, view_id)) {
            Some(*view_id)
        } else {
            None
        }
    }

    pub fn is_child_focused(&self, view: impl Into<AnyViewHandle>) -> bool {
        let view = view.into();
        if let Some(focused_view_id) = self.focused_view_id(view.window_id) {
            self.ancestors(view.window_id, focused_view_id)
                .skip(1) // Skip self id
                .any(|parent| parent == view.view_id)
        } else {
            false
        }
    }
}

impl ReadModel for AppContext {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        if let Some(model) = self.models.get(&handle.model_id) {
            model
                .as_any()
                .downcast_ref()
                .expect("downcast should be type safe")
        } else {
            panic!("circular model reference");
        }
    }
}

impl UpgradeModelHandle for AppContext {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        if self.models.contains_key(&handle.model_id) {
            Some(ModelHandle::new(handle.model_id, &self.ref_counts))
        } else {
            None
        }
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.models.contains_key(&handle.model_id)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        if self.models.contains_key(&handle.model_id) {
            Some(AnyModelHandle::new(
                handle.model_id,
                handle.model_type,
                self.ref_counts.clone(),
            ))
        } else {
            None
        }
    }
}

impl UpgradeViewHandle for AppContext {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
        if self.ref_counts.lock().is_entity_alive(handle.view_id) {
            Some(ViewHandle::new(
                handle.window_id,
                handle.view_id,
                &self.ref_counts,
            ))
        } else {
            None
        }
    }

    fn upgrade_any_view_handle(&self, handle: &AnyWeakViewHandle) -> Option<AnyViewHandle> {
        if self.ref_counts.lock().is_entity_alive(handle.view_id) {
            Some(AnyViewHandle::new(
                handle.window_id,
                handle.view_id,
                handle.view_type,
                self.ref_counts.clone(),
            ))
        } else {
            None
        }
    }
}

impl ReadView for AppContext {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        if let Some(view) = self.views.get(&(handle.window_id, handle.view_id)) {
            view.as_any()
                .downcast_ref()
                .expect("downcast should be type safe")
        } else {
            panic!("circular view reference");
        }
    }
}

struct Window {
    root_view: AnyViewHandle,
    focused_view_id: Option<usize>,
    is_active: bool,
    is_fullscreen: bool,
    invalidation: Option<WindowInvalidation>,
}

#[derive(Default, Clone)]
pub struct WindowInvalidation {
    pub updated: HashSet<usize>,
    pub removed: Vec<usize>,
}

pub enum Effect {
    Subscription {
        entity_id: usize,
        subscription_id: usize,
        callback: SubscriptionCallback,
    },
    Event {
        entity_id: usize,
        payload: Box<dyn Any>,
    },
    GlobalSubscription {
        type_id: TypeId,
        subscription_id: usize,
        callback: GlobalSubscriptionCallback,
    },
    GlobalEvent {
        payload: Box<dyn Any>,
    },
    Observation {
        entity_id: usize,
        subscription_id: usize,
        callback: ObservationCallback,
    },
    ModelNotification {
        model_id: usize,
    },
    ViewNotification {
        window_id: usize,
        view_id: usize,
    },
    Deferred {
        callback: Box<dyn FnOnce(&mut MutableAppContext)>,
        after_window_update: bool,
    },
    GlobalNotification {
        type_id: TypeId,
    },
    ModelRelease {
        model_id: usize,
        model: Box<dyn AnyModel>,
    },
    ViewRelease {
        view_id: usize,
        view: Box<dyn AnyView>,
    },
    Focus {
        window_id: usize,
        view_id: Option<usize>,
    },
    FocusObservation {
        view_id: usize,
        subscription_id: usize,
        callback: FocusObservationCallback,
    },
    ResizeWindow {
        window_id: usize,
    },
    MoveWindow {
        window_id: usize,
    },
    ActivateWindow {
        window_id: usize,
        is_active: bool,
    },
    WindowActivationObservation {
        window_id: usize,
        subscription_id: usize,
        callback: WindowActivationCallback,
    },
    FullscreenWindow {
        window_id: usize,
        is_fullscreen: bool,
    },
    WindowFullscreenObservation {
        window_id: usize,
        subscription_id: usize,
        callback: WindowFullscreenCallback,
    },
    WindowBoundsObservation {
        window_id: usize,
        subscription_id: usize,
        callback: WindowBoundsCallback,
    },
    Keystroke {
        window_id: usize,
        keystroke: Keystroke,
        handled_by: Option<Box<dyn Action>>,
        result: MatchResult,
    },
    RefreshWindows,
    DispatchActionFrom {
        window_id: usize,
        view_id: usize,
        action: Box<dyn Action>,
    },
    ActionDispatchNotification {
        action_id: TypeId,
    },
    WindowShouldCloseSubscription {
        window_id: usize,
        callback: WindowShouldCloseSubscriptionCallback,
    },
    ActiveLabeledTasksChanged,
    ActiveLabeledTasksObservation {
        subscription_id: usize,
        callback: ActiveLabeledTasksCallback,
    },
}

impl Debug for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Effect::Subscription {
                entity_id,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::Subscribe")
                .field("entity_id", entity_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::Event { entity_id, .. } => f
                .debug_struct("Effect::Event")
                .field("entity_id", entity_id)
                .finish(),
            Effect::GlobalSubscription {
                type_id,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::Subscribe")
                .field("type_id", type_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::GlobalEvent { payload, .. } => f
                .debug_struct("Effect::GlobalEvent")
                .field("type_id", &(&*payload).type_id())
                .finish(),
            Effect::Observation {
                entity_id,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::Observation")
                .field("entity_id", entity_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::ModelNotification { model_id } => f
                .debug_struct("Effect::ModelNotification")
                .field("model_id", model_id)
                .finish(),
            Effect::ViewNotification { window_id, view_id } => f
                .debug_struct("Effect::ViewNotification")
                .field("window_id", window_id)
                .field("view_id", view_id)
                .finish(),
            Effect::GlobalNotification { type_id } => f
                .debug_struct("Effect::GlobalNotification")
                .field("type_id", type_id)
                .finish(),
            Effect::Deferred { .. } => f.debug_struct("Effect::Deferred").finish(),
            Effect::ModelRelease { model_id, .. } => f
                .debug_struct("Effect::ModelRelease")
                .field("model_id", model_id)
                .finish(),
            Effect::ViewRelease { view_id, .. } => f
                .debug_struct("Effect::ViewRelease")
                .field("view_id", view_id)
                .finish(),
            Effect::Focus { window_id, view_id } => f
                .debug_struct("Effect::Focus")
                .field("window_id", window_id)
                .field("view_id", view_id)
                .finish(),
            Effect::FocusObservation {
                view_id,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::FocusObservation")
                .field("view_id", view_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::DispatchActionFrom {
                window_id, view_id, ..
            } => f
                .debug_struct("Effect::DispatchActionFrom")
                .field("window_id", window_id)
                .field("view_id", view_id)
                .finish(),
            Effect::ActionDispatchNotification { action_id, .. } => f
                .debug_struct("Effect::ActionDispatchNotification")
                .field("action_id", action_id)
                .finish(),
            Effect::ResizeWindow { window_id } => f
                .debug_struct("Effect::RefreshWindow")
                .field("window_id", window_id)
                .finish(),
            Effect::MoveWindow { window_id } => f
                .debug_struct("Effect::MoveWindow")
                .field("window_id", window_id)
                .finish(),
            Effect::WindowActivationObservation {
                window_id,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::WindowActivationObservation")
                .field("window_id", window_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::ActivateWindow {
                window_id,
                is_active,
            } => f
                .debug_struct("Effect::ActivateWindow")
                .field("window_id", window_id)
                .field("is_active", is_active)
                .finish(),
            Effect::FullscreenWindow {
                window_id,
                is_fullscreen,
            } => f
                .debug_struct("Effect::FullscreenWindow")
                .field("window_id", window_id)
                .field("is_fullscreen", is_fullscreen)
                .finish(),
            Effect::WindowFullscreenObservation {
                window_id,
                subscription_id,
                callback: _,
            } => f
                .debug_struct("Effect::WindowFullscreenObservation")
                .field("window_id", window_id)
                .field("subscription_id", subscription_id)
                .finish(),

            Effect::WindowBoundsObservation {
                window_id,
                subscription_id,
                callback: _,
            } => f
                .debug_struct("Effect::WindowBoundsObservation")
                .field("window_id", window_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::RefreshWindows => f.debug_struct("Effect::FullViewRefresh").finish(),
            Effect::WindowShouldCloseSubscription { window_id, .. } => f
                .debug_struct("Effect::WindowShouldCloseSubscription")
                .field("window_id", window_id)
                .finish(),
            Effect::Keystroke {
                window_id,
                keystroke,
                handled_by,
                result,
            } => f
                .debug_struct("Effect::Keystroke")
                .field("window_id", window_id)
                .field("keystroke", keystroke)
                .field(
                    "keystroke",
                    &handled_by.as_ref().map(|handled_by| handled_by.name()),
                )
                .field("result", result)
                .finish(),
            Effect::ActiveLabeledTasksChanged => {
                f.debug_struct("Effect::ActiveLabeledTasksChanged").finish()
            }
            Effect::ActiveLabeledTasksObservation {
                subscription_id,
                callback: _,
            } => f
                .debug_struct("Effect::ActiveLabeledTasksObservation")
                .field("subscription_id", subscription_id)
                .finish(),
        }
    }
}

pub trait AnyModel {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn release(&mut self, cx: &mut MutableAppContext);
    fn app_will_quit(
        &mut self,
        cx: &mut MutableAppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>>;
}

impl<T> AnyModel for T
where
    T: Entity,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn release(&mut self, cx: &mut MutableAppContext) {
        self.release(cx);
    }

    fn app_will_quit(
        &mut self,
        cx: &mut MutableAppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>> {
        self.app_will_quit(cx)
    }
}

pub trait AnyView {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn release(&mut self, cx: &mut MutableAppContext);
    fn app_will_quit(
        &mut self,
        cx: &mut MutableAppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>>;
    fn ui_name(&self) -> &'static str;
    fn render(&mut self, params: RenderParams, cx: &mut MutableAppContext) -> ElementBox;
    fn focus_in(
        &mut self,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
        focused_id: usize,
    );
    fn focus_out(
        &mut self,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
        focused_id: usize,
    );
    fn key_down(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) -> bool;
    fn key_up(
        &mut self,
        event: &KeyUpEvent,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) -> bool;
    fn modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) -> bool;
    fn keymap_context(&self, cx: &AppContext) -> KeymapContext;
    fn debug_json(&self, cx: &AppContext) -> serde_json::Value;

    fn text_for_range(&self, range: Range<usize>, cx: &AppContext) -> Option<String>;
    fn selected_text_range(&self, cx: &AppContext) -> Option<Range<usize>>;
    fn marked_text_range(&self, cx: &AppContext) -> Option<Range<usize>>;
    fn unmark_text(&mut self, cx: &mut MutableAppContext, window_id: usize, view_id: usize);
    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    );
    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    );
    fn any_handle(&self, window_id: usize, view_id: usize, cx: &AppContext) -> AnyViewHandle {
        AnyViewHandle::new(
            window_id,
            view_id,
            self.as_any().type_id(),
            cx.ref_counts.clone(),
        )
    }
}

impl<T> AnyView for T
where
    T: View,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn release(&mut self, cx: &mut MutableAppContext) {
        self.release(cx);
    }

    fn app_will_quit(
        &mut self,
        cx: &mut MutableAppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>> {
        self.app_will_quit(cx)
    }

    fn ui_name(&self) -> &'static str {
        T::ui_name()
    }

    fn render(&mut self, params: RenderParams, cx: &mut MutableAppContext) -> ElementBox {
        View::render(self, &mut RenderContext::new(params, cx))
    }

    fn focus_in(
        &mut self,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
        focused_id: usize,
    ) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        let focused_view_handle: AnyViewHandle = if view_id == focused_id {
            cx.handle().into()
        } else {
            let focused_type = cx
                .views
                .get(&(window_id, focused_id))
                .unwrap()
                .as_any()
                .type_id();
            AnyViewHandle::new(window_id, focused_id, focused_type, cx.ref_counts.clone())
        };
        View::focus_in(self, focused_view_handle, &mut cx);
    }

    fn focus_out(
        &mut self,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
        blurred_id: usize,
    ) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        let blurred_view_handle: AnyViewHandle = if view_id == blurred_id {
            cx.handle().into()
        } else {
            let blurred_type = cx
                .views
                .get(&(window_id, blurred_id))
                .unwrap()
                .as_any()
                .type_id();
            AnyViewHandle::new(window_id, blurred_id, blurred_type, cx.ref_counts.clone())
        };
        View::focus_out(self, blurred_view_handle, &mut cx);
    }

    fn key_down(
        &mut self,
        event: &KeyDownEvent,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) -> bool {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::key_down(self, event, &mut cx)
    }

    fn key_up(
        &mut self,
        event: &KeyUpEvent,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) -> bool {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::key_up(self, event, &mut cx)
    }

    fn modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) -> bool {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::modifiers_changed(self, event, &mut cx)
    }

    fn keymap_context(&self, cx: &AppContext) -> KeymapContext {
        View::keymap_context(self, cx)
    }

    fn debug_json(&self, cx: &AppContext) -> serde_json::Value {
        View::debug_json(self, cx)
    }

    fn text_for_range(&self, range: Range<usize>, cx: &AppContext) -> Option<String> {
        View::text_for_range(self, range, cx)
    }

    fn selected_text_range(&self, cx: &AppContext) -> Option<Range<usize>> {
        View::selected_text_range(self, cx)
    }

    fn marked_text_range(&self, cx: &AppContext) -> Option<Range<usize>> {
        View::marked_text_range(self, cx)
    }

    fn unmark_text(&mut self, cx: &mut MutableAppContext, window_id: usize, view_id: usize) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::unmark_text(self, &mut cx)
    }

    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::replace_text_in_range(self, range, text, &mut cx)
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut MutableAppContext,
        window_id: usize,
        view_id: usize,
    ) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::replace_and_mark_text_in_range(self, range, new_text, new_selected_range, &mut cx)
    }
}

pub struct ModelContext<'a, T: ?Sized> {
    app: &'a mut MutableAppContext,
    model_id: usize,
    model_type: PhantomData<T>,
    halt_stream: bool,
}

impl<'a, T: Entity> ModelContext<'a, T> {
    fn new(app: &'a mut MutableAppContext, model_id: usize) -> Self {
        Self {
            app,
            model_id,
            model_type: PhantomData,
            halt_stream: false,
        }
    }

    pub fn background(&self) -> &Arc<executor::Background> {
        &self.app.cx.background
    }

    pub fn halt_stream(&mut self) {
        self.halt_stream = true;
    }

    pub fn model_id(&self) -> usize {
        self.model_id
    }

    pub fn add_model<S, F>(&mut self, build_model: F) -> ModelHandle<S>
    where
        S: Entity,
        F: FnOnce(&mut ModelContext<S>) -> S,
    {
        self.app.add_model(build_model)
    }

    pub fn defer(&mut self, callback: impl 'static + FnOnce(&mut T, &mut ModelContext<T>)) {
        let handle = self.handle();
        self.app.defer(move |cx| {
            handle.update(cx, |model, cx| {
                callback(model, cx);
            })
        })
    }

    pub fn emit(&mut self, payload: T::Event) {
        self.app.pending_effects.push_back(Effect::Event {
            entity_id: self.model_id,
            payload: Box::new(payload),
        });
    }

    pub fn notify(&mut self) {
        self.app.notify_model(self.model_id);
    }

    pub fn subscribe<S: Entity, F>(
        &mut self,
        handle: &ModelHandle<S>,
        mut callback: F,
    ) -> Subscription
    where
        S::Event: 'static,
        F: 'static + FnMut(&mut T, ModelHandle<S>, &S::Event, &mut ModelContext<T>),
    {
        let subscriber = self.weak_handle();
        self.app
            .subscribe_internal(handle, move |emitter, event, cx| {
                if let Some(subscriber) = subscriber.upgrade(cx) {
                    subscriber.update(cx, |subscriber, cx| {
                        callback(subscriber, emitter, event, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe<S, F>(&mut self, handle: &ModelHandle<S>, mut callback: F) -> Subscription
    where
        S: Entity,
        F: 'static + FnMut(&mut T, ModelHandle<S>, &mut ModelContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_internal(handle, move |observed, cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| {
                    callback(observer, observed, cx);
                });
                true
            } else {
                false
            }
        })
    }

    pub fn observe_global<G, F>(&mut self, mut callback: F) -> Subscription
    where
        G: Any,
        F: 'static + FnMut(&mut T, &mut ModelContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_global::<G, _>(move |cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| callback(observer, cx));
            }
        })
    }

    pub fn observe_release<S, F>(
        &mut self,
        handle: &ModelHandle<S>,
        mut callback: F,
    ) -> Subscription
    where
        S: Entity,
        F: 'static + FnMut(&mut T, &S, &mut ModelContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_release(handle, move |released, cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| {
                    callback(observer, released, cx);
                });
            }
        })
    }

    pub fn handle(&self) -> ModelHandle<T> {
        ModelHandle::new(self.model_id, &self.app.cx.ref_counts)
    }

    pub fn weak_handle(&self) -> WeakModelHandle<T> {
        WeakModelHandle::new(self.model_id)
    }

    pub fn spawn<F, Fut, S>(&mut self, f: F) -> Task<S>
    where
        F: FnOnce(ModelHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.handle();
        self.app.spawn(|cx| f(handle, cx))
    }

    pub fn spawn_weak<F, Fut, S>(&mut self, f: F) -> Task<S>
    where
        F: FnOnce(WeakModelHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.weak_handle();
        self.app.spawn(|cx| f(handle, cx))
    }
}

impl<M> AsRef<AppContext> for ModelContext<'_, M> {
    fn as_ref(&self) -> &AppContext {
        &self.app.cx
    }
}

impl<M> AsMut<MutableAppContext> for ModelContext<'_, M> {
    fn as_mut(&mut self) -> &mut MutableAppContext {
        self.app
    }
}

impl<M> ReadModel for ModelContext<'_, M> {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        self.app.read_model(handle)
    }
}

impl<M> UpdateModel for ModelContext<'_, M> {
    fn update_model<T: Entity, V>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> V,
    ) -> V {
        self.app.update_model(handle, update)
    }
}

impl<M> UpgradeModelHandle for ModelContext<'_, M> {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        self.cx.upgrade_model_handle(handle)
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.cx.model_handle_is_upgradable(handle)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        self.cx.upgrade_any_model_handle(handle)
    }
}

impl<M> Deref for ModelContext<'_, M> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<M> DerefMut for ModelContext<'_, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

pub struct ViewContext<'a, T: ?Sized> {
    app: &'a mut MutableAppContext,
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
}

impl<'a, T: View> ViewContext<'a, T> {
    fn new(app: &'a mut MutableAppContext, window_id: usize, view_id: usize) -> Self {
        Self {
            app,
            window_id,
            view_id,
            view_type: PhantomData,
        }
    }

    pub fn handle(&self) -> ViewHandle<T> {
        ViewHandle::new(self.window_id, self.view_id, &self.app.cx.ref_counts)
    }

    pub fn weak_handle(&self) -> WeakViewHandle<T> {
        WeakViewHandle::new(self.window_id, self.view_id)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn view_id(&self) -> usize {
        self.view_id
    }

    pub fn foreground(&self) -> &Rc<executor::Foreground> {
        self.app.foreground()
    }

    pub fn background_executor(&self) -> &Arc<executor::Background> {
        &self.app.cx.background
    }

    pub fn platform(&self) -> Arc<dyn Platform> {
        self.app.platform()
    }

    pub fn show_character_palette(&self) {
        self.app.show_character_palette(self.window_id);
    }

    pub fn minimize_window(&self) {
        self.app.minimize_window(self.window_id)
    }

    pub fn zoom_window(&self) {
        self.app.zoom_window(self.window_id)
    }

    pub fn toggle_full_screen(&self) {
        self.app.toggle_window_full_screen(self.window_id)
    }

    pub fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        self.app.prompt(self.window_id, level, msg, answers)
    }

    pub fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        self.app.prompt_for_paths(options)
    }

    pub fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        self.app.prompt_for_new_path(directory)
    }

    pub fn reveal_path(&self, path: &Path) {
        self.app.reveal_path(path)
    }

    pub fn debug_elements(&self) -> crate::json::Value {
        self.app.debug_elements(self.window_id).unwrap()
    }

    pub fn focus<S>(&mut self, handle: S)
    where
        S: Into<AnyViewHandle>,
    {
        let handle = handle.into();
        self.app.focus(handle.window_id, Some(handle.view_id));
    }

    pub fn focus_self(&mut self) {
        self.app.focus(self.window_id, Some(self.view_id));
    }

    pub fn is_self_focused(&self) -> bool {
        self.app.focused_view_id(self.window_id) == Some(self.view_id)
    }

    pub fn is_child(&self, view: impl Into<AnyViewHandle>) -> bool {
        let view = view.into();
        if self.window_id != view.window_id {
            return false;
        }
        self.ancestors(view.window_id, view.view_id)
            .skip(1) // Skip self id
            .any(|parent| parent == self.view_id)
    }

    pub fn blur(&mut self) {
        self.app.focus(self.window_id, None);
    }

    pub fn set_window_title(&mut self, title: &str) {
        let window_id = self.window_id();
        if let Some((_, window)) = self.presenters_and_platform_windows.get_mut(&window_id) {
            window.set_title(title);
        }
    }

    pub fn set_window_edited(&mut self, edited: bool) {
        let window_id = self.window_id();
        if let Some((_, window)) = self.presenters_and_platform_windows.get_mut(&window_id) {
            window.set_edited(edited);
        }
    }

    pub fn on_window_should_close<F>(&mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut T, &mut ViewContext<T>) -> bool,
    {
        let window_id = self.window_id();
        let view = self.weak_handle();
        self.pending_effects
            .push_back(Effect::WindowShouldCloseSubscription {
                window_id,
                callback: Box::new(move |cx| {
                    if let Some(view) = view.upgrade(cx) {
                        view.update(cx, |view, cx| callback(view, cx))
                    } else {
                        true
                    }
                }),
            });
    }

    pub fn add_model<S, F>(&mut self, build_model: F) -> ModelHandle<S>
    where
        S: Entity,
        F: FnOnce(&mut ModelContext<S>) -> S,
    {
        self.app.add_model(build_model)
    }

    pub fn add_view<S, F>(&mut self, build_view: F) -> ViewHandle<S>
    where
        S: View,
        F: FnOnce(&mut ViewContext<S>) -> S,
    {
        self.app
            .build_and_insert_view(self.window_id, ParentId::View(self.view_id), |cx| {
                Some(build_view(cx))
            })
            .unwrap()
    }

    pub fn add_option_view<S, F>(&mut self, build_view: F) -> Option<ViewHandle<S>>
    where
        S: View,
        F: FnOnce(&mut ViewContext<S>) -> Option<S>,
    {
        self.app
            .build_and_insert_view(self.window_id, ParentId::View(self.view_id), build_view)
    }

    pub fn parent(&mut self) -> Option<usize> {
        self.cx.parent(self.window_id, self.view_id)
    }

    pub fn reparent(&mut self, view_handle: impl Into<AnyViewHandle>) {
        let view_handle = view_handle.into();
        if self.window_id != view_handle.window_id {
            panic!("Can't reparent view to a view from a different window");
        }
        self.cx
            .parents
            .remove(&(view_handle.window_id, view_handle.view_id));
        let new_parent_id = self.view_id;
        self.cx.parents.insert(
            (view_handle.window_id, view_handle.view_id),
            ParentId::View(new_parent_id),
        );
    }

    pub fn replace_root_view<V, F>(&mut self, build_root_view: F) -> ViewHandle<V>
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        let window_id = self.window_id;
        self.update(|this| {
            let root_view = this
                .build_and_insert_view(window_id, ParentId::Root, |cx| Some(build_root_view(cx)))
                .unwrap();
            let window = this.cx.windows.get_mut(&window_id).unwrap();
            window.root_view = root_view.clone().into();
            window.focused_view_id = Some(root_view.id());
            root_view
        })
    }

    pub fn subscribe<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(&mut T, H, &E::Event, &mut ViewContext<T>),
    {
        let subscriber = self.weak_handle();
        self.app
            .subscribe_internal(handle, move |emitter, event, cx| {
                if let Some(subscriber) = subscriber.upgrade(cx) {
                    subscriber.update(cx, |subscriber, cx| {
                        callback(subscriber, emitter, event, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe<E, F, H>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        H: Handle<E>,
        F: 'static + FnMut(&mut T, H, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_internal(handle, move |observed, cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| {
                    callback(observer, observed, cx);
                });
                true
            } else {
                false
            }
        })
    }

    pub fn observe_focus<F, V>(&mut self, handle: &ViewHandle<V>, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut T, ViewHandle<V>, bool, &mut ViewContext<T>),
        V: View,
    {
        let observer = self.weak_handle();
        self.app
            .observe_focus(handle, move |observed, focused, cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, observed, focused, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe_release<E, F, H>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        H: Handle<E>,
        F: 'static + FnMut(&mut T, &E, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_release(handle, move |released, cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| {
                    callback(observer, released, cx);
                });
            }
        })
    }

    pub fn observe_actions<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut T, TypeId, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_actions(move |action_id, cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| {
                    callback(observer, action_id, cx);
                });
            }
        })
    }

    pub fn observe_window_activation<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut T, bool, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app
            .observe_window_activation(self.window_id(), move |active, cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, active, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe_fullscreen<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut T, bool, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app
            .observe_fullscreen(self.window_id(), move |active, cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, active, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe_keystrokes<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static
            + FnMut(
                &mut T,
                &Keystroke,
                Option<&Box<dyn Action>>,
                &MatchResult,
                &mut ViewContext<T>,
            ) -> bool,
    {
        let observer = self.weak_handle();
        self.app.observe_keystrokes(
            self.window_id(),
            move |keystroke, result, handled_by, cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, keystroke, handled_by, result, cx);
                    });
                    true
                } else {
                    false
                }
            },
        )
    }

    pub fn observe_window_bounds<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut T, WindowBounds, Uuid, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app
            .observe_window_bounds(self.window_id(), move |bounds, display, cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, bounds, display, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe_active_labeled_tasks<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut T, &mut ViewContext<T>),
    {
        let observer = self.weak_handle();
        self.app.observe_active_labeled_tasks(move |cx| {
            if let Some(observer) = observer.upgrade(cx) {
                observer.update(cx, |observer, cx| {
                    callback(observer, cx);
                });
                true
            } else {
                false
            }
        })
    }

    pub fn emit(&mut self, payload: T::Event) {
        self.app.pending_effects.push_back(Effect::Event {
            entity_id: self.view_id,
            payload: Box::new(payload),
        });
    }

    pub fn notify(&mut self) {
        self.app.notify_view(self.window_id, self.view_id);
    }

    pub fn dispatch_action(&mut self, action: impl Action) {
        self.app
            .dispatch_action_at(self.window_id, self.view_id, action)
    }

    pub fn dispatch_any_action(&mut self, action: Box<dyn Action>) {
        self.app
            .dispatch_any_action_at(self.window_id, self.view_id, action)
    }

    pub fn defer(&mut self, callback: impl 'static + FnOnce(&mut T, &mut ViewContext<T>)) {
        let handle = self.handle();
        self.app.defer(move |cx| {
            handle.update(cx, |view, cx| {
                callback(view, cx);
            })
        })
    }

    pub fn after_window_update(
        &mut self,
        callback: impl 'static + FnOnce(&mut T, &mut ViewContext<T>),
    ) {
        let handle = self.handle();
        self.app.after_window_update(move |cx| {
            handle.update(cx, |view, cx| {
                callback(view, cx);
            })
        })
    }

    pub fn propagate_action(&mut self) {
        self.app.halt_action_dispatch = false;
    }

    pub fn spawn_labeled<F, Fut, S>(&mut self, task_label: &'static str, f: F) -> Task<S>
    where
        F: FnOnce(ViewHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.handle();
        self.app.spawn_labeled(task_label, |cx| f(handle, cx))
    }

    pub fn spawn<F, Fut, S>(&mut self, f: F) -> Task<S>
    where
        F: FnOnce(ViewHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.handle();
        self.app.spawn(|cx| f(handle, cx))
    }

    pub fn spawn_weak<F, Fut, S>(&mut self, f: F) -> Task<S>
    where
        F: FnOnce(WeakViewHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.weak_handle();
        self.app.spawn(|cx| f(handle, cx))
    }
}

pub struct RenderParams {
    pub window_id: usize,
    pub view_id: usize,
    pub titlebar_height: f32,
    pub hovered_region_ids: HashSet<MouseRegionId>,
    pub clicked_region_ids: Option<(HashSet<MouseRegionId>, MouseButton)>,
    pub refreshing: bool,
    pub appearance: Appearance,
}

pub struct RenderContext<'a, T: View> {
    pub(crate) window_id: usize,
    pub(crate) view_id: usize,
    pub(crate) view_type: PhantomData<T>,
    pub(crate) hovered_region_ids: HashSet<MouseRegionId>,
    pub(crate) clicked_region_ids: Option<(HashSet<MouseRegionId>, MouseButton)>,
    pub app: &'a mut MutableAppContext,
    pub titlebar_height: f32,
    pub appearance: Appearance,
    pub refreshing: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MouseState {
    pub(crate) hovered: bool,
    pub(crate) clicked: Option<MouseButton>,
    pub(crate) accessed_hovered: bool,
    pub(crate) accessed_clicked: bool,
}

impl MouseState {
    pub fn hovered(&mut self) -> bool {
        self.accessed_hovered = true;
        self.hovered
    }

    pub fn clicked(&mut self) -> Option<MouseButton> {
        self.accessed_clicked = true;
        self.clicked
    }

    pub fn accessed_hovered(&self) -> bool {
        self.accessed_hovered
    }

    pub fn accessed_clicked(&self) -> bool {
        self.accessed_clicked
    }
}

impl<'a, V: View> RenderContext<'a, V> {
    fn new(params: RenderParams, app: &'a mut MutableAppContext) -> Self {
        Self {
            app,
            window_id: params.window_id,
            view_id: params.view_id,
            view_type: PhantomData,
            titlebar_height: params.titlebar_height,
            hovered_region_ids: params.hovered_region_ids.clone(),
            clicked_region_ids: params.clicked_region_ids.clone(),
            refreshing: params.refreshing,
            appearance: params.appearance,
        }
    }

    pub fn handle(&self) -> WeakViewHandle<V> {
        WeakViewHandle::new(self.window_id, self.view_id)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn view_id(&self) -> usize {
        self.view_id
    }

    pub fn mouse_state<Tag: 'static>(&self, region_id: usize) -> MouseState {
        let region_id = MouseRegionId::new::<Tag>(self.view_id, region_id);
        MouseState {
            hovered: self.hovered_region_ids.contains(&region_id),
            clicked: self.clicked_region_ids.as_ref().and_then(|(ids, button)| {
                if ids.contains(&region_id) {
                    Some(*button)
                } else {
                    None
                }
            }),
            accessed_hovered: false,
            accessed_clicked: false,
        }
    }

    pub fn element_state<Tag: 'static, T: 'static>(
        &mut self,
        element_id: usize,
        initial: T,
    ) -> ElementStateHandle<T> {
        let id = ElementStateId {
            view_id: self.view_id(),
            element_id,
            tag: TypeId::of::<Tag>(),
        };
        self.cx
            .element_states
            .entry(id)
            .or_insert_with(|| Box::new(initial));
        ElementStateHandle::new(id, self.frame_count, &self.cx.ref_counts)
    }

    pub fn default_element_state<Tag: 'static, T: 'static + Default>(
        &mut self,
        element_id: usize,
    ) -> ElementStateHandle<T> {
        self.element_state::<Tag, T>(element_id, T::default())
    }
}

impl AsRef<AppContext> for &AppContext {
    fn as_ref(&self) -> &AppContext {
        self
    }
}

impl<V: View> Deref for RenderContext<'_, V> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<V: View> DerefMut for RenderContext<'_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.app
    }
}

impl<V: View> ReadModel for RenderContext<'_, V> {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        self.app.read_model(handle)
    }
}

impl<V: View> UpdateModel for RenderContext<'_, V> {
    fn update_model<T: Entity, O>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> O,
    ) -> O {
        self.app.update_model(handle, update)
    }
}

impl<V: View> ReadView for RenderContext<'_, V> {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        self.app.read_view(handle)
    }
}

impl<M> AsRef<AppContext> for ViewContext<'_, M> {
    fn as_ref(&self) -> &AppContext {
        &self.app.cx
    }
}

impl<M> Deref for ViewContext<'_, M> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<M> DerefMut for ViewContext<'_, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl<M> AsMut<MutableAppContext> for ViewContext<'_, M> {
    fn as_mut(&mut self) -> &mut MutableAppContext {
        self.app
    }
}

impl<V> ReadModel for ViewContext<'_, V> {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        self.app.read_model(handle)
    }
}

impl<V> UpgradeModelHandle for ViewContext<'_, V> {
    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        self.cx.upgrade_model_handle(handle)
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.cx.model_handle_is_upgradable(handle)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        self.cx.upgrade_any_model_handle(handle)
    }
}

impl<V> UpgradeViewHandle for ViewContext<'_, V> {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
        self.cx.upgrade_view_handle(handle)
    }

    fn upgrade_any_view_handle(&self, handle: &AnyWeakViewHandle) -> Option<AnyViewHandle> {
        self.cx.upgrade_any_view_handle(handle)
    }
}

impl<V: View> UpgradeViewHandle for RenderContext<'_, V> {
    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
        self.cx.upgrade_view_handle(handle)
    }

    fn upgrade_any_view_handle(&self, handle: &AnyWeakViewHandle) -> Option<AnyViewHandle> {
        self.cx.upgrade_any_view_handle(handle)
    }
}

impl<V: View> UpdateModel for ViewContext<'_, V> {
    fn update_model<T: Entity, O>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> O,
    ) -> O {
        self.app.update_model(handle, update)
    }
}

impl<V: View> ReadView for ViewContext<'_, V> {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        self.app.read_view(handle)
    }
}

impl<V: View> UpdateView for ViewContext<'_, V> {
    fn update_view<T, S>(
        &mut self,
        handle: &ViewHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
    ) -> S
    where
        T: View,
    {
        self.app.update_view(handle, update)
    }
}

pub trait Handle<T> {
    type Weak: 'static;
    fn id(&self) -> usize;
    fn location(&self) -> EntityLocation;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak, cx: &AppContext) -> Option<Self>
    where
        Self: Sized;
}

pub trait WeakHandle {
    fn id(&self) -> usize;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EntityLocation {
    Model(usize),
    View(usize, usize),
}

pub struct ModelHandle<T: Entity> {
    model_id: usize,
    model_type: PhantomData<T>,
    ref_counts: Arc<Mutex<RefCounts>>,

    #[cfg(any(test, feature = "test-support"))]
    handle_id: usize,
}

impl<T: Entity> ModelHandle<T> {
    fn new(model_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_model(model_id);

        #[cfg(any(test, feature = "test-support"))]
        let handle_id = ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_created(Some(type_name::<T>()), model_id);

        Self {
            model_id,
            model_type: PhantomData,
            ref_counts: ref_counts.clone(),

            #[cfg(any(test, feature = "test-support"))]
            handle_id,
        }
    }

    pub fn downgrade(&self) -> WeakModelHandle<T> {
        WeakModelHandle::new(self.model_id)
    }

    pub fn id(&self) -> usize {
        self.model_id
    }

    pub fn read<'a, C: ReadModel>(&self, cx: &'a C) -> &'a T {
        cx.read_model(self)
    }

    pub fn read_with<C, F, S>(&self, cx: &C, read: F) -> S
    where
        C: ReadModelWith,
        F: FnOnce(&T, &AppContext) -> S,
    {
        let mut read = Some(read);
        cx.read_model_with(self, &mut |model, cx| {
            let read = read.take().unwrap();
            read(model, cx)
        })
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> S
    where
        C: UpdateModel,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        let mut update = Some(update);
        cx.update_model(self, &mut |model, cx| {
            let update = update.take().unwrap();
            update(model, cx)
        })
    }
}

impl<T: Entity> Clone for ModelHandle<T> {
    fn clone(&self) -> Self {
        Self::new(self.model_id, &self.ref_counts)
    }
}

impl<T: Entity> PartialEq for ModelHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.model_id == other.model_id
    }
}

impl<T: Entity> Eq for ModelHandle<T> {}

impl<T: Entity> PartialEq<WeakModelHandle<T>> for ModelHandle<T> {
    fn eq(&self, other: &WeakModelHandle<T>) -> bool {
        self.model_id == other.model_id
    }
}

impl<T: Entity> Hash for ModelHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.model_id.hash(state);
    }
}

impl<T: Entity> std::borrow::Borrow<usize> for ModelHandle<T> {
    fn borrow(&self) -> &usize {
        &self.model_id
    }
}

impl<T: Entity> Debug for ModelHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("ModelHandle<{}>", type_name::<T>()))
            .field(&self.model_id)
            .finish()
    }
}

unsafe impl<T: Entity> Send for ModelHandle<T> {}
unsafe impl<T: Entity> Sync for ModelHandle<T> {}

impl<T: Entity> Drop for ModelHandle<T> {
    fn drop(&mut self) {
        let mut ref_counts = self.ref_counts.lock();
        ref_counts.dec_model(self.model_id);

        #[cfg(any(test, feature = "test-support"))]
        ref_counts
            .leak_detector
            .lock()
            .handle_dropped(self.model_id, self.handle_id);
    }
}

impl<T: Entity> Handle<T> for ModelHandle<T> {
    type Weak = WeakModelHandle<T>;

    fn id(&self) -> usize {
        self.model_id
    }

    fn location(&self) -> EntityLocation {
        EntityLocation::Model(self.model_id)
    }

    fn downgrade(&self) -> Self::Weak {
        self.downgrade()
    }

    fn upgrade_from(weak: &Self::Weak, cx: &AppContext) -> Option<Self>
    where
        Self: Sized,
    {
        weak.upgrade(cx)
    }
}

pub struct WeakModelHandle<T> {
    model_id: usize,
    model_type: PhantomData<T>,
}

impl<T> WeakHandle for WeakModelHandle<T> {
    fn id(&self) -> usize {
        self.model_id
    }
}

unsafe impl<T> Send for WeakModelHandle<T> {}
unsafe impl<T> Sync for WeakModelHandle<T> {}

impl<T: Entity> WeakModelHandle<T> {
    fn new(model_id: usize) -> Self {
        Self {
            model_id,
            model_type: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.model_id
    }

    pub fn is_upgradable(&self, cx: &impl UpgradeModelHandle) -> bool {
        cx.model_handle_is_upgradable(self)
    }

    pub fn upgrade(&self, cx: &impl UpgradeModelHandle) -> Option<ModelHandle<T>> {
        cx.upgrade_model_handle(self)
    }
}

impl<T> Hash for WeakModelHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.model_id.hash(state)
    }
}

impl<T> PartialEq for WeakModelHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.model_id == other.model_id
    }
}

impl<T> Eq for WeakModelHandle<T> {}

impl<T: Entity> PartialEq<ModelHandle<T>> for WeakModelHandle<T> {
    fn eq(&self, other: &ModelHandle<T>) -> bool {
        self.model_id == other.model_id
    }
}

impl<T> Clone for WeakModelHandle<T> {
    fn clone(&self) -> Self {
        Self {
            model_id: self.model_id,
            model_type: PhantomData,
        }
    }
}

impl<T> Copy for WeakModelHandle<T> {}

pub struct ViewHandle<T> {
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
    ref_counts: Arc<Mutex<RefCounts>>,
    #[cfg(any(test, feature = "test-support"))]
    handle_id: usize,
}

impl<T: View> ViewHandle<T> {
    fn new(window_id: usize, view_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_view(window_id, view_id);
        #[cfg(any(test, feature = "test-support"))]
        let handle_id = ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_created(Some(type_name::<T>()), view_id);

        Self {
            window_id,
            view_id,
            view_type: PhantomData,
            ref_counts: ref_counts.clone(),

            #[cfg(any(test, feature = "test-support"))]
            handle_id,
        }
    }

    pub fn downgrade(&self) -> WeakViewHandle<T> {
        WeakViewHandle::new(self.window_id, self.view_id)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn read<'a, C: ReadView>(&self, cx: &'a C) -> &'a T {
        cx.read_view(self)
    }

    pub fn read_with<C, F, S>(&self, cx: &C, read: F) -> S
    where
        C: ReadViewWith,
        F: FnOnce(&T, &AppContext) -> S,
    {
        let mut read = Some(read);
        cx.read_view_with(self, &mut |view, cx| {
            let read = read.take().unwrap();
            read(view, cx)
        })
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> S
    where
        C: UpdateView,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        let mut update = Some(update);
        cx.update_view(self, &mut |view, cx| {
            let update = update.take().unwrap();
            update(view, cx)
        })
    }

    pub fn defer<C, F>(&self, cx: &mut C, update: F)
    where
        C: AsMut<MutableAppContext>,
        F: 'static + FnOnce(&mut T, &mut ViewContext<T>),
    {
        let this = self.clone();
        cx.as_mut().defer(move |cx| {
            this.update(cx, |view, cx| update(view, cx));
        });
    }

    pub fn is_focused(&self, cx: &AppContext) -> bool {
        cx.focused_view_id(self.window_id)
            .map_or(false, |focused_id| focused_id == self.view_id)
    }
}

impl<T: View> Clone for ViewHandle<T> {
    fn clone(&self) -> Self {
        ViewHandle::new(self.window_id, self.view_id, &self.ref_counts)
    }
}

impl<T> PartialEq for ViewHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.window_id == other.window_id && self.view_id == other.view_id
    }
}

impl<T> PartialEq<WeakViewHandle<T>> for ViewHandle<T> {
    fn eq(&self, other: &WeakViewHandle<T>) -> bool {
        self.window_id == other.window_id && self.view_id == other.view_id
    }
}

impl<T> PartialEq<ViewHandle<T>> for WeakViewHandle<T> {
    fn eq(&self, other: &ViewHandle<T>) -> bool {
        self.window_id == other.window_id && self.view_id == other.view_id
    }
}

impl<T> Eq for ViewHandle<T> {}

impl<T> Hash for ViewHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.window_id.hash(state);
        self.view_id.hash(state);
    }
}

impl<T> Debug for ViewHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("ViewHandle<{}>", type_name::<T>()))
            .field("window_id", &self.window_id)
            .field("view_id", &self.view_id)
            .finish()
    }
}

impl<T> Drop for ViewHandle<T> {
    fn drop(&mut self) {
        self.ref_counts
            .lock()
            .dec_view(self.window_id, self.view_id);
        #[cfg(any(test, feature = "test-support"))]
        self.ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_dropped(self.view_id, self.handle_id);
    }
}

impl<T: View> Handle<T> for ViewHandle<T> {
    type Weak = WeakViewHandle<T>;

    fn id(&self) -> usize {
        self.view_id
    }

    fn location(&self) -> EntityLocation {
        EntityLocation::View(self.window_id, self.view_id)
    }

    fn downgrade(&self) -> Self::Weak {
        self.downgrade()
    }

    fn upgrade_from(weak: &Self::Weak, cx: &AppContext) -> Option<Self>
    where
        Self: Sized,
    {
        weak.upgrade(cx)
    }
}

pub struct AnyViewHandle {
    window_id: usize,
    view_id: usize,
    view_type: TypeId,
    ref_counts: Arc<Mutex<RefCounts>>,

    #[cfg(any(test, feature = "test-support"))]
    handle_id: usize,
}

impl AnyViewHandle {
    fn new(
        window_id: usize,
        view_id: usize,
        view_type: TypeId,
        ref_counts: Arc<Mutex<RefCounts>>,
    ) -> Self {
        ref_counts.lock().inc_view(window_id, view_id);

        #[cfg(any(test, feature = "test-support"))]
        let handle_id = ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_created(None, view_id);

        Self {
            window_id,
            view_id,
            view_type,
            ref_counts,
            #[cfg(any(test, feature = "test-support"))]
            handle_id,
        }
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.view_type
    }

    pub fn is_focused(&self, cx: &AppContext) -> bool {
        cx.focused_view_id(self.window_id)
            .map_or(false, |focused_id| focused_id == self.view_id)
    }

    pub fn downcast<T: View>(self) -> Option<ViewHandle<T>> {
        if self.is::<T>() {
            let result = Some(ViewHandle {
                window_id: self.window_id,
                view_id: self.view_id,
                ref_counts: self.ref_counts.clone(),
                view_type: PhantomData,
                #[cfg(any(test, feature = "test-support"))]
                handle_id: self.handle_id,
            });
            unsafe {
                Arc::decrement_strong_count(Arc::as_ptr(&self.ref_counts));
            }
            std::mem::forget(self);
            result
        } else {
            None
        }
    }

    pub fn downgrade(&self) -> AnyWeakViewHandle {
        AnyWeakViewHandle {
            window_id: self.window_id,
            view_id: self.view_id,
            view_type: self.view_type,
        }
    }

    pub fn view_type(&self) -> TypeId {
        self.view_type
    }

    pub fn debug_json(&self, cx: &AppContext) -> serde_json::Value {
        cx.views
            .get(&(self.window_id, self.view_id))
            .map_or_else(|| serde_json::Value::Null, |view| view.debug_json(cx))
    }
}

impl Clone for AnyViewHandle {
    fn clone(&self) -> Self {
        Self::new(
            self.window_id,
            self.view_id,
            self.view_type,
            self.ref_counts.clone(),
        )
    }
}

impl From<&AnyViewHandle> for AnyViewHandle {
    fn from(handle: &AnyViewHandle) -> Self {
        handle.clone()
    }
}

impl<T: View> From<&ViewHandle<T>> for AnyViewHandle {
    fn from(handle: &ViewHandle<T>) -> Self {
        Self::new(
            handle.window_id,
            handle.view_id,
            TypeId::of::<T>(),
            handle.ref_counts.clone(),
        )
    }
}

impl<T: View> From<ViewHandle<T>> for AnyViewHandle {
    fn from(handle: ViewHandle<T>) -> Self {
        let any_handle = AnyViewHandle {
            window_id: handle.window_id,
            view_id: handle.view_id,
            view_type: TypeId::of::<T>(),
            ref_counts: handle.ref_counts.clone(),
            #[cfg(any(test, feature = "test-support"))]
            handle_id: handle.handle_id,
        };

        unsafe {
            Arc::decrement_strong_count(Arc::as_ptr(&handle.ref_counts));
        }
        std::mem::forget(handle);
        any_handle
    }
}

impl<T> PartialEq<ViewHandle<T>> for AnyViewHandle {
    fn eq(&self, other: &ViewHandle<T>) -> bool {
        self.window_id == other.window_id && self.view_id == other.view_id
    }
}

impl Drop for AnyViewHandle {
    fn drop(&mut self) {
        self.ref_counts
            .lock()
            .dec_view(self.window_id, self.view_id);
        #[cfg(any(test, feature = "test-support"))]
        self.ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_dropped(self.view_id, self.handle_id);
    }
}

pub struct AnyModelHandle {
    model_id: usize,
    model_type: TypeId,
    ref_counts: Arc<Mutex<RefCounts>>,

    #[cfg(any(test, feature = "test-support"))]
    handle_id: usize,
}

impl AnyModelHandle {
    fn new(model_id: usize, model_type: TypeId, ref_counts: Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_model(model_id);

        #[cfg(any(test, feature = "test-support"))]
        let handle_id = ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_created(None, model_id);

        Self {
            model_id,
            model_type,
            ref_counts,

            #[cfg(any(test, feature = "test-support"))]
            handle_id,
        }
    }

    pub fn downcast<T: Entity>(self) -> Option<ModelHandle<T>> {
        if self.is::<T>() {
            let result = Some(ModelHandle {
                model_id: self.model_id,
                model_type: PhantomData,
                ref_counts: self.ref_counts.clone(),

                #[cfg(any(test, feature = "test-support"))]
                handle_id: self.handle_id,
            });
            unsafe {
                Arc::decrement_strong_count(Arc::as_ptr(&self.ref_counts));
            }
            std::mem::forget(self);
            result
        } else {
            None
        }
    }

    pub fn downgrade(&self) -> AnyWeakModelHandle {
        AnyWeakModelHandle {
            model_id: self.model_id,
            model_type: self.model_type,
        }
    }

    pub fn is<T: Entity>(&self) -> bool {
        self.model_type == TypeId::of::<T>()
    }

    pub fn model_type(&self) -> TypeId {
        self.model_type
    }
}

impl<T: Entity> From<ModelHandle<T>> for AnyModelHandle {
    fn from(handle: ModelHandle<T>) -> Self {
        Self::new(
            handle.model_id,
            TypeId::of::<T>(),
            handle.ref_counts.clone(),
        )
    }
}

impl Clone for AnyModelHandle {
    fn clone(&self) -> Self {
        Self::new(self.model_id, self.model_type, self.ref_counts.clone())
    }
}

impl Drop for AnyModelHandle {
    fn drop(&mut self) {
        let mut ref_counts = self.ref_counts.lock();
        ref_counts.dec_model(self.model_id);

        #[cfg(any(test, feature = "test-support"))]
        ref_counts
            .leak_detector
            .lock()
            .handle_dropped(self.model_id, self.handle_id);
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct AnyWeakModelHandle {
    model_id: usize,
    model_type: TypeId,
}

impl AnyWeakModelHandle {
    pub fn upgrade(&self, cx: &impl UpgradeModelHandle) -> Option<AnyModelHandle> {
        cx.upgrade_any_model_handle(self)
    }
    pub fn model_type(&self) -> TypeId {
        self.model_type
    }

    fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.model_type
    }

    pub fn downcast<T: Entity>(&self) -> Option<WeakModelHandle<T>> {
        if self.is::<T>() {
            let result = Some(WeakModelHandle {
                model_id: self.model_id,
                model_type: PhantomData,
            });

            result
        } else {
            None
        }
    }
}

impl<T: Entity> From<WeakModelHandle<T>> for AnyWeakModelHandle {
    fn from(handle: WeakModelHandle<T>) -> Self {
        AnyWeakModelHandle {
            model_id: handle.model_id,
            model_type: TypeId::of::<T>(),
        }
    }
}

#[derive(Debug)]
pub struct WeakViewHandle<T> {
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
}

impl<T> WeakHandle for WeakViewHandle<T> {
    fn id(&self) -> usize {
        self.view_id
    }
}

impl<T: View> WeakViewHandle<T> {
    fn new(window_id: usize, view_id: usize) -> Self {
        Self {
            window_id,
            view_id,
            view_type: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn upgrade(&self, cx: &impl UpgradeViewHandle) -> Option<ViewHandle<T>> {
        cx.upgrade_view_handle(self)
    }
}

impl<T> Clone for WeakViewHandle<T> {
    fn clone(&self) -> Self {
        Self {
            window_id: self.window_id,
            view_id: self.view_id,
            view_type: PhantomData,
        }
    }
}

impl<T> PartialEq for WeakViewHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.window_id == other.window_id && self.view_id == other.view_id
    }
}

impl<T> Eq for WeakViewHandle<T> {}

impl<T> Hash for WeakViewHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.window_id.hash(state);
        self.view_id.hash(state);
    }
}

pub struct AnyWeakViewHandle {
    window_id: usize,
    view_id: usize,
    view_type: TypeId,
}

impl AnyWeakViewHandle {
    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn upgrade(&self, cx: &impl UpgradeViewHandle) -> Option<AnyViewHandle> {
        cx.upgrade_any_view_handle(self)
    }
}

impl<T: View> From<WeakViewHandle<T>> for AnyWeakViewHandle {
    fn from(handle: WeakViewHandle<T>) -> Self {
        AnyWeakViewHandle {
            window_id: handle.window_id,
            view_id: handle.view_id,
            view_type: TypeId::of::<T>(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ElementStateId {
    view_id: usize,
    element_id: usize,
    tag: TypeId,
}

pub struct ElementStateHandle<T> {
    value_type: PhantomData<T>,
    id: ElementStateId,
    ref_counts: Weak<Mutex<RefCounts>>,
}

impl<T: 'static> ElementStateHandle<T> {
    fn new(id: ElementStateId, frame_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_element_state(id, frame_id);
        Self {
            value_type: PhantomData,
            id,
            ref_counts: Arc::downgrade(ref_counts),
        }
    }

    pub fn id(&self) -> ElementStateId {
        self.id
    }

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a T {
        cx.element_states
            .get(&self.id)
            .unwrap()
            .downcast_ref()
            .unwrap()
    }

    pub fn update<C, R>(&self, cx: &mut C, f: impl FnOnce(&mut T, &mut C) -> R) -> R
    where
        C: DerefMut<Target = MutableAppContext>,
    {
        let mut element_state = cx.deref_mut().cx.element_states.remove(&self.id).unwrap();
        let result = f(element_state.downcast_mut().unwrap(), cx);
        cx.deref_mut()
            .cx
            .element_states
            .insert(self.id, element_state);
        result
    }
}

impl<T> Drop for ElementStateHandle<T> {
    fn drop(&mut self) {
        if let Some(ref_counts) = self.ref_counts.upgrade() {
            ref_counts.lock().dec_element_state(self.id);
        }
    }
}

#[must_use]
pub enum Subscription {
    Subscription(callback_collection::Subscription<usize, SubscriptionCallback>),
    Observation(callback_collection::Subscription<usize, ObservationCallback>),
    GlobalSubscription(callback_collection::Subscription<TypeId, GlobalSubscriptionCallback>),
    GlobalObservation(callback_collection::Subscription<TypeId, GlobalObservationCallback>),
    FocusObservation(callback_collection::Subscription<usize, FocusObservationCallback>),
    WindowActivationObservation(callback_collection::Subscription<usize, WindowActivationCallback>),
    WindowFullscreenObservation(callback_collection::Subscription<usize, WindowFullscreenCallback>),
    WindowBoundsObservation(callback_collection::Subscription<usize, WindowBoundsCallback>),
    KeystrokeObservation(callback_collection::Subscription<usize, KeystrokeCallback>),
    ReleaseObservation(callback_collection::Subscription<usize, ReleaseObservationCallback>),
    ActionObservation(callback_collection::Subscription<(), ActionObservationCallback>),
    ActiveLabeledTasksObservation(
        callback_collection::Subscription<(), ActiveLabeledTasksCallback>,
    ),
}

impl Subscription {
    pub fn id(&self) -> usize {
        match self {
            Subscription::Subscription(subscription) => subscription.id(),
            Subscription::Observation(subscription) => subscription.id(),
            Subscription::GlobalSubscription(subscription) => subscription.id(),
            Subscription::GlobalObservation(subscription) => subscription.id(),
            Subscription::FocusObservation(subscription) => subscription.id(),
            Subscription::WindowActivationObservation(subscription) => subscription.id(),
            Subscription::WindowFullscreenObservation(subscription) => subscription.id(),
            Subscription::WindowBoundsObservation(subscription) => subscription.id(),
            Subscription::KeystrokeObservation(subscription) => subscription.id(),
            Subscription::ReleaseObservation(subscription) => subscription.id(),
            Subscription::ActionObservation(subscription) => subscription.id(),
            Subscription::ActiveLabeledTasksObservation(subscription) => subscription.id(),
        }
    }

    pub fn detach(&mut self) {
        match self {
            Subscription::Subscription(subscription) => subscription.detach(),
            Subscription::GlobalSubscription(subscription) => subscription.detach(),
            Subscription::Observation(subscription) => subscription.detach(),
            Subscription::GlobalObservation(subscription) => subscription.detach(),
            Subscription::FocusObservation(subscription) => subscription.detach(),
            Subscription::KeystrokeObservation(subscription) => subscription.detach(),
            Subscription::WindowActivationObservation(subscription) => subscription.detach(),
            Subscription::WindowFullscreenObservation(subscription) => subscription.detach(),
            Subscription::WindowBoundsObservation(subscription) => subscription.detach(),
            Subscription::ReleaseObservation(subscription) => subscription.detach(),
            Subscription::ActionObservation(subscription) => subscription.detach(),
            Subscription::ActiveLabeledTasksObservation(subscription) => subscription.detach(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{actions, elements::*, impl_actions, MouseButton, MouseButtonEvent};
    use itertools::Itertools;
    use postage::{sink::Sink, stream::Stream};
    use serde::Deserialize;
    use smol::future::poll_once;
    use std::{
        cell::Cell,
        sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
    };

    #[crate::test(self)]
    fn test_model_handles(cx: &mut MutableAppContext) {
        struct Model {
            other: Option<ModelHandle<Model>>,
            events: Vec<String>,
        }

        impl Entity for Model {
            type Event = usize;
        }

        impl Model {
            fn new(other: Option<ModelHandle<Self>>, cx: &mut ModelContext<Self>) -> Self {
                if let Some(other) = other.as_ref() {
                    cx.observe(other, |me, _, _| {
                        me.events.push("notified".into());
                    })
                    .detach();
                    cx.subscribe(other, |me, _, event, _| {
                        me.events.push(format!("observed event {}", event));
                    })
                    .detach();
                }

                Self {
                    other,
                    events: Vec::new(),
                }
            }
        }

        let handle_1 = cx.add_model(|cx| Model::new(None, cx));
        let handle_2 = cx.add_model(|cx| Model::new(Some(handle_1.clone()), cx));
        assert_eq!(cx.cx.models.len(), 2);

        handle_1.update(cx, |model, cx| {
            model.events.push("updated".into());
            cx.emit(1);
            cx.notify();
            cx.emit(2);
        });
        assert_eq!(handle_1.read(cx).events, vec!["updated".to_string()]);
        assert_eq!(
            handle_2.read(cx).events,
            vec![
                "observed event 1".to_string(),
                "notified".to_string(),
                "observed event 2".to_string(),
            ]
        );

        handle_2.update(cx, |model, _| {
            drop(handle_1);
            model.other.take();
        });

        assert_eq!(cx.cx.models.len(), 1);
        assert!(cx.subscriptions.is_empty());
        assert!(cx.observations.is_empty());
    }

    #[crate::test(self)]
    fn test_model_events(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct Model {
            events: Vec<usize>,
        }

        impl Entity for Model {
            type Event = usize;
        }

        let handle_1 = cx.add_model(|_| Model::default());
        let handle_2 = cx.add_model(|_| Model::default());

        handle_1.update(cx, |_, cx| {
            cx.subscribe(&handle_2, move |model: &mut Model, emitter, event, cx| {
                model.events.push(*event);

                cx.subscribe(&emitter, |model, _, event, _| {
                    model.events.push(*event * 2);
                })
                .detach();
            })
            .detach();
        });

        handle_2.update(cx, |_, c| c.emit(7));
        assert_eq!(handle_1.read(cx).events, vec![7]);

        handle_2.update(cx, |_, c| c.emit(5));
        assert_eq!(handle_1.read(cx).events, vec![7, 5, 10]);
    }

    #[crate::test(self)]
    fn test_model_emit_before_subscribe_in_same_update_cycle(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let events = Rc::new(RefCell::new(Vec::new()));
        cx.add_model(|cx| {
            drop(cx.subscribe(&cx.handle(), {
                let events = events.clone();
                move |_, _, _, _| events.borrow_mut().push("dropped before flush")
            }));
            cx.subscribe(&cx.handle(), {
                let events = events.clone();
                move |_, _, _, _| events.borrow_mut().push("before emit")
            })
            .detach();
            cx.emit(());
            cx.subscribe(&cx.handle(), {
                let events = events.clone();
                move |_, _, _, _| events.borrow_mut().push("after emit")
            })
            .detach();
            Model
        });
        assert_eq!(*events.borrow(), ["before emit"]);
    }

    #[crate::test(self)]
    fn test_observe_and_notify_from_model(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct Model {
            count: usize,
            events: Vec<usize>,
        }

        impl Entity for Model {
            type Event = ();
        }

        let handle_1 = cx.add_model(|_| Model::default());
        let handle_2 = cx.add_model(|_| Model::default());

        handle_1.update(cx, |_, c| {
            c.observe(&handle_2, move |model, observed, c| {
                model.events.push(observed.read(c).count);
                c.observe(&observed, |model, observed, c| {
                    model.events.push(observed.read(c).count * 2);
                })
                .detach();
            })
            .detach();
        });

        handle_2.update(cx, |model, c| {
            model.count = 7;
            c.notify()
        });
        assert_eq!(handle_1.read(cx).events, vec![7]);

        handle_2.update(cx, |model, c| {
            model.count = 5;
            c.notify()
        });
        assert_eq!(handle_1.read(cx).events, vec![7, 5, 10])
    }

    #[crate::test(self)]
    fn test_model_notify_before_observe_in_same_update_cycle(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let events = Rc::new(RefCell::new(Vec::new()));
        cx.add_model(|cx| {
            drop(cx.observe(&cx.handle(), {
                let events = events.clone();
                move |_, _, _| events.borrow_mut().push("dropped before flush")
            }));
            cx.observe(&cx.handle(), {
                let events = events.clone();
                move |_, _, _| events.borrow_mut().push("before notify")
            })
            .detach();
            cx.notify();
            cx.observe(&cx.handle(), {
                let events = events.clone();
                move |_, _, _| events.borrow_mut().push("after notify")
            })
            .detach();
            Model
        });
        assert_eq!(*events.borrow(), ["before notify"]);
    }

    #[crate::test(self)]
    fn test_defer_and_after_window_update(cx: &mut MutableAppContext) {
        struct View {
            render_count: usize,
        }

        impl Entity for View {
            type Event = usize;
        }

        impl super::View for View {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                post_inc(&mut self.render_count);
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let (_, view) = cx.add_window(Default::default(), |_| View { render_count: 0 });
        let called_defer = Rc::new(AtomicBool::new(false));
        let called_after_window_update = Rc::new(AtomicBool::new(false));

        view.update(cx, |this, cx| {
            assert_eq!(this.render_count, 1);
            cx.defer({
                let called_defer = called_defer.clone();
                move |this, _| {
                    assert_eq!(this.render_count, 1);
                    called_defer.store(true, SeqCst);
                }
            });
            cx.after_window_update({
                let called_after_window_update = called_after_window_update.clone();
                move |this, cx| {
                    assert_eq!(this.render_count, 2);
                    called_after_window_update.store(true, SeqCst);
                    cx.notify();
                }
            });
            assert!(!called_defer.load(SeqCst));
            assert!(!called_after_window_update.load(SeqCst));
            cx.notify();
        });

        assert!(called_defer.load(SeqCst));
        assert!(called_after_window_update.load(SeqCst));
        assert_eq!(view.read(cx).render_count, 3);
    }

    #[crate::test(self)]
    fn test_view_handles(cx: &mut MutableAppContext) {
        struct View {
            other: Option<ViewHandle<View>>,
            events: Vec<String>,
        }

        impl Entity for View {
            type Event = usize;
        }

        impl super::View for View {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        impl View {
            fn new(other: Option<ViewHandle<View>>, cx: &mut ViewContext<Self>) -> Self {
                if let Some(other) = other.as_ref() {
                    cx.subscribe(other, |me, _, event, _| {
                        me.events.push(format!("observed event {}", event));
                    })
                    .detach();
                }
                Self {
                    other,
                    events: Vec::new(),
                }
            }
        }

        let (_, root_view) = cx.add_window(Default::default(), |cx| View::new(None, cx));
        let handle_1 = cx.add_view(&root_view, |cx| View::new(None, cx));
        let handle_2 = cx.add_view(&root_view, |cx| View::new(Some(handle_1.clone()), cx));
        assert_eq!(cx.cx.views.len(), 3);

        handle_1.update(cx, |view, cx| {
            view.events.push("updated".into());
            cx.emit(1);
            cx.emit(2);
        });
        assert_eq!(handle_1.read(cx).events, vec!["updated".to_string()]);
        assert_eq!(
            handle_2.read(cx).events,
            vec![
                "observed event 1".to_string(),
                "observed event 2".to_string(),
            ]
        );

        handle_2.update(cx, |view, _| {
            drop(handle_1);
            view.other.take();
        });

        assert_eq!(cx.cx.views.len(), 2);
        assert!(cx.subscriptions.is_empty());
        assert!(cx.observations.is_empty());
    }

    #[crate::test(self)]
    fn test_add_window(cx: &mut MutableAppContext) {
        struct View {
            mouse_down_count: Arc<AtomicUsize>,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
                enum Handler {}
                let mouse_down_count = self.mouse_down_count.clone();
                MouseEventHandler::<Handler>::new(0, cx, |_, _| Empty::new().boxed())
                    .on_down(MouseButton::Left, move |_, _| {
                        mouse_down_count.fetch_add(1, SeqCst);
                    })
                    .boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let mouse_down_count = Arc::new(AtomicUsize::new(0));
        let (window_id, _) = cx.add_window(Default::default(), |_| View {
            mouse_down_count: mouse_down_count.clone(),
        });
        let presenter = cx.presenters_and_platform_windows[&window_id].0.clone();
        // Ensure window's root element is in a valid lifecycle state.
        presenter.borrow_mut().dispatch_event(
            Event::MouseDown(MouseButtonEvent {
                position: Default::default(),
                button: MouseButton::Left,
                modifiers: Default::default(),
                click_count: 1,
            }),
            false,
            cx,
        );
        assert_eq!(mouse_down_count.load(SeqCst), 1);
    }

    #[crate::test(self)]
    fn test_entity_release_hooks(cx: &mut MutableAppContext) {
        struct Model {
            released: Rc<Cell<bool>>,
        }

        struct View {
            released: Rc<Cell<bool>>,
        }

        impl Entity for Model {
            type Event = ();

            fn release(&mut self, _: &mut MutableAppContext) {
                self.released.set(true);
            }
        }

        impl Entity for View {
            type Event = ();

            fn release(&mut self, _: &mut MutableAppContext) {
                self.released.set(true);
            }
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "View"
            }

            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }
        }

        let model_released = Rc::new(Cell::new(false));
        let model_release_observed = Rc::new(Cell::new(false));
        let view_released = Rc::new(Cell::new(false));
        let view_release_observed = Rc::new(Cell::new(false));

        let model = cx.add_model(|_| Model {
            released: model_released.clone(),
        });
        let (window_id, view) = cx.add_window(Default::default(), |_| View {
            released: view_released.clone(),
        });
        assert!(!model_released.get());
        assert!(!view_released.get());

        cx.observe_release(&model, {
            let model_release_observed = model_release_observed.clone();
            move |_, _| model_release_observed.set(true)
        })
        .detach();
        cx.observe_release(&view, {
            let view_release_observed = view_release_observed.clone();
            move |_, _| view_release_observed.set(true)
        })
        .detach();

        cx.update(move |_| {
            drop(model);
        });
        assert!(model_released.get());
        assert!(model_release_observed.get());

        drop(view);
        cx.remove_window(window_id);
        assert!(view_released.get());
        assert!(view_release_observed.get());
    }

    #[crate::test(self)]
    fn test_view_events(cx: &mut MutableAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = String;
        }

        let (_, handle_1) = cx.add_window(Default::default(), |_| TestView::default());
        let handle_2 = cx.add_view(&handle_1, |_| TestView::default());
        let handle_3 = cx.add_model(|_| Model);

        handle_1.update(cx, |_, cx| {
            cx.subscribe(&handle_2, move |me, emitter, event, cx| {
                me.events.push(event.clone());

                cx.subscribe(&emitter, |me, _, event, _| {
                    me.events.push(format!("{event} from inner"));
                })
                .detach();
            })
            .detach();

            cx.subscribe(&handle_3, |me, _, event, _| {
                me.events.push(event.clone());
            })
            .detach();
        });

        handle_2.update(cx, |_, c| c.emit("7".into()));
        assert_eq!(handle_1.read(cx).events, vec!["7"]);

        handle_2.update(cx, |_, c| c.emit("5".into()));
        assert_eq!(handle_1.read(cx).events, vec!["7", "5", "5 from inner"]);

        handle_3.update(cx, |_, c| c.emit("9".into()));
        assert_eq!(
            handle_1.read(cx).events,
            vec!["7", "5", "5 from inner", "9"]
        );
    }

    #[crate::test(self)]
    fn test_global_events(cx: &mut MutableAppContext) {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct GlobalEvent(u64);

        let events = Rc::new(RefCell::new(Vec::new()));
        let first_subscription;
        let second_subscription;

        {
            let events = events.clone();
            first_subscription = cx.subscribe_global(move |e: &GlobalEvent, _| {
                events.borrow_mut().push(("First", e.clone()));
            });
        }

        {
            let events = events.clone();
            second_subscription = cx.subscribe_global(move |e: &GlobalEvent, _| {
                events.borrow_mut().push(("Second", e.clone()));
            });
        }

        cx.update(|cx| {
            cx.emit_global(GlobalEvent(1));
            cx.emit_global(GlobalEvent(2));
        });

        drop(first_subscription);

        cx.update(|cx| {
            cx.emit_global(GlobalEvent(3));
        });

        drop(second_subscription);

        cx.update(|cx| {
            cx.emit_global(GlobalEvent(4));
        });

        assert_eq!(
            &*events.borrow(),
            &[
                ("First", GlobalEvent(1)),
                ("Second", GlobalEvent(1)),
                ("First", GlobalEvent(2)),
                ("Second", GlobalEvent(2)),
                ("Second", GlobalEvent(3)),
            ]
        );
    }

    #[crate::test(self)]
    fn test_global_events_emitted_before_subscription_in_same_update_cycle(
        cx: &mut MutableAppContext,
    ) {
        let events = Rc::new(RefCell::new(Vec::new()));
        cx.update(|cx| {
            {
                let events = events.clone();
                drop(cx.subscribe_global(move |_: &(), _| {
                    events.borrow_mut().push("dropped before emit");
                }));
            }

            {
                let events = events.clone();
                cx.subscribe_global(move |_: &(), _| {
                    events.borrow_mut().push("before emit");
                })
                .detach();
            }

            cx.emit_global(());

            {
                let events = events.clone();
                cx.subscribe_global(move |_: &(), _| {
                    events.borrow_mut().push("after emit");
                })
                .detach();
            }
        });

        assert_eq!(*events.borrow(), ["before emit"]);
    }

    #[crate::test(self)]
    fn test_global_nested_events(cx: &mut MutableAppContext) {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct GlobalEvent(u64);

        let events = Rc::new(RefCell::new(Vec::new()));

        {
            let events = events.clone();
            cx.subscribe_global(move |e: &GlobalEvent, cx| {
                events.borrow_mut().push(("Outer", e.clone()));

                if e.0 == 1 {
                    let events = events.clone();
                    cx.subscribe_global(move |e: &GlobalEvent, _| {
                        events.borrow_mut().push(("Inner", e.clone()));
                    })
                    .detach();
                }
            })
            .detach();
        }

        cx.update(|cx| {
            cx.emit_global(GlobalEvent(1));
            cx.emit_global(GlobalEvent(2));
            cx.emit_global(GlobalEvent(3));
        });
        cx.update(|cx| {
            cx.emit_global(GlobalEvent(4));
        });

        assert_eq!(
            &*events.borrow(),
            &[
                ("Outer", GlobalEvent(1)),
                ("Outer", GlobalEvent(2)),
                ("Outer", GlobalEvent(3)),
                ("Outer", GlobalEvent(4)),
                ("Inner", GlobalEvent(4)),
            ]
        );
    }

    #[crate::test(self)]
    fn test_global(cx: &mut MutableAppContext) {
        type Global = usize;

        let observation_count = Rc::new(RefCell::new(0));
        let subscription = cx.observe_global::<Global, _>({
            let observation_count = observation_count.clone();
            move |_| {
                *observation_count.borrow_mut() += 1;
            }
        });

        assert!(!cx.has_global::<Global>());
        assert_eq!(cx.default_global::<Global>(), &0);
        assert_eq!(*observation_count.borrow(), 1);
        assert!(cx.has_global::<Global>());
        assert_eq!(
            cx.update_global::<Global, _, _>(|global, _| {
                *global = 1;
                "Update Result"
            }),
            "Update Result"
        );
        assert_eq!(*observation_count.borrow(), 2);
        assert_eq!(cx.global::<Global>(), &1);

        drop(subscription);
        cx.update_global::<Global, _, _>(|global, _| {
            *global = 2;
        });
        assert_eq!(*observation_count.borrow(), 2);

        type OtherGlobal = f32;

        let observation_count = Rc::new(RefCell::new(0));
        cx.observe_global::<OtherGlobal, _>({
            let observation_count = observation_count.clone();
            move |_| {
                *observation_count.borrow_mut() += 1;
            }
        })
        .detach();

        assert_eq!(
            cx.update_default_global::<OtherGlobal, _, _>(|global, _| {
                assert_eq!(global, &0.0);
                *global = 2.0;
                "Default update result"
            }),
            "Default update result"
        );
        assert_eq!(cx.global::<OtherGlobal>(), &2.0);
        assert_eq!(*observation_count.borrow(), 1);
    }

    #[crate::test(self)]
    fn test_dropping_subscribers(cx: &mut MutableAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let (_, root_view) = cx.add_window(Default::default(), |_| TestView::default());
        let observing_view = cx.add_view(&root_view, |_| TestView::default());
        let emitting_view = cx.add_view(&root_view, |_| TestView::default());
        let observing_model = cx.add_model(|_| Model);
        let observed_model = cx.add_model(|_| Model);

        observing_view.update(cx, |_, cx| {
            cx.subscribe(&emitting_view, |_, _, _, _| {}).detach();
            cx.subscribe(&observed_model, |_, _, _, _| {}).detach();
        });
        observing_model.update(cx, |_, cx| {
            cx.subscribe(&observed_model, |_, _, _, _| {}).detach();
        });

        cx.update(|_| {
            drop(observing_view);
            drop(observing_model);
        });

        emitting_view.update(cx, |_, cx| cx.emit(Default::default()));
        observed_model.update(cx, |_, cx| cx.emit(()));
    }

    #[crate::test(self)]
    fn test_view_emit_before_subscribe_in_same_update_cycle(cx: &mut MutableAppContext) {
        let (_, view) = cx.add_window::<TestView, _>(Default::default(), |cx| {
            drop(cx.subscribe(&cx.handle(), {
                move |this, _, _, _| this.events.push("dropped before flush".into())
            }));
            cx.subscribe(&cx.handle(), {
                move |this, _, _, _| this.events.push("before emit".into())
            })
            .detach();
            cx.emit("the event".into());
            cx.subscribe(&cx.handle(), {
                move |this, _, _, _| this.events.push("after emit".into())
            })
            .detach();
            TestView { events: Vec::new() }
        });

        assert_eq!(view.read(cx).events, ["before emit"]);
    }

    #[crate::test(self)]
    fn test_observe_and_notify_from_view(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct Model {
            state: String,
        }

        impl Entity for Model {
            type Event = ();
        }

        let (_, view) = cx.add_window(Default::default(), |_| TestView::default());
        let model = cx.add_model(|_| Model {
            state: "old-state".into(),
        });

        view.update(cx, |_, c| {
            c.observe(&model, |me, observed, c| {
                me.events.push(observed.read(c).state.clone())
            })
            .detach();
        });

        model.update(cx, |model, cx| {
            model.state = "new-state".into();
            cx.notify();
        });
        assert_eq!(view.read(cx).events, vec!["new-state"]);
    }

    #[crate::test(self)]
    fn test_view_notify_before_observe_in_same_update_cycle(cx: &mut MutableAppContext) {
        let (_, view) = cx.add_window::<TestView, _>(Default::default(), |cx| {
            drop(cx.observe(&cx.handle(), {
                move |this, _, _| this.events.push("dropped before flush".into())
            }));
            cx.observe(&cx.handle(), {
                move |this, _, _| this.events.push("before notify".into())
            })
            .detach();
            cx.notify();
            cx.observe(&cx.handle(), {
                move |this, _, _| this.events.push("after notify".into())
            })
            .detach();
            TestView { events: Vec::new() }
        });

        assert_eq!(view.read(cx).events, ["before notify"]);
    }

    #[crate::test(self)]
    fn test_notify_and_drop_observe_subscription_in_same_update_cycle(cx: &mut MutableAppContext) {
        struct Model;
        impl Entity for Model {
            type Event = ();
        }

        let model = cx.add_model(|_| Model);
        let (_, view) = cx.add_window(Default::default(), |_| TestView::default());

        view.update(cx, |_, cx| {
            model.update(cx, |_, cx| cx.notify());
            drop(cx.observe(&model, move |this, _, _| {
                this.events.push("model notified".into());
            }));
            model.update(cx, |_, cx| cx.notify());
        });

        for _ in 0..3 {
            model.update(cx, |_, cx| cx.notify());
        }

        assert_eq!(view.read(cx).events, Vec::<String>::new());
    }

    #[crate::test(self)]
    fn test_dropping_observers(cx: &mut MutableAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let (_, root_view) = cx.add_window(Default::default(), |_| TestView::default());
        let observing_view = cx.add_view(root_view, |_| TestView::default());
        let observing_model = cx.add_model(|_| Model);
        let observed_model = cx.add_model(|_| Model);

        observing_view.update(cx, |_, cx| {
            cx.observe(&observed_model, |_, _, _| {}).detach();
        });
        observing_model.update(cx, |_, cx| {
            cx.observe(&observed_model, |_, _, _| {}).detach();
        });

        cx.update(|_| {
            drop(observing_view);
            drop(observing_model);
        });

        observed_model.update(cx, |_, cx| cx.notify());
    }

    #[crate::test(self)]
    fn test_dropping_subscriptions_during_callback(cx: &mut MutableAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = u64;
        }

        // Events
        let observing_model = cx.add_model(|_| Model);
        let observed_model = cx.add_model(|_| Model);

        let events = Rc::new(RefCell::new(Vec::new()));

        observing_model.update(cx, |_, cx| {
            let events = events.clone();
            let subscription = Rc::new(RefCell::new(None));
            *subscription.borrow_mut() = Some(cx.subscribe(&observed_model, {
                let subscription = subscription.clone();
                move |_, _, e, _| {
                    subscription.borrow_mut().take();
                    events.borrow_mut().push(*e);
                }
            }));
        });

        observed_model.update(cx, |_, cx| {
            cx.emit(1);
            cx.emit(2);
        });

        assert_eq!(*events.borrow(), [1]);

        // Global Events
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct GlobalEvent(u64);

        let events = Rc::new(RefCell::new(Vec::new()));

        {
            let events = events.clone();
            let subscription = Rc::new(RefCell::new(None));
            *subscription.borrow_mut() = Some(cx.subscribe_global({
                let subscription = subscription.clone();
                move |e: &GlobalEvent, _| {
                    subscription.borrow_mut().take();
                    events.borrow_mut().push(e.clone());
                }
            }));
        }

        cx.update(|cx| {
            cx.emit_global(GlobalEvent(1));
            cx.emit_global(GlobalEvent(2));
        });

        assert_eq!(*events.borrow(), [GlobalEvent(1)]);

        // Model Observation
        let observing_model = cx.add_model(|_| Model);
        let observed_model = cx.add_model(|_| Model);

        let observation_count = Rc::new(RefCell::new(0));

        observing_model.update(cx, |_, cx| {
            let observation_count = observation_count.clone();
            let subscription = Rc::new(RefCell::new(None));
            *subscription.borrow_mut() = Some(cx.observe(&observed_model, {
                let subscription = subscription.clone();
                move |_, _, _| {
                    subscription.borrow_mut().take();
                    *observation_count.borrow_mut() += 1;
                }
            }));
        });

        observed_model.update(cx, |_, cx| {
            cx.notify();
        });

        observed_model.update(cx, |_, cx| {
            cx.notify();
        });

        assert_eq!(*observation_count.borrow(), 1);

        // View Observation
        struct View;

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let (_, root_view) = cx.add_window(Default::default(), |_| View);
        let observing_view = cx.add_view(&root_view, |_| View);
        let observed_view = cx.add_view(&root_view, |_| View);

        let observation_count = Rc::new(RefCell::new(0));
        observing_view.update(cx, |_, cx| {
            let observation_count = observation_count.clone();
            let subscription = Rc::new(RefCell::new(None));
            *subscription.borrow_mut() = Some(cx.observe(&observed_view, {
                let subscription = subscription.clone();
                move |_, _, _| {
                    subscription.borrow_mut().take();
                    *observation_count.borrow_mut() += 1;
                }
            }));
        });

        observed_view.update(cx, |_, cx| {
            cx.notify();
        });

        observed_view.update(cx, |_, cx| {
            cx.notify();
        });

        assert_eq!(*observation_count.borrow(), 1);

        // Global Observation
        let observation_count = Rc::new(RefCell::new(0));
        let subscription = Rc::new(RefCell::new(None));
        *subscription.borrow_mut() = Some(cx.observe_global::<(), _>({
            let observation_count = observation_count.clone();
            let subscription = subscription.clone();
            move |_| {
                subscription.borrow_mut().take();
                *observation_count.borrow_mut() += 1;
            }
        }));

        cx.default_global::<()>();
        cx.set_global(());
        assert_eq!(*observation_count.borrow(), 1);
    }

    #[crate::test(self)]
    fn test_focus(cx: &mut MutableAppContext) {
        struct View {
            name: String,
            events: Arc<Mutex<Vec<String>>>,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }

            fn focus_in(&mut self, focused: AnyViewHandle, cx: &mut ViewContext<Self>) {
                if cx.handle().id() == focused.id() {
                    self.events.lock().push(format!("{} focused", &self.name));
                }
            }

            fn focus_out(&mut self, blurred: AnyViewHandle, cx: &mut ViewContext<Self>) {
                if cx.handle().id() == blurred.id() {
                    self.events.lock().push(format!("{} blurred", &self.name));
                }
            }
        }

        let view_events: Arc<Mutex<Vec<String>>> = Default::default();
        let (_, view_1) = cx.add_window(Default::default(), |_| View {
            events: view_events.clone(),
            name: "view 1".to_string(),
        });
        let view_2 = cx.add_view(&view_1, |_| View {
            events: view_events.clone(),
            name: "view 2".to_string(),
        });

        let observed_events: Arc<Mutex<Vec<String>>> = Default::default();
        view_1.update(cx, |_, cx| {
            cx.observe_focus(&view_2, {
                let observed_events = observed_events.clone();
                move |this, view, focused, cx| {
                    let label = if focused { "focus" } else { "blur" };
                    observed_events.lock().push(format!(
                        "{} observed {}'s {}",
                        this.name,
                        view.read(cx).name,
                        label
                    ))
                }
            })
            .detach();
        });
        view_2.update(cx, |_, cx| {
            cx.observe_focus(&view_1, {
                let observed_events = observed_events.clone();
                move |this, view, focused, cx| {
                    let label = if focused { "focus" } else { "blur" };
                    observed_events.lock().push(format!(
                        "{} observed {}'s {}",
                        this.name,
                        view.read(cx).name,
                        label
                    ))
                }
            })
            .detach();
        });
        assert_eq!(mem::take(&mut *view_events.lock()), ["view 1 focused"]);
        assert_eq!(mem::take(&mut *observed_events.lock()), Vec::<&str>::new());

        view_1.update(cx, |_, cx| {
            // Ensure focus events are sent for all intermediate focuses
            cx.focus(&view_2);
            cx.focus(&view_1);
            cx.focus(&view_2);
        });
        assert!(cx.is_child_focused(view_1.clone()));
        assert!(!cx.is_child_focused(view_2.clone()));
        assert_eq!(
            mem::take(&mut *view_events.lock()),
            [
                "view 1 blurred",
                "view 2 focused",
                "view 2 blurred",
                "view 1 focused",
                "view 1 blurred",
                "view 2 focused"
            ],
        );
        assert_eq!(
            mem::take(&mut *observed_events.lock()),
            [
                "view 2 observed view 1's blur",
                "view 1 observed view 2's focus",
                "view 1 observed view 2's blur",
                "view 2 observed view 1's focus",
                "view 2 observed view 1's blur",
                "view 1 observed view 2's focus"
            ]
        );

        view_1.update(cx, |_, cx| cx.focus(&view_1));
        assert!(!cx.is_child_focused(view_1.clone()));
        assert!(!cx.is_child_focused(view_2.clone()));
        assert_eq!(
            mem::take(&mut *view_events.lock()),
            ["view 2 blurred", "view 1 focused"],
        );
        assert_eq!(
            mem::take(&mut *observed_events.lock()),
            [
                "view 1 observed view 2's blur",
                "view 2 observed view 1's focus"
            ]
        );

        view_1.update(cx, |_, cx| cx.focus(&view_2));
        assert_eq!(
            mem::take(&mut *view_events.lock()),
            ["view 1 blurred", "view 2 focused"],
        );
        assert_eq!(
            mem::take(&mut *observed_events.lock()),
            [
                "view 2 observed view 1's blur",
                "view 1 observed view 2's focus"
            ]
        );

        view_1.update(cx, |_, _| drop(view_2));
        assert_eq!(mem::take(&mut *view_events.lock()), ["view 1 focused"]);
        assert_eq!(mem::take(&mut *observed_events.lock()), Vec::<&str>::new());
    }

    #[crate::test(self)]
    fn test_deserialize_actions(cx: &mut MutableAppContext) {
        #[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
        pub struct ComplexAction {
            arg: String,
            count: usize,
        }

        actions!(test::something, [SimpleAction]);
        impl_actions!(test::something, [ComplexAction]);

        cx.add_global_action(move |_: &SimpleAction, _: &mut MutableAppContext| {});
        cx.add_global_action(move |_: &ComplexAction, _: &mut MutableAppContext| {});

        let action1 = cx
            .deserialize_action(
                "test::something::ComplexAction",
                Some(r#"{"arg": "a", "count": 5}"#),
            )
            .unwrap();
        let action2 = cx
            .deserialize_action("test::something::SimpleAction", None)
            .unwrap();
        assert_eq!(
            action1.as_any().downcast_ref::<ComplexAction>().unwrap(),
            &ComplexAction {
                arg: "a".to_string(),
                count: 5,
            }
        );
        assert_eq!(
            action2.as_any().downcast_ref::<SimpleAction>().unwrap(),
            &SimpleAction
        );
    }

    #[crate::test(self)]
    fn test_dispatch_action(cx: &mut MutableAppContext) {
        struct ViewA {
            id: usize,
        }

        impl Entity for ViewA {
            type Event = ();
        }

        impl View for ViewA {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct ViewB {
            id: usize,
        }

        impl Entity for ViewB {
            type Event = ();
        }

        impl View for ViewB {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        #[derive(Clone, Default, Deserialize, PartialEq)]
        pub struct Action(pub String);

        impl_actions!(test, [Action]);

        let actions = Rc::new(RefCell::new(Vec::new()));

        cx.add_global_action({
            let actions = actions.clone();
            move |_: &Action, _: &mut MutableAppContext| {
                actions.borrow_mut().push("global".to_string());
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewA, action: &Action, cx| {
                assert_eq!(action.0, "bar");
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} a", view.id));
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewA, _: &Action, cx| {
                if view.id != 1 {
                    cx.add_view(|cx| {
                        cx.propagate_action(); // Still works on a nested ViewContext
                        ViewB { id: 5 }
                    });
                }
                actions.borrow_mut().push(format!("{} b", view.id));
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewB, _: &Action, cx| {
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} c", view.id));
            }
        });

        cx.add_action({
            let actions = actions.clone();
            move |view: &mut ViewB, _: &Action, cx| {
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} d", view.id));
            }
        });

        cx.capture_action({
            let actions = actions.clone();
            move |view: &mut ViewA, _: &Action, cx| {
                cx.propagate_action();
                actions.borrow_mut().push(format!("{} capture", view.id));
            }
        });

        let observed_actions = Rc::new(RefCell::new(Vec::new()));
        cx.observe_actions({
            let observed_actions = observed_actions.clone();
            move |action_id, _| observed_actions.borrow_mut().push(action_id)
        })
        .detach();

        let (window_id, view_1) = cx.add_window(Default::default(), |_| ViewA { id: 1 });
        let view_2 = cx.add_view(&view_1, |_| ViewB { id: 2 });
        let view_3 = cx.add_view(&view_2, |_| ViewA { id: 3 });
        let view_4 = cx.add_view(&view_3, |_| ViewB { id: 4 });

        cx.handle_dispatch_action_from_effect(
            window_id,
            Some(view_4.id()),
            &Action("bar".to_string()),
        );

        assert_eq!(
            *actions.borrow(),
            vec![
                "1 capture",
                "3 capture",
                "4 d",
                "4 c",
                "3 b",
                "3 a",
                "2 d",
                "2 c",
                "1 b"
            ]
        );
        assert_eq!(*observed_actions.borrow(), [Action::default().id()]);

        // Remove view_1, which doesn't propagate the action

        let (window_id, view_2) = cx.add_window(Default::default(), |_| ViewB { id: 2 });
        let view_3 = cx.add_view(&view_2, |_| ViewA { id: 3 });
        let view_4 = cx.add_view(&view_3, |_| ViewB { id: 4 });

        actions.borrow_mut().clear();
        cx.handle_dispatch_action_from_effect(
            window_id,
            Some(view_4.id()),
            &Action("bar".to_string()),
        );

        assert_eq!(
            *actions.borrow(),
            vec![
                "3 capture",
                "4 d",
                "4 c",
                "3 b",
                "3 a",
                "2 d",
                "2 c",
                "global"
            ]
        );
        assert_eq!(
            *observed_actions.borrow(),
            [Action::default().id(), Action::default().id()]
        );
    }

    #[crate::test(self)]
    fn test_dispatch_keystroke(cx: &mut MutableAppContext) {
        #[derive(Clone, Deserialize, PartialEq)]
        pub struct Action(String);

        impl_actions!(test, [Action]);

        struct View {
            id: usize,
            keymap_context: KeymapContext,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }

            fn keymap_context(&self, _: &AppContext) -> KeymapContext {
                self.keymap_context.clone()
            }
        }

        impl View {
            fn new(id: usize) -> Self {
                View {
                    id,
                    keymap_context: KeymapContext::default(),
                }
            }
        }

        let mut view_1 = View::new(1);
        let mut view_2 = View::new(2);
        let mut view_3 = View::new(3);
        view_1.keymap_context.add_identifier("a");
        view_2.keymap_context.add_identifier("a");
        view_2.keymap_context.add_identifier("b");
        view_3.keymap_context.add_identifier("a");
        view_3.keymap_context.add_identifier("b");
        view_3.keymap_context.add_identifier("c");

        let (window_id, view_1) = cx.add_window(Default::default(), |_| view_1);
        let view_2 = cx.add_view(&view_1, |_| view_2);
        let _view_3 = cx.add_view(&view_2, |cx| {
            cx.focus_self();
            view_3
        });

        // This binding only dispatches an action on view 2 because that view will have
        // "a" and "b" in its context, but not "c".
        cx.add_bindings(vec![Binding::new(
            "a",
            Action("a".to_string()),
            Some("a && b && !c"),
        )]);

        cx.add_bindings(vec![Binding::new("b", Action("b".to_string()), None)]);

        // This binding only dispatches an action on views 2 and 3, because they have
        // a parent view with a in its context
        cx.add_bindings(vec![Binding::new(
            "c",
            Action("c".to_string()),
            Some("b > c"),
        )]);

        // This binding only dispatches an action on view 2, because they have
        // a parent view with a in its context
        cx.add_bindings(vec![Binding::new(
            "d",
            Action("d".to_string()),
            Some("a && !b > b"),
        )]);

        let actions = Rc::new(RefCell::new(Vec::new()));
        cx.add_action({
            let actions = actions.clone();
            move |view: &mut View, action: &Action, cx| {
                actions
                    .borrow_mut()
                    .push(format!("{} {}", view.id, action.0));

                if action.0 == "b" {
                    cx.propagate_action();
                }
            }
        });

        cx.add_global_action({
            let actions = actions.clone();
            move |action: &Action, _| {
                actions.borrow_mut().push(format!("global {}", action.0));
            }
        });

        cx.dispatch_keystroke(window_id, &Keystroke::parse("a").unwrap());
        assert_eq!(&*actions.borrow(), &["2 a"]);
        actions.borrow_mut().clear();

        cx.dispatch_keystroke(window_id, &Keystroke::parse("b").unwrap());
        assert_eq!(&*actions.borrow(), &["3 b", "2 b", "1 b", "global b"]);
        actions.borrow_mut().clear();

        cx.dispatch_keystroke(window_id, &Keystroke::parse("c").unwrap());
        assert_eq!(&*actions.borrow(), &["3 c"]);
        actions.borrow_mut().clear();

        cx.dispatch_keystroke(window_id, &Keystroke::parse("d").unwrap());
        assert_eq!(&*actions.borrow(), &["2 d"]);
        actions.borrow_mut().clear();
    }

    #[crate::test(self)]
    fn test_keystrokes_for_action(cx: &mut MutableAppContext) {
        actions!(test, [Action1, Action2, GlobalAction]);

        struct View1 {}
        struct View2 {}

        impl Entity for View1 {
            type Event = ();
        }
        impl Entity for View2 {
            type Event = ();
        }

        impl super::View for View1 {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }
            fn ui_name() -> &'static str {
                "View1"
            }
        }
        impl super::View for View2 {
            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }
            fn ui_name() -> &'static str {
                "View2"
            }
        }

        let (window_id, view_1) = cx.add_window(Default::default(), |_| View1 {});
        let view_2 = cx.add_view(&view_1, |cx| {
            cx.focus_self();
            View2 {}
        });

        cx.add_action(|_: &mut View1, _: &Action1, _cx| {});
        cx.add_action(|_: &mut View2, _: &Action2, _cx| {});
        cx.add_global_action(|_: &GlobalAction, _| {});

        cx.add_bindings(vec![
            Binding::new("a", Action1, Some("View1")),
            Binding::new("b", Action2, Some("View1 > View2")),
            Binding::new("c", GlobalAction, Some("View3")), // View 3 does not exist
        ]);

        // Sanity check
        assert_eq!(
            cx.keystrokes_for_action(window_id, view_1.id(), &Action1)
                .unwrap()
                .as_slice(),
            &[Keystroke::parse("a").unwrap()]
        );
        assert_eq!(
            cx.keystrokes_for_action(window_id, view_2.id(), &Action2)
                .unwrap()
                .as_slice(),
            &[Keystroke::parse("b").unwrap()]
        );

        // The 'a' keystroke propagates up the view tree from view_2
        // to view_1. The action, Action1, is handled by view_1.
        assert_eq!(
            cx.keystrokes_for_action(window_id, view_2.id(), &Action1)
                .unwrap()
                .as_slice(),
            &[Keystroke::parse("a").unwrap()]
        );

        // Actions that are handled below the current view don't have bindings
        assert_eq!(
            cx.keystrokes_for_action(window_id, view_1.id(), &Action2),
            None
        );

        // Actions that are handled in other branches of the tree should not have a binding
        assert_eq!(
            cx.keystrokes_for_action(window_id, view_2.id(), &GlobalAction),
            None
        );

        // Produces a list of actions and keybindings
        fn available_actions(
            window_id: usize,
            view_id: usize,
            cx: &mut MutableAppContext,
        ) -> Vec<(&'static str, Vec<Keystroke>)> {
            cx.available_actions(window_id, view_id)
                .map(|(action_name, _, bindings)| {
                    (
                        action_name,
                        bindings
                            .iter()
                            .map(|binding| binding.keystrokes()[0].clone())
                            .collect::<Vec<_>>(),
                    )
                })
                .sorted_by(|(name1, _), (name2, _)| name1.cmp(name2))
                .collect()
        }

        // Check that global actions do not have a binding, even if a binding does exist in another view
        assert_eq!(
            &available_actions(window_id, view_1.id(), cx),
            &[
                ("test::Action1", vec![Keystroke::parse("a").unwrap()]),
                ("test::GlobalAction", vec![])
            ],
        );

        // Check that view 1 actions and bindings are available even when called from view 2
        assert_eq!(
            &available_actions(window_id, view_2.id(), cx),
            &[
                ("test::Action1", vec![Keystroke::parse("a").unwrap()]),
                ("test::Action2", vec![Keystroke::parse("b").unwrap()]),
                ("test::GlobalAction", vec![]),
            ],
        );
    }

    #[crate::test(self)]
    async fn test_model_condition(cx: &mut TestAppContext) {
        struct Counter(usize);

        impl super::Entity for Counter {
            type Event = ();
        }

        impl Counter {
            fn inc(&mut self, cx: &mut ModelContext<Self>) {
                self.0 += 1;
                cx.notify();
            }
        }

        let model = cx.add_model(|_| Counter(0));

        let condition1 = model.condition(cx, |model, _| model.0 == 2);
        let condition2 = model.condition(cx, |model, _| model.0 == 3);
        smol::pin!(condition1, condition2);

        model.update(cx, |model, cx| model.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, None);
        assert_eq!(poll_once(&mut condition2).await, None);

        model.update(cx, |model, cx| model.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, Some(()));
        assert_eq!(poll_once(&mut condition2).await, None);

        model.update(cx, |model, cx| model.inc(cx));
        assert_eq!(poll_once(&mut condition2).await, Some(()));

        model.update(cx, |_, cx| cx.notify());
    }

    #[crate::test(self)]
    #[should_panic]
    async fn test_model_condition_timeout(cx: &mut TestAppContext) {
        struct Model;

        impl super::Entity for Model {
            type Event = ();
        }

        let model = cx.add_model(|_| Model);
        model.condition(cx, |_, _| false).await;
    }

    #[crate::test(self)]
    #[should_panic(expected = "model dropped with pending condition")]
    async fn test_model_condition_panic_on_drop(cx: &mut TestAppContext) {
        struct Model;

        impl super::Entity for Model {
            type Event = ();
        }

        let model = cx.add_model(|_| Model);
        let condition = model.condition(cx, |_, _| false);
        cx.update(|_| drop(model));
        condition.await;
    }

    #[crate::test(self)]
    async fn test_view_condition(cx: &mut TestAppContext) {
        struct Counter(usize);

        impl super::Entity for Counter {
            type Event = ();
        }

        impl super::View for Counter {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }
        }

        impl Counter {
            fn inc(&mut self, cx: &mut ViewContext<Self>) {
                self.0 += 1;
                cx.notify();
            }
        }

        let (_, view) = cx.add_window(|_| Counter(0));

        let condition1 = view.condition(cx, |view, _| view.0 == 2);
        let condition2 = view.condition(cx, |view, _| view.0 == 3);
        smol::pin!(condition1, condition2);

        view.update(cx, |view, cx| view.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, None);
        assert_eq!(poll_once(&mut condition2).await, None);

        view.update(cx, |view, cx| view.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, Some(()));
        assert_eq!(poll_once(&mut condition2).await, None);

        view.update(cx, |view, cx| view.inc(cx));
        assert_eq!(poll_once(&mut condition2).await, Some(()));
        view.update(cx, |_, cx| cx.notify());
    }

    #[crate::test(self)]
    #[should_panic]
    async fn test_view_condition_timeout(cx: &mut TestAppContext) {
        let (_, view) = cx.add_window(|_| TestView::default());
        view.condition(cx, |_, _| false).await;
    }

    #[crate::test(self)]
    #[should_panic(expected = "view dropped with pending condition")]
    async fn test_view_condition_panic_on_drop(cx: &mut TestAppContext) {
        let (_, root_view) = cx.add_window(|_| TestView::default());
        let view = cx.add_view(&root_view, |_| TestView::default());

        let condition = view.condition(cx, |_, _| false);
        cx.update(|_| drop(view));
        condition.await;
    }

    #[crate::test(self)]
    fn test_refresh_windows(cx: &mut MutableAppContext) {
        struct View(usize);

        impl super::Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().named(format!("render count: {}", post_inc(&mut self.0)))
            }
        }

        let (window_id, root_view) = cx.add_window(Default::default(), |_| View(0));
        let presenter = cx.presenters_and_platform_windows[&window_id].0.clone();

        assert_eq!(
            presenter.borrow().rendered_views[&root_view.id()].name(),
            Some("render count: 0")
        );

        let view = cx.add_view(&root_view, |cx| {
            cx.refresh_windows();
            View(0)
        });

        assert_eq!(
            presenter.borrow().rendered_views[&root_view.id()].name(),
            Some("render count: 1")
        );
        assert_eq!(
            presenter.borrow().rendered_views[&view.id()].name(),
            Some("render count: 0")
        );

        cx.update(|cx| cx.refresh_windows());
        assert_eq!(
            presenter.borrow().rendered_views[&root_view.id()].name(),
            Some("render count: 2")
        );
        assert_eq!(
            presenter.borrow().rendered_views[&view.id()].name(),
            Some("render count: 1")
        );

        cx.update(|cx| {
            cx.refresh_windows();
            drop(view);
        });
        assert_eq!(
            presenter.borrow().rendered_views[&root_view.id()].name(),
            Some("render count: 3")
        );
        assert_eq!(presenter.borrow().rendered_views.len(), 1);
    }

    #[crate::test(self)]
    async fn test_labeled_tasks(cx: &mut TestAppContext) {
        assert_eq!(None, cx.update(|cx| cx.active_labeled_tasks().next()));
        let (mut sender, mut reciever) = postage::oneshot::channel::<()>();
        let task = cx
            .update(|cx| cx.spawn_labeled("Test Label", |_| async move { reciever.recv().await }));

        assert_eq!(
            Some("Test Label"),
            cx.update(|cx| cx.active_labeled_tasks().next())
        );
        sender
            .send(())
            .await
            .expect("Could not send message to complete task");
        task.await;

        assert_eq!(None, cx.update(|cx| cx.active_labeled_tasks().next()));
    }

    #[crate::test(self)]
    async fn test_window_activation(cx: &mut TestAppContext) {
        struct View(&'static str);

        impl super::Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                Empty::new().boxed()
            }
        }

        let events = Rc::new(RefCell::new(Vec::new()));
        let (window_1, _) = cx.add_window(|cx: &mut ViewContext<View>| {
            cx.observe_window_activation({
                let events = events.clone();
                move |this, active, _| events.borrow_mut().push((this.0, active))
            })
            .detach();
            View("window 1")
        });
        assert_eq!(mem::take(&mut *events.borrow_mut()), [("window 1", true)]);

        let (window_2, _) = cx.add_window(|cx: &mut ViewContext<View>| {
            cx.observe_window_activation({
                let events = events.clone();
                move |this, active, _| events.borrow_mut().push((this.0, active))
            })
            .detach();
            View("window 2")
        });
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 1", false), ("window 2", true)]
        );

        let (window_3, _) = cx.add_window(|cx: &mut ViewContext<View>| {
            cx.observe_window_activation({
                let events = events.clone();
                move |this, active, _| events.borrow_mut().push((this.0, active))
            })
            .detach();
            View("window 3")
        });
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 2", false), ("window 3", true)]
        );

        cx.simulate_window_activation(Some(window_2));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 3", false), ("window 2", true)]
        );

        cx.simulate_window_activation(Some(window_1));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 2", false), ("window 1", true)]
        );

        cx.simulate_window_activation(Some(window_3));
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 1", false), ("window 3", true)]
        );

        cx.simulate_window_activation(Some(window_3));
        assert_eq!(mem::take(&mut *events.borrow_mut()), []);
    }

    #[crate::test(self)]
    fn test_child_view(cx: &mut MutableAppContext) {
        struct Child {
            rendered: Rc<Cell<bool>>,
            dropped: Rc<Cell<bool>>,
        }

        impl super::Entity for Child {
            type Event = ();
        }

        impl super::View for Child {
            fn ui_name() -> &'static str {
                "child view"
            }

            fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
                self.rendered.set(true);
                Empty::new().boxed()
            }
        }

        impl Drop for Child {
            fn drop(&mut self) {
                self.dropped.set(true);
            }
        }

        struct Parent {
            child: Option<ViewHandle<Child>>,
        }

        impl super::Entity for Parent {
            type Event = ();
        }

        impl super::View for Parent {
            fn ui_name() -> &'static str {
                "parent view"
            }

            fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
                if let Some(child) = self.child.as_ref() {
                    ChildView::new(child, cx).boxed()
                } else {
                    Empty::new().boxed()
                }
            }
        }

        let child_rendered = Rc::new(Cell::new(false));
        let child_dropped = Rc::new(Cell::new(false));
        let (_, root_view) = cx.add_window(Default::default(), |cx| Parent {
            child: Some(cx.add_view(|_| Child {
                rendered: child_rendered.clone(),
                dropped: child_dropped.clone(),
            })),
        });
        assert!(child_rendered.take());
        assert!(!child_dropped.take());

        root_view.update(cx, |view, cx| {
            view.child.take();
            cx.notify();
        });
        assert!(!child_rendered.take());
        assert!(child_dropped.take());
    }

    #[derive(Default)]
    struct TestView {
        events: Vec<String>,
    }

    impl Entity for TestView {
        type Event = String;
    }

    impl View for TestView {
        fn ui_name() -> &'static str {
            "TestView"
        }

        fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
            Empty::new().boxed()
        }
    }
}
