use super::*;
use crate::{compute_diff_between_snapshots, udiff::apply_diff_to_string, zeta1::MAX_EVENT_TOKENS};
use client::{UserStore, test::FakeServer};
use clock::{FakeSystemClock, ReplicaId};
use cloud_api_types::{CreateLlmTokenResponse, LlmToken};
use cloud_llm_client::{
    EditPredictionRejectReason, EditPredictionRejection, PredictEditsBody, PredictEditsResponse,
    RejectEditPredictionsBody,
    predict_edits_v3::{PredictEditsV3Request, PredictEditsV3Response},
};
use futures::{
    AsyncReadExt, StreamExt,
    channel::{mpsc, oneshot},
};
use gpui::App;
use gpui::{
    Entity, TestAppContext,
    http_client::{FakeHttpClient, Response},
};
use indoc::indoc;
use language::{Buffer, Point};
use lsp::LanguageServerId;
use parking_lot::Mutex;
use pretty_assertions::{assert_eq, assert_matches};
use project::{FakeFs, Project};
use serde_json::json;
use settings::SettingsStore;
use std::{path::Path, sync::Arc, time::Duration};
use util::{path, rel_path::rel_path};
use uuid::Uuid;
use zeta_prompt::ZetaPromptInput;

use crate::{BufferEditPrediction, EditPredictionId, EditPredictionStore, REJECT_REQUEST_DEBOUNCE};

#[gpui::test]
async fn test_current_state(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "1.txt": "Hello!\nHow\nBye\n",
            "2.txt": "Hola!\nComo\nAdios\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer1 = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("/root/1.txt"), cx).unwrap();
            project.set_active_path(Some(path.clone()), cx);
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot1 = buffer1.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot1.anchor_before(language::Point::new(1, 3));

    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_project(&project, cx);
        ep_store.register_buffer(&buffer1, &project, cx);
    });

    // Prediction for current file

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer1.clone(), position, cx)
    });
    let (request, respond_tx) = requests.predict.next().await.unwrap();

    respond_tx
        .send(model_response(
            &request,
            indoc! {r"
                --- a/root/1.txt
                +++ b/root/1.txt
                @@ ... @@
                 Hello!
                -How
                +How are you?
                 Bye
            "},
        ))
        .unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        let prediction = ep_store
            .prediction_at(&buffer1, None, &project, cx)
            .unwrap();
        assert_matches!(prediction, BufferEditPrediction::Local { .. });
    });

    ep_store.update(cx, |ep_store, _cx| {
        ep_store.reject_current_prediction(EditPredictionRejectReason::Discarded, &project);
    });

    // Prediction for diagnostic in another file

    let diagnostic = lsp::Diagnostic {
        range: lsp::Range::new(lsp::Position::new(1, 1), lsp::Position::new(1, 5)),
        severity: Some(lsp::DiagnosticSeverity::ERROR),
        message: "Sentence is incomplete".to_string(),
        ..Default::default()
    };

    project.update(cx, |project, cx| {
        project.lsp_store().update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Uri::from_file_path(path!("/root/2.txt")).unwrap(),
                        diagnostics: vec![diagnostic],
                        version: None,
                    },
                    None,
                    language::DiagnosticSourceKind::Pushed,
                    &[],
                    cx,
                )
                .unwrap();
        });
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();
    respond_tx
        .send(model_response(
            &request,
            indoc! {r#"
                --- a/root/2.txt
                +++ b/root/2.txt
                @@ ... @@
                 Hola!
                -Como
                +Como estas?
                 Adios
            "#},
        ))
        .unwrap();
    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        let prediction = ep_store
            .prediction_at(&buffer1, None, &project, cx)
            .unwrap();
        assert_matches!(
            prediction,
            BufferEditPrediction::Jump { prediction } if prediction.snapshot.file().unwrap().full_path(cx) == Path::new(path!("root/2.txt"))
        );
    });

    let buffer2 = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/2.txt"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    ep_store.update(cx, |ep_store, cx| {
        let prediction = ep_store
            .prediction_at(&buffer2, None, &project, cx)
            .unwrap();
        assert_matches!(prediction, BufferEditPrediction::Local { .. });
    });
}

