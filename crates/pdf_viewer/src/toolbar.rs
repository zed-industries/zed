use gpui::{Context, EventEmitter, IntoElement, Subscription, WeakEntity, Window};
use ui::prelude::*;
use ui::{IconButton, IconName, IconSize, Tooltip};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, item::ItemHandle};

use super::{
    CopyDocumentText, FitToView, PdfViewer, ResetZoom, ZoomIn, ZoomOut,
};

pub struct PdfViewToolbarControls {
    pdf_view: Option<WeakEntity<PdfViewer>>,
    _subscription: Option<Subscription>,
}

impl PdfViewToolbarControls {
    pub fn new() -> Self {
        Self {
            pdf_view: None,
            _subscription: None,
        }
    }
}

impl Render for PdfViewToolbarControls {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(pdf_view) = self.pdf_view.as_ref().and_then(|v| v.upgrade()) else {
            return div().into_any_element();
        };

        let zoom_level = pdf_view.read(cx).zoom_level;
        let zoom_percentage = format!("{}%", (zoom_level * 100.0).round() as i32);

        h_flex()
            .gap_1()
            .child(
                IconButton::new("zoom-out", IconName::Dash)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Zoom Out", &ZoomOut, cx))
                    .on_click({
                        let pdf_view = pdf_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = pdf_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.zoom_out(&ZoomOut, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                Button::new("zoom-level", zoom_percentage)
                    .label_size(LabelSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Reset Zoom", &ResetZoom, cx))
                    .on_click({
                        let pdf_view = pdf_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = pdf_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.reset_zoom(&ResetZoom, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                IconButton::new("zoom-in", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Zoom In", &ZoomIn, cx))
                    .on_click({
                        let pdf_view = pdf_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = pdf_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.zoom_in(&ZoomIn, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                IconButton::new("fit-to-view", IconName::Maximize)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Fit to View", &FitToView, cx))
                    .on_click({
                        let pdf_view = pdf_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = pdf_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.fit_to_view(&FitToView, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                IconButton::new("copy-text", IconName::Copy)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Copy Document Text", &CopyDocumentText, cx)
                    })
                    .on_click({
                        let pdf_view = pdf_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = pdf_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.copy_document_text(&CopyDocumentText, window, cx);
                                });
                            }
                        }
                    }),
            )
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for PdfViewToolbarControls {}

impl ToolbarItemView for PdfViewToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.pdf_view = None;
        self._subscription = None;

        if let Some(item) = active_pane_item.and_then(|i| i.downcast::<PdfViewer>()) {
            self._subscription = Some(cx.observe(&item, |_, _, cx| {
                cx.notify();
            }));
            self.pdf_view = Some(item.downgrade());
            cx.notify();
            return ToolbarItemLocation::PrimaryRight;
        }

        ToolbarItemLocation::Hidden
    }
}