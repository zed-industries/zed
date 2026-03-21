use anyhow::Context as _;
use fuzzy::StringMatchCandidate;
use gpui::{
    AnyElement, App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Task,
    WeakEntity, Window, actions,
};
use picker::{Picker, PickerDelegate};
use project::git_store::Repository;
use serde::Deserialize;
use std::sync::Arc;
use time::{OffsetDateTime, UtcOffset};
use time_format;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

actions!(
    pull_request_picker,
    [
        /// Enter PR creation mode where the search input becomes the PR title.
        CreatePullRequestInline,
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestEntry {
    pub number: u32,
    pub title: String,
    pub url: String,
    pub state: String,
    pub head_ref_name: String,
    pub created_at: String,
    pub author: PullRequestAuthor,
    pub is_draft: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestAuthor {
    pub login: String,
}

#[derive(Debug, Clone)]
enum PullRequestLoadState {
    Loading,
    Loaded(Vec<PullRequestEntry>),
    Error(String),
}

pub fn create_embedded(
    repository: Option<Entity<Repository>>,
    workspace: WeakEntity<Workspace>,
    width: Rems,
    window: &mut Window,
    cx: &mut Context<PullRequestList>,
) -> PullRequestList {
    PullRequestList::new_embedded(repository, workspace, width, window, cx)
}

pub struct PullRequestList {
    width: Rems,
    pub picker: Entity<Picker<PullRequestListDelegate>>,
    picker_focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl PullRequestList {
    fn new_inner(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        embedded: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let _subscriptions = Vec::new();

        let repo_work_dir = repository
            .as_ref()
            .map(|repo| repo.read(cx).work_directory_abs_path.to_path_buf());

        let fetch_work_dir = repo_work_dir.clone();
        let delegate = PullRequestListDelegate::new(workspace, repository, repo_work_dir, window, cx);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(!embedded));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });

        cx.spawn_in(window, async move |this, cx| {
            let entries = fetch_pull_requests(fetch_work_dir).await;

            this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.load_state = match entries {
                        Ok(entries) => PullRequestLoadState::Loaded(entries),
                        Err(e) => PullRequestLoadState::Error(e.to_string()),
                    };
                    picker.refresh(window, cx);
                });
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        Self {
            picker,
            picker_focus_handle,
            width,
            _subscriptions,
        }
    }

    fn new_embedded(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner(repository, workspace, width, true, window, cx);
        this._subscriptions
            .push(cx.subscribe(&this.picker, |_, _, _, cx| {
                cx.emit(DismissEvent);
            }));
        this
    }

    pub fn handle_create_pr(
        &mut self,
        _: &CreatePullRequestInline,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let repo = self.picker.read(cx).delegate.repository.clone();
        if let Some(repo) = &repo {
            let repo_read = repo.read(cx);

            let has_changes = repo_read.status().next().is_some();
            if has_changes {
                let _ = window.prompt(
                    gpui::PromptLevel::Warning,
                    "Uncommitted Changes",
                    Some("You have uncommitted changes. Please commit or stash them before creating a pull request."),
                    &["OK"],
                    cx,
                );
                return;
            }

            let has_unpushed = repo_read
                .branch
                .as_ref()
                .and_then(|b| b.upstream.as_ref())
                .map(|u| match &u.tracking {
                    git::repository::UpstreamTracking::Tracked(status) => status.ahead > 0,
                    _ => false,
                })
                .unwrap_or(false);
            if has_unpushed {
                let _ = window.prompt(
                    gpui::PromptLevel::Warning,
                    "Unpushed Commits",
                    Some("You have unpushed commits. Please push your changes before creating a pull request."),
                    &["OK"],
                    cx,
                );
                return;
            }
        }

        self.picker.update(cx, |picker, cx| {
            picker.delegate.creating_pr = true;
            picker.set_query("", window, cx);
            picker.refresh_placeholder(window, cx);
            cx.notify();
        });
    }

    pub fn handle_modifiers_changed(
        &mut self,
        _ev: &gpui::ModifiersChangedEvent,
        _: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl ModalView for PullRequestList {}
impl EventEmitter<DismissEvent> for PullRequestList {}
impl Focusable for PullRequestList {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for PullRequestList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("PullRequestList")
            .w(self.width)
            .on_action(cx.listener(Self::handle_create_pr))
            .child(self.picker.clone())
    }
}

async fn fetch_pull_requests(
    work_dir: Option<std::path::PathBuf>,
) -> anyhow::Result<Vec<PullRequestEntry>> {
    let gh_path = which::which("gh").map_err(|_| {
        anyhow::anyhow!(
            "GitHub CLI not found. Install it from https://cli.github.com to view pull requests."
        )
    })?;

    let mut cmd = smol::process::Command::new(gh_path);
    cmd.args([
        "pr",
        "list",
        "--json",
        "number,title,url,state,headRefName,createdAt,author,isDraft",
        "--limit",
        "50",
    ]);
    if let Some(dir) = &work_dir {
        cmd.current_dir(dir);
    }
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let output = cmd.output().await.context("Failed to run gh CLI")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("gh auth login") || stderr.contains("not logged") {
            return Err(anyhow::anyhow!(
                "GitHub CLI not authenticated. Run `gh auth login` in your terminal."
            ));
        }
        if stderr.contains("not a git repository") {
            return Err(anyhow::anyhow!("Not a git repository."));
        }
        return Err(anyhow::anyhow!("gh CLI error: {}", stderr.trim()));
    }

    let entries: Vec<PullRequestEntry> =
        serde_json::from_slice(&output.stdout).context("Failed to parse gh CLI output")?;

    Ok(entries)
}

pub async fn fetch_current_branch_pr(
    work_dir: Option<std::path::PathBuf>,
    branch_name: &str,
) -> anyhow::Result<Option<PullRequestEntry>> {
    let gh_path = match which::which("gh") {
        Ok(path) => path,
        Err(_) => return Ok(None),
    };

    let mut cmd = smol::process::Command::new(gh_path);
    cmd.args([
        "pr",
        "list",
        "--head",
        branch_name,
        "--json",
        "number,title,url,state,headRefName,createdAt,author,isDraft",
        "--limit",
        "1",
    ]);
    if let Some(dir) = &work_dir {
        cmd.current_dir(dir);
    }
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let output = cmd.output().await.context("Failed to run gh CLI")?;

    if !output.status.success() {
        return Ok(None);
    }

    let entries: Vec<PullRequestEntry> =
        serde_json::from_slice(&output.stdout).context("Failed to parse gh CLI output")?;

    Ok(entries.into_iter().next())
}

async fn create_pull_request_with_gh(
    work_dir: Option<std::path::PathBuf>,
    title: &str,
) -> anyhow::Result<String> {
    let gh_path = which::which("gh").map_err(|_| {
        anyhow::anyhow!("GitHub CLI not found. Install it from https://cli.github.com")
    })?;

    let mut cmd = smol::process::Command::new(gh_path);
    cmd.args(["pr", "create", "--title", title, "--fill"]);
    if let Some(dir) = &work_dir {
        cmd.current_dir(dir);
    }
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let output = cmd.output().await.context("Failed to run gh CLI")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("{}", stderr.trim()));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(url)
}

