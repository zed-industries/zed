use crate::{
    elements::Element,
    executor,
    keymap::{self, Keystroke},
    util::post_inc,
};
use anyhow::{anyhow, Result};
use keymap::MatchResult;
use parking_lot::Mutex;
use smol::{channel, prelude::*};
use std::{
    any::{type_name, Any, TypeId},
    borrow,
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
    rc::{self, Rc},
    sync::{Arc, Weak},
};

pub trait Entity: 'static + Send + Sync {
    type Event;
}

pub trait View: Entity {
    fn ui_name() -> &'static str;
    fn render<'a>(&self, app: &AppContext) -> Box<dyn Element>;
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

pub trait ModelAsRef {
    fn model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T;
}

pub trait UpdateModel {
    fn update_model<T, F, S>(&mut self, handle: &ModelHandle<T>, update: F) -> S
    where
        T: Entity,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S;
}

pub trait ViewAsRef {
    fn view<T: View>(&self, handle: &ViewHandle<T>) -> &T;
}

pub trait UpdateView {
    fn update_view<T, F, S>(&mut self, handle: &ViewHandle<T>, update: F) -> S
    where
        T: View,
        F: FnOnce(&mut T, &mut ViewContext<T>) -> S;
}

#[derive(Clone)]
pub struct App(Rc<RefCell<MutableAppContext>>);

impl App {
    pub fn test<T, F: Future<Output = T>>(f: impl FnOnce(App) -> F) -> T {
        let foreground = Rc::new(executor::Foreground::test());
        let app = Self(Rc::new(RefCell::new(
            MutableAppContext::with_foreground_executor(foreground.clone()),
        )));
        app.0.borrow_mut().weak_self = Some(Rc::downgrade(&app.0));
        smol::block_on(foreground.run(f(app)))
    }

    pub fn new() -> Result<Self> {
        let app = Self(Rc::new(RefCell::new(MutableAppContext::new()?)));
        app.0.borrow_mut().weak_self = Some(Rc::downgrade(&app.0));
        Ok(app)
    }

    pub fn on_window_invalidated<F: 'static + FnMut(WindowInvalidation, &mut MutableAppContext)>(
        &self,
        window_id: usize,
        callback: F,
    ) {
        self.0
            .borrow_mut()
            .on_window_invalidated(window_id, callback);
    }

    pub fn add_action<S, V, T, F>(&self, name: S, handler: F)
    where
        S: Into<String>,
        V: View,
        T: Any,
        F: 'static + FnMut(&mut V, &T, &mut ViewContext<V>),
    {
        self.0.borrow_mut().add_action(name, handler);
    }

    pub fn add_global_action<S, T, F>(&self, name: S, handler: F)
    where
        S: Into<String>,
        T: 'static + Any,
        F: 'static + FnMut(&T, &mut MutableAppContext),
    {
        self.0.borrow_mut().add_global_action(name, handler);
    }

