mod async_context;
mod entity_map;
mod model_context;
#[cfg(any(test, feature = "test-support"))]
mod test_context;

pub use async_context::*;
use derive_more::{Deref, DerefMut};
pub use entity_map::*;
pub use model_context::*;
use refineable::Refineable;
use smallvec::SmallVec;
#[cfg(any(test, feature = "test-support"))]
pub use test_context::*;

use crate::{
    current_platform, image_cache::ImageCache, Action, AnyBox, AnyView, AnyWindowHandle,
    AppMetadata, AssetSource, BackgroundExecutor, ClipboardItem, Context, DispatchPhase, DisplayId,
    Entity, EventEmitter, FocusEvent, FocusHandle, FocusId, ForegroundExecutor, KeyBinding, Keymap,
    LayoutId, PathPromptOptions, Pixels, Platform, PlatformDisplay, Point, Render, SubscriberSet,
    Subscription, SvgRenderer, Task, TextStyle, TextStyleRefinement, TextSystem, View, ViewContext,
    Window, WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use collections::{HashMap, HashSet, VecDeque};
use futures::{channel::oneshot, future::LocalBoxFuture, Future};
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{atomic::Ordering::SeqCst, Arc},
    time::Duration,
};
use util::http::{self, HttpClient};

/// Temporary(?) wrapper around RefCell<AppContext> to help us debug any double borrows.
/// Strongly consider removing after stabilization.
pub struct AppCell {
    app: RefCell<AppContext>,
}

impl AppCell {
    #[track_caller]
    pub fn borrow(&self) -> AppRef {
        if let Some(_) = option_env!("TRACK_THREAD_BORROWS") {
            let thread_id = std::thread::current().id();
            eprintln!("borrowed {thread_id:?}");
        }
        AppRef(self.app.borrow())
    }

    #[track_caller]
    pub fn borrow_mut(&self) -> AppRefMut {
        if let Some(_) = option_env!("TRACK_THREAD_BORROWS") {
            let thread_id = std::thread::current().id();
            eprintln!("borrowed {thread_id:?}");
        }
        AppRefMut(self.app.borrow_mut())
    }
}

#[derive(Deref, DerefMut)]
pub struct AppRef<'a>(Ref<'a, AppContext>);

impl<'a> Drop for AppRef<'a> {
    fn drop(&mut self) {
        if let Some(_) = option_env!("TRACK_THREAD_BORROWS") {
            let thread_id = std::thread::current().id();
            eprintln!("dropped borrow from {thread_id:?}");
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct AppRefMut<'a>(RefMut<'a, AppContext>);

impl<'a> Drop for AppRefMut<'a> {
    fn drop(&mut self) {
        if let Some(_) = option_env!("TRACK_THREAD_BORROWS") {
            let thread_id = std::thread::current().id();
            eprintln!("dropped {thread_id:?}");
        }
    }
}

pub struct App(Rc<AppCell>);

/// Represents an application before it is fully launched. Once your app is
/// configured, you'll start the app with `App::run`.
impl App {
    /// Builds an app with the given asset source.
    pub fn production(asset_source: Arc<dyn AssetSource>) -> Self {
        Self(AppContext::new(
            current_platform(),
            asset_source,
            http::client(),
        ))
    }

    /// Start the application. The provided callback will be called once the
    /// app is fully launched.
    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut AppContext),
    {
        let this = self.0.clone();
        let platform = self.0.borrow().platform.clone();
        platform.run(Box::new(move || {
            let cx = &mut *this.borrow_mut();
            on_finish_launching(cx);
        }));
    }

    /// Register a handler to be invoked when the platform instructs the application
    /// to open one or more URLs.
    pub fn on_open_urls<F>(&self, mut callback: F) -> &Self
    where
        F: 'static + FnMut(Vec<String>, &mut AppContext),
    {
        let this = Rc::downgrade(&self.0);
        self.0.borrow().platform.on_open_urls(Box::new(move |urls| {
            if let Some(app) = this.upgrade() {
                callback(urls, &mut *app.borrow_mut());
            }
        }));
        self
    }

    pub fn on_reopen<F>(&self, mut callback: F) -> &Self
    where
        F: 'static + FnMut(&mut AppContext),
    {
        let this = Rc::downgrade(&self.0);
        self.0.borrow_mut().platform.on_reopen(Box::new(move || {
            if let Some(app) = this.upgrade() {
                callback(&mut app.borrow_mut());
            }
        }));
        self
    }

    pub fn metadata(&self) -> AppMetadata {
        self.0.borrow().app_metadata.clone()
    }

    pub fn background_executor(&self) -> BackgroundExecutor {
        self.0.borrow().background_executor.clone()
    }

    pub fn foreground_executor(&self) -> ForegroundExecutor {
        self.0.borrow().foreground_executor.clone()
    }

    pub fn text_system(&self) -> Arc<TextSystem> {
        self.0.borrow().text_system.clone()
    }
}

pub(crate) type FrameCallback = Box<dyn FnOnce(&mut AppContext)>;
type Handler = Box<dyn FnMut(&mut AppContext) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut AppContext) -> bool + 'static>;
type QuitHandler = Box<dyn FnOnce(&mut AppContext) -> LocalBoxFuture<'static, ()> + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut AppContext) + 'static>;
type NewViewListener = Box<dyn FnMut(AnyView, &mut WindowContext) + 'static>;

