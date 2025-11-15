use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    sync::Arc,
};

use memmap2::MmapMut;

use gpui::ParentElement;
use gpui::{App, Context, FocusHandle, IntoElement, Render, SharedString, Window};
use ui::{Button, Color, Label, LabelCommon, LabelSize, Styled, div, h_flex, v_flex};

pub struct HexEditorView {
    pub file_path: Arc<PathBuf>,
    pub file_data: Option<MmapMut>,
    pub focus_handle: FocusHandle,
    pub error: Option<SharedString>,

    // Windowed rendering state
    pub scroll_offset: usize, // Row offset (first visible row)
    pub row_height: f32,      // Height of a row in pixels
    pub visible_rows: usize,  // Number of rows visible in viewport
    pub total_rows: usize,    // Total number of rows in the file
}

impl HexEditorView {
    pub fn new(file_path: PathBuf, _window: &mut Window, cx: &mut App) -> Self {
        let mut error = None;
        let mmap = match OpenOptions::new().read(true).write(true).open(&file_path) {
            Ok(file) => match unsafe { MmapMut::map_mut(&file) } {
                Ok(mmap) => Some(mmap),
                Err(e) => {
                    error = Some(format!("Failed to memory-map file: {e}").into());
                    None
                }
            },
            Err(e) => {
                error = Some(format!("Failed to open file: {e}").into());
                None
            }
        };
        let total_rows = mmap.as_ref().map(|m| (m.len() + 15) / 16).unwrap_or(0);
        Self {
            file_path: Arc::new(file_path),
            file_data: mmap,
            focus_handle: cx.focus_handle(),
            error,
            scroll_offset: 0,
            row_height: 18.0,  // Default row height in pixels (tune as needed)
            visible_rows: 100, // Render 100 rows at a time (tune as needed)
            total_rows,
        }
    }

    /// Save changes by flushing the mmap to disk.
    pub fn save(&mut self) -> anyhow::Result<()> {
        if let Some(mmap) = &mut self.file_data {
            mmap.flush()?;
        }
        Ok(())
    }

    fn render_hex_ascii(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let bytes_per_row = 16;
        let mut rows = Vec::new();
        if let Some(data) = &self.file_data {
            let start_row = self.scroll_offset;
            let end_row = (self.scroll_offset + self.visible_rows).min(self.total_rows);
            for row_idx in start_row..end_row {
                let offset = row_idx * bytes_per_row;
                let chunk = &data[offset..(offset + bytes_per_row).min(data.len())];
                let hex_cells = chunk
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                let ascii_cells = chunk
                    .iter()
                    .map(|b| {
                        let c = *b as char;
                        if c.is_ascii_graphic() { c } else { '.' }
                    })
                    .collect::<String>();
                rows.push(
                    h_flex()
                        .gap_x_2()
                        .child(
                            Label::new(format!("{:08X}", offset))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(Label::new(hex_cells).buffer_font(cx).size(LabelSize::Small))
                        .child(
                            Label::new(ascii_cells)
                                .buffer_font(cx)
                                .size(LabelSize::Small),
                        ),
                );
            }
        }
        v_flex().gap_y_0p5().children(rows)
    }
}

impl Render for HexEditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(error) = &self.error {
            return v_flex()
                .size_full()
                .justify_center()
                .child(Label::new(error.clone()).color(Color::Error));
        }

        let file_len = self.file_data.as_ref().map(|d| d.len()).unwrap_or(0);

        v_flex()
            .size_full()
            .gap_y_2()
            .child(
                Label::new(format!(
                    "Hex Editor — {} ({} bytes)",
                    self.file_path.display(),
                    file_len
                ))
                .size(LabelSize::Large),
            )
            .child(v_flex().size_full().child(self.render_hex_ascii(cx)))
    }
}

impl gpui::Focusable for HexEditorView {
    fn focus_handle(&self, _cx: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl gpui::EventEmitter<()> for HexEditorView {}
