use client::test::FakeServer;
use clock::{FakeSystemClock, ReplicaId};
use cloud_api_types::{CreateLlmTokenResponse, LlmToken};
use cloud_llm_client::{PredictEditsBody, PredictEditsResponse};
use gpui::TestAppContext;
use http_client::FakeHttpClient;
use indoc::indoc;
use language::Point;
use parking_lot::Mutex;
use serde_json::json;
use settings::SettingsStore;
use util::{path, rel_path::rel_path};

use crate::zeta1::MAX_EVENT_TOKENS;

use super::*;

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

    let completion = EditPrediction {
        edits,
        edit_preview,
        buffer: buffer.clone(),
        snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
        id: EditPredictionId("the-id".into()),
        inputs: EditPredictionInputs {
            events: Default::default(),
            included_files: Default::default(),
            cursor_point: cloud_llm_client::predict_edits_v3::Point {
                line: Line(0),
                column: 0,
            },
            cursor_path: Path::new("").into(),
        },
        buffer_snapshotted_at: Instant::now(),
        response_received_at: Instant::now(),
    };

    cx.update(|cx| {
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..5, "REM".into()), (9..11, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..2, "REM".into()), (6..8, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..5, "REM".into()), (9..11, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(3..3, "EM".into()), (7..9, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".into()), (8..10, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(9..11, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".into()), (8..10, "".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".into())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
        assert_eq!(completion.interpolate(&buffer.read(cx).snapshot()), None);
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        true
    );

    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Disabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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

    let (zeta, captured_request, _) = make_test_zeta(&project, cx).await;
    zeta.update(cx, |zeta, _cx| {
        zeta.data_collection_choice = DataCollectionChoice::Enabled
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        true
    );

    // this has a side effect of registering the buffer to watch for edits
    run_edit_prediction(&private_buffer, &project, &zeta, cx).await;
    assert_eq!(
        captured_request.lock().clone().unwrap().can_collect_data,
        false
    );

    private_buffer.update(cx, |private_buffer, cx| {
        private_buffer.edit([(0..0, "An edit for the history!")], None, cx);
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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
                " ".repeat(MAX_EVENT_TOKENS * zeta1::BYTES_PER_TOKEN_GUESS),
            )],
            None,
            cx,
        );
    });

    run_edit_prediction(&buffer, &project, &zeta, cx).await;
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
    let (zeta, _, response) = make_test_zeta(&project, cx).await;
    *response.lock() = completion_response.to_string();
    let edit_prediction = run_edit_prediction(&buffer, &project, &zeta, cx).await;
    buffer.update(cx, |buffer, cx| {
        buffer.edit(edit_prediction.edits.iter().cloned(), None, cx)
    });
    buffer.read_with(cx, |buffer, _| buffer.text())
}

async fn run_edit_prediction(
    buffer: &Entity<Buffer>,
    project: &Entity<Project>,
    zeta: &Entity<Zeta>,
    cx: &mut TestAppContext,
) -> EditPrediction {
    let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
    zeta.update(cx, |zeta, cx| zeta.register_buffer(buffer, &project, cx));
    cx.background_executor.run_until_parked();
    let prediction_task = zeta.update(cx, |zeta, cx| {
        zeta.request_prediction(&project, buffer, cursor, Default::default(), cx)
    });
    prediction_task.await.unwrap().unwrap().prediction.unwrap()
}

async fn make_test_zeta(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> (
    Entity<Zeta>,
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

    let zeta = cx.new(|cx| {
        let mut zeta = Zeta::new(client, project.read(cx).user_store(), cx);
        zeta.set_edit_prediction_model(ZetaEditPredictionModel::Zeta1);

        let worktrees = project.read(cx).worktrees(cx).collect::<Vec<_>>();
        for worktree in worktrees {
            let worktree_id = worktree.read(cx).id();
            zeta.get_or_init_zeta_project(project, cx)
                .license_detection_watchers
                .entry(worktree_id)
                .or_insert_with(|| Rc::new(LicenseDetectionWatcher::new(&worktree, cx)));
        }

        zeta
    });

    (zeta, captured_request, completion_response)
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

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}