// struct FrameConsumer {
//     next_frame_callbacks: Vec<FrameCallback>,
//     task: Task<()>,
//     display_linker
// }

pub struct AppContext {
    pub(crate) this: Weak<AppCell>,
    pub(crate) platform: Rc<dyn Platform>,
    app_metadata: AppMetadata,
    text_system: Arc<TextSystem>,
    flushing_effects: bool,
    pending_updates: usize,
    pub(crate) active_drag: Option<AnyDrag>,
    pub(crate) active_tooltip: Option<AnyTooltip>,
    pub(crate) next_frame_callbacks: HashMap<DisplayId, Vec<FrameCallback>>,
    pub(crate) frame_consumers: HashMap<DisplayId, Task<()>>,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    pub(crate) svg_renderer: SvgRenderer,
    asset_source: Arc<dyn AssetSource>,
    pub(crate) image_cache: ImageCache,
    pub(crate) text_style_stack: Vec<TextStyleRefinement>,
    pub(crate) globals_by_type: HashMap<TypeId, AnyBox>,
    pub(crate) entities: EntityMap,
    pub(crate) new_view_observers: SubscriberSet<TypeId, NewViewListener>,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) keymap: Arc<Mutex<Keymap>>,
    pub(crate) global_action_listeners:
        HashMap<TypeId, Vec<Box<dyn Fn(&dyn Action, DispatchPhase, &mut Self)>>>,
    pending_effects: VecDeque<Effect>,
    pub(crate) pending_notifications: HashSet<EntityId>,
    pub(crate) pending_global_notifications: HashSet<TypeId>,
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    // TypeId is the type of the event that the listener callback expects
    pub(crate) event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    pub(crate) release_listeners: SubscriberSet<EntityId, ReleaseListener>,
    pub(crate) global_observers: SubscriberSet<TypeId, Handler>,
    pub(crate) quit_observers: SubscriberSet<(), QuitHandler>,
    pub(crate) layout_id_buffer: Vec<LayoutId>, // We recycle this memory across layout requests.
    pub(crate) propagate_event: bool,
}

impl AppContext {
    pub(crate) fn new(
        platform: Rc<dyn Platform>,
        asset_source: Arc<dyn AssetSource>,
        http_client: Arc<dyn HttpClient>,
    ) -> Rc<AppCell> {
        let executor = platform.background_executor();
        let foreground_executor = platform.foreground_executor();
        assert!(
            executor.is_main_thread(),
            "must construct App on main thread"
        );

        let text_system = Arc::new(TextSystem::new(platform.text_system()));
        let entities = EntityMap::new();

        let app_metadata = AppMetadata {
            os_name: platform.os_name(),
            os_version: platform.os_version().ok(),
            app_version: platform.app_version().ok(),
        };

        Rc::new_cyclic(|this| AppCell {
            app: RefCell::new(AppContext {
                this: this.clone(),
                platform,
                app_metadata,
                text_system,
                flushing_effects: false,
                pending_updates: 0,
                active_drag: None,
                active_tooltip: None,
                next_frame_callbacks: HashMap::default(),
                frame_consumers: HashMap::default(),
                background_executor: executor,
                foreground_executor,
                svg_renderer: SvgRenderer::new(asset_source.clone()),
                asset_source,
                image_cache: ImageCache::new(http_client),
                text_style_stack: Vec::new(),
                globals_by_type: HashMap::default(),
                entities,
                new_view_observers: SubscriberSet::new(),
                windows: SlotMap::with_key(),
                keymap: Arc::new(Mutex::new(Keymap::default())),
                global_action_listeners: HashMap::default(),
                pending_effects: VecDeque::new(),
                pending_notifications: HashSet::default(),
                pending_global_notifications: HashSet::default(),
                observers: SubscriberSet::new(),
                event_listeners: SubscriberSet::new(),
                release_listeners: SubscriberSet::new(),
                global_observers: SubscriberSet::new(),
                quit_observers: SubscriberSet::new(),
                layout_id_buffer: Default::default(),
                propagate_event: true,
            }),
        })
    }

