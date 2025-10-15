mod components;
mod page_data;

use anyhow::Result;
use editor::{Editor, EditorEvent};
use feature_flags::FeatureFlag;
use fuzzy::StringMatchCandidate;
use gpui::{
    Action, App, Div, Entity, FocusHandle, Focusable, FontWeight, Global, ReadGlobal as _,
    ScrollHandle, Stateful, Subscription, Task, TitlebarOptions, UniformListScrollHandle, Window,
    WindowBounds, WindowHandle, WindowOptions, actions, div, point, prelude::*, px, size,
    uniform_list,
};
use heck::ToTitleCase as _;
use project::WorktreeId;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{SettingsContent, SettingsStore};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashMap,
    num::{NonZero, NonZeroU32},
    ops::Range,
    rc::Rc,
    sync::{Arc, LazyLock, RwLock},
};
use title_bar::platform_title_bar::PlatformTitleBar;
use ui::{
    ContextMenu, Divider, DividerColor, DropdownMenu, DropdownStyle, IconButtonShape, KeyBinding,
    KeybindingHint, PopoverMenu, Switch, SwitchColor, Tooltip, TreeViewItem, WithScrollbar,
    prelude::*,
};
use ui_input::{NumberField, NumberFieldType};
use util::{ResultExt as _, paths::PathStyle, rel_path::RelPath};
use workspace::{OpenOptions, OpenVisible, Workspace, client_side_decorations};
use zed_actions::OpenSettings;

use crate::components::SettingsEditor;

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
    pick: fn(&SettingsContent) -> &Option<T>,
    pick_mut: fn(&mut SettingsContent) -> &mut Option<T>,
}

impl<T: 'static> Clone for SettingField<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// manual impl because derive puts a Copy bound on T, which is inaccurate in our case
impl<T: 'static> Copy for SettingField<T> {}

