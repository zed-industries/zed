pub mod action;
mod callback_collection;
mod menu;
pub(crate) mod ref_counts;
#[cfg(any(test, feature = "test-support"))]
pub mod test_app_context;
pub(crate) mod window;
mod window_input_handler;

use crate::{
    elements::{AnyElement, AnyRootElement, RootElement},
    executor::{self, Task},
    fonts::TextStyle,
    json,
    keymap_matcher::{self, Binding, KeymapContext, KeymapMatcher, Keystroke, MatchResult},
    platform::{
        self, FontSystem, KeyDownEvent, KeyUpEvent, ModifiersChangedEvent, MouseButton,
        PathPromptOptions, Platform, PromptLevel, WindowBounds, WindowOptions,
    },
    util::post_inc,
    window::{Window, WindowContext},
    AssetCache, AssetSource, ClipboardItem, FontCache, MouseRegionId,
};
pub use action::*;
use anyhow::{anyhow, Context, Result};
use callback_collection::CallbackCollection;
use collections::{hash_map::Entry, BTreeMap, HashMap, HashSet, VecDeque};
use derive_more::Deref;
pub use menu::*;
use parking_lot::Mutex;
use platform::Event;
use postage::oneshot;
#[cfg(any(test, feature = "test-support"))]
use ref_counts::LeakDetector;
use ref_counts::RefCounts;
use smallvec::SmallVec;
use smol::prelude::*;
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
#[cfg(any(test, feature = "test-support"))]
pub use test_app_context::{ContextHandle, TestAppContext};
use util::ResultExt;
use uuid::Uuid;
use window_input_handler::WindowInputHandler;

pub trait Entity: 'static {
    type Event;

    fn release(&mut self, _: &mut AppContext) {}
    fn app_will_quit(
        &mut self,
        _: &mut AppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>> {
        None
    }
}

pub trait View: Entity + Sized {
    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self>;
    fn focus_in(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {}
    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {}
    fn ui_name() -> &'static str {
        type_name::<Self>()
    }
    fn key_down(&mut self, _: &KeyDownEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn key_up(&mut self, _: &KeyUpEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn modifiers_changed(&mut self, _: &ModifiersChangedEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }

    fn update_keymap_context(&self, keymap: &mut keymap_matcher::KeymapContext, _: &AppContext) {
        Self::reset_to_default_keymap_context(keymap);
    }

    fn reset_to_default_keymap_context(keymap: &mut keymap_matcher::KeymapContext) {
        keymap.clear();
        keymap.add_identifier(Self::ui_name());
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

pub trait BorrowAppContext {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T;
    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T;
}

pub trait BorrowWindowContext {
    type Result<T>;

    fn read_window<T, F>(&self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&WindowContext) -> T;
    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>;
    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&mut WindowContext) -> T;
    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>;
}

#[derive(Clone)]
pub struct App(Rc<RefCell<AppContext>>);

impl App {
    pub fn new(
        asset_source: impl AssetSource,
        platform: Arc<dyn Platform>,
        foreground_platform: Rc<dyn platform::ForegroundPlatform>,
    ) -> Result<Self> {
        let foreground = Rc::new(executor::Foreground::platform(platform.dispatcher())?);
        let app = Self(Rc::new(RefCell::new(AppContext::new(
            foreground,
            Arc::new(executor::Background::new()),
            platform.clone(),
            foreground_platform.clone(),
            Arc::new(FontCache::new(platform.fonts())),
            Default::default(),
            asset_source,
        ))));

        foreground_platform.on_event(Box::new({
            let cx = app.0.clone();
            move |event| {
                if let Event::KeyDown(KeyDownEvent { keystroke, .. }) = &event {
                    // Allow system menu "cmd-?" shortcut to be overridden
                    if keystroke.cmd
                        && !keystroke.shift
                        && !keystroke.alt
                        && !keystroke.function
                        && keystroke.key == "?"
                    {
                        if cx
                            .borrow_mut()
                            .update_active_window(|cx| cx.dispatch_keystroke(keystroke))
                            .unwrap_or(false)
                        {
                            return true;
                        }
                    }
                }
                false
            }
        }));
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
        F: 'static + FnMut(&mut AppContext),
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
        F: 'static + FnMut(&mut AppContext),
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
        F: 'static + FnMut(&mut AppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_quit(Box::new(move || callback(&mut *cx.borrow_mut())));
        self
    }

    /// Handle the application being re-activated when no windows are open.
    pub fn on_reopen<F>(&mut self, mut callback: F) -> &mut Self
    where
        F: 'static + FnMut(&mut AppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_reopen(Box::new(move || callback(&mut *cx.borrow_mut())));
        self
    }

    pub fn on_event<F>(&mut self, mut callback: F) -> &mut Self
    where
        F: 'static + FnMut(Event, &mut AppContext) -> bool,
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
        F: 'static + FnMut(Vec<String>, &mut AppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_open_urls(Box::new(move |urls| callback(urls, &mut *cx.borrow_mut())));
        self
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut AppContext),
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
        self.0.borrow().platform.clone()
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.0.borrow().font_cache.clone()
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.0.borrow_mut();
        let result = state.update(callback);
        state.pending_notifications.clear();
        result
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, callback: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> T,
    {
        let mut state = self.0.borrow_mut();
        let result = state.update_window(window, callback);
        state.pending_notifications.clear();
        result
    }
}

#[derive(Clone)]
pub struct AsyncAppContext(Rc<RefCell<AppContext>>);

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
        callback(&*self.0.borrow())
    }

    pub fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, callback: F) -> T {
        self.0.borrow_mut().update(callback)
    }

    pub fn windows(&self) -> Vec<AnyWindowHandle> {
        self.0.borrow().windows().collect()
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
    ) -> WindowHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.update(|cx| cx.add_window(window_options, build_root_view))
    }

    pub fn platform(&self) -> Arc<dyn Platform> {
        self.0.borrow().platform().clone()
    }

    pub fn foreground(&self) -> Rc<executor::Foreground> {
        self.0.borrow().foreground.clone()
    }

    pub fn background(&self) -> Arc<executor::Background> {
        self.0.borrow().background.clone()
    }
}

impl BorrowAppContext for AsyncAppContext {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        self.0.borrow().read_with(f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        self.0.borrow_mut().update(f)
    }
}

impl BorrowWindowContext for AsyncAppContext {
    type Result<T> = Option<T>;

    fn read_window<T, F>(&self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&WindowContext) -> T,
    {
        self.0.borrow().read_with(|cx| cx.read_window(window, f))
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        self.0
            .borrow_mut()
            .update(|cx| cx.read_window_optional(window, f))
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&mut WindowContext) -> T,
    {
        self.0.borrow_mut().update(|cx| cx.update_window(window, f))
    }

    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        self.0
            .borrow_mut()
            .update(|cx| cx.update_window_optional(window, f))
    }
}

type ActionCallback = dyn FnMut(&mut dyn AnyView, &dyn Action, &mut WindowContext, usize);
type GlobalActionCallback = dyn FnMut(&dyn Action, &mut AppContext);

type SubscriptionCallback = Box<dyn FnMut(&dyn Any, &mut AppContext) -> bool>;
type GlobalSubscriptionCallback = Box<dyn FnMut(&dyn Any, &mut AppContext)>;
type ObservationCallback = Box<dyn FnMut(&mut AppContext) -> bool>;
type GlobalObservationCallback = Box<dyn FnMut(&mut AppContext)>;
type FocusObservationCallback = Box<dyn FnMut(bool, &mut WindowContext) -> bool>;
type ReleaseObservationCallback = Box<dyn FnMut(&dyn Any, &mut AppContext)>;
type ActionObservationCallback = Box<dyn FnMut(TypeId, &mut AppContext)>;
type WindowActivationCallback = Box<dyn FnMut(bool, &mut WindowContext) -> bool>;
type WindowFullscreenCallback = Box<dyn FnMut(bool, &mut WindowContext) -> bool>;
type WindowBoundsCallback = Box<dyn FnMut(WindowBounds, Uuid, &mut WindowContext) -> bool>;
type KeystrokeCallback =
    Box<dyn FnMut(&Keystroke, &MatchResult, Option<&Box<dyn Action>>, &mut WindowContext) -> bool>;
type ActiveLabeledTasksCallback = Box<dyn FnMut(&mut AppContext) -> bool>;
type DeserializeActionCallback = fn(json: serde_json::Value) -> anyhow::Result<Box<dyn Action>>;
type WindowShouldCloseSubscriptionCallback = Box<dyn FnMut(&mut AppContext) -> bool>;

pub struct AppContext {
    models: HashMap<usize, Box<dyn AnyModel>>,
    views: HashMap<(AnyWindowHandle, usize), Box<dyn AnyView>>,
    views_metadata: HashMap<(AnyWindowHandle, usize), ViewMetadata>,
    windows: HashMap<AnyWindowHandle, Window>,
    globals: HashMap<TypeId, Box<dyn Any>>,
    element_states: HashMap<ElementStateId, Box<dyn Any>>,
    background: Arc<executor::Background>,
    ref_counts: Arc<Mutex<RefCounts>>,

    weak_self: Option<rc::Weak<RefCell<Self>>>,
    platform: Arc<dyn Platform>,
    foreground_platform: Rc<dyn platform::ForegroundPlatform>,
    pub asset_cache: Arc<AssetCache>,
    font_system: Arc<dyn FontSystem>,
    pub font_cache: Arc<FontCache>,
    action_deserializers: HashMap<&'static str, (TypeId, DeserializeActionCallback)>,
    capture_actions: HashMap<TypeId, HashMap<TypeId, Vec<Box<ActionCallback>>>>,
    // Entity Types -> { Action Types -> Action Handlers }
    actions: HashMap<TypeId, HashMap<TypeId, Vec<Box<ActionCallback>>>>,
    // Action Types -> Action Handlers
    global_actions: HashMap<TypeId, Box<GlobalActionCallback>>,
    keystroke_matcher: KeymapMatcher,
    next_id: usize,
    // next_window: AnyWindowHandle,
    next_subscription_id: usize,
    frame_count: usize,

    subscriptions: CallbackCollection<usize, SubscriptionCallback>,
    global_subscriptions: CallbackCollection<TypeId, GlobalSubscriptionCallback>,
    observations: CallbackCollection<usize, ObservationCallback>,
    global_observations: CallbackCollection<TypeId, GlobalObservationCallback>,
    focus_observations: CallbackCollection<usize, FocusObservationCallback>,
    release_observations: CallbackCollection<usize, ReleaseObservationCallback>,
    action_dispatch_observations: CallbackCollection<(), ActionObservationCallback>,
    window_activation_observations: CallbackCollection<AnyWindowHandle, WindowActivationCallback>,
    window_fullscreen_observations: CallbackCollection<AnyWindowHandle, WindowFullscreenCallback>,
    window_bounds_observations: CallbackCollection<AnyWindowHandle, WindowBoundsCallback>,
    keystroke_observations: CallbackCollection<AnyWindowHandle, KeystrokeCallback>,
    active_labeled_task_observations: CallbackCollection<(), ActiveLabeledTasksCallback>,

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

impl AppContext {
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
            models: Default::default(),
            views: Default::default(),
            views_metadata: Default::default(),
            windows: Default::default(),
            globals: Default::default(),
            element_states: Default::default(),
            ref_counts: Arc::new(Mutex::new(ref_counts)),
            background,

            weak_self: None,
            font_system: platform.fonts(),
            platform,
            foreground_platform,
            font_cache,
            asset_cache: Arc::new(AssetCache::new(asset_source)),
            action_deserializers: Default::default(),
            capture_actions: Default::default(),
            actions: Default::default(),
            global_actions: Default::default(),
            keystroke_matcher: KeymapMatcher::default(),
            next_id: 0,
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

    pub fn optional_global<T: 'static>(&self) -> Option<&T> {
        if let Some(global) = self.globals.get(&TypeId::of::<T>()) {
            Some(global.downcast_ref().unwrap())
        } else {
            None
        }
    }

    pub fn upgrade(&self) -> App {
        App(self.weak_self.as_ref().unwrap().upgrade().unwrap())
    }