    /// Quit the application gracefully. Handlers registered with `ModelContext::on_app_quit`
    /// will be given 100ms to complete before exiting.
    pub fn quit(&mut self) {
        let mut futures = Vec::new();

        for observer in self.quit_observers.remove(&()) {
            futures.push(observer(self));
        }

        self.windows.clear();
        self.flush_effects();

        let futures = futures::future::join_all(futures);
        if self
            .background_executor
            .block_with_timeout(Duration::from_millis(100), futures)
            .is_err()
        {
            log::error!("timed out waiting on app_will_quit");
        }

        self.globals_by_type.clear();
    }

    pub fn app_metadata(&self) -> AppMetadata {
        self.app_metadata.clone()
    }

    /// Schedules all windows in the application to be redrawn. This can be called
    /// multiple times in an update cycle and still result in a single redraw.
    pub fn refresh(&mut self) {
        self.pending_effects.push_back(Effect::Refresh);
    }
    pub(crate) fn update<R>(&mut self, update: impl FnOnce(&mut Self) -> R) -> R {
        self.pending_updates += 1;
        let result = update(self);
        if !self.flushing_effects && self.pending_updates == 1 {
            self.flushing_effects = true;
            self.flush_effects();
            self.flushing_effects = false;
        }
        self.pending_updates -= 1;
        result
    }

