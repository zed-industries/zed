//! Provides hooks for customizing the behavior of the command palette.

#![deny(missing_docs)]

use std::any::TypeId;

use collections::HashSet;
use derive_more::{Deref, DerefMut};
use gpui::{Action, App, BorrowAppContext, Global};

/// Initializes the command palette hooks.
pub fn init(cx: &mut App) {
    cx.set_global(GlobalCommandPaletteFilter::default());
    cx.set_global(GlobalCommandPaletteInterceptor::default());
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
pub struct CommandInterceptResult {
    /// The action produced as a result of the interception.
    pub action: Box<dyn Action>,
    /// The display string to show in the command palette for this result.
    pub string: String,
    /// The character positions in the string that match the query.
    /// Used for highlighting matched characters in the command palette UI.
    pub positions: Vec<usize>,
}

/// An interceptor for the command palette.
#[derive(Default)]
pub struct CommandPaletteInterceptor(
    Option<Box<dyn Fn(&str, &App) -> Vec<CommandInterceptResult>>>,
);

#[derive(Default)]
struct GlobalCommandPaletteInterceptor(CommandPaletteInterceptor);

impl Global for GlobalCommandPaletteInterceptor {}

impl CommandPaletteInterceptor {
    /// Returns the global [`CommandPaletteInterceptor`], if one is set.
    pub fn try_global(cx: &App) -> Option<&CommandPaletteInterceptor> {
        cx.try_global::<GlobalCommandPaletteInterceptor>()
            .map(|interceptor| &interceptor.0)
    }

    /// Updates the global [`CommandPaletteInterceptor`] using the given closure.
    pub fn update_global<F, R>(cx: &mut App, update: F) -> R
    where
        F: FnOnce(&mut Self, &mut App) -> R,
    {
        cx.update_global(|this: &mut GlobalCommandPaletteInterceptor, cx| update(&mut this.0, cx))
    }

    /// Intercepts the given query from the command palette.
    pub fn intercept(&self, query: &str, cx: &App) -> Vec<CommandInterceptResult> {
        if let Some(handler) = self.0.as_ref() {
            (handler)(query, cx)
        } else {
            Vec::new()
        }
    }

    /// Clears the global interceptor.
    pub fn clear(&mut self) {
        self.0 = None;
    }

    /// Sets the global interceptor.
    ///
    /// This will override the previous interceptor, if it exists.
    pub fn set(&mut self, handler: Box<dyn Fn(&str, &App) -> Vec<CommandInterceptResult>>) {
        self.0 = Some(handler);
    }
}
