use anyhow::Result;
use collections::HashMap;
use git::ParsedGitRemote;
use gpui::{App, Context, EventEmitter, Task};
use std::sync::Arc;

use crate::api::PullRequestApi;
use crate::models::{PullRequest, PullRequestState};
use crate::PullRequestStoreEvent;

pub fn init(_cx: &mut App) {}

pub struct PullRequestStore {
    api_client: Arc<dyn PullRequestApi>,
    pull_requests: HashMap<u32, PullRequest>,
    active_pr: Option<u32>,
    loading: bool,
    pub error: Option<String>,
}

impl PullRequestStore {
    pub fn new(api_client: Arc<dyn PullRequestApi>, _cx: &mut Context<Self>) -> Self {
        Self {
            api_client,
            pull_requests: HashMap::default(),
            active_pr: None,
            loading: false,
            error: None,
        }
    }

    pub fn pull_requests(&self) -> Vec<&PullRequest> {
        let mut prs: Vec<_> = self.pull_requests.values().collect();
        prs.sort_by_key(|pr| std::cmp::Reverse(pr.updated_at));
        prs
    }

    pub fn get_pull_request(&self, number: u32) -> Option<&PullRequest> {
        self.pull_requests.get(&number)
    }

    pub fn update_pull_request(&mut self, pr: PullRequest) {
        self.pull_requests.insert(pr.number, pr);
    }

    pub fn active_pull_request(&self) -> Option<&PullRequest> {
        self.active_pr.and_then(|num| self.pull_requests.get(&num))
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn update_api_client(&mut self, api_client: Arc<dyn PullRequestApi>) {
        self.api_client = api_client;
    }

    pub fn fetch_pull_requests(
        &mut self,
        remote: ParsedGitRemote,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if self.loading {
            return Ok(());
        }

        self.loading = true;
        self.error = None;
        cx.notify();

        let api_client = self.api_client.clone();

        cx.spawn(async move |this, cx| {
            let result = api_client.list_pull_requests(&remote, Some("open")).await;

            this.update(cx, |store, cx| {
                store.loading = false;

                match result {
                    Ok(prs) => {
                        store.pull_requests.clear();
                        // First, insert all PRs with basic data
                        for pr in prs {
                            store.pull_requests.insert(pr.number, pr);
                        }
                        store.error = None;
                        cx.emit(PullRequestStoreEvent::PullRequestsUpdated);
                        cx.notify();
                        
                        // Then fetch full details for each PR to get accurate comment counts
                        let pr_numbers: Vec<u32> = store.pull_requests.keys().copied().collect();
                        log::info!("Fetching full details for {} PRs to get comment counts", pr_numbers.len());
                        for pr_number in pr_numbers {
                            let api_client = api_client.clone();
                            let remote = ParsedGitRemote {
                                owner: remote.owner.clone(),
                                repo: remote.repo.clone(),
                            };
                            cx.spawn(async move |this, cx| {
                                log::info!("Fetching full details for PR #{}", pr_number);
                                if let Ok(full_pr) = api_client.get_pull_request(&remote, pr_number).await {
                                    log::info!("Got PR #{} with {} comments", pr_number, full_pr.comments + full_pr.review_comments);
                                    this.update(cx, |store, cx| {
                                        store.update_pull_request(full_pr);
                                        cx.notify();
                                    })?;
                                }
                                Ok::<(), anyhow::Error>(())
                            })
                            .detach();
                        }
                    }
                    Err(e) => {
                        store.error = Some(format!("Failed to fetch pull requests: {}", e));
                        log::error!("Failed to fetch pull requests: {}", e);
                        cx.notify();
                    }
                }
            })?;

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);

        Ok(())
    }

    pub fn fetch_pull_request(
        &mut self,
        remote: ParsedGitRemote,
        number: u32,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let api_client = self.api_client.clone();

        cx.spawn(async move |this, cx| {
            let pr = api_client.get_pull_request(&remote, number).await?;

            this.update(cx, |store, cx| {
                store.pull_requests.insert(number, pr);
                cx.emit(PullRequestStoreEvent::PullRequestsUpdated);
                cx.notify();
            })?;

            Ok::<(), anyhow::Error>(())
        })
    }

    pub fn set_active_pull_request(&mut self, number: Option<u32>, cx: &mut Context<Self>) {
        if self.active_pr != number {
            self.active_pr = number;
            cx.emit(PullRequestStoreEvent::ActivePullRequestChanged);
            cx.notify();
        }
    }

    pub fn checkout_pull_request(&mut self, pr_number: u32, cx: &mut Context<Self>) -> Result<()> {
        let pr = self
            .pull_requests
            .get(&pr_number)
            .ok_or_else(|| anyhow::anyhow!("Pull request {} not found", pr_number))?;

        let branch_name = pr.head.ref_name.clone();

        cx.spawn(async move |this, cx| {
            smol::process::Command::new("git")
                .arg("fetch")
                .arg("origin")
                .arg(&format!("pull/{}/head:{}", pr_number, branch_name))
                .output()
                .await?;

            smol::process::Command::new("git")
                .arg("checkout")
                .arg(&branch_name)
                .output()
                .await?;

            this.update(cx, |store, cx| {
                store.set_active_pull_request(Some(pr_number), cx);
            })?;

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);

        Ok(())
    }

    pub fn filter_pull_requests(&self, filter: PullRequestFilter) -> Vec<&PullRequest> {
        self.pull_requests()
            .into_iter()
            .filter(|pr| match &filter {
                PullRequestFilter::All => true,
                PullRequestFilter::Open => pr.state == PullRequestState::Open,
                PullRequestFilter::Closed => pr.state == PullRequestState::Closed,
                PullRequestFilter::Merged => pr.state == PullRequestState::Merged,
                PullRequestFilter::Draft => pr.draft,
                PullRequestFilter::ReadyForReview => !pr.draft,
                PullRequestFilter::Author(login) => pr.user.login == *login,
                PullRequestFilter::Assignee(login) => {
                    pr.assignees.iter().any(|a| a.login == *login)
                }
                PullRequestFilter::Label(label) => pr.labels.iter().any(|l| l.name == *label),
            })
            .collect()
    }
}

impl EventEmitter<PullRequestStoreEvent> for PullRequestStore {}

#[derive(Clone, Debug)]
pub enum PullRequestFilter {
    All,
    Open,
    Closed,
    Merged,
    Draft,
    ReadyForReview,
    Author(String),
    Assignee(String),
    Label(String),
}
