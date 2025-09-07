use anyhow::Result;
use fs;
use git::{parse_git_remote_url, GitHostingProviderRegistry, ParsedGitRemote};
use gpui::{
    actions, div, px, App, AsyncWindowContext, ClipboardItem, Context, DismissEvent, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Pixels,
    Render, SharedString, Styled, WeakEntity, Window,
};
use menu::SelectNext;
use project::Project;
use pull_requests::{
    GithubAuth, GithubPrClient, PullRequest, PullRequestManager, PullRequestState, 
    PullRequestStore, PullRequestStoreEvent,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};
use std::sync::Arc;
use theme::{ActiveTheme, observe_buffer_font_size_adjustment};
use ui::{
    h_flex, prelude::*, v_flex, Clickable, Color, FluentBuilder, Icon, IconButton, 
    IconName, IconSize, Label, LabelCommon, LabelSize, ListItem, ListItemSpacing, Tooltip,
};
use util::command::new_smol_command;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    ItemHandle, Workspace,
};
use zed_actions::{DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize};


actions!(
    pull_requests,
    [
        ToggleFocus,
        RefreshPullRequests,
        CheckoutPullRequest,
        OpenInBrowser,
        CopyPullRequestUrl,
        CreatePullRequest,
        LoginWithGithub,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<PullRequestListPanel>(window, cx);
        });
    })
    .detach();
}

pub struct PullRequestListPanel {
    project: Entity<Project>,
    manager: Entity<PullRequestManager>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    selected_index: usize,
    filter_state: FilterState,
    fs: Arc<dyn fs::Fs>,
    _font_size_subscription: gpui::Subscription,
    width: Option<Pixels>,
    has_auto_fetched: bool,
    is_loading: bool,
    _subscriptions: Vec<gpui::Subscription>,
}

#[derive(Default)]
struct FilterState {
    show_open: bool,
    show_closed: bool,
    show_merged: bool,
    show_drafts: bool,
}

