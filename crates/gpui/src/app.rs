use std::{
    any::{type_name, TypeId},
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{atomic::Ordering::SeqCst, Arc},
    time::Duration,
};

use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use futures::{channel::oneshot, future::LocalBoxFuture, Future};
use slotmap::SlotMap;
use smol::future::FutureExt;
use time::UtcOffset;

pub use async_context::*;
use collections::{FxHashMap, FxHashSet, VecDeque};
pub use entity_map::*;
use http::{self, HttpClient};
pub use model_context::*;
#[cfg(any(test, feature = "test-support"))]
pub use test_context::*;
use util::ResultExt;

use crate::{
    current_platform, init_app_menus, Action, ActionRegistry, Any, AnyView, AnyWindowHandle,
    AppMetadata, AssetCache, AssetSource, BackgroundExecutor, ClipboardItem, Context,
    DispatchPhase, DisplayId, Entity, EventEmitter, ForegroundExecutor, Global, KeyBinding, Keymap,
    Keystroke, LayoutId, Menu, MenuItem, PathPromptOptions, Pixels, Platform, PlatformDisplay,
    Point, PromptBuilder, PromptHandle, PromptLevel, Render, RenderablePromptHandle, Reservation,
    SharedString, SubscriberSet, Subscription, SvgRenderer, Task, TextSystem, View, ViewContext,
    Window, WindowAppearance, WindowContext, WindowHandle, WindowId,
};

mod async_context;
mod entity_map;
mod model_context;
#[cfg(any(test, feature = "test-support"))]
mod test_context;

/// The duration for which futures returned from [AppContext::on_app_context] or [ModelContext::on_app_quit] can run before the application fully quits.
pub const SHUTDOWN_TIMEOUT: Duration = Duration::from_millis(100);

/// Temporary(?) wrapper around [`RefCell<AppContext>`] to help us debug any double borrows.
/// Strongly consider removing after stabilization.
#[doc(hidden)]
pub struct AppCell {
    app: RefCell<AppContext>,
}

impl AppCell {
    #[doc(hidden)]
    #[track_caller]
    pub fn borrow(&self) -> AppRef {
        if option_env!("TRACK_THREAD_BORROWS").is_some() {
            let thread_id = std::thread::current().id();
            eprintln!("borrowed {thread_id:?}");
        }
        AppRef(self.app.borrow())
    }

    #[doc(hidden)]
    #[track_caller]
    pub fn borrow_mut(&self) -> AppRefMut {
        if option_env!("TRACK_THREAD_BORROWS").is_some() {
            let thread_id = std::thread::current().id();
            eprintln!("borrowed {thread_id:?}");
        }
        AppRefMut(self.app.borrow_mut())
    }
}

#[doc(hidden)]
#[derive(Deref, DerefMut)]
pub struct AppRef<'a>(Ref<'a, AppContext>);

impl<'a> Drop for AppRef<'a> {
    fn drop(&mut self) {
        if option_env!("TRACK_THREAD_BORROWS").is_some() {
            let thread_id = std::thread::current().id();
            eprintln!("dropped borrow from {thread_id:?}");
        }
    }
}

#[doc(hidden)]
#[derive(Deref, DerefMut)]
pub struct AppRefMut<'a>(RefMut<'a, AppContext>);

impl<'a> Drop for AppRefMut<'a> {
    fn drop(&mut self) {
        if option_env!("TRACK_THREAD_BORROWS").is_some() {
            let thread_id = std::thread::current().id();
            eprintln!("dropped {thread_id:?}");
        }
    }
}

/// A reference to a GPUI application, typically constructed in the `main` function of your app.
/// You won't interact with this type much outside of initial configuration and startup.
pub struct App(Rc<AppCell>);

/// Represents an application before it is fully launched. Once your app is
/// configured, you'll start the app with `App::run`.
impl App {
    /// Builds an app with the given asset source.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[cfg(any(test, feature = "test-support"))]
        log::info!("GPUI was compiled in test mode");

