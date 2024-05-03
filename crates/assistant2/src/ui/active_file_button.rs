use crate::attachments::{ActiveEditorAttachmentTool, UserAttachmentStore};
use gpui::prelude::*;
use std::sync::Arc;
use ui::{prelude::*, ButtonLike, Color, Icon, IconName, Tooltip};

#[derive(Clone)]
enum Status {
    ActiveFile(String),
    #[allow(dead_code)]
    NoFile,
}

pub struct ActiveFileButton {
    attachment_store: Arc<UserAttachmentStore>,
}

impl ActiveFileButton {
    pub fn new(attachment_store: Arc<UserAttachmentStore>) -> Self {
        Self { attachment_store }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.attachment_store
            .set_attachment_tool_enabled::<ActiveEditorAttachmentTool>(enabled);
    }
}

impl Render for ActiveFileButton {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_enabled = self
            .attachment_store
            .is_attachment_tool_enabled::<ActiveEditorAttachmentTool>();

        let icon = if is_enabled {
            Icon::new(IconName::File)
                .size(IconSize::XSmall)
                .color(Color::Default)
        } else {
            Icon::new(IconName::File)
                .size(IconSize::XSmall)
                .color(Color::Disabled)
        };

        let indicator = None;

        let status = Status::ActiveFile("example-of-filename".to_string());

        ButtonLike::new("active-file-button")
            .child(
                ui::IconWithIndicator::new(icon, indicator)
                    .indicator_border_color(Some(gpui::transparent_black())),
            )
            .tooltip({
                move |cx| {
                    let status = status.clone();
                    let (tooltip, meta) = match (is_enabled, status) {
                        (false, _) => (
                            "Active file disabled".to_string(),
                            Some("Click to enable".to_string()),
                        ),
                        (true, Status::ActiveFile(file)) => {
                            ("Active file enabled".to_string(), Some(file))
                        }
                        (true, Status::NoFile) => {
                            ("No file active for conversation".to_string(), None)
                        }
                    };

                    if let Some(meta) = meta {
                        Tooltip::with_meta(tooltip, None, meta, cx)
                    } else {
                        Tooltip::text(tooltip, cx)
                    }
                }
            })
            .on_click(cx.listener(move |this, _, cx| {
                dbg!("Active file button clicked");
                dbg!(is_enabled);
                this.set_enabled(!is_enabled);
                cx.notify();
            }))
    }
}
