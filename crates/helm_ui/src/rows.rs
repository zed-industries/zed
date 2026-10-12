//! Shared row renderers for Helm's GitHub lists.

use gpui::{App, ParentElement as _, Styled as _, div, prelude::FluentBuilder as _};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _, StyledExt as _, avatar::Avatar, h_flex,
    list::ListItem, tag::Tag as Pill, v_flex,
};
use helm_backend::github::{Branch, CommitSummary, Issue, Pull, Release, Tag, WorkflowRun};

use crate::chip;

/// One row of the Issues screen: title, number and author, comment count,
/// and the issue's labels.
pub fn issue_row(ix: usize, issue: &Issue, cx: &App) -> ListItem {
    let muted_foreground = cx.theme().muted_foreground;
    let foreground = cx.theme().foreground;
    let number = issue.number;
    let closed = issue.state == "closed";
    let author = issue
        .user
        .as_ref()
        .map(|u| u.login.clone())
        .unwrap_or_default();
    let comments = issue.comments;
    let labels = issue.labels.clone();
    ListItem::new(("helm-issue", ix))
        .child(
            v_flex()
                .gap_0p5()
                .min_w_0()
                .child(
                    div()
                        .truncate()
                        .text_sm()
                        .font_semibold()
                        .text_color(if closed { muted_foreground } else { foreground })
                        .child(issue.title.clone()),
                )
                .child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_foreground)
                                .child(format!("#{number} · {author}")),
                        )
                        .when(comments > 0, |row| {
                            row.child(
                                div()
                                    .text_xs()
                                    .text_color(muted_foreground)
                                    .child(format!("{comments} comments")),
                            )
                        }),
                ),
        )
        .suffix(move |_, _| {
            h_flex()
                .items_center()
                .gap_2()
                .children(labels.iter().map(|label| chip(label.name.clone())))
                .child(
                    Icon::new(IconName::ChevronRight)
                        .xsmall()
                        .text_color(muted_foreground),
                )
        })
}

/// One row of the Pull Requests screen: title, number, and the branches it
/// merges from and into.
pub fn pull_row(ix: usize, pr: &Pull, cx: &App) -> ListItem {
    let muted_foreground = cx.theme().muted_foreground;
    let foreground = cx.theme().foreground;
    let number = pr.number;
    let open = !pr.merged && pr.state != "closed";
    let head_label = if pr.head.label.is_empty() {
        pr.head.r#ref.clone()
    } else {
        pr.head.label.clone()
    };
    let base_label = if pr.base.label.is_empty() {
        pr.base.r#ref.clone()
    } else {
        pr.base.label.clone()
    };
    ListItem::new(("helm-pr", ix))
        .child(
            v_flex()
                .gap_0p5()
                .min_w_0()
                .child(
                    div()
                        .truncate()
                        .text_sm()
                        .font_semibold()
                        .text_color(if open { foreground } else { muted_foreground })
                        .child(pr.title.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_foreground)
                        .child(format!("#{number}")),
                ),
        )
        .suffix(move |_, _| {
            h_flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .font_family("Cascadia Mono")
                        .text_color(muted_foreground)
                        .child(format!("{head_label} → {base_label}")),
                )
                .child(
                    Icon::new(IconName::ChevronRight)
                        .xsmall()
                        .text_color(muted_foreground),
                )
        })
}

/// One row of the Branches screen: branch name, default state and protection.
pub fn branch_row(ix: usize, branch: &Branch, default_branch: &str, cx: &App) -> ListItem {
    let muted_foreground = cx.theme().muted_foreground;
    let foreground = cx.theme().foreground;
    let is_default = branch.name == default_branch;
    let protected = branch.protected;
    ListItem::new(("helm-branch", ix))
        .child(div().text_color(foreground).child(branch.name.clone()))
        .suffix(move |_, _| {
            h_flex()
                .items_center()
                .gap_2()
                .when(is_default, |row| row.child(chip("default")))
                .when(protected, |row| {
                    row.child(
                        div()
                            .text_xs()
                            .text_color(muted_foreground)
                            .child("protected"),
                    )
                })
        })
}

/// One row of the Commits screen: the author's avatar, short id and login.
pub fn commit_row(ix: usize, commit: &CommitSummary, cx: &App) -> ListItem {
    let foreground = cx.theme().foreground;
    let muted_foreground = cx.theme().muted_foreground;
    let short_sha: String = commit.sha.chars().take(7).collect();
    let author = commit
        .author
        .as_ref()
        .map(|author| author.login.clone())
        .unwrap_or_else(|| "unknown".to_string());
    let avatar_url = commit
        .author
        .as_ref()
        .map(|author| author.avatar_url.clone())
        .unwrap_or_default();
    ListItem::new(("helm-commit", ix)).child(
        h_flex()
            .items_center()
            .gap_2()
            .child(
                Avatar::new()
                    .src(avatar_url)
                    .name(author.clone())
                    .with_size(gpui::px(20.)),
            )
            .child(
                div()
                    .text_sm()
                    .font_family("Cascadia Mono")
                    .text_color(foreground)
                    .child(short_sha),
            )
            .child(div().text_xs().text_color(muted_foreground).child(author)),
    )
}