        Self(AppContext::new(
            current_platform(),
            Arc::new(()),
            http::client(None),
        ))
    }

    /// Assign
    pub fn with_assets(self, asset_source: impl AssetSource) -> Self {
        let mut context_lock = self.0.borrow_mut();
        let asset_source = Arc::new(asset_source);
        context_lock.asset_source = asset_source.clone();
        context_lock.svg_renderer = SvgRenderer::new(asset_source);
        drop(context_lock);
        self
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
        F: 'static + FnMut(Vec<String>),
    {
        self.0.borrow().platform.on_open_urls(Box::new(callback));
        self
    }

    /// Invokes a handler when an already-running application is launched.
    /// On macOS, this can occur when the application icon is double-clicked or the app is launched via the dock.
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

    /// Returns metadata associated with the application
    pub fn metadata(&self) -> AppMetadata {
        self.0.borrow().app_metadata.clone()
    }

    /// Returns a handle to the [`BackgroundExecutor`] associated with this app, which can be used to spawn futures in the background.
    pub fn background_executor(&self) -> BackgroundExecutor {
        self.0.borrow().background_executor.clone()
    }

    /// Returns a handle to the [`ForegroundExecutor`] associated with this app, which can be used to spawn futures in the foreground.
    pub fn foreground_executor(&self) -> ForegroundExecutor {
        self.0.borrow().foreground_executor.clone()
    }

    /// Returns a reference to the [`TextSystem`] associated with this app.
    pub fn text_system(&self) -> Arc<TextSystem> {
        self.0.borrow().text_system.clone()
    }

    /// Returns the file URL of the executable with the specified name in the application bundle
    pub fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        self.0.borrow().path_for_auxiliary_executable(name)
    }
}

type Handler = Box<dyn FnMut(&mut AppContext) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut AppContext) -> bool + 'static>;
type KeystrokeObserver = Box<dyn FnMut(&KeystrokeEvent, &mut WindowContext) + 'static>;
type QuitHandler = Box<dyn FnOnce(&mut AppContext) -> LocalBoxFuture<'static, ()> + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut AppContext) + 'static>;
type NewViewListener = Box<dyn FnMut(AnyView, &mut WindowContext) + 'static>;

/// Contains the state of the full application, and passed as a reference to a variety of callbacks.
/// Other contexts such as [ModelContext], [WindowContext], and [ViewContext] deref to this type, making it the most general context type.
/// You need a reference to an `AppContext` to access the state of a [Model].
pub struct AppContext {
    pub(crate) this: Weak<AppCell>,
    pub(crate) platform: Rc<dyn Platform>,
    app_metadata: AppMetadata,
    text_system: Arc<TextSystem>,
    flushing_effects: bool,
    pending_updates: usize,
    pub(crate) actions: Rc<ActionRegistry>,
    pub(crate) active_drag: Option<AnyDrag>,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
    pub(crate) loading_assets: FxHashMap<(TypeId, u64), Box<dyn Any>>,
    pub(crate) asset_cache: AssetCache,
    asset_source: Arc<dyn AssetSource>,
    pub(crate) svg_renderer: SvgRenderer,
    http_client: Arc<dyn HttpClient>,
    pub(crate) globals_by_type: FxHashMap<TypeId, Box<dyn Any>>,
    pub(crate) entities: EntityMap,
    pub(crate) new_view_observers: SubscriberSet<TypeId, NewViewListener>,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    pub(crate) window_handles: FxHashMap<WindowId, AnyWindowHandle>,
    pub(crate) keymap: Rc<RefCell<Keymap>>,
    pub(crate) global_action_listeners:
        FxHashMap<TypeId, Vec<Rc<dyn Fn(&dyn Any, DispatchPhase, &mut Self)>>>,
    pending_effects: VecDeque<Effect>,
    pub(crate) pending_notifications: FxHashSet<EntityId>,
    pub(crate) pending_global_notifications: FxHashSet<TypeId>,
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    // TypeId is the type of the event that the listener callback expects
    pub(crate) event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    pub(crate) keystroke_observers: SubscriberSet<(), KeystrokeObserver>,
    pub(crate) release_listeners: SubscriberSet<EntityId, ReleaseListener>,
    pub(crate) global_observers: SubscriberSet<TypeId, Handler>,
    pub(crate) quit_observers: SubscriberSet<(), QuitHandler>,
    pub(crate) layout_id_buffer: Vec<LayoutId>, // We recycle this memory across layout requests.
    pub(crate) propagate_event: bool,
    pub(crate) prompt_builder: Option<PromptBuilder>,
}