    fn quit(&mut self) {
        let mut futures = Vec::new();

        self.update(|cx| {
            for model_id in cx.models.keys().copied().collect::<Vec<_>>() {
                let mut model = cx.models.remove(&model_id).unwrap();
                futures.extend(model.app_will_quit(cx));
                cx.models.insert(model_id, model);
            }

            for view_id in cx.views.keys().copied().collect::<Vec<_>>() {
                let mut view = cx.views.remove(&view_id).unwrap();
                futures.extend(view.app_will_quit(cx));
                cx.views.insert(view_id, view);
            }
        });

        self.windows.clear();
        self.flush_effects();

        let futures = futures::future::join_all(futures);
        if self
            .background
            .block_with_timeout(Duration::from_millis(100), futures)
            .is_err()
        {
            log::error!("timed out waiting on app_will_quit");
        }
    }

    pub fn foreground(&self) -> &Rc<executor::Foreground> {
        &self.foreground
    }

    pub fn deserialize_action(
        &self,
        name: &str,
        argument: Option<serde_json::Value>,
    ) -> Result<Box<dyn Action>> {
        let callback = self
            .action_deserializers
            .get(name)
            .ok_or_else(|| anyhow!("unknown action {}", name))?
            .1;
        callback(argument.unwrap_or_else(|| serde_json::Value::Object(Default::default())))
            .with_context(|| format!("invalid data for action {}", name))
    }

    pub fn add_action<A, V, F, R>(&mut self, handler: F)
    where
        A: Action,
        V: 'static,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>) -> R,
    {
        self.add_action_internal(handler, false)
    }

    pub fn capture_action<A, V, F>(&mut self, handler: F)
    where
        A: Action,
        V: 'static,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>),
    {
        self.add_action_internal(handler, true)
    }

    fn add_action_internal<A, V, F, R>(&mut self, mut handler: F, capture: bool)
    where
        A: Action,
        V: 'static,
        F: 'static + FnMut(&mut V, &A, &mut ViewContext<V>) -> R,
    {
        let handler = Box::new(
            move |view: &mut dyn AnyView,
                  action: &dyn Action,
                  cx: &mut WindowContext,
                  view_id: usize| {
                let action = action.as_any().downcast_ref().unwrap();
                let mut cx = ViewContext::mutable(cx, view_id);
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
        V: 'static,
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
        F: 'static + FnMut(&A, &mut AppContext),
    {
        let handler = Box::new(move |action: &dyn Action, cx: &mut AppContext| {
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

    pub fn view_ui_name(&self, window: AnyWindowHandle, view_id: usize) -> Option<&'static str> {
        Some(self.views.get(&(window, view_id))?.ui_name())
    }

    pub fn view_type_id(&self, window: AnyWindowHandle, view_id: usize) -> Option<TypeId> {
        self.views_metadata
            .get(&(window, view_id))
            .map(|metadata| metadata.type_id)
    }

    pub fn active_labeled_tasks<'a>(
        &'a self,
    ) -> impl DoubleEndedIterator<Item = &'static str> + 'a {
        self.active_labeled_tasks.values().cloned()
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

    fn read_window<T, F: FnOnce(&WindowContext) -> T>(
        &self,
        handle: AnyWindowHandle,
        callback: F,
    ) -> Option<T> {
        let window = self.windows.get(&handle)?;
        let window_context = WindowContext::immutable(self, &window, handle);
        Some(callback(&window_context))
    }

    pub fn update_active_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        callback: F,
    ) -> Option<T> {
        self.active_window()
            .and_then(|window| window.update(self, callback))
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

    fn subscribe_internal<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
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
                if let Some(emitter) = H::upgrade_from(&emitter, cx) {
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
        V: 'static,
        F: 'static + FnMut(ViewHandle<V>, bool, &mut WindowContext) -> bool,
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
        F: 'static + FnMut(&mut AppContext),
    {
        let type_id = TypeId::of::<G>();
        let id = post_inc(&mut self.next_subscription_id);

        self.global_observations.add_callback(
            type_id,
            id,
            Box::new(move |cx: &mut AppContext| observe(cx)),
        );
        Subscription::GlobalObservation(self.global_observations.subscribe(type_id, id))
    }

    pub fn observe_default_global<G, F>(&mut self, observe: F) -> Subscription
    where
        G: Any + Default,
        F: 'static + FnMut(&mut AppContext),
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
        F: 'static + FnMut(TypeId, &mut AppContext),
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.action_dispatch_observations
            .add_callback((), subscription_id, Box::new(callback));
        Subscription::ActionObservation(
            self.action_dispatch_observations
                .subscribe((), subscription_id),
        )
    }

    fn observe_active_labeled_tasks<F>(&mut self, callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut AppContext) -> bool,
    {
        let subscription_id = post_inc(&mut self.next_subscription_id);
        self.active_labeled_task_observations
            .add_callback((), subscription_id, Box::new(callback));
        Subscription::ActiveLabeledTasksObservation(
            self.active_labeled_task_observations
                .subscribe((), subscription_id),
        )
    }

    pub fn defer(&mut self, callback: impl 'static + FnOnce(&mut AppContext)) {
        self.pending_effects.push_back(Effect::Deferred {
            callback: Box::new(callback),
            after_window_update: false,
        })
    }

    pub fn after_window_update(&mut self, callback: impl 'static + FnOnce(&mut AppContext)) {
        self.pending_effects.push_back(Effect::Deferred {
            callback: Box::new(callback),
            after_window_update: true,
        })
    }

    fn notify_model(&mut self, model_id: usize) {
        if self.pending_notifications.insert(model_id) {
            self.pending_effects
                .push_back(Effect::ModelNotification { model_id });
        }
    }

    fn notify_view(&mut self, window: AnyWindowHandle, view_id: usize) {
        if self.pending_notifications.insert(view_id) {
            self.pending_effects
                .push_back(Effect::ViewNotification { window, view_id });
        }
    }

    fn notify_global(&mut self, type_id: TypeId) {
        if self.pending_global_notifications.insert(type_id) {
            self.pending_effects
                .push_back(Effect::GlobalNotification { type_id });
        }
    }

    pub fn all_action_names<'a>(&'a self) -> impl Iterator<Item = &'static str> + 'a {
        self.action_deserializers.keys().copied()
    }

    pub fn is_action_available(&self, action: &dyn Action) -> bool {
        let mut available_in_window = false;
        let action_id = action.id();
        if let Some(window) = self.active_window() {
            available_in_window = self
                .read_window(window, |cx| {
                    if let Some(focused_view_id) = cx.focused_view_id() {
                        for view_id in cx.ancestors(focused_view_id) {
                            if let Some(view_metadata) =
                                cx.views_metadata.get(&(cx.window_handle, view_id))
                            {
                                if let Some(actions) = cx.actions.get(&view_metadata.type_id) {
                                    if actions.contains_key(&action_id) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                    false
                })
                .unwrap_or(false);
        }
        available_in_window || self.global_actions.contains_key(&action_id)
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

    pub fn binding_for_action(&self, action: &dyn Action) -> Option<&Binding> {
        self.keystroke_matcher
            .bindings_for_action(action.id())
            .find(|binding| binding.action().eq(action))
    }

    pub fn default_global<T: 'static + Default>(&mut self) -> &T {
        let type_id = TypeId::of::<T>();
        self.update(|this| {
            if let Entry::Vacant(entry) = this.globals.entry(type_id) {
                entry.insert(Box::new(T::default()));
                this.notify_global(type_id);
            }
        });
        self.globals.get(&type_id).unwrap().downcast_ref().unwrap()
    }

    pub fn set_global<T: 'static>(&mut self, state: T) {
        self.update(|this| {
            let type_id = TypeId::of::<T>();
            this.globals.insert(type_id, Box::new(state));
            this.notify_global(type_id);
        });
    }

    pub fn update_default_global<T, F, U>(&mut self, update: F) -> U
    where
        T: 'static + Default,
        F: FnOnce(&mut T, &mut AppContext) -> U,
    {
        self.update(|mut this| {
            Self::update_default_global_internal(&mut this, |global, cx| update(global, cx))
        })
    }

    fn update_default_global_internal<C, T, F, U>(this: &mut C, update: F) -> U
    where
        C: DerefMut<Target = AppContext>,
        T: 'static + Default,
        F: FnOnce(&mut T, &mut C) -> U,
    {
        let type_id = TypeId::of::<T>();
        let mut state = this
            .globals
            .remove(&type_id)
            .unwrap_or_else(|| Box::new(T::default()));
        let result = update(state.downcast_mut().unwrap(), this);
        this.globals.insert(type_id, state);
        this.notify_global(type_id);
        result
    }

    pub fn update_global<T, F, U>(&mut self, update: F) -> U
    where
        T: 'static,
        F: FnOnce(&mut T, &mut AppContext) -> U,
    {
        self.update(|mut this| {
            Self::update_global_internal(&mut this, |global, cx| update(global, cx))
        })
    }

    fn update_global_internal<C, T, F, U>(this: &mut C, update: F) -> U
    where
        C: DerefMut<Target = AppContext>,
        T: 'static,
        F: FnOnce(&mut T, &mut C) -> U,
    {
        let type_id = TypeId::of::<T>();
        if let Some(mut state) = this.globals.remove(&type_id) {
            let result = update(state.downcast_mut().unwrap(), this);
            this.globals.insert(type_id, state);
            this.notify_global(type_id);
            result
        } else {
            panic!("no global added for {}", std::any::type_name::<T>());
        }
    }

    pub fn clear_globals(&mut self) {
        self.globals.clear();
    }

    pub fn remove_global<T: 'static>(&mut self) -> T {
        *self
            .globals
            .remove(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("no global added for {}", std::any::type_name::<T>()))
            .downcast()
            .unwrap()
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.update(|this| {
            let model_id = post_inc(&mut this.next_id);
            let handle = ModelHandle::new(model_id, &this.ref_counts);
            let mut cx = ModelContext::new(this, model_id);
            let model = build_model(&mut cx);
            this.models.insert(model_id, Box::new(model));
            handle
        })
    }

    pub fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        if let Some(model) = self.models.get(&handle.model_id) {
            model
                .as_any()
                .downcast_ref()
                .expect("downcast is type safe")
        } else {
            panic!("circular model reference");
        }
    }

    fn update_model<T: Entity, V>(
        &mut self,
        handle: &ModelHandle<T>,
        update: &mut dyn FnMut(&mut T, &mut ModelContext<T>) -> V,
    ) -> V {
        if let Some(mut model) = self.models.remove(&handle.model_id) {
            self.update(|this| {
                let mut cx = ModelContext::new(this, handle.model_id);
                let result = update(
                    model
                        .as_any_mut()
                        .downcast_mut()
                        .expect("downcast is type safe"),
                    &mut cx,
                );
                this.models.insert(handle.model_id, model);
                result
            })
        } else {
            panic!("circular model update");
        }
    }

    fn upgrade_model_handle<T: Entity>(
        &self,
        handle: &WeakModelHandle<T>,
    ) -> Option<ModelHandle<T>> {
        if self.ref_counts.lock().is_entity_alive(handle.model_id) {
            Some(ModelHandle::new(handle.model_id, &self.ref_counts))
        } else {
            None
        }
    }

    fn model_handle_is_upgradable<T: Entity>(&self, handle: &WeakModelHandle<T>) -> bool {
        self.ref_counts.lock().is_entity_alive(handle.model_id)
    }

    fn upgrade_any_model_handle(&self, handle: &AnyWeakModelHandle) -> Option<AnyModelHandle> {
        if self.ref_counts.lock().is_entity_alive(handle.model_id) {
            Some(AnyModelHandle::new(
                handle.model_id,
                handle.model_type,
                self.ref_counts.clone(),
            ))
        } else {
            None
        }
    }

    pub fn add_window<V, F>(
        &mut self,
        window_options: WindowOptions,
        build_root_view: F,
    ) -> WindowHandle<V>
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        self.update(|this| {
            let handle = WindowHandle::<V>::new(post_inc(&mut this.next_id));
            let platform_window =
                this.platform
                    .open_window(handle.into(), window_options, this.foreground.clone());
            let window = this.build_window(handle.into(), platform_window, build_root_view);
            this.windows.insert(handle.into(), window);
            handle
        })
    }

    pub fn add_status_bar_item<V, F>(&mut self, build_root_view: F) -> WindowHandle<V>
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        self.update(|this| {
            let handle = WindowHandle::<V>::new(post_inc(&mut this.next_id));
            let platform_window = this.platform.add_status_item(handle.into());
            let window = this.build_window(handle.into(), platform_window, build_root_view);
            this.windows.insert(handle.into(), window);
            handle.update_root(this, |view, cx| view.focus_in(cx.handle().into_any(), cx));
            handle
        })
    }

    pub fn build_window<V, F>(
        &mut self,
        handle: AnyWindowHandle,
        mut platform_window: Box<dyn platform::Window>,
        build_root_view: F,
    ) -> Window
    where
        V: View,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        {
            let mut app = self.upgrade();

            platform_window.on_event(Box::new(move |event| {
                app.update_window(handle, |cx| {
                    if let Event::KeyDown(KeyDownEvent { keystroke, .. }) = &event {
                        if cx.dispatch_keystroke(keystroke) {
                            return true;
                        }
                    }

                    cx.dispatch_event(event, false)
                })
                .unwrap_or(false)
            }));
        }

        {
            let mut app = self.upgrade();
            platform_window.on_active_status_change(Box::new(move |is_active| {
                app.update(|cx| cx.window_changed_active_status(handle, is_active))
            }));
        }

        {
            let mut app = self.upgrade();
            platform_window.on_resize(Box::new(move || {
                app.update(|cx| cx.window_was_resized(handle))
            }));
        }

        {
            let mut app = self.upgrade();
            platform_window.on_moved(Box::new(move || {
                app.update(|cx| cx.window_was_moved(handle))
            }));
        }

        {
            let mut app = self.upgrade();
            platform_window.on_fullscreen(Box::new(move |is_fullscreen| {
                app.update(|cx| cx.window_was_fullscreen_changed(handle, is_fullscreen))
            }));
        }

        {
            let mut app = self.upgrade();
            platform_window.on_close(Box::new(move || {
                app.update(|cx| cx.update_window(handle, |cx| cx.remove_window()));
            }));
        }

        {
            let mut app = self.upgrade();
            platform_window
                .on_appearance_changed(Box::new(move || app.update(|cx| cx.refresh_windows())));
        }

        platform_window.set_input_handler(Box::new(WindowInputHandler {
            app: self.upgrade().0,
            window: handle,
        }));

        let mut window = Window::new(handle, platform_window, self, build_root_view);
        let mut cx = WindowContext::mutable(self, &mut window, handle);
        cx.layout(false).expect("initial layout should not error");
        let scene = cx.paint().expect("initial paint should not error");
        window.platform_window.present_scene(scene);
        window
    }

    pub fn active_window(&self) -> Option<AnyWindowHandle> {
        self.platform.main_window()
    }

    pub fn windows(&self) -> impl '_ + Iterator<Item = AnyWindowHandle> {
        self.windows.keys().copied()
    }

    pub fn read_view<V: 'static>(&self, handle: &ViewHandle<V>) -> &V {
        if let Some(view) = self.views.get(&(handle.window, handle.view_id)) {
            view.as_any().downcast_ref().expect("downcast is type safe")
        } else {
            panic!("circular view reference for type {}", type_name::<V>());
        }
    }