/// Helper for unimplemented settings, used in combination with `SettingField::unimplemented`
/// to keep the setting around in the UI with valid pick and pick_mut implementations, but don't actually try to render it.
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
            pick: |_| &Some(UnimplementedSettingField),
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
    fn reset_to_default_fn(
        &self,
        current_file: &SettingsUiFile,
        file_set_in: &settings::SettingsFile,
        cx: &App,
    ) -> Option<Box<dyn Fn(&mut App)>>;
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
        let this = *self;
        let store = SettingsStore::global(cx);
        let default_value = (this.pick)(store.raw_default_settings());
        let is_default = store
            .get_content_for_file(file_set_in.clone())
            .map_or(&None, this.pick)
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
                default_value.clone()
            } else {
                None
            };
            update_settings_file(current_file.clone(), cx, move |settings, _| {
                *(this.pick_mut)(settings) = value_to_set;
            })
            // todo(settings_ui): Don't log err
            .log_err();
        }));
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
                  window: &mut Window,
                  cx: &mut Context<SettingsWindow>| {
                render_settings_item(
                    settings_window,
                    item,
                    settings_file.clone(),
                    render_control(field, settings_file, metadata, window, cx),
                    window,
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
                move |_, window, _| {
                    window.focus_next();
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

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

pub fn init(cx: &mut App) {
    init_renderers(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenSettings, window, cx| {
            let window_handle = window
                .window_handle()
                .downcast::<Workspace>()
                .expect("Workspaces are root Windows");
            open_settings_editor(workspace, window_handle, cx);
        });
    })
    .detach();
}

fn init_renderers(cx: &mut App) {
    cx.default_global::<SettingFieldRenderer>()
        .add_basic_renderer::<UnimplementedSettingField>(|_, _, _, _, _| {
            Button::new("open-in-settings-file", "Edit in settings.json")
                .style(ButtonStyle::Outlined)
                .size(ButtonSize::Medium)
                .tab_index(0_isize)
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(OpenCurrentFile), cx);
                })
                .into_any_element()
        })
        .add_basic_renderer::<bool>(render_toggle_button)
        .add_basic_renderer::<String>(render_text_field)
        .add_basic_renderer::<settings::SaturatingBool>(render_toggle_button)
        .add_basic_renderer::<settings::CursorShape>(render_dropdown)
        .add_basic_renderer::<settings::RestoreOnStartupBehavior>(render_dropdown)
        .add_basic_renderer::<settings::BottomDockLayout>(render_dropdown)
        .add_basic_renderer::<settings::OnLastWindowClosed>(render_dropdown)
        .add_basic_renderer::<settings::CloseWindowWhenNoItems>(render_dropdown)
        .add_basic_renderer::<settings::FontFamilyName>(render_font_picker)
        // todo(settings_ui): This needs custom ui
        // .add_renderer::<settings::BufferLineHeight>(|settings_field, file, _, window, cx| {
        //     // todo(settings_ui): Do we want to expose the custom variant of buffer line height?
        //     // right now there's a manual impl of strum::VariantArray
        //     render_dropdown(*settings_field, file, window, cx)
        // })
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
        .add_basic_renderer::<settings::DiagnosticSeverityContent>(render_dropdown)
        .add_basic_renderer::<settings::SeedQuerySetting>(render_dropdown)
        .add_basic_renderer::<settings::DoubleClickInMultibuffer>(render_dropdown)
        .add_basic_renderer::<settings::GoToDefinitionFallback>(render_dropdown)
        .add_basic_renderer::<settings::ActivateOnClose>(render_dropdown)
        .add_basic_renderer::<settings::ShowDiagnostics>(render_dropdown)
        .add_basic_renderer::<settings::ShowCloseButton>(render_dropdown)
        .add_basic_renderer::<settings::ProjectPanelEntrySpacing>(render_dropdown)
        .add_basic_renderer::<settings::RewrapBehavior>(render_dropdown)
        .add_basic_renderer::<settings::FormatOnSave>(render_dropdown)
        .add_basic_renderer::<settings::IndentGuideColoring>(render_dropdown)
        .add_basic_renderer::<settings::IndentGuideBackgroundColoring>(render_dropdown)
        .add_basic_renderer::<settings::FileFinderWidthContent>(render_dropdown)
        .add_basic_renderer::<settings::ShowDiagnostics>(render_dropdown)
        .add_basic_renderer::<settings::WordsCompletionMode>(render_dropdown)
        .add_basic_renderer::<settings::LspInsertMode>(render_dropdown)
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
        .add_basic_renderer::<FontWeight>(render_number_field)
        .add_basic_renderer::<settings::MinimumContrast>(render_number_field)
        .add_basic_renderer::<settings::ShowScrollbar>(render_dropdown)
        .add_basic_renderer::<settings::ScrollbarDiagnostics>(render_dropdown)
        .add_basic_renderer::<settings::ShowMinimap>(render_dropdown)
        .add_basic_renderer::<settings::DisplayIn>(render_dropdown)
        .add_basic_renderer::<settings::MinimapThumb>(render_dropdown)
        .add_basic_renderer::<settings::MinimapThumbBorder>(render_dropdown)
        .add_basic_renderer::<settings::SteppingGranularity>(render_dropdown)
        .add_basic_renderer::<settings::NotifyWhenAgentWaiting>(render_dropdown)
        .add_basic_renderer::<settings::ImageFileSizeUnit>(render_dropdown)
        .add_basic_renderer::<settings::StatusStyle>(render_dropdown)
        .add_basic_renderer::<settings::PaneSplitDirectionHorizontal>(render_dropdown)
        .add_basic_renderer::<settings::PaneSplitDirectionVertical>(render_dropdown)
        .add_basic_renderer::<settings::PaneSplitDirectionVertical>(render_dropdown)
        .add_basic_renderer::<settings::DocumentColorsRenderMode>(render_dropdown)
    // please semicolon stay on next line
    ;
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
                is_movable: true,
                kind: gpui::WindowKind::Floating,
                window_background: cx.theme().window_background_appearance(),
                window_min_size: Some(size(px(900.), px(750.))), // 4:3 Aspect Ratio
                window_bounds: Some(WindowBounds::centered(size(px(900.), px(750.)), cx)),
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
    title_bar: Option<Entity<PlatformTitleBar>>,
    original_window: Option<WindowHandle<Workspace>>,
    files: Vec<(SettingsUiFile, FocusHandle)>,
    drop_down_file: Option<usize>,
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
    page_scroll_handle: ScrollHandle,
    focus_handle: FocusHandle,
    navbar_focus_handle: Entity<NonFocusableHandle>,
    content_focus_handle: Entity<NonFocusableHandle>,
    files_focus_handle: FocusHandle,
    search_index: Option<Arc<SearchIndex>>,
}

