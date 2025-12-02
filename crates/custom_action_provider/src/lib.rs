#![allow(dead_code)]

mod extension_custom_action;
mod registry;

use std::sync::Arc;

use extension::Extension;
use gpui::{Action, App};
use parking_lot::RwLock;
pub use registry::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, schemars::JsonSchema, Action)]
#[action(namespace = zed)]
pub struct ExtensionAction {
    pub extension_id: String,
    pub action: String,
    pub args: Vec<String>,
}

pub fn init(cx: &mut App) {
    extension_custom_action::init(cx);
}

/// A custom action provided by an extension
#[derive(Clone)]
pub struct CustomAction {
    /// The extension that provides this action
    pub extension: Arc<dyn Extension>,
    /// The name of the custom action
    pub name: String,
    /// The description of the custom action
    pub description: String,
}

pub struct CustomActionProvider {
    actions: RwLock<Vec<CustomAction>>,
}

impl CustomActionProvider {
    pub fn new() -> Self {
        Self {
            actions: RwLock::new(Vec::new()),
        }
    }

    /// Register a custom action
    pub fn register(&self, action: CustomAction) {
        self.actions.write().push(action);
    }

    /// Unregister a custom action by extension ID and action name
    pub fn unregister(&self, extension_id: &str, action_name: &str) {
        let mut actions = self.actions.write();
        actions.retain(|a| {
            a.extension.manifest().id.as_ref() != extension_id || a.name != action_name
        });
    }

    /// Get a custom action by extension ID and action name
    pub fn get(&self, extension_id: &str, action_name: &str) -> Option<CustomAction> {
        self.actions
            .read()
            .iter()
            .find(|a| a.extension.manifest().id.as_ref() == extension_id && a.name == action_name)
            .cloned()
    }

    /// Get all registered custom actions
    pub fn all_actions(&self) -> Vec<CustomAction> {
        self.actions.read().clone()
    }
}

impl Default for CustomActionProvider {
    fn default() -> Self {
        Self::new()
    }
}
