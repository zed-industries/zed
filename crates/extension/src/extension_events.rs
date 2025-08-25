use std::sync::Arc;

use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global};

use crate::ExtensionManifest;

pub fn init(cx: &mut App) {
    let extension_events = cx.new(ExtensionEvents::new);
    cx.set_global(GlobalExtensionEvents(extension_events));
}

struct GlobalExtensionEvents(Entity<ExtensionEvents>);

impl Global for GlobalExtensionEvents {}

/// An event bus for broadcasting extension-related events throughout the app.
pub struct ExtensionEvents;

impl ExtensionEvents {
    /// Returns the global [`ExtensionEvents`].
    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalExtensionEvents>()
            .map(|g| g.0.clone())
    }

    fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }

    pub fn emit(&mut self, event: Event, cx: &mut Context<Self>) {
        cx.emit(event)
    }
}

#[derive(Clone)]
pub enum Event {
    ExtensionInstalled(Arc<ExtensionManifest>),
    ExtensionUninstalled(Arc<ExtensionManifest>),
    ExtensionsInstalledChanged,
    ConfigureExtensionRequested(Arc<ExtensionManifest>),
}

impl EventEmitter<Event> for ExtensionEvents {}