#[gpui::test]
async fn test_simple_request(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    let prediction_task = ep_store.update(cx, |ep_store, cx| {
        ep_store.request_prediction(&project, &buffer, position, Default::default(), cx)
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();

    // TODO Put back when we have a structured request again
    // assert_eq!(
    //     request.excerpt_path.as_ref(),
    //     Path::new(path!("root/foo.md"))
    // );
    // assert_eq!(
    //     request.cursor_point,
    //     Point {
    //         line: Line(1),
    //         column: 3
    //     }
    // );

    respond_tx
        .send(model_response(
            &request,
            indoc! { r"
                --- a/root/foo.md
                +++ b/root/foo.md
                @@ ... @@
                 Hello!
                -How
                +How are you?
                 Bye
            "},
        ))
        .unwrap();

    let prediction = prediction_task.await.unwrap().unwrap().prediction.unwrap();

    assert_eq!(prediction.edits.len(), 1);
    assert_eq!(
        prediction.edits[0].0.to_point(&snapshot).start,
        language::Point::new(1, 3)
    );
    assert_eq!(prediction.edits[0].1.as_ref(), " are you?");
}

#[gpui::test]
async fn test_request_events(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md": "Hello!\n\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_buffer(&buffer, &project, cx);
    });

    buffer.update(cx, |buffer, cx| {
        buffer.edit(vec![(7..7, "How")], None, cx);
    });

    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    let prediction_task = ep_store.update(cx, |ep_store, cx| {
        ep_store.request_prediction(&project, &buffer, position, Default::default(), cx)
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();

    let prompt = prompt_from_request(&request);
    assert!(
        prompt.contains(indoc! {"
        --- a/root/foo.md
        +++ b/root/foo.md
        @@ -1,3 +1,3 @@
         Hello!
        -
        +How
         Bye
    "}),
        "{prompt}"
    );

    respond_tx
        .send(model_response(
            &request,
            indoc! {r#"
                --- a/root/foo.md
                +++ b/root/foo.md
                @@ ... @@
                 Hello!
                -How
                +How are you?
                 Bye
        "#},
        ))
        .unwrap();

    let prediction = prediction_task.await.unwrap().unwrap().prediction.unwrap();

    assert_eq!(prediction.edits.len(), 1);
    assert_eq!(prediction.edits[0].1.as_ref(), " are you?");
}

#[gpui::test]
async fn test_edit_history_getter_pause_splits_last_event(cx: &mut TestAppContext) {
    let (ep_store, _requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md": "Hello!\n\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_buffer(&buffer, &project, cx);
    });

    // First burst: insert "How"
    buffer.update(cx, |buffer, cx| {
        buffer.edit(vec![(7..7, "How")], None, cx);
    });

    // Simulate a pause longer than the grouping threshold (e.g. 500ms).
    cx.executor().advance_clock(LAST_CHANGE_GROUPING_TIME * 2);
    cx.run_until_parked();

    // Second burst: append " are you?" immediately after "How" on the same line.
    //
    // Keeping both bursts on the same line ensures the existing line-span coalescing logic
    // groups them into a single `LastEvent`, allowing the pause-split getter to return two diffs.
    buffer.update(cx, |buffer, cx| {
        buffer.edit(vec![(10..10, " are you?")], None, cx);
    });

    // A second edit shortly after the first post-pause edit ensures the last edit timestamp is
    // advanced after the pause boundary is recorded, making pause-splitting deterministic.
    buffer.update(cx, |buffer, cx| {
        buffer.edit(vec![(19..19, "!")], None, cx);
    });

    // Without time-based splitting, there is one event.
    let events = ep_store.update(cx, |ep_store, cx| {
        ep_store.edit_history_for_project(&project, cx)
    });
    assert_eq!(events.len(), 1);
    let zeta_prompt::Event::BufferChange { diff, .. } = events[0].event.as_ref();
    assert_eq!(
        diff.as_str(),
        indoc! {"
            @@ -1,3 +1,3 @@
             Hello!
            -
            +How are you?!
             Bye
        "}
    );

    // With time-based splitting, there are two distinct events.
    let events = ep_store.update(cx, |ep_store, cx| {
        ep_store.edit_history_for_project_with_pause_split_last_event(&project, cx)
    });
    assert_eq!(events.len(), 2);
    let zeta_prompt::Event::BufferChange { diff, .. } = events[0].event.as_ref();
    assert_eq!(
        diff.as_str(),
        indoc! {"
            @@ -1,3 +1,3 @@
             Hello!
            -
            +How
             Bye
        "}
    );

    let zeta_prompt::Event::BufferChange { diff, .. } = events[1].event.as_ref();
    assert_eq!(
        diff.as_str(),
        indoc! {"
            @@ -1,3 +1,3 @@
             Hello!
            -How
            +How are you?!
             Bye
        "}
    );
}

#[gpui::test]
async fn test_event_grouping_line_span_coalescing(cx: &mut TestAppContext) {
    let (ep_store, _requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());

    // Create a file with 30 lines to test line-based coalescing
    let content = (1..=30)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    fs.insert_tree(
        "/root",
        json!({
            "foo.md": content
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_buffer(&buffer, &project, cx);
    });

    // First edit: multi-line edit spanning rows 10-12 (replacing lines 11-13)
    buffer.update(cx, |buffer, cx| {
        let start = Point::new(10, 0).to_offset(buffer);
        let end = Point::new(13, 0).to_offset(buffer);
        buffer.edit(vec![(start..end, "Middle A\nMiddle B\n")], None, cx);
    });

    let events = ep_store.update(cx, |ep_store, cx| {
        ep_store.edit_history_for_project(&project, cx)
    });
    assert_eq!(
        render_events(&events),
        indoc! {"
            @@ -8,9 +8,8 @@
             Line 8
             Line 9
             Line 10
            -Line 11
            -Line 12
            -Line 13
            +Middle A
            +Middle B
             Line 14
             Line 15
             Line 16
        "},
        "After first edit"
    );

    // Second edit: insert ABOVE the first edit's range (row 5, within 8 lines of row 10)
    // This tests that coalescing considers the START of the existing range
    buffer.update(cx, |buffer, cx| {
        let offset = Point::new(5, 0).to_offset(buffer);
        buffer.edit(vec![(offset..offset, "Above\n")], None, cx);
    });

    let events = ep_store.update(cx, |ep_store, cx| {
        ep_store.edit_history_for_project(&project, cx)
    });
    assert_eq!(
        render_events(&events),
        indoc! {"
            @@ -3,14 +3,14 @@
             Line 3
             Line 4
             Line 5
            +Above
             Line 6
             Line 7
             Line 8
             Line 9
             Line 10
            -Line 11
            -Line 12
            -Line 13
            +Middle A
            +Middle B
             Line 14
             Line 15
             Line 16
        "},
        "After inserting above (should coalesce)"
    );

    // Third edit: insert BELOW the first edit's range (row 14 in current buffer, within 8 lines of row 12)
    // This tests that coalescing considers the END of the existing range
    buffer.update(cx, |buffer, cx| {
        let offset = Point::new(14, 0).to_offset(buffer);
        buffer.edit(vec![(offset..offset, "Below\n")], None, cx);
    });

    let events = ep_store.update(cx, |ep_store, cx| {
        ep_store.edit_history_for_project(&project, cx)
    });
    assert_eq!(
        render_events(&events),
        indoc! {"
            @@ -3,15 +3,16 @@
             Line 3
             Line 4
             Line 5
            +Above
             Line 6
             Line 7
             Line 8
             Line 9
             Line 10
            -Line 11
            -Line 12
            -Line 13
            +Middle A
            +Middle B
             Line 14
            +Below
             Line 15
             Line 16
             Line 17
        "},
        "After inserting below (should coalesce)"
    );

    // Fourth edit: insert FAR BELOW (row 25, beyond 8 lines from the current range end ~row 15)
    // This should NOT coalesce - creates a new event
    buffer.update(cx, |buffer, cx| {
        let offset = Point::new(25, 0).to_offset(buffer);
        buffer.edit(vec![(offset..offset, "Far below\n")], None, cx);
    });

    let events = ep_store.update(cx, |ep_store, cx| {
        ep_store.edit_history_for_project(&project, cx)
    });
    assert_eq!(
        render_events(&events),
        indoc! {"
            @@ -3,15 +3,16 @@
             Line 3
             Line 4
             Line 5
            +Above
             Line 6
             Line 7
             Line 8
             Line 9
             Line 10
            -Line 11
            -Line 12
            -Line 13
            +Middle A
            +Middle B
             Line 14
            +Below
             Line 15
             Line 16
             Line 17

            ---
            @@ -23,6 +23,7 @@
             Line 22
             Line 23
             Line 24
            +Far below
             Line 25
             Line 26
             Line 27
        "},
        "After inserting far below (should NOT coalesce)"
    );
}

fn render_events(events: &[StoredEvent]) -> String {
    events
        .iter()
        .map(|e| {
            let zeta_prompt::Event::BufferChange { diff, .. } = e.event.as_ref();
            diff.as_str()
        })
        .collect::<Vec<_>>()
        .join("\n---\n")
}

#[gpui::test]
async fn test_empty_prediction(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();
    let response = model_response(&request, "");
    let id = response.request_id.clone();
    respond_tx.send(response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        assert!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .is_none()
        );
    });

    // prediction is reported as rejected
    let (reject_request, _) = requests.reject.next().await.unwrap();

    assert_eq!(
        &reject_request.rejections,
        &[EditPredictionRejection {
            request_id: id,
            reason: EditPredictionRejectReason::Empty,
            was_shown: false
        }]
    );
}

#[gpui::test]
async fn test_interpolated_empty(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();

    buffer.update(cx, |buffer, cx| {
        buffer.set_text("Hello!\nHow are you?\nBye", cx);
    });

    let response = model_response(&request, SIMPLE_DIFF);
    let id = response.request_id.clone();
    respond_tx.send(response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        assert!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .is_none()
        );
    });

    // prediction is reported as rejected
    let (reject_request, _) = requests.reject.next().await.unwrap();

    assert_eq!(
        &reject_request.rejections,
        &[EditPredictionRejection {
            request_id: id,
            reason: EditPredictionRejectReason::InterpolatedEmpty,
            was_shown: false
        }]
    );
}

const SIMPLE_DIFF: &str = indoc! { r"
    --- a/root/foo.md
    +++ b/root/foo.md
    @@ ... @@
     Hello!
    -How
    +How are you?
     Bye
"};

#[gpui::test]
async fn test_replace_current(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();
    let first_response = model_response(&request, SIMPLE_DIFF);
    let first_id = first_response.request_id.clone();
    respond_tx.send(first_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            first_id
        );
    });

    // a second request is triggered
    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();
    let second_response = model_response(&request, SIMPLE_DIFF);
    let second_id = second_response.request_id.clone();
    respond_tx.send(second_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // second replaces first
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            second_id
        );
    });

    // first is reported as replaced
    let (reject_request, _) = requests.reject.next().await.unwrap();

    assert_eq!(
        &reject_request.rejections,
        &[EditPredictionRejection {
            request_id: first_id,
            reason: EditPredictionRejectReason::Replaced,
            was_shown: false
        }]
    );
}

#[gpui::test]
async fn test_current_preferred(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();
    let first_response = model_response(&request, SIMPLE_DIFF);
    let first_id = first_response.request_id.clone();
    respond_tx.send(first_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            first_id
        );
    });

    // a second request is triggered
    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_tx) = requests.predict.next().await.unwrap();
    // worse than current prediction
    let second_response = model_response(
        &request,
        indoc! { r"
            --- a/root/foo.md
            +++ b/root/foo.md
            @@ ... @@
             Hello!
            -How
            +How are
             Bye
        "},
    );
    let second_id = second_response.request_id.clone();
    respond_tx.send(second_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // first is preferred over second
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            first_id
        );
    });

    // second is reported as rejected
    let (reject_request, _) = requests.reject.next().await.unwrap();

    assert_eq!(
        &reject_request.rejections,
        &[EditPredictionRejection {
            request_id: second_id,
            reason: EditPredictionRejectReason::CurrentPreferred,
            was_shown: false
        }]
    );
}

#[gpui::test]
async fn test_cancel_earlier_pending_requests(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    // start two refresh tasks
    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request1, respond_first) = requests.predict.next().await.unwrap();

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request, respond_second) = requests.predict.next().await.unwrap();

    // wait for throttle
    cx.run_until_parked();

    // second responds first
    let second_response = model_response(&request, SIMPLE_DIFF);
    let second_id = second_response.request_id.clone();
    respond_second.send(second_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // current prediction is second
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            second_id
        );
    });

    let first_response = model_response(&request1, SIMPLE_DIFF);
    let first_id = first_response.request_id.clone();
    respond_first.send(first_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // current prediction is still second, since first was cancelled
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            second_id
        );
    });

    // first is reported as rejected
    let (reject_request, _) = requests.reject.next().await.unwrap();

    cx.run_until_parked();

    assert_eq!(
        &reject_request.rejections,
        &[EditPredictionRejection {
            request_id: first_id,
            reason: EditPredictionRejectReason::Canceled,
            was_shown: false
        }]
    );
}

