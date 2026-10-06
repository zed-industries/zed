use std::sync::Arc;

use fs::Fs;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, WeakEntity};
use project::{
    Project,
    binary_downloads::{self, BinaryDownload, BinaryDownloads},
};
use theme::ActiveTheme;
use ui::{AlertModal, KeyBinding, prelude::*};

use crate::{ModalView, ToggleBinaryDownloads};

pub struct BinaryDownloadsModal {
    project: WeakEntity<Project>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    _binary_downloads_subscription: Option<Subscription>,
}

impl BinaryDownloadsModal {
    pub fn new(project: &Entity<Project>, fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> Self {
        let binary_downloads_subscription = BinaryDownloads::try_get_global(cx)
            .map(|binary_downloads| cx.observe(&binary_downloads, |_, _, cx| cx.notify()));
        Self {
            project: project.downgrade(),
            fs,
            focus_handle: cx.focus_handle(),
            _binary_downloads_subscription: binary_downloads_subscription,
        }
    }

    fn pending_downloads(&self, cx: &App) -> Vec<BinaryDownload> {
        self.project
            .upgrade()
            .map(|project| binary_downloads::pending_downloads(project.read(cx), cx))
            .unwrap_or_default()
    }

    fn allow(&self, downloads: Vec<BinaryDownload>, cx: &mut Context<Self>) {
        let (Some(project), Some(binary_downloads)) =
            (self.project.upgrade(), BinaryDownloads::try_get_global(cx))
        else {
            return;
        };
        telemetry::event!(
            "Binary Downloads Allowed",
            source = "Binary Downloads Modal",
            count = downloads.len()
        );
        binary_downloads.update(cx, |binary_downloads, cx| {
            for download in downloads {
                binary_downloads.allow_download(&project, download, cx);
            }
        });
    }

    fn deny(&self, downloads: Vec<BinaryDownload>, cx: &mut Context<Self>) {
        let (Some(project), Some(binary_downloads)) =
            (self.project.upgrade(), BinaryDownloads::try_get_global(cx))
        else {
            return;
        };
        telemetry::event!(
            "Binary Downloads Denied",
            source = "Binary Downloads Modal",
            count = downloads.len()
        );
        binary_downloads.update(cx, |binary_downloads, cx| {
            for download in downloads {
                binary_downloads.deny_download(&project, download, cx);
            }
        });
    }

    fn always_allow(&self, cx: &mut Context<Self>) {
        telemetry::event!(
            "Binary Downloads Always Allowed",
            source = "Binary Downloads Modal"
        );
        self.allow(self.pending_downloads(cx), cx);
        settings::update_settings_file(self.fs.clone(), cx, |settings, _| {
            settings.allow_binary_downloads = Some(true);
        });
        cx.emit(DismissEvent);
    }
}

impl Focusable for BinaryDownloadsModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for BinaryDownloadsModal {}

impl ModalView for BinaryDownloadsModal {}

impl Render for BinaryDownloadsModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pending_downloads = self.pending_downloads(cx);
        if pending_downloads.is_empty() {
            cx.emit(DismissEvent);
            return v_flex().into_any_element();
        }

        AlertModal::new("binary-downloads-modal")
            .width(rems(34.))
            .key_context("BinaryDownloadsModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|modal, _: &menu::Confirm, _, cx| {
                modal.allow(modal.pending_downloads(cx), cx);
            }))
            .on_action(cx.listener(|_, _: &ToggleBinaryDownloads, _, cx| {
                cx.emit(DismissEvent);
            }))
            .header(
                v_flex()
                    .p_3()
                    .gap_1()
                    .rounded_t_md()
                    .bg(cx.theme().colors().editor_background.opacity(0.5))
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Icon::new(IconName::CloudDownload).color(Color::Warning))
                            .child(Label::new("Downloads Awaiting Approval")),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Label::new(
                            "Binary downloads are disabled. Zed will not download these tools until you allow it. Denied tools stay blocked until Zed restarts.",
                        )
                        .color(Color::Muted),
                    )
                    .children(pending_downloads.into_iter().enumerate().map(
                        |(index, download)| {
                            h_flex()
                                .justify_between()
                                .gap_2()
                                .child(Label::new(download.tool.clone()))
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Button::new(("deny-binary-download", index), "Deny")
                                                .label_size(LabelSize::Small)
                                                .on_click(cx.listener({
                                                    let download = download.clone();
                                                    move |modal, _, _, cx| {
                                                        modal.deny(vec![download.clone()], cx);
                                                        cx.stop_propagation();
                                                    }
                                                })),
                                        )
                                        .child(
                                            Button::new(
                                                ("allow-binary-download", index),
                                                "Allow",
                                            )
                                            .label_size(LabelSize::Small)
                                            .on_click(cx.listener(
                                                move |modal, _, _, cx| {
                                                    modal.allow(vec![download.clone()], cx);
                                                    cx.stop_propagation();
                                                },
                                            )),
                                        ),
                                )
                        },
                    )),
            )
            .footer(
                h_flex()
                    .px_3()
                    .pb_3()
                    .gap_1()
                    .justify_end()
                    .child(
                        Button::new("deny-all-binary-downloads", "Deny All").on_click(
                            cx.listener(|modal, _, _, cx| {
                                modal.deny(modal.pending_downloads(cx), cx);
                                cx.stop_propagation();
                            }),
                        ),
                    )
                    .child(
                        Button::new("always-allow-binary-downloads", "Always Allow Downloads")
                            .on_click(cx.listener(|modal, _, _, cx| {
                                modal.always_allow(cx);
                                cx.stop_propagation();
                            })),
                    )
                    .child(
                        Button::new("allow-all-binary-downloads", "Allow All")
                            .style(ButtonStyle::Filled)
                            .layer(ui::ElevationIndex::ModalSurface)
                            .key_binding(
                                KeyBinding::for_action(&menu::Confirm, cx)
                                    .map(|key_binding| key_binding.size(rems_from_px(12_f32))),
                            )
                            .on_click(cx.listener(|modal, _, _, cx| {
                                modal.allow(modal.pending_downloads(cx), cx);
                                cx.stop_propagation();
                            })),
                    ),
            )
            .into_any_element()
    }
}
