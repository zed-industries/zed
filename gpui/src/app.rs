use crate::{
    elements::ElementBox,
    executor,
    keymap::{self, Keystroke},
    platform::{self, WindowOptions},
    presenter::Presenter,
    util::{post_inc, timeout},
    AssetCache, AssetSource, ClipboardItem, FontCache, PathPromptOptions, TextLayoutCache,
};
use anyhow::{anyhow, Result};
use keymap::MatchResult;
use parking_lot::Mutex;
use pathfinder_geometry::{rect::RectF, vector::vec2f};
use platform::Event;
use postage::{sink::Sink as _, stream::Stream as _};
use smol::prelude::*;
use std::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    path::PathBuf,
    rc::{self, Rc},
    sync::{Arc, Weak},
    time::Duration,
};

pub trait Entity: 'static + Send + Sync {
    type Event;
}

pub trait View: Entity {
    fn ui_name() -> &'static str;
    fn render<'a>(&self, app: &AppContext) -> ElementBox;
    fn on_focus(&mut self, _ctx: &mut ViewContext<Self>) {}
    fn on_blur(&mut self, _ctx: &mut ViewContext<Self>) {}
    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        Self::default_keymap_context()
    }
    fn default_keymap_context() -> keymap::Context {
        let mut ctx = keymap::Context::default();
        ctx.set.insert(Self::ui_name().into());
        ctx
    }
}

pub trait ReadModel {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T;
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
pub struct TestAppContext(Rc<RefCell<MutableAppContext>>);

impl App {
    pub fn test<T, A: AssetSource, F: FnOnce(&mut MutableAppContext) -> T>(
        asset_source: A,
        f: F,
    ) -> T {
        let platform = platform::test::platform();
        let foreground = Rc::new(executor::Foreground::test());
        let ctx = Rc::new(RefCell::new(MutableAppContext::new(
            foreground,
            Rc::new(platform),
            asset_source,
        )));
        ctx.borrow_mut().weak_self = Some(Rc::downgrade(&ctx));
        let mut ctx = ctx.borrow_mut();
        f(&mut *ctx)
    }

    pub fn test_async<T, F, A: AssetSource, Fn>(asset_source: A, f: Fn) -> T
    where
        Fn: FnOnce(TestAppContext) -> F,
        F: Future<Output = T>,
    {
        let platform = platform::test::platform();
        let foreground = Rc::new(executor::Foreground::test());
        let ctx = TestAppContext(Rc::new(RefCell::new(MutableAppContext::new(
            foreground.clone(),
            Rc::new(platform),
            asset_source,
        ))));
        ctx.0.borrow_mut().weak_self = Some(Rc::downgrade(&ctx.0));

        let future = f(ctx);
        smol::block_on(foreground.run(future))
    }

    pub fn new(asset_source: impl AssetSource) -> Result<Self> {
        let platform = platform::current::platform();
        let foreground = Rc::new(executor::Foreground::platform(platform.dispatcher())?);
        let app = Self(Rc::new(RefCell::new(MutableAppContext::new(
            foreground,
            platform.clone(),
            asset_source,
        ))));

        let ctx = app.0.clone();
        platform.on_menu_command(Box::new(move |command, arg| {
            let mut ctx = ctx.borrow_mut();
            if let Some(key_window_id) = ctx.platform.key_window_id() {
                if let Some((presenter, _)) =
                    ctx.presenters_and_platform_windows.get(&key_window_id)
                {
                    let presenter = presenter.clone();
                    let path = presenter.borrow().dispatch_path(ctx.as_ref());
                    ctx.dispatch_action_any(key_window_id, &path, command, arg.unwrap_or(&()));
                } else {
                    ctx.dispatch_global_action_any(command, arg.unwrap_or(&()));
                }
            } else {
                ctx.dispatch_global_action_any(command, arg.unwrap_or(&()));
            }
        }));

        app.0.borrow_mut().weak_self = Some(Rc::downgrade(&app.0));
        Ok(app)
    }

    pub fn on_become_active<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(&mut MutableAppContext),
    {
        let ctx = self.0.clone();
        self.0
            .borrow()
            .platform
            .on_become_active(Box::new(move || callback(&mut *ctx.borrow_mut())));
        self
    }

    pub fn on_resign_active<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(&mut MutableAppContext),
    {
        let ctx = self.0.clone();
        self.0
            .borrow()
            .platform
            .on_resign_active(Box::new(move || callback(&mut *ctx.borrow_mut())));
        self
    }

    pub fn on_event<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(Event, &mut MutableAppContext) -> bool,
    {
        let ctx = self.0.clone();
        self.0.borrow().platform.on_event(Box::new(move |event| {
            callback(event, &mut *ctx.borrow_mut())
        }));
        self
    }

    pub fn on_open_files<F>(self, mut callback: F) -> Self
    where
        F: 'static + FnMut(Vec<PathBuf>, &mut MutableAppContext),
    {
        let ctx = self.0.clone();
        self.0
            .borrow()
            .platform
            .on_open_files(Box::new(move |paths| {
                callback(paths, &mut *ctx.borrow_mut())
            }));
        self
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut MutableAppContext),
    {
        let platform = self.0.borrow().platform.clone();
        platform.run(Box::new(move || {
            let mut ctx = self.0.borrow_mut();
            on_finish_launching(&mut *ctx);
        }))
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.0.borrow().font_cache.clone()
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
    pub fn dispatch_action<T: 'static + Any>(
        &self,
        window_id: usize,
        responder_chain: Vec<usize>,
        name: &str,
        arg: T,
    ) {
        self.0.borrow_mut().dispatch_action_any(
            window_id,
            &responder_chain,
            name,
            Box::new(arg).as_ref(),
        );
    }

    pub fn dispatch_keystroke(
        &self,
        window_id: usize,
        responder_chain: Vec<usize>,
        keystroke: &Keystroke,
    ) -> Result<bool> {
        let mut state = self.0.borrow_mut();
        state.dispatch_keystroke(window_id, responder_chain, keystroke)
    }

    pub fn add_model<T, F>(&mut self, build_model: F) -> ModelHandle<T>
    where
        T: Entity,
        F: FnOnce(&mut ModelContext<T>) -> T,
    {
        let mut state = self.0.borrow_mut();
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
        self.0.borrow_mut().add_window(build_root_view)
    }

    pub fn window_ids(&self) -> Vec<usize> {
        self.0.borrow().window_ids().collect()
    }

    pub fn root_view<T: View>(&self, window_id: usize) -> Option<ViewHandle<T>> {
        self.0.borrow().root_view(window_id)
    }

    pub fn add_view<T, F>(&mut self, window_id: usize, build_view: F) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        let mut state = self.0.borrow_mut();
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
        let mut state = self.0.borrow_mut();
        state.pending_flushes += 1;
        let handle = state.add_option_view(window_id, build_view);
        state.flush_effects();
        handle
    }

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&self, callback: F) -> T {
        callback(self.0.borrow().as_ref())
    }

    pub fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.0.borrow_mut();
        // Don't increment pending flushes in order to effects to be flushed before the callback
        // completes, which is helpful in tests.
        let result = callback(&mut *state);
        // Flush effects after the callback just in case there are any. This can happen in edge
        // cases such as the closure dropping handles.
        state.flush_effects();
        result
    }

    pub fn font_cache(&self) -> Arc<FontCache> {
        self.0.borrow().font_cache.clone()
    }

    pub fn platform(&self) -> Rc<dyn platform::Platform> {
        self.0.borrow().platform.clone()
    }
}