#[gpui::test]
async fn test_cancel_second_on_third_request(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "foo.md":  "Hello!\nHow\nBye\n"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();
    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(1, 3));

    // start two refresh tasks
    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request1, respond_first) = requests.predict.next().await.unwrap();

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (request2, respond_second) = requests.predict.next().await.unwrap();

    // wait for throttle, so requests are sent
    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // start a third request
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);

        // 2 are pending, so 2nd is cancelled
        assert_eq!(
            ep_store
                .get_or_init_project(&project, cx)
                .cancelled_predictions
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            [1]
        );
    });

    // wait for throttle
    cx.run_until_parked();

    let (request3, respond_third) = requests.predict.next().await.unwrap();

    let first_response = model_response(&request1, SIMPLE_DIFF);
    let first_id = first_response.request_id.clone();
    respond_first.send(first_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // current prediction is first
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            first_id
        );
    });

    let cancelled_response = model_response(&request2, SIMPLE_DIFF);
    let cancelled_id = cancelled_response.request_id.clone();
    respond_second.send(cancelled_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // current prediction is still first, since second was cancelled
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            first_id
        );
    });

    let third_response = model_response(&request3, SIMPLE_DIFF);
    let third_response_id = third_response.request_id.clone();
    respond_third.send(third_response).unwrap();

    cx.run_until_parked();

    ep_store.update(cx, |ep_store, cx| {
        // third completes and replaces first
        assert_eq!(
            ep_store
                .prediction_at(&buffer, None, &project, cx)
                .unwrap()
                .id
                .0,
            third_response_id
        );
    });

    // second is reported as rejected
    let (reject_request, _) = requests.reject.next().await.unwrap();

    cx.run_until_parked();

    assert_eq!(
        &reject_request.rejections,
        &[
            EditPredictionRejection {
                request_id: cancelled_id,
                reason: EditPredictionRejectReason::Canceled,
                was_shown: false
            },
            EditPredictionRejection {
                request_id: first_id,
                reason: EditPredictionRejectReason::Replaced,
                was_shown: false
            }
        ]
    );
}