struct SearchIndex {
    bm25_engine: bm25::SearchEngine<usize>,
    fuzzy_match_candidates: Vec<StringMatchCandidate>,
    key_lut: Vec<SearchItemKey>,
}

struct SearchItemKey {
    page_index: usize,
    header_index: usize,
    item_index: usize,
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
                .gap_1p5()
                .child(
                    Label::new(SharedString::new_static(header))
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .buffer_font(cx),
                )
                .child(Divider::horizontal().color(DividerColor::BorderFaded))
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => {
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
                    Ok(field_renderer) => field_renderer(
                        settings_window,
                        setting_item,
                        file,
                        setting_item.metadata.as_deref(),
                        window,
                        cx,
                    ),
                    Err(warning) => render_settings_item(
                        settings_window,
                        setting_item,
                        file,
                        Button::new("error-warning", warning)
                            .style(ButtonStyle::Outlined)
                            .size(ButtonSize::Medium)
                            .icon(Some(IconName::Debug))
                            .icon_position(IconPosition::Start)
                            .icon_color(Color::Error)
                            .tab_index(0_isize)
                            .tooltip(Tooltip::text(setting_item.field.type_name()))
                            .into_any_element(),
                        window,
                        cx,
                    ),
                };

                field
                    .pt_4()
                    .map(|this| {
                        if is_last {
                            this.pb_10()
                        } else {
                            this.pb_4()
                                .border_b_1()
                                .border_color(cx.theme().colors().border_variant)
                        }
                    })
                    .into_any_element()
            }
            SettingsPageItem::SubPageLink(sub_page_link) => h_flex()
                .id(sub_page_link.title.clone())
                .w_full()
                .min_w_0()
                .gap_2()
                .justify_between()
                .pt_4()
                .map(|this| {
                    if is_last {
                        this.pb_10()
                    } else {
                        this.pb_4()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                    }
                })
                .child(
                    v_flex()
                        .w_full()
                        .max_w_1_2()
                        .child(Label::new(sub_page_link.title.clone())),
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
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .on_click({
                        let sub_page_link = sub_page_link.clone();
                        cx.listener(move |this, _, _, cx| {
                            this.push_sub_page(sub_page_link.clone(), section_header, cx)
                        })
                    }),
                )
                .into_any_element(),
        }
    }
}

