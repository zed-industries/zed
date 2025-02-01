use crate::ImageView;
use anyhow;
use gpui::{div, Context, Entity, IntoElement, ParentElement, Render, Subscription};
use project::image_store::ImageMetadata;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use ui::{prelude::*, Button, LabelSize, Window};
use workspace::{ItemHandle, StatusItemView, Workspace};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct ImageViewerSettings {
    #[serde(default)]
    unit_type: ImageFileSizeUnitType,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageFileSizeUnitType {
    #[default]
    Binary,
    Decimal,
}

impl Settings for ImageViewerSettings {
    const KEY: Option<&'static str> = Some("image_viewer");

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut App,
    ) -> Result<Self, anyhow::Error> {
        sources.json_merge().or_else(|_| Ok(Self::default()))
    }
}

pub struct ImageInfo {
    metadata: Option<ImageMetadata>,
    _observe_active_image: Option<Subscription>,
}

impl ImageInfo {
    pub fn new(_workspace: &Workspace) -> Self {
        Self {
            metadata: None,
            _observe_active_image: None,
        }
    }

    fn update_metadata(&mut self, image_view: &Entity<ImageView>, cx: &mut Context<Self>) {
        let image_item = image_view.read(cx).image_item.read(cx);
        self.metadata = image_item.image_meta.clone();
        cx.notify();
    }

    fn format_file_size(&self, image_unit_type: &ImageFileSizeUnitType) -> Option<String> {
        self.metadata.as_ref().map(|meta| {
            let size = meta.file_size;
            match image_unit_type {
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
        })
    }
}

impl Render for ImageInfo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ImageViewerSettings::get_global(cx);
        let unit_type = &settings.unit_type;

        let components = [
            self.metadata
                .as_ref()
                .map(|meta| format!("{}x{}", meta.width, meta.height)),
            self.format_file_size(unit_type),
            self.metadata
                .as_ref()
                .map(|meta| meta.color_type.to_string()),
            self.metadata.as_ref().map(|meta| meta.format.clone()),
        ];

        let text = components
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" • ");

        div().when(!text.is_empty(), |el| {
            el.child(Button::new("image-metadata", text).label_size(LabelSize::Small))
        })
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

        if let Some(image_view) = active_pane_item.and_then(|item| item.act_as::<ImageView>(cx)) {
            self.update_metadata(&image_view, cx);
            self._observe_active_image = Some(cx.observe(&image_view, |this, view, cx| {
                this.update_metadata(&view, cx);
            }));
        } else {
            self.metadata = None
        }
        cx.notify();
    }
}
