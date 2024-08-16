use assistant_slash_command::SlashCommandRegistry;
use gpui::DismissEvent;
use gpui::WeakView;
use picker::PickerEditorPosition;

use std::sync::Arc;
use ui::ListItemSpacing;

use gpui::SharedString;
use gpui::Task;
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

use crate::assistant_panel::ContextEditor;

#[derive(IntoElement)]
pub struct SlashCommandSelector<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<Picker<SlashCommandDelegate>>>,
    registry: Arc<SlashCommandRegistry>,
    active_context_editor: WeakView<ContextEditor>,
    trigger: T,
    info_text: Option<SharedString>,
}

#[derive(Clone)]
struct SlashCommandInfo {
    name: SharedString,
    description: SharedString,
}

pub struct SlashCommandDelegate {
    all_commands: Vec<SlashCommandInfo>,
    filtered_commands: Vec<SlashCommandInfo>,
    active_context_editor: WeakView<ContextEditor>,
    selected_index: usize,
}

impl<T: PopoverTrigger> SlashCommandSelector<T> {
    pub fn new(
        registry: Arc<SlashCommandRegistry>,
        active_context_editor: WeakView<ContextEditor>,
        trigger: T,
    ) -> Self {
        SlashCommandSelector {
            handle: None,
            registry,
            active_context_editor,
            trigger,
            info_text: None,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<Picker<SlashCommandDelegate>>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn with_info_text(mut self, text: impl Into<SharedString>) -> Self {
        self.info_text = Some(text.into());
        self
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
                                    .name
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

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(command) = self.filtered_commands.get(self.selected_index) {
            self.active_context_editor
                .update(cx, |context_editor, cx| {
                    context_editor.insert_command(&command.name, cx)
                })
                .ok();
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
        _: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let command_info = self.filtered_commands.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(
                    h_flex().w_full().min_w(px(220.)).child(
                        v_flex()
                            .child(
                                Label::new(format!("/{}", command_info.name))
                                    .size(LabelSize::Small),
                            )
                            .child(
                                Label::new(command_info.description.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    ),
                ),
        )
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
                Some(SlashCommandInfo {
                    name: command_name.into(),
                    description: menu_text,
                })
            })
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

        PopoverMenu::new("model-switcher")
            .menu(move |_cx| Some(picker_view.clone()))
            .trigger(self.trigger)
            .attach(gpui::AnchorCorner::TopLeft)
            .anchor(gpui::AnchorCorner::BottomLeft)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-16.0),
            })
    }
}
