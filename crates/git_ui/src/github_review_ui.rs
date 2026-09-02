use crate::github_review::{
    self, Checkout, CommentDraft, CommentKind, CommentTarget, DiscussionAction, DiscussionComment,
    GitHubClient, GitHubFailure, GitHubRepo, PullRequest,
};
use crate::review_provider::{
    ReviewBackend, ReviewHeader, ReviewProviderIdentity, ReviewRequestSummary,
};
use anyhow::{Context as _, Result, ensure};
use db::kvp::KeyValueStore;
use editor::{
    Editor, EditorEvent,
    display_map::{BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, ClipboardItem, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    Task, WeakEntity,
};
use picker::{Picker, PickerDelegate};
use project::{Project, git_store::Repository};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use ui::{ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt as _;

pub(crate) enum GitHubReviewEvent {
    Open { repo: GitHubRepo, pr: PullRequest },
    CommentsLoaded(Vec<crate::review_provider::PlacedReviewComment>),
}
pub(crate) type ReviewService = GitHubReview;
pub(crate) type ReviewServiceEvent = GitHubReviewEvent;
impl EventEmitter<GitHubReviewEvent> for GitHubReview {}

pub(crate) struct ReviewRequestPicker {
    picker: Entity<Picker<ReviewRequestPickerDelegate>>,
    _subscription: Subscription,
}

impl ReviewRequestPicker {
    pub(crate) fn new(
        review: Entity<GitHubReview>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ReviewRequestPickerDelegate::new(review.downgrade(), review.read(cx));
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .initial_width(rems(24.))
                .show_scrollbar(true)
        });
        let picker_for_subscription = picker.clone();
        let review_subscription = cx.observe_in(&review, window, move |_, review, window, cx| {
            let requests = review.read(cx).request_summaries().to_vec();
            picker_for_subscription.update(cx, |picker, cx| {
                picker.delegate.set_requests(requests);
                picker.refresh(window, cx);
            });
        });
        let dismiss_subscription =
            cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| cx.emit(DismissEvent));
        Self {
            picker,
            _subscription: Subscription::join(review_subscription, dismiss_subscription),
        }
    }
}

impl EventEmitter<DismissEvent> for ReviewRequestPicker {}

