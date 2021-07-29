use crate::{
    elements::ElementBox,
    executor,
    keymap::{self, Keystroke},
    platform::{self, Platform, PromptLevel, WindowOptions},
    presenter::Presenter,
    util::{post_inc, timeout},
    AssetCache, AssetSource, ClipboardItem, EventContext, FontCache, PathPromptOptions,
    TextLayoutCache,
};
use anyhow::{anyhow, Result};
use async_task::Task;
use keymap::MatchResult;
use parking_lot::{Mutex, RwLock};
use pathfinder_geometry::{rect::RectF, vector::vec2f};
use platform::Event;
use postage::{mpsc, sink::Sink as _, stream::Stream as _};
use smol::prelude::*;
use std::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    rc::{self, Rc},
    sync::{Arc, Weak},
    time::Duration,
};

pub trait Entity: 'static + Send + Sync {
    type Event;

    fn release(&mut self, _: &mut MutableAppContext) {}
}

pub trait View: Entity {
    fn ui_name() -> &'static str;
    fn render<'a>(&self, cx: &AppContext) -> ElementBox;
    fn on_focus(&mut self, _: &mut ViewContext<Self>) {}
    fn on_blur(&mut self, _: &mut ViewContext<Self>) {}
    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        Self::default_keymap_context()
    }
    fn default_keymap_context() -> keymap::Context {
        let mut cx = keymap::Context::default();
        cx.set.insert(Self::ui_name().into());
        cx
    }
}

pub trait ReadModel {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T;
}

pub trait ReadModelWith {
    fn read_model_with<E: Entity, F: FnOnce(&E, &AppContext) -> T, T>(
        &self,
        handle: &ModelHandle<E>,
        read: F,
    ) -> T;
}

pub trait UpdateModel {
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S;
}

pub trait ReadView {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T;
}

pub trait ReadViewWith {
    fn read_view_with<V, F, T>(&self, handle: &ViewHandle<V>, read: F) -> T
    where
        V: View,
        F: FnOnce(&V, &AppContext) -> T;
}

pub trait UpdateView {
    fn update_view<T, F, S>(&mut self, handle: &ViewHandle<T>, update: F) -> S
    where
        T: View,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S;
}

pub struct Menu<'a> {
    pub name: &'a str,
    pub items: Vec<MenuItem<'a>>,
}

pub enum MenuItem<'a> {
    Action {
        name: &'a str,
        keystroke: Option<&'a str>,
        action: &'a str,
        arg: Option<Box<dyn Any + 'static>>,
    },
    Separator,
}

#[derive(Clone)]
pub struct App(Rc<RefCell<MutableAppContext>>);

#[derive(Clone)]
pub struct AsyncAppContext(Rc<RefCell<MutableAppContext>>);

pub struct BackgroundAppContext(*const RefCell<MutableAppContext>);

#[derive(Clone)]
pub struct TestAppContext {
    cx: Rc<RefCell<MutableAppContext>>,
    foreground_platform: Rc<platform::test::ForegroundPlatform>,
}

impl App {
    pub fn test<T, F: FnOnce(&mut MutableAppContext) -> T>(
        foreground_platform: Rc<platform::test::ForegroundPlatform>,
        platform: Arc<dyn Platform>,
        font_cache: Arc<FontCache>,
        f: F,
    ) -> T {
        let foreground = Rc::new(executor::Foreground::test());
        let cx = Rc::new(RefCell::new(MutableAppContext::new(
            foreground,
            Arc::new(executor::Background::new()),
            platform,
            foreground_platform,
            font_cache,
            (),
        )));
        cx.borrow_mut().weak_self = Some(Rc::downgrade(&cx));
        let mut cx = cx.borrow_mut();
        f(&mut *cx)
    }

    pub fn new(asset_source: impl AssetSource) -> Result<Self> {
        let platform = platform::current::platform();
        let foreground_platform = platform::current::foreground_platform();
        let foreground = Rc::new(executor::Foreground::platform(platform.dispatcher())?);
        let app = Self(Rc::new(RefCell::new(MutableAppContext::new(
            foreground,
            Arc::new(executor::Background::new()),
            platform.clone(),
            foreground_platform.clone(),
            Arc::new(FontCache::new(platform.fonts())),
            asset_source,
        ))));

        let cx = app.0.clone();
        foreground_platform.on_menu_command(Box::new(move |command, arg| {
            let mut cx = cx.borrow_mut();
            if let Some(key_window_id) = cx.cx.platform.key_window_id() {
                if let Some((presenter, _)) = cx.presenters_and_platform_windows.get(&key_window_id)
                {
                    let presenter = presenter.clone();
                    let path = presenter.borrow().dispatch_path(cx.as_ref());
                    cx.dispatch_action_any(key_window_id, &path, command, arg.unwrap_or(&()));
                } else {
                    cx.dispatch_global_action_any(command, arg.unwrap_or(&()));
                }
            } else {
                cx.dispatch_global_action_any(command, arg.unwrap_or(&()));
            }
        }));

        app.0.borrow_mut().weak_self = Some(Rc::downgrade(&app.0));
        Ok(app)
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

    pub fn on_event<F>(self, mut callback: F) -> Self
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

    pub fn on_open_files<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(Vec<PathBuf>, &mut MutableAppContext),
    {
        let cx = self.0.clone();
        self.0
            .borrow_mut()
            .foreground_platform
            .on_open_files(Box::new(move |paths| {
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
            on_finish_launching(&mut *cx);
        }))
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.0.borrow().cx.font_cache.clone()
    }

    fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.0.borrow_mut();
        state.pending_flushes += 1;
        let result = callback(&mut *state);
        state.flush_effects();
        result
    }
}

impl TestAppContext {
    pub fn new(
        foreground_platform: Rc<platform::test::ForegroundPlatform>,
        platform: Arc<dyn Platform>,
        foreground: Rc<executor::Foreground>,
        background: Arc<executor::Background>,
        font_cache: Arc<FontCache>,
        first_entity_id: usize,
    ) -> Self {
        let mut cx = MutableAppContext::new(
            foreground.clone(),
            background,
            platform,
            foreground_platform.clone(),
            font_cache,
            (),
        );
        cx.next_entity_id = first_entity_id;
        let cx = TestAppContext {
            cx: Rc::new(RefCell::new(cx)),
            foreground_platform,
        };
        cx.cx.borrow_mut().weak_self = Some(Rc::downgrade(&cx.cx));
        cx
    }

    pub fn dispatch_action<T: 'static + Any>(
        &self,
        window_id: usize,
        responder_chain: Vec<usize>,
        name: &str,
        arg: T,
    ) {
        self.cx.borrow_mut().dispatch_action_any(
            window_id,
            &responder_chain,
            name,
            Box::new(arg).as_ref(),
        );
    }

    pub fn dispatch_global_action<T: 'static + Any>(&self, name: &str, arg: T) {
        self.cx.borrow_mut().dispatch_global_action(name, arg);
    }

    pub fn dispatch_keystroke(
        &self,
        window_id: usize,
        responder_chain: Vec<usize>,
        keystroke: &Keystroke,
    ) -> Result<bool> {
        let mut state = self.cx.borrow_mut();
        state.dispatch_keystroke(window_id, responder_chain, keystroke)
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        let mut state = self.cx.borrow_mut();
        state.pending_flushes += 1;
        let handle = state.add_model(build_model);
        state.flush_effects();
        handle
    }

    pub fn add_window<T, F>(&mut self, build_root_view: F) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.cx.borrow_mut().add_window(build_root_view)
    }

    pub fn window_ids(&self) -> Vec<usize> {
        self.cx.borrow().window_ids().collect()
    }

    pub fn root_view<T: View>(&self, window_id: usize) -> Option<ViewHandle<T>> {
        self.cx.borrow().root_view(window_id)
    }

    pub fn add_view<T, F>(&mut self, window_id: usize, build_view: F) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        let mut state = self.cx.borrow_mut();
        state.pending_flushes += 1;
        let handle = state.add_view(window_id, build_view);
        state.flush_effects();
        handle
    }

    pub fn add_option_view<T, F>(
        &mut self,
        window_id: usize,
        build_view: F,
    ) -> Option<ViewHandle<T>>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> Option<T>,
    {
        let mut state = self.cx.borrow_mut();
        state.pending_flushes += 1;
        let handle = state.add_option_view(window_id, build_view);
        state.flush_effects();
        handle
    }

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&self, callback: F) -> T {
        callback(self.cx.borrow().as_ref())
    }

    pub fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.cx.borrow_mut();
        // Don't increment pending flushes in order to effects to be flushed before the callback
        // completes, which is helpful in tests.
        let result = callback(&mut *state);
        // Flush effects after the callback just in case there are any. This can happen in edge
        // cases such as the closure dropping handles.
        state.flush_effects();
        result
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext(self.cx.clone())
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.cx.borrow().cx.font_cache.clone()
    }

    pub fn platform(&self) -> Arc<dyn platform::Platform> {
        self.cx.borrow().cx.platform.clone()
    }

    pub fn foreground(&self) -> Rc<executor::Foreground> {
        self.cx.borrow().foreground().clone()
    }

    pub fn background(&self) -> Arc<executor::Background> {
        self.cx.borrow().background().clone()
    }

    pub fn simulate_new_path_selection(&self, result: impl FnOnce(PathBuf) -> Option<PathBuf>) {
        self.foreground_platform.simulate_new_path_selection(result);
    }

    pub fn did_prompt_for_new_path(&self) -> bool {
        self.foreground_platform.as_ref().did_prompt_for_new_path()
    }

    pub fn simulate_prompt_answer(&self, window_id: usize, answer: usize) {
        let mut state = self.cx.borrow_mut();
        let (_, window) = state
            .presenters_and_platform_windows
            .get_mut(&window_id)
            .unwrap();
        let test_window = window
            .as_any_mut()
            .downcast_mut::<platform::test::Window>()
            .unwrap();
        let callback = test_window
            .last_prompt
            .take()
            .expect("prompt was not called");
        (callback)(answer);
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

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&mut self, callback: F) -> T {
        callback(self.0.borrow().as_ref())
    }

    pub fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.0.borrow_mut();
        state.pending_flushes += 1;
        let result = callback(&mut *state);
        state.flush_effects();
        result
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.update(|cx| cx.add_model(build_model))
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
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        let mut state = self.0.borrow_mut();
        state.pending_flushes += 1;
        let result = state.update_model(handle, update);
        state.flush_effects();
        result
    }
}

