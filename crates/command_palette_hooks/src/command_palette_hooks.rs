//! Provides hooks for customizing the behavior of the command palette.

#![deny(missing_docs)]

use std::{any::TypeId, rc::Rc};

use collections::HashSet;
use derive_more::{Deref, DerefMut};
use gpui::{Action, App, BorrowAppContext, Global, Task, WeakEntity};
use workspace::Workspace;

/// Initializes the command palette hooks.
pub fn init(cx: &mut App) {
    cx.set_global(GlobalCommandPaletteFilter::default());
}

/// A filter for the command palette.
#[derive(Default)]
pub struct CommandPaletteFilter {
    hidden_namespaces: HashSet<&'static str>,
    hidden_action_types: HashSet<TypeId>,
    /// Actions that have explicitly been shown. These should be shown even if
    /// they are in a hidden namespace.
    shown_action_types: HashSet<TypeId>,
}

#[derive(Deref, DerefMut, Default)]
struct GlobalCommandPaletteFilter(CommandPaletteFilter);

impl Global for GlobalCommandPaletteFilter {}

impl CommandPaletteFilter {
    /// Returns the global [`CommandPaletteFilter`], if one is set.
    pub fn try_global(cx: &App) -> Option<&CommandPaletteFilter> {
        cx.try_global::<GlobalCommandPaletteFilter>()
            .map(|filter| &filter.0)
    }

    /// Returns a mutable reference to the global [`CommandPaletteFilter`].
    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<GlobalCommandPaletteFilter>()
    }

    /// Updates the global [`CommandPaletteFilter`] using the given closure.
    pub fn update_global<F>(cx: &mut App, update: F)
    where
        F: FnOnce(&mut Self, &mut App),
    {
        if cx.has_global::<GlobalCommandPaletteFilter>() {
            cx.update_global(|this: &mut GlobalCommandPaletteFilter, cx| update(&mut this.0, cx))
        }
    }

    /// Returns whether the given [`Action`] is hidden by the filter.
    pub fn is_hidden(&self, action: &dyn Action) -> bool {
        let name = action.name();
        let namespace = name.split("::").next().unwrap_or("malformed action name");

        // If this action has specifically been shown then it should be visible.
        if self.shown_action_types.contains(&action.type_id()) {
            return false;
        }

        self.hidden_namespaces.contains(namespace)
            || self.hidden_action_types.contains(&action.type_id())
    }

    /// Hides all actions in the given namespace.
    pub fn hide_namespace(&mut self, namespace: &'static str) {
        self.hidden_namespaces.insert(namespace);
    }

    /// Shows all actions in the given namespace.
    pub fn show_namespace(&mut self, namespace: &'static str) {
        self.hidden_namespaces.remove(namespace);
    }

    /// Hides all actions with the given types.
    pub fn hide_action_types<'a>(&mut self, action_types: impl IntoIterator<Item = &'a TypeId>) {
        for action_type in action_types {
            self.hidden_action_types.insert(*action_type);
            self.shown_action_types.remove(action_type);
        }
    }

    /// Shows all actions with the given types.
    pub fn show_action_types<'a>(&mut self, action_types: impl IntoIterator<Item = &'a TypeId>) {
        for action_type in action_types {
            self.shown_action_types.insert(*action_type);
            self.hidden_action_types.remove(action_type);
        }
    }
}

/// The result of intercepting a command palette command.
#[derive(Debug)]
pub struct CommandInterceptItem {
    /// The action produced as a result of the interception.
    pub action: Box<dyn Action>,
    /// The display string to show in the command palette for this result.
    pub string: String,
    /// The character positions in the string that match the query.
    /// Used for highlighting matched characters in the command palette UI.
    pub positions: Vec<usize>,
}

/// The result of intercepting a command palette command.
#[derive(Default, Debug)]
pub struct CommandInterceptResult {
    /// The items
    pub results: Vec<CommandInterceptItem>,
    /// Whether or not to continue to show the normal matches
    pub exclusive: bool,
}

type InterceptorFn =
    Rc<dyn Fn(&str, WeakEntity<Workspace>, &mut App) -> Task<CommandInterceptResult>>;

/// Holds all registered command palette interceptors, keyed by [`TypeId`].
///
/// Multiple interceptors compose: their results are merged in registration order.
#[derive(Default)]
struct GlobalCommandPaletteInterceptors {
    interceptors: Vec<(TypeId, InterceptorFn)>,
}

impl Global for GlobalCommandPaletteInterceptors {}

/// Namespace for registering and invoking command palette interceptors.
///
/// Multiple interceptors can be registered simultaneously using different key
/// types `K`. Their results are merged when the command palette queries them,
/// so registering a new interceptor never silently discards an existing one.
pub struct GlobalCommandPaletteInterceptor;

impl GlobalCommandPaletteInterceptor {
    /// Registers (or replaces) the interceptor associated with key `K`.
    ///
    /// If no interceptor for `K` exists yet it is appended; otherwise the
    /// existing entry for `K` is updated in place. Interceptors registered
    /// under different keys are unaffected and will continue to run.
    pub fn set<K: 'static>(
        cx: &mut App,
        interceptor: impl Fn(&str, WeakEntity<Workspace>, &mut App) -> Task<CommandInterceptResult>
        + 'static,
    ) {
        let key = TypeId::of::<K>();
        let handler: InterceptorFn = Rc::new(interceptor);
        let global = cx.default_global::<GlobalCommandPaletteInterceptors>();
        if let Some(entry) = global.interceptors.iter_mut().find(|(id, _)| *id == key) {
            entry.1 = handler;
        } else {
            global.interceptors.push((key, handler));
        }
    }

    /// Removes the interceptor registered under key `K`, if any.
    ///
    /// Interceptors registered under other keys are unaffected.
    pub fn clear<K: 'static>(cx: &mut App) {
        let key = TypeId::of::<K>();
        if cx.has_global::<GlobalCommandPaletteInterceptors>() {
            cx.global_mut::<GlobalCommandPaletteInterceptors>()
                .interceptors
                .retain(|(id, _)| *id != key);
        }
    }

    /// Runs all registered interceptors against `query` and merges their results.
    ///
    /// Returns `None` when no interceptors are registered.
    pub fn intercept(
        query: &str,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> Option<Task<CommandInterceptResult>> {
        let handlers: Vec<InterceptorFn> = cx
            .try_global::<GlobalCommandPaletteInterceptors>()?
            .interceptors
            .iter()
            .map(|(_, handler)| handler.clone())
            .collect();

        if handlers.is_empty() {
            return None;
        }

        let tasks: Vec<Task<CommandInterceptResult>> = handlers
            .iter()
            .map(|handler| handler(query, workspace.clone(), cx))
            .collect();

        Some(cx.foreground_executor().spawn(async move {
            let mut merged = CommandInterceptResult::default();
            for task in tasks {
                let result = task.await;
                merged.results.extend(result.results);
                if result.exclusive {
                    merged.exclusive = true;
                }
            }
            merged
        }))
    }
}