#[derive(Debug, Clone)]
struct PullRequestMatch {
    entry: PullRequestEntry,
    positions: Vec<usize>,
}

pub struct PullRequestListDelegate {
    matches: Vec<PullRequestMatch>,
    load_state: PullRequestLoadState,
    _workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    work_dir: Option<std::path::PathBuf>,
    creating_pr: bool,
    selected_index: usize,
    last_query: String,
    focus_handle: FocusHandle,
    timezone: UtcOffset,
}

impl PullRequestListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        work_dir: Option<std::path::PathBuf>,
        _window: &mut Window,
        cx: &mut Context<PullRequestList>,
    ) -> Self {
        let timezone = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);

        Self {
            matches: vec![],
            load_state: PullRequestLoadState::Loading,
            _workspace: workspace,
            repository,
            work_dir,
            creating_pr: false,
            selected_index: 0,
            last_query: Default::default(),
            focus_handle: cx.focus_handle(),
            timezone,
        }
    }

    fn format_search_string(entry: &PullRequestEntry) -> String {
        format!("#{} {}", entry.number, entry.title)
    }

    fn format_timestamp(&self, created_at: &str) -> String {
        if let Ok(timestamp) = OffsetDateTime::parse(
            created_at,
            &time::format_description::well_known::Rfc3339,
        ) {
            time_format::format_localized_timestamp(
                timestamp,
                OffsetDateTime::now_utc(),
                self.timezone,
                time_format::TimestampFormat::EnhancedAbsolute,
            )
        } else {
            created_at.to_string()
        }
    }

    fn open_in_browser(&self, ix: usize, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(entry_match) = self.matches.get(ix) {
            cx.open_url(entry_match.entry.url.as_str());
        }
    }

    fn copy_url(&self, ix: usize, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(entry_match) = self.matches.get(ix) {
            cx.write_to_clipboard(ClipboardItem::new_string(entry_match.entry.url.clone()));
        }
    }
}