impl ReadModelWith for AsyncAppContext {
    fn read_model_with<E: Entity, F: FnOnce(&E, &AppContext) -> T, T>(
        &self,
        handle: &ModelHandle<E>,
        read: F,
    ) -> T {
        let cx = self.0.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

impl UpdateView for AsyncAppContext {
    fn update_view<T, F, S>(&mut self, handle: &ViewHandle<T>, update: F) -> S
    where
        T: View,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        let mut state = self.0.borrow_mut();
        state.pending_flushes += 1;
        let result = state.update_view(handle, update);
        state.flush_effects();
        result
    }
}

impl ReadViewWith for AsyncAppContext {
    fn read_view_with<V, F, T>(&self, handle: &ViewHandle<V>, read: F) -> T
    where
        V: View,
        F: FnOnce(&V, &AppContext) -> T,
    {
        let cx = self.0.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

impl UpdateModel for TestAppContext {
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        let mut state = self.cx.borrow_mut();
        state.pending_flushes += 1;
        let result = state.update_model(handle, update);
        state.flush_effects();
        result
    }
}

impl ReadModelWith for TestAppContext {
    fn read_model_with<E: Entity, F: FnOnce(&E, &AppContext) -> T, T>(
        &self,
        handle: &ModelHandle<E>,
        read: F,
    ) -> T {
        let cx = self.cx.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

impl UpdateView for TestAppContext {
    fn update_view<T, F, S>(&mut self, handle: &ViewHandle<T>, update: F) -> S
    where
        T: View,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        let mut state = self.cx.borrow_mut();
        state.pending_flushes += 1;
        let result = state.update_view(handle, update);
        state.flush_effects();
        result
    }
}

impl ReadViewWith for TestAppContext {
    fn read_view_with<V, F, T>(&self, handle: &ViewHandle<V>, read: F) -> T
    where
        V: View,
        F: FnOnce(&V, &AppContext) -> T,
    {
        let cx = self.cx.borrow();
        let cx = cx.as_ref();
        read(handle.read(cx), cx)
    }
}

type ActionCallback =
    dyn FnMut(&mut dyn AnyView, &dyn Any, &mut MutableAppContext, usize, usize) -> bool;

type GlobalActionCallback = dyn FnMut(&dyn Any, &mut MutableAppContext);

pub struct MutableAppContext {
    weak_self: Option<rc::Weak<RefCell<Self>>>,
    foreground_platform: Rc<dyn platform::ForegroundPlatform>,
    assets: Arc<AssetCache>,
    cx: AppContext,
    actions: HashMap<TypeId, HashMap<String, Vec<Box<ActionCallback>>>>,
    global_actions: HashMap<String, Vec<Box<GlobalActionCallback>>>,
    keystroke_matcher: keymap::Matcher,
    next_entity_id: usize,
    next_window_id: usize,
    subscriptions: HashMap<usize, Vec<Subscription>>,
    model_observations: HashMap<usize, Vec<ModelObservation>>,
    view_observations: HashMap<usize, Vec<ViewObservation>>,
    presenters_and_platform_windows:
        HashMap<usize, (Rc<RefCell<Presenter>>, Box<dyn platform::Window>)>,
    debug_elements_callbacks: HashMap<usize, Box<dyn Fn(&AppContext) -> crate::json::Value>>,
    foreground: Rc<executor::Foreground>,
    pending_effects: VecDeque<Effect>,
    pending_flushes: usize,
    flushing_effects: bool,
}

impl MutableAppContext {
    fn new(
        foreground: Rc<executor::Foreground>,
        background: Arc<executor::Background>,
        platform: Arc<dyn platform::Platform>,
        foreground_platform: Rc<dyn platform::ForegroundPlatform>,
        font_cache: Arc<FontCache>,
        asset_source: impl AssetSource,
    ) -> Self {
        Self {
            weak_self: None,
            foreground_platform,
            assets: Arc::new(AssetCache::new(asset_source)),
            cx: AppContext {
                models: Default::default(),
                views: Default::default(),
                windows: Default::default(),
                values: Default::default(),
                ref_counts: Arc::new(Mutex::new(RefCounts::default())),
                background,
                font_cache,
                platform,
            },
            actions: HashMap::new(),
            global_actions: HashMap::new(),
            keystroke_matcher: keymap::Matcher::default(),
            next_entity_id: 0,
            next_window_id: 0,
            subscriptions: HashMap::new(),
            model_observations: HashMap::new(),
            view_observations: HashMap::new(),
            presenters_and_platform_windows: HashMap::new(),
            debug_elements_callbacks: HashMap::new(),
            foreground,
            pending_effects: VecDeque::new(),
            pending_flushes: 0,
            flushing_effects: false,
        }
    }

    pub fn upgrade(&self) -> App {
        App(self.weak_self.as_ref().unwrap().upgrade().unwrap())
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

    pub fn on_debug_elements<F>(&mut self, window_id: usize, callback: F)
    where
        F: 'static + Fn(&AppContext) -> crate::json::Value,
    {
        self.debug_elements_callbacks
            .insert(window_id, Box::new(callback));
    }

    pub fn debug_elements(&self, window_id: usize) -> Option<crate::json::Value> {
        self.debug_elements_callbacks
            .get(&window_id)
            .map(|debug_elements| debug_elements(&self.cx))
    }

    pub fn add_action<S, V, T, F>(&mut self, name: S, mut handler: F)
    where
        S: Into<String>,
        V: View,
        T: Any,
        F: 'static + FnMut(&mut V, &T, &mut ViewContext<V>),
    {
        let name = name.into();
        let name_clone = name.clone();
        let handler = Box::new(
            move |view: &mut dyn AnyView,
                  arg: &dyn Any,
                  cx: &mut MutableAppContext,
                  window_id: usize,
                  view_id: usize| {
                match arg.downcast_ref() {
                    Some(arg) => {
                        let mut cx = ViewContext::new(cx, window_id, view_id);
                        handler(
                            view.as_any_mut()
                                .downcast_mut()
                                .expect("downcast is type safe"),
                            arg,
                            &mut cx,
                        );
                        cx.halt_action_dispatch
                    }
                    None => {
                        log::error!("Could not downcast argument for action {}", name_clone);
                        false
                    }
                }
            },
        );

        self.actions
            .entry(TypeId::of::<V>())
            .or_default()
            .entry(name)
            .or_default()
            .push(handler);
    }

    pub fn add_global_action<S, T, F>(&mut self, name: S, mut handler: F)
    where
        S: Into<String>,
        T: 'static + Any,
        F: 'static + FnMut(&T, &mut MutableAppContext),
    {
        let name = name.into();
        let name_clone = name.clone();
        let handler = Box::new(move |arg: &dyn Any, cx: &mut MutableAppContext| {
            if let Some(arg) = arg.downcast_ref() {
                handler(arg, cx);
            } else {
                log::error!("Could not downcast argument for action {}", name_clone);
            }
        });

        self.global_actions.entry(name).or_default().push(handler);
    }

    pub fn window_ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.cx.windows.keys().cloned()
    }

    pub fn root_view<T: View>(&self, window_id: usize) -> Option<ViewHandle<T>> {
        self.cx
            .windows
            .get(&window_id)
            .and_then(|window| window.root_view.clone().downcast::<T>())
    }

    pub fn root_view_id(&self, window_id: usize) -> Option<usize> {
        self.cx.root_view_id(window_id)
    }

    pub fn focused_view_id(&self, window_id: usize) -> Option<usize> {
        self.cx.focused_view_id(window_id)
    }

    pub fn render_view(&self, window_id: usize, view_id: usize) -> Result<ElementBox> {
        self.cx.render_view(window_id, view_id)
    }

    pub fn render_views(&self, window_id: usize) -> HashMap<usize, ElementBox> {
        self.cx.render_views(window_id)
    }

    pub fn update<T, F: FnOnce() -> T>(&mut self, callback: F) -> T {
        self.pending_flushes += 1;
        let result = callback();
        self.flush_effects();
        result
    }

    pub fn set_menus(&mut self, menus: Vec<Menu>) {
        self.foreground_platform.set_menus(menus);
    }

    fn prompt<F>(
        &self,
        window_id: usize,
        level: PromptLevel,
        msg: &str,
        answers: &[&str],
        done_fn: F,
    ) where
        F: 'static + FnOnce(usize, &mut MutableAppContext),
    {
        let app = self.weak_self.as_ref().unwrap().upgrade().unwrap();
        let foreground = self.foreground.clone();
        let (_, window) = &self.presenters_and_platform_windows[&window_id];
        window.prompt(
            level,
            msg,
            answers,
            Box::new(move |answer| {
                foreground
                    .spawn(async move { (done_fn)(answer, &mut *app.borrow_mut()) })
                    .detach();
            }),
        );
    }

    pub fn prompt_for_paths<F>(&self, options: PathPromptOptions, done_fn: F)
    where
        F: 'static + FnOnce(Option<Vec<PathBuf>>, &mut MutableAppContext),
    {
        let app = self.weak_self.as_ref().unwrap().upgrade().unwrap();
        let foreground = self.foreground.clone();
        self.foreground_platform.prompt_for_paths(
            options,
            Box::new(move |paths| {
                foreground
                    .spawn(async move { (done_fn)(paths, &mut *app.borrow_mut()) })
                    .detach();
            }),
        );
    }

    pub fn prompt_for_new_path<F>(&self, directory: &Path, done_fn: F)
    where
        F: 'static + FnOnce(Option<PathBuf>, &mut MutableAppContext),
    {
        let app = self.weak_self.as_ref().unwrap().upgrade().unwrap();
        let foreground = self.foreground.clone();
        self.foreground_platform.prompt_for_new_path(
            directory,
            Box::new(move |path| {
                foreground
                    .spawn(async move { (done_fn)(path, &mut *app.borrow_mut()) })
                    .detach();
            }),
        );
    }

    pub(crate) fn notify_view(&mut self, window_id: usize, view_id: usize) {
        self.pending_effects
            .push_back(Effect::ViewNotification { window_id, view_id });
    }

