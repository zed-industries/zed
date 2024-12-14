use gpui::{div, prelude::*, Model, Render, ViewContext, WeakView};
use project::ImageItem;
use workspace::{ItemHandle, StatusItemView, Workspace};

pub struct ImageInfoView {
    workspace: WeakView<Workspace>,
    width: Option<u32>,
    height: Option<u32>,
    file_size: Option<u64>,
    color_type: Option<&'static str>,
}

impl ImageInfoView {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            workspace: workspace.weak_handle(),
            width: None,
            height: None,
            file_size: None,
            color_type: None,
        }
    }

    fn format_file_size(&self) -> String {
        self.file_size.map_or("--".to_string(), |size| {
            if size < 1024 {
                format!("{} B", size)
            } else if size < 1024 * 1024 {
                format!("{:.1} KB", size as f64 / 1024.0)
            } else {
                format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
            }
        })
    }
}

impl Render for ImageInfoView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.width.is_some()
            || self.height.is_some()
            || self.file_size.is_some()
            || self.color_type.is_some()
        {
            div().flex().items_center().gap_2().text_xs().child(format!(
                "Whole image {} × {}  {}  {}",
                self.width.map_or("--".to_string(), |w| w.to_string()),
                self.height.map_or("--".to_string(), |h| h.to_string()),
                self.format_file_size(),
                self.color_type.as_deref().unwrap_or("")
            ))
        } else {
            div()
        }
    }
}

impl StatusItemView for ImageInfoView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        self.width = None;
        self.height = None;
        self.file_size = None;
        self.color_type = None;

        if let Some(item) = active_pane_item {
            if let Some(image_model) = item.downcast::<Model<ImageItem>>() {
                let image_item = image_model.read(cx);

                self.width = image_item.read(cx).width;
                self.height = image_item.read(cx).height;
                self.file_size = image_item.read(cx).file_size;
                self.color_type = image_item.read(cx).color_type;
            }
        }

        cx.notify();
    }
}
