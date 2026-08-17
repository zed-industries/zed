use egui::{Color32, Ui};

mod allocation_reports;
mod memory_chunks;

pub(crate) use allocation_reports::*;
pub(crate) use memory_chunks::*;

use crate::allocator::AllocationType;

pub const DEFAULT_COLOR_ALLOCATION_TYPE_FREE: Color32 = Color32::from_rgb(159, 159, 159); // gray
pub const DEFAULT_COLOR_ALLOCATION_TYPE_LINEAR: Color32 = Color32::from_rgb(91, 206, 250); // blue
pub const DEFAULT_COLOR_ALLOCATION_TYPE_NON_LINEAR: Color32 = Color32::from_rgb(250, 169, 184); // pink

#[derive(Clone)]
pub struct ColorScheme {
    pub free_color: Color32,
    pub linear_color: Color32,
    pub non_linear_color: Color32,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            free_color: DEFAULT_COLOR_ALLOCATION_TYPE_FREE,
            linear_color: DEFAULT_COLOR_ALLOCATION_TYPE_LINEAR,
            non_linear_color: DEFAULT_COLOR_ALLOCATION_TYPE_NON_LINEAR,
        }
    }
}

impl ColorScheme {
    pub(crate) fn get_allocation_type_color(&self, allocation_type: AllocationType) -> Color32 {
        match allocation_type {
            AllocationType::Free => self.free_color,
            AllocationType::Linear => self.linear_color,
            AllocationType::NonLinear => self.non_linear_color,
        }
    }
}

pub(crate) trait SubAllocatorVisualizer {
    fn supports_visualization(&self) -> bool {
        false
    }
    fn draw_base_info(&self, ui: &mut Ui) {
        ui.label("No sub allocator information available");
    }
    fn draw_visualization(
        &self,
        _color_scheme: &ColorScheme,
        _ui: &mut Ui,
        _settings: &MemoryChunksVisualizationSettings,
    ) {
    }
}
