#![allow(unused, dead_code)]

//! # UI – Text Field
//!
//! This crate provides a text field component that can be used to create text fields like search inputs, form fields, etc.
//!
//! It can't be located in the `ui` crate because it depends on `editor`.
//!

use std::default;

use editor::*;
use gpui::*;
use settings::Settings;
use theme::ThemeSettings;
use ui::{List, *};
use workspace::{ModalView, Workspace};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldLabelLayout {
    Hidden,
    Inline,
    Stacked,
}

pub struct TextFieldStyle {
    text_color: Hsla,
    background_color: Hsla,
    border_color: Hsla,
}

/// A Text Field view that can be used to create text fields like search inputs, form fields, etc.
///
/// It wraps a single line [`Editor`] view and allows for common field properties like labels, placeholders, icons, etc.
pub struct TextField {
    /// An optional label for the text field.
    ///
    /// Its position is determined by the [`FieldLabelLayout`].
    label: SharedString,
    /// The placeholder text for the text field.
    placeholder: SharedString,
    /// Exposes the underlying [`View<Editor>`] to allow for customizing the editor beyond the provided API.
    ///
    /// This likely will only be public in the short term, ideally the API will be expanded to cover necessary use cases.
    pub editor: View<Editor>,
    /// An optional icon that is displayed at the start of the text field.
    ///
    /// For example, a magnifying glass icon in a search field.
    start_icon: Option<IconName>,
    /// The layout of the label relative to the text field.
    with_label: FieldLabelLayout,
    /// Whether the text field is disabled.
    disabled: bool,
}

impl FocusableView for TextField {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl TextField {
    pub fn new(
        cx: &mut WindowContext,
        label: impl Into<SharedString>,
        placeholder: impl Into<SharedString>,
    ) -> Self {
        let placeholder_text = placeholder.into();

        let editor = cx.new_view(|cx| {
            let mut input = Editor::single_line(cx);
            input.set_placeholder_text(placeholder_text.clone(), cx);
            input
        });

        Self {
            label: label.into(),
            placeholder: placeholder_text,
            editor,
            start_icon: None,
            with_label: FieldLabelLayout::Hidden,
            disabled: false,
        }
    }

    pub fn start_icon(mut self, icon: IconName) -> Self {
        self.start_icon = Some(icon);
        self
    }

    pub fn with_label(mut self, layout: FieldLabelLayout) -> Self {
        self.with_label = layout;
        self
    }

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut ViewContext<Self>) {
        self.disabled = disabled;
        self.editor
            .update(cx, |editor, _| editor.set_read_only(disabled))
    }

    pub fn editor(&self) -> &View<Editor> {
        &self.editor
    }
}

impl Render for TextField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let theme_color = cx.theme().colors();

        let mut style = TextFieldStyle {
            text_color: theme_color.text,
            background_color: theme_color.ghost_element_background,
            border_color: theme_color.border,
        };

        if self.disabled {
            style.text_color = theme_color.text_disabled;
            style.background_color = theme_color.ghost_element_disabled;
            style.border_color = theme_color.border_disabled;
        }

        // if self.error_message.is_some() {
        //     style.text_color = cx.theme().status().error;
        //     style.border_color = cx.theme().status().error_border
        // }

        let text_style = TextStyle {
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.2),
            color: style.text_color,
            ..Default::default()
        };

        let editor_style = EditorStyle {
            background: theme_color.ghost_element_background,
            local_player: cx.theme().players().local(),
            text: text_style,
            ..Default::default()
        };

        div()
            .id(self.placeholder.clone())
            .group("text-field")
            .w_full()
            .when(self.with_label == FieldLabelLayout::Stacked, |this| {
                this.child(
                    Label::new(self.label.clone())
                        .size(LabelSize::Default)
                        .color(if self.disabled {
                            Color::Disabled
                        } else {
                            Color::Muted
                        }),
                )
            })
            .child(
                v_flex().w_full().child(
                    h_flex()
                        .w_full()
                        .flex_grow()
                        .gap_2()
                        .when(self.with_label == FieldLabelLayout::Inline, |this| {
                            this.child(Label::new(self.label.clone()).size(LabelSize::Default))
                        })
                        .child(
                            h_flex()
                                .px_2()
                                .py_1()
                                .bg(style.background_color)
                                .text_color(style.text_color)
                                .rounded_lg()
                                .border_1()
                                .border_color(style.border_color)
                                .min_w_48()
                                .w_full()
                                .flex_grow()
                                .gap_1()
                                .when_some(self.start_icon, |this, icon| {
                                    this.child(
                                        Icon::new(icon).size(IconSize::Small).color(Color::Muted),
                                    )
                                })
                                .child(EditorElement::new(&self.editor, editor_style)),
                        ),
                ),
            )
    }
}

// -------------------------------------------------------------------------------------------------

actions!(quick_commit, [ToggleStageAll]);

pub const MODAL_WIDTH: f32 = 700.0;
pub const MODAL_HEIGHT: f32 = 300.0;

