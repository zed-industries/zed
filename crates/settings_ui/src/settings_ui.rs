//! # settings_ui
mod components;
mod page_data;

use anyhow::Result;
use editor::{Editor, EditorEvent};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use fuzzy::StringMatchCandidate;
use gpui::{
    Action, App, Div, Entity, FocusHandle, Focusable, FontWeight, Global, ReadGlobal as _,
    ScrollHandle, Task, TitlebarOptions, UniformListScrollHandle, Window, WindowHandle,
    WindowOptions, actions, div, point, prelude::*, px, size, uniform_list,
};
use heck::ToTitleCase as _;
use project::WorktreeId;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{
    BottomDockLayout, CloseWindowWhenNoItems, CodeFade, CursorShape, OnLastWindowClosed,
    RestoreOnStartupBehavior, SaturatingBool, SettingsContent, SettingsStore,
};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashMap,
    num::{NonZero, NonZeroU32},
    ops::Range,
    rc::Rc,
    sync::{Arc, LazyLock, RwLock, atomic::AtomicBool},
};
use ui::{
    ContextMenu, Divider, DropdownMenu, DropdownStyle, IconButtonShape, KeyBinding, KeybindingHint,
    PopoverMenu, Switch, SwitchColor, TreeViewItem, WithScrollbar, prelude::*,
};
use ui_input::{NumberField, NumberFieldType};
use util::{ResultExt as _, paths::PathStyle, rel_path::RelPath};
use workspace::{OpenOptions, OpenVisible, Workspace};
use zed_actions::OpenSettingsEditor;

use crate::components::SettingsEditor;

const NAVBAR_CONTAINER_TAB_INDEX: isize = 0;
const NAVBAR_GROUP_TAB_INDEX: isize = 1;
const CONTENT_CONTAINER_TAB_INDEX: isize = 2;
const CONTENT_GROUP_TAB_INDEX: isize = 3;

actions!(
    settings_editor,
    [
        /// Minimizes the settings UI window.
        Minimize,
        /// Toggles focus between the navbar and the main content.
        ToggleFocusNav,
        /// Focuses the next file in the file list.
        FocusNextFile,
        /// Focuses the previous file in the file list.
        FocusPreviousFile
    ]
);

#[derive(Action, PartialEq, Eq, Clone, Copy, Debug, JsonSchema, Deserialize)]
#[action(namespace = settings_editor)]
struct FocusFile(pub u32);

#[derive(Clone, Copy)]
struct SettingField<T: 'static> {
    pick: fn(&SettingsContent) -> &Option<T>,
    pick_mut: fn(&mut SettingsContent) -> &mut Option<T>,
}

/// Helper for unimplemented settings, used in combination with `SettingField::unimplemented`
/// to keep the setting around in the UI with valid pick and pick_mut implementations, but don't actually try to render it.
/// TODO(settings_ui): In non-dev builds (`#[cfg(not(debug_assertions))]`) make this render as edit-in-json
struct UnimplementedSettingField;

impl<T: 'static> SettingField<T> {
    /// Helper for settings with types that are not yet implemented.
    #[allow(unused)]
    fn unimplemented(self) -> SettingField<UnimplementedSettingField> {
        SettingField {
            pick: |_| &None,
            pick_mut: |_| unreachable!(),
        }
    }
}

trait AnySettingField {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    // Returns the file this value was set in and true, or File::Default and false to indicate it was not found in any file (missing default)
    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> (settings::SettingsFile, bool);
}

impl<T> AnySettingField for SettingField<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> (settings::SettingsFile, bool) {
        if AnySettingField::type_id(self) == TypeId::of::<UnimplementedSettingField>() {
            return (file.to_settings(), true);
        }

        let (file, value) = cx
            .global::<SettingsStore>()
            .get_value_from_file(file.to_settings(), self.pick);
        return (file, value.is_some());
    }
}

#[derive(Default, Clone)]
struct SettingFieldRenderer {
    renderers: Rc<
        RefCell<
            HashMap<
                TypeId,
                Box<
                    dyn Fn(
                        &dyn AnySettingField,
                        SettingsUiFile,
                        Option<&SettingsFieldMetadata>,
                        &mut Window,
                        &mut App,
                    ) -> AnyElement,
                >,
            >,
        >,
    >,
}

impl Global for SettingFieldRenderer {}

impl SettingFieldRenderer {
    fn add_renderer<T: 'static>(
        &mut self,
        renderer: impl Fn(
            &SettingField<T>,
            SettingsUiFile,
            Option<&SettingsFieldMetadata>,
            &mut Window,
            &mut App,
        ) -> AnyElement
        + 'static,
    ) -> &mut Self {
        let key = TypeId::of::<T>();
        let renderer = Box::new(
            move |any_setting_field: &dyn AnySettingField,
                  settings_file: SettingsUiFile,
                  metadata: Option<&SettingsFieldMetadata>,
                  window: &mut Window,
                  cx: &mut App| {
                let field = any_setting_field
                    .as_any()
                    .downcast_ref::<SettingField<T>>()
                    .unwrap();
                renderer(field, settings_file, metadata, window, cx)
            },
        );
        self.renderers.borrow_mut().insert(key, renderer);
        self
    }

    fn render(
        &self,
        any_setting_field: &dyn AnySettingField,
        settings_file: SettingsUiFile,
        metadata: Option<&SettingsFieldMetadata>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let key = any_setting_field.type_id();
        if let Some(renderer) = self.renderers.borrow().get(&key) {
            renderer(any_setting_field, settings_file, metadata, window, cx)
        } else {
            panic!(
                "No renderer found for type: {}",
                any_setting_field.type_name()
            )
        }
    }
}

struct SettingsFieldMetadata {
    placeholder: Option<&'static str>,
}

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

pub fn init(cx: &mut App) {
    init_renderers(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action_renderer(|div, _, _, cx| {
            let settings_ui_actions = [
                TypeId::of::<OpenSettingsEditor>(),
                TypeId::of::<ToggleFocusNav>(),
                TypeId::of::<FocusFile>(),
                TypeId::of::<FocusNextFile>(),
                TypeId::of::<FocusPreviousFile>(),
            ];
            let has_flag = cx.has_flag::<SettingsUiFeatureFlag>();
            command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _| {
                if has_flag {
                    filter.show_action_types(&settings_ui_actions);
                } else {
                    filter.hide_action_types(&settings_ui_actions);
                }
            });
            if has_flag {
                div.on_action(
                    cx.listener(|workspace, _: &OpenSettingsEditor, window, cx| {
                        let window_handle = window
                            .window_handle()
                            .downcast::<Workspace>()
                            .expect("Workspaces are root Windows");
                        open_settings_editor(workspace, window_handle, cx);
                    }),
                )
            } else {
                div
            }
        });
    })
    .detach();
}

