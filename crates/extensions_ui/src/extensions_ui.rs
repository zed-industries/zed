use client::telemetry::Telemetry;
use editor::{Editor, EditorElement, EditorStyle};
use extension::{Extension, ExtensionStatus, ExtensionStore};
use gpui::{
    actions, canvas, uniform_list, AnyElement, AppContext, AvailableSpace, EventEmitter,
    FocusableView, FontStyle, FontWeight, InteractiveElement, KeyContext, ParentElement, Render,
    Styled, Task, TextStyle, UniformListScrollHandle, View, ViewContext, VisualContext, WhiteSpace,
    WindowContext,
};
use settings::Settings;
use std::time::Duration;
use std::{ops::Range, sync::Arc};
use theme::ThemeSettings;
use ui::{prelude::*, ToggleButton, Tooltip};

use workspace::{
    item::{Item, ItemEvent},
    Workspace, WorkspaceId,
};

actions!(zed, [Extensions]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, _cx| {
        workspace.register_action(move |workspace, _: &Extensions, cx| {
            let extensions_page = ExtensionsPage::new(workspace, cx);
            workspace.add_item_to_active_pane(Box::new(extensions_page), cx)
        });
    })
    .detach();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ExtensionFilter {
    All,
    Installed,
    NotInstalled,
}

pub struct ExtensionsPage {
    list: UniformListScrollHandle,
    telemetry: Arc<Telemetry>,
    is_fetching_extensions: bool,
    filter: ExtensionFilter,
    extension_entries: Vec<Extension>,
    query_editor: View<Editor>,
    query_contains_error: bool,
    _subscription: gpui::Subscription,
    extension_fetch_task: Option<Task<()>>,
}

impl ExtensionsPage {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let store = ExtensionStore::global(cx);
            let subscription = cx.observe(&store, |_, _, cx| cx.notify());

            let query_editor = cx.new_view(|cx| {
                let mut input = Editor::single_line(cx);
                input.set_placeholder_text("Search extensions...", cx);
                input
            });
            cx.subscribe(&query_editor, Self::on_query_change).detach();