    pub fn observe<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(E, &mut AppContext) + 'static,
    ) -> Subscription
    where
        W: 'static,
        E: Entity<W>,
    {
        self.observe_internal(entity, move |e, cx| {
            on_notify(e, cx);
            true
        })
    }

    pub fn observe_internal<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(E, &mut AppContext) -> bool + 'static,
    ) -> Subscription
    where
        W: 'static,
        E: Entity<W>,
    {
        let entity_id = entity.entity_id();
        let handle = entity.downgrade();
        self.observers.insert(
            entity_id,
            Box::new(move |cx| {
                if let Some(handle) = E::upgrade_from(&handle) {
                    on_notify(handle, cx)
                } else {
                    false
                }
            }),
        )
    }

    pub fn subscribe<T, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Evt, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Evt>,
        E: Entity<T>,
        Evt: 'static,
    {
        self.subscribe_internal(entity, move |entity, event, cx| {
            on_event(entity, event, cx);
            true
        })
    }

    pub(crate) fn subscribe_internal<T, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Evt, &mut AppContext) -> bool + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Evt>,
        E: Entity<T>,
        Evt: 'static,
    {
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();

        self.event_listeners.insert(
            entity_id,
            (
                TypeId::of::<Evt>(),
                Box::new(move |event, cx| {
                    let event: &Evt = event.downcast_ref().expect("invalid event type");
                    if let Some(handle) = E::upgrade_from(&entity) {
                        on_event(handle, event, cx)
                    } else {
                        false
                    }
                }),
            ),
        )
    }

    pub fn windows(&self) -> Vec<AnyWindowHandle> {
        self.windows
            .values()
            .filter_map(|window| Some(window.as_ref()?.handle.clone()))
            .collect()
    }

    /// Opens a new window with the given option and the root view returned by the given function.
    /// The function is invoked with a `WindowContext`, which can be used to interact with window-specific
    /// functionality.
    pub fn open_window<V: Render>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> View<V>,
    ) -> WindowHandle<V> {
        self.update(|cx| {
            let id = cx.windows.insert(None);
            let handle = WindowHandle::new(id);
            let mut window = Window::new(handle.into(), options, cx);
            let root_view = build_root_view(&mut WindowContext::new(cx, &mut window));
            window.root_view.replace(root_view.into());
            cx.windows.get_mut(id).unwrap().replace(window);
            handle
        })
    }

    /// Instructs the platform to activate the application by bringing it to the foreground.
    pub fn activate(&self, ignoring_other_apps: bool) {
        self.platform.activate(ignoring_other_apps);
    }

    /// Returns the list of currently active displays.
    pub fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.platform.displays()
    }

    /// Writes data to the platform clipboard.
    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform.write_to_clipboard(item)
    }

    /// Reads data from the platform clipboard.
    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform.read_from_clipboard()
    }

    /// Writes credentials to the platform keychain.
    pub fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Result<()> {
        self.platform.write_credentials(url, username, password)
    }

    /// Reads credentials from the platform keychain.
    pub fn read_credentials(&self, url: &str) -> Result<Option<(String, Vec<u8>)>> {
        self.platform.read_credentials(url)
    }

    /// Deletes credentials from the platform keychain.
    pub fn delete_credentials(&self, url: &str) -> Result<()> {
        self.platform.delete_credentials(url)
    }

    /// Directs the platform's default browser to open the given URL.
    pub fn open_url(&self, url: &str) {
        self.platform.open_url(url);
    }

    pub fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        self.platform.path_for_auxiliary_executable(name)
    }

    pub fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        self.platform.prompt_for_paths(options)
    }

    pub fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        self.platform.prompt_for_new_path(directory)
    }

    pub fn reveal_path(&self, path: &Path) {
        self.platform.reveal_path(path)
    }

    pub fn should_auto_hide_scrollbars(&self) -> bool {
        self.platform.should_auto_hide_scrollbars()
    }

    pub(crate) fn push_effect(&mut self, effect: Effect) {
        match &effect {
            Effect::Notify { emitter } => {
                if !self.pending_notifications.insert(*emitter) {
                    return;
                }
            }
            Effect::NotifyGlobalObservers { global_type } => {
                if !self.pending_global_notifications.insert(*global_type) {
                    return;
                }
            }
            _ => {}
        };

        self.pending_effects.push_back(effect);
    }

    /// Called at the end of AppContext::update to complete any side effects
    /// such as notifying observers, emitting events, etc. Effects can themselves
    /// cause effects, so we continue looping until all effects are processed.
    fn flush_effects(&mut self) {
        loop {
            self.release_dropped_entities();
            self.release_dropped_focus_handles();
            if let Some(effect) = self.pending_effects.pop_front() {
                match effect {
                    Effect::Notify { emitter } => {
                        self.apply_notify_effect(emitter);
                    }
                    Effect::Emit {
                        emitter,
                        event_type,
                        event,
                    } => self.apply_emit_effect(emitter, event_type, event),
                    Effect::FocusChanged {
                        window_handle,
                        focused,
                    } => {
                        self.apply_focus_changed_effect(window_handle, focused);
                    }
                    Effect::Refresh => {
                        self.apply_refresh_effect();
                    }
                    Effect::NotifyGlobalObservers { global_type } => {
                        self.apply_notify_global_observers_effect(global_type);
                    }
                    Effect::Defer { callback } => {
                        self.apply_defer_effect(callback);
                    }
                }
            } else {
                break;
            }
        }

        let dirty_window_ids = self
            .windows
            .iter()
            .filter_map(|(_, window)| {
                let window = window.as_ref().unwrap();
                if window.dirty {
                    Some(window.handle.clone())
                } else {
                    None
                }
            })
            .collect::<SmallVec<[_; 8]>>();

        for dirty_window_handle in dirty_window_ids {
            dirty_window_handle.update(self, |_, cx| cx.draw()).unwrap();
        }
    }

    /// Repeatedly called during `flush_effects` to release any entities whose
    /// reference count has become zero. We invoke any release observers before dropping
    /// each entity.
    fn release_dropped_entities(&mut self) {
        loop {
            let dropped = self.entities.take_dropped();
            if dropped.is_empty() {
                break;
            }

            for (entity_id, mut entity) in dropped {
                self.observers.remove(&entity_id);
                self.event_listeners.remove(&entity_id);
                for release_callback in self.release_listeners.remove(&entity_id) {
                    release_callback(entity.as_mut(), self);
                }
            }
        }
    }

    /// Repeatedly called during `flush_effects` to handle a focused handle being dropped.
    /// For now, we simply blur the window if this happens, but we may want to support invoking
    /// a window blur handler to restore focus to some logical element.
    fn release_dropped_focus_handles(&mut self) {
        for window_handle in self.windows() {
            window_handle
                .update(self, |_, cx| {
                    let mut blur_window = false;
                    let focus = cx.window.focus;
                    cx.window.focus_handles.write().retain(|handle_id, count| {
                        if count.load(SeqCst) == 0 {
                            if focus == Some(handle_id) {
                                blur_window = true;
                            }
                            false
                        } else {
                            true
                        }
                    });

                    if blur_window {
                        cx.blur();
                    }
                })
                .unwrap();
        }
    }

    fn apply_notify_effect(&mut self, emitter: EntityId) {
        self.pending_notifications.remove(&emitter);

        self.observers
            .clone()
            .retain(&emitter, |handler| handler(self));
    }

    fn apply_emit_effect(&mut self, emitter: EntityId, event_type: TypeId, event: Box<dyn Any>) {
        self.event_listeners
            .clone()
            .retain(&emitter, |(stored_type, handler)| {
                if *stored_type == event_type {
                    handler(event.as_ref(), self)
                } else {
                    true
                }
            });
    }

    fn apply_focus_changed_effect(
        &mut self,
        window_handle: AnyWindowHandle,
        focused: Option<FocusId>,
    ) {
        window_handle
            .update(self, |_, cx| {
                // The window might change focus multiple times in an effect cycle.
                // We only honor effects for the most recently focused handle.
                if cx.window.focus == focused {
                    let focused = focused
                        .map(|id| FocusHandle::for_id(id, &cx.window.focus_handles).unwrap());
                    let blurred = cx
                        .window
                        .last_blur
                        .take()
                        .unwrap()
                        .and_then(|id| FocusHandle::for_id(id, &cx.window.focus_handles));
                    let focus_changed = focused.is_some() || blurred.is_some();
                    let event = FocusEvent { focused, blurred };

                    let mut listeners = mem::take(&mut cx.window.current_frame.focus_listeners);
                    if focus_changed {
                        for listener in &mut listeners {
                            listener(&event, cx);
                        }
                    }
                    listeners.extend(cx.window.current_frame.focus_listeners.drain(..));
                    cx.window.current_frame.focus_listeners = listeners;

                    if focus_changed {
                        cx.window
                            .focus_listeners
                            .clone()
                            .retain(&(), |listener| listener(&event, cx));
                    }
                }
            })
            .ok();
    }

    fn apply_refresh_effect(&mut self) {
        for window in self.windows.values_mut() {
            if let Some(window) = window.as_mut() {
                window.dirty = true;
            }
        }
    }

    fn apply_notify_global_observers_effect(&mut self, type_id: TypeId) {
        self.pending_global_notifications.remove(&type_id);
        self.global_observers
            .clone()
            .retain(&type_id, |observer| observer(self));
    }

    fn apply_defer_effect(&mut self, callback: Box<dyn FnOnce(&mut Self) + 'static>) {
        callback(self);
    }

    /// Creates an `AsyncAppContext`, which can be cloned and has a static lifetime
    /// so it can be held across `await` points.
    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: unsafe { mem::transmute(self.this.clone()) },
            background_executor: self.background_executor.clone(),
            foreground_executor: self.foreground_executor.clone(),
        }
    }

    /// Obtains a reference to the executor, which can be used to spawn futures.
    pub fn background_executor(&self) -> &BackgroundExecutor {
        &self.background_executor
    }

    /// Obtains a reference to the executor, which can be used to spawn futures.
    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        &self.foreground_executor
    }

    /// Spawns the future returned by the given function on the thread pool. The closure will be invoked
    /// with AsyncAppContext, which allows the application state to be accessed across await points.
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn(f(self.to_async()))
    }

    /// Schedules the given function to be run at the end of the current effect cycle, allowing entities
    /// that are currently on the stack to be returned to the app.
    pub fn defer(&mut self, f: impl FnOnce(&mut AppContext) + 'static) {
        self.push_effect(Effect::Defer {
            callback: Box::new(f),
        });
    }

    /// Accessor for the application's asset source, which is provided when constructing the `App`.
    pub fn asset_source(&self) -> &Arc<dyn AssetSource> {
        &self.asset_source
    }

    /// Accessor for the text system.
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    /// The current text style. Which is composed of all the style refinements provided to `with_text_style`.
    pub fn text_style(&self) -> TextStyle {
        let mut style = TextStyle::default();
        for refinement in &self.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    /// Check whether a global of the given type has been assigned.
    pub fn has_global<G: 'static>(&self) -> bool {
        self.globals_by_type.contains_key(&TypeId::of::<G>())
    }

    /// Access the global of the given type. Panics if a global for that type has not been assigned.
    #[track_caller]
    pub fn global<G: 'static>(&self) -> &G {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    /// Access the global of the given type if a value has been assigned.
    pub fn try_global<G: 'static>(&self) -> Option<&G> {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
    }

    /// Access the global of the given type mutably. Panics if a global for that type has not been assigned.
    #[track_caller]
    pub fn global_mut<G: 'static>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .get_mut(&global_type)
            .and_then(|any_state| any_state.downcast_mut::<G>())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    /// Access the global of the given type mutably. A default value is assigned if a global of this type has not
    /// yet been assigned.
    pub fn default_global<G: 'static + Default>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .entry(global_type)
            .or_insert_with(|| Box::new(G::default()))
            .downcast_mut::<G>()
            .unwrap()
    }

    /// Set the value of the global of the given type.
    pub fn set_global<G: Any>(&mut self, global: G) {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, Box::new(global));
    }

    /// Clear all stored globals. Does not notify global observers.
    #[cfg(any(test, feature = "test-support"))]
    pub fn clear_globals(&mut self) {
        self.globals_by_type.drain();
    }

    /// Remove the global of the given type from the app context. Does not notify global observers.
    #[cfg(any(test, feature = "test-support"))]
    pub fn remove_global<G: Any>(&mut self) -> G {
        let global_type = TypeId::of::<G>();
        *self
            .globals_by_type
            .remove(&global_type)
            .unwrap_or_else(|| panic!("no global added for {}", std::any::type_name::<G>()))
            .downcast()
            .unwrap()
    }

    /// Update the global of the given type with a closure. Unlike `global_mut`, this method provides
    /// your closure with mutable access to the `AppContext` and the global simultaneously.
    pub fn update_global<G: 'static, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R {
        let mut global = self.lease_global::<G>();
        let result = f(&mut global, self);
        self.end_global_lease(global);
        result
    }

    /// Register a callback to be invoked when a global of the given type is updated.
    pub fn observe_global<G: 'static>(
        &mut self,
        mut f: impl FnMut(&mut Self) + 'static,
    ) -> Subscription {
        self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                f(cx);
                true
            }),
        )
    }

    /// Move the global of the given type to the stack.
    pub(crate) fn lease_global<G: 'static>(&mut self) -> GlobalLease<G> {
        GlobalLease::new(
            self.globals_by_type
                .remove(&TypeId::of::<G>())
                .ok_or_else(|| anyhow!("no global registered of type {}", type_name::<G>()))
                .unwrap(),
        )
    }

    /// Restore the global of the given type after it is moved to the stack.
    pub(crate) fn end_global_lease<G: 'static>(&mut self, lease: GlobalLease<G>) {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, lease.global);
    }

    pub fn observe_new_views<V: 'static>(
        &mut self,
        on_new: impl 'static + Fn(&mut V, &mut ViewContext<V>),
    ) -> Subscription {
        self.new_view_observers.insert(
            TypeId::of::<V>(),
            Box::new(move |any_view: AnyView, cx: &mut WindowContext| {
                any_view
                    .downcast::<V>()
                    .unwrap()
                    .update(cx, |view_state, cx| {
                        on_new(view_state, cx);
                    })
            }),
        )
    }

    pub fn observe_release<E, T>(
        &mut self,
        handle: &E,
        on_release: impl FnOnce(&mut T, &mut AppContext) + 'static,
    ) -> Subscription
    where
        E: Entity<T>,
        T: 'static,
    {
        self.release_listeners.insert(
            handle.entity_id(),
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                on_release(entity, cx)
            }),
        )
    }

    pub(crate) fn push_text_style(&mut self, text_style: TextStyleRefinement) {
        self.text_style_stack.push(text_style);
    }

    pub(crate) fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    /// Register key bindings.
    pub fn bind_keys(&mut self, bindings: impl IntoIterator<Item = KeyBinding>) {
        self.keymap.lock().add_bindings(bindings);
        self.pending_effects.push_back(Effect::Refresh);
    }

    /// Register a global listener for actions invoked via the keyboard.
    pub fn on_action<A: Action>(&mut self, listener: impl Fn(&A, &mut Self) + 'static) {
        self.global_action_listeners
            .entry(TypeId::of::<A>())
            .or_default()
            .push(Box::new(move |action, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    let action = action.as_any().downcast_ref().unwrap();
                    listener(action, cx)
                }
            }));
    }

    /// Event handlers propagate events by default. Call this method to stop dispatching to
    /// event handlers with a lower z-index (mouse) or higher in the tree (keyboard). This is
    /// the opposite of [propagate]. It's also possible to cancel a call to [propagate] by
    /// calling this method before effects are flushed.
    pub fn stop_propagation(&mut self) {
        self.propagate_event = false;
    }

    /// Action handlers stop propagation by default during the bubble phase of action dispatch
    /// dispatching to action handlers higher in the element tree. This is the opposite of
    /// [stop_propagation]. It's also possible to cancel a call to [stop_propagate] by calling
    /// this method before effects are flushed.
    pub fn propagate(&mut self) {
        self.propagate_event = true;
    }
}

