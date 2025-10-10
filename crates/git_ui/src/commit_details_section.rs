use gpui::{Pixels, ScrollHandle, Window};
use theme::ActiveTheme;
use ui::{IntoElement, ParentElement, Render, WithScrollbar, div, h_flex, prelude::*, v_flex};

#[derive(Clone, Debug)]
pub struct CommitDetails {
    pub hash: String,
    pub author: String,
    pub author_email: String,
    pub date: String,
    pub full_message: String,
    pub files: Vec<crate::files_changed_section::CommitFileInfo>,
}

pub struct CommitDetailsSection {
    commit_details_scroll_handle: ScrollHandle,
}

impl CommitDetailsSection {
    pub fn new() -> Self {
        Self {
            commit_details_scroll_handle: ScrollHandle::new(),
        }
    }

    pub fn render(
        &mut self,
        details: Option<&CommitDetails>,
        height: Pixels,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> impl IntoElement {
        let text_muted = cx.theme().colors().text_muted;
        let text = cx.theme().colors().text;

        div()
            .h(height)
            .relative()
            .child(if let Some(details) = details {
                div()
                    .size_full()
                    .child(
                        div()
                            .id("commit-details-scroll")
                            .size_full()
                            .overflow_y_scroll()
                            .track_scroll(&self.commit_details_scroll_handle)
                            .p_2()
                            .child(
                                v_flex()
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_color(text)
                                            .text_xs()
                                            .font_family("monospace")
                                            .child(details.full_message.clone()),
                                    )
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_color(text_muted)
                                                            .text_xs()
                                                            .child("Commit:"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(text)
                                                            .text_xs()
                                                            .font_family("monospace")
                                                            .child(details.hash.clone()),
                                                    ),
                                            )
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_color(text_muted)
                                                            .text_xs()
                                                            .child("Author:"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(text)
                                                            .text_xs()
                                                            .child(details.author.clone()),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(text_muted)
                                                            .text_xs()
                                                            .child(format!(
                                                                "<{}>",
                                                                details.author_email
                                                            )),
                                                    ),
                                            )
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_color(text_muted)
                                                            .text_xs()
                                                            .child("Date:"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(text)
                                                            .text_xs()
                                                            .child(details.date.clone()),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .custom_scrollbars(
                        ui::Scrollbars::new(ui::ScrollAxes::Vertical)
                            .tracked_scroll_handle(self.commit_details_scroll_handle.clone())
                            .id("commit-details-scrollbar"),
                        window,
                        cx,
                    )
            } else {
                div()
                    .id("commit-details-empty")
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_color(text_muted)
                            .text_sm()
                            .child("Select a commit to view details"),
                    )
            })
    }
}

impl Render for CommitDetailsSection {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