    fn upgrade_view_handle<V: 'static>(&self, handle: &WeakViewHandle<V>) -> Option<ViewHandle<V>> {
        if self.ref_counts.lock().is_entity_alive(handle.view_id) {
            Some(ViewHandle::new(
                handle.window,
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
                handle.window,
                handle.view_id,
                handle.view_type,
                self.ref_counts.clone(),
            ))
        } else {
            None
        }
    }

    fn remove_dropped_entities(&mut self) {
        loop {
            let (dropped_models, dropped_views, dropped_element_states) =
                self.ref_counts.lock().take_dropped();
            if dropped_models.is_empty()
                && dropped_views.is_empty()
                && dropped_element_states.is_empty()
            {
                break;
            }

            for model_id in dropped_models {
                self.subscriptions.remove(model_id);
                self.observations.remove(model_id);
                let mut model = self.models.remove(&model_id).unwrap();
                model.release(self);
                self.pending_effects
                    .push_back(Effect::ModelRelease { model_id, model });
            }

            for (window, view_id) in dropped_views {
                self.subscriptions.remove(view_id);
                self.observations.remove(view_id);
                self.views_metadata.remove(&(window, view_id));
                let mut view = self.views.remove(&(window, view_id)).unwrap();
                view.release(self);
                if let Some(window) = self.windows.get_mut(&window) {
                    window.parents.remove(&view_id);
                    window
                        .invalidation
                        .get_or_insert_with(Default::default)
                        .removed
                        .push(view_id);
                }

                self.pending_effects
                    .push_back(Effect::ViewRelease { view_id, view });
            }

            for key in dropped_element_states {
                self.element_states.remove(&key);
            }
        }
    }

    fn flush_effects(&mut self) {
        self.pending_flushes = self.pending_flushes.saturating_sub(1);
        let mut after_window_update_callbacks = Vec::new();

        if !self.flushing_effects && self.pending_flushes == 0 {
            self.flushing_effects = true;

            let mut refreshing = false;
            let mut updated_windows = HashSet::default();
            let mut focus_effects = HashMap::<AnyWindowHandle, FocusEffect>::default();
            loop {
                self.remove_dropped_entities();
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
                            subscriptions
                                .emit(entity_id, |callback| callback(payload.as_ref(), self))
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
                            observations.emit(model_id, |callback| callback(self));
                        }

                        Effect::ViewNotification {
                            window: window_id,
                            view_id,
                        } => self.handle_view_notification_effect(window_id, view_id),

                        Effect::GlobalNotification { type_id } => {
                            let mut subscriptions = self.global_observations.clone();
                            subscriptions.emit(type_id, |callback| {
                                callback(self);
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

                        Effect::Focus(mut effect) => {
                            if focus_effects
                                .get(&effect.window())
                                .map_or(false, |prev_effect| prev_effect.is_forced())
                            {
                                effect.force();
                            }

                            focus_effects.insert(effect.window(), effect);
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

                        Effect::ResizeWindow { window } => {
                            if let Some(window) = self.windows.get_mut(&window) {
                                window
                                    .invalidation
                                    .get_or_insert(WindowInvalidation::default());
                            }
                            self.handle_window_moved(window);
                        }

                        Effect::MoveWindow { window } => {
                            self.handle_window_moved(window);
                        }

                        Effect::WindowActivationObservation {
                            window,
                            subscription_id,
                            callback,
                        } => self.window_activation_observations.add_callback(
                            window,
                            subscription_id,
                            callback,
                        ),

                        Effect::ActivateWindow { window, is_active } => {
                            if self.handle_window_activation_effect(window, is_active) && is_active
                            {
                                focus_effects
                                    .entry(window)
                                    .or_insert_with(|| FocusEffect::View {
                                        window,
                                        view_id: self
                                            .read_window(window, |cx| cx.focused_view_id())
                                            .flatten(),
                                        is_forced: true,
                                    })
                                    .force();
                            }
                        }

                        Effect::WindowFullscreenObservation {
                            window,
                            subscription_id,
                            callback,
                        } => self.window_fullscreen_observations.add_callback(
                            window,
                            subscription_id,
                            callback,
                        ),

                        Effect::FullscreenWindow {
                            window,
                            is_fullscreen,
                        } => self.handle_fullscreen_effect(window, is_fullscreen),

                        Effect::WindowBoundsObservation {
                            window,
                            subscription_id,
                            callback,
                        } => self.window_bounds_observations.add_callback(
                            window,
                            subscription_id,
                            callback,
                        ),

                        Effect::RefreshWindows => {
                            refreshing = true;
                        }

                        Effect::ActionDispatchNotification { action_id } => {
                            self.handle_action_dispatch_notification_effect(action_id)
                        }
                        Effect::WindowShouldCloseSubscription { window, callback } => {
                            self.handle_window_should_close_subscription_effect(window, callback)
                        }
                        Effect::Keystroke {
                            window,
                            keystroke,
                            handled_by,
                            result,
                        } => self.handle_keystroke_effect(window, keystroke, handled_by, result),
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
                        Effect::RepaintWindow { window } => {
                            self.handle_repaint_window_effect(window)
                        }
                    }
                    self.pending_notifications.clear();
                } else {
                    for window in self.windows().collect::<Vec<_>>() {
                        self.update_window(window, |cx| {
                            let invalidation = if refreshing {
                                let mut invalidation =
                                    cx.window.invalidation.take().unwrap_or_default();
                                invalidation
                                    .updated
                                    .extend(cx.window.rendered_views.keys().copied());
                                Some(invalidation)
                            } else {
                                cx.window.invalidation.take()
                            };

                            if let Some(invalidation) = invalidation {
                                let appearance = cx.window.platform_window.appearance();
                                cx.invalidate(invalidation, appearance);
                                if let Some(old_parents) = cx.layout(refreshing).log_err() {
                                    updated_windows.insert(window);

                                    if let Some(focused_view_id) = cx.focused_view_id() {
                                        let old_ancestors = std::iter::successors(
                                            Some(focused_view_id),
                                            |&view_id| old_parents.get(&view_id).copied(),
                                        )
                                        .collect::<HashSet<_>>();
                                        let new_ancestors =
                                            cx.ancestors(focused_view_id).collect::<HashSet<_>>();

                                        // Notify the old ancestors of the focused view when they don't contain it anymore.
                                        for old_ancestor in old_ancestors.iter().copied() {
                                            if !new_ancestors.contains(&old_ancestor) {
                                                if let Some(mut view) =
                                                    cx.views.remove(&(window, old_ancestor))
                                                {
                                                    view.focus_out(
                                                        focused_view_id,
                                                        cx,
                                                        old_ancestor,
                                                    );
                                                    cx.views.insert((window, old_ancestor), view);
                                                }
                                            }
                                        }

                                        // Notify the new ancestors of the focused view if they contain it now.
                                        for new_ancestor in new_ancestors.iter().copied() {
                                            if !old_ancestors.contains(&new_ancestor) {
                                                if let Some(mut view) =
                                                    cx.views.remove(&(window, new_ancestor))
                                                {
                                                    view.focus_in(
                                                        focused_view_id,
                                                        cx,
                                                        new_ancestor,
                                                    );
                                                    cx.views.insert((window, new_ancestor), view);
                                                }
                                            }
                                        }

                                        // When the previously-focused view has been dropped and
                                        // there isn't any pending focus, focus the root view.
                                        let root_view_id = cx.window.root_view().id();
                                        if focused_view_id != root_view_id
                                            && !cx.views.contains_key(&(window, focused_view_id))
                                            && !focus_effects.contains_key(&window)
                                        {
                                            focus_effects.insert(
                                                window,
                                                FocusEffect::View {
                                                    window,
                                                    view_id: Some(root_view_id),
                                                    is_forced: false,
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                        });
                    }

                    for (_, effect) in focus_effects.drain() {
                        self.handle_focus_effect(effect);
                    }

                    if self.pending_effects.is_empty() {
                        for callback in after_window_update_callbacks.drain(..) {
                            callback(self);
                        }

                        for window in updated_windows.drain() {
                            self.update_window(window, |cx| {
                                if let Some(scene) = cx.paint().log_err() {
                                    cx.window.platform_window.present_scene(scene);
                                }
                            });
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

    fn window_was_resized(&mut self, window: AnyWindowHandle) {
        self.pending_effects
            .push_back(Effect::ResizeWindow { window });
    }

    fn window_was_moved(&mut self, window: AnyWindowHandle) {
        self.pending_effects
            .push_back(Effect::MoveWindow { window });
    }

    fn window_was_fullscreen_changed(&mut self, window: AnyWindowHandle, is_fullscreen: bool) {
        self.pending_effects.push_back(Effect::FullscreenWindow {
            window,
            is_fullscreen,
        });
    }

    fn window_changed_active_status(&mut self, window: AnyWindowHandle, is_active: bool) {
        self.pending_effects
            .push_back(Effect::ActivateWindow { window, is_active });
    }

    fn keystroke(
        &mut self,
        window: AnyWindowHandle,
        keystroke: Keystroke,
        handled_by: Option<Box<dyn Action>>,
        result: MatchResult,
    ) {
        self.pending_effects.push_back(Effect::Keystroke {
            window,
            keystroke,
            handled_by,
            result,
        });
    }

    pub fn refresh_windows(&mut self) {
        self.pending_effects.push_back(Effect::RefreshWindows);
    }

    fn emit_global_event(&mut self, payload: Box<dyn Any>) {
        let type_id = (&*payload).type_id();

        let mut subscriptions = self.global_subscriptions.clone();
        subscriptions.emit(type_id, |callback| {
            callback(payload.as_ref(), self);
            true //Always alive
        });
    }

    fn handle_view_notification_effect(
        &mut self,
        observed_window: AnyWindowHandle,
        observed_view_id: usize,
    ) {
        let view_key = (observed_window, observed_view_id);
        if let Some((view, mut view_metadata)) = self
            .views
            .remove(&view_key)
            .zip(self.views_metadata.remove(&view_key))
        {
            if let Some(window) = self.windows.get_mut(&observed_window) {
                window
                    .invalidation
                    .get_or_insert_with(Default::default)
                    .updated
                    .insert(observed_view_id);
            }

            view.update_keymap_context(&mut view_metadata.keymap_context, self);
            self.views.insert(view_key, view);
            self.views_metadata.insert(view_key, view_metadata);

            let mut observations = self.observations.clone();
            observations.emit(observed_view_id, |callback| callback(self));
        }
    }

    fn handle_entity_release_effect(&mut self, entity_id: usize, entity: &dyn Any) {
        self.release_observations
            .clone()
            .emit(entity_id, |callback| {
                callback(entity, self);
                // Release observations happen one time. So clear the callback by returning false
                false
            })
    }

    fn handle_fullscreen_effect(&mut self, window: AnyWindowHandle, is_fullscreen: bool) {
        self.update_window(window, |cx| {
            cx.window.is_fullscreen = is_fullscreen;

            let mut fullscreen_observations = cx.window_fullscreen_observations.clone();
            fullscreen_observations.emit(window, |callback| callback(is_fullscreen, cx));

            if let Some(uuid) = cx.window_display_uuid() {
                let bounds = cx.window_bounds();
                let mut bounds_observations = cx.window_bounds_observations.clone();
                bounds_observations.emit(window, |callback| callback(bounds, uuid, cx));
            }

            Some(())
        });
    }

    fn handle_keystroke_effect(
        &mut self,
        window: AnyWindowHandle,
        keystroke: Keystroke,
        handled_by: Option<Box<dyn Action>>,
        result: MatchResult,
    ) {
        self.update_window(window, |cx| {
            let mut observations = cx.keystroke_observations.clone();
            observations.emit(window, move |callback| {
                callback(&keystroke, &result, handled_by.as_ref(), cx)
            });
        });
    }

    fn handle_repaint_window_effect(&mut self, window: AnyWindowHandle) {
        self.update_window(window, |cx| {
            cx.layout(false).log_err();
            if let Some(scene) = cx.paint().log_err() {
                cx.window.platform_window.present_scene(scene);
            }
        });
    }

    fn handle_window_activation_effect(&mut self, window: AnyWindowHandle, active: bool) -> bool {
        self.update_window(window, |cx| {
            if cx.window.is_active == active {
                return false;
            }
            cx.window.is_active = active;

            let mut observations = cx.window_activation_observations.clone();
            observations.emit(window, |callback| callback(active, cx));
            true
        })
        .unwrap_or(false)
    }

    fn handle_focus_effect(&mut self, effect: FocusEffect) {
        let window = effect.window();
        self.update_window(window, |cx| {
            // Ensure the newly-focused view still exists, otherwise focus
            // the root view instead.
            let focused_id = match effect {
                FocusEffect::View { view_id, .. } => {
                    if let Some(view_id) = view_id {
                        if cx.views.contains_key(&(window, view_id)) {
                            Some(view_id)
                        } else {
                            Some(cx.root_view().id())
                        }
                    } else {
                        None
                    }
                }
                FocusEffect::ViewParent { view_id, .. } => Some(
                    cx.window
                        .parents
                        .get(&view_id)
                        .copied()
                        .unwrap_or(cx.root_view().id()),
                ),
            };

            let focus_changed = cx.window.focused_view_id != focused_id;
            let blurred_id = cx.window.focused_view_id;
            cx.window.focused_view_id = focused_id;

            if focus_changed {
                if let Some(blurred_id) = blurred_id {
                    for view_id in cx.ancestors(blurred_id).collect::<Vec<_>>() {
                        if let Some(mut view) = cx.views.remove(&(window, view_id)) {
                            view.focus_out(blurred_id, cx, view_id);
                            cx.views.insert((window, view_id), view);
                        }
                    }

                    let mut subscriptions = cx.focus_observations.clone();
                    subscriptions.emit(blurred_id, |callback| callback(false, cx));
                }
            }

            if focus_changed || effect.is_forced() {
                if let Some(focused_id) = focused_id {
                    for view_id in cx.ancestors(focused_id).collect::<Vec<_>>() {
                        if let Some(mut view) = cx.views.remove(&(window, view_id)) {
                            view.focus_in(focused_id, cx, view_id);
                            cx.views.insert((window, view_id), view);
                        }
                    }

                    let mut subscriptions = cx.focus_observations.clone();
                    subscriptions.emit(focused_id, |callback| callback(true, cx));
                }
            }
        });
    }

    fn handle_action_dispatch_notification_effect(&mut self, action_id: TypeId) {
        self.action_dispatch_observations
            .clone()
            .emit((), |callback| {
                callback(action_id, self);
                true
            });
    }

    fn handle_window_should_close_subscription_effect(
        &mut self,
        window: AnyWindowHandle,
        mut callback: WindowShouldCloseSubscriptionCallback,
    ) {
        let mut app = self.upgrade();
        if let Some(window) = self.windows.get_mut(&window) {
            window
                .platform_window
                .on_should_close(Box::new(move || app.update(|cx| callback(cx))))
        }
    }

    fn handle_window_moved(&mut self, window: AnyWindowHandle) {
        self.update_window(window, |cx| {
            if let Some(display) = cx.window_display_uuid() {
                let bounds = cx.window_bounds();
                cx.window_bounds_observations
                    .clone()
                    .emit(window, move |callback| {
                        callback(bounds, display, cx);
                        true
                    });
            }
        });
    }

    fn handle_active_labeled_tasks_changed_effect(&mut self) {
        self.active_labeled_task_observations
            .clone()
            .emit((), move |callback| {
                callback(self);
                true
            });
    }

    pub fn focus(&mut self, window: AnyWindowHandle, view_id: Option<usize>) {
        self.pending_effects
            .push_back(Effect::Focus(FocusEffect::View {
                window,
                view_id,
                is_forced: false,
            }));
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
        self.platform.write_to_clipboard(item);
    }

    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform.read_from_clipboard()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn leak_detector(&self) -> Arc<Mutex<LeakDetector>> {
        self.ref_counts.lock().leak_detector.clone()
    }
}

impl BorrowAppContext for AppContext {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        f(self)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        f(self)
    }
}

impl BorrowWindowContext for AppContext {
    type Result<T> = Option<T>;

    fn read_window<T, F>(&self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&WindowContext) -> T,
    {
        AppContext::read_window(self, window, f)
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        AppContext::read_window(self, window, f).flatten()
    }

    fn update_window<T, F>(&mut self, handle: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&mut WindowContext) -> T,
    {
        self.update(|cx| {
            let mut window = cx.windows.remove(&handle)?;
            let mut window_context = WindowContext::mutable(cx, &mut window, handle);
            let result = f(&mut window_context);
            if !window_context.removed {
                cx.windows.insert(handle, window);
            }
            Some(result)
        })
    }

    fn update_window_optional<T, F>(&mut self, handle: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        AppContext::update_window(self, handle, f).flatten()
    }
}

#[derive(Debug)]
pub enum ParentId {
    View(usize),
    Root,
}

struct ViewMetadata {
    type_id: TypeId,
    keymap_context: KeymapContext,
}

#[derive(Default, Clone, Debug)]
pub struct WindowInvalidation {
    pub updated: HashSet<usize>,
    pub removed: Vec<usize>,
}

#[derive(Debug)]
pub enum FocusEffect {
    View {
        window: AnyWindowHandle,
        view_id: Option<usize>,
        is_forced: bool,
    },
    ViewParent {
        window: AnyWindowHandle,
        view_id: usize,
        is_forced: bool,
    },
}

impl FocusEffect {
    fn window(&self) -> AnyWindowHandle {
        match self {
            FocusEffect::View { window, .. } => *window,
            FocusEffect::ViewParent { window, .. } => *window,
        }
    }

    fn is_forced(&self) -> bool {
        match self {
            FocusEffect::View { is_forced, .. } => *is_forced,
            FocusEffect::ViewParent { is_forced, .. } => *is_forced,
        }
    }

    fn force(&mut self) {
        match self {
            FocusEffect::View { is_forced, .. } => *is_forced = true,
            FocusEffect::ViewParent { is_forced, .. } => *is_forced = true,
        }
    }
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
        window: AnyWindowHandle,
        view_id: usize,
    },
    Deferred {
        callback: Box<dyn FnOnce(&mut AppContext)>,
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
    Focus(FocusEffect),
    FocusObservation {
        view_id: usize,
        subscription_id: usize,
        callback: FocusObservationCallback,
    },
    ResizeWindow {
        window: AnyWindowHandle,
    },
    MoveWindow {
        window: AnyWindowHandle,
    },
    ActivateWindow {
        window: AnyWindowHandle,
        is_active: bool,
    },
    RepaintWindow {
        window: AnyWindowHandle,
    },
    WindowActivationObservation {
        window: AnyWindowHandle,
        subscription_id: usize,
        callback: WindowActivationCallback,
    },
    FullscreenWindow {
        window: AnyWindowHandle,
        is_fullscreen: bool,
    },
    WindowFullscreenObservation {
        window: AnyWindowHandle,
        subscription_id: usize,
        callback: WindowFullscreenCallback,
    },
    WindowBoundsObservation {
        window: AnyWindowHandle,
        subscription_id: usize,
        callback: WindowBoundsCallback,
    },
    Keystroke {
        window: AnyWindowHandle,
        keystroke: Keystroke,
        handled_by: Option<Box<dyn Action>>,
        result: MatchResult,
    },
    RefreshWindows,
    ActionDispatchNotification {
        action_id: TypeId,
    },
    WindowShouldCloseSubscription {
        window: AnyWindowHandle,
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
            Effect::ViewNotification { window, view_id } => f
                .debug_struct("Effect::ViewNotification")
                .field("window_id", &window.id())
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
            Effect::Focus(focus) => f.debug_tuple("Effect::Focus").field(focus).finish(),
            Effect::FocusObservation {
                view_id,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::FocusObservation")
                .field("view_id", view_id)
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::ActionDispatchNotification { action_id, .. } => f
                .debug_struct("Effect::ActionDispatchNotification")
                .field("action_id", action_id)
                .finish(),
            Effect::ResizeWindow { window } => f
                .debug_struct("Effect::RefreshWindow")
                .field("window_id", &window.id())
                .finish(),
            Effect::MoveWindow { window } => f
                .debug_struct("Effect::MoveWindow")
                .field("window_id", &window.id())
                .finish(),
            Effect::WindowActivationObservation {
                window,
                subscription_id,
                ..
            } => f
                .debug_struct("Effect::WindowActivationObservation")
                .field("window_id", &window.id())
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::ActivateWindow { window, is_active } => f
                .debug_struct("Effect::ActivateWindow")
                .field("window_id", &window.id())
                .field("is_active", is_active)
                .finish(),
            Effect::FullscreenWindow {
                window,
                is_fullscreen,
            } => f
                .debug_struct("Effect::FullscreenWindow")
                .field("window_id", &window.id())
                .field("is_fullscreen", is_fullscreen)
                .finish(),
            Effect::WindowFullscreenObservation {
                window,
                subscription_id,
                callback: _,
            } => f
                .debug_struct("Effect::WindowFullscreenObservation")
                .field("window_id", &window.id())
                .field("subscription_id", subscription_id)
                .finish(),

            Effect::WindowBoundsObservation {
                window,
                subscription_id,
                callback: _,
            } => f
                .debug_struct("Effect::WindowBoundsObservation")
                .field("window_id", &window.id())
                .field("subscription_id", subscription_id)
                .finish(),
            Effect::RefreshWindows => f.debug_struct("Effect::FullViewRefresh").finish(),
            Effect::WindowShouldCloseSubscription { window, .. } => f
                .debug_struct("Effect::WindowShouldCloseSubscription")
                .field("window_id", &window.id())
                .finish(),
            Effect::Keystroke {
                window,
                keystroke,
                handled_by,
                result,
            } => f
                .debug_struct("Effect::Keystroke")
                .field("window_id", &window.id())
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
            Effect::RepaintWindow { window } => f
                .debug_struct("Effect::RepaintWindow")
                .field("window_id", &window.id())
                .finish(),
        }
    }
}

pub trait AnyModel {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn release(&mut self, cx: &mut AppContext);
    fn app_will_quit(
        &mut self,
        cx: &mut AppContext,
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

    fn release(&mut self, cx: &mut AppContext) {
        self.release(cx);
    }

    fn app_will_quit(
        &mut self,
        cx: &mut AppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>> {
        self.app_will_quit(cx)
    }
}

pub trait AnyView {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn release(&mut self, cx: &mut AppContext);
    fn app_will_quit(
        &mut self,
        cx: &mut AppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>>;
    fn ui_name(&self) -> &'static str;
    fn render(&mut self, cx: &mut WindowContext, view_id: usize) -> Box<dyn AnyRootElement>;
    fn focus_in<'a, 'b>(&mut self, focused_id: usize, cx: &mut WindowContext<'a>, view_id: usize);
    fn focus_out(&mut self, focused_id: usize, cx: &mut WindowContext, view_id: usize);
    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut WindowContext, view_id: usize) -> bool;
    fn key_up(&mut self, event: &KeyUpEvent, cx: &mut WindowContext, view_id: usize) -> bool;
    fn modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut WindowContext,
        view_id: usize,
    ) -> bool;
    fn update_keymap_context(&self, keymap: &mut KeymapContext, cx: &AppContext);
    fn debug_json(&self, cx: &WindowContext) -> serde_json::Value;

    fn text_for_range(&self, range: Range<usize>, cx: &WindowContext) -> Option<String>;
    fn selected_text_range(&self, cx: &WindowContext) -> Option<Range<usize>>;
    fn marked_text_range(&self, cx: &WindowContext) -> Option<Range<usize>>;
    fn unmark_text(&mut self, cx: &mut WindowContext, view_id: usize);
    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
        view_id: usize,
    );
    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut WindowContext,
        view_id: usize,
    );
    fn any_handle(
        &self,
        window: AnyWindowHandle,
        view_id: usize,
        cx: &AppContext,
    ) -> AnyViewHandle {
        AnyViewHandle::new(
            window,
            view_id,
            self.as_any().type_id(),
            cx.ref_counts.clone(),
        )
    }
}

impl<V: View> AnyView for V {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn release(&mut self, cx: &mut AppContext) {
        self.release(cx);
    }

    fn app_will_quit(
        &mut self,
        cx: &mut AppContext,
    ) -> Option<Pin<Box<dyn 'static + Future<Output = ()>>>> {
        self.app_will_quit(cx)
    }

    fn ui_name(&self) -> &'static str {
        V::ui_name()
    }

    fn render(&mut self, cx: &mut WindowContext, view_id: usize) -> Box<dyn AnyRootElement> {
        let mut view_context = ViewContext::mutable(cx, view_id);
        let element = V::render(self, &mut view_context);
        let view = WeakViewHandle::new(cx.window_handle, view_id);
        Box::new(RootElement::new(element, view))
    }

    fn focus_in(&mut self, focused_id: usize, cx: &mut WindowContext, view_id: usize) {
        let mut cx = ViewContext::mutable(cx, view_id);
        let focused_view_handle: AnyViewHandle = if view_id == focused_id {
            cx.handle().into_any()
        } else {
            let focused_type = cx
                .views_metadata
                .get(&(cx.window_handle, focused_id))
                .unwrap()
                .type_id;
            AnyViewHandle::new(
                cx.window_handle,
                focused_id,
                focused_type,
                cx.ref_counts.clone(),
            )
        };
        View::focus_in(self, focused_view_handle, &mut cx);
    }

    fn focus_out(&mut self, blurred_id: usize, cx: &mut WindowContext, view_id: usize) {
        let mut cx = ViewContext::mutable(cx, view_id);
        let blurred_view_handle: AnyViewHandle = if view_id == blurred_id {
            cx.handle().into_any()
        } else {
            let blurred_type = cx
                .views_metadata
                .get(&(cx.window_handle, blurred_id))
                .unwrap()
                .type_id;
            AnyViewHandle::new(
                cx.window_handle,
                blurred_id,
                blurred_type,
                cx.ref_counts.clone(),
            )
        };
        View::focus_out(self, blurred_view_handle, &mut cx);
    }

    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut WindowContext, view_id: usize) -> bool {
        let mut cx = ViewContext::mutable(cx, view_id);
        View::key_down(self, event, &mut cx)
    }

    fn key_up(&mut self, event: &KeyUpEvent, cx: &mut WindowContext, view_id: usize) -> bool {
        let mut cx = ViewContext::mutable(cx, view_id);
        View::key_up(self, event, &mut cx)
    }

    fn modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut WindowContext,
        view_id: usize,
    ) -> bool {
        let mut cx = ViewContext::mutable(cx, view_id);
        View::modifiers_changed(self, event, &mut cx)
    }

    fn update_keymap_context(&self, keymap: &mut KeymapContext, cx: &AppContext) {
        View::update_keymap_context(self, keymap, cx)
    }

    fn debug_json(&self, cx: &WindowContext) -> serde_json::Value {
        View::debug_json(self, cx)
    }

    fn text_for_range(&self, range: Range<usize>, cx: &WindowContext) -> Option<String> {
        View::text_for_range(self, range, cx)
    }

    fn selected_text_range(&self, cx: &WindowContext) -> Option<Range<usize>> {
        View::selected_text_range(self, cx)
    }

    fn marked_text_range(&self, cx: &WindowContext) -> Option<Range<usize>> {
        View::marked_text_range(self, cx)
    }

    fn unmark_text(&mut self, cx: &mut WindowContext, view_id: usize) {
        let mut cx = ViewContext::mutable(cx, view_id);
        View::unmark_text(self, &mut cx)
    }

    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
        view_id: usize,
    ) {
        let mut cx = ViewContext::mutable(cx, view_id);
        View::replace_text_in_range(self, range, text, &mut cx)
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut WindowContext,
        view_id: usize,
    ) {
        let mut cx = ViewContext::mutable(cx, view_id);
        View::replace_and_mark_text_in_range(self, range, new_text, new_selected_range, &mut cx)
    }
}

pub struct ModelContext<'a, T: ?Sized> {
    app: &'a mut AppContext,
    model_id: usize,
    model_type: PhantomData<T>,
    halt_stream: bool,
}

impl<'a, T: Entity> ModelContext<'a, T> {
    fn new(app: &'a mut AppContext, model_id: usize) -> Self {
        Self {
            app,
            model_id,
            model_type: PhantomData,
            halt_stream: false,
        }
    }

    pub fn background(&self) -> &Arc<executor::Background> {
        &self.app.background
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
        ModelHandle::new(self.model_id, &self.app.ref_counts)
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
        &self.app
    }
}

impl<M> AsMut<AppContext> for ModelContext<'_, M> {
    fn as_mut(&mut self) -> &mut AppContext {
        self.app
    }
}

impl<M> BorrowAppContext for ModelContext<'_, M> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        self.app.read_with(f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        self.app.update(f)
    }
}

impl<M> Deref for ModelContext<'_, M> {
    type Target = AppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<M> DerefMut for ModelContext<'_, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

pub struct ViewContext<'a, 'b, T: ?Sized> {
    window_context: Reference<'b, WindowContext<'a>>,
    view_id: usize,
    view_type: PhantomData<T>,
}

impl<'a, 'b, V> Deref for ViewContext<'a, 'b, V> {
    type Target = WindowContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.window_context
    }
}

impl<'a, 'b, V> DerefMut for ViewContext<'a, 'b, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window_context
    }
}