impl Context for AppContext {
    type Result<T> = T;

    /// Build an entity that is owned by the application. The given function will be invoked with
    /// a `ModelContext` and must return an object representing the entity. A `Model` will be returned
    /// which can be used to access the entity in a context.
    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|cx| {
            let slot = cx.entities.reserve();
            let entity = build_model(&mut ModelContext::new(cx, slot.downgrade()));
            cx.entities.insert(slot, entity)
        })
    }

    /// Update the entity referenced by the given model. The function is passed a mutable reference to the
    /// entity along with a `ModelContext` for the entity.
    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> R {
        self.update(|cx| {
            let mut entity = cx.entities.lease(model);
            let result = update(&mut entity, &mut ModelContext::new(cx, model.downgrade()));
            cx.entities.end_lease(entity);
            result
        })
    }

    fn update_window<T, F>(&mut self, handle: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        self.update(|cx| {
            let mut window = cx
                .windows
                .get_mut(handle.id)
                .ok_or_else(|| anyhow!("window not found"))?
                .take()
                .unwrap();

            let root_view = window.root_view.clone().unwrap();
            let result = update(root_view, &mut WindowContext::new(cx, &mut window));

            if !window.removed {
                cx.windows
                    .get_mut(handle.id)
                    .ok_or_else(|| anyhow!("window not found"))?
                    .replace(window);
            }

            Ok(result)
        })
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let entity = self.entities.read(handle);
        read(entity, self)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let window = self
            .windows
            .get(window.id)
            .ok_or_else(|| anyhow!("window not found"))?
            .as_ref()
            .unwrap();

        let root_view = window.root_view.clone().unwrap();
        let view = root_view
            .downcast::<T>()
            .map_err(|_| anyhow!("root view's type has changed"))?;

        Ok(read(view.read(self), self))
    }
}

