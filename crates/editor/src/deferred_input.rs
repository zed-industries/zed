use super::*;
use crate::display_map::RowGeometryLease;
use futures::{channel::oneshot, task::AtomicWaker};
use gpui::AnyWindowHandle;
use std::{cell::Cell, future::poll_fn, rc::Weak, task::Poll};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipboardAccess {
    Read,
    Write,
    Observe,
}

#[derive(Clone, Copy, Default)]
pub struct NativeInput {
    pub clipboard: Option<ClipboardAccess>,
    pub primary: Option<ClipboardAccess>,
}

pub struct SharedInputScope(CapturedInput);

impl SharedInputScope {
    pub fn run<C: std::borrow::BorrowMut<App>, R>(
        self,
        cx: &mut C,
        handler: impl FnOnce(&mut C) -> R,
    ) -> R {
        self.0.run(cx, handler)
    }
}

#[derive(Clone, Default)]
pub(super) struct CapturedInput {
    pub(super) completion: Option<completions::CapturedCompletionAction>,
    pub(super) edit_prediction: Option<edit_prediction::CapturedEditPredictionAction>,
    pub(super) clipboard: Option<clipboard::CapturedClipboardAction>,
    pub(super) clipboard_scope: Option<Rc<()>>,
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
            if self.matches_clipboard_scope(current.clipboard_scope.as_ref()) {
                self.clipboard = self.clipboard.or_else(|| current.clipboard.clone());
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                {
                    self.primary = self.primary.or_else(|| current.primary.clone());
                }
            }
        }
        Self::run_optional(Some(self), cx, handler)
    }

    fn for_action(window: &mut Window, cx: &App) -> Self {
        let mut input = Self::current(cx).cloned().unwrap_or_default();
        let scope = window.action_dispatch_scope();
        if !input.matches_clipboard_scope(scope.as_ref()) {
            input.clipboard = None;
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            {
                input.primary = None;
            }
        }
        input.clipboard_scope = scope;
        input
    }

    fn matches_clipboard_scope(&self, scope: Option<&Rc<()>>) -> bool {
        match (self.clipboard_scope.as_ref(), scope) {
            (Some(current), Some(scope)) => Rc::ptr_eq(current, scope),
            (None, None) => true,
            _ => false,
        }
    }

    fn capture(&mut self, required: NativeInput, editor: &Editor, cx: &App) {
        if required
            .clipboard
            .is_some_and(|access| access != ClipboardAccess::Write)
            && self.clipboard.is_none()
        {
            self.clipboard = Some(editor.capture_clipboard(cx));
        }
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        if required
            .primary
            .is_some_and(|access| access != ClipboardAccess::Write)
            && self.primary.is_none()
        {
            self.primary = Some(editor.capture_primary(cx));
        }
    }

    fn is_ready(&self, required: NativeInput) -> bool {
        self.required_captures(required)
            .all(|(_, capture)| capture.is_none_or(|capture| capture.is_ready()))
    }

    async fn wait(&self, required: NativeInput) -> Result<()> {
        futures::future::try_join_all(self.required_captures(required).map(
            async |(access, capture)| {
                let capture =
                    capture.context("Required native clipboard capture is unavailable")?;
                access.admit(capture.wait().await)
            },
        ))
        .await?;
        Ok(())
    }

    fn check_ready(&self, required: NativeInput) -> Result<()> {
        for (access, capture) in self.required_captures(required) {
            let capture = capture.context("Required native clipboard capture is unavailable")?;
            access.admit(capture.check_ready())?;
        }
        Ok(())
    }

    fn required_captures(
        &self,
        required: NativeInput,
    ) -> impl Iterator<Item = (ClipboardAccess, Option<&clipboard::CapturedClipboardAction>)> {
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        let primary = self.primary.as_ref();
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        let primary = None;
        [
            required
                .clipboard
                .map(|mode| (mode, self.clipboard.as_ref())),
            required.primary.map(|mode| (mode, primary)),
        ]
        .into_iter()
        .flatten()
        .filter(|(access, _)| *access != ClipboardAccess::Write)
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

impl ClipboardAccess {
    fn admit(self, result: Result<()>) -> Result<()> {
        match self {
            Self::Observe => Ok(()),
            Self::Read | Self::Write => result,
        }
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
    active_checkpoints: Vec<Weak<RefCell<InputCheckpoint>>>,
    executing: bool,
    insertion_index: usize,
    after_effects: Vec<QueuedInput>,
    geometry: Vec<(EntityId, RowGeometryLease)>,
    window_closed: Option<Subscription>,
}

struct InputOwner {
    editor: WeakEntity<Editor>,
    buffer: WeakEntity<MultiBuffer>,
    window: AnyWindowHandle,
    pending: Cell<usize>,
    publications: Cell<usize>,
    cancelled: Cell<bool>,
    failure: RefCell<Option<String>>,
    completions: RefCell<Vec<oneshot::Sender<Result<()>>>>,
}

struct InputCheckpoint {
    completion: Option<oneshot::Sender<Result<()>>>,
    failure: Option<String>,
    editors: Vec<Entity<Editor>>,
}

struct QueuedInput {
    owner: Rc<InputOwner>,
    checkpoints: Vec<Weak<RefCell<InputCheckpoint>>>,
    publication: bool,
    operation: DeferredInputOperation,
}

enum InputPrerequisite {
    None,
    Geometry(PendingRowGeometry),
    Native(NativeInput),
    Publication(Task<Result<()>>),
}

struct DeferredInputOperation {
    geometry: Vec<(EntityId, RowGeometryLease)>,
    prerequisite: InputPrerequisite,
    input: Option<CapturedInput>,
    continuation: DeferredInputContinuation,
}

type DeferredInputContinuation = Box<dyn FnOnce(Result<()>, &mut Window, &mut App)>;

impl DeferredInput {
    pub(super) fn new(
        buffer: WeakEntity<MultiBuffer>,
        window: &Window,
        cx: &mut Context<Editor>,
    ) -> Self {
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
                buffer,
                window: window.window_handle(),
                pending: Cell::new(0),
                publications: Cell::new(0),
                cancelled: Cell::new(false),
                failure: RefCell::new(None),
                completions: RefCell::new(Vec::new()),
            }),
        }
    }
}

impl InputCheckpoint {
    fn register(
        checkpoint: &Rc<RefCell<Self>>,
        owner: &InputOwner,
        checkpoints: &mut Vec<Weak<RefCell<Self>>>,
    ) {
        checkpoints.retain(|checkpoint| {
            checkpoint
                .upgrade()
                .is_some_and(|checkpoint| checkpoint.borrow().completion.is_some())
        });
        checkpoints.push(Rc::downgrade(checkpoint));
        checkpoint.borrow_mut().retain_owner(owner);
    }

    fn retain_owner(&mut self, owner: &InputOwner) {
        if !self
            .editors
            .iter()
            .any(|editor| editor.entity_id() == owner.editor.entity_id())
            && let Some(editor) = owner.editor.upgrade()
        {
            self.editors.push(editor);
        }
    }

    fn record_failure(checkpoints: &mut Vec<Weak<RefCell<Self>>>, error: &str) {
        checkpoints.retain(|checkpoint| {
            let Some(checkpoint) = checkpoint.upgrade() else {
                return false;
            };
            let mut checkpoint = checkpoint.borrow_mut();
            if checkpoint.completion.is_none() {
                return false;
            }
            checkpoint.failure.get_or_insert_with(|| error.to_owned());
            true
        });
    }