fn render_settings_item(
    settings_window: &SettingsWindow,
    setting_item: &SettingItem,
    file: SettingsUiFile,
    control: AnyElement,
    _window: &mut Window,
    cx: &mut Context<'_, SettingsWindow>,
) -> Stateful<Div> {
    let (found_in_file, _) = setting_item.field.file_set_in(file.clone(), cx);
    let file_set_in = SettingsUiFile::from_settings(found_in_file.clone());

    h_flex()
        .id(setting_item.title)
        .min_w_0()
        .gap_2()
        .justify_between()
        .child(
            v_flex()
                .w_1_2()
                .child(
                    h_flex()
                        .w_full()
                        .gap_1()
                        .child(Label::new(SharedString::new_static(setting_item.title)))
                        .when_some(
                            setting_item
                                .field
                                .reset_to_default_fn(&file, &found_in_file, cx)
                                .filter(|_| file_set_in.as_ref() == Some(&file)),
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
    title: SharedString,
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
#[derive(Clone, PartialEq)]
enum SettingsUiFile {
    User,                                // Uses all settings.
    Project((WorktreeId, Arc<RelPath>)), // Has a special name, and special set of settings
    Server(&'static str),                // Uses a special name, and the user settings
}

impl SettingsUiFile {
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

        cx.observe_global_in::<SettingsStore>(window, move |this, window, cx| {
            this.fetch_files(window, cx);
            cx.notify();
        })
        .detach();

        let title_bar = if !cfg!(target_os = "macos") {
            Some(cx.new(|cx| PlatformTitleBar::new("settings-title-bar", cx)))
        } else {
            None
        };

        let mut this = Self {
            title_bar,
            original_window,
            worktree_root_dirs: HashMap::default(),
            files: vec![],
            drop_down_file: None,
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
            page_scroll_handle: ScrollHandle::new(),
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
        };

        this.fetch_files(window, cx);
        this.build_ui(window, cx);
        this.build_search_index();

        this.search_bar.update(cx, |editor, cx| {
            editor.focus_handle(cx).focus(window);
        });

        this
    }

    fn toggle_navbar_entry(&mut self, nav_entry_index: usize) {
        // We can only toggle root entries
        if !self.navbar_entries[nav_entry_index].is_root {
            return;
        }

        let expanded = &mut self.navbar_entries[nav_entry_index].expanded;
        *expanded = !*expanded;
        let expanded = *expanded;

        let toggle_page_index = self.page_index_from_navbar_index(nav_entry_index);
        let selected_page_index = self.page_index_from_navbar_index(self.navbar_entry);
        // if currently selected page is a child of the parent page we are folding,
        // set the current page to the parent page
        if !expanded && selected_page_index == toggle_page_index {
            self.navbar_entry = nav_entry_index;
            // note: not opening page. Toggling does not change content just selected page
        }
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
                    this.open_and_scroll_to_navbar_entry(entry_index, window, cx, false);
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
                    | SettingsPageItem::SubPageLink(SubPageLink { files, .. }) => {
                        if !files.contains(current_file) {
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
        if query.is_empty() || self.search_index.is_none() {
            for page in &mut self.filter_table {
                page.fill(true);
            }
            self.has_query = false;
            self.filter_matches_to_file();
            cx.notify();
            return;
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
                let SearchItemKey {
                    page_index,
                    header_index,
                    item_index,
                } = search_index.key_lut[match_index];
                let page = &mut this.filter_table[page_index];
                page[header_index] = true;
                page[item_index] = true;
            }
            this.has_query = true;
            this.filter_matches_to_file();
            this.open_first_nav_page();
            cx.notify();
        }

        self.search_task = Some(cx.spawn(async move |this, cx| {
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
        let mut key_lut: Vec<SearchItemKey> = vec![];
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
                match item {
                    SettingsPageItem::SettingItem(item) => {
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
                }
                push_candidates(&mut fuzzy_match_candidates, key_index, page.title);
                push_candidates(&mut fuzzy_match_candidates, key_index, header_str);

                key_lut.push(SearchItemKey {
                    page_index,
                    header_index,
                    item_index,
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
        self.update_matches(cx);

        cx.notify();
    }

    fn fetch_files(&mut self, window: &mut Window, cx: &mut Context<SettingsWindow>) {
        self.worktree_root_dirs.clear();
        let prev_files = self.files.clone();
        let settings_store = cx.global::<SettingsStore>();
        let mut ui_files = vec![];
        let all_files = settings_store.get_all_files();
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
        self.files = ui_files;
        let current_file_still_exists = self
            .files
            .iter()
            .any(|(file, _)| file == &self.current_file);
        if !current_file_still_exists {
            self.change_file(0, window, false, cx);
        }
    }

    fn open_navbar_entry_page(&mut self, navbar_entry: usize) {
        if !self.is_nav_entry_visible(navbar_entry) {
            self.open_first_nav_page();
        }
        self.navbar_entry = navbar_entry;
        sub_page_stack_mut().clear();
    }

    fn open_first_nav_page(&mut self) {
        let Some(first_navbar_entry_index) = self.visible_navbar_entries().next().map(|e| e.0)
        else {
            return;
        };
        self.open_navbar_entry_page(first_navbar_entry_index);
    }

    fn change_file(
        &mut self,
        ix: usize,
        window: &mut Window,
        drop_down_file: bool,
        cx: &mut Context<SettingsWindow>,
    ) {
        if ix >= self.files.len() {
            self.current_file = SettingsUiFile::User;
            self.build_ui(window, cx);
            return;
        }
        if drop_down_file {
            self.drop_down_file = Some(ix);
        }

        if self.files[ix].0 == self.current_file {
            return;
        }
        self.current_file = self.files[ix].0.clone();

        self.build_ui(window, cx);

        if self
            .visible_navbar_entries()
            .any(|(index, _)| index == self.navbar_entry)
        {
            self.open_and_scroll_to_navbar_entry(self.navbar_entry, window, cx, true);
        } else {
            self.open_first_nav_page();
        };
    }

    fn render_files_header(
        &self,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        const OVERFLOW_LIMIT: usize = 1;

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
                        this.change_file(ix, window, false, cx);
                        focus_handle.focus(window);
                    }
                }))
            };

        let this = cx.entity();

        h_flex()
            .w_full()
            .pb_4()
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
                        div.children(
                            self.files
                                .iter()
                                .enumerate()
                                .skip(OVERFLOW_LIMIT)
                                .find(|(_, (file, _))| file == &self.current_file)
                                .map(|(ix, (file, focus_handle))| {
                                    file_button(ix, file, focus_handle, cx)
                                })
                                .or_else(|| {
                                    let ix = self.drop_down_file.unwrap_or(OVERFLOW_LIMIT);
                                    self.files.get(ix).map(|(file, focus_handle)| {
                                        file_button(ix, file, focus_handle, cx)
                                    })
                                }),
                        )
                        .when(
                            self.files.len() > OVERFLOW_LIMIT + 1,
                            |div| {
                                div.child(
                                    DropdownMenu::new(
                                        "more-files",
                                        format!("+{}", self.files.len() - (OVERFLOW_LIMIT + 1)),
                                        ContextMenu::build(window, cx, move |mut menu, _, _| {
                                            for (ix, (file, focus_handle)) in self
                                                .files
                                                .iter()
                                                .enumerate()
                                                .skip(OVERFLOW_LIMIT + 1)
                                            {
                                                menu = menu.entry(
                                                    self.display_name(file)
                                                        .expect("Files should always have a name"),
                                                    None,
                                                    {
                                                        let this = this.clone();
                                                        let focus_handle = focus_handle.clone();
                                                        move |window, cx| {
                                                            this.update(cx, |this, cx| {
                                                                this.change_file(
                                                                    ix, window, true, cx,
                                                                );
                                                            });
                                                            focus_handle.focus(window);
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
                            },
                        )
                    }),
            )
            .child(
                Button::new("edit-in-json", "Edit in settings.json")
                    .tab_index(0_isize)
                    .style(ButtonStyle::OutlinedGhost)
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
        {
            "Focus Content"
        } else {
            "Focus Navbar"
        };

        v_flex()
            .w_64()
            .p_2p5()
            .when(cfg!(target_os = "macos"), |c| c.pt_10())
            .h_full()
            .flex_none()
            .border_r_1()
            .key_context("NavigationMenu")
            .on_action(cx.listener(|this, _: &CollapseNavEntry, window, cx| {
                let Some(focused_entry) = this.focused_nav_entry(window, cx) else {
                    return;
                };
                let focused_entry_parent = this.root_entry_containing(focused_entry);
                if this.navbar_entries[focused_entry_parent].expanded {
                    this.toggle_navbar_entry(focused_entry_parent);
                    window.focus(&this.navbar_entries[focused_entry_parent].focus_handle);
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
                this.open_and_scroll_to_navbar_entry(next_entry_index, window, cx, false);
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
                this.open_and_scroll_to_navbar_entry(prev_entry_index, window, cx, false);
            }))
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
                                    .map(|(ix, entry)| {
                                        TreeViewItem::new(
                                            ("settings-ui-navbar-entry", ix),
                                            entry.title,
                                        )
                                        .track_focus(&entry.focus_handle)
                                        .root_item(entry.is_root)
                                        .toggle_state(this.is_navbar_entry_selected(ix))
                                        .when(entry.is_root, |item| {
                                            item.expanded(entry.expanded || this.has_query)
                                                .on_toggle(cx.listener(
                                                    move |this, _, window, cx| {
                                                        this.toggle_navbar_entry(ix);
                                                        // Update selection state immediately before cx.notify
                                                        // to prevent double selection flash
                                                        this.navbar_entry = ix;
                                                        window.focus(
                                                            &this.navbar_entries[ix].focus_handle,
                                                        );
                                                        cx.notify();
                                                    },
                                                ))
                                        })
                                        .on_click(
                                            cx.listener(move |this, _, window, cx| {
                                                this.open_and_scroll_to_navbar_entry(
                                                    ix, window, cx, true,
                                                );
                                            }),
                                        )
                                    })
                                    .collect()
                            }),
                        )
                        .size_full()
                        .track_scroll(self.navbar_scroll_handle.clone()),
                    )
                    .vertical_scrollbar_for(self.navbar_scroll_handle.clone(), window, cx),
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

    fn open_and_scroll_to_navbar_entry(
        &mut self,
        navbar_entry_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
        focus_content: bool,
    ) {
        self.open_navbar_entry_page(navbar_entry_index);
        cx.notify();

        if self.navbar_entries[navbar_entry_index].is_root
            || !self.is_nav_entry_visible(navbar_entry_index)
        {
            self.page_scroll_handle.set_offset(point(px(0.), px(0.)));
            if focus_content {
                let Some(first_item_index) =
                    self.visible_page_items().next().map(|(index, _)| index)
                else {
                    return;
                };
                self.focus_content_element(first_item_index, window, cx);
            } else {
                window.focus(&self.navbar_entries[navbar_entry_index].focus_handle);
            }
        } else {
            let entry_item_index = self.navbar_entries[navbar_entry_index]
                .item_index
                .expect("Non-root items should have an item index");
            let Some(selected_item_index) = self
                .visible_page_items()
                .position(|(index, _)| index == entry_item_index)
            else {
                return;
            };
            self.page_scroll_handle
                .scroll_to_top_of_item(selected_item_index + 1);

            if focus_content {
                self.focus_content_element(entry_item_index, window, cx);
            } else {
                window.focus(&self.navbar_entries[navbar_entry_index].focus_handle);
            }
        }

        // Page scroll handle updates the active item index
        // in it's next paint call after using scroll_handle.scroll_to_top_of_item
        // The call after that updates the offset of the scroll handle. So to
        // ensure the scroll handle doesn't lag behind we need to render three frames
        // back to back.
        cx.on_next_frame(window, |_, window, cx| {
            cx.on_next_frame(window, |_, _, cx| {
                cx.notify();
            });
            cx.notify();
        });
        cx.notify();
    }

    fn is_nav_entry_visible(&self, nav_entry_index: usize) -> bool {
        self.visible_navbar_entries()
            .any(|(index, _)| index == nav_entry_index)
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
        window.focus(&self.navbar_entries[nav_entry_index].focus_handle);
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

    fn render_page_items<'a, Items: Iterator<Item = (usize, &'a SettingsPageItem)>>(
        &self,
        items: Items,
        page_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> impl IntoElement {
        let mut page_content = v_flex()
            .id("settings-ui-page")
            .size_full()
            .overflow_y_scroll()
            .track_scroll(&self.page_scroll_handle);

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
                                section_header.expect("All items rendered after a section header"),
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
                .render_page_items(
                    self.visible_page_items(),
                    Some(self.current_page_index()),
                    window,
                    cx,
                )
                .into_any_element();
        } else {
            page_header = h_flex()
                .ml_neg_1p5()
                .pb_4()
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
            .size_full()
            .pt_6()
            .pb_8()
            .px_8()
            .bg(cx.theme().colors().editor_background)
            .child(page_header)
            .vertical_scrollbar_for(self.page_scroll_handle.clone(), window, cx)
            .track_focus(&self.content_focus_handle.focus_handle(cx))
            .child(
                div()
                    .size_full()
                    .tab_group()
                    .tab_index(CONTENT_GROUP_TAB_INDEX)
                    .child(page_content),
            );
    }

    fn open_current_settings_file(&mut self, cx: &mut Context<Self>) {
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
                        corresponding_workspace = Some(*workspace);

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

    fn focus_content_element(&self, item_index: usize, window: &mut Window, cx: &mut App) {
        if !sub_page_stack().is_empty() {
            return;
        }
        let page_index = self.current_page_index();
        window.focus(&self.content_handles[page_index][item_index].focus_handle(cx));
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
                        .on_action(cx.listener(|this, _: &OpenCurrentFile, _, cx| {
                            this.open_current_settings_file(cx);
                        }))
                        .on_action(|_: &Minimize, window, _cx| {
                            window.minimize_window();
                        })
                        .on_action(cx.listener(|this, _: &search::FocusSearch, window, cx| {
                            this.search_bar.focus_handle(cx).focus(window);
                        }))
                        .on_action(cx.listener(|this, _: &ToggleFocusNav, window, cx| {
                            if this
                                .navbar_focus_handle
                                .focus_handle(cx)
                                .contains_focused(window, cx)
                            {
                                this.open_and_scroll_to_navbar_entry(
                                    this.navbar_entry,
                                    window,
                                    cx,
                                    true,
                                );
                            } else {
                                this.focus_and_scroll_to_nav_entry(this.navbar_entry, window, cx);
                            }
                        }))
                        .on_action(cx.listener(
                            |this, FocusFile(file_index): &FocusFile, window, _| {
                                this.focus_file_at_index(*file_index as usize, window);
                            },
                        ))
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
                        .flex_1()
                        .min_h_0()
                        .font(ui_font)
                        .bg(cx.theme().colors().background)
                        .text_color(cx.theme().colors().text)
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
    _window: &mut Window,
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
    _metadata: Option<&SettingsFieldMetadata>,
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
    _metadata: Option<&SettingsFieldMetadata>,
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
        .into_any_element()
}

fn render_dropdown<T>(
    field: SettingField<T>,
    file: SettingsUiFile,
    metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
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

    let current_value_label =
        labels()[variants().iter().position(|v| *v == current_value).unwrap()];

    DropdownMenu::new(
        "dropdown",
        if should_do_titlecase {
            current_value_label.to_title_case()
        } else {
            current_value_label.to_string()
        },
        ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (&value, &label) in std::iter::zip(variants(), labels()) {
                let file = file.clone();
                menu = menu.toggleable_entry(
                    if should_do_titlecase {
                        label.to_title_case()
                    } else {
                        label.to_string()
                    },
                    value == current_value,
                    IconPosition::End,
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
            title_bar: None,
            original_window: None,
            worktree_root_dirs: HashMap::default(),
            files: Vec::default(),
            current_file: crate::SettingsUiFile::User,
            drop_down_file: None,
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
            page_scroll_handle: ScrollHandle::new(),
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
}