/// These effects are processed at the end of each application update cycle.
pub(crate) enum Effect {
    Notify {
        emitter: EntityId,
    },
    Emit {
        emitter: EntityId,
        event_type: TypeId,
        event: Box<dyn Any>,
    },
    FocusChanged {
        window_handle: AnyWindowHandle,
        focused: Option<FocusId>,
    },
    Refresh,
    NotifyGlobalObservers {
        global_type: TypeId,
    },
    Defer {
        callback: Box<dyn FnOnce(&mut AppContext) + 'static>,
    },
}

/// Wraps a global variable value during `update_global` while the value has been moved to the stack.
pub(crate) struct GlobalLease<G: 'static> {
    global: AnyBox,
    global_type: PhantomData<G>,
}

impl<G: 'static> GlobalLease<G> {
    fn new(global: AnyBox) -> Self {
        GlobalLease {
            global,
            global_type: PhantomData,
        }
    }
}

impl<G: 'static> Deref for GlobalLease<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.global.downcast_ref().unwrap()
    }
}

impl<G: 'static> DerefMut for GlobalLease<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.global.downcast_mut().unwrap()
    }
}

/// Contains state associated with an active drag operation, started by dragging an element
/// within the window or by dragging into the app from the underlying platform.
pub(crate) struct AnyDrag {
    pub view: AnyView,
    pub cursor_offset: Point<Pixels>,
}

#[derive(Clone)]
pub(crate) struct AnyTooltip {
    pub view: AnyView,
    pub cursor_offset: Point<Pixels>,
}