impl PullRequestListPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        // For now, skip persistence and just create a new panel
        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = cx.new(|cx| Self::new(workspace, window, cx));
            panel
        })
    }
    

    pub fn new(workspace: &Workspace, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        log::info!("Creating new PullRequestListPanel");
        let project = workspace.project().clone();
        let focus_handle = cx.focus_handle();
        let workspace_weak = workspace.weak_handle();

        let http_client = project.read(cx).client().http_client();
        
        // Get authentication token from GithubAuth
        let token = GithubAuth::global(cx).token().map(|t| t.to_string());
        let api_client = Arc::new(GithubPrClient::new_with_auth(http_client, token));
        let fs = workspace.app_state().fs.clone();

        // Create the store
        let store = cx.new(|cx| PullRequestStore::new(api_client.clone(), cx));

        // Create the manager
        let manager =
            cx.new(|cx| PullRequestManager::new_with_store(project.clone(), store, api_client, cx));

        // Subscribe to manager events
        let subscriptions =
            vec![cx.subscribe(&manager.read(cx).store().clone(), Self::on_store_event)];

        // Subscribe to font size changes to trigger re-rendering
        let font_size_subscription = observe_buffer_font_size_adjustment(cx, |_this, cx| {
            log::info!("PR panel: Font size changed, re-rendering");
            cx.notify();
        });

        // Don't auto-fetch on startup - wait for panel to become active
        // This prevents fetching from the wrong repository

        Self {
            project,
            manager,
            workspace: workspace_weak,
            focus_handle,
            selected_index: 0,
            filter_state: FilterState {
                show_open: true,
                show_closed: false,
                show_merged: false,
                show_drafts: true,
            },
            fs,
            _font_size_subscription: font_size_subscription,
            width: None,
            has_auto_fetched: false,
            is_loading: false,
            _subscriptions: subscriptions,
        }
    }

    fn on_store_event(
        &mut self,
        _store: Entity<PullRequestStore>,
        event: &PullRequestStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            PullRequestStoreEvent::PullRequestsUpdated
            | PullRequestStoreEvent::ActivePullRequestChanged => {
                cx.notify();
            }
        }
    }

    fn refresh_pull_requests(&mut self, _: &RefreshPullRequests, _window: &mut Window, cx: &mut Context<Self>) {
        // Get the actual project's working directory
        let project = self.project.clone();
        let cwd = project.read(cx)
            .worktrees(cx)
            .next()
            .and_then(|worktree| {
                let worktree = worktree.read(cx);
                worktree.root_entry()
                    .filter(|entry| entry.is_dir())
                    .map(|_| worktree.abs_path().to_path_buf())
            });
        
        let Some(cwd) = cwd else {
            log::warn!("PR panel refresh: No project directory found");
            return;
        };
        
        log::info!("PR panel refresh: Project directory: {:?}", cwd);
        
        let manager = self.manager.clone();
        cx.spawn(async move |this, cx| {
            
            // Get origin remote first to determine the actual repository
            let origin_url = if let Ok(output) = new_smol_command("git")
                .current_dir(&cwd)
                .args(&["remote", "get-url", "origin"])
                .output()
                .await
            {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            
            // Use origin by default
            let mut remote_url = origin_url.clone();
            
            // Only check upstream if this appears to be a Zed fork
            // Be specific: only match URLs ending with /zed or /zed.git
            let is_zed_fork = !origin_url.is_empty() && 
               !origin_url.contains("zed-industries") &&
               (origin_url.ends_with("/zed.git") || 
                origin_url.ends_with("/zed") ||
                origin_url.contains(":zed.git") ||
                origin_url.contains(":zed"));
                
            if is_zed_fork {
                log::info!("Refresh: Detected Zed fork (origin: {}), checking for upstream", origin_url);
                // This is a Zed fork, try to use upstream
                if let Ok(output) = new_smol_command("git")
                    .current_dir(&cwd)
                    .args(&["remote", "get-url", "upstream"])
                    .output()
                    .await
                {
                    if output.status.success() {
                        let upstream_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !upstream_url.is_empty() && upstream_url.contains("zed-industries/zed") {
                            remote_url = upstream_url;
                        }
                    }
                }
            }
            
            // If still no remote found, fallback
            if remote_url.is_empty() {
                let _ = manager.update(cx, |manager, cx| manager.refresh_pull_requests(cx))?;
                return Ok::<(), anyhow::Error>(());
            }
            
            // Parse the remote URL to get owner/repo
            let parsed_remote = this.update(cx, |_this, cx| {
                let provider_registry = GitHostingProviderRegistry::global(cx);
                parse_git_remote_url(provider_registry, &remote_url)
                    .map(|(_provider, parsed_remote)| parsed_remote)
            })?;
            
            if let Some(remote) = parsed_remote {
                // Fetch PRs directly from the detected repository (including Zed)
                let _ = manager.update(cx, |manager, cx| {
                    manager.store().update(cx, |store, cx| {
                        store.fetch_pull_requests(remote, cx)
                    })
                })?;
            } else {
                // Fallback to manager's method if parsing fails
                let _ = manager.update(cx, |manager, cx| manager.refresh_pull_requests(cx))?;
            }
            
            Ok::<(), anyhow::Error>(())
        }).detach_and_log_err(cx);
    }

    fn checkout_pull_request(&mut self, _: &CheckoutPullRequest, _window: &mut Window, cx: &mut Context<Self>) {
        let prs = self.filtered_pull_requests(cx);

        if let Some(pr) = prs.get(self.selected_index) {
            self.manager
                .update(cx, |manager, cx| {
                    manager.checkout_pull_request(pr.number, cx)
                })
                .log_err();
        }
    }

    fn open_in_browser(&mut self, _: &OpenInBrowser, _window: &mut Window, cx: &mut Context<Self>) {
        let prs = self.filtered_pull_requests(cx);
        if let Some(pr) = prs.get(self.selected_index) {
            cx.open_url(pr.html_url.as_str());
        }
    }

    fn copy_url(&mut self, _: &CopyPullRequestUrl, _window: &mut Window, cx: &mut Context<Self>) {
        let prs = self.filtered_pull_requests(cx);
        if let Some(pr) = prs.get(self.selected_index) {
            cx.write_to_clipboard(ClipboardItem::new_string(pr.html_url.to_string()));
        }
    }

    fn login_with_github(&mut self, _: &LoginWithGithub, cx: &mut Context<Self>) {
        // Open GitHub personal access token page directly
        let url = "https://github.com/settings/tokens/new?scopes=repo,read:user&description=Zed%20Pull%20Requests";
        cx.open_url(url);
        
        // Show instructions
        cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                this.manager.update(cx, |manager, cx| {
                    manager.store().update(cx, |store, _cx| {
                        store.error = Some(
                            "Opening GitHub to create a token. After creating the token:\n\
                            1. Copy the token\n\
                            2. Run: export GITHUB_TOKEN=<your_token>\n\
                            3. Restart Zed".to_string()
                        );
                    });
                });
                cx.notify();
            })?;
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    fn open_pr_detail(&mut self, pr: PullRequest, window: &mut Window, cx: &mut Context<Self>) {
        // Open PR detail view natively in Zed
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                use crate::pr_detail_view::PullRequestDetailView;
                
                // Check if a tab for this PR already exists across all panes
                let pr_number = pr.number;
                let all_items: Vec<_> = workspace.items(cx).collect();
                
                let existing_view = all_items.into_iter()
                    .find_map(|item| {
                        let downcast = item.downcast::<PullRequestDetailView>();
                        if let Some(view) = &downcast {
                            let view_pr_number = view.read(cx).pr.number;
                            if view_pr_number == pr_number {
                                return downcast;
                            }
                        }
                        None
                    });
                
                if let Some(view) = existing_view {
                    // Check if already active before activating to prevent re-render
                    let is_active = workspace.active_item(cx).map_or(false, |item| {
                        item.item_id() == view.item_id()
                    });
                    
                    if !is_active {
                        // Only activate if not already active
                        workspace.activate_item(&view, true, false, window, cx);
                    }
                    // Always focus to ensure user sees the tab
                    window.focus(&view.focus_handle(cx));
                } else {
                    // Create new tab and focus it
                    let detail_view = cx.new(|cx| {
                        PullRequestDetailView::new_with_project(pr.clone(), &self.project, cx)
                            .unwrap_or_else(|e| {
                                log::error!("Failed to create PR detail view: {}", e);
                                // Fallback: create a basic view without API access
                                let http_client = self.project.read(cx).client().http_client();
                                let remote = ParsedGitRemote {
                                    owner: "unknown".into(),
                                    repo: "unknown".into()
                                };
                                PullRequestDetailView::new_with_authentication(pr.clone(), http_client, remote, cx)
                            })
                    });
                    let focus_handle = detail_view.focus_handle(cx);
                    workspace.add_item_to_active_pane(Box::new(detail_view), None, true, window, cx);
                    window.focus(&focus_handle);
                }
            });
        }
        
        // Also fetch the full PR details to update comment count
        let manager = self.manager.clone();
        let pr_number = pr.number;
        
        if let Ok(remote) = self.manager.read(cx).get_current_remote(cx) {
            let api_client = manager.read(cx).api_client();
            
            cx.spawn(async move |this, cx| {
                // Fetch the full PR details with accurate comment counts
                let full_pr = api_client.get_pull_request(&remote, pr_number).await?;
                
                // Update the PR in the store with accurate data
                this.update(cx, |this, cx| {
                    this.manager.update(cx, |manager, cx| {
                        manager.store().update(cx, |store, _cx| {
                            store.update_pull_request(full_pr);
                        });
                    });
                    cx.notify();
                })?;
                
                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);
        }
    }

    fn create_pull_request(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // Open GitHub PR creation page with current branch
        if let Ok(remote) = self.manager.read(cx).get_current_remote(cx) {
            cx.spawn(async move |this, cx| {
                // Get current branch
                let cwd = match std::env::current_dir() {
                    Ok(dir) => dir,
                    Err(_) => return Ok::<(), anyhow::Error>(()),
                };
                
                // Get current branch name
                let branch_output = match new_smol_command("git")
                    .current_dir(&cwd)
                    .args(&["branch", "--show-current"])
                    .output()
                    .await
                {
                    Ok(output) if output.status.success() => output,
                    _ => {
                        // Fallback to basic compare URL
                        this.update(cx, |_this, cx| {
                            let url = format!("https://github.com/{}/{}/compare", 
                                remote.owner.as_ref(), remote.repo.as_ref());
                            cx.open_url(&url);
                        })?;
                        return Ok::<(), anyhow::Error>(());
                    }
                };
                
                let current_branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();
                
                // Get default branch (usually main or master)
                let default_branch_output = match new_smol_command("git")
                    .current_dir(&cwd)
                    .args(&["symbolic-ref", "refs/remotes/origin/HEAD"])
                    .output()
                    .await
                {
                    Ok(output) if output.status.success() => {
                        let ref_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        // Extract branch name from refs/remotes/origin/branch_name
                        ref_path.split('/').last().unwrap_or("main").to_string()
                    },
                    _ => "main".to_string(), // Default fallback
                };
                
                this.update(cx, |_this, cx| {
                    let url = if current_branch != default_branch_output && !current_branch.is_empty() {
                        format!("https://github.com/{}/{}/compare/{}...{}", 
                            remote.owner.as_ref(), remote.repo.as_ref(), 
                            default_branch_output, current_branch)
                    } else {
                        format!("https://github.com/{}/{}/compare", 
                            remote.owner.as_ref(), remote.repo.as_ref())
                    };
                    cx.open_url(&url);
                })?;
                
                Ok::<(), anyhow::Error>(())
            }).detach_and_log_err(cx);
        }
    }

    fn filtered_pull_requests(&self, cx: &App) -> Vec<PullRequest> {
        let store = self.manager.read(cx).store().read(cx);
        store
            .pull_requests()
            .into_iter()
            .filter(|pr| {
                let state_filter = match pr.state {
                    PullRequestState::Open => self.filter_state.show_open,
                    PullRequestState::Closed => self.filter_state.show_closed,
                    PullRequestState::Merged => self.filter_state.show_merged,
                };
                state_filter && (!pr.draft || self.filter_state.show_drafts)
            })
            .cloned()
            .collect()
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let prs = self.filtered_pull_requests(cx);
        if !prs.is_empty() {
            self.selected_index = (self.selected_index + 1) % prs.len();
            cx.notify();
        }
    }

    fn select_prev(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let prs = self.filtered_pull_requests(cx);
        if !prs.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = prs.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            cx.notify();
        }
    }

    fn increase_font_size(
        &mut self,
        action: &IncreaseBufferFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!("PR panel: increase_font_size called, persist={}", action.persist);
        if !action.persist {
            theme::adjust_buffer_font_size(cx, |size| {
                let new_size = size + px(1.0);
                log::info!("PR panel: Adjusting font size from {} to {}", size.0, new_size.0);
                new_size
            });
            cx.notify();
            log::info!("PR panel: Font size increased and notify called");
        }
    }

    fn decrease_font_size(
        &mut self,
        action: &DecreaseBufferFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!("PR panel: decrease_font_size called, persist={}", action.persist);
        if !action.persist {
            theme::adjust_buffer_font_size(cx, |size| {
                let new_size = size - px(1.0);
                log::info!("PR panel: Adjusting font size from {} to {}", size.0, new_size.0);
                new_size
            });
            cx.notify();
            log::info!("PR panel: Font size decreased and notify called");
        }
    }

    fn reset_font_size(
        &mut self,
        action: &ResetBufferFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !action.persist {
            theme::reset_buffer_font_size(cx);
            cx.notify();
        }
    }
}

