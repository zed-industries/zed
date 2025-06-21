use editor::{Editor, EditorElement, EditorStyle};
use extension_host::ExtensionStore;
use gpui::{
    App, ClickEvent, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    FontWeight, IntoElement, KeyContext, ParentElement, Render, TextStyle, WeakEntity,
};
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{ContextMenu, Modal, ModalFooter, ModalHeader, PopoverMenu, Section, prelude::*};
use url::Url;
use workspace::{ModalView, Workspace};

#[derive(PartialEq)]
pub enum RepoType {
    Github,
    Gitlab,
    Custom,
}

pub struct ExternalExtensionsModal {
    workspace: WeakEntity<Workspace>,
    custom_url_editor: Entity<Editor>,
    github_repo_user: Entity<Editor>,
    github_repo_name: Entity<Editor>,
    gitlab_domain: Entity<Editor>,
    gitlab_repo_id: Entity<Editor>,
    custom_header_type: Entity<Editor>,
    custom_header_value: Entity<Editor>,
    token_editor: Entity<Editor>,
    repo_type: RepoType,
}

impl Render for ExternalExtensionsModal {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let colors = theme.colors();

        div().elevation_3(cx).w(rems(34.)).child(
            v_flex()
                .border_1()
                .rounded_md()
                .bg(colors.background)
                .child(
                    Modal::new("add-context-server", None)
                        .header(
                            ModalHeader::new().child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .child(
                                        Headline::new("Add an External Extension")
                                            .size(HeadlineSize::Medium),
                                    )
                                    .child(self.render_repo_options_button(cx)),
                            ),
                        )
                        .section(self.render_section(cx))
                        .footer(
                            ModalFooter::new().end_slot(
                                h_flex()
                                    .gap_2()
                                    .child(Button::new("close-button", "Close").on_click(
                                        cx.listener(|_, _: &ClickEvent, _window, cx| {
                                            cx.emit(DismissEvent);
                                        }),
                                    ))
                                    .child(self.render_confirm_button(cx)),
                            ),
                        ),
                ),
        )
    }
}