    pub fn dispatch_action<T: 'static + Any>(
        &self,
        window_id: usize,
        responder_chain: Vec<usize>,
        name: &str,
        arg: T,
    ) {
        self.0.borrow_mut().dispatch_action(
            window_id,
            &responder_chain,
            name,
            Box::new(arg).as_ref(),
        );
    }

    pub fn dispatch_global_action<T: 'static + Any>(&self, name: &str, arg: T) {
        self.0
            .borrow_mut()
            .dispatch_global_action(name, Box::new(arg).as_ref());
    }

    pub fn add_bindings<T: IntoIterator<Item = keymap::Binding>>(&self, bindings: T) {
        self.0.borrow_mut().add_bindings(bindings);
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

    fn read_model<T, F, S>(&self, handle: &ModelHandle<T>, read: F) -> S
    where
        T: Entity,
        F: FnOnce(&T, &AppContext) -> S,
    {
        let state = self.0.borrow();
        read(state.model(handle), &state.ctx)
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

    pub fn read<T, F: FnOnce(&AppContext) -> T>(&mut self, callback: F) -> T {
        callback(self.0.borrow().ctx())
    }

    pub fn update<T, F: FnOnce(&mut MutableAppContext) -> T>(&mut self, callback: F) -> T {
        let mut state = self.0.borrow_mut();
        state.pending_flushes += 1;
        let result = callback(&mut *state);
        state.flush_effects();
        result
    }

    fn read_view<T, F, S>(&self, handle: &ViewHandle<T>, read: F) -> S
    where
        T: View,
        F: FnOnce(&T, &AppContext) -> S,
    {
        let state = self.0.borrow();
        read(state.view(handle), state.ctx())
    }

    #[cfg(test)]
    pub fn finish_pending_tasks(&self) -> impl Future<Output = ()> {
        self.0.borrow().finish_pending_tasks()
    }
}

impl UpdateModel for App {
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

impl UpdateView for App {
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
    ctx: AppContext,
    actions: HashMap<TypeId, HashMap<String, Vec<Box<ActionCallback>>>>,
    global_actions: HashMap<String, Vec<Box<GlobalActionCallback>>>,
    keystroke_matcher: keymap::Matcher,
    next_entity_id: usize,
    next_window_id: usize,
    next_task_id: usize,
    weak_self: Option<rc::Weak<RefCell<Self>>>,
    subscriptions: HashMap<usize, Vec<Subscription>>,
    observations: HashMap<usize, Vec<Observation>>,
    window_invalidations: HashMap<usize, WindowInvalidation>,
    invalidation_callbacks:
        HashMap<usize, Box<dyn FnMut(WindowInvalidation, &mut MutableAppContext)>>,
    foreground: Rc<executor::Foreground>,
    background: Arc<executor::Background>,
    task_callbacks: HashMap<usize, TaskCallback>,
    task_done: (channel::Sender<usize>, channel::Receiver<usize>),
    pending_effects: VecDeque<Effect>,
    pending_flushes: usize,
    flushing_effects: bool,
}

impl MutableAppContext {
    pub fn new() -> Result<Self> {
        Ok(Self::with_foreground_executor(Rc::new(
            executor::Foreground::platform(todo!())?,
        )))
    }

    fn with_foreground_executor(foreground: Rc<executor::Foreground>) -> Self {
        Self {
            ctx: AppContext {
                models: HashMap::new(),
                windows: HashMap::new(),
                ref_counts: Arc::new(Mutex::new(RefCounts::default())),
            },
            actions: HashMap::new(),
            global_actions: HashMap::new(),
            keystroke_matcher: keymap::Matcher::default(),
            next_entity_id: 0,
            next_window_id: 0,
            next_task_id: 0,
            weak_self: None,
            subscriptions: HashMap::new(),
            observations: HashMap::new(),
            window_invalidations: HashMap::new(),
            invalidation_callbacks: HashMap::new(),
            foreground,
            background: Arc::new(executor::Background::new()),
            task_callbacks: HashMap::new(),
            task_done: channel::unbounded(),
            pending_effects: VecDeque::new(),
            pending_flushes: 0,
            flushing_effects: false,
        }
    }

    pub fn ctx(&self) -> &AppContext {
        &self.ctx
    }

    pub fn foreground_executor(&self) -> Rc<executor::Foreground> {
        self.foreground.clone()
    }

    pub fn on_window_invalidated<F: 'static + FnMut(WindowInvalidation, &mut MutableAppContext)>(
        &mut self,
        window_id: usize,
        callback: F,
    ) {
        self.invalidation_callbacks
            .insert(window_id, Box::new(callback));
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

    pub fn render_view(&self, window_id: usize, view_id: usize) -> Result<Box<dyn Element>> {
        self.ctx.render_view(window_id, view_id)
    }

    pub fn render_views(&self, window_id: usize) -> Result<HashMap<usize, Box<dyn Element>>> {
        self.ctx.render_views(window_id)
    }

    pub fn dispatch_action(
        &mut self,
        window_id: usize,
        responder_chain: &[usize],
        name: &str,
        arg: &dyn Any,
    ) -> bool {
        self.pending_flushes += 1;
        let mut halted_dispatch = false;

        for view_id in responder_chain.iter().rev() {
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
            self.dispatch_global_action(name, arg);
        }

        self.flush_effects();
        halted_dispatch
    }

    fn dispatch_global_action(&mut self, name: &str, arg: &dyn Any) {
        if let Some((name, mut handlers)) = self.global_actions.remove_entry(name) {
            self.pending_flushes += 1;
            for handler in handlers.iter_mut().rev() {
                handler(arg, self);
            }
            self.global_actions.insert(name, handlers);
            self.flush_effects();
        }
    }

    fn add_bindings<T: IntoIterator<Item = keymap::Binding>>(&mut self, bindings: T) {
        self.keystroke_matcher.add_bindings(bindings);
    }

    pub fn dispatch_keystroke(
        &mut self,
        window_id: usize,
        responder_chain: Vec<usize>,
        keystroke: &Keystroke,
    ) -> Result<bool> {
        log::info!(
            "dispatch_keystroke {} {:?} {:?}",
            window_id,
            responder_chain,
            keystroke
        );

        let mut context_chain = Vec::new();
        let mut context = keymap::Context::default();
        for view_id in &responder_chain {
            if let Some(view) = self
                .ctx
                .windows
                .get(&window_id)
                .and_then(|w| w.views.get(view_id))
            {
                context.extend(view.keymap_context(self.ctx()));
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
                    if self.dispatch_action(
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
        let window_id = post_inc(&mut self.next_window_id);
        self.ctx.windows.insert(window_id, Window::default());

        let root_handle = self.add_view(window_id, build_root_view);
        self.ctx.windows.get_mut(&window_id).unwrap().root_view = Some(root_handle.clone().into());
        self.focus(window_id, root_handle.id());

        // self.emit_ui_update(UiUpdate::OpenWindow {
        //     window_id,
        //     width: 1024.0,
        //     height: 768.0,
        // });

        (window_id, root_handle)
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
            }

            for (window_id, view_id) in dropped_views {
                self.subscriptions.remove(&view_id);
                self.observations.remove(&view_id);
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
        self.pending_flushes -= 1;

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
            if let Some(mut callback) = self.invalidation_callbacks.remove(&window_id) {
                callback(invalidation, self);
                self.invalidation_callbacks.insert(window_id, callback);
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
    }

    fn notify_view_observers(&mut self, window_id: usize, view_id: usize) {
        self.window_invalidations
            .entry(window_id)
            .or_default()
            .updated
            .insert(view_id);
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

    fn spawn_local<F>(&mut self, future: F) -> usize
    where
        F: 'static + Future,
    {
        let task_id = post_inc(&mut self.next_task_id);
        let app = self.weak_self.as_ref().unwrap().clone();
        self.foreground
            .spawn(async move {
                let output = future.await;
                if let Some(app) = app.upgrade() {
                    app.borrow_mut()
                        .relay_task_output(task_id, Box::new(output));
                }
            })
            .detach();
        task_id
    }

    fn spawn_stream_local<F>(&mut self, mut stream: F, done_tx: channel::Sender<()>) -> usize
    where
        F: 'static + Stream + Unpin,
    {
        let task_id = post_inc(&mut self.next_task_id);
        let app = self.weak_self.as_ref().unwrap().clone();
        self.foreground
            .spawn(async move {
                loop {
                    match stream.next().await {
                        item @ Some(_) => {
                            if let Some(app) = app.upgrade() {
                                let mut app = app.borrow_mut();
                                if app.relay_task_output(task_id, Box::new(item)) {
                                    app.stream_completed(task_id);
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        item @ None => {
                            if let Some(app) = app.upgrade() {
                                let mut app = app.borrow_mut();
                                app.relay_task_output(task_id, Box::new(item));
                                app.stream_completed(task_id);
                            }
                            let _ = done_tx.send(()).await;
                            break;
                        }
                    }
                }
            })
            .detach();
        task_id
    }

    fn relay_task_output(&mut self, task_id: usize, output: Box<dyn Any>) -> bool {
        self.pending_flushes += 1;
        let task_callback = self.task_callbacks.remove(&task_id).unwrap();

        let halt = match task_callback {
            TaskCallback::OnModelFromFuture { model_id, callback } => {
                if let Some(mut model) = self.ctx.models.remove(&model_id) {
                    callback(
                        model.as_any_mut(),
                        output,
                        self,
                        model_id,
                        self.foreground.clone(),
                    );
                    self.ctx.models.insert(model_id, model);
                }
                self.task_done(task_id);
                true
            }
            TaskCallback::OnModelFromStream {
                model_id,
                mut callback,
            } => {
                if let Some(mut model) = self.ctx.models.remove(&model_id) {
                    let halt = callback(model.as_any_mut(), output, self, model_id);
                    self.ctx.models.insert(model_id, model);
                    self.task_callbacks.insert(
                        task_id,
                        TaskCallback::OnModelFromStream { model_id, callback },
                    );
                    halt
                } else {
                    true
                }
            }
            TaskCallback::OnViewFromFuture {
                window_id,
                view_id,
                callback,
            } => {
                if let Some(mut view) = self
                    .ctx
                    .windows
                    .get_mut(&window_id)
                    .and_then(|w| w.views.remove(&view_id))
                {
                    callback(
                        view.as_mut(),
                        output,
                        self,
                        window_id,
                        view_id,
                        self.foreground.clone(),
                    );
                    self.ctx
                        .windows
                        .get_mut(&window_id)
                        .unwrap()
                        .views
                        .insert(view_id, view);
                }
                self.task_done(task_id);
                true
            }
            TaskCallback::OnViewFromStream {
                window_id,
                view_id,
                mut callback,
            } => {
                if let Some(mut view) = self
                    .ctx
                    .windows
                    .get_mut(&window_id)
                    .and_then(|w| w.views.remove(&view_id))
                {
                    let halt = callback(view.as_mut(), output, self, window_id, view_id);
                    self.ctx
                        .windows
                        .get_mut(&window_id)
                        .unwrap()
                        .views
                        .insert(view_id, view);
                    self.task_callbacks.insert(
                        task_id,
                        TaskCallback::OnViewFromStream {
                            window_id,
                            view_id,
                            callback,
                        },
                    );
                    halt
                } else {
                    true
                }
            }
        };
        self.flush_effects();
        halt
    }

    fn stream_completed(&mut self, task_id: usize) {
        self.task_callbacks.remove(&task_id);
        self.task_done(task_id);
    }

    fn task_done(&self, task_id: usize) {
        let task_done = self.task_done.0.clone();
        self.foreground
            .spawn(async move {
                let _ = task_done.send(task_id).await;
            })
            .detach()
    }

    #[cfg(test)]
    pub fn finish_pending_tasks(&self) -> impl Future<Output = ()> {
        let mut pending_tasks = self.task_callbacks.keys().cloned().collect::<HashSet<_>>();
        let task_done = self.task_done.1.clone();

        async move {
            while !pending_tasks.is_empty() {
                if let Ok(task_id) = task_done.recv().await {
                    pending_tasks.remove(&task_id);
                } else {
                    break;
                }
            }
        }
    }
}

impl ModelAsRef for MutableAppContext {
    fn model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
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

impl ViewAsRef for MutableAppContext {
    fn view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
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

pub struct AppContext {
    models: HashMap<usize, Box<dyn AnyModel>>,
    windows: HashMap<usize, Window>,
    ref_counts: Arc<Mutex<RefCounts>>,
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

    pub fn render_view(&self, window_id: usize, view_id: usize) -> Result<Box<dyn Element>> {
        self.windows
            .get(&window_id)
            .and_then(|w| w.views.get(&view_id))
            .map(|v| v.render(self))
            .ok_or(anyhow!("view not found"))
    }

    pub fn render_views(&self, window_id: usize) -> Result<HashMap<usize, Box<dyn Element>>> {
        self.windows
            .get(&window_id)
            .map(|w| {
                w.views
                    .iter()
                    .map(|(id, view)| (*id, view.render(self)))
                    .collect::<HashMap<_, Box<dyn Element>>>()
            })
            .ok_or(anyhow!("window not found"))
    }
}

impl ModelAsRef for AppContext {
    fn model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
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

impl ViewAsRef for AppContext {
    fn view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
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
    fn render<'a>(&self, app: &AppContext) -> Box<dyn Element>;
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

    fn render<'a>(&self, app: &AppContext) -> Box<dyn Element> {
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

    pub fn app(&self) -> &AppContext {
        &self.app.ctx
    }

    pub fn app_mut(&mut self) -> &mut MutableAppContext {
        self.app
    }

    pub fn background_executor(&self) -> Arc<executor::Background> {
        self.app.background.clone()
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

    pub fn spawn_local<S, F, U>(&mut self, future: S, callback: F) -> impl Future<Output = U>
    where
        S: 'static + Future,
        F: 'static + FnOnce(&mut T, S::Output, &mut ModelContext<T>) -> U,
        U: 'static,
    {
        let (tx, rx) = channel::bounded(1);

        let task_id = self.app.spawn_local(future);

        self.app.task_callbacks.insert(
            task_id,
            TaskCallback::OnModelFromFuture {
                model_id: self.model_id,
                callback: Box::new(move |model, output, app, model_id, executor| {
                    let model = model.downcast_mut().unwrap();
                    let output = *output.downcast().unwrap();
                    let result = callback(model, output, &mut ModelContext::new(app, model_id));
                    executor
                        .spawn(async move { tx.send(result).await })
                        .detach();
                }),
            },
        );

        async move { rx.recv().await.unwrap() }
    }

    pub fn spawn<S, F, U>(&mut self, future: S, callback: F) -> impl Future<Output = U>
    where
        S: 'static + Future + Send,
        S::Output: Send,
        F: 'static + FnOnce(&mut T, S::Output, &mut ModelContext<T>) -> U,
        U: 'static,
    {
        let (tx, rx) = channel::bounded(1);

        self.app
            .background
            .spawn(async move {
                if let Err(_) = tx.send(future.await).await {
                    log::error!("Error sending background task result to main thread",);
                }
            })
            .detach();

        self.spawn_local(async move { rx.recv().await.unwrap() }, callback)
    }

    pub fn spawn_stream_local<S, F>(
        &mut self,
        stream: S,
        mut callback: F,
    ) -> impl Future<Output = ()>
    where
        S: 'static + Stream + Unpin,
        F: 'static + FnMut(&mut T, Option<S::Item>, &mut ModelContext<T>),
    {
        let (tx, rx) = channel::bounded(1);

        let task_id = self.app.spawn_stream_local(stream, tx);
        self.app.task_callbacks.insert(
            task_id,
            TaskCallback::OnModelFromStream {
                model_id: self.model_id,
                callback: Box::new(move |model, output, app, model_id| {
                    let model = model.downcast_mut().unwrap();
                    let output = *output.downcast().unwrap();
                    let mut ctx = ModelContext::new(app, model_id);
                    callback(model, output, &mut ctx);
                    ctx.halt_stream
                }),
            },
        );

        async move { rx.recv().await.unwrap() }
    }
}

impl<M> ModelAsRef for ModelContext<'_, M> {
    fn model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        self.app.model(handle)
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

    pub fn handle(&self) -> WeakViewHandle<T> {
        WeakViewHandle::new(self.window_id, self.view_id)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn app(&self) -> &AppContext {
        &self.app.ctx
    }

    pub fn app_mut(&mut self) -> &mut MutableAppContext {
        self.app
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
                    if let Some(emitter_handle) = emitter_handle.upgrade(app.ctx()) {
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
                    if let Some(emitter_handle) = emitter_handle.upgrade(app.ctx()) {
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

    pub fn spawn_local<S, F, U>(&mut self, future: S, callback: F) -> impl Future<Output = U>
    where
        S: 'static + Future,
        F: 'static + FnOnce(&mut T, S::Output, &mut ViewContext<T>) -> U,
        U: 'static,
    {
        let (tx, rx) = channel::bounded(1);

        let task_id = self.app.spawn_local(future);

        self.app.task_callbacks.insert(
            task_id,
            TaskCallback::OnViewFromFuture {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |view, output, app, window_id, view_id, executor| {
                    let view = view.as_any_mut().downcast_mut().unwrap();
                    let output = *output.downcast().unwrap();
                    let result =
                        callback(view, output, &mut ViewContext::new(app, window_id, view_id));
                    executor
                        .spawn(async move { tx.send(result).await })
                        .detach();
                }),
            },
        );

        async move { rx.recv().await.unwrap() }
    }

    pub fn spawn<S, F, U>(&mut self, future: S, callback: F) -> impl Future<Output = U>
    where
        S: 'static + Future + Send,
        S::Output: Send,
        F: 'static + FnOnce(&mut T, S::Output, &mut ViewContext<T>) -> U,
        U: 'static,
    {
        let (tx, rx) = channel::bounded(1);

        self.app
            .background
            .spawn(async move {
                if let Err(_) = tx.send(future.await).await {
                    log::error!("Error sending background task result to main thread",);
                }
            })
            .detach();

        self.spawn_local(async move { rx.recv().await.unwrap() }, callback)
    }

    pub fn spawn_stream_local<S, F>(
        &mut self,
        stream: S,
        mut callback: F,
    ) -> impl Future<Output = ()>
    where
        S: 'static + Stream + Unpin,
        F: 'static + FnMut(&mut T, Option<S::Item>, &mut ViewContext<T>),
    {
        let (tx, rx) = channel::bounded(1);

        let task_id = self.app.spawn_stream_local(stream, tx);
        self.app.task_callbacks.insert(
            task_id,
            TaskCallback::OnViewFromStream {
                window_id: self.window_id,
                view_id: self.view_id,
                callback: Box::new(move |view, output, app, window_id, view_id| {
                    let view = view.as_any_mut().downcast_mut().unwrap();
                    let output = *output.downcast().unwrap();
                    let mut ctx = ViewContext::new(app, window_id, view_id);
                    callback(view, output, &mut ctx);
                    ctx.halt_stream
                }),
            },
        );

        async move { rx.recv().await.unwrap() }
    }
}

impl<V> ModelAsRef for ViewContext<'_, V> {
    fn model<T: Entity>(&self, handle: &ModelHandle<T>) -> &T {
        self.app.model(handle)
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

impl<V: View> ViewAsRef for ViewContext<'_, V> {
    fn view<T: View>(&self, handle: &ViewHandle<T>) -> &T {
        self.app.view(handle)
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

    pub fn as_ref<'a, A: ModelAsRef>(&self, app: &'a A) -> &'a T {
        app.model(self)
    }

    pub fn read<'a, S, F>(&self, app: &'a App, read: F) -> S
    where
        F: FnOnce(&T, &AppContext) -> S,
    {
        app.read_model(self, read)
    }

    pub fn update<A, F, S>(&self, app: &mut A, update: F) -> S
    where
        A: UpdateModel,
        F: FnOnce(&mut T, &mut ModelContext<T>) -> S,
    {
        app.update_model(self, update)
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

impl<T> borrow::Borrow<usize> for ModelHandle<T> {
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

    fn downgrade(&self) -> WeakViewHandle<T> {
        WeakViewHandle::new(self.window_id, self.view_id)
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn id(&self) -> usize {
        self.view_id
    }

    pub fn as_ref<'a, A: ViewAsRef>(&self, app: &'a A) -> &'a T {
        app.view(self)
    }

    pub fn read<'a, F, S>(&self, app: &'a App, read: F) -> S
    where
        F: FnOnce(&T, &AppContext) -> S,
    {
        app.read_view(self, read)
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

enum TaskCallback {
    OnModelFromFuture {
        model_id: usize,
        callback: Box<
            dyn FnOnce(
                &mut dyn Any,
                Box<dyn Any>,
                &mut MutableAppContext,
                usize,
                Rc<executor::Foreground>,
            ),
        >,
    },
    OnModelFromStream {
        model_id: usize,
        callback: Box<dyn FnMut(&mut dyn Any, Box<dyn Any>, &mut MutableAppContext, usize) -> bool>,
    },
    OnViewFromFuture {
        window_id: usize,
        view_id: usize,
        callback: Box<
            dyn FnOnce(
                &mut dyn AnyView,
                Box<dyn Any>,
                &mut MutableAppContext,
                usize,
                usize,
                Rc<executor::Foreground>,
            ),
        >,
    },
    OnViewFromStream {
        window_id: usize,
        view_id: usize,
        callback: Box<
            dyn FnMut(&mut dyn AnyView, Box<dyn Any>, &mut MutableAppContext, usize, usize) -> bool,
        >,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::*;

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

        let mut app = App::new().unwrap();
        let app = &mut app;

        let handle_1 = app.add_model(|ctx| Model::new(None, ctx));
        let handle_2 = app.add_model(|ctx| Model::new(Some(handle_1.clone()), ctx));
        assert_eq!(app.0.borrow().ctx.models.len(), 2);

        handle_1.update(app, |model, ctx| {
            model.events.push("updated".into());
            ctx.emit(1);
            ctx.notify();
            ctx.emit(2);
        });
        handle_1.read(app, |model, _| {
            assert_eq!(model.events, vec!["updated".to_string()]);
        });
        handle_2.read(app, |model, _| {
            assert_eq!(
                model.events,
                vec![
                    "observed event 1".to_string(),
                    "notified".to_string(),
                    "observed event 2".to_string(),
                ]
            );
        });

        handle_2.update(app, |model, _| {
            drop(handle_1);
            model.other.take();
        });

        let app_state = app.0.borrow();
        assert_eq!(app_state.ctx.models.len(), 1);
        assert!(app_state.subscriptions.is_empty());
        assert!(app_state.observations.is_empty());
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

        let mut app = App::new().unwrap();
        let app = &mut app;

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
        handle_1.read(app, |model, _| assert_eq!(model.events, vec![7]));

        handle_2.update(app, |_, c| c.emit(5));
        handle_1.read(app, |model, _| assert_eq!(model.events, vec![7, 10, 5]));
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

        let mut app = App::new().unwrap();

        let app = &mut app;
        let handle_1 = app.add_model(|_| Model::default());
        let handle_2 = app.add_model(|_| Model::default());
        let handle_2b = handle_2.clone();

        handle_1.update(app, |_, c| {
            c.observe(&handle_2, move |model, observed, c| {
                model.events.push(observed.as_ref(c).count);
                c.observe(&handle_2b, |model, observed, c| {
                    model.events.push(observed.as_ref(c).count * 2);
                });
            });
        });

        handle_2.update(app, |model, c| {
            model.count = 7;
            c.notify()
        });
        handle_1.read(app, |model, _| assert_eq!(model.events, vec![7]));

        handle_2.update(app, |model, c| {
            model.count = 5;
            c.notify()
        });
        handle_1.read(app, |model, _| assert_eq!(model.events, vec![7, 10, 5]))
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

        App::test(|mut app| async move {
            let handle = app.add_model(|_| Model::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn_local(async { 7 }, |model, output, _| {
                        model.count = output;
                    })
                })
                .await;
            handle.read(&app, |model, _| assert_eq!(model.count, 7));

            handle
                .update(&mut app, |_, c| {
                    c.spawn(async { 14 }, |model, output, _| {
                        model.count = output;
                    })
                })
                .await;
            handle.read(&app, |model, _| assert_eq!(model.count, 14));
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

        App::test(|mut app| async move {
            let handle = app.add_model(|_| Model::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn_stream_local(smol::stream::iter(vec![1, 2, 3]), |model, output, _| {
                        model.events.push(output);
                    })
                })
                .await;

            handle.read(&app, |model, _| {
                assert_eq!(model.events, [Some(1), Some(2), Some(3), None])
            });
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();
        let app = &mut app;

        let (window_id, _) = app.add_window(|ctx| View::new(None, ctx));
        let handle_1 = app.add_view(window_id, |ctx| View::new(None, ctx));
        let handle_2 = app.add_view(window_id, |ctx| View::new(Some(handle_1.clone()), ctx));
        assert_eq!(app.0.borrow().ctx.windows[&window_id].views.len(), 3);

        handle_1.update(app, |view, ctx| {
            view.events.push("updated".into());
            ctx.emit(1);
            ctx.emit(2);
        });
        handle_1.read(app, |view, _| {
            assert_eq!(view.events, vec!["updated".to_string()]);
        });
        handle_2.read(app, |view, _| {
            assert_eq!(
                view.events,
                vec![
                    "observed event 1".to_string(),
                    "observed event 2".to_string(),
                ]
            );
        });

        handle_2.update(app, |view, _| {
            drop(handle_1);
            view.other.take();
        });

        let app_state = app.0.borrow();
        assert_eq!(app_state.ctx.windows[&window_id].views.len(), 2);
        assert!(app_state.subscriptions.is_empty());
        assert!(app_state.observations.is_empty());
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();
        let app = &mut app;

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
        handle_1.read(app, |view, _| assert_eq!(view.events, vec![7]));

        handle_2.update(app, |_, c| c.emit(5));
        handle_1.read(app, |view, _| assert_eq!(view.events, vec![7, 10, 5]));

        handle_3.update(app, |_, c| c.emit(9));
        handle_1.read(app, |view, _| assert_eq!(view.events, vec![7, 10, 5, 9]));
    }

    #[test]
    fn test_dropping_subscribers() {
        struct View;

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();
        let app = &mut app;

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

        app.update(|_| {
            drop(observing_view);
            drop(observing_model);
        });

        emitting_view.update(app, |_, ctx| ctx.emit(()));
        observed_model.update(app, |_, ctx| ctx.emit(()));
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();
        let app = &mut app;
        let (_, view) = app.add_window(|_| View::default());
        let model = app.add_model(|_| Model::default());

        view.update(app, |_, c| {
            c.observe(&model, |me, observed, c| {
                me.events.push(observed.as_ref(c).count)
            });
        });

        model.update(app, |model, c| {
            model.count = 11;
            c.notify();
        });
        view.read(app, |view, _| assert_eq!(view.events, vec![11]));
    }

    #[test]
    fn test_dropping_observers() {
        struct View;

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();
        let app = &mut app;

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

        app.update(|_| {
            drop(observing_view);
            drop(observing_model);
        });

        observed_model.update(app, |_, ctx| ctx.notify());
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();
        let app = &mut app;
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

        view_1.read(app, |view_1, _| {
            assert_eq!(
                view_1.events,
                [
                    "self focused".to_string(),
                    "self blurred".to_string(),
                    "view 2 focused".to_string(),
                    "self focused".to_string(),
                    "view 2 blurred".to_string(),
                ],
            );
        });
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        App::test(|mut app| async move {
            let (_, handle) = app.add_window(|_| View::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn_local(async { 7 }, |me, output, _| {
                        me.count = output;
                    })
                })
                .await;
            handle.read(&app, |view, _| assert_eq!(view.count, 7));
            handle
                .update(&mut app, |_, c| {
                    c.spawn(async { 14 }, |me, output, _| {
                        me.count = output;
                    })
                })
                .await;
            handle.read(&app, |view, _| assert_eq!(view.count, 14));
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        App::test(|mut app| async move {
            let (_, handle) = app.add_window(|_| View::default());
            handle
                .update(&mut app, |_, c| {
                    c.spawn_stream_local(smol::stream::iter(vec![1, 2, 3]), |me, output, _| {
                        me.events.push(output);
                    })
                })
                .await;

            handle.read(&app, |view, _| {
                assert_eq!(view.events, [Some(1), Some(2), Some(3), None])
            });
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
                Empty::new().boxed()
            }

            fn ui_name() -> &'static str {
                "View"
            }
        }

        struct ActionArg {
            foo: String,
        }

        let mut app = App::new().unwrap();
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
    }

    #[test]
    fn test_dispatch_keystroke() -> Result<()> {
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
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        let mut app = App::new().unwrap();

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
            &Keystroke::parse("a")?,
        )?;

        assert!(handled_action.get());
        Ok(())
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
    //         fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
    //             Empty::new().boxed()
    //         }

    //         fn ui_name() -> &'static str {
    //             "View"
    //         }
    //     }

    //     App::test(|mut app| async move {
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
    //         view_1.update(&mut app, |view, ctx| {
    //             view.count = 7;
    //             ctx.notify();
    //             drop(view_2);
    //         });

    //         let invalidation = window_invalidations.borrow_mut().drain(..).next().unwrap();
    //         assert_eq!(invalidation.updated.len(), 1);
    //         assert!(invalidation.updated.contains(&view_1.id()));
    //         assert_eq!(invalidation.removed, vec![view_2_id]);

    //         let view_3 = view_1.update(&mut app, |_, ctx| ctx.add_view(|_| View { count: 8 }));

    //         let invalidation = window_invalidations.borrow_mut().drain(..).next().unwrap();
    //         assert_eq!(invalidation.updated.len(), 1);
    //         assert!(invalidation.updated.contains(&view_3.id()));
    //         assert!(invalidation.removed.is_empty());

    //         view_3
    //             .update(&mut app, |_, ctx| {
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

    #[test]
    fn test_finish_pending_tasks() {
        struct View;

        impl Entity for View {
            type Event = ();
        }

        impl super::View for View {
            fn render<'a>(&self, _: &AppContext) -> Box<dyn Element> {
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

        App::test(|mut app| async move {
            let model = app.add_model(|_| Model);
            let (_, view) = app.add_window(|_| View);

            model.update(&mut app, |_, ctx| {
                let _ = ctx.spawn(async {}, |_, _, _| {});
                let _ = ctx.spawn_local(async {}, |_, _, _| {});
                let _ = ctx.spawn_stream_local(smol::stream::iter(vec![1, 2, 3]), |_, _, _| {});
            });

            view.update(&mut app, |_, ctx| {
                let _ = ctx.spawn(async {}, |_, _, _| {});
                let _ = ctx.spawn_local(async {}, |_, _, _| {});
                let _ = ctx.spawn_stream_local(smol::stream::iter(vec![1, 2, 3]), |_, _, _| {});
            });

            assert!(!app.0.borrow().task_callbacks.is_empty());
            app.finish_pending_tasks().await;
            assert!(app.0.borrow().task_callbacks.is_empty());
            app.finish_pending_tasks().await; // Don't block if there are no tasks
        });
    }
}
