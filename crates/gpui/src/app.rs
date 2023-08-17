pub mod action;
mod callback_collection;
mod menu;
pub(crate) mod ref_counts;
#[cfg(any(test, feature = "test-support"))]
pub mod test_app_context;
pub(crate) mod window;
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

use derive_more::Deref;
use parking_lot::Mutex;
use postage::oneshot;
use smallvec::SmallVec;
use smol::prelude::*;
use util::ResultExt;
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

use self::ref_counts::RefCounts;

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
    fn ui_name() -> &'static str;
    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self>;
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
    pub fn new(asset_source: impl AssetSource) -> Result<Self> {
        let platform = platform::current::platform();
        let foreground = Rc::new(executor::Foreground::platform(platform.dispatcher())?);
        let foreground_platform = platform::current::foreground_platform(foreground.clone());
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
        F: 'static + FnMut(ViewHandle<V>, bool, &mut WindowContext) -> bool,
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

    pub fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        if let Some(view) = self.views.get(&(handle.window, handle.view_id)) {
            view.as_any().downcast_ref().expect("downcast is type safe")
        } else {
            panic!("circular view reference for type {}", type_name::<T>());
        }
    }

    fn upgrade_view_handle<T: View>(&self, handle: &WeakViewHandle<T>) -> Option<ViewHandle<T>> {
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

#[derive(Default, Clone)]
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

impl<V> AnyView for V
where
    V: View,
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

impl<'a, 'b, T: View> Deref for ViewContext<'a, 'b, T> {
    type Target = WindowContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.window_context
    }
}

impl<T: View> DerefMut for ViewContext<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window_context
    }
}

impl<'a, 'b, V: View> ViewContext<'a, 'b, V> {
    pub(crate) fn mutable(window_context: &'b mut WindowContext<'a>, view_id: usize) -> Self {
        Self {
            window_context: Reference::Mutable(window_context),
            view_id,
            view_type: PhantomData,
        }
    }

    pub(crate) fn immutable(window_context: &'b WindowContext<'a>, view_id: usize) -> Self {
        Self {
            window_context: Reference::Immutable(window_context),
            view_id,
            view_type: PhantomData,
        }
    }

    pub fn window_context(&mut self) -> &mut WindowContext<'a> {
        &mut self.window_context
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

    pub fn emit(&mut self, payload: V::Event) {
        self.window_context
            .pending_effects
            .push_back(Effect::Event {
                entity_id: self.view_id,
                payload: Box::new(payload),
            });
    }

    pub fn notify(&mut self) {
        let window = self.window_handle;
        let view_id = self.view_id;
        self.window_context.notify_view(window, view_id);
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
            clicked: if let Some((clicked_region_id, button)) = self.window.clicked_region {
                if region_id == clicked_region_id {
                    Some(button)
                } else {
                    None
                }
            } else {
                None
            },
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeTag {
    tag: TypeId,
    #[cfg(debug_assertions)]
    tag_type_name: &'static str,
}

impl TypeTag {
    pub fn new<Tag: 'static>() -> Self {
        Self {
            tag: TypeId::of::<Tag>(),
            #[cfg(debug_assertions)]
            tag_type_name: std::any::type_name::<Tag>(),
        }
    }

    pub fn dynamic(tag: TypeId, #[cfg(debug_assertions)] type_name: &'static str) -> Self {
        Self {
            tag,
            #[cfg(debug_assertions)]
            tag_type_name: type_name,
        }
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

pub struct LayoutContext<'a, 'b, 'c, V: View> {
    view_context: &'c mut ViewContext<'a, 'b, V>,
    new_parents: &'c mut HashMap<usize, usize>,
    views_to_notify_if_ancestors_change: &'c mut HashMap<usize, SmallVec<[usize; 2]>>,
    text_style_stack: Vec<Arc<TextStyle>>,
    pub refreshing: bool,
}

impl<'a, 'b, 'c, V: View> LayoutContext<'a, 'b, 'c, V> {
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

    pub fn text_style(&self) -> Arc<TextStyle> {
        self.text_style_stack
            .last()
            .cloned()
            .unwrap_or(Default::default())
    }

    pub fn with_text_style<S, F, T>(&mut self, style: S, f: F) -> T
    where
        S: Into<Arc<TextStyle>>,
        F: FnOnce(&mut Self) -> T,
    {
        self.text_style_stack.push(style.into());
        let result = f(self);
        self.text_style_stack.pop();
        result
    }
}

impl<'a, 'b, 'c, V: View> Deref for LayoutContext<'a, 'b, 'c, V> {
    type Target = ViewContext<'a, 'b, V>;

    fn deref(&self) -> &Self::Target {
        &self.view_context
    }
}

impl<V: View> DerefMut for LayoutContext<'_, '_, '_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view_context
    }
}

