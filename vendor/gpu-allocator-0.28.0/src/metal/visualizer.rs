#![allow(clippy::new_without_default)]

use super::Allocator;
use crate::visualizer::{
    render_allocation_reports_ui, AllocationReportVisualizeSettings, ColorScheme,
    MemoryChunksVisualizationSettings,
};

struct AllocatorVisualizerBlockWindow {
    memory_type_index: usize,
    block_index: usize,
    settings: MemoryChunksVisualizationSettings,
}
impl AllocatorVisualizerBlockWindow {
    fn new(memory_type_index: usize, block_index: usize) -> Self {
        Self {
            memory_type_index,
            block_index,
            settings: Default::default(),
        }
    }
}

pub struct AllocatorVisualizer {
    selected_blocks: Vec<AllocatorVisualizerBlockWindow>,
    color_scheme: ColorScheme,
    breakdown_settings: AllocationReportVisualizeSettings,
}

impl AllocatorVisualizer {
    pub fn new() -> Self {
        Self {
            selected_blocks: Vec::default(),
            color_scheme: ColorScheme::default(),
            breakdown_settings: Default::default(),
        }
    }

    pub fn set_color_scheme(&mut self, color_scheme: ColorScheme) {
        self.color_scheme = color_scheme;
    }

    pub fn render_memory_block_ui(&mut self, ui: &mut egui::Ui, alloc: &Allocator) {
        ui.collapsing(
            format!("Memory Types: ({} types)", alloc.memory_types.len()),
            |ui| {
                for (mem_type_idx, mem_type) in alloc.memory_types.iter().enumerate() {
                    ui.collapsing(
                        format!(
                            "Type: {} ({} blocks)",
                            mem_type_idx,
                            mem_type.memory_blocks.len(),
                        ),
                        |ui| {
                            let mut total_block_size = 0;
                            let mut total_allocated = 0;

                            for block in mem_type.memory_blocks.iter().flatten() {
                                total_block_size += block.size;
                                total_allocated += block.sub_allocator.allocated();
                            }

                            let active_block_count = mem_type
                                .memory_blocks
                                .iter()
                                .filter(|block| block.is_some())
                                .count();

                            ui.label(format!("properties: {:?}", mem_type.heap_properties));
                            ui.label(format!("memory type index: {}", mem_type.memory_type_index));
                            ui.label(format!("total block size: {} KiB", total_block_size / 1024));
                            ui.label(format!("total allocated:  {} KiB", total_allocated / 1024));
                            ui.label(format!("block count: {active_block_count}"));

                            for (block_idx, block) in mem_type.memory_blocks.iter().enumerate() {
                                let Some(block) = block else { continue };

                                ui.collapsing(format!("Block: {block_idx}"), |ui| {
                                    ui.label(format!("size: {} KiB", block.size / 1024));
                                    ui.label(format!(
                                        "allocated: {} KiB",
                                        block.sub_allocator.allocated() / 1024
                                    ));
                                    ui.label(format!("Heap: {:?}", &block.heap));

                                    block.sub_allocator.draw_base_info(ui);

                                    if block.sub_allocator.supports_visualization()
                                        && ui.button("visualize").clicked()
                                        && !self.selected_blocks.iter().any(|x| {
                                            x.memory_type_index == mem_type_idx
                                                && x.block_index == block_idx
                                        })
                                    {
                                        self.selected_blocks.push(
                                            AllocatorVisualizerBlockWindow::new(
                                                mem_type_idx,
                                                block_idx,
                                            ),
                                        );
                                    }
                                });
                            }
                        },
                    );
                }
            },
        );
    }

    pub fn render_memory_block_window(
        &mut self,
        ctx: &egui::Context,
        allocator: &Allocator,
        open: &mut bool,
    ) {
        egui::Window::new("Allocator Memory Blocks")
            .open(open)
            .show(ctx, |ui| self.render_breakdown_ui(ui, allocator));
    }

    pub fn render_memory_block_visualization_windows(
        &mut self,
        ctx: &egui::Context,
        allocator: &Allocator,
    ) {
        // Draw each window.
        let color_scheme = &self.color_scheme;

        self.selected_blocks.retain_mut(|window| {
            let mut open = true;

            egui::Window::new(format!(
                "Block Visualizer {}:{}",
                window.memory_type_index, window.block_index
            ))
            .default_size([1920.0 * 0.5, 1080.0 * 0.5])
            .open(&mut open)
            .show(ctx, |ui| {
                let memblock = &allocator.memory_types[window.memory_type_index].memory_blocks
                    [window.block_index]
                    .as_ref();
                if let Some(memblock) = memblock {
                    ui.label(format!(
                        "Memory type {}, Memory block {}, Block size: {} KiB",
                        window.memory_type_index,
                        window.block_index,
                        memblock.size / 1024
                    ));

                    window
                        .settings
                        .ui(ui, allocator.debug_settings.store_stack_traces);

                    ui.separator();

                    memblock
                        .sub_allocator
                        .draw_visualization(color_scheme, ui, &window.settings);
                } else {
                    ui.label("Deallocated memory block");
                }
            });

            open
        });
    }

    pub fn render_breakdown_ui(&mut self, ui: &mut egui::Ui, allocator: &Allocator) {
        render_allocation_reports_ui(
            ui,
            &mut self.breakdown_settings,
            allocator
                .memory_types
                .iter()
                .flat_map(|memory_type| memory_type.memory_blocks.iter())
                .flatten()
                .flat_map(|memory_block| memory_block.sub_allocator.report_allocations()),
        );
    }

    pub fn render_breakdown_window(
        &mut self,
        ctx: &egui::Context,
        allocator: &Allocator,
        open: &mut bool,
    ) {
        egui::Window::new("Allocator Breakdown")
            .open(open)
            .show(ctx, |ui| self.render_breakdown_ui(ui, allocator));
    }
}