impl<'a, 'b, V: 'static> ViewContext<'a, 'b, V> {
    pub fn mutable(window_context: &'b mut WindowContext<'a>, view_id: usize) -> Self {
        Self {
            window_context: Reference::Mutable(window_context),
            view_id,
            view_type: PhantomData,
        }
    }

    pub fn immutable(window_context: &'b WindowContext<'a>, view_id: usize) -> Self {
        Self {
            window_context: Reference::Immutable(window_context),
            view_id,
            view_type: PhantomData,
        }
    }

    pub fn window_context(&mut self) -> &mut WindowContext<'a> {
        &mut self.window_context
    }

    pub fn notify(&mut self) {
        let window = self.window_handle;
        let view_id = self.view_id;
        self.window_context.notify_view(window, view_id);
    }

    pub fn handle(&self) -> ViewHandle<V> {
        ViewHandle::new(
            self.window_handle,
            self.view_id,
            &self.window_context.ref_counts,
        )
    }

    pub fn weak_handle(&self) -> WeakViewHandle<V> {
        WeakViewHandle::new(self.window_handle, self.view_id)
    }

    pub fn window(&self) -> AnyWindowHandle {
        self.window_handle
    }

    pub fn view_id(&self) -> usize {
        self.view_id
    }

    pub fn foreground(&self) -> &Rc<executor::Foreground> {
        self.window_context.foreground()
    }

    pub fn background_executor(&self) -> &Arc<executor::Background> {
        &self.window_context.background
    }

    pub fn platform(&self) -> &Arc<dyn Platform> {
        self.window_context.platform()
    }

    pub fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        self.window_context.prompt_for_paths(options)
    }

    pub fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        self.window_context.prompt_for_new_path(directory)
    }

    pub fn reveal_path(&self, path: &Path) {
        self.window_context.reveal_path(path)
    }

    pub fn focus(&mut self, handle: &AnyViewHandle) {
        self.window_context.focus(Some(handle.view_id));
    }

    pub fn focus_self(&mut self) {
        let view_id = self.view_id;
        self.window_context.focus(Some(view_id));
    }

    pub fn is_self_focused(&self) -> bool {
        self.window.focused_view_id == Some(self.view_id)
    }

    pub fn focus_parent(&mut self) {
        let window = self.window_handle;
        let view_id = self.view_id;
        self.pending_effects
            .push_back(Effect::Focus(FocusEffect::ViewParent {
                window,
                view_id,
                is_forced: false,
            }));
    }

    pub fn blur(&mut self) {
        self.window_context.focus(None);
    }

    pub fn on_window_should_close<F>(&mut self, mut callback: F)
    where
        F: 'static + FnMut(&mut V, &mut ViewContext<V>) -> bool,
    {
        let window = self.window_handle;
        let view = self.weak_handle();
        self.pending_effects
            .push_back(Effect::WindowShouldCloseSubscription {
                window,
                callback: Box::new(move |cx| {
                    cx.update_window(window, |cx| {
                        if let Some(view) = view.upgrade(cx) {
                            view.update(cx, |view, cx| callback(view, cx))
                        } else {
                            true
                        }
                    })
                    .unwrap_or(true)
                }),
            });
    }

    pub fn subscribe<E, H, F>(&mut self, handle: &H, mut callback: F) -> Subscription
    where
        E: Entity,
        E::Event: 'static,
        H: Handle<E>,
        F: 'static + FnMut(&mut V, H, &E::Event, &mut ViewContext<V>),
    {
        let subscriber = self.weak_handle();
        self.window_context
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
        F: 'static + FnMut(&mut V, H, &mut ViewContext<V>),
    {
        let window = self.window_handle;
        let observer = self.weak_handle();
        self.window_context
            .observe_internal(handle, move |observed, cx| {
                cx.update_window(window, |cx| {
                    if let Some(observer) = observer.upgrade(cx) {
                        observer.update(cx, |observer, cx| {
                            callback(observer, observed, cx);
                        });
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
            })
    }

    pub fn observe_global<G, F>(&mut self, mut callback: F) -> Subscription
    where
        G: Any,
        F: 'static + FnMut(&mut V, &mut ViewContext<V>),
    {
        let window = self.window_handle;
        let observer = self.weak_handle();
        self.window_context.observe_global::<G, _>(move |cx| {
            cx.update_window(window, |cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| callback(observer, cx));
                }
            });
        })
    }

    pub fn observe_focus<F, W>(&mut self, handle: &ViewHandle<W>, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut V, ViewHandle<W>, bool, &mut ViewContext<V>),
        W: View,
    {
        let observer = self.weak_handle();
        self.window_context
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
        F: 'static + FnMut(&mut V, &E, &mut ViewContext<V>),
    {
        let window = self.window_handle;
        let observer = self.weak_handle();
        self.window_context
            .observe_release(handle, move |released, cx| {
                cx.update_window(window, |cx| {
                    if let Some(observer) = observer.upgrade(cx) {
                        observer.update(cx, |observer, cx| {
                            callback(observer, released, cx);
                        });
                    }
                });
            })
    }

    pub fn observe_actions<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut V, TypeId, &mut ViewContext<V>),
    {
        let window = self.window_handle;
        let observer = self.weak_handle();
        self.window_context.observe_actions(move |action_id, cx| {
            cx.update_window(window, |cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, action_id, cx);
                    });
                }
            });
        })
    }

    pub fn observe_window_activation<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut V, bool, &mut ViewContext<V>),
    {
        let observer = self.weak_handle();
        self.window_context
            .observe_window_activation(move |active, cx| {
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
        F: 'static + FnMut(&mut V, bool, &mut ViewContext<V>),
    {
        let observer = self.weak_handle();
        self.window_context.observe_fullscreen(move |active, cx| {
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
                &mut V,
                &Keystroke,
                Option<&Box<dyn Action>>,
                &MatchResult,
                &mut ViewContext<V>,
            ) -> bool,
    {
        let observer = self.weak_handle();
        self.window_context
            .observe_keystrokes(move |keystroke, result, handled_by, cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, keystroke, handled_by, result, cx);
                    });
                    true
                } else {
                    false
                }
            })
    }

    pub fn observe_window_bounds<F>(&mut self, mut callback: F) -> Subscription
    where
        F: 'static + FnMut(&mut V, WindowBounds, Uuid, &mut ViewContext<V>),
    {
        let observer = self.weak_handle();
        self.window_context
            .observe_window_bounds(move |bounds, display, cx| {
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
        F: 'static + FnMut(&mut V, &mut ViewContext<V>),
    {
        let window = self.window_handle;
        let observer = self.weak_handle();
        self.window_context.observe_active_labeled_tasks(move |cx| {
            cx.update_window(window, |cx| {
                if let Some(observer) = observer.upgrade(cx) {
                    observer.update(cx, |observer, cx| {
                        callback(observer, cx);
                    });
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false)
        })
    }

    pub fn defer(&mut self, callback: impl 'static + FnOnce(&mut V, &mut ViewContext<V>)) {
        let handle = self.handle();
        self.window_context
            .defer(move |cx| handle.update(cx, |view, cx| callback(view, cx)))
    }

    pub fn after_window_update(
        &mut self,
        callback: impl 'static + FnOnce(&mut V, &mut ViewContext<V>),
    ) {
        let window = self.window_handle;
        let handle = self.handle();
        self.window_context.after_window_update(move |cx| {
            cx.update_window(window, |cx| {
                handle.update(cx, |view, cx| {
                    callback(view, cx);
                })
            });
        })
    }

    pub fn propagate_action(&mut self) {
        self.window_context.halt_action_dispatch = false;
    }

    pub fn spawn_labeled<F, Fut, S>(&mut self, task_label: &'static str, f: F) -> Task<S>
    where
        F: FnOnce(WeakViewHandle<V>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.weak_handle();
        self.window_context
            .spawn_labeled(task_label, |cx| f(handle, cx))
    }

    pub fn spawn<F, Fut, S>(&mut self, f: F) -> Task<S>
    where
        F: FnOnce(WeakViewHandle<V>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.weak_handle();
        self.window_context.spawn(|cx| f(handle, cx))
    }

    pub fn mouse_state<Tag: 'static>(&self, region_id: usize) -> MouseState {
        self.mouse_state_dynamic(TypeTag::new::<Tag>(), region_id)
    }

    pub fn mouse_state_dynamic(&self, tag: TypeTag, region_id: usize) -> MouseState {
        let region_id = MouseRegionId::new(tag, self.view_id, region_id);
        MouseState {
            hovered: self.window.hovered_region_ids.contains(&region_id),
            mouse_down: !self.window.clicked_region_ids.is_empty(),
            clicked: self
                .window
                .clicked_region_ids
                .iter()
                .find(|click_region_id| **click_region_id == region_id)
                // If we've gotten here, there should always be a clicked region.
                // But let's be defensive and return None if there isn't.
                .and_then(|_| self.window.clicked_region.map(|(_, button)| button)),
            accessed_hovered: false,
            accessed_clicked: false,
        }
    }

    pub fn element_state<Tag: 'static, T: 'static>(
        &mut self,
        element_id: usize,
        initial: T,
    ) -> ElementStateHandle<T> {
        self.element_state_dynamic(TypeTag::new::<Tag>(), element_id, initial)
    }

    pub fn element_state_dynamic<T: 'static>(
        &mut self,
        tag: TypeTag,
        element_id: usize,
        initial: T,
    ) -> ElementStateHandle<T> {
        let id = ElementStateId {
            view_id: self.view_id(),
            element_id,
            tag,
        };
        self.element_states
            .entry(id)
            .or_insert_with(|| Box::new(initial));
        ElementStateHandle::new(id, self.frame_count, &self.ref_counts)
    }

    pub fn default_element_state<Tag: 'static, T: 'static + Default>(
        &mut self,
        element_id: usize,
    ) -> ElementStateHandle<T> {
        self.element_state::<Tag, T>(element_id, T::default())
    }

    pub fn rem_pixels(&self) -> f32 {
        16.
    }

    pub fn default_element_state_dynamic<T: 'static + Default>(
        &mut self,
        tag: TypeTag,
        element_id: usize,
    ) -> ElementStateHandle<T> {
        self.element_state_dynamic::<T>(tag, element_id, T::default())
    }
}

impl<V: View> ViewContext<'_, '_, V> {
    pub fn emit(&mut self, event: V::Event) {
        self.window_context
            .pending_effects
            .push_back(Effect::Event {
                entity_id: self.view_id,
                payload: Box::new(event),
            });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeTag {
    tag: TypeId,
    composed: Option<TypeId>,
    #[cfg(debug_assertions)]
    tag_type_name: &'static str,
}

impl TypeTag {
    pub fn new<Tag: 'static>() -> Self {
        Self {
            tag: TypeId::of::<Tag>(),
            composed: None,
            #[cfg(debug_assertions)]
            tag_type_name: std::any::type_name::<Tag>(),
        }
    }

    pub fn dynamic(tag: TypeId, #[cfg(debug_assertions)] type_name: &'static str) -> Self {
        Self {
            tag,
            composed: None,
            #[cfg(debug_assertions)]
            tag_type_name: type_name,
        }
    }

    pub fn compose(mut self, other: TypeTag) -> Self {
        self.composed = Some(other.tag);
        self
    }

    #[cfg(debug_assertions)]
    pub(crate) fn type_name(&self) -> &'static str {
        self.tag_type_name
    }
}

impl<V> BorrowAppContext for ViewContext<'_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.window_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.window_context, f)
    }
}

