use gpui::{Action, Context, Div, Entity, InteractiveElement, Window, div};
use workspace::Workspace;

use crate::BufferSearchBar;

/// Registrar inverts the dependency between search and its downstream user, allowing said downstream user to register search action without knowing exactly what those actions are.
pub trait SearchActionsRegistrar {
    fn register_handler<A: Action>(&mut self, callback: impl ActionExecutor<A>);
}

type SearchBarActionCallback<A> =
    fn(&mut BufferSearchBar, &A, &mut Window, &mut Context<BufferSearchBar>);

type GetSearchBar<T> =
    for<'a, 'b> fn(&'a T, &'a mut Window, &mut Context<'b, T>) -> Option<Entity<BufferSearchBar>>;

/// Registers search actions on a div that can be taken out.
pub struct DivRegistrar<'a, 'b, T: 'static> {
    div: Option<Div>,
    cx: &'a mut Context<'b, T>,
    search_getter: GetSearchBar<T>,
}

impl<'a, 'b, T: 'static> DivRegistrar<'a, 'b, T> {
    pub fn new(search_getter: GetSearchBar<T>, cx: &'a mut Context<'b, T>) -> Self {
        Self {
            div: Some(div()),
            cx,
            search_getter,
        }
    }
    pub fn into_div(self) -> Div {
        // This option is always Some; it's an option in the first place because we want to call methods
        // on div that require ownership.
        self.div.unwrap()
    }
}

impl<T: 'static> SearchActionsRegistrar for DivRegistrar<'_, '_, T> {
    fn register_handler<A: Action>(&mut self, callback: impl ActionExecutor<A>) {
        let getter = self.search_getter;
        self.div = self.div.take().map(|div| {
            div.on_action(self.cx.listener(move |this, action, window, cx| {
                let should_notify = (getter)(this, window, cx)
                    .map(|search_bar| {
                        search_bar.update(cx, |search_bar, cx| {
                            callback.execute(search_bar, action, window, cx)
                        })
                    })
                    .unwrap_or(false);
                if should_notify {
                    cx.notify();
                } else {
                    cx.propagate();
                }
            }))
        });
    }
}

/// Register actions for an active pane.
impl SearchActionsRegistrar for Workspace {
    fn register_handler<A: Action>(&mut self, callback: impl ActionExecutor<A>) {
        self.register_action(move |workspace, action: &A, window, cx| {
            if workspace.has_active_modal(window, cx) {
                cx.propagate();
                return;
            }

            let pane = workspace.active_pane();
            let callback = callback.clone();
            pane.update(cx, |this, cx| {
                this.toolbar().update(cx, move |this, cx| {
                    if let Some(search_bar) = this.item_of_type::<BufferSearchBar>() {
                        let should_notify = search_bar.update(cx, move |search_bar, cx| {
                            callback.execute(search_bar, action, window, cx)
                        });
                        if should_notify {
                            cx.notify();
                        } else {
                            cx.propagate();
                        }
                    }
                })
            });
        });
    }
}

type DidHandleAction = bool;
/// Potentially executes the underlying action if some preconditions are met (e.g. buffer search bar is visible)
pub trait ActionExecutor<A: Action>: 'static + Clone {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction;
}

/// Run an action when the search bar has been dismissed from the panel.
pub struct ForDismissed<A>(pub(super) SearchBarActionCallback<A>);
impl<A> Clone for ForDismissed<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A: Action> ActionExecutor<A> for ForDismissed<A> {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction {
        if search_bar.is_dismissed() {
            self.0(search_bar, action, window, cx);
            true
        } else {
            false
        }
    }
}

/// Run an action when the search bar is deployed.
pub struct ForDeployed<A>(pub(super) SearchBarActionCallback<A>);
impl<A> Clone for ForDeployed<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A: Action> ActionExecutor<A> for ForDeployed<A> {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction {
        if search_bar.is_dismissed() || search_bar.active_searchable_item.is_none() {
            false
        } else {
            self.0(search_bar, action, window, cx);
            true
        }
    }
}

/// Run an action when the search bar has any matches, regardless of whether it
/// is visible or not.
pub struct WithResults<A>(pub(super) SearchBarActionCallback<A>);
impl<A> Clone for WithResults<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A: Action> ActionExecutor<A> for WithResults<A> {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction {
        if search_bar.active_match_index.is_some() {
            self.0(search_bar, action, window, cx);
            true
        } else {
            false
        }
    }
}