impl AppContext {
    #[allow(clippy::new_ret_no_self)]
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

        let app = Rc::new_cyclic(|this| AppCell {
            app: RefCell::new(AppContext {
                this: this.clone(),
                platform: platform.clone(),
                app_metadata,
                text_system,
                actions: Rc::new(ActionRegistry::default()),
                flushing_effects: false,
                pending_updates: 0,
                active_drag: None,
                background_executor: executor,
                foreground_executor,
                svg_renderer: SvgRenderer::new(asset_source.clone()),
                asset_cache: AssetCache::new(),
                loading_assets: Default::default(),
                asset_source,
                http_client,
                globals_by_type: FxHashMap::default(),
                entities,
                new_view_observers: SubscriberSet::new(),
                window_handles: FxHashMap::default(),
                windows: SlotMap::with_key(),
                keymap: Rc::new(RefCell::new(Keymap::default())),
                global_action_listeners: FxHashMap::default(),
                pending_effects: VecDeque::new(),
                pending_notifications: FxHashSet::default(),
                pending_global_notifications: FxHashSet::default(),
                observers: SubscriberSet::new(),
                event_listeners: SubscriberSet::new(),
                release_listeners: SubscriberSet::new(),
                keystroke_observers: SubscriberSet::new(),
                global_observers: SubscriberSet::new(),
                quit_observers: SubscriberSet::new(),
                layout_id_buffer: Default::default(),
                propagate_event: true,
                prompt_builder: Some(PromptBuilder::Default),
            }),
        });

        init_app_menus(platform.as_ref(), &mut app.borrow_mut());

        platform.on_quit(Box::new({
            let cx = app.clone();
            move || {
                cx.borrow_mut().shutdown();
            }
        }));