impl<V> BorrowWindowContext for ViewContext<'_, '_, V> {
    type Result<T> = T;

    fn read_window<T, F: FnOnce(&WindowContext) -> T>(&self, window: AnyWindowHandle, f: F) -> T {
        BorrowWindowContext::read_window(&*self.window_context, window, f)
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        BorrowWindowContext::read_window_optional(&*self.window_context, window, f)
    }

    fn update_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        window: AnyWindowHandle,
        f: F,
    ) -> T {
        BorrowWindowContext::update_window(&mut *self.window_context, window, f)
    }

    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        BorrowWindowContext::update_window_optional(&mut *self.window_context, window, f)
    }
}

/// Methods shared by both LayoutContext and PaintContext
///
/// It's that PaintContext should be implemented in terms of layout context and
/// deref to it, in which case we wouldn't need this.
pub trait RenderContext<'a, 'b, V> {
    fn text_style(&self) -> TextStyle;
    fn push_text_style(&mut self, style: TextStyle);
    fn pop_text_style(&mut self);
    fn as_view_context(&mut self) -> &mut ViewContext<'a, 'b, V>;
}

pub struct LayoutContext<'a, 'b, 'c, V> {
    // Nathan: Making this is public while I work on playground.
    pub view_context: &'c mut ViewContext<'a, 'b, V>,
    new_parents: &'c mut HashMap<usize, usize>,
    views_to_notify_if_ancestors_change: &'c mut HashMap<usize, SmallVec<[usize; 2]>>,
    text_style_stack: Vec<TextStyle>,
    pub refreshing: bool,
}

