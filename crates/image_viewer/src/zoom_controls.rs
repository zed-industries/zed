use crate::ImageView;
use gpui::{div, IntoElement, ParentElement, Point, Render, Size, Subscription, WeakEntity};
use ui::prelude::*;
use ui::{IconName, Tooltip};
use workspace::{ItemHandle, StatusItemView, Workspace};

pub struct ZoomControls {
    zoom_level: f32,
    active_image_view: Option<WeakEntity<ImageView>>,
    _observe_active_image: Option<Subscription>,
}

impl ZoomControls {
    pub fn new(_workspace: &Workspace) -> Self {
        Self {
            zoom_level: 1.0,
            active_image_view: None,
            _observe_active_image: None,
        }
    }

    fn update_zoom_level(&mut self, cx: &mut Context<Self>) {
        if let Some(image_view) = self.active_image_view.as_ref().and_then(|v| v.upgrade()) {
            let current_zoom = image_view.read(cx).zoom_level;
            if (self.zoom_level - current_zoom).abs() > f32::EPSILON {
                self.zoom_level = current_zoom;
                cx.notify();
            }
        }
    }

    fn handle_zoom_in(&mut self, window_size: Size<Pixels>, cx: &mut Context<Self>) {
        if let Some(image_view) = self.active_image_view.as_ref().and_then(|v| v.upgrade()) {
            image_view.update(cx, |view, cx| {
                let new_zoom = (view.zoom_level * 1.2).clamp(0.1, 10.0);
                let center = Point::new(window_size.width / 2.0, window_size.height / 2.0);
                view.update_zoom(new_zoom, center, cx);
            });
        }
    }

    fn handle_zoom_out(&mut self, window_size: Size<Pixels>, cx: &mut Context<Self>) {
        if let Some(image_view) = self.active_image_view.as_ref().and_then(|v| v.upgrade()) {
            image_view.update(cx, |view, cx| {
                let new_zoom = (view.zoom_level / 1.2).clamp(0.1, 10.0);
                let center = Point::new(window_size.width / 2.0, window_size.height / 2.0);
                view.update_zoom(new_zoom, center, cx);
            });
        }
    }

    fn handle_reset(&mut self, window_size: Size<Pixels>, cx: &mut Context<Self>) {
        if let Some(image_view) = self.active_image_view.as_ref().and_then(|v| v.upgrade()) {
            image_view.update(cx, |view, cx| {
                view.reset_view(window_size, cx);
            });
        }
    }
}

impl Render for ZoomControls {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bounds = window.bounds();

        match self.active_image_view.as_ref().and_then(|v| v.upgrade()) {
            Some(_) => {
                div()
                    .flex()
                    .gap_2()
                    .child(
                        IconButton::new("zoom-out", IconName::Dash)
                            .icon_size(IconSize::XSmall)
                            .tooltip(Tooltip::text("Click to Zoom Out"))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.handle_zoom_out(bounds.size, cx)
                            })),
                    )
                    .child(
                        Button::new("zoom-level", format!("{:.0}%", self.zoom_level * 100.0))
                            .label_size(LabelSize::Small),
                    )
                    .child(
                        IconButton::new("zoom-in", IconName::Plus)
                            .icon_size(IconSize::XSmall)
                            .tooltip(Tooltip::text("Click to Zoom In"))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.handle_zoom_in(bounds.size, cx)
                            })),
                    )
                    .child(
                        IconButton::new("reset-zoom", IconName::RotateCcw)
                            .icon_size(IconSize::XSmall)
                            .tooltip(Tooltip::text("Click to reset"))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.handle_reset(bounds.size, cx)
                            })),
                    )
            }
            None => div(),
        }
    }
}

impl StatusItemView for ZoomControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_image_view = active_pane_item
            .and_then(|item| item.act_as::<ImageView>(cx))
            .map(|view| view.downgrade());

        if let Some(image_view) = self.active_image_view.as_ref().and_then(|v| v.upgrade()) {
            self._observe_active_image = Some(cx.observe(&image_view, |this, _, cx| {
                this.update_zoom_level(cx);
            }));
        }

        self.update_zoom_level(cx);
    }
}