    pub fn dispatch_action<T: 'static + Any>(
        &mut self,
        window_id: usize,
        responder_chain: Vec<usize>,
        name: &str,
        arg: T,
    ) {
        self.dispatch_action_any(window_id, &responder_chain, name, Box::new(arg).as_ref());
    }

    pub(crate) fn dispatch_action_any(
        &mut self,
        window_id: usize,
        path: &[usize],
        name: &str,
        arg: &dyn Any,
    ) -> bool {
        self.pending_flushes += 1;
        let mut halted_dispatch = false;

        for view_id in path.iter().rev() {
            if let Some(mut view) = self.cx.views.remove(&(window_id, *view_id)) {
                let type_id = view.as_any().type_id();

                if let Some((name, mut handlers)) = self
                    .actions
                    .get_mut(&type_id)
                    .and_then(|h| h.remove_entry(name))
                {
                    for handler in handlers.iter_mut().rev() {
                        let halt_dispatch = handler(view.as_mut(), arg, self, window_id, *view_id);
                        if halt_dispatch {
                            halted_dispatch = true;
                            break;
                        }
                    }
                    self.actions
                        .get_mut(&type_id)
                        .unwrap()
                        .insert(name, handlers);
                }

                self.cx.views.insert((window_id, *view_id), view);

                if halted_dispatch {
                    break;
                }
            }
        }

        if !halted_dispatch {
            self.dispatch_global_action_any(name, arg);
        }

        self.flush_effects();
        halted_dispatch
    }

    pub fn dispatch_global_action<T: 'static + Any>(&mut self, name: &str, arg: T) {
        self.dispatch_global_action_any(name, Box::new(arg).as_ref());
    }

    fn dispatch_global_action_any(&mut self, name: &str, arg: &dyn Any) {
        if let Some((name, mut handlers)) = self.global_actions.remove_entry(name) {
            self.pending_flushes += 1;
            for handler in handlers.iter_mut().rev() {
                handler(arg, self);
            }
            self.global_actions.insert(name, handlers);
            self.flush_effects();
        }
    }

    pub fn add_bindings<T: IntoIterator<Item = keymap::Binding>>(&mut self, bindings: T) {
        self.keystroke_matcher.add_bindings(bindings);
    }

    pub fn dispatch_keystroke(
        &mut self,
        window_id: usize,
        responder_chain: Vec<usize>,
        keystroke: &Keystroke,
    ) -> Result<bool> {
        let mut context_chain = Vec::new();
        let mut context = keymap::Context::default();
        for view_id in &responder_chain {
            if let Some(view) = self.cx.views.get(&(window_id, *view_id)) {
                context.extend(view.keymap_context(self.as_ref()));
                context_chain.push(context.clone());
            } else {
                return Err(anyhow!(
                    "View {} in responder chain does not exist",
                    view_id
                ));
            }
        }

        let mut pending = false;
        for (i, cx) in context_chain.iter().enumerate().rev() {
            match self
                .keystroke_matcher
                .push_keystroke(keystroke.clone(), responder_chain[i], cx)
            {
                MatchResult::None => {}
                MatchResult::Pending => pending = true,
                MatchResult::Action { name, arg } => {
                    if self.dispatch_action_any(
                        window_id,
                        &responder_chain[0..=i],
                        &name,
                        arg.as_ref().map(|arg| arg.as_ref()).unwrap_or(&()),
                    ) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(pending)
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        self.pending_flushes += 1;
        let model_id = post_inc(&mut self.next_entity_id);
        let handle = ModelHandle::new(model_id, &self.cx.ref_counts);
        let mut cx = ModelContext::new(self, model_id);
        let model = build_model(&mut cx);
        self.cx.models.insert(model_id, Box::new(model));
        self.flush_effects();
        handle
    }

    pub fn add_window<T, F>(&mut self, build_root_view: F) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.pending_flushes += 1;
        let window_id = post_inc(&mut self.next_window_id);
        let root_view = self.add_view(window_id, build_root_view);

        self.cx.windows.insert(
            window_id,
            Window {
                root_view: root_view.clone().into(),
                focused_view_id: root_view.id(),
                invalidation: None,
            },
        );
        self.open_platform_window(window_id);
        root_view.update(self, |view, cx| {
            view.on_focus(cx);
            cx.notify();
        });
        self.flush_effects();

        (window_id, root_view)
    }

    pub fn remove_window(&mut self, window_id: usize) {
        self.cx.windows.remove(&window_id);
        self.presenters_and_platform_windows.remove(&window_id);
        self.remove_dropped_entities();
    }

    fn open_platform_window(&mut self, window_id: usize) {
        let mut window = self.cx.platform.open_window(
            window_id,
            WindowOptions {
                bounds: RectF::new(vec2f(0., 0.), vec2f(1024., 768.)),
                title: "Zed".into(),
            },
            self.foreground.clone(),
        );
        let text_layout_cache = TextLayoutCache::new(self.cx.platform.fonts());
        let presenter = Rc::new(RefCell::new(Presenter::new(
            window_id,
            self.cx.font_cache.clone(),
            text_layout_cache,
            self.assets.clone(),
            self,
        )));

        {
            let mut app = self.upgrade();
            let presenter = presenter.clone();
            window.on_event(Box::new(move |event| {
                app.update(|cx| {
                    if let Event::KeyDown { keystroke, .. } = &event {
                        if cx
                            .dispatch_keystroke(
                                window_id,
                                presenter.borrow().dispatch_path(cx.as_ref()),
                                keystroke,
                            )
                            .unwrap()
                        {
                            return;
                        }
                    }

                    presenter.borrow_mut().dispatch_event(event, cx);
                })
            }));
        }

        {
            let mut app = self.upgrade();
            let presenter = presenter.clone();
            window.on_resize(Box::new(move |window| {
                app.update(|cx| {
                    let scene = presenter.borrow_mut().build_scene(
                        window.size(),
                        window.scale_factor(),
                        cx,
                    );
                    window.present_scene(scene);
                })
            }));
        }

        {
            let mut app = self.upgrade();
            window.on_close(Box::new(move || {
                app.update(|cx| cx.remove_window(window_id));
            }));
        }

        self.presenters_and_platform_windows
            .insert(window_id, (presenter.clone(), window));

        self.on_debug_elements(window_id, move |cx| {
            presenter.borrow().debug_elements(cx).unwrap()
        });
    }

    pub fn add_view<T, F>(&mut self, window_id: usize, build_view: F) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.add_option_view(window_id, |cx| Some(build_view(cx)))
            .unwrap()
    }

    pub fn add_option_view<T, F>(
        &mut self,
        window_id: usize,
        build_view: F,
    ) -> Option<ViewHandle<T>>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> Option<T>,
    {
        let view_id = post_inc(&mut self.next_entity_id);
        self.pending_flushes += 1;
        let handle = ViewHandle::new(window_id, view_id, &self.cx.ref_counts);
        let mut cx = ViewContext::new(self, window_id, view_id);
        let handle = if let Some(view) = build_view(&mut cx) {
            self.cx.views.insert((window_id, view_id), Box::new(view));
            if let Some(window) = self.cx.windows.get_mut(&window_id) {
                window
                    .invalidation
                    .get_or_insert_with(Default::default)
                    .updated
                    .insert(view_id);
            }
            Some(handle)
        } else {
            None
        };
        self.flush_effects();
        handle
    }

    fn remove_dropped_entities(&mut self) {
        loop {
            let (dropped_models, dropped_views, dropped_values) =
                self.cx.ref_counts.lock().take_dropped();
            if dropped_models.is_empty() && dropped_views.is_empty() && dropped_values.is_empty() {
                break;
            }

            for model_id in dropped_models {
                self.subscriptions.remove(&model_id);
                self.model_observations.remove(&model_id);
                let mut model = self.cx.models.remove(&model_id).unwrap();
                model.release(self);
            }

            for (window_id, view_id) in dropped_views {
                self.subscriptions.remove(&view_id);
                self.model_observations.remove(&view_id);
                let mut view = self.cx.views.remove(&(window_id, view_id)).unwrap();
                view.release(self);
                let change_focus_to = self.cx.windows.get_mut(&window_id).and_then(|window| {
                    window
                        .invalidation
                        .get_or_insert_with(Default::default)
                        .removed
                        .push(view_id);
                    if window.focused_view_id == view_id {
                        Some(window.root_view.id())
                    } else {
                        None
                    }
                });

                if let Some(view_id) = change_focus_to {
                    self.focus(window_id, view_id);
                }
            }

            let mut values = self.cx.values.write();
            for key in dropped_values {
                values.remove(&key);
            }
        }
    }

    fn flush_effects(&mut self) {
        self.pending_flushes = self.pending_flushes.saturating_sub(1);

        if !self.flushing_effects && self.pending_flushes == 0 {
            self.flushing_effects = true;

            loop {
                if let Some(effect) = self.pending_effects.pop_front() {
                    match effect {
                        Effect::Event { entity_id, payload } => self.emit_event(entity_id, payload),
                        Effect::ModelNotification { model_id } => {
                            self.notify_model_observers(model_id)
                        }
                        Effect::ViewNotification { window_id, view_id } => {
                            self.notify_view_observers(window_id, view_id)
                        }
                        Effect::Focus { window_id, view_id } => {
                            self.focus(window_id, view_id);
                        }
                    }
                    self.remove_dropped_entities();
                } else {
                    self.remove_dropped_entities();
                    self.update_windows();

                    if self.pending_effects.is_empty() {
                        self.flushing_effects = false;
                        break;
                    }
                }
            }
        }
    }

    fn update_windows(&mut self) {
        let mut invalidations = HashMap::new();
        for (window_id, window) in &mut self.cx.windows {
            if let Some(invalidation) = window.invalidation.take() {
                invalidations.insert(*window_id, invalidation);
            }
        }

        for (window_id, invalidation) in invalidations {
            if let Some((presenter, mut window)) =
                self.presenters_and_platform_windows.remove(&window_id)
            {
                {
                    let mut presenter = presenter.borrow_mut();
                    presenter.invalidate(invalidation, self.as_ref());
                    let scene = presenter.build_scene(window.size(), window.scale_factor(), self);
                    window.present_scene(scene);
                }
                self.presenters_and_platform_windows
                    .insert(window_id, (presenter, window));
            }
        }
    }

    fn emit_event(&mut self, entity_id: usize, payload: Box<dyn Any>) {
        if let Some(subscriptions) = self.subscriptions.remove(&entity_id) {
            for mut subscription in subscriptions {
                let alive = match &mut subscription {
                    Subscription::FromModel { model_id, callback } => {
                        if let Some(mut model) = self.cx.models.remove(model_id) {
                            callback(model.as_any_mut(), payload.as_ref(), self, *model_id);
                            self.cx.models.insert(*model_id, model);
                            true
                        } else {
                            false
                        }
                    }
                    Subscription::FromView {
                        window_id,
                        view_id,
                        callback,
                    } => {
                        if let Some(mut view) = self.cx.views.remove(&(*window_id, *view_id)) {
                            callback(
                                view.as_any_mut(),
                                payload.as_ref(),
                                self,
                                *window_id,
                                *view_id,
                            );
                            self.cx.views.insert((*window_id, *view_id), view);
                            true
                        } else {
                            false
                        }
                    }
                };

                if alive {
                    self.subscriptions
                        .entry(entity_id)
                        .or_default()
                        .push(subscription);
                }
            }
        }
    }

    fn notify_model_observers(&mut self, observed_id: usize) {
        if let Some(observations) = self.model_observations.remove(&observed_id) {
            if self.cx.models.contains_key(&observed_id) {
                for mut observation in observations {
                    let alive = match &mut observation {
                        ModelObservation::FromModel { model_id, callback } => {
                            if let Some(mut model) = self.cx.models.remove(model_id) {
                                callback(model.as_any_mut(), observed_id, self, *model_id);
                                self.cx.models.insert(*model_id, model);
                                true
                            } else {
                                false
                            }
                        }
                        ModelObservation::FromView {
                            window_id,
                            view_id,
                            callback,
                        } => {
                            if let Some(mut view) = self.cx.views.remove(&(*window_id, *view_id)) {
                                callback(
                                    view.as_any_mut(),
                                    observed_id,
                                    self,
                                    *window_id,
                                    *view_id,
                                );
                                self.cx.views.insert((*window_id, *view_id), view);
                                true
                            } else {
                                false
                            }
                        }
                    };

                    if alive {
                        self.model_observations
                            .entry(observed_id)
                            .or_default()
                            .push(observation);
                    }
                }
            }
        }
    }

    fn notify_view_observers(&mut self, window_id: usize, view_id: usize) {
        if let Some(window) = self.cx.windows.get_mut(&window_id) {
            window
                .invalidation
                .get_or_insert_with(Default::default)
                .updated
                .insert(view_id);
        }

        if let Some(observations) = self.view_observations.remove(&view_id) {
            if self.cx.views.contains_key(&(window_id, view_id)) {
                for mut observation in observations {
                    let alive = if let Some(mut view) = self
                        .cx
                        .views
                        .remove(&(observation.window_id, observation.view_id))
                    {
                        (observation.callback)(
                            view.as_any_mut(),
                            view_id,
                            window_id,
                            self,
                            observation.window_id,
                            observation.view_id,
                        );
                        self.cx
                            .views
                            .insert((observation.window_id, observation.view_id), view);
                        true
                    } else {
                        false
                    };

                    if alive {
                        self.view_observations
                            .entry(view_id)
                            .or_default()
                            .push(observation);
                    }
                }
            }
        }
    }

    fn focus(&mut self, window_id: usize, focused_id: usize) {
        if self
            .cx
            .windows
            .get(&window_id)
            .map(|w| w.focused_view_id)
            .map_or(false, |cur_focused| cur_focused == focused_id)
        {
            return;
        }

        self.pending_flushes += 1;

        let blurred_id = self.cx.windows.get_mut(&window_id).map(|window| {
            let blurred_id = window.focused_view_id;
            window.focused_view_id = focused_id;
            blurred_id
        });

        if let Some(blurred_id) = blurred_id {
            if let Some(mut blurred_view) = self.cx.views.remove(&(window_id, blurred_id)) {
                blurred_view.on_blur(self, window_id, blurred_id);
                self.cx.views.insert((window_id, blurred_id), blurred_view);
            }
        }

        if let Some(mut focused_view) = self.cx.views.remove(&(window_id, focused_id)) {
            focused_view.on_focus(self, window_id, focused_id);
            self.cx.views.insert((window_id, focused_id), focused_view);
        }

        self.flush_effects();
    }

    pub fn spawn<F, Fut, T>(&self, f: F) -> Task<T>
    where
        F: FnOnce(AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = T>,
        T: 'static,
    {
        let cx = self.to_async();
        self.foreground.spawn(f(cx))
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
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        if let Some(mut model) = self.cx.models.remove(&handle.model_id) {
            self.pending_flushes += 1;
            let mut cx = ModelContext::new(self, handle.model_id);
            let result = update(
                model
                    .as_any_mut()
                    .downcast_mut()
                    .expect("downcast is type safe"),
                &mut cx,
            );
            self.cx.models.insert(handle.model_id, model);
            self.flush_effects();
            result
        } else {
            panic!("circular model update");
        }
    }
}

impl ReadView for MutableAppContext {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        if let Some(view) = self.cx.views.get(&(handle.window_id, handle.view_id)) {
            view.as_any().downcast_ref().expect("downcast is type safe")
        } else {
            panic!("circular view reference");
        }
    }
}

impl UpdateView for MutableAppContext {
    fn update_view<T, F, S>(&mut self, handle: &ViewHandle<T>, update: F) -> S
    where
        T: View,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        self.pending_flushes += 1;
        let mut view = self
            .cx
            .views
            .remove(&(handle.window_id, handle.view_id))
            .expect("circular view update");

