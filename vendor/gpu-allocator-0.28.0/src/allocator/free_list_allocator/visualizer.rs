use super::FreeListAllocator;
use crate::visualizer::{
    render_memory_chunks_ui, ColorScheme, MemoryChunksVisualizationSettings, SubAllocatorVisualizer,
};

impl SubAllocatorVisualizer for FreeListAllocator {
    fn supports_visualization(&self) -> bool {
        true
    }

    fn draw_base_info(&self, ui: &mut egui::Ui) {
        ui.label("free list sub-allocator");
        ui.label(format!("chunk count: {}", self.chunks.len()));
        ui.label(format!("chunk id counter: {}", self.chunk_id_counter));
    }

    fn draw_visualization(
        &self,
        color_scheme: &ColorScheme,
        ui: &mut egui::Ui,
        settings: &MemoryChunksVisualizationSettings,
    ) {
        render_memory_chunks_ui(ui, color_scheme, settings, self.size, self.chunks.values());
    }
}
