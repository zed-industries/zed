use std::{cmp::Reverse, rc::Rc, sync::Arc};

use acp_thread::{AgentModelSelector, LanguageModelInfo, LanguageModelInfoList};
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{BackgroundExecutor, DismissEvent, Task};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use ui::{
    AnyElement, App, Context, ListItem, ListItemSpacing, SharedString, Window, prelude::*, rems,
};

pub type AcpModelSelector = Picker<AcpModelPickerDelegate>;

pub fn acp_model_selector(
    selector: Rc<dyn AgentModelSelector>,
    window: &mut Window,
    cx: &mut Context<AcpModelSelector>,
) -> AcpModelSelector {
    let delegate = AcpModelPickerDelegate::new(selector, window, cx);
    Picker::list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems(20.))
        .max_height(Some(rems(20.).into()))
}

enum AcpModelPickerEntry {
    Separator(SharedString),
    Model(LanguageModelInfo),
}

pub struct AcpModelPickerDelegate {
    selector: Rc<dyn AgentModelSelector>,
    filtered_entries: Vec<AcpModelPickerEntry>,
    models: Option<LanguageModelInfoList>,
    selected_index: usize,
    selected_model: Option<LanguageModelInfo>,
}

impl AcpModelPickerDelegate {
    fn new(
        selector: Rc<dyn AgentModelSelector>,
        window: &mut Window,
        cx: &mut Context<AcpModelSelector>,
    ) -> Self {
        Self {
            selector,
            filtered_entries: Vec::new(),
            models: None,
            selected_model: None,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for AcpModelPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_entries.len().saturating_sub(1));
        cx.notify();
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        match self.filtered_entries.get(ix) {
            Some(AcpModelPickerEntry::Model(_)) => true,
            Some(AcpModelPickerEntry::Separator(_)) | None => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a model…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_models = self.all_models.clone();
        let active_model = (self.get_active_model)(cx);
        let bg_executor = cx.background_executor();

        let matcher_rec = ModelMatcher::new(recommended_models, bg_executor.clone());
        let matcher_all = ModelMatcher::new(available_models, bg_executor.clone());

        let recommended = matcher_rec.exact_search(&query);
        let all = matcher_all.fuzzy_search(&query);

        let filtered_models = GroupedModels::new(all, recommended);

        cx.spawn_in(window, async move |this, cx| {
            this.update_in(cx, |this, window, cx| {
                this.delegate.filtered_entries = filtered_models.entries();
                // Finds the currently selected model in the list
                let new_index =
                    Self::get_active_model_index(&this.delegate.filtered_entries, active_model);
                this.set_selected_index(new_index, Some(picker::Direction::Down), true, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(AcpModelPickerEntry::Model(model_info)) =
            self.filtered_entries.get(self.selected_index)
        {
            let model = model_info.model.clone();
            (self.on_model_changed)(model.clone(), cx);

            let current_index = self.selected_index;
            self.set_selected_index(current_index, window, cx);

            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered_entries.get(ix)? {
            AcpModelPickerEntry::Separator(title) => Some(
                div()
                    .px_2()
                    .pb_1()
                    .when(ix > 1, |this| {
                        this.mt_1()
                            .pt_2()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        Label::new(title)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            ),
            AcpModelPickerEntry::Model(model_info) => {
                let active_model = (self.get_active_model)(cx);
                let active_provider_id = active_model.as_ref().map(|m| m.provider.id());
                let active_model_id = active_model.map(|m| m.model.id());

                let is_selected = Some(model_info.model.provider_id()) == active_provider_id
                    && Some(model_info.model.id()) == active_model_id;

                let model_icon_color = if is_selected {
                    Color::Accent
                } else {
                    Color::Muted
                };

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .start_slot(
                            Icon::new(model_info.icon)
                                .color(model_icon_color)
                                .size(IconSize::Small),
                        )
                        .child(
                            h_flex()
                                .w_full()
                                .pl_0p5()
                                .gap_1p5()
                                .w(px(240.))
                                .child(Label::new(model_info.model.name().0.clone()).truncate()),
                        )
                        .end_slot(div().pr_3().when(is_selected, |this| {
                            this.child(
                                Icon::new(IconName::Check)
                                    .color(Color::Accent)
                                    .size(IconSize::Small),
                            )
                        }))
                        .into_any_element(),
                )
            }
        }
    }

    fn render_footer(
        &self,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        use feature_flags::FeatureFlagAppExt;

        let plan = proto::Plan::ZedPro;

        Some(
            h_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .gap_4()
                .justify_between()
                .when(cx.has_flag::<ZedProFeatureFlag>(), |this| {
                    this.child(match plan {
                        Plan::ZedPro => Button::new("zed-pro", "Zed Pro")
                            .icon(IconName::ZedAssistant)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .icon_position(IconPosition::Start)
                            .on_click(|_, window, cx| {
                                window
                                    .dispatch_action(Box::new(zed_actions::OpenAccountSettings), cx)
                            }),
                        Plan::Free | Plan::ZedProTrial => Button::new(
                            "try-pro",
                            if plan == Plan::ZedProTrial {
                                "Upgrade to Pro"
                            } else {
                                "Try Pro"
                            },
                        )
                        .on_click(|_, _, cx| cx.open_url(TRY_ZED_PRO_URL)),
                    })
                })
                .child(
                    Button::new("configure", "Configure")
                        .icon(IconName::Settings)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::Start)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(
                                zed_actions::agent::OpenSettings.boxed_clone(),
                                cx,
                            );
                        }),
                )
                .into_any(),
        )
    }
}

pub async fn fuzzy_search(
    model_list: &LanguageModelInfoList,
    query: &str,
    executor: BackgroundExecutor,
) -> LanguageModelInfoList {
    let candidates = model_list
        .all_models()
        .enumerate()
        .map(|(ix, model)| StringMatchCandidate::new(ix, model.id.0.as_ref()))
        .collect::<Vec<_>>();

    let mut matches = match_strings(
        &candidates,
        &query,
        false,
        true,
        100,
        &Default::default(),
        executor,
    )
    .await;

    matches.sort_unstable_by_key(|mat| {
        let candidate = &candidates[mat.candidate_id];
        (Reverse(OrderedFloat(mat.score)), candidate.id)
    });

    let matched_models: Vec<_> = matches
        .into_iter()
        .map(|mat| self.models[mat.candidate_id].clone())
        .collect();

    matched_models
}
