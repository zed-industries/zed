use anyhow::{bail, Context};
use client::{telemetry::Telemetry, ClientSettings};
use db::smol::io::AsyncReadExt as _;
use editor::{Editor, EditorElement, EditorStyle};
use extension::ExtensionStore;
use gpui::{
    uniform_list, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, FontStyle,
    FontWeight, InteractiveElement, KeyContext, ParentElement, Render, Styled, TextStyle,
    UniformListScrollHandle, View, ViewContext, VisualContext, WeakView, WhiteSpace, WindowContext,
};
use serde::Deserialize;
use settings::Settings;
use std::{ops::Range, sync::Arc};
use theme::ThemeSettings;
use ui::prelude::*;
use util::http::{AsyncBody, HttpClient};

const EXTENSIONS_PATH: &str = "/api/extensions";

use workspace::{
    item::{Item, ItemEvent},
    Extensions, Workspace, WorkspaceId,
};

pub fn init(http_client: Arc<dyn HttpClient>, cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, _cx| {
        let http_client = http_client.clone();
        workspace.register_action(move |workspace, _: &Extensions, cx| {
            let extensions_page = ExtensionsPage::new(workspace, http_client.clone(), cx);
            workspace.add_item(Box::new(extensions_page), cx)
        });
    })
    .detach();
}

#[derive(Deserialize)]
pub struct Extension {
    pub id: Arc<str>,
    pub version: Arc<str>,
    pub name: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub repository: String,
}

pub struct ExtensionsPage {
    workspace: WeakView<Workspace>,
    http_client: Arc<dyn HttpClient>,
    focus_handle: FocusHandle,
    list: UniformListScrollHandle,
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
                        h_flex().flex_col().items_start().full().child(
                            uniform_list::<_, Div, _>(
                                cx.view().clone(),
                                "entries",
                                self.extensions_entries.len(),
                                Self::render_extensions,
                            )
                            .size_full()
                            .track_scroll(self.list.clone()),
                        ),
                    ),
            )
    }
}

impl ExtensionsPage {
    pub fn new(
        workspace: &Workspace,
        http_client: Arc<dyn HttpClient>,
        cx: &mut ViewContext<Workspace>,
    ) -> View<Self> {
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
                list: UniformListScrollHandle::new(),
                http_client,
                telemetry: workspace.client().telemetry().clone(),
                extensions_entries: Vec::new(),
                query_contains_error: false,
                query_editor,
            };
            this.get_extensions_from_server(cx);

            this
        });
        extensions_panel
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        self.get_extensions_from_server(cx);
    }

    fn get_extensions_from_server(&mut self, cx: &mut ViewContext<Self>) {
        let url = format!(
            "{}/{}",
            ClientSettings::get_global(cx).server_url,
            EXTENSIONS_PATH
        );
        let http_client = self.http_client.clone();
        cx.spawn(move |this, mut cx| async move {
            let mut response = http_client.get(&url, AsyncBody::empty(), true).await?;

            let mut body = Vec::new();
            response
                .body_mut()
                .read_to_end(&mut body)
                .await
                .context("error reading extensions")?;

            if response.status().is_client_error() {
                let text = String::from_utf8_lossy(body.as_slice());
                bail!(
                    "status error {}, response: {text:?}",
                    response.status().as_u16()
                );
            }

            let extensions = serde_json::from_slice(&body)?;

            this.update(&mut cx, |this, cx| {
                this.extensions_entries = extensions;
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
    }

    fn install_extension(
        &self,
        extension_id: Arc<str>,
        version: Arc<str>,
        _cx: &mut ViewContext<Self>,
    ) {
        println!("INSTALL EXTENSION {} {}", extension_id, version);
    }

    fn uninstall_extension(&self, extension_id: Arc<str>, _cx: &mut ViewContext<Self>) {
        println!("UNINSTALL EXTENSION {}", extension_id);
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

    fn render_extensions(&mut self, range: Range<usize>, cx: &mut ViewContext<Self>) -> Vec<Div> {
        self.extensions_entries[range]
            .iter()
            .map(|extension| self.render_entry(extension, cx))
            .collect()
    }

    fn render_entry(&self, extension: &Extension, cx: &mut ViewContext<Self>) -> Div {
        let installed = ExtensionStore::global(cx)
            .read(cx)
            .is_extension_installed(&extension.id);

        let button;
        if installed {
            button = Button::new(
                SharedString::from(format!("uninstall-{}", extension.id)),
                "Uninstall",
            )
            .color(Color::Accent)
            .on_click(cx.listener({
                let extension_id = extension.id.clone();
                move |this, _, cx| {
                    this.telemetry
                        .report_app_event("extensions page: uninstall extension".to_string());
                    this.uninstall_extension(extension_id.clone(), cx);
                }
            }))
            .disabled(installed);
        } else {
            button = Button::new(
                SharedString::from(format!("install-{}", extension.id)),
                "Install",
            )
            .color(Color::Accent)
            .on_click(cx.listener({
                let extension_id = extension.id.clone();
                let version = extension.version.clone();
                move |this, _, cx| {
                    this.telemetry
                        .report_app_event("extensions page: install extension".to_string());
                    this.install_extension(extension_id.clone(), version.clone(), cx);
                }
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
                        .child(Headline::new(extension.name.clone()).size(HeadlineSize::Medium)),
                )
                .child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .justify_between()
                        .children(extension.description.as_ref().map(|description| {
                            Label::new(description.clone())
                                .size(LabelSize::Small)
                                .color(Color::Default)
                        }))
                        .child(button),
                )
                .child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .justify_between()
                        .child(
                            Label::new(format!("Author: {}", extension.authors.join(", ")))
                                .size(LabelSize::Small)
                                .color(Color::Default),
                        )
                        .child(
                            Label::new(format!("Version: {}", extension.version))
                                .size(LabelSize::Small)
                                .color(Color::Default),
                        ),
                ),
        )
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
            http_client: self.http_client.clone(),
            list: UniformListScrollHandle::new(),
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