impl UpdateModel for TestAppContext {
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

impl UpdateView for TestAppContext {
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

type ActionCallback =
    dyn FnMut(&mut dyn AnyView, &dyn Any, &mut MutableAppContext, usize, usize) -> bool;

type GlobalActionCallback = dyn FnMut(&dyn Any, &mut MutableAppContext);

pub struct MutableAppContext {
    weak_self: Option<rc::Weak<RefCell<Self>>>,
    platform: Rc<dyn platform::Platform>,
    font_cache: Arc<FontCache>,
    assets: Arc<AssetCache>,
    ctx: AppContext,
    actions: HashMap<TypeId, HashMap<String, Vec<Box<ActionCallback>>>>,
    global_actions: HashMap<String, Vec<Box<GlobalActionCallback>>>,
    keystroke_matcher: keymap::Matcher,
    next_entity_id: usize,
    next_window_id: usize,
    next_task_id: usize,
    subscriptions: HashMap<usize, Vec<Subscription>>,
    observations: HashMap<usize, Vec<Observation>>,
    async_observations: HashMap<usize, postage::broadcast::Sender<()>>,
    window_invalidations: HashMap<usize, WindowInvalidation>,
    presenters_and_platform_windows:
        HashMap<usize, (Rc<RefCell<Presenter>>, Box<dyn platform::Window>)>,
    debug_elements_callbacks: HashMap<usize, Box<dyn Fn(&AppContext) -> crate::json::Value>>,
    foreground: Rc<executor::Foreground>,
    future_handlers: Rc<RefCell<HashMap<usize, FutureHandler>>>,
    stream_handlers: Rc<RefCell<HashMap<usize, StreamHandler>>>,
    pending_effects: VecDeque<Effect>,
    pending_flushes: usize,
    flushing_effects: bool,
}

impl MutableAppContext {
    pub fn new(
        foreground: Rc<executor::Foreground>,
        platform: Rc<dyn platform::Platform>,
        asset_source: impl AssetSource,
    ) -> Self {
        let fonts = platform.fonts();
        Self {
            weak_self: None,
            platform,
            font_cache: Arc::new(FontCache::new(fonts)),
            assets: Arc::new(AssetCache::new(asset_source)),
            ctx: AppContext {
                models: HashMap::new(),
                windows: HashMap::new(),
                ref_counts: Arc::new(Mutex::new(RefCounts::default())),
                background: Arc::new(executor::Background::new()),
                thread_pool: scoped_pool::Pool::new(num_cpus::get(), "app"),
            },
            actions: HashMap::new(),
            global_actions: HashMap::new(),
            keystroke_matcher: keymap::Matcher::default(),
            next_entity_id: 0,
            next_window_id: 0,
            next_task_id: 0,
            subscriptions: HashMap::new(),
            observations: HashMap::new(),
            async_observations: HashMap::new(),
            window_invalidations: HashMap::new(),
            presenters_and_platform_windows: HashMap::new(),
            debug_elements_callbacks: HashMap::new(),
            foreground,
            future_handlers: Default::default(),
            stream_handlers: Default::default(),
            pending_effects: VecDeque::new(),
            pending_flushes: 0,
            flushing_effects: false,
        }
    }

    pub fn upgrade(&self) -> App {
        App(self.weak_self.as_ref().unwrap().upgrade().unwrap())
    }

    pub fn platform(&self) -> Rc<dyn platform::Platform> {
        self.platform.clone()
    }

    pub fn font_cache(&self) -> &Arc<FontCache> {
        &self.font_cache
    }

    pub fn foreground_executor(&self) -> &Rc<executor::Foreground> {
        &self.foreground
    }