#[gpui::test]
async fn test_rejections_flushing(cx: &mut TestAppContext) {
    let (ep_store, mut requests) = init_test_with_fake_client(cx);

    ep_store.update(cx, |ep_store, _cx| {
        ep_store.reject_prediction(
            EditPredictionId("test-1".into()),
            EditPredictionRejectReason::Discarded,
            false,
        );
        ep_store.reject_prediction(
            EditPredictionId("test-2".into()),
            EditPredictionRejectReason::Canceled,
            true,
        );
    });

    cx.executor().advance_clock(REJECT_REQUEST_DEBOUNCE);
    cx.run_until_parked();

    let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
    respond_tx.send(()).unwrap();

    // batched
    assert_eq!(reject_request.rejections.len(), 2);
    assert_eq!(
        reject_request.rejections[0],
        EditPredictionRejection {
            request_id: "test-1".to_string(),
            reason: EditPredictionRejectReason::Discarded,
            was_shown: false
        }
    );
    assert_eq!(
        reject_request.rejections[1],
        EditPredictionRejection {
            request_id: "test-2".to_string(),
            reason: EditPredictionRejectReason::Canceled,
            was_shown: true
        }
    );

    // Reaching batch size limit sends without debounce
    ep_store.update(cx, |ep_store, _cx| {
        for i in 0..70 {
            ep_store.reject_prediction(
                EditPredictionId(format!("batch-{}", i).into()),
                EditPredictionRejectReason::Discarded,
                false,
            );
        }
    });

    // First MAX/2 items are sent immediately
    cx.run_until_parked();
    let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
    respond_tx.send(()).unwrap();

    assert_eq!(reject_request.rejections.len(), 50);
    assert_eq!(reject_request.rejections[0].request_id, "batch-0");
    assert_eq!(reject_request.rejections[49].request_id, "batch-49");

    // Remaining items are debounced with the next batch
    cx.executor().advance_clock(Duration::from_secs(15));
    cx.run_until_parked();

    let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
    respond_tx.send(()).unwrap();

    assert_eq!(reject_request.rejections.len(), 20);
    assert_eq!(reject_request.rejections[0].request_id, "batch-50");
    assert_eq!(reject_request.rejections[19].request_id, "batch-69");

    // Request failure
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.reject_prediction(
            EditPredictionId("retry-1".into()),
            EditPredictionRejectReason::Discarded,
            false,
        );
    });

    cx.executor().advance_clock(REJECT_REQUEST_DEBOUNCE);
    cx.run_until_parked();

    let (reject_request, _respond_tx) = requests.reject.next().await.unwrap();
    assert_eq!(reject_request.rejections.len(), 1);
    assert_eq!(reject_request.rejections[0].request_id, "retry-1");
    // Simulate failure
    drop(_respond_tx);

    // Add another rejection
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.reject_prediction(
            EditPredictionId("retry-2".into()),
            EditPredictionRejectReason::Discarded,
            false,
        );
    });

    cx.executor().advance_clock(REJECT_REQUEST_DEBOUNCE);
    cx.run_until_parked();

    // Retry should include both the failed item and the new one
    let (reject_request, respond_tx) = requests.reject.next().await.unwrap();
    respond_tx.send(()).unwrap();

    assert_eq!(reject_request.rejections.len(), 2);
    assert_eq!(reject_request.rejections[0].request_id, "retry-1");
    assert_eq!(reject_request.rejections[1].request_id, "retry-2");
}

// Skipped until we start including diagnostics in prompt
// #[gpui::test]
// async fn test_request_diagnostics(cx: &mut TestAppContext) {
//     let (ep_store, mut req_rx) = init_test_with_fake_client(cx);
//     let fs = FakeFs::new(cx.executor());
//     fs.insert_tree(
//         "/root",
//         json!({
//             "foo.md": "Hello!\nBye"
//         }),
//     )
//     .await;
//     let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

//     let path_to_buffer_uri = lsp::Uri::from_file_path(path!("/root/foo.md")).unwrap();
//     let diagnostic = lsp::Diagnostic {
//         range: lsp::Range::new(lsp::Position::new(1, 1), lsp::Position::new(1, 5)),
//         severity: Some(lsp::DiagnosticSeverity::ERROR),
//         message: "\"Hello\" deprecated. Use \"Hi\" instead".to_string(),
//         ..Default::default()
//     };

//     project.update(cx, |project, cx| {
//         project.lsp_store().update(cx, |lsp_store, cx| {
//             // Create some diagnostics
//             lsp_store
//                 .update_diagnostics(
//                     LanguageServerId(0),
//                     lsp::PublishDiagnosticsParams {
//                         uri: path_to_buffer_uri.clone(),
//                         diagnostics: vec![diagnostic],
//                         version: None,
//                     },
//                     None,
//                     language::DiagnosticSourceKind::Pushed,
//                     &[],
//                     cx,
//                 )
//                 .unwrap();
//         });
//     });

