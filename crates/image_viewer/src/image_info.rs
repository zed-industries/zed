use crate::ImageView;
use gpui::{div, AppContext, IntoElement, ParentElement, Render, Subscription, View, ViewContext};
use settings::{ImageFileSizeUnitType, Settings};
use ui::{prelude::*, Button, LabelSize};
use workspace::{ItemHandle, StatusItemView, Workspace};

pub struct ImageInfo {
    width: Option<u32>,
    height: Option<u32>,
    file_size: Option<u64>,
    color_type: Option<String>,
    _observe_active_image: Option<Subscription>,
    image_unit_type: ImageFileSizeUnitType,
}

impl ImageInfo {
    pub fn new(_workspace: &Workspace, cx: &mut AppContext) -> Self {
        let unit_type = ImageFileSizeUnitType::get_global(cx);

        Self {
            width: None,
            height: None,
            file_size: None,
            color_type: None,
            _observe_active_image: None,
            image_unit_type: unit_type.clone(),
        }
    }

    fn update_metadata(&mut self, image_view: &View<ImageView>, cx: &mut ViewContext<Self>) {
        let image_item = image_view.read(cx).image_item.read(cx);

        self.width = image_item.width;
        self.height = image_item.height;
        self.file_size = image_item.file_size;
        self.color_type = image_item.color_type.map(String::from);

        cx.notify();
    }

    fn format_file_size(&self, size: u64) -> String {
        match self.image_unit_type {
            ImageFileSizeUnitType::Binary => {
                if size < 1024 {
                    format!("{}B", size)
                } else if size < 1024 * 1024 {
                    format!("{:.1}KB", size as f64 / 1024.0)
                } else {
                    format!("{:.1}MB", size as f64 / (1024.0 * 1024.0))
                }
            }
            ImageFileSizeUnitType::Decimal => {
                if size < 1000 {
                    format!("{}B", size)
                } else if size < 1000 * 1000 {
                    format!("{:.1}KB", size as f64 / 1000.0)
                } else {
                    format!("{:.1}MB", size as f64 / (1000.0 * 1000.0))
                }
            }
        }
    }
}

impl Render for ImageInfo {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut text = String::new();

        if let (Some(width), Some(height)) = (self.width, self.height) {
            text.push_str(&format!("{}×{}", width, height));
        }

        if let Some(size) = self.file_size {
            if !text.is_empty() {
                text.push_str(" • ");
            }
            text.push_str(&Self::format_file_size(self, size));
        }

        if let Some(color_type) = &self.color_type {
            if !text.is_empty() {
                text.push_str(" • ");
            }
            text.push_str(color_type);
        }

        div().when(!text.is_empty(), |el| {
            el.child(Button::new("image-metadata", text).label_size(LabelSize::Small))
        })
    }
}

impl StatusItemView for ImageInfo {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(image_view) = active_pane_item.and_then(|item| item.act_as::<ImageView>(cx)) {
            self.update_metadata(&image_view, cx);
            self._observe_active_image = Some(cx.observe(&image_view, |this, view, cx| {
                this.update_metadata(&view, cx);
            }));
        } else {
            self.width = None;
            self.height = None;
            self.file_size = None;
            self.color_type = None;
            self._observe_active_image = None;
        }
        cx.notify();
    }
}
