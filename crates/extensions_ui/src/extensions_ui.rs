use client::telemetry::Telemetry;
use editor::{Editor, EditorElement, EditorStyle};
use extension::{Extension, ExtensionStatus, ExtensionStore};
use fs::Fs;
use gpui::{
    actions, uniform_list, AnyElement, AppContext, EventEmitter, FocusableView, FontStyle,
    FontWeight, InteractiveElement, KeyContext, ParentElement, Render, Styled, Task, TextStyle,
    UniformListScrollHandle, View, ViewContext, VisualContext, WeakView, WhiteSpace, WindowContext,
};
use settings::Settings;
use std::time::Duration;
use std::{ops::Range, sync::Arc};
use theme::ThemeSettings;
use ui::prelude::*;

use workspace::{
    item::{Item, ItemEvent},
    Workspace, WorkspaceId,
};

actions!(zed, [Extensions]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, _cx| {
        workspace.register_action(move |workspace, _: &Extensions, cx| {
            let extensions_page = ExtensionsPage::new(workspace, cx);
            workspace.add_item(Box::new(extensions_page), cx)
        });
    })
    .detach();
}

pub struct ExtensionsPage {
    workspace: WeakView<Workspace>,
    fs: Arc<dyn Fs>,
    list: UniformListScrollHandle,
    telemetry: Arc<Telemetry>,
    extensions_entries: Vec<Extension>,
    query_editor: View<Editor>,
    query_contains_error: bool,
    extension_fetch_task: Option<Task<()>>,
}

impl Render for ExtensionsPage {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .full()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .full()
                    .p_4()
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
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let extensions_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            let query_editor = cx.new_view(|cx| Editor::single_line(cx));
            cx.subscribe(&query_editor, Self::on_query_change).detach();

