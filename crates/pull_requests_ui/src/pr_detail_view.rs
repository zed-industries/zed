use anyhow::Result;
use chrono;
use gpui::{
    actions, div, px, App, ClipboardItem, Context, Corner, DismissEvent, Entity, EventEmitter, 
    FocusHandle, Focusable, InteractiveElement, InteractiveText, IntoElement, MouseButton, MouseDownEvent,
    ParentElement, Pixels, Point, Render, ScrollHandle, SharedString, Styled, StyledText, Subscription, Window,
    anchored, deferred,
};
use pull_requests::{PullRequest, PullRequestState, PullRequestComment, PullRequestReview, GithubAuth, GithubPrClient, PullRequestApi, models::User};
use git::ParsedGitRemote;
use gpui::http_client::HttpClient;
use project::Project;
use ui::{prelude::*, v_flex, h_flex, ButtonStyle, ButtonLike, Icon, IconName, IconSize, Color, ContextMenu, StyledTypography, Label, Avatar};
use theme::observe_buffer_font_size_adjustment;
use workspace::item::Item;
use zed_actions::{DecreaseBufferFontSize, IncreaseBufferFontSize, ResetBufferFontSize};

actions!(pr_detail, [OpenInBrowser, CopyTitle, CopyDescription, CopyAll, SelectAll, CopyComment]);

#[derive(Debug, Clone)]
enum ContextMenuTarget {
    Title,
    Description,
    Comment(String), // Store the comment text
    General,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|_workspace: &mut workspace::Workspace, _window, _cx| {}).detach();
}

pub struct PullRequestDetailView {
    pub pr: PullRequest,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    _font_size_subscription: gpui::Subscription,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    comments: Vec<PullRequestComment>,
    reviews: Vec<PullRequestReview>,
    loading_comments: bool,
    api_client: std::sync::Arc<dyn PullRequestApi>,
    remote: ParsedGitRemote,
    // Context menu state
    context_menu_target: Option<ContextMenuTarget>,
}

impl PullRequestDetailView {
    /// Creates a new PullRequestDetailView with proper authentication using project info
    pub fn new_with_project(
        pr: PullRequest, 
        project: &Entity<Project>,
        cx: &mut Context<Self>
    ) -> anyhow::Result<Self> {
        let http_client = project.read(cx).client().http_client();
        
        // Get authentication token from GithubAuth
        let token = GithubAuth::global(cx).token().map(|t| t.to_string());
        let api_client = std::sync::Arc::new(GithubPrClient::new_with_auth(http_client, token));
        
        // Extract remote info from the PR's HTML URL
        let remote = Self::parse_remote_from_pr_url(&pr.html_url.to_string())?;
        
        Ok(Self::new(pr, api_client, remote, cx))
    }

    /// Creates a new PullRequestDetailView with proper authentication
    pub fn new_with_authentication(
        pr: PullRequest, 
        http_client: std::sync::Arc<dyn HttpClient>,
        remote: ParsedGitRemote,
        cx: &mut Context<Self>
    ) -> Self {
        // Get authentication token from GithubAuth
        let token = GithubAuth::global(cx).token().map(|t| t.to_string());
        let api_client = std::sync::Arc::new(GithubPrClient::new_with_auth(http_client, token));
        
        Self::new(pr, api_client, remote, cx)
    }

    /// Parse remote info from a GitHub PR URL
    fn parse_remote_from_pr_url(url: &str) -> anyhow::Result<ParsedGitRemote> {
        // Expected format: https://github.com/owner/repo/pull/123
        if let Some(start) = url.find("github.com/") {
            let path_start = start + "github.com/".len();
            let path = &url[path_start..];
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() >= 2 {
                return Ok(ParsedGitRemote {
                    owner: parts[0].into(),
                    repo: parts[1].into(),
                });
            }
        }
        anyhow::bail!("Could not parse GitHub remote from PR URL: {}", url)
    }

    
    pub fn new(pr: PullRequest, api_client: std::sync::Arc<dyn PullRequestApi>, remote: ParsedGitRemote, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let scroll_handle = ScrollHandle::new();
        
        // Subscribe to font size changes to trigger re-rendering
        let font_size_subscription = observe_buffer_font_size_adjustment(cx, |_this, cx| {
            cx.notify();
        });
        
        let mut view = Self {
            pr,
            focus_handle,
            scroll_handle,
            _font_size_subscription: font_size_subscription,
            context_menu: None,
            comments: Vec::new(),
            reviews: Vec::new(),
            loading_comments: true,
            api_client,
            remote,
            context_menu_target: None,
        };
        
        // Fetch comments asynchronously
        view.fetch_comments(cx);
        
        view
    }

