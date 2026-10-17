use super::*;
use crate::display_map::RowGeometryLease;
use futures::{channel::oneshot, task::AtomicWaker};
use gpui::AnyWindowHandle;
use std::{cell::Cell, future::poll_fn, task::Poll};

#[derive(Clone, Default)]
pub(super) struct CapturedInput {
    pub(super) completion: Option<completions::CapturedCompletionAction>,
    pub(super) edit_prediction: Option<edit_prediction::CapturedEditPredictionAction>,
    pub(super) clipboard: Option<clipboard::CapturedClipboardAction>,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub(super) primary: Option<clipboard::CapturedClipboardAction>,
}

impl CapturedInput {
    pub(super) fn current(cx: &App) -> Option<&Self> {
        cx.try_global::<InputScope>()?.0.as_ref()
    }

    pub(super) fn completion(completion: completions::CapturedCompletionAction) -> Self {
        Self {
            completion: Some(completion),
            ..Self::default()
        }
    }

    pub(super) fn edit_prediction(
        edit_prediction: edit_prediction::CapturedEditPredictionAction,
    ) -> Self {
        Self {
            edit_prediction: Some(edit_prediction),
            ..Self::default()
        }
    }

    pub(super) fn run<C: std::borrow::BorrowMut<App>, R>(
        mut self,
        cx: &mut C,
        handler: impl FnOnce(&mut C) -> R,
    ) -> R {
        if let Some(current) = Self::current(cx.borrow_mut()) {
            self.completion = self.completion.or_else(|| current.completion.clone());
            self.edit_prediction = self
                .edit_prediction
                .or_else(|| current.edit_prediction.clone());
            self.clipboard = self.clipboard.or_else(|| current.clipboard.clone());
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            {
                self.primary = self.primary.or_else(|| current.primary.clone());
            }
        }
        Self::run_optional(Some(self), cx, handler)
    }

    fn run_optional<C: std::borrow::BorrowMut<App>, R>(
        input: Option<Self>,
        cx: &mut C,
        handler: impl FnOnce(&mut C) -> R,
    ) -> R {
        let previous = mem::replace(&mut cx.borrow_mut().default_global::<InputScope>().0, input);
        let result = handler(cx);
        cx.borrow_mut().default_global::<InputScope>().0 = previous;
        result
    }
}

#[derive(Default)]
struct InputScope(Option<CapturedInput>);

impl Global for InputScope {}

pub(super) type InputContinuation = Box<dyn FnOnce(&mut Editor, &mut Window, &mut Context<Editor>)>;

pub(super) struct DeferredInput {
    pub(super) transactions: HashMap<TransactionId, TransactionId>,
    shared: InputQueue,
    owner: Rc<InputOwner>,
}

#[derive(Clone, Default)]
struct InputQueue {
    state: Rc<RefCell<InputQueueState>>,
    wake: Arc<AtomicWaker>,
}

impl Global for InputQueue {}

#[derive(Default)]
struct InputQueueState {
    queue: VecDeque<QueuedInput>,
    task: Option<Task<()>>,
    active: Option<Rc<InputOwner>>,
    executing: bool,
    insertion_index: usize,
    after_effects: Vec<QueuedInput>,
    geometry: Vec<(EntityId, RowGeometryLease)>,
    window_closed: Option<Subscription>,
}

struct InputOwner {
    editor: WeakEntity<Editor>,
    window: AnyWindowHandle,
    pending: Cell<usize>,
    publications: Cell<usize>,
    cancelled: Cell<bool>,
    completions: RefCell<Vec<oneshot::Sender<Result<()>>>>,
}

struct QueuedInput {
    owner: Rc<InputOwner>,
    publication: bool,
    operation: DeferredInputOperation,
}

struct DeferredInputOperation {
    geometry: Vec<(EntityId, RowGeometryLease)>,
    pending: Option<PendingRowGeometry>,
    input: Option<CapturedInput>,
    continuation: DeferredInputContinuation,
}

type DeferredInputContinuation = Box<dyn FnOnce(&mut Window, &mut App)>;

impl DeferredInput {
    pub(super) fn new(window: &Window, cx: &mut Context<Editor>) -> Self {
        cx.default_global::<clipboard::ClipboardChannels>();
        let shared = cx.default_global::<InputQueue>().clone();
        if shared.state.borrow().window_closed.is_none() {
            let queue = Rc::downgrade(&shared.state);
            let wake = shared.wake.clone();
            let subscription = cx.on_window_closed(move |cx, window_id| {
                let Some(state) = queue.upgrade() else { return };
                let queue = InputQueue {
                    state,
                    wake: wake.clone(),
                };
                let owners = {
                    let state = queue.state.borrow();
                    state
                        .queue
                        .iter()
                        .chain(state.after_effects.iter())
                        .map(|entry| entry.owner.clone())
                        .chain(state.active.iter().cloned())
                        .filter(|owner| owner.window.window_id() == window_id)
                        .collect::<Vec<_>>()
                };
                for owner in owners {
                    if owner.cancelled.get() {
                        continue;
                    }
                    queue.cancel(
                        &owner,
                        "editor window closed before deferred input completed",
                    );
                    owner
                        .editor
                        .update(cx, |editor, _| {
                            if Rc::ptr_eq(&editor.deferred_input.owner, &owner) {
                                editor.clear_finished_input();
                            }
                        })
                        .ok();
                }
            });
            shared.state.borrow_mut().window_closed = Some(subscription);
        }
        cx.on_release(|editor, _| {
            editor.deferred_input.shared.cancel(
                &editor.deferred_input.owner,
                "editor closed before deferred input completed",
            );
        })
        .detach();
        Self {
            transactions: HashMap::default(),
            shared,
            owner: Rc::new(InputOwner {
                editor: cx.weak_entity(),
                window: window.window_handle(),
                pending: Cell::new(0),
                publications: Cell::new(0),
                cancelled: Cell::new(false),
                completions: RefCell::new(Vec::new()),
            }),
        }
    }
}

impl InputQueue {
    fn cancel(&self, owner: &Rc<InputOwner>, error: &str) {
        if owner.cancelled.replace(true)
            || (owner.pending.get() == 0 && owner.publications.get() == 0)
        {
            return;
        }
        let removed = {
            let mut state = self.state.borrow_mut();
            let mut removed = Vec::new();
            let mut retained = VecDeque::new();
            let mut insertion_index = 0;
            let previous_insertion_index = state.insertion_index;
            for (index, entry) in state.queue.drain(..).enumerate() {
                if Rc::ptr_eq(&entry.owner, owner) {
                    removed.push(entry);
                } else {
                    retained.push_back(entry);
                    insertion_index += usize::from(index < previous_insertion_index);
                }
            }
            state.queue = retained;
            state.insertion_index = insertion_index;
            let after_effects = mem::take(&mut state.after_effects);
            for entry in after_effects {
                if Rc::ptr_eq(&entry.owner, owner) {
                    removed.push(entry);
                } else {
                    state.after_effects.push(entry);
                }
            }
            removed
        };
        owner.pending.set(0);
        owner.publications.set(0);
        for completion in owner.completions.borrow_mut().drain(..) {
            completion.send(Err(anyhow!(error.to_owned()))).ok();
        }
        drop(removed);
        self.wake.wake();
    }

    fn enqueue(&self, mut entry: QueuedInput, after_effects: bool, fresh: bool) {
        if entry.owner.cancelled.get() {
            return;
        }
        let count = if entry.publication {
            &entry.owner.publications
        } else {
            &entry.owner.pending
        };
        count.set(count.get() + 1);
        let mut state = self.state.borrow_mut();
        if state.executing && !fresh {
            if !entry.publication
                && entry.operation.geometry.is_empty()
                && state
                    .active
                    .as_ref()
                    .is_some_and(|owner| Rc::ptr_eq(owner, &entry.owner))
            {
                entry.operation.geometry = state.geometry.clone();
            }
            if after_effects {
                state.after_effects.push(entry);
            } else {
                let index = state.insertion_index;
                state.queue.insert(index, entry);
                state.insertion_index += 1;
            }
        } else {
            state.queue.push_back(entry);
        }
    }

    fn start(&self, cx: &mut App) {
        if self.state.borrow().task.is_some() {
            return;
        }
        let state = Rc::downgrade(&self.state);
        let wake = self.wake.clone();
        let task = cx.spawn(async move |cx| {
            loop {
                let Some(queue_state) = state.upgrade() else {
                    return;
                };
                let entry = {
                    let mut state = queue_state.borrow_mut();
                    let entry = state.queue.pop_front();
                    state.active = entry.as_ref().map(|entry| entry.owner.clone());
                    if entry.is_none() {
                        state.task.take();
                    }
                    entry
                };
                drop(queue_state);
                let Some(entry) = entry else { return };
                let owner = entry.owner;
                let DeferredInputOperation {
                    geometry,
                    pending,
                    input,
                    continuation,
                } = entry.operation;
                cx.update(|_| {});
                if !owner.cancelled.get()
                    && let Some(pending) = &pending
                {
                    futures_lite::future::race(
                        pending.wait(),
                        poll_fn(|cx| {
                            wake.register(cx.waker());
                            if owner.cancelled.get() {
                                Poll::Ready(())
                            } else {
                                Poll::Pending
                            }
                        }),
                    )
                    .await;
                    cx.update(|_| {});
                }
                let Some(queue_state) = state.upgrade() else {
                    return;
                };
                let queue = InputQueue {
                    state: queue_state,
                    wake: wake.clone(),
                };
                if !owner.cancelled.get() {
                    let (previous, result) = cx.update(|cx| {
                        {
                            let mut state = queue.state.borrow_mut();
                            state.executing = true;
                            state.insertion_index = 0;
                            state.geometry = geometry;
                        }
                        let previous =
                            mem::replace(&mut cx.default_global::<InputScope>().0, input);
                        let result = owner
                            .window
                            .update(cx, |_, window, cx| continuation(window, cx));
                        (previous, result)
                    });
                    cx.update(|cx| {
                        cx.default_global::<InputScope>().0 = previous;
                        let mut state = queue.state.borrow_mut();
                        let after_effects = mem::take(&mut state.after_effects);
                        for entry in after_effects {
                            let index = state.insertion_index;
                            state.queue.insert(index, entry);
                            state.insertion_index += 1;
                        }
                        state.executing = false;
                        state.insertion_index = 0;
                        state.geometry.clear();
                        drop(state);
                        if result.is_err() {
                            queue.cancel(
                                &owner,
                                "editor window closed before deferred input completed",
                            );
                        }
                    });
                }
                queue.state.borrow_mut().active = None;
                if !owner.cancelled.get() {
                    let count = if entry.publication {
                        &owner.publications
                    } else {
                        &owner.pending
                    };
                    count.set(count.get() - 1);
                    cx.update(|cx| {
                        if owner.pending.get() == 0 {
                            owner
                                .editor
                                .update(cx, |editor, _| editor.clear_finished_input())
                                .ok();
                        }
                        if owner.pending.get() == 0 && owner.publications.get() == 0 {
                            for completion in owner.completions.borrow_mut().drain(..) {
                                completion.send(Ok(())).ok();
                            }
                        }
                    });
                }
                drop(queue);
                futures_lite::future::yield_now().await;
            }
        });
        self.state.borrow_mut().task = Some(task);
    }
}

