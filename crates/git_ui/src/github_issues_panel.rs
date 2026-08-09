use std::path::PathBuf;
use std::sync::Arc;

use git::GitHostingProviderRegistry;
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Pixels, ScrollStrategy, SharedString, Subscription, Task, UniformListScrollHandle,
    WeakEntity, Window, uniform_list,
};
use project::Project;
use settings::Settings as _;
use time::OffsetDateTime;
use ui::{Avatar, Chip, Divider, IconButton, Tooltip, WithScrollbar, prelude::*};
use util::ResultExt as _;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

pub use zed_actions::github_issues_panel::{Refresh, ToggleFocus};

use crate::{
    github_issue_view::GithubIssueView,
    github_issues::{
        GhNotAuthenticatedError, GhNotInstalledError, GithubIssuesClient, IssueLabel, IssueSummary,
    },
    github_issues_panel_settings::GithubIssuesPanelSettings,
};

const GITHUB_ISSUES_PANEL_KEY: &str = "GithubIssuesPanel";

/// Matches the git panel history tab's cap on tag chips per row.
const MAX_ROW_LABEL_CHIPS: usize = 3;

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
        workspace.toggle_panel_focus::<GithubIssuesPanel>(window, cx);
    });
}

/// What the panel's list area shows.
///
/// A refresh while issues are already listed does not go through `Loading`:
/// the stale list stays on screen (with the refresh button disabled via
/// [`GithubIssuesPanel::fetch_task`]) so the panel never flashes empty.
enum IssuesState {
    NoRepository,
    Unavailable { message: SharedString },
    GhNotInstalled,
    NotAuthenticated { message: SharedString },
    Loading,
    Loaded { issues: Vec<IssueSummary> },
    Failed { message: SharedString },
}

pub struct GithubIssuesPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    fs: Arc<dyn fs::Fs>,
    focus_handle: FocusHandle,
    list_scroll_handle: UniformListScrollHandle,
    client: Option<GithubIssuesClient>,
    state: IssuesState,
    /// The in-flight fetch. Replacing it drops the stale task, so a repo
    /// switch mid-fetch cannot deliver another repository's issues.
    fetch_task: Option<Task<()>>,
    selected_index: Option<usize>,
    _subscriptions: Vec<Subscription>,
}

