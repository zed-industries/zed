use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ImageSource, ScrollEvent,
    ScrollHandle, Size, Subscription, Task, img, size,
};
use markdown::{MERMAID_MAX_ZOOM, MERMAID_ZOOM_DEBOUNCE, MERMAID_ZOOM_STEP, Markdown};
use ui::{ScrollAxes, Scrollbars, Tooltip, WithScrollbar, prelude::*};
use workspace::{DismissDecision, ModalView};

/// Fraction of the window width the modal takes up.
const MODAL_WIDTH_FRACTION: f32 = 0.9;

/// Shows a single mermaid diagram from a markdown preview in a large modal,
/// initially fitted to the modal, and zoomable with ctrl/cmd+scroll.
pub struct MermaidDiagramModal {
    markdown: Entity<Markdown>,
    source_offset: usize,
    /// The zoom the user chose, or `None` while the diagram is fitted to the
    /// modal, so that the fit follows the window when it's resized.
    zoom: Option<f32>,
    /// The fitted zoom as of the last render.
    fit_zoom: f32,
    /// A pending debounced re-raster at the displayed zoom.
    rasterize_task: Option<Task<()>>,
    scroll_handle: ScrollHandle,
    focus_handle: FocusHandle,
    _markdown_subscription: Subscription,
}

impl MermaidDiagramModal {
    pub fn new(markdown: Entity<Markdown>, source_offset: usize, cx: &mut Context<Self>) -> Self {
        // Repaint when a new raster of the diagram becomes available.
        let markdown_subscription = cx.observe(&markdown, |_, _, cx| cx.notify());
        Self {
            markdown,
            source_offset,
            zoom: None,
            fit_zoom: 1.0,
            rasterize_task: None,
            scroll_handle: ScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            _markdown_subscription: markdown_subscription,
        }
    }

    fn displayed_zoom(&self) -> f32 {
        self.zoom.unwrap_or(self.fit_zoom)
    }

    fn set_zoom(&mut self, zoom: f32, cx: &mut Context<Self>) {
        let zoom = zoom.clamp(self.fit_zoom, MERMAID_MAX_ZOOM.max(self.fit_zoom));
        self.zoom = (zoom - self.fit_zoom > 0.001).then_some(zoom);
        self.schedule_rasterize(cx);
        cx.notify();
    }

    fn reset_zoom(&mut self, cx: &mut Context<Self>) {
        self.zoom = None;
        self.schedule_rasterize(cx);
        cx.notify();
    }

    /// Re-rasterizes the diagram at the displayed zoom once zoom changes
    /// settle. Until then, the current raster is shown scaled.
    fn schedule_rasterize(&mut self, cx: &mut Context<Self>) {
        self.rasterize_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(MERMAID_ZOOM_DEBOUNCE).await;
            this.update(cx, |this, cx| {
                this.rasterize_task = None;
                let zoom = this.displayed_zoom();
                this.markdown.update(cx, |markdown, cx| {
                    markdown.rasterize_mermaid_diagram_at(this.source_offset, zoom, cx);
                });
            })
            .ok();
        }));
    }

    fn on_scroll(&mut self, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        if !(event.modifiers.control || event.modifiers.platform) {
            return;
        }
        let ticks = markdown::mermaid_zoom_ticks(event.delta);
        if ticks != 0.0 {
            self.set_zoom(self.displayed_zoom() + ticks * MERMAID_ZOOM_STEP, cx);
        }
        cx.stop_propagation();
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn render_header(&self, header_height: Pixels, cx: &mut Context<Self>) -> impl IntoElement {
        let percentage = (self.displayed_zoom() * 100.0).round() as i32;
        h_flex()
            .h(header_height)
            .flex_none()
            .px_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                Label::new("Mermaid Diagram")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Label::new(format!("{percentage}%"))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        IconButton::new("mermaid-modal-fit", IconName::RotateCcw)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .disabled(self.zoom.is_none())
                            .tooltip(Tooltip::text("Fit to Window"))
                            .on_click(cx.listener(|this, _, _, cx| this.reset_zoom(cx))),
                    )
                    .child(
                        IconButton::new("mermaid-modal-close", IconName::Close)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip(Tooltip::text("Close"))
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    ),
            )
    }
}

