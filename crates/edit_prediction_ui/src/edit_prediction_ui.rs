mod edit_prediction_button;
mod edit_prediction_context_view;
mod rate_prediction_modal;

use command_palette_hooks::CommandPaletteFilter;
use edit_prediction::{EditPredictionStore, ResetOnboarding, Zeta2FeatureFlag, capture_example};
use edit_prediction_context_view::EditPredictionContextView;
use editor::Editor;
use feature_flags::FeatureFlagAppExt as _;
use gpui::actions;
use language::language_settings::AllLanguageSettings;
use project::DisableAiSettings;
use rate_prediction_modal::RatePredictionsModal;
use settings::{Settings as _, SettingsStore};
use std::any::{Any as _, TypeId};
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

        workspace.register_action(|workspace, _: &CaptureExample, window, cx| {
            capture_example_as_markdown(workspace, window, cx);
        });
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

fn capture_example_as_markdown(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Option<()> {
    let markdown_language = workspace
        .app_state()
        .languages
        .language_for_name("Markdown");

    let fs = workspace.app_state().fs.clone();
    let project = workspace.project().clone();
    let editor = workspace.active_item_as::<Editor>(cx)?;
    let editor = editor.read(cx);
    let (buffer, cursor_anchor) = editor
        .buffer()
        .read(cx)
        .text_anchor_for_position(editor.selections.newest_anchor().head(), cx)?;
    let ep_store = EditPredictionStore::try_global(cx)?;
    let events = ep_store.update(cx, |store, cx| {
        store.edit_history_for_project_with_pause_split_last_event(&project, cx)
    });
    let example = capture_example(project.clone(), buffer, cursor_anchor, events, true, cx)?;

    let examples_dir = AllLanguageSettings::get_global(cx)
        .edit_predictions
        .examples_dir
        .clone();

    cx.spawn_in(window, async move |workspace_entity, cx| {
        let markdown_language = markdown_language.await?;
        let example_spec = example.await?;
        let buffer = if let Some(dir) = examples_dir {
            fs.create_dir(&dir).await.ok();
            let mut path = dir.join(&example_spec.name.replace(' ', "--").replace(':', "-"));
            path.set_extension("md");
            project
                .update(cx, |project, cx| project.open_local_buffer(&path, cx))
                .await?
        } else {
            project
                .update(cx, |project, cx| project.create_buffer(false, cx))
                .await?
        };

        buffer.update(cx, |buffer, cx| {
            buffer.set_text(example_spec.to_markdown(), cx);
            buffer.set_language(Some(markdown_language), cx);
        });
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
    None
}