impl<V: View> BorrowAppContext for LayoutContext<'_, '_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.view_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.view_context, f)
    }
}

impl<V: View> BorrowWindowContext for LayoutContext<'_, '_, '_, V> {
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

pub struct PaintContext<'a, 'b, 'c, V: View> {
    view_context: &'c mut ViewContext<'a, 'b, V>,
    text_style_stack: Vec<Arc<TextStyle>>,
}

impl<'a, 'b, 'c, V: View> PaintContext<'a, 'b, 'c, V> {
    pub fn new(view_context: &'c mut ViewContext<'a, 'b, V>) -> Self {
        Self {
            view_context,
            text_style_stack: Vec::new(),
        }
    }

    pub fn text_style(&self) -> Arc<TextStyle> {
        self.text_style_stack
            .last()
            .cloned()
            .unwrap_or(Default::default())
    }

    pub fn with_text_style<S, F, T>(&mut self, style: S, f: F) -> T
    where
        S: Into<Arc<TextStyle>>,
        F: FnOnce(&mut Self) -> T,
    {
        self.text_style_stack.push(style.into());
        let result = f(self);
        self.text_style_stack.pop();
        result
    }
}

impl<'a, 'b, 'c, V: View> Deref for PaintContext<'a, 'b, 'c, V> {
    type Target = ViewContext<'a, 'b, V>;

    fn deref(&self) -> &Self::Target {
        &self.view_context
    }
}

impl<V: View> DerefMut for PaintContext<'_, '_, '_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view_context
    }
}

impl<V: View> BorrowAppContext for PaintContext<'_, '_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.view_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.view_context, f)
    }
}

impl<V: View> BorrowWindowContext for PaintContext<'_, '_, '_, V> {
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

pub struct EventContext<'a, 'b, 'c, V: View> {
    view_context: &'c mut ViewContext<'a, 'b, V>,
    pub(crate) handled: bool,
}

impl<'a, 'b, 'c, V: View> EventContext<'a, 'b, 'c, V> {
    pub(crate) fn new(view_context: &'c mut ViewContext<'a, 'b, V>) -> Self {
        EventContext {
            view_context,
            handled: true,
        }
    }

    pub fn propagate_event(&mut self) {
        self.handled = false;
    }
}

impl<'a, 'b, 'c, V: View> Deref for EventContext<'a, 'b, 'c, V> {
    type Target = ViewContext<'a, 'b, V>;

    fn deref(&self) -> &Self::Target {
        &self.view_context
    }
}

impl<V: View> DerefMut for EventContext<'_, '_, '_, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view_context
    }
}

impl<V: View> BorrowAppContext for EventContext<'_, '_, '_, V> {
    fn read_with<T, F: FnOnce(&AppContext) -> T>(&self, f: F) -> T {
        BorrowAppContext::read_with(&*self.view_context, f)
    }

    fn update<T, F: FnOnce(&mut AppContext) -> T>(&mut self, f: F) -> T {
        BorrowAppContext::update(&mut *self.view_context, f)
    }
}

impl<V: View> BorrowWindowContext for EventContext<'_, '_, '_, V> {
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

impl<V: View> WindowHandle<V> {
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