impl ExternalExtensionsModal {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        workspace: WeakEntity<Workspace>,
    ) -> Self {
        let custom_url_editor = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Extension URL", cx);
            input
        });

        let github_repo_user = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("User Name", cx);
            input
        });

        let github_repo_name = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Repo Name", cx);
            input
        });

        let gitlab_domain = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Domain (e.g. gitlab.com)", cx);
            input
        });

        let gitlab_repo_id = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Repo ID", cx);
            input
        });

        let custom_header_type = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Header Name (e.g. AUTHORIZATION)", cx);
            input
        });

        let custom_header_value = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Header Value (optional)", cx);
            input
        });

        let token_editor = cx.new(|cx| {
            let mut input = Editor::single_line(window, cx);
            input.set_placeholder_text("Token (optional)", cx);
            input
        });

        Self {
            custom_url_editor,
            github_repo_user,
            github_repo_name,
            gitlab_domain,
            gitlab_repo_id,
            custom_header_type,
            custom_header_value,
            token_editor,
            workspace,
            repo_type: RepoType::Github,
        }
    }

    fn repo_type_str(&self) -> String {
        match self.repo_type {
            RepoType::Github => "GitHub".to_string(),
            RepoType::Gitlab => "GitLab".to_string(),
            RepoType::Custom => "Custom".to_string(),
        }
    }

    fn render_repo_option(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity().clone();

        PopoverMenu::new(SharedString::from("external-repo-options".to_string()))
            .trigger(
                IconButton::new(
                    SharedString::from("external-repo-options".to_string()),
                    IconName::ChevronDown,
                )
                .icon_color(Color::Accent)
                .icon_size(IconSize::Small),
            )
            .menu(move |window, cx| Some(Self::render_repo_type_options(&this, window, cx)))
    }

    fn change_repo_type(&mut self, repo_type: RepoType) {
        self.repo_type = repo_type;
    }

    fn render_repo_options_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        h_flex()
            .pl_1p5()
            .pr_2()
            .py_1()
            .gap_1()
            .bg(theme.colors().editor_background)
            .border_1()
            .border_color(theme.colors().border)
            .rounded_lg()
            .child(Label::new(self.repo_type_str()))
            .child(self.render_repo_option(cx))
    }

    fn render_repo_type_options(
        this: &Entity<Self>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        let context_menu = ContextMenu::build(window, cx, |context_menu, window, _| {
            context_menu
                .entry(
                    "GitHub",
                    None,
                    window.handler_for(this, {
                        move |this, _window, _cx| this.change_repo_type(RepoType::Github)
                    }),
                )
                .entry(
                    "GitLab",
                    None,
                    window.handler_for(this, {
                        move |this, _window, _cx| this.change_repo_type(RepoType::Gitlab)
                    }),
                )
                .entry(
                    "Custom",
                    None,
                    window.handler_for(this, {
                        move |this, _window, _cx| this.change_repo_type(RepoType::Custom)
                    }),
                )
        });

        context_menu
    }

    fn render_section(&self, cx: &mut Context<Self>) -> Section {
        if self.repo_type == RepoType::Github {
            return Section::new().child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .child(self.render_input(cx, &self.github_repo_user))
                    .child(self.render_input(cx, &self.github_repo_name))
                    .child(self.render_input(cx, &self.token_editor))
                    .children(self.render_error_message(cx)),
            );
        } else if self.repo_type == RepoType::Gitlab {
            return Section::new().child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .child(self.render_input(cx, &self.gitlab_domain))
                    .child(self.render_input(cx, &self.gitlab_repo_id))
                    .child(self.render_input(cx, &self.token_editor))
                    .children(self.render_error_message(cx)),
            );
        } else if self.repo_type == RepoType::Custom {
            return Section::new().child(
                v_flex()
                    .mt_2()
                    .gap_2()
                    .child(self.render_instructions_message(cx))
                    .child(self.render_input(cx, &self.custom_url_editor))
                    .child(self.render_input(cx, &self.custom_header_type))
                    .child(self.render_input(cx, &self.custom_header_value))
                    .children(self.render_error_message(cx)),
            );
        }

        return Section::new();
    }

    fn render_instructions_message(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .pl_1p5()
            .pr_2()
            .py_1()
            .gap_2()
            .child(
                Label::new("Enter a direct URL to a .tar.gz archive (e.g. GitHub tarball).")
                    .italic(),
            )
            .child(Label::new("Use a token if the link requires authentication.").italic())
    }

    fn render_input(&self, cx: &mut Context<Self>, editor: &Entity<Editor>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("BufferSearchBar");

        h_flex()
            .key_context(key_context)
            .h_8()
            .flex_1()
            .min_w(rems_from_px(384.))
            .pl_1p5()
            .pr_2()
            .py_1()
            .gap_2()
            .bg(theme.colors().editor_background)
            .border_1()
            .border_color(theme.colors().border)
            .rounded_lg()
            .child(self.render_text_input(cx, editor))
    }

    fn render_text_input(
        &self,
        cx: &mut Context<Self>,
        editor: &Entity<Editor>,
    ) -> impl IntoElement {
        let theme = cx.theme().clone();
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: theme.colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            editor,
            EditorStyle {
                background: theme.colors().editor_background,
                local_player: theme.players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn valid_form(&self, cx: &mut Context<Self>) -> bool {
        if self.repo_type == RepoType::Github {
            let github_repo_name_content = self.get_editor_text(cx, &self.github_repo_name);
            let github_repo_user_content = self.get_editor_text(cx, &self.github_repo_user);
            return github_repo_name_content != None && github_repo_user_content != None;
        } else if self.repo_type == RepoType::Custom {
            let custom_url_editor_content = self.get_editor_text(cx, &self.custom_url_editor);
            let custom_header_type_content = self.get_editor_text(cx, &self.custom_header_type);
            let custom_header_value_content = self.get_editor_text(cx, &self.custom_header_value);
            return custom_url_editor_content != None
                && Url::parse(custom_url_editor_content.as_ref().unwrap()).is_ok()
                && ((custom_header_type_content == None && custom_header_value_content == None)
                    || (custom_header_type_content != None
                        && custom_header_value_content != None));
        } else if self.repo_type == RepoType::Gitlab {
            let gitlab_domain_content = self.get_editor_text(cx, &self.gitlab_domain);
            let gitlab_repo_id_content = self.get_editor_text(cx, &self.gitlab_repo_id);
            return gitlab_domain_content != None && gitlab_repo_id_content != None;
        }

        return false;
    }

    fn repo_type_error_msg(&self) -> String {
        match self.repo_type {
            RepoType::Github => "Both user name and repo name should be non-empty.".to_string(),
            RepoType::Gitlab => "Both GitLab domain and repo id should be non-empty.".to_string(),
            RepoType::Custom => "Repo location must be a valid URL.\nHeader type and token must both be set or both left empty.".to_string(),
        }
    }

    fn render_error_message(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        if self.valid_form(cx) {
            return None;
        }

        Some(
            h_flex()
                .h_8()
                .flex_1()
                .min_w(rems_from_px(384.))
                .pl_1p5()
                .pr_2()
                .py_1()
                .gap_2()
                .child(Icon::new(IconName::Warning).color(Color::Muted))
                .child(Label::new(self.repo_type_error_msg()).weight(FontWeight::BOLD)),
        )
    }

    fn render_confirm_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("confirm-button", "Confirm")
            .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                let mut token;
                let mut header_type = None;
                let download_link;
                match this.repo_type {
                    RepoType::Github => {
                        // Check https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#download-a-repository-archive-tar
                        // for more details
                        let repo_user = this.get_editor_text(cx, &this.github_repo_user);
                        let repo_name = this.get_editor_text(cx, &this.github_repo_name);

                        download_link = format!(
                            "https://api.github.com/repos/{}/{}/tarball/main",
                            repo_user.unwrap_or_default(),
                            repo_name.unwrap_or_default()
                        )
                        .into();

                        token = this.get_editor_text(cx, &this.token_editor);
                        if token != None {
                            header_type = Some("Authorization".to_string());
                            token = format!("Bearer {}", token.unwrap_or_default()).into();
                        }
                    }
                    RepoType::Gitlab => {
                        // Check https://docs.gitlab.com/api/repositories/#get-file-archive
                        // for more details
                        let repo_domain = this.get_editor_text(cx, &this.gitlab_domain);
                        let repo_id = this.get_editor_text(cx, &this.gitlab_repo_id);

                        download_link = format!(
                            "https://{}/api/v4/projects/{}/repository/archive.tar.gz",
                            repo_domain.unwrap_or_default(),
                            repo_id.unwrap_or_default()
                        )
                        .into();

                        token = this.get_editor_text(cx, &this.token_editor);
                        if token != None {
                            header_type = Some("PRIVATE-TOKEN".to_string());
                        }
                    }
                    RepoType::Custom => {
                        download_link = this.get_editor_text(cx, &this.custom_url_editor);

                        token = this.get_editor_text(cx, &this.custom_header_value);
                        if token != None {
                            header_type = this.get_editor_text(cx, &this.custom_header_type);
                        }
                    }
                }

                cx.emit(DismissEvent);

                let store = ExtensionStore::global(cx);
                let url_str = download_link.clone();
                let token: Option<Arc<str>> = token.clone().map(|s| Arc::from(s.into_boxed_str()));

                // Call async function to fetch, save and install external extension
                let workspace_handle = this.workspace.clone();
                window
                    .spawn(cx, async move |cx| {
                        // Used to convert from Option<Url> to Url
                        let extension_url = url_str
                            .and_then(|text| Url::parse(&text).ok())
                            .expect("Please enter a valid URL.");

                        let install_task = store
                            .update(cx, |store, cx| {
                                store.install_external_extension(
                                    cx,
                                    extension_url,
                                    header_type.unwrap_or_default(),
                                    token,
                                )
                            })
                            .ok()?;

                        match install_task.await {
                            Ok(_) => {
                                log::info!("External extension successfully installed!");
                            }
                            Err(err) => {
                                log::error!("Failed to install external extension: {:?}", err);
                                workspace_handle
                                    .update(cx, |workspace, cx| {
                                        workspace.show_error(
                                            // NOTE: using `anyhow::context` here ends up not printing
                                            // the error
                                            &format!(
                                                "Failed to install external extension: {}",
                                                err
                                            ),
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }

                        Some(())
                    })
                    .detach();
            }))
            .disabled(!self.valid_form(cx))
    }

    pub fn get_editor_text(&self, cx: &mut App, editor: &Entity<Editor>) -> Option<String> {
        let txt = editor.read(cx).text(cx);
        if txt.trim().is_empty() {
            None
        } else {
            Some(txt)
        }
    }
}

impl EventEmitter<DismissEvent> for ExternalExtensionsModal {}
impl Focusable for ExternalExtensionsModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.custom_url_editor.focus_handle(cx)
    }
}

impl ModalView for ExternalExtensionsModal {}