        let mut cx = ViewContext::new(self, handle.window_id, handle.view_id);
        let result = update(
            view.as_any_mut()
                .downcast_mut()
                .expect("downcast is type safe"),
            &mut cx,
        );
        self.cx
            .views
            .insert((handle.window_id, handle.view_id), view);
        self.flush_effects();
        result
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

pub struct AppContext {
    models: HashMap<usize, Box<dyn AnyModel>>,
    views: HashMap<(usize, usize), Box<dyn AnyView>>,
    windows: HashMap<usize, Window>,
    values: RwLock<HashMap<(TypeId, usize), Box<dyn Any>>>,
    background: Arc<executor::Background>,
    ref_counts: Arc<Mutex<RefCounts>>,
    font_cache: Arc<FontCache>,
    platform: Arc<dyn Platform>,
}

impl AppContext {
    pub fn root_view_id(&self, window_id: usize) -> Option<usize> {
        self.windows
            .get(&window_id)
            .map(|window| window.root_view.id())
    }

    pub fn focused_view_id(&self, window_id: usize) -> Option<usize> {
        self.windows
            .get(&window_id)
            .map(|window| window.focused_view_id)
    }

    pub fn render_view(&self, window_id: usize, view_id: usize) -> Result<ElementBox> {
        self.views
            .get(&(window_id, view_id))
            .map(|v| v.render(self))
            .ok_or(anyhow!("view not found"))
    }

    pub fn render_views(&self, window_id: usize) -> HashMap<usize, ElementBox> {
        self.views
            .iter()
            .filter_map(|((win_id, view_id), view)| {
                if *win_id == window_id {
                    Some((*view_id, view.render(self)))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, ElementBox>>()
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

    pub fn value<Tag: 'static, T: 'static + Default>(&self, id: usize) -> ValueHandle<T> {
        let key = (TypeId::of::<Tag>(), id);
        self.values
            .write()
            .entry(key)
            .or_insert_with(|| Box::new(T::default()));
        ValueHandle::new(TypeId::of::<Tag>(), id, &self.ref_counts)
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
    focused_view_id: usize,
    invalidation: Option<WindowInvalidation>,
}

#[derive(Default, Clone)]
pub struct WindowInvalidation {
    pub updated: HashSet<usize>,
    pub removed: Vec<usize>,
}

pub enum Effect {
    Event {
        entity_id: usize,
        payload: Box<dyn Any>,
    },
    ModelNotification {
        model_id: usize,
    },
    ViewNotification {
        window_id: usize,
        view_id: usize,
    },
    Focus {
        window_id: usize,
        view_id: usize,
    },
}

impl Debug for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Effect::Event { entity_id, .. } => f
                .debug_struct("Effect::Event")
                .field("entity_id", entity_id)
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
            Effect::Focus { window_id, view_id } => f
                .debug_struct("Effect::Focus")
                .field("window_id", window_id)
                .field("view_id", view_id)
                .finish(),
        }
    }
}

pub trait AnyModel: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn release(&mut self, cx: &mut MutableAppContext);
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
}

pub trait AnyView: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn release(&mut self, cx: &mut MutableAppContext);
    fn ui_name(&self) -> &'static str;
    fn render<'a>(&self, cx: &AppContext) -> ElementBox;
    fn on_focus(&mut self, cx: &mut MutableAppContext, window_id: usize, view_id: usize);
    fn on_blur(&mut self, cx: &mut MutableAppContext, window_id: usize, view_id: usize);
    fn keymap_context(&self, cx: &AppContext) -> keymap::Context;
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

    fn ui_name(&self) -> &'static str {
        T::ui_name()
    }

    fn render<'a>(&self, cx: &AppContext) -> ElementBox {
        View::render(self, cx)
    }

    fn on_focus(&mut self, cx: &mut MutableAppContext, window_id: usize, view_id: usize) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::on_focus(self, &mut cx);
    }

    fn on_blur(&mut self, cx: &mut MutableAppContext, window_id: usize, view_id: usize) {
        let mut cx = ViewContext::new(cx, window_id, view_id);
        View::on_blur(self, &mut cx);
    }