impl Focusable for ReviewRequestPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ReviewRequestPicker {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

struct ReviewRequestPickerDelegate {
    review: WeakEntity<GitHubReview>,
    requests: Vec<ReviewRequestSummary>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl ReviewRequestPickerDelegate {
    fn new(review: WeakEntity<GitHubReview>, state: &GitHubReview) -> Self {
        let requests = state.request_summaries().to_vec();
        let matches = Self::all_matches(&requests);
        Self {
            review,
            requests,
            matches,
            selected_index: 0,
        }
    }

    fn all_matches(requests: &[ReviewRequestSummary]) -> Vec<StringMatch> {
        requests
            .iter()
            .enumerate()
            .map(|(index, request)| StringMatch {
                candidate_id: index,
                string: format!("#{} {}", request.number, request.title),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect()
    }

    fn set_requests(&mut self, requests: Vec<ReviewRequestSummary>) {
        self.requests = requests;
        self.matches = Self::all_matches(&self.requests);
        self.selected_index = self
            .selected_index
            .min(self.matches.len().saturating_sub(1));
    }
}

impl PickerDelegate for ReviewRequestPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "review request picker"
    }

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> std::sync::Arc<str> {
        "Search reviews…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, index: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
        self.selected_index = index;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.matches = Self::all_matches(&self.requests);
            return Task::ready(());
        }
        let candidates = self
            .requests
            .iter()
            .enumerate()
            .map(|(index, request)| {
                StringMatchCandidate::new(index, &format!("#{} {}", request.number, request.title))
            })
            .collect::<Vec<_>>();
        cx.spawn_in(window, async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;
            picker
                .update(cx, |picker, _| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = picker
                        .delegate
                        .selected_index
                        .min(picker.delegate.matches.len().saturating_sub(1));
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(request) = self
            .matches
            .get(self.selected_index)
            .and_then(|matched| self.requests.get(matched.candidate_id))
        else {
            return;
        };
        self.review
            .update(cx, |review, cx| {
                review.open_request(request.number, window, cx)
            })
            .log_err();
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let request = self
            .matches
            .get(index)
            .and_then(|matched| self.requests.get(matched.candidate_id))?;
        Some(
            ListItem::new(("review-request", index))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .tooltip(Tooltip::text(format!(
                    "#{} {}",
                    request.number, request.title
                )))
                .child(
                    h_flex()
                        .w_full()
                        .min_w_0()
                        .gap_1()
                        .child(
                            div().flex_none().child(
                                Label::new(format!("#{}", request.number))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                        )
                        .child(
                            div().flex_1().min_w_0().child(
                                Label::new(request.title.clone())
                                    .size(LabelSize::Small)
                                    .single_line()
                                    .truncate(),
                            ),
                        ),
                ),
        )
    }
}

#[derive(Default, Serialize, Deserialize)]
struct SavedDrafts {
    repository: Option<String>,
    #[serde(default)]
    pending_actions: BTreeMap<String, DiscussionAction>,
    drafts: BTreeMap<String, CommentDraft>,
}

struct InlineComposerBlock {
    editor: gpui::WeakEntity<Editor>,
    block_id: CustomBlockId,
    path: String,
    comparison: Option<(Option<String>, Option<String>)>,
    _view: Entity<InlineReviewComposer>,
    _subscription: Subscription,
}

pub(crate) struct InlineReviewComposer {
    github: gpui::WeakEntity<GitHubReview>,
}

impl Render for InlineReviewComposer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.github
            .update(cx, |github, cx| github.render_inline_composer(window, cx))
            .unwrap_or_else(|_| gpui::Empty.into_any_element())
    }
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
    requests: Vec<ReviewRequestSummary>,
    preview: Option<PullRequest>,
    pub checkout: Option<Checkout>,
    discussion: Vec<DiscussionComment>,
    viewer: Option<String>,
    thread_error: Option<String>,
    markdown: BTreeMap<String, (String, Entity<markdown::Markdown>)>,
    expanded_comments: std::collections::BTreeSet<String>,
    preview_markdown: Option<(String, Entity<markdown::Markdown>)>,
    previewing: bool,
    pending_delete: Option<(CommentKind, u64)>,
    target: CommentTarget,
    state: &'static str,
    page: u32,
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
    inline_composer: Option<InlineComposerBlock>,
    inline_reply: Option<(
        crate::review_provider::ReviewThreadId,
        Entity<InlineReviewComposer>,
    )>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<DismissEvent> for GitHubReview {}

impl Focusable for GitHubReview {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.composer.focus_handle(cx)
    }
}

impl GitHubReview {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let query = cx.new(|cx| Editor::single_line(window, cx));
        query.update(cx, |editor, cx| {
            editor.set_placeholder_text("PR number, URL, or title", window, cx)
        });
        let composer = cx.new(|cx| Editor::auto_height(3, 8, window, cx));
        composer.update(cx, |editor, cx| {
            editor.set_placeholder_text("Write a review comment…", window, cx)
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
            markdown: BTreeMap::new(),
            expanded_comments: Default::default(),
            preview_markdown: None,
            previewing: false,
            pending_delete: None,
            target: CommentTarget::General,
            state: "open",
            page: 1,
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
            inline_composer: None,
            inline_reply: None,
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
        self.remove_inline_composer(true, cx);
        self.save_draft(cx);
        self.generation += 1;
        self.selected_repo = Some(checkout.repository.clone());
        self.preview = Some(checkout.pull_request.clone());
        self.checkout = Some(checkout);
        self.detached = false;
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
        self.remove_inline_composer(true, cx);
        self.save_draft(cx);
        self.detached = true;
        self.checkout = None;
        self.review = None;
        self.preview = None;
        if !self.posting {
            self.generation += 1;
            self.task = None;
            self.busy = false;
        }
        cx.notify();
    }

    pub fn detach(&mut self, cx: &mut Context<Self>) {
        self.remove_inline_composer(true, cx);
        self.detached = true;
        cx.notify();
    }
    pub(crate) fn load_review_requests(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.state = "open";
        self.page = 1;
        self.query
            .update(cx, |query, cx| query.set_text("", window, cx));
        self.search(cx);
    }
    pub fn has_provider(&self) -> bool {
        !self.choices.is_empty()
    }
    pub fn matches_repository(&self, repository: &Entity<Repository>, cx: &App) -> bool {
        self.root.as_deref() == Some(repository.read(cx).work_directory_abs_path.as_ref())
    }

    pub fn is_posting(&self) -> bool {
        self.posting
    }

    pub(crate) fn current_review_header(&self) -> Option<ReviewHeader> {
        let checkout = self.checkout.as_ref()?;
        Some(ReviewHeader {
            number: checkout.pull_request.number,
            title: checkout.pull_request.title.clone(),
            repository: checkout.repository.full_name.clone(),
            base_branch: checkout.pull_request.base.branch.clone(),
            review_branch: checkout.pull_request.head.branch.clone(),
        })
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
        self.remove_inline_composer(true, cx);
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
        cx.notify();
    }

    pub fn select_inline_target(
        &mut self,
        target: CommentTarget,
        editor: Entity<Editor>,
        anchor: multi_buffer::Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_target(target, window, cx);
        if self.posting {
            return;
        }
        let github = cx.weak_entity();
        let view = cx.new(|_| InlineReviewComposer { github });
        let path = match &self.target {
            CommentTarget::Inline { path, .. } => path.clone(),
            _ => return,
        };
        let comparison = self
            .review
            .as_ref()
            .and_then(|review| {
                review
                    .read_with(cx, |review, cx| review.comparison_for_path(&path, cx))
                    .ok()
            })
            .flatten();
        let subscription = cx.subscribe(&editor, |_, _, event, cx| {
            if matches!(
                event,
                EditorEvent::BufferEdited
                    | EditorEvent::Edited { .. }
                    | EditorEvent::BuffersEdited { .. }
                    | EditorEvent::BuffersRemoved { .. }
                    | EditorEvent::FileHandleChanged
            ) {
                let this = cx.weak_entity();
                cx.defer(move |cx| {
                    this.update(cx, |this, cx| {
                        if !this.inline_composer_is_current(cx) {
                            this.remove_inline_composer(true, cx);
                        }
                    })
                    .ok();
                });
            }
        });
        let view_for_block = view.clone();
        let block_ids = editor.update(cx, |editor, cx| {
            editor.insert_blocks(
                [BlockProperties {
                    placement: BlockPlacement::Below(anchor),
                    height: Some(9),
                    style: BlockStyle::Sticky,
                    priority: 1,
                    render: std::sync::Arc::new(move |_| view_for_block.clone().into_any_element()),
                }],
                None,
                cx,
            )
        });
        let Some(block_id) = block_ids.into_iter().next() else {
            self.error = Some("Could not open the inline review composer".into());
            cx.notify();
            return;
        };
        self.inline_composer = Some(InlineComposerBlock {
            editor: editor.downgrade(),
            block_id,
            path,
            comparison,
            _view: view,
            _subscription: subscription,
        });
        self.composer.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn remove_inline_composer(&mut self, save_draft: bool, cx: &mut Context<Self>) {
        if save_draft {
            self.save_draft(cx);
        }
        self.inline_reply = None;
        if let Some(inline) = self.inline_composer.take() {
            inline
                .editor
                .update(cx, |editor, cx| {
                    editor.remove_blocks([inline.block_id].into_iter().collect(), None, cx)
                })
                .ok();
        }
        self.notify_review_ui(cx);
        cx.notify();
    }

    fn inline_composer_is_current(&self, cx: &App) -> bool {
        let Some(inline) = &self.inline_composer else {
            return true;
        };
        self.review
            .as_ref()
            .and_then(|review| {
                review
                    .read_with(cx, |review, cx| {
                        review.comparison_for_path(&inline.path, cx)
                    })
                    .ok()
            })
            .flatten()
            == inline.comparison
    }

    fn render_inline_composer(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
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
        let pending = self.busy || self.posting;
        let unknown = self.draft().is_some_and(|draft| draft.outcome_unknown);
        let target = target_label(&self.target);
        let provider_name = self.identity().name;
        v_flex()
            .id("inline-review-comment-composer")
            .w_full()
            .min_w_0()
            .gap_1()
            .p_2()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new(target).size(LabelSize::XSmall))
                    .child(
                        Button::new("close-inline-github-comment", "Cancel").on_click(
                            cx.listener(|this, _, _, cx| this.remove_inline_composer(true, cx)),
                        ),
                    ),
            )
            .when_some(self.error.clone(), |view, error| {
                view.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("write-inline-github-comment", "Write")
                            .toggle_state(!self.previewing)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.previewing = false;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("preview-inline-github-comment", "Preview")
                            .toggle_state(self.previewing)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.previewing = true;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .min_w_0()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .map(|view| {
                        if self.previewing {
                            view.child(crate::review_markdown::render(preview, window, cx))
                        } else {
                            view.child(self.composer.clone())
                        }
                    }),
            )
            .when(self.detached, |view| {
                view.child(
                    Label::new("Review detached from the checkout. Draft saved.")
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                )
            })
            .child(
                Button::new(
                    "post-inline-review-comment",
                    format!("Post to {provider_name}"),
                )
                .disabled(pending || self.detached || unknown || self.load_failed)
                .on_click(cx.listener(|this, _, window, cx| this.post(window, cx))),
            )
            .into_any_element()
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
                    Ok((repo, requests, _next)) => {
                        this.selected_repo = Some(repo);
                        this.requests = requests
                            .into_iter()
                            .map(|request| ReviewRequestSummary {
                                number: request.number,
                                title: request.title,
                            })
                            .collect();
                    }
                    Err(error) => this.error = Some(format!("{error:#}")),
                }
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }

