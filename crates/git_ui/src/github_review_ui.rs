use crate::github_review::{
    self, Checkout, CommentDraft, CommentKind, CommentTarget, DiscussionAction, DiscussionComment,
    GitHubClient, GitHubFailure, GitHubRepo, PullRequest,
};
use anyhow::{Context as _, Result, ensure};
use db::kvp::KeyValueStore;
use editor::{Editor, EditorEvent};
use gpui::{ClipboardItem, Entity, EventEmitter, Subscription, Task};
use project::{Project, git_store::Repository};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use ui::prelude::*;
use util::ResultExt as _;

pub(crate) enum GitHubReviewEvent {
    Open { repo: GitHubRepo, pr: PullRequest },
    CommentSelection,
    CommentsLoaded(Vec<github_review::PublishedComment>),
}
impl EventEmitter<GitHubReviewEvent> for GitHubReview {}

#[derive(Default, Serialize, Deserialize)]
struct SavedDrafts {
    repository: Option<String>,
    #[serde(default)]
    pending_actions: BTreeMap<String, DiscussionAction>,
    drafts: BTreeMap<String, CommentDraft>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum DiscussionFilter {
    #[default]
    All,
    Unresolved,
    Resolved,
    Outdated,
}

pub(crate) struct GitHubReview {
    project: Entity<Project>,
    review: Option<gpui::WeakEntity<crate::branch_review::BranchReview>>,
    repository: Option<Entity<Repository>>,
    root: Option<PathBuf>,
    client: GitHubClient,
    query: Entity<Editor>,
    composer: Entity<Editor>,
    choices: Vec<String>,
    saved: SavedDrafts,
    storage_key: Option<String>,
    load_failed: bool,
    selected_repo: Option<GitHubRepo>,
    requests: Vec<github_review::PullRequestSummary>,
    preview: Option<PullRequest>,
    pub checkout: Option<Checkout>,
    discussion: Vec<DiscussionComment>,
    viewer: Option<String>,
    thread_error: Option<String>,
    discussion_filter: DiscussionFilter,
    markdown: BTreeMap<String, (String, Entity<markdown::Markdown>)>,
    expanded_comments: std::collections::BTreeSet<String>,
    preview_markdown: Option<(String, Entity<markdown::Markdown>)>,
    previewing: bool,
    pending_delete: Option<(CommentKind, u64)>,
    target: CommentTarget,
    state: &'static str,
    page: u32,
    has_next: bool,
    browsing: bool,
    pub showing_discussion: bool,
    busy: bool,
    posting: bool,
    detached: bool,
    reconciled: bool,
    generation: u64,
    error: Option<String>,
    task: Option<Task<()>>,
    remotes_task: Option<Task<()>>,
    write_task: Option<Task<()>>,
    copied_prompt: bool,
    copied_prompt_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl GitHubReview {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let query = cx.new(|cx| Editor::single_line(window, cx));
        query.update(cx, |editor, cx| {
            editor.set_placeholder_text("PR number, URL, or title", window, cx)
        });
        let composer = cx.new(|cx| Editor::auto_height(3, 8, window, cx));
        composer.update(cx, |editor, cx| {
            editor.set_placeholder_text("Write a GitHub comment…", window, cx)
        });
        let subscriptions = vec![
            cx.subscribe(&composer, |this, _, event, cx| {
                if matches!(event, EditorEvent::Edited { .. }) {
                    this.save_draft(cx);
                }
            }),
            cx.on_app_quit(|this, _| {
                let write = this.write_task.take();
                async move {
                    if let Some(write) = write {
                        write.await;
                    }
                }
            }),
        ];
        Self {
            project,
            review: None,
            repository: None,
            root: None,
            client: GitHubClient::new(cx.background_executor().clone()),
            query,
            composer,
            choices: Vec::new(),
            saved: SavedDrafts::default(),
            storage_key: None,
            load_failed: false,
            selected_repo: None,
            requests: Vec::new(),
            preview: None,
            checkout: None,
            discussion: Vec::new(),
            viewer: None,
            thread_error: None,
            discussion_filter: DiscussionFilter::All,
            markdown: BTreeMap::new(),
            expanded_comments: Default::default(),
            preview_markdown: None,
            previewing: false,
            pending_delete: None,
            target: CommentTarget::General,
            state: "open",
            page: 1,
            has_next: false,
            browsing: false,
            showing_discussion: false,
            busy: false,
            posting: false,
            detached: true,
            reconciled: false,
            generation: 0,
            error: None,
            task: None,
            remotes_task: None,
            write_task: None,
            copied_prompt: false,
            copied_prompt_task: None,
            _subscriptions: subscriptions,
        }
    }

    pub fn set_repository(
        &mut self,
        repository: Option<Entity<Repository>>,
        cx: &mut Context<Self>,
    ) {
        let root = repository
            .as_ref()
            .map(|repo| repo.read(cx).work_directory_abs_path.to_path_buf());
        if root == self.root {
            self.repository = repository;
            return;
        }
        // Never abandon a request whose server outcome may already be committed.
        if self.posting {
            self.detached = true;
            cx.notify();
            return;
        }
        self.generation += 1;
        self.task = None;
        self.busy = false;
        self.repository = repository;
        self.root = root;
        self.checkout = None;
        self.selected_repo = None;
        self.preview = None;
        self.requests.clear();
        self.discussion.clear();
        self.markdown.clear();
        self.expanded_comments.clear();
        self.pending_delete = None;
        self.viewer = None;
        self.thread_error = None;
        self.reconciled = false;
        self.choices.clear();
        if let Some(repo) = &self.repository {
            let repo = repo.read(cx);
            for url in [&repo.remote_upstream_url, &repo.remote_origin_url]
                .into_iter()
                .flatten()
            {
                if let Some(name) = github_review::repository_from_remote(url) {
                    if !self.choices.contains(&name) {
                        self.choices.push(name);
                    }
                }
            }
        }
        self.storage_key = self.root.as_ref().map(|root| {
            format!(
                "github_review_drafts_v1:{}",
                crate::review_state::digest(&[root.as_os_str().as_encoded_bytes()])
            )
        });
        let restored = self
            .storage_key
            .as_ref()
            .map(|key| {
                KeyValueStore::global(cx).read_kvp(key).and_then(|value| {
                    value
                        .map(|value| serde_json::from_str(&value))
                        .transpose()
                        .map_err(Into::into)
                })
            })
            .transpose();
        match restored {
            Ok(saved) => {
                self.saved = saved.flatten().unwrap_or_default();
                self.load_failed = false;
                self.error = None;
            }
            Err(error) => {
                self.saved = SavedDrafts::default();
                self.load_failed = true;
                self.error = Some(format!("Could not restore comment drafts: {error}"));
            }
        }
        if !self
            .saved
            .repository
            .as_ref()
            .is_some_and(|repo| self.choices.contains(repo))
        {
            self.saved.repository = self.choices.first().cloned();
        }
        self.detached = true;
        if let Some(repository) = &self.repository {
            let remotes = repository.update(cx, |repository, _| repository.remote_urls());
            let root = self.root.clone();
            self.remotes_task = Some(cx.spawn(async move |this, cx| {
                let result = remotes.await;
                this.update(cx, |this, cx| {
                    if this.root != root {
                        return;
                    }
                    match result {
                        Ok(Ok(remotes)) => {
                            let mut remotes: Vec<_> = remotes.into_iter().collect();
                            remotes.sort_by_key(|(name, _)| {
                                (
                                    match name.as_str() {
                                        "upstream" => 0,
                                        "origin" => 1,
                                        _ => 2,
                                    },
                                    name.clone(),
                                )
                            });
                            this.choices.clear();
                            for (_, url) in remotes {
                                if let Some(name) = github_review::repository_from_remote(&url) {
                                    if !this.choices.contains(&name) {
                                        this.choices.push(name);
                                    }
                                }
                            }
                            if !this
                                .saved
                                .repository
                                .as_ref()
                                .is_some_and(|name| this.choices.contains(name))
                            {
                                this.saved.repository = this.choices.first().cloned();
                            }
                        }
                        Ok(Err(error)) => {
                            this.error = Some(format!("Could not load GitHub remotes: {error}"))
                        }
                        Err(error) => {
                            this.error = Some(format!("Could not load GitHub remotes: {error}"))
                        }
                    }
                    cx.notify();
                })
                .log_err();
            }));
        }
        cx.notify();
    }