//     let buffer = project
//         .update(cx, |project, cx| {
//             let path = project.find_project_path(path!("root/foo.md"), cx).unwrap();
//             project.open_buffer(path, cx)
//         })
//         .await
//         .unwrap();

//     let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
//     let position = snapshot.anchor_before(language::Point::new(0, 0));

//     let _prediction_task = ep_store.update(cx, |ep_store, cx| {
//         ep_store.request_prediction(&project, &buffer, position, cx)
//     });

//     let (request, _respond_tx) = req_rx.next().await.unwrap();

//     assert_eq!(request.diagnostic_groups.len(), 1);
//     let value = serde_json::from_str::<serde_json::Value>(request.diagnostic_groups[0].0.get())
//         .unwrap();
//     // We probably don't need all of this. TODO define a specific diagnostic type in predict_edits_v3
//     assert_eq!(
//         value,
//         json!({
//             "entries": [{
//                 "range": {
//                     "start": 8,
//                     "end": 10
//                 },
//                 "diagnostic": {
//                     "source": null,
//                     "code": null,
//                     "code_description": null,
//                     "severity": 1,
//                     "message": "\"Hello\" deprecated. Use \"Hi\" instead",
//                     "markdown": null,
//                     "group_id": 0,
//                     "is_primary": true,
//                     "is_disk_based": false,
//                     "is_unnecessary": false,
//                     "source_kind": "Pushed",
//                     "data": null,
//                     "underline": true
//                 }
//             }],
//             "primary_ix": 0
//         })
//     );
// }

// Generate a model response that would apply the given diff to the active file.
fn model_response(request: &PredictEditsV3Request, diff_to_apply: &str) -> PredictEditsV3Response {
    let excerpt =
        request.input.cursor_excerpt[request.input.editable_range_in_excerpt.clone()].to_string();
    let new_excerpt = apply_diff_to_string(diff_to_apply, &excerpt).unwrap();

    PredictEditsV3Response {
        request_id: Uuid::new_v4().to_string(),
        output: new_excerpt,
    }
}

fn prompt_from_request(request: &PredictEditsV3Request) -> String {
    zeta_prompt::format_zeta_prompt(&request.input, request.prompt_version)
}

struct RequestChannels {
    predict: mpsc::UnboundedReceiver<(
        PredictEditsV3Request,
        oneshot::Sender<PredictEditsV3Response>,
    )>,
    reject: mpsc::UnboundedReceiver<(RejectEditPredictionsBody, oneshot::Sender<()>)>,
}

fn init_test_with_fake_client(
    cx: &mut TestAppContext,
) -> (Entity<EditPredictionStore>, RequestChannels) {
    cx.update(move |cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        zlog::init_test();

        let (predict_req_tx, predict_req_rx) = mpsc::unbounded();
        let (reject_req_tx, reject_req_rx) = mpsc::unbounded();

        let http_client = FakeHttpClient::create({
            move |req| {
                let uri = req.uri().path().to_string();
                let mut body = req.into_body();
                let predict_req_tx = predict_req_tx.clone();
                let reject_req_tx = reject_req_tx.clone();
                async move {
                    let resp = match uri.as_str() {
                        "/client/llm_tokens" => serde_json::to_string(&json!({
                            "token": "test"
                        }))
                        .unwrap(),
                        "/predict_edits/v3" => {
                            let mut buf = Vec::new();
                            body.read_to_end(&mut buf).await.ok();
                            let req = serde_json::from_slice(&buf).unwrap();

                            let (res_tx, res_rx) = oneshot::channel();
                            predict_req_tx.unbounded_send((req, res_tx)).unwrap();
                            serde_json::to_string(&res_rx.await?).unwrap()
                        }
                        "/predict_edits/reject" => {
                            let mut buf = Vec::new();
                            body.read_to_end(&mut buf).await.ok();
                            let req = serde_json::from_slice(&buf).unwrap();

                            let (res_tx, res_rx) = oneshot::channel();
                            reject_req_tx.unbounded_send((req, res_tx)).unwrap();
                            serde_json::to_string(&res_rx.await?).unwrap()
                        }
                        _ => {
                            panic!("Unexpected path: {}", uri)
                        }
                    };

                    Ok(Response::builder().body(resp.into()).unwrap())
                }
            }
        });

        let client = client::Client::new(Arc::new(FakeSystemClock::new()), http_client, cx);
        client.cloud_client().set_credentials(1, "test".into());

        language_model::init(client.clone(), cx);

        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let ep_store = EditPredictionStore::global(&client, &user_store, cx);

        (
            ep_store,
            RequestChannels {
                predict: predict_req_rx,
                reject: reject_req_rx,
            },
        )
    })
}

const BSD_0_TXT: &str = include_str!("../license_examples/0bsd.txt");

