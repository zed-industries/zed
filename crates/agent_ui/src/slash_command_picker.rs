use crate::text_thread_editor::TextThreadEditor;
use assistant_slash_command::SlashCommandWorkingSet;
use gpui::{AnyElement, AnyView, DismissEvent, SharedString, Task, WeakEntity};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, PopoverMenu, PopoverTrigger, Tooltip, prelude::*};

#[derive(IntoElement)]
pub(super) struct SlashCommandSelector<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    working_set: Arc<SlashCommandWorkingSet>,
    active_context_editor: WeakEntity<TextThreadEditor>,
    trigger: T,
    tooltip: TT,
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
        renderer: fn(&mut Window, &mut App) -> AnyElement,
        on_confirm: fn(&mut Window, &mut App),
    },
}

impl AsRef<str> for SlashCommandEntry {
    fn as_ref(&self) -> &str {
        match self {
            SlashCommandEntry::Info(SlashCommandInfo { name, .. })
            | SlashCommandEntry::Advert { name, .. } => name,
        }
    }
}

pub(crate) struct SlashCommandDelegate {
    all_commands: Vec<SlashCommandEntry>,
    filtered_commands: Vec<SlashCommandEntry>,
    active_context_editor: WeakEntity<TextThreadEditor>,
    selected_index: usize,
}

impl<T, TT> SlashCommandSelector<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    pub(crate) fn new(
        working_set: Arc<SlashCommandWorkingSet>,
        active_context_editor: WeakEntity<TextThreadEditor>,
        trigger: T,
        tooltip: TT,
    ) -> Self {
        SlashCommandSelector {
            working_set,
            active_context_editor,
            trigger,
            tooltip,
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

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_commands.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a command...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_commands = self.all_commands.clone();
        cx.spawn_in(window, async move |this, cx| {
            let filtered_commands = cx
                .background_spawn(async move {
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

            this.update_in(cx, |this, window, cx| {
                this.delegate.filtered_commands = filtered_commands;
                this.delegate.set_selected_index(0, window, cx);
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
            } else if let SlashCommandEntry::Advert { .. } = command {
                previous_is_advert = true;
                if index != 0 {
                    ret.push(index - 1);
                }
            }
        }
        ret
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(command) = self.filtered_commands.get(self.selected_index) {
            match command {
                SlashCommandEntry::Info(info) => {
                    self.active_context_editor
                        .update(cx, |text_thread_editor, cx| {
                            text_thread_editor.insert_command(&info.name, window, cx)
                        })
                        .ok();
                }
                SlashCommandEntry::Advert { on_confirm, .. } => {
                    on_confirm(window, cx);
                }
            }
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let command_info = self.filtered_commands.get(ix)?;

        match command_info {
            SlashCommandEntry::Info(info) => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .toggle_state(selected)
                    .tooltip({
                        let description = info.description.clone();
                        move |_, cx| cx.new(|_| Tooltip::new(description.clone())).into()
                    })
                    .child(
                        v_flex()
                            .group(format!("command-entry-label-{ix}"))
                            .w_full()
                            .py_0p5()
                            .min_w(px(250.))
                            .max_w(px(400.))
                            .child(
                                h_flex()
                                    .gap_1p5()
                                    .child(
                                        Icon::new(info.icon)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child({
                                        let mut label = format!("{}", info.name);
                                        if let Some(args) = info.args.as_ref().filter(|_| selected)
                                        {
                                            label.push_str(args);
                                        }
                                        Label::new(label)
                                            .single_line()
                                            .size(LabelSize::Small)
                                            .buffer_font(cx)
                                    })
                                    .children(info.args.clone().filter(|_| !selected).map(
                                        |args| {
                                            div()
                                                .child(
                                                    Label::new(args)
                                                        .single_line()
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted)
                                                        .buffer_font(cx),
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
                                    .color(Color::Muted)
                                    .truncate(),
                            ),
                    ),
            ),
            SlashCommandEntry::Advert { renderer, .. } => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .toggle_state(selected)
                    .child(renderer(window, cx)),
            ),
        }
    }
}

impl<T, TT> RenderOnce for SlashCommandSelector<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let all_models = self
            .working_set
            .featured_command_names(cx)
            .into_iter()
            .filter_map(|command_name| {
                let command = self.working_set.command(&command_name, cx)?;
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
            .chain([SlashCommandEntry::Advert {
                name: "create-your-command".into(),
                renderer: |_, cx| {
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
                                            Label::new("create-your-command")
                                                .size(LabelSize::Small)
                                                .buffer_font(cx),
                                        ),
                                )
                                .child(
                                    Icon::new(IconName::ArrowUpRight)
                                        .size(IconSize::Small)
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
                on_confirm: |_, cx| cx.open_url("https://zed.dev/docs/extensions/slash-commands"),
            }])
            .collect::<Vec<_>>();

        let delegate = SlashCommandDelegate {
            all_commands: all_models.clone(),
            active_context_editor: self.active_context_editor.clone(),
            filtered_commands: all_models,
            selected_index: 0,
        };

        let picker_view = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx).max_height(Some(rems(20.).into()))
        });

        let handle = self
            .active_context_editor
            .read_with(cx, |this, _| this.slash_menu_handle.clone())
            .ok();
        PopoverMenu::new("model-switcher")
            .menu(move |_window, _cx| Some(picker_view.clone()))
            .trigger_with_tooltip(self.trigger, self.tooltip)
            .attach(gpui::Corner::TopLeft)
            .anchor(gpui::Corner::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
            .when_some(handle, |this, handle| this.with_handle(handle))
    }
}