fn init_renderers(cx: &mut App) {
    // fn (field: SettingsField, current_file: SettingsFile, cx) -> (currently_set_in: SettingsFile, overridden_in: Vec<SettingsFile>)
    cx.default_global::<SettingFieldRenderer>()
        .add_renderer::<UnimplementedSettingField>(|_, _, _, _, _| {
            // TODO(settings_ui): In non-dev builds (`#[cfg(not(debug_assertions))]`) make this render as edit-in-json
            Button::new("unimplemented-field", "UNIMPLEMENTED")
                .size(ButtonSize::Medium)
                .icon(IconName::XCircle)
                .icon_position(IconPosition::Start)
                .icon_color(Color::Error)
                .icon_size(IconSize::Small)
                .style(ButtonStyle::Outlined)
                .into_any_element()
        })
        .add_renderer::<bool>(|settings_field, file, _, _, cx| {
            render_toggle_button(*settings_field, file, cx).into_any_element()
        })
        .add_renderer::<String>(|settings_field, file, metadata, _, cx| {
            render_text_field(settings_field.clone(), file, metadata, cx)
        })
        .add_renderer::<SaturatingBool>(|settings_field, file, _, _, cx| {
            render_toggle_button(*settings_field, file, cx)
        })
        .add_renderer::<CursorShape>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<RestoreOnStartupBehavior>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<BottomDockLayout>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<OnLastWindowClosed>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<CloseWindowWhenNoItems>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::FontFamilyName>(|settings_field, file, _, window, cx| {
            // todo(settings_ui): We need to pass in a validator for this to ensure that users that type in invalid font names
            render_font_picker(settings_field.clone(), file, window, cx)
        })
        // todo(settings_ui): This needs custom ui
        // .add_renderer::<settings::BufferLineHeight>(|settings_field, file, _, window, cx| {
        //     // todo(settings_ui): Do we want to expose the custom variant of buffer line height?
        //     // right now there's a manual impl of strum::VariantArray
        //     render_dropdown(*settings_field, file, window, cx)
        // })
        .add_renderer::<settings::BaseKeymapContent>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::MultiCursorModifier>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::HideMouseMode>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::CurrentLineHighlight>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowWhitespaceSetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::SoftWrap>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ScrollBeyondLastLine>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::SnippetSortOrder>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ClosePosition>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DockSide>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::TerminalDockPosition>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DockPosition>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::GitGutterSetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::GitHunkStyleSetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DiagnosticSeverityContent>(
            |settings_field, file, _, window, cx| {
                render_dropdown(*settings_field, file, window, cx)
            },
        )
        .add_renderer::<settings::SeedQuerySetting>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DoubleClickInMultibuffer>(
            |settings_field, file, _, window, cx| {
                render_dropdown(*settings_field, file, window, cx)
            },
        )
        .add_renderer::<settings::GoToDefinitionFallback>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ActivateOnClose>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowDiagnostics>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowCloseButton>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ProjectPanelEntrySpacing>(
            |settings_field, file, _, window, cx| {
                render_dropdown(*settings_field, file, window, cx)
            },
        )
        .add_renderer::<settings::RewrapBehavior>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::FormatOnSave>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::IndentGuideColoring>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::IndentGuideBackgroundColoring>(
            |settings_field, file, _, window, cx| {
                render_dropdown(*settings_field, file, window, cx)
            },
        )
        .add_renderer::<settings::FileFinderWidthContent>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowDiagnostics>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::WordsCompletionMode>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::LspInsertMode>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<f32>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<u32>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<u64>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<usize>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<NonZero<usize>>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<NonZeroU32>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<CodeFade>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<FontWeight>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::MinimumContrast>(|settings_field, file, _, window, cx| {
            render_number_field(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowScrollbar>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ScrollbarDiagnostics>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::ShowMinimap>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::DisplayIn>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::MinimapThumb>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::MinimapThumbBorder>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        })
        .add_renderer::<settings::SteppingGranularity>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        });

    // todo(settings_ui): Figure out how we want to handle discriminant unions
    // .add_renderer::<ThemeSelection>(|settings_field, file, _, window, cx| {
    //     render_dropdown(*settings_field, file, window, cx)
    // });
}

pub fn open_settings_editor(
    _workspace: &mut Workspace,
    workspace_handle: WindowHandle<Workspace>,
    cx: &mut App,
) {
    let existing_window = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<SettingsWindow>());

    if let Some(existing_window) = existing_window {
        existing_window
            .update(cx, |settings_window, window, _| {
                settings_window.original_window = Some(workspace_handle);
                window.activate_window();
            })
            .ok();
        return;
    }

    // We have to defer this to get the workspace off the stack.

    cx.defer(move |cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Settings Window".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(12.0))),
                }),
                focus: true,
                show: true,
                kind: gpui::WindowKind::Normal,
                window_background: cx.theme().window_background_appearance(),
                window_min_size: Some(size(px(800.), px(600.))), // 4:3 Aspect Ratio
                ..Default::default()
            },
            |window, cx| cx.new(|cx| SettingsWindow::new(Some(workspace_handle), window, cx)),
        )
        .log_err();
    });
}

/// The current sub page path that is selected.
/// If this is empty the selected page is rendered,
/// otherwise the last sub page gets rendered.
///
/// Global so that `pick` and `pick_mut` callbacks can access it
/// and use it to dynamically render sub pages (e.g. for language settings)
static SUB_PAGE_STACK: LazyLock<RwLock<Vec<SubPage>>> = LazyLock::new(|| RwLock::new(Vec::new()));

fn sub_page_stack() -> std::sync::RwLockReadGuard<'static, Vec<SubPage>> {
    SUB_PAGE_STACK
        .read()
        .expect("SUB_PAGE_STACK is never poisoned")
}

fn sub_page_stack_mut() -> std::sync::RwLockWriteGuard<'static, Vec<SubPage>> {
    SUB_PAGE_STACK
        .write()
        .expect("SUB_PAGE_STACK is never poisoned")
}

pub struct SettingsWindow {
    original_window: Option<WindowHandle<Workspace>>,
    files: Vec<(SettingsUiFile, FocusHandle)>,
    worktree_root_dirs: HashMap<WorktreeId, String>,
    current_file: SettingsUiFile,
    pages: Vec<SettingsPage>,
    search_bar: Entity<Editor>,
    search_task: Option<Task<()>>,
    navbar_entry: usize, // Index into pages - should probably be (usize, Option<usize>) for section + page
    navbar_entries: Vec<NavBarEntry>,
    list_handle: UniformListScrollHandle,
    search_matches: Vec<Vec<bool>>,
    scroll_handle: ScrollHandle,
    focus_handle: FocusHandle,
    navbar_focus_handle: FocusHandle,
    content_focus_handle: FocusHandle,
    files_focus_handle: FocusHandle,
}

struct SubPage {
    link: SubPageLink,
    section_header: &'static str,
}

#[derive(PartialEq, Debug)]
struct NavBarEntry {
    title: &'static str,
    is_root: bool,
    expanded: bool,
    page_index: usize,
    item_index: Option<usize>,
}

struct SettingsPage {
    title: &'static str,
    items: Vec<SettingsPageItem>,
}

#[derive(PartialEq)]
enum SettingsPageItem {
    SectionHeader(&'static str),
    SettingItem(SettingItem),
    SubPageLink(SubPageLink),
}

impl std::fmt::Debug for SettingsPageItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsPageItem::SectionHeader(header) => write!(f, "SectionHeader({})", header),
            SettingsPageItem::SettingItem(setting_item) => {
                write!(f, "SettingItem({})", setting_item.title)
            }
            SettingsPageItem::SubPageLink(sub_page_link) => {
                write!(f, "SubPageLink({})", sub_page_link.title)
            }
        }
    }
}