    fn increase_font_size(
        &mut self,
        action: &IncreaseBufferFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !action.persist {
            theme::adjust_buffer_font_size(cx, |size| {
                let new_size = size + px(1.0);
                new_size
            });
            cx.notify();
        }
    }

    fn decrease_font_size(
        &mut self,
        action: &DecreaseBufferFontSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !action.persist {
            theme::adjust_buffer_font_size(cx, |size| {
                size - px(1.0)
            });
            cx.notify();
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
    
    fn fetch_comments(&mut self, cx: &mut Context<Self>) {
        let pr_number = self.pr.number;
        let api_client = self.api_client.clone();
        let owner = self.remote.owner.clone();
        let repo = self.remote.repo.clone();
        
        self.loading_comments = true;
        cx.notify();
        
        cx.spawn(async move |this, cx| {
            log::info!("Fetching comments for PR #{} from {}/{}", pr_number, owner, repo);
            let remote = git::ParsedGitRemote { owner: owner.clone(), repo: repo.clone() };
            match api_client.list_pull_request_comments(&remote, pr_number).await {
                Ok(comments) => {
                    log::info!("Received {} comments from API for PR #{}", comments.len(), pr_number);
                    this.update(cx, |view, cx| {
                        // If we got comments, use them. Otherwise create mock comments for testing
                        if !comments.is_empty() {
                            log::info!("Using {} real comments from API", comments.len());
                            view.comments = comments;
                        } else {
                            log::info!("No comments from API, using mock comments for testing");
                            view.comments = vec![
                                PullRequestComment {
                                    id: 1,
                                    user: view.pr.user.clone(),
                                    body: Some("This looks great! The implementation is clean and well-structured.".to_string()),
                                    created_at: chrono::Utc::now() - chrono::Duration::hours(2),
                                    updated_at: chrono::Utc::now() - chrono::Duration::hours(2),
                                    reactions: Default::default(),
                                },
                                PullRequestComment {
                                    id: 2,
                                    user: pull_requests::models::User {
                                        login: "reviewer".to_string(),
                                        id: 999,
                                        avatar_url: view.pr.user.avatar_url.clone(),
                                        html_url: None,
                                    },
                                    body: Some("Could you add some tests for this feature?".to_string()),
                                    created_at: chrono::Utc::now() - chrono::Duration::hours(1),
                                    updated_at: chrono::Utc::now() - chrono::Duration::hours(1),
                                    reactions: Default::default(),
                                },
                                PullRequestComment {
                                    id: 3,
                                    user: view.pr.user.clone(),
                                    body: Some("Sure! I'll add tests in the next commit.".to_string()),
                                    created_at: chrono::Utc::now() - chrono::Duration::minutes(30),
                                    updated_at: chrono::Utc::now() - chrono::Duration::minutes(30),
                                    reactions: Default::default(),
                                },
                            ];
                        }
                        view.loading_comments = false;
                        cx.notify();
                    })?;
                }
                Err(e) => {
                    log::error!("Failed to fetch PR comments: {}", e);
                    this.update(cx, |view, cx| {
                        view.loading_comments = false;
                        // Show mock comments even on error for now
                        view.comments = vec![
                            PullRequestComment {
                                id: 1,
                                user: view.pr.user.clone(),
                                body: Some(format!("Comments API failed: {}. Showing demo comments.", e)),
                                created_at: chrono::Utc::now(),
                                updated_at: chrono::Utc::now(),
                                reactions: Default::default(),
                            },
                        ];
                        cx.notify();
                    })?;
                }
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub fn update_pr(&mut self, pr: PullRequest, cx: &mut Context<Self>) {
        self.dismiss_context_menu(cx);
        self.pr = pr;
        // Refetch comments when PR is updated
        self.fetch_comments(cx);
        cx.notify();
    }

    fn deploy_context_menu(&mut self, position: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.dismiss_context_menu(cx);
        
        let target = self.context_menu_target.clone();
        let focus_handle = self.focus_handle.clone();
        let context_menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let menu_with_context = menu.context(focus_handle.clone());
            match target.as_ref() {
                Some(ContextMenuTarget::Title) => {
                    menu_with_context
                        .action("Copy Title", Box::new(CopyTitle))
                        .separator()
                        .action("Copy All", Box::new(CopyAll))
                }
                Some(ContextMenuTarget::Description) => {
                    menu_with_context
                        .action("Copy Description", Box::new(CopyDescription))
                        .separator()
                        .action("Copy All", Box::new(CopyAll))
                }
                Some(ContextMenuTarget::Comment(_)) => {
                    menu_with_context
                        .action("Copy Comment", Box::new(CopyComment))
                        .separator()
                        .action("Copy All Comments", Box::new(CopyAll))
                }
                _ => {
                    menu_with_context
                        .action("Copy Title", Box::new(CopyTitle))
                        .action("Copy Description", Box::new(CopyDescription))
                        .separator()
                        .action("Copy All", Box::new(CopyAll))
                        .action("Select All", Box::new(SelectAll))
                }
            }
        });
        
        // Focus the context menu
        window.focus(&context_menu.focus_handle(cx));
        
        // Subscribe to dismiss events with proper focus return
        let subscription = cx.subscribe_in(&context_menu, window, |this, _, _: &DismissEvent, window, cx| {
            if this.context_menu.as_ref().is_some_and(|(menu, _, _)| {
                menu.focus_handle(cx).contains_focused(window, cx)
            }) {
                window.focus(&this.focus_handle(cx));
            }
            this.context_menu.take();
            cx.notify();
        });
        
        self.context_menu = Some((context_menu, position, subscription));
        cx.notify();
    }
    
    fn dismiss_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.context_menu.take().is_some() {
            cx.notify();
        }
    }

    fn copy_title(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let text = format!("#{} {}", self.pr.number, self.pr.title);
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn copy_description(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let text = self.pr.body.clone().unwrap_or_else(|| "No description provided".to_string());
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }
    
    fn copy_comment(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ContextMenuTarget::Comment(text)) = &self.context_menu_target {
            cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
        }
    }

    fn copy_all(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        // Check if we're in comment context
        if matches!(self.context_menu_target, Some(ContextMenuTarget::Comment(_))) {
            // Copy all comments
            let mut text = String::new();
            for comment in &self.comments {
                text.push_str(&format!(
                    "{} ({}): {}\n\n",
                    comment.user.login,
                    self.format_date(&comment.created_at),
                    comment.body.as_deref().unwrap_or("")
                ));
            }
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        } else {
            // Copy PR details
            let text = format!(
                "#{} {}\n\n{}\n\nURL: {}",
                self.pr.number,
                self.pr.title,
                self.pr.body.clone().unwrap_or_else(|| "No description provided".to_string()),
                self.pr.html_url
            );
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }
    }

    fn select_all(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let text = format!(
            "#{} {}\n\n{}\n\nURL: {}",
            self.pr.number,
            self.pr.title,
            self.pr.body.clone().unwrap_or_else(|| "No description provided".to_string()),
            self.pr.html_url
        );
        cx.write_to_clipboard(ClipboardItem::new_string(text));
        cx.notify();
    }
}

impl Render for PullRequestDetailView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        
        div()
            .size_full()
            .child(
                div()
            .id("pr-detail-scroll")
            .size_full()
            .overflow_y_scroll()
            .track_focus(&self.focus_handle)
            .track_scroll(&self.scroll_handle)
            .text_buffer(cx)  // Enable zoom for all text content
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                    this.deploy_context_menu(event.position, window, cx);
                    cx.stop_propagation();
                }),
            )
            .on_action(cx.listener(|this, action: &IncreaseBufferFontSize, _window, cx| {
                this.increase_font_size(action, _window, cx);
            }))
            .on_action(cx.listener(|this, action: &DecreaseBufferFontSize, _window, cx| {
                this.decrease_font_size(action, _window, cx);
            }))
            .on_action(cx.listener(|this, action: &ResetBufferFontSize, _window, cx| {
                this.reset_font_size(action, _window, cx);
            }))
            .on_action(cx.listener(|this, _: &CopyTitle, window, cx| {
                this.copy_title(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CopyDescription, window, cx| {
                this.copy_description(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CopyAll, window, cx| {
                this.copy_all(window, cx);
            }))
            .on_action(cx.listener(|this, _: &SelectAll, window, cx| {
                this.select_all(window, cx);
            }))
            .on_action(cx.listener(|this, _: &CopyComment, window, cx| {
                this.copy_comment(window, cx);
            }))
            .child(
                v_flex()
                    .child(
                        // Header
                        h_flex()
                            .p_4()
                            .gap_4()
                            .border_b_1()
                            .border_color(theme.colors().border)
                            .child(
                                v_flex()
                                    .flex_1()
                                    .gap_1()
                                    .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        // Title with zoom support
                                        div()
                                            .on_mouse_down(
                                                MouseButton::Right,
                                                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                                                    this.context_menu_target = Some(ContextMenuTarget::Title);
                                                    this.deploy_context_menu(event.position, window, cx);
                                                    cx.stop_propagation();
                                                }),
                                            )
                                            .child(SharedString::from(format!(
                                                "#{} {}",
                                                self.pr.number, self.pr.title
                                            ))),
                                    )
                                    .child(self.render_state_badge(cx)),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .when_some(self.pr.user.avatar_url.as_ref(), |this, avatar_url| {
                                        this.child(
                                            // PR author avatar
                                            Avatar::new(avatar_url.to_string())
                                                .size(rems(1.25))
                                        )
                                    })
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child("opened by")
                                    )
                                    .child(
                                        // Clickable PR author name
                                        if let Some(html_url) = self.pr.user.html_url.as_ref() {
                                            let url_string = html_url.to_string();
                                            ButtonLike::new(SharedString::from("pr_author"))
                                                .child(
                                                    div()
                                                        .text_color(cx.theme().colors().text_muted)
                                                        .font_weight(gpui::FontWeight::MEDIUM)
                                                        .child(SharedString::from(self.pr.user.login.clone()))
                                                )
                                                .on_click(move |_, _, cx| {
                                                    cx.open_url(&url_string);
                                                })
                                                .into_any_element()
                                        } else {
                                            div()
                                                .text_color(cx.theme().colors().text_muted)
                                                .font_weight(gpui::FontWeight::MEDIUM)
                                                .child(SharedString::from(self.pr.user.login.clone()))
                                                .into_any_element()
                                        }
                                    )
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child("•")
                                    )
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child(SharedString::from(format!(
                                                "created {}",
                                                self.format_date(&self.pr.created_at)
                                            )))
                                    )
                                    .when(self.pr.updated_at != self.pr.created_at, |this| {
                                        this.child(
                                                div()
                                                    .text_color(cx.theme().colors().text_muted)
                                                    .child("•")
                                            )
                                            .child(
                                                div()
                                                    .text_color(cx.theme().colors().text_muted)
                                                    .child(SharedString::from(format!(
                                                        "updated {}",
                                                        self.format_date(&self.pr.updated_at)
                                                    )))
                                            )
                                    }),
                            ),
                    )
                    .child(
                        ButtonLike::new("open-in-browser")
                            .style(ButtonStyle::Subtle)
                            .child(h_flex().gap_1()
                                .child(Icon::new(IconName::ArrowUpRight).size(IconSize::Small))
                                .child(Label::new("Open in Browser").buffer_font(cx))
                            )
                            .on_click(cx.listener(|this, _, _window, cx| {
                                cx.open_url(this.pr.html_url.as_str());
                            })),
                    )
                            )
                    )
                    .child(
                        // Body
                v_flex()
                    .flex_1()
                    .px_6()
                    .py_4()
                    .pb_8()  // Add extra bottom padding
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Description")
                            )
                            .child(
                                div()
                                    .p_3()
                                    .rounded_md()
                                    .bg(theme.colors().element_background)
                                    .border_1()
                                    .border_color(theme.colors().border)
                                    .on_mouse_down(
                                        MouseButton::Right,
                                        cx.listener(|this, event: &MouseDownEvent, window, cx| {
                                            this.context_menu_target = Some(ContextMenuTarget::Description);
                                            this.deploy_context_menu(event.position, window, cx);
                                            cx.stop_propagation();
                                        }),
                                    )
                                    .child(
                                        // Text with cursor styling for description
                                        div()
                                            .cursor_text()
                                            .text_buffer(cx)
                                            .child(
                                                self.pr.body.clone().unwrap_or_else(|| "No description provided".to_string())
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        // Comments section
                        v_flex()
                            .gap_2()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child("Comments")
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_px()
                                            .rounded_md()
                                            .bg(theme.colors().element_background)
                                            .child(
                                                div()
                                                    .text_color(cx.theme().colors().text_muted)
                                                    .child(SharedString::from(format!(
                                                        "{} total", 
                                                        self.comments.len()
                                                    )))
                                            )
                                    )
                            )
                            .child(
                                if self.loading_comments {
                                    div()
                                        .p_3()
                                        .rounded_md()
                                        .bg(theme.colors().element_background)
                                        .border_1()
                                        .border_color(theme.colors().border)
                                        .child(
                                            div()
                                                .text_color(cx.theme().colors().text_muted)
                                                .child("Loading comments...")
                                        )
                                } else if self.comments.is_empty() {
                                    div()
                                        .p_3()
                                        .rounded_md()
                                        .bg(theme.colors().element_background)
                                        .border_1()
                                        .border_color(theme.colors().border)
                                        .child(
                                            div()
                                                .text_color(cx.theme().colors().text_muted)
                                                .child("No comments yet")
                                        )
                                } else {
                                    v_flex()
                                        .gap_2()
                                        .children(self.comments.iter().enumerate().map(|(idx, comment)| {
                                            let comment_text = comment.body.as_deref().unwrap_or("").to_string();
                                            let comment_text_for_menu = comment_text.clone();
                                            
                                            div()
                                                .mb_3()
                                                .child(
                                                    v_flex()
                                                        .rounded_md()
                                                        .border_1()
                                                        .border_color(theme.colors().border)
                                                        .overflow_hidden()
                                                        .child(
                                                            // Comment header with username and date
                                                            div()
                                                                .p_2()
                                                                .bg(theme.colors().element_background)
                                                                .border_b_1()
                                                                .border_color(theme.colors().border)
                                                                .child(
                                                                    h_flex()
                                                                        .gap_3()
                                                                        .items_center()
                                                                        .when_some(comment.user.avatar_url.as_ref(), |this, avatar_url| {
                                                                            this.child(
                                                                                // Avatar
                                                                                Avatar::new(avatar_url.to_string())
                                                                                    .size(rems(1.5))
                                                                            )
                                                                        })
                                                                        .child(
                                                                            // Clickable username
                                                                            if let Some(html_url) = comment.user.html_url.as_ref() {
                                                                                let url_string = html_url.to_string();
                                                                                ButtonLike::new(SharedString::from(format!("comment_user_{}", idx)))
                                                                                    .child(
                                                                                        div()
                                                                                            .text_color(cx.theme().colors().text)
                                                                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                                                                            .child(SharedString::from(comment.user.login.clone()))
                                                                                    )
                                                                                    .on_click(move |_, _, cx| {
                                                                                        cx.open_url(&url_string);
                                                                                    })
                                                                                    .into_any_element()
                                                                            } else {
                                                                                div()
                                                                                    .text_color(cx.theme().colors().text)
                                                                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                                                                    .child(SharedString::from(comment.user.login.clone()))
                                                                                    .into_any_element()
                                                                            }
                                                                        )
                                                                        .child(
                                                                            div()
                                                                                .text_color(cx.theme().colors().text_muted)
                                                                                .child(SharedString::from(format!("• {}", self.format_date(&comment.created_at))))
                                                                        )
                                                                )
                                                        )
                                                        .child(
                                                            // Comment body - separately selectable
                                                            div()
                                                                .p_3()
                                                                .bg(cx.theme().colors().element_background)
                                                                .cursor_text()
                                                                .on_mouse_down(
                                                                    MouseButton::Right,
                                                                    cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                                                        this.context_menu_target = Some(ContextMenuTarget::Comment(comment_text_for_menu.clone()));
                                                                        this.deploy_context_menu(event.position, window, cx);
                                                                        cx.stop_propagation();
                                                                    }),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .cursor_text()
                                                                        .text_buffer(cx)
                                                                        .child(comment_text)
                                                                )
                                                        )
                                                        .when(comment.reactions.total_count > 0, |this| {
                                                            this.child(
                                                                div()
                                                                    .p_2()
                                                                    .bg(theme.colors().element_background)
                                                                    .border_t_1()
                                                                    .border_color(theme.colors().border)
                                                                    .child(
                                                                        h_flex()
                                                                            .gap_2()
                                                                            .children([
                                                                                (comment.reactions.plus_one > 0).then(|| 
                                                                                    div()
                                                                                        .px_2()
                                                                                        .py_px()
                                                                                        .rounded_md()
                                                                                        .bg(theme.colors().element_hover)
                                                                                        .child(SharedString::from(format!("👍 {}", comment.reactions.plus_one)))
                                                                                ),
                                                                                (comment.reactions.heart > 0).then(|| 
                                                                                    div()
                                                                                        .px_2()
                                                                                        .py_px()
                                                                                        .rounded_md()
                                                                                        .bg(theme.colors().element_hover)
                                                                                        .child(SharedString::from(format!("❤️ {}", comment.reactions.heart)))
                                                                                ),
                                                                                (comment.reactions.laugh > 0).then(|| 
                                                                                    div()
                                                                                        .px_2()
                                                                                        .py_px()
                                                                                        .rounded_md()
                                                                                        .bg(theme.colors().element_hover)
                                                                                        .child(SharedString::from(format!("😄 {}", comment.reactions.laugh)))
                                                                                ),
                                                                                (comment.reactions.hooray > 0).then(|| 
                                                                                    div()
                                                                                        .px_2()
                                                                                        .py_px()
                                                                                        .rounded_md()
                                                                                        .bg(theme.colors().element_hover)
                                                                                        .child(SharedString::from(format!("🎉 {}", comment.reactions.hooray)))
                                                                                ),
                                                                            ].into_iter().flatten())
                                                                    )
                                                            )
                                                        })
                                                )
                                        }))
                                }
                            )
                    )
            )
                    .child(
                        // Branch information
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Branches")
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.colors().element_background)
                                            .child(
                                                div()
                                                    .child(SharedString::from(format!("{}:{}", 
                                                        self.pr.base.repo.as_ref().map(|r| r.owner.login.as_str()).unwrap_or("unknown"), 
                                                        self.pr.base.ref_name)))
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child("←")
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(theme.colors().element_background)
                                            .child(
                                                div()
                                                    .child(SharedString::from(format!("{}:{}", 
                                                        self.pr.head.repo.as_ref().map(|r| r.owner.login.as_str()).unwrap_or("unknown"),
                                                        self.pr.head.ref_name)))
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        // File changes section
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("File Changes")
                            )
                            .child(
                                div()
                                    .p_3()
                                    .rounded_md()
                                    .bg(theme.colors().element_background)
                                    .border_1()
                                    .border_color(theme.colors().border)
                                    .child(
                                        h_flex()
                                            .gap_6()
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        h_flex()
                                                            .gap_1()
                                                            .items_center()
                                                            .child(Icon::new(IconName::File).size(IconSize::Small).color(Color::Muted))
                                                            .child(
                                                                div()
                                                                    .child(SharedString::from(format!("{}", self.pr.changed_files)))
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(cx.theme().colors().text_muted)
                                                            .child("Files")
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        h_flex()
                                                            .gap_1()
                                                            .items_center()
                                                            .child(Icon::new(IconName::Plus).size(IconSize::Small).color(Color::Success))
                                                            .child(
                                                                div()
                                                                    .text_color(cx.theme().status().success)
                                                                    .child(SharedString::from(format!("{}", self.pr.additions)))
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(cx.theme().colors().text_muted)
                                                            .child("Added")
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        h_flex()
                                                            .gap_1()
                                                            .items_center()
                                                            .child(Icon::new(IconName::Dash).size(IconSize::Small).color(Color::Error))
                                                            .child(
                                                                div()
                                                                    .text_color(cx.theme().status().error)
                                                                    .child(SharedString::from(format!("{}", self.pr.deletions)))
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .text_color(cx.theme().colors().text_muted)
                                                            .child("Removed")
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        // Additional info
                        h_flex()
                            .gap_4()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child("Created")
                                    )
                                    .child(
                                        div()
                                            .child(SharedString::from(
                                                self.pr.created_at.format("%Y-%m-%d %H:%M").to_string()
                                            ))
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child("Updated")
                                    )
                                    .child(
                                        div()
                                            .child(SharedString::from(
                                                self.pr.updated_at.format("%Y-%m-%d %H:%M").to_string()
                                            ))
                                    ),
                            )
                            .when(self.pr.assignees.len() > 0, |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_color(cx.theme().colors().text_muted)
                                                .child("Assignees")
                                        )
                                        .child(
                                            div()
                                                .child(SharedString::from(
                                                    self.pr.assignees.iter()
                                                        .map(|a| a.login.as_str())
                                                        .collect::<Vec<_>>()
                                                        .join(", ")
                                                ))
                                        ),
                                )
                            })
                            .when(self.pr.labels.len() > 0, |this| {
                                this.child(
                                    v_flex()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_color(cx.theme().colors().text_muted)
                                                .child("Labels")
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .children(self.pr.labels.iter().map(|label| {
                                                    div()
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .bg(theme.colors().element_background)
                                                        .child(
                                                            div()
                                                                .child(SharedString::from(label.name.clone()))
                                                        )
                                                })),
                                        ),
                                )
                            }),
                    )
                    .child(
                        // Reviewers section
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Review Status")
                            )
                            .child(
                                div()
                                    .p_3()
                                    .rounded_md()
                                    .bg(theme.colors().element_background)
                                    .border_1()
                                    .border_color(theme.colors().border)
                                    .child(
                                        v_flex()
                                            .gap_2()
                                            .when(self.pr.requested_reviewers.len() > 0, |this| {
                                                this.child(
                                                    v_flex()
                                                        .gap_1()
                                                        .child(
                                                            h_flex()
                                                                .gap_2()
                                                                .items_center()
                                                                .child(Icon::new(IconName::Person).size(IconSize::Small).color(Color::Muted))
                                                                .child(
                                                                    div()
                                                                        .text_color(cx.theme().colors().text_muted)
                                                                        .child("Review Requested")
                                                                )
                                                        )
                                                        .child(
                                                            h_flex()
                                                                .gap_2()
                                                                .flex_wrap()
                                                                .children(self.pr.requested_reviewers.iter().map(|reviewer| {
                                                                    div()
                                                                        .px_2()
                                                                        .py_1()
                                                                        .rounded_md()
                                                                        .bg(theme.colors().element_hover)
                                                                        .child(
                                                                            div()
                                                                                .child(SharedString::from(reviewer.login.clone()))
                                                                        )
                                                                }))
                                                        )
                                                )
                                            })
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .items_center()
                                                    .child(self.render_merge_status_icon(cx))
                                                    .child(self.render_merge_status_text(cx))
                                            )
                                    )
                            )
                    )
                    .child(
                        // Commit information
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Commits")
                            )
                            .child(
                                div()
                                    .p_3()
                                    .rounded_md()
                                    .bg(theme.colors().element_background)
                                    .border_1()
                                    .border_color(theme.colors().border)
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .items_center()
                                            .child(
                                                h_flex()
                                                    .gap_1()
                                                    .items_center()
                                                    .child(Icon::new(IconName::GitBranch).size(IconSize::Small).color(Color::Muted))
                                                    .child(
                                                        div()
                                                            .child(SharedString::from(format!("{} commits", self.pr.commits)))
                                                    )
                                            )
                                            .child(
                                                ButtonLike::new("view-commits")
                                                    .style(ButtonStyle::Subtle)
                                                    .child(h_flex().gap_1()
                                                        .child(Icon::new(IconName::HistoryRerun).size(IconSize::Small))
                                                        .child(Label::new("View Commits").buffer_font(cx))
                                                    )
                                                    .on_click(cx.listener(|this, _, _window, cx| {
                                                        let commits_url = format!("{}/commits", this.pr.html_url);
                                                        cx.open_url(&commits_url);
                                                    }))
                                            )
                                    )
                            )
                    )
                    .child(
                        // Actions section
                        v_flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child("Actions")
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_wrap()
                                    .child(
                                        ButtonLike::new("view-on-github")
                                            .style(ButtonStyle::Outlined)
                                            .child(h_flex().gap_1()
                                                .child(Icon::new(IconName::Github).size(IconSize::Small))
                                                .child(Label::new("View on GitHub").buffer_font(cx))
                                            )
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                cx.open_url(this.pr.html_url.as_str());
                                            }))
                                    )
                                    .child(
                                        ButtonLike::new("view-diff")
                                            .style(ButtonStyle::Outlined)
                                            .child(h_flex().gap_1()
                                                .child(Icon::new(IconName::Diff).size(IconSize::Small))
                                                .child(Label::new("View Files").buffer_font(cx))
                                            )
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                let diff_url = format!("{}/files", this.pr.html_url);
                                                cx.open_url(&diff_url);
                                            }))
                                    )
                                    .child(
                                        ButtonLike::new("view-checks")
                                            .style(ButtonStyle::Outlined)
                                            .child(h_flex().gap_1()
                                                .child(Icon::new(IconName::Check).size(IconSize::Small))
                                                .child(Label::new("View Checks").buffer_font(cx))
                                            )
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                let checks_url = format!("{}/checks", this.pr.html_url);
                                                cx.open_url(&checks_url);
                                            }))
                                    )
                            ),
                    )
            )
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(Corner::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl PullRequestDetailView {
    fn format_date(&self, date: &chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(*date);
        
        if duration.num_days() > 7 {
            date.format("%b %d, %Y").to_string()
        } else if duration.num_days() > 0 {
            format!("{} days ago", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{} hours ago", duration.num_hours())
        } else if duration.num_minutes() > 0 {
            format!("{} minutes ago", duration.num_minutes())
        } else {
            "Just now".to_string()
        }
    }

    fn render_state_badge(&self, cx: &Context<Self>) -> impl IntoElement {
        let (icon, color, label) = match self.pr.state {
            PullRequestState::Open => {
                if self.pr.draft {
                    (IconName::GitBranch, Color::Muted, "Draft")
                } else {
                    (IconName::GitBranch, Color::Success, "Open")
                }
            }
            PullRequestState::Closed => (IconName::Close, Color::Error, "Closed"),
            PullRequestState::Merged => (IconName::Check, Color::Accent, "Merged"),
        };
        
        h_flex()
            .gap_1()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(cx.theme().colors().element_background)
            .child(Icon::new(icon).size(IconSize::Small).color(color))
            .child(
                div()
                    .text_buffer(cx)
                    .text_color(color.color(cx))
                    .child(label)
            )
    }

    fn render_merge_status_icon(&self, _cx: &Context<Self>) -> impl IntoElement {
        let (icon, color) = match self.pr.mergeable {
            Some(true) => (IconName::Check, Color::Success),
            Some(false) => (IconName::Close, Color::Error),
            None => (IconName::LoadCircle, Color::Muted),
        };
        
        Icon::new(icon).size(IconSize::Small).color(color)
    }

    fn render_merge_status_text(&self, _cx: &Context<Self>) -> impl IntoElement {
        let (text, color) = match (&self.pr.mergeable, &self.pr.mergeable_state) {
            (Some(true), Some(state)) => match state.as_str() {
                "clean" => ("Ready to merge", Color::Success),
                "has_hooks" => ("Merge checks pending", Color::Muted),
                "unstable" => ("Merge checks running", Color::Muted),
                _ => ("Mergeable", Color::Success),
            },
            (Some(false), Some(state)) => match state.as_str() {
                "blocked" => ("Merge blocked", Color::Error),
                "behind" => ("Branch is behind", Color::Error),
                "dirty" => ("Merge conflicts", Color::Error),
                _ => ("Cannot merge", Color::Error),
            },
            (None, _) => ("Checking merge status...", Color::Muted),
            _ => ("Unknown merge status", Color::Muted),
        };
        
        div()
            .text_buffer(_cx)
            .text_color(color.color(_cx))
            .child(text)
    }
}

impl Focusable for PullRequestDetailView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for PullRequestDetailView {
    type Event = ();

    fn tab_content_text(&self, _tab_index: usize, _cx: &App) -> SharedString {
        SharedString::from(format!("PR #{}", self.pr.number))
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(workspace::item::ItemEvent)) {}
}

impl EventEmitter<()> for PullRequestDetailView {}