/// The zoom at which a diagram fits entirely within `available`, never
/// enlarging it past its natural size.
fn fit_zoom(natural_size: Size<Pixels>, available: Size<Pixels>) -> f32 {
    if natural_size.width <= px(0.) || natural_size.height <= px(0.) {
        return 1.0;
    }
    (available.width / natural_size.width)
        .min(available.height / natural_size.height)
        .min(1.0)
}

impl EventEmitter<DismissEvent> for MermaidDiagramModal {}

impl Focusable for MermaidDiagramModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for MermaidDiagramModal {
    fn on_before_dismiss(&mut self, _: &mut Window, cx: &mut Context<Self>) -> DismissDecision {
        // Make the inline diagram sharp again at its own zoom.
        self.markdown.update(cx, |markdown, cx| {
            markdown.rerasterize_mermaid_diagram(self.source_offset, cx);
        });
        DismissDecision::Dismiss(true)
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Render for MermaidDiagramModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The modal layer places modals 5rem from the top of the window; keep
        // the same margin at the bottom.
        let rem_size = window.rem_size();
        let viewport = window.viewport_size();
        let modal_size = size(
            viewport.width * MODAL_WIDTH_FRACTION,
            (viewport.height - rem_size * 10.).max(rem_size * 10.),
        );
        let header_height = rem_size * 2.;
        let body_padding = rem_size * 0.5;
        let body_size = size(
            modal_size.width - body_padding * 2.,
            modal_size.height - header_height - body_padding * 2.,
        );

        let image = self
            .markdown
            .read(cx)
            .mermaid_diagram_image(self.source_offset);

        let body = match image {
            Some(image) => {
                self.fit_zoom = fit_zoom(image.natural_size, body_size);
                let zoom = self.displayed_zoom();
                if self.rasterize_task.is_none() && (image.rasterized_zoom - zoom).abs() > 0.001 {
                    self.schedule_rasterize(cx);
                }
                let display_size = size(
                    image.natural_size.width * zoom,
                    image.natural_size.height * zoom,
                );
                // Center the diagram while it's smaller than the modal, and
                // let it overflow (and scroll) once it's larger.
                let content_size = size(
                    display_size.width.max(body_size.width),
                    display_size.height.max(body_size.height),
                );
                let scrollable = div()
                    .id("mermaid-modal-diagram")
                    .size_full()
                    .overflow_scroll()
                    .track_scroll(&self.scroll_handle)
                    .on_scroll(cx.listener(Self::on_scroll))
                    .child(
                        div()
                            .w(content_size.width)
                            .h(content_size.height)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                img(ImageSource::Render(image.image))
                                    .flex_none()
                                    .w(display_size.width)
                                    .h(display_size.height),
                            ),
                    );
                div()
                    .size_full()
                    .custom_scrollbars(
                        Scrollbars::always_visible(ScrollAxes::Both)
                            .tracked_scroll_handle(&self.scroll_handle),
                        window,
                        cx,
                    )
                    .child(scrollable)
                    .into_any_element()
            }
            None => h_flex()
                .size_full()
                .justify_center()
                .child(Label::new("Diagram is no longer available").color(Color::Muted))
                .into_any_element(),
        };

        v_flex()
            .key_context("MermaidDiagramModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w(modal_size.width)
            .h(modal_size.height)
            .overflow_hidden()
            .child(self.render_header(header_height, cx))
            .child(
                div()
                    .w(modal_size.width)
                    .h(body_size.height + body_padding * 2.)
                    .p(body_padding)
                    .bg(cx.theme().colors().editor_background)
                    .child(body),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::fit_zoom;
    use gpui::{px, size};

    #[test]
    fn test_fit_zoom() {
        let available = size(px(1000.), px(500.));

        // Wider than available: fitted to the width.
        assert_eq!(fit_zoom(size(px(4000.), px(200.)), available), 0.25);
        // Taller than available: fitted to the height.
        assert_eq!(fit_zoom(size(px(100.), px(1000.)), available), 0.5);
        // Already fits: never enlarged past its natural size.
        assert_eq!(fit_zoom(size(px(200.), px(100.)), available), 1.0);
        // Degenerate sizes don't divide by zero.
        assert_eq!(fit_zoom(size(px(0.), px(0.)), available), 1.0);
    }
}
