mod edit_prediction_button;
mod edit_prediction_context_view;
mod rate_prediction_modal;

use std::any::{Any as _, TypeId};
use std::path::Path;
use std::sync::Arc;

use command_palette_hooks::CommandPaletteFilter;
use edit_prediction::{
    EditPredictionStore, ResetOnboarding, Zeta2FeatureFlag, example_spec::ExampleSpec,
};
use edit_prediction_context_view::EditPredictionContextView;
use editor::Editor;
use feature_flags::FeatureFlagAppExt as _;
use git::repository::DiffType;
use gpui::{Window, actions};
use language::ToPoint as _;
use log;
use project::DisableAiSettings;
use rate_prediction_modal::RatePredictionsModal;
use settings::{Settings as _, SettingsStore};
use text::ToOffset as _;
use ui::{App, prelude::*};
use workspace::{SplitDirection, Workspace};

pub use edit_prediction_button::{EditPredictionButton, ToggleMenu};

use crate::rate_prediction_modal::PredictEditsRatePredictionsFeatureFlag;

actions!(
    dev,
    [
        /// Opens the edit prediction context view.
        OpenEditPredictionContextView,
    ]
);

actions!(
    edit_prediction,
    [
        /// Opens the rate completions modal.
        RatePredictions,
        /// Captures an ExampleSpec from the current editing session and opens it as Markdown.
        CaptureExample,
    ]
);

pub fn init(cx: &mut App) {
    feature_gate_predict_edits_actions(cx);

    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &RatePredictions, window, cx| {
            if cx.has_flag::<PredictEditsRatePredictionsFeatureFlag>() {
                RatePredictionsModal::toggle(workspace, window, cx);
            }
        });

        workspace.register_action(capture_edit_prediction_example);
        workspace.register_action_renderer(|div, _, _, cx| {
            let has_flag = cx.has_flag::<Zeta2FeatureFlag>();
            div.when(has_flag, |div| {
                div.on_action(cx.listener(
                    move |workspace, _: &OpenEditPredictionContextView, window, cx| {
                        let project = workspace.project();
                        workspace.split_item(
                            SplitDirection::Right,
                            Box::new(cx.new(|cx| {
                                EditPredictionContextView::new(
                                    project.clone(),
                                    workspace.client(),
                                    workspace.user_store(),
                                    window,
                                    cx,
                                )
                            })),
                            window,
                            cx,
                        );
                    },
                ))
            })
        });
    })
    .detach();
}

fn feature_gate_predict_edits_actions(cx: &mut App) {
    let rate_completion_action_types = [TypeId::of::<RatePredictions>()];
    let reset_onboarding_action_types = [TypeId::of::<ResetOnboarding>()];
    let all_action_types = [
        TypeId::of::<RatePredictions>(),
        TypeId::of::<CaptureExample>(),
        TypeId::of::<edit_prediction::ResetOnboarding>(),
        zed_actions::OpenZedPredictOnboarding.type_id(),
        TypeId::of::<edit_prediction::ClearHistory>(),
        TypeId::of::<rate_prediction_modal::ThumbsUpActivePrediction>(),
        TypeId::of::<rate_prediction_modal::ThumbsDownActivePrediction>(),
        TypeId::of::<rate_prediction_modal::NextEdit>(),
        TypeId::of::<rate_prediction_modal::PreviousEdit>(),
    ];

    CommandPaletteFilter::update_global(cx, |filter, _cx| {
        filter.hide_action_types(&rate_completion_action_types);
        filter.hide_action_types(&reset_onboarding_action_types);
        filter.hide_action_types(&[zed_actions::OpenZedPredictOnboarding.type_id()]);
    });

    cx.observe_global::<SettingsStore>(move |cx| {
        let is_ai_disabled = DisableAiSettings::get_global(cx).disable_ai;
        let has_feature_flag = cx.has_flag::<PredictEditsRatePredictionsFeatureFlag>();

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            if is_ai_disabled {
                filter.hide_action_types(&all_action_types);
            } else if has_feature_flag {
                filter.show_action_types(&rate_completion_action_types);
            } else {
                filter.hide_action_types(&rate_completion_action_types);
            }
        });
    })
    .detach();

    cx.observe_flag::<PredictEditsRatePredictionsFeatureFlag, _>(move |is_enabled, cx| {
        if !DisableAiSettings::get_global(cx).disable_ai {
            if is_enabled {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.show_action_types(&rate_completion_action_types);
                });
            } else {
                CommandPaletteFilter::update_global(cx, |filter, _cx| {
                    filter.hide_action_types(&rate_completion_action_types);
                });
            }
        }
    })
    .detach();
}