fn test_files() -> Vec<ChangedFile> {
    vec![
        ChangedFile {
            id: 0,
            state: FileVCSState::Modified,
            file_name: "file1.txt".into(),
            file_path: "/path/to/file1.txt".into(),
        },
        ChangedFile {
            id: 1,
            state: FileVCSState::Deleted,
            file_name: "file2.txt".into(),
            file_path: "/path/to/file2.txt".into(),
        },
        ChangedFile {
            id: 2,
            state: FileVCSState::Created,
            file_name: "file3.txt".into(),
            file_path: "/path/to/file3.txt".into(),
        },
    ]
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum FileVCSState {
    Deleted,
    Modified,
    Created,
}

struct ChangedFileId(usize);

impl ChangedFileId {
    fn new(id: usize) -> Self {
        Self(id)
    }
}

// placeholder for ui
#[derive(Debug, Clone)]
struct ChangedFile {
    id: usize,
    state: FileVCSState,
    file_name: SharedString,
    file_path: SharedString,
}

struct QuickCommitState {
    placeholder_text: SharedString,
    tracked_files: Vec<ChangedFile>,
    staged_files: Vec<usize>,
    active_participant_handles: Vec<SharedString>,
    editor: View<Editor>,
    workspace: WeakView<Workspace>,
}

impl QuickCommitState {
    fn init(
        editor: View<Editor>,
        workspace: WeakView<Workspace>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let workspace = workspace.clone();

        Self {
            placeholder_text: "Add a message".into(),
            tracked_files: Default::default(),
            staged_files: Default::default(),
            active_participant_handles: Default::default(),
            editor,
            workspace,
        }
    }

    fn stage_state(&self) -> Selection {
        let staged_files = self.staged_files.clone();
        let tracked_files = self.tracked_files.clone();

        if staged_files.len() == tracked_files.len() {
            Selection::Selected
        } else if staged_files.is_empty() {
            Selection::Unselected
        } else {
            Selection::Indeterminate
        }
    }

    fn stage_all(&mut self) -> &mut Self {
        let tracked_files = self.tracked_files.clone();

        self.staged_files = tracked_files.iter().map(|file| file.id).collect();
        self
    }

    fn toggle_stage_all(&mut self) {
        let stage_state = self.stage_state();

        let staged_files = self.staged_files.clone();
        let tracked_files = self.tracked_files.clone();

        match stage_state {
            Selection::Selected => {
                self.staged_files.clear();
            }
            Selection::Unselected | Selection::Indeterminate => {
                self.stage_all();
            }
        }
    }

    fn toggle_file_staged(&mut self, file_id: usize) {
        if let Some(pos) = self.staged_files.iter().position(|&id| id == file_id) {
            self.staged_files.swap_remove(pos);
        } else {
            self.staged_files.push(file_id);
        }
    }
}

pub struct QuickCommit {
    state: Model<QuickCommitState>,
}

impl QuickCommit {
    pub fn init(workspace: WeakView<Workspace>, cx: &mut WindowContext) -> View<Self> {
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.set_show_gutter(false, cx);
            editor
        });

        cx.new_view(|cx| {
            let state = cx
                .new_model(move |cx| QuickCommitState::init(editor.clone(), workspace.clone(), cx));

            Self { state }
        })
    }

    fn stage_state(&self, cx: &ViewContext<Self>) -> Selection {
        self.state.read(cx).stage_state()
    }
    fn toggle_stage_all(&mut self, _: &ToggleStageAll, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| state.toggle_stage_all());
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }

    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.state.read(cx).editor.focus_handle(cx)
    }
}

impl QuickCommit {
    fn render_file_list(&mut self, cx: &mut ViewContext<Self>) -> List {
        List::new().empty_message("No changes")
    }
}

impl Render for QuickCommit {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let staged_files = self.state.read(cx).staged_files.clone();
        let total_tracked_files = self.state.read(cx).tracked_files.clone();
        let staged_state = self.stage_state(cx);

        h_flex()
            .id("quick_commit_modal")
            .key_context("quick_commit")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::cancel))
            .occlude()
            .h(px(MODAL_HEIGHT))
            .w(px(MODAL_WIDTH))
            .child(
                // commit editor
                div()
                    .h_full()
                    .flex_1()
                    // .child(self.editor.clone())
                    .child(
                        div()
                            .absolute()
                            .bottom_2()
                            .right_2()
                            .child(Button::new("submit_commit", "Commit")),
                    ),
            )
            .child(
                // file list
                div()
                    .w(relative(0.42))
                    .h_full()
                    .border_l_1()
                    .border_color(cx.theme().colors().border)
                    // sticky header
                    .child(
                        h_flex()
                            .h_10()
                            .w_full()
                            .child(Label::new(format!(
                                "Staged Files: {}/{}",
                                staged_files.len(),
                                total_tracked_files.len()
                            )))
                            .child(Checkbox::new("toggle-stage-all", staged_state).on_click(
                                |_, cx| {
                                    cx.dispatch_action(ToggleStageAll.boxed_clone());
                                },
                            )),
                    )
                    // file list
                    .child(self.render_file_list(cx)),
            )
    }
}

impl EventEmitter<DismissEvent> for QuickCommit {}

impl FocusableView for QuickCommit {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        // TODO: Not sure this is right
        self.focus_handle(cx)
    }
}

impl ModalView for QuickCommit {
    fn fade_out_background(&self) -> bool {
        true
    }
}