#[gpui::test]
async fn test_edit_prediction_basic_interpolation(cx: &mut TestAppContext) {
    let buffer = cx.new(|cx| Buffer::local("Lorem ipsum dolor", cx));
    let edits: Arc<[(Range<Anchor>, Arc<str>)]> = cx.update(|cx| {
        to_completion_edits([(2..5, "REM".into()), (9..11, "".into())], &buffer, cx).into()
    });

    let edit_preview = cx
        .read(|cx| buffer.read(cx).preview_edits(edits.clone(), cx))
        .await;

    let prediction = EditPrediction {
        edits,
        edit_preview,
        buffer: buffer.clone(),
        snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
        id: EditPredictionId("the-id".into()),
        inputs: ZetaPromptInput {
            events: Default::default(),
            related_files: Default::default(),
            cursor_path: Path::new("").into(),
            cursor_excerpt: "".into(),
            editable_range_in_excerpt: 0..0,
            cursor_offset_in_excerpt: 0,
            excerpt_start_row: None,
        },
        buffer_snapshotted_at: Instant::now(),
        response_received_at: Instant::now(),
    };

    cx.update(|cx| {
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..5, "REM".into()), (9..11, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..2, "REM".into()), (6..8, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..5, "REM".into()), (9..11, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(3..3, "EM".into()), (7..9, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".into()), (8..10, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(9..11, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".into()), (8..10, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &prediction.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
        assert_eq!(prediction.interpolate(&buffer.read(cx).snapshot()), None);
    })
}

#[gpui::test]
async fn test_clean_up_diff(cx: &mut TestAppContext) {
    init_test(cx);

    assert_eq!(
        apply_edit_prediction(
            indoc! {"
                    fn main() {
                        let word_1 = \"lorem\";
                        let range = word.len()..word.len();
                    }
                "},
            indoc! {"
                    <|editable_region_start|>
                    fn main() {
                        let word_1 = \"lorem\";
                        let range = word_1.len()..word_1.len();
                    }

                    <|editable_region_end|>
                "},
            cx,
        )
        .await,
        indoc! {"
                fn main() {
                    let word_1 = \"lorem\";
                    let range = word_1.len()..word_1.len();
                }
            "},
    );

    assert_eq!(
        apply_edit_prediction(
            indoc! {"
                    fn main() {
                        let story = \"the quick\"
                    }
                "},
            indoc! {"
                    <|editable_region_start|>
                    fn main() {
                        let story = \"the quick brown fox jumps over the lazy dog\";
                    }

                    <|editable_region_end|>
                "},
            cx,
        )
        .await,
        indoc! {"
                fn main() {
                    let story = \"the quick brown fox jumps over the lazy dog\";
                }
            "},
    );
}

#[gpui::test]
async fn test_edit_prediction_end_of_buffer(cx: &mut TestAppContext) {
    init_test(cx);

    let buffer_content = "lorem\n";
    let completion_response = indoc! {"
            ```animals.js
            <|start_of_file|>
            <|editable_region_start|>
            lorem
            ipsum
            <|editable_region_end|>
            ```"};

    assert_eq!(
        apply_edit_prediction(buffer_content, completion_response, cx).await,
        "lorem\nipsum"
    );
}

#[gpui::test]
async fn test_edit_prediction_no_spurious_trailing_newline(cx: &mut TestAppContext) {
    // Test that zeta2's newline normalization logic doesn't insert spurious newlines.
    // When the buffer ends without a trailing newline, but the model returns output
    // with a trailing newline, zeta2 should normalize both sides before diffing
    // so no spurious newline is inserted.
    let (ep_store, mut requests) = init_test_with_fake_client(cx);
    let fs = FakeFs::new(cx.executor());

    // Single line buffer with no trailing newline
    fs.insert_tree(
        "/root",
        json!({
            "foo.txt": "hello"
        }),
    )
    .await;
    let project = Project::test(fs, vec![path!("/root").as_ref()], cx).await;

    let buffer = project
        .update(cx, |project, cx| {
            let path = project
                .find_project_path(path!("root/foo.txt"), cx)
                .unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
    let position = snapshot.anchor_before(language::Point::new(0, 5));

    ep_store.update(cx, |ep_store, cx| {
        ep_store.refresh_prediction_from_buffer(project.clone(), buffer.clone(), position, cx);
    });

    let (_request, respond_tx) = requests.predict.next().await.unwrap();

    // Model returns output WITH a trailing newline, even though the buffer doesn't have one.
    // Zeta2 should normalize both sides before diffing, so no spurious newline is inserted.
    let response = PredictEditsV3Response {
        request_id: Uuid::new_v4().to_string(),
        output: "hello world\n".to_string(),
    };
    respond_tx.send(response).unwrap();

    cx.run_until_parked();

    // The prediction should insert " world" without adding a newline
    ep_store.update(cx, |ep_store, cx| {
        let prediction = ep_store
            .prediction_at(&buffer, None, &project, cx)
            .expect("should have prediction");
        let edits: Vec<_> = prediction
            .edits
            .iter()
            .map(|(range, text)| {
                let snapshot = buffer.read(cx).snapshot();
                (range.to_offset(&snapshot), text.clone())
            })
            .collect();
        assert_eq!(edits, vec![(5..5, " world".into())]);
    });
}

#[gpui::test]
async fn test_can_collect_data(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({ "LICENSE": BSD_0_TXT }))
        .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/src/main.rs"), cx)
        })
        .await
        .unwrap();

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        true
    );

    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Disabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );
}

#[gpui::test]
async fn test_no_data_collection_for_remote_file(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), [], cx).await;

    let buffer = cx.new(|_cx| {
        Buffer::remote(
            language::BufferId::new(1).unwrap(),
            ReplicaId::new(1),
            language::Capability::ReadWrite,
            "fn main() {\n    println!(\"Hello\");\n}",
        )
    });

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );
}

#[gpui::test]
async fn test_no_data_collection_for_private_file(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            "LICENSE": BSD_0_TXT,
            ".env": "SECRET_KEY=secret"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/project/.env", cx)
        })
        .await
        .unwrap();

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );
}

#[gpui::test]
async fn test_no_data_collection_for_untitled_buffer(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), [], cx).await;
    let buffer = cx.new(|cx| Buffer::local("", cx));

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );
}

#[gpui::test]
async fn test_no_data_collection_when_closed_source(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/project/main.rs", cx)
        })
        .await
        .unwrap();

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );
}

#[gpui::test]
async fn test_data_collection_status_changes_on_move(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/open_source_worktree"),
        json!({ "LICENSE": BSD_0_TXT, "main.rs": "" }),
    )
    .await;
    fs.insert_tree(path!("/closed_source_worktree"), json!({ "main.rs": "" }))
        .await;

    let project = Project::test(
        fs.clone(),
        [
            path!("/open_source_worktree").as_ref(),
            path!("/closed_source_worktree").as_ref(),
        ],
        cx,
    )
    .await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/open_source_worktree/main.rs"), cx)
        })
        .await
        .unwrap();

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        true
    );

    let closed_source_file = project
        .update(cx, |project, cx| {
            let worktree2 = project
                .worktree_for_root_name("closed_source_worktree", cx)
                .unwrap();
            worktree2.update(cx, |worktree2, cx| {
                worktree2.load_file(rel_path("main.rs"), cx)
            })
        })
        .await
        .unwrap()
        .file;

    buffer.update(cx, |buffer, cx| {
        buffer.file_updated(closed_source_file, cx);
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );
}

