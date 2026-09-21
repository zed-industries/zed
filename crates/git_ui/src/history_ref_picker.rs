use fuzzy_nucleo::StringMatchCandidate;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    IntoElement, ParentElement, Render, SharedString, Styled, Task, WeakEntity, Window, rems,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{
    Color, Icon, IconName, IconSize, Label, LabelSize, ListItem, ListItemSpacing, h_flex,
    prelude::*,
};
use util::ResultExt;
use workspace::ModalView;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryRefSelection {
    All,
    Auto,
    Branch(SharedString),
}

impl HistoryRefSelection {
    pub fn label(&self) -> SharedString {
        match self {
            HistoryRefSelection::All => "All".into(),
            HistoryRefSelection::Auto => "Auto".into(),
            HistoryRefSelection::Branch(name) => {
                if name.chars().count() > 12 {
                    let truncated: String = name.chars().take(11).collect();
                    format!("{truncated}…").into()
                } else {
                    name.clone()
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct HistoryRefOption {
    pub selection: HistoryRefSelection,
    pub title: SharedString,
    pub subtitle: Option<SharedString>,
    pub tag: Option<&'static str>,
    pub is_remote: bool,
}

pub struct HistoryRefPicker {
    picker: Entity<Picker<HistoryRefPickerDelegate>>,
}

impl HistoryRefPicker {
    pub fn new(
        options: Vec<HistoryRefOption>,
        current_selection: HistoryRefSelection,
        on_select: impl Fn(HistoryRefSelection, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let history_ref_picker = cx.entity().downgrade();
        let picker = cx.new(|cx| {
            let delegate = HistoryRefPickerDelegate::new(
                history_ref_picker,
                options,
                current_selection,
                Arc::new(on_select),
            );
            Picker::uniform_list(delegate, window, cx)
                .initial_width(rems(36.))
                .show_scrollbar(true)
        });
        Self { picker }
    }
}

impl ModalView for HistoryRefPicker {}

impl EventEmitter<DismissEvent> for HistoryRefPicker {}

impl Focusable for HistoryRefPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for HistoryRefPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("HistoryRefPicker")
            .child(self.picker.clone())
    }
}

pub struct HistoryRefPickerDelegate {
    history_ref_picker: WeakEntity<HistoryRefPicker>,
    options: Vec<HistoryRefOption>,
    matches: Vec<usize>,
    selected_index: usize,
    current_selection: HistoryRefSelection,
    on_select: Arc<dyn Fn(HistoryRefSelection, &mut Window, &mut App) + 'static>,
}

impl HistoryRefPickerDelegate {
    pub fn new(
        history_ref_picker: WeakEntity<HistoryRefPicker>,
        options: Vec<HistoryRefOption>,
        current_selection: HistoryRefSelection,
        on_select: Arc<dyn Fn(HistoryRefSelection, &mut Window, &mut App) + 'static>,
    ) -> Self {
        let matches = (0..options.len()).collect();
        let selected_index = options
            .iter()
            .position(|opt| opt.selection == current_selection)
            .unwrap_or(0);

        Self {
            history_ref_picker,
            options,
            matches,
            selected_index,
            current_selection,
            on_select,
        }
    }
}

impl PickerDelegate for HistoryRefPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "history ref picker"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select one/more history item references to view, type to filter".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let mut separators = Vec::new();
        for (ix, &opt_ix) in self.matches.iter().enumerate() {
            if let Some(opt) = self.options.get(opt_ix) {
                if opt.selection == HistoryRefSelection::Auto {
                    separators.push(ix);
                } else if !opt.is_remote {
                    if let Some(&next_opt_ix) = self.matches.get(ix + 1) {
                        if let Some(next_opt) = self.options.get(next_opt_ix) {
                            if next_opt.is_remote {
                                separators.push(ix);
                            }
                        }
                    }
                }
            }
        }
        separators
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |picker, cx| {
            let candidates = picker.read_with(cx, |picker, _| {
                picker
                    .delegate
                    .options
                    .iter()
                    .enumerate()
                    .map(|(ix, opt)| {
                        let text = match &opt.subtitle {
                            Some(sub) => format!("{} {}", opt.title, sub),
                            None => opt.title.to_string(),
                        };
                        StringMatchCandidate::new(ix, text)
                    })
                    .collect::<Vec<_>>()
            });

            let Some(candidates) = candidates.log_err() else {
                return;
            };

            let matches = if query.is_empty() {
                (0..candidates.len()).collect::<Vec<_>>()
            } else {
                fuzzy_nucleo::match_strings_async(
                    &candidates,
                    &query,
                    fuzzy_nucleo::Case::Smart,
                    fuzzy_nucleo::LengthPenalty::On,
                    1000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|m| m.candidate_id)
                .collect::<Vec<_>>()
            };

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(&opt_ix) = self.matches.get(self.selected_index) {
            if let Some(opt) = self.options.get(opt_ix) {
                (self.on_select)(opt.selection.clone(), window, cx);
            }
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.history_ref_picker
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let opt_idx = *self.matches.get(ix)?;
        let opt = self.options.get(opt_idx)?;

        let is_active = opt.selection == self.current_selection;

        let check_box = div()
            .w_4()
            .h_4()
            .rounded_xs()
            .border_1()
            .border_color(if is_active {
                cx.theme().colors().border_focused
            } else {
                cx.theme().colors().border
            })
            .bg(if is_active {
                cx.theme().colors().element_selected
            } else {
                gpui::transparent_black()
            })
            .flex()
            .items_center()
            .justify_center()
            .when(is_active, |this| {
                this.child(
                    Icon::new(IconName::Check)
                        .size(IconSize::XSmall)
                        .color(Color::Accent),
                )
            });

        let branch_icon = if opt.selection == HistoryRefSelection::All
            || opt.selection == HistoryRefSelection::Auto
        {
            None
        } else if opt.is_remote {
            Some(
                Icon::new(IconName::CloudDownload)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
        } else {
            Some(
                Icon::new(IconName::GitBranch)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
        };

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .gap_2()
                        .child(check_box)
                        .children(branch_icon)
                        .child(
                            h_flex()
                                .flex_1()
                                .min_w_0()
                                .gap_2()
                                .items_baseline()
                                .child(
                                    Label::new(opt.title.clone())
                                        .weight(if is_active {
                                            FontWeight::BOLD
                                        } else {
                                            FontWeight::NORMAL
                                        }),
                                )
                                .children(opt.subtitle.as_ref().map(|sub| {
                                    Label::new(sub.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                })),
                        )
                        .children(opt.tag.map(|tag| {
                            Label::new(tag)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                        })),
                ),
        )
    }
}