impl<'a, 'b, 'c, V> LayoutContext<'a, 'b, 'c, V> {
    pub fn new(
        view_context: &'c mut ViewContext<'a, 'b, V>,
        new_parents: &'c mut HashMap<usize, usize>,
        views_to_notify_if_ancestors_change: &'c mut HashMap<usize, SmallVec<[usize; 2]>>,
        refreshing: bool,
    ) -> Self {
        Self {
            view_context,
            new_parents,
            views_to_notify_if_ancestors_change,
            text_style_stack: Vec::new(),
            refreshing,
        }
    }

    pub fn view_context(&mut self) -> &mut ViewContext<'a, 'b, V> {
        self.view_context
    }

    /// Return keystrokes that would dispatch the given action on the given view.
    pub(crate) fn keystrokes_for_action(
        &mut self,
        view_id: usize,
        action: &dyn Action,
    ) -> Option<SmallVec<[Keystroke; 2]>> {
        self.notify_if_view_ancestors_change(view_id);

        let window = self.window_handle;
        let mut contexts = Vec::new();
        let mut handler_depth = None;
        for (i, view_id) in self.ancestors(view_id).enumerate() {
            if let Some(view_metadata) = self.views_metadata.get(&(window, view_id)) {
                if let Some(actions) = self.actions.get(&view_metadata.type_id) {
                    if actions.contains_key(&action.id()) {
                        handler_depth = Some(i);
                    }
                }
                contexts.push(view_metadata.keymap_context.clone());
            }
        }

        if self.global_actions.contains_key(&action.id()) {
            handler_depth = Some(contexts.len())
        }

        let action_contexts = if let Some(depth) = handler_depth {
            &contexts[depth..]
        } else {
            &contexts
        };

        self.keystroke_matcher
            .keystrokes_for_action(action, action_contexts)
    }

    fn notify_if_view_ancestors_change(&mut self, view_id: usize) {
        let self_view_id = self.view_id;
        self.views_to_notify_if_ancestors_change
            .entry(view_id)
            .or_default()
            .push(self_view_id);
    }

    pub fn with_text_style<F, T>(&mut self, style: TextStyle, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.push_text_style(style);
        let result = f(self);
        self.pop_text_style();
        result
    }
}

impl<'a, 'b, 'c, V> RenderContext<'a, 'b, V> for LayoutContext<'a, 'b, 'c, V> {
    fn text_style(&self) -> TextStyle {
        self.text_style_stack
            .last()
            .cloned()
            .unwrap_or(TextStyle::default(&self.font_cache))
    }

    fn push_text_style(&mut self, style: TextStyle) {
        self.text_style_stack.push(style);
    }

    fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    fn as_view_context(&mut self) -> &mut ViewContext<'a, 'b, V> {
        &mut self.view_context
    }
}

impl<'a, 'b, 'c, V> Deref for LayoutContext<'a, 'b, 'c, V> {
    type Target = ViewContext<'a, 'b, V>;

    fn deref(&self) -> &Self::Target {
        &self.view_context
    }
}

impl<V> DerefMut for LayoutContext<'_, '_, '_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view_context
    }
}

impl<V> BorrowAppContext for LayoutContext<'_, '_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.view_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.view_context, f)
    }
}

impl<V> BorrowWindowContext for LayoutContext<'_, '_, '_, V> {
    type Result<T> = T;

    fn read_window<T, F: FnOnce(&WindowContext) -> T>(&self, window: AnyWindowHandle, f: F) -> T {
        BorrowWindowContext::read_window(&*self.view_context, window, f)
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        BorrowWindowContext::read_window_optional(&*self.view_context, window, f)
    }

    fn update_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        window: AnyWindowHandle,
        f: F,
    ) -> T {
        BorrowWindowContext::update_window(&mut *self.view_context, window, f)
    }

    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        BorrowWindowContext::update_window_optional(&mut *self.view_context, window, f)
    }
}

pub struct PaintContext<'a, 'b, 'c, V> {
    pub view_context: &'c mut ViewContext<'a, 'b, V>,
    text_style_stack: Vec<TextStyle>,
}

