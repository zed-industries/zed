use gpui::{
    div, px, AnyElement, ClipboardItem, Context, IntoElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, Styled, Window,
};

use super::pdf_renderer;
use super::{CopyDocumentText, PdfViewer, TextPosition, PAGE_GAP_PX};

impl PdfViewer {
    pub(crate) fn extract_all_text(&mut self, cx: &mut Context<Self>) {
        let page_count = self.page_count();
        if page_count == 0 {
            log::debug!("pdf_viewer: extract_all_text called but page_count=0");
            return;
        }
        if self.text_layouts.len() >= page_count {
            log::debug!(
                "pdf_viewer: extract_all_text skipped, already have {}/{} layouts",
                self.text_layouts.len(),
                page_count
            );
            return;
        }
        log::debug!(
            "pdf_viewer: starting text extraction for {} pages (have {} so far)",
            page_count,
            self.text_layouts.len()
        );

        let pdf_bytes = self.pdf_item.read(cx).pdf_bytes().clone();

        let (sender, receiver) = smol::channel::unbounded::<(usize, pdf_renderer::PageTextLayout)>();

        if let Err(error) = std::thread::Builder::new()
            .name("pdf-text-extractor".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                let pdf = match pdf_renderer::open_pdf(&pdf_bytes) {
                    Ok(pdf) => pdf,
                    Err(error) => {
                        log::error!(
                            "pdf_viewer: failed to open PDF for text extraction: {error:#}"
                        );
                        return;
                    }
                };
                for page_index in 0..page_count {
                    match pdf_renderer::extract_page_text(&pdf, page_index) {
                        Ok(layout) => {
                            log::debug!(
                                "pdf_viewer: extracted {} glyphs from page {}",
                                layout.glyphs.len(),
                                page_index + 1
                            );
                            if sender.send_blocking((page_index, layout)).is_err() {
                                log::error!("pdf_viewer: channel closed, aborting text extraction");
                                break;
                            }
                        }
                        Err(error) => {
                            log::error!(
                                "pdf_viewer: failed to extract text from page {}: {error:#}",
                                page_index + 1
                            );
                        }
                    }
                }
            })
        {
            log::error!("pdf_viewer: failed to spawn text extractor thread: {error:#}");
        }

