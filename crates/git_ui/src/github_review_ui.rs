use crate::github_review::{
    self, Checkout, CommentDraft, CommentKind, CommentTarget, DiscussionComment, GitHubClient,
    GitHubFailure, GitHubRepo, PullRequest,
};
use anyhow::{Context as _, Result, ensure};
use db::kvp::KeyValueStore;
use editor::{Editor, EditorEvent};
use gpui::{Entity, EventEmitter, Subscription, Task};
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
    drafts: BTreeMap<String, CommentDraft>,
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
                let comments = client.discussion(&checkout.repository, pr.number).await?;
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
                Ok((pr, comments, inline, warning))
            }
            .await;
            this.update(cx, |this, cx| {
                if generation != this.generation {
                    return;
                }
                this.busy = false;
                match result {
                    Ok((pr, comments, inline, warning)) => {
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

    fn post(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.posting
            || self.busy
            || self.detached
            || self.load_failed
            || self.draft().is_some_and(|draft| draft.outcome_unknown)
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
                    client.post(&checkout.repository, &pr, &target, &body).await
                }
                .await;
            this.update_in(cx, |this, window, cx| {
                this.posting = false;
                this.composer
                    .update(cx, |editor, _| editor.set_read_only(false));
                match result {
                    Ok(comment) => {
                        this.discussion.push(DiscussionComment {
                            kind: if matches!(target, CommentTarget::General) {
                                CommentKind::Conversation
                            } else {
                                CommentKind::Inline
                            },
                            comment,
                        });
                        this.saved.drafts.remove(&key);
                        this.composer
                            .update(cx, |editor, cx| editor.set_text("", window, cx));
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        .child(h_flex().gap_1().child(Button::new("refresh-github-discussion", "Refresh").disabled(pending).on_click(cx.listener(|this, _, _, cx| this.refresh_discussion(cx))))
                            .child(Button::new("open-pr-in-browser", "GitHub ↗").on_click(move |_, _, cx| cx.open_url(&checkout.pull_request.url(&checkout.repository)))))
                        .when_some(update, |view, pr| { let repo = self.checkout.as_ref().map(|checkout| checkout.repository.clone());
                            view.child(Label::new("A newer PR revision is available. Your checkout has not changed.").size(LabelSize::Small))
                                .child(Button::new("update-pr-checkout", "Update checkout").disabled(pending || self.detached).on_click(cx.listener(move |_, _, _, cx| { if let Some(repo) = &repo { cx.emit(GitHubReviewEvent::Open { repo: repo.clone(), pr: pr.clone() }); } })))
                        })
                })
                .children(self.discussion.iter().map(|entry| {
                    let comment = &entry.comment;
                    let id = comment.in_reply_to_id.unwrap_or(comment.id);
                    let url = self.checkout.as_ref().map(|checkout| format!("{}#{}{}", checkout.pull_request.url(&checkout.repository), match entry.kind { CommentKind::Conversation => "issuecomment-", CommentKind::Review => "pullrequestreview-", CommentKind::Inline => "discussion_r" }, comment.id));
                    v_flex().gap_1().p_2().border_b_1().border_color(cx.theme().colors().border)
                        .child(Label::new(format!("{} · {:?}", comment.user.login, entry.kind)).size(LabelSize::XSmall).color(Color::Muted))
                        .when_some(comment.path.clone(), |view, path| view.child(Label::new(format!("{} · {}:{}{}", path, comment.side.map(|side| match side { github_review::DiffSide::Left => "Left", github_review::DiffSide::Right => "Right" }).unwrap_or("Line"), comment.line.or(comment.original_line).unwrap_or(0), if comment.line.is_none() { " (outdated)" } else { "" })).size(LabelSize::XSmall)))
                        .child(Label::new(comment.body.clone().unwrap_or_default()).size(LabelSize::Small))
                        .when(comment.line.is_none(), |view| view.when_some(comment.diff_hunk.clone(), |view, hunk| view.child(Label::new(hunk).size(LabelSize::XSmall).color(Color::Muted).line_clamp(8))))
                        .when_some(url, |view, url| view.child(Button::new(("github-comment-link", comment.id as usize), "View on GitHub ↗").on_click(move |_, _, cx| cx.open_url(&url))))
                        .when(entry.kind == CommentKind::Inline, |view| view.child(Button::new(("reply-to-comment", comment.id as usize), "Reply").disabled(pending || self.detached).on_click(cx.listener(move |this, _, window, cx| this.select_target(CommentTarget::Reply { comment_id: id }, window, cx)))))
                }))
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
                    .child(div().border_1().border_color(cx.theme().colors().border).p_2().child(self.composer.clone()))
                    .when(self.detached, |view| view.child(Label::new("Review detached from the checkout. Drafts are kept; reopen the PR to post.").size(LabelSize::Small).color(Color::Warning)))
                    .when(unknown, |view| view.child(Label::new("The last post may have succeeded. Refresh and inspect the discussion before retrying.").size(LabelSize::Small).color(Color::Warning))
                        .child(Button::new("confirm-comment-retry", "I checked GitHub; allow retry").disabled(pending || !self.reconciled).on_click(cx.listener(|this, _, _, cx| {
                            if let Some(key) = this.draft_key() { if let Some(draft) = this.saved.drafts.get_mut(&key) { draft.outcome_unknown = false; } }
                            this.composer.update(cx, |editor, _| editor.set_read_only(false));
                            this.persist(cx); cx.notify();
                        }))))
                    .child(Button::new("post-github-comment", "Post to GitHub").disabled(pending || self.detached || unknown || self.load_failed).on_click(cx.listener(|this, _, window, cx| this.post(window, cx))))))
            })
    }
}

fn target_label(target: &CommentTarget) -> String {
    match target {
        CommentTarget::General => "PR conversation".into(),
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