impl SettingsPageItem {
    fn render(
        &self,
        settings_window: &SettingsWindow,
        section_header: &'static str,
        is_last: bool,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> AnyElement {
        let file = settings_window.current_file.clone();
        match self {
            SettingsPageItem::SectionHeader(header) => v_flex()
                .w_full()
                .gap_1()
                .child(
                    Label::new(SharedString::new_static(header))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .buffer_font(cx),
                )
                .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => {
                let renderer = cx.default_global::<SettingFieldRenderer>().clone();
                let (found_in_file, found) = setting_item.field.file_set_in(file.clone(), cx);
                let file_set_in = SettingsUiFile::from_settings(found_in_file);

                h_flex()
                    .id(setting_item.title)
                    .w_full()
                    .gap_2()
                    .flex_wrap()
                    .justify_between()
                    .map(|this| {
                        if is_last {
                            this.pb_6()
                        } else {
                            this.pb_4()
                                .border_b_1()
                                .border_color(cx.theme().colors().border_variant)
                        }
                    })
                    .child(
                        v_flex()
                            .max_w_1_2()
                            .flex_shrink()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_1()
                                    .child(Label::new(SharedString::new_static(setting_item.title)))
                                    .when_some(
                                        file_set_in.filter(|file_set_in| file_set_in != &file),
                                        |this, file_set_in| {
                                            this.child(
                                                Label::new(format!(
                                                    "— set in {}",
                                                    settings_window
                                                        .display_name(&file_set_in)
                                                        .expect("File name should exist")
                                                ))
                                                .color(Color::Muted)
                                                .size(LabelSize::Small),
                                            )
                                        },
                                    ),
                            )
                            .child(
                                Label::new(SharedString::new_static(setting_item.description))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(if cfg!(debug_assertions) && !found {
                        Button::new("no-default-field", "NO DEFAULT")
                            .size(ButtonSize::Medium)
                            .icon(IconName::XCircle)
                            .icon_position(IconPosition::Start)
                            .icon_color(Color::Error)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Outlined)
                            .into_any_element()
                    } else {
                        renderer.render(
                            setting_item.field.as_ref(),
                            file,
                            setting_item.metadata.as_deref(),
                            window,
                            cx,
                        )
                    })
                    .into_any_element()
            }
            SettingsPageItem::SubPageLink(sub_page_link) => h_flex()
                .id(sub_page_link.title)
                .w_full()
                .gap_2()
                .flex_wrap()
                .justify_between()
                .when(!is_last, |this| {
                    this.pb_4()
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                })
                .child(
                    v_flex()
                        .max_w_1_2()
                        .flex_shrink()
                        .child(Label::new(SharedString::new_static(sub_page_link.title))),
                )
                .child(
                    Button::new(("sub-page".into(), sub_page_link.title), "Configure")
                        .size(ButtonSize::Medium)
                        .icon(IconName::ChevronRight)
                        .icon_position(IconPosition::End)
                        .icon_color(Color::Muted)
                        .icon_size(IconSize::Small)
                        .style(ButtonStyle::Outlined),
                )
                .on_click({
                    let sub_page_link = sub_page_link.clone();
                    cx.listener(move |this, _, _, cx| {
                        this.push_sub_page(sub_page_link.clone(), section_header, cx)
                    })
                })
                .into_any_element(),
        }
    }
}

struct SettingItem {
    title: &'static str,
    description: &'static str,
    field: Box<dyn AnySettingField>,
    metadata: Option<Box<SettingsFieldMetadata>>,
    files: FileMask,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct FileMask(u8);

impl std::fmt::Debug for FileMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileMask(")?;
        let mut items = vec![];

        if self.contains(USER) {
            items.push("USER");
        }
        if self.contains(LOCAL) {
            items.push("LOCAL");
        }
        if self.contains(SERVER) {
            items.push("SERVER");
        }

        write!(f, "{})", items.join(" | "))
    }
}

const USER: FileMask = FileMask(1 << 0);
const LOCAL: FileMask = FileMask(1 << 2);
const SERVER: FileMask = FileMask(1 << 3);

impl std::ops::BitAnd for FileMask {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl std::ops::BitOr for FileMask {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl FileMask {
    fn contains(&self, other: FileMask) -> bool {
        self.0 & other.0 != 0
    }
}

impl PartialEq for SettingItem {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && (match (&self.metadata, &other.metadata) {
                (None, None) => true,
                (Some(m1), Some(m2)) => m1.placeholder == m2.placeholder,
                _ => false,
            })
    }
}

#[derive(Clone)]
struct SubPageLink {
    title: &'static str,
    files: FileMask,
    render: Arc<
        dyn Fn(&mut SettingsWindow, &mut Window, &mut Context<SettingsWindow>) -> AnyElement
            + 'static
            + Send
            + Sync,
    >,
}

impl PartialEq for SubPageLink {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
enum SettingsUiFile {
    User,                                // Uses all settings.
    Project((WorktreeId, Arc<RelPath>)), // Has a special name, and special set of settings
    Server(&'static str),                // Uses a special name, and the user settings
}

impl SettingsUiFile {
    fn worktree_id(&self) -> Option<WorktreeId> {
        match self {
            SettingsUiFile::Project((worktree_id, _)) => Some(*worktree_id),
            _ => None,
        }
    }

    fn from_settings(file: settings::SettingsFile) -> Option<Self> {
        Some(match file {
            settings::SettingsFile::User => SettingsUiFile::User,
            settings::SettingsFile::Project(location) => SettingsUiFile::Project(location),
            settings::SettingsFile::Server => SettingsUiFile::Server("todo: server name"),
            settings::SettingsFile::Default => return None,
        })
    }

    fn to_settings(&self) -> settings::SettingsFile {
        match self {
            SettingsUiFile::User => settings::SettingsFile::User,
            SettingsUiFile::Project(location) => settings::SettingsFile::Project(location.clone()),
            SettingsUiFile::Server(_) => settings::SettingsFile::Server,
        }
    }

    fn mask(&self) -> FileMask {
        match self {
            SettingsUiFile::User => USER,
            SettingsUiFile::Project(_) => LOCAL,
            SettingsUiFile::Server(_) => SERVER,
        }
    }
}

impl SettingsWindow {
    pub fn new(
        original_window: Option<WindowHandle<Workspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let font_family_cache = theme::FontFamilyCache::global(cx);

        cx.spawn(async move |this, cx| {
            font_family_cache.prefetch(cx).await;
            this.update(cx, |_, cx| {
                cx.notify();
            })
        })
        .detach();

        let current_file = SettingsUiFile::User;
        let search_bar = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search settings…", window, cx);
            editor
        });

        cx.subscribe(&search_bar, |this, _, event: &EditorEvent, cx| {
            let EditorEvent::Edited { transaction_id: _ } = event else {
                return;
            };

            this.update_matches(cx);
        })
        .detach();

        cx.observe_global_in::<SettingsStore>(window, move |this, _, cx| {
            this.fetch_files(cx);
            cx.notify();
        })
        .detach();

        let mut this = Self {
            original_window,
            worktree_root_dirs: HashMap::default(),
            files: vec![],
            current_file: current_file,
            pages: vec![],
            navbar_entries: vec![],
            navbar_entry: 0,
            list_handle: UniformListScrollHandle::default(),
            search_bar,
            search_task: None,
            search_matches: vec![],
            scroll_handle: ScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            navbar_focus_handle: cx
                .focus_handle()
                .tab_index(NAVBAR_CONTAINER_TAB_INDEX)
                .tab_stop(false),
            content_focus_handle: cx
                .focus_handle()
                .tab_index(CONTENT_CONTAINER_TAB_INDEX)
                .tab_stop(false),
            files_focus_handle: cx.focus_handle().tab_stop(false),
        };

        this.fetch_files(cx);
        this.build_ui(cx);

        this.search_bar.update(cx, |editor, cx| {
            editor.focus_handle(cx).focus(window);
        });

        this
    }

    fn toggle_navbar_entry(&mut self, ix: usize) {
        // We can only toggle root entries
        if !self.navbar_entries[ix].is_root {
            return;
        }

        let toggle_page_index = self.page_index_from_navbar_index(ix);
        let selected_page_index = self.page_index_from_navbar_index(self.navbar_entry);

        let expanded = &mut self.navbar_entries[ix].expanded;
        *expanded = !*expanded;
        // if currently selected page is a child of the parent page we are folding,
        // set the current page to the parent page
        if !*expanded && selected_page_index == toggle_page_index {
            self.navbar_entry = ix;
        }
    }

    fn build_navbar(&mut self) {
        let mut prev_navbar_state = HashMap::new();
        let mut root_entry = "";
        let mut prev_selected_entry = None;
        for (index, entry) in self.navbar_entries.iter().enumerate() {
            let sub_entry_title;
            if entry.is_root {
                sub_entry_title = None;
                root_entry = entry.title;
            } else {
                sub_entry_title = Some(entry.title);
            }
            let key = (root_entry, sub_entry_title);
            if index == self.navbar_entry {
                prev_selected_entry = Some(key);
            }
            prev_navbar_state.insert(key, entry.expanded);
        }

        let mut navbar_entries = Vec::with_capacity(self.navbar_entries.len());
        for (page_index, page) in self.pages.iter().enumerate() {
            navbar_entries.push(NavBarEntry {
                title: page.title,
                is_root: true,
                expanded: false,
                page_index,
                item_index: None,
            });

            for (item_index, item) in page.items.iter().enumerate() {
                let SettingsPageItem::SectionHeader(title) = item else {
                    continue;
                };
                navbar_entries.push(NavBarEntry {
                    title,
                    is_root: false,
                    expanded: false,
                    page_index,
                    item_index: Some(item_index),
                });
            }
        }

        let mut root_entry = "";
        let mut found_nav_entry = false;
        for (index, entry) in navbar_entries.iter_mut().enumerate() {
            let sub_entry_title;
            if entry.is_root {
                root_entry = entry.title;
                sub_entry_title = None;
            } else {
                sub_entry_title = Some(entry.title);
            };
            let key = (root_entry, sub_entry_title);
            if Some(key) == prev_selected_entry {
                self.navbar_entry = index;
                found_nav_entry = true;
            }
            entry.expanded = *prev_navbar_state.get(&key).unwrap_or(&false);
        }
        if !found_nav_entry {
            self.navbar_entry = 0;
        }
        self.navbar_entries = navbar_entries;
    }