        self.text_extraction_task = cx.spawn(async move |this, cx| {
            while let Ok((page_index, layout)) = receiver.recv().await {
                let glyph_count = layout.glyphs.len();
                this.update(cx, |this, cx| {
                    log::debug!(
                        "pdf_viewer: stored text layout for page {} ({} glyphs, {}/{} pages done)",
                        page_index + 1,
                        glyph_count,
                        this.text_layouts.len() + 1,
                        this.page_count()
                    );
                    this.text_layouts.insert(page_index, layout);
                    cx.notify();
                })
                .ok();
            }
            log::debug!("pdf_viewer: text extraction channel closed, all pages done");
        });
    }

    pub(crate) fn copy_document_text(
        &mut self,
        _: &CopyDocumentText,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let page_count = self.page_count();
        if page_count == 0 {
            return;
        }

        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            if start != end {
                let (start, end) = if start > end {
                    (end, start)
                } else {
                    (start, end)
                };

                let mut selected_text = String::new();
                for page_index in start.page..=end.page {
                    if let Some(layout) = self.text_layouts.get(&page_index) {
                        let page_start = if page_index == start.page {
                            start.glyph_index
                        } else {
                            0
                        };
                        let page_end = if page_index == end.page {
                            end.glyph_index
                        } else {
                            layout.glyphs.len().saturating_sub(1)
                        };
                        if !selected_text.is_empty() {
                            selected_text.push('\n');
                        }
                        selected_text.push_str(&layout.selected_text(page_start, page_end));
                    }
                }
                cx.write_to_clipboard(ClipboardItem::new_string(selected_text));
                log::debug!(
                    "pdf_viewer: copied selected text (pages {}-{}) to clipboard",
                    start.page + 1,
                    end.page + 1
                );
                return;
            }
        }

        if self.text_layouts.len() >= page_count {
            let mut full_text = String::new();
            for page_index in 0..page_count {
                if !full_text.is_empty() {
                    full_text.push_str("\n\n");
                }
                if let Some(layout) = self.text_layouts.get(&page_index) {
                    full_text.push_str(&layout.full_text());
                }
            }
            cx.write_to_clipboard(ClipboardItem::new_string(full_text));
            log::debug!(
                "pdf_viewer: copied text from {} pages to clipboard",
                page_count
            );
        } else {
            log::debug!(
                "pdf_viewer: text extraction in progress ({}/{}), starting extraction",
                self.text_layouts.len(),
                page_count
            );
            self.extract_all_text(cx);
        }
    }

    pub(crate) fn point_to_text_position(
        &self,
        window_position: Point<Pixels>,
    ) -> Option<TextPosition> {
        let dimensions = self.page_dimensions();
        if dimensions.is_empty() {
            log::debug!("pdf_viewer: hit test — no page dimensions");
            return None;
        }

        let bounds = self.scroll_handle.bounds();
        let scroll_offset = self.scroll_handle.offset();

        let container_width: f32 = {
            let w: f32 = bounds.size.width.into();
            if w > 0.0 { w } else {
                log::debug!("pdf_viewer: hit test — container_width=0");
                return None;
            }
        };
        let viewport_height: f32 = {
            let h: f32 = bounds.size.height.into();
            if h > 0.0 { h } else { 800.0 }
        };

        let window_x: f32 = window_position.x.into();
        let window_y: f32 = window_position.y.into();
        let bounds_origin_x: f32 = bounds.origin.x.into();
        let bounds_origin_y: f32 = bounds.origin.y.into();
        let scroll_offset_y: f32 = scroll_offset.y.into();

        let content_y = (window_y - bounds_origin_y) + scroll_offset_y.abs();

        let total_content_h = self.total_content_height(container_width);
        let centering_pad = if total_content_h < viewport_height {
            (viewport_height - total_content_h) / 2.0
        } else {
            0.0
        };

        let content_y_adjusted = content_y - centering_pad;

        log::debug!(
            "pdf_viewer: hit test — window=({:.0},{:.0}) bounds_origin=({:.0},{:.0}) \
             scroll_y={:.0} content_y={:.0} centering_pad={:.0} content_y_adjusted={:.0} \
             container_width={:.0} viewport_height={:.0} total_content_h={:.0} \
             text_layouts_count={} zoom={:.2}",
            window_x, window_y,
            bounds_origin_x, bounds_origin_y,
            scroll_offset_y,
            content_y,
            centering_pad,
            content_y_adjusted,
            container_width, viewport_height, total_content_h,
            self.text_layouts.len(),
            self.zoom_level,
        );

        if content_y_adjusted < 0.0 {
            log::debug!("pdf_viewer: hit test — above content (content_y_adjusted < 0)");
            return None;
        }

        let mut accumulated_y = 0.0_f32;
        let mut target_page = None;

        for (index, dim) in dimensions.iter().enumerate() {
            let page_height = Self::display_height(dim, container_width, self.zoom_level);
            let page_top = accumulated_y;
            let page_bottom = accumulated_y + page_height;

            if content_y_adjusted >= page_top && content_y_adjusted < page_bottom {
                target_page = Some((index, dim, page_top));
                break;
            }

            accumulated_y = page_bottom + PAGE_GAP_PX;

            if content_y_adjusted < accumulated_y {
                log::debug!(
                    "pdf_viewer: hit test — in gap between pages {} and {}",
                    index + 1, index + 2
                );
                return None;
            }
        }

        let (page_index, page_dim, page_top_y) = match target_page {
            Some(t) => t,
            None => {
                log::debug!("pdf_viewer: hit test — below all pages");
                return None;
            }
        };

        let page_local_y = content_y_adjusted - page_top_y;

        let fit_scale = if page_dim.width > 0.0 {
            container_width / page_dim.width
        } else {
            1.0
        };
        let scale = fit_scale * self.zoom_level;
        let page_display_width = page_dim.width * scale;

        let page_left_in_container = (container_width - page_display_width) / 2.0;
        let pan_x_f32: f32 = self.pan_x.into();
        let content_x = (window_x - bounds_origin_x) - page_left_in_container - pan_x_f32;

        log::debug!(
            "pdf_viewer: hit test — page {} (dim={:.0}x{:.0}) fit_scale={:.3} scale={:.3} \
             page_display_width={:.0} page_left={:.0} pan_x={:.0} content_x={:.0} page_local_y={:.0}",
            page_index + 1,
            page_dim.width, page_dim.height,
            fit_scale, scale,
            page_display_width, page_left_in_container,
            pan_x_f32,
            content_x, page_local_y,
        );

        if content_x < 0.0 || content_x > page_display_width {
            log::debug!(
                "pdf_viewer: hit test — outside page horizontally (content_x={:.0}, page_width={:.0})",
                content_x, page_display_width
            );
            return None;
        }

        let pdf_x = content_x / scale;
        let pdf_y = page_local_y / scale;

        log::debug!(
            "pdf_viewer: hit test — pdf coords ({:.1}, {:.1})",
            pdf_x, pdf_y
        );

        let layout = match self.text_layouts.get(&page_index) {
            Some(l) => l,
            None => {
                log::debug!(
                    "pdf_viewer: hit test — no text layout for page {} (have layouts for pages: {:?})",
                    page_index + 1,
                    self.text_layouts.keys().map(|k| k + 1).collect::<Vec<_>>()
                );
                return None;
            }
        };

        log::debug!(
            "pdf_viewer: hit test — page {} has {} glyphs",
            page_index + 1,
            layout.glyphs.len()
        );

        let glyph_index = match layout.glyph_index_at_point(pdf_x, pdf_y) {
            Some(idx) => {
                if let Some(glyph) = layout.glyphs.get(idx) {
                    log::debug!(
                        "pdf_viewer: hit test — matched glyph {} '{}' at ({:.1},{:.1}) w={:.1} fs={:.1}",
                        idx, glyph.character, glyph.x, glyph.y, glyph.width, glyph.font_size
                    );
                }
                idx
            }
            None => {
                log::debug!("pdf_viewer: hit test — glyph_index_at_point returned None");
                return None;
            }
        };

        Some(TextPosition {
            page: page_index,
            glyph_index,
        })
    }

    pub(crate) fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button != MouseButton::Left {
            return;
        }

        log::debug!(
            "pdf_viewer: mouse_down at ({:.0},{:.0})",
            f32::from(event.position.x),
            f32::from(event.position.y),
        );

        if let Some(position) = self.point_to_text_position(event.position) {
            log::debug!(
                "pdf_viewer: selection start — page {} glyph {}",
                position.page + 1,
                position.glyph_index
            );
            self.selection_start = Some(position);
            self.selection_end = Some(position);
            self.is_selecting = true;
        } else {
            log::debug!("pdf_viewer: mouse_down — no text position found, clearing selection");
            self.selection_start = None;
            self.selection_end = None;
            self.is_selecting = false;
        }
        cx.notify();
    }

    pub(crate) fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_selecting {
            return;
        }

        if event.pressed_button != Some(MouseButton::Left) {
            log::debug!("pdf_viewer: mouse_move — left button released, stopping selection");
            self.is_selecting = false;
            return;
        }

        if let Some(position) = self.point_to_text_position(event.position) {
            log::debug!(
                "pdf_viewer: selection drag — page {} glyph {}",
                position.page + 1,
                position.glyph_index
            );
            self.selection_end = Some(position);
        }
        cx.notify();
    }

    pub(crate) fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = false;

        if self.selection_start == self.selection_end {
            log::debug!("pdf_viewer: mouse_up — click without drag, clearing selection");
            self.selection_start = None;
            self.selection_end = None;
        } else {
            log::debug!(
                "pdf_viewer: mouse_up — selection: {:?} to {:?}",
                self.selection_start,
                self.selection_end
            );
        }
        cx.notify();
    }

    pub(crate) fn selection_highlights_for_page(
        &self,
        page_index: usize,
        fit_scale: f32,
    ) -> Vec<AnyElement> {
        let (start, end) = match (self.selection_start, self.selection_end) {
            (Some(start), Some(end)) if start != end => {
                if start > end {
                    (end, start)
                } else {
                    (start, end)
                }
            }
            _ => return Vec::new(),
        };

        let layout = match self.text_layouts.get(&page_index) {
            Some(layout) => layout,
            None => return Vec::new(),
        };

        if page_index < start.page || page_index > end.page {
            return Vec::new();
        }

        let glyph_start = if page_index == start.page {
            start.glyph_index
        } else {
            0
        };
        let glyph_end = if page_index == end.page {
            end.glyph_index
        } else {
            layout.glyphs.len().saturating_sub(1)
        };

        let scale = fit_scale * self.zoom_level;
        let mut highlights = Vec::new();

        if glyph_start <= glyph_end {
            log::debug!(
                "pdf_viewer: rendering {} highlights for page {} (glyphs {}..={}, scale={:.3})",
                glyph_end - glyph_start + 1,
                page_index + 1,
                glyph_start,
                glyph_end,
                scale,
            );
        }

        for glyph_idx in glyph_start..=glyph_end {
            if let Some(glyph) = layout.glyphs.get(glyph_idx) {
                let display_x = glyph.x * scale;
                let display_width = glyph.width * scale;
                let display_height = glyph.font_size * scale;
                // glyph.y is the baseline; text ascenders extend above it
                // (smaller Y in top-down coords). Shift up by ~80% of
                // font_size to cover the main glyph body.
                let display_y = (glyph.y - glyph.font_size * 0.8) * scale;

                if glyph_idx == glyph_start {
                    log::debug!(
                        "pdf_viewer: first highlight glyph '{}' — pdf({:.1},{:.1}) display({:.0},{:.0}) size({:.0}x{:.0})",
                        glyph.character,
                        glyph.x, glyph.y,
                        display_x, display_y,
                        display_width, display_height,
                    );
                }

                highlights.push(
                    div()
                        .absolute()
                        .left(px(display_x))
                        .top(px(display_y))
                        .w(px(display_width))
                        .h(px(display_height))
                        .bg(gpui::rgba(0x3584e430))
                        .into_any_element(),
                );
            }
        }

        highlights
    }
}