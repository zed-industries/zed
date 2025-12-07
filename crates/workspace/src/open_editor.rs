use std::sync::Arc;

use gpui::{App, Global, Window};

use crate::{ItemHandle, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|workspace, _: &crate::OpenEditor, window, cx| {
            if let Some(active_item) = workspace.active_item(cx) {
                if let Some(source_item) =
                    get_source_item_from_preview(active_item.as_ref(), window, cx)
                {
                    // Open the source item using the standard workspace flow
                    workspace.add_item_to_active_pane(source_item, None, true, window, cx);
                }
            }
        });
    })
    .detach();
}

/// Registry for extracting source items from preview items.
#[derive(Clone, Default)]
pub struct PreviewSourceRegistry {
    extractors: Vec<Arc<dyn PreviewSourceExtractor>>,
}

impl Global for PreviewSourceRegistry {}

impl PreviewSourceRegistry {
    pub fn register(&mut self, extractor: Arc<dyn PreviewSourceExtractor>) {
        self.extractors.push(extractor);
    }

    pub fn get_source_item(
        &self,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Box<dyn ItemHandle>> {
        let extractors = self.extractors.clone();
        for extractor in extractors {
            if let Some(source) = extractor.extract_source(item, window, cx) {
                return Some(source);
            }
        }
        None
    }
}

/// Trait for extracting source items from preview items.
pub trait PreviewSourceExtractor: Send + Sync {
    /// Attempts to extract the source item from a preview.
    /// Returns None if this extractor doesn't handle this preview type.
    fn extract_source(
        &self,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Box<dyn ItemHandle>>;
}

/// Register a preview source extractor globally.
pub fn register_preview_source_extractor(extractor: Arc<dyn PreviewSourceExtractor>, cx: &mut App) {
    cx.default_global::<PreviewSourceRegistry>()
        .register(extractor);
}

/// Attempts to extract the source item (editor/buffer) from a preview item.
/// Returns None if the item is not a preview or if extraction fails.
fn get_source_item_from_preview(
    item: &dyn ItemHandle,
    window: &mut Window,
    cx: &mut App,
) -> Option<Box<dyn ItemHandle>> {
    let registry = cx.try_global::<PreviewSourceRegistry>()?.clone();
    registry.get_source_item(item, window, cx)
}
