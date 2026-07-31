use std::sync::Arc;

pub(crate) fn text_system() -> Arc<dyn gpui::PlatformTextSystem> {
    Arc::new(gpui_wgpu::CosmicTextSystem::new("HarmonyOS Sans"))
}