    pub fn background_executor(&self) -> &Arc<executor::Background> {
        &self.ctx.background
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
            .map(|debug_elements| debug_elements(&self.ctx))
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
                  app: &mut MutableAppContext,
                  window_id: usize,
                  view_id: usize| {
                match arg.downcast_ref() {
                    Some(arg) => {
                        let mut ctx = ViewContext::new(app, window_id, view_id);
                        handler(
                            view.as_any_mut()
                                .downcast_mut()
                                .expect("downcast is type safe"),
                            arg,
                            &mut ctx,
                        );
                        ctx.halt_action_dispatch
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
        let handler = Box::new(move |arg: &dyn Any, app: &mut MutableAppContext| {
            if let Some(arg) = arg.downcast_ref() {
                handler(arg, app);
            } else {
                log::error!("Could not downcast argument for action {}", name_clone);
            }
        });

        self.global_actions.entry(name).or_default().push(handler);
    }

    pub fn window_ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.ctx.windows.keys().cloned()
    }

    pub fn root_view<T: View>(&self, window_id: usize) -> Option<ViewHandle<T>> {
        self.ctx
            .windows
            .get(&window_id)
            .and_then(|window| window.root_view.as_ref().unwrap().clone().downcast::<T>())
    }

    pub fn root_view_id(&self, window_id: usize) -> Option<usize> {
        self.ctx.root_view_id(window_id)
    }

    pub fn focused_view_id(&self, window_id: usize) -> Option<usize> {
        self.ctx.focused_view_id(window_id)
    }

    pub fn render_view(&self, window_id: usize, view_id: usize) -> Result<ElementBox> {
        self.ctx.render_view(window_id, view_id)
    }

    pub fn render_views(&self, window_id: usize) -> Result<HashMap<usize, ElementBox>> {
        self.ctx.render_views(window_id)
    }

    pub fn update<T, F: FnOnce() -> T>(&mut self, callback: F) -> T {
        self.pending_flushes += 1;
        let result = callback();
        self.flush_effects();
        result
    }

    pub fn set_menus(&self, menus: Vec<Menu>) {
        self.platform.set_menus(menus);
    }

    pub fn prompt_for_paths<F>(&self, options: PathPromptOptions, done_fn: F)
    where
        F: 'static + FnOnce(Option<Vec<PathBuf>>, &mut MutableAppContext),
    {
        let app = self.weak_self.as_ref().unwrap().upgrade().unwrap();
        let foreground = self.foreground.clone();
        self.platform().prompt_for_paths(
            options,
            Box::new(move |paths| {
                foreground
                    .spawn(async move { (done_fn)(paths, &mut *app.borrow_mut()) })
                    .detach();
            }),
        );
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

    fn dispatch_action_any(
        &mut self,
        window_id: usize,
        path: &[usize],
        name: &str,
        arg: &dyn Any,
    ) -> bool {
        self.pending_flushes += 1;
        let mut halted_dispatch = false;

        for view_id in path.iter().rev() {
            if let Some(mut view) = self
                .ctx
                .windows
                .get_mut(&window_id)
                .and_then(|w| w.views.remove(view_id))
            {
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

                self.ctx
                    .windows
                    .get_mut(&window_id)
                    .unwrap()
                    .views
                    .insert(*view_id, view);

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
            if let Some(view) = self
                .ctx
                .windows
                .get(&window_id)
                .and_then(|w| w.views.get(view_id))
            {
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
        for (i, ctx) in context_chain.iter().enumerate().rev() {
            match self
                .keystroke_matcher
                .push_keystroke(keystroke.clone(), responder_chain[i], ctx)
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
        let mut ctx = ModelContext::new(self, model_id);
        let model = build_model(&mut ctx);
        self.ctx.models.insert(model_id, Box::new(model));
        self.flush_effects();
        ModelHandle::new(model_id, &self.ctx.ref_counts)
    }

    pub fn add_window<T, F>(&mut self, build_root_view: F) -> (usize, ViewHandle<T>)
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.pending_flushes += 1;
        let window_id = post_inc(&mut self.next_window_id);
        self.ctx.windows.insert(window_id, Window::default());
        self.open_platform_window(window_id);
        let root_handle = self.add_view(window_id, build_root_view);
        self.ctx.windows.get_mut(&window_id).unwrap().root_view = Some(root_handle.clone().into());
        self.focus(window_id, root_handle.id());
        self.flush_effects();

        (window_id, root_handle)
    }

    fn open_platform_window(&mut self, window_id: usize) {
        let mut window = self.platform.open_window(
            window_id,
            WindowOptions {
                bounds: RectF::new(vec2f(0., 0.), vec2f(1024., 768.)),
                title: "Zed".into(),
            },
            self.foreground.clone(),
        );
        let text_layout_cache = TextLayoutCache::new(self.platform.fonts());
        let presenter = Rc::new(RefCell::new(Presenter::new(
            window_id,
            self.font_cache.clone(),
            text_layout_cache,
            self.assets.clone(),
            self,
        )));

        {
            let mut app = self.upgrade();
            let presenter = presenter.clone();
            window.on_event(Box::new(move |event| {
                app.update(|ctx| {
                    if let Event::KeyDown { keystroke, .. } = &event {
                        if ctx
                            .dispatch_keystroke(
                                window_id,
                                presenter.borrow().dispatch_path(ctx.as_ref()),
                                keystroke,
                            )
                            .unwrap()
                        {
                            return;
                        }
                    }

                    let actions = presenter.borrow_mut().dispatch_event(event, ctx.as_ref());
                    for action in actions {
                        ctx.dispatch_action_any(
                            window_id,
                            &action.path,
                            action.name,
                            action.arg.as_ref(),
                        );
                    }
                })
            }));
        }

        {
            let mut app = self.upgrade();
            let presenter = presenter.clone();
            window.on_resize(Box::new(move |window| {
                app.update(|ctx| {
                    let scene = presenter.borrow_mut().build_scene(
                        window.size(),
                        window.scale_factor(),
                        ctx,
                    );
                    window.present_scene(scene);
                })
            }));
        }

        self.presenters_and_platform_windows
            .insert(window_id, (presenter.clone(), window));

        self.on_debug_elements(window_id, move |ctx| {
            presenter.borrow().debug_elements(ctx).unwrap()
        });
    }

    pub fn add_view<T, F>(&mut self, window_id: usize, build_view: F) -> ViewHandle<T>
    where
        T: View,
        F: FnOnce(&mut ViewContext<T>) -> T,
    {
        self.add_option_view(window_id, |ctx| Some(build_view(ctx)))
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
        let mut ctx = ViewContext::new(self, window_id, view_id);
        let handle = if let Some(view) = build_view(&mut ctx) {
            if let Some(window) = self.ctx.windows.get_mut(&window_id) {
                window.views.insert(view_id, Box::new(view));
            } else {
                panic!("Window does not exist");
            }
            self.window_invalidations
                .entry(window_id)
                .or_default()
                .updated
                .insert(view_id);
            Some(ViewHandle::new(window_id, view_id, &self.ctx.ref_counts))
        } else {
            None
        };
        self.flush_effects();
        handle
    }

    fn remove_dropped_entities(&mut self) {
        loop {
            let (dropped_models, dropped_views) = self.ctx.ref_counts.lock().take_dropped();
            if dropped_models.is_empty() && dropped_views.is_empty() {
                break;
            }

            for model_id in dropped_models {
                self.ctx.models.remove(&model_id);
                self.subscriptions.remove(&model_id);
                self.observations.remove(&model_id);
                self.async_observations.remove(&model_id);
            }

            for (window_id, view_id) in dropped_views {
                self.subscriptions.remove(&view_id);
                self.observations.remove(&view_id);
                self.async_observations.remove(&view_id);
                if let Some(window) = self.ctx.windows.get_mut(&window_id) {
                    self.window_invalidations
                        .entry(window_id)
                        .or_default()
                        .removed
                        .push(view_id);
                    window.views.remove(&view_id);
                }
            }
        }
    }

    fn flush_effects(&mut self) {
        self.pending_flushes = self.pending_flushes.saturating_sub(1);

        if !self.flushing_effects && self.pending_flushes == 0 {
            self.flushing_effects = true;

            while let Some(effect) = self.pending_effects.pop_front() {
                match effect {
                    Effect::Event { entity_id, payload } => self.emit_event(entity_id, payload),
                    Effect::ModelNotification { model_id } => self.notify_model_observers(model_id),
                    Effect::ViewNotification { window_id, view_id } => {
                        self.notify_view_observers(window_id, view_id)
                    }
                    Effect::Focus { window_id, view_id } => {
                        self.focus(window_id, view_id);
                    }
                }
            }

            self.flushing_effects = false;
            self.remove_dropped_entities();
            self.update_windows();
        }
    }

    fn update_windows(&mut self) {
        let mut invalidations = HashMap::new();
        std::mem::swap(&mut invalidations, &mut self.window_invalidations);

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
                        if let Some(mut model) = self.ctx.models.remove(model_id) {
                            callback(model.as_any_mut(), payload.as_ref(), self, *model_id);
                            self.ctx.models.insert(*model_id, model);
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
                        if let Some(mut view) = self
                            .ctx
                            .windows
                            .get_mut(&window_id)
                            .and_then(|window| window.views.remove(view_id))
                        {
                            callback(
                                view.as_any_mut(),
                                payload.as_ref(),
                                self,
                                *window_id,
                                *view_id,
                            );
                            self.ctx
                                .windows
                                .get_mut(&window_id)
                                .unwrap()
                                .views
                                .insert(*view_id, view);
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
        if let Some(observations) = self.observations.remove(&observed_id) {
            if self.ctx.models.contains_key(&observed_id) {
                for mut observation in observations {
                    let alive = match &mut observation {
                        Observation::FromModel { model_id, callback } => {
                            if let Some(mut model) = self.ctx.models.remove(model_id) {
                                callback(model.as_any_mut(), observed_id, self, *model_id);
                                self.ctx.models.insert(*model_id, model);
                                true
                            } else {
                                false
                            }
                        }
                        Observation::FromView {
                            window_id,
                            view_id,
                            callback,
                        } => {
                            if let Some(mut view) = self
                                .ctx
                                .windows
                                .get_mut(window_id)
                                .and_then(|w| w.views.remove(view_id))
                            {
                                callback(
                                    view.as_any_mut(),
                                    observed_id,
                                    self,
                                    *window_id,
                                    *view_id,
                                );
                                self.ctx
                                    .windows
                                    .get_mut(window_id)
                                    .unwrap()
                                    .views
                                    .insert(*view_id, view);
                                true
                            } else {
                                false
                            }
                        }
                    };

                    if alive {
                        self.observations
                            .entry(observed_id)
                            .or_default()
                            .push(observation);
                    }
                }
            }
        }

        if let Entry::Occupied(mut entry) = self.async_observations.entry(observed_id) {
            if entry.get_mut().blocking_send(()).is_err() {
                entry.remove_entry();
            }
        }
    }

    fn notify_view_observers(&mut self, window_id: usize, view_id: usize) {
        self.window_invalidations
            .entry(window_id)
            .or_default()
            .updated
            .insert(view_id);

        if let Entry::Occupied(mut entry) = self.async_observations.entry(view_id) {
            if entry.get_mut().blocking_send(()).is_err() {
                entry.remove_entry();
            }
        }
    }

    fn focus(&mut self, window_id: usize, focused_id: usize) {
        if self
            .ctx
            .windows
            .get(&window_id)
            .and_then(|w| w.focused_view)
            .map_or(false, |cur_focused| cur_focused == focused_id)
        {
            return;
        }

        self.pending_flushes += 1;

        if let Some((blurred_id, mut blurred)) =
            self.ctx.windows.get_mut(&window_id).and_then(|w| {
                let blurred_view = w.focused_view;
                w.focused_view = Some(focused_id);
                blurred_view.and_then(|id| w.views.remove(&id).map(|view| (id, view)))
            })
        {
            blurred.on_blur(self, window_id, blurred_id);
            self.ctx
                .windows
                .get_mut(&window_id)
                .unwrap()
                .views
                .insert(blurred_id, blurred);
        }

        if let Some(mut focused) = self
            .ctx
            .windows
            .get_mut(&window_id)
            .and_then(|w| w.views.remove(&focused_id))
        {
            focused.on_focus(self, window_id, focused_id);
            self.ctx
                .windows
                .get_mut(&window_id)
                .unwrap()
                .views
                .insert(focused_id, focused);
        }

        self.flush_effects();
    }

    fn spawn<F, T>(&mut self, future: F) -> EntityTask<T>
    where
        F: 'static + Future,
        T: 'static,
    {
        let task_id = post_inc(&mut self.next_task_id);
        let app = self.weak_self.as_ref().unwrap().upgrade().unwrap();
        let task = {
            let app = app.clone();
            self.foreground.spawn(async move {
                let output = future.await;
                *app.borrow_mut()
                    .handle_future_output(task_id, Box::new(output))
                    .downcast::<T>()
                    .unwrap()
            })
        };
        EntityTask::new(
            task_id,
            task,
            TaskHandlerMap::Future(self.future_handlers.clone()),
        )
    }

    fn spawn_stream<F, T>(&mut self, mut stream: F) -> EntityTask<T>
    where
        F: 'static + Stream + Unpin,
        T: 'static,
    {
        let task_id = post_inc(&mut self.next_task_id);
        let app = self.weak_self.as_ref().unwrap().upgrade().unwrap();
        let task = self.foreground.spawn(async move {
            loop {
                match stream.next().await {
                    Some(item) => {
                        let mut app = app.borrow_mut();
                        if app.handle_stream_item(task_id, Box::new(item)) {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            *app.borrow_mut()
                .stream_completed(task_id)
                .downcast::<T>()
                .unwrap()
        });

        EntityTask::new(
            task_id,
            task,
            TaskHandlerMap::Stream(self.stream_handlers.clone()),
        )
    }

    fn handle_future_output(&mut self, task_id: usize, output: Box<dyn Any>) -> Box<dyn Any> {
        self.pending_flushes += 1;
        let future_callback = self.future_handlers.borrow_mut().remove(&task_id).unwrap();
        let result = future_callback(output, self);
        self.flush_effects();
        result
    }

    fn handle_stream_item(&mut self, task_id: usize, output: Box<dyn Any>) -> bool {
        self.pending_flushes += 1;

        let mut handler = self.stream_handlers.borrow_mut().remove(&task_id).unwrap();
        let halt = (handler.item_callback)(output, self);
        self.stream_handlers.borrow_mut().insert(task_id, handler);

        self.flush_effects();
        halt
    }

    fn stream_completed(&mut self, task_id: usize) -> Box<dyn Any> {
        self.pending_flushes += 1;

        let handler = self.stream_handlers.borrow_mut().remove(&task_id).unwrap();
        let result = (handler.done_callback)(self);

        self.flush_effects();
        result
    }

    pub fn write_to_clipboard(&self, item: ClipboardItem) {
        self.platform.write_to_clipboard(item);
    }

    pub fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.platform.read_from_clipboard()
    }
}

impl ReadModel for MutableAppContext {
    fn read_model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        if let Some(model) = self.ctx.models.get(&handle.model_id) {
            model
                .as_any()
                .downcast_ref()
                .expect("Downcast is type safe")
        } else {
            panic!("Circular model reference");
        }
    }
}

impl UpdateModel for MutableAppContext {
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        if let Some(mut model) = self.ctx.models.remove(&handle.model_id) {
            self.pending_flushes += 1;
            let mut ctx = ModelContext::new(self, handle.model_id);
            let result = update(
                model
                    .as_any_mut()
                    .downcast_mut()
                    .expect("Downcast is type safe"),
                &mut ctx,
            );
            self.ctx.models.insert(handle.model_id, model);
            self.flush_effects();
            result
        } else {
            panic!("Circular model update");
        }
    }
}

impl ReadView for MutableAppContext {
    fn read_view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        if let Some(window) = self.ctx.windows.get(&handle.window_id) {
            if let Some(view) = window.views.get(&handle.view_id) {
                view.as_any().downcast_ref().expect("Downcast is type safe")
            } else {
                panic!("Circular view reference");
            }
        } else {
            panic!("Window does not exist");
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
        let mut view = if let Some(window) = self.ctx.windows.get_mut(&handle.window_id) {
            if let Some(view) = window.views.remove(&handle.view_id) {
                view
            } else {
                panic!("Circular view update");
            }
        } else {
            panic!("Window does not exist");
        };

        let mut ctx = ViewContext::new(self, handle.window_id, handle.view_id);
        let result = update(
            view.as_any_mut()
                .downcast_mut()
                .expect("Downcast is type safe"),
            &mut ctx,
        );
        self.ctx
            .windows
            .get_mut(&handle.window_id)
            .unwrap()
            .views
            .insert(handle.view_id, view);
        self.flush_effects();
        result
    }
}

impl AsRef<AppContext> for MutableAppContext {
    fn as_ref(&self) -> &AppContext {
        &self.ctx
    }
}

pub struct AppContext {
    models: HashMap<usize, Box<dyn AnyModel>>,
    windows: HashMap<usize, Window>,
    background: Arc<executor::Background>,
    ref_counts: Arc<Mutex<RefCounts>>,
    thread_pool: scoped_pool::Pool,
}

impl AppContext {
    pub fn root_view_id(&self, window_id: usize) -> Option<usize> {
        self.windows
            .get(&window_id)
            .and_then(|window| window.root_view.as_ref().map(|v| v.id()))
    }

    pub fn focused_view_id(&self, window_id: usize) -> Option<usize> {
        self.windows
            .get(&window_id)
            .and_then(|window| window.focused_view)
    }

    pub fn render_view(&self, window_id: usize, view_id: usize) -> Result<ElementBox> {
        self.windows
            .get(&window_id)
            .and_then(|w| w.views.get(&view_id))
            .map(|v| v.render(self))
            .ok_or(anyhow!("view not found"))
    }

    pub fn render_views(&self, window_id: usize) -> Result<HashMap<usize, ElementBox>> {
        self.windows
            .get(&window_id)
            .map(|w| {
                w.views
                    .iter()
                    .map(|(id, view)| (*id, view.render(self)))
                    .collect::<HashMap<_, ElementBox>>()
            })
            .ok_or(anyhow!("window not found"))
    }

    pub fn background_executor(&self) -> &Arc<executor::Background> {
        &self.background
    }

    pub fn thread_pool(&self) -> &scoped_pool::Pool {
        &self.thread_pool
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
        if let Some(window) = self.windows.get(&handle.window_id) {
            if let Some(view) = window.views.get(&handle.view_id) {
                view.as_any()
                    .downcast_ref()
                    .expect("downcast should be type safe")
            } else {
                panic!("circular view reference");
            }
        } else {
            panic!("window does not exist");
        }
    }
}

#[derive(Default)]
struct Window {
    views: HashMap<usize, Box<dyn AnyView>>,
    root_view: Option<AnyViewHandle>,
    focused_view: Option<usize>,
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

pub trait AnyModel: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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
}

pub trait AnyView: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn ui_name(&self) -> &'static str;
    fn render<'a>(&self, app: &AppContext) -> ElementBox;
    fn on_focus(&mut self, app: &mut MutableAppContext, window_id: usize, view_id: usize);
    fn on_blur(&mut self, app: &mut MutableAppContext, window_id: usize, view_id: usize);
    fn keymap_context(&self, app: &AppContext) -> keymap::Context;
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

    fn ui_name(&self) -> &'static str {
        T::ui_name()
    }

    fn render<'a>(&self, app: &AppContext) -> ElementBox {
        View::render(self, app)
    }

    fn on_focus(&mut self, app: &mut MutableAppContext, window_id: usize, view_id: usize) {
        let mut ctx = ViewContext::new(app, window_id, view_id);
        View::on_focus(self, &mut ctx);
    }

    fn on_blur(&mut self, app: &mut MutableAppContext, window_id: usize, view_id: usize) {
        let mut ctx = ViewContext::new(app, window_id, view_id);
        View::on_blur(self, &mut ctx);
    }

    fn keymap_context(&self, app: &AppContext) -> keymap::Context {
        View::keymap_context(self, app)
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

    pub fn background_executor(&self) -> &Arc<executor::Background> {
        &self.app.ctx.background
    }

    pub fn thread_pool(&self) -> &scoped_pool::Pool {
        &self.app.ctx.thread_pool
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
                    let mut ctx = ModelContext::new(app, model_id);
                    callback(model, payload, &mut ctx);
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
            .observations
            .entry(handle.model_id)
            .or_default()
            .push(Observation::FromModel {
                model_id: self.model_id,
                callback: Box::new(move |model, observed_id, app, model_id| {
                    let model = model.downcast_mut().expect("downcast is type safe");
                    let observed = ModelHandle::new(observed_id, &app.ctx.ref_counts);
                    let mut ctx = ModelContext::new(app, model_id);
                    callback(model, observed, &mut ctx);
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

    fn handle(&self) -> ModelHandle<T> {
        ModelHandle::new(self.model_id, &self.app.ctx.ref_counts)
    }

    pub fn spawn<S, F, U>(&mut self, future: S, callback: F) -> EntityTask<U>
    where
        S: 'static + Future,
        F: 'static + FnOnce(&mut T, S::Output, &mut ModelContext<T>) -> U,
        U: 'static,
    {
        let handle = self.handle();
        let task = self.app.spawn::<S, U>(future);

        self.app.future_handlers.borrow_mut().insert(
            task.id,
            Box::new(move |output, ctx| {
                let output = *output.downcast().unwrap();
                handle.update(ctx, |model, ctx| Box::new(callback(model, output, ctx)))
            }),
        );

        task
    }

    pub fn spawn_stream<S, F, G, U>(
        &mut self,
        stream: S,
        mut item_callback: F,
        done_callback: G,
    ) -> EntityTask<U>
    where
        S: 'static + Stream + Unpin,
        F: 'static + FnMut(&mut T, S::Item, &mut ModelContext<T>),
        G: 'static + FnOnce(&mut T, &mut ModelContext<T>) -> U,
        U: 'static + Any,
    {
        let handle = self.handle();
        let task = self.app.spawn_stream(stream);

        self.app.stream_handlers.borrow_mut().insert(
            task.id,
            StreamHandler {
                item_callback: {
                    let handle = handle.clone();
                    Box::new(move |output, app| {
                        let output = *output.downcast().unwrap();
                        handle.update(app, |model, ctx| {
                            item_callback(model, output, ctx);
                            ctx.halt_stream
                        })
                    })
                },
                done_callback: Box::new(move |app| {
                    handle.update(app, |model, ctx| Box::new(done_callback(model, ctx)))
                }),
            },
        );

        task
    }
}

impl<M> AsRef<AppContext> for ModelContext<'_, M> {
    fn as_ref(&self) -> &AppContext {
        &self.app.ctx
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

pub struct ViewContext<'a, T: ?Sized> {
    app: &'a mut MutableAppContext,
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
    halt_action_dispatch: bool,
    halt_stream: bool,
}

impl<'a, T: View> ViewContext<'a, T> {
    fn new(app: &'a mut MutableAppContext, window_id: usize, view_id: usize) -> Self {
        Self {
            app,
            window_id,
            view_id,
            view_type: PhantomData,
            halt_action_dispatch: true,
            halt_stream: false,
        }
    }

    pub fn handle(&self) -> ViewHandle<T> {
        ViewHandle::new(self.window_id, self.view_id, &self.app.ctx.ref_counts)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn background_executor(&self) -> &Arc<executor::Background> {
        &self.app.ctx.background
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
        self.app
            .subscriptions
            .entry(handle.id())
            .or_default()
            .push(Subscription::FromView {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |view, payload, app, window_id, view_id| {
                    if let Some(emitter_handle) = emitter_handle.upgrade(app.as_ref()) {
                        let model = view.downcast_mut().expect("downcast is type safe");
                        let payload = payload.downcast_ref().expect("downcast is type safe");
                        let mut ctx = ViewContext::new(app, window_id, view_id);
                        callback(model, emitter_handle, payload, &mut ctx);
                    }
                }),
            });
    }

    pub fn subscribe_to_view<V, F>(&mut self, handle: &ViewHandle<V>, mut callback: F)
    where
        V: View,
        V::Event: 'static,
        F: 'static + FnMut(&mut T, ViewHandle<V>, &V::Event, &mut ViewContext<T>),
    {
        let emitter_handle = handle.downgrade();

        self.app
            .subscriptions
            .entry(handle.id())
            .or_default()
            .push(Subscription::FromView {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |view, payload, app, window_id, view_id| {
                    if let Some(emitter_handle) = emitter_handle.upgrade(app.as_ref()) {
                        let model = view.downcast_mut().expect("downcast is type safe");
                        let payload = payload.downcast_ref().expect("downcast is type safe");
                        let mut ctx = ViewContext::new(app, window_id, view_id);
                        callback(model, emitter_handle, payload, &mut ctx);
                    }
                }),
            });
    }

    pub fn emit(&mut self, payload: T::Event) {
        self.app.pending_effects.push_back(Effect::Event {
            entity_id: self.view_id,
            payload: Box::new(payload),
        });
    }

    pub fn observe<S, F>(&mut self, handle: &ModelHandle<S>, mut callback: F)
    where
        S: Entity,
        F: 'static + FnMut(&mut T, ModelHandle<S>, &mut ViewContext<T>),
    {
        self.app
            .observations
            .entry(handle.id())
            .or_default()
            .push(Observation::FromView {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |view, observed_id, app, window_id, view_id| {
                    let view = view.downcast_mut().expect("downcast is type safe");
                    let observed = ModelHandle::new(observed_id, &app.ctx.ref_counts);
                    let mut ctx = ViewContext::new(app, window_id, view_id);
                    callback(view, observed, &mut ctx);
                }),
            });
    }

    pub fn notify(&mut self) {
        self.app
            .pending_effects
            .push_back(Effect::ViewNotification {
                window_id: self.window_id,
                view_id: self.view_id,
            });
    }

    pub fn propagate_action(&mut self) {
        self.halt_action_dispatch = false;
    }

    pub fn halt_stream(&mut self) {
        self.halt_stream = true;
    }

    pub fn spawn<S, F, U>(&mut self, future: S, callback: F) -> EntityTask<U>
    where
        S: 'static + Future,
        F: 'static + FnOnce(&mut T, S::Output, &mut ViewContext<T>) -> U,
        U: 'static,
    {
        let handle = self.handle();
        let task = self.app.spawn(future);

        self.app.future_handlers.borrow_mut().insert(
            task.id,
            Box::new(move |output, app| {
                let output = *output.downcast().unwrap();
                handle.update(app, |view, ctx| Box::new(callback(view, output, ctx)))
            }),
        );

        task
    }

    pub fn spawn_stream<S, F, G, U>(
        &mut self,
        stream: S,
        mut item_callback: F,
        done_callback: G,
    ) -> EntityTask<U>
    where
        S: 'static + Stream + Unpin,
        F: 'static + FnMut(&mut T, S::Item, &mut ViewContext<T>),
        G: 'static + FnOnce(&mut T, &mut ViewContext<T>) -> U,
        U: 'static + Any,
    {
        let handle = self.handle();
        let task = self.app.spawn_stream(stream);
        self.app.stream_handlers.borrow_mut().insert(
            task.id,
            StreamHandler {
                item_callback: {
                    let handle = handle.clone();
                    Box::new(move |output, app| {
                        let output = *output.downcast().unwrap();
                        handle.update(app, |view, ctx| {
                            item_callback(view, output, ctx);
                            ctx.halt_stream
                        })
                    })
                },
                done_callback: Box::new(move |app| {
                    handle.update(app, |view, ctx| Box::new(done_callback(view, ctx)))
                }),
            },
        );
        task
    }
}

impl<M> AsRef<AppContext> for ViewContext<'_, M> {
    fn as_ref(&self) -> &AppContext {
        &self.app.ctx
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
    ref_counts: Weak<Mutex<RefCounts>>,
}

impl<T: Entity> ModelHandle<T> {
    fn new(model_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc(model_id);
        Self {
            model_id,
            model_type: PhantomData,
            ref_counts: Arc::downgrade(ref_counts),
        }
    }

    fn downgrade(&self) -> WeakModelHandle<T> {
        WeakModelHandle::new(self.model_id)
    }

    pub fn id(&self) -> usize {
        self.model_id
    }

    pub fn read<'a, A: ReadModel>(&self, app: &'a A) -> &'a T {
        app.read_model(self)
    }

    pub fn update<A, F, S>(&self, app: &mut A, update: F) -> S
    where
        A: UpdateModel,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        app.update_model(self, update)
    }

    pub fn condition(
        &self,
        ctx: &TestAppContext,
        mut predicate: impl 'static + FnMut(&T, &AppContext) -> bool,
    ) -> impl 'static + Future<Output = ()> {
        let mut ctx = ctx.0.borrow_mut();
        let tx = ctx
            .async_observations
            .entry(self.id())
            .or_insert_with(|| postage::broadcast::channel(128).0);
        let mut rx = tx.subscribe();
        let ctx = ctx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();

        async move {
            timeout(Duration::from_millis(200), async move {
                loop {
                    {
                        let ctx = ctx.borrow();
                        let ctx = ctx.as_ref();
                        if predicate(
                            handle
                                .upgrade(ctx)
                                .expect("model dropped with pending condition")
                                .read(ctx),
                            ctx,
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
        if let Some(ref_counts) = self.ref_counts.upgrade() {
            ref_counts.lock().inc(self.model_id);
        }

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
        if let Some(ref_counts) = self.ref_counts.upgrade() {
            ref_counts.lock().dec_model(self.model_id);
        }
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

    pub fn upgrade(&self, app: &AppContext) -> Option<ModelHandle<T>> {
        if app.models.contains_key(&self.model_id) {
            Some(ModelHandle::new(self.model_id, &app.ref_counts))
        } else {
            None
        }
    }
}

pub struct ViewHandle<T> {
    window_id: usize,
    view_id: usize,
    view_type: PhantomData<T>,
    ref_counts: Weak<Mutex<RefCounts>>,
}

impl<T: View> ViewHandle<T> {
    fn new(window_id: usize, view_id: usize, ref_counts: &Arc<Mutex<RefCounts>>) -> Self {
        ref_counts.lock().inc(view_id);
        Self {
            window_id,
            view_id,
            view_type: PhantomData,
            ref_counts: Arc::downgrade(ref_counts),
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

    pub fn read<'a, A: ReadView>(&self, app: &'a A) -> &'a T {
        app.read_view(self)
    }

    pub fn update<A, F, S>(&self, app: &mut A, update: F) -> S
    where
        A: UpdateView,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S,
    {
        app.update_view(self, update)
    }

    pub fn is_focused(&self, app: &AppContext) -> bool {
        app.focused_view_id(self.window_id)
            .map_or(false, |focused_id| focused_id == self.view_id)
    }

    pub fn condition(
        &self,
        ctx: &TestAppContext,
        mut predicate: impl 'static + FnMut(&T, &AppContext) -> bool,
    ) -> impl 'static + Future<Output = ()> {
        let mut ctx = ctx.0.borrow_mut();
        let tx = ctx
            .async_observations
            .entry(self.id())
            .or_insert_with(|| postage::broadcast::channel(128).0);
        let mut rx = tx.subscribe();
        let ctx = ctx.weak_self.as_ref().unwrap().upgrade().unwrap();
        let handle = self.downgrade();

        async move {
            timeout(Duration::from_millis(200), async move {
                loop {
                    {
                        let ctx = ctx.borrow();
                        let ctx = ctx.as_ref();
                        if predicate(
                            handle
                                .upgrade(ctx)
                                .expect("model dropped with pending condition")
                                .read(ctx),
                            ctx,
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

impl<T> Clone for ViewHandle<T> {
    fn clone(&self) -> Self {
        if let Some(ref_counts) = self.ref_counts.upgrade() {
            ref_counts.lock().inc(self.view_id);
        }

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
        if let Some(ref_counts) = self.ref_counts.upgrade() {
            ref_counts.lock().dec_view(self.window_id, self.view_id);
        }
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

#[derive(Clone)]
pub struct AnyViewHandle {
    window_id: usize,
    view_id: usize,
    view_type: TypeId,
    ref_counts: Weak<Mutex<RefCounts>>,
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
            if let Some(ref_counts) = self.ref_counts.upgrade() {
                return Some(ViewHandle::new(self.window_id, self.view_id, &ref_counts));
            }
        }
        None
    }
}

impl<T: View> From<&ViewHandle<T>> for AnyViewHandle {
    fn from(handle: &ViewHandle<T>) -> Self {
        if let Some(ref_counts) = handle.ref_counts.upgrade() {
            ref_counts.lock().inc(handle.view_id);
        }
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
        (&handle).into()
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

    pub fn upgrade(&self, app: &AppContext) -> Option<ViewHandle<T>> {
        if app
            .windows
            .get(&self.window_id)
            .and_then(|w| w.views.get(&self.view_id))
            .is_some()
        {
            Some(ViewHandle::new(
                self.window_id,
                self.view_id,
                &app.ref_counts,
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

#[derive(Default)]
struct RefCounts {
    counts: HashMap<usize, usize>,
    dropped_models: HashSet<usize>,
    dropped_views: HashSet<(usize, usize)>,
}

impl RefCounts {
    fn inc(&mut self, model_id: usize) {
        *self.counts.entry(model_id).or_insert(0) += 1;
    }

    fn dec_model(&mut self, model_id: usize) {
        if let Some(count) = self.counts.get_mut(&model_id) {
            *count -= 1;
            if *count == 0 {
                self.counts.remove(&model_id);
                self.dropped_models.insert(model_id);
            }
        } else {
            panic!("Expected ref count to be positive")
        }
    }

    fn dec_view(&mut self, window_id: usize, view_id: usize) {
        if let Some(count) = self.counts.get_mut(&view_id) {
            *count -= 1;
            if *count == 0 {
                self.counts.remove(&view_id);
                self.dropped_views.insert((window_id, view_id));
            }
        } else {
            panic!("Expected ref count to be positive")
        }
    }

    fn take_dropped(&mut self) -> (HashSet<usize>, HashSet<(usize, usize)>) {
        let mut dropped_models = HashSet::new();
        let mut dropped_views = HashSet::new();
        std::mem::swap(&mut self.dropped_models, &mut dropped_models);
        std::mem::swap(&mut self.dropped_views, &mut dropped_views);
        (dropped_models, dropped_views)
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

enum Observation {
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

type FutureHandler = Box<dyn FnOnce(Box<dyn Any>, &mut MutableAppContext) -> Box<dyn Any>>;

struct StreamHandler {
    item_callback: Box<dyn FnMut(Box<dyn Any>, &mut MutableAppContext) -> bool>,
    done_callback: Box<dyn FnOnce(&mut MutableAppContext) -> Box<dyn Any>>,
}

#[must_use]
pub struct EntityTask<T> {
    id: usize,
    task: Option<executor::Task<T>>,
    handler_map: TaskHandlerMap,
}

enum TaskHandlerMap {
    Detached,
    Future(Rc<RefCell<HashMap<usize, FutureHandler>>>),
    Stream(Rc<RefCell<HashMap<usize, StreamHandler>>>),
}

impl<T> EntityTask<T> {
    fn new(id: usize, task: executor::Task<T>, handler_map: TaskHandlerMap) -> Self {
        Self {
            id,
            task: Some(task),
            handler_map,
        }
    }

    pub fn detach(mut self) {
        self.handler_map = TaskHandlerMap::Detached;
        self.task.take().unwrap().detach();
    }

    pub async fn cancel(mut self) -> Option<T> {
        let task = self.task.take().unwrap();
        task.cancel().await
    }
}

impl<T> Future for EntityTask<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let task = unsafe { self.map_unchecked_mut(|task| task.task.as_mut().unwrap()) };
        task.poll(ctx)
    }
}

impl<T> Drop for EntityTask<T> {
    fn drop(self: &mut Self) {
        match &self.handler_map {
            TaskHandlerMap::Detached => {
                return;
            }
            TaskHandlerMap::Future(map) => {
                map.borrow_mut().remove(&self.id);
            }
            TaskHandlerMap::Stream(map) => {
                map.borrow_mut().remove(&self.id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::*;
    use smol::future::poll_once;

    #[test]
    fn test_model_handles() {
        struct Model {
            other: Option<ModelHandle<Model>>,
            events: Vec<String>,
        }

        impl Entity for Model {
            type Event = usize;
        }

        impl Model {
            fn new(other: Option<ModelHandle<Self>>, ctx: &mut ModelContext<Self>) -> Self {
                if let Some(other) = other.as_ref() {
                    ctx.observe(other, |me, _, _| {
                        me.events.push("notified".into());
                    });
                    ctx.subscribe(other, |me, event, _| {
                        me.events.push(format!("observed event {}", event));
                    });
                }

                Self {
                    other,
                    events: Vec::new(),
                }
            }
        }

        App::test((), |app| {
            let handle_1 = app.add_model(|ctx| Model::new(None, ctx));
            let handle_2 = app.add_model(|ctx| Model::new(Some(handle_1.clone()), ctx));
            assert_eq!(app.ctx.models.len(), 2);

            handle_1.update(app, |model, ctx| {
                model.events.push("updated".into());
                ctx.emit(1);
                ctx.notify();
                ctx.emit(2);
            });
            assert_eq!(handle_1.read(app).events, vec!["updated".to_string()]);
            assert_eq!(
                handle_2.read(app).events,
                vec![
                    "observed event 1".to_string(),
                    "notified".to_string(),
                    "observed event 2".to_string(),
                ]
            );

            handle_2.update(app, |model, _| {
                drop(handle_1);
                model.other.take();
            });

            assert_eq!(app.ctx.models.len(), 1);
            assert!(app.subscriptions.is_empty());
            assert!(app.observations.is_empty());
        });
    }

    #[test]
    fn test_subscribe_and_emit_from_model() {
        #[derive(Default)]
        struct Model {
            events: Vec<usize>,
        }

        impl Entity for Model {
            type Event = usize;
        }

        App::test((), |app| {
            let handle_1 = app.add_model(|_| Model::default());
            let handle_2 = app.add_model(|_| Model::default());
            let handle_2b = handle_2.clone();

            handle_1.update(app, |_, c| {
                c.subscribe(&handle_2, move |model: &mut Model, event, c| {
                    model.events.push(*event);

                    c.subscribe(&handle_2b, |model, event, _| {
                        model.events.push(*event * 2);
                    });
                });
            });

            handle_2.update(app, |_, c| c.emit(7));
            assert_eq!(handle_1.read(app).events, vec![7]);

            handle_2.update(app, |_, c| c.emit(5));
            assert_eq!(handle_1.read(app).events, vec![7, 10, 5]);
        })
    }

    #[test]
    fn test_observe_and_notify_from_model() {
        #[derive(Default)]
        struct Model {
            count: usize,
            events: Vec<usize>,
        }

        impl Entity for Model {
            type Event = ();
        }

        App::test((), |app| {
            let handle_1 = app.add_model(|_| Model::default());
            let handle_2 = app.add_model(|_| Model::default());
            let handle_2b = handle_2.clone();

            handle_1.update(app, |_, c| {
                c.observe(&handle_2, move |model, observed, c| {
                    model.events.push(observed.read(c).count);
                    c.observe(&handle_2b, |model, observed, c| {
                        model.events.push(observed.read(c).count * 2);
                    });
                });
            });

            handle_2.update(app, |model, c| {
                model.count = 7;
                c.notify()
            });
            assert_eq!(handle_1.read(app).events, vec![7]);

            handle_2.update(app, |model, c| {
                model.count = 5;
                c.notify()
            });
            assert_eq!(handle_1.read(app).events, vec![7, 10, 5])
        })
    }

    #[test]
    fn test_spawn_from_model() {
        #[derive(Default)]
        struct Model {
            count: usize,
        }

        impl Entity for Model {
            type Event = ();
        }

        App::test_async((), |mut app| async move {
            let handle = app.add_model(|_| Model::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn(async { 7 }, |model, output, _| {
                        model.count = output;
                    })
                })
                .await;
            app.read(|ctx| assert_eq!(handle.read(ctx).count, 7));

            handle
                .update(&mut app, |_, c| {
                    c.spawn(async { 14 }, |model, output, _| {
                        model.count = output;
                    })
                })
                .await;
            app.read(|ctx| assert_eq!(handle.read(ctx).count, 14));
        });
    }

    #[test]
    fn test_spawn_stream_local_from_model() {
        #[derive(Default)]
        struct Model {
            events: Vec<Option<usize>>,
        }

        impl Entity for Model {
            type Event = ();
        }

        App::test_async((), |mut app| async move {
            let handle = app.add_model(|_| Model::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn_stream(
                        smol::stream::iter(vec![1, 2, 3]),
                        |model, output, _| {
                            model.events.push(Some(output));
                        },
                        |model, _| {
                            model.events.push(None);
                        },
                    )
                })
                .await;
            app.read(|ctx| assert_eq!(handle.read(ctx).events, [Some(1), Some(2), Some(3), None]));
        })
    }

    #[test]
    fn test_view_handles() {
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
            fn new(other: Option<ViewHandle<View>>, ctx: &mut ViewContext<Self>) -> Self {
                if let Some(other) = other.as_ref() {
                    ctx.subscribe_to_view(other, |me, _, event, _| {
                        me.events.push(format!("observed event {}", event));
                    });
                }
                Self {
                    other,
                    events: Vec::new(),
                }
            }
        }

        App::test((), |app| {
            let (window_id, _) = app.add_window(|ctx| View::new(None, ctx));
            let handle_1 = app.add_view(window_id, |ctx| View::new(None, ctx));
            let handle_2 = app.add_view(window_id, |ctx| View::new(Some(handle_1.clone()), ctx));
            assert_eq!(app.ctx.windows[&window_id].views.len(), 3);

            handle_1.update(app, |view, ctx| {
                view.events.push("updated".into());
                ctx.emit(1);
                ctx.emit(2);
            });
            assert_eq!(handle_1.read(app).events, vec!["updated".to_string()]);
            assert_eq!(
                handle_2.read(app).events,
                vec![
                    "observed event 1".to_string(),
                    "observed event 2".to_string(),
                ]
            );

            handle_2.update(app, |view, _| {
                drop(handle_1);
                view.other.take();
            });

            assert_eq!(app.ctx.windows[&window_id].views.len(), 2);
            assert!(app.subscriptions.is_empty());
            assert!(app.observations.is_empty());
        })
    }

    #[test]
    fn test_subscribe_and_emit_from_view() {
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

        App::test((), |app| {
            let (window_id, handle_1) = app.add_window(|_| View::default());
            let handle_2 = app.add_view(window_id, |_| View::default());
            let handle_2b = handle_2.clone();
            let handle_3 = app.add_model(|_| Model);

            handle_1.update(app, |_, c| {
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

            handle_2.update(app, |_, c| c.emit(7));
            assert_eq!(handle_1.read(app).events, vec![7]);

            handle_2.update(app, |_, c| c.emit(5));
            assert_eq!(handle_1.read(app).events, vec![7, 10, 5]);

            handle_3.update(app, |_, c| c.emit(9));
            assert_eq!(handle_1.read(app).events, vec![7, 10, 5, 9]);
        })
    }

    #[test]
    fn test_dropping_subscribers() {
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

        App::test((), |app| {
            let (window_id, _) = app.add_window(|_| View);
            let observing_view = app.add_view(window_id, |_| View);
            let emitting_view = app.add_view(window_id, |_| View);
            let observing_model = app.add_model(|_| Model);
            let observed_model = app.add_model(|_| Model);

            observing_view.update(app, |_, ctx| {
                ctx.subscribe_to_view(&emitting_view, |_, _, _, _| {});
                ctx.subscribe_to_model(&observed_model, |_, _, _, _| {});
            });
            observing_model.update(app, |_, ctx| {
                ctx.subscribe(&observed_model, |_, _, _| {});
            });

            app.update(|| {
                drop(observing_view);
                drop(observing_model);
            });

            emitting_view.update(app, |_, ctx| ctx.emit(()));
            observed_model.update(app, |_, ctx| ctx.emit(()));
        })
    }

    #[test]
    fn test_observe_and_notify_from_view() {
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

        App::test((), |app| {
            let (_, view) = app.add_window(|_| View::default());
            let model = app.add_model(|_| Model::default());

            view.update(app, |_, c| {
                c.observe(&model, |me, observed, c| {
                    me.events.push(observed.read(c).count)
                });
            });

            model.update(app, |model, c| {
                model.count = 11;
                c.notify();
            });
            assert_eq!(view.read(app).events, vec![11]);
        })
    }

    #[test]
    fn test_dropping_observers() {
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

        App::test((), |app| {
            let (window_id, _) = app.add_window(|_| View);
            let observing_view = app.add_view(window_id, |_| View);
            let observing_model = app.add_model(|_| Model);
            let observed_model = app.add_model(|_| Model);

            observing_view.update(app, |_, ctx| {
                ctx.observe(&observed_model, |_, _, _| {});
            });
            observing_model.update(app, |_, ctx| {
                ctx.observe(&observed_model, |_, _, _| {});
            });

            app.update(|| {
                drop(observing_view);
                drop(observing_model);
            });

            observed_model.update(app, |_, ctx| ctx.notify());
        })
    }

    #[test]
    fn test_focus() {
        #[derive(Default)]
        struct View {
            events: Vec<String>,
        }

        impl Entity for View {
            type Event = String;
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> ElementBox {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }

            fn on_focus(&mut self, ctx: &mut ViewContext<Self>) {
                self.events.push("self focused".into());
                ctx.emit("focused".into());
            }

            fn on_blur(&mut self, ctx: &mut ViewContext<Self>) {
                self.events.push("self blurred".into());
                ctx.emit("blurred".into());
            }
        }

        App::test((), |app| {
            let (window_id, view_1) = app.add_window(|_| View::default());
            let view_2 = app.add_view(window_id, |_| View::default());

            view_1.update(app, |_, ctx| {
                ctx.subscribe_to_view(&view_2, |view_1, _, event, _| {
                    view_1.events.push(format!("view 2 {}", event));
                });
                ctx.focus(&view_2);
            });

            view_1.update(app, |_, ctx| {
                ctx.focus(&view_1);
            });

            assert_eq!(
                view_1.read(app).events,
                [
                    "self focused".to_string(),
                    "self blurred".to_string(),
                    "view 2 focused".to_string(),
                    "self focused".to_string(),
                    "view 2 blurred".to_string(),
                ],
            );
        })
    }

    #[test]
    fn test_spawn_from_view() {
        #[derive(Default)]
        struct View {
            count: usize,
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
        }

        App::test_async((), |mut app| async move {
            let handle = app.add_window(|_| View::default()).1;
            handle
                .update(&mut app, |_, c| {
                    c.spawn(async { 7 }, |me, output, _| {
                        me.count = output;
                    })
                })
                .await;
            app.read(|ctx| assert_eq!(handle.read(ctx).count, 7));
            handle
                .update(&mut app, |_, c| {
                    c.spawn(async { 14 }, |me, output, _| {
                        me.count = output;
                    })
                })
                .await;
            app.read(|ctx| assert_eq!(handle.read(ctx).count, 14));
        });
    }

    #[test]
    fn test_spawn_stream_local_from_view() {
        #[derive(Default)]
        struct View {
            events: Vec<Option<usize>>,
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
        }

        App::test_async((), |mut app| async move {
            let (_, handle) = app.add_window(|_| View::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn_stream(
                        smol::stream::iter(vec![1_usize, 2, 3]),
                        |me, output, _| {
                            me.events.push(Some(output));
                        },
                        |me, _| {
                            me.events.push(None);
                        },
                    )
                })
                .await;

            app.read(|ctx| assert_eq!(handle.read(ctx).events, [Some(1), Some(2), Some(3), None]))
        });
    }

    #[test]
    fn test_dispatch_action() {
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

        App::test((), |app| {
            let actions = Rc::new(RefCell::new(Vec::new()));

            let actions_clone = actions.clone();
            app.add_global_action("action", move |_: &ActionArg, _: &mut MutableAppContext| {
                actions_clone.borrow_mut().push("global a".to_string());
            });

            let actions_clone = actions.clone();
            app.add_global_action("action", move |_: &ActionArg, _: &mut MutableAppContext| {
                actions_clone.borrow_mut().push("global b".to_string());
            });

            let actions_clone = actions.clone();
            app.add_action("action", move |view: &mut ViewA, arg: &ActionArg, ctx| {
                assert_eq!(arg.foo, "bar");
                ctx.propagate_action();
                actions_clone.borrow_mut().push(format!("{} a", view.id));
            });

            let actions_clone = actions.clone();
            app.add_action("action", move |view: &mut ViewA, _: &ActionArg, ctx| {
                if view.id != 1 {
                    ctx.propagate_action();
                }
                actions_clone.borrow_mut().push(format!("{} b", view.id));
            });

            let actions_clone = actions.clone();
            app.add_action("action", move |view: &mut ViewB, _: &ActionArg, ctx| {
                ctx.propagate_action();
                actions_clone.borrow_mut().push(format!("{} c", view.id));
            });

            let actions_clone = actions.clone();
            app.add_action("action", move |view: &mut ViewB, _: &ActionArg, ctx| {
                ctx.propagate_action();
                actions_clone.borrow_mut().push(format!("{} d", view.id));
            });

            let (window_id, view_1) = app.add_window(|_| ViewA { id: 1 });
            let view_2 = app.add_view(window_id, |_| ViewB { id: 2 });
            let view_3 = app.add_view(window_id, |_| ViewA { id: 3 });
            let view_4 = app.add_view(window_id, |_| ViewB { id: 4 });

            app.dispatch_action(
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
            app.dispatch_action(
                window_id,
                vec![view_2.id(), view_3.id(), view_4.id()],
                "action",
                ActionArg { foo: "bar".into() },
            );

            assert_eq!(
                *actions.borrow(),
                vec!["4 d", "4 c", "3 b", "3 a", "2 d", "2 c", "global b", "global a"]
            );
        })
    }

    #[test]
    fn test_dispatch_keystroke() {
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

        App::test((), |app| {
            let mut view_1 = View::new(1);
            let mut view_2 = View::new(2);
            let mut view_3 = View::new(3);
            view_1.keymap_context.set.insert("a".into());
            view_2.keymap_context.set.insert("b".into());
            view_3.keymap_context.set.insert("c".into());

            let (window_id, view_1) = app.add_window(|_| view_1);
            let view_2 = app.add_view(window_id, |_| view_2);
            let view_3 = app.add_view(window_id, |_| view_3);

            // This keymap's only binding dispatches an action on view 2 because that view will have
            // "a" and "b" in its context, but not "c".
            let binding = keymap::Binding::new("a", "action", Some("a && b && !c"))
                .with_arg(ActionArg { key: "a".into() });
            app.add_bindings(vec![binding]);

            let handled_action = Rc::new(Cell::new(false));
            let handled_action_clone = handled_action.clone();
            app.add_action("action", move |view: &mut View, arg: &ActionArg, _ctx| {
                handled_action_clone.set(true);
                assert_eq!(view.id, 2);
                assert_eq!(arg.key, "a");
            });

            app.dispatch_keystroke(
                window_id,
                vec![view_1.id(), view_2.id(), view_3.id()],
                &Keystroke::parse("a").unwrap(),
            )
            .unwrap();

            assert!(handled_action.get());
        });
    }

    #[test]
    fn test_model_condition() {
        struct Counter(usize);

        impl super::Entity for Counter {
            type Event = ();
        }

        impl Counter {
            fn inc(&mut self, ctx: &mut ModelContext<Self>) {
                self.0 += 1;
                ctx.notify();
            }
        }

        App::test_async((), |mut app| async move {
            let model = app.add_model(|_| Counter(0));

            let condition1 = model.condition(&app, |model, _| model.0 == 2);
            let condition2 = model.condition(&app, |model, _| model.0 == 3);
            smol::pin!(condition1, condition2);

            model.update(&mut app, |model, ctx| model.inc(ctx));
            assert_eq!(poll_once(&mut condition1).await, None);
            assert_eq!(poll_once(&mut condition2).await, None);

            model.update(&mut app, |model, ctx| model.inc(ctx));
            assert_eq!(poll_once(&mut condition1).await, Some(()));
            assert_eq!(poll_once(&mut condition2).await, None);

            model.update(&mut app, |model, ctx| model.inc(ctx));
            assert_eq!(poll_once(&mut condition2).await, Some(()));

            // Broadcast channel should be removed if no conditions remain on next notification.
            model.update(&mut app, |_, ctx| ctx.notify());
            app.update(|ctx| assert!(ctx.async_observations.get(&model.id()).is_none()));
        });
    }

    #[test]
    #[should_panic]
    fn test_model_condition_timeout() {
        struct Model;

        impl super::Entity for Model {
            type Event = ();
        }

        App::test_async((), |mut app| async move {
            let model = app.add_model(|_| Model);
            model.condition(&app, |_, _| false).await;
        });
    }

    #[test]
    #[should_panic(expected = "model dropped with pending condition")]
    fn test_model_condition_panic_on_drop() {
        struct Model;

        impl super::Entity for Model {
            type Event = ();
        }

        App::test_async((), |mut app| async move {
            let model = app.add_model(|_| Model);
            let condition = model.condition(&app, |_, _| false);
            app.update(|_| drop(model));
            condition.await;
        });
    }

    #[test]
    fn test_view_condition() {
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
            fn inc(&mut self, ctx: &mut ViewContext<Self>) {
                self.0 += 1;
                ctx.notify();
            }
        }

        App::test_async((), |mut app| async move {
            let (_, view) = app.add_window(|_| Counter(0));

            let condition1 = view.condition(&app, |view, _| view.0 == 2);
            let condition2 = view.condition(&app, |view, _| view.0 == 3);
            smol::pin!(condition1, condition2);

            view.update(&mut app, |view, ctx| view.inc(ctx));
            assert_eq!(poll_once(&mut condition1).await, None);
            assert_eq!(poll_once(&mut condition2).await, None);

            view.update(&mut app, |view, ctx| view.inc(ctx));
            assert_eq!(poll_once(&mut condition1).await, Some(()));
            assert_eq!(poll_once(&mut condition2).await, None);

            view.update(&mut app, |view, ctx| view.inc(ctx));
            assert_eq!(poll_once(&mut condition2).await, Some(()));

            // Broadcast channel should be removed if no conditions remain on next notification.
            view.update(&mut app, |_, ctx| ctx.notify());
            app.update(|ctx| assert!(ctx.async_observations.get(&view.id()).is_none()));
        });
    }

    #[test]
    #[should_panic]
    fn test_view_condition_timeout() {
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

        App::test_async((), |mut app| async move {
            let (_, view) = app.add_window(|_| View);
            view.condition(&app, |_, _| false).await;
        });
    }

    #[test]
    #[should_panic(expected = "model dropped with pending condition")]
    fn test_view_condition_panic_on_drop() {
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

        App::test_async((), |mut app| async move {
            let window_id = app.add_window(|_| View).0;
            let view = app.add_view(window_id, |_| View);

            let condition = view.condition(&app, |_, _| false);
            app.update(|_| drop(view));
            condition.await;
        });
    }

    // #[test]
    // fn test_ui_and_window_updates() {
    //     struct View {
    //         count: usize,
    //     }

    //     impl Entity for View {
    //         type Event = ();
    //     }

    //     impl super::View for View {
    //         fn render<'a>(&self, _: &AppContext) -> ElementBox {
    //             Empty::new().boxed()
    //         }

    //         fn ui_name() -> &'static str {
    //             "View"
    //         }
    //     }

    //     App::test(|app| async move {
    //         let (window_id, _) = app.add_window(|_| View { count: 3 });
    //         let view_1 = app.add_view(window_id, |_| View { count: 1 });
    //         let view_2 = app.add_view(window_id, |_| View { count: 2 });

    //         // Ensure that registering for UI updates after mutating the app still gives us all the
    //         // updates.
    //         let ui_updates = Rc::new(RefCell::new(Vec::new()));
    //         let ui_updates_ = ui_updates.clone();
    //         app.on_ui_update(move |update, _| ui_updates_.borrow_mut().push(update));

    //         assert_eq!(
    //             ui_updates.borrow_mut().drain(..).collect::<Vec<_>>(),
    //             vec![UiUpdate::OpenWindow {
    //                 window_id,
    //                 width: 1024.0,
    //                 height: 768.0,
    //             }]
    //         );

    //         let window_invalidations = Rc::new(RefCell::new(Vec::new()));
    //         let window_invalidations_ = window_invalidations.clone();
    //         app.on_window_invalidated(window_id, move |update, _| {
    //             window_invalidations_.borrow_mut().push(update)
    //         });

    //         let view_2_id = view_2.id();
    //         view_1.update(app, |view, ctx| {
    //             view.count = 7;
    //             ctx.notify();
    //             drop(view_2);
    //         });

    //         let invalidation = window_invalidations.borrow_mut().drain(..).next().unwrap();
    //         assert_eq!(invalidation.updated.len(), 1);
    //         assert!(invalidation.updated.contains(&view_1.id()));
    //         assert_eq!(invalidation.removed, vec![view_2_id]);

    //         let view_3 = view_1.update(app, |_, ctx| ctx.add_view(|_| View { count: 8 }));

    //         let invalidation = window_invalidations.borrow_mut().drain(..).next().unwrap();
    //         assert_eq!(invalidation.updated.len(), 1);
    //         assert!(invalidation.updated.contains(&view_3.id()));
    //         assert!(invalidation.removed.is_empty());

    //         view_3
    //             .update(app, |_, ctx| {
    //                 ctx.spawn_local(async { 9 }, |me, output, ctx| {
    //                     me.count = output;
    //                     ctx.notify();
    //                 })
    //             })
    //             .await;

    //         let invalidation = window_invalidations.borrow_mut().drain(..).next().unwrap();
    //         assert_eq!(invalidation.updated.len(), 1);
    //         assert!(invalidation.updated.contains(&view_3.id()));
    //         assert!(invalidation.removed.is_empty());
    //     });
    // }
}