#[gpui::test]
async fn test_no_data_collection_for_events_in_uncollectable_buffers(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = project::FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/worktree1"),
        json!({ "LICENSE": BSD_0_TXT, "main.rs": "", "other.rs": "" }),
    )
    .await;
    fs.insert_tree(path!("/worktree2"), json!({ "private.rs": "" }))
        .await;

    let project = Project::test(
        fs.clone(),
        [path!("/worktree1").as_ref(), path!("/worktree2").as_ref()],
        cx,
    )
    .await;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/worktree1/main.rs"), cx)
        })
        .await
        .unwrap();
    let private_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/worktree2/file.rs"), cx)
        })
        .await
        .unwrap();

    let (ep_store, captured_request, _) = make_test_ep_store(&project, cx).await;
    ep_store.update(cx, |ep_store, _cx| {
        ep_store.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        true
    );

    // this has a side effect of registering the buffer to watch for edits
    run_edit_prediction(&private_buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );

    private_buffer.update(cx, |private_buffer, cx| {
        private_buffer.edit([(0..0, "An edit for the history!")], None, cx);
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );

    // make an edit that uses too many bytes, causing private_buffer edit to not be able to be
    // included
    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            [(
                0..0,
                " ".repeat(MAX_EVENT_TOKENS * cursor_excerpt::BYTES_PER_TOKEN_GUESS),
            )],
            None,
            cx,
        );
    });

    run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        true
    );
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });
}

async fn apply_edit_prediction(
    buffer_content: &str,
    completion_response: &str,
    cx: &mut TestAppContext,
) -> String {
    let fs = project::FakeFs::new(cx.executor());
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let buffer = cx.new(|cx| Buffer::local(buffer_content, cx));
    let (ep_store, _, response) = make_test_ep_store(&project, cx).await;
    *response.lock() = completion_response.to_string();
    let edit_prediction = run_edit_prediction(&buffer, &project, &ep_store, cx).await;
    buffer.update(cx, |buffer, cx| {
        buffer.edit(edit_prediction.edits.iter().cloned(), None, cx)
    });
    buffer.read_with(cx, |buffer, _| buffer.text())
}

async fn run_edit_prediction(
    buffer: &Entity<Buffer>,
    project: &Entity<Project>,
    ep_store: &Entity<EditPredictionStore>,
    cx: &mut TestAppContext,
) -> EditPrediction {
    let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_buffer(buffer, &project, cx)
    });
    cx.background_executor.run_until_parked();
    let prediction_task = ep_store.update(cx, |ep_store, cx| {
        ep_store.request_prediction(&project, buffer, cursor, Default::default(), cx)
    });
    prediction_task.await.unwrap().unwrap().prediction.unwrap()
}

async fn make_test_ep_store(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> (
    Entity<EditPredictionStore>,
    Arc<Mutex<Option<PredictEditsBody>>>,
    Arc<Mutex<String>>,
) {
    let default_response = indoc! {"
            ```main.rs
            <|start_of_file|>
            <|editable_region_start|>
            hello world
            <|editable_region_end|>
            ```"
    };
    let captured_request: Arc<Mutex<Option<PredictEditsBody>>> = Arc::new(Mutex::new(None));
    let completion_response: Arc<Mutex<String>> =
        Arc::new(Mutex::new(default_response.to_string()));
    let http_client = FakeHttpClient::create({
        let captured_request = captured_request.clone();
        let completion_response = completion_response.clone();
        let mut next_request_id = 0;
        move |req| {
            let captured_request = captured_request.clone();
            let completion_response = completion_response.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, "/client/llm_tokens") => Ok(http_client::Response::builder()
                        .status(200)
                        .body(
                            serde_json::to_string(&CreateLlmTokenResponse {
                                token: LlmToken("the-llm-token".to_string()),
                            })
                            .unwrap()
                            .into(),
                        )
                        .unwrap()),
                    (&Method::POST, "/predict_edits/v2") => {
                        let mut request_body = String::new();
                        req.into_body().read_to_string(&mut request_body).await?;
                        *captured_request.lock() =
                            Some(serde_json::from_str(&request_body).unwrap());
                        next_request_id += 1;
                        Ok(http_client::Response::builder()
                            .status(200)
                            .body(
                                serde_json::to_string(&PredictEditsResponse {
                                    request_id: format!("request-{next_request_id}"),
                                    output_excerpt: completion_response.lock().clone(),
                                })
                                .unwrap()
                                .into(),
                            )
                            .unwrap())
                    }
                    _ => Ok(http_client::Response::builder()
                        .status(404)
                        .body("Not Found".into())
                        .unwrap()),
                }
            }
        }
    });

    let client = cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
    cx.update(|cx| {
        RefreshLlmTokenListener::register(client.clone(), cx);
    });
    let _server = FakeServer::for_client(42, &client, cx).await;

    let ep_store = cx.new(|cx| {
        let mut ep_store = EditPredictionStore::new(client, project.read(cx).user_store(), cx);
        ep_store.set_edit_prediction_model(EditPredictionModel::Zeta1);

        let worktrees = project.read(cx).worktrees(cx).collect::<Vec<_>>();
        for worktree in worktrees {
            let worktree_id = worktree.read(cx).id();
            ep_store
                .get_or_init_project(project, cx)
                .license_detection_watchers
                .entry(worktree_id)
                .or_insert_with(|| Rc::new(LicenseDetectionWatcher::new(&worktree, cx)));
        }

        ep_store
    });

    (ep_store, captured_request, completion_response)
}

fn to_completion_edits(
    iterator: impl IntoIterator<Item = (Range<usize>, Arc<str>)>,
    buffer: &Entity<Buffer>,
    cx: &App,
) -> Vec<(Range<Anchor>, Arc<str>)> {
    let buffer = buffer.read(cx);
    iterator
        .into_iter()
        .map(|(range, text)| {
            (
                buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                text,
            )
        })
        .collect()
}

