use crate::{InlineCompletion, Zeta};
use editor::Editor;
use gpui::{
    prelude::*, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, View,
    ViewContext,
};
use ui::{prelude::*, ListItem, ListItemSpacing};
use workspace::{ModalView, Workspace};

pub struct RateCompletionModal {
    zeta: Model<Zeta>,
    active_completion: Option<ActiveCompletion>,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
}

struct ActiveCompletion {
    completion: InlineCompletion,
    feedback_editor: View<Editor>,
}

impl RateCompletionModal {
    pub fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        if let Some(zeta) = Zeta::global(cx) {
            workspace.toggle_modal(cx, |cx| RateCompletionModal::new(zeta, cx));
        }
    }

    pub fn new(zeta: Model<Zeta>, cx: &mut ViewContext<Self>) -> Self {
        let subscription = cx.observe(&zeta, |_, _, cx| cx.notify());
        Self {
            zeta,
            focus_handle: cx.focus_handle(),
            active_completion: None,
            _subscription: subscription,
        }
    }

    pub fn select_completion(
        &mut self,
        completion: Option<InlineCompletion>,
        cx: &mut ViewContext<Self>,
    ) {
        // Avoid resetting completion rating if it's already selected.
        if let Some(completion) = completion.as_ref() {
            if let Some(prev_completion) = self.active_completion.as_ref() {
                if completion.id == prev_completion.completion.id {
                    return;
                }
            }
        }

        self.active_completion = completion.map(|completion| ActiveCompletion {
            completion,
            feedback_editor: cx.new_view(|cx| Editor::multi_line(cx)),
        });
    }
}

impl Render for RateCompletionModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .bg(cx.theme().colors().elevated_surface_background)
            .w(cx.viewport_size().width - px(256.))
            .h(cx.viewport_size().height - px(256.))
            .rounded_lg()
            .shadow_lg()
            .p_2()
            .track_focus(&self.focus_handle)
            .child(
                div().w_96().h_full().child(
                    ui::List::new()
                        .empty_message(
                            "No completions, use the editor to generate some and rate them!",
                        )
                        .children(self.zeta.read(cx).recent_completions().cloned().map(
                            |completion| {
                                let selected =
                                    self.active_completion.as_ref().map_or(false, |selected| {
                                        selected.completion.id == completion.id
                                    });
                                let rated = self.zeta.read(cx).is_completion_rated(completion.id);
                                ListItem::new(completion.id)
                                    .spacing(ListItemSpacing::Sparse)
                                    .selected(selected)
                                    .end_slot(if rated {
                                        Icon::new(IconName::Check).color(Color::Success)
                                    } else if completion.edits.is_empty() {
                                        Icon::new(IconName::Ellipsis).color(Color::Muted)
                                    } else {
                                        Icon::new(IconName::Diff).color(Color::Muted)
                                    })
                                    .child(Label::new(completion.id.to_string()))
                                    .on_click(cx.listener(move |this, _, cx| {
                                        this.select_completion(Some(completion.clone()), cx);
                                    }))
                            },
                        )),
                ),
            )
            .children(self.active_completion.as_ref().map(|completion| {
                v_flex()
                    .flex_1()
                    .size_full()
                    .child(SharedString::from(
                        completion.completion.output_excerpt.clone(),
                    ))
                    .child(completion.feedback_editor.clone())
            }))
            .on_mouse_down_out(cx.listener(|_, _, cx| cx.emit(DismissEvent)))
    }
}

impl EventEmitter<DismissEvent> for RateCompletionModal {}

impl FocusableView for RateCompletionModal {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for RateCompletionModal {}