    fn open_request(&mut self, number: u64, window: &mut Window, cx: &mut Context<Self>) {
        if self.posting || self.busy {
            return;
        }
        let Some(repository) = self.selected_repo.clone() else {
            self.error = Some("Select a review repository first".into());
            cx.notify();
            return;
        };
        let client = self.client.clone();
        self.generation += 1;
        let generation = self.generation;
        self.busy = true;
        self.task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = client.pull_request(&repository, number).await;
            this.update(cx, |this, cx| {
                if generation != this.generation {
                    return;
                }
                this.busy = false;
                match result {
                    Ok(request) => {
                        this.preview = Some(request.clone());
                        this.error = None;
                        cx.emit(GitHubReviewEvent::Open {
                            repo: repository,
                            pr: request,
                        });
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
                        let checkout = this.checkout.clone();
                        cx.emit(GitHubReviewEvent::CommentsLoaded(
                            inline
                                .into_iter()
                                .filter_map(github_review::PublishedComment::into_placed)
                                .map(|mut comment| {
                                    comment.url = checkout.as_ref().and_then(|checkout| {
                                        comment.id.parse::<u64>().ok().map(|id| {
                                            format!(
                                                "{}#discussion_r{id}",
                                                checkout.pull_request.url(&checkout.repository)
                                            )
                                        })
                                    });
                                    comment
                                })
                                .collect(),
                        ));
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

    pub(crate) fn reply_inline(
        &mut self,
        comment_id: &str,
        thread_id: crate::review_provider::ReviewThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Ok(comment_id) = comment_id.parse() else {
            self.error = Some("The review provider returned an invalid comment identity".into());
            cx.notify();
            return;
        };
        self.select_target(CommentTarget::Reply { comment_id }, window, cx);
        let github = cx.weak_entity();
        self.inline_reply = Some((thread_id, cx.new(|_| InlineReviewComposer { github })));
        self.composer.focus_handle(cx).focus(window, cx);
        self.notify_review_ui(cx);
        cx.notify();
    }

    pub(crate) fn inline_reply_view(
        &self,
        thread_id: &crate::review_provider::ReviewThreadId,
        _cx: &App,
    ) -> Option<Entity<InlineReviewComposer>> {
        self.inline_reply
            .as_ref()
            .filter(|(active, _)| active == thread_id)
            .map(|(_, view)| view.clone())
    }

    fn notify_review_ui(&self, cx: &mut Context<Self>) {
        if let Some(review) = &self.review {
            let review = review.clone();
            cx.defer(move |cx| {
                review
                    .update(cx, |review, cx| review.remote_review_updated(cx))
                    .log_err();
            });
        }
    }

    pub(crate) fn set_thread_resolved(
        &mut self,
        thread_id: String,
        resolved: bool,
        cx: &mut Context<Self>,
    ) {
        self.run_discussion_action(
            DiscussionAction::Resolve {
                thread_id,
                resolved,
            },
            cx,
        );
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
            if entry.kind == CommentKind::Inline {
                continue;
            }
            let key = comment_key(entry);
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
                        this.remove_inline_composer(false, cx);
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

impl ReviewBackend for GitHubReview {
    fn identity(&self) -> ReviewProviderIdentity {
        ReviewProviderIdentity {
            name: "GitHub".into(),
            repository: self
                .selected_repo
                .as_ref()
                .map(|repository| repository.full_name.clone())
                .or_else(|| self.saved.repository.clone())
                .unwrap_or_default(),
        }
    }

    fn request_summaries(&self) -> &[ReviewRequestSummary] {
        &self.requests
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
        let pending = self.busy || self.posting;
        let unknown = self.draft().is_some_and(|draft| draft.outcome_unknown);
        let provider_name = self.identity().name;
        v_flex()
            .id("review-conversation-content")
            .w(rems(22.))
            .max_h(rems(28.))
            .min_h_0()
            .gap_2()
            .p_2()
            .elevation_3(cx)
            .overflow_hidden()
            .overflow_y_scroll()
            .when_some(self.error.clone(), |view, error| {
                view.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .when(pending, |view| {
                view.child(
                    Label::new(if self.posting {
                        format!("Posting to {provider_name}…")
                    } else {
                        format!("Loading {provider_name}…")
                    })
                    .size(LabelSize::Small),
                )
            })
            .when_some(self.checkout.clone(), |view, checkout| {
                let update = self.preview.clone().filter(|request| {
                    request.head.sha != checkout.pull_request.head.sha
                        || request.base.sha != checkout.pull_request.base.sha
                });
                let provider_name = provider_name.clone();
                view.child(
                    h_flex()
                        .gap_1()
                        .flex_wrap()
                        .child(
                            Button::new("refresh-review-conversation", "Refresh")
                                .disabled(pending)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.refresh_discussion(cx)
                                })),
                        )
                        .child(
                            Button::new(
                                "copy-agent-prompt",
                                if self.copied_prompt {
                                    "Copied"
                                } else {
                                    "Copy Agent Prompt"
                                },
                            )
                            .on_click(cx.listener(|this, _, _, cx| this.copy_agent_prompt(cx))),
                        )
                        .child(
                            Button::new(
                                "open-review-in-provider",
                                format!("{provider_name} ↗"),
                            )
                            .on_click(move |_, _, cx| {
                                cx.open_url(
                                    &checkout.pull_request.url(&checkout.repository),
                                )
                            }),
                        ),
                )
                .when_some(update, |view, request| {
                    let repository =
                        self.checkout.as_ref().map(|checkout| checkout.repository.clone());
                    view.child(
                        Label::new(
                            "A newer review revision is available. Your checkout has not changed.",
                        )
                        .size(LabelSize::Small)
                        .color(Color::Warning),
                    )
                    .child(
                        Button::new("update-review-checkout", "Update checkout")
                            .disabled(pending || self.detached)
                            .on_click(cx.listener(move |_, _, _, cx| {
                                if let Some(repository) = &repository {
                                    cx.emit(GitHubReviewEvent::Open {
                                        repo: repository.clone(),
                                        pr: request.clone(),
                                    });
                                }
                            })),
                    )
                })
            })
            .when_some(self.thread_error.clone(), |view, error| {
                view.child(Label::new(error).size(LabelSize::Small).color(Color::Warning))
            })
            .when(unknown_action, |view| {
                let provider_name = provider_name.clone();
                view.child(
                    Label::new(format!(
                        "A review action may have succeeded. Refresh and inspect {provider_name} before retrying."
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Warning),
                )
                .child(
                    Button::new(
                        "clear-unknown-review-action",
                        format!("I checked {provider_name}; allow another action"),
                    )
                    .disabled(pending || !self.reconciled)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(key) = this.action_key() {
                            this.saved.pending_actions.remove(&key);
                        }
                        this.persist(cx);
                        cx.notify();
                    })),
                )
            })
            .children(discussion)
            .when(self.checkout.is_some(), |view| {
                let provider_name = provider_name.clone();
                view.child(
                    v_flex()
                        .gap_2()
                        .child(Label::new("Conversation").size(LabelSize::Small))
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("write-conversation", "Write")
                                        .toggle_state(!self.previewing)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.previewing = false;
                                            cx.notify();
                                        })),
                                )
                                .child(
                                    Button::new("preview-conversation", "Preview")
                                        .toggle_state(self.previewing)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.previewing = true;
                                            cx.notify();
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .min_w_0()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .p_2()
                                .map(|view| {
                                    if self.previewing {
                                        view.child(crate::review_markdown::render(
                                            preview, window, cx,
                                        ))
                                    } else {
                                        view.child(self.composer.clone())
                                    }
                                }),
                        )
                        .when(self.detached, |view| {
                            view.child(
                                Label::new(
                                    "Review detached from the checkout. Drafts are kept.",
                                )
                                .size(LabelSize::Small)
                                .color(Color::Warning),
                            )
                        })
                        .when(unknown, |view| {
                            let provider_name = provider_name.clone();
                            view.child(
                                Label::new(format!(
                                    "The last post may have succeeded. Refresh and inspect {provider_name} before retrying."
                                ))
                                .size(LabelSize::Small)
                                .color(Color::Warning),
                            )
                            .child(
                                Button::new(
                                    "confirm-conversation-retry",
                                    format!("I checked {provider_name}; allow retry"),
                                )
                                .disabled(pending || !self.reconciled)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    if let Some(key) = this.draft_key()
                                        && let Some(draft) = this.saved.drafts.get_mut(&key)
                                    {
                                        draft.outcome_unknown = false;
                                    }
                                    this.composer
                                        .update(cx, |editor, _| editor.set_read_only(false));
                                    this.persist(cx);
                                    cx.notify();
                                })),
                            )
                        })
                        .child(
                            Button::new(
                                "post-review-conversation",
                                format!("Post to {provider_name}"),
                            )
                            .disabled(
                                pending
                                    || self.detached
                                    || unknown
                                    || unknown_action
                                    || self.load_failed,
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.select_target(CommentTarget::General, window, cx);
                                this.post(window, cx);
                            })),
                        ),
                )
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
            warning: None,
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