fn from_completion_edits(
    editor_edits: &[(Range<Anchor>, Arc<str>)],
    buffer: &Entity<Buffer>,
    cx: &App,
) -> Vec<(Range<usize>, Arc<str>)> {
    let buffer = buffer.read(cx);
    editor_edits
        .iter()
        .map(|(range, text)| {
            (
                range.start.to_offset(buffer)..range.end.to_offset(buffer),
                text.clone(),
            )
        })
        .collect()
}

#[gpui::test]
async fn test_unauthenticated_without_custom_url_blocks_prediction_impl(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project",
        serde_json::json!({
            "main.rs": "fn main() {\n    \n}\n"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

    let http_client = FakeHttpClient::create(|_req| async move {
        Ok(gpui::http_client::Response::builder()
            .status(401)
            .body("Unauthorized".into())
            .unwrap())
    });

    let client =
        cx.update(|cx| client::Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
    cx.update(|cx| {
        language_model::RefreshLlmTokenListener::register(client.clone(), cx);
    });

    let ep_store = cx.new(|cx| EditPredictionStore::new(client, project.read(cx).user_store(), cx));

    let buffer = project
        .update(cx, |project, cx| {
            let path = project
                .find_project_path(path!("/project/main.rs"), cx)
                .unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 4)));
    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_buffer(&buffer, &project, cx)
    });
    cx.background_executor.run_until_parked();

    let completion_task = ep_store.update(cx, |ep_store, cx| {
        ep_store.set_edit_prediction_model(EditPredictionModel::Zeta1);
        ep_store.request_prediction(&project, &buffer, cursor, Default::default(), cx)
    });

    let result = completion_task.await;
    assert!(
        result.is_err(),
        "Without authentication and without custom URL, prediction should fail"
    );
}

#[gpui::test]
async fn test_unauthenticated_with_custom_url_allows_prediction_impl(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/project",
        serde_json::json!({
            "main.rs": "fn main() {\n    \n}\n"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;

    let predict_called = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let predict_called_clone = predict_called.clone();

    let http_client = FakeHttpClient::create({
        move |req| {
            let uri = req.uri().path().to_string();
            let predict_called = predict_called_clone.clone();
            async move {
                if uri.contains("predict") {
                    predict_called.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(gpui::http_client::Response::builder()
                        .body(
                            serde_json::to_string(&open_ai::Response {
                                id: "test-123".to_string(),
                                object: "chat.completion".to_string(),
                                created: 0,
                                model: "test".to_string(),
                                usage: open_ai::Usage {
                                    prompt_tokens: 0,
                                    completion_tokens: 0,
                                    total_tokens: 0,
                                },
                                choices: vec![open_ai::Choice {
                                    index: 0,
                                    message: open_ai::RequestMessage::Assistant {
                                        content: Some(open_ai::MessageContent::Plain(
                                            indoc! {"
                                                ```main.rs
                                                <|start_of_file|>
                                                <|editable_region_start|>
                                                fn main() {
                                                    println!(\"Hello, world!\");
                                                }
                                                <|editable_region_end|>
                                                ```
                                            "}
                                            .to_string(),
                                        )),
                                        tool_calls: vec![],
                                    },
                                    finish_reason: Some("stop".to_string()),
                                }],
                            })
                            .unwrap()
                            .into(),
                        )
                        .unwrap())
                } else {
                    Ok(gpui::http_client::Response::builder()
                        .status(401)
                        .body("Unauthorized".into())
                        .unwrap())
                }
            }
        }
    });

    let client =
        cx.update(|cx| client::Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
    cx.update(|cx| {
        language_model::RefreshLlmTokenListener::register(client.clone(), cx);
    });

    let ep_store = cx.new(|cx| EditPredictionStore::new(client, project.read(cx).user_store(), cx));

    let buffer = project
        .update(cx, |project, cx| {
            let path = project
                .find_project_path(path!("/project/main.rs"), cx)
                .unwrap();
            project.open_buffer(path, cx)
        })
        .await
        .unwrap();

    let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 4)));
    ep_store.update(cx, |ep_store, cx| {
        ep_store.register_buffer(&buffer, &project, cx)
    });
    cx.background_executor.run_until_parked();

    let completion_task = ep_store.update(cx, |ep_store, cx| {
        ep_store.set_custom_predict_edits_url(Url::parse("http://test/predict").unwrap());
        ep_store.set_edit_prediction_model(EditPredictionModel::Zeta1);
        ep_store.request_prediction(&project, &buffer, cursor, Default::default(), cx)
    });

    let _ = completion_task.await;

    assert!(
        predict_called.load(std::sync::atomic::Ordering::SeqCst),
        "With custom URL, predict endpoint should be called even without authentication"
    );
}

#[gpui::test]
fn test_compute_diff_between_snapshots(cx: &mut TestAppContext) {
    let buffer = cx.new(|cx| {
        Buffer::local(
            indoc! {"
                zero
                one
                two
                three
                four
                five
                six
                seven
                eight
                nine
                ten
                eleven
                twelve
                thirteen
                fourteen
                fifteen
                sixteen
                seventeen
                eighteen
                nineteen
                twenty
                twenty-one
                twenty-two
                twenty-three
                twenty-four
            "},
            cx,
        )
    });

    let old_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());

    buffer.update(cx, |buffer, cx| {
        let point = Point::new(12, 0);
        buffer.edit([(point..point, "SECOND INSERTION\n")], None, cx);
        let point = Point::new(8, 0);
        buffer.edit([(point..point, "FIRST INSERTION\n")], None, cx);
    });

    let new_snapshot = buffer.read_with(cx, |buffer, _| buffer.text_snapshot());

    let diff = compute_diff_between_snapshots(&old_snapshot, &new_snapshot).unwrap();

    assert_eq!(
        diff,
        indoc! {"
            @@ -6,10 +6,12 @@
             five
             six
             seven
            +FIRST INSERTION
             eight
             nine
             ten
             eleven
            +SECOND INSERTION
             twelve
             thirteen
             fourteen
            "}
    );
}

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}
