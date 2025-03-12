use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global, ReadGlobal as _};

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
    pub fn global(cx: &App) -> Entity<Self> {
        GlobalExtensionEvents::global(cx).0.clone()
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
    ExtensionsUpdated,
}

impl EventEmitter<Event> for ExtensionEvents {}
