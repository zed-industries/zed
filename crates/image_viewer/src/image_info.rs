use gpui::{Context, Entity, IntoElement, ParentElement, Render, Subscription, div};
use project::image_store::{ImageFormat, ImageMetadata};
use settings::Settings;
use ui::prelude::*;
use util::size::format_file_size;
use workspace::{ItemHandle, StatusItemView, Workspace};

use crate::{ImageFileSizeUnit, ImageView, ImageViewerSettings};

pub struct ImageInfo {
    metadata: Option<ImageMetadata>,
    _observe_active_image: Option<Subscription>,
    observe_image_item: Option<Subscription>,
}

impl ImageInfo {
    pub fn new(_workspace: &Workspace) -> Self {
        Self {
            metadata: None,
            _observe_active_image: None,
            observe_image_item: None,
        }
    }

    fn update_metadata(&mut self, image_view: &Entity<ImageView>, cx: &mut Context<Self>) {
        let image_item = image_view.read(cx).image_item.clone();
        let current_metadata = image_item.read(cx).image_metadata;
        if current_metadata.is_some() {
            self.metadata = current_metadata;
            cx.notify();
        } else {
            self.observe_image_item = Some(cx.observe(&image_item, |this, item, cx| {
                this.metadata = item.read(cx).image_metadata;
                cx.notify();
            }));
        }
    }
}

fn format_image_size(size: u64, image_unit_type: ImageFileSizeUnit) -> String {
    let use_decimal = matches!(image_unit_type, ImageFileSizeUnit::Decimal);
    format_file_size(size, use_decimal)
}

impl Render for ImageInfo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ImageViewerSettings::get_global(cx);

        let Some(metadata) = self.metadata.as_ref() else {
            return div().hidden();
        };

        let mut components = Vec::new();
        components.push(format!("{}x{}", metadata.width, metadata.height));
        components.push(format_image_size(metadata.file_size, settings.unit));

        if let Some(colors) = metadata.colors {
            components.push(format!(
                "{} channels, {} bits per pixel",
                colors.channels,
                colors.bits_per_pixel()
            ));
        }

        components.push(
            match metadata.format {
                ImageFormat::Png => "PNG",
                ImageFormat::Jpeg => "JPEG",
                ImageFormat::Gif => "GIF",
                ImageFormat::WebP => "WebP",
                ImageFormat::Tiff => "TIFF",
                ImageFormat::Bmp => "BMP",
                ImageFormat::Ico => "ICO",
                ImageFormat::Avif => "Avif",
                _ => "Unknown",
            }
            .to_string(),
        );

        div().child(
            Button::new("image-metadata", components.join(" • ")).label_size(LabelSize::Small),
        )
    }
}

impl StatusItemView for ImageInfo {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self._observe_active_image = None;
        self.observe_image_item = None;

        if let Some(image_view) = active_pane_item.and_then(|item| item.act_as::<ImageView>(cx)) {
            self.update_metadata(&image_view, cx);

            self._observe_active_image = Some(cx.observe(&image_view, |this, view, cx| {
                this.update_metadata(&view, cx);
            }));
        } else {
            self.metadata = None;
        }
        cx.notify();
    }
}