    pub fn set_review(&mut self, review: gpui::WeakEntity<crate::branch_review::BranchReview>) {
        self.review = Some(review);
    }

    pub fn attach(&mut self, checkout: Checkout, window: &mut Window, cx: &mut Context<Self>) {
        self.save_draft(cx);
        self.generation += 1;
        self.selected_repo = Some(checkout.repository.clone());
        self.preview = Some(checkout.pull_request.clone());
        self.checkout = Some(checkout);
        self.detached = false;
        self.browsing = false;
        self.showing_discussion = false;
        self.discussion.clear();
        self.markdown.clear();
        self.expanded_comments.clear();
        self.pending_delete = None;
        self.viewer = None;
        self.thread_error = None;
        self.reconciled = false;
        self.target = CommentTarget::General;
        let body = self
            .draft()
            .map(|draft| draft.body.clone())
            .unwrap_or_default();
        let uncertain = self.draft().is_some_and(|draft| draft.outcome_unknown);
        self.composer.update(cx, |editor, cx| {
            editor.set_text(body, window, cx);
            editor.set_read_only(uncertain);
        });
        cx.notify();
    }
    pub fn close_review(&mut self, cx: &mut Context<Self>) {
        self.save_draft(cx);
        self.detached = true;
        self.checkout = None;
        self.review = None;
        self.preview = None;
        self.browsing = false;
        self.showing_discussion = false;
        if !self.posting {
            self.generation += 1;
            self.task = None;
            self.busy = false;
        }
        cx.notify();
    }

    pub fn detach(&mut self, cx: &mut Context<Self>) {
        self.detached = true;
        cx.notify();
    }
    pub fn show_browser(&mut self, cx: &mut Context<Self>) {
        self.browsing = true;
        self.showing_discussion = false;
        self.search(cx);
    }
    pub fn show_files(&mut self, cx: &mut Context<Self>) {
        self.browsing = false;
        self.showing_discussion = false;
        cx.notify();
    }
    pub fn is_visible(&self) -> bool {
        self.browsing || self.showing_discussion
    }
    pub fn has_github(&self) -> bool {
        !self.choices.is_empty()
    }
    pub fn matches_repository(&self, repository: &Entity<Repository>, cx: &App) -> bool {
        self.root.as_deref() == Some(repository.read(cx).work_directory_abs_path.as_ref())
    }

    pub fn is_posting(&self) -> bool {
        self.posting
    }