    fn visible_navbar_entries(&self) -> impl Iterator<Item = (usize, &NavBarEntry)> {
        let mut index = 0;
        let entries = &self.navbar_entries;
        let search_matches = &self.search_matches;
        std::iter::from_fn(move || {
            while index < entries.len() {
                let entry = &entries[index];
                let included_in_search = if let Some(item_index) = entry.item_index {
                    search_matches[entry.page_index][item_index]
                } else {
                    search_matches[entry.page_index].iter().any(|b| *b)
                        || search_matches[entry.page_index].is_empty()
                };
                if included_in_search {
                    break;
                }
                index += 1;
            }
            if index >= self.navbar_entries.len() {
                return None;
            }
            let entry = &entries[index];
            let entry_index = index;

            index += 1;
            if entry.is_root && !entry.expanded {
                while index < entries.len() {
                    if entries[index].is_root {
                        break;
                    }
                    index += 1;
                }
            }

            return Some((entry_index, entry));
        })
    }

    fn filter_matches_to_file(&mut self) {
        let current_file = self.current_file.mask();
        for (page, page_filter) in std::iter::zip(&self.pages, &mut self.search_matches) {
            let mut header_index = 0;
            let mut any_found_since_last_header = true;

            for (index, item) in page.items.iter().enumerate() {
                match item {
                    SettingsPageItem::SectionHeader(_) => {
                        if !any_found_since_last_header {
                            page_filter[header_index] = false;
                        }
                        header_index = index;
                        any_found_since_last_header = false;
                    }
                    SettingsPageItem::SettingItem(setting_item) => {
                        if !setting_item.files.contains(current_file) {
                            page_filter[index] = false;
                        } else {
                            any_found_since_last_header = true;
                        }
                    }
                    SettingsPageItem::SubPageLink(sub_page_link) => {
                        if !sub_page_link.files.contains(current_file) {
                            page_filter[index] = false;
                        } else {
                            any_found_since_last_header = true;
                        }
                    }
                }
            }
            if let Some(last_header) = page_filter.get_mut(header_index)
                && !any_found_since_last_header
            {
                *last_header = false;
            }
        }
    }

    fn update_matches(&mut self, cx: &mut Context<SettingsWindow>) {
        self.search_task.take();
        let query = self.search_bar.read(cx).text(cx);
        if query.is_empty() {
            for page in &mut self.search_matches {
                page.fill(true);
            }
            self.filter_matches_to_file();
            cx.notify();
            return;
        }

        struct ItemKey {
            page_index: usize,
            header_index: usize,
            item_index: usize,
        }
        let mut key_lut: Vec<ItemKey> = vec![];
        let mut candidates = Vec::default();

        for (page_index, page) in self.pages.iter().enumerate() {
            let mut header_index = 0;
            for (item_index, item) in page.items.iter().enumerate() {
                let key_index = key_lut.len();
                match item {
                    SettingsPageItem::SettingItem(item) => {
                        candidates.push(StringMatchCandidate::new(key_index, item.title));
                        candidates.push(StringMatchCandidate::new(key_index, item.description));
                    }
                    SettingsPageItem::SectionHeader(header) => {
                        candidates.push(StringMatchCandidate::new(key_index, header));
                        header_index = item_index;
                    }
                    SettingsPageItem::SubPageLink(sub_page_link) => {
                        candidates.push(StringMatchCandidate::new(key_index, sub_page_link.title));
                    }
                }
                key_lut.push(ItemKey {
                    page_index,
                    header_index,
                    item_index,
                });
            }
        }
        let atomic_bool = AtomicBool::new(false);

        self.search_task = Some(cx.spawn(async move |this, cx| {
            let string_matches = fuzzy::match_strings(
                candidates.as_slice(),
                &query,
                false,
                true,
                candidates.len(),
                &atomic_bool,
                cx.background_executor().clone(),
            );
            let string_matches = string_matches.await;

            this.update(cx, |this, cx| {
                for page in &mut this.search_matches {
                    page.fill(false);
                }

                for string_match in string_matches {
                    let ItemKey {
                        page_index,
                        header_index,
                        item_index,
                    } = key_lut[string_match.candidate_id];
                    let page = &mut this.search_matches[page_index];
                    page[header_index] = true;
                    page[item_index] = true;
                }
                this.filter_matches_to_file();
                let first_navbar_entry_index = this
                    .visible_navbar_entries()
                    .next()
                    .map(|e| e.0)
                    .unwrap_or(0);
                this.navbar_entry = first_navbar_entry_index;
                cx.notify();
            })
            .ok();
        }));
    }

    fn build_search_matches(&mut self) {
        self.search_matches = self
            .pages
            .iter()
            .map(|page| vec![true; page.items.len()])
            .collect::<Vec<_>>();
    }

    fn build_ui(&mut self, cx: &mut Context<SettingsWindow>) {
        if self.pages.is_empty() {
            self.pages = page_data::settings_data();
        }
        self.build_search_matches();
        self.build_navbar();

        self.update_matches(cx);

        cx.notify();
    }

    fn calculate_navbar_entry_from_scroll_position(&mut self) {
        let top = self.scroll_handle.top_item();
        let bottom = self.scroll_handle.bottom_item();

        let scroll_index = (top + bottom) / 2;
        let scroll_index = scroll_index.clamp(top, bottom);
        let mut page_index = self.navbar_entry;

        while !self.navbar_entries[page_index].is_root {
            page_index -= 1;
        }

        if self.navbar_entries[page_index].expanded {
            let section_index = self
                .page_items()
                .take(scroll_index + 1)
                .filter(|item| matches!(item, SettingsPageItem::SectionHeader(_)))
                .count();

            self.navbar_entry = section_index + page_index;
        }
    }

    fn fetch_files(&mut self, cx: &mut Context<SettingsWindow>) {
        self.worktree_root_dirs.clear();
        let prev_files = self.files.clone();
        let settings_store = cx.global::<SettingsStore>();
        let mut ui_files = vec![];
        let all_files = settings_store.get_all_files();
        for file in all_files {
            let Some(settings_ui_file) = SettingsUiFile::from_settings(file) else {
                continue;
            };

            if let Some(worktree_id) = settings_ui_file.worktree_id() {
                let directory_name = all_projects(cx)
                    .find_map(|project| project.read(cx).worktree_for_id(worktree_id, cx))
                    .and_then(|worktree| worktree.read(cx).root_dir())
                    .and_then(|root_dir| {
                        root_dir
                            .file_name()
                            .map(|os_string| os_string.to_string_lossy().to_string())
                    });

                let Some(directory_name) = directory_name else {
                    log::error!(
                        "No directory name found for settings file at worktree ID: {}",
                        worktree_id
                    );
                    continue;
                };

                self.worktree_root_dirs.insert(worktree_id, directory_name);
            }

            let focus_handle = prev_files
                .iter()
                .find_map(|(prev_file, handle)| {
                    (prev_file == &settings_ui_file).then(|| handle.clone())
                })
                .unwrap_or_else(|| cx.focus_handle());
            ui_files.push((settings_ui_file, focus_handle));
        }
        ui_files.reverse();
        self.files = ui_files;
        let current_file_still_exists = self
            .files
            .iter()
            .any(|(file, _)| file == &self.current_file);
        if !current_file_still_exists {
            self.change_file(0, cx);
        }
    }

    fn change_file(&mut self, ix: usize, cx: &mut Context<SettingsWindow>) {
        if ix >= self.files.len() {
            self.current_file = SettingsUiFile::User;
            return;
        }
        if self.files[ix].0 == self.current_file {
            return;
        }
        self.current_file = self.files[ix].0.clone();
        // self.navbar_entry = 0;
        self.build_ui(cx);
    }