impl Editor {
    pub fn defer_keystroke_input(
        &mut self,
        continuation: impl FnOnce(&mut Window, &mut App) -> Box<dyn FnOnce(&mut Window, &mut App)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let was_pending = self.input_is_deferred();
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        self.capture_input(None, cx).run(cx, |cx| {
            let continuation = continuation(window, cx);
            self.enqueue_deferred_input(DeferredInputOperation {
                geometry: Vec::new(),
                pending: None,
                input: CapturedInput::current(cx).cloned(),
                continuation,
            });
        });
        self.deferred_input.shared.start(cx);
        true
    }

    pub fn defer_geometry(
        &mut self,
        pending: PendingRowGeometry,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.defer_external_geometry(
            pending,
            self.editor_input_continuation(continuation),
            window,
            cx,
        );
    }

    pub fn defer_external_geometry(
        &mut self,
        pending: PendingRowGeometry,
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.bind_input_window(window);
        let mut input = Some(self.capture_native_channels(cx));
        let continuation = match window.defer_current_action_dispatch(cx, continuation) {
            Ok(continuation) => {
                input = Some(self.capture_input(None, cx));
                continuation
            }
            Err(continuation) => Box::new(continuation),
        };
        let mut geometry = {
            let state = self.deferred_input.shared.state.borrow();
            if state.executing
                && state
                    .active
                    .as_ref()
                    .is_some_and(|owner| Rc::ptr_eq(owner, &self.deferred_input.owner))
            {
                state.geometry.clone()
            } else {
                Vec::new()
            }
        };
        let mut requested_source = None;
        for source in self.geometry_sources(cx) {
            if !geometry.iter().any(|(id, _)| *id == source.entity_id()) {
                geometry.push((source.entity_id(), source.read(cx).retain_row_geometry()));
            }
            if source.entity_id() == pending.display_map_id() {
                requested_source = Some(source);
            }
        }
        let requested = requested_source.is_some();
        if let Some(source) = requested_source {
            source.update(cx, |map, cx| map.request_row_geometry(pending.clone(), cx));
        }
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry,
            pending: requested.then_some(pending),
            input,
            continuation,
        });
        self.deferred_input.shared.start(cx);
    }

    pub fn defer_external_input(
        &mut self,
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
        cx: &App,
    ) -> bool {
        if !self.input_is_deferred() {
            return false;
        }
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            pending: None,
            input: CapturedInput::current(cx).cloned(),
            continuation: Box::new(continuation),
        });
        true
    }

    pub fn defer_external_input_after_pending(
        &mut self,
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
    ) -> bool {
        if self.deferred_input.owner.pending.get() == 0 {
            return false;
        }
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                publication: false,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    pending: None,
                    input: None,
                    continuation: Box::new(continuation),
                },
            },
            false,
            true,
        );
        true
    }

    pub fn defer_clipboard_input(
        &mut self,
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let was_pending = self.clipboard_input_is_deferred();
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        let continuation = match window.defer_current_action_dispatch(cx, continuation) {
            Ok(continuation) => continuation,
            Err(continuation) => Box::new(continuation),
        };
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            pending: None,
            input: Some(self.capture_native_channels(cx)),
            continuation,
        });
        self.deferred_input.shared.start(cx);
        true
    }

    pub fn schedule_input_continuation(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
    ) {
        self.bind_input_window(window);
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                publication: false,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    pending: None,
                    input: Some(self.capture_native_channels(cx)),
                    continuation: Box::new(continuation),
                },
            },
            true,
            false,
        );
        self.deferred_input.shared.start(cx);
    }

    pub fn request_row_geometry(&self, pending: PendingRowGeometry, cx: &mut App) -> bool {
        let Some(source) = self.display_map_for_geometry(&pending, cx) else {
            return false;
        };
        source.update(cx, |map, cx| map.request_row_geometry(pending, cx));
        true
    }

    pub fn defer_input(
        &mut self,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let was_pending = self.input_is_deferred();
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            pending: None,
            input: CapturedInput::current(cx).cloned(),
            continuation: Box::new(self.editor_input_continuation(continuation)),
        });
        self.deferred_input.shared.start(cx);
        true
    }

    pub fn input_completion(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        if self.deferred_input.owner.pending.get() == 0
            && self.deferred_input.owner.publications.get() == 0
        {
            return Task::ready(Ok(()));
        }
        let (sender, receiver) = oneshot::channel();
        {
            let mut completions = self.deferred_input.owner.completions.borrow_mut();
            completions.retain(|completion| !completion.is_canceled());
            completions.push(sender);
        }
        cx.background_spawn(async move {
            receiver
                .await
                .context("editor closed before deferred input completed")?
        })
    }

    pub(super) fn defer_native_input(
        &mut self,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let was_pending = self.input_is_deferred();
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        self.enqueue_deferred_input_preserving_composition(DeferredInputOperation {
            geometry: Vec::new(),
            pending: None,
            input: Some(self.capture_native_channels(cx)),
            continuation: Box::new(self.editor_input_continuation(continuation)),
        });
        self.deferred_input.shared.start(cx);
        true
    }

    pub(super) fn defer_input_task<T: Send + 'static>(
        &mut self,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) -> Task<Result<T>>
        + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<T>> {
        if !self.input_is_deferred() {
            self.bind_input_window(window);
            return continuation(self, window, cx);
        }
        let (sender, receiver) = oneshot::channel();
        self.defer_input(
            move |editor, window, cx| {
                if !sender.is_canceled() {
                    sender.send(continuation(editor, window, cx)).ok();
                }
            },
            window,
            cx,
        );
        cx.background_spawn(async move {
            receiver
                .await
                .context("editor closed before deferred input started")?
                .await
        })
    }

    pub(super) fn input_is_deferred(&self) -> bool {
        let state = self.deferred_input.shared.state.borrow();
        if state.executing {
            state.insertion_index > 0
        } else {
            self.deferred_input.owner.pending.get() > 0
        }
    }

    pub(super) fn input_is_replaying(&self) -> bool {
        self.deferred_input.shared.state.borrow().executing
    }

    pub(super) fn clipboard_input_is_deferred(&self) -> bool {
        let state = self.deferred_input.shared.state.borrow();
        if state.executing {
            state.insertion_index > 0
        } else {
            state.active.is_some() || !state.queue.is_empty()
        }
    }

    pub(super) fn ready_input_plan<T>(
        &mut self,
        plan: Result<T, PendingRowGeometry>,
        retry: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<T> {
        match plan {
            Ok(plan) => Some(plan),
            Err(pending) => {
                self.defer_geometry(pending, retry, window, cx);
                None
            }
        }
    }

    pub(super) fn defer_clipboard_action<A: Clone + 'static>(
        &mut self,
        action: &A,
        apply: fn(&mut Editor, &A, &mut Window, &mut Context<Editor>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let action = action.clone();
        let editor = cx.weak_entity();
        self.defer_clipboard_input(
            move |window, cx| {
                editor
                    .update(cx, |editor, cx| apply(editor, &action, window, cx))
                    .ok();
            },
            window,
            cx,
        )
    }

    pub(super) fn defer_clipboard_publication(
        &mut self,
        input: CapturedInput,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                publication: true,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    pending: None,
                    input: Some(input),
                    continuation: Box::new(self.editor_input_continuation(continuation)),
                },
            },
            false,
            false,
        );
        self.deferred_input.shared.start(cx);
    }

    pub(super) fn defer_action<A: Clone + 'static>(
        &mut self,
        action: &A,
        apply: fn(&mut Editor, &A, &mut Window, &mut Context<Editor>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.input_is_deferred() {
            self.bind_input_window(window);
            return false;
        }
        let input = self.capture_input(Some(action), cx);
        let action = action.clone();
        input.run(cx, |cx| {
            self.defer_input(
                move |editor, window, cx| apply(editor, &action, window, cx),
                window,
                cx,
            )
        })
    }

    pub(super) fn defer_action_dispatch<A: Action>(
        &mut self,
        action: &A,
        window: &mut Window,
        cx: &mut Context<Self>,
        handler: impl FnOnce(&A, &mut Window, &mut App) + 'static,
    ) -> bool {
        let was_pending = self.input_is_deferred();
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        let Some(continuation) =
            window.defer_action_dispatch(action, cx, move |action, window, cx| {
                if let Some(action) = action.as_any().downcast_ref::<A>() {
                    handler(action, window, cx);
                } else {
                    cx.propagate();
                }
            })
        else {
            return false;
        };
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            pending: None,
            input: Some(self.capture_input(Some(action), cx)),
            continuation,
        });
        self.deferred_input.shared.start(cx);
        true
    }

    pub(super) fn bind_input_window(&mut self, window: &Window) {
        if self.deferred_input.owner.window != window.window_handle() {
            self.deferred_input.shared.cancel(
                &self.deferred_input.owner,
                "editor moved to another window before deferred input completed",
            );
            self.deferred_input.owner = Rc::new(InputOwner {
                editor: self.deferred_input.owner.editor.clone(),
                window: window.window_handle(),
                pending: Cell::new(0),
                publications: Cell::new(0),
                cancelled: Cell::new(false),
                completions: RefCell::new(Vec::new()),
            });
            self.clear_finished_input();
        }
    }

    fn capture_native_channels(&self, cx: &App) -> CapturedInput {
        CapturedInput {
            clipboard: Some(self.capture_clipboard(cx)),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            primary: Some(self.capture_primary(cx)),
            ..CapturedInput::current(cx).cloned().unwrap_or_default()
        }
    }

    fn capture_input(&self, action: Option<&dyn Any>, cx: &Context<Self>) -> CapturedInput {
        CapturedInput {
            completion: Some(
                action
                    .and_then(|action| self.capture_completion_action(action, cx))
                    .unwrap_or_else(|| self.capture_completion_target(None, cx)),
            ),
            edit_prediction: Some(self.capture_edit_prediction_target(cx)),
            clipboard: Some(self.capture_clipboard(cx)),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            primary: Some(self.capture_primary(cx)),
        }
    }

    fn editor_input_continuation<F>(
        &self,
        continuation: F,
    ) -> impl FnOnce(&mut Window, &mut App) + 'static + use<F>
    where
        F: FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
    {
        let editor = self.deferred_input.owner.editor.clone();
        move |window, cx| {
            editor
                .update(cx, |editor, cx| continuation(editor, window, cx))
                .ok();
        }
    }

    fn clear_finished_input(&mut self) {
        self.queued_ime_composition = None;
        self.deferred_input.transactions.clear();
    }

    fn display_map_for_geometry(
        &self,
        pending: &PendingRowGeometry,
        cx: &App,
    ) -> Option<Entity<DisplayMap>> {
        self.geometry_sources(cx)
            .into_iter()
            .find(|source| source.entity_id() == pending.display_map_id())
    }

    fn geometry_sources(&self, cx: &App) -> Vec<Entity<DisplayMap>> {
        let mut sources = vec![self.display_map.clone()];
        sources.extend(self.placeholder_display_map.clone());
        let mut index = 0;
        while let Some(source) = sources.get(index) {
            if let Some((companion, _)) = &source.read(cx).companion
                && let Some(companion) = companion.upgrade()
                && !sources.contains(&companion)
            {
                sources.push(companion);
            }
            index += 1;
        }
        sources
    }

    fn enqueue_deferred_input(&mut self, operation: DeferredInputOperation) {
        if !self.input_is_replaying() {
            self.queued_ime_composition = None;
        }
        self.enqueue_deferred_input_preserving_composition(operation);
    }

    fn enqueue_deferred_input_preserving_composition(&mut self, operation: DeferredInputOperation) {
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                publication: false,
                operation,
            },
            false,
            false,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor_tests::init_test,
        inlays::Inlay,
        test::{build_editor, editor_test_context::EditorTestContext},
    };
    use gpui::{
        Empty, Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, PlatformInput, TestAppContext,
        rgba,
    };
    use language::{LanguageConfig, PLAIN_TEXT, language_settings::SoftWrap};

    gpui::actions!(deferred_input_test, [InsertQueuedAction]);

    #[derive(Clone, PartialEq, Action)]
    #[action(no_json, no_register)]
    struct QueuedSelectionAction(Arc<str>);

    #[gpui::test]
    async fn test_shared_fifo_moved_editor_cancels_only_old_input(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for ingress in [
            "paste_item",
            "mouse",
            "typed",
            "native",
            "marked",
            "helper",
            "native_helper",
            "task",
            "action",
            "action_dispatch",
            "registered_action",
            "keystroke",
            "clipboard",
            "query",
        ] {
            let mut source = EditorTestContext::new(cx).await;
            source.set_state("Aˇ");
            let destination = AnyWindowHandle::from(cx.add_window(|_, _| Empty));
            let _subscription = (ingress == "registered_action").then(|| {
                source.update_editor(|editor, _, cx| {
                    let editor_handle = cx.weak_entity();
                    editor.register_action(move |_: &InsertQueuedAction, window, cx| {
                        editor_handle
                            .update(cx, |editor, cx| {
                                editor.handle_input("x", window, cx);
                            })
                            .expect("editor");
                    })
                })
            });
            let old_input = source.update_editor(|editor, window, cx| {
                editor.schedule_input_continuation(window, cx, |_, _| {
                    panic!("old-window input executed")
                });
                editor.replace_and_mark_text_in_range(None, "old", None, window, cx);
                editor.input_completion(cx)
            });
            source.update(|window, cx| {
                window.replace_root(cx, |_, _| Empty);
            });
            let mut input_task = None;
            let new_input = destination
                .update(cx, |_, window, cx| {
                    window.replace_root(cx, |_, _| MovedEditor(source.editor.clone()));
                    if ingress == "mouse" || ingress == "registered_action" {
                        window.draw(cx).clear(cx);
                    }
                    if ingress == "registered_action" {
                        source.editor.read(cx).focus_handle.clone().dispatch_action(
                            &InsertQueuedAction,
                            window,
                            cx,
                        );
                        assert_eq!(source.editor.read(cx).text(cx), "A");
                    }
                    if ingress == "mouse" {
                        let position_map = source
                            .editor
                            .read(cx)
                            .last_position_map
                            .clone()
                            .expect("ready frame");
                        let position = position_map.text_hitbox.origin
                            + point(
                                position_map.em_layout_width * 0.1,
                                position_map.line_height * 0.5,
                            );
                        window.simulate_mouse_move(position, cx);
                        window.dispatch_event(
                            PlatformInput::MouseDown(MouseDownEvent {
                                position,
                                modifiers: Modifiers::none(),
                                button: MouseButton::Left,
                                click_count: 1,
                                first_mouse: false,
                            }),
                            cx,
                        );
                        window.dispatch_event(
                            PlatformInput::MouseUp(MouseUpEvent {
                                position,
                                modifiers: Modifiers::none(),
                                button: MouseButton::Left,
                                click_count: 1,
                            }),
                            cx,
                        );
                    }
                    source.editor.update(cx, |editor, cx| {
                        match ingress {
                            "paste_item" => editor.paste_item(
                                &ClipboardItem::new_string(String::from("x")),
                                window,
                                cx,
                            ),
                            "native" => editor.replace_text_in_range(None, "x", window, cx),
                            "marked" => {
                                editor.replace_and_mark_text_in_range(None, "x", None, window, cx)
                            }
                            "helper" => {
                                assert!(editor.defer_input(
                                    |editor, window, cx| editor.handle_input("x", window, cx),
                                    window,
                                    cx,
                                ));
                            }
                            "native_helper" => {
                                assert!(editor.defer_native_input(
                                    |editor, window, cx| {
                                        editor.replace_text_in_range(None, "x", window, cx)
                                    },
                                    window,
                                    cx,
                                ));
                            }
                            "task" => {
                                input_task = Some(editor.defer_input_task(
                                    |editor, window, cx| {
                                        editor.handle_input("x", window, cx);
                                        Task::ready(Ok(()))
                                    },
                                    window,
                                    cx,
                                ));
                                assert_eq!(editor.text(cx), "A");
                            }
                            "registered_action" => {}
                            "action" => {
                                assert!(editor.defer_action(
                                    &MoveLeft,
                                    |editor, _, window, cx| editor.handle_input("x", window, cx),
                                    window,
                                    cx,
                                ));
                            }
                            "action_dispatch" => {
                                assert!(!editor.defer_action_dispatch(
                                    &MoveLeft,
                                    window,
                                    cx,
                                    |_, _, _| panic!("ready action was deferred"),
                                ));
                                assert_eq!(editor.deferred_input.owner.window, destination);
                                editor.handle_input("x", window, cx);
                            }
                            "keystroke" => {
                                let editor_handle = cx.weak_entity();
                                assert!(editor.defer_keystroke_input(
                                    |_, _| Box::new(move |window, cx| {
                                        editor_handle
                                            .update(cx, |editor, cx| {
                                                editor.handle_input("x", window, cx);
                                            })
                                            .expect("editor");
                                    }),
                                    window,
                                    cx,
                                ));
                            }
                            "clipboard" => {
                                let editor_handle = cx.weak_entity();
                                assert!(editor.defer_clipboard_input(
                                    move |window, cx| {
                                        editor_handle
                                            .update(cx, |editor, cx| {
                                                editor.paste_item(
                                                    &ClipboardItem::new_string(String::from("x")),
                                                    window,
                                                    cx,
                                                );
                                            })
                                            .expect("editor");
                                    },
                                    window,
                                    cx,
                                ));
                            }
                            "query" => {
                                assert_eq!(
                                    editor
                                        .selected_text_range(true, window, cx)
                                        .expect("selection")
                                        .range,
                                    4..4
                                );
                                let mut adjusted_range = None;
                                assert_eq!(
                                    editor.text_for_range(0..4, &mut adjusted_range, window, cx),
                                    Some(String::from("old"))
                                );
                                assert_eq!(adjusted_range, Some(1..4));
                                assert_eq!(editor.deferred_input.owner.window, source.window);
                                editor.handle_input("x", window, cx);
                            }
                            _ => editor.handle_input("x", window, cx),
                        }
                        editor.input_completion(cx)
                    })
                })
                .expect("destination window");
            source.update(|window, _| window.remove_window());
            cx.run_until_parked();
            assert_eq!(
                old_input
                    .now_or_never()
                    .expect("old input completed")
                    .expect_err("old input cancelled")
                    .to_string(),
                "editor moved to another window before deferred input completed",
                "{ingress}",
            );
            new_input
                .now_or_never()
                .expect("new input completed")
                .expect("new input survived old window close");
            if let Some(input_task) = input_task {
                input_task
                    .now_or_never()
                    .expect("input task completed")
                    .expect("input task succeeded");
            }
            source.editor.read_with(cx, |editor, cx| {
                assert_eq!(
                    editor.text(cx),
                    if ingress == "mouse" { "xA" } else { "Ax" },
                    "{ingress}"
                );
                assert!(editor.queued_ime_composition.is_none());
            });
        }
    }

    #[gpui::test]
    async fn test_input_completion_prunes_cancelled_waiters(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut editor_cx = EditorTestContext::new(cx).await;
        editor_cx.set_state("Aˇ");
        editor_cx.update_editor(|editor, window, cx| {
            editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
        });
        cx.run_until_parked();
        let live = editor_cx.update_editor(|editor, _, cx| {
            let pending = pending_placeholder_geometry(editor, cx);
            editor.enqueue_deferred_input(DeferredInputOperation {
                geometry: Vec::new(),
                pending: Some(pending),
                input: None,
                continuation: Box::new(
                    editor.editor_input_continuation(|_, _, _| panic!("blocked input executed")),
                ),
            });
            editor.deferred_input.shared.start(cx);
            editor.input_completion(cx)
        });
        for _ in 0..8 {
            let cancelled = editor_cx.update_editor(|editor, _, cx| editor.input_completion(cx));
            drop(cancelled);
            cx.run_until_parked();
        }
        let next = editor_cx.update_editor(|editor, _, cx| {
            assert_eq!(editor.deferred_input.owner.completions.borrow().len(), 2);
            let next = editor.input_completion(cx);
            assert_eq!(editor.deferred_input.owner.completions.borrow().len(), 2);
            assert!(
                editor
                    .deferred_input
                    .owner
                    .completions
                    .borrow()
                    .iter()
                    .all(|completion| !completion.is_canceled())
            );
            next
        });
        editor_cx.update(|window, _| window.remove_window());
        cx.run_until_parked();
        for completion in [live, next] {
            assert_eq!(
                completion
                    .now_or_never()
                    .expect("waiter completed")
                    .expect_err("window closed")
                    .to_string(),
                "editor window closed before deferred input completed"
            );
        }
    }

    #[gpui::test]
    async fn test_shared_fifo_preexisting_effect_has_no_capture(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for cancel in [false, true] {
            let mut source = EditorTestContext::new(cx).await;
            let mut target = EditorTestContext::new(cx).await;
            source.set_state("Sˇ");
            target.set_state("Bˇ");
            let source_window = source.window;
            let target_window = target.window;
            let target_handle = target.editor.clone();
            source.update_editor(|editor, window, cx| {
                editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
            });
            cx.run_until_parked();
            let (queue, completion) = source.update_editor(|editor, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                let pending = cancel.then(|| pending_placeholder_geometry(editor, cx));
                editor.enqueue_deferred_input(DeferredInputOperation {
                    geometry: Vec::new(),
                    pending,
                    input: Some(editor.capture_native_channels(cx)),
                    continuation: Box::new(move |_, _| {
                        assert!(!cancel, "cancelled operation executed");
                    }),
                });
                (
                    editor.deferred_input.shared.clone(),
                    editor.input_completion(cx),
                )
            });
            cx.run_until_parked();
            {
                let mut app = cx.app.borrow_mut();
                app.write_to_clipboard(ClipboardItem::new_string(String::from("X")));
                app.defer(move |cx| {
                    if cancel {
                        source_window
                            .update(cx, |_, window, _| window.remove_window())
                            .expect("source window");
                    }
                    target_window
                        .update(cx, |_, window, cx| {
                            target_handle.update(cx, |editor, cx| editor.paste(&Paste, window, cx));
                        })
                        .expect("target window");
                });
                queue.start(&mut app);
            }
            cx.run_until_parked();
            assert_eq!(target.editor(|editor, _, cx| editor.text(cx)), "BX");
            let result = completion.now_or_never().expect("source completed");
            if cancel {
                assert_eq!(
                    result.expect_err("source cancelled").to_string(),
                    "editor window closed before deferred input completed",
                );
            } else {
                result.expect("source succeeded");
            }
        }
    }

    #[gpui::test]
    async fn test_shared_fifo_geometry_stays_with_input_owner(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut source = EditorTestContext::new(cx).await;
        let mut target = EditorTestContext::new(cx).await;
        source.set_state("Sˇ");
        target.set_state("Tˇ");
        for editor_cx in [&mut source, &mut target] {
            editor_cx.update_editor(|editor, window, cx| {
                editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
            });
        }
        cx.run_until_parked();
        let target_handle = target.editor.downgrade();
        let target_window = target.window;
        let completion = source.update_editor(|editor, window, cx| {
            let pending = pending_placeholder_geometry(editor, cx);
            editor.defer_geometry(
                pending,
                move |editor, window, cx| {
                    let source_geometry = vec![
                        editor.display_map.entity_id(),
                        editor
                            .placeholder_display_map
                            .as_ref()
                            .expect("source placeholder")
                            .entity_id(),
                    ];
                    let target_geometry = target_window
                        .update(cx, |_, window, cx| {
                            target_handle
                                .update(cx, |editor, cx| {
                                    let placeholder = editor
                                        .placeholder_display_map
                                        .as_ref()
                                        .expect("target placeholder");
                                    let geometry = vec![
                                        editor.display_map.entity_id(),
                                        placeholder.entity_id(),
                                    ];
                                    let pending = pending_placeholder_geometry(editor, cx);
                                    editor.defer_geometry(pending, |_, _, _| {}, window, cx);
                                    geometry
                                })
                                .expect("target editor")
                        })
                        .expect("target window");
                    editor.publish_clipboard_item(
                        ClipboardItem::new_string(String::from("prepared")),
                        window,
                        cx,
                    );
                    assert!(editor.defer_input(|_, _, _| {}, window, cx));
                    let state = editor.deferred_input.shared.state.borrow();
                    assert_eq!(
                        state
                            .queue
                            .iter()
                            .map(|entry| entry
                                .operation
                                .geometry
                                .iter()
                                .map(|(id, _)| *id)
                                .collect::<Vec<_>>())
                            .collect::<Vec<_>>(),
                        vec![target_geometry, Vec::new(), source_geometry],
                    );
                },
                window,
                cx,
            );
            editor.input_completion(cx)
        });
        cx.run_until_parked();
        completion
            .now_or_never()
            .expect("source completed")
            .expect("source succeeded");
    }

    #[gpui::test]
    async fn test_shared_fifo_select_copy_paste_copy_paste(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut source = EditorTestContext::new(cx).await;
        let mut target = EditorTestContext::new(cx).await;
        let mut idle = EditorTestContext::new(cx).await;
        target.set_state("Bˇ");
        idle.set_state("idleˇ");
        let long_row = "x".repeat(2_000);
        source.update_editor(|editor, window, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(format!("head\n{long_row}\ntail"), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)]);
            });
            editor.select_down(&SelectDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.copy(&Copy, window, cx);
        });
        idle.update_editor(|editor, window, cx| editor.handle_input("!", window, cx));
        assert_eq!(idle.editor(|editor, _, cx| editor.text(cx)), "idle!");
        let target_completion = target.update_editor(|editor, window, cx| {
            editor.paste(&Paste, window, cx);
            editor.select_all(&SelectAll, window, cx);
            editor.copy(&Copy, window, cx);
            editor.input_completion(cx)
        });
        let source_completion = source.update_editor(|editor, window, cx| {
            editor.paste(&Paste, window, cx);
            editor.input_completion(cx)
        });
        cx.run_until_parked();
        source_completion
            .now_or_never()
            .expect("source completed")
            .expect("source succeeded");
        target_completion
            .now_or_never()
            .expect("target completed")
            .expect("target succeeded");
        assert_eq!(target.editor(|editor, _, cx| editor.text(cx)), "Bhead\n");
        assert_eq!(
            source.editor(|editor, _, cx| editor.text(cx)),
            format!("Bhead\n{long_row}\ntail")
        );
    }

    #[gpui::test]
    async fn test_shared_fifo_event_followup_before_next_step(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for handled in [false, true] {
            let mut source = EditorTestContext::new(cx).await;
            let mut target = EditorTestContext::new(cx).await;
            target.set_state("Bˇ");
            let target_handle = target.editor.downgrade();
            let target_window = target.window;
            let events = Rc::new(RefCell::new(Vec::new()));
            let stored = Rc::new(RefCell::new(Vec::new()));
            let _subscription = source.update_editor(|editor, _, cx| {
                editor.set_input_enabled(handled);
                let events = events.clone();
                let stored = stored.clone();
                cx.subscribe_self(move |_, event: &EditorEvent, cx| {
                    if !matches!(
                        event,
                        EditorEvent::InputIgnored { .. } | EditorEvent::InputHandled { .. }
                    ) {
                        return;
                    }
                    assert!(CapturedInput::current(cx).is_some());
                    stored.borrow_mut().push(event.clone());
                    let events = events.clone();
                    target_window
                        .update(cx, |_, window, cx| {
                            target_handle
                                .update(cx, |editor, cx| {
                                    editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
                                    let pending = pending_placeholder_geometry(editor, cx);
                                    editor.defer_geometry(
                                        pending,
                                        move |editor, _, cx| {
                                            events.borrow_mut().push("event");
                                            editor
                                                .write_to_clipboard(
                                                    ClipboardItem::new_string(String::from(
                                                        "event",
                                                    )),
                                                    cx,
                                                )
                                                .expect("event publication");
                                        },
                                        window,
                                        cx,
                                    );
                                })
                                .expect("target editor");
                        })
                        .expect("target window");
                })
            });
            let completion = source.update_editor(|editor, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                let source = cx.weak_entity();
                let events = events.clone();
                editor.schedule_input_continuation(window, cx, move |window, cx| {
                    source
                        .update(cx, |editor, cx| {
                            editor.replay_insert_event("x", None, window, cx);
                            let source = cx.weak_entity();
                            editor.schedule_input_continuation(window, cx, move |_, cx| {
                                assert_eq!(
                                    cx.read_from_clipboard().and_then(|item| item.text()),
                                    Some(String::from("event"))
                                );
                                events.borrow_mut().push("next");
                                source
                                    .update(cx, |editor, cx| {
                                        editor
                                            .write_to_clipboard(
                                                ClipboardItem::new_string(String::from("next")),
                                                cx,
                                            )
                                            .expect("next publication");
                                    })
                                    .expect("source editor");
                            });
                        })
                        .expect("source editor");
                });
                editor.input_completion(cx)
            });
            let paste = target.update_editor(|editor, window, cx| {
                editor.paste(&Paste, window, cx);
                editor.input_completion(cx)
            });
            cx.run_until_parked();
            completion
                .now_or_never()
                .expect("replay completed")
                .expect("replay succeeded");
            paste
                .now_or_never()
                .expect("paste completed")
                .expect("paste succeeded");
            assert_eq!(*events.borrow(), ["event", "next"]);
            assert_eq!(target.editor(|editor, _, cx| editor.text(cx)), "Bnext");
            assert_eq!(stored.borrow().len(), 1);
            source.update_editor(|_, _, cx| assert!(CapturedInput::current(cx).is_none()));
        }
    }

    #[gpui::test]
    async fn test_shared_fifo_cancel_head_keeps_other_editors(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for close_window in [false, true] {
            let mut source = EditorTestContext::new(cx).await;
            let mut target = EditorTestContext::new(cx).await;
            source.set_state("«alphaˇ»");
            target.set_state("Bˇ");
            source.update_editor(|editor, window, cx| {
                editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
            });
            cx.run_until_parked();
            let mut completion = source.update_editor(|editor, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                let pending = pending_placeholder_geometry(editor, cx);
                editor.enqueue_deferred_input(DeferredInputOperation {
                    geometry: Vec::new(),
                    pending: Some(pending),
                    input: Some(editor.capture_native_channels(cx)),
                    continuation: Box::new(
                        editor.editor_input_continuation(|_, _, _| panic!("cancelled input ran")),
                    ),
                });
                editor.deferred_input.shared.start(cx);
                editor.copy(&Copy, window, cx);
                editor.input_completion(cx)
            });
            let paste = target.update_editor(|editor, window, cx| {
                editor.paste(&Paste, window, cx);
                editor.input_completion(cx)
            });
            cx.run_until_parked();
            assert!((&mut completion).now_or_never().is_none());
            if close_window {
                source.update(|window, _| window.remove_window());
            } else {
                source.update(|window, cx| {
                    window.replace_root(cx, |_, _| Empty);
                    window.draw(cx).clear(cx);
                });
            }
            let source_handle = source.editor.downgrade();
            drop(source);
            target.update_editor(|_, _, _| {});
            assert!(source_handle.upgrade().is_none());
            cx.run_until_parked();
            assert_eq!(
                completion
                    .now_or_never()
                    .expect("cancelled input completed")
                    .expect_err("cancelled input failed")
                    .to_string(),
                if close_window {
                    "editor window closed before deferred input completed"
                } else {
                    "editor closed before deferred input completed"
                }
            );
            paste
                .now_or_never()
                .expect("other editor resumed")
                .expect("paste succeeded");
            assert_eq!(target.editor(|editor, _, cx| editor.text(cx)), "BZ");
        }
    }

    #[gpui::test]
    async fn test_shared_fifo_prepared_publication_keeps_input_immediate(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for external in [false, true] {
            let mut source = EditorTestContext::new(cx).await;
            let mut publisher = EditorTestContext::new(cx).await;
            let mut target = EditorTestContext::new(cx).await;
            publisher.set_state("Pˇ");
            target.set_state("Bˇ");
            source.update_editor(|editor, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("Z")));
                editor.schedule_input_continuation(window, cx, |_, _| {});
            });
            let publication = publisher.update_editor(|editor, window, cx| {
                editor.publish_clipboard_item(
                    ClipboardItem::new_string(String::from("prepared")),
                    window,
                    cx,
                );
                assert!(!editor.input_is_deferred());
                editor.handle_input("!", window, cx);
                assert_eq!(editor.text(cx), "P!");
                editor.input_completion(cx)
            });
            let paste = target.update_editor(|editor, window, cx| {
                editor.paste(&Paste, window, cx);
                if external {
                    cx.write_to_clipboard(ClipboardItem::new_string(String::from("external")));
                }
                editor.input_completion(cx)
            });
            cx.run_until_parked();
            publication
                .now_or_never()
                .expect("publication completed")
                .expect("publication succeeded");
            paste
                .now_or_never()
                .expect("paste completed")
                .expect("paste succeeded");
            assert_eq!(target.editor(|editor, _, cx| editor.text(cx)), "Bprepared");
            target.update_editor(|_, _, cx| {
                assert_eq!(
                    cx.read_from_clipboard().and_then(|item| item.text()),
                    Some(String::from(if external { "external" } else { "prepared" }))
                )
            });
        }
    }

    #[gpui::test]
    async fn test_deferred_input_fifo_replans_all_selections(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(format!("ab\ncd\nef\n{}\ngh", "x".repeat(2_000)), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([
                    Point::new(0, 1)..Point::new(0, 1),
                    Point::new(2, 1)..Point::new(2, 1),
                ]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(&editor.display_snapshot(cx))
                    .into_iter()
                    .map(|selection| selection.head())
                    .collect::<Vec<_>>(),
                [Point::new(0, 1), Point::new(2, 1)],
            );
            editor.handle_input("A", window, cx);
            editor.move_down(&MoveDown, window, cx);
            editor.replay_insert_event("B", Some(-1..0), window, cx);
            editor.buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(0)..MultiBufferOffset(0), "q\n")],
                    None,
                    cx,
                );
            });
            editor.input_completion(cx)
        });
        completion.await.expect("deferred input completed");
        cx.assert_editor_state(&format!("q\nab\ncAd\neBˇ\nxA{}\ngBˇ", "x".repeat(1_999)));
        cx.update_editor(|editor, window, cx| {
            editor.move_left(&MoveLeft, window, cx);
            assert!(!editor.input_is_deferred());
            editor
                .input_completion(cx)
                .now_or_never()
                .expect("ready input completes synchronously")
                .expect("input completed");
        });
    }

    #[gpui::test]
    async fn test_deferred_multicursor_geometry_lifetime(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        cx.background_executor.set_block_on_ticks(0..=0);
        let text = format!(
            "{}{}",
            "xx\n".repeat(200),
            format!("xx\n{}\n", "x".repeat(1_025)).repeat(64)
        );
        let sources = (0..64)
            .map(|index| Point::new(200 + index * 2, 1))
            .collect::<Vec<_>>();
        for cancel in [false, true] {
            let window = cx.add_window(|window, cx| {
                let buffer = MultiBuffer::build_simple(&text, cx);
                let mut editor = build_editor(buffer, window, cx);
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                let style = editor.style(cx).clone();
                editor.set_style(style, window, cx);
                editor
            });
            let mut editor_cx = EditorTestContext::for_editor(window, cx).await;
            if cancel {
                editor_cx.update(|window, cx| {
                    window.replace_root(cx, |_, _| Empty);
                });
            }
            let completion = editor_cx.update_editor(|editor, window, cx| {
                if cancel {
                    let position = editor
                        .buffer
                        .read(cx)
                        .snapshot(cx)
                        .anchor_after(Point::new(327, 10));
                    editor.display_map.update(cx, |map, cx| {
                        map.splice_inlays(
                            &[],
                            vec![Inlay::color(0, position, rgba(0xff0000ff))],
                            cx,
                        );
                    });
                }
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges(sources.iter().copied().map(|point| point..point));
                });
                assert_eq!(
                    editor.display_map.read(cx).retained_row_counts(None),
                    (0, 0, 0)
                );
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                editor.input_completion(cx)
            });
            editor_cx.run_until_parked();
            if cancel {
                editor_cx.update_editor(|editor, _, cx| {
                    assert!(editor.input_is_deferred());
                    assert_eq!(
                        editor.display_map.read(cx).retained_row_counts(None),
                        (64, 63, 64)
                    );
                    let pending = editor
                        .display_snapshot(cx)
                        .long_row(DisplayRow(327))
                        .err()
                        .expect("unmeasured final target");
                    assert!(editor.request_row_geometry(pending, cx));
                });
                editor_cx.update(|window, _| window.remove_window());
                assert_eq!(
                    completion
                        .await
                        .expect_err("window closed during retry")
                        .to_string(),
                    "editor window closed before deferred input completed"
                );
            } else {
                completion.await.expect("all cold targets resolved");
            }
            cx.run_until_parked();
            editor_cx.editor.update(cx, |editor, cx| {
                assert_eq!(editor.deferred_input.owner.pending.get(), 0);
                assert!(editor.deferred_input.shared.state.borrow().queue.is_empty());
                assert!(
                    editor
                        .deferred_input
                        .shared
                        .state
                        .borrow()
                        .geometry
                        .is_empty()
                );
                if !cancel {
                    let position_map = editor.last_position_map.as_ref().expect("painted viewport");
                    assert_eq!(
                        editor
                            .display_map
                            .read(cx)
                            .retained_row_counts(Some(&position_map.snapshot.display_snapshot)),
                        (0, 0, 0)
                    );
                }
                assert_eq!(editor.text(cx), text);
                let snapshot = editor.display_snapshot(cx);
                assert_eq!(
                    editor.selections.ranges::<Point>(&snapshot),
                    sources
                        .iter()
                        .map(|source| {
                            let point = Point::new(source.row + u32::from(!cancel), source.column);
                            point..point
                        })
                        .collect::<Vec<_>>()
                );
            });
            if !cancel {
                editor_cx.update_editor(|editor, window, cx| {
                    editor.set_scroll_position(gpui::Point::new(0., 0.), window, cx);
                });
                editor_cx.update(|window, cx| window.draw(cx).clear(cx));
                editor_cx.run_until_parked();
                editor_cx.editor.read_with(cx, |editor, _| {
                    let position_map = editor.last_position_map.as_ref().expect("short viewport");
                    assert_eq!(position_map.visible_row_range.start, DisplayRow(0));
                    assert!(position_map.visible_row_range.end <= DisplayRow(200));
                });
            }
            editor_cx.editor.read_with(cx, |editor, cx| {
                assert_eq!(
                    editor.display_map.read(cx).retained_row_counts(None),
                    (0, 0, 0)
                );
            });
        }
    }

    #[gpui::test]
    async fn test_deferred_geometry_routes_placeholder_and_removed_source(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let events = Rc::new(RefCell::new(Vec::new()));
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_placeholder_text(&"x".repeat(2_000), window, cx);
            let pending = editor
                .placeholder_display_map
                .as_ref()
                .expect("placeholder map")
                .update(cx, |map, cx| {
                    map.snapshot(cx)
                        .long_row(DisplayRow(0))
                        .err()
                        .expect("cold placeholder geometry")
                });
            let events = events.clone();
            editor.defer_geometry(
                pending.clone(),
                move |editor, window, cx| {
                    assert_placeholder_geometry_ready(editor, pending, events, window, cx);
                },
                window,
                cx,
            );
            editor.input_completion(cx)
        });
        completion.await.expect("placeholder geometry completed");
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_placeholder_text(&"y".repeat(2_000), window, cx);
            let pending = editor
                .placeholder_display_map
                .as_ref()
                .expect("placeholder map")
                .update(cx, |map, cx| {
                    map.snapshot(cx)
                        .long_row(DisplayRow(0))
                        .err()
                        .expect("cold replacement placeholder geometry")
                });
            editor.set_placeholder_text("", window, cx);
            let events = events.clone();
            editor.defer_geometry(
                pending,
                move |editor, _, cx| {
                    assert_eq!(editor.placeholder_text(cx), Some(String::new()));
                    events.borrow_mut().push("removed");
                },
                window,
                cx,
            );
            editor.input_completion(cx)
        });
        completion.await.expect("removed geometry source replanned");
        assert_eq!(*events.borrow(), ["placeholder", "removed"]);
    }

    #[gpui::test]
    async fn test_deferred_input_window_close_cancels_waiters(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut editor_cx = EditorTestContext::new(cx).await;
        let text = format!("ab\n{}", "x".repeat(2_000));
        let completions = editor_cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.replace_and_mark_text_in_range(None, "a", None, window, cx);
            [editor.input_completion(cx), editor.input_completion(cx)]
        });
        editor_cx.update(|window, _| window.remove_window());
        editor_cx.editor.read_with(cx, |editor, cx| {
            assert_eq!(editor.deferred_input.owner.pending.get(), 0);
            assert!(editor.deferred_input.shared.state.borrow().queue.is_empty());
            assert!(editor.queued_ime_composition.is_none());
            assert_eq!(editor.text(cx), text);
        });
        for completion in completions {
            assert_eq!(
                completion
                    .await
                    .expect_err("closed window cancels input")
                    .to_string(),
                "editor window closed before deferred input completed"
            );
        }
    }

    #[gpui::test]
    async fn test_deferred_clipboard_commands_preserve_fifo_and_payload(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!("ab\n{}\ncd", "x".repeat(2_000));
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            cx.write_to_clipboard(ClipboardItem::new_string(String::from("old")));
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.cut(&Cut, window, cx);
            editor.kill_ring_cut(&KillRingCut, window, cx);
            editor.kill_ring_yank(&KillRingYank, window, cx);
            editor.paste(&Paste, window, cx);
            editor.paste_item(&ClipboardItem::new_string(String::from("!")), window, cx);
            assert_eq!(editor.text(cx), text);
            assert_eq!(
                cx.read_from_clipboard().and_then(|item| item.text()),
                Some(String::from("old"))
            );
            editor.input_completion(cx)
        });
        completion
            .await
            .expect("deferred clipboard input completed");
        cx.assert_editor_state(&format!("{text}!ˇ"));
        cx.update_editor(|_, _, cx| {
            assert_eq!(
                cx.read_from_clipboard().and_then(|item| item.text()),
                Some(format!("{}\n", "x".repeat(2_000)))
            );
        });
    }

    #[gpui::test]
    async fn test_deferred_registered_action_releases_editor_borrow(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let _subscription = cx.update_editor(|editor, _, cx| {
            let handle = cx.weak_entity();
            let subscription = editor.register_action(move |_: &InsertQueuedAction, window, cx| {
                handle
                    .update(cx, |editor, cx| {
                        assert_eq!(
                            editor
                                .selected_text_range(false, window, cx)
                                .map(|selection| selection.range),
                            Some(4..4)
                        );
                        editor.buffer.update(cx, |buffer, cx| {
                            let end = buffer.len(cx);
                            buffer.edit([(end..end, format!("\n{}", "y".repeat(2_000)))], None, cx);
                        });
                        editor.move_down(&MoveDown, window, cx);
                        assert!(editor.input_is_deferred());
                        editor.replace_text_in_range(None, "A", window, cx);
                    })
                    .ok();
            });
            let events = cx.subscribe_self::<EditorEvent>(|editor, event, cx| {
                if let EditorEvent::InputHandled { text, .. } = event
                    && text.as_ref() == "A"
                {
                    assert!(!editor.defer_external_input(
                        |_, _| { panic!("event followup must remain in the executing input") },
                        cx
                    ));
                }
            });
            cx.notify();
            Subscription::join(subscription, events)
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let editor = cx.editor.clone();
        let completion = cx.update(|window, cx| {
            let focus = editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.set_text(format!("ab\n{}", "x".repeat(2_000)), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                editor.focus_handle.clone()
            });
            focus.dispatch_action(&InsertQueuedAction, window, cx);
            editor.update(cx, |editor, cx| {
                editor.replace_and_mark_text_in_range(None, "B", None, window, cx);
                editor.input_completion(cx)
            })
        });
        completion
            .await
            .expect("deferred registered action completed");
        cx.assert_editor_state(&format!(
            "ab\n{}\nyABˇ{}",
            "x".repeat(2_000),
            "y".repeat(1_999)
        ));
    }

    #[gpui::test]
    async fn test_deferred_rendered_action_preserves_payload_and_propagation(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let events = Rc::new(RefCell::new(Vec::new()));
        let payload = Arc::<str>::from("A");
        let _subscription = cx.update_editor(|editor, _, cx| {
            let events = events.clone();
            let payload = payload.clone();
            let subscription = editor.register_action_renderer(move |editor, window, cx| {
                for name in ["first", "second"] {
                    let editor_handle = cx.weak_entity();
                    let events = events.clone();
                    let payload = payload.clone();
                    editor.register_action_handler(
                        window,
                        cx,
                        move |action: &QueuedSelectionAction, window, cx| {
                            assert!(Arc::ptr_eq(&action.0, &payload));
                            editor_handle
                                .update(cx, |editor, cx| {
                                    events.borrow_mut().push((
                                        name,
                                        editor
                                            .selections
                                            .newest::<Point>(&editor.display_snapshot(cx))
                                            .head(),
                                    ));
                                    if name == "second" {
                                        editor.handle_input(&action.0, window, cx);
                                    }
                                })
                                .expect("registered editor is alive");
                            if name == "first" {
                                cx.propagate();
                            }
                        },
                    );
                }
            });
            cx.notify();
            subscription
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let editor = cx.editor.clone();
        let action = QueuedSelectionAction(payload);
        let completion = cx.update(|window, cx| {
            let focus = editor.update(cx, |editor, cx| {
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.set_text(format!("ab\n{}", "x".repeat(2_000)), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                editor.focus_handle.clone()
            });
            focus.dispatch_action(&action, window, cx);
            assert!(events.borrow().is_empty());
            editor.update(cx, |editor, cx| {
                editor.handle_input("!", window, cx);
                editor.input_completion(cx)
            })
        });
        completion.await.expect("rendered action completed");
        assert_eq!(
            *events.borrow(),
            [("first", Point::new(1, 1)), ("second", Point::new(1, 1))]
        );
        cx.assert_editor_state(&format!("ab\nxA!ˇ{}", "x".repeat(1_999)));
        cx.update(|window, cx| {
            editor
                .read(cx)
                .focus_handle
                .clone()
                .dispatch_action(&action, window, cx);
            assert_eq!(
                *events.borrow(),
                [
                    ("first", Point::new(1, 1)),
                    ("second", Point::new(1, 1)),
                    ("first", Point::new(1, 3)),
                    ("second", Point::new(1, 3)),
                ]
            );
        });
        cx.assert_editor_state(&format!("ab\nxA!Aˇ{}", "x".repeat(1_999)));
    }

    #[gpui::test]
    async fn test_deferred_columnar_selection_reanchors_tail(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(format!("ab\n{}\ncd", "x".repeat(2_000)), window, cx);
            editor.select(
                SelectPhase::BeginColumnar {
                    position: DisplayPoint::new(DisplayRow(0), 2),
                    goal_column: 4,
                    reset: true,
                    mode: ColumnarMode::FromMouse,
                },
                window,
                cx,
            );
            editor.select(
                SelectPhase::Update {
                    position: DisplayPoint::new(DisplayRow(2), 1),
                    goal_column: 1,
                    scroll_delta: gpui::Point::default(),
                },
                window,
                cx,
            );
            assert!(editor.input_is_deferred());
            editor.select(SelectPhase::End, window, cx);
            editor.buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(0)..MultiBufferOffset(0), "prefix\n")],
                    None,
                    cx,
                );
            });
            editor.input_completion(cx)
        });
        completion.await.expect("deferred selection completed");
        cx.update_editor(|editor, _, cx| {
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(&editor.display_snapshot(cx))
                    .into_iter()
                    .map(|selection| (selection.range(), selection.reversed))
                    .collect::<Vec<_>>(),
                [
                    (Point::new(1, 1)..Point::new(1, 2), true),
                    (Point::new(2, 1)..Point::new(2, 4), true),
                    (Point::new(3, 1)..Point::new(3, 2), true),
                ],
            );
        });
    }

    #[gpui::test]
    async fn test_deferred_comment_advance_preserves_nested_redo_selection(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_buffer(|buffer, cx| {
            buffer.set_language(
                Some(Arc::new(Language::new(
                    LanguageConfig {
                        line_comments: vec![Arc::from("# ")],
                        ..LanguageConfig::default()
                    },
                    None,
                ))),
                cx,
            );
        });
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(format!("ab\n{}", "x".repeat(2_000)), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.finalize_last_transaction(cx);
            editor.transact(window, cx, |editor, window, cx| {
                editor.toggle_comments(
                    &ToggleComments {
                        advance_downwards: true,
                        ignore_indent: false,
                        comment_empty_lines: false,
                    },
                    window,
                    cx,
                );
            });
            assert!(editor.input_is_deferred());
            editor.undo(&Undo, window, cx);
            editor.redo(&Redo, window, cx);
            editor.input_completion(cx)
        });
        completion
            .await
            .expect("deferred comment advance completed");
        cx.assert_editor_state(&format!("# ab\nxxxˇ{}", "x".repeat(1_997)));
    }

    #[gpui::test]
    async fn test_deferred_ime_autoclose_skip_records_existing_text(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for replace_closer in [false, true] {
            let mut editor_cx = EditorTestContext::new(cx).await;
            editor_cx.update_buffer(|buffer, cx| {
                buffer.set_language(Some(PLAIN_TEXT.clone()), cx);
            });
            let completion = editor_cx.update_editor(|editor, window, cx| {
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.set_text(format!("ab\n{}", "x".repeat(2_000)), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                editor.move_to_end_of_line(&MoveToEndOfLine::default(), window, cx);
                editor.replace_text_in_range(None, "(", window, cx);
                editor.replace_text_in_range(None, ")", window, cx);
                assert_eq!(
                    editor
                        .selected_text_range(false, window, cx)
                        .map(|selection| selection.range),
                    Some(3..3)
                );
                if replace_closer {
                    editor.replace_text_in_range(Some(2..3), "]", window, cx);
                }
                editor.input_completion(cx)
            });
            completion.await.expect("autoclose input completed");
            let pair = if replace_closer { "(]" } else { "()" };
            editor_cx.assert_editor_state(&format!("ab\n{}{pair}ˇ", "x".repeat(2_000)));
        }
    }

    #[gpui::test]
    async fn test_deferred_ime_surrounding_query_excludes_unresolved_destination(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        for use_adjusted_range in [false, true] {
            let mut editor_cx = EditorTestContext::new(cx).await;
            let text = format!("ab\n{}", "x".repeat(2_000));
            let completion = editor_cx.update_editor(|editor, window, cx| {
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.set_text(text.clone(), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                assert_eq!(editor.text_for_range(0..2, &mut None, window, cx), None);
                editor.replace_text_in_range(None, "a", window, cx);
                let selection = editor
                    .selected_text_range(false, window, cx)
                    .expect("projected selection");
                assert_eq!(selection.range, 2..2);
                let requested = selection.range.start - 2..selection.range.start;
                let mut adjusted_range = None;
                assert_eq!(
                    editor.text_for_range(requested.clone(), &mut adjusted_range, window, cx),
                    Some(String::from("a"))
                );
                assert_eq!(adjusted_range, Some(1..2));
                let replacement = if use_adjusted_range {
                    adjusted_range.expect("known queued text range")
                } else {
                    requested
                };
                editor.replace_text_in_range(Some(replacement), "A", window, cx);
                assert_eq!(
                    editor
                        .selected_text_range(false, window, cx)
                        .map(|selection| selection.range),
                    Some(if use_adjusted_range { 2..2 } else { 1..1 })
                );
                assert_eq!(editor.text(cx), text);
                editor.input_completion(cx)
            });
            completion.await.expect("surrounding replacement completed");
            let expected = if use_adjusted_range {
                format!("ab\nxAˇ{}", "x".repeat(1_999))
            } else {
                format!("ab\nAˇ{}", "x".repeat(1_999))
            };
            editor_cx.assert_editor_state(&expected);
        }
    }

    #[gpui::test]
    async fn test_deferred_ime_explicit_replacement_after_roundtrip_movement(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        for (replacement, selection, expected) in
            [(0..2, 1..1, "Aˇb"), (1..3, 2..2, "aAˇ"), (0..3, 1..1, "Aˇ")]
        {
            let mut editor_cx = EditorTestContext::new(cx).await;
            let text = format!("ab\n{}", "x".repeat(2_000));
            let completion = editor_cx.update_editor(|editor, window, cx| {
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.set_text(text.clone(), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                editor.move_up(&MoveUp, window, cx);
                editor.replace_text_in_range(None, "a", window, cx);
                editor.replace_text_in_range(Some(replacement), "A", window, cx);
                assert_eq!(
                    editor
                        .selected_text_range(false, window, cx)
                        .map(|selection| selection.range),
                    Some(selection)
                );
                assert_eq!(editor.text(cx), text);
                editor.input_completion(cx)
            });
            completion.await.expect("roundtrip replacement completed");
            editor_cx.assert_editor_state(&format!("{expected}\n{}", "x".repeat(2_000)));
        }
    }

    #[gpui::test]
    async fn test_deferred_ime_mixed_replacements_share_replay_origin(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!("ab\n{}", "x".repeat(2_000));
        let destination = "ab\n".len();
        let mut expected = text.clone();
        for (range, replacement) in [(1..1, "a"), (0..1, "Z"), (0..2, "Q")] {
            expected.replace_range(
                destination + range.start..destination + range.end,
                replacement,
            );
        }
        assert_eq!(expected, format!("ab\nQ{}", "x".repeat(1_999)));
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.replace_text_in_range(None, "a", window, cx);
            editor.replace_text_in_range(Some(0..1), "Z", window, cx);
            assert_eq!(
                editor.text_for_range(0..2, &mut None, window, cx),
                Some(String::from("Za"))
            );
            editor.replace_text_in_range(Some(0..2), "Q", window, cx);
            assert_eq!(editor.text(cx), text);
            editor.input_completion(cx)
        });
        completion.await.expect("mixed replacement input completed");
        expected.insert(destination + 1, 'ˇ');
        cx.assert_editor_state(&expected);
    }

    #[gpui::test]
    async fn test_deferred_ime_restarts_origin_after_queued_movement(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!("ab\n{}", "x".repeat(2_000));
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.replace_text_in_range(None, "a", window, cx);
            assert_eq!(
                editor.text_for_range(1..2, &mut None, window, cx),
                Some(String::from("a"))
            );
            editor.move_left(&MoveLeft, window, cx);
            assert!(editor.queued_ime_composition.is_none());
            editor.replace_text_in_range(None, "b", window, cx);
            let mut adjusted_range = None;
            assert_eq!(
                editor.text_for_range(1..3, &mut adjusted_range, window, cx),
                Some(String::from("b"))
            );
            assert_eq!(adjusted_range, Some(1..2));
            editor.replace_text_in_range(Some(1..3), "Q", window, cx);
            assert_eq!(editor.text(cx), text);
            editor.input_completion(cx)
        });
        completion
            .await
            .expect("movement-separated input completed");
        cx.assert_editor_state(&format!("ab\nxQˇ{}", "x".repeat(1_999)));
    }

    #[gpui::test]
    async fn test_deferred_ime_tracks_newest_writable_selection(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        for writable in [true, false] {
            let first = format!("ab\n{}", "x".repeat(2_000));
            let second = format!("cd\n{}", "y".repeat(2_000));
            let buffers =
                [(first.as_str(), writable), (second.as_str(), false)].map(|(text, writable)| {
                    cx.new(|cx| {
                        let mut buffer = Buffer::local(text, cx);
                        if !writable {
                            buffer.set_capability(language::Capability::ReadOnly, cx);
                        }
                        buffer
                    })
                });
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
                for (index, buffer) in buffers.iter().enumerate() {
                    multibuffer.set_excerpts_for_path(
                        PathKey::sorted(index as u64),
                        buffer.clone(),
                        [Point::zero()..buffer.read(cx).max_point()],
                        0,
                        cx,
                    );
                }
                multibuffer
            });
            let window = cx.add_window(|window, cx| {
                let editor = build_editor(multibuffer, window, cx);
                window.focus(&editor.focus_handle(cx), cx);
                editor
            });
            let mut editor_cx = EditorTestContext::for_editor(window, cx).await;
            let completion = editor_cx.update_editor(|editor, window, cx| {
                assert!(!editor.read_only(cx));
                editor.set_soft_wrap_mode(SoftWrap::None, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([
                        Point::new(0, 1)..Point::new(0, 1),
                        Point::new(2, 1)..Point::new(2, 1),
                    ]);
                });
                assert_eq!(
                    editor
                        .selections
                        .newest::<Point>(&editor.display_snapshot(cx))
                        .head(),
                    Point::new(2, 1)
                );
                editor.move_down(&MoveDown, window, cx);
                assert!(editor.input_is_deferred());
                editor.replace_text_in_range(None, "a", window, cx);
                let selection = editor
                    .selected_text_range(false, window, cx)
                    .expect("projected input selection");
                let end = selection.range.end;
                assert_eq!(
                    editor.text_for_range(end - 1..end, &mut None, window, cx),
                    Some(String::from("a"))
                );
                editor.replace_text_in_range(Some(end - 1..end), "Q", window, cx);
                editor.input_completion(cx)
            });
            completion.await.expect("mixed-capability input completed");
            editor_cx.update_editor(|editor, _, cx| assert!(!editor.read_only(cx)));
            let expected_first = if writable {
                format!("ab\nxQ{}", "x".repeat(1_999))
            } else {
                first
            };
            let mut texts = editor_cx.multibuffer(|buffer, cx| {
                buffer
                    .all_buffers()
                    .into_iter()
                    .map(|buffer| buffer.read(cx).text())
                    .collect::<Vec<_>>()
            });
            texts.sort();
            assert_eq!(texts, [expected_first, second]);
            editor_cx.update_editor(|editor, _, cx| {
                let heads = editor
                    .selections
                    .all::<Point>(&editor.display_snapshot(cx))
                    .into_iter()
                    .map(|selection| selection.head())
                    .collect::<Vec<_>>();
                assert_eq!(
                    heads,
                    if writable {
                        vec![Point::new(1, 2)]
                    } else {
                        vec![Point::new(1, 1), Point::new(3, 1)]
                    }
                );
            });
        }
    }

    #[gpui::test]
    async fn test_deferred_manipulation_helpers_preserve_fifo(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!("ab\n{}", "x".repeat(2_000));
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.manipulate_text(window, cx, |text| {
                assert_eq!(text, "x".repeat(2_000));
                text.to_uppercase()
            });
            editor.manipulate_immutable_lines(window, cx, |lines| {
                assert_eq!(lines.as_slice(), ["X".repeat(2_000).as_str()]);
                lines.push("done");
            });
            assert_eq!(editor.text(cx), text);
            editor.input_completion(cx)
        });
        completion
            .await
            .expect("shared manipulation helpers completed");
        cx.assert_editor_state(&format!("ab\n«{}\ndoneˇ»", "X".repeat(2_000)));
    }

    #[gpui::test]
    async fn test_deferred_ime_projects_consecutive_commits(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!("ab\n{}", "x".repeat(2_000));
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text.clone(), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.replace_text_in_range(None, "a", window, cx);
            assert_eq!(
                editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| selection.range),
                Some(2..2)
            );
            editor.replace_text_in_range(None, "b", window, cx);
            assert_eq!(
                editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| selection.range),
                Some(3..3)
            );
            assert_eq!(
                editor.text_for_range(1..3, &mut None, window, cx),
                Some(String::from("ab"))
            );
            editor.replace_text_in_range(Some(1..2), "A", window, cx);
            assert_eq!(
                editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| selection.range),
                Some(2..2)
            );
            assert_eq!(editor.marked_text_range(window, cx), None);
            let mut adjusted_range = None;
            assert_eq!(
                editor.text_for_range(0..4, &mut adjusted_range, window, cx),
                Some(String::from("Ab"))
            );
            assert_eq!(adjusted_range, Some(1..3));
            assert_eq!(editor.text(cx), text);
            editor.input_completion(cx)
        });
        completion.await.expect("projected IME input completed");
        cx.assert_editor_state(&format!("ab\nxAˇb{}", "x".repeat(1_999)));
    }

    #[gpui::test]
    async fn test_deferred_ime_preserves_marked_and_replacement_ranges(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorTestContext::new(cx).await;
        let completion = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(format!("ab\n{}", "x".repeat(2_000)), window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
            });
            editor.move_down(&MoveDown, window, cx);
            assert!(editor.input_is_deferred());
            editor.replace_and_mark_text_in_range(Some(1..1), "é", Some(1..1), window, cx);
            editor.replace_and_mark_text_in_range(Some(0..1), "😀", Some(2..2), window, cx);
            assert_eq!(editor.marked_text_range(window, cx), Some(1..3));
            assert_eq!(
                editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| selection.range),
                Some(3..3),
            );
            let mut adjusted_range = None;
            assert_eq!(
                editor.text_for_range(2..2, &mut adjusted_range, window, cx),
                Some(String::from("😀")),
            );
            assert_eq!(adjusted_range, Some(1..3));
            let replacement = editor
                .marked_text_range(window, cx)
                .expect("queued composition range");
            editor.unmark_text(window, cx);
            assert_eq!(editor.marked_text_range(window, cx), None);
            assert_eq!(
                editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| selection.range),
                Some(3..3)
            );
            editor.replace_text_in_range(Some(replacement), "界", window, cx);
            assert_eq!(editor.marked_text_range(window, cx), None);
            assert_eq!(
                editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| selection.range),
                Some(2..2)
            );
            assert_eq!(
                editor.text_for_range(1..2, &mut None, window, cx),
                Some(String::from("界"))
            );
            editor.replace_text_in_range(Some(0..1), "Z", window, cx);
            editor.handle_input("!", window, cx);
            editor.buffer.update(cx, |buffer, cx| {
                buffer.edit(
                    [(MultiBufferOffset(0)..MultiBufferOffset(0), "q\n")],
                    None,
                    cx,
                );
            });
            editor.input_completion(cx)
        });
        completion.await.expect("deferred input completed");
        cx.assert_editor_state(&format!("q\nab\nZ!ˇ界{}", "x".repeat(1_999)));
        for (add_cursor, length) in [(false, 2_001), (true, 2_002)] {
            let completion = cx.update_editor(|editor, window, cx| {
                editor.clear_highlights(HighlightKey::InputComposition, cx);
                editor.set_text(format!("ab\n{}", "y".repeat(length)), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([Point::new(0, 1)..Point::new(0, 1)]);
                });
                editor.replace_and_mark_text_in_range(None, "é", None, window, cx);
                if add_cursor {
                    editor.add_selection_below(
                        &AddSelectionBelow {
                            skip_soft_wrap: false,
                        },
                        window,
                        cx,
                    );
                } else {
                    editor.move_down(&MoveDown, window, cx);
                }
                assert!(editor.input_is_deferred());
                editor.replace_and_mark_text_in_range(Some(0..1), "界", None, window, cx);
                editor.clear_highlights(HighlightKey::InputComposition, cx);
                editor.input_completion(cx)
            });
            completion.await.expect("captured marked range completed");
            cx.assert_editor_state(&format!("a界ˇb\n{}", "y".repeat(length)));
        }
    }

    #[gpui::test]
    #[expect(clippy::reversed_empty_ranges)]
    async fn test_deferred_ime_normalizes_replacement_ranges(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let text = "x".repeat(2_000);
        for (replacement, inserted, selected, retained, marked, selection) in [
            (5000..5000, "a", 1..1, 2_000, Some(2000..2001), 2001..2001),
            (5000..0, "a", 1..1, 2_000, Some(2000..2001), 2001..2001),
            (1999..5000, "a", 1..1, 1_999, Some(1999..2000), 2000..2000),
            (0..5000, "", 5000..0, 0, None, 0..0),
        ] {
            for deferred in [false, true] {
                let mut cx = ime_test_context(&text, deferred, cx).await;
                let (actual_marked, actual_selection, completion) =
                    cx.update_editor(|editor, window, cx| {
                        assert_eq!(editor.input_is_deferred(), deferred);
                        editor.replace_and_mark_text_in_range(
                            Some(replacement.clone()),
                            inserted,
                            Some(selected.clone()),
                            window,
                            cx,
                        );
                        let marked = editor.marked_text_range(window, cx);
                        let selection = editor
                            .selected_text_range(false, window, cx)
                            .map(|selection| (selection.range, selection.reversed));
                        if inserted.is_empty() {
                            editor.replace_and_mark_text_in_range(
                                Some(5000..5000),
                                "😀",
                                Some(1..1),
                                window,
                                cx,
                            );
                        }
                        editor.replace_text_in_range(None, "b", window, cx);
                        (marked, selection, editor.input_completion(cx))
                    });
                cx.background_executor.set_block_on_ticks(0..=50);
                completion.await.expect("IME replacement completed");
                let actual = cx.buffer_text();
                assert_eq!(
                    actual.strip_prefix(&"x".repeat(retained)),
                    Some("b"),
                    "deferred={deferred}, replacement={replacement:?}"
                );
                assert_eq!(actual_marked, marked, "deferred={deferred}");
                assert_eq!(
                    actual_selection,
                    Some((selection.clone(), false)),
                    "deferred={deferred}"
                );
            }
        }
    }

    #[gpui::test]
    async fn test_deferred_ime_normalizes_original_surrogate_boundaries(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let trailing_text = "x".repeat(2_000);
        for deferred in [false, true] {
            let mut cx = ime_test_context(&format!("a😀b{trailing_text}"), deferred, cx).await;
            let (marked, selection, completion) = cx.update_editor(|editor, window, cx| {
                assert_eq!(editor.input_is_deferred(), deferred);
                editor.replace_and_mark_text_in_range(Some(2..2), "Q", Some(1..1), window, cx);
                let marked = editor.marked_text_range(window, cx);
                let selection = editor
                    .selected_text_range(false, window, cx)
                    .map(|selection| (selection.range, selection.reversed));
                editor.replace_text_in_range(None, "Z", window, cx);
                editor.replace_text_in_range(Some(2..3), "B", window, cx);
                (marked, selection, editor.input_completion(cx))
            });
            cx.background_executor.set_block_on_ticks(0..=50);
            completion.await.expect("surrogate replacement completed");
            let actual = cx.buffer_text();
            assert_eq!(
                actual.strip_suffix(&trailing_text),
                Some("aZB"),
                "deferred={deferred}"
            );
            assert_eq!(marked, Some(1..2), "deferred={deferred}");
            assert_eq!(selection, Some((2..2, false)), "deferred={deferred}");
        }
    }

    #[gpui::test]
    #[expect(clippy::reversed_empty_ranges)]
    async fn test_deferred_ime_normalizes_selected_ranges(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let text = "x".repeat(2_000);
        for (selected, expected) in [
            (usize::MAX..usize::MAX, (2003..2003, false)),
            (3..1, (3..4, true)),
            (5000..5000, (2003..2003, false)),
            (1..1, (1..3, false)),
        ] {
            for deferred in [false, true] {
                let mut cx = ime_test_context(&text, deferred, cx).await;
                let (selection, completion) = cx.update_editor(|editor, window, cx| {
                    assert_eq!(editor.input_is_deferred(), deferred);
                    editor.replace_and_mark_text_in_range(
                        Some(1..1),
                        "😀q",
                        Some(selected.clone()),
                        window,
                        cx,
                    );
                    let selection = editor
                        .selected_text_range(false, window, cx)
                        .map(|selection| (selection.range, selection.reversed));
                    editor.replace_text_in_range(None, "Z", window, cx);
                    (selection, editor.input_completion(cx))
                });
                cx.background_executor.set_block_on_ticks(0..=50);
                completion.await.expect("selected IME range completed");
                assert_eq!(cx.buffer_text(), format!("xZ{}", "x".repeat(1_999)));
                assert_eq!(selection, Some(expected.clone()), "deferred={deferred}");
            }
        }
    }

    #[gpui::test]
    #[expect(clippy::reversed_empty_ranges)]
    async fn test_deferred_ime_normalizes_existing_marked_range(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let text = "x".repeat(2_000);
        for deferred in [false, true] {
            let mut cx = ime_test_context(&text, false, cx).await;
            cx.background_executor.set_block_on_ticks(0..=0);
            let movement = cx.update_editor(|editor, window, cx| {
                editor.replace_and_mark_text_in_range(Some(1..2), "a", Some(0..0), window, cx);
                editor.move_up(&MoveUp, window, cx);
                assert!(editor.input_is_deferred());
                (!deferred).then(|| editor.input_completion(cx))
            });
            if let Some(movement) = movement {
                cx.background_executor.set_block_on_ticks(0..=50);
                movement.await.expect("marked reference movement completed");
            }
            let completion = cx.update_editor(|editor, window, cx| {
                assert_eq!(editor.input_is_deferred(), deferred);
                editor.replace_and_mark_text_in_range(Some(3..0), "b", Some(1..1), window, cx);
                editor.replace_text_in_range(None, "Z", window, cx);
                editor.replace_text_in_range(Some(5..6), "B", window, cx);
                editor.input_completion(cx)
            });
            cx.background_executor.set_block_on_ticks(0..=50);
            completion.await.expect("marked replacement completed");
            let actual = cx.buffer_text();
            assert_eq!(
                actual.strip_suffix(&"x".repeat(1_992)),
                Some("xaxxZBxxx"),
                "deferred={deferred}"
            );
        }
    }

    struct MovedEditor(Entity<Editor>);

    impl Render for MovedEditor {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            EditorElement::new(&self.0, self.0.read(cx).create_style(cx))
        }
    }

    async fn ime_test_context(
        text: &str,
        deferred: bool,
        cx: &mut TestAppContext,
    ) -> EditorTestContext {
        let mut cx = EditorTestContext::new(cx).await;
        cx.background_executor.set_block_on_ticks(0..=0);
        let movement = cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor.set_text(text, window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::zero()..Point::zero()]);
            });
            editor.move_up(&MoveUp, window, cx);
            assert!(editor.input_is_deferred());
            (!deferred).then(|| editor.input_completion(cx))
        });
        if let Some(movement) = movement {
            cx.background_executor.set_block_on_ticks(0..=50);
            movement.await.expect("warm reference movement completed");
        }
        cx
    }

    fn assert_placeholder_geometry_ready(
        editor: &mut Editor,
        requested: PendingRowGeometry,
        events: Rc<RefCell<Vec<&'static str>>>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let source = editor
            .placeholder_display_map
            .as_ref()
            .expect("placeholder map");
        assert_eq!(source.entity_id(), requested.display_map_id());
        let snapshot = source.update(cx, |map, cx| map.snapshot(cx));
        match snapshot.long_row(DisplayRow(0)) {
            Ok(ready) => {
                assert!(ready.is_some());
                events.borrow_mut().push("placeholder");
            }
            Err(pending) => {
                assert!(
                    !requested.is_current(&snapshot),
                    "current placeholder request completed without geometry"
                );
                editor.defer_geometry(
                    pending.clone(),
                    move |editor, window, cx| {
                        assert_placeholder_geometry_ready(editor, pending, events, window, cx);
                    },
                    window,
                    cx,
                );
            }
        }
    }

    fn pending_placeholder_geometry(editor: &Editor, cx: &mut App) -> PendingRowGeometry {
        editor
            .placeholder_display_map
            .as_ref()
            .expect("placeholder")
            .update(cx, |map, cx| {
                map.snapshot(cx)
                    .long_row(DisplayRow(0))
                    .err()
                    .expect("cold placeholder")
            })
    }
}