    fn copy_agent_prompt(&mut self, cx: &mut Context<Self>) {
        let Some(checkout) = self.checkout.as_ref() else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new_string(agent_prompt(checkout)));
        self.copied_prompt = true;
        self.copied_prompt_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(2))
                .await;
            this.update(cx, |this, cx| {
                this.copied_prompt = false;
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn draft_key(&self) -> Option<String> {
        let checkout = self.checkout.as_ref()?;
        Some(format!(
            "{}:{}:{}",
            checkout.repository.id,
            checkout.pull_request.number,
            crate::review_state::digest(&[&serde_json::to_vec(&self.target).ok()?])
        ))
    }
    fn draft(&self) -> Option<&CommentDraft> {
        self.draft_key().and_then(|key| self.saved.drafts.get(&key))
    }
    fn save_draft(&mut self, cx: &mut Context<Self>) {
        if self.load_failed {
            return;
        }
        if let Some(key) = self.draft_key() {
            let body = self.composer.read(cx).text(cx);
            let draft = self
                .saved
                .drafts
                .entry(key)
                .or_insert_with(|| CommentDraft {
                    target: self.target.clone(),
                    original_body: None,
                    body: String::new(),
                    outcome_unknown: false,
                });
            draft.body = body;
            self.persist(cx);
        }
    }
    fn persist(&mut self, cx: &mut Context<Self>) {
        if self.load_failed {
            return;
        }
        let Some(key) = self.storage_key.clone() else {
            return;
        };
        let value = match serde_json::to_string(&self.saved) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error.to_string());
                return;
            }
        };
        let database = KeyValueStore::global(cx);
        let previous = self.write_task.take();
        self.write_task = Some(cx.spawn(async move |this, cx| {
            if let Some(previous) = previous {
                previous.await;
            }
            if let Err(error) = database.write_kvp(key, value).await {
                this.update(cx, |this, cx| {
                    this.error = Some(format!("Comment drafts were not saved: {error}"));
                    cx.notify();
                })
                .log_err();
            }
        }));
    }
    pub fn select_target(
        &mut self,
        target: CommentTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.posting {
            return;
        }
        self.save_draft(cx);
        self.target = target;
        self.previewing = false;
        let body = self
            .draft()
            .map(|draft| draft.body.clone())
            .unwrap_or_default();
        let uncertain = self.draft().is_some_and(|draft| draft.outcome_unknown);
        self.composer.update(cx, |editor, cx| {
            editor.set_text(body, window, cx);
            editor.set_read_only(uncertain);
        });
        self.showing_discussion = true;
        self.browsing = false;
        cx.notify();
    }
    pub fn show_error(&mut self, error: String, cx: &mut Context<Self>) {
        self.error = Some(error);
        cx.notify();
    }

    fn search(&mut self, cx: &mut Context<Self>) {
        if self.posting {
            return;
        }
        let Some(name) = self.saved.repository.clone() else {
            return;
        };
        self.generation += 1;
        let generation = self.generation;
        self.busy = true;
        self.error = None;
        self.preview = None;
        let client = self.client.clone();
        let query = self.query.read(cx).text(cx);
        let state = self.state;
        let page = self.page;
        self.task = Some(cx.spawn(async move |this, cx| {
            let result: Result<_> = async {
                let repo = client.repository(&name).await?;
                if let Ok(number) = github_review::pr_number(&query, &repo) {
                    let pr = client.pull_request(&repo, number).await?;
                    return Ok((repo, vec![pr.into()], false));
                }
                ensure!(
                    !query.starts_with("https://"),
                    "Enter a PR URL from the selected GitHub repository"
                );
                let (requests, has_next) = if query.trim().is_empty() {
                    let values = client.pull_requests(&repo, state, page).await?;
                    let next = values.len() == 100;
                    (values.into_iter().map(Into::into).collect(), next)
                } else {
                    client
                        .search_pull_requests(&repo, &query, state, page)
                        .await?
                };
                Ok((repo, requests, has_next))
            }
            .await;
            this.update(cx, |this, cx| {
                if generation != this.generation {
                    return;
                }
                this.busy = false;
                match result {
                    Ok((repo, requests, next)) => {
                        this.selected_repo = Some(repo);
                        this.requests = requests;
                        this.has_next = next;
                    }
                    Err(error) => this.error = Some(format!("{error:#}")),
                }
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn preview_request(&mut self, number: u64, cx: &mut Context<Self>) {
        if self.posting {
            return;
        }
        let Some(repo) = self.selected_repo.clone() else {
            return;
        };
        let client = self.client.clone();
        self.generation += 1;
        let generation = self.generation;
        self.busy = true;
        self.task = Some(cx.spawn(async move |this, cx| {
            let result = client.pull_request(&repo, number).await;
            this.update(cx, |this, cx| {
                if generation != this.generation {
                    return;
                }
                this.busy = false;
                match result {
                    Ok(pr) => {
                        this.preview = Some(pr);
                        this.error = None;
                    }
                    Err(error) => this.error = Some(format!("{error:#}")),
                }
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    pub fn refresh_discussion(&mut self, cx: &mut Context<Self>) {
        if self.posting {
            return;
        }
        let Some(checkout) = self.checkout.clone() else {
            return;
        };
        self.generation += 1;
        let generation = self.generation;
        let root = self.root.clone();
        let client = self.client.clone();
        self.busy = true;
        self.showing_discussion = true;
        self.browsing = false;
        self.task = Some(cx.spawn(async move |this, cx| {
            let result: Result<_> = async {
                let pr = client
                    .pull_request(&checkout.repository, checkout.pull_request.number)
                    .await?;
                let mut comments = client.discussion(&checkout.repository, pr.number).await?;
                let (viewer, thread_error) = match client.viewer().await {
                    Ok(viewer) => (Some(viewer.login), None),
                    Err(error) => (
                        None,
                        Some(format!("Comment permissions unavailable: {error}")),
                    ),
                };
                let thread_error =
                    match client.review_threads(&checkout.repository, pr.number).await {
                        Ok(threads) => {
                            let by_comment: BTreeMap<_, _> = threads
                                .into_iter()
                                .flat_map(|thread| {
                                    let thread = std::sync::Arc::new(thread);
                                    thread
                                        .comments
                                        .iter()
                                        .map(|comment| (comment.database_id, thread.clone()))
                                        .collect::<Vec<_>>()
                                })
                                .collect();
                            for entry in &mut comments {
                                if entry.kind == CommentKind::Inline {
                                    entry.comment.thread =
                                        by_comment.get(&entry.comment.id).cloned();
                                }
                            }
                            thread_error
                        }
                        Err(error) => Some(format!(
                            "Thread state unavailable: {error}. Existing comments remain readable."
                        )),
                    };
                let placement = if let Some(root) = root {
                    github_review::published_comments(
                        cx.background_executor(),
                        &root,
                        &checkout,
                        &comments,
                    )
                    .await
                } else {
                    Ok(Vec::new())
                };
                let (inline, warning) = match placement {
                    Ok(inline) => (inline, None),
                    Err(error) => (
                        Vec::new(),
                        Some(format!(
                            "Discussion refreshed; inline placement is unavailable: {error}"
                        )),
                    ),
                };
                Ok((pr, comments, inline, warning, viewer, thread_error))
            }
            .await;
            this.update(cx, |this, cx| {
                if generation != this.generation {
                    return;
                }
                this.busy = false;
                match result {
                    Ok((pr, comments, inline, warning, viewer, thread_error)) => {
                        this.viewer = viewer;
                        this.thread_error = thread_error;
                        this.markdown.retain(|key, _| {
                            comments.iter().any(|entry| key == &comment_key(entry))
                        });
                        cx.emit(GitHubReviewEvent::CommentsLoaded(inline));
                        this.preview = Some(pr);
                        this.discussion = comments;
                        this.reconciled = true;
                        this.error = warning;
                    }
                    Err(error) => this.error = Some(format!("{error:#}")),
                }
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn action_key(&self) -> Option<String> {
        let checkout = self.checkout.as_ref()?;
        Some(format!(
            "{}:{}",
            checkout.repository.id, checkout.pull_request.number
        ))
    }

    fn start_edit(
        &mut self,
        entry: DiscussionComment,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.posting || self.busy {
            return;
        }
        self.select_target(
            CommentTarget::Edit {
                comment_id: entry.comment.id,
                comment_kind: entry.kind,
            },
            window,
            cx,
        );
        if let Some(key) = self.draft_key() {
            if self
                .saved
                .drafts
                .get(&key)
                .is_none_or(|draft| draft.original_body.is_none())
            {
                let body = entry.comment.body.unwrap_or_default();
                self.saved.drafts.insert(
                    key,
                    CommentDraft {
                        target: self.target.clone(),
                        original_body: Some(body.clone()),
                        body: body.clone(),
                        outcome_unknown: false,
                    },
                );
                self.composer
                    .update(cx, |editor, cx| editor.set_text(body, window, cx));
                self.persist(cx);
            }
        }
    }

    fn run_discussion_action(&mut self, action: DiscussionAction, cx: &mut Context<Self>) {
        if self.busy || self.posting || self.detached || self.load_failed {
            return;
        }
        let (Some(checkout), Some(key), Some(storage_key)) = (
            self.checkout.clone(),
            self.action_key(),
            self.storage_key.clone(),
        ) else {
            return;
        };
        if self.saved.pending_actions.contains_key(&key) {
            return;
        }
        self.saved
            .pending_actions
            .insert(key.clone(), action.clone());
        self.pending_delete = None;
        self.posting = true;
        self.reconciled = false;
        self.persist(cx);
        let persisted = self.write_task.take();
        let value = serde_json::to_string(&self.saved);
        let database = KeyValueStore::global(cx);
        let client = self.client.clone();
        self.task = Some(cx.spawn(async move |this, cx| {
            if let Some(persisted) = persisted {
                persisted.await;
            }
            let result: Result<()> = async {
                database
                    .write_kvp(storage_key, value?)
                    .await
                    .context("Could not save the pending action")?;
                client
                    .discussion_action(&checkout.repository, checkout.pull_request.number, &action)
                    .await
            }
            .await;
            this.update(cx, |this, cx| {
                this.posting = false;
                match result {
                    Ok(()) => {
                        this.saved.pending_actions.remove(&key);
                        this.error = None;
                    }
                    Err(error) => {
                        if !error
                            .downcast_ref::<GitHubFailure>()
                            .is_some_and(|error| error.outcome_unknown)
                        {
                            this.saved.pending_actions.remove(&key);
                        }
                        this.error = Some(format!("{error:#}"));
                    }
                }
                this.persist(cx);
                if this.action_key().as_ref() == Some(&key) && this.error.is_none() {
                    this.refresh_discussion(cx);
                }
                let repository = this.project.read(cx).active_repository(cx);
                this.set_repository(repository, cx);
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn comment_markdown(
        &mut self,
        entry: &DiscussionComment,
        cx: &mut Context<Self>,
    ) -> Entity<markdown::Markdown> {
        let body = entry.comment.body.clone().unwrap_or_default();
        let key = comment_key(entry);
        let languages = self.project.read(cx).languages().clone();
        let cached = self.markdown.entry(key).or_insert_with(|| {
            (
                body.clone(),
                crate::review_markdown::new(&body, languages, cx),
            )
        });
        if cached.0 != body {
            cached.1.update(cx, |markdown, cx| {
                markdown.replace(crate::review_markdown::source(&body), cx)
            });
            cached.0 = body;
        }
        cached.1.clone()
    }

    fn render_discussion(&mut self, window: &Window, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut groups: Vec<Vec<DiscussionComment>> = Vec::new();
        let mut indices = BTreeMap::new();
        for entry in &self.discussion {
            let key = if entry.kind == CommentKind::Inline {
                entry
                    .comment
                    .thread
                    .as_ref()
                    .map(|thread| thread.id.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "inline-{}",
                            entry.comment.in_reply_to_id.unwrap_or(entry.comment.id)
                        )
                    })
            } else {
                comment_key(entry)
            };
            let index = *indices.entry(key).or_insert_with(|| {
                groups.push(Vec::new());
                groups.len() - 1
            });
            groups[index].push(entry.clone());
        }
        let pending = self.busy
            || self.posting
            || self.detached
            || self
                .action_key()
                .is_some_and(|key| self.saved.pending_actions.contains_key(&key));
        groups
            .into_iter()
            .filter_map(|group| {
                let first = group.first()?;
                let thread = first.comment.thread.clone();
                if !match self.discussion_filter {
                    DiscussionFilter::All => true,
                    DiscussionFilter::Unresolved => {
                        thread.as_ref().is_some_and(|thread| !thread.is_resolved)
                    }
                    DiscussionFilter::Resolved => {
                        thread.as_ref().is_some_and(|thread| thread.is_resolved)
                    }
                    DiscussionFilter::Outdated => {
                        thread.as_ref().is_some_and(|thread| thread.is_outdated)
                    }
                } {
                    return None;
                }
                let mut card = v_flex()
                    .gap_1()
                    .p_2()
                    .border_1()
                    .border_color(cx.theme().colors().border);
                if let Some(thread) = thread {
                    let id = thread.id.clone();
                    let resolved = thread.is_resolved;
                    let allowed = if resolved {
                        thread.viewer_can_unresolve
                    } else {
                        thread.viewer_can_resolve
                    };
                    card = card.child(
                        h_flex()
                            .gap_1()
                            .flex_wrap()
                            .child(
                                Label::new(format!(
                                    "{}{}",
                                    if resolved { "Resolved" } else { "Unresolved" },
                                    if thread.is_outdated {
                                        " · Outdated"
                                    } else {
                                        ""
                                    }
                                ))
                                .size(LabelSize::XSmall),
                            )
                            .child(
                                Button::new(
                                    SharedString::from(format!("resolve-{id}")),
                                    if resolved { "Reopen" } else { "Resolve" },
                                )
                                .disabled(pending || !allowed)
                                .on_click(cx.listener(
                                    move |this, _, _, cx| {
                                        this.run_discussion_action(
                                            DiscussionAction::Resolve {
                                                thread_id: id.clone(),
                                                resolved: !resolved,
                                            },
                                            cx,
                                        )
                                    },
                                )),
                            ),
                    );
                }
                for entry in group {
                    let comment = &entry.comment;
                    let key = comment_key(&entry);
                    let expanded = self.expanded_comments.contains(&key);
                    let long = comment
                        .body
                        .as_ref()
                        .is_some_and(|body| body.len() > 500 || body.lines().count() > 8);
                    let own = self
                        .viewer
                        .as_ref()
                        .is_some_and(|viewer| comment.user.login.eq_ignore_ascii_case(viewer));
                    let permissions = comment.thread.as_ref().and_then(|thread| {
                        thread
                            .comments
                            .iter()
                            .find(|item| item.database_id == comment.id)
                    });
                    let can_edit = own
                        && entry.kind != CommentKind::Review
                        && (entry.kind == CommentKind::Conversation
                            || permissions
                                .is_some_and(|p| p.viewer_did_author && p.viewer_can_update));
                    let can_delete = own
                        && entry.kind != CommentKind::Review
                        && (entry.kind == CommentKind::Conversation
                            || permissions
                                .is_some_and(|p| p.viewer_did_author && p.viewer_can_delete));
                    let url = self.checkout.as_ref().map(|checkout| {
                        format!(
                            "{}#{}{}",
                            checkout.pull_request.url(&checkout.repository),
                            match entry.kind {
                                CommentKind::Conversation => "issuecomment-",
                                CommentKind::Review => "pullrequestreview-",
                                CommentKind::Inline => "discussion_r",
                            },
                            comment.id
                        )
                    });
                    let markdown = self.comment_markdown(&entry, cx);
                    let mut content = v_flex()
                        .min_w_0()
                        .gap_1()
                        .child(
                            Label::new(format!("{} · {:?}", comment.user.login, entry.kind))
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                        .when_some(comment.path.clone(), |view, path| {
                            view.child(
                                Label::new(match comment.line.or(comment.original_line) {
                                    Some(line) => format!(
                                        "{} · {}:{}{}",
                                        path,
                                        match comment.side {
                                            Some(github_review::DiffSide::Left) => "Left",
                                            Some(github_review::DiffSide::Right) => "Right",
                                            None => "Line",
                                        },
                                        line,
                                        if comment.line.is_none() {
                                            " (outdated)"
                                        } else {
                                            ""
                                        }
                                    ),
                                    None => format!("{path} · File comment"),
                                })
                                .size(LabelSize::XSmall),
                            )
                        })
                        .child(
                            div()
                                .min_w_0()
                                .when(long && !expanded, |view| {
                                    view.max_h(px(160.)).overflow_hidden()
                                })
                                .child(crate::review_markdown::render(markdown, window, cx)),
                        )
                        .when(long, |view| {
                            view.child(
                                Button::new(
                                    SharedString::from(format!("expand-{key}")),
                                    if expanded { "Show less" } else { "Show more" },
                                )
                                .on_click(cx.listener(
                                    move |this, _, _, cx| {
                                        if !this.expanded_comments.remove(&key) {
                                            this.expanded_comments.insert(key.clone());
                                        }
                                        cx.notify();
                                    },
                                )),
                            )
                        });
                    if comment.line.is_none() && entry.kind == CommentKind::Inline {
                        if let Some(hunk) = &comment.diff_hunk {
                            content = content.child(
                                Label::new(hunk.clone())
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            );
                        }
                    }
                    let id = comment.id;
                    let kind = entry.kind;
                    let edit = entry.clone();
                    let reply_id = comment.in_reply_to_id.unwrap_or(id);
                    content = content.child(
                        h_flex()
                            .gap_1()
                            .flex_wrap()
                            .when_some(url, |view, url| {
                                view.child(
                                    Button::new(("github-comment-link", id as usize), "GitHub ↗")
                                        .on_click(move |_, _, cx| cx.open_url(&url)),
                                )
                            })
                            .when(kind == CommentKind::Inline, |view| {
                                view.child(
                                    Button::new(("reply", id as usize), "Reply")
                                        .disabled(
                                            pending
                                                || comment
                                                    .thread
                                                    .as_ref()
                                                    .is_none_or(|thread| !thread.viewer_can_reply),
                                        )
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            this.select_target(
                                                CommentTarget::Reply {
                                                    comment_id: reply_id,
                                                },
                                                window,
                                                cx,
                                            )
                                        })),
                                )
                            })
                            .when(can_edit, |view| {
                                view.child(
                                    Button::new(("edit-comment", id as usize), "Edit")
                                        .disabled(pending)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            this.start_edit(edit.clone(), window, cx)
                                        })),
                                )
                            })
                            .when(can_delete, |view| {
                                view.child(
                                    Button::new(("delete-comment", id as usize), "Delete")
                                        .disabled(pending)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.pending_delete = Some((kind, id));
                                            cx.notify();
                                        })),
                                )
                            }),
                    );
                    if self.pending_delete == Some((kind, id)) {
                        content = content
                            .child(
                                Label::new("Permanently delete this comment from GitHub?")
                                    .size(LabelSize::Small)
                                    .color(Color::Warning),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Button::new(
                                            ("confirm-delete", id as usize),
                                            "Delete comment",
                                        )
                                        .disabled(pending)
                                        .on_click(
                                            cx.listener(move |this, _, _, cx| {
                                                this.run_discussion_action(
                                                    DiscussionAction::Delete {
                                                        comment_id: id,
                                                        comment_kind: kind,
                                                    },
                                                    cx,
                                                )
                                            }),
                                        ),
                                    )
                                    .child(
                                        Button::new(("cancel-delete", id as usize), "Cancel")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.pending_delete = None;
                                                cx.notify();
                                            })),
                                    ),
                            );
                    }
                    card = card.child(content);
                }
                Some(card.into_any_element())
            })
            .collect()
    }

    fn post(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.posting
            || self.busy
            || self.detached
            || self.load_failed
            || self.draft().is_some_and(|draft| draft.outcome_unknown)
            || self
                .action_key()
                .is_some_and(|key| self.saved.pending_actions.contains_key(&key))
        {
            return;
        }
        let (Some(checkout), Some(root), Some(key)) =
            (self.checkout.clone(), self.root.clone(), self.draft_key())
        else {
            return;
        };
        self.save_draft(cx);
        let body = self.composer.read(cx).text(cx);
        if body.trim().is_empty() {
            return;
        }
        let target = self.target.clone();
        let original_body = self.draft().and_then(|draft| draft.original_body.clone());
        let client = self.client.clone();
        let project = self.project.clone();
        let review = self.review.clone();
        let repository = self.repository.clone();
        self.posting = true;
        self.reconciled = false;
        self.composer
            .update(cx, |editor, _| editor.set_read_only(true));
        self.error = None;
        // Persist the uncertain outcome before sending. A process crash must not offer a blind retry.
        if let Some(draft) = self.saved.drafts.get_mut(&key) {
            draft.outcome_unknown = true;
        }
        self.persist(cx);
        let persisted = self.write_task.take();
        let durable = self
            .storage_key
            .clone()
            .zip(serde_json::to_string(&self.saved).ok());
        let database = KeyValueStore::global(cx);
        self.task = Some(cx.spawn_in(window, async move |this, cx| {
            if let Some(persisted) = persisted {
                persisted.await;
            }
            let result: Result<_> =
                async {
                    let (storage_key, value) =
                        durable.context("Draft storage is unavailable; posting is disabled")?;
                    database
                        .write_kvp(storage_key, value)
                        .await
                        .context("Could not save the draft before posting")?;
                    let pr = client
                        .pull_request(&checkout.repository, checkout.pull_request.number)
                        .await?;
                    let effective = cx.update(|_, cx| -> Result<Option<(Option<String>, Option<String>)>> {
                    let repository = repository.as_ref().context("Repository is unavailable")?;
                    ensure!(
                        project.read(cx).active_repository(cx).as_ref() == Some(repository)
                            && repository
                                .read(cx)
                                .branch
                                .as_ref()
                                .is_some_and(|branch| branch.name() == checkout.branch),
                        "The active checkout changed. This draft still belongs to the original PR."
                    );
                    if let CommentTarget::Inline { path, .. } = &target {
                        let comparison = review.as_ref().context("The PR diff is unavailable")?.read_with(cx, |review, cx| review.comparison_for_path(path, cx))?.context("The selected file is no longer in this PR comparison")?;
                        return Ok(Some(comparison));
                    }
                    Ok(None)
                })??;
                    github_review::validate_inline(
                        cx.background_executor(),
                        &root,
                        &pr,
                        &target,
                        effective.as_ref().and_then(|(current, _)| current.as_deref()),
                        effective.as_ref().and_then(|(_, base)| base.as_deref()),
                    )
                    .await?;
                    client.validate_published_target(&checkout.repository, &pr, &target).await?;
                    cx.update(|_, cx| -> Result<()> {
                        let repository =
                            repository.as_ref().context("Repository is unavailable")?;
                        ensure!(
                            project.read(cx).active_repository(cx).as_ref() == Some(repository)
                                && repository
                                    .read(cx)
                                    .branch
                                    .as_ref()
                                    .is_some_and(|branch| branch.name() == checkout.branch),
                            "The checkout changed while validating this comment"
                        );
                        if let CommentTarget::Inline { path, .. } = &target {
                            let now = review.as_ref().context("The PR diff is unavailable")?.read_with(cx, |review, cx| review.comparison_for_path(path, cx))?;
                            ensure!(
                                now == effective,
                                "The file changed while validating the comment. Your draft is kept."
                            );
                        }
                        Ok(())
                    })??;
                    if let CommentTarget::Edit { comment_id, comment_kind } = target {
                        client.update_comment(&checkout.repository, comment_kind, comment_id, original_body.as_deref().context("Original comment unavailable; reopen the edit")?, &body).await
                    } else {
                        client.post(&checkout.repository, &pr, &target, &body).await
                    }
                }
                .await;
            this.update_in(cx, |this, window, cx| {
                this.posting = false;
                this.composer
                    .update(cx, |editor, _| editor.set_read_only(false));
                match result {
                    Ok(comment) => {
                        if let CommentTarget::Edit { comment_id, comment_kind } = target {
                            this.discussion.retain(|entry| entry.kind != comment_kind || entry.comment.id != comment_id);
                        }
                        this.discussion.push(DiscussionComment {
                            kind: match target {
                                CommentTarget::General => CommentKind::Conversation,
                                CommentTarget::Edit { comment_kind, .. } => comment_kind,
                                _ => CommentKind::Inline,
                            },
                            comment,
                        });
                        this.saved.drafts.remove(&key);
                        this.composer
                            .update(cx, |editor, cx| editor.set_text("", window, cx));
                        this.target = CommentTarget::General;
                        this.refresh_discussion(cx);
                    }
                    Err(error) => {
                        let unknown = error
                            .downcast_ref::<GitHubFailure>()
                            .is_some_and(|error| error.outcome_unknown);
                        if let Some(draft) = this.saved.drafts.get_mut(&key) {
                            draft.outcome_unknown = unknown;
                        }
                        this.composer
                            .update(cx, |editor, _| editor.set_read_only(unknown));
                        this.error = Some(format!("{error:#}"));
                    }
                }
                this.persist(cx);
                let repository = this.project.read(cx).active_repository(cx);
                this.set_repository(repository, cx);
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }
}

impl Render for GitHubReview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let discussion = self.render_discussion(window, cx);
        let body = self.composer.read(cx).text(cx);
        let languages = self.project.read(cx).languages().clone();
        let preview = self.preview_markdown.get_or_insert_with(|| {
            (
                body.clone(),
                crate::review_markdown::new(&body, languages, cx),
            )
        });
        if preview.0 != body {
            preview.1.update(cx, |markdown, cx| {
                markdown.replace(crate::review_markdown::source(&body), cx)
            });
            preview.0 = body;
        }
        let preview = preview.1.clone();
        let unknown_action = self
            .action_key()
            .is_some_and(|key| self.saved.pending_actions.contains_key(&key));
        let changed_edit = if let CommentTarget::Edit {
            comment_id,
            comment_kind,
        } = self.target
        {
            self.discussion
                .iter()
                .find(|entry| entry.kind == comment_kind && entry.comment.id == comment_id)
                .and_then(|entry| entry.comment.body.clone())
                .filter(|body| {
                    self.draft().and_then(|draft| draft.original_body.as_ref()) != Some(body)
                })
        } else {
            None
        };
        let pending = self.busy || self.posting;
        let unknown = self.draft().is_some_and(|draft| draft.outcome_unknown);
        v_flex().id("github-review-content").size_full().min_h_0().gap_2().p_2().overflow_y_scroll()
            .when_some(self.error.clone(), |view, error| view.child(Label::new(error).size(LabelSize::Small).color(Color::Error)))
            .when(pending, |view| view.child(Label::new(if self.posting { "Posting to GitHub…" } else { "Loading GitHub…" }).size(LabelSize::Small)))
            .map(|view| if self.browsing {
                view.children(self.choices.clone().into_iter().enumerate().map(|(index, name)| {
                    Button::new(("github-repository", index), name.clone()).toggle_state(self.saved.repository.as_ref() == Some(&name)).disabled(pending)
                        .on_click(cx.listener(move |this, _, _, cx| { this.saved.repository = Some(name.clone()); this.page = 1; this.persist(cx); this.search(cx); }))
                }))
                .child(div().p_1().border_1().border_color(cx.theme().colors().border).child(self.query.clone()))
                .child(h_flex().gap_1().child(Button::new("github-search", "Search").disabled(pending).on_click(cx.listener(|this, _, _, cx| { this.page = 1; this.search(cx); })))
                    .children(["open", "closed", "all"].into_iter().map(|state| Button::new(state, state).toggle_state(self.state == state).disabled(pending).on_click(cx.listener(move |this, _, _, cx| { this.state = state; this.page = 1; this.search(cx); })))))
                .when_some(self.preview.clone(), |view, pr| {
                    let repo = self.selected_repo.clone();
                    view.child(v_flex().gap_1().p_2().border_1().border_color(cx.theme().colors().border)
                        .child(Label::new(format!("#{} {}", pr.number, pr.title)).size(LabelSize::Small))
                        .child(Label::new(format!("{} · {} → {} · {}", pr.user.login, pr.head.branch, pr.base.branch, pr.state)).size(LabelSize::XSmall))
                        .child(Label::new(pr.body.clone().unwrap_or_default()).size(LabelSize::Small).line_clamp(6))
                        .child(Button::new("open-github-pr", "Open PR in this checkout").disabled(pending).on_click(cx.listener(move |_, _, _, cx| {
                            if let Some(repo) = &repo { cx.emit(GitHubReviewEvent::Open { repo: repo.clone(), pr: pr.clone() }); }
                        }))))
                })
                .children(self.requests.clone().into_iter().map(|pr| Button::new(("github-pr", pr.number as usize), format!("#{} {}", pr.number, pr.title))
                    .full_width().disabled(pending).on_click(cx.listener(move |this, _, _, cx| { this.preview_request(pr.number, cx); }))))
                .when(!self.query.read(cx).text(cx).trim().is_empty(), |view| view.child(Label::new("Title search: up to 1,000 matches").size(LabelSize::XSmall).color(Color::Muted)))
                .child(h_flex().gap_1().child(Button::new("previous-pr-page", "Previous").disabled(pending || self.page <= 1).on_click(cx.listener(|this, _, _, cx| { this.page -= 1; this.search(cx); })))
                    .child(Label::new(format!("Page {}", self.page)).size(LabelSize::Small))
                    .child(Button::new("next-pr-page", "Next").disabled(pending || !self.has_next).on_click(cx.listener(|this, _, _, cx| { this.page += 1; this.search(cx); }))))
            } else {
                view.when_some(self.checkout.clone(), |view, checkout| {
                    let update = self.preview.clone().filter(|pr| pr.head.sha != checkout.pull_request.head.sha || pr.base.sha != checkout.pull_request.base.sha);
                    view.child(Label::new(format!("#{} {}", checkout.pull_request.number, checkout.pull_request.title)).size(LabelSize::Small))
                        .child(h_flex().gap_1().flex_wrap().child(Button::new("refresh-github-discussion", "Refresh").disabled(pending).on_click(cx.listener(|this, _, _, cx| this.refresh_discussion(cx))))
                            .child(Button::new("copy-agent-prompt", if self.copied_prompt { "Copied" } else { "Copy Agent Prompt" }).on_click(cx.listener(|this, _, _, cx| this.copy_agent_prompt(cx))))
                            .child(Button::new("open-pr-in-browser", "GitHub ↗").on_click(move |_, _, cx| cx.open_url(&checkout.pull_request.url(&checkout.repository)))))
                        .when_some(update, |view, pr| { let repo = self.checkout.as_ref().map(|checkout| checkout.repository.clone());
                            view.child(Label::new("A newer PR revision is available. Your checkout has not changed.").size(LabelSize::Small))
                                .child(Button::new("update-pr-checkout", "Update checkout").disabled(pending || self.detached).on_click(cx.listener(move |_, _, _, cx| { if let Some(repo) = &repo { cx.emit(GitHubReviewEvent::Open { repo: repo.clone(), pr: pr.clone() }); } })))
                        })
                })
                .when_some(self.thread_error.clone(), |view, error| view.child(Label::new(error).size(LabelSize::Small).color(Color::Warning)))
                .child(h_flex().gap_1().flex_wrap().children([
                    ("all-discussion", "All", DiscussionFilter::All), ("unresolved-discussion", "Unresolved", DiscussionFilter::Unresolved),
                    ("resolved-discussion", "Resolved", DiscussionFilter::Resolved), ("outdated-discussion", "Outdated", DiscussionFilter::Outdated),
                ].into_iter().map(|(id, label, filter)| Button::new(id, label).toggle_state(self.discussion_filter == filter).on_click(cx.listener(move |this, _, _, cx| { this.discussion_filter = filter; cx.notify(); })))))
                .when(unknown_action, |view| view.child(Label::new("A thread action may have succeeded. Refresh and inspect GitHub before retrying.").size(LabelSize::Small).color(Color::Warning))
                    .child(Button::new("clear-unknown-action", "I checked GitHub; allow another action").disabled(pending || !self.reconciled).on_click(cx.listener(|this, _, _, cx| {
                        if let Some(key) = this.action_key() { this.saved.pending_actions.remove(&key); } this.persist(cx); cx.notify();
                    }))))
                .children(discussion)
                .children(self.checkout.as_ref().map(|checkout| format!("{}:{}:", checkout.repository.id, checkout.pull_request.number)).into_iter().flat_map(|prefix| {
                    self.saved.drafts.iter().filter(move |(key, draft)| key.starts_with(&prefix) && (!draft.body.is_empty() || draft.outcome_unknown))
                }).enumerate().map(|(index, (_, draft))| {
                    let target = draft.target.clone();
                    Button::new(("saved-comment-draft", index), format!("Draft: {}", target_label(&target))).disabled(pending)
                        .on_click(cx.listener(move |this, _, window, cx| this.select_target(target.clone(), window, cx)))
                }))
                .when(self.checkout.is_some(), |view| view.child(v_flex().gap_2()
                    .child(h_flex().gap_1().child(Button::new("general-pr-comment", "General").disabled(pending).on_click(cx.listener(|this, _, window, cx| this.select_target(CommentTarget::General, window, cx))))
                        .child(Button::new("inline-pr-comment", "Selected lines").disabled(pending || self.detached).on_click(cx.listener(|_, _, _, cx| cx.emit(GitHubReviewEvent::CommentSelection)))))
                    .child(Label::new(target_label(&self.target)).size(LabelSize::XSmall))
                    .when_some(changed_edit, |view, latest| view
                        .child(Label::new("This comment changed on GitHub. Compare the discussion above with your draft before replacing it.").size(LabelSize::Small).color(Color::Warning))
                        .child(Button::new("accept-latest-edit-base", "Keep draft; use refreshed comment as base").disabled(pending || unknown).on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(key) = this.draft_key() { if let Some(draft) = this.saved.drafts.get_mut(&key) { draft.original_body = Some(latest.clone()); } }
                            this.persist(cx); cx.notify();
                        }))))
                    .child(h_flex().gap_1()
                        .child(Button::new("write-comment", "Write").toggle_state(!self.previewing).on_click(cx.listener(|this, _, _, cx| { this.previewing = false; cx.notify(); })))
                        .child(Button::new("preview-comment", "Preview").toggle_state(self.previewing).on_click(cx.listener(|this, _, _, cx| { this.previewing = true; cx.notify(); }))))
                    .child(div().min_w_0().border_1().border_color(cx.theme().colors().border).p_2().map(|view| if self.previewing { view.child(crate::review_markdown::render(preview, window, cx)) } else { view.child(self.composer.clone()) }))
                    .when(self.detached, |view| view.child(Label::new("Review detached from the checkout. Drafts are kept; reopen the PR to post.").size(LabelSize::Small).color(Color::Warning)))
                    .when(unknown, |view| view.child(Label::new("The last post may have succeeded. Refresh and inspect the discussion before retrying.").size(LabelSize::Small).color(Color::Warning))
                        .child(Button::new("confirm-comment-retry", "I checked GitHub; allow retry").disabled(pending || !self.reconciled).on_click(cx.listener(|this, _, _, cx| {
                            if let Some(key) = this.draft_key() { if let Some(draft) = this.saved.drafts.get_mut(&key) { draft.outcome_unknown = false; } }
                            this.composer.update(cx, |editor, _| editor.set_read_only(false));
                            this.persist(cx); cx.notify();
                        }))))
                    .child(Button::new("post-github-comment", if matches!(self.target, CommentTarget::Edit { .. }) { "Save changes" } else { "Post to GitHub" }).disabled(pending || self.detached || unknown || unknown_action || self.load_failed).on_click(cx.listener(|this, _, window, cx| this.post(window, cx))))))
            })
    }
}

fn agent_prompt(checkout: &Checkout) -> String {
    let pull_request = &checkout.pull_request;
    format!(
        "Address my outstanding review feedback on {url} in the current checkout.\n\n\
Repository: {repository}\n\
PR: #{number}\n\
Checkout branch: {branch}\n\
Reviewed base revision: {base_sha}\n\
Reviewed head revision: {head_sha}\n\n\
Read and follow every applicable repository instruction file before editing. Use the authenticated `gh` CLI to verify the repository and PR, identify the current GitHub login, and query GitHub directly for review threads, inline review comments, submitted review feedback, and PR conversation comments authored by that login. Address every unresolved thread and every still-applicable code-change request from that feedback. Re-query GitHub if the PR revision or thread state may have changed.\n\n\
Work only in the current checkout. Preserve unrelated changes. Do not switch branches, reset, clean, discard work, post or edit GitHub comments, resolve or reopen threads, commit, or push. Make the requested code changes, run the relevant tests and checks, and finish with a concise summary of changes, validation, and any feedback that remains blocked or ambiguous. Do not create a feedback packet or verification-receipt file.",
        url = pull_request.url(&checkout.repository),
        repository = checkout.repository.full_name,
        number = pull_request.number,
        branch = checkout.branch,
        base_sha = pull_request.base.sha,
        head_sha = pull_request.head.sha,
    )
}

fn comment_key(entry: &DiscussionComment) -> String {
    format!("{:?}:{}", entry.kind, entry.comment.id)
}

fn target_label(target: &CommentTarget) -> String {
    match target {
        CommentTarget::General => "PR conversation".into(),
        CommentTarget::Edit { comment_id, .. } => format!("Edit comment {comment_id}"),
        CommentTarget::Reply { comment_id } => format!("Reply to thread {comment_id}"),
        CommentTarget::Inline {
            path,
            side,
            start_line,
            line,
            ..
        } => format!("{path} {side:?} {start_line}-{line}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn checkout(number: u64) -> Checkout {
        let repo = GitHubRepo {
            id: 42,
            full_name: "owner/project".into(),
        };
        Checkout {
            repository: repo.clone(),
            branch: "feature".into(),
            base_ref: "main".into(),
            pull_request: PullRequest {
                number,
                title: "Fixture".into(),
                body: None,
                user: github_review::GitHubUser {
                    login: "author".into(),
                },
                state: "open".into(),
                merged_at: None,
                head: github_review::PullRequestRef {
                    branch: "feature".into(),
                    sha: "a".repeat(40),
                    repo: Some(repo.clone()),
                },
                base: github_review::PullRequestRef {
                    branch: "main".into(),
                    sha: "b".repeat(40),
                    repo: Some(repo),
                },
            },
        }
    }

    #[test]
    fn copied_agent_prompt_uses_trusted_pr_metadata_and_safe_scope() {
        let mut checkout = checkout(17);
        checkout.pull_request.user.login = "untrusted-login-42".into();
        let prompt = agent_prompt(&checkout);
        for expected in [
            "https://github.com/owner/project/pull/17",
            "Repository: owner/project",
            "Checkout branch: feature",
            &format!("Reviewed head revision: {}", "a".repeat(40)),
            "identify the current GitHub login",
            "authored by that login",
            "Do not switch branches",
            "Do not create a feedback packet",
        ] {
            assert!(prompt.contains(expected), "missing {expected:?}");
        }
        assert!(!prompt.contains("Fixture"));
        assert!(!prompt.contains("untrusted-login-42"));
    }

    #[gpui::test]
    async fn editing_and_thread_actions_keep_durable_pr_scoped_state(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"a.txt":"base"}))
            .await;
        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (view, cx) = cx.add_window_view(|window, cx| GitHubReview::new(project, window, cx));
        view.update_in(cx, |view, window, cx| {
            view.storage_key = Some("test-review-edit-drafts".into());
            view.attach(checkout(1), window, cx);
            view.copy_agent_prompt(cx);
            assert!(
                cx.read_from_clipboard()
                    .and_then(|item| item.text())
                    .is_some_and(|prompt| prompt.contains("owner/project/pull/1"))
            );
            let entry = DiscussionComment {
                kind: CommentKind::Inline,
                comment: serde_json::from_value(json!({"id":7,"body":"**Original**","user":{"login":"author"},"thread":{"id":"t1","isResolved":true,"isOutdated":true,"viewerCanResolve":false,"viewerCanUnresolve":true,"viewerCanReply":true,"comments":[]}})).unwrap(),
            };
            let mut reply = entry.clone();
            reply.comment.id = 8;
            reply.comment.in_reply_to_id = Some(7);
            view.discussion = vec![entry.clone(), reply];
            assert_eq!(view.render_discussion(window, cx).len(), 1);
            view.discussion_filter = DiscussionFilter::Unresolved;
            assert!(view.render_discussion(window, cx).is_empty());
            view.discussion_filter = DiscussionFilter::Outdated;
            assert_eq!(view.render_discussion(window, cx).len(), 1);
            view.start_edit(entry, window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "**Original**");
            assert_eq!(view.draft().unwrap().original_body.as_deref(), Some("**Original**"));
            view.composer.update(cx, |editor, cx| editor.set_text("Updated draft", window, cx));
            view.save_draft(cx);
            let action_key = view.action_key().unwrap();
            view.saved.pending_actions.insert(action_key, DiscussionAction::Resolve { thread_id:"t1".into(), resolved:false });
            view.attach(checkout(2), window, cx);
            view.select_target(CommentTarget::Edit { comment_id:7, comment_kind:CommentKind::Inline }, window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "");
            view.attach(checkout(1), window, cx);
            view.select_target(CommentTarget::Edit { comment_id:7, comment_kind:CommentKind::Inline }, window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "Updated draft");
            assert_eq!(view.draft().unwrap().original_body.as_deref(), Some("**Original**"));
            view.run_discussion_action(DiscussionAction::Resolve { thread_id:"t1".into(), resolved:false }, cx);
            assert!(!view.posting, "An uncertain action must not be resubmitted");
            view.persist(cx);
        });
        cx.run_until_parked();
        if let Some(write) = view.update(cx, |view, _| view.write_task.take()) {
            write.await;
        }
        view.read_with(cx, |view, cx| {
            let saved: SavedDrafts = serde_json::from_str(
                &KeyValueStore::global(cx)
                    .read_kvp("test-review-edit-drafts")
                    .unwrap()
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(saved.pending_actions.len(), 1);
            assert_eq!(
                saved.drafts[&view.draft_key().unwrap()].body,
                "Updated draft"
            );
        });
    }

    #[gpui::test]
    async fn drafts_survive_pr_switches_recreation_and_uncertain_outcomes(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({"a.txt":"base"}))
            .await;
        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        let (view, cx) =
            cx.add_window_view(|window, cx| GitHubReview::new(project.clone(), window, cx));
        view.update_in(cx, |view, window, cx| {
            view.storage_key = Some("test-github-drafts".into());
            view.attach(checkout(1), window, cx);
            view.composer.update(cx, |editor, cx| {
                editor.set_text("first PR draft", window, cx)
            });
            view.save_draft(cx);
            view.attach(checkout(2), window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "");
            view.composer.update(cx, |editor, cx| {
                editor.set_text("second PR draft", window, cx)
            });
            view.save_draft(cx);
            view.attach(checkout(1), window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "first PR draft");
            let key = view.draft_key().unwrap();
            view.saved.drafts.get_mut(&key).unwrap().outcome_unknown = true;
            view.persist(cx);
        });
        cx.run_until_parked();
        let write = view.update(cx, |view, _| view.write_task.take()).unwrap();
        write.await;
        let restored: SavedDrafts = cx.update(|_, cx| {
            serde_json::from_str(
                &KeyValueStore::global(cx)
                    .read_kvp("test-github-drafts")
                    .unwrap()
                    .unwrap(),
            )
            .unwrap()
        });
        assert_eq!(restored.drafts.len(), 2);
        let recreated = cx.update(|window, cx| cx.new(|cx| GitHubReview::new(project, window, cx)));
        recreated.update_in(cx, |view, window, cx| {
            view.saved = restored;
            view.attach(checkout(1), window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "first PR draft");
            assert!(view.draft().unwrap().outcome_unknown);
            view.post(window, cx);
            assert!(
                !view.posting,
                "Uncertain posts must not be automatically retried"
            );
            view.attach(checkout(2), window, cx);
            assert_eq!(view.composer.read(cx).text(cx), "second PR draft");
        });
    }
}