    fn render_files_header(
        &self,
        _window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .gap_1()
            .justify_between()
            .child(
                h_flex()
                    .id("file_buttons_container")
                    .w_64() // Temporary fix until long-term solution is a fixed set of buttons representing a file location (User, Project, and Remote)
                    .gap_1()
                    .overflow_x_scroll()
                    .children(
                        self.files
                            .iter()
                            .enumerate()
                            .map(|(ix, (file, focus_handle))| {
                                Button::new(
                                    ix,
                                    self.display_name(&file)
                                        .expect("Files should always have a name"),
                                )
                                .toggle_state(file == &self.current_file)
                                .selected_style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                .track_focus(focus_handle)
                                .on_click(cx.listener(
                                    move |this, evt: &gpui::ClickEvent, window, cx| {
                                        this.change_file(ix, cx);
                                        if evt.is_keyboard() {
                                            this.focus_first_nav_item(window, cx);
                                        }
                                    },
                                ))
                            }),
                    ),
            )
            .child(
                Button::new(
                    "edit-in-json",
                    format!("Edit in {}", self.file_location_str()),
                )
                .style(ButtonStyle::Outlined)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.open_current_settings_file(cx);
                })),
            )
    }

    pub(crate) fn display_name(&self, file: &SettingsUiFile) -> Option<String> {
        match file {
            SettingsUiFile::User => Some("User".to_string()),
            SettingsUiFile::Project((worktree_id, path)) => self
                .worktree_root_dirs
                .get(&worktree_id)
                .map(|directory_name| {
                    let path_style = PathStyle::local();
                    if path.is_empty() {
                        directory_name.clone()
                    } else {
                        format!(
                            "{}{}{}",
                            directory_name,
                            path_style.separator(),
                            path.display(path_style)
                        )
                    }
                }),
            SettingsUiFile::Server(file) => Some(file.to_string()),
        }
    }

    fn file_location_str(&self) -> String {
        match &self.current_file {
            SettingsUiFile::User => "settings.json".to_string(),
            SettingsUiFile::Project((worktree_id, path)) => self
                .worktree_root_dirs
                .get(&worktree_id)
                .map(|directory_name| {
                    let path_style = PathStyle::local();
                    let file_path = path.join(paths::local_settings_file_relative_path());
                    format!(
                        "{}{}{}",
                        directory_name,
                        path_style.separator(),
                        file_path.display(path_style)
                    )
                })
                .expect("Current file should always be present in root dir map"),
            SettingsUiFile::Server(file) => file.to_string(),
        }
    }

    fn render_search(&self, _window: &mut Window, cx: &mut App) -> Div {
        h_flex()
            .py_1()
            .px_1p5()
            .gap_1p5()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(self.search_bar.clone())
    }

    fn render_nav(
        &self,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        let visible_count = self.visible_navbar_entries().count();
        let nav_background = cx.theme().colors().panel_background;
        let focus_keybind_label = if self.navbar_focus_handle.contains_focused(window, cx) {
            "Focus Content"
        } else {
            "Focus Navbar"
        };

        v_flex()
            .w_64()
            .p_2p5()
            .pt_10()
            .gap_3()
            .flex_none()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .bg(nav_background)
            .child(self.render_search(window, cx))
            .child(
                v_flex()
                    .flex_grow()
                    .track_focus(&self.navbar_focus_handle)
                    .tab_group()
                    .tab_index(NAVBAR_GROUP_TAB_INDEX)
                    .child(
                        uniform_list(
                            "settings-ui-nav-bar",
                            visible_count,
                            cx.processor(move |this, range: Range<usize>, _, cx| {
                                let entries: Vec<_> = this.visible_navbar_entries().collect();
                                range
                                    .filter_map(|ix| entries.get(ix).copied())
                                    .map(|(ix, entry)| {
                                        TreeViewItem::new(
                                            ("settings-ui-navbar-entry", ix),
                                            entry.title,
                                        )
                                        .tab_index(0)
                                        .root_item(entry.is_root)
                                        .toggle_state(this.is_navbar_entry_selected(ix))
                                        .when(entry.is_root, |item| {
                                            item.expanded(entry.expanded).on_toggle(cx.listener(
                                                move |this, _, _, cx| {
                                                    this.toggle_navbar_entry(ix);
                                                    cx.notify();
                                                },
                                            ))
                                        })
                                        .on_click(cx.listener(
                                            move |this, evt: &gpui::ClickEvent, window, cx| {
                                                this.navbar_entry = ix;

                                                if !this.navbar_entries[ix].is_root {
                                                    let mut selected_page_ix = ix;

                                                    while !this.navbar_entries[selected_page_ix]
                                                        .is_root
                                                    {
                                                        selected_page_ix -= 1;
                                                    }

                                                    let section_header = ix - selected_page_ix;

                                                    if let Some(section_index) = this
                                                        .page_items()
                                                        .enumerate()
                                                        .filter(|item| {
                                                            matches!(
                                                                item.1,
                                                                SettingsPageItem::SectionHeader(_)
                                                            )
                                                        })
                                                        .take(section_header)
                                                        .last()
                                                        .map(|pair| pair.0)
                                                    {
                                                        this.scroll_handle
                                                            .scroll_to_top_of_item(section_index);
                                                    }
                                                }

                                                if evt.is_keyboard() {
                                                    // todo(settings_ui): Focus the actual item and scroll to it
                                                    this.focus_first_content_item(window, cx);
                                                }
                                                cx.notify();
                                            },
                                        ))
                                        .into_any_element()
                                    })
                                    .collect()
                            }),
                        )
                        .track_scroll(self.list_handle.clone())
                        .flex_grow(),
                    )
                    .vertical_scrollbar_for(self.list_handle.clone(), window, cx),
            )
            .child(
                h_flex()
                    .w_full()
                    .p_2()
                    .pb_0p5()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .children(
                        KeyBinding::for_action(&ToggleFocusNav, window, cx).map(|this| {
                            KeybindingHint::new(
                                this,
                                cx.theme().colors().surface_background.opacity(0.5),
                            )
                            .suffix(focus_keybind_label)
                        }),
                    ),
            )
    }

    fn focus_first_nav_item(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.navbar_focus_handle.focus(window);
        window.focus_next();
        cx.notify();
    }

    fn focus_first_content_item(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.content_focus_handle.focus(window);
        window.focus_next();
        cx.notify();
    }

    fn page_items(&self) -> impl Iterator<Item = &SettingsPageItem> {
        let page_idx = self.current_page_index();

        self.current_page()
            .items
            .iter()
            .enumerate()
            .filter_map(move |(item_index, item)| {
                self.search_matches[page_idx][item_index].then_some(item)
            })
    }

    fn render_sub_page_breadcrumbs(&self) -> impl IntoElement {
        let mut items = vec![];
        items.push(self.current_page().title);
        items.extend(
            sub_page_stack()
                .iter()
                .flat_map(|page| [page.section_header, page.link.title]),
        );

        let last = items.pop().unwrap();
        h_flex()
            .gap_1()
            .children(
                items
                    .into_iter()
                    .flat_map(|item| [item, "/"])
                    .map(|item| Label::new(item).color(Color::Muted)),
            )
            .child(Label::new(last))
    }

    fn render_page_items<'a, Items: Iterator<Item = &'a SettingsPageItem>>(
        &self,
        items: Items,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        let mut page_content = v_flex()
            .id("settings-ui-page")
            .size_full()
            .gap_4()
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle);

        let items: Vec<_> = items.collect();
        let items_len = items.len();
        let mut section_header = None;

        let has_active_search = !self.search_bar.read(cx).is_empty(cx);
        let has_no_results = items_len == 0 && has_active_search;

        if has_no_results {
            let search_query = self.search_bar.read(cx).text(cx);
            page_content = page_content.child(
                v_flex()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .child(div().child("No Results"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().colors().text_muted)
                            .child(format!("No settings match \"{}\"", search_query)),
                    ),
            )
        } else {
            let last_non_header_index = items
                .iter()
                .enumerate()
                .rev()
                .find(|(_, item)| !matches!(item, SettingsPageItem::SectionHeader(_)))
                .map(|(index, _)| index);

            page_content =
                page_content.children(items.clone().into_iter().enumerate().map(|(index, item)| {
                    let no_bottom_border = items
                        .get(index + 1)
                        .map(|next_item| matches!(next_item, SettingsPageItem::SectionHeader(_)))
                        .unwrap_or(false);
                    let is_last = Some(index) == last_non_header_index;

                    if let SettingsPageItem::SectionHeader(header) = item {
                        section_header = Some(*header);
                    }
                    item.render(
                        self,
                        section_header.expect("All items rendered after a section header"),
                        no_bottom_border || is_last,
                        window,
                        cx,
                    )
                }))
        }
        page_content
    }

    fn render_page(
        &mut self,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        let page_header;
        let page_content;

        if sub_page_stack().len() == 0 {
            page_header = self.render_files_header(window, cx).into_any_element();

            page_content = self
                .render_page_items(self.page_items(), window, cx)
                .into_any_element();
        } else {
            page_header = h_flex()
                .ml_neg_1p5()
                .gap_1()
                .child(
                    IconButton::new("back-btn", IconName::ArrowLeft)
                        .icon_size(IconSize::Small)
                        .shape(IconButtonShape::Square)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.pop_sub_page(cx);
                        })),
                )
                .child(self.render_sub_page_breadcrumbs())
                .into_any_element();

            let active_page_render_fn = sub_page_stack().last().unwrap().link.render.clone();
            page_content = (active_page_render_fn)(self, window, cx);
        }

        return v_flex()
            .w_full()
            .pt_4()
            .pb_6()
            .px_6()
            .gap_4()
            .track_focus(&self.content_focus_handle)
            .bg(cx.theme().colors().editor_background)
            .vertical_scrollbar_for(self.scroll_handle.clone(), window, cx)
            .child(page_header)
            .child(
                div()
                    .size_full()
                    .track_focus(&self.content_focus_handle)
                    .tab_group()
                    .tab_index(CONTENT_GROUP_TAB_INDEX)
                    .child(page_content),
            );
    }

    fn open_current_settings_file(&mut self, cx: &mut Context<Self>) {
        match &self.current_file {
            SettingsUiFile::User => {
                let Some(original_window) = self.original_window.clone() else {
                    return;
                };
                original_window
                    .update(cx, |workspace, window, cx| {
                        workspace
                            .with_local_workspace(window, cx, |workspace, window, cx| {
                                let create_task = workspace.project().update(cx, |project, cx| {
                                    project.find_or_create_worktree(
                                        paths::config_dir().as_path(),
                                        false,
                                        cx,
                                    )
                                });
                                let open_task = workspace.open_paths(
                                    vec![paths::settings_file().to_path_buf()],
                                    OpenOptions {
                                        visible: Some(OpenVisible::None),
                                        ..Default::default()
                                    },
                                    None,
                                    window,
                                    cx,
                                );

                                cx.spawn_in(window, async move |workspace, cx| {
                                    create_task.await.ok();
                                    open_task.await;

                                    workspace.update_in(cx, |_, window, cx| {
                                        window.activate_window();
                                        cx.notify();
                                    })
                                })
                                .detach();
                            })
                            .detach();
                    })
                    .ok();
            }
            SettingsUiFile::Project((worktree_id, path)) => {
                let mut corresponding_workspace: Option<WindowHandle<Workspace>> = None;
                let settings_path = path.join(paths::local_settings_file_relative_path());
                let Some(app_state) = workspace::AppState::global(cx).upgrade() else {
                    return;
                };
                for workspace in app_state.workspace_store.read(cx).workspaces() {
                    let contains_settings_file = workspace
                        .read_with(cx, |workspace, cx| {
                            workspace.project().read(cx).contains_local_settings_file(
                                *worktree_id,
                                settings_path.as_ref(),
                                cx,
                            )
                        })
                        .ok();
                    if Some(true) == contains_settings_file {
                        corresponding_workspace = Some(workspace.clone());

                        break;
                    }
                }

                let Some(corresponding_workspace) = corresponding_workspace else {
                    log::error!(
                        "No corresponding workspace found for settings file {}",
                        settings_path.as_std_path().display()
                    );

                    return;
                };

                // TODO: move zed::open_local_file() APIs to this crate, and
                // re-implement the "initial_contents" behavior
                corresponding_workspace
                    .update(cx, |workspace, window, cx| {
                        let open_task = workspace.open_path(
                            (*worktree_id, settings_path.clone()),
                            None,
                            true,
                            window,
                            cx,
                        );

                        cx.spawn_in(window, async move |workspace, cx| {
                            if open_task.await.log_err().is_some() {
                                workspace
                                    .update_in(cx, |_, window, cx| {
                                        window.activate_window();
                                        cx.notify();
                                    })
                                    .ok();
                            }
                        })
                        .detach();
                    })
                    .ok();
            }
            SettingsUiFile::Server(_) => {
                return;
            }
        };
    }

    fn current_page_index(&self) -> usize {
        self.page_index_from_navbar_index(self.navbar_entry)
    }

    fn current_page(&self) -> &SettingsPage {
        &self.pages[self.current_page_index()]
    }

    fn page_index_from_navbar_index(&self, index: usize) -> usize {
        if self.navbar_entries.is_empty() {
            return 0;
        }

        self.navbar_entries[index].page_index
    }

    fn is_navbar_entry_selected(&self, ix: usize) -> bool {
        ix == self.navbar_entry
    }

    fn push_sub_page(
        &mut self,
        sub_page_link: SubPageLink,
        section_header: &'static str,
        cx: &mut Context<SettingsWindow>,
    ) {
        sub_page_stack_mut().push(SubPage {
            link: sub_page_link,
            section_header,
        });
        cx.notify();
    }

    fn pop_sub_page(&mut self, cx: &mut Context<SettingsWindow>) {
        sub_page_stack_mut().pop();
        cx.notify();
    }

    fn focus_file_at_index(&mut self, index: usize, window: &mut Window) {
        if let Some((_, handle)) = self.files.get(index) {
            handle.focus(window);
        }
    }

    fn focused_file_index(&self, window: &Window, cx: &Context<Self>) -> usize {
        if self.files_focus_handle.contains_focused(window, cx)
            && let Some(index) = self
                .files
                .iter()
                .position(|(_, handle)| handle.is_focused(window))
        {
            return index;
        }
        if let Some(current_file_index) = self
            .files
            .iter()
            .position(|(file, _)| file == &self.current_file)
        {
            return current_file_index;
        }
        0
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);
        self.calculate_navbar_entry_from_scroll_position();

        div()
            .id("settings-window")
            .key_context("SettingsWindow")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Minimize, window, _cx| {
                window.minimize_window();
            })
            .on_action(cx.listener(|this, _: &search::FocusSearch, window, cx| {
                this.search_bar.focus_handle(cx).focus(window);
            }))
            .on_action(cx.listener(|this, _: &ToggleFocusNav, window, cx| {
                if this.navbar_focus_handle.contains_focused(window, cx) {
                    this.focus_first_content_item(window, cx);
                } else {
                    this.focus_first_nav_item(window, cx);
                }
            }))
            .on_action(
                cx.listener(|this, FocusFile(file_index): &FocusFile, window, _| {
                    this.focus_file_at_index(*file_index as usize, window);
                }),
            )
            .on_action(cx.listener(|this, _: &FocusNextFile, window, cx| {
                let next_index = usize::min(
                    this.focused_file_index(window, cx) + 1,
                    this.files.len().saturating_sub(1),
                );
                this.focus_file_at_index(next_index, window);
            }))
            .on_action(cx.listener(|this, _: &FocusPreviousFile, window, cx| {
                let prev_index = this.focused_file_index(window, cx).saturating_sub(1);
                this.focus_file_at_index(prev_index, window);
            }))
            .on_action(|_: &menu::SelectNext, window, _| {
                window.focus_next();
            })
            .on_action(|_: &menu::SelectPrevious, window, _| {
                window.focus_prev();
            })
            .flex()
            .flex_row()
            .size_full()
            .font(ui_font)
            .bg(cx.theme().colors().background)
            .text_color(cx.theme().colors().text)
            .child(self.render_nav(window, cx))
            .child(self.render_page(window, cx))
    }
}

