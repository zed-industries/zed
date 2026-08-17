use std::backtrace::BacktraceStatus;

use egui::{Color32, DragValue, Rect, ScrollArea, Sense, Ui, Vec2};

use super::ColorScheme;
use crate::allocator::free_list_allocator::MemoryChunk;

pub(crate) struct MemoryChunksVisualizationSettings {
    pub width_in_bytes: u64,
    pub show_backtraces: bool,
}

impl Default for MemoryChunksVisualizationSettings {
    fn default() -> Self {
        Self {
            width_in_bytes: 1024,
            show_backtraces: false,
        }
    }
}

impl MemoryChunksVisualizationSettings {
    pub fn ui(&mut self, ui: &mut Ui, store_stack_traces: bool) {
        if store_stack_traces {
            ui.checkbox(&mut self.show_backtraces, "Show backtraces");
        }

        // Slider for changing the 'zoom' level of the visualizer.
        const BYTES_PER_UNIT_MIN: i32 = 1;
        const BYTES_PER_UNIT_MAX: i32 = 1024 * 1024;

        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut self.width_in_bytes)
                    .clamp_range(BYTES_PER_UNIT_MIN..=BYTES_PER_UNIT_MAX)
                    .speed(10.0),
            );
            ui.label("Bytes per line");
        });
    }
}

pub(crate) fn render_memory_chunks_ui<'a>(
    ui: &mut Ui,
    color_scheme: &ColorScheme,
    settings: &MemoryChunksVisualizationSettings,
    total_size_in_bytes: u64,
    data: impl IntoIterator<Item = &'a MemoryChunk>,
) {
    let line_height = ui.text_style_height(&egui::TextStyle::Body);
    let number_of_rows =
        (total_size_in_bytes as f32 / settings.width_in_bytes as f32).ceil() as usize;

    ScrollArea::new([false, true]).show_rows(ui, line_height, number_of_rows, |ui, range| {
        // Let range be in bytes
        let start_in_bytes = range.start as u64 * settings.width_in_bytes;
        let end_in_bytes = range.end as u64 * settings.width_in_bytes;

        let mut data = data
            .into_iter()
            .filter(|chunk| {
                (chunk.offset + chunk.size) > start_in_bytes && chunk.offset < end_in_bytes
            })
            .collect::<Vec<_>>();
        data.sort_by_key(|chunk| chunk.offset);

        let screen_width = ui.available_width();
        let mut cursor_idx = 0;
        let mut bytes_required = data[cursor_idx].offset + data[cursor_idx].size - start_in_bytes;

        for _ in range {
            ui.horizontal(|ui| {
                let mut bytes_left = settings.width_in_bytes;
                let mut cursor = ui.cursor().min;

                while cursor_idx < data.len() && bytes_left > 0 {
                    // Block is depleted, so reset for more chunks
                    while bytes_required == 0 {
                        cursor_idx += 1;
                        if cursor_idx < data.len() {
                            bytes_required = data[cursor_idx].size;
                        }
                    }

                    let bytes_used = bytes_required.min(bytes_left);
                    let width_used =
                        bytes_used as f32 * screen_width / settings.width_in_bytes as f32;

                    // Draw the rectangle
                    let resp = ui.allocate_rect(
                        Rect::from_min_size(cursor, Vec2::new(width_used, line_height)),
                        Sense::click(),
                    );

                    if ui.is_rect_visible(resp.rect) {
                        ui.painter().rect(
                            resp.rect,
                            egui::Rounding::ZERO,
                            color_scheme
                                .get_allocation_type_color(data[cursor_idx].allocation_type),
                            egui::Stroke::new(1.0, Color32::BLACK),
                        );

                        resp.on_hover_ui_at_pointer(|ui| {
                            let chunk = &data[cursor_idx];
                            ui.label(format!("id: {}", chunk.chunk_id));
                            ui.label(format!("offset: 0x{:x}", chunk.offset));
                            ui.label(format!("size: 0x{:x}", chunk.size));
                            ui.label(format!(
                                "allocation_type: {}",
                                chunk.allocation_type.as_str()
                            ));
                            if let Some(name) = &chunk.name {
                                ui.label(format!("name: {name}"));
                            }
                            if settings.show_backtraces
                                && chunk.backtrace.status() == BacktraceStatus::Captured
                            {
                                ui.label(chunk.backtrace.to_string());
                            }
                        });
                    }

                    // Update our cursors
                    cursor.x += width_used;
                    bytes_left -= bytes_used;
                    bytes_required -= bytes_used;
                }
            });
        }
    });
}