    fn complete(&mut self) {
        if let Some(completion) = self.completion.take() {
            let result = self
                .failure
                .take()
                .map_or(Ok(()), |error| Err(anyhow!(error)));
            completion.send(result).ok();
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
        let (removed, mut checkpoints) = {
            let mut state = self.state.borrow_mut();
            let checkpoints = if state
                .active
                .as_ref()
                .is_some_and(|active| Rc::ptr_eq(active, owner))
            {
                state.active_checkpoints.clone()
            } else {
                Vec::new()
            };
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
            (removed, checkpoints)
        };
        for entry in &removed {
            checkpoints.extend(entry.checkpoints.iter().cloned());
        }
        owner.pending.set(0);
        owner.publications.set(0);
        owner
            .failure
            .borrow_mut()
            .get_or_insert_with(|| error.to_owned());
        InputCheckpoint::record_failure(&mut checkpoints, error);
        for checkpoint in checkpoints {
            if let Some(checkpoint) = checkpoint.upgrade() {
                checkpoint.borrow_mut().complete();
            }
        }
        for completion in owner.completions.borrow_mut().drain(..) {
            completion.send(Err(anyhow!(error.to_owned()))).ok();
        }
        drop(removed);
        self.wake.wake();
    }

    fn enqueue(&self, mut entry: QueuedInput, after_effects: bool, fresh: bool) {
        let mut state = self.state.borrow_mut();
        if state.executing && !fresh {
            entry
                .checkpoints
                .extend(state.active_checkpoints.iter().cloned());
            entry.checkpoints.retain(|checkpoint| {
                let Some(checkpoint) = checkpoint.upgrade() else {
                    return false;
                };
                let mut checkpoint = checkpoint.borrow_mut();
                if checkpoint.completion.is_none() {
                    return false;
                }
                checkpoint.retain_owner(&entry.owner);
                true
            });
        }
        if entry.owner.cancelled.get() {
            let error = "editor closed before deferred input completed";
            entry
                .owner
                .failure
                .borrow_mut()
                .get_or_insert_with(|| error.to_owned());
            InputCheckpoint::record_failure(&mut entry.checkpoints, error);
            return;
        }
        let count = if entry.publication {
            &entry.owner.publications
        } else {
            &entry.owner.pending
        };
        count.set(count.get() + 1);
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

    fn record_failure(&self, error: &anyhow::Error) {
        let mut state = self.state.borrow_mut();
        let Some(owner) = &state.active else { return };
        let error = format!("Clipboard operation failed: {error:#}");
        owner
            .failure
            .borrow_mut()
            .get_or_insert_with(|| error.clone());
        InputCheckpoint::record_failure(&mut state.active_checkpoints, &error);
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
                    let mut entry = state.queue.pop_front();
                    state.active = entry.as_ref().map(|entry| entry.owner.clone());
                    state.active_checkpoints = entry
                        .as_mut()
                        .map(|entry| mem::take(&mut entry.checkpoints))
                        .unwrap_or_default();
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
                    mut prerequisite,
                    mut input,
                    continuation,
                } = entry.operation;
                cx.update(|_| {});
                let readiness = futures_lite::future::race(
                    async {
                        match &mut prerequisite {
                            InputPrerequisite::None => {}
                            InputPrerequisite::Publication(task) => task.await?,
                            InputPrerequisite::Geometry(pending) => pending.wait().await,
                            InputPrerequisite::Native(required) => {
                                if !owner.cancelled.get() {
                                    let input = input.get_or_insert_with(CapturedInput::default);
                                    owner.editor.read_with(cx, |editor, cx| {
                                        input.capture(*required, editor, cx);
                                    })?;
                                    input.wait(*required).await?;
                                }
                            }
                        }
                        anyhow::Ok(())
                    },
                    poll_fn(|cx| {
                        wake.register(cx.waker());
                        if owner.cancelled.get() {
                            Poll::Ready(Ok(()))
                        } else {
                            Poll::Pending
                        }
                    }),
                )
                .await;
                drop(prerequisite);
                cx.update(|_| {});
                let Some(queue_state) = state.upgrade() else {
                    return;
                };
                let queue = InputQueue {
                    state: queue_state,
                    wake: wake.clone(),
                };
                if let Err(error) = &readiness {
                    queue.record_failure(error);
                }
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
                        let result = owner.window.update(cx, |_, window, cx| {
                            owner
                                .editor
                                .update(cx, |editor, cx| {
                                    if editor
                                        .display_snapshot(cx)
                                        .tab_snapshot()
                                        .has_unrepresentable_rows()
                                    {
                                        editor.cancel_unrepresentable_input();
                                    }
                                })
                                .ok();
                            if !owner.cancelled.get() {
                                continuation(readiness, window, cx);
                            }
                        });
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
                {
                    let mut state = queue.state.borrow_mut();
                    state.active = None;
                    state.active_checkpoints.clear();
                }
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
                            let failure = owner.failure.borrow_mut().take();
                            for completion in owner.completions.borrow_mut().drain(..) {
                                let result = failure
                                    .as_ref()
                                    .map_or(Ok(()), |error| Err(anyhow!(error.clone())));
                                completion.send(result).ok();
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
    pub fn has_unrepresentable_rows(&self, cx: &mut App) -> bool {
        self.display_snapshot(cx)
            .tab_snapshot()
            .has_unrepresentable_rows()
    }

    pub fn defer_keystroke_input(
        &mut self,
        continuation: impl FnOnce(&mut Window, &mut App) -> Box<dyn FnOnce(&mut Window, &mut App)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let was_pending = self.input_is_deferred(cx);
        if was_pending && self.has_unrepresentable_rows(cx) {
            self.cancel_unrepresentable_input();
            self.bind_input_window(window);
            return false;
        }
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        self.capture_input(None, cx).run(cx, |cx| {
            let continuation = continuation(window, cx);
            self.enqueue_deferred_input(DeferredInputOperation {
                geometry: Vec::new(),
                prerequisite: InputPrerequisite::None,
                input: CapturedInput::current(cx).cloned(),
                continuation: Box::new(move |_, window, cx| continuation(window, cx)),
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
        let continuation = self.editor_input_continuation(continuation);
        self.defer_external_geometry(
            pending,
            move |window, cx| continuation(Ok(()), window, cx),
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
        let (input, continuation) = match window.defer_current_action_dispatch(cx, continuation) {
            Ok(continuation) => (Some(self.capture_input(None, cx)), continuation),
            Err(continuation) => (
                CapturedInput::current(cx).cloned(),
                Box::new(continuation) as Box<dyn FnOnce(&mut Window, &mut App)>,
            ),
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
            prerequisite: if requested {
                InputPrerequisite::Geometry(pending)
            } else {
                InputPrerequisite::None
            },
            input,
            continuation: Box::new(move |_, window, cx| continuation(window, cx)),
        });
        self.deferred_input.shared.start(cx);
    }

    pub fn defer_external_input(
        &mut self,
        continuation: impl FnOnce(&mut Window, &mut App) + 'static,
        cx: &App,
    ) -> bool {
        if !self.input_is_deferred(cx) {
            return false;
        }
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            prerequisite: InputPrerequisite::None,
            input: CapturedInput::current(cx).cloned(),
            continuation: Box::new(move |_, window, cx| continuation(window, cx)),
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
                checkpoints: Vec::new(),
                publication: false,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    prerequisite: InputPrerequisite::None,
                    input: None,
                    continuation: Box::new(move |_, window, cx| continuation(window, cx)),
                },
            },
            false,
            true,
        );
        true
    }

    pub fn defer_shared_input(
        &mut self,
        required: NativeInput,
        continuation: impl FnOnce(Result<()>, &mut Window, &mut App) + Clone + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<Option<SharedInputScope>> {
        self.bind_input_window(window);
        let mut input = CapturedInput::for_action(window, cx);
        if !self.shared_input_is_deferred() {
            input.capture(required, self, cx);
            if input.is_ready(required) {
                if let Err(error) = input.check_ready(required) {
                    if self.input_is_replaying() {
                        self.deferred_input.shared.record_failure(&error);
                    }
                    return Err(error);
                }
                return Ok(Some(SharedInputScope(input)));
            }
        }
        let rejected = continuation.clone();
        let continuation = move |window: &mut Window, cx: &mut App| {
            continuation(Ok(()), window, cx);
        };
        let continuation = match window.defer_current_action_dispatch(cx, continuation) {
            Ok(continuation) => continuation,
            Err(continuation) => Box::new(continuation),
        };
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            prerequisite: InputPrerequisite::Native(required),
            input: Some(input),
            continuation: Box::new(move |result, window, cx| match result {
                Ok(()) => continuation(window, cx),
                Err(error) => {
                    cx.stop_propagation();
                    rejected(Err(error), window, cx);
                }
            }),
        });
        self.deferred_input.shared.start(cx);
        Ok(None)
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
                checkpoints: Vec::new(),
                publication: false,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    prerequisite: InputPrerequisite::None,
                    input: None,
                    continuation: Box::new(move |_, window, cx| continuation(window, cx)),
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
        let was_pending = self.input_is_deferred(cx);
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        self.enqueue_deferred_input(DeferredInputOperation {
            geometry: Vec::new(),
            prerequisite: InputPrerequisite::None,
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

    pub(super) fn has_pending_document_input(&self, cx: &App) -> bool {
        !self.document_input_owners(cx).is_empty()
    }

    pub(super) fn pending_document_input(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let owners = self.document_input_owners(cx);
        if owners.is_empty() {
            return Task::ready(Ok(()));
        }
        if self.deferred_input.owner.cancelled.get() {
            return Task::ready(Err(anyhow!(
                "editor closed before deferred input completed"
            )));
        }

        let (sender, receiver) = oneshot::channel();
        let checkpoint = Rc::new(RefCell::new(InputCheckpoint {
            completion: Some(sender),
            failure: owners
                .iter()
                .find_map(|owner| owner.failure.borrow().clone()),
            editors: Vec::new(),
        }));
        {
            let relevant = |owner: &Rc<InputOwner>| {
                owners.iter().any(|candidate| Rc::ptr_eq(candidate, owner))
            };
            let mut state = self.deferred_input.shared.state.borrow_mut();
            let active = state.active.clone().filter(&relevant);
            let replaying = active.is_some() && state.executing;
            if let Some(owner) = active {
                InputCheckpoint::register(&checkpoint, &owner, &mut state.active_checkpoints);
            }
            let insertion_index = if replaying { state.insertion_index } else { 0 };
            for (index, entry) in state.queue.iter_mut().enumerate() {
                if index < insertion_index || relevant(&entry.owner) {
                    InputCheckpoint::register(&checkpoint, &entry.owner, &mut entry.checkpoints);
                }
            }
            for entry in &mut state.after_effects {
                if replaying || relevant(&entry.owner) {
                    InputCheckpoint::register(&checkpoint, &entry.owner, &mut entry.checkpoints);
                }
            }
        }
        checkpoint
            .borrow_mut()
            .retain_owner(&self.deferred_input.owner);
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                checkpoints: vec![Rc::downgrade(&checkpoint)],
                publication: false,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    prerequisite: InputPrerequisite::None,
                    input: None,
                    continuation: Box::new({
                        let checkpoint = Rc::downgrade(&checkpoint);
                        move |_, _, _| {
                            if let Some(checkpoint) = checkpoint.upgrade() {
                                checkpoint.borrow_mut().complete();
                            }
                        }
                    }),
                },
            },
            false,
            true,
        );
        self.deferred_input.shared.start(cx);
        cx.foreground_executor().spawn(async move {
            let result = receiver
                .await
                .context("editor closed before deferred input completed")?;
            drop(checkpoint);
            result
        })
    }

    pub(super) fn defer_native_input(
        &mut self,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let was_pending = self.input_is_deferred(cx);
        self.bind_input_window(window);
        if !was_pending {
            return false;
        }
        self.enqueue_deferred_input_preserving_composition(DeferredInputOperation {
            geometry: Vec::new(),
            prerequisite: InputPrerequisite::None,
            input: CapturedInput::current(cx).cloned(),
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
        if !self.input_is_deferred(cx) {
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

    pub(super) fn input_is_deferred(&self, cx: &App) -> bool {
        let state = self.deferred_input.shared.state.borrow();
        if state.executing {
            state.insertion_index > 0
        } else {
            self.document_input_owners(cx)
                .iter()
                .any(|owner| owner.pending.get() > 0)
        }
    }

    pub(super) fn input_is_replaying(&self) -> bool {
        self.deferred_input.shared.state.borrow().executing
    }

    pub(super) fn shared_input_is_deferred(&self) -> bool {
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
        self.defer_clipboard_action_with_access(ClipboardAccess::Write, action, apply, window, cx)
    }

    pub(super) fn defer_clipboard_action_with_access<A: Clone + 'static>(
        &mut self,
        access: ClipboardAccess,
        action: &A,
        apply: fn(&mut Editor, &A, &mut Window, &mut Context<Editor>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.bind_input_window(window);
        if access == ClipboardAccess::Write && !self.shared_input_is_deferred() {
            return false;
        }
        if !self.shared_input_is_deferred()
            && let Some(capture) = CapturedInput::current(cx)
                .filter(|input| {
                    input.matches_clipboard_scope(window.action_dispatch_scope().as_ref())
                })
                .and_then(|input| input.clipboard.as_ref())
            && capture.is_ready()
        {
            if let Err(error) = access.admit(capture.check_ready()) {
                self.notify_clipboard_error(error, cx);
                return true;
            }
            return false;
        }
        let retry = action.clone();
        let editor = cx.weak_entity();
        match self.defer_shared_input(
            NativeInput {
                clipboard: Some(access),
                ..NativeInput::default()
            },
            move |result, window, cx| {
                editor
                    .update(cx, |editor, cx| match result {
                        Ok(()) => apply(editor, &retry, window, cx),
                        Err(error) => editor.notify_clipboard_error(error, cx),
                    })
                    .ok();
            },
            window,
            cx,
        ) {
            Ok(Some(scope)) => scope.run(cx, |cx| apply(self, action, window, cx)),
            Ok(None) => {}
            Err(error) => self.notify_clipboard_error(error, cx),
        }
        true
    }

    pub(super) fn defer_clipboard_publication(
        &mut self,
        continuation: impl FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                checkpoints: Vec::new(),
                publication: true,
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    prerequisite: InputPrerequisite::None,
                    input: None,
                    continuation: Box::new(self.editor_input_continuation(continuation)),
                },
            },
            false,
            false,
        );
        self.deferred_input.shared.start(cx);
    }

    pub(super) fn defer_clipboard_write(
        &mut self,
        task: Task<Result<()>>,
        continuation: Option<InputContinuation>,
        cx: &mut Context<Self>,
    ) {
        self.deferred_input.shared.enqueue(
            QueuedInput {
                owner: self.deferred_input.owner.clone(),
                checkpoints: Vec::new(),
                publication: continuation.is_none(),
                operation: DeferredInputOperation {
                    geometry: Vec::new(),
                    prerequisite: InputPrerequisite::Publication(task),
                    input: CapturedInput::current(cx).cloned(),
                    continuation: Box::new(self.editor_input_continuation(
                        move |editor, window, cx| {
                            if let Some(continuation) = continuation {
                                continuation(editor, window, cx);
                            }
                        },
                    )),
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
        if !self.input_is_deferred(cx) {
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
        recovery: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
        handler: impl FnOnce(&A, &mut Window, &mut App) + 'static,
    ) -> bool {
        if self.has_unrepresentable_rows(cx) {
            self.cancel_unrepresentable_input();
            if recovery {
                self.bind_input_window(window);
                return false;
            }
            cx.stop_propagation();
            return true;
        }
        let was_pending = self.input_is_deferred(cx);
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
            prerequisite: InputPrerequisite::None,
            input: Some(self.capture_input(Some(action), cx)),
            continuation: Box::new(move |_, window, cx| continuation(window, cx)),
        });
        self.deferred_input.shared.start(cx);
        true
    }

    pub(super) fn notify_clipboard_error(&self, error: anyhow::Error, cx: &mut App) {
        if self.input_is_replaying() {
            self.deferred_input.shared.record_failure(&error);
        }
        let message = format!("Clipboard operation failed: {error:#}");
        log::error!("{message}");
        if let Some(workspace) = self.workspace() {
            cx.defer(move |cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.show_toast(
                        Toast::new(
                            NotificationId::unique::<clipboard::CapturedClipboardAction>(),
                            message,
                        ),
                        cx,
                    );
                });
            });
        }
    }

    pub(super) fn cancel_unrepresentable_input(&mut self) {
        self.deferred_input.shared.cancel(
            &self.deferred_input.owner,
            "tab-expanded line exceeds the supported column range; queued input cancelled",
        );
        self.clear_finished_input();
    }

    pub(super) fn bind_input_window(&mut self, window: &Window) {
        if self.deferred_input.owner.cancelled.get()
            || self.deferred_input.owner.window != window.window_handle()
        {
            self.deferred_input.shared.cancel(
                &self.deferred_input.owner,
                "editor moved to another window before deferred input completed",
            );
            self.deferred_input.owner = Rc::new(InputOwner {
                editor: self.deferred_input.owner.editor.clone(),
                buffer: self.buffer.downgrade(),
                window: window.window_handle(),
                pending: Cell::new(0),
                publications: Cell::new(0),
                cancelled: Cell::new(false),
                failure: RefCell::new(None),
                completions: RefCell::new(Vec::new()),
            });
            self.clear_finished_input();
        }
    }

    fn document_input_owners(&self, cx: &App) -> Vec<Rc<InputOwner>> {
        let state = self.deferred_input.shared.state.borrow();
        if self.deferred_input.owner.pending.get() == 0
            && self.deferred_input.owner.publications.get() == 0
            && state.active.is_none()
            && state.queue.is_empty()
            && state.after_effects.is_empty()
        {
            return Vec::new();
        }
        let buffer_id = |mut buffer: Entity<Buffer>| {
            while let Some(base_buffer) = buffer.read(cx).base_buffer() {
                buffer = base_buffer;
            }
            buffer.entity_id()
        };
        let buffers = self
            .buffer
            .read(cx)
            .all_buffers_iter()
            .map(&buffer_id)
            .collect::<HashSet<_>>();
        let mut seen = HashSet::default();
        std::iter::once(&self.deferred_input.owner)
            .chain(state.active.iter())
            .chain(state.queue.iter().map(|entry| &entry.owner))
            .chain(state.after_effects.iter().map(|entry| &entry.owner))
            .filter(|owner| {
                !owner.cancelled.get()
                    && (owner.pending.get() > 0 || owner.publications.get() > 0)
                    && seen.insert(owner.editor.entity_id())
            })
            .filter(|owner| {
                Rc::ptr_eq(owner, &self.deferred_input.owner)
                    || owner.buffer.upgrade().is_some_and(|buffer| {
                        buffer
                            .read(cx)
                            .all_buffers_iter()
                            .any(|buffer| buffers.contains(&buffer_id(buffer)))
                    })
            })
            .cloned()
            .collect()
    }

    fn capture_input(&self, action: Option<&dyn Any>, cx: &Context<Self>) -> CapturedInput {
        CapturedInput {
            completion: Some(
                action
                    .and_then(|action| self.capture_completion_action(action, cx))
                    .unwrap_or_else(|| self.capture_completion_target(None, cx)),
            ),
            edit_prediction: Some(self.capture_edit_prediction_target(cx)),
            ..CapturedInput::current(cx).cloned().unwrap_or_default()
        }
    }

    fn editor_input_continuation<F>(
        &self,
        continuation: F,
    ) -> impl FnOnce(Result<()>, &mut Window, &mut App) + 'static + use<F>
    where
        F: FnOnce(&mut Editor, &mut Window, &mut Context<Editor>) + 'static,
    {
        let editor = self.deferred_input.owner.editor.clone();
        move |result, window, cx| {
            editor
                .update(cx, |editor, cx| match result {
                    Ok(()) => continuation(editor, window, cx),
                    Err(error) => editor.notify_clipboard_error(error, cx),
                })
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
                checkpoints: Vec::new(),
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
        test::{NativeClipboardCapture, build_editor, editor_test_context::EditorTestContext},
    };
    use gpui::{
        ClipboardReadError, ClipboardSnapshot, ClipboardSnapshotRead, ElementInputHandler, Empty,
        InputHandler as _, KeyBinding, Keystroke, Modifiers, MouseButton, MouseDownEvent,
        MouseUpEvent, PlatformInput, TestAppContext, rgba,
    };
    use language::{LanguageConfig, PLAIN_TEXT, language_settings::SoftWrap};

    gpui::actions!(deferred_input_test, [InsertQueuedAction]);

    #[derive(Clone, PartialEq, Action)]
    #[action(no_json, no_register)]
    struct QueuedSelectionAction(Arc<str>);

    #[gpui::test]
    async fn test_shared_input_nested_actions_preserve_only_their_own_capture(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        for delayed in [false, true] {
            let mut editor_cx = EditorTestContext::new(cx).await;
            let observed = Rc::new(RefCell::new(Vec::new()));
            let subscription = editor_cx.update_editor(|editor, _, cx| {
                let handle = cx.weak_entity();
                let observed = observed.clone();
                editor.register_action(move |action: &QueuedSelectionAction, window, cx| {
                    let nested = action.0.as_ref() == "nested";
                    let required = NativeInput {
                        clipboard: Some(ClipboardAccess::Read),
                        ..NativeInput::default()
                    };
                    let complete = {
                        let handle = handle.clone();
                        let observed = observed.clone();
                        move |result: Result<()>, window: &mut Window, cx: &mut App| {
                            result.expect("native admission");
                            let record = |cx: &App| {
                                let text = handle
                                    .read_with(cx, |editor, cx| editor.read_from_clipboard(cx))
                                    .expect("editor")
                                    .expect("clipboard")
                                    .and_then(|item| item.text());
                                observed.borrow_mut().push(text);
                            };
                            record(cx);
                            if nested {
                                return;
                            }
                            cx.write_to_clipboard(ClipboardItem::new_string(String::from("B")));
                            let scope = handle
                                .update(cx, |editor, cx| {
                                    editor.defer_shared_input(
                                        required,
                                        |_, _, _| panic!("same-command read was recaptured"),
                                        window,
                                        cx,
                                    )
                                })
                                .expect("editor")
                                .expect("same-command admission")
                                .expect("same-command capture is ready");
                            scope.run(cx, |cx| {
                                record(cx);
                                let focus = handle
                                    .read_with(cx, |editor, _| editor.focus_handle.clone())
                                    .expect("editor");
                                focus.dispatch_action(
                                    &QueuedSelectionAction(Arc::from("nested")),
                                    window,
                                    cx,
                                );
                                record(cx);
                            });
                        }
                    };
                    let admission = handle
                        .update(cx, |editor, cx| {
                            editor.defer_shared_input(required, complete.clone(), window, cx)
                        })
                        .expect("editor")
                        .expect("native admission");
                    if let Some(scope) = admission {
                        scope.run(cx, |cx| complete(Ok(()), window, cx));
                    }
                })
            });
            let (read, resolver) = ClipboardSnapshotRead::pending();
            editor_cx.update(|window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(String::from("A")));
                if delayed {
                    cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
                }
                window.draw(cx).clear(cx);
                window.focused(cx).expect("editor focus").dispatch_action(
                    &QueuedSelectionAction(Arc::from("outer")),
                    window,
                    cx,
                );
            });
            if delayed {
                editor_cx.update(|_, cx| {
                    cx.remove_global::<NativeClipboardCapture>();
                    cx.write_to_clipboard(ClipboardItem::new_string(String::from("later")));
                });
                resolver.resolve(Ok(Arc::new(ClipboardSnapshot {
                    item: Some(ClipboardItem::new_string(String::from("A"))),
                    revision: None,
                })));
            }
            editor_cx
                .update_editor(|editor, _, cx| editor.input_completion(cx))
                .await
                .expect("nested actions completed");
            assert_eq!(
                *observed.borrow(),
                ["A", "A", "B", "A"].map(|text| Some(String::from(text))),
                "delayed={delayed}",
            );
            drop(subscription);
        }
    }

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
                                    false,
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
                                if let Some(scope) = editor
                                    .defer_shared_input(
                                        NativeInput::default(),
                                        move |result, window, cx| {
                                            result.expect("queued admission");
                                            editor_handle
                                                .update(cx, |editor, cx| {
                                                    editor.paste_item(
                                                        &ClipboardItem::new_string(String::from(
                                                            "x",
                                                        )),
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .expect("editor");
                                        },
                                        window,
                                        cx,
                                    )
                                    .expect("shared admission")
                                {
                                    scope.run(cx, |cx| {
                                        editor.paste_item(
                                            &ClipboardItem::new_string(String::from("x")),
                                            window,
                                            cx,
                                        );
                                    });
                                }
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
    async fn test_unrepresentable_rows_cancel_queued_input_and_recover(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.tab_size = NonZeroU32::new(128);
        });
        let mut editor_cx = EditorTestContext::new(cx).await;
        let line = format!("{}{}", "\t".repeat(255), "x".repeat(127));
        let text = format!("A\n{line}");
        editor_cx.set_state(&format!("Aˇ\n{line}"));
        editor_cx.update(|window, cx| window.replace_root(cx, |_, _| Empty));
        let (read, resolver) = ClipboardSnapshotRead::pending();
        let (mut completion, checkpoint) = editor_cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor
                .display_map
                .update(cx, |map, cx| map.set_tab_column_limit(32_767, cx));
            let snapshot = editor.display_snapshot(cx);
            assert!(!snapshot.tab_snapshot().has_unrepresentable_rows());
            assert_eq!(snapshot.tab_snapshot().line_len(1), 32_767);
            cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
            assert!(
                editor
                    .defer_shared_input(
                        NativeInput {
                            clipboard: Some(ClipboardAccess::Read),
                            ..NativeInput::default()
                        },
                        |_, _, _| panic!("input replayed against unrepresentable geometry"),
                        window,
                        cx,
                    )
                    .expect("native input queued")
                    .is_none()
            );
            editor.replace_and_mark_text_in_range(None, "old", None, window, cx);
            editor.replace_text_in_range(None, "committed", window, cx);
            (
                editor.input_completion(cx),
                editor.pending_document_input(cx),
            )
        });
        cx.run_until_parked();
        assert!((&mut completion).now_or_never().is_none());
        editor_cx.update_editor(|editor, window, cx| {
            assert!(editor.queued_ime_composition.is_some());
            editor.finalize_last_transaction(cx);
            editor.transact(window, cx, |editor, _, cx| {
                editor.buffer.update(cx, |buffer, cx| {
                    let end = Point::new(1, line.len() as u32);
                    buffer.edit([(end..end, "x")], None, cx);
                });
            });
            assert!(
                editor
                    .display_snapshot(cx)
                    .tab_snapshot()
                    .has_unrepresentable_rows()
            );
            assert!(!editor.deferred_input.owner.cancelled.get());
        });
        resolver.resolve(Ok(Arc::new(ClipboardSnapshot {
            item: None,
            revision: None,
        })));
        for result in [completion.await, checkpoint.await] {
            assert_eq!(
                result.expect_err("invalid input cancelled").to_string(),
                "tab-expanded line exceeds the supported column range; queued input cancelled",
            );
        }
        cx.run_until_parked();
        editor_cx.update_editor(|editor, window, cx| {
            cx.remove_global::<NativeClipboardCapture>();
            assert!(editor.deferred_input.owner.cancelled.get());
            assert!(editor.queued_ime_composition.is_none());
            editor.replace_text_in_range(None, "rejected", window, cx);
            editor.replace_and_mark_text_in_range(None, "rejected", None, window, cx);
            assert_eq!(editor.text(cx), format!("{text}x"));
            assert!(editor.queued_ime_composition.is_none());
            let cancelled_owner = editor.deferred_input.owner.clone();
            editor.undo(&Undo, window, cx);
            assert_eq!(editor.text(cx), text);
            assert!(!editor.deferred_input.owner.cancelled.get());
            assert!(!Rc::ptr_eq(&cancelled_owner, &editor.deferred_input.owner));
            assert!(
                !editor
                    .display_snapshot(cx)
                    .tab_snapshot()
                    .has_unrepresentable_rows()
            );
        });
        let editor_handle = editor_cx.editor.clone();
        let recovered = editor_cx.update(|window, cx| {
            window.replace_root(cx, |_, _| MovedEditor(editor_handle.clone()));
            window.draw(cx).clear(cx);
            let focus = editor_handle.focus_handle(cx);
            focus.dispatch_action(&Redo, window, cx);
            assert_eq!(editor_handle.read(cx).text(cx), format!("{text}x"));
            focus.dispatch_action(&Delete, window, cx);
            assert_eq!(editor_handle.read(cx).text(cx), format!("{text}x"));
            assert!(editor_handle.read(cx).deferred_input.owner.cancelled.get());
            focus.dispatch_action(&Redo, window, cx);
            assert!(!editor_handle.read(cx).deferred_input.owner.cancelled.get());
            focus.dispatch_action(&Undo, window, cx);
            editor_handle.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), text);
                editor.schedule_input_continuation(window, cx, |_, _| {});
                editor.replace_and_mark_text_in_range(None, "y", None, window, cx);
                editor.replace_text_in_range(None, "z", window, cx);
                editor.input_completion(cx)
            })
        });
        recovered.await.expect("input resumed after undo");
        editor_cx.assert_editor_state(&format!("Azˇ\n{line}"));
        let cancelled = editor_cx.update_editor(|editor, window, cx| {
            editor.schedule_input_continuation(window, cx, |_, _| {
                panic!("pending input ran before recovery key");
            });
            let completion = editor.input_completion(cx);
            editor.finalize_last_transaction(cx);
            editor.transact(window, cx, |editor, _, cx| {
                editor.buffer.update(cx, |buffer, cx| {
                    let end = Point::new(1, line.len() as u32);
                    buffer.edit([(end..end, "x")], None, cx);
                });
            });
            assert!(editor.input_is_deferred(cx));
            let owner = editor.deferred_input.owner.clone();
            assert!(!editor.defer_keystroke_input(
                |_, _| panic!("recovery key was deferred"),
                window,
                cx,
            ));
            assert!(owner.cancelled.get());
            assert!(!editor.input_is_deferred(cx));
            editor.undo(&Undo, window, cx);
            assert_eq!(editor.text(cx), format!("Az\n{line}"));
            completion
        });
        assert_eq!(
            cancelled
                .await
                .expect_err("pending input cancelled")
                .to_string(),
            "tab-expanded line exceeds the supported column range; queued input cancelled",
        );
    }

    #[gpui::test]
    async fn test_native_input_queries_reject_unrepresentable_rows_before_redraw(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |settings| {
            settings.defaults.tab_size = NonZeroU32::new(128);
        });
        let mut editor_cx = EditorTestContext::new(cx).await;
        editor_cx.set_state("Aˇ");
        editor_cx.update_editor(|editor, window, cx| {
            editor.set_soft_wrap_mode(SoftWrap::None, cx);
            editor
                .display_map
                .update(cx, |map, cx| map.set_tab_column_limit(32_767, cx));
            editor.replace_and_mark_text_in_range(None, "B", None, window, cx);
        });
        let editor_handle = editor_cx.editor.clone();
        editor_cx.update(|window, cx| {
            window.draw(cx).clear(cx);
            editor_handle.update(cx, |editor, cx| {
                let element_bounds = editor.last_bounds.expect("painted editor bounds");
                let position_map = editor.last_position_map.as_ref().expect("ready frame");
                let position =
                    position_map.text_hitbox.origin + point(px(1.), position_map.line_height * 0.5);
                let original_bounds = editor
                    .bounds_for_range(0..0, element_bounds, window, cx)
                    .expect("valid native geometry");
                assert_eq!(
                    editor.character_index_for_point(position, window, cx),
                    Some(0)
                );
                assert_eq!(editor.marked_text_range(window, cx), Some(1..2));
                assert!(editor.accepts_text_input(window, cx));
                editor.finalize_last_transaction(cx);
                editor.transact(window, cx, |editor, _, cx| {
                    editor.buffer.update(cx, |buffer, cx| {
                        let end = Point::new(0, 2);
                        buffer.edit([(end..end, format!("\n{}", "\t".repeat(256)))], None, cx);
                    });
                });
                assert!(
                    editor
                        .display_snapshot(cx)
                        .tab_snapshot()
                        .has_unrepresentable_rows()
                );
                assert!(editor.last_position_map.is_some());
                assert_eq!(
                    editor.bounds_for_range(0..0, element_bounds, window, cx),
                    None
                );
                assert_eq!(editor.character_index_for_point(position, window, cx), None);
                let mut adjusted_range = None;
                assert_eq!(
                    editor.text_for_range(0..2, &mut adjusted_range, window, cx),
                    None
                );
                assert!(editor.selected_text_range(true, window, cx).is_none());
                assert_eq!(editor.marked_text_range(window, cx), None);
                assert!(!editor.accepts_text_input(window, cx));
                editor.unmark_text(window, cx);
                assert!(editor.deferred_input.owner.cancelled.get());
                assert!(editor.ime_transaction.is_none());
                editor.undo(&Undo, window, cx);
                assert_eq!(editor.text(cx), "AB");
                assert_eq!(
                    editor.bounds_for_range(0..0, element_bounds, window, cx),
                    Some(original_bounds),
                );
                assert_eq!(
                    editor.character_index_for_point(position, window, cx),
                    Some(0)
                );
                assert_eq!(
                    editor.text_for_range(0..2, &mut adjusted_range, window, cx),
                    Some(String::from("AB")),
                );
                assert_eq!(
                    editor
                        .selected_text_range(true, window, cx)
                        .map(|selection| selection.range),
                    Some(2..2),
                );
                assert_eq!(editor.marked_text_range(window, cx), None);
                assert!(editor.accepts_text_input(window, cx));
            });
        });
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
                prerequisite: InputPrerequisite::Geometry(pending),
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
    async fn test_document_input_checkpoint_excludes_later_capture_and_unrelated_failure(
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});
        let required = NativeInput {
            clipboard: Some(ClipboardAccess::Read),
            ..NativeInput::default()
        };
        for cancel_later in [false, true] {
            let mut source = EditorTestContext::new(cx).await;
            let mut unrelated = EditorTestContext::new(cx).await;
            source.update_editor(|editor, _, cx| {
                editor.buffer.clone().update(cx, |_, cx| {
                    assert!(!editor.has_pending_document_input(cx));
                });
            });
            let buffer = source
                .editor(|editor, _, cx| editor.buffer.read(cx).as_singleton().expect("singleton"));
            let alias = source.update(|window, cx| {
                let buffer = buffer.update(cx, |buffer, cx| buffer.branch(cx));
                let buffer = buffer.update(cx, |buffer, cx| buffer.branch(cx));
                let buffer = MultiBuffer::build_from_buffer(buffer, cx);
                cx.new(|cx| build_editor(buffer, window, cx))
            });
            let unrelated_handle = unrelated.editor.downgrade();
            let unrelated_window = unrelated.window;
            source.update_editor(|editor, window, cx| {
                let source = cx.weak_entity();
                editor.schedule_input_continuation(window, cx, move |window, cx| {
                    source
                        .update(cx, |editor, cx| {
                            let source = cx.weak_entity();
                            editor.schedule_input_continuation(window, cx, move |window, cx| {
                                source
                                    .update(cx, |editor, cx| editor.handle_input("a", window, cx))
                                    .expect("source editor");
                            });
                        })
                        .expect("source editor");
                    unrelated_window
                        .update(cx, |_, window, cx| {
                            unrelated_handle
                                .update(cx, |editor, cx| {
                                    editor.schedule_input_continuation(window, cx, |_, _| {});
                                })
                                .expect("causal editor");
                        })
                        .expect("causal window");
                });
            });
            let (unrelated_read, unrelated_resolver) = ClipboardSnapshotRead::pending();
            let (later_read, resolver) = ClipboardSnapshotRead::pending();
            let failure = unrelated.update_editor(|editor, window, cx| {
                let reads = RefCell::new(VecDeque::from([unrelated_read, later_read]));
                cx.set_global(NativeClipboardCapture(Rc::new(move |_| {
                    reads
                        .borrow_mut()
                        .pop_front()
                        .expect("capture in execution order")
                })));
                assert!(
                    editor
                        .defer_shared_input(
                            required,
                            |result, _, _| {
                                result.expect_err("unrelated capture failed");
                            },
                            window,
                            cx,
                        )
                        .expect("queued unrelated capture")
                        .is_none()
                );
                editor.input_completion(cx)
            });
            let alias_handle = alias.downgrade();
            let mut checkpoint = alias.update(cx, |editor, cx| {
                assert!(editor.has_pending_document_input(cx));
                editor.pending_document_input(cx)
            });
            drop(alias);
            assert!(alias_handle.upgrade().is_some());
            let mut later = source.update_editor(|editor, window, cx| {
                assert!(
                    editor
                        .defer_shared_input(
                            required,
                            |result, _, _| result.expect("later capture resolved"),
                            window,
                            cx,
                        )
                        .expect("queued later capture")
                        .is_none()
                );
                editor.input_completion(cx)
            });
            cx.run_until_parked();
            let cancelled =
                unrelated.update_editor(|editor, _, cx| editor.pending_document_input(cx));
            drop(cancelled);
            cx.run_until_parked();
            let next = unrelated.update_editor(|editor, _, cx| {
                let next = editor.pending_document_input(cx);
                assert_eq!(
                    editor
                        .deferred_input
                        .shared
                        .state
                        .borrow()
                        .active_checkpoints
                        .len(),
                    1
                );
                next
            });
            if cancel_later {
                source.update_editor(|editor, _, _| {
                    editor
                        .deferred_input
                        .shared
                        .cancel(&editor.deferred_input.owner, "later input cancelled");
                });
                cx.run_until_parked();
            }
            assert!((&mut checkpoint).now_or_never().is_none());
            unrelated_resolver.resolve(Err(ClipboardReadError::Unavailable));
            cx.run_until_parked();
            checkpoint
                .now_or_never()
                .expect("checkpoint did not wait for later input")
                .expect("unrelated failure did not contaminate checkpoint");
            assert_eq!(source.editor(|editor, _, cx| editor.text(cx)), "a");
            assert!(alias_handle.upgrade().is_none());
            if cancel_later {
                assert_eq!(
                    later.await.expect_err("later input cancelled").to_string(),
                    "later input cancelled"
                );
                drop(resolver);
            } else {
                assert!((&mut later).now_or_never().is_none());
                resolver.resolve(Ok(Arc::new(ClipboardSnapshot {
                    item: None,
                    revision: None,
                })));
                later.await.expect("later input completed");
            }
            failure.await.expect_err("unrelated capture failed");
            next.await
                .expect_err("next checkpoint includes the failed capture");
        }

        for scenario in [
            "queued",
            "after_effects",
            "ready_paste",
            "ready_shared",
            "cancel_child",
            "cancel_marker",
            "prior_failure",
        ] {
            let mut source = EditorTestContext::new(cx).await;
            let target = EditorTestContext::new(cx).await;
            let buffer = source.editor(|editor, _, _| editor.buffer.clone());
            let alias = source.update(|window, cx| cx.new(|cx| build_editor(buffer, window, cx)));
            let alias_handle = alias.downgrade();
            let target_handle = target.editor.downgrade();
            let (read, resolver) = ClipboardSnapshotRead::pending();
            let (sender, receiver) = oneshot::channel();
            let completion = source.update_editor(|editor, window, cx| {
                cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
                let source = cx.weak_entity();
                editor.schedule_input_continuation(window, cx, move |window, cx| {
                    if scenario == "prior_failure" {
                        resolver.resolve(Err(ClipboardReadError::Unavailable));
                        source
                            .update(cx, |editor, cx| {
                                assert!(
                                    editor
                                        .defer_shared_input(
                                            required,
                                            |_, _, _| panic!("ready failure queued"),
                                            window,
                                            cx,
                                        )
                                        .is_err()
                                );
                            })
                            .expect("source editor");
                        let checkpoint = alias_handle
                            .update(cx, |editor, cx| editor.pending_document_input(cx))
                            .expect("checkpoint editor");
                        assert!(sender.send(checkpoint).is_ok());
                        return;
                    }
                    target_handle
                        .update(cx, |editor, cx| {
                            let input = CapturedInput {
                                clipboard: Some(editor.capture_clipboard(cx)),
                                ..CapturedInput::default()
                            };
                            let target = cx.weak_entity();
                            let ready = scenario == "ready_paste" || scenario == "ready_shared";
                            let operation = DeferredInputOperation {
                                geometry: Vec::new(),
                                prerequisite: if ready {
                                    InputPrerequisite::None
                                } else {
                                    InputPrerequisite::Native(required)
                                },
                                input: Some(input),
                                continuation: Box::new(move |result, window, cx| {
                                    if !ready {
                                        result.expect_err("causal capture failed");
                                        return;
                                    }
                                    result.expect("optional capture replayed");
                                    target
                                        .update(cx, |editor, cx| {
                                            if scenario == "ready_paste" {
                                                editor.paste(&Paste, window, cx);
                                            } else {
                                                assert!(
                                                    editor
                                                        .defer_shared_input(
                                                            required,
                                                            |_, _, _| panic!(
                                                                "ready failure queued"
                                                            ),
                                                            window,
                                                            cx,
                                                        )
                                                        .is_err()
                                                );
                                            }
                                        })
                                        .expect("target editor");
                                }),
                            };
                            editor.deferred_input.shared.enqueue(
                                QueuedInput {
                                    owner: editor.deferred_input.owner.clone(),
                                    checkpoints: Vec::new(),
                                    publication: false,
                                    operation,
                                },
                                scenario == "after_effects",
                                false,
                            );
                        })
                        .expect("target editor");
                    let checkpoint = alias_handle
                        .update(cx, |editor, cx| editor.pending_document_input(cx))
                        .expect("checkpoint editor");
                    assert!(sender.send(checkpoint).is_ok());
                    let cancelled = match scenario {
                        "cancel_child" => Some(&target_handle),
                        "cancel_marker" => Some(&alias_handle),
                        _ => None,
                    };
                    if let Some(cancelled) = cancelled {
                        cancelled
                            .update(cx, |editor, _| {
                                editor
                                    .deferred_input
                                    .shared
                                    .cancel(&editor.deferred_input.owner, scenario);
                            })
                            .expect("cancelled editor");
                    }
                    resolver.resolve(Err(ClipboardReadError::Unavailable));
                });
                editor.input_completion(cx)
            });
            let checkpoint = receiver.await.expect("checkpoint registered during replay");
            let error = checkpoint.await.expect_err(scenario);
            if scenario == "prior_failure" {
                assert_eq!(
                    completion
                        .await
                        .expect_err("preceding failure retained")
                        .to_string(),
                    error.to_string()
                );
            } else {
                completion
                    .await
                    .expect("child failure does not poison the root owner");
            }
            if scenario == "cancel_child" || scenario == "cancel_marker" {
                assert_eq!(error.to_string(), scenario);
            }
            cx.run_until_parked();
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
                    prerequisite: pending
                        .map_or(InputPrerequisite::None, InputPrerequisite::Geometry),
                    input: Some(editor.capture_input(None, cx)),
                    continuation: Box::new(move |_, _, _| {
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
            assert!(editor.input_is_deferred(cx));
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
                    assert!(CapturedInput::current(cx).is_none());
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
                                assert!(CapturedInput::current(cx).is_none());
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
                    prerequisite: InputPrerequisite::Geometry(pending),
                    input: Some(editor.capture_input(None, cx)),
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
                assert!(!editor.input_is_deferred(cx));
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
                    Some(String::from("prepared"))
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
            assert!(editor.input_is_deferred(cx));
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
            assert!(!editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
                editor.input_completion(cx)
            });
            editor_cx.run_until_parked();
            if cancel {
                editor_cx.update_editor(|editor, _, cx| {
                    assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
                        assert!(editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
    async fn test_shared_document_input_orders_dispatched_undo(cx: &mut TestAppContext) {
        init_test(cx, |_| {});
        let mut editor_cx = EditorTestContext::new(cx).await;
        let source = editor_cx.editor.clone();
        let target =
            editor_cx.update_editor(|editor, window, cx| cx.new(|cx| editor.clone(window, cx)));
        let (read, resolver) = ClipboardSnapshotRead::pending();
        let completions = editor_cx.update(|window, cx| {
            cx.set_global(NativeClipboardCapture(Rc::new(move |_| read.clone())));
            cx.bind_keys([
                KeyBinding::new("ctrl-v", Paste, Some("Editor")),
                KeyBinding::new("ctrl-z", Undo, Some("Editor")),
            ]);
            window.draw(cx).clear(cx);
            assert!(window.dispatch_keystroke(Keystroke::parse("ctrl-v").expect("paste key"), cx));
            assert!(source.read(cx).input_is_deferred(cx));
            assert_eq!(source.read(cx).text(cx), "");
            let buffer = source
                .read(cx)
                .buffer
                .read(cx)
                .as_singleton()
                .expect("source buffer");
            let branch = buffer.update(cx, |buffer, cx| buffer.branch(cx));
            let branch = branch.update(cx, |buffer, cx| buffer.branch(cx));
            let buffer = MultiBuffer::build_from_buffer(branch, cx);
            let alias = cx.new(|cx| build_editor(buffer, window, cx));
            assert!(alias.read(cx).input_is_deferred(cx));
            let unrelated = cx.new(|cx| Editor::single_line(window, cx));
            unrelated.update(cx, |editor, cx| {
                editor.replace_text_in_range(None, "independent", window, cx);
                assert_eq!(editor.text(cx), "independent");
                assert!(!editor.input_is_deferred(cx));
            });
            window.replace_root(cx, |_, _| MovedEditor(target.clone()));
            window.focus(&target.focus_handle(cx), cx);
            window.draw(cx).clear(cx);
            assert!(window.dispatch_keystroke(Keystroke::parse("ctrl-z").expect("undo key"), cx));
            [source, target]
                .map(|editor| editor.update(cx, |editor, cx| editor.input_completion(cx)))
        });
        resolver.resolve(Ok(Arc::new(ClipboardSnapshot {
            item: Some(ClipboardItem::new_string(String::from("X"))),
            revision: None,
        })));
        for completion in completions {
            completion.await.expect("document input completed");
        }
        assert_eq!(editor_cx.buffer_text(), "");
    }

    #[gpui::test]
    async fn test_deferred_ime_applied_ranges_survive_input_transforms(cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.auto_indent = Some(settings::AutoIndentMode::SyntaxAware);
        });
        for (case, state, inputs, lengths, expected) in [
            ("surround", "«abcˇ»", &["("][..], (1, 1), "Xabc)"),
            ("surround", "«ˇ😀é»", &["("][..], (1, 1), "X😀é)"),
            ("surround", "«aˇ»«bˇ»", &["("][..], (1, 1), "Xa)Xb)"),
            (
                "surround",
                "«abcˇ»\n«defˇ»",
                &["("][..],
                (1, 1),
                "Xabc)\nXdef)",
            ),
            (
                "surround",
                "«abcˇ»\n«ˇ😀é»",
                &["("][..],
                (1, 1),
                "Xabc)\nX😀é)",
            ),
            ("adjacent", "«aˇ»«bˇ»", &["xy"][..], (1, 1), "xXxX"),
            ("emoji", ":waveˇ", &[":"][..], (2, 1), "X"),
            ("autoclose", "ˇ", &["(", ")"][..], (1, 1), "(X"),
            (
                "autoindent",
                "fn f() {\n«    let a = 1;\n    let b = 2;ˇ»\n}\nfn g() {\n«    let a = 1;\n    let b = 2;ˇ»\n}",
                &["foo"][..],
                (1, 1),
                "fn f() {\n    foX\n}\nfn g() {\n    foX\n}",
            ),
        ] {
            for (deferred, marking) in [(false, false), (true, false), (false, true), (true, true)]
            {
                let mut editor_cx = EditorTestContext::new(cx).await;
                let language = if case == "autoindent" {
                    Arc::new(
                        Language::new(
                            LanguageConfig::default(),
                            Some(tree_sitter_rust::LANGUAGE.into()),
                        )
                        .with_indents_query(r#"(_ "{" "}" @end) @indent"#)
                        .expect("Rust indent query"),
                    )
                } else {
                    PLAIN_TEXT.clone()
                };
                editor_cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
                editor_cx.set_state(state);
                editor_cx
                    .condition(|editor, cx| !editor.buffer.read(cx).is_parsing(cx))
                    .await;
                editor_cx
                    .update_editor(|editor, _, _| editor.set_auto_replace_emoji_shortcode(true));
                let editor = editor_cx.editor.clone();
                let (read, resolver) = ClipboardSnapshotRead::pending();
                let captures = Rc::new(Cell::new(0));
                let (ids, completion) = editor_cx.update(|window, cx| {
                    let ids = editor
                        .read(cx)
                        .selections
                        .disjoint_anchors()
                        .iter()
                        .map(|selection| selection.id)
                        .collect::<Vec<_>>();
                    cx.set_global(NativeClipboardCapture(Rc::new({
                        let captures = captures.clone();
                        move |_| {
                            captures.set(captures.get() + 1);
                            read.clone()
                        }
                    })));
                    window.draw(cx).clear(cx);
                    if deferred {
                        editor.focus_handle(cx).dispatch_action(&Paste, window, cx);
                    }
                    assert_eq!(editor.read(cx).input_is_deferred(cx), deferred);
                    let captures_before_native_input = captures.get();
                    let mut input = ElementInputHandler::new(Bounds::default(), editor.clone());
                    for text in inputs {
                        input.replace_text_in_range(None, text, window, cx);
                    }
                    let selection = input
                        .selected_text_range(false, window, cx)
                        .expect("input selection");
                    let end = if case == "surround" {
                        selection.range.start
                    } else {
                        selection.range.end
                    };
                    let length = if deferred { lengths.1 } else { lengths.0 };
                    let range = Some(end - length..end);
                    if marking {
                        input.replace_and_mark_text_in_range(range, "X", Some(0..0), window, cx);
                        input.replace_text_in_range(None, "X", window, cx);
                    } else {
                        input.replace_text_in_range(range, "X", window, cx);
                    }
                    assert_eq!(captures.get(), captures_before_native_input);
                    (
                        ids,
                        editor.update(cx, |editor, cx| editor.input_completion(cx)),
                    )
                });
                resolver.resolve(Ok(Arc::new(ClipboardSnapshot::default())));
                completion.await.expect("transformed input completed");
                editor_cx.wait_for_autoindent_applied().await;
                assert_eq!(
                    editor_cx.buffer_text(),
                    expected,
                    "case={case}, state={state}, deferred={deferred}, marking={marking}"
                );
                editor_cx.update_editor(|editor, _, _| {
                    assert_eq!(
                        editor
                            .selections
                            .disjoint_anchors()
                            .iter()
                            .map(|selection| selection.id)
                            .collect::<Vec<_>>(),
                        ids
                    );
                });
            }
        }
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
                assert!(editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
            assert!(editor.input_is_deferred(cx));
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
                assert!(editor.input_is_deferred(cx));
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
                        assert_eq!(editor.input_is_deferred(cx), deferred);
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
                assert_eq!(editor.input_is_deferred(cx), deferred);
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
                    assert_eq!(editor.input_is_deferred(cx), deferred);
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
                assert!(editor.input_is_deferred(cx));
                (!deferred).then(|| editor.input_completion(cx))
            });
            if let Some(movement) = movement {
                cx.background_executor.set_block_on_ticks(0..=50);
                movement.await.expect("marked reference movement completed");
            }
            let completion = cx.update_editor(|editor, window, cx| {
                assert_eq!(editor.input_is_deferred(cx), deferred);
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
            assert!(editor.input_is_deferred(cx));
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