fn all_projects(cx: &App) -> impl Iterator<Item = Entity<project::Project>> {
    workspace::AppState::global(cx)
        .upgrade()
        .map(|app_state| {
            app_state
                .workspace_store
                .read(cx)
                .workspaces()
                .iter()
                .filter_map(|workspace| Some(workspace.read(cx).ok()?.project().clone()))
        })
        .into_iter()
        .flatten()
}

fn update_settings_file(
    file: SettingsUiFile,
    cx: &mut App,
    update: impl 'static + Send + FnOnce(&mut SettingsContent, &App),
) -> Result<()> {
    match file {
        SettingsUiFile::Project((worktree_id, rel_path)) => {
            let rel_path = rel_path.join(paths::local_settings_file_relative_path());
            let project = all_projects(cx).find(|project| {
                project.read_with(cx, |project, cx| {
                    project.contains_local_settings_file(worktree_id, &rel_path, cx)
                })
            });
            let Some(project) = project else {
                anyhow::bail!(
                    "Could not find worktree containing settings file: {}",
                    &rel_path.display(PathStyle::local())
                );
            };
            project.update(cx, |project, cx| {
                project.update_local_settings_file(worktree_id, rel_path, cx, update);
            });
            return Ok(());
        }
        SettingsUiFile::User => {
            // todo(settings_ui) error?
            SettingsStore::global(cx).update_settings_file(<dyn fs::Fs>::global(cx), update);
            Ok(())
        }
        SettingsUiFile::Server(_) => unimplemented!(),
    }
}

