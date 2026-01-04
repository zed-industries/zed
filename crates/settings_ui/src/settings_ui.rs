mod components;
mod page_data;
mod pages;

use anyhow::Result;
use editor::{Editor, EditorEvent};
use fuzzy::StringMatchCandidate;
use gpui::{
    Action, App, ClipboardItem, DEFAULT_ADDITIONAL_WINDOW_SIZE, Div, Entity, FocusHandle,
    Focusable, Global, KeyContext, ListState, ReadGlobal as _, ScrollHandle, Stateful,
    Subscription, Task, TitlebarOptions, UniformListScrollHandle, Window, WindowBounds,
    WindowHandle, WindowOptions, actions, div, list, point, prelude::*, px, uniform_list,
};
use project::{Project, WorktreeId};
use release_channel::ReleaseChannel;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{Settings, SettingsContent, SettingsStore, initial_project_settings_content};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::{HashMap, HashSet},
    num::{NonZero, NonZeroU32},
    ops::Range,
    rc::Rc,
    sync::{Arc, LazyLock, RwLock},
    time::Duration,
};
use title_bar::platform_title_bar::PlatformTitleBar;
use ui::{
    Banner, ContextMenu, Divider, DropdownMenu, DropdownStyle, IconButtonShape, KeyBinding,
    KeybindingHint, PopoverMenu, Switch, Tooltip, TreeViewItem, WithScrollbar, prelude::*,
};
use ui_input::{NumberField, NumberFieldMode, NumberFieldType};
use util::{ResultExt as _, paths::PathStyle, rel_path::RelPath};
use workspace::{AppState, OpenOptions, OpenVisible, Workspace, client_side_decorations};
use zed_actions::{OpenProjectSettings, OpenSettings, OpenSettingsAt};

use crate::components::{
    EnumVariantDropdown, SettingsInputField, SettingsSectionHeader, font_picker, icon_theme_picker,
    theme_picker,
};

const NAVBAR_CONTAINER_TAB_INDEX: isize = 0;
const NAVBAR_GROUP_TAB_INDEX: isize = 1;

const HEADER_CONTAINER_TAB_INDEX: isize = 2;
const HEADER_GROUP_TAB_INDEX: isize = 3;

const CONTENT_CONTAINER_TAB_INDEX: isize = 4;
const CONTENT_GROUP_TAB_INDEX: isize = 5;

actions!(
    settings_editor,
    [
        /// Minimizes the settings UI window.
        Minimize,
        /// Toggles focus between the navbar and the main content.
        ToggleFocusNav,
        /// Expands the navigation entry.
        ExpandNavEntry,
        /// Collapses the navigation entry.
        CollapseNavEntry,
        /// Focuses the next file in the file list.
        FocusNextFile,
        /// Focuses the previous file in the file list.
        FocusPreviousFile,
        /// Opens an editor for the current file
        OpenCurrentFile,
        /// Focuses the previous root navigation entry.
        FocusPreviousRootNavEntry,
        /// Focuses the next root navigation entry.
        FocusNextRootNavEntry,
        /// Focuses the first navigation entry.
        FocusFirstNavEntry,
        /// Focuses the last navigation entry.
        FocusLastNavEntry,
        /// Focuses and opens the next navigation entry without moving focus to content.
        FocusNextNavEntry,
        /// Focuses and opens the previous navigation entry without moving focus to content.
        FocusPreviousNavEntry
    ]
);

#[derive(Action, PartialEq, Eq, Clone, Copy, Debug, JsonSchema, Deserialize)]
#[action(namespace = settings_editor)]
struct FocusFile(pub u32);

struct SettingField<T: 'static> {
    pick: fn(&SettingsContent) -> Option<&T>,
    write: fn(&mut SettingsContent, Option<T>),

    /// A json-path-like string that gives a unique-ish string that identifies
    /// where in the JSON the setting is defined.
    ///
    /// The syntax is `jq`-like, but modified slightly to be URL-safe (and
    /// without the leading dot), e.g. `foo.bar`.
    ///
    /// They are URL-safe (this is important since links are the main use-case
    /// for these paths).
    ///
    /// There are a couple of special cases:
    /// - discrimminants are represented with a trailing `$`, for example
    /// `terminal.working_directory$`. This is to distinguish the discrimminant
    /// setting (i.e. the setting that changes whether the value is a string or
    /// an object) from the setting in the case that it is a string.
    /// - language-specific settings begin `languages.$(language)`. Links
    /// targeting these settings should take the form `languages/Rust/...`, for
    /// example, but are not currently supported.
    json_path: Option<&'static str>,
}

impl<T: 'static> Clone for SettingField<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// manual impl because derive puts a Copy bound on T, which is inaccurate in our case
impl<T: 'static> Copy for SettingField<T> {}

/// Helper for unimplemented settings, used in combination with `SettingField::unimplemented`
/// to keep the setting around in the UI with valid pick and write implementations, but don't actually try to render it.
/// TODO(settings_ui): In non-dev builds (`#[cfg(not(debug_assertions))]`) make this render as edit-in-json
#[derive(Clone, Copy)]
struct UnimplementedSettingField;

impl PartialEq for UnimplementedSettingField {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: 'static> SettingField<T> {
    /// Helper for settings with types that are not yet implemented.
    #[allow(unused)]
    fn unimplemented(self) -> SettingField<UnimplementedSettingField> {
        SettingField {
            pick: |_| Some(&UnimplementedSettingField),
            write: |_, _| unreachable!(),
            json_path: self.json_path,
        }
    }
}

trait AnySettingField {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    // Returns the file this value was set in and true, or File::Default and false to indicate it was not found in any file (missing default)
    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> (settings::SettingsFile, bool);
    fn reset_to_default_fn(
        &self,
        current_file: &SettingsUiFile,
        file_set_in: &settings::SettingsFile,
        cx: &App,
    ) -> Option<Box<dyn Fn(&mut App)>>;

    fn json_path(&self) -> Option<&'static str>;
}

impl<T: PartialEq + Clone + Send + Sync + 'static> AnySettingField for SettingField<T> {
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
        let (file, value) = cx
            .global::<SettingsStore>()
            .get_value_from_file(file.to_settings(), self.pick);
        return (file, value.is_some());
    }

    fn reset_to_default_fn(
        &self,
        current_file: &SettingsUiFile,
        file_set_in: &settings::SettingsFile,
        cx: &App,
    ) -> Option<Box<dyn Fn(&mut App)>> {
        if file_set_in == &settings::SettingsFile::Default {
            return None;
        }
        if file_set_in != &current_file.to_settings() {
            return None;
        }
        let this = *self;
        let store = SettingsStore::global(cx);
        let default_value = (this.pick)(store.raw_default_settings());
        let is_default = store
            .get_content_for_file(file_set_in.clone())
            .map_or(None, this.pick)
            == default_value;
        if is_default {
            return None;
        }
        let current_file = current_file.clone();

        return Some(Box::new(move |cx| {
            let store = SettingsStore::global(cx);
            let default_value = (this.pick)(store.raw_default_settings());
            let is_set_somewhere_other_than_default = store
                .get_value_up_to_file(current_file.to_settings(), this.pick)
                .0
                != settings::SettingsFile::Default;
            let value_to_set = if is_set_somewhere_other_than_default {
                default_value.cloned()
            } else {
                None
            };
            update_settings_file(current_file.clone(), None, cx, move |settings, _| {
                (this.write)(settings, value_to_set);
            })
            // todo(settings_ui): Don't log err
            .log_err();
        }));
    }

    fn json_path(&self) -> Option<&'static str> {
        self.json_path
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
                        &SettingsWindow,
                        &SettingItem,
                        SettingsUiFile,
                        Option<&SettingsFieldMetadata>,
                        bool,
                        &mut Window,
                        &mut Context<SettingsWindow>,
                    ) -> Stateful<Div>,
                >,
            >,
        >,
    >,
}

impl Global for SettingFieldRenderer {}

impl SettingFieldRenderer {
    fn add_basic_renderer<T: 'static>(
        &mut self,
        render_control: impl Fn(
            SettingField<T>,
            SettingsUiFile,
            Option<&SettingsFieldMetadata>,
            &mut Window,
            &mut App,
        ) -> AnyElement
        + 'static,
    ) -> &mut Self {
        self.add_renderer(
            move |settings_window: &SettingsWindow,
                  item: &SettingItem,
                  field: SettingField<T>,
                  settings_file: SettingsUiFile,
                  metadata: Option<&SettingsFieldMetadata>,
                  sub_field: bool,
                  window: &mut Window,
                  cx: &mut Context<SettingsWindow>| {
                render_settings_item(
                    settings_window,
                    item,
                    settings_file.clone(),
                    render_control(field, settings_file, metadata, window, cx),
                    sub_field,
                    cx,
                )
            },
        )
    }

    fn add_renderer<T: 'static>(
        &mut self,
        renderer: impl Fn(
            &SettingsWindow,
            &SettingItem,
            SettingField<T>,
            SettingsUiFile,
            Option<&SettingsFieldMetadata>,
            bool,
            &mut Window,
            &mut Context<SettingsWindow>,
        ) -> Stateful<Div>
        + 'static,
    ) -> &mut Self {
        let key = TypeId::of::<T>();
        let renderer = Box::new(
            move |settings_window: &SettingsWindow,
                  item: &SettingItem,
                  settings_file: SettingsUiFile,
                  metadata: Option<&SettingsFieldMetadata>,
                  sub_field: bool,
                  window: &mut Window,
                  cx: &mut Context<SettingsWindow>| {
                let field = *item
                    .field
                    .as_ref()
                    .as_any()
                    .downcast_ref::<SettingField<T>>()
                    .unwrap();
                renderer(
                    settings_window,
                    item,
                    field,
                    settings_file,
                    metadata,
                    sub_field,
                    window,
                    cx,
                )
            },
        );
        self.renderers.borrow_mut().insert(key, renderer);
        self
    }
}

struct NonFocusableHandle {
    handle: FocusHandle,
    _subscription: Subscription,
}

impl NonFocusableHandle {
    fn new(tab_index: isize, tab_stop: bool, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let handle = cx.focus_handle().tab_index(tab_index).tab_stop(tab_stop);
        Self::from_handle(handle, window, cx)
    }

    fn from_handle(handle: FocusHandle, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let _subscription = cx.on_focus(&handle, window, {
                move |_, window, cx| {
                    window.focus_next(cx);
                }
            });
            Self {
                handle,
                _subscription,
            }
        })
    }
}

impl Focusable for NonFocusableHandle {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.handle.clone()
    }
}

#[derive(Default)]
struct SettingsFieldMetadata {
    placeholder: Option<&'static str>,
    should_do_titlecase: Option<bool>,
}

pub fn init(cx: &mut App) {
    init_renderers(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace
            .register_action(
                |workspace, OpenSettingsAt { path }: &OpenSettingsAt, window, cx| {
                    let window_handle = window
                        .window_handle()
                        .downcast::<Workspace>()
                        .expect("Workspaces are root Windows");
                    open_settings_editor(workspace, Some(&path), false, window_handle, cx);
                },
            )
            .register_action(|workspace, _: &OpenSettings, window, cx| {
                let window_handle = window
                    .window_handle()
                    .downcast::<Workspace>()
                    .expect("Workspaces are root Windows");
                open_settings_editor(workspace, None, false, window_handle, cx);
            })
            .register_action(|workspace, _: &OpenProjectSettings, window, cx| {
                let window_handle = window
                    .window_handle()
                    .downcast::<Workspace>()
                    .expect("Workspaces are root Windows");
                open_settings_editor(workspace, None, true, window_handle, cx);
            });
    })
    .detach();
}

