use std::rc::Rc;

use gpui::{
    AnyView, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Global,
    IntoElement, ManagedView, Render, Subscription, WeakEntity, Window,
};
use project::{ProjectPath, git_store::Repository};
use workspace::Workspace;

pub mod askpass_modal;
pub mod created_worktrees;
pub mod file_diff_view;
pub mod notifications;
pub mod worktree_names;
pub mod worktree_picker;
pub mod worktree_service;

/// A type-erased picker view, e.g. for the title bar's branch popover.
///
/// The concrete picker lives in a higher-level crate, which installs a
/// builder via [`set_branch_picker_builder`]; keeping only this wrapper here
/// lets consumers avoid depending on that crate.
pub struct GitPickerPopover {
    view: AnyView,
    focus_handle: FocusHandle,
    _dismiss_subscription: Subscription,
}

impl GitPickerPopover {
    pub fn new<V: ManagedView>(view: Entity<V>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: view.focus_handle(cx),
            _dismiss_subscription: cx
                .subscribe(&view, |_, _, _: &DismissEvent, cx| cx.emit(DismissEvent)),
            view: view.into(),
        }
    }
}

impl EventEmitter<DismissEvent> for GitPickerPopover {}

impl Focusable for GitPickerPopover {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for GitPickerPopover {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.view.clone()
    }
}

type BranchPickerBuilder = dyn Fn(
    WeakEntity<Workspace>,
    Option<Entity<Repository>>,
    &mut Window,
    &mut App,
) -> Entity<GitPickerPopover>;

struct BranchPickerBuilderGlobal(Rc<BranchPickerBuilder>);

impl Global for BranchPickerBuilderGlobal {}

pub fn set_branch_picker_builder(
    builder: impl Fn(
        WeakEntity<Workspace>,
        Option<Entity<Repository>>,
        &mut Window,
        &mut App,
    ) -> Entity<GitPickerPopover>
    + 'static,
    cx: &mut App,
) {
    cx.set_global(BranchPickerBuilderGlobal(Rc::new(builder)));
}

pub fn build_branch_picker(
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    window: &mut Window,
    cx: &mut App,
) -> Option<Entity<GitPickerPopover>> {
    let builder = cx.try_global::<BranchPickerBuilderGlobal>()?.0.clone();
    Some(builder(workspace, repository, window, cx))
}

type FileHistoryOpener = dyn Fn(&mut Workspace, &ProjectPath, &mut Window, &mut Context<Workspace>);

struct FileHistoryOpenerGlobal(Rc<FileHistoryOpener>);

impl Global for FileHistoryOpenerGlobal {}

pub fn set_file_history_opener(
    opener: impl Fn(&mut Workspace, &ProjectPath, &mut Window, &mut Context<Workspace>) + 'static,
    cx: &mut App,
) {
    cx.set_global(FileHistoryOpenerGlobal(Rc::new(opener)));
}

/// Opens the file history view for `path`, if an opener was installed via
/// [`set_file_history_opener`].
pub fn open_file_history(
    workspace: &mut Workspace,
    path: &ProjectPath,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(opener) = cx
        .try_global::<FileHistoryOpenerGlobal>()
        .map(|opener| opener.0.clone())
    else {
        return;
    };
    opener(workspace, path, window, cx);
}