    fn keymap_context(&self, cx: &AppContext) -> keymap::Context {
        View::keymap_context(self, cx)
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

    pub fn subscribe<S: Entity, F>(&mut self, handle: &ModelHandle<S>, mut callback: F)
    where
        S::Event: 'static,
        F: 'static + FnMut(&mut T, &S::Event, &mut ModelContext<T>),
    {
        self.app
            .subscriptions
            .entry(handle.model_id)
            .or_default()
            .push(Subscription::FromModel {
                model_id: self.model_id,
                callback: Box::new(move |model, payload, app, model_id| {
                    let model = model.downcast_mut().expect("downcast is type safe");
                    let payload = payload.downcast_ref().expect("downcast is type safe");
                    let mut cx = ModelContext::new(app, model_id);
                    callback(model, payload, &mut cx);
                }),
            });
    }

    pub fn emit(&mut self, payload: T::Event) {
        self.app.pending_effects.push_back(Effect::Event {
            entity_id: self.model_id,
            payload: Box::new(payload),
        });
    }

    pub fn observe<S, F>(&mut self, handle: &ModelHandle<S>, mut callback: F)
    where
        S: Entity,
        F: 'static + FnMut(&mut T, ModelHandle<S>, &mut ModelContext<T>),
    {
        self.app
            .model_observations
            .entry(handle.model_id)
            .or_default()
            .push(ModelObservation::FromModel {
                model_id: self.model_id,
                callback: Box::new(move |model, observed_id, app, model_id| {
                    let model = model.downcast_mut().expect("downcast is type safe");
                    let observed = ModelHandle::new(observed_id, &app.cx.ref_counts);
                    let mut cx = ModelContext::new(app, model_id);
                    callback(model, observed, &mut cx);
                }),
            });
    }

    pub fn notify(&mut self) {
        self.app
            .pending_effects
            .push_back(Effect::ModelNotification {
                model_id: self.model_id,
            });
    }

    pub fn handle(&self) -> ModelHandle<T> {
        ModelHandle::new(self.model_id, &self.app.cx.ref_counts)
    }

    pub fn spawn<F, Fut, S>(&self, f: F) -> Task<S>
    where
        F: FnOnce(ModelHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.handle();
        self.app.spawn(|cx| f(handle, cx))
    }

    pub fn spawn_weak<F, Fut, S>(&self, f: F) -> Task<S>
    where
        F: FnOnce(WeakModelHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.handle().downgrade();
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
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        self.app.update_model(handle, update)
    }
}

impl<M> Deref for ModelContext<'_, M> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        &self.app
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
    halt_action_dispatch: bool,
}

impl<'a, T: View> ViewContext<'a, T> {
    fn new(app: &'a mut MutableAppContext, window_id: usize, view_id: usize) -> Self {
        Self {
            app,
            window_id,
            view_id,
            view_type: PhantomData,
            halt_action_dispatch: true,
        }
    }

    pub fn handle(&self) -> ViewHandle<T> {
        ViewHandle::new(self.window_id, self.view_id, &self.app.cx.ref_counts)
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

    pub fn prompt<F>(&self, level: PromptLevel, msg: &str, answers: &[&str], done_fn: F)
    where
        F: 'static + FnOnce(usize, &mut MutableAppContext),
    {
        self.app
            .prompt(self.window_id, level, msg, answers, done_fn)
    }

    pub fn prompt_for_paths<F>(&self, options: PathPromptOptions, done_fn: F)
    where
        F: 'static + FnOnce(Option<Vec<PathBuf>>, &mut MutableAppContext),
    {
        self.app.prompt_for_paths(options, done_fn)
    }

    pub fn prompt_for_new_path<F>(&self, directory: &Path, done_fn: F)
    where
        F: 'static + FnOnce(Option<PathBuf>, &mut MutableAppContext),
    {
        self.app.prompt_for_new_path(directory, done_fn)
    }

    pub fn debug_elements(&self) -> crate::json::Value {
        self.app.debug_elements(self.window_id).unwrap()
    }

    pub fn focus<S>(&mut self, handle: S)
    where
        S: Into<AnyViewHandle>,
    {
        let handle = handle.into();
        self.app.pending_effects.push_back(Effect::Focus {
            window_id: handle.window_id,
            view_id: handle.view_id,
        });
    }

    pub fn focus_self(&mut self) {
        self.app.pending_effects.push_back(Effect::Focus {
            window_id: self.window_id,
            view_id: self.view_id,
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
        self.app.add_view(self.window_id, build_view)
    }

    pub fn add_option_view<S, F>(&mut self, build_view: F) -> Option<ViewHandle<S>>
    where
        S: View,
        F: FnOnce(&mut ViewContext<S>) -> Option<S>,
    {
        self.app.add_option_view(self.window_id, build_view)
    }

    pub fn subscribe_to_model<E, F>(&mut self, handle: &ModelHandle<E>, mut callback: F)
    where
        E: Entity,
        E::Event: 'static,
        F: 'static + FnMut(&mut T, ModelHandle<E>, &E::Event, &mut ViewContext<T>),
    {
        let emitter_handle = handle.downgrade();
        self.subscribe(handle, move |model, payload, cx| {
            if let Some(emitter_handle) = emitter_handle.upgrade(cx.as_ref()) {
                callback(model, emitter_handle, payload, cx);
            }
        });
    }

    pub fn subscribe_to_view<V, F>(&mut self, handle: &ViewHandle<V>, mut callback: F)
    where
        V: View,
        V::Event: 'static,
        F: 'static + FnMut(&mut T, ViewHandle<V>, &V::Event, &mut ViewContext<T>),
    {
        let emitter_handle = handle.downgrade();
        self.subscribe(handle, move |view, payload, cx| {
            if let Some(emitter_handle) = emitter_handle.upgrade(cx.as_ref()) {
                callback(view, emitter_handle, payload, cx);
            }
        });
    }

    pub fn subscribe<E, F>(&mut self, handle: &impl Handle<E>, mut callback: F)
    where
        E: Entity,
        E::Event: 'static,
        F: 'static + FnMut(&mut T, &E::Event, &mut ViewContext<T>),
    {
        self.app
            .subscriptions
            .entry(handle.id())
            .or_default()
            .push(Subscription::FromView {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |entity, payload, app, window_id, view_id| {
                    let entity = entity.downcast_mut().expect("downcast is type safe");
                    let payload = payload.downcast_ref().expect("downcast is type safe");
                    let mut cx = ViewContext::new(app, window_id, view_id);
                    callback(entity, payload, &mut cx);
                }),
            });
    }

    pub fn emit(&mut self, payload: T::Event) {
        self.app.pending_effects.push_back(Effect::Event {
            entity_id: self.view_id,
            payload: Box::new(payload),
        });
    }

    pub fn observe_model<S, F>(&mut self, handle: &ModelHandle<S>, mut callback: F)
    where
        S: Entity,
        F: 'static + FnMut(&mut T, ModelHandle<S>, &mut ViewContext<T>),
    {
        self.app
            .model_observations
            .entry(handle.id())
            .or_default()
            .push(ModelObservation::FromView {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |view, observed_id, app, window_id, view_id| {
                    let view = view.downcast_mut().expect("downcast is type safe");
                    let observed = ModelHandle::new(observed_id, &app.cx.ref_counts);
                    let mut cx = ViewContext::new(app, window_id, view_id);
                    callback(view, observed, &mut cx);
                }),
            });
    }

    pub fn observe_view<S, F>(&mut self, handle: &ViewHandle<S>, mut callback: F)
    where
        S: View,
        F: 'static + FnMut(&mut T, ViewHandle<S>, &mut ViewContext<T>),
    {
        self.app
            .view_observations
            .entry(handle.id())
            .or_default()
            .push(ViewObservation {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(
                    move |view,
                          observed_view_id,
                          observed_window_id,
                          app,
                          observing_window_id,
                          observing_view_id| {
                        let view = view.downcast_mut().expect("downcast is type safe");
                        let observed_handle = ViewHandle::new(
                            observed_view_id,
                            observed_window_id,
                            &app.cx.ref_counts,
                        );
                        let mut cx = ViewContext::new(app, observing_window_id, observing_view_id);
                        callback(view, observed_handle, &mut cx);
                    },
                ),
            });
    }

    pub fn notify(&mut self) {
        self.app.notify_view(self.window_id, self.view_id);
    }

    pub fn propagate_action(&mut self) {
        self.halt_action_dispatch = false;
    }

    pub fn spawn<F, Fut, S>(&self, f: F) -> Task<S>
    where
        F: FnOnce(ViewHandle<T>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = S>,
        S: 'static,
    {
        let handle = self.handle();
        self.app.spawn(|cx| f(handle, cx))
    }
}

impl AsRef<AppContext> for &AppContext {
    fn as_ref(&self) -> &AppContext {
        self
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
        &self.app
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

impl<V: View> UpdateModel for ViewContext<'_, V> {
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        self.app.update_model(handle, update)
    }
}

impl<V: View> ReadView for ViewContext<'_, V> {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        self.app.read_view(handle)
    }
}

impl<V: View> UpdateView for ViewContext<'_, V> {
    fn update_view<T, F, S>(&mut self, handle: &ViewHandle<T>, update: F) -> S
    where
        T: View,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        self.app.update_view(handle, update)
    }
}

pub trait Handle<T> {
    fn id(&self) -> usize;
    fn location(&self) -> EntityLocation;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EntityLocation {
    Model(usize),
    View(usize, usize),
}

pub struct ModelHandle<T> {
    model_id: usize,
    model_type: PhantomData<T>,
    ref_counts: Arc<Mutex<RefCounts>>,
}

impl<T: Entity> ModelHandle<T> {
    fn new(model_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_model(model_id);
        Self {
            model_id,
            model_type: PhantomData,
            ref_counts: ref_counts.clone(),
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

    pub fn read_with<'a, C, F, S>(&self, cx: &C, read: F) -> S
    where
        C: ReadModelWith,
        F: FnOnce(&T, &AppContext) -> S,
    {
        cx.read_model_with(self, read)
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> S
    where
        C: UpdateModel,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        cx.update_model(self, update)
    }

    pub fn condition(
        &self,
        cx: &TestAppContext,
        mut predicate: impl FnMut(&T, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        let (tx, mut rx) = mpsc::channel(1024);

        let mut cx = cx.cx.borrow_mut();
        self.update(&mut *cx, |_, cx| {
            cx.observe(self, {
                let mut tx = tx.clone();
                move |_, _, _| {
                    tx.blocking_send(()).ok();
                }
            });
            cx.subscribe(self, {
                let mut tx = tx.clone();
                move |_, _, _| {
                    tx.blocking_send(()).ok();
                }
            })
        });

        let cx = cx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();
        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(5)
        } else {
            Duration::from_secs(1)
        };

        async move {
            timeout(duration, async move {
                loop {
                    {
                        let cx = cx.borrow();
                        let cx = cx.as_ref();
                        if predicate(
                            handle
                                .upgrade(cx)
                                .expect("model dropped with pending condition")
                                .read(cx),
                            cx,
                        ) {
                            break;
                        }
                    }

                    rx.recv()
                        .await
                        .expect("model dropped with pending condition");
                }
            })
            .await
            .expect("condition timed out");
        }
    }
}

impl<T> Clone for ModelHandle<T> {
    fn clone(&self) -> Self {
        self.ref_counts.lock().inc_model(self.model_id);
        Self {
            model_id: self.model_id,
            model_type: PhantomData,
            ref_counts: self.ref_counts.clone(),
        }
    }
}

impl<T> PartialEq for ModelHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.model_id == other.model_id
    }
}

impl<T> Eq for ModelHandle<T> {}

impl<T> Hash for ModelHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.model_id.hash(state);
    }
}