impl PickerDelegate for PullRequestListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        if self.creating_pr {
            "Enter PR title…".into()
        } else {
            "Search pull requests…".into()
        }
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.last_query = query.clone();
        if self.creating_pr {
            self.matches = vec![];
            return Task::ready(());
        }
        let all_entries = match &self.load_state {
            PullRequestLoadState::Loaded(entries) => entries.clone(),
            _ => return Task::ready(()),
        };

        cx.spawn_in(window, async move |picker, cx| {
            let matches: Vec<PullRequestMatch> = if query.is_empty() {
                all_entries
                    .into_iter()
                    .map(|entry| PullRequestMatch {
                        entry,
                        positions: Vec::new(),
                    })
                    .collect()
            } else {
                let candidates = all_entries
                    .iter()
                    .enumerate()
                    .map(|(ix, entry)| {
                        StringMatchCandidate::new(
                            ix,
                            &Self::format_search_string(entry),
                        )
                    })
                    .collect::<Vec<StringMatchCandidate>>();
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|candidate| {
                    let entry = all_entries[candidate.candidate_id].clone();
                    PullRequestMatch {
                        entry,
                        positions: candidate.positions,
                    }
                })
                .collect()
            };

            picker
                .update(cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                    delegate.last_query = query;
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.creating_pr {
            let title = self.last_query.clone();
            if title.is_empty() {
                return;
            }
            let work_dir = self.work_dir.clone();
            self.creating_pr = false;

            cx.spawn(async move |_picker, _cx| {
                let result = create_pull_request_with_gh(work_dir, &title).await;
                if let Ok(url) = &result {
                    log::info!("PR created: {}", url);
                }
                result.map(|_| ())
            })
            .detach_and_prompt_err("Failed to create pull request", window, cx, |e: &anyhow::Error, _, _| {
                Some(e.to_string())
            });
            return;
        }
        self.open_in_browser(self.selected_index(), window, cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_match = &self.matches[ix];
        let entry = &entry_match.entry;

        let title_text = Self::format_search_string(entry);
        let positions = entry_match.positions.clone();
        let title_label = HighlightedLabel::new(title_text, positions)
            .truncate()
            .into_any_element();

        let state_indicator = if entry.is_draft {
            "Draft"
        } else {
            match entry.state.as_str() {
                "OPEN" => "Open",
                "CLOSED" => "Closed",
                "MERGED" => "Merged",
                _ => "",
            }
        };

        let timestamp = self.format_timestamp(&entry.created_at);

        let detail_info = h_flex()
            .gap_1p5()
            .w_full()
            .child(
                Label::new(entry.head_ref_name.clone())
                    .truncate()
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                Label::new("•")
                    .alpha(0.5)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                Label::new(entry.author.login.clone())
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                Label::new("•")
                    .alpha(0.5)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                Label::new(timestamp)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .when(!state_indicator.is_empty(), |this| {
                this.child(
                    Label::new("•")
                        .alpha(0.5)
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .child(
                    Label::new(state_indicator)
                        .color(if entry.state == "OPEN" && !entry.is_draft {
                            Color::Success
                        } else {
                            Color::Muted
                        })
                        .size(LabelSize::Small),
                )
            });

        Some(
            ListItem::new(format!("pr-{ix}"))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(v_flex().w_full().child(title_label).child(detail_info))
                .tooltip(Tooltip::text(format!("PR #{}", entry.number))),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if self.creating_pr {
            return Some("Type a title and press Enter to create a PR for the current branch".into());
        }
        match &self.load_state {
            PullRequestLoadState::Loading => Some("Loading pull requests…".into()),
            PullRequestLoadState::Error(msg) => Some(msg.clone().into()),
            PullRequestLoadState::Loaded(_) => Some("No pull requests found".into()),
        }
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_0p5()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("copy-url", "Copy URL")
                        .on_click(cx.listener(move |picker, _, window, cx| {
                            cx.stop_propagation();
                            let selected_ix = picker.delegate.selected_index();
                            picker.delegate.copy_url(selected_ix, window, cx);
                        })),
                )
                .child(
                    Button::new("open-in-browser", "Open in Browser")
                        .style(ButtonStyle::Filled)
                        .on_click(cx.listener(move |picker, _, window, cx| {
                            cx.stop_propagation();
                            let selected_ix = picker.delegate.selected_index();
                            picker.delegate.open_in_browser(selected_ix, window, cx);
                            cx.emit(DismissEvent);
                        })),
                )
                .into_any(),
        )
    }
}