        app
    }

    /// Quit the application gracefully. Handlers registered with [`ModelContext::on_app_quit`]
    /// will be given 100ms to complete before exiting.
    pub fn shutdown(&mut self) {
        let mut futures = Vec::new();

        for observer in self.quit_observers.remove(&()) {
            futures.push(observer(self));
        }

        self.windows.clear();
        self.window_handles.clear();
        self.flush_effects();

        let futures = futures::future::join_all(futures);
        if self
            .background_executor
            .block_with_timeout(SHUTDOWN_TIMEOUT, futures)
            .is_err()
        {
            log::error!("timed out waiting on app_will_quit");
        }
    }

    /// Gracefully quit the application via the platform's standard routine.
    pub fn quit(&mut self) {
        self.platform.quit();
    }

    /// Get metadata about the app and platform.
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

    /// Arrange a callback to be invoked when the given model or view calls `notify` on its respective context.
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

    pub(crate) fn new_observer(&mut self, key: EntityId, value: Handler) -> Subscription {
        let (subscription, activate) = self.observers.insert(key, value);
        self.defer(move |_| activate());
        subscription
    }
    pub(crate) fn observe_internal<W, E>(
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
        self.new_observer(
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

    /// Arrange for the given callback to be invoked whenever the given model or view emits an event of a given type.
    /// The callback is provided a handle to the emitting entity and a reference to the emitted event.
    pub fn subscribe<T, E, Event>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Event, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Event>,
        E: Entity<T>,
        Event: 'static,
    {
        self.subscribe_internal(entity, move |entity, event, cx| {
            on_event(entity, event, cx);
            true
        })
    }

    pub(crate) fn new_subscription(
        &mut self,
        key: EntityId,
        value: (TypeId, Listener),
    ) -> Subscription {
        let (subscription, activate) = self.event_listeners.insert(key, value);
        self.defer(move |_| activate());
        subscription
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
        self.new_subscription(
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

    /// Returns handles to all open windows in the application.
    /// Each handle could be downcast to a handle typed for the root view of that window.
    /// To find all windows of a given type, you could filter on
    pub fn windows(&self) -> Vec<AnyWindowHandle> {
        self.windows
            .keys()
            .flat_map(|window_id| self.window_handles.get(&window_id).copied())
            .collect()
    }

    /// Returns a handle to the window that is currently focused at the platform level, if one exists.
    pub fn active_window(&self) -> Option<AnyWindowHandle> {
        self.platform.active_window()
    }

    /// Opens a new window with the given option and the root view returned by the given function.
    /// The function is invoked with a `WindowContext`, which can be used to interact with window-specific
    /// functionality.
    pub fn open_window<V: 'static + Render>(
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
            cx.window_handles.insert(id, window.handle);
            cx.windows.get_mut(id).unwrap().replace(window);
            handle
        })
    }

    /// Returns Ok() if the platform supports opening windows.
    /// This returns false (for example) on linux when we could
    /// not establish a connection to X or Wayland.
    pub fn can_open_windows(&self) -> anyhow::Result<()> {
        self.platform.can_open_windows()
    }

    /// Instructs the platform to activate the application by bringing it to the foreground.
    pub fn activate(&self, ignoring_other_apps: bool) {
        self.platform.activate(ignoring_other_apps);
    }

    /// Hide the application at the platform level.
    pub fn hide(&self) {
        self.platform.hide();
    }

    /// Hide other applications at the platform level.
    pub fn hide_other_apps(&self) {
        self.platform.hide_other_apps();
    }

    /// Unhide other applications at the platform level.
    pub fn unhide_other_apps(&self) {
        self.platform.unhide_other_apps();
    }

    /// Returns the list of currently active displays.
    pub fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.platform.displays()
    }

    /// Returns the primary display that will be used for new windows.
    pub fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.platform.primary_display()
    }

    /// Returns the display with the given ID, if one exists.
    pub fn find_display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        self.displays()
            .iter()
            .find(|display| display.id() == id)
            .cloned()
    }

    /// Returns the appearance of the application's windows.
    pub fn window_appearance(&self) -> WindowAppearance {
        self.platform.window_appearance()
    }

    /// Writes data to the primary selection buffer.
    /// Only available on Linux.
    #[cfg(target_os = "linux")]
    pub fn write_to_primary(&self, item: ClipboardItem) {
        self.platform.write_to_primary(item)
    }

    /// Writes data to the platform clipboard.
    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform.write_to_clipboard(item)
    }

    /// Reads data from the primary selection buffer.
    /// Only available on Linux.
    #[cfg(target_os = "linux")]
    pub fn read_from_primary(&self) -> Option<ClipboardItem> {
        self.platform.read_from_primary()
    }

    /// Reads data from the platform clipboard.
    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform.read_from_clipboard()
    }

    /// Writes credentials to the platform keychain.
    pub fn write_credentials(
        &self,
        url: &str,
        username: &str,
        password: &[u8],
    ) -> Task<Result<()>> {
        self.platform.write_credentials(url, username, password)
    }

    /// Reads credentials from the platform keychain.
    pub fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        self.platform.read_credentials(url)
    }

    /// Deletes credentials from the platform keychain.
    pub fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        self.platform.delete_credentials(url)
    }

    /// Directs the platform's default browser to open the given URL.
    pub fn open_url(&self, url: &str) {
        self.platform.open_url(url);
    }

    /// register_url_scheme requests that the given scheme (e.g. `zed` for `zed://` urls)
    /// is opened by the current app.
    /// On some platforms (e.g. macOS) you may be able to register URL schemes as part of app
    /// distribution, but this method exists to let you register schemes at runtime.
    pub fn register_url_scheme(&self, scheme: &str) -> Task<Result<()>> {
        self.platform.register_url_scheme(scheme)
    }

    /// Returns the full pathname of the current app bundle.
    /// If the app is not being run from a bundle, returns an error.
    pub fn app_path(&self) -> Result<PathBuf> {
        self.platform.app_path()
    }

    /// Returns the file URL of the executable with the specified name in the application bundle
    pub fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        self.platform.path_for_auxiliary_executable(name)
    }

    /// Displays a platform modal for selecting paths.
    /// When one or more paths are selected, they'll be relayed asynchronously via the returned oneshot channel.
    /// If cancelled, a `None` will be relayed instead.
    pub fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        self.platform.prompt_for_paths(options)
    }

    /// Displays a platform modal for selecting a new path where a file can be saved.
    /// The provided directory will be used to set the initial location.
    /// When a path is selected, it is relayed asynchronously via the returned oneshot channel.
    /// If cancelled, a `None` will be relayed instead.
    pub fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        self.platform.prompt_for_new_path(directory)
    }

    /// Reveals the specified path at the platform level, such as in Finder on macOS.
    pub fn reveal_path(&self, path: &Path) {
        self.platform.reveal_path(path)
    }

    /// Returns whether the user has configured scrollbars to auto-hide at the platform level.
    pub fn should_auto_hide_scrollbars(&self) -> bool {
        self.platform.should_auto_hide_scrollbars()
    }

    /// Restart the application.
    pub fn restart(&self, binary_path: Option<PathBuf>) {
        self.platform.restart(binary_path)
    }

    /// Returns the local timezone at the platform level.
    pub fn local_timezone(&self) -> UtcOffset {
        self.platform.local_timezone()
    }

    /// Updates the http client assigned to GPUI
    pub fn update_http_client(&mut self, new_client: Arc<dyn HttpClient>) {
        self.http_client = new_client;
    }

    /// Returns the http client assigned to GPUI
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    /// Returns the SVG renderer GPUI uses
    pub(crate) fn svg_renderer(&self) -> SvgRenderer {
        self.svg_renderer.clone()
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

    /// Called at the end of [`AppContext::update`] to complete any side effects
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
                #[cfg(any(test, feature = "test-support"))]
                for window in self
                    .windows
                    .values()
                    .filter_map(|window| {
                        let window = window.as_ref()?;
                        window.dirty.get().then_some(window.handle)
                    })
                    .collect::<Vec<_>>()
                {
                    self.update_window(window, |_, cx| cx.draw()).unwrap();
                }

                if self.pending_effects.is_empty() {
                    break;
                }
            }
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

    fn apply_refresh_effect(&mut self) {
        for window in self.windows.values_mut() {
            if let Some(window) = window.as_mut() {
                window.dirty.set(true);
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
            app: self.this.clone(),
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
    /// with [AsyncAppContext], which allows the application state to be accessed across await points.
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

    /// Check whether a global of the given type has been assigned.
    pub fn has_global<G: Global>(&self) -> bool {
        self.globals_by_type.contains_key(&TypeId::of::<G>())
    }

    /// Access the global of the given type. Panics if a global for that type has not been assigned.
    #[track_caller]
    pub fn global<G: Global>(&self) -> &G {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    /// Access the global of the given type if a value has been assigned.
    pub fn try_global<G: Global>(&self) -> Option<&G> {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
    }

    /// Access the global of the given type mutably. Panics if a global for that type has not been assigned.
    #[track_caller]
    pub fn global_mut<G: Global>(&mut self) -> &mut G {
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
    pub fn default_global<G: Global + Default>(&mut self) -> &mut G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type
            .entry(global_type)
            .or_insert_with(|| Box::<G>::default())
            .downcast_mut::<G>()
            .unwrap()
    }

    /// Sets the value of the global of the given type.
    pub fn set_global<G: Global>(&mut self, global: G) {
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
    pub fn remove_global<G: Global>(&mut self) -> G {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        *self
            .globals_by_type
            .remove(&global_type)
            .unwrap_or_else(|| panic!("no global added for {}", std::any::type_name::<G>()))
            .downcast()
            .unwrap()
    }

    /// Register a callback to be invoked when a global of the given type is updated.
    pub fn observe_global<G: Global>(
        &mut self,
        mut f: impl FnMut(&mut Self) + 'static,
    ) -> Subscription {
        let (subscription, activate) = self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                f(cx);
                true
            }),
        );
        self.defer(move |_| activate());
        subscription
    }

    /// Move the global of the given type to the stack.
    pub(crate) fn lease_global<G: Global>(&mut self) -> GlobalLease<G> {
        GlobalLease::new(
            self.globals_by_type
                .remove(&TypeId::of::<G>())
                .ok_or_else(|| anyhow!("no global registered of type {}", type_name::<G>()))
                .unwrap(),
        )
    }

    /// Restore the global of the given type after it is moved to the stack.
    pub(crate) fn end_global_lease<G: Global>(&mut self, lease: GlobalLease<G>) {
        let global_type = TypeId::of::<G>();
        self.push_effect(Effect::NotifyGlobalObservers { global_type });
        self.globals_by_type.insert(global_type, lease.global);
    }

    pub(crate) fn new_view_observer(
        &mut self,
        key: TypeId,
        value: NewViewListener,
    ) -> Subscription {
        let (subscription, activate) = self.new_view_observers.insert(key, value);
        activate();
        subscription
    }
    /// Arrange for the given function to be invoked whenever a view of the specified type is created.
    /// The function will be passed a mutable reference to the view along with an appropriate context.
    pub fn observe_new_views<V: 'static>(
        &mut self,
        on_new: impl 'static + Fn(&mut V, &mut ViewContext<V>),
    ) -> Subscription {
        self.new_view_observer(
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

    /// Observe the release of a model or view. The callback is invoked after the model or view
    /// has no more strong references but before it has been dropped.
    pub fn observe_release<E, T>(
        &mut self,
        handle: &E,
        on_release: impl FnOnce(&mut T, &mut AppContext) + 'static,
    ) -> Subscription
    where
        E: Entity<T>,
        T: 'static,
    {
        let (subscription, activate) = self.release_listeners.insert(
            handle.entity_id(),
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                on_release(entity, cx)
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when a keystroke is received by the application
    /// in any window. Note that this fires after all other action and event mechanisms have resolved
    /// and that this API will not be invoked if the event's propagation is stopped.
    pub fn observe_keystrokes(
        &mut self,
        f: impl FnMut(&KeystrokeEvent, &mut WindowContext) + 'static,
    ) -> Subscription {
        fn inner(
            keystroke_observers: &mut SubscriberSet<(), KeystrokeObserver>,
            handler: KeystrokeObserver,
        ) -> Subscription {
            let (subscription, activate) = keystroke_observers.insert((), handler);
            activate();
            subscription
        }
        inner(&mut self.keystroke_observers, Box::new(f))
    }

    /// Register key bindings.
    pub fn bind_keys(&mut self, bindings: impl IntoIterator<Item = KeyBinding>) {
        self.keymap.borrow_mut().add_bindings(bindings);
        self.pending_effects.push_back(Effect::Refresh);
    }

    /// Clear all key bindings in the app.
    pub fn clear_key_bindings(&mut self) {
        self.keymap.borrow_mut().clear();
        self.pending_effects.push_back(Effect::Refresh);
    }

    /// Register a global listener for actions invoked via the keyboard.
    pub fn on_action<A: Action>(&mut self, listener: impl Fn(&A, &mut Self) + 'static) {
        self.global_action_listeners
            .entry(TypeId::of::<A>())
            .or_default()
            .push(Rc::new(move |action, phase, cx| {
                if phase == DispatchPhase::Bubble {
                    let action = action.downcast_ref().unwrap();
                    listener(action, cx)
                }
            }));
    }

    /// Event handlers propagate events by default. Call this method to stop dispatching to
    /// event handlers with a lower z-index (mouse) or higher in the tree (keyboard). This is
    /// the opposite of [`Self::propagate`]. It's also possible to cancel a call to [`Self::propagate`] by
    /// calling this method before effects are flushed.
    pub fn stop_propagation(&mut self) {
        self.propagate_event = false;
    }

    /// Action handlers stop propagation by default during the bubble phase of action dispatch
    /// dispatching to action handlers higher in the element tree. This is the opposite of
    /// [`Self::stop_propagation`]. It's also possible to cancel a call to [`Self::stop_propagation`] by calling
    /// this method before effects are flushed.
    pub fn propagate(&mut self) {
        self.propagate_event = true;
    }

    /// Build an action from some arbitrary data, typically a keymap entry.
    pub fn build_action(
        &self,
        name: &str,
        data: Option<serde_json::Value>,
    ) -> Result<Box<dyn Action>> {
        self.actions.build_action(name, data)
    }

    /// Get a list of all action names that have been registered.
    /// in the application. Note that registration only allows for
    /// actions to be built dynamically, and is unrelated to binding
    /// actions in the element tree.
    pub fn all_action_names(&self) -> &[SharedString] {
        self.actions.all_action_names()
    }

    /// Register a callback to be invoked when the application is about to quit.
    /// It is not possible to cancel the quit event at this point.
    pub fn on_app_quit<Fut>(
        &mut self,
        mut on_quit: impl FnMut(&mut AppContext) -> Fut + 'static,
    ) -> Subscription
    where
        Fut: 'static + Future<Output = ()>,
    {
        let (subscription, activate) = self.quit_observers.insert(
            (),
            Box::new(move |cx| {
                let future = on_quit(cx);
                future.boxed_local()
            }),
        );
        activate();
        subscription
    }

    pub(crate) fn clear_pending_keystrokes(&mut self) {
        for window in self.windows() {
            window
                .update(self, |_, cx| {
                    cx.window
                        .rendered_frame
                        .dispatch_tree
                        .clear_pending_keystrokes();
                    cx.window
                        .next_frame
                        .dispatch_tree
                        .clear_pending_keystrokes();
                })
                .ok();
        }
    }

    /// Checks if the given action is bound in the current context, as defined by the app's current focus,
    /// the bindings in the element tree, and any global action listeners.
    pub fn is_action_available(&mut self, action: &dyn Action) -> bool {
        let mut action_available = false;
        if let Some(window) = self.active_window() {
            if let Ok(window_action_available) =
                window.update(self, |_, cx| cx.is_action_available(action))
            {
                action_available = window_action_available;
            }
        }

        action_available
            || self
                .global_action_listeners
                .contains_key(&action.as_any().type_id())
    }

    /// Sets the menu bar for this application. This will replace any existing menu bar.
    pub fn set_menus(&mut self, menus: Vec<Menu>) {
        self.platform.set_menus(menus, &self.keymap.borrow());
    }

    /// Sets the right click menu for the app icon in the dock
    pub fn set_dock_menu(&mut self, menus: Vec<MenuItem>) {
        self.platform.set_dock_menu(menus, &self.keymap.borrow());
    }

    /// Adds given path to the bottom of the list of recent paths for the application.
    /// The list is usually shown on the application icon's context menu in the dock,
    /// and allows to open the recent files via that context menu.
    /// If the path is already in the list, it will be moved to the bottom of the list.
    pub fn add_recent_document(&mut self, path: &Path) {
        self.platform.add_recent_document(path);
    }

    /// Dispatch an action to the currently active window or global action handler
    /// See [action::Action] for more information on how actions work
    pub fn dispatch_action(&mut self, action: &dyn Action) {
        if let Some(active_window) = self.active_window() {
            active_window
                .update(self, |_, cx| cx.dispatch_action(action.boxed_clone()))
                .log_err();
        } else {
            self.dispatch_global_action(action);
        }
    }

    fn dispatch_global_action(&mut self, action: &dyn Action) {
        self.propagate_event = true;

        if let Some(mut global_listeners) = self
            .global_action_listeners
            .remove(&action.as_any().type_id())
        {
            for listener in &global_listeners {
                listener(action.as_any(), DispatchPhase::Capture, self);
                if !self.propagate_event {
                    break;
                }
            }

            global_listeners.extend(
                self.global_action_listeners
                    .remove(&action.as_any().type_id())
                    .unwrap_or_default(),
            );

            self.global_action_listeners
                .insert(action.as_any().type_id(), global_listeners);
        }

        if self.propagate_event {
            if let Some(mut global_listeners) = self
                .global_action_listeners
                .remove(&action.as_any().type_id())
            {
                for listener in global_listeners.iter().rev() {
                    listener(action.as_any(), DispatchPhase::Bubble, self);
                    if !self.propagate_event {
                        break;
                    }
                }

                global_listeners.extend(
                    self.global_action_listeners
                        .remove(&action.as_any().type_id())
                        .unwrap_or_default(),
                );

                self.global_action_listeners
                    .insert(action.as_any().type_id(), global_listeners);
            }
        }
    }

    /// Is there currently something being dragged?
    pub fn has_active_drag(&self) -> bool {
        self.active_drag.is_some()
    }

    /// Set the prompt renderer for GPUI. This will replace the default or platform specific
    /// prompts with this custom implementation.
    pub fn set_prompt_builder(
        &mut self,
        renderer: impl Fn(
                PromptLevel,
                &str,
                Option<&str>,
                &[&str],
                PromptHandle,
                &mut WindowContext,
            ) -> RenderablePromptHandle
            + 'static,
    ) {
        self.prompt_builder = Some(PromptBuilder::Custom(Box::new(renderer)))
    }
}

impl Context for AppContext {
    type Result<T> = T;

    /// Build an entity that is owned by the application. The given function will be invoked with
    /// a `ModelContext` and must return an object representing the entity. A `Model` handle will be returned,
    /// which can be used to access the entity in a context.
    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|cx| {
            let slot = cx.entities.reserve();
            let entity = build_model(&mut ModelContext::new(cx, slot.downgrade()));
            cx.entities.insert(slot, entity)
        })
    }

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<Reservation<T>> {
        Reservation(self.entities.reserve())
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>> {
        self.update(|cx| {
            let slot = reservation.0;
            let entity = build_model(&mut ModelContext::new(cx, slot.downgrade()));
            cx.entities.insert(slot, entity)
        })
    }

    /// Updates the entity referenced by the given model. The function is passed a mutable reference to the
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
                .ok_or_else(|| anyhow!("window not found"))?;

            let root_view = window.root_view.clone().unwrap();
            let result = update(root_view, &mut WindowContext::new(cx, &mut window));

            if window.removed {
                cx.window_handles.remove(&handle.id);
                cx.windows.remove(handle.id);
            } else {
                cx.windows
                    .get_mut(handle.id)
                    .ok_or_else(|| anyhow!("window not found"))?
                    .replace(window);
            }

            Ok(result)
        })
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
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

        Ok(read(view, self))
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
    Refresh,
    NotifyGlobalObservers {
        global_type: TypeId,
    },
    Defer {
        callback: Box<dyn FnOnce(&mut AppContext) + 'static>,
    },
}

/// Wraps a global variable value during `update_global` while the value has been moved to the stack.
pub(crate) struct GlobalLease<G: Global> {
    global: Box<dyn Any>,
    global_type: PhantomData<G>,
}

impl<G: Global> GlobalLease<G> {
    fn new(global: Box<dyn Any>) -> Self {
        GlobalLease {
            global,
            global_type: PhantomData,
        }
    }
}

impl<G: Global> Deref for GlobalLease<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.global.downcast_ref().unwrap()
    }
}

impl<G: Global> DerefMut for GlobalLease<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.global.downcast_mut().unwrap()
    }
}

/// Contains state associated with an active drag operation, started by dragging an element
/// within the window or by dragging into the app from the underlying platform.
pub struct AnyDrag {
    /// The view used to render this drag
    pub view: AnyView,

    /// The value of the dragged item, to be dropped
    pub value: Box<dyn Any>,

    /// This is used to render the dragged item in the same place
    /// on the original element that the drag was initiated
    pub cursor_offset: Point<Pixels>,
}

/// Contains state associated with a tooltip. You'll only need this struct if you're implementing
/// tooltip behavior on a custom element. Otherwise, use [Div::tooltip].
#[derive(Clone)]
pub struct AnyTooltip {
    /// The view used to display the tooltip
    pub view: AnyView,

    /// The absolute position of the mouse when the tooltip was deployed.
    pub mouse_position: Point<Pixels>,
}

/// A keystroke event, and potentially the associated action
#[derive(Debug)]
pub struct KeystrokeEvent {
    /// The keystroke that occurred
    pub keystroke: Keystroke,

    /// The action that was resolved for the keystroke, if any
    pub action: Option<Box<dyn Action>>,
}