impl<'a, 'b, 'c, V> PaintContext<'a, 'b, 'c, V> {
    pub fn new(view_context: &'c mut ViewContext<'a, 'b, V>) -> Self {
        Self {
            view_context,
            text_style_stack: Vec::new(),
        }
    }
}

impl<'a, 'b, 'c, V> RenderContext<'a, 'b, V> for PaintContext<'a, 'b, 'c, V> {
    fn text_style(&self) -> TextStyle {
        self.text_style_stack
            .last()
            .cloned()
            .unwrap_or(TextStyle::default(&self.font_cache))
    }

    fn push_text_style(&mut self, style: TextStyle) {
        self.text_style_stack.push(style);
    }

    fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    fn as_view_context(&mut self) -> &mut ViewContext<'a, 'b, V> {
        &mut self.view_context
    }
}

impl<'a, 'b, 'c, V> Deref for PaintContext<'a, 'b, 'c, V> {
    type Target = ViewContext<'a, 'b, V>;

    fn deref(&self) -> &Self::Target {
        &self.view_context
    }
}

impl<V> DerefMut for PaintContext<'_, '_, '_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view_context
    }
}

impl<V> BorrowAppContext for PaintContext<'_, '_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.view_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.view_context, f)
    }
}

impl<V> BorrowWindowContext for PaintContext<'_, '_, '_, V> {
    type Result<T> = T;

    fn read_window<T, F>(&self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&WindowContext) -> T,
    {
        BorrowWindowContext::read_window(self.view_context, window, f)
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        BorrowWindowContext::read_window_optional(self.view_context, window, f)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Self::Result<T>
    where
        F: FnOnce(&mut WindowContext) -> T,
    {
        BorrowWindowContext::update_window(self.view_context, window, f)
    }

    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        BorrowWindowContext::update_window_optional(self.view_context, window, f)
    }
}

pub struct EventContext<'a, 'b, 'c, V> {
    view_context: &'c mut ViewContext<'a, 'b, V>,
    pub(crate) handled: bool,
    // I would like to replace handled with this.
    // Being additive for now.
    pub bubble: bool,
}

impl<'a, 'b, 'c, V: 'static> EventContext<'a, 'b, 'c, V> {
    pub fn new(view_context: &'c mut ViewContext<'a, 'b, V>) -> Self {
        EventContext {
            view_context,
            handled: true,
            bubble: false,
        }
    }

    pub fn propagate_event(&mut self) {
        self.handled = false;
    }

    pub fn bubble_event(&mut self) {
        self.bubble = true;
    }

    pub fn event_bubbled(&self) -> bool {
        self.bubble
    }
}

impl<'a, 'b, 'c, V> Deref for EventContext<'a, 'b, 'c, V> {
    type Target = ViewContext<'a, 'b, V>;

    fn deref(&self) -> &Self::Target {
        &self.view_context
    }
}

impl<V> DerefMut for EventContext<'_, '_, '_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view_context
    }
}

impl<V> BorrowAppContext for EventContext<'_, '_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.view_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.view_context, f)
    }
}

impl<V> BorrowWindowContext for EventContext<'_, '_, '_, V> {
    type Result<T> = T;

    fn read_window<T, F: FnOnce(&WindowContext) -> T>(&self, window: AnyWindowHandle, f: F) -> T {
        BorrowWindowContext::read_window(&*self.view_context, window, f)
    }

    fn read_window_optional<T, F>(&self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&WindowContext) -> Option<T>,
    {
        BorrowWindowContext::read_window_optional(&*self.view_context, window, f)
    }

    fn update_window<T, F: FnOnce(&mut WindowContext) -> T>(
        &mut self,
        window: AnyWindowHandle,
        f: F,
    ) -> T {
        BorrowWindowContext::update_window(&mut *self.view_context, window, f)
    }

    fn update_window_optional<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Option<T>
    where
        F: FnOnce(&mut WindowContext) -> Option<T>,
    {
        BorrowWindowContext::update_window_optional(&mut *self.view_context, window, f)
    }
}

pub(crate) enum Reference<'a, T> {
    Immutable(&'a T),
    Mutable(&'a mut T),
}

impl<'a, T> Deref for Reference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Reference::Immutable(target) => target,
            Reference::Mutable(target) => target,
        }
    }
}

impl<'a, T> DerefMut for Reference<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Reference::Immutable(_) => {
                panic!("cannot mutably deref an immutable reference. this is a bug in GPUI.");
            }
            Reference::Mutable(target) => target,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MouseState {
    pub(crate) hovered: bool,
    pub(crate) clicked: Option<MouseButton>,
    pub(crate) mouse_down: bool,
    pub(crate) accessed_hovered: bool,
    pub(crate) accessed_clicked: bool,
}

impl MouseState {
    pub fn dragging(&mut self) -> bool {
        self.accessed_hovered = true;
        self.hovered && self.mouse_down
    }

    pub fn hovered(&mut self) -> bool {
        self.accessed_hovered = true;
        self.hovered && (!self.mouse_down || self.clicked.is_some())
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
    any_handle: AnyModelHandle,
    model_type: PhantomData<T>,
}

impl<T: Entity> Deref for ModelHandle<T> {
    type Target = AnyModelHandle;

    fn deref(&self) -> &Self::Target {
        &self.any_handle
    }
}

impl<T: Entity> ModelHandle<T> {
    fn new(model_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        Self {
            any_handle: AnyModelHandle::new(model_id, TypeId::of::<T>(), ref_counts.clone()),
            model_type: PhantomData,
        }
    }

    pub fn downgrade(&self) -> WeakModelHandle<T> {
        WeakModelHandle::new(self.model_id)
    }

    pub fn id(&self) -> usize {
        self.model_id
    }

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a T {
        cx.read_model(self)
    }

    pub fn read_with<C, F, S>(&self, cx: &C, read: F) -> S
    where
        C: BorrowAppContext,
        F: FnOnce(&T, &AppContext) -> S,
    {
        cx.read_with(|cx| read(self.read(cx), cx))
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> S
    where
        C: BorrowAppContext,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        let mut update = Some(update);
        cx.update(|cx| {
            cx.update_model(self, &mut |model, cx| {
                let update = update.take().unwrap();
                update(model, cx)
            })
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
    any_handle: AnyWeakModelHandle,
    model_type: PhantomData<T>,
}

impl<T> WeakModelHandle<T> {
    pub fn into_any(self) -> AnyWeakModelHandle {
        self.any_handle
    }
}

impl<T> Deref for WeakModelHandle<T> {
    type Target = AnyWeakModelHandle;

    fn deref(&self) -> &Self::Target {
        &self.any_handle
    }
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
            any_handle: AnyWeakModelHandle {
                model_id,
                model_type: TypeId::of::<T>(),
            },
            model_type: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.model_id
    }

    pub fn is_upgradable(&self, cx: &impl BorrowAppContext) -> bool {
        cx.read_with(|cx| cx.model_handle_is_upgradable(self))
    }

    pub fn upgrade(&self, cx: &impl BorrowAppContext) -> Option<ModelHandle<T>> {
        cx.read_with(|cx| cx.upgrade_model_handle(self))
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
            any_handle: self.any_handle.clone(),
            model_type: PhantomData,
        }
    }
}

impl<T> Copy for WeakModelHandle<T> {}

#[derive(Deref)]
pub struct WindowHandle<V> {
    #[deref]
    any_handle: AnyWindowHandle,
    root_view_type: PhantomData<V>,
}

impl<V> Clone for WindowHandle<V> {
    fn clone(&self) -> Self {
        Self {
            any_handle: self.any_handle.clone(),
            root_view_type: PhantomData,
        }
    }
}

impl<V> Copy for WindowHandle<V> {}

impl<V: 'static> WindowHandle<V> {
    fn new(window_id: usize) -> Self {
        WindowHandle {
            any_handle: AnyWindowHandle::new(window_id, TypeId::of::<V>()),
            root_view_type: PhantomData,
        }
    }

    pub fn root<C: BorrowWindowContext>(&self, cx: &C) -> C::Result<ViewHandle<V>> {
        self.read_with(cx, |cx| cx.root_view().clone().downcast().unwrap())
    }

    pub fn read_root_with<C, F, R>(&self, cx: &C, read: F) -> C::Result<R>
    where
        C: BorrowWindowContext,
        F: FnOnce(&V, &ViewContext<V>) -> R,
    {
        self.read_with(cx, |cx| {
            cx.root_view()
                .downcast_ref::<V>()
                .unwrap()
                .read_with(cx, read)
        })
    }

