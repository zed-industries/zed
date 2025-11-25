use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use multi_buffer::{MultiBuffer, MultiBufferFilterMode};
use ui::{
    App, Context, InteractiveElement, IntoElement as _, ParentElement as _, Render, SharedString,
    Window, div,
};
use workspace::{
    ActivePaneDecorator, Item, ItemHandle as _, Pane, PaneGroup, SplitDirection, Workspace,
};

use crate::{Editor, EditorEvent};

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
pub(crate) struct SplitDiff;

#[derive(Clone, Copy, PartialEq, Eq, Action)]
#[action(namespace = editor)]
pub(crate) struct UnsplitDiff;

pub(crate) struct SplittableEditor {
    primary: Entity<Editor>,
    secondary: Option<(Entity<Editor>, Entity<Pane>)>,
    panes: PaneGroup,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl SplittableEditor {
    pub(crate) fn new_unsplit(
        buffer: Entity<MultiBuffer>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = workspace.read(cx).project().clone();
        let primary =
            cx.new(|cx| Editor::for_multibuffer(buffer, Some(project.clone()), window, cx));
        let pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.downgrade(),
                project,
                Default::default(),
                None,
                NoAction.boxed_clone(),
                true,
                window,
                cx,
            );
            pane.set_should_display_tab_bar(|_, _| false);
            pane.add_item(primary.boxed_clone(), true, true, None, window, cx);
            pane
        });
        let panes = PaneGroup::new(pane);
        let subscriptions = vec![cx.subscribe(&primary, |_, _, event: &EditorEvent, cx| {
            cx.emit(event.clone())
        })];
        Self {
            primary,
            secondary: None,
            panes,
            workspace: workspace.downgrade(),
            _subscriptions: subscriptions,
        }
    }

    pub(crate) fn split(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.secondary.is_some() {
            return;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let follower = self.primary.update(cx, |primary, cx| {
            primary.buffer().update(cx, |buffer, cx| {
                let follower = buffer.get_or_create_follower(cx);
                buffer.set_all_diff_hunks_expanded(cx);
                buffer.set_filter_mode(Some(MultiBufferFilterMode::KeepInsertions));
                follower
            })
        });
        follower.update(cx, |follower, cx| {
            follower.set_all_diff_hunks_expanded(cx);
            follower.set_filter_mode(Some(MultiBufferFilterMode::KeepDeletions));
            // FIXME set readonly here too?
        });
        let secondary = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                follower,
                Some(workspace.read(cx).project().clone()),
                window,
                cx,
            );
            editor.set_use_base_text_line_numbers(true, cx);
            editor
        });
        let secondary_pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.downgrade(),
                workspace.read(cx).project().clone(),
                Default::default(),
                None,
                NoAction.boxed_clone(),
                true,
                window,
                cx,
            );
            pane.set_should_display_tab_bar(|_, _| false);
            pane.add_item(secondary.boxed_clone(), false, false, None, window, cx);
            pane
        });
        self.secondary = Some((secondary.clone(), secondary_pane.clone()));
        let primary_pane = self.panes.first_pane();
        self.panes
            .split(&primary_pane, &secondary_pane, SplitDirection::Left)
            .unwrap();
        cx.notify();
    }

    pub(crate) fn unsplit(&mut self, cx: &mut Context<Self>) {
        let Some((_, secondary_pane)) = self.secondary.take() else {
            return;
        };
        self.panes.remove(&secondary_pane).unwrap();
        cx.notify();
    }
}

impl EventEmitter<EditorEvent> for SplittableEditor {}
impl Focusable for SplittableEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.primary.read(cx).focus_handle(cx)
    }
}

impl Render for SplittableEditor {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let Some(active) = self.panes.panes().into_iter().next() else {
            return div().into_any_element();
        };
        self.panes
            .render(
                None,
                &ActivePaneDecorator::new(active, &self.workspace),
                window,
                cx,
            )
            .into_any_element()
    }
}

impl Item for SplittableEditor {
    type Event = EditorEvent;

    fn tab_content_text(&self, detail: usize, cx: &App) -> SharedString {
        self.primary.read(cx).tab_content_text(detail, cx)
    }
}
