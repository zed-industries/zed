use std::sync::Arc;

use gpui::{App, Global, ReadGlobal, UpdateGlobal};

use crate::CustomActionProvider;

pub(crate) struct GlobalCustomActionProvider(pub(crate) Arc<CustomActionProvider>);

impl Global for GlobalCustomActionProvider {}

pub struct CustomActionRegistry;

impl CustomActionRegistry {
    pub fn global(cx: &App) -> Arc<CustomActionProvider> {
        GlobalCustomActionProvider::global(cx).0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Arc<CustomActionProvider>> {
        cx.try_global::<GlobalCustomActionProvider>()
            .map(|provider| provider.0.clone())
    }

    pub fn init_global(cx: &mut App) {
        GlobalCustomActionProvider::set_global(
            cx,
            GlobalCustomActionProvider(Arc::new(CustomActionProvider::new())),
        )
    }
}