/// One row of the Actions screen: workflow name, run number, branch and result.
pub fn workflow_run_row(ix: usize, run: &WorkflowRun, cx: &App) -> ListItem {
    let foreground = cx.theme().foreground;
    let muted_foreground = cx.theme().muted_foreground;
    let status_label = run.conclusion.clone().unwrap_or_else(|| run.status.clone());
    let color = match status_label.as_str() {
        "success" => cx.theme().success,
        "failure" | "cancelled" | "timed_out" => cx.theme().danger,
        _ => muted_foreground,
    };
    ListItem::new(("helm-run", ix))
        .child(
            v_flex()
                .gap_0p5()
                .min_w_0()
                .child(
                    div()
                        .truncate()
                        .text_sm()
                        .font_semibold()
                        .text_color(foreground)
                        .child(run.name.clone()),
                )
                .child(div().text_xs().text_color(muted_foreground).child(format!(
                    "#{} · {}",
                    run.run_number,
                    run.head_branch.clone().unwrap_or_default()
                ))),
        )
        .suffix(move |_, _| {
            div()
                .text_xs()
                .text_color(color)
                .child(status_label.clone())
        })
}

/// The theme color for a GitHub Actions status/conclusion pair.
pub fn workflow_status_color(cx: &App, status: &str, conclusion: &Option<String>) -> gpui::Hsla {
    match conclusion.as_deref() {
        Some("success") => cx.theme().success,
        Some("failure") | Some("cancelled") | Some("timed_out") | Some("action_required") => {
            cx.theme().danger
        }
        Some("skipped") | Some("neutral") => cx.theme().muted_foreground,
        _ if status == "in_progress" || status == "queued" => cx.theme().primary,
        _ => cx.theme().muted_foreground,
    }
}

/// One row of the Tags screen: the tag's name and the short commit it points at.
pub fn tag_row(ix: usize, tag: &Tag, cx: &App) -> ListItem {
    let foreground = cx.theme().foreground;
    let muted_foreground = cx.theme().muted_foreground;
    let short_sha: String = tag.commit.sha.chars().take(7).collect();
    ListItem::new(("helm-tag", ix))
        .child(
            div()
                .text_sm()
                .font_family("Cascadia Mono")
                .text_color(foreground)
                .child(tag.name.clone()),
        )
        .suffix(move |_, _| {
            div()
                .text_xs()
                .text_color(muted_foreground)
                .child(short_sha.clone())
        })
}

/// The second line of a release's row: asset count and a one-line notes preview.
pub fn release_summary(release: &Release) -> String {
    let assets = match release.assets.len() {
        0 => None,
        1 => Some("1 asset".to_string()),
        count => Some(format!("{count} assets")),
    };
    let notes = release
        .body
        .as_deref()
        .map(|body| body.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|body| !body.is_empty())
        .map(|body| {
            let mut preview: String = body.chars().take(120).collect();
            if body.chars().count() > 120 {
                preview.push('…');
            }
            preview
        });
    match (assets, notes) {
        (Some(assets), Some(notes)) => format!("{assets} · {notes}"),
        (Some(assets), None) => assets,
        (None, Some(notes)) => notes,
        (None, None) => "No release notes".to_string(),
    }
}

/// One row of the Releases screen: title and badges, then its one-line summary.
pub fn release_row(ix: usize, release: &Release, cx: &App) -> ListItem {
    let muted_foreground = cx.theme().muted_foreground;
    let foreground = cx.theme().foreground;
    let title = release
        .name
        .clone()
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| release.tag_name.clone());
    ListItem::new(("helm-release", ix))
        .child(
            v_flex()
                .gap_0p5()
                .min_w_0()
                .child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .truncate()
                                .text_sm()
                                .font_semibold()
                                .text_color(foreground)
                                .child(title),
                        )
                        .when(release.draft, |row| row.child(chip("draft")))
                        .when(release.prerelease, |row| {
                            // Tinted, not filled: the filled warning pill's
                            // label is unreadable in themes where the fill
                            // and its foreground are close in colour.
                            let warning = cx.theme().warning;
                            row.child(
                                Pill::custom(warning.opacity(0.2), warning, warning.opacity(0.3))
                                    .xsmall()
                                    .rounded_full()
                                    .child("pre-release"),
                            )
                        }),
                )
                .child(
                    div()
                        .truncate()
                        .text_xs()
                        .text_color(muted_foreground)
                        .child(release_summary(release)),
                ),
        )
        .suffix(move |_, _| {
            Icon::new(IconName::ExternalLink)
                .xsmall()
                .text_color(muted_foreground)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(body: Option<&str>, assets: usize) -> Release {
        let assets: Vec<serde_json::Value> = (0..assets)
            .map(|n| serde_json::json!({ "name": format!("asset-{n}.zip") }))
            .collect();
        serde_json::from_value(serde_json::json!({
            "id": 1,
            "tag_name": "v1.0.0",
            "name": "One",
            "body": body,
            "draft": false,
            "prerelease": false,
            "html_url": "https://github.com/o/r/releases/tag/v1.0.0",
            "assets": assets,
        }))
        .expect("a release the backend's type accepts")
    }

    #[test]
    fn a_release_row_always_has_a_second_line() {
        assert_eq!(release_summary(&release(None, 0)), "No release notes");
        assert_eq!(
            release_summary(&release(Some("   \n  "), 0)),
            "No release notes"
        );
        assert_eq!(release_summary(&release(None, 1)), "1 asset");
        assert_eq!(release_summary(&release(None, 3)), "3 assets");
        assert_eq!(release_summary(&release(Some("Fixes"), 0)), "Fixes");
        assert_eq!(
            release_summary(&release(Some("Fixes"), 2)),
            "2 assets · Fixes"
        );
    }

    #[test]
    fn release_notes_are_flattened_to_one_line_and_cut_at_120() {
        assert_eq!(
            release_summary(&release(Some("## Changes\n\n- one\n- two"), 0)),
            "## Changes - one - two"
        );
        let long = "x".repeat(200);
        let summary = release_summary(&release(Some(&long), 0));
        assert_eq!(summary.chars().count(), 121);
        assert!(summary.ends_with('…'));
    }
}