impl Render for PullRequestListPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pr_count = self.filtered_pull_requests(cx).len();
        log::info!("Rendering PullRequestListPanel with {} PRs", pr_count);
        let store = self.manager.read(cx).store().read(cx);
        let prs = self.filtered_pull_requests(cx);

        v_flex()
            .size_full()
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        h_flex()
                            .gap_1()
                            .flex_1()
                            .child(Label::new("Pull Requests").size(LabelSize::Small)),
                    )
                    .child(
                        IconButton::new("refresh", IconName::Rerun)
                            .icon_size(IconSize::Small)
                            .disabled({
                                // Disable if no repository or not authenticated
                                self.manager.read(cx).get_current_remote(cx).is_err() || !GithubAuth::global(cx).is_authenticated()
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.refresh_pull_requests(&RefreshPullRequests, window, cx);
                            }))
                            .tooltip(Tooltip::text("Refresh Pull Requests")),
                    )
                    .child(
                        IconButton::new("create-pr", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .disabled(self.manager.read(cx).get_current_remote(cx).is_err())
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.create_pull_request(window, cx);
                            }))
                            .tooltip(Tooltip::text("Create Pull Request")),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .track_focus(&self.focus_handle)
                    .on_action(cx.listener(|this, action: &IncreaseBufferFontSize, _window, cx| {
                        log::info!("PR panel: IncreaseBufferFontSize action received");
                        this.increase_font_size(action, _window, cx);
                    }))
                    .on_action(cx.listener(|this, action: &DecreaseBufferFontSize, _window, cx| {
                        log::info!("PR panel: DecreaseBufferFontSize action received");
                        this.decrease_font_size(action, _window, cx);
                    }))
                    .on_action(cx.listener(|this, action: &ResetBufferFontSize, _window, cx| {
                        log::info!("PR panel: ResetBufferFontSize action received");
                        this.reset_font_size(action, _window, cx);
                    }))
                    .on_action(cx.listener(Self::select_next))
                    .on_action(cx.listener(Self::checkout_pull_request))
                    .on_action(cx.listener(Self::open_in_browser))
                    .on_action(cx.listener(Self::copy_url))
                    .on_action(cx.listener(Self::refresh_pull_requests))
                    .key_context("PullRequestListPanel")
                    .child(if self.is_loading || store.is_loading() {
                        div()
                            .p_4()
                            .child(Label::new("Loading pull requests...").color(Color::Muted))
                    } else if let Some(error) = store.error() {
                        let error_message = error.to_string();
                        let is_auth_error = error_message.contains("404") || 
                                           error_message.contains("401") ||
                                           error_message.contains("403");
                        
                        div()
                            .p_4()
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        Label::new(SharedString::from(error_message))
                                            .color(Color::Error)
                                    )
                                    .when(is_auth_error, |this| {
                                        this.child(
                                            ui::Button::new("login", "Login with GitHub")
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.login_with_github(&LoginWithGithub, cx);
                                                }))
                                        )
                                    })
                            )
                    } else if prs.is_empty() {
                        let auth_state = GithubAuth::global(cx);
                        if !auth_state.is_authenticated() {
                            div()
                                .p_4()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .child(
                                            Label::new("GitHub authentication required to view pull requests")
                                                .color(Color::Muted)
                                        )
                                        .child(
                                            ui::Button::new("login", "Login with GitHub")
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.login_with_github(&LoginWithGithub, cx);
                                                }))
                                        )
                                )
                        } else {
                            // Check if there's a valid git repository
                            if let Ok(_remote) = self.manager.read(cx).get_current_remote(cx) {
                                div()
                                    .p_4()
                                    .child(Label::new("No pull requests found").color(Color::Muted))
                            } else {
                                div()
                                    .p_4()
                                    .child(
                                        v_flex()
                                            .gap_2()
                                            .child(
                                                Label::new("No Git repository detected")
                                                    .color(Color::Muted)
                                            )
                                            .child(
                                                Label::new("Open a project with a Git repository to view pull requests")
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small)
                                            )
                                    )
                            }
                        }
                    } else {
                        let selected_index = self.selected_index;
                        
                        div()
                            .size_full()
                            .child(
                                v_flex()
                                    .children(prs.iter().enumerate().map(move |(idx, pr)| {
                            let is_selected = idx == selected_index;

                            let state_icon = match pr.state {
                                PullRequestState::Open => {
                                    if pr.draft {
                                        Icon::new(IconName::GitBranch)
                                            .color(Color::Modified)
                                            .size(IconSize::Small)
                                    } else {
                                        Icon::new(IconName::GitBranch)
                                            .color(Color::Success)
                                            .size(IconSize::Small)
                                    }
                                }
                                PullRequestState::Closed => Icon::new(IconName::Close)
                                    .color(Color::Error)
                                    .size(IconSize::Small),
                                PullRequestState::Merged => Icon::new(IconName::Check)
                                    .color(Color::Accent)
                                    .size(IconSize::Small),
                            };

                            let title = SharedString::from(pr.title.clone());
                            let total_comments = pr.comments + pr.review_comments;
                            let subtitle = SharedString::from(format!(
                                "#{} opened by {} • {} comments",
                                pr.number, pr.user.login, total_comments
                            ));

                            let pr_clone = pr.clone();
                            
                            ListItem::new(SharedString::from(format!("pr-{}", pr.number)))
                                .spacing(ListItemSpacing::Sparse)
                                .toggle_state(is_selected)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.selected_index = idx;
                                    this.open_pr_detail(pr_clone.clone(), window, cx);
                                    cx.notify();
                                }))
                                .child(
                                    h_flex().gap_2().child(state_icon).child(
                                        v_flex().gap_1().child(Label::new(title)).child(
                                            Label::new(subtitle)
                                                .color(Color::Muted)
                                                .size(LabelSize::Small),
                                        ),
                                    ),
                                )
                        })))
                    }),
            )
    }
}