    pub fn update_root<C, F, R>(&self, cx: &mut C, update: F) -> C::Result<R>
    where
        C: BorrowWindowContext,
        F: FnOnce(&mut V, &mut ViewContext<V>) -> R,
    {
        cx.update_window(self.any_handle, |cx| {
            cx.root_view()
                .clone()
                .downcast::<V>()
                .unwrap()
                .update(cx, update)
        })
    }
}

impl<V: View> WindowHandle<V> {
    pub fn replace_root<C, F>(&self, cx: &mut C, build_root: F) -> C::Result<ViewHandle<V>>
    where
        C: BorrowWindowContext,
        F: FnOnce(&mut ViewContext<V>) -> V,
    {
        cx.update_window(self.any_handle, |cx| {
            let root_view = self.add_view(cx, |cx| build_root(cx));
            cx.window.root_view = Some(root_view.clone().into_any());
            cx.window.focused_view_id = Some(root_view.id());
            root_view
        })
    }
}

impl<V> Into<AnyWindowHandle> for WindowHandle<V> {
    fn into(self) -> AnyWindowHandle {
        self.any_handle
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnyWindowHandle {
    window_id: usize,
    root_view_type: TypeId,
}

impl AnyWindowHandle {
    fn new(window_id: usize, root_view_type: TypeId) -> Self {
        Self {
            window_id,
            root_view_type,
        }
    }

    pub fn id(&self) -> usize {
        self.window_id
    }

    pub fn read_with<C, F, R>(&self, cx: &C, read: F) -> C::Result<R>
    where
        C: BorrowWindowContext,
        F: FnOnce(&WindowContext) -> R,
    {
        cx.read_window(*self, |cx| read(cx))
    }

    pub fn read_optional_with<C, F, R>(&self, cx: &C, read: F) -> Option<R>
    where
        C: BorrowWindowContext,
        F: FnOnce(&WindowContext) -> Option<R>,
    {
        cx.read_window_optional(*self, |cx| read(cx))
    }

    pub fn update<C, F, R>(&self, cx: &mut C, update: F) -> C::Result<R>
    where
        C: BorrowWindowContext,
        F: FnOnce(&mut WindowContext) -> R,
    {
        cx.update_window(*self, update)
    }

    pub fn update_optional<C, F, R>(&self, cx: &mut C, update: F) -> Option<R>
    where
        C: BorrowWindowContext,
        F: FnOnce(&mut WindowContext) -> Option<R>,
    {
        cx.update_window_optional(*self, update)
    }

    pub fn add_view<C, U, F>(&self, cx: &mut C, build_view: F) -> C::Result<ViewHandle<U>>
    where
        C: BorrowWindowContext,
        U: View,
        F: FnOnce(&mut ViewContext<U>) -> U,
    {
        self.update(cx, |cx| cx.add_view(build_view))
    }

    pub fn downcast<V: 'static>(self) -> Option<WindowHandle<V>> {
        if self.root_view_type == TypeId::of::<V>() {
            Some(WindowHandle {
                any_handle: self,
                root_view_type: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn root_is<V: 'static>(&self) -> bool {
        self.root_view_type == TypeId::of::<V>()
    }

    pub fn is_active<C: BorrowWindowContext>(&self, cx: &C) -> C::Result<bool> {
        self.read_with(cx, |cx| cx.window.is_active)
    }

    pub fn remove<C: BorrowWindowContext>(&self, cx: &mut C) -> C::Result<()> {
        self.update(cx, |cx| cx.remove_window())
    }

    pub fn debug_elements<C: BorrowWindowContext>(&self, cx: &C) -> Option<json::Value> {
        self.read_optional_with(cx, |cx| {
            let root_view = cx.window.root_view();
            let root_element = cx.window.rendered_views.get(&root_view.id())?;
            root_element.debug(cx).log_err()
        })
    }

    pub fn activate<C: BorrowWindowContext>(&mut self, cx: &mut C) -> C::Result<()> {
        self.update(cx, |cx| cx.activate_window())
    }

    pub fn prompt<C: BorrowWindowContext>(
        &self,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
        cx: &mut C,
    ) -> C::Result<oneshot::Receiver<usize>> {
        self.update(cx, |cx| cx.prompt(level, msg, answers))
    }

    pub fn dispatch_action<C: BorrowWindowContext>(
        &self,
        view_id: usize,
        action: &dyn Action,
        cx: &mut C,
    ) -> C::Result<()> {
        self.update(cx, |cx| {
            cx.dispatch_action(Some(view_id), action);
        })
    }

    pub fn available_actions<C: BorrowWindowContext>(
        &self,
        view_id: usize,
        cx: &C,
    ) -> C::Result<Vec<(&'static str, Box<dyn Action>, SmallVec<[Binding; 1]>)>> {
        self.read_with(cx, |cx| cx.available_actions(view_id))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_activation(&self, cx: &mut TestAppContext) {
        self.update(cx, |cx| {
            let other_windows = cx
                .windows()
                .filter(|window| *window != *self)
                .collect::<Vec<_>>();

            for window in other_windows {
                cx.window_changed_active_status(window, false)
            }

            cx.window_changed_active_status(*self, true)
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_deactivation(&self, cx: &mut TestAppContext) {
        self.update(cx, |cx| {
            cx.window_changed_active_status(*self, false);
        })
    }
}

#[repr(transparent)]
pub struct ViewHandle<V> {
    any_handle: AnyViewHandle,
    view_type: PhantomData<V>,
}

impl<T> Deref for ViewHandle<T> {
    type Target = AnyViewHandle;

    fn deref(&self) -> &Self::Target {
        &self.any_handle
    }
}

impl<V: 'static> ViewHandle<V> {
    fn new(window: AnyWindowHandle, view_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        Self {
            any_handle: AnyViewHandle::new(window, view_id, TypeId::of::<V>(), ref_counts.clone()),
            view_type: PhantomData,
        }
    }

    pub fn downgrade(&self) -> WeakViewHandle<V> {
        WeakViewHandle::new(self.window, self.view_id)
    }

    pub fn into_any(self) -> AnyViewHandle {
        self.any_handle
    }

    pub fn window(&self) -> AnyWindowHandle {
        self.window
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a V {
        cx.read_view(self)
    }

    pub fn read_with<C, F, S>(&self, cx: &C, read: F) -> C::Result<S>
    where
        C: BorrowWindowContext,
        F: FnOnce(&V, &ViewContext<V>) -> S,
    {
        cx.read_window(self.window, |cx| {
            let cx = ViewContext::immutable(cx, self.view_id);
            read(cx.read_view(self), &cx)
        })
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> C::Result<S>
    where
        C: BorrowWindowContext,
        F: FnOnce(&mut V, &mut ViewContext<V>) -> S,
    {
        let mut update = Some(update);

        cx.update_window(self.window, |cx| {
            cx.update_view(self, &mut |view, cx| {
                let update = update.take().unwrap();
                update(view, cx)
            })
        })
    }

    pub fn is_focused(&self, cx: &WindowContext) -> bool {
        cx.focused_view_id() == Some(self.view_id)
    }
}

impl<T: View> Clone for ViewHandle<T> {
    fn clone(&self) -> Self {
        ViewHandle::new(self.window, self.view_id, &self.ref_counts)
    }
}

impl<T> PartialEq for ViewHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl<T> PartialEq<AnyViewHandle> for ViewHandle<T> {
    fn eq(&self, other: &AnyViewHandle) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl<T> PartialEq<WeakViewHandle<T>> for ViewHandle<T> {
    fn eq(&self, other: &WeakViewHandle<T>) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl<T> PartialEq<ViewHandle<T>> for WeakViewHandle<T> {
    fn eq(&self, other: &ViewHandle<T>) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl<T> Eq for ViewHandle<T> {}

impl<T> Hash for ViewHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.window.hash(state);
        self.view_id.hash(state);
    }
}

impl<T> Debug for ViewHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("ViewHandle<{}>", type_name::<T>()))
            .field("window_id", &self.window)
            .field("view_id", &self.view_id)
            .finish()
    }
}

impl<T: View> Handle<T> for ViewHandle<T> {
    type Weak = WeakViewHandle<T>;

    fn id(&self) -> usize {
        self.view_id
    }

    fn location(&self) -> EntityLocation {
        EntityLocation::View(self.window.id(), self.view_id)
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
    window: AnyWindowHandle,
    view_id: usize,
    view_type: TypeId,
    ref_counts: Arc<Mutex<RefCounts>>,

    #[cfg(any(test, feature = "test-support"))]
    handle_id: usize,
}

impl AnyViewHandle {
    fn new(
        window: AnyWindowHandle,
        view_id: usize,
        view_type: TypeId,
        ref_counts: Arc<Mutex<RefCounts>>,
    ) -> Self {
        ref_counts.lock().inc_view(window, view_id);

        #[cfg(any(test, feature = "test-support"))]
        let handle_id = ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_created(None, view_id);

        Self {
            window,
            view_id,
            view_type,
            ref_counts,
            #[cfg(any(test, feature = "test-support"))]
            handle_id,
        }
    }

    pub fn window(&self) -> AnyWindowHandle {
        self.window
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.view_type
    }

    pub fn downcast<V: 'static>(self) -> Option<ViewHandle<V>> {
        if self.is::<V>() {
            Some(ViewHandle {
                any_handle: self,
                view_type: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn downcast_ref<V: 'static>(&self) -> Option<&ViewHandle<V>> {
        if self.is::<V>() {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }

    pub fn downgrade(&self) -> AnyWeakViewHandle {
        AnyWeakViewHandle {
            window: self.window,
            view_id: self.view_id,
            view_type: self.view_type,
        }
    }

    pub fn view_type(&self) -> TypeId {
        self.view_type
    }

    pub fn debug_json<'a, 'b>(&self, cx: &'b WindowContext<'a>) -> serde_json::Value {
        cx.views
            .get(&(self.window, self.view_id))
            .map_or_else(|| serde_json::Value::Null, |view| view.debug_json(cx))
    }
}

impl Clone for AnyViewHandle {
    fn clone(&self) -> Self {
        Self::new(
            self.window,
            self.view_id,
            self.view_type,
            self.ref_counts.clone(),
        )
    }
}

impl PartialEq for AnyViewHandle {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl<T> PartialEq<ViewHandle<T>> for AnyViewHandle {
    fn eq(&self, other: &ViewHandle<T>) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl Drop for AnyViewHandle {
    fn drop(&mut self) {
        self.ref_counts.lock().dec_view(self.window, self.view_id);
        #[cfg(any(test, feature = "test-support"))]
        self.ref_counts
            .lock()
            .leak_detector
            .lock()
            .handle_dropped(self.view_id, self.handle_id);
    }
}

impl Debug for AnyViewHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyViewHandle")
            .field("window_id", &self.window.id())
            .field("view_id", &self.view_id)
            .finish()
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
            Some(ModelHandle {
                any_handle: self,
                model_type: PhantomData,
            })
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

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct AnyWeakModelHandle {
    model_id: usize,
    model_type: TypeId,
}

impl AnyWeakModelHandle {
    pub fn upgrade(&self, cx: &impl BorrowAppContext) -> Option<AnyModelHandle> {
        cx.read_with(|cx| cx.upgrade_any_model_handle(self))
    }

    pub fn model_type(&self) -> TypeId {
        self.model_type
    }

    fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.model_type
    }

    pub fn downcast<T: Entity>(self) -> Option<WeakModelHandle<T>> {
        if self.is::<T>() {
            let result = Some(WeakModelHandle {
                any_handle: self,
                model_type: PhantomData,
            });

            result
        } else {
            None
        }
    }
}

pub struct WeakViewHandle<T> {
    any_handle: AnyWeakViewHandle,
    view_type: PhantomData<T>,
}

impl<T> Copy for WeakViewHandle<T> {}

impl<T> Debug for WeakViewHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("WeakViewHandle<{}>", type_name::<T>()))
            .field("any_handle", &self.any_handle)
            .finish()
    }
}

impl<T> WeakHandle for WeakViewHandle<T> {
    fn id(&self) -> usize {
        self.view_id
    }
}

impl<V: 'static> WeakViewHandle<V> {
    fn new(window: AnyWindowHandle, view_id: usize) -> Self {
        Self {
            any_handle: AnyWeakViewHandle {
                window,
                view_id,
                view_type: TypeId::of::<V>(),
            },
            view_type: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn window(&self) -> AnyWindowHandle {
        self.window
    }

    pub fn window_id(&self) -> usize {
        self.window.id()
    }

    pub fn into_any(self) -> AnyWeakViewHandle {
        self.any_handle
    }

    pub fn upgrade(&self, cx: &impl BorrowAppContext) -> Option<ViewHandle<V>> {
        cx.read_with(|cx| cx.upgrade_view_handle(self))
    }

    pub fn read_with<T>(
        &self,
        cx: &AsyncAppContext,
        read: impl FnOnce(&V, &ViewContext<V>) -> T,
    ) -> Result<T> {
        cx.read(|cx| {
            let handle = cx
                .upgrade_view_handle(self)
                .ok_or_else(|| anyhow!("view was dropped"))?;
            cx.read_window(self.window, |cx| handle.read_with(cx, read))
                .ok_or_else(|| anyhow!("window was removed"))
        })
    }

    pub fn update<T, B>(
        &self,
        cx: &mut B,
        update: impl FnOnce(&mut V, &mut ViewContext<V>) -> T,
    ) -> Result<T>
    where
        B: BorrowWindowContext,
        B::Result<Option<T>>: Flatten<T>,
    {
        cx.update_window(self.window(), |cx| {
            cx.upgrade_view_handle(self)
                .map(|handle| handle.update(cx, update))
        })
        .flatten()
        .ok_or_else(|| anyhow!("window was removed"))
    }
}

pub trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        self.flatten()
    }
}

impl<T> Flatten<T> for Option<T> {
    fn flatten(self) -> Option<T> {
        self
    }
}

impl<V> Deref for WeakViewHandle<V> {
    type Target = AnyWeakViewHandle;

    fn deref(&self) -> &Self::Target {
        &self.any_handle
    }
}

impl<V> Clone for WeakViewHandle<V> {
    fn clone(&self) -> Self {
        Self {
            any_handle: self.any_handle.clone(),
            view_type: PhantomData,
        }
    }
}

impl<T> PartialEq for WeakViewHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window && self.view_id == other.view_id
    }
}

impl<T> Eq for WeakViewHandle<T> {}

impl<T> Hash for WeakViewHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_handle.hash(state);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnyWeakViewHandle {
    window: AnyWindowHandle,
    view_id: usize,
    view_type: TypeId,
}

impl AnyWeakViewHandle {
    pub fn id(&self) -> usize {
        self.view_id
    }

    fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.view_type
    }

    pub fn upgrade(&self, cx: &impl BorrowAppContext) -> Option<AnyViewHandle> {
        cx.read_with(|cx| cx.upgrade_any_view_handle(self))
    }

    pub fn downcast<T: View>(self) -> Option<WeakViewHandle<T>> {
        if self.is::<T>() {
            Some(WeakViewHandle {
                any_handle: self,
                view_type: PhantomData,
            })
        } else {
            None
        }
    }
}

impl Hash for AnyWeakViewHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.window.hash(state);
        self.view_id.hash(state);
        self.view_type.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ElementStateId {
    view_id: usize,
    element_id: usize,
    tag: TypeTag,
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

    pub fn update<C, D, R>(&self, cx: &mut C, f: impl FnOnce(&mut T, &mut C) -> R) -> R
    where
        C: DerefMut<Target = D>,
        D: DerefMut<Target = AppContext>,
    {
        let mut element_state = cx.deref_mut().element_states.remove(&self.id).unwrap();
        let result = f(element_state.downcast_mut().unwrap(), cx);
        cx.deref_mut().element_states.insert(self.id, element_state);
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
    WindowActivationObservation(
        callback_collection::Subscription<AnyWindowHandle, WindowActivationCallback>,
    ),
    WindowFullscreenObservation(
        callback_collection::Subscription<AnyWindowHandle, WindowFullscreenCallback>,
    ),
    WindowBoundsObservation(
        callback_collection::Subscription<AnyWindowHandle, WindowBoundsCallback>,
    ),
    KeystrokeObservation(callback_collection::Subscription<AnyWindowHandle, KeystrokeCallback>),
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