impl GithubIssuesPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, Self::new)
    }

    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = cx.weak_entity();
        let project = workspace.project().clone();
        let fs = workspace.app_state().fs.clone();
        let git_store = project.read(cx).git_store().clone();

        cx.new(|cx| {
            let subscriptions = vec![cx.subscribe_in(
                &git_store,
                window,
                |this: &mut Self, _, event, window, cx| {
                    if matches!(
                        event,
                        project::git_store::GitStoreEvent::ActiveRepositoryChanged(_)
                    ) {
                        this.client = None;
                        this.refresh(window, cx);
                    }
                },
            )];

            let mut this = Self {
                workspace: workspace_handle,
                project,
                fs,
                focus_handle: cx.focus_handle(),
                list_scroll_handle: UniformListScrollHandle::new(),
                client: None,
                state: IssuesState::NoRepository,
                fetch_task: None,
                selected_index: None,
                _subscriptions: subscriptions,
            };
            this.refresh(window, cx);
            this
        })
    }

    fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let project = self.project.read(cx);
        if project.is_via_collab() || project.is_via_remote_server() {
            self.set_idle_state(
                IssuesState::Unavailable {
                    message: "GitHub issues aren't available in remote projects".into(),
                },
                cx,
            );
            return;
        }
        let Some(repository) = project.active_repository(cx) else {
            self.set_idle_state(IssuesState::NoRepository, cx);
            return;
        };
        let Some(remote_url) = repository.read(cx).default_remote_url() else {
            self.set_idle_state(
                IssuesState::Unavailable {
                    message: "No Git remote configured".into(),
                },
                cx,
            );
            return;
        };
        let provider_registry = GitHostingProviderRegistry::default_global(cx);
        let is_github = git::parse_git_remote_url(provider_registry, &remote_url)
            .is_some_and(|(provider, _)| provider.name() == "GitHub");
        if !is_github {
            self.set_idle_state(
                IssuesState::Unavailable {
                    message: "The Git remote isn't a GitHub repository".into(),
                },
                cx,
            );
            return;
        }

        let repo_workdir: PathBuf = repository.read(cx).work_directory_abs_path.to_path_buf();
        let environment = self.project.read(cx).environment().clone();
        // Reopened on every refresh so that installing `gh` or signing in
        // takes effect without restarting Zed.
        let open_client = GithubIssuesClient::open(environment, repo_workdir, cx);

        if !matches!(self.state, IssuesState::Loaded { .. }) {
            self.state = IssuesState::Loading;
        }
        self.fetch_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = async {
                let client = open_client.await?;
                let issues = this.update(cx, |this, cx| {
                    this.client = Some(client.clone());
                    client.list_open_issues(cx)
                })?;
                issues.await
            }
            .await;
            this.update(cx, |this, cx| {
                this.fetch_task = None;
                match result {
                    Ok(issues) => {
                        this.selected_index = None;
                        this.state = IssuesState::Loaded { issues };
                    }
                    Err(error) => {
                        if error.downcast_ref::<GhNotInstalledError>().is_some() {
                            this.state = IssuesState::GhNotInstalled;
                        } else if let Some(auth_error) =
                            error.downcast_ref::<GhNotAuthenticatedError>()
                        {
                            this.state = IssuesState::NotAuthenticated {
                                message: auth_error.message.clone().into(),
                            };
                        } else {
                            // A failed refresh should not cost the user the
                            // list they were looking at.
                            if !matches!(this.state, IssuesState::Loaded { .. }) {
                                this.state = IssuesState::Failed {
                                    message: format!("{error:#}").into(),
                                };
                            }
                            if let Some(workspace) = this.workspace.upgrade() {
                                git_ui_core::notifications::show_error_toast(
                                    workspace,
                                    "load GitHub issues",
                                    error,
                                    cx,
                                );
                            }
                        }
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn set_idle_state(&mut self, state: IssuesState, cx: &mut Context<Self>) {
        self.client = None;
        self.fetch_task = None;
        self.selected_index = None;
        self.state = state;
        cx.notify();
    }

    fn handle_refresh(&mut self, _: &Refresh, window: &mut Window, cx: &mut Context<Self>) {
        if self.fetch_task.is_none() {
            self.refresh(window, cx);
        }
    }

    fn issues(&self) -> &[IssueSummary] {
        match &self.state {
            IssuesState::Loaded { issues } => issues,
            _ => &[],
        }
    }

    fn select_index(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        if self.selected_index != index {
            self.selected_index = index;
            if let Some(index) = index {
                self.list_scroll_handle
                    .scroll_to_item(index, ScrollStrategy::Nearest);
            }
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let count = self.issues().len();
        if count == 0 {
            return;
        }
        let next = match self.selected_index {
            Some(current) => (current + 1).min(count - 1),
            None => 0,
        };
        self.select_index(Some(next), cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.issues().len();
        if count == 0 {
            return;
        }
        let previous = match self.selected_index {
            Some(current) => current.saturating_sub(1),
            None => count - 1,
        };
        self.select_index(Some(previous), cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        if !self.issues().is_empty() {
            self.select_index(Some(0), cx);
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        let count = self.issues().len();
        if count > 0 {
            self.select_index(Some(count - 1), cx);
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            self.open_issue(index, window, cx);
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.select_index(None, cx);
    }

    fn open_issue(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(issue) = self.issues().get(index) else {
            return;
        };
        let Some(client) = self.client.clone() else {
            return;
        };
        GithubIssueView::open(
            issue.number,
            issue.title.clone().into(),
            client,
            self.workspace.clone(),
            window,
            cx,
        );
    }

    fn render_issue_rows(
        &mut self,
        range: std::ops::Range<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let local_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        let now = OffsetDateTime::now_utc();
        let is_panel_focused = self.focus_handle.is_focused(window);
        let selected_index = self.selected_index;

        let mut rows = Vec::with_capacity(range.len());
        for index in range {
            let Some(issue) = self.issues().get(index).cloned() else {
                continue;
            };
            let issue_reference: SharedString = format!("#{}", issue.number).into();
            let author_login: Option<SharedString> = issue
                .author
                .as_ref()
                .map(|author| SharedString::from(author.login.clone()));
            let relative_time =
                relative_timestamp(&issue.updated_at, now, local_offset).unwrap_or_default();
            let avatar = github_author_avatar(author_login.as_deref(), px(14.), cx);
            let is_selected = selected_index == Some(index);

            let dot_separator = || {
                Label::new("•")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .alpha(0.5)
                    .flex_none()
            };

            rows.push(
                v_flex()
                    .id(("github-issue-item", index))
                    .cursor_pointer()
                    .w_full()
                    .py_1()
                    .px_2()
                    .gap_0p5()
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .when(is_selected && is_panel_focused, |this| {
                        this.border_color(cx.theme().colors().panel_focused_border)
                    })
                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                    .child(
                        h_flex()
                            .gap_1()
                            .w_full()
                            .min_w_0()
                            .child(Label::new(issue.title.clone()).truncate())
                            .children((!issue.labels.is_empty()).then(|| {
                                let hidden_label_count =
                                    issue.labels.len().saturating_sub(MAX_ROW_LABEL_CHIPS);
                                h_flex()
                                    .gap_1()
                                    .min_w_0()
                                    .children(
                                        issue
                                            .labels
                                            .iter()
                                            .take(MAX_ROW_LABEL_CHIPS)
                                            .map(|label| issue_label_chip(label, cx).truncate()),
                                    )
                                    .when(hidden_label_count > 0, |this| {
                                        let hidden_label_names = issue.labels
                                            [MAX_ROW_LABEL_CHIPS..]
                                            .iter()
                                            .map(|label| label.name.as_str())
                                            .collect::<Vec<_>>()
                                            .join(", ");
                                        this.child(
                                            Chip::new(format!("+{hidden_label_count}"))
                                                .bg_color(
                                                    cx.theme().colors().element_active.opacity(0.8),
                                                )
                                                .tooltip(Tooltip::text(hidden_label_names)),
                                        )
                                    })
                            })),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .min_w_0()
                            .gap_1p5()
                            .child(div().flex_none().child(avatar))
                            .when_some(author_login, |this, author_login| {
                                this.child(
                                    Label::new(author_login)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .truncate(),
                                )
                                .child(dot_separator())
                            })
                            .when(!relative_time.is_empty(), |this| {
                                this.child(
                                    Label::new(relative_time)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .flex_none(),
                                )
                                .child(dot_separator())
                            })
                            .child(
                                Label::new(issue_reference.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .flex_none(),
                            ),
                    )
                    .tooltip(move |_, cx| {
                        Tooltip::with_meta("View Issue", None, issue_reference.clone(), cx)
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.select_index(Some(index), cx);
                        this.open_issue(index, window, cx);
                    }))
                    .into_any_element(),
            );
        }
        rows
    }

    fn render_placeholder(
        &self,
        message: impl Into<SharedString>,
        action: Option<AnyElement>,
    ) -> AnyElement {
        v_flex()
            .p_4()
            .gap_2()
            .items_center()
            .child(
                Label::new(message)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .children(action)
            .into_any_element()
    }

    fn render_list_area(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match &self.state {
            IssuesState::NoRepository => {
                self.render_placeholder("No Git repository in this project", None)
            }
            IssuesState::Unavailable { message } => self.render_placeholder(message.clone(), None),
            IssuesState::GhNotInstalled => self.render_placeholder(
                "GitHub CLI (gh) not found",
                Some(
                    Button::new("install-gh", "Install GitHub CLI")
                        .on_click(|_, _, cx| cx.open_url("https://cli.github.com"))
                        .into_any_element(),
                ),
            ),
            IssuesState::NotAuthenticated { message } => self.render_placeholder(
                "GitHub CLI is not signed in. Run `gh auth login` in a terminal, then refresh.",
                Some(
                    Label::new(message.clone())
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .into_any_element(),
                ),
            ),
            IssuesState::Loading => self.render_placeholder("Loading issues…", None),
            IssuesState::Failed { message } => self.render_placeholder(message.clone(), None),
            IssuesState::Loaded { issues } => {
                if issues.is_empty() {
                    return self.render_placeholder("No open issues", None);
                }
                v_flex()
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .child(
                        uniform_list(
                            "github-issues-list",
                            issues.len(),
                            cx.processor(Self::render_issue_rows),
                        )
                        .size_full()
                        .track_scroll(&self.list_scroll_handle),
                    )
                    .vertical_scrollbar_for(&self.list_scroll_handle, window, cx)
                    .into_any_element()
            }
        }
    }
}

/// An issue or comment author's avatar, from GitHub's avatar redirect for the
/// login, falling back to the same placeholder circle as [`CommitAvatar`].
///
/// The `gh` CLI's issue JSON has no avatar URL, but GitHub serves every user's
/// avatar at `github.com/<login>.png`.
///
/// [`CommitAvatar`]: crate::commit_tooltip::CommitAvatar
pub(crate) fn github_author_avatar(login: Option<&str>, size: Pixels, cx: &App) -> AnyElement {
    let border_color = cx.theme().colors().border_variant;
    match login {
        Some(login) => Avatar::new(format!("https://github.com/{login}.png?size=64"))
            .size(size)
            .border_color(border_color)
            .into_any_element(),
        None => h_flex()
            .size(size + px(2.))
            .justify_center()
            .rounded_full()
            .border_1()
            .border_color(border_color)
            .bg(cx.theme().colors().element_disabled)
            .child(
                Icon::new(IconName::Person)
                    .color(Color::Muted)
                    .size(IconSize::XSmall),
            )
            .into_any_element(),
    }
}

/// A chip tinted with the label's color from GitHub, or an untinted chip when
/// the color doesn't parse.
pub(crate) fn issue_label_chip(label: &IssueLabel, _cx: &App) -> Chip {
    let chip = Chip::new(label.name.clone());
    match gpui::Rgba::try_from(format!("#{}", label.color).as_str()) {
        Ok(color) => {
            let color: gpui::Hsla = color.into();
            chip.bg_color(color.opacity(0.15))
                .border_color(color.opacity(0.5))
        }
        Err(_) => chip,
    }
}

/// `created_at`/`updated_at` from `gh` rendered like the git history tab's
/// commit times ("3 days ago"), or `None` when the timestamp doesn't parse.
pub(crate) fn relative_timestamp(
    timestamp: &str,
    now: OffsetDateTime,
    local_offset: time::UtcOffset,
) -> Option<SharedString> {
    let parsed =
        OffsetDateTime::parse(timestamp, &time::format_description::well_known::Rfc3339).ok()?;
    Some(
        time_format::format_localized_timestamp(
            parsed,
            now,
            local_offset,
            time_format::TimestampFormat::Relative,
        )
        .into(),
    )
}

impl Render for GithubIssuesPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_fetching = self.fetch_task.is_some();

        v_flex()
            .key_context("GithubIssuesPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::handle_refresh))
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .child(
                        Label::new("GitHub Issues")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        IconButton::new("refresh-issues", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .disabled(is_fetching)
                            .tooltip(Tooltip::text("Refresh Issues"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.handle_refresh(&Refresh, window, cx)
                            })),
                    ),
            )
            .child(Divider::horizontal())
            .child(self.render_list_area(window, cx))
    }
}

impl Focusable for GithubIssuesPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for GithubIssuesPanel {}

impl Panel for GithubIssuesPanel {
    fn persistent_name() -> &'static str {
        "GithubIssuesPanel"
    }

    fn panel_key() -> &'static str {
        GITHUB_ISSUES_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        GithubIssuesPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.github_issues_panel.get_or_insert_default().dock = Some(position.into())
        });
    }

    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        GithubIssuesPanelSettings::get_global(cx).default_width
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        Some(IconName::Github).filter(|_| GithubIssuesPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("GitHub Issues Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        8
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.github_issues_panel.get_or_insert_default().button = Some(false);
        }))
    }
}