fn capture_edit_prediction_example(
    workspace: &mut Workspace,
    _: &CaptureExample,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(ep_store) = EditPredictionStore::try_global(cx) else {
        return;
    };

    let project = workspace.project().clone();

    let (worktree_root, repository) = {
        let project_ref = project.read(cx);
        let worktree_root = project_ref
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path());
        let repository = project_ref.active_repository(cx);
        (worktree_root, repository)
    };

    let (Some(worktree_root), Some(repository)) = (worktree_root, repository) else {
        log::error!("CaptureExampleSpec: missing worktree or active repository");
        return;
    };

    let repository_snapshot = repository.read(cx).snapshot();
    if worktree_root.as_ref() != repository_snapshot.work_directory_abs_path.as_ref() {
        log::error!(
            "repository is not at worktree root (repo={:?}, worktree={:?})",
            repository_snapshot.work_directory_abs_path,
            worktree_root
        );
        return;
    }

    let Some(repository_url) = repository_snapshot
        .remote_origin_url
        .clone()
        .or_else(|| repository_snapshot.remote_upstream_url.clone())
    else {
        log::error!("active repository has no origin/upstream remote url");
        return;
    };

    let Some(revision) = repository_snapshot
        .head_commit
        .as_ref()
        .map(|commit| commit.sha.to_string())
    else {
        log::error!("active repository has no head commit");
        return;
    };

    let mut events = ep_store.update(cx, |store, cx| {
        store.edit_history_for_project_with_pause_split_last_event(&project, cx)
    });

    let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
        log::error!("no active editor");
        return;
    };

    let Some(project_path) = editor.read(cx).project_path(cx) else {
        log::error!("active editor has no project path");
        return;
    };

    let Some((buffer, cursor_anchor)) = editor
        .read(cx)
        .buffer()
        .read(cx)
        .text_anchor_for_position(editor.read(cx).selections.newest_anchor().head(), cx)
    else {
        log::error!("failed to resolve cursor buffer/anchor");
        return;
    };

    let snapshot = buffer.read(cx).snapshot();
    let cursor_point = cursor_anchor.to_point(&snapshot);
    let (_editable_range, context_range) =
        edit_prediction::cursor_excerpt::editable_and_context_ranges_for_cursor_position(
            cursor_point,
            &snapshot,
            100,
            50,
        );

    let cursor_path: Arc<Path> = repository
        .read(cx)
        .project_path_to_repo_path(&project_path, cx)
        .map(|repo_path| Path::new(repo_path.as_unix_str()).into())
        .unwrap_or_else(|| Path::new(project_path.path.as_unix_str()).into());

    let cursor_position = {
        let context_start_offset = context_range.start.to_offset(&snapshot);
        let cursor_offset = cursor_anchor.to_offset(&snapshot);
        let cursor_offset_in_excerpt = cursor_offset.saturating_sub(context_start_offset);
        let mut excerpt = snapshot.text_for_range(context_range).collect::<String>();
        if cursor_offset_in_excerpt <= excerpt.len() {
            excerpt.insert_str(cursor_offset_in_excerpt, zeta_prompt::CURSOR_MARKER);
        }
        excerpt
    };

    let markdown_language = workspace
        .app_state()
        .languages
        .language_for_name("Markdown");

    cx.spawn_in(window, async move |workspace_entity, cx| {
        let markdown_language = markdown_language.await?;

        let uncommitted_diff_rx = repository.update(cx, |repository, cx| {
            repository.diff(DiffType::HeadToWorktree, cx)
        })?;

        let uncommitted_diff = match uncommitted_diff_rx.await {
            Ok(Ok(diff)) => diff,
            Ok(Err(error)) => {
                log::error!("failed to compute uncommitted diff: {error:#}");
                return Ok(());
            }
            Err(error) => {
                log::error!("uncommitted diff channel dropped: {error:#}");
                return Ok(());
            }
        };

        let mut edit_history = String::new();
        let mut expected_patch = String::new();
        if let Some(last_event) = events.pop() {
            for event in &events {
                zeta_prompt::write_event(&mut edit_history, event);
                if !edit_history.ends_with('\n') {
                    edit_history.push('\n');
                }
                edit_history.push('\n');
            }

            zeta_prompt::write_event(&mut expected_patch, &last_event);
        }

        let format =
            time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]");
        let name = match format {
            Ok(format) => {
                let now = time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
                now.format(&format)
                    .unwrap_or_else(|_| "unknown-time".to_string())
            }
            Err(_) => "unknown-time".to_string(),
        };

        let markdown = ExampleSpec {
            name,
            repository_url,
            revision,
            uncommitted_diff,
            cursor_path,
            cursor_position,
            edit_history,
            expected_patch,
        }
        .to_markdown();

        let buffer = project
            .update(cx, |project, cx| project.create_buffer(false, cx))?
            .await?;
        buffer.update(cx, |buffer, cx| {
            buffer.set_text(markdown, cx);
            buffer.set_language(Some(markdown_language), cx);
        })?;

        workspace_entity.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(
                Box::new(
                    cx.new(|cx| Editor::for_buffer(buffer, Some(project.clone()), window, cx)),
                ),
                None,
                true,
                window,
                cx,
            );
        })
    })
    .detach_and_log_err(cx);
}
