use std::sync::Arc;

use assistant_slash_command::SlashCommandRegistry;

use gpui::{AnyElement, DismissEvent, SharedString, Task, WeakView};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use ui::{prelude::*, KeyBinding, ListItem, ListItemSpacing, PopoverMenu, PopoverTrigger};

use crate::assistant_panel::ContextEditor;
use crate::QuoteSelection;

#[derive(IntoElement)]
pub(super) struct SlashCommandSelector<T: PopoverTrigger> {
    registry: Arc<SlashCommandRegistry>,
    active_context_editor: WeakView<ContextEditor>,
    trigger: T,
}

#[derive(Clone)]
struct SlashCommandInfo {
    name: SharedString,
    description: SharedString,
    args: Option<SharedString>,
    icon: IconName,
}

#[derive(Clone)]
enum SlashCommandEntry {
    Info(SlashCommandInfo),
    Advert {
        name: SharedString,
        renderer: fn(&mut WindowContext<'_>) -> AnyElement,
        on_confirm: fn(&mut WindowContext<'_>),
    },
    QuoteButton,
}

impl AsRef<str> for SlashCommandEntry {
    fn as_ref(&self) -> &str {
        match self {
            SlashCommandEntry::Info(SlashCommandInfo { name, .. })
            | SlashCommandEntry::Advert { name, .. } => name,
            SlashCommandEntry::QuoteButton => "Quote Selection",
        }
    }
}

pub(crate) struct SlashCommandDelegate {
    all_commands: Vec<SlashCommandEntry>,
    filtered_commands: Vec<SlashCommandEntry>,
    active_context_editor: WeakView<ContextEditor>,
    selected_index: usize,
}

impl<T: PopoverTrigger> SlashCommandSelector<T> {
    pub(crate) fn new(
        registry: Arc<SlashCommandRegistry>,
        active_context_editor: WeakView<ContextEditor>,
        trigger: T,
    ) -> Self {
        SlashCommandSelector {
            registry,
            active_context_editor,
            trigger,
        }
    }
}

impl PickerDelegate for SlashCommandDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_commands.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_commands.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a command...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let all_commands = self.all_commands.clone();
        cx.spawn(|this, mut cx| async move {
            let filtered_commands = cx
                .background_executor()
                .spawn(async move {
                    if query.is_empty() {
                        all_commands
                    } else {
                        all_commands
                            .into_iter()
                            .filter(|model_info| {
                                model_info
                                    .as_ref()
                                    .to_lowercase()
                                    .contains(&query.to_lowercase())
                            })
                            .collect()
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.delegate.filtered_commands = filtered_commands;
                this.delegate.set_selected_index(0, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let mut ret = vec![];
        let mut previous_is_advert = false;

        for (index, command) in self.filtered_commands.iter().enumerate() {
            if previous_is_advert {
                if let SlashCommandEntry::Info(_) = command {
                    previous_is_advert = false;
                    debug_assert_ne!(
                        index, 0,
                        "index cannot be zero, as we can never have a separator at 0th position"
                    );
                    ret.push(index - 1);
                }
            } else {
                if let SlashCommandEntry::Advert { .. } = command {
                    previous_is_advert = true;
                    if index != 0 {
                        ret.push(index - 1);
                    }
                }
            }
        }
        ret
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(command) = self.filtered_commands.get(self.selected_index) {
            match command {
                SlashCommandEntry::Info(info) => {
                    self.active_context_editor
                        .update(cx, |context_editor, cx| {
                            context_editor.insert_command(&info.name, cx)
                        })
                        .ok();
                }
                SlashCommandEntry::QuoteButton => {
                    cx.dispatch_action(Box::new(QuoteSelection));
                }
                SlashCommandEntry::Advert { on_confirm, .. } => {
                    on_confirm(cx);
                }
            }
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let command_info = self.filtered_commands.get(ix)?;

        match command_info {
            SlashCommandEntry::Info(info) => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .selected(selected)
                    .child(
                        v_flex()
                            .group(format!("command-entry-label-{ix}"))
                            .w_full()
                            .min_w(px(250.))
                            .child(
                                h_flex()
                                    .gap_1p5()
                                    .child(Icon::new(info.icon).size(IconSize::XSmall))
                                    .child(div().font_buffer(cx).child({
                                        let mut label = format!("{}", info.name);
                                        if let Some(args) = info.args.as_ref().filter(|_| selected)
                                        {
                                            label.push_str(&args);
                                        }
                                        Label::new(label).size(LabelSize::Small)
                                    }))
                                    .children(info.args.clone().filter(|_| !selected).map(
                                        |args| {
                                            div()
                                                .font_buffer(cx)
                                                .child(
                                                    Label::new(args)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .visible_on_hover(format!(
                                                    "command-entry-label-{ix}"
                                                ))
                                        },
                                    )),
                            )
                            .child(
                                Label::new(info.description.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    ),
            ),
            SlashCommandEntry::QuoteButton => {
                let focus = cx.focus_handle();
                let key_binding = KeyBinding::for_action_in(&QuoteSelection, &focus, cx);

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Dense)
                        .selected(selected)
                        .child(
                            v_flex()
                                .child(
                                    h_flex()
                                        .gap_1p5()
                                        .child(Icon::new(IconName::Quote).size(IconSize::XSmall))
                                        .child(
                                            div().font_buffer(cx).child(
                                                Label::new("selection").size(LabelSize::Small),
                                            ),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap_1p5()
                                        .child(
                                            Label::new("Insert editor selection")
                                                .color(Color::Muted)
                                                .size(LabelSize::Small),
                                        )
                                        .children(key_binding.map(|kb| kb.render(cx))),
                                ),
                        ),
                )
            }
            SlashCommandEntry::Advert { renderer, .. } => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .selected(selected)
                    .child(renderer(cx)),
            ),
        }
    }
}

impl<T: PopoverTrigger> RenderOnce for SlashCommandSelector<T> {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let all_models = self
            .registry
            .featured_command_names()
            .into_iter()
            .filter_map(|command_name| {
                let command = self.registry.command(&command_name)?;
                let menu_text = SharedString::from(Arc::from(command.menu_text()));
                let label = command.label(cx);
                let args = label.filter_range.end.ne(&label.text.len()).then(|| {
                    SharedString::from(
                        label.text[label.filter_range.end..label.text.len()].to_owned(),
                    )
                });
                Some(SlashCommandEntry::Info(SlashCommandInfo {
                    name: command_name.into(),
                    description: menu_text,
                    args,
                    icon: command.icon(),
                }))
            })
            .chain([
                SlashCommandEntry::Advert {
                    name: "create-your-command".into(),
                    renderer: |cx| {
                        v_flex()
                            .w_full()
                            .child(
                                h_flex()
                                    .w_full()
                                    .font_buffer(cx)
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap_1p5()
                                            .child(Icon::new(IconName::Plus).size(IconSize::XSmall))
                                            .child(
                                                div().font_buffer(cx).child(
                                                    Label::new("create-your-command")
                                                        .size(LabelSize::Small),
                                                ),
                                            ),
                                    )
                                    .child(
                                        Icon::new(IconName::ArrowUpRight)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                Label::new("Create your custom command")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .into_any_element()
                    },
                    on_confirm: |cx| cx.open_url("https://zed.dev/docs/extensions/slash-commands"),
                },
                SlashCommandEntry::QuoteButton,
            ])
            .collect::<Vec<_>>();

        let delegate = SlashCommandDelegate {
            all_commands: all_models.clone(),
            active_context_editor: self.active_context_editor.clone(),
            filtered_commands: all_models,
            selected_index: 0,
        };

        let picker_view = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into()));
            picker
        });

        let handle = self
            .active_context_editor
            .update(cx, |this, _| this.slash_menu_handle.clone())
            .ok();
        PopoverMenu::new("model-switcher")
            .menu(move |_cx| Some(picker_view.clone()))
            .trigger(self.trigger)
            .attach(gpui::AnchorCorner::TopLeft)
            .anchor(gpui::AnchorCorner::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-16.0),
            })
            .when_some(handle, |this, handle| this.with_handle(handle))
    }
}