impl<T> std::borrow::Borrow<usize> for ModelHandle<T> {
    fn borrow(&self) -> &usize {
        &self.model_id
    }
}

impl<T> Debug for ModelHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("ModelHandle<{}>", type_name::<T>()))
            .field(&self.model_id)
            .finish()
    }
}

unsafe impl<T> Send for ModelHandle<T> {}
unsafe impl<T> Sync for ModelHandle<T> {}

impl<T> Drop for ModelHandle<T> {
    fn drop(&mut self) {
        self.ref_counts.lock().dec_model(self.model_id);
    }
}

impl<T> Handle<T> for ModelHandle<T> {
    fn id(&self) -> usize {
        self.model_id
    }

    fn location(&self) -> EntityLocation {
        EntityLocation::Model(self.model_id)
    }
}

pub struct WeakModelHandle<T> {
    model_id: usize,
    model_type: PhantomData<T>,
}

impl<T: Entity> WeakModelHandle<T> {
    fn new(model_id: usize) -> Self {
        Self {
            model_id,
            model_type: PhantomData,
        }
    }

    pub fn upgrade(&self, cx: impl AsRef<AppContext>) -> Option<ModelHandle<T>> {
        let cx = cx.as_ref();
        if cx.models.contains_key(&self.model_id) {
            Some(ModelHandle::new(self.model_id, &cx.ref_counts))
        } else {
            None
        }
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

impl<T> Clone for WeakModelHandle<T> {
    fn clone(&self) -> Self {
        Self {
            model_id: self.model_id,
            model_type: PhantomData,
        }
    }
}

pub struct ViewHandle<T> {
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
    ref_counts: Arc<Mutex<RefCounts>>,
}

impl<T: View> ViewHandle<T> {
    fn new(window_id: usize, view_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_view(window_id, view_id);
        Self {
            window_id,
            view_id,
            view_type: PhantomData,
            ref_counts: ref_counts.clone(),
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
        cx.read_view_with(self, read)
    }

    pub fn update<C, F, S>(&self, cx: &mut C, update: F) -> S
    where
        C: UpdateView,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        cx.update_view(self, update)
    }

    pub fn is_focused(&self, cx: &AppContext) -> bool {
        cx.focused_view_id(self.window_id)
            .map_or(false, |focused_id| focused_id == self.view_id)
    }

    pub fn condition(
        &self,
        cx: &TestAppContext,
        mut predicate: impl FnMut(&T, &AppContext) -> bool,
    ) -> impl Future<Output = ()> {
        let (tx, mut rx) = mpsc::channel(1024);

        let mut cx = cx.cx.borrow_mut();
        self.update(&mut *cx, |_, cx| {
            cx.observe_view(self, {
                let mut tx = tx.clone();
                move |_, _, _| {
                    tx.blocking_send(()).ok();
                }
            });

            cx.subscribe(self, {
                let mut tx = tx.clone();
                move |_, _, _| {
                    tx.blocking_send(()).ok();
                }
            })
        });

        let cx = cx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();
        let duration = if std::env::var("CI").is_ok() {
            Duration::from_secs(2)
        } else {
            Duration::from_millis(500)
        };

        async move {
            timeout(duration, async move {
                loop {
                    {
                        let cx = cx.borrow();
                        let cx = cx.as_ref();
                        if predicate(
                            handle
                                .upgrade(cx)
                                .expect("view dropped with pending condition")
                                .read(cx),
                            cx,
                        ) {
                            break;
                        }
                    }

                    rx.recv()
                        .await
                        .expect("view dropped with pending condition");
                }
            })
            .await
            .expect("condition timed out");
        }
    }
}

impl<T> Clone for ViewHandle<T> {
    fn clone(&self) -> Self {
        self.ref_counts
            .lock()
            .inc_view(self.window_id, self.view_id);
        Self {
            window_id: self.window_id,
            view_id: self.view_id,
            view_type: PhantomData,
            ref_counts: self.ref_counts.clone(),
        }
    }
}

impl<T> PartialEq for ViewHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.window_id == other.window_id && self.view_id == other.view_id
    }
}

impl<T> Eq for ViewHandle<T> {}

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
    }
}

impl<T> Handle<T> for ViewHandle<T> {
    fn id(&self) -> usize {
        self.view_id
    }

    fn location(&self) -> EntityLocation {
        EntityLocation::View(self.window_id, self.view_id)
    }
}

pub struct AnyViewHandle {
    window_id: usize,
    view_id: usize,
    view_type: TypeId,
    ref_counts: Arc<Mutex<RefCounts>>,
}

impl AnyViewHandle {
    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn is<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.view_type
    }

    pub fn downcast<T: View>(self) -> Option<ViewHandle<T>> {
        if self.is::<T>() {
            let result = Some(ViewHandle {
                window_id: self.window_id,
                view_id: self.view_id,
                ref_counts: self.ref_counts.clone(),
                view_type: PhantomData,
            });
            unsafe {
                Arc::decrement_strong_count(&self.ref_counts);
            }
            std::mem::forget(self);
            result
        } else {
            None
        }
    }
}

impl Clone for AnyViewHandle {
    fn clone(&self) -> Self {
        self.ref_counts
            .lock()
            .inc_view(self.window_id, self.view_id);
        Self {
            window_id: self.window_id,
            view_id: self.view_id,
            view_type: self.view_type,
            ref_counts: self.ref_counts.clone(),
        }
    }
}

impl<T: View> From<&ViewHandle<T>> for AnyViewHandle {
    fn from(handle: &ViewHandle<T>) -> Self {
        handle
            .ref_counts
            .lock()
            .inc_view(handle.window_id, handle.view_id);
        AnyViewHandle {
            window_id: handle.window_id,
            view_id: handle.view_id,
            view_type: TypeId::of::<T>(),
            ref_counts: handle.ref_counts.clone(),
        }
    }
}

impl<T: View> From<ViewHandle<T>> for AnyViewHandle {
    fn from(handle: ViewHandle<T>) -> Self {
        let any_handle = AnyViewHandle {
            window_id: handle.window_id,
            view_id: handle.view_id,
            view_type: TypeId::of::<T>(),
            ref_counts: handle.ref_counts.clone(),
        };
        unsafe {
            Arc::decrement_strong_count(&handle.ref_counts);
        }
        std::mem::forget(handle);
        any_handle
    }
}

impl Drop for AnyViewHandle {
    fn drop(&mut self) {
        self.ref_counts
            .lock()
            .dec_view(self.window_id, self.view_id);
    }
}

pub struct AnyModelHandle {
    model_id: usize,
    ref_counts: Arc<Mutex<RefCounts>>,
}

impl<T: Entity> From<ModelHandle<T>> for AnyModelHandle {
    fn from(handle: ModelHandle<T>) -> Self {
        handle.ref_counts.lock().inc_model(handle.model_id);
        Self {
            model_id: handle.model_id,
            ref_counts: handle.ref_counts.clone(),
        }
    }
}

impl Drop for AnyModelHandle {
    fn drop(&mut self) {
        self.ref_counts.lock().dec_model(self.model_id);
    }
}
pub struct WeakViewHandle<T> {
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
}

impl<T: View> WeakViewHandle<T> {
    fn new(window_id: usize, view_id: usize) -> Self {
        Self {
            window_id,
            view_id,
            view_type: PhantomData,
        }
    }

    pub fn upgrade(&self, cx: &AppContext) -> Option<ViewHandle<T>> {
        if cx.ref_counts.lock().is_entity_alive(self.view_id) {
            Some(ViewHandle::new(
                self.window_id,
                self.view_id,
                &cx.ref_counts,
            ))
        } else {
            None
        }
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

pub struct ValueHandle<T> {
    value_type: PhantomData<T>,
    tag_type_id: TypeId,
    id: usize,
    ref_counts: Weak<Mutex<RefCounts>>,
}

impl<T: 'static> ValueHandle<T> {
    fn new(tag_type_id: TypeId, id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc_value(tag_type_id, id);
        Self {
            value_type: PhantomData,
            tag_type_id,
            id,
            ref_counts: Arc::downgrade(ref_counts),
        }
    }

    pub fn read<R>(&self, cx: &AppContext, f: impl FnOnce(&T) -> R) -> R {
        f(cx.values
            .read()
            .get(&(self.tag_type_id, self.id))
            .unwrap()
            .downcast_ref()
            .unwrap())
    }

    pub fn update<R>(
        &self,
        cx: &mut EventContext,
        f: impl FnOnce(&mut T, &mut EventContext) -> R,
    ) -> R {
        let mut value = cx
            .app
            .cx
            .values
            .write()
            .remove(&(self.tag_type_id, self.id))
            .unwrap();
        let result = f(value.downcast_mut().unwrap(), cx);
        cx.app
            .cx
            .values
            .write()
            .insert((self.tag_type_id, self.id), value);
        result
    }
}

impl<T> Drop for ValueHandle<T> {
    fn drop(&mut self) {
        if let Some(ref_counts) = self.ref_counts.upgrade() {
            ref_counts.lock().dec_value(self.tag_type_id, self.id);
        }
    }
}

#[derive(Default)]
struct RefCounts {
    entity_counts: HashMap<usize, usize>,
    value_counts: HashMap<(TypeId, usize), usize>,
    dropped_models: HashSet<usize>,
    dropped_views: HashSet<(usize, usize)>,
    dropped_values: HashSet<(TypeId, usize)>,
}

impl RefCounts {
    fn inc_model(&mut self, model_id: usize) {
        match self.entity_counts.entry(model_id) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(entry) => {
                entry.insert(1);
                self.dropped_models.remove(&model_id);
            }
        }
    }