    pub fn downcast<V: View>(self) -> Option<WindowHandle<V>> {
        if self.root_view_type == TypeId::of::<V>() {
            Some(WindowHandle {
                any_handle: self,
                root_view_type: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn root_is<V: View>(&self) -> bool {
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
pub struct ViewHandle<T> {
    any_handle: AnyViewHandle,
    view_type: PhantomData<T>,
}

impl<T> Deref for ViewHandle<T> {
    type Target = AnyViewHandle;

    fn deref(&self) -> &Self::Target {
        &self.any_handle
    }
}

impl<T: View> ViewHandle<T> {
    fn new(window: AnyWindowHandle, view_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        Self {
            any_handle: AnyViewHandle::new(window, view_id, TypeId::of::<T>(), ref_counts.clone()),
            view_type: PhantomData,
        }
    }

    pub fn downgrade(&self) -> WeakViewHandle<T> {
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

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a T {
        cx.read_view(self)
    }

    pub fn read_with<C, F, S>(&self, cx: &C, read: F) -> C::Result<S>
    where
        C: BorrowWindowContext,
        F: FnOnce(&T, &ViewContext<T>) -> S,
    {
        cx.read_window(self.window, |cx| {
            let cx = ViewContext::immutable(cx, self.view_id);
            read(cx.read_view(self), &cx)
        })
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> C::Result<S>
    where
        C: BorrowWindowContext,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
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

    pub fn downcast<T: View>(self) -> Option<ViewHandle<T>> {
        if self.is::<T>() {
            Some(ViewHandle {
                any_handle: self,
                view_type: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn downcast_ref<T: View>(&self) -> Option<&ViewHandle<T>> {
        if self.is::<T>() {
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

#[derive(Copy)]
pub struct WeakViewHandle<T> {
    any_handle: AnyWeakViewHandle,
    view_type: PhantomData<T>,
}

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

impl<V: View> WeakViewHandle<V> {
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
                .ok_or_else(|| anyhow!("view {} was dropped", V::ui_name()))?;
            cx.read_window(self.window, |cx| handle.read_with(cx, read))
                .ok_or_else(|| anyhow!("window was removed"))
        })
    }

    pub fn update<T>(
        &self,
        cx: &mut AsyncAppContext,
        update: impl FnOnce(&mut V, &mut ViewContext<V>) -> T,
    ) -> Result<T> {
        cx.update(|cx| {
            let handle = cx
                .upgrade_view_handle(self)
                .ok_or_else(|| anyhow!("view {} was dropped", V::ui_name()))?;
            cx.update_window(self.window, |cx| handle.update(cx, update))
                .ok_or_else(|| anyhow!("window was removed"))
        })
    }
}

impl<T> Deref for WeakViewHandle<T> {
    type Target = AnyWeakViewHandle;

    fn deref(&self) -> &Self::Target {
        &self.any_handle
    }
}

impl<T> Clone for WeakViewHandle<T> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions,
        elements::*,
        impl_actions,
        platform::{MouseButton, MouseButtonEvent},
        window::ChildView,
    };
    use itertools::Itertools;
    use postage::{sink::Sink, stream::Stream};
    use serde::Deserialize;
    use smol::future::poll_once;
    use std::{
        cell::Cell,
        sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst},
    };

    #[crate::test(self)]
    fn test_model_handles(cx: &mut AppContext) {
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
        assert_eq!(cx.models.len(), 2);

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

        assert_eq!(cx.models.len(), 1);
        assert!(cx.subscriptions.is_empty());
        assert!(cx.observations.is_empty());
    }

    #[crate::test(self)]
    fn test_model_events(cx: &mut AppContext) {
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
    fn test_model_emit_before_subscribe_in_same_update_cycle(cx: &mut AppContext) {
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
    fn test_observe_and_notify_from_model(cx: &mut AppContext) {
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
    fn test_model_notify_before_observe_in_same_update_cycle(cx: &mut AppContext) {
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
    fn test_defer_and_after_window_update(cx: &mut TestAppContext) {
        struct View {
            render_count: usize,
        }

        impl Entity for View {
            type Event = usize;
        }

        impl super::View for View {
            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                post_inc(&mut self.render_count);
                Empty::new().into_any()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let window = cx.add_window(|_| View { render_count: 0 });
        let called_defer = Rc::new(AtomicBool::new(false));
        let called_after_window_update = Rc::new(AtomicBool::new(false));

        window.root(cx).update(cx, |this, cx| {
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
        assert_eq!(window.read_root_with(cx, |view, _| view.render_count), 3);
    }

    #[crate::test(self)]
    fn test_view_handles(cx: &mut TestAppContext) {
        struct View {
            other: Option<ViewHandle<View>>,
            events: Vec<String>,
        }

        impl Entity for View {
            type Event = usize;
        }

        impl super::View for View {
            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
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

        let window = cx.add_window(|cx| View::new(None, cx));
        let handle_1 = window.add_view(cx, |cx| View::new(None, cx));
        let handle_2 = window.add_view(cx, |cx| View::new(Some(handle_1.clone()), cx));
        assert_eq!(cx.read(|cx| cx.views.len()), 3);

        handle_1.update(cx, |view, cx| {
            view.events.push("updated".into());
            cx.emit(1);
            cx.emit(2);
        });
        handle_1.read_with(cx, |view, _| {
            assert_eq!(view.events, vec!["updated".to_string()]);
        });
        handle_2.read_with(cx, |view, _| {
            assert_eq!(
                view.events,
                vec![
                    "observed event 1".to_string(),
                    "observed event 2".to_string(),
                ]
            );
        });

        handle_2.update(cx, |view, _| {
            drop(handle_1);
            view.other.take();
        });

        cx.read(|cx| {
            assert_eq!(cx.views.len(), 2);
            assert!(cx.subscriptions.is_empty());
            assert!(cx.observations.is_empty());
        });
    }

    #[crate::test(self)]
    fn test_add_window(cx: &mut AppContext) {
        struct View {
            mouse_down_count: Arc<AtomicUsize>,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                enum Handler {}
                let mouse_down_count = self.mouse_down_count.clone();
                MouseEventHandler::new::<Handler, _>(0, cx, |_, _| Empty::new())
                    .on_down(MouseButton::Left, move |_, _, _| {
                        mouse_down_count.fetch_add(1, SeqCst);
                    })
                    .into_any()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let mouse_down_count = Arc::new(AtomicUsize::new(0));
        let window = cx.add_window(Default::default(), |_| View {
            mouse_down_count: mouse_down_count.clone(),
        });

        window.update(cx, |cx| {
            // Ensure window's root element is in a valid lifecycle state.
            cx.dispatch_event(
                Event::MouseDown(MouseButtonEvent {
                    position: Default::default(),
                    button: MouseButton::Left,
                    modifiers: Default::default(),
                    click_count: 1,
                }),
                false,
            );
            assert_eq!(mouse_down_count.load(SeqCst), 1);
        });
    }

    #[crate::test(self)]
    fn test_entity_release_hooks(cx: &mut TestAppContext) {
        struct Model {
            released: Rc<Cell<bool>>,
        }

        struct View {
            released: Rc<Cell<bool>>,
        }

        impl Entity for Model {
            type Event = ();

            fn release(&mut self, _: &mut AppContext) {
                self.released.set(true);
            }
        }

        impl Entity for View {
            type Event = ();

            fn release(&mut self, _: &mut AppContext) {
                self.released.set(true);
            }
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "View"
            }

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
        }

        let model_released = Rc::new(Cell::new(false));
        let model_release_observed = Rc::new(Cell::new(false));
        let view_released = Rc::new(Cell::new(false));
        let view_release_observed = Rc::new(Cell::new(false));

        let model = cx.add_model(|_| Model {
            released: model_released.clone(),
        });
        let window = cx.add_window(|_| View {
            released: view_released.clone(),
        });
        let view = window.root(cx);

        assert!(!model_released.get());
        assert!(!view_released.get());

        cx.update(|cx| {
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
        });

        cx.update(move |_| {
            drop(model);
        });
        assert!(model_released.get());
        assert!(model_release_observed.get());

        drop(view);
        window.update(cx, |cx| cx.remove_window());
        assert!(view_released.get());
        assert!(view_release_observed.get());
    }

    #[crate::test(self)]
    fn test_view_events(cx: &mut TestAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = String;
        }

        let window = cx.add_window(|_| TestView::default());
        let handle_1 = window.root(cx);
        let handle_2 = window.add_view(cx, |_| TestView::default());
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
        handle_1.read_with(cx, |view, _| assert_eq!(view.events, ["7"]));

        handle_2.update(cx, |_, c| c.emit("5".into()));
        handle_1.read_with(cx, |view, _| {
            assert_eq!(view.events, ["7", "5", "5 from inner"])
        });

        handle_3.update(cx, |_, c| c.emit("9".into()));
        handle_1.read_with(cx, |view, _| {
            assert_eq!(view.events, ["7", "5", "5 from inner", "9"])
        });
    }

    #[crate::test(self)]
    fn test_global_events(cx: &mut AppContext) {
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
    fn test_global_events_emitted_before_subscription_in_same_update_cycle(cx: &mut AppContext) {
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
    fn test_global_nested_events(cx: &mut AppContext) {
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
    fn test_global(cx: &mut AppContext) {
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
    fn test_dropping_subscribers(cx: &mut TestAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let window = cx.add_window(|_| TestView::default());
        let observing_view = window.add_view(cx, |_| TestView::default());
        let emitting_view = window.add_view(cx, |_| TestView::default());
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
    fn test_view_emit_before_subscribe_in_same_update_cycle(cx: &mut AppContext) {
        let window = cx.add_window::<TestView, _>(Default::default(), |cx| {
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

        window.read_root_with(cx, |view, _| assert_eq!(view.events, ["before emit"]));
    }

    #[crate::test(self)]
    fn test_observe_and_notify_from_view(cx: &mut TestAppContext) {
        #[derive(Default)]
        struct Model {
            state: String,
        }

        impl Entity for Model {
            type Event = ();
        }

        let window = cx.add_window(|_| TestView::default());
        let view = window.root(cx);
        let model = cx.add_model(|_| Model {
            state: "old-state".into(),
        });

        view.update(cx, |_, c| {
            c.observe(&model, |me, observed, cx| {
                me.events.push(observed.read(cx).state.clone())
            })
            .detach();
        });

        model.update(cx, |model, cx| {
            model.state = "new-state".into();
            cx.notify();
        });
        view.read_with(cx, |view, _| assert_eq!(view.events, ["new-state"]));
    }

    #[crate::test(self)]
    fn test_view_notify_before_observe_in_same_update_cycle(cx: &mut AppContext) {
        let window = cx.add_window::<TestView, _>(Default::default(), |cx| {
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

        window.read_root_with(cx, |view, _| assert_eq!(view.events, ["before notify"]));
    }

    #[crate::test(self)]
    fn test_notify_and_drop_observe_subscription_in_same_update_cycle(cx: &mut TestAppContext) {
        struct Model;
        impl Entity for Model {
            type Event = ();
        }

        let model = cx.add_model(|_| Model);
        let window = cx.add_window(|_| TestView::default());
        let view = window.root(cx);

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
        view.read_with(cx, |view, _| assert_eq!(view.events, Vec::<&str>::new()));
    }

    #[crate::test(self)]
    fn test_dropping_observers(cx: &mut TestAppContext) {
        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let window = cx.add_window(|_| TestView::default());
        let observing_view = window.add_view(cx, |_| TestView::default());
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
    fn test_dropping_subscriptions_during_callback(cx: &mut TestAppContext) {
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
            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let window = cx.add_window(|_| View);
        let observing_view = window.add_view(cx, |_| View);
        let observed_view = window.add_view(cx, |_| View);

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

        cx.update(|cx| {
            cx.default_global::<()>();
            cx.set_global(());
        });
        assert_eq!(*observation_count.borrow(), 1);
    }

    #[crate::test(self)]
    fn test_focus(cx: &mut TestAppContext) {
        struct View {
            name: String,
            events: Arc<Mutex<Vec<String>>>,
            child: Option<AnyViewHandle>,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                self.child
                    .as_ref()
                    .map(|child| ChildView::new(child, cx).into_any())
                    .unwrap_or(Empty::new().into_any())
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
        let window = cx.add_window(|_| View {
            events: view_events.clone(),
            name: "view 1".to_string(),
            child: None,
        });
        let view_1 = window.root(cx);
        let view_2 = window.update(cx, |cx| {
            let view_2 = cx.add_view(|_| View {
                events: view_events.clone(),
                name: "view 2".to_string(),
                child: None,
            });
            view_1.update(cx, |view_1, cx| {
                view_1.child = Some(view_2.clone().into_any());
                cx.notify();
            });
            view_2
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
            // Ensure only the last focus event is honored.
            cx.focus(&view_2);
            cx.focus(&view_1);
            cx.focus(&view_2);
        });

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

        view_1.update(cx, |_, cx| cx.focus(&view_1));
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

        println!("=====================");
        view_1.update(cx, |view, _| {
            drop(view_2);
            view.child = None;
        });
        assert_eq!(mem::take(&mut *view_events.lock()), ["view 1 focused"]);
        assert_eq!(mem::take(&mut *observed_events.lock()), Vec::<&str>::new());
    }

    #[crate::test(self)]
    fn test_deserialize_actions(cx: &mut AppContext) {
        #[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
        pub struct ComplexAction {
            arg: String,
            count: usize,
        }

        actions!(test::something, [SimpleAction]);
        impl_actions!(test::something, [ComplexAction]);

        cx.add_global_action(move |_: &SimpleAction, _: &mut AppContext| {});
        cx.add_global_action(move |_: &ComplexAction, _: &mut AppContext| {});

        let action1 = cx
            .deserialize_action(
                "test::something::ComplexAction",
                Some(serde_json::from_str(r#"{"arg": "a", "count": 5}"#).unwrap()),
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
    fn test_dispatch_action(cx: &mut TestAppContext) {
        struct ViewA {
            id: usize,
            child: Option<AnyViewHandle>,
        }

        impl Entity for ViewA {
            type Event = ();
        }

        impl View for ViewA {
            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                self.child
                    .as_ref()
                    .map(|child| ChildView::new(child, cx).into_any())
                    .unwrap_or(Empty::new().into_any())
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct ViewB {
            id: usize,
            child: Option<AnyViewHandle>,
        }

        impl Entity for ViewB {
            type Event = ();
        }

        impl View for ViewB {
            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                self.child
                    .as_ref()
                    .map(|child| ChildView::new(child, cx).into_any())
                    .unwrap_or(Empty::new().into_any())
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        #[derive(Clone, Default, Deserialize, PartialEq)]
        pub struct Action(pub String);

        impl_actions!(test, [Action]);

        let actions = Rc::new(RefCell::new(Vec::new()));
        let observed_actions = Rc::new(RefCell::new(Vec::new()));

        cx.update(|cx| {
            cx.add_global_action({
                let actions = actions.clone();
                move |_: &Action, _: &mut AppContext| {
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
                            ViewB { id: 5, child: None }
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

            cx.observe_actions({
                let observed_actions = observed_actions.clone();
                move |action_id, _| observed_actions.borrow_mut().push(action_id)
            })
            .detach();
        });

        let window = cx.add_window(|_| ViewA { id: 1, child: None });
        let view_1 = window.root(cx);
        let view_2 = window.update(cx, |cx| {
            let child = cx.add_view(|_| ViewB { id: 2, child: None });
            view_1.update(cx, |view, cx| {
                view.child = Some(child.clone().into_any());
                cx.notify();
            });
            child
        });
        let view_3 = window.update(cx, |cx| {
            let child = cx.add_view(|_| ViewA { id: 3, child: None });
            view_2.update(cx, |view, cx| {
                view.child = Some(child.clone().into_any());
                cx.notify();
            });
            child
        });
        let view_4 = window.update(cx, |cx| {
            let child = cx.add_view(|_| ViewB { id: 4, child: None });
            view_3.update(cx, |view, cx| {
                view.child = Some(child.clone().into_any());
                cx.notify();
            });
            child
        });

        window.update(cx, |cx| {
            cx.dispatch_action(Some(view_4.id()), &Action("bar".to_string()))
        });

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

        let window = cx.add_window(|_| ViewB { id: 2, child: None });
        let view_2 = window.root(cx);
        let view_3 = window.update(cx, |cx| {
            let child = cx.add_view(|_| ViewA { id: 3, child: None });
            view_2.update(cx, |view, cx| {
                view.child = Some(child.clone().into_any());
                cx.notify();
            });
            child
        });
        let view_4 = window.update(cx, |cx| {
            let child = cx.add_view(|_| ViewB { id: 4, child: None });
            view_3.update(cx, |view, cx| {
                view.child = Some(child.clone().into_any());
                cx.notify();
            });
            child
        });

        actions.borrow_mut().clear();
        window.update(cx, |cx| {
            cx.dispatch_action(Some(view_4.id()), &Action("bar".to_string()))
        });

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
    fn test_dispatch_keystroke(cx: &mut AppContext) {
        #[derive(Clone, Deserialize, PartialEq)]
        pub struct Action(String);

        impl_actions!(test, [Action]);

        struct View {
            id: usize,
            keymap_context: KeymapContext,
            child: Option<AnyViewHandle>,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                self.child
                    .as_ref()
                    .map(|child| ChildView::new(child, cx).into_any())
                    .unwrap_or(Empty::new().into_any())
            }

            fn ui_name() -> &'static str {
                "View"
            }

            fn update_keymap_context(&self, keymap: &mut KeymapContext, _: &AppContext) {
                *keymap = self.keymap_context.clone();
            }
        }

        impl View {
            fn new(id: usize) -> Self {
                View {
                    id,
                    keymap_context: KeymapContext::default(),
                    child: None,
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

        let window = cx.add_window(Default::default(), |cx| {
            let view_2 = cx.add_view(|cx| {
                let view_3 = cx.add_view(|cx| {
                    cx.focus_self();
                    view_3
                });
                view_2.child = Some(view_3.into_any());
                view_2
            });
            view_1.child = Some(view_2.into_any());
            view_1
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

        window.update(cx, |cx| {
            cx.dispatch_keystroke(&Keystroke::parse("a").unwrap())
        });
        assert_eq!(&*actions.borrow(), &["2 a"]);
        actions.borrow_mut().clear();

        window.update(cx, |cx| {
            cx.dispatch_keystroke(&Keystroke::parse("b").unwrap());
        });

        assert_eq!(&*actions.borrow(), &["3 b", "2 b", "1 b", "global b"]);
        actions.borrow_mut().clear();

        window.update(cx, |cx| {
            cx.dispatch_keystroke(&Keystroke::parse("c").unwrap());
        });
        assert_eq!(&*actions.borrow(), &["3 c"]);
        actions.borrow_mut().clear();

        window.update(cx, |cx| {
            cx.dispatch_keystroke(&Keystroke::parse("d").unwrap());
        });
        assert_eq!(&*actions.borrow(), &["2 d"]);
        actions.borrow_mut().clear();
    }

    #[crate::test(self)]
    fn test_keystrokes_for_action(cx: &mut TestAppContext) {
        actions!(test, [Action1, Action2, GlobalAction]);

        struct View1 {
            child: ViewHandle<View2>,
        }
        struct View2 {}

        impl Entity for View1 {
            type Event = ();
        }
        impl Entity for View2 {
            type Event = ();
        }

        impl super::View for View1 {
            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                ChildView::new(&self.child, cx).into_any()
            }
            fn ui_name() -> &'static str {
                "View1"
            }
        }
        impl super::View for View2 {
            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
            fn ui_name() -> &'static str {
                "View2"
            }
        }

        let window = cx.add_window(|cx| {
            let view_2 = cx.add_view(|cx| {
                cx.focus_self();
                View2 {}
            });
            View1 { child: view_2 }
        });
        let view_1 = window.root(cx);
        let view_2 = view_1.read_with(cx, |view, _| view.child.clone());

        cx.update(|cx| {
            cx.add_action(|_: &mut View1, _: &Action1, _cx| {});
            cx.add_action(|_: &mut View2, _: &Action2, _cx| {});
            cx.add_global_action(|_: &GlobalAction, _| {});
            cx.add_bindings(vec![
                Binding::new("a", Action1, Some("View1")),
                Binding::new("b", Action2, Some("View1 > View2")),
                Binding::new("c", GlobalAction, Some("View3")), // View 3 does not exist
            ]);
        });

        let view_1_id = view_1.id();
        view_1.update(cx, |_, cx| {
            view_2.update(cx, |_, cx| {
                // Sanity check
                let mut new_parents = Default::default();
                let mut notify_views_if_parents_change = Default::default();
                let mut layout_cx = LayoutContext::new(
                    cx,
                    &mut new_parents,
                    &mut notify_views_if_parents_change,
                    false,
                );
                assert_eq!(
                    layout_cx
                        .keystrokes_for_action(view_1_id, &Action1)
                        .unwrap()
                        .as_slice(),
                    &[Keystroke::parse("a").unwrap()]
                );
                assert_eq!(
                    layout_cx
                        .keystrokes_for_action(view_2.id(), &Action2)
                        .unwrap()
                        .as_slice(),
                    &[Keystroke::parse("b").unwrap()]
                );

                // The 'a' keystroke propagates up the view tree from view_2
                // to view_1. The action, Action1, is handled by view_1.
                assert_eq!(
                    layout_cx
                        .keystrokes_for_action(view_2.id(), &Action1)
                        .unwrap()
                        .as_slice(),
                    &[Keystroke::parse("a").unwrap()]
                );

                // Actions that are handled below the current view don't have bindings
                assert_eq!(layout_cx.keystrokes_for_action(view_1_id, &Action2), None);

                // Actions that are handled in other branches of the tree should not have a binding
                assert_eq!(
                    layout_cx.keystrokes_for_action(view_2.id(), &GlobalAction),
                    None
                );
            });
        });

        // Check that global actions do not have a binding, even if a binding does exist in another view
        assert_eq!(
            &available_actions(window.into(), view_1.id(), cx),
            &[
                ("test::Action1", vec![Keystroke::parse("a").unwrap()]),
                ("test::GlobalAction", vec![])
            ],
        );

        // Check that view 1 actions and bindings are available even when called from view 2
        assert_eq!(
            &available_actions(window.into(), view_2.id(), cx),
            &[
                ("test::Action1", vec![Keystroke::parse("a").unwrap()]),
                ("test::Action2", vec![Keystroke::parse("b").unwrap()]),
                ("test::GlobalAction", vec![]),
            ],
        );

        // Produces a list of actions and key bindings
        fn available_actions(
            window: AnyWindowHandle,
            view_id: usize,
            cx: &TestAppContext,
        ) -> Vec<(&'static str, Vec<Keystroke>)> {
            cx.available_actions(window.into(), view_id)
                .into_iter()
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
    }

    #[crate::test(self)]
    fn test_keystrokes_for_action_with_data(cx: &mut TestAppContext) {
        #[derive(Clone, Debug, Deserialize, PartialEq)]
        struct ActionWithArg {
            #[serde(default)]
            arg: bool,
        }

        struct View;
        impl super::Entity for View {
            type Event = ();
        }
        impl super::View for View {
            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
            fn ui_name() -> &'static str {
                "View"
            }
        }

        impl_actions!(test, [ActionWithArg]);

        let window = cx.add_window(|_| View);
        let view = window.root(cx);
        cx.update(|cx| {
            cx.add_global_action(|_: &ActionWithArg, _| {});
            cx.add_bindings(vec![
                Binding::new("a", ActionWithArg { arg: false }, None),
                Binding::new("shift-a", ActionWithArg { arg: true }, None),
            ]);
        });

        let actions = cx.available_actions(window.into(), view.id());
        assert_eq!(
            actions[0].1.as_any().downcast_ref::<ActionWithArg>(),
            Some(&ActionWithArg { arg: false })
        );
        assert_eq!(
            actions[0]
                .2
                .iter()
                .map(|b| b.keystrokes()[0].clone())
                .collect::<Vec<_>>(),
            vec![Keystroke::parse("a").unwrap()],
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

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
        }

        impl Counter {
            fn inc(&mut self, cx: &mut ViewContext<Self>) {
                self.0 += 1;
                cx.notify();
            }
        }

        let window = cx.add_window(|_| Counter(0));
        let view = window.root(cx);

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
        let window = cx.add_window(|_| TestView::default());
        window.root(cx).condition(cx, |_, _| false).await;
    }

    #[crate::test(self)]
    #[should_panic(expected = "view dropped with pending condition")]
    async fn test_view_condition_panic_on_drop(cx: &mut TestAppContext) {
        let window = cx.add_window(|_| TestView::default());
        let view = window.add_view(cx, |_| TestView::default());

        let condition = view.condition(cx, |_, _| false);
        cx.update(|_| drop(view));
        condition.await;
    }

    #[crate::test(self)]
    fn test_refresh_windows(cx: &mut TestAppContext) {
        struct View(usize);

        impl super::Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any_named(format!("render count: {}", post_inc(&mut self.0)))
            }
        }

        let window = cx.add_window(|_| View(0));
        let root_view = window.root(cx);
        window.update(cx, |cx| {
            assert_eq!(
                cx.window.rendered_views[&root_view.id()].name(),
                Some("render count: 0")
            );
        });

        let view = window.update(cx, |cx| {
            cx.refresh_windows();
            cx.add_view(|_| View(0))
        });

        window.update(cx, |cx| {
            assert_eq!(
                cx.window.rendered_views[&root_view.id()].name(),
                Some("render count: 1")
            );
            assert_eq!(
                cx.window.rendered_views[&view.id()].name(),
                Some("render count: 0")
            );
        });

        cx.update(|cx| cx.refresh_windows());

        window.update(cx, |cx| {
            assert_eq!(
                cx.window.rendered_views[&root_view.id()].name(),
                Some("render count: 2")
            );
            assert_eq!(
                cx.window.rendered_views[&view.id()].name(),
                Some("render count: 1")
            );
        });

        cx.update(|cx| {
            cx.refresh_windows();
            drop(view);
        });

        window.update(cx, |cx| {
            assert_eq!(
                cx.window.rendered_views[&root_view.id()].name(),
                Some("render count: 3")
            );
            assert_eq!(cx.window.rendered_views.len(), 1);
        });
    }

    #[crate::test(self)]
    async fn test_labeled_tasks(cx: &mut TestAppContext) {
        assert_eq!(None, cx.update(|cx| cx.active_labeled_tasks().next()));
        let (mut sender, mut receiver) = postage::oneshot::channel::<()>();
        let task = cx
            .update(|cx| cx.spawn_labeled("Test Label", |_| async move { receiver.recv().await }));

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

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
        }

        let events = Rc::new(RefCell::new(Vec::new()));
        let window_1 = cx.add_window(|cx: &mut ViewContext<View>| {
            cx.observe_window_activation({
                let events = events.clone();
                move |this, active, _| events.borrow_mut().push((this.0, active))
            })
            .detach();
            View("window 1")
        });
        assert_eq!(mem::take(&mut *events.borrow_mut()), [("window 1", true)]);

        let window_2 = cx.add_window(|cx: &mut ViewContext<View>| {
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

        let window_3 = cx.add_window(|cx: &mut ViewContext<View>| {
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

        window_2.simulate_activation(cx);
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 3", false), ("window 2", true)]
        );

        window_1.simulate_activation(cx);
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 2", false), ("window 1", true)]
        );

        window_3.simulate_activation(cx);
        assert_eq!(
            mem::take(&mut *events.borrow_mut()),
            [("window 1", false), ("window 3", true)]
        );

        window_3.simulate_activation(cx);
        assert_eq!(mem::take(&mut *events.borrow_mut()), []);
    }

    #[crate::test(self)]
    fn test_child_view(cx: &mut TestAppContext) {
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

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                self.rendered.set(true);
                Empty::new().into_any()
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

            fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
                if let Some(child) = self.child.as_ref() {
                    ChildView::new(child, cx).into_any()
                } else {
                    Empty::new().into_any()
                }
            }
        }

        let child_rendered = Rc::new(Cell::new(false));
        let child_dropped = Rc::new(Cell::new(false));
        let window = cx.add_window(|cx| Parent {
            child: Some(cx.add_view(|_| Child {
                rendered: child_rendered.clone(),
                dropped: child_dropped.clone(),
            })),
        });
        let root_view = window.root(cx);
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

        fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }
    }
}
