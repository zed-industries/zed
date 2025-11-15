#![allow(dead_code)]

use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    sync::Arc,
};

use gpui::{
    App, Context, FocusHandle, IntoElement, Render, SharedString, Window,
};
use ui::{v_flex, h_flex, div, Button, Label, LabelSize, Color, Styled, LabelCommon};
use gpui::ParentElement;


/// A simple file-backed hex editor view.
/// This is a minimal version; it will be extended to support editing and saving.
pub struct HexEditorView {
    pub file_path: Arc<PathBuf>,
    pub file_data: Vec<u8>,
    pub focus_handle: FocusHandle,
    pub error: Option<SharedString>,
}

impl HexEditorView {
    pub fn new(file_path: PathBuf, _window: &mut Window, cx: &mut App) -> Self {
        let mut file_data = Vec::new();
        let mut error = None;
        match File::open(&file_path) {
            Ok(mut file) => {
                if let Err(e) = file.read_to_end(&mut file_data) {
                    error = Some(format!("Failed to read file: {e}").into());
                }
            }
            Err(e) => {
                error = Some(format!("Failed to open file: {e}").into());
            }
        }
        Self {
            file_path: Arc::new(file_path),
            file_data,
            focus_handle: cx.focus_handle(),
            error,
        }
    }

    fn render_hex_ascii(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let bytes_per_row = 16;
        let mut rows = Vec::new();
        for (row_idx, chunk) in self.file_data.chunks(bytes_per_row).enumerate() {
            let offset = row_idx * bytes_per_row;
            let hex_cells = chunk
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            let ascii_cells = chunk
                .iter()
                .map(|b| {
                    let c = *b as char;
                    if c.is_ascii_graphic() {
                        c
                    } else {
                        '.'
                    }
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
                    .child(
                        Label::new(hex_cells)
                            .buffer_font(cx)
                            .size(LabelSize::Small),
                    )
                    .child(
                        Label::new(ascii_cells)
                            .buffer_font(cx)
                            .size(LabelSize::Small),
                    ),
            );
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

        v_flex()
            .size_full()
            .gap_y_2()
            .child(
                Label::new(format!(
                    "Hex Editor — {} ({} bytes)",
                    self.file_path.display(),
                    self.file_data.len()
                ))
                .size(LabelSize::Large),
            )
            .child(self.render_hex_ascii(cx))
    }
}

impl gpui::Focusable for HexEditorView {
    fn focus_handle(&self, _cx: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl gpui::EventEmitter<()> for HexEditorView {}
