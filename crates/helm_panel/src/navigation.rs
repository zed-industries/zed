//! Moving between Helm's screens: switching screen, the back and forward
//! history, and the navigation bar.

use super::*;

impl HelmPanel {
    /// Lands on `screen` without touching `back_stack`/`forward_stack` —
    /// for internal state-machine transitions. User-initiated navigation
    /// should go through [`Self::navigate_to`] instead.
    pub(super) fn set_screen(&mut self, screen: HelmScreen, cx: &mut Context<Self>) {
        self.screen = screen;
        self.error_msg.clear();
        // Landing on a screen that isn't org-scoped drops whichever org was
        // being browsed — `OrgList` itself doesn't need `selected_org`, only
        // `OrgDetail` does. `RepoList`/`RepoDetail` and the screens drilled
        // into from it (Branches/Collaborators/the repo tabs) are also kept:
        // they can be showing an org's own repos (reached via
        // `open_repo_list` from `OrgDetail`), and clearing
        // `selected_org`/`org_detail` here meant going back to `OrgDetail`
        // afterwards found no org data left to render — it fell into the
        // "Failed to load" empty state, whose Retry button then no-op'd too
        // since `selected_org` was already gone.
        // `UserProfile` is here for the same reason: it is reached from
        // `OrgMembers`, and going back must find the organisation still there.
        const ORG_SCOPED: &[HelmScreen] = &[
            HelmScreen::OrgDetail,
            HelmScreen::OrgMembers,
            HelmScreen::UserProfile,
            HelmScreen::RepoList,
            HelmScreen::RepoDetail,
            HelmScreen::Branches,
            HelmScreen::Collaborators,
            HelmScreen::Issues,
            HelmScreen::IssueDetail,
            HelmScreen::Pulls,
            HelmScreen::PrDetail,
            HelmScreen::Releases,
            HelmScreen::Packages,
            HelmScreen::Traffic,
            HelmScreen::Commits,
            HelmScreen::WorkflowRuns,
            HelmScreen::Deployments,
            HelmScreen::Tags,
            HelmScreen::Security,
        ];
        const REPO_DRIVEN: &[HelmScreen] = &[
            HelmScreen::RepoDetail,
            HelmScreen::Branches,
            HelmScreen::Collaborators,
            HelmScreen::Issues,
            HelmScreen::IssueDetail,
            HelmScreen::Pulls,
            HelmScreen::PrDetail,
            HelmScreen::Releases,
            HelmScreen::Packages,
            HelmScreen::Traffic,
            HelmScreen::Commits,
            HelmScreen::WorkflowRuns,
            HelmScreen::Deployments,
            HelmScreen::Tags,
            HelmScreen::Security,
        ];
        if !ORG_SCOPED.contains(&screen) {
            self.selected_org = None;
            self.org_detail = None;
            self.org_role = None;
            self.org_members.clear();
        }
        // `selected_repo`/clone state is only meaningful on `RepoDetail` and
        // the screens drilled into from it; clearing here also drops the tab
        // caches so going back to `RepoDetail` doesn't resurrect stale data.
        if !REPO_DRIVEN.contains(&screen) {
            self.selected_repo = None;
            self.cloning = false;
            self.clone_lines.clear();
            self.clone_error = None;
            self.issues.clear();
            self.pulls.clear();
            self.releases.clear();
            self.packages.clear();
            self.package_versions.clear();
            self.expanded_package = None;
            self.traffic = None;
            self.commits.clear();
            self.workflow_runs.clear();
            self.deployments.clear();
            self.tags.clear();
            self.dependabot_alerts.clear();
            self.secret_scanning_alerts.clear();
        }
        if screen != HelmScreen::UserProfile {
            self.viewed_user = None;
        }
        // Refresh the invitations badge every time the Profile screen (or the
        // Invitations screen itself) is landed on, including via back/forward.
        if matches!(screen, HelmScreen::Profile | HelmScreen::Invitations) {
            self.load_repo_invitations(cx);
        }
        cx.notify();
    }

    /// Navigate to `screen` as a user-initiated action: push the current
    /// screen onto the back stack and clear the forward stack (same as
    /// following a link in a browser).
    pub(super) fn navigate_to(&mut self, screen: HelmScreen, cx: &mut Context<Self>) {
        self.back_stack.push(self.screen);
        self.forward_stack.clear();
        self.set_screen(screen, cx);
    }

    pub(super) fn go_back(&mut self, cx: &mut Context<Self>) {
        if let Some(prev) = self.back_stack.pop() {
            self.forward_stack.push(self.screen);
            self.set_screen(prev, cx);
        }
    }

    pub(super) fn go_forward(&mut self, cx: &mut Context<Self>) {
        if let Some(next) = self.forward_stack.pop() {
            self.back_stack.push(self.screen);
            self.set_screen(next, cx);
        }
    }

    pub(super) fn go_home(&mut self, cx: &mut Context<Self>) {
        if self.screen != HelmScreen::Menu {
            self.navigate_to(HelmScreen::Menu, cx);
        }
    }

    /// Home/Back/Forward controls, shown in the panel header once
    /// authenticated.
    pub(super) fn render_nav_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let buttons: [(&'static str, IconName, &'static str, bool); 3] = [
            (
                "helm-nav-home",
                IconName::Home,
                "Home",
                self.screen != HelmScreen::Menu,
            ),
            (
                "helm-nav-back",
                IconName::ArrowLeft,
                "Back",
                !self.back_stack.is_empty(),
            ),
            (
                "helm-nav-forward",
                IconName::ArrowRight,
                "Forward",
                !self.forward_stack.is_empty(),
            ),
        ];

        h_flex()
            .items_center()
            .gap_1()
            .children(buttons.into_iter().map(|(id, icon, tooltip, enabled)| {
                Button::new(id)
                    .ghost()
                    .xsmall()
                    .icon(icon)
                    .disabled(!enabled)
                    .tooltip(tooltip)
                    .on_click(cx.listener(move |this, _, _, cx| match id {
                        "helm-nav-home" => this.go_home(cx),
                        "helm-nav-back" => this.go_back(cx),
                        "helm-nav-forward" => this.go_forward(cx),
                        _ => {}
                    }))
            }))
    }
}
