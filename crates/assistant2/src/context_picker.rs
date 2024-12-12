mod file_context_picker;

use std::sync::Arc;

use gpui::{DismissEvent, SharedString, Task, View, WeakView};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use ui::{
    prelude::*, IconButtonShape, ListItem, ListItemSpacing, PopoverMenu, PopoverTrigger, Tooltip,
};
use workspace::notifications::NotificationHandle;
use workspace::Workspace;

use crate::context_picker::file_context_picker::FileContextPicker;
use crate::message_editor::MessageEditor;

#[derive(Debug, Clone)]
enum ContextPickerMode {
    Default,
    File(View<FileContextPicker>),
}

pub(super) struct ContextPicker {
    workspace: WeakView<Workspace>,
    message_editor: WeakView<MessageEditor>,
    picker: View<Picker<ContextPickerDelegate>>,
}

impl ContextPicker {
    pub(crate) fn new(
        workspace: WeakView<Workspace>,
        message_editor: WeakView<MessageEditor>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let entries = vec![
            ContextPickerEntry {
                name: "directory".into(),
                description: "Insert any directory".into(),
                icon: IconName::Folder,
            },
            ContextPickerEntry {
                name: "file".into(),
                description: "Insert any file".into(),
                icon: IconName::File,
            },
            ContextPickerEntry {
                name: "web".into(),
                description: "Fetch content from URL".into(),
                icon: IconName::Globe,
            },
        ];

        let delegate = ContextPickerDelegate {
            workspace: workspace.clone(),
            mode: ContextPickerMode::Default,
            all_entries: entries.clone(),
            message_editor: message_editor.clone(),
            filtered_entries: entries,
            selected_ix: 0,
        };

        let picker =
            cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into())));

        ContextPicker {
            workspace,
            message_editor,
            picker,
        }
    }
}

impl Render for ContextPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let picker = self.picker.clone();
        let mode = picker.read(cx).delegate.mode.clone();

        match mode {
            ContextPickerMode::Default => {
                let handle = self
                    .message_editor
                    .update(cx, |this, _| this.context_picker_handle.clone())
                    .ok();

                PopoverMenu::new("context-picker")
                    .menu(move |_cx| Some(picker.clone()))
                    .trigger(
                        IconButton::new("add-context", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small),
                    )
                    .attach(gpui::AnchorCorner::TopLeft)
                    .anchor(gpui::AnchorCorner::BottomLeft)
                    .offset(gpui::Point {
                        x: px(0.0),
                        y: px(-16.0),
                    })
                    .when_some(handle, |this, handle| this.with_handle(handle))
                    .into_any()
            }
            ContextPickerMode::File(picker) => {
                let handle = self
                    .message_editor
                    .update(cx, |this, _| this.context_picker_handle.clone())
                    .ok();

                PopoverMenu::new("context-picker")
                    .menu(move |_cx| Some(picker.clone()))
                    .trigger(
                        IconButton::new("add-context", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small),
                    )
                    .attach(gpui::AnchorCorner::TopLeft)
                    .anchor(gpui::AnchorCorner::BottomLeft)
                    .offset(gpui::Point {
                        x: px(0.0),
                        y: px(-16.0),
                    })
                    // .when_some(handle, |this, handle| this.with_handle(handle))
                    .into_any()
            }
        }
    }
}

#[derive(Clone)]
struct ContextPickerEntry {
    name: SharedString,
    description: SharedString,
    icon: IconName,
}

pub(crate) struct ContextPickerDelegate {
    mode: ContextPickerMode,
    workspace: WeakView<Workspace>,
    all_entries: Vec<ContextPickerEntry>,
    filtered_entries: Vec<ContextPickerEntry>,
    message_editor: WeakView<MessageEditor>,
    selected_ix: usize,
}

impl PickerDelegate for ContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_ix = ix.min(self.filtered_entries.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a context source…".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let all_commands = self.all_entries.clone();
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
                this.delegate.filtered_entries = filtered_commands;
                this.delegate.set_selected_index(0, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(entry) = self.filtered_entries.get(self.selected_ix) {
            match entry.name.to_string().as_str() {
                "file" => {
                    self.mode = ContextPickerMode::File(cx.new_view(|cx| {
                        FileContextPicker::new(
                            self.workspace.clone(),
                            self.message_editor.clone(),
                            cx,
                        )
                    }));
                }
                _ => {}
            }
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
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.filtered_entries.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Dense)
                .selected(selected)
                .tooltip({
                    let description = entry.description.clone();
                    move |cx| cx.new_view(|_cx| Tooltip::new(description.clone())).into()
                })
                .child(
                    v_flex()
                        .group(format!("context-entry-label-{ix}"))
                        .w_full()
                        .py_0p5()
                        .min_w(px(250.))
                        .max_w(px(400.))
                        .child(
                            h_flex()
                                .gap_1p5()
                                .child(Icon::new(entry.icon).size(IconSize::XSmall))
                                .child(
                                    Label::new(entry.name.clone())
                                        .single_line()
                                        .size(LabelSize::Small),
                                ),
                        )
                        .child(
                            div().overflow_hidden().text_ellipsis().child(
                                Label::new(entry.description.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                        ),
                ),
        )
    }
}