            let mut this = Self {
                list: UniformListScrollHandle::new(),
                telemetry: workspace.client().telemetry().clone(),
                is_fetching_extensions: false,
                filter: ExtensionFilter::All,
                extension_entries: Vec::new(),
                query_contains_error: false,
                extension_fetch_task: None,
                _subscription: subscription,
                query_editor,
            };
            this.fetch_extensions(None, cx);
            this
        })
    }

    fn filtered_extension_entries(&self, cx: &mut ViewContext<Self>) -> Vec<Extension> {
        let extension_store = ExtensionStore::global(cx).read(cx);

        self.extension_entries
            .iter()
            .filter(|extension| match self.filter {
                ExtensionFilter::All => true,
                ExtensionFilter::Installed => {
                    let status = extension_store.extension_status(&extension.id);

                    matches!(status, ExtensionStatus::Installed(_))
                }
                ExtensionFilter::NotInstalled => {
                    let status = extension_store.extension_status(&extension.id);

                    matches!(status, ExtensionStatus::NotInstalled)
                }
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    fn install_extension(
        &self,
        extension_id: Arc<str>,
        version: Arc<str>,
        cx: &mut ViewContext<Self>,
    ) {
        ExtensionStore::global(cx).update(cx, |store, cx| {
            store.install_extension(extension_id, version, cx)
        });
        cx.notify();
    }

    fn uninstall_extension(&self, extension_id: Arc<str>, cx: &mut ViewContext<Self>) {
        ExtensionStore::global(cx)
            .update(cx, |store, cx| store.uninstall_extension(extension_id, cx));
        cx.notify();
    }

    fn fetch_extensions(&mut self, search: Option<&str>, cx: &mut ViewContext<Self>) {
        self.is_fetching_extensions = true;
        cx.notify();

        let extensions =
            ExtensionStore::global(cx).update(cx, |store, cx| store.fetch_extensions(search, cx));

        cx.spawn(move |this, mut cx| async move {
            let fetch_result = extensions.await;
            match fetch_result {
                Ok(extensions) => this.update(&mut cx, |this, cx| {
                    this.extension_entries = extensions;
                    this.is_fetching_extensions = false;
                    cx.notify();
                }),
                Err(err) => {
                    this.update(&mut cx, |this, cx| {
                        this.is_fetching_extensions = false;
                        cx.notify();
                    })
                    .ok();

                    Err(err)
                }
            }
        })
        .detach_and_log_err(cx);
    }

    fn render_extensions(&mut self, range: Range<usize>, cx: &mut ViewContext<Self>) -> Vec<Div> {
        self.filtered_extension_entries(cx)[range]
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
            ExtensionStatus::NotInstalled | ExtensionStatus::Installing => Button::new(
                SharedString::from(extension.id.clone()),
                if status.is_installing() {
                    "Installing..."
                } else {
                    "Install"
                },
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
            .disabled(status.is_installing()),
            ExtensionStatus::Installed(_)
            | ExtensionStatus::Upgrading
            | ExtensionStatus::Removing => Button::new(
                SharedString::from(extension.id.clone()),
                if status.is_upgrading() {
                    "Upgrading..."
                } else if status.is_removing() {
                    "Removing..."
                } else {
                    "Uninstall"
                },
            )
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
            )),
        }
        .color(Color::Accent);

        let repository_url = extension.repository.clone();
        let tooltip_text = Tooltip::text(repository_url.clone(), cx);

        div().w_full().child(
            v_flex()
                .w_full()
                .h(rems(7.))
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
                    h_flex()
                        .justify_between()
                        .child(
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
                        )
                        .child(
                            Label::new(format!("Downloads: {}", extension.download_count))
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
                        }))
                        .child(
                            IconButton::new(
                                SharedString::from(format!("repository-{}", extension.id)),
                                IconName::Github,
                            )
                            .icon_color(Color::Accent)
                            .icon_size(IconSize::Small)
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(move |_, _, cx| {
                                cx.open_url(&repository_url);
                            }))
                            .tooltip(move |_| tooltip_text.clone()),
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
            self.extension_fetch_task = Some(cx.spawn(|this, mut cx| async move {
                let search = this
                    .update(&mut cx, |this, cx| this.search_query(cx))
                    .ok()
                    .flatten();

                // Only debounce the fetching of extensions if we have a search
                // query.
                //
                // If the search was just cleared then we can just reload the list
                // of extensions without a debounce, which allows us to avoid seeing
                // an intermittent flash of a "no extensions" state.
                if let Some(_) = search {
                    cx.background_executor()
                        .timer(Duration::from_millis(250))
                        .await;
                };

                this.update(&mut cx, |this, cx| {
                    this.fetch_extensions(search.as_deref(), cx);
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

    fn render_empty_state(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let has_search = self.search_query(cx).is_some();

        let message = if self.is_fetching_extensions {
            "Loading extensions..."
        } else {
            match self.filter {
                ExtensionFilter::All => {
                    if has_search {
                        "No extensions that match your search."
                    } else {
                        "No extensions."
                    }
                }
                ExtensionFilter::Installed => {
                    if has_search {
                        "No installed extensions that match your search."
                    } else {
                        "No installed extensions."
                    }
                }
                ExtensionFilter::NotInstalled => {
                    if has_search {
                        "No not installed extensions that match your search."
                    } else {
                        "No not installed extensions."
                    }
                }
            }
        };

        Label::new(message)
    }
}

impl Render for ExtensionsPage {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .gap_4()
                    .p_4()
                    .border_b()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        h_flex()
                            .w_full()
                            .child(Headline::new("Extensions").size(HeadlineSize::XLarge)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .justify_between()
                            .child(h_flex().child(self.render_search(cx)))
                            .child(
                                h_flex()
                                    .child(
                                        ToggleButton::new("filter-all", "All")
                                            .style(ButtonStyle::Filled)
                                            .size(ButtonSize::Large)
                                            .selected(self.filter == ExtensionFilter::All)
                                            .on_click(cx.listener(|this, _event, _cx| {
                                                this.filter = ExtensionFilter::All;
                                            }))
                                            .tooltip(move |cx| {
                                                Tooltip::text("Show all extensions", cx)
                                            })
                                            .first(),
                                    )
                                    .child(
                                        ToggleButton::new("filter-installed", "Installed")
                                            .style(ButtonStyle::Filled)
                                            .size(ButtonSize::Large)
                                            .selected(self.filter == ExtensionFilter::Installed)
                                            .on_click(cx.listener(|this, _event, _cx| {
                                                this.filter = ExtensionFilter::Installed;
                                            }))
                                            .tooltip(move |cx| {
                                                Tooltip::text("Show installed extensions", cx)
                                            })
                                            .middle(),
                                    )
                                    .child(
                                        ToggleButton::new("filter-not-installed", "Not Installed")
                                            .style(ButtonStyle::Filled)
                                            .size(ButtonSize::Large)
                                            .selected(self.filter == ExtensionFilter::NotInstalled)
                                            .on_click(cx.listener(|this, _event, _cx| {
                                                this.filter = ExtensionFilter::NotInstalled;
                                            }))
                                            .tooltip(move |cx| {
                                                Tooltip::text("Show not installed extensions", cx)
                                            })
                                            .last(),
                                    ),
                            ),
                    ),
            )
            .child(v_flex().px_4().size_full().overflow_y_hidden().map(|this| {
                let entries = self.filtered_extension_entries(cx);
                if entries.is_empty() {
                    return this.py_4().child(self.render_empty_state(cx));
                }

                this.child(
                    canvas({
                        let view = cx.view().clone();
                        let scroll_handle = self.list.clone();
                        let item_count = entries.len();
                        move |bounds, cx| {
                            uniform_list::<_, Div, _>(
                                view,
                                "entries",
                                item_count,
                                Self::render_extensions,
                            )
                            .size_full()
                            .pb_4()
                            .track_scroll(scroll_handle)
                            .into_any_element()
                            .draw(
                                bounds.origin,
                                bounds.size.map(AvailableSpace::Definite),
                                cx,
                            )
                        }
                    })
                    .size_full(),
                )
            }))
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
        _: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        None
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