            let mut this = Self {
                fs: workspace.project().read(cx).fs().clone(),
                workspace: workspace.weak_handle(),
                list: UniformListScrollHandle::new(),
                telemetry: workspace.client().telemetry().clone(),
                extensions_entries: Vec::new(),
                query_contains_error: false,
                extension_fetch_task: None,
                query_editor,
            };
            this.fetch_extensions(None, cx);
            this
        });
        extensions_panel
    }

    fn install_extension(
        &self,
        extension_id: Arc<str>,
        version: Arc<str>,
        cx: &mut ViewContext<Self>,
    ) {
        let install = ExtensionStore::global(cx).update(cx, |store, cx| {
            store.install_extension(extension_id, version, cx)
        });
        cx.spawn(move |this, mut cx| async move {
            install.await?;
            this.update(&mut cx, |_, cx| cx.notify())
        })
        .detach_and_log_err(cx);
        cx.notify();
    }

    fn uninstall_extension(&self, extension_id: Arc<str>, cx: &mut ViewContext<Self>) {
        let install = ExtensionStore::global(cx)
            .update(cx, |store, cx| store.uninstall_extension(extension_id, cx));
        cx.spawn(move |this, mut cx| async move {
            install.await?;
            this.update(&mut cx, |_, cx| cx.notify())
        })
        .detach_and_log_err(cx);
        cx.notify();
    }

    fn fetch_extensions(&mut self, search: Option<&str>, cx: &mut ViewContext<Self>) {
        let extensions =
            ExtensionStore::global(cx).update(cx, |store, cx| store.fetch_extensions(search, cx));

        cx.spawn(move |this, mut cx| async move {
            let extensions = extensions.await?;
            this.update(&mut cx, |this, cx| {
                this.extensions_entries = extensions;
                cx.notify();
            })
        })
        .detach_and_log_err(cx);
    }

    fn render_extensions(&mut self, range: Range<usize>, cx: &mut ViewContext<Self>) -> Vec<Div> {
        self.extensions_entries[range]
            .iter()
            .map(|extension| self.render_entry(extension, cx))
            .collect()
    }

    fn render_entry(&self, extension: &Extension, cx: &mut ViewContext<Self>) -> Div {
        let status = ExtensionStore::global(cx)
            .read(cx)
            .extension_status(&extension.id);

        let upgrade_button = match status.clone() {
            ExtensionStatus::NotInstalled
            | ExtensionStatus::Installing
            | ExtensionStatus::Removing => None,
            ExtensionStatus::Installed(installed_version) => {
                if installed_version != extension.version {
                    Some(
                        Button::new(
                            SharedString::from(format!("upgrade-{}", extension.id)),
                            "Upgrade",
                        )
                        .on_click(cx.listener({
                            let extension_id = extension.id.clone();
                            let version = extension.version.clone();
                            move |this, _, cx| {
                                this.telemetry
                                    .report_app_event("extensions: install extension".to_string());
                                this.install_extension(extension_id.clone(), version.clone(), cx);
                            }
                        }))
                        .color(Color::Accent),
                    )
                } else {
                    None
                }
            }
            ExtensionStatus::Upgrading => Some(
                Button::new(
                    SharedString::from(format!("upgrade-{}", extension.id)),
                    "Upgrade",
                )
                .color(Color::Accent)
                .disabled(true),
            ),
        };

        let install_or_uninstall_button = match status {
            ExtensionStatus::NotInstalled | ExtensionStatus::Installing => {
                Button::new(SharedString::from(extension.id.clone()), "Install")
                    .on_click(cx.listener({
                        let extension_id = extension.id.clone();
                        let version = extension.version.clone();
                        move |this, _, cx| {
                            this.telemetry
                                .report_app_event("extensions: install extension".to_string());
                            this.install_extension(extension_id.clone(), version.clone(), cx);
                        }
                    }))
                    .disabled(matches!(status, ExtensionStatus::Installing))
            }
            ExtensionStatus::Installed(_)
            | ExtensionStatus::Upgrading
            | ExtensionStatus::Removing => {
                Button::new(SharedString::from(extension.id.clone()), "Uninstall")
                    .on_click(cx.listener({
                        let extension_id = extension.id.clone();
                        move |this, _, cx| {
                            this.telemetry
                                .report_app_event("extensions: uninstall extension".to_string());
                            this.uninstall_extension(extension_id.clone(), cx);
                        }
                    }))
                    .disabled(matches!(
                        status,
                        ExtensionStatus::Upgrading | ExtensionStatus::Removing
                    ))
            }
        }
        .color(Color::Accent);

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
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_2()
                                .items_end()
                                .child(
                                    Headline::new(extension.name.clone())
                                        .size(HeadlineSize::Medium),
                                )
                                .child(
                                    Headline::new(format!("v{}", extension.version))
                                        .size(HeadlineSize::XSmall),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .justify_between()
                                .children(upgrade_button)
                                .child(install_or_uninstall_button),
                        ),
                )
                .child(
                    h_flex().justify_between().child(
                        Label::new(format!(
                            "{}: {}",
                            if extension.authors.len() > 1 {
                                "Authors"
                            } else {
                                "Author"
                            },
                            extension.authors.join(", ")
                        ))
                        .size(LabelSize::Small),
                    ),
                )
                .child(
                    h_flex()
                        .justify_between()
                        .children(extension.description.as_ref().map(|description| {
                            Label::new(description.clone())
                                .size(LabelSize::Small)
                                .color(Color::Default)
                        })),
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
            self.extension_fetch_task = Some(cx.spawn(|this, mut cx| async move {
                cx.background_executor()
                    .timer(Duration::from_millis(250))
                    .await;
                this.update(&mut cx, |this, cx| {
                    this.fetch_extensions(this.search_query(cx).as_deref(), cx);
                })
                .ok();
            }));
        }
    }

    pub fn search_query(&self, cx: &WindowContext) -> Option<String> {
        let search = self.query_editor.read(cx).text(cx);
        if search.trim().is_empty() {
            None
        } else {
            Some(search)
        }
    }
}

impl EventEmitter<ItemEvent> for ExtensionsPage {}

impl FocusableView for ExtensionsPage {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.query_editor.read(cx).focus_handle(cx)
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
        Some(cx.new_view(|_| ExtensionsPage {
            fs: self.fs.clone(),
            workspace: self.workspace.clone(),
            list: UniformListScrollHandle::new(),
            telemetry: self.telemetry.clone(),
            extensions_entries: Default::default(),
            query_editor: self.query_editor.clone(),
            query_contains_error: false,
            extension_fetch_task: None,
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