    fn inc_view(&mut self, window_id: usize, view_id: usize) {
        match self.entity_counts.entry(view_id) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(entry) => {
                entry.insert(1);
                self.dropped_views.remove(&(window_id, view_id));
            }
        }
    }

    fn inc_value(&mut self, tag_type_id: TypeId, id: usize) {
        *self.value_counts.entry((tag_type_id, id)).or_insert(0) += 1;
    }

    fn dec_model(&mut self, model_id: usize) {
        let count = self.entity_counts.get_mut(&model_id).unwrap();
        *count -= 1;
        if *count == 0 {
            self.entity_counts.remove(&model_id);
            self.dropped_models.insert(model_id);
        }
    }

    fn dec_view(&mut self, window_id: usize, view_id: usize) {
        let count = self.entity_counts.get_mut(&view_id).unwrap();
        *count -= 1;
        if *count == 0 {
            self.entity_counts.remove(&view_id);
            self.dropped_views.insert((window_id, view_id));
        }
    }

    fn dec_value(&mut self, tag_type_id: TypeId, id: usize) {
        let key = (tag_type_id, id);
        let count = self.value_counts.get_mut(&key).unwrap();
        *count -= 1;
        if *count == 0 {
            self.value_counts.remove(&key);
            self.dropped_values.insert(key);
        }
    }

    fn is_entity_alive(&self, entity_id: usize) -> bool {
        self.entity_counts.contains_key(&entity_id)
    }

    fn take_dropped(
        &mut self,
    ) -> (
        HashSet<usize>,
        HashSet<(usize, usize)>,
        HashSet<(TypeId, usize)>,
    ) {
        let mut dropped_models = HashSet::new();
        let mut dropped_views = HashSet::new();
        let mut dropped_values = HashSet::new();
        std::mem::swap(&mut self.dropped_models, &mut dropped_models);
        std::mem::swap(&mut self.dropped_views, &mut dropped_views);
        std::mem::swap(&mut self.dropped_values, &mut dropped_values);
        (dropped_models, dropped_views, dropped_values)
    }
}

enum Subscription {
    FromModel {
        model_id: usize,
        callback: Box<dyn FnMut(&mut dyn Any, &dyn Any, &mut MutableAppContext, usize)>,
    },
    FromView {
        window_id: usize,
        view_id: usize,
        callback: Box<dyn FnMut(&mut dyn Any, &dyn Any, &mut MutableAppContext, usize, usize)>,
    },
}

enum ModelObservation {
    FromModel {
        model_id: usize,
        callback: Box<dyn FnMut(&mut dyn Any, usize, &mut MutableAppContext, usize)>,
    },
    FromView {
        window_id: usize,
        view_id: usize,
        callback: Box<dyn FnMut(&mut dyn Any, usize, &mut MutableAppContext, usize, usize)>,
    },
}

