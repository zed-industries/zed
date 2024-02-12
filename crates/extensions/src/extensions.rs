use client::telemetry::Telemetry;
use editor::{actions::Tab, Editor, EditorElement, EditorStyle};
use gpui::{
    svg, uniform_list, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle,
    FontWeight, InteractiveElement, KeyContext, ParentElement, Render, Styled, TextStyle, View,
    ViewContext, VisualContext, WeakView, WhiteSpace, WindowContext,
};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{prelude::*, Checkbox};

use workspace::{
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new, AppState, Extensions, Workspace, WorkspaceId,
};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &Extensions, cx| {
            let extensions_page = ExtensionsPage::new(workspace, cx);
            workspace.add_item(Box::new(extensions_page), cx)
        });
    })
    .detach();
}

pub fn show_extensions_view(app_state: &Arc<AppState>, cx: &mut AppContext) {
    open_new(&app_state, cx, |workspace, cx| {
        workspace.toggle_dock(DockPosition::Left, cx);
        let extensions_page = ExtensionsPage::new(workspace, cx);
        workspace.add_item_to_center(Box::new(extensions_page.clone()), cx);
        cx.focus_view(&extensions_page);
        cx.notify();
    })
    .detach();
}
pub struct Extension {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub repository: String,
    pub download_url: String,
    pub installed: bool,
}

pub struct ExtensionsPage {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    telemetry: Arc<Telemetry>,
    extensions_entries: Vec<Extension>,
    query_editor: View<Editor>,
    query_contains_error: bool,
}

impl Render for ExtensionsPage {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .full()
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus_handle)
            .child(
                v_flex()
                    .full()
                    .p(px(16.))
                    .child(
                        h_flex()
                            .w_full()
                            .child(Headline::new("Extensions").size(HeadlineSize::XLarge)),
                    )
                    .child(h_flex().w_56().my_4().child(self.render_search(cx)))
                    .child(
                        h_flex()
                            .flex_col()
                            .items_start()
                            .full()
                            .child(self.render_extensions(cx)),
                    ),
            )
    }
}

impl ExtensionsPage {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let extensions_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();

            cx.on_focus(&focus_handle, Self::focus_in).detach();

            cx.on_release(|this: &mut Self, _, _| {
                this.telemetry
                    .report_app_event("extensions page: close".to_string());
            })
            .detach();

            let query_editor = cx.new_view(|cx| Editor::single_line(cx));
            cx.subscribe(&query_editor, Self::on_query_change).detach();

            let mut this = Self {
                focus_handle: cx.focus_handle(),
                workspace: workspace.weak_handle(),
                telemetry: workspace.client().telemetry().clone(),
                extensions_entries: Vec::new(),
                query_contains_error: false,
                query_editor,
            };
            this.get_extensions_from_server();