fn render_text_field<T: From<String> + Into<String> + AsRef<str> + Clone>(
    field: SettingField<T>,
    file: SettingsUiFile,
    metadata: Option<&SettingsFieldMetadata>,
    cx: &mut App,
) -> AnyElement {
    let (_, initial_text) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let initial_text = initial_text.filter(|s| !s.as_ref().is_empty());

    SettingsEditor::new()
        .tab_index(0)
        .when_some(initial_text, |editor, text| {
            editor.with_initial_text(text.as_ref().to_string())
        })
        .when_some(
            metadata.and_then(|metadata| metadata.placeholder),
            |editor, placeholder| editor.with_placeholder(placeholder),
        )
        .on_confirm({
            move |new_text, cx| {
                update_settings_file(file.clone(), cx, move |settings, _cx| {
                    *(field.pick_mut)(settings) = new_text.map(Into::into);
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .into_any_element()
}

fn render_toggle_button<B: Into<bool> + From<bool> + Copy>(
    field: SettingField<B>,
    file: SettingsUiFile,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);

    let toggle_state = if value.copied().map_or(false, Into::into) {
        ToggleState::Selected
    } else {
        ToggleState::Unselected
    };

    Switch::new("toggle_button", toggle_state)
        .color(ui::SwitchColor::Accent)
        .on_click({
            move |state, _window, cx| {
                let state = *state == ui::ToggleState::Selected;
                update_settings_file(file.clone(), cx, move |settings, _cx| {
                    *(field.pick_mut)(settings) = Some(state.into());
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .tab_index(0_isize)
        .color(SwitchColor::Accent)
        .into_any_element()
}

fn render_font_picker(
    field: SettingField<settings::FontFamilyName>,
    file: SettingsUiFile,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let current_value = SettingsStore::global(cx)
        .get_value_from_file(file.to_settings(), field.pick)
        .1
        .cloned()
        .unwrap_or_else(|| SharedString::default().into());

    let font_picker = cx.new(|cx| {
        ui_input::font_picker(
            current_value.clone().into(),
            move |font_name, cx| {
                update_settings_file(file.clone(), cx, move |settings, _cx| {
                    *(field.pick_mut)(settings) = Some(font_name.into());
                })
                .log_err(); // todo(settings_ui) don't log err
            },
            window,
            cx,
        )
    });

    PopoverMenu::new("font-picker")
        .menu(move |_window, _cx| Some(font_picker.clone()))
        .trigger(
            Button::new("font-family-button", current_value)
                .tab_index(0_isize)
                .style(ButtonStyle::Outlined)
                .size(ButtonSize::Medium)
                .icon(IconName::ChevronUpDown)
                .icon_color(Color::Muted)
                .icon_size(IconSize::Small)
                .icon_position(IconPosition::End),
        )
        .anchor(gpui::Corner::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(ui::PopoverMenuHandle::default())
        .into_any_element()
}

fn render_number_field<T: NumberFieldType + Send + Sync>(
    field: SettingField<T>,
    file: SettingsUiFile,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let value = value.copied().unwrap_or_else(T::min_value);
    NumberField::new("numeric_stepper", value, window, cx)
        .on_change({
            move |value, _window, cx| {
                let value = *value;
                update_settings_file(file.clone(), cx, move |settings, _cx| {
                    *(field.pick_mut)(settings) = Some(value);
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .tab_index(0)
        .into_any_element()
}

fn render_dropdown<T>(
    field: SettingField<T>,
    file: SettingsUiFile,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    let variants = || -> &'static [T] { <T as strum::VariantArray>::VARIANTS };
    let labels = || -> &'static [&'static str] { <T as strum::VariantNames>::VARIANTS };

    let (_, current_value) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value = current_value.copied().unwrap_or(variants()[0]);

    let current_value_label =
        labels()[variants().iter().position(|v| *v == current_value).unwrap()];

    DropdownMenu::new(
        "dropdown",
        current_value_label.to_title_case(),
        ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (&value, &label) in std::iter::zip(variants(), labels()) {
                let file = file.clone();
                menu = menu.toggleable_entry(
                    label.to_title_case(),
                    value == current_value,
                    IconPosition::Start,
                    None,
                    move |_, cx| {
                        if value == current_value {
                            return;
                        }
                        update_settings_file(file.clone(), cx, move |settings, _cx| {
                            *(field.pick_mut)(settings) = Some(value);
                        })
                        .log_err(); // todo(settings_ui) don't log err
                    },
                );
            }
            menu
        }),
    )
    .trigger_size(ButtonSize::Medium)
    .style(DropdownStyle::Outlined)
    .offset(gpui::Point {
        x: px(0.0),
        y: px(2.0),
    })
    .tab_index(0)
    .into_any_element()
}

#[cfg(test)]
mod test {

    use super::*;

    impl SettingsWindow {
        fn navbar_entry(&self) -> usize {
            self.navbar_entry
        }

        fn new_builder(window: &mut Window, cx: &mut Context<Self>) -> Self {
            let mut this = Self::new(None, window, cx);
            this.navbar_entries.clear();
            this.pages.clear();
            this
        }

        fn build(mut self) -> Self {
            self.build_search_matches();
            self.build_navbar();
            self
        }

        fn add_page(
            mut self,
            title: &'static str,
            build_page: impl Fn(SettingsPage) -> SettingsPage,
        ) -> Self {
            let page = SettingsPage {
                title,
                items: Vec::default(),
            };

            self.pages.push(build_page(page));
            self
        }

        fn search(&mut self, search_query: &str, window: &mut Window, cx: &mut Context<Self>) {
            self.search_task.take();
            self.search_bar.update(cx, |editor, cx| {
                editor.set_text(search_query, window, cx);
            });
            self.update_matches(cx);
        }

        fn assert_search_results(&self, other: &Self) {
            // page index could be different because of filtered out pages
            #[derive(Debug, PartialEq)]
            struct EntryMinimal {
                is_root: bool,
                title: &'static str,
            }
            pretty_assertions::assert_eq!(
                other
                    .visible_navbar_entries()
                    .map(|(_, entry)| EntryMinimal {
                        is_root: entry.is_root,
                        title: entry.title,
                    })
                    .collect::<Vec<_>>(),
                self.visible_navbar_entries()
                    .map(|(_, entry)| EntryMinimal {
                        is_root: entry.is_root,
                        title: entry.title,
                    })
                    .collect::<Vec<_>>(),
            );
            assert_eq!(
                self.current_page().items.iter().collect::<Vec<_>>(),
                other.page_items().collect::<Vec<_>>()
            );
        }
    }

    impl SettingsPage {
        fn item(mut self, item: SettingsPageItem) -> Self {
            self.items.push(item);
            self
        }
    }

    impl SettingsPageItem {
        fn basic_item(title: &'static str, description: &'static str) -> Self {
            SettingsPageItem::SettingItem(SettingItem {
                files: USER,
                title,
                description,
                field: Box::new(SettingField {
                    pick: |settings_content| &settings_content.auto_update,
                    pick_mut: |settings_content| &mut settings_content.auto_update,
                }),
                metadata: None,
            })
        }
    }

    fn register_settings(cx: &mut App) {
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        workspace::init_settings(cx);
        project::Project::init_settings(cx);
        language::init(cx);
        editor::init(cx);
        menu::init();
    }

    fn parse(input: &'static str, window: &mut Window, cx: &mut App) -> SettingsWindow {
        let mut pages: Vec<SettingsPage> = Vec::new();
        let mut expanded_pages = Vec::new();
        let mut selected_idx = None;
        let mut index = 0;
        let mut in_expanded_section = false;

        for mut line in input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
        {
            if let Some(pre) = line.strip_suffix('*') {
                assert!(selected_idx.is_none(), "Only one selected entry allowed");
                selected_idx = Some(index);
                line = pre;
            }
            let (kind, title) = line.split_once(" ").unwrap();
            assert_eq!(kind.len(), 1);
            let kind = kind.chars().next().unwrap();
            if kind == 'v' {
                let page_idx = pages.len();
                expanded_pages.push(page_idx);
                pages.push(SettingsPage {
                    title,
                    items: vec![],
                });
                index += 1;
                in_expanded_section = true;
            } else if kind == '>' {
                pages.push(SettingsPage {
                    title,
                    items: vec![],
                });
                index += 1;
                in_expanded_section = false;
            } else if kind == '-' {
                pages
                    .last_mut()
                    .unwrap()
                    .items
                    .push(SettingsPageItem::SectionHeader(title));
                if selected_idx == Some(index) && !in_expanded_section {
                    panic!("Items in unexpanded sections cannot be selected");
                }
                index += 1;
            } else {
                panic!(
                    "Entries must start with one of 'v', '>', or '-'\n line: {}",
                    line
                );
            }
        }

        let mut settings_window = SettingsWindow {
            original_window: None,
            worktree_root_dirs: HashMap::default(),
            files: Vec::default(),
            current_file: crate::SettingsUiFile::User,
            pages,
            search_bar: cx.new(|cx| Editor::single_line(window, cx)),
            navbar_entry: selected_idx.expect("Must have a selected navbar entry"),
            navbar_entries: Vec::default(),
            list_handle: UniformListScrollHandle::default(),
            search_matches: vec![],
            search_task: None,
            scroll_handle: ScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            navbar_focus_handle: cx.focus_handle(),
            content_focus_handle: cx.focus_handle(),
            files_focus_handle: cx.focus_handle(),
        };

        settings_window.build_search_matches();
        settings_window.build_navbar();
        for expanded_page_index in expanded_pages {
            for entry in &mut settings_window.navbar_entries {
                if entry.page_index == expanded_page_index && entry.is_root {
                    entry.expanded = true;
                }
            }
        }
        settings_window
    }

    #[track_caller]
    fn check_navbar_toggle(
        before: &'static str,
        toggle_page: &'static str,
        after: &'static str,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut settings_window = parse(before, window, cx);
        let toggle_page_idx = settings_window
            .pages
            .iter()
            .position(|page| page.title == toggle_page)
            .expect("page not found");
        let toggle_idx = settings_window
            .navbar_entries
            .iter()
            .position(|entry| entry.page_index == toggle_page_idx)
            .expect("page not found");
        settings_window.toggle_navbar_entry(toggle_idx);

        let expected_settings_window = parse(after, window, cx);

        pretty_assertions::assert_eq!(
            settings_window
                .visible_navbar_entries()
                .map(|(_, entry)| entry)
                .collect::<Vec<_>>(),
            expected_settings_window
                .visible_navbar_entries()
                .map(|(_, entry)| entry)
                .collect::<Vec<_>>(),
        );
        pretty_assertions::assert_eq!(
            settings_window.navbar_entries[settings_window.navbar_entry()],
            expected_settings_window.navbar_entries[expected_settings_window.navbar_entry()],
        );
    }

    macro_rules! check_navbar_toggle {
        ($name:ident, before: $before:expr, toggle_page: $toggle_page:expr, after: $after:expr) => {
            #[gpui::test]
            fn $name(cx: &mut gpui::TestAppContext) {
                let window = cx.add_empty_window();
                window.update(|window, cx| {
                    register_settings(cx);
                    check_navbar_toggle($before, $toggle_page, $after, window, cx);
                });
            }
        };
    }

    check_navbar_toggle!(
        navbar_basic_open,
        before: r"
        v General
        - General
        - Privacy*
        v Project
        - Project Settings
        ",
        toggle_page: "General",
        after: r"
        > General*
        v Project
        - Project Settings
        "
    );

    check_navbar_toggle!(
        navbar_basic_close,
        before: r"
        > General*
        - General
        - Privacy
        v Project
        - Project Settings
        ",
        toggle_page: "General",
        after: r"
        v General*
        - General
        - Privacy
        v Project
        - Project Settings
        "
    );

    check_navbar_toggle!(
        navbar_basic_second_root_entry_close,
        before: r"
        > General
        - General
        - Privacy
        v Project
        - Project Settings*
        ",
        toggle_page: "Project",
        after: r"
        > General
        > Project*
        "
    );

    check_navbar_toggle!(
        navbar_toggle_subroot,
        before: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content*
        v AI
        - General
        > Appearance & Behavior
        ",
        toggle_page: "Project",
        after: r"
        v General Page
        - General
        - Privacy
        > Project*
        v AI
        - General
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_close_propagates_selected_index,
        before: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_page: "General Page",
        after: r"
        > General Page
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_expand_propagates_selected_index,
        before: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_page: "General Page",
        after: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    #[gpui::test]
    fn test_basic_search(cx: &mut gpui::TestAppContext) {
        let cx = cx.add_empty_window();
        let (actual, expected) = cx.update(|window, cx| {
            register_settings(cx);

            let expected = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item("test title", "General test"))
                    })
                    .build()
            });

            let actual = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item("test title", "General test"))
                    })
                    .add_page("Theme", |page| {
                        page.item(SettingsPageItem::SectionHeader("Theme settings"))
                    })
                    .build()
            });

            actual.update(cx, |settings, cx| settings.search("gen", window, cx));

            (actual, expected)
        });

        cx.cx.run_until_parked();

        cx.update(|_window, cx| {
            let expected = expected.read(cx);
            let actual = actual.read(cx);
            expected.assert_search_results(&actual);
        })
    }

    #[gpui::test]
    fn test_search_render_page_with_filtered_out_navbar_entries(cx: &mut gpui::TestAppContext) {
        let cx = cx.add_empty_window();
        let (actual, expected) = cx.update(|window, cx| {
            register_settings(cx);

            let actual = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item(
                                "Confirm Quit",
                                "Whether to confirm before quitting Zed",
                            ))
                            .item(SettingsPageItem::basic_item(
                                "Auto Update",
                                "Automatically update Zed",
                            ))
                    })
                    .add_page("AI", |page| {
                        page.item(SettingsPageItem::basic_item(
                            "Disable AI",
                            "Whether to disable all AI features in Zed",
                        ))
                    })
                    .add_page("Appearance & Behavior", |page| {
                        page.item(SettingsPageItem::SectionHeader("Cursor")).item(
                            SettingsPageItem::basic_item(
                                "Cursor Shape",
                                "Cursor shape for the editor",
                            ),
                        )
                    })
                    .build()
            });

            let expected = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("Appearance & Behavior", |page| {
                        page.item(SettingsPageItem::SectionHeader("Cursor")).item(
                            SettingsPageItem::basic_item(
                                "Cursor Shape",
                                "Cursor shape for the editor",
                            ),
                        )
                    })
                    .build()
            });

            actual.update(cx, |settings, cx| settings.search("cursor", window, cx));

            (actual, expected)
        });

        cx.cx.run_until_parked();

        cx.update(|_window, cx| {
            let expected = expected.read(cx);
            let actual = actual.read(cx);
            expected.assert_search_results(&actual);
        })
    }
}