fn init_renderers(cx: &mut App) {
    cx.default_global::<SettingFieldRenderer>()
        .add_renderer::<UnimplementedSettingField>(
            |settings_window, item, _, settings_file, _, sub_field, _, cx| {
                render_settings_item(
                    settings_window,
                    item,
                    settings_file,
                    Button::new("open-in-settings-file", "Edit in settings.json")
                        .style(ButtonStyle::Outlined)
                        .size(ButtonSize::Medium)
                        .tab_index(0_isize)
                        .tooltip(Tooltip::for_action_title_in(
                            "Edit in settings.json",
                            &OpenCurrentFile,
                            &settings_window.focus_handle,
                        ))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.open_current_settings_file(window, cx);
                        }))
                        .into_any_element(),
                    sub_field,
                    cx,
                )
            },
        )
        .add_basic_renderer::<bool>(render_toggle_button)
        .add_basic_renderer::<String>(render_text_field)
        .add_basic_renderer::<SharedString>(render_text_field)
        .add_basic_renderer::<settings::SaturatingBool>(render_toggle_button)
        .add_basic_renderer::<settings::CursorShape>(render_dropdown)
        .add_basic_renderer::<settings::RestoreOnStartupBehavior>(render_dropdown)
        .add_basic_renderer::<settings::BottomDockLayout>(render_dropdown)
        .add_basic_renderer::<settings::OnLastWindowClosed>(render_dropdown)
        .add_basic_renderer::<settings::CloseWindowWhenNoItems>(render_dropdown)
        .add_basic_renderer::<settings::FontFamilyName>(render_font_picker)
        .add_basic_renderer::<settings::BaseKeymapContent>(render_dropdown)
        .add_basic_renderer::<settings::MultiCursorModifier>(render_dropdown)
        .add_basic_renderer::<settings::HideMouseMode>(render_dropdown)
        .add_basic_renderer::<settings::CurrentLineHighlight>(render_dropdown)
        .add_basic_renderer::<settings::ShowWhitespaceSetting>(render_dropdown)
        .add_basic_renderer::<settings::SoftWrap>(render_dropdown)
        .add_basic_renderer::<settings::ScrollBeyondLastLine>(render_dropdown)
        .add_basic_renderer::<settings::SnippetSortOrder>(render_dropdown)
        .add_basic_renderer::<settings::ClosePosition>(render_dropdown)
        .add_basic_renderer::<settings::DockSide>(render_dropdown)
        .add_basic_renderer::<settings::TerminalDockPosition>(render_dropdown)
        .add_basic_renderer::<settings::DockPosition>(render_dropdown)
        .add_basic_renderer::<settings::GitGutterSetting>(render_dropdown)
        .add_basic_renderer::<settings::GitHunkStyleSetting>(render_dropdown)
        .add_basic_renderer::<settings::GitPathStyle>(render_dropdown)
        .add_basic_renderer::<settings::DiagnosticSeverityContent>(render_dropdown)
        .add_basic_renderer::<settings::SeedQuerySetting>(render_dropdown)
        .add_basic_renderer::<settings::DoubleClickInMultibuffer>(render_dropdown)
        .add_basic_renderer::<settings::GoToDefinitionFallback>(render_dropdown)
        .add_basic_renderer::<settings::ActivateOnClose>(render_dropdown)
        .add_basic_renderer::<settings::ShowDiagnostics>(render_dropdown)
        .add_basic_renderer::<settings::ShowCloseButton>(render_dropdown)
        .add_basic_renderer::<settings::ProjectPanelEntrySpacing>(render_dropdown)
        .add_basic_renderer::<settings::ProjectPanelSortMode>(render_dropdown)
        .add_basic_renderer::<settings::RewrapBehavior>(render_dropdown)
        .add_basic_renderer::<settings::FormatOnSave>(render_dropdown)
        .add_basic_renderer::<settings::IndentGuideColoring>(render_dropdown)
        .add_basic_renderer::<settings::IndentGuideBackgroundColoring>(render_dropdown)
        .add_basic_renderer::<settings::FileFinderWidthContent>(render_dropdown)
        .add_basic_renderer::<settings::ShowDiagnostics>(render_dropdown)
        .add_basic_renderer::<settings::WordsCompletionMode>(render_dropdown)
        .add_basic_renderer::<settings::LspInsertMode>(render_dropdown)
        .add_basic_renderer::<settings::CompletionDetailAlignment>(render_dropdown)
        .add_basic_renderer::<settings::AlternateScroll>(render_dropdown)
        .add_basic_renderer::<settings::TerminalBlink>(render_dropdown)
        .add_basic_renderer::<settings::CursorShapeContent>(render_dropdown)
        .add_basic_renderer::<f32>(render_number_field)
        .add_basic_renderer::<u32>(render_number_field)
        .add_basic_renderer::<u64>(render_number_field)
        .add_basic_renderer::<usize>(render_number_field)
        .add_basic_renderer::<NonZero<usize>>(render_number_field)
        .add_basic_renderer::<NonZeroU32>(render_number_field)
        .add_basic_renderer::<settings::CodeFade>(render_number_field)
        .add_basic_renderer::<settings::DelayMs>(render_number_field)
        .add_basic_renderer::<gpui::FontWeight>(render_number_field)
        .add_basic_renderer::<settings::CenteredPaddingSettings>(render_number_field)
        .add_basic_renderer::<settings::InactiveOpacity>(render_number_field)
        .add_basic_renderer::<settings::MinimumContrast>(render_number_field)
        .add_basic_renderer::<settings::ShowScrollbar>(render_dropdown)
        .add_basic_renderer::<settings::ScrollbarDiagnostics>(render_dropdown)
        .add_basic_renderer::<settings::ShowMinimap>(render_dropdown)
        .add_basic_renderer::<settings::DisplayIn>(render_dropdown)
        .add_basic_renderer::<settings::MinimapThumb>(render_dropdown)
        .add_basic_renderer::<settings::MinimapThumbBorder>(render_dropdown)
        .add_basic_renderer::<settings::SteppingGranularity>(render_dropdown)
        .add_basic_renderer::<settings::NotifyWhenAgentWaiting>(render_dropdown)
        .add_basic_renderer::<settings::NotifyWhenAgentWaiting>(render_dropdown)
        .add_basic_renderer::<settings::ImageFileSizeUnit>(render_dropdown)
        .add_basic_renderer::<settings::StatusStyle>(render_dropdown)
        .add_basic_renderer::<settings::PaneSplitDirectionHorizontal>(render_dropdown)
        .add_basic_renderer::<settings::PaneSplitDirectionVertical>(render_dropdown)
        .add_basic_renderer::<settings::PaneSplitDirectionVertical>(render_dropdown)
        .add_basic_renderer::<settings::DocumentColorsRenderMode>(render_dropdown)
        .add_basic_renderer::<settings::ThemeSelectionDiscriminants>(render_dropdown)
        .add_basic_renderer::<settings::ThemeAppearanceMode>(render_dropdown)
        .add_basic_renderer::<settings::ThemeName>(render_theme_picker)
        .add_basic_renderer::<settings::IconThemeSelectionDiscriminants>(render_dropdown)
        .add_basic_renderer::<settings::IconThemeName>(render_icon_theme_picker)
        .add_basic_renderer::<settings::BufferLineHeightDiscriminants>(render_dropdown)
        .add_basic_renderer::<settings::AutosaveSettingDiscriminants>(render_dropdown)
        .add_basic_renderer::<settings::WorkingDirectoryDiscriminants>(render_dropdown)
        .add_basic_renderer::<settings::IncludeIgnoredContent>(render_dropdown)
        .add_basic_renderer::<settings::ShowIndentGuides>(render_dropdown)
        .add_basic_renderer::<settings::ShellDiscriminants>(render_dropdown)
        .add_basic_renderer::<settings::EditPredictionsMode>(render_dropdown)
        .add_basic_renderer::<settings::RelativeLineNumbers>(render_dropdown)
        .add_basic_renderer::<settings::WindowDecorations>(render_dropdown)
        .add_basic_renderer::<settings::FontSize>(render_editable_number_field)
        // please semicolon stay on next line
        ;
}

pub fn open_settings_editor(
    _workspace: &mut Workspace,
    path: Option<&str>,
    open_project_settings: bool,
    workspace_handle: WindowHandle<Workspace>,
    cx: &mut App,
) {
    telemetry::event!("Settings Viewed");

    /// Assumes a settings GUI window is already open
    fn open_path(
        path: &str,
        // Note: This option is unsupported right now
        _open_project_settings: bool,
        settings_window: &mut SettingsWindow,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) {
        if path.starts_with("languages.$(language)") {
            log::error!("language-specific settings links are not currently supported");
            return;
        }

        settings_window.search_bar.update(cx, |editor, cx| {
            editor.set_text(format!("#{path}"), window, cx);
        });
        settings_window.update_matches(cx);
    }

    let existing_window = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<SettingsWindow>());

    if let Some(existing_window) = existing_window {
        existing_window
            .update(cx, |settings_window, window, cx| {
                settings_window.original_window = Some(workspace_handle);
                window.activate_window();
                if let Some(path) = path {
                    open_path(path, open_project_settings, settings_window, window, cx);
                } else if open_project_settings {
                    if let Some(file_index) = settings_window
                        .files
                        .iter()
                        .position(|(file, _)| file.worktree_id().is_some())
                    {
                        settings_window.change_file(file_index, window, cx);
                    }

                    cx.notify();
                }
            })
            .ok();
        return;
    }

    // We have to defer this to get the workspace off the stack.

    let path = path.map(ToOwned::to_owned);
    cx.defer(move |cx| {
        let current_rem_size: f32 = theme::ThemeSettings::get_global(cx).ui_font_size(cx).into();

        let default_bounds = DEFAULT_ADDITIONAL_WINDOW_SIZE;
        let default_rem_size = 16.0;
        let scale_factor = current_rem_size / default_rem_size;
        let scaled_bounds: gpui::Size<Pixels> = default_bounds.map(|axis| axis * scale_factor);

        let app_id = ReleaseChannel::global(cx).app_id();
        let window_decorations = match std::env::var("ZED_WINDOW_DECORATIONS") {
            Ok(val) if val == "server" => gpui::WindowDecorations::Server,
            Ok(val) if val == "client" => gpui::WindowDecorations::Client,
            _ => gpui::WindowDecorations::Client,
        };

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Zed — Settings".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(12.0))),
                }),
                focus: true,
                show: true,
                is_movable: true,
                kind: gpui::WindowKind::Normal,
                window_background: cx.theme().window_background_appearance(),
                app_id: Some(app_id.to_owned()),
                window_decorations: Some(window_decorations),
                window_min_size: Some(gpui::Size {
                    // Don't make the settings window thinner than this,
                    // otherwise, it gets unusable. Users with smaller res monitors
                    // can customize the height, but not the width.
                    width: px(900.0),
                    height: px(240.0),
                }),
                window_bounds: Some(WindowBounds::centered(scaled_bounds, cx)),
                ..Default::default()
            },
            |window, cx| {
                let settings_window =
                    cx.new(|cx| SettingsWindow::new(Some(workspace_handle), window, cx));
                settings_window.update(cx, |settings_window, cx| {
                    if let Some(path) = path {
                        open_path(&path, open_project_settings, settings_window, window, cx);
                    } else if open_project_settings {
                        if let Some(file_index) = settings_window
                            .files
                            .iter()
                            .position(|(file, _)| file.worktree_id().is_some())
                        {
                            settings_window.change_file(file_index, window, cx);
                        }

                        settings_window.fetch_files(window, cx);
                    }
                });

                settings_window
            },
        )
        .log_err();
    });
}

/// The current sub page path that is selected.
/// If this is empty the selected page is rendered,
/// otherwise the last sub page gets rendered.
///
/// Global so that `pick` and `write` callbacks can access it
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
    title_bar: Option<Entity<PlatformTitleBar>>,
    original_window: Option<WindowHandle<Workspace>>,
    files: Vec<(SettingsUiFile, FocusHandle)>,
    worktree_root_dirs: HashMap<WorktreeId, String>,
    current_file: SettingsUiFile,
    pages: Vec<SettingsPage>,
    search_bar: Entity<Editor>,
    search_task: Option<Task<()>>,
    /// Index into navbar_entries
    navbar_entry: usize,
    navbar_entries: Vec<NavBarEntry>,
    navbar_scroll_handle: UniformListScrollHandle,
    /// [page_index][page_item_index] will be false
    /// when the item is filtered out either by searches
    /// or by the current file
    navbar_focus_subscriptions: Vec<gpui::Subscription>,
    filter_table: Vec<Vec<bool>>,
    has_query: bool,
    content_handles: Vec<Vec<Entity<NonFocusableHandle>>>,
    sub_page_scroll_handle: ScrollHandle,
    focus_handle: FocusHandle,
    navbar_focus_handle: Entity<NonFocusableHandle>,
    content_focus_handle: Entity<NonFocusableHandle>,
    files_focus_handle: FocusHandle,
    search_index: Option<Arc<SearchIndex>>,
    list_state: ListState,
    shown_errors: HashSet<String>,
}

struct SearchIndex {
    bm25_engine: bm25::SearchEngine<usize>,
    fuzzy_match_candidates: Vec<StringMatchCandidate>,
    key_lut: Vec<SearchKeyLUTEntry>,
}

struct SearchKeyLUTEntry {
    page_index: usize,
    header_index: usize,
    item_index: usize,
    json_path: Option<&'static str>,
}

struct SubPage {
    link: SubPageLink,
    section_header: &'static str,
}

#[derive(Debug)]
struct NavBarEntry {
    title: &'static str,
    is_root: bool,
    expanded: bool,
    page_index: usize,
    item_index: Option<usize>,
    focus_handle: FocusHandle,
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
    DynamicItem(DynamicItem),
    ActionLink(ActionLink),
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
            SettingsPageItem::DynamicItem(dynamic_item) => {
                write!(f, "DynamicItem({})", dynamic_item.discriminant.title)
            }
            SettingsPageItem::ActionLink(action_link) => {
                write!(f, "ActionLink({})", action_link.title)
            }
        }
    }
}

