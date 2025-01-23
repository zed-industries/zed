use std::sync::Arc;

use client::UserStore;
use fs::Fs;
use gpui::{Model, Render, View, WeakView};
use ui::{prelude::*, ButtonLike};
use workspace::Workspace;

use crate::ZedPredictModal;

/// Prompts user to try AI inline prediction feature
pub struct ZedPredictBanner {
    workspace: WeakView<Workspace>,
    user_store: Model<UserStore>,
    fs: Arc<dyn Fs>,
}

impl ZedPredictBanner {
    pub fn new(
        workspace: WeakView<Workspace>,
        user_store: Model<UserStore>,
        fs: Arc<dyn Fs>,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_| ZedPredictBanner {
            workspace,
            user_store,
            fs,
        })
    }
}

impl Render for ZedPredictBanner {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .h_5()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().editor_foreground.opacity(0.3))
            .child(
                ButtonLike::new("try-zed-predict")
                    .child(
                        h_flex()
                            .h_full()
                            .items_center()
                            .gap_1p5()
                            .child(Icon::new(IconName::ZedPredict).size(IconSize::Small))
                            .child(
                                h_flex()
                                    .gap_0p5()
                                    .child(
                                        Label::new("Introducing:")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(Label::new("Edit Prediction").size(LabelSize::Small)),
                            ),
                    )
                    .on_click({
                        let workspace = self.workspace.clone();
                        let user_store = self.user_store.clone();
                        let fs = self.fs.clone();
                        move |_, cx| {
                            let Some(workspace) = workspace.upgrade() else {
                                return;
                            };
                            ZedPredictModal::toggle(workspace, user_store.clone(), fs.clone(), cx);
                        }
                    }),
            )
            .child(
                div()
                    .border_l_1()
                    .border_color(cx.theme().colors().editor_foreground.opacity(0.1))
                    .child(
                        IconButton::new("close", IconName::Close).icon_size(IconSize::Indicator),
                    ),
            )
    }
}