struct ViewObservation {
    window_id: usize,
    view_id: usize,
    callback: Box<dyn FnMut(&mut dyn Any, usize, usize, &mut MutableAppContext, usize, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::*;
    use smol::future::poll_once;
    use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

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
                    });
                    cx.subscribe(other, |me, event, _| {
                        me.events.push(format!("observed event {}", event));
                    });
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
        assert!(cx.model_observations.is_empty());
    }

    #[crate::test(self)]
    fn test_subscribe_and_emit_from_model(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct Model {
            events: Vec<usize>,
        }

        impl Entity for Model {
            type Event = usize;
        }

        let handle_1 = cx.add_model(|_| Model::default());
        let handle_2 = cx.add_model(|_| Model::default());
        let handle_2b = handle_2.clone();

        handle_1.update(cx, |_, c| {
            c.subscribe(&handle_2, move |model: &mut Model, event, c| {
                model.events.push(*event);

                c.subscribe(&handle_2b, |model, event, _| {
                    model.events.push(*event * 2);
                });
            });
        });

        handle_2.update(cx, |_, c| c.emit(7));
        assert_eq!(handle_1.read(cx).events, vec![7]);

        handle_2.update(cx, |_, c| c.emit(5));
        assert_eq!(handle_1.read(cx).events, vec![7, 10, 5]);
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
        let handle_2b = handle_2.clone();

        handle_1.update(cx, |_, c| {
            c.observe(&handle_2, move |model, observed, c| {
                model.events.push(observed.read(c).count);
                c.observe(&handle_2b, |model, observed, c| {
                    model.events.push(observed.read(c).count * 2);
                });
            });
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
        assert_eq!(handle_1.read(cx).events, vec![7, 10, 5])
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
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        impl View {
            fn new(other: Option<ViewHandle<View>>, cx: &mut ViewContext<Self>) -> Self {
                if let Some(other) = other.as_ref() {
                    cx.subscribe_to_view(other, |me, _, event, _| {
                        me.events.push(format!("observed event {}", event));
                    });
                }
                Self {
                    other,
                    events: Vec::new(),
                }
            }
        }

        let (window_id, _) = cx.add_window(|cx| View::new(None, cx));
        let handle_1 = cx.add_view(window_id, |cx| View::new(None, cx));
        let handle_2 = cx.add_view(window_id, |cx| View::new(Some(handle_1.clone()), cx));
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
        assert!(cx.model_observations.is_empty());
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
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                let mouse_down_count = self.mouse_down_count.clone();
                EventHandler::new(Empty::new().boxed())
                    .on_mouse_down(move |_| {
                        mouse_down_count.fetch_add(1, SeqCst);
                        true
                    })
                    .boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        let mouse_down_count = Arc::new(AtomicUsize::new(0));
        let (window_id, _) = cx.add_window(|_| View {
            mouse_down_count: mouse_down_count.clone(),
        });
        let presenter = cx.presenters_and_platform_windows[&window_id].0.clone();
        // Ensure window's root element is in a valid lifecycle state.
        presenter.borrow_mut().dispatch_event(
            Event::LeftMouseDown {
                position: Default::default(),
                cmd: false,
            },
            cx,
        );
        assert_eq!(mouse_down_count.load(SeqCst), 1);
    }

    #[crate::test(self)]
    fn test_entity_release_hooks(cx: &mut MutableAppContext) {
        struct Model {
            released: Arc<Mutex<bool>>,
        }

        struct View {
            released: Arc<Mutex<bool>>,
        }

        impl Entity for Model {
            type Event = ();

            fn release(&mut self, _: &mut MutableAppContext) {
                *self.released.lock() = true;
            }
        }

        impl Entity for View {
            type Event = ();

            fn release(&mut self, _: &mut MutableAppContext) {
                *self.released.lock() = true;
            }
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "View"
            }

            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }
        }

        let model_released = Arc::new(Mutex::new(false));
        let view_released = Arc::new(Mutex::new(false));

        let model = cx.add_model(|_| Model {
            released: model_released.clone(),
        });

        let (window_id, _) = cx.add_window(|_| View {
            released: view_released.clone(),
        });

        assert!(!*model_released.lock());
        assert!(!*view_released.lock());

        cx.update(move || {
            drop(model);
        });
        assert!(*model_released.lock());

        drop(cx.remove_window(window_id));
        assert!(*view_released.lock());
    }

    #[crate::test(self)]
    fn test_subscribe_and_emit_from_view(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct View {
            events: Vec<usize>,
        }

        impl Entity for View {
            type Event = usize;
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct Model;

        impl Entity for Model {
            type Event = usize;
        }

        let (window_id, handle_1) = cx.add_window(|_| View::default());
        let handle_2 = cx.add_view(window_id, |_| View::default());
        let handle_2b = handle_2.clone();
        let handle_3 = cx.add_model(|_| Model);

        handle_1.update(cx, |_, c| {
            c.subscribe_to_view(&handle_2, move |me, _, event, c| {
                me.events.push(*event);

                c.subscribe_to_view(&handle_2b, |me, _, event, _| {
                    me.events.push(*event * 2);
                });
            });

            c.subscribe_to_model(&handle_3, |me, _, event, _| {
                me.events.push(*event);
            })
        });

        handle_2.update(cx, |_, c| c.emit(7));
        assert_eq!(handle_1.read(cx).events, vec![7]);

        handle_2.update(cx, |_, c| c.emit(5));
        assert_eq!(handle_1.read(cx).events, vec![7, 10, 5]);

        handle_3.update(cx, |_, c| c.emit(9));
        assert_eq!(handle_1.read(cx).events, vec![7, 10, 5, 9]);
    }

    #[crate::test(self)]
    fn test_dropping_subscribers(cx: &mut MutableAppContext) {
        struct View;

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let (window_id, _) = cx.add_window(|_| View);
        let observing_view = cx.add_view(window_id, |_| View);
        let emitting_view = cx.add_view(window_id, |_| View);
        let observing_model = cx.add_model(|_| Model);
        let observed_model = cx.add_model(|_| Model);

        observing_view.update(cx, |_, cx| {
            cx.subscribe_to_view(&emitting_view, |_, _, _, _| {});
            cx.subscribe_to_model(&observed_model, |_, _, _, _| {});
        });
        observing_model.update(cx, |_, cx| {
            cx.subscribe(&observed_model, |_, _, _| {});
        });

        cx.update(|| {
            drop(observing_view);
            drop(observing_model);
        });

        emitting_view.update(cx, |_, cx| cx.emit(()));
        observed_model.update(cx, |_, cx| cx.emit(()));
    }

    #[crate::test(self)]
    fn test_observe_and_notify_from_view(cx: &mut MutableAppContext) {
        #[derive(Default)]
        struct View {
            events: Vec<usize>,
        }

        impl Entity for View {
            type Event = usize;
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        #[derive(Default)]
        struct Model {
            count: usize,
        }

        impl Entity for Model {
            type Event = ();
        }

        let (_, view) = cx.add_window(|_| View::default());
        let model = cx.add_model(|_| Model::default());

        view.update(cx, |_, c| {
            c.observe_model(&model, |me, observed, c| {
                me.events.push(observed.read(c).count)
            });
        });

        model.update(cx, |model, c| {
            model.count = 11;
            c.notify();
        });
        assert_eq!(view.read(cx).events, vec![11]);
    }

    #[crate::test(self)]
    fn test_dropping_observers(cx: &mut MutableAppContext) {
        struct View;

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct Model;

        impl Entity for Model {
            type Event = ();
        }

        let (window_id, _) = cx.add_window(|_| View);
        let observing_view = cx.add_view(window_id, |_| View);
        let observing_model = cx.add_model(|_| Model);
        let observed_model = cx.add_model(|_| Model);

        observing_view.update(cx, |_, cx| {
            cx.observe_model(&observed_model, |_, _, _| {});
        });
        observing_model.update(cx, |_, cx| {
            cx.observe(&observed_model, |_, _, _| {});
        });

        cx.update(|| {
            drop(observing_view);
            drop(observing_model);
        });

        observed_model.update(cx, |_, cx| cx.notify());
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
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }

            fn on_focus(&mut self, _: &mut ViewContext<Self>) {
                self.events.lock().push(format!("{} focused", &self.name));
            }

            fn on_blur(&mut self, _: &mut ViewContext<Self>) {
                self.events.lock().push(format!("{} blurred", &self.name));
            }
        }

        let events: Arc<Mutex<Vec<String>>> = Default::default();
        let (window_id, view_1) = cx.add_window(|_| View {
            events: events.clone(),
            name: "view 1".to_string(),
        });
        let view_2 = cx.add_view(window_id, |_| View {
            events: events.clone(),
            name: "view 2".to_string(),
        });

        view_1.update(cx, |_, cx| cx.focus(&view_2));
        view_1.update(cx, |_, cx| cx.focus(&view_1));
        view_1.update(cx, |_, cx| cx.focus(&view_2));
        view_1.update(cx, |_, _| drop(view_2));

        assert_eq!(
            *events.lock(),
            [
                "view 1 focused".to_string(),
                "view 1 blurred".to_string(),
                "view 2 focused".to_string(),
                "view 2 blurred".to_string(),
                "view 1 focused".to_string(),
                "view 1 blurred".to_string(),
                "view 2 focused".to_string(),
                "view 1 focused".to_string(),
            ],
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
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
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
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct ActionArg {
            foo: String,
        }

        let actions = Rc::new(RefCell::new(Vec::new()));

        let actions_clone = actions.clone();
        cx.add_global_action("action", move |_: &ActionArg, _: &mut MutableAppContext| {
            actions_clone.borrow_mut().push("global a".to_string());
        });

        let actions_clone = actions.clone();
        cx.add_global_action("action", move |_: &ActionArg, _: &mut MutableAppContext| {
            actions_clone.borrow_mut().push("global b".to_string());
        });

        let actions_clone = actions.clone();
        cx.add_action("action", move |view: &mut ViewA, arg: &ActionArg, cx| {
            assert_eq!(arg.foo, "bar");
            cx.propagate_action();
            actions_clone.borrow_mut().push(format!("{} a", view.id));
        });

        let actions_clone = actions.clone();
        cx.add_action("action", move |view: &mut ViewA, _: &ActionArg, cx| {
            if view.id != 1 {
                cx.propagate_action();
            }
            actions_clone.borrow_mut().push(format!("{} b", view.id));
        });

        let actions_clone = actions.clone();
        cx.add_action("action", move |view: &mut ViewB, _: &ActionArg, cx| {
            cx.propagate_action();
            actions_clone.borrow_mut().push(format!("{} c", view.id));
        });

        let actions_clone = actions.clone();
        cx.add_action("action", move |view: &mut ViewB, _: &ActionArg, cx| {
            cx.propagate_action();
            actions_clone.borrow_mut().push(format!("{} d", view.id));
        });

        let (window_id, view_1) = cx.add_window(|_| ViewA { id: 1 });
        let view_2 = cx.add_view(window_id, |_| ViewB { id: 2 });
        let view_3 = cx.add_view(window_id, |_| ViewA { id: 3 });
        let view_4 = cx.add_view(window_id, |_| ViewB { id: 4 });

        cx.dispatch_action(
            window_id,
            vec![view_1.id(), view_2.id(), view_3.id(), view_4.id()],
            "action",
            ActionArg { foo: "bar".into() },
        );

        assert_eq!(
            *actions.borrow(),
            vec!["4 d", "4 c", "3 b", "3 a", "2 d", "2 c", "1 b"]
        );

        // Remove view_1, which doesn't propagate the action
        actions.borrow_mut().clear();
        cx.dispatch_action(
            window_id,
            vec![view_2.id(), view_3.id(), view_4.id()],
            "action",
            ActionArg { foo: "bar".into() },
        );

        assert_eq!(
            *actions.borrow(),
            vec!["4 d", "4 c", "3 b", "3 a", "2 d", "2 c", "global b", "global a"]
        );
    }

    #[crate::test(self)]
    fn test_dispatch_keystroke(cx: &mut MutableAppContext) {
        use std::cell::Cell;

        #[derive(Clone)]
        struct ActionArg {
            key: String,
        }

        struct View {
            id: usize,
            keymap_context: keymap::Context,
        }

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }

            fn keymap_context(&self, _: &AppContext) -> keymap::Context {
                self.keymap_context.clone()
            }
        }

        impl View {
            fn new(id: usize) -> Self {
                View {
                    id,
                    keymap_context: keymap::Context::default(),
                }
            }
        }

        let mut view_1 = View::new(1);
        let mut view_2 = View::new(2);
        let mut view_3 = View::new(3);
        view_1.keymap_context.set.insert("a".into());
        view_2.keymap_context.set.insert("b".into());
        view_3.keymap_context.set.insert("c".into());

        let (window_id, view_1) = cx.add_window(|_| view_1);
        let view_2 = cx.add_view(window_id, |_| view_2);
        let view_3 = cx.add_view(window_id, |_| view_3);

        // This keymap's only binding dispatches an action on view 2 because that view will have
        // "a" and "b" in its context, but not "c".
        let binding = keymap::Binding::new("a", "action", Some("a && b && !c"))
            .with_arg(ActionArg { key: "a".into() });
        cx.add_bindings(vec![binding]);

        let handled_action = Rc::new(Cell::new(false));
        let handled_action_clone = handled_action.clone();
        cx.add_action("action", move |view: &mut View, arg: &ActionArg, _| {
            handled_action_clone.set(true);
            assert_eq!(view.id, 2);
            assert_eq!(arg.key, "a");
        });

        cx.dispatch_keystroke(
            window_id,
            vec![view_1.id(), view_2.id(), view_3.id()],
            &Keystroke::parse("a").unwrap(),
        )
        .unwrap();

        assert!(handled_action.get());
    }

    #[crate::test(self)]
    async fn test_model_condition(mut cx: TestAppContext) {
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

        let condition1 = model.condition(&cx, |model, _| model.0 == 2);
        let condition2 = model.condition(&cx, |model, _| model.0 == 3);
        smol::pin!(condition1, condition2);

        model.update(&mut cx, |model, cx| model.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, None);
        assert_eq!(poll_once(&mut condition2).await, None);

        model.update(&mut cx, |model, cx| model.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, Some(()));
        assert_eq!(poll_once(&mut condition2).await, None);

        model.update(&mut cx, |model, cx| model.inc(cx));
        assert_eq!(poll_once(&mut condition2).await, Some(()));

        model.update(&mut cx, |_, cx| cx.notify());
    }

    #[crate::test(self)]
    #[should_panic]
    async fn test_model_condition_timeout(mut cx: TestAppContext) {
        struct Model;

        impl super::Entity for Model {
            type Event = ();
        }

        let model = cx.add_model(|_| Model);
        model.condition(&cx, |_, _| false).await;
    }

    #[crate::test(self)]
    #[should_panic(expected = "model dropped with pending condition")]
    async fn test_model_condition_panic_on_drop(mut cx: TestAppContext) {
        struct Model;

        impl super::Entity for Model {
            type Event = ();
        }

        let model = cx.add_model(|_| Model);
        let condition = model.condition(&cx, |_, _| false);
        cx.update(|_| drop(model));
        condition.await;
    }

    #[crate::test(self)]
    async fn test_view_condition(mut cx: TestAppContext) {
        struct Counter(usize);

        impl super::Entity for Counter {
            type Event = ();
        }

        impl super::View for Counter {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&self, _: &AppContext) -> ElementBox {
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

        let condition1 = view.condition(&cx, |view, _| view.0 == 2);
        let condition2 = view.condition(&cx, |view, _| view.0 == 3);
        smol::pin!(condition1, condition2);

        view.update(&mut cx, |view, cx| view.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, None);
        assert_eq!(poll_once(&mut condition2).await, None);

        view.update(&mut cx, |view, cx| view.inc(cx));
        assert_eq!(poll_once(&mut condition1).await, Some(()));
        assert_eq!(poll_once(&mut condition2).await, None);

        view.update(&mut cx, |view, cx| view.inc(cx));
        assert_eq!(poll_once(&mut condition2).await, Some(()));
        view.update(&mut cx, |_, cx| cx.notify());
    }

    #[crate::test(self)]
    #[should_panic]
    async fn test_view_condition_timeout(mut cx: TestAppContext) {
        struct View;

        impl super::Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }
        }

        let (_, view) = cx.add_window(|_| View);
        view.condition(&cx, |_, _| false).await;
    }

    #[crate::test(self)]
    #[should_panic(expected = "view dropped with pending condition")]
    async fn test_view_condition_panic_on_drop(mut cx: TestAppContext) {
        struct View;

        impl super::Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn ui_name() -> &'static str {
                "test view"
            }

            fn render(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }
        }

        let window_id = cx.add_window(|_| View).0;
        let view = cx.add_view(window_id, |_| View);

        let condition = view.condition(&cx, |_, _| false);
        cx.update(|_| drop(view));
        condition.await;
    }
}