impl SettingsPageItem {
    fn render(
        &self,
        settings_window: &SettingsWindow,
        item_index: usize,
        is_last: bool,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> AnyElement {
        let file = settings_window.current_file.clone();

        let apply_padding = |element: Stateful<Div>| -> Stateful<Div> {
            let element = element.pt_4();
            if is_last {
                element.pb_10()
            } else {
                element.pb_4()
            }
        };

        let mut render_setting_item_inner =
            |setting_item: &SettingItem,
             padding: bool,
             sub_field: bool,
             cx: &mut Context<SettingsWindow>| {
                let renderer = cx.default_global::<SettingFieldRenderer>().clone();
                let (_, found) = setting_item.field.file_set_in(file.clone(), cx);

                let renderers = renderer.renderers.borrow();

                let field_renderer =
                    renderers.get(&AnySettingField::type_id(setting_item.field.as_ref()));
                let field_renderer_or_warning =
                    field_renderer.ok_or("NO RENDERER").and_then(|renderer| {
                        if cfg!(debug_assertions) && !found {
                            Err("NO DEFAULT")
                        } else {
                            Ok(renderer)
                        }
                    });

                let field = match field_renderer_or_warning {
                    Ok(field_renderer) => window.with_id(item_index, |window| {
                        field_renderer(
                            settings_window,
                            setting_item,
                            file.clone(),
                            setting_item.metadata.as_deref(),
                            sub_field,
                            window,
                            cx,
                        )
                    }),
                    Err(warning) => render_settings_item(
                        settings_window,
                        setting_item,
                        file.clone(),
                        Button::new("error-warning", warning)
                            .style(ButtonStyle::Outlined)
                            .size(ButtonSize::Medium)
                            .icon(Some(IconName::Debug))
                            .icon_position(IconPosition::Start)
                            .icon_color(Color::Error)
                            .tab_index(0_isize)
                            .tooltip(Tooltip::text(setting_item.field.type_name()))
                            .into_any_element(),
                        sub_field,
                        cx,
                    ),
                };

                let field = if padding {
                    field.map(apply_padding)
                } else {
                    field
                };

                (field, field_renderer_or_warning.is_ok())
            };

        match self {
            SettingsPageItem::SectionHeader(header) => {
                SettingsSectionHeader::new(SharedString::new_static(header)).into_any_element()
            }
            SettingsPageItem::SettingItem(setting_item) => {
                let (field_with_padding, _) =
                    render_setting_item_inner(setting_item, true, false, cx);

                v_flex()
                    .group("setting-item")
                    .px_8()
                    .child(field_with_padding)
                    .when(!is_last, |this| this.child(Divider::horizontal()))
                    .into_any_element()
            }
            SettingsPageItem::SubPageLink(sub_page_link) => v_flex()
                .group("setting-item")
                .px_8()
                .child(
                    h_flex()
                        .id(sub_page_link.title.clone())
                        .w_full()
                        .min_w_0()
                        .justify_between()
                        .map(apply_padding)
                        .child(
                            v_flex()
                                .relative()
                                .w_full()
                                .max_w_1_2()
                                .child(Label::new(sub_page_link.title.clone()))
                                .when_some(
                                    sub_page_link.description.as_ref(),
                                    |this, description| {
                                        this.child(
                                            Label::new(description.clone())
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    },
                                ),
                        )
                        .child(
                            Button::new(
                                ("sub-page".into(), sub_page_link.title.clone()),
                                "Configure",
                            )
                            .icon(IconName::ChevronRight)
                            .tab_index(0_isize)
                            .icon_position(IconPosition::End)
                            .icon_color(Color::Muted)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::OutlinedGhost)
                            .size(ButtonSize::Medium)
                            .on_click({
                                let sub_page_link = sub_page_link.clone();
                                cx.listener(move |this, _, window, cx| {
                                    let mut section_index = item_index;
                                    let current_page = this.current_page();

                                    while !matches!(
                                        current_page.items[section_index],
                                        SettingsPageItem::SectionHeader(_)
                                    ) {
                                        section_index -= 1;
                                    }

                                    let SettingsPageItem::SectionHeader(header) =
                                        current_page.items[section_index]
                                    else {
                                        unreachable!(
                                            "All items always have a section header above them"
                                        )
                                    };

                                    this.push_sub_page(sub_page_link.clone(), header, window, cx)
                                })
                            }),
                        )
                        .child(render_settings_item_link(
                            sub_page_link.title.clone(),
                            sub_page_link.json_path,
                            false,
                            cx,
                        )),
                )
                .when(!is_last, |this| this.child(Divider::horizontal()))
                .into_any_element(),
            SettingsPageItem::DynamicItem(DynamicItem {
                discriminant: discriminant_setting_item,
                pick_discriminant,
                fields,
            }) => {
                let file = file.to_settings();
                let discriminant = SettingsStore::global(cx)
                    .get_value_from_file(file, *pick_discriminant)
                    .1;

                let (discriminant_element, rendered_ok) =
                    render_setting_item_inner(discriminant_setting_item, true, false, cx);

                let has_sub_fields =
                    rendered_ok && discriminant.map(|d| !fields[d].is_empty()).unwrap_or(false);

                let mut content = v_flex()
                    .id("dynamic-item")
                    .child(
                        div()
                            .group("setting-item")
                            .px_8()
                            .child(discriminant_element.when(has_sub_fields, |this| this.pb_4())),
                    )
                    .when(!has_sub_fields && !is_last, |this| {
                        this.child(h_flex().px_8().child(Divider::horizontal()))
                    });

                if rendered_ok {
                    let discriminant =
                        discriminant.expect("This should be Some if rendered_ok is true");
                    let sub_fields = &fields[discriminant];
                    let sub_field_count = sub_fields.len();

                    for (index, field) in sub_fields.iter().enumerate() {
                        let is_last_sub_field = index == sub_field_count - 1;
                        let (raw_field, _) = render_setting_item_inner(field, false, true, cx);

                        content = content.child(
                            raw_field
                                .group("setting-sub-item")
                                .mx_8()
                                .p_4()
                                .border_t_1()
                                .when(is_last_sub_field, |this| this.border_b_1())
                                .when(is_last_sub_field && is_last, |this| this.mb_8())
                                .border_dashed()
                                .border_color(cx.theme().colors().border_variant)
                                .bg(cx.theme().colors().element_background.opacity(0.2)),
                        );
                    }
                }

                return content.into_any_element();
            }
            SettingsPageItem::ActionLink(action_link) => v_flex()
                .group("setting-item")
                .px_8()
                .child(
                    h_flex()
                        .id(action_link.title.clone())
                        .w_full()
                        .min_w_0()
                        .justify_between()
                        .map(apply_padding)
                        .child(
                            v_flex()
                                .relative()
                                .w_full()
                                .max_w_1_2()
                                .child(Label::new(action_link.title.clone()))
                                .when_some(
                                    action_link.description.as_ref(),
                                    |this, description| {
                                        this.child(
                                            Label::new(description.clone())
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                    },
                                ),
                        )
                        .child(
                            Button::new(
                                ("action-link".into(), action_link.title.clone()),
                                action_link.button_text.clone(),
                            )
                            .icon(IconName::ArrowUpRight)
                            .tab_index(0_isize)
                            .icon_position(IconPosition::End)
                            .icon_color(Color::Muted)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::OutlinedGhost)
                            .size(ButtonSize::Medium)
                            .on_click({
                                let on_click = action_link.on_click.clone();
                                cx.listener(move |this, _, window, cx| {
                                    on_click(this, window, cx);
                                })
                            }),
                        ),
                )
                .when(!is_last, |this| this.child(Divider::horizontal()))
                .into_any_element(),
        }
    }
}

fn render_settings_item(
    settings_window: &SettingsWindow,
    setting_item: &SettingItem,
    file: SettingsUiFile,
    control: AnyElement,
    sub_field: bool,
    cx: &mut Context<'_, SettingsWindow>,
) -> Stateful<Div> {
    let (found_in_file, _) = setting_item.field.file_set_in(file.clone(), cx);
    let file_set_in = SettingsUiFile::from_settings(found_in_file.clone());

    h_flex()
        .id(setting_item.title)
        .min_w_0()
        .justify_between()
        .child(
            v_flex()
                .relative()
                .w_1_2()
                .child(
                    h_flex()
                        .w_full()
                        .gap_1()
                        .child(Label::new(SharedString::new_static(setting_item.title)))
                        .when_some(
                            if sub_field {
                                None
                            } else {
                                setting_item
                                    .field
                                    .reset_to_default_fn(&file, &found_in_file, cx)
                            },
                            |this, reset_to_default| {
                                this.child(
                                    IconButton::new("reset-to-default-btn", IconName::Undo)
                                        .icon_color(Color::Muted)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Reset to Default"))
                                        .on_click({
                                            move |_, _, cx| {
                                                reset_to_default(cx);
                                            }
                                        }),
                                )
                            },
                        )
                        .when_some(
                            file_set_in.filter(|file_set_in| file_set_in != &file),
                            |this, file_set_in| {
                                this.child(
                                    Label::new(format!(
                                        "—  Modified in {}",
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
        .child(control)
        .when(sub_page_stack().is_empty(), |this| {
            this.child(render_settings_item_link(
                setting_item.description,
                setting_item.field.json_path(),
                sub_field,
                cx,
            ))
        })
}

fn render_settings_item_link(
    id: impl Into<ElementId>,
    json_path: Option<&'static str>,
    sub_field: bool,
    cx: &mut Context<'_, SettingsWindow>,
) -> impl IntoElement {
    let clipboard_has_link = cx
        .read_from_clipboard()
        .and_then(|entry| entry.text())
        .map_or(false, |maybe_url| {
            json_path.is_some() && maybe_url.strip_prefix("zed://settings/") == json_path
        });

    let (link_icon, link_icon_color) = if clipboard_has_link {
        (IconName::Check, Color::Success)
    } else {
        (IconName::Link, Color::Muted)
    };

    div()
        .absolute()
        .top(rems_from_px(18.))
        .map(|this| {
            if sub_field {
                this.visible_on_hover("setting-sub-item")
                    .left(rems_from_px(-8.5))
            } else {
                this.visible_on_hover("setting-item")
                    .left(rems_from_px(-22.))
            }
        })
        .child(
            IconButton::new((id.into(), "copy-link-btn"), link_icon)
                .icon_color(link_icon_color)
                .icon_size(IconSize::Small)
                .shape(IconButtonShape::Square)
                .tooltip(Tooltip::text("Copy Link"))
                .when_some(json_path, |this, path| {
                    this.on_click(cx.listener(move |_, _, _, cx| {
                        let link = format!("zed://settings/{}", path);
                        cx.write_to_clipboard(ClipboardItem::new_string(link));
                        cx.notify();
                    }))
                }),
        )
}

struct SettingItem {
    title: &'static str,
    description: &'static str,
    field: Box<dyn AnySettingField>,
    metadata: Option<Box<SettingsFieldMetadata>>,
    files: FileMask,
}

struct DynamicItem {
    discriminant: SettingItem,
    pick_discriminant: fn(&SettingsContent) -> Option<usize>,
    fields: Vec<Vec<SettingItem>>,
}

impl PartialEq for DynamicItem {
    fn eq(&self, other: &Self) -> bool {
        self.discriminant == other.discriminant && self.fields == other.fields
    }
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
        if self.contains(PROJECT) {
            items.push("LOCAL");
        }
        if self.contains(SERVER) {
            items.push("SERVER");
        }

        write!(f, "{})", items.join(" | "))
    }
}

const USER: FileMask = FileMask(1 << 0);
const PROJECT: FileMask = FileMask(1 << 2);
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
    title: SharedString,
    description: Option<SharedString>,
    /// See [`SettingField.json_path`]
    json_path: Option<&'static str>,
    /// Whether or not the settings in this sub page are configurable in settings.json
    /// Removes the "Edit in settings.json" button from the page.
    in_json: bool,
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

#[derive(Clone)]
struct ActionLink {
    title: SharedString,
    description: Option<SharedString>,
    button_text: SharedString,
    on_click: Arc<dyn Fn(&mut SettingsWindow, &mut Window, &mut App) + Send + Sync>,
}

impl PartialEq for ActionLink {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

fn all_language_names(cx: &App) -> Vec<SharedString> {
    workspace::AppState::global(cx)
        .upgrade()
        .map_or(vec![], |state| {
            state
                .languages
                .language_names()
                .into_iter()
                .filter(|name| name.as_ref() != "Zed Keybind Context")
                .map(Into::into)
                .collect()
        })
}

#[allow(unused)]
#[derive(Clone, PartialEq, Debug)]
enum SettingsUiFile {
    User,                                // Uses all settings.
    Project((WorktreeId, Arc<RelPath>)), // Has a special name, and special set of settings
    Server(&'static str),                // Uses a special name, and the user settings
}

impl SettingsUiFile {
    fn setting_type(&self) -> &'static str {
        match self {
            SettingsUiFile::User => "User",
            SettingsUiFile::Project(_) => "Project",
            SettingsUiFile::Server(_) => "Server",
        }
    }

    fn is_server(&self) -> bool {
        matches!(self, SettingsUiFile::Server(_))
    }

    fn worktree_id(&self) -> Option<WorktreeId> {
        match self {
            SettingsUiFile::User => None,
            SettingsUiFile::Project((worktree_id, _)) => Some(*worktree_id),
            SettingsUiFile::Server(_) => None,
        }
    }

    fn from_settings(file: settings::SettingsFile) -> Option<Self> {
        Some(match file {
            settings::SettingsFile::User => SettingsUiFile::User,
            settings::SettingsFile::Project(location) => SettingsUiFile::Project(location),
            settings::SettingsFile::Server => SettingsUiFile::Server("todo: server name"),
            settings::SettingsFile::Default => return None,
            settings::SettingsFile::Global => return None,
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
            SettingsUiFile::Project(_) => PROJECT,
            SettingsUiFile::Server(_) => SERVER,
        }
    }
}

impl SettingsWindow {
    fn new(
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

        cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
            this.fetch_files(window, cx);
            cx.notify();
        })
        .detach();

        cx.on_window_closed(|cx| {
            if let Some(existing_window) = cx
                .windows()
                .into_iter()
                .find_map(|window| window.downcast::<SettingsWindow>())
                && cx.windows().len() == 1
            {
                cx.update_window(*existing_window, |_, window, _| {
                    window.remove_window();
                })
                .ok();

                telemetry::event!("Settings Closed")
            }
        })
        .detach();

        if let Some(app_state) = AppState::global(cx).upgrade() {
            for project in app_state
                .workspace_store
                .read(cx)
                .workspaces()
                .iter()
                .filter_map(|space| {
                    space
                        .read(cx)
                        .ok()
                        .map(|workspace| workspace.project().clone())
                })
                .collect::<Vec<_>>()
            {
                cx.observe_release_in(&project, window, |this, _, window, cx| {
                    this.fetch_files(window, cx)
                })
                .detach();
                cx.subscribe_in(&project, window, Self::handle_project_event)
                    .detach();
            }

            for workspace in app_state
                .workspace_store
                .read(cx)
                .workspaces()
                .iter()
                .filter_map(|space| space.entity(cx).ok())
            {
                cx.observe_release_in(&workspace, window, |this, _, window, cx| {
                    this.fetch_files(window, cx)
                })
                .detach();
            }
        } else {
            log::error!("App state doesn't exist when creating a new settings window");
        }

        let this_weak = cx.weak_entity();
        cx.observe_new::<Project>({
            let this_weak = this_weak.clone();

            move |_, window, cx| {
                let project = cx.entity();
                let Some(window) = window else {
                    return;
                };

                this_weak
                    .update(cx, |this, cx| {
                        this.fetch_files(window, cx);
                        cx.observe_release_in(&project, window, |_, _, window, cx| {
                            cx.defer_in(window, |this, window, cx| this.fetch_files(window, cx));
                        })
                        .detach();

                        cx.subscribe_in(&project, window, Self::handle_project_event)
                            .detach();
                    })
                    .ok();
            }
        })
        .detach();

        cx.observe_new::<Workspace>(move |_, window, cx| {
            let workspace = cx.entity();
            let Some(window) = window else {
                return;
            };

            this_weak
                .update(cx, |this, cx| {
                    this.fetch_files(window, cx);
                    cx.observe_release_in(&workspace, window, |this, _, window, cx| {
                        this.fetch_files(window, cx)
                    })
                    .detach();
                })
                .ok();
        })
        .detach();

        let title_bar = if !cfg!(target_os = "macos") {
            Some(cx.new(|cx| PlatformTitleBar::new("settings-title-bar", cx)))
        } else {
            None
        };

        // high overdraw value so the list scrollbar len doesn't change too much
        let list_state = gpui::ListState::new(0, gpui::ListAlignment::Top, px(0.0)).measure_all();
        list_state.set_scroll_handler(|_, _, _| {});

        let mut this = Self {
            title_bar,
            original_window,

            worktree_root_dirs: HashMap::default(),
            files: vec![],

            current_file: current_file,
            pages: vec![],
            navbar_entries: vec![],
            navbar_entry: 0,
            navbar_scroll_handle: UniformListScrollHandle::default(),
            search_bar,
            search_task: None,
            filter_table: vec![],
            has_query: false,
            content_handles: vec![],
            sub_page_scroll_handle: ScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            navbar_focus_handle: NonFocusableHandle::new(
                NAVBAR_CONTAINER_TAB_INDEX,
                false,
                window,
                cx,
            ),
            navbar_focus_subscriptions: vec![],
            content_focus_handle: NonFocusableHandle::new(
                CONTENT_CONTAINER_TAB_INDEX,
                false,
                window,
                cx,
            ),
            files_focus_handle: cx
                .focus_handle()
                .tab_index(HEADER_CONTAINER_TAB_INDEX)
                .tab_stop(false),
            search_index: None,
            shown_errors: HashSet::default(),
            list_state,
        };

        this.fetch_files(window, cx);
        this.build_ui(window, cx);
        this.build_search_index();

        this.search_bar.update(cx, |editor, cx| {
            editor.focus_handle(cx).focus(window, cx);
        });

        this
    }

    fn handle_project_event(
        &mut self,
        _: &Entity<Project>,
        event: &project::Event,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) {
        match event {
            project::Event::WorktreeRemoved(_) | project::Event::WorktreeAdded(_) => {
                cx.defer_in(window, |this, window, cx| {
                    this.fetch_files(window, cx);
                });
            }
            _ => {}
        }
    }

    fn toggle_navbar_entry(&mut self, nav_entry_index: usize) {
        // We can only toggle root entries
        if !self.navbar_entries[nav_entry_index].is_root {
            return;
        }

        let expanded = &mut self.navbar_entries[nav_entry_index].expanded;
        *expanded = !*expanded;
        self.navbar_entry = nav_entry_index;
        self.reset_list_state();
    }

    fn build_navbar(&mut self, cx: &App) {
        let mut navbar_entries = Vec::new();

        for (page_index, page) in self.pages.iter().enumerate() {
            navbar_entries.push(NavBarEntry {
                title: page.title,
                is_root: true,
                expanded: false,
                page_index,
                item_index: None,
                focus_handle: cx.focus_handle().tab_index(0).tab_stop(true),
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
                    focus_handle: cx.focus_handle().tab_index(0).tab_stop(true),
                });
            }
        }

        self.navbar_entries = navbar_entries;
    }

    fn setup_navbar_focus_subscriptions(
        &mut self,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) {
        let mut focus_subscriptions = Vec::new();

        for entry_index in 0..self.navbar_entries.len() {
            let focus_handle = self.navbar_entries[entry_index].focus_handle.clone();

            let subscription = cx.on_focus(
                &focus_handle,
                window,
                move |this: &mut SettingsWindow,
                      window: &mut Window,
                      cx: &mut Context<SettingsWindow>| {
                    this.open_and_scroll_to_navbar_entry(entry_index, None, false, window, cx);
                },
            );
            focus_subscriptions.push(subscription);
        }
        self.navbar_focus_subscriptions = focus_subscriptions;
    }

    fn visible_navbar_entries(&self) -> impl Iterator<Item = (usize, &NavBarEntry)> {
        let mut index = 0;
        let entries = &self.navbar_entries;
        let search_matches = &self.filter_table;
        let has_query = self.has_query;
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
            if entry.is_root && !entry.expanded && !has_query {
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
        for (page, page_filter) in std::iter::zip(&self.pages, &mut self.filter_table) {
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
                    SettingsPageItem::SettingItem(SettingItem { files, .. })
                    | SettingsPageItem::SubPageLink(SubPageLink { files, .. })
                    | SettingsPageItem::DynamicItem(DynamicItem {
                        discriminant: SettingItem { files, .. },
                        ..
                    }) => {
                        if !files.contains(current_file) {
                            page_filter[index] = false;
                        } else {
                            any_found_since_last_header = true;
                        }
                    }
                    SettingsPageItem::ActionLink(_) => {
                        any_found_since_last_header = true;
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
        let mut query = self.search_bar.read(cx).text(cx);
        if query.is_empty() || self.search_index.is_none() {
            for page in &mut self.filter_table {
                page.fill(true);
            }
            self.has_query = false;
            self.filter_matches_to_file();
            self.reset_list_state();
            cx.notify();
            return;
        }

        let is_json_link_query;
        if query.starts_with("#") {
            query.remove(0);
            is_json_link_query = true;
        } else {
            is_json_link_query = false;
        }

        let search_index = self.search_index.as_ref().unwrap().clone();

        fn update_matches_inner(
            this: &mut SettingsWindow,
            search_index: &SearchIndex,
            match_indices: impl Iterator<Item = usize>,
            cx: &mut Context<SettingsWindow>,
        ) {
            for page in &mut this.filter_table {
                page.fill(false);
            }

            for match_index in match_indices {
                let SearchKeyLUTEntry {
                    page_index,
                    header_index,
                    item_index,
                    ..
                } = search_index.key_lut[match_index];
                let page = &mut this.filter_table[page_index];
                page[header_index] = true;
                page[item_index] = true;
            }
            this.has_query = true;
            this.filter_matches_to_file();
            this.open_first_nav_page();
            this.reset_list_state();
            cx.notify();
        }

        self.search_task = Some(cx.spawn(async move |this, cx| {
            if is_json_link_query {
                let mut indices = vec![];
                for (index, SearchKeyLUTEntry { json_path, .. }) in
                    search_index.key_lut.iter().enumerate()
                {
                    let Some(json_path) = json_path else {
                        continue;
                    };

                    if let Some(post) = query.strip_prefix(json_path)
                        && (post.is_empty() || post.starts_with('.'))
                    {
                        indices.push(index);
                    }
                }
                if !indices.is_empty() {
                    this.update(cx, |this, cx| {
                        update_matches_inner(this, search_index.as_ref(), indices.into_iter(), cx);
                    })
                    .ok();
                    return;
                }
            }
            let bm25_task = cx.background_spawn({
                let search_index = search_index.clone();
                let max_results = search_index.key_lut.len();
                let query = query.clone();
                async move { search_index.bm25_engine.search(&query, max_results) }
            });
            let cancel_flag = std::sync::atomic::AtomicBool::new(false);
            let fuzzy_search_task = fuzzy::match_strings(
                search_index.fuzzy_match_candidates.as_slice(),
                &query,
                false,
                true,
                search_index.fuzzy_match_candidates.len(),
                &cancel_flag,
                cx.background_executor().clone(),
            );

            let fuzzy_matches = fuzzy_search_task.await;

            _ = this
                .update(cx, |this, cx| {
                    // For tuning the score threshold
                    // for fuzzy_match in &fuzzy_matches {
                    //     let SearchItemKey {
                    //         page_index,
                    //         header_index,
                    //         item_index,
                    //     } = search_index.key_lut[fuzzy_match.candidate_id];
                    //     let SettingsPageItem::SectionHeader(header) =
                    //         this.pages[page_index].items[header_index]
                    //     else {
                    //         continue;
                    //     };
                    //     let SettingsPageItem::SettingItem(SettingItem {
                    //         title, description, ..
                    //     }) = this.pages[page_index].items[item_index]
                    //     else {
                    //         continue;
                    //     };
                    //     let score = fuzzy_match.score;
                    //     eprint!("# {header} :: QUERY = {query} :: SCORE = {score}\n{title}\n{description}\n\n");
                    // }
                    update_matches_inner(
                        this,
                        search_index.as_ref(),
                        fuzzy_matches
                            .into_iter()
                            // MAGIC NUMBER: Was found to have right balance between not too many weird matches, but also
                            // flexible enough to catch misspellings and <4 letter queries
                            // More flexible is good for us here because fuzzy matches will only be used for things that don't
                            // match using bm25
                            .take_while(|fuzzy_match| fuzzy_match.score >= 0.3)
                            .map(|fuzzy_match| fuzzy_match.candidate_id),
                        cx,
                    );
                })
                .ok();

            let bm25_matches = bm25_task.await;

            _ = this
                .update(cx, |this, cx| {
                    if bm25_matches.is_empty() {
                        return;
                    }
                    update_matches_inner(
                        this,
                        search_index.as_ref(),
                        bm25_matches
                            .into_iter()
                            .map(|bm25_match| bm25_match.document.id),
                        cx,
                    );
                })
                .ok();

            cx.background_executor().timer(Duration::from_secs(1)).await;
            telemetry::event!("Settings Searched", query = query)
        }));
    }

    fn build_filter_table(&mut self) {
        self.filter_table = self
            .pages
            .iter()
            .map(|page| vec![true; page.items.len()])
            .collect::<Vec<_>>();
    }

    fn build_search_index(&mut self) {
        let mut key_lut: Vec<SearchKeyLUTEntry> = vec![];
        let mut documents = Vec::default();
        let mut fuzzy_match_candidates = Vec::default();

        fn push_candidates(
            fuzzy_match_candidates: &mut Vec<StringMatchCandidate>,
            key_index: usize,
            input: &str,
        ) {
            for word in input.split_ascii_whitespace() {
                fuzzy_match_candidates.push(StringMatchCandidate::new(key_index, word));
            }
        }

        // PERF: We are currently searching all items even in project files
        // where many settings are filtered out, using the logic in filter_matches_to_file
        // we could only search relevant items based on the current file
        for (page_index, page) in self.pages.iter().enumerate() {
            let mut header_index = 0;
            let mut header_str = "";
            for (item_index, item) in page.items.iter().enumerate() {
                let key_index = key_lut.len();
                let mut json_path = None;
                match item {
                    SettingsPageItem::DynamicItem(DynamicItem {
                        discriminant: item, ..
                    })
                    | SettingsPageItem::SettingItem(item) => {
                        json_path = item
                            .field
                            .json_path()
                            .map(|path| path.trim_end_matches('$'));
                        documents.push(bm25::Document {
                            id: key_index,
                            contents: [page.title, header_str, item.title, item.description]
                                .join("\n"),
                        });
                        push_candidates(&mut fuzzy_match_candidates, key_index, item.title);
                        push_candidates(&mut fuzzy_match_candidates, key_index, item.description);
                    }
                    SettingsPageItem::SectionHeader(header) => {
                        documents.push(bm25::Document {
                            id: key_index,
                            contents: header.to_string(),
                        });
                        push_candidates(&mut fuzzy_match_candidates, key_index, header);
                        header_index = item_index;
                        header_str = *header;
                    }
                    SettingsPageItem::SubPageLink(sub_page_link) => {
                        json_path = sub_page_link.json_path;
                        documents.push(bm25::Document {
                            id: key_index,
                            contents: [page.title, header_str, sub_page_link.title.as_ref()]
                                .join("\n"),
                        });
                        push_candidates(
                            &mut fuzzy_match_candidates,
                            key_index,
                            sub_page_link.title.as_ref(),
                        );
                    }
                    SettingsPageItem::ActionLink(action_link) => {
                        documents.push(bm25::Document {
                            id: key_index,
                            contents: [page.title, header_str, action_link.title.as_ref()]
                                .join("\n"),
                        });
                        push_candidates(
                            &mut fuzzy_match_candidates,
                            key_index,
                            action_link.title.as_ref(),
                        );
                    }
                }
                push_candidates(&mut fuzzy_match_candidates, key_index, page.title);
                push_candidates(&mut fuzzy_match_candidates, key_index, header_str);

                key_lut.push(SearchKeyLUTEntry {
                    page_index,
                    header_index,
                    item_index,
                    json_path,
                });
            }
        }
        let engine =
            bm25::SearchEngineBuilder::with_documents(bm25::Language::English, documents).build();
        self.search_index = Some(Arc::new(SearchIndex {
            bm25_engine: engine,
            key_lut,
            fuzzy_match_candidates,
        }));
    }

    fn build_content_handles(&mut self, window: &mut Window, cx: &mut Context<SettingsWindow>) {
        self.content_handles = self
            .pages
            .iter()
            .map(|page| {
                std::iter::repeat_with(|| NonFocusableHandle::new(0, false, window, cx))
                    .take(page.items.len())
                    .collect()
            })
            .collect::<Vec<_>>();
    }

    fn reset_list_state(&mut self) {
        // plus one for the title
        let mut visible_items_count = self.visible_page_items().count();

        if visible_items_count > 0 {
            // show page title if page is non empty
            visible_items_count += 1;
        }

        self.list_state.reset(visible_items_count);
    }

    fn build_ui(&mut self, window: &mut Window, cx: &mut Context<SettingsWindow>) {
        if self.pages.is_empty() {
            self.pages = page_data::settings_data(cx);
            self.build_navbar(cx);
            self.setup_navbar_focus_subscriptions(window, cx);
            self.build_content_handles(window, cx);
        }
        sub_page_stack_mut().clear();
        // PERF: doesn't have to be rebuilt, can just be filled with true. pages is constant once it is built
        self.build_filter_table();
        self.reset_list_state();
        self.update_matches(cx);

        cx.notify();
    }

    #[track_caller]
    fn fetch_files(&mut self, window: &mut Window, cx: &mut Context<SettingsWindow>) {
        self.worktree_root_dirs.clear();
        let prev_files = self.files.clone();
        let settings_store = cx.global::<SettingsStore>();
        let mut ui_files = vec![];
        let mut all_files = settings_store.get_all_files();
        if !all_files.contains(&settings::SettingsFile::User) {
            all_files.push(settings::SettingsFile::User);
        }
        for file in all_files {
            let Some(settings_ui_file) = SettingsUiFile::from_settings(file) else {
                continue;
            };
            if settings_ui_file.is_server() {
                continue;
            }

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
                .unwrap_or_else(|| cx.focus_handle().tab_index(0).tab_stop(true));
            ui_files.push((settings_ui_file, focus_handle));
        }

        ui_files.reverse();

        let mut missing_worktrees = Vec::new();

        for worktree in all_projects(cx)
            .flat_map(|project| project.read(cx).visible_worktrees(cx))
            .filter(|tree| !self.worktree_root_dirs.contains_key(&tree.read(cx).id()))
        {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            let Some(directory_name) = worktree.root_dir().and_then(|file| {
                file.file_name()
                    .map(|os_string| os_string.to_string_lossy().to_string())
            }) else {
                continue;
            };

            missing_worktrees.push((worktree_id, directory_name.clone()));
            let path = RelPath::empty().to_owned().into_arc();

            let settings_ui_file = SettingsUiFile::Project((worktree_id, path));

            let focus_handle = prev_files
                .iter()
                .find_map(|(prev_file, handle)| {
                    (prev_file == &settings_ui_file).then(|| handle.clone())
                })
                .unwrap_or_else(|| cx.focus_handle().tab_index(0).tab_stop(true));

            ui_files.push((settings_ui_file, focus_handle));
        }

        self.worktree_root_dirs.extend(missing_worktrees);

        self.files = ui_files;
        let current_file_still_exists = self
            .files
            .iter()
            .any(|(file, _)| file == &self.current_file);
        if !current_file_still_exists {
            self.change_file(0, window, cx);
        }
    }

    fn open_navbar_entry_page(&mut self, navbar_entry: usize) {
        if !self.is_nav_entry_visible(navbar_entry) {
            self.open_first_nav_page();
        }

        let is_new_page = self.navbar_entries[self.navbar_entry].page_index
            != self.navbar_entries[navbar_entry].page_index;
        self.navbar_entry = navbar_entry;

        // We only need to reset visible items when updating matches
        // and selecting a new page
        if is_new_page {
            self.reset_list_state();
        }

        sub_page_stack_mut().clear();
    }

    fn open_first_nav_page(&mut self) {
        let Some(first_navbar_entry_index) = self.visible_navbar_entries().next().map(|e| e.0)
        else {
            return;
        };
        self.open_navbar_entry_page(first_navbar_entry_index);
    }

    fn change_file(&mut self, ix: usize, window: &mut Window, cx: &mut Context<SettingsWindow>) {
        if ix >= self.files.len() {
            self.current_file = SettingsUiFile::User;
            self.build_ui(window, cx);
            return;
        }

        if self.files[ix].0 == self.current_file {
            return;
        }
        self.current_file = self.files[ix].0.clone();

        if let SettingsUiFile::Project((_, _)) = &self.current_file {
            telemetry::event!("Setting Project Clicked");
        }

        self.build_ui(window, cx);

        if self
            .visible_navbar_entries()
            .any(|(index, _)| index == self.navbar_entry)
        {
            self.open_and_scroll_to_navbar_entry(self.navbar_entry, None, true, window, cx);
        } else {
            self.open_first_nav_page();
        };
    }

    fn render_files_header(
        &self,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        static OVERFLOW_LIMIT: usize = 1;

        let file_button =
            |ix, file: &SettingsUiFile, focus_handle, cx: &mut Context<SettingsWindow>| {
                Button::new(
                    ix,
                    self.display_name(&file)
                        .expect("Files should always have a name"),
                )
                .toggle_state(file == &self.current_file)
                .selected_style(ButtonStyle::Tinted(ui::TintColor::Accent))
                .track_focus(focus_handle)
                .on_click(cx.listener({
                    let focus_handle = focus_handle.clone();
                    move |this, _: &gpui::ClickEvent, window, cx| {
                        this.change_file(ix, window, cx);
                        focus_handle.focus(window, cx);
                    }
                }))
            };

        let this = cx.entity();

        let selected_file_ix = self
            .files
            .iter()
            .enumerate()
            .skip(OVERFLOW_LIMIT)
            .find_map(|(ix, (file, _))| {
                if file == &self.current_file {
                    Some(ix)
                } else {
                    None
                }
            })
            .unwrap_or(OVERFLOW_LIMIT);
        let edit_in_json_id = SharedString::new(format!("edit-in-json-{}", selected_file_ix));

        h_flex()
            .w_full()
            .gap_1()
            .justify_between()
            .track_focus(&self.files_focus_handle)
            .tab_group()
            .tab_index(HEADER_GROUP_TAB_INDEX)
            .child(
                h_flex()
                    .gap_1()
                    .children(
                        self.files.iter().enumerate().take(OVERFLOW_LIMIT).map(
                            |(ix, (file, focus_handle))| file_button(ix, file, focus_handle, cx),
                        ),
                    )
                    .when(self.files.len() > OVERFLOW_LIMIT, |div| {
                        let (file, focus_handle) = &self.files[selected_file_ix];

                        div.child(file_button(selected_file_ix, file, focus_handle, cx))
                            .when(self.files.len() > OVERFLOW_LIMIT + 1, |div| {
                                div.child(
                                    DropdownMenu::new(
                                        "more-files",
                                        format!("+{}", self.files.len() - (OVERFLOW_LIMIT + 1)),
                                        ContextMenu::build(window, cx, move |mut menu, _, _| {
                                            for (mut ix, (file, focus_handle)) in self
                                                .files
                                                .iter()
                                                .enumerate()
                                                .skip(OVERFLOW_LIMIT + 1)
                                            {
                                                let (display_name, focus_handle) =
                                                    if selected_file_ix == ix {
                                                        ix = OVERFLOW_LIMIT;
                                                        (
                                                            self.display_name(&self.files[ix].0),
                                                            self.files[ix].1.clone(),
                                                        )
                                                    } else {
                                                        (
                                                            self.display_name(&file),
                                                            focus_handle.clone(),
                                                        )
                                                    };

                                                menu = menu.entry(
                                                    display_name
                                                        .expect("Files should always have a name"),
                                                    None,
                                                    {
                                                        let this = this.clone();
                                                        move |window, cx| {
                                                            this.update(cx, |this, cx| {
                                                                this.change_file(ix, window, cx);
                                                            });
                                                            focus_handle.focus(window, cx);
                                                        }
                                                    },
                                                );
                                            }

                                            menu
                                        }),
                                    )
                                    .style(DropdownStyle::Subtle)
                                    .trigger_tooltip(Tooltip::text("View Other Projects"))
                                    .trigger_icon(IconName::ChevronDown)
                                    .attach(gpui::Corner::BottomLeft)
                                    .offset(gpui::Point {
                                        x: px(0.0),
                                        y: px(2.0),
                                    })
                                    .tab_index(0),
                                )
                            })
                    }),
            )
            .child(
                Button::new(edit_in_json_id, "Edit in settings.json")
                    .tab_index(0_isize)
                    .style(ButtonStyle::OutlinedGhost)
                    .tooltip(Tooltip::for_action_title_in(
                        "Edit in settings.json",
                        &OpenCurrentFile,
                        &self.focus_handle,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open_current_settings_file(window, cx);
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
                            path_style.primary_separator(),
                            path.display(path_style)
                        )
                    }
                }),
            SettingsUiFile::Server(file) => Some(file.to_string()),
        }
    }

    // TODO:
    //  Reconsider this after preview launch
    // fn file_location_str(&self) -> String {
    //     match &self.current_file {
    //         SettingsUiFile::User => "settings.json".to_string(),
    //         SettingsUiFile::Project((worktree_id, path)) => self
    //             .worktree_root_dirs
    //             .get(&worktree_id)
    //             .map(|directory_name| {
    //                 let path_style = PathStyle::local();
    //                 let file_path = path.join(paths::local_settings_file_relative_path());
    //                 format!(
    //                     "{}{}{}",
    //                     directory_name,
    //                     path_style.separator(),
    //                     file_path.display(path_style)
    //                 )
    //             })
    //             .expect("Current file should always be present in root dir map"),
    //         SettingsUiFile::Server(file) => file.to_string(),
    //     }
    // }

    fn render_search(&self, _window: &mut Window, cx: &mut App) -> Div {
        h_flex()
            .py_1()
            .px_1p5()
            .mb_3()
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

        let focus_keybind_label = if self
            .navbar_focus_handle
            .read(cx)
            .handle
            .contains_focused(window, cx)
            || self
                .visible_navbar_entries()
                .any(|(_, entry)| entry.focus_handle.is_focused(window))
        {
            "Focus Content"
        } else {
            "Focus Navbar"
        };

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("NavigationMenu");
        key_context.add("menu");
        if self.search_bar.focus_handle(cx).is_focused(window) {
            key_context.add("search");
        }

        v_flex()
            .key_context(key_context)
            .on_action(cx.listener(|this, _: &CollapseNavEntry, window, cx| {
                let Some(focused_entry) = this.focused_nav_entry(window, cx) else {
                    return;
                };
                let focused_entry_parent = this.root_entry_containing(focused_entry);
                if this.navbar_entries[focused_entry_parent].expanded {
                    this.toggle_navbar_entry(focused_entry_parent);
                    window.focus(&this.navbar_entries[focused_entry_parent].focus_handle, cx);
                }
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ExpandNavEntry, window, cx| {
                let Some(focused_entry) = this.focused_nav_entry(window, cx) else {
                    return;
                };
                if !this.navbar_entries[focused_entry].is_root {
                    return;
                }
                if !this.navbar_entries[focused_entry].expanded {
                    this.toggle_navbar_entry(focused_entry);
                }
                cx.notify();
            }))
            .on_action(
                cx.listener(|this, _: &FocusPreviousRootNavEntry, window, cx| {
                    let entry_index = this
                        .focused_nav_entry(window, cx)
                        .unwrap_or(this.navbar_entry);
                    let mut root_index = None;
                    for (index, entry) in this.visible_navbar_entries() {
                        if index >= entry_index {
                            break;
                        }
                        if entry.is_root {
                            root_index = Some(index);
                        }
                    }
                    let Some(previous_root_index) = root_index else {
                        return;
                    };
                    this.focus_and_scroll_to_nav_entry(previous_root_index, window, cx);
                }),
            )
            .on_action(cx.listener(|this, _: &FocusNextRootNavEntry, window, cx| {
                let entry_index = this
                    .focused_nav_entry(window, cx)
                    .unwrap_or(this.navbar_entry);
                let mut root_index = None;
                for (index, entry) in this.visible_navbar_entries() {
                    if index <= entry_index {
                        continue;
                    }
                    if entry.is_root {
                        root_index = Some(index);
                        break;
                    }
                }
                let Some(next_root_index) = root_index else {
                    return;
                };
                this.focus_and_scroll_to_nav_entry(next_root_index, window, cx);
            }))
            .on_action(cx.listener(|this, _: &FocusFirstNavEntry, window, cx| {
                if let Some((first_entry_index, _)) = this.visible_navbar_entries().next() {
                    this.focus_and_scroll_to_nav_entry(first_entry_index, window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &FocusLastNavEntry, window, cx| {
                if let Some((last_entry_index, _)) = this.visible_navbar_entries().last() {
                    this.focus_and_scroll_to_nav_entry(last_entry_index, window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &FocusNextNavEntry, window, cx| {
                let entry_index = this
                    .focused_nav_entry(window, cx)
                    .unwrap_or(this.navbar_entry);
                let mut next_index = None;
                for (index, _) in this.visible_navbar_entries() {
                    if index > entry_index {
                        next_index = Some(index);
                        break;
                    }
                }
                let Some(next_entry_index) = next_index else {
                    return;
                };
                this.open_and_scroll_to_navbar_entry(
                    next_entry_index,
                    Some(gpui::ScrollStrategy::Bottom),
                    false,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &FocusPreviousNavEntry, window, cx| {
                let entry_index = this
                    .focused_nav_entry(window, cx)
                    .unwrap_or(this.navbar_entry);
                let mut prev_index = None;
                for (index, _) in this.visible_navbar_entries() {
                    if index >= entry_index {
                        break;
                    }
                    prev_index = Some(index);
                }
                let Some(prev_entry_index) = prev_index else {
                    return;
                };
                this.open_and_scroll_to_navbar_entry(
                    prev_entry_index,
                    Some(gpui::ScrollStrategy::Top),
                    false,
                    window,
                    cx,
                );
            }))
            .w_56()
            .h_full()
            .p_2p5()
            .when(cfg!(target_os = "macos"), |this| this.pt_10())
            .flex_none()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().panel_background)
            .child(self.render_search(window, cx))
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .track_focus(&self.navbar_focus_handle.focus_handle(cx))
                    .tab_group()
                    .tab_index(NAVBAR_GROUP_TAB_INDEX)
                    .child(
                        uniform_list(
                            "settings-ui-nav-bar",
                            visible_count + 1,
                            cx.processor(move |this, range: Range<usize>, _, cx| {
                                this.visible_navbar_entries()
                                    .skip(range.start.saturating_sub(1))
                                    .take(range.len())
                                    .map(|(entry_index, entry)| {
                                        TreeViewItem::new(
                                            ("settings-ui-navbar-entry", entry_index),
                                            entry.title,
                                        )
                                        .track_focus(&entry.focus_handle)
                                        .root_item(entry.is_root)
                                        .toggle_state(this.is_navbar_entry_selected(entry_index))
                                        .when(entry.is_root, |item| {
                                            item.expanded(entry.expanded || this.has_query)
                                                .on_toggle(cx.listener(
                                                    move |this, _, window, cx| {
                                                        this.toggle_navbar_entry(entry_index);
                                                        window.focus(
                                                            &this.navbar_entries[entry_index]
                                                                .focus_handle,
                                                            cx,
                                                        );
                                                        cx.notify();
                                                    },
                                                ))
                                        })
                                        .on_click({
                                            let category = this.pages[entry.page_index].title;
                                            let subcategory =
                                                (!entry.is_root).then_some(entry.title);

                                            cx.listener(move |this, _, window, cx| {
                                                telemetry::event!(
                                                    "Settings Navigation Clicked",
                                                    category = category,
                                                    subcategory = subcategory
                                                );

                                                this.open_and_scroll_to_navbar_entry(
                                                    entry_index,
                                                    None,
                                                    true,
                                                    window,
                                                    cx,
                                                );
                                            })
                                        })
                                    })
                                    .collect()
                            }),
                        )
                        .size_full()
                        .track_scroll(&self.navbar_scroll_handle),
                    )
                    .vertical_scrollbar_for(&self.navbar_scroll_handle, window, cx),
            )
            .child(
                h_flex()
                    .w_full()
                    .h_8()
                    .p_2()
                    .pb_0p5()
                    .flex_shrink_0()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        KeybindingHint::new(
                            KeyBinding::for_action_in(
                                &ToggleFocusNav,
                                &self.navbar_focus_handle.focus_handle(cx),
                                cx,
                            ),
                            cx.theme().colors().surface_background.opacity(0.5),
                        )
                        .suffix(focus_keybind_label),
                    ),
            )
    }

    fn open_and_scroll_to_navbar_entry(
        &mut self,
        navbar_entry_index: usize,
        scroll_strategy: Option<gpui::ScrollStrategy>,
        focus_content: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_navbar_entry_page(navbar_entry_index);
        cx.notify();

        let mut handle_to_focus = None;

        if self.navbar_entries[navbar_entry_index].is_root
            || !self.is_nav_entry_visible(navbar_entry_index)
        {
            self.sub_page_scroll_handle
                .set_offset(point(px(0.), px(0.)));
            if focus_content {
                let Some(first_item_index) =
                    self.visible_page_items().next().map(|(index, _)| index)
                else {
                    return;
                };
                handle_to_focus = Some(self.focus_handle_for_content_element(first_item_index, cx));
            } else if !self.is_nav_entry_visible(navbar_entry_index) {
                let Some(first_visible_nav_entry_index) =
                    self.visible_navbar_entries().next().map(|(index, _)| index)
                else {
                    return;
                };
                self.focus_and_scroll_to_nav_entry(first_visible_nav_entry_index, window, cx);
            } else {
                handle_to_focus =
                    Some(self.navbar_entries[navbar_entry_index].focus_handle.clone());
            }
        } else {
            let entry_item_index = self.navbar_entries[navbar_entry_index]
                .item_index
                .expect("Non-root items should have an item index");
            self.scroll_to_content_item(entry_item_index, window, cx);
            if focus_content {
                handle_to_focus = Some(self.focus_handle_for_content_element(entry_item_index, cx));
            } else {
                handle_to_focus =
                    Some(self.navbar_entries[navbar_entry_index].focus_handle.clone());
            }
        }

        if let Some(scroll_strategy) = scroll_strategy
            && let Some(logical_entry_index) = self
                .visible_navbar_entries()
                .into_iter()
                .position(|(index, _)| index == navbar_entry_index)
        {
            self.navbar_scroll_handle
                .scroll_to_item(logical_entry_index + 1, scroll_strategy);
        }

        // Page scroll handle updates the active item index
        // in it's next paint call after using scroll_handle.scroll_to_top_of_item
        // The call after that updates the offset of the scroll handle. So to
        // ensure the scroll handle doesn't lag behind we need to render three frames
        // back to back.
        cx.on_next_frame(window, move |_, window, cx| {
            if let Some(handle) = handle_to_focus.as_ref() {
                window.focus(handle, cx);
            }

            cx.on_next_frame(window, |_, _, cx| {
                cx.notify();
            });
            cx.notify();
        });
        cx.notify();
    }

    fn scroll_to_content_item(
        &self,
        content_item_index: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let index = self
            .visible_page_items()
            .position(|(index, _)| index == content_item_index)
            .unwrap_or(0);
        if index == 0 {
            self.sub_page_scroll_handle
                .set_offset(point(px(0.), px(0.)));
            self.list_state.scroll_to(gpui::ListOffset {
                item_ix: 0,
                offset_in_item: px(0.),
            });
            return;
        }
        self.list_state.scroll_to(gpui::ListOffset {
            item_ix: index + 1,
            offset_in_item: px(0.),
        });
        cx.notify();
    }

    fn is_nav_entry_visible(&self, nav_entry_index: usize) -> bool {
        self.visible_navbar_entries()
            .any(|(index, _)| index == nav_entry_index)
    }

    fn focus_and_scroll_to_first_visible_nav_entry(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(nav_entry_index) = self.visible_navbar_entries().next().map(|(index, _)| index)
        {
            self.focus_and_scroll_to_nav_entry(nav_entry_index, window, cx);
        }
    }

    fn focus_and_scroll_to_nav_entry(
        &self,
        nav_entry_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(position) = self
            .visible_navbar_entries()
            .position(|(index, _)| index == nav_entry_index)
        else {
            return;
        };
        self.navbar_scroll_handle
            .scroll_to_item(position, gpui::ScrollStrategy::Top);
        window.focus(&self.navbar_entries[nav_entry_index].focus_handle, cx);
        cx.notify();
    }

    fn visible_page_items(&self) -> impl Iterator<Item = (usize, &SettingsPageItem)> {
        let page_idx = self.current_page_index();

        self.current_page()
            .items
            .iter()
            .enumerate()
            .filter_map(move |(item_index, item)| {
                self.filter_table[page_idx][item_index].then_some((item_index, item))
            })
    }

    fn render_sub_page_breadcrumbs(&self) -> impl IntoElement {
        let mut items = vec![];
        items.push(self.current_page().title.into());
        items.extend(
            sub_page_stack()
                .iter()
                .flat_map(|page| [page.section_header.into(), page.link.title.clone()]),
        );

        let last = items.pop().unwrap();
        h_flex()
            .gap_1()
            .children(
                items
                    .into_iter()
                    .flat_map(|item| [item, "/".into()])
                    .map(|item| Label::new(item).color(Color::Muted)),
            )
            .child(Label::new(last))
    }

    fn render_empty_state(&self, search_query: SharedString) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_1()
            .child(Label::new("No Results"))
            .child(
                Label::new(search_query)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
    }

    fn render_page_items(
        &mut self,
        page_index: usize,
        _window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        let mut page_content = v_flex().id("settings-ui-page").size_full();

        let has_active_search = !self.search_bar.read(cx).is_empty(cx);
        let has_no_results = self.visible_page_items().next().is_none() && has_active_search;

        if has_no_results {
            let search_query = self.search_bar.read(cx).text(cx);
            page_content = page_content.child(
                self.render_empty_state(format!("No settings match \"{}\"", search_query).into()),
            )
        } else {
            let last_non_header_index = self
                .visible_page_items()
                .filter_map(|(index, item)| {
                    (!matches!(item, SettingsPageItem::SectionHeader(_))).then_some(index)
                })
                .last();

            let root_nav_label = self
                .navbar_entries
                .iter()
                .find(|entry| entry.is_root && entry.page_index == self.current_page_index())
                .map(|entry| entry.title);

            let list_content = list(
                self.list_state.clone(),
                cx.processor(move |this, index, window, cx| {
                    if index == 0 {
                        return div()
                            .px_8()
                            .when(sub_page_stack().is_empty(), |this| {
                                this.when_some(root_nav_label, |this, title| {
                                    this.child(
                                        Label::new(title).size(LabelSize::Large).mt_2().mb_3(),
                                    )
                                })
                            })
                            .into_any_element();
                    }

                    let mut visible_items = this.visible_page_items();
                    let Some((actual_item_index, item)) = visible_items.nth(index - 1) else {
                        return gpui::Empty.into_any_element();
                    };

                    let no_bottom_border = visible_items
                        .next()
                        .map(|(_, item)| matches!(item, SettingsPageItem::SectionHeader(_)))
                        .unwrap_or(false);

                    let is_last = Some(actual_item_index) == last_non_header_index;

                    let item_focus_handle =
                        this.content_handles[page_index][actual_item_index].focus_handle(cx);

                    v_flex()
                        .id(("settings-page-item", actual_item_index))
                        .track_focus(&item_focus_handle)
                        .w_full()
                        .min_w_0()
                        .child(item.render(
                            this,
                            actual_item_index,
                            no_bottom_border || is_last,
                            window,
                            cx,
                        ))
                        .into_any_element()
                }),
            );

            page_content = page_content.child(list_content.size_full())
        }
        page_content
    }

    fn render_sub_page_items<'a, Items>(
        &self,
        items: Items,
        page_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement
    where
        Items: Iterator<Item = (usize, &'a SettingsPageItem)>,
    {
        let page_content = v_flex()
            .id("settings-ui-page")
            .size_full()
            .overflow_y_scroll()
            .track_scroll(&self.sub_page_scroll_handle);
        self.render_sub_page_items_in(page_content, items, page_index, window, cx)
    }

    fn render_sub_page_items_section<'a, Items>(
        &self,
        items: Items,
        page_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement
    where
        Items: Iterator<Item = (usize, &'a SettingsPageItem)>,
    {
        let page_content = v_flex().id("settings-ui-sub-page-section").size_full();
        self.render_sub_page_items_in(page_content, items, page_index, window, cx)
    }

    fn render_sub_page_items_in<'a, Items>(
        &self,
        mut page_content: Stateful<Div>,
        items: Items,
        page_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement
    where
        Items: Iterator<Item = (usize, &'a SettingsPageItem)>,
    {
        let items: Vec<_> = items.collect();
        let items_len = items.len();
        let mut section_header = None;

        let has_active_search = !self.search_bar.read(cx).is_empty(cx);
        let has_no_results = items_len == 0 && has_active_search;

        if has_no_results {
            let search_query = self.search_bar.read(cx).text(cx);
            page_content = page_content.child(
                self.render_empty_state(format!("No settings match \"{}\"", search_query).into()),
            )
        } else {
            let last_non_header_index = items
                .iter()
                .enumerate()
                .rev()
                .find(|(_, (_, item))| !matches!(item, SettingsPageItem::SectionHeader(_)))
                .map(|(index, _)| index);

            let root_nav_label = self
                .navbar_entries
                .iter()
                .find(|entry| entry.is_root && entry.page_index == self.current_page_index())
                .map(|entry| entry.title);

            page_content = page_content
                .when(sub_page_stack().is_empty(), |this| {
                    this.when_some(root_nav_label, |this, title| {
                        this.child(Label::new(title).size(LabelSize::Large).mt_2().mb_3())
                    })
                })
                .children(items.clone().into_iter().enumerate().map(
                    |(index, (actual_item_index, item))| {
                        let no_bottom_border = items
                            .get(index + 1)
                            .map(|(_, next_item)| {
                                matches!(next_item, SettingsPageItem::SectionHeader(_))
                            })
                            .unwrap_or(false);
                        let is_last = Some(index) == last_non_header_index;

                        if let SettingsPageItem::SectionHeader(header) = item {
                            section_header = Some(*header);
                        }
                        v_flex()
                            .w_full()
                            .min_w_0()
                            .id(("settings-page-item", actual_item_index))
                            .when_some(page_index, |element, page_index| {
                                element.track_focus(
                                    &self.content_handles[page_index][actual_item_index]
                                        .focus_handle(cx),
                                )
                            })
                            .child(item.render(
                                self,
                                actual_item_index,
                                no_bottom_border || is_last,
                                window,
                                cx,
                            ))
                    },
                ))
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

        if sub_page_stack().is_empty() {
            page_header = self.render_files_header(window, cx).into_any_element();

            page_content = self
                .render_page_items(self.current_page_index(), window, cx)
                .into_any_element();
        } else {
            page_header = h_flex()
                .w_full()
                .justify_between()
                .child(
                    h_flex()
                        .ml_neg_1p5()
                        .gap_1()
                        .child(
                            IconButton::new("back-btn", IconName::ArrowLeft)
                                .icon_size(IconSize::Small)
                                .shape(IconButtonShape::Square)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.pop_sub_page(window, cx);
                                })),
                        )
                        .child(self.render_sub_page_breadcrumbs()),
                )
                .when(
                    sub_page_stack()
                        .last()
                        .is_none_or(|sub_page| sub_page.link.in_json),
                    |this| {
                        this.child(
                            Button::new("open-in-settings-file", "Edit in settings.json")
                                .tab_index(0_isize)
                                .style(ButtonStyle::OutlinedGhost)
                                .tooltip(Tooltip::for_action_title_in(
                                    "Edit in settings.json",
                                    &OpenCurrentFile,
                                    &self.focus_handle,
                                ))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.open_current_settings_file(window, cx);
                                })),
                        )
                    },
                )
                .into_any_element();

            let active_page_render_fn = sub_page_stack().last().unwrap().link.render.clone();
            page_content = (active_page_render_fn)(self, window, cx);
        }

        let mut warning_banner = gpui::Empty.into_any_element();
        if let Some(error) =
            SettingsStore::global(cx).error_for_file(self.current_file.to_settings())
        {
            fn banner(
                label: &'static str,
                error: String,
                shown_errors: &mut HashSet<String>,
                cx: &mut Context<SettingsWindow>,
            ) -> impl IntoElement {
                if shown_errors.insert(error.clone()) {
                    telemetry::event!("Settings Error Shown", label = label, error = &error);
                }
                Banner::new()
                    .severity(Severity::Warning)
                    .child(
                        v_flex()
                            .my_0p5()
                            .gap_0p5()
                            .child(Label::new(label))
                            .child(Label::new(error).size(LabelSize::Small).color(Color::Muted)),
                    )
                    .action_slot(
                        div().pr_1().pb_1().child(
                            Button::new("fix-in-json", "Fix in settings.json")
                                .tab_index(0_isize)
                                .style(ButtonStyle::Tinted(ui::TintColor::Warning))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.open_current_settings_file(window, cx);
                                })),
                        ),
                    )
            }

            let parse_error = error.parse_error();
            let parse_failed = parse_error.is_some();

            warning_banner = v_flex()
                .gap_2()
                .when_some(parse_error, |this, err| {
                    this.child(banner(
                        "Failed to load your settings. Some values may be incorrect and changes may be lost.",
                        err,
                        &mut self.shown_errors,
                        cx,
                    ))
                })
                .map(|this| match &error.migration_status {
                    settings::MigrationStatus::Succeeded => this.child(banner(
                        "Your settings are out of date, and need to be updated.",
                        match &self.current_file {
                            SettingsUiFile::User => "They can be automatically migrated to the latest version.",
                            SettingsUiFile::Server(_) | SettingsUiFile::Project(_)  => "They must be manually migrated to the latest version."
                        }.to_string(),
                        &mut self.shown_errors,
                        cx,
                    )),
                    settings::MigrationStatus::Failed { error: err } if !parse_failed => this
                        .child(banner(
                            "Your settings file is out of date, automatic migration failed",
                            err.clone(),
                            &mut self.shown_errors,
                            cx,
                        )),
                    _ => this,
                })
                .into_any_element()
        }

        return v_flex()
            .id("settings-ui-page")
            .on_action(cx.listener(|this, _: &menu::SelectNext, window, cx| {
                if !sub_page_stack().is_empty() {
                    window.focus_next(cx);
                    return;
                }
                for (logical_index, (actual_index, _)) in this.visible_page_items().enumerate() {
                    let handle = this.content_handles[this.current_page_index()][actual_index]
                        .focus_handle(cx);
                    let mut offset = 1; // for page header

                    if let Some((_, next_item)) = this.visible_page_items().nth(logical_index + 1)
                        && matches!(next_item, SettingsPageItem::SectionHeader(_))
                    {
                        offset += 1;
                    }
                    if handle.contains_focused(window, cx) {
                        let next_logical_index = logical_index + offset + 1;
                        this.list_state.scroll_to_reveal_item(next_logical_index);
                        // We need to render the next item to ensure it's focus handle is in the element tree
                        cx.on_next_frame(window, |_, window, cx| {
                            cx.notify();
                            cx.on_next_frame(window, |_, window, cx| {
                                window.focus_next(cx);
                                cx.notify();
                            });
                        });
                        cx.notify();
                        return;
                    }
                }
                window.focus_next(cx);
            }))
            .on_action(cx.listener(|this, _: &menu::SelectPrevious, window, cx| {
                if !sub_page_stack().is_empty() {
                    window.focus_prev(cx);
                    return;
                }
                let mut prev_was_header = false;
                for (logical_index, (actual_index, item)) in this.visible_page_items().enumerate() {
                    let is_header = matches!(item, SettingsPageItem::SectionHeader(_));
                    let handle = this.content_handles[this.current_page_index()][actual_index]
                        .focus_handle(cx);
                    let mut offset = 1; // for page header

                    if prev_was_header {
                        offset -= 1;
                    }
                    if handle.contains_focused(window, cx) {
                        let next_logical_index = logical_index + offset - 1;
                        this.list_state.scroll_to_reveal_item(next_logical_index);
                        // We need to render the next item to ensure it's focus handle is in the element tree
                        cx.on_next_frame(window, |_, window, cx| {
                            cx.notify();
                            cx.on_next_frame(window, |_, window, cx| {
                                window.focus_prev(cx);
                                cx.notify();
                            });
                        });
                        cx.notify();
                        return;
                    }
                    prev_was_header = is_header;
                }
                window.focus_prev(cx);
            }))
            .when(sub_page_stack().is_empty(), |this| {
                this.vertical_scrollbar_for(&self.list_state, window, cx)
            })
            .when(!sub_page_stack().is_empty(), |this| {
                this.vertical_scrollbar_for(&self.sub_page_scroll_handle, window, cx)
            })
            .track_focus(&self.content_focus_handle.focus_handle(cx))
            .pt_6()
            .gap_4()
            .flex_1()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .px_8()
                    .gap_2()
                    .child(page_header)
                    .child(warning_banner),
            )
            .child(
                div()
                    .flex_1()
                    .size_full()
                    .tab_group()
                    .tab_index(CONTENT_GROUP_TAB_INDEX)
                    .child(page_content),
            );
    }

    /// This function will create a new settings file if one doesn't exist
    /// if the current file is a project settings with a valid worktree id
    /// We do this because the settings ui allows initializing project settings
    fn open_current_settings_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &self.current_file {
            SettingsUiFile::User => {
                let Some(original_window) = self.original_window else {
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

                window.remove_window();
            }
            SettingsUiFile::Project((worktree_id, path)) => {
                let settings_path = path.join(paths::local_settings_file_relative_path());
                let Some(app_state) = workspace::AppState::global(cx).upgrade() else {
                    return;
                };

                let Some((worktree, corresponding_workspace)) = app_state
                    .workspace_store
                    .read(cx)
                    .workspaces()
                    .iter()
                    .find_map(|workspace| {
                        workspace
                            .read_with(cx, |workspace, cx| {
                                workspace
                                    .project()
                                    .read(cx)
                                    .worktree_for_id(*worktree_id, cx)
                            })
                            .ok()
                            .flatten()
                            .zip(Some(*workspace))
                    })
                else {
                    log::error!(
                        "No corresponding workspace contains worktree id: {}",
                        worktree_id
                    );

                    return;
                };

                let create_task = if worktree.read(cx).entry_for_path(&settings_path).is_some() {
                    None
                } else {
                    Some(worktree.update(cx, |tree, cx| {
                        tree.create_entry(
                            settings_path.clone(),
                            false,
                            Some(initial_project_settings_content().as_bytes().to_vec()),
                            cx,
                        )
                    }))
                };

                let worktree_id = *worktree_id;

                // TODO: move zed::open_local_file() APIs to this crate, and
                // re-implement the "initial_contents" behavior
                corresponding_workspace
                    .update(cx, |_, window, cx| {
                        cx.spawn_in(window, async move |workspace, cx| {
                            if let Some(create_task) = create_task {
                                create_task.await.ok()?;
                            };

                            workspace
                                .update_in(cx, |workspace, window, cx| {
                                    workspace.open_path(
                                        (worktree_id, settings_path.clone()),
                                        None,
                                        true,
                                        window,
                                        cx,
                                    )
                                })
                                .ok()?
                                .await
                                .log_err()?;

                            workspace
                                .update_in(cx, |_, window, cx| {
                                    window.activate_window();
                                    cx.notify();
                                })
                                .ok();

                            Some(())
                        })
                        .detach();
                    })
                    .ok();

                window.remove_window();
            }
            SettingsUiFile::Server(_) => {
                // Server files are not editable
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
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) {
        sub_page_stack_mut().push(SubPage {
            link: sub_page_link,
            section_header,
        });
        self.sub_page_scroll_handle
            .set_offset(point(px(0.), px(0.)));
        self.content_focus_handle.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn pop_sub_page(&mut self, window: &mut Window, cx: &mut Context<SettingsWindow>) {
        sub_page_stack_mut().pop();
        self.content_focus_handle.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn focus_file_at_index(&mut self, index: usize, window: &mut Window, cx: &mut App) {
        if let Some((_, handle)) = self.files.get(index) {
            handle.focus(window, cx);
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

    fn focus_handle_for_content_element(
        &self,
        actual_item_index: usize,
        cx: &Context<Self>,
    ) -> FocusHandle {
        let page_index = self.current_page_index();
        self.content_handles[page_index][actual_item_index].focus_handle(cx)
    }

    fn focused_nav_entry(&self, window: &Window, cx: &App) -> Option<usize> {
        if !self
            .navbar_focus_handle
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            return None;
        }
        for (index, entry) in self.navbar_entries.iter().enumerate() {
            if entry.focus_handle.is_focused(window) {
                return Some(index);
            }
        }
        None
    }

    fn root_entry_containing(&self, nav_entry_index: usize) -> usize {
        let mut index = Some(nav_entry_index);
        while let Some(prev_index) = index
            && !self.navbar_entries[prev_index].is_root
        {
            index = prev_index.checked_sub(1);
        }
        return index.expect("No root entry found");
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);

        client_side_decorations(
            v_flex()
                .text_color(cx.theme().colors().text)
                .size_full()
                .children(self.title_bar.clone())
                .child(
                    div()
                        .id("settings-window")
                        .key_context("SettingsWindow")
                        .track_focus(&self.focus_handle)
                        .on_action(cx.listener(|this, _: &OpenCurrentFile, window, cx| {
                            this.open_current_settings_file(window, cx);
                        }))
                        .on_action(|_: &Minimize, window, _cx| {
                            window.minimize_window();
                        })
                        .on_action(cx.listener(|this, _: &search::FocusSearch, window, cx| {
                            this.search_bar.focus_handle(cx).focus(window, cx);
                        }))
                        .on_action(cx.listener(|this, _: &ToggleFocusNav, window, cx| {
                            if this
                                .navbar_focus_handle
                                .focus_handle(cx)
                                .contains_focused(window, cx)
                            {
                                this.open_and_scroll_to_navbar_entry(
                                    this.navbar_entry,
                                    None,
                                    true,
                                    window,
                                    cx,
                                );
                            } else {
                                this.focus_and_scroll_to_nav_entry(this.navbar_entry, window, cx);
                            }
                        }))
                        .on_action(cx.listener(
                            |this, FocusFile(file_index): &FocusFile, window, cx| {
                                this.focus_file_at_index(*file_index as usize, window, cx);
                            },
                        ))
                        .on_action(cx.listener(|this, _: &FocusNextFile, window, cx| {
                            let next_index = usize::min(
                                this.focused_file_index(window, cx) + 1,
                                this.files.len().saturating_sub(1),
                            );
                            this.focus_file_at_index(next_index, window, cx);
                        }))
                        .on_action(cx.listener(|this, _: &FocusPreviousFile, window, cx| {
                            let prev_index = this.focused_file_index(window, cx).saturating_sub(1);
                            this.focus_file_at_index(prev_index, window, cx);
                        }))
                        .on_action(cx.listener(|this, _: &menu::SelectNext, window, cx| {
                            if this
                                .search_bar
                                .focus_handle(cx)
                                .contains_focused(window, cx)
                            {
                                this.focus_and_scroll_to_first_visible_nav_entry(window, cx);
                            } else {
                                window.focus_next(cx);
                            }
                        }))
                        .on_action(|_: &menu::SelectPrevious, window, cx| {
                            window.focus_prev(cx);
                        })
                        .flex()
                        .flex_row()
                        .flex_1()
                        .min_h_0()
                        .font(ui_font)
                        .bg(cx.theme().colors().background)
                        .text_color(cx.theme().colors().text)
                        .when(!cfg!(target_os = "macos"), |this| {
                            this.border_t_1().border_color(cx.theme().colors().border)
                        })
                        .child(self.render_nav(window, cx))
                        .child(self.render_page(window, cx)),
                ),
            window,
            cx,
        )
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
    file_name: Option<&'static str>,
    cx: &mut App,
    update: impl 'static + Send + FnOnce(&mut SettingsContent, &App),
) -> Result<()> {
    telemetry::event!("Settings Change", setting = file_name, type = file.setting_type());

    match file {
        SettingsUiFile::Project((worktree_id, rel_path)) => {
            let rel_path = rel_path.join(paths::local_settings_file_relative_path());
            let Some((worktree, project)) = all_projects(cx).find_map(|project| {
                project
                    .read(cx)
                    .worktree_for_id(worktree_id, cx)
                    .zip(Some(project))
            }) else {
                anyhow::bail!("Could not find project with worktree id: {}", worktree_id);
            };

            project.update(cx, |project, cx| {
                let task = if project.contains_local_settings_file(worktree_id, &rel_path, cx) {
                    None
                } else {
                    Some(worktree.update(cx, |worktree, cx| {
                        worktree.create_entry(rel_path.clone(), false, None, cx)
                    }))
                };

                cx.spawn(async move |project, cx| {
                    if let Some(task) = task
                        && task.await.is_err()
                    {
                        return;
                    };

                    project
                        .update(cx, |project, cx| {
                            project.update_local_settings_file(worktree_id, rel_path, cx, update);
                        })
                        .ok();
                })
                .detach();
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
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, initial_text) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let initial_text = initial_text.filter(|s| !s.as_ref().is_empty());

    SettingsInputField::new()
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
                update_settings_file(file.clone(), field.json_path, cx, move |settings, _cx| {
                    (field.write)(settings, new_text.map(Into::into));
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .into_any_element()
}

fn render_toggle_button<B: Into<bool> + From<bool> + Copy>(
    field: SettingField<B>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);

    let toggle_state = if value.copied().map_or(false, Into::into) {
        ToggleState::Selected
    } else {
        ToggleState::Unselected
    };

    Switch::new("toggle_button", toggle_state)
        .tab_index(0_isize)
        .on_click({
            move |state, _window, cx| {
                telemetry::event!("Settings Change", setting = field.json_path, type = file.setting_type());

                let state = *state == ui::ToggleState::Selected;
                update_settings_file(file.clone(), field.json_path, cx, move |settings, _cx| {
                    (field.write)(settings, Some(state.into()));
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .into_any_element()
}

fn render_number_field<T: NumberFieldType + Send + Sync>(
    field: SettingField<T>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let value = value.copied().unwrap_or_else(T::min_value);

    let id = field
        .json_path
        .map(|p| format!("numeric_stepper_{}", p))
        .unwrap_or_else(|| "numeric_stepper".to_string());

    NumberField::new(id, value, window, cx)
        .tab_index(0_isize)
        .on_change({
            move |value, _window, cx| {
                let value = *value;
                update_settings_file(file.clone(), field.json_path, cx, move |settings, _cx| {
                    (field.write)(settings, Some(value));
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .into_any_element()
}

fn render_editable_number_field<T: NumberFieldType + Send + Sync>(
    field: SettingField<T>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let value = value.copied().unwrap_or_else(T::min_value);

    let id = field
        .json_path
        .map(|p| format!("numeric_stepper_{}", p))
        .unwrap_or_else(|| "numeric_stepper".to_string());

    NumberField::new(id, value, window, cx)
        .mode(NumberFieldMode::Edit, cx)
        .tab_index(0_isize)
        .on_change({
            move |value, _window, cx| {
                let value = *value;
                update_settings_file(file.clone(), field.json_path, cx, move |settings, _cx| {
                    (field.write)(settings, Some(value));
                })
                .log_err(); // todo(settings_ui) don't log err
            }
        })
        .into_any_element()
}

fn render_dropdown<T>(
    field: SettingField<T>,
    file: SettingsUiFile,
    metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    let variants = || -> &'static [T] { <T as strum::VariantArray>::VARIANTS };
    let labels = || -> &'static [&'static str] { <T as strum::VariantNames>::VARIANTS };
    let should_do_titlecase = metadata
        .and_then(|metadata| metadata.should_do_titlecase)
        .unwrap_or(true);

    let (_, current_value) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value = current_value.copied().unwrap_or(variants()[0]);

    EnumVariantDropdown::new("dropdown", current_value, variants(), labels(), {
        move |value, cx| {
            if value == current_value {
                return;
            }
            update_settings_file(file.clone(), field.json_path, cx, move |settings, _cx| {
                (field.write)(settings, Some(value));
            })
            .log_err(); // todo(settings_ui) don't log err
        }
    })
    .tab_index(0)
    .title_case(should_do_titlecase)
    .into_any_element()
}

fn render_picker_trigger_button(id: SharedString, label: SharedString) -> Button {
    Button::new(id, label)
        .tab_index(0_isize)
        .style(ButtonStyle::Outlined)
        .size(ButtonSize::Medium)
        .icon(IconName::ChevronUpDown)
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .icon_position(IconPosition::End)
}

fn render_font_picker(
    field: SettingField<settings::FontFamilyName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let current_value = SettingsStore::global(cx)
        .get_value_from_file(file.to_settings(), field.pick)
        .1
        .cloned()
        .unwrap_or_else(|| SharedString::default().into());

    PopoverMenu::new("font-picker")
        .trigger(render_picker_trigger_button(
            "font_family_picker_trigger".into(),
            current_value.clone().into(),
        ))
        .menu(move |window, cx| {
            let file = file.clone();
            let current_value = current_value.clone();

            Some(cx.new(move |cx| {
                font_picker(
                    current_value.clone().into(),
                    move |font_name, cx| {
                        update_settings_file(
                            file.clone(),
                            field.json_path,
                            cx,
                            move |settings, _cx| {
                                (field.write)(settings, Some(font_name.into()));
                            },
                        )
                        .log_err(); // todo(settings_ui) don't log err
                    },
                    window,
                    cx,
                )
            }))
        })
        .anchor(gpui::Corner::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(ui::PopoverMenuHandle::default())
        .into_any_element()
}

fn render_theme_picker(
    field: SettingField<settings::ThemeName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value = value
        .cloned()
        .map(|theme_name| theme_name.0.into())
        .unwrap_or_else(|| cx.theme().name.clone());

    PopoverMenu::new("theme-picker")
        .trigger(render_picker_trigger_button(
            "theme_picker_trigger".into(),
            current_value.clone(),
        ))
        .menu(move |window, cx| {
            Some(cx.new(|cx| {
                let file = file.clone();
                let current_value = current_value.clone();
                theme_picker(
                    current_value,
                    move |theme_name, cx| {
                        update_settings_file(
                            file.clone(),
                            field.json_path,
                            cx,
                            move |settings, _cx| {
                                (field.write)(
                                    settings,
                                    Some(settings::ThemeName(theme_name.into())),
                                );
                            },
                        )
                        .log_err(); // todo(settings_ui) don't log err
                    },
                    window,
                    cx,
                )
            }))
        })
        .anchor(gpui::Corner::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(ui::PopoverMenuHandle::default())
        .into_any_element()
}

fn render_icon_theme_picker(
    field: SettingField<settings::IconThemeName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value = value
        .cloned()
        .map(|theme_name| theme_name.0.into())
        .unwrap_or_else(|| cx.theme().name.clone());

    PopoverMenu::new("icon-theme-picker")
        .trigger(render_picker_trigger_button(
            "icon_theme_picker_trigger".into(),
            current_value.clone(),
        ))
        .menu(move |window, cx| {
            Some(cx.new(|cx| {
                let file = file.clone();
                let current_value = current_value.clone();
                icon_theme_picker(
                    current_value,
                    move |theme_name, cx| {
                        update_settings_file(
                            file.clone(),
                            field.json_path,
                            cx,
                            move |settings, _cx| {
                                (field.write)(
                                    settings,
                                    Some(settings::IconThemeName(theme_name.into())),
                                );
                            },
                        )
                        .log_err(); // todo(settings_ui) don't log err
                    },
                    window,
                    cx,
                )
            }))
        })
        .anchor(gpui::Corner::TopLeft)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .with_handle(ui::PopoverMenuHandle::default())
        .into_any_element()
}

#[cfg(test)]
pub mod test {

    use super::*;

    impl SettingsWindow {
        fn navbar_entry(&self) -> usize {
            self.navbar_entry
        }
    }

    impl PartialEq for NavBarEntry {
        fn eq(&self, other: &Self) -> bool {
            self.title == other.title
                && self.is_root == other.is_root
                && self.expanded == other.expanded
                && self.page_index == other.page_index
                && self.item_index == other.item_index
            // ignoring focus_handle
        }
    }

    pub fn register_settings(cx: &mut App) {
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
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
            title_bar: None,
            original_window: None,
            worktree_root_dirs: HashMap::default(),
            files: Vec::default(),
            current_file: crate::SettingsUiFile::User,
            pages,
            search_bar: cx.new(|cx| Editor::single_line(window, cx)),
            navbar_entry: selected_idx.expect("Must have a selected navbar entry"),
            navbar_entries: Vec::default(),
            navbar_scroll_handle: UniformListScrollHandle::default(),
            navbar_focus_subscriptions: vec![],
            filter_table: vec![],
            has_query: false,
            content_handles: vec![],
            search_task: None,
            sub_page_scroll_handle: ScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            navbar_focus_handle: NonFocusableHandle::new(
                NAVBAR_CONTAINER_TAB_INDEX,
                false,
                window,
                cx,
            ),
            content_focus_handle: NonFocusableHandle::new(
                CONTENT_CONTAINER_TAB_INDEX,
                false,
                window,
                cx,
            ),
            files_focus_handle: cx.focus_handle(),
            search_index: None,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(0.0)),
            shown_errors: HashSet::default(),
        };

        settings_window.build_filter_table();
        settings_window.build_navbar(cx);
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
        > General Page*
        v Project
        - Worktree Settings Content
        v AI
        - General
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
        v General Page*
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General
        > Appearance & Behavior
        "
    );
}