    #[test]
    fn request_picker_keeps_review_numbers_separate_from_truncatable_titles() {
        let requests = vec![
            ReviewRequestSummary {
                number: 4,
                title: "feat(cli): add targeted catch-up and user listing".into(),
            },
            ReviewRequestSummary {
                number: 23,
                title: "fix authentication".into(),
            },
        ];
        let matches = ReviewRequestPickerDelegate::all_matches(&requests);
        assert_eq!(
            matches[0].string,
            "#4 feat(cli): add targeted catch-up and user listing"
        );
        assert_eq!(matches[1].string, "#23 fix authentication");
        assert_eq!(matches[0].candidate_id, 0);
        assert_eq!(matches[1].candidate_id, 1);
    }

    #[gpui::test]
    async fn inline_composer_uses_and_preserves_the_durable_github_draft(cx: &mut TestAppContext) {
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
            view.storage_key = Some("test-inline-github-draft".into());
            view.attach(checkout(1), window, cx);
            let editor = cx.new(|cx| Editor::single_line(window, cx));
            let target = CommentTarget::Inline {
                path: "a.txt".into(),
                side: crate::github_review::DiffSide::Right,
                start_line: 1,
                line: 1,
                head_sha: "a".repeat(40),
                base_sha: "b".repeat(40),
            };
            view.select_inline_target(
                target.clone(),
                editor,
                multi_buffer::Anchor::Min,
                window,
                cx,
            );
            assert!(view.inline_composer.is_some());
            assert_eq!(view.target, target);
            view.composer
                .update(cx, |editor, cx| editor.set_text("inline draft", window, cx));
            view.remove_inline_composer(true, cx);
            assert!(view.inline_composer.is_none());
            assert_eq!(view.draft().unwrap().body, "inline draft");
        });
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
            assert!(view.render_discussion(window, cx).is_empty());
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