impl Focusable for PullRequestListPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for PullRequestListPanel {
    fn persistent_name() -> &'static str {
        "PullRequestListPanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match PullRequestListPanelSettings::get_global(cx).dock {
            PullRequestListPanelDockPosition::Left => DockPosition::Left,
            PullRequestListPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file::<PullRequestListPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left => PullRequestListPanelDockPosition::Left,
                    DockPosition::Right => PullRequestListPanelDockPosition::Right,
                    _ => return,
                };
                settings.dock = dock;
            },
        );
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(300.0)
    }

    fn set_size(&mut self, _size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        // Size is fixed for now
    }
    
    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        log::info!("PullRequestListPanel::set_active called with active={}", active);
        
        if active {
            // Focus the panel when it becomes active
            self.focus_handle.focus(window);
        }
        
        if !active {
            // Reset auto-fetch flag when panel becomes inactive so it will fetch again next time
            self.has_auto_fetched = false;
            log::info!("Panel deactivated, reset has_auto_fetched");
        } else if active && !self.has_auto_fetched {
            if GithubAuth::global(cx).is_authenticated() {
                log::info!("Panel activated and authenticated, starting auto-fetch");
                self.has_auto_fetched = true;
                self.is_loading = true;
                
                // Get the actual project's working directory
                let project = self.project.clone();
                let cwd = project.read(cx)
                    .worktrees(cx)
                    .next()
                    .and_then(|worktree| {
                        let worktree = worktree.read(cx);
                        worktree.root_entry()
                            .filter(|entry| entry.is_dir())
                            .map(|_| worktree.abs_path().to_path_buf())
                    });
                
                let Some(cwd) = cwd else {
                    log::warn!("PR panel: No project directory found");
                    self.is_loading = false;
                    cx.notify();
                    return;
                };
                
                log::info!("PR panel: Project directory: {:?}", cwd);
                
                // Detect repository from project's directory
                let manager = self.manager.clone();
                cx.spawn(async move |this, cx| {
                
                // Get origin remote first to determine the actual repository
                let origin_url = if let Ok(output) = new_smol_command("git")
                    .current_dir(&cwd)
                    .args(&["remote", "get-url", "origin"])
                    .output()
                    .await
                {
                    if output.status.success() {
                        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        log::info!("PR panel: Git origin URL: {}", url);
                        url
                    } else {
                        log::warn!("PR panel: Git command failed");
                        String::new()
                    }
                } else {
                    log::warn!("PR panel: Failed to execute git remote command");
                    String::new()
                };
                
                // Use origin by default
                let mut remote_url = origin_url.clone();
                
                // Only check upstream if this appears to be a Zed fork
                // Be specific: only match URLs ending with /zed or /zed.git
                let is_zed_fork = !origin_url.is_empty() && 
                   !origin_url.contains("zed-industries") &&
                   (origin_url.ends_with("/zed.git") || 
                    origin_url.ends_with("/zed") ||
                    origin_url.contains(":zed.git") ||
                    origin_url.contains(":zed"));
                    
                if is_zed_fork {
                    log::info!("Detected Zed fork (origin: {}), checking for upstream remote", origin_url);
                    // This is a Zed fork, try to use upstream
                    if let Ok(output) = new_smol_command("git")
                        .current_dir(&cwd)
                        .args(&["remote", "get-url", "upstream"])
                        .output()
                        .await
                    {
                        if output.status.success() {
                            let upstream_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if !upstream_url.is_empty() && upstream_url.contains("zed-industries/zed") {
                                log::info!("Using upstream remote: {}", upstream_url);
                                remote_url = upstream_url;
                            }
                        }
                    }
                }
                
                // If still no remote found, mark loading as complete
                if remote_url.is_empty() {
                    this.update(cx, |this, cx| {
                        this.is_loading = false;
                        cx.notify();
                    })?;
                    return Ok::<(), anyhow::Error>(());
                }
                
                // Parse the remote URL to get owner/repo
                log::info!("PR panel: Attempting to parse remote URL: {}", remote_url);
                let parsed_remote = this.update(cx, |_this, cx| {
                    let provider_registry = GitHostingProviderRegistry::global(cx);
                    let result = parse_git_remote_url(provider_registry, &remote_url)
                        .map(|(_provider, parsed_remote)| parsed_remote);
                    if let Some(ref remote) = result {
                        log::info!("PR panel: Successfully parsed - owner: {}, repo: {}", 
                            remote.owner.as_ref(), remote.repo.as_ref());
                    } else {
                        log::warn!("PR panel: Failed to parse remote URL");
                    }
                    result
                })?;
                
                if let Some(remote) = parsed_remote {
                    // Log what we're fetching
                    log::info!("Auto-fetching PRs for {}/{}", remote.owner.as_ref(), remote.repo.as_ref());
                    
                    // Fetch PRs directly from the detected repository (including Zed)
                    let _ = manager.update(cx, |manager, cx| {
                        manager.store().update(cx, |store, cx| {
                            store.fetch_pull_requests(remote, cx)
                        })
                    })?;
                } else {
                    log::warn!("Failed to parse remote URL: {}", remote_url);
                }
                
                // Mark loading as complete after fetch attempt
                this.update(cx, |this, cx| {
                    this.is_loading = false;
                    cx.notify();
                })?;
                
                Ok::<(), anyhow::Error>(())
            }).detach_and_log_err(cx);
            } else {
                log::info!("Panel activated but not authenticated, skipping auto-fetch");
            }
        } else if active {
            log::info!("Panel activated but has_auto_fetched={}, skipping", self.has_auto_fetched);
        }
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::PullRequest)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Pull Requests")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        0
    }
    
}

impl EventEmitter<PanelEvent> for PullRequestListPanel {}
impl EventEmitter<DismissEvent> for PullRequestListPanel {}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, SettingsUi, SettingsKey)]
#[settings_key(key = "pull_request_panel")]
pub struct PullRequestListPanelSettings {
    pub dock: PullRequestListPanelDockPosition,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, SettingsUi)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestListPanelDockPosition {
    Left,
    Right,
}

impl Default for PullRequestListPanelDockPosition {
    fn default() -> Self {
        Self::Left
    }
}

impl Settings for PullRequestListPanelSettings {
    type FileContent = Self;

    fn load(sources: SettingsSources<'_, Self::FileContent>, _cx: &mut App) -> Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}

use util::ResultExt;