            this
        });
        extensions_panel
    }
    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        // if !self.focus_handle.contains_focused(cx) {
        //     cx.emit(Event::Focus);
        // }
        self.get_extensions_from_server();
    }

    fn get_extensions_from_server(&mut self) {
        let extensions = vec![
            Extension {
                name: "Vim Mode".to_string(),
                version: "1.0.0".to_string(),
                author: "Zed".to_string(),
                description: "Zed extension for lang rust".to_string(),
                repository: "https://github.com/zed-industries/zed-extensions".to_string(),
                download_url: "https://download-url".to_string(),
                installed: false,
            },
            Extension {
                name: "Zed Mode".to_string(),
                version: "1.0.0".to_string(),
                author: "Zed".to_string(),
                description: "Zed extension for lang rust".to_string(),
                repository: "https://github.com/zed-industries/zed-extensions".to_string(),
                download_url: "https://download-url".to_string(),
                installed: false,
            },
            Extension {
                name: "Lang Mode".to_string(),
                version: "1.0.0".to_string(),
                author: "Zed".to_string(),
                description: "Zed extension for lang rust".to_string(),
                repository: "https://github.com/zed-industries/zed-extensions".to_string(),
                download_url: "https://download-url".to_string(),
                installed: false,
            },
        ];

        self.extensions_entries = extensions;
    }

    fn install_extension(&self, extension_name: String) {
        println!("INSTALL EXTENSION {}", extension_name.to_string());
        if let Some(extension) = self.get_extension(extension_name.to_string()) {
            let download_url = &extension.download_url;
            // download extension from blob
            // copy to extensions folder
        }
    }
    fn uninstall_extension(&self, extension_name: String) {
        println!("UNINSTALL EXTENSION {}", extension_name.to_string());
        if let Some(extension) = self.get_extension(extension_name.to_string()) {
            // remove extension from extensions folder
        }
    }

    fn search_extension(&self, cx: &mut ViewContext<Self>) {
        println!("SEARCH EXTENSION {}", self.search_query(cx));
        let query = self.search_query(cx);
        if query.contains('.') {
            // search by file extension ex: .rs
        } else {
            // search by extension name
        }
        // &self.get_extensions_from_server()
    }

    fn get_extension(&self, name: String) -> Option<&Extension> {
        self.extensions_entries.iter().find(|e| e.name == name)
    }

    fn render_extensions(&self, cx: &mut ViewContext<Self>) -> Div {
        let mut items = div().flex_col().full().justify_start().gap_4();
        for extension in &self.extensions_entries {
            items = items.child(self.render_entry(extension.name.to_string(), cx));
        }
        items
    }
    fn render_entry(&self, name: String, cx: &mut ViewContext<Self>) -> Div {
        if let Some(extension) = self.get_extension(name.to_string()) {
            let installed = extension.installed;
            let name = extension.name.to_string();
            let version = extension.version.to_string();
            let author = extension.author.to_string();
            let description = extension.description.to_string();
            let repository = extension.repository.to_string();
            let name_cloned = name.clone();

            let button;
            if installed {
                button = Button::new("uninstall", "Uninstall")
                    .color(Color::Accent)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.telemetry
                            .report_app_event("extensions page: uninstall extension".to_string());
                        this.uninstall_extension(name_cloned.to_string());
                    }))
                    .disabled(installed);
            } else {
                button = Button::new("install", "Install")
                    .color(Color::Accent)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.telemetry
                            .report_app_event("extensions page: install extension".to_string());
                        this.install_extension(name_cloned.to_string());
                    }))
                    .disabled(installed);
            }

            div().w_full().child(
                v_flex()
                    .w_full()
                    .p_3()
                    .mt_4()
                    .gap_2()
                    .bg(cx.theme().colors().elevated_surface_background)
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(Headline::new(name.to_string()).size(HeadlineSize::Medium)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(
                                Label::new(description.to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Default),
                            )
                            .child(button),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(
                                Label::new(format!("Author: {}", author))
                                    .size(LabelSize::Small)
                                    .color(Color::Default),
                            )
                            .child(
                                Label::new(format!("Version: {}", version))
                                    .size(LabelSize::Small)
                                    .color(Color::Default),
                            ),
                    ),
            )
        } else {
            div().child(Label::new("Extension not found").color(Color::Error))
        }
    }

    fn render_search(&self, cx: &mut ViewContext<Self>) -> Div {
        let mut key_context = KeyContext::default();
        key_context.add("BufferSearchBar");

        let editor_border = if self.query_contains_error {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        };

        h_flex()
            .w_full()
            .gap_2()
            .key_context(key_context)
            // .capture_action(cx.listener(Self::tab))
            // .on_action(cx.listener(Self::dismiss))
            .child(
                h_flex()
                    .flex_1()
                    .px_2()
                    .py_1()
                    .gap_2()
                    .border_1()
                    .border_color(editor_border)
                    .min_w(rems(384. / 16.))
                    .rounded_lg()
                    .child(Icon::new(IconName::MagnifyingGlass))
                    .child(self.render_text_input(&self.query_editor, cx)),
            )
    }

    fn render_text_input(&self, editor: &View<Editor>, cx: &ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };

        EditorElement::new(
            &editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
    fn on_query_change(
        &mut self,
        _: View<Editor>,
        event: &editor::EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let editor::EditorEvent::Edited = event {
            self.query_contains_error = false;
            self.search_extension(cx);
        }
    }
    pub fn search_query(&self, cx: &WindowContext) -> String {
        self.query_editor.read(cx).text(cx)
    }
}

impl EventEmitter<ItemEvent> for ExtensionsPage {}

impl FocusableView for ExtensionsPage {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ExtensionsPage {
    type Event = ItemEvent;

    fn tab_content(&self, _: Option<usize>, selected: bool, _: &WindowContext) -> AnyElement {
        Label::new("Extensions")
            .color(if selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("extensions page")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        Some(cx.new_view(|cx| ExtensionsPage {
            focus_handle: cx.focus_handle(),
            workspace: self.workspace.clone(),
            telemetry: self.telemetry.clone(),
            extensions_entries: Default::default(),
            query_editor: self.query_editor.clone(),
            query_contains_error: false,
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
