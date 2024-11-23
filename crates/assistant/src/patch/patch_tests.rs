use super::*;
use fs::FakeFs;
use gpui::{AppContext, Context, TestAppContext};
use language::{
    language_settings::AllLanguageSettings, Buffer, Language, LanguageConfig, LanguageMatcher,
};
use serde_json::json;
use settings::SettingsStore;
use ui::BorrowAppContext;
use unindent::Unindent as _;
use util::test::{generate_marked_text, marked_text_ranges};

#[gpui::test]
async fn test_patch_store(cx: &mut TestAppContext) {
    let settings_store = cx.update(SettingsStore::test);
    cx.set_global(settings_store);
    cx.update(language::init);
    cx.update(Project::init_settings);

    let fs = FakeFs::new(cx.background_executor.clone());

    fs.insert_tree(
        "/root",
        json!({
            "src": {
                "lib.rs": "
                    fn one() -> usize {
                        1
                    }
                    fn two() -> usize {
                        2
                    }
                    fn three() -> usize {
                        3
                    }
                ".unindent(),
                "main.rs": "
                    use crate::one;
                    fn main() { one(); }
                ".unindent(),
            }
        }),
    )
    .await;

    let project = Project::test(fs, [Path::new("/root")], cx).await;
    project.update(cx, |project, _| {
        project.languages().add(Arc::new(rust_lang()));
    });
    let patch_store = cx.new_model(|_| PatchStore::new(project.clone()));
    let context_buffer = cx.new_model(|cx| Buffer::local("hello", cx));
    let context_buffer = context_buffer.read_with(cx, |buffer, _| buffer.snapshot());

    let range = context_buffer.anchor_before(0)..context_buffer.anchor_before(1);

    let patch_id = patch_store.update(cx, |store, cx| {
        store.insert(
            AssistantPatch {
                range: range.clone(),
                title: "first patch".into(),
                edits: vec![AssistantEdit {
                    path: "src/lib.rs".into(),
                    kind: AssistantEditKind::Update {
                        old_text: "1".into(),
                        new_text: "100".into(),
                        description: None,
                    },
                }]
                .into(),
                status: AssistantPatchStatus::Pending,
            },
            cx,
        )
    });

    cx.run_until_parked();
    let patch = patch_store
        .update(cx, |store, cx| store.resolve_patch(patch_id, cx))
        .await
        .unwrap();
    assert_apply_patch(
        &patch,
        cx,
        &[(
            Path::new("src/lib.rs").into(),
            "
            fn one() -> usize {
                100
            }
            fn two() -> usize {
                2
            }
            fn three() -> usize {
                3
            }
            "
            .unindent(),
        )],
    );

    patch_store.update(cx, |store, cx| {
        store
            .update(
                patch_id,
                AssistantPatch {
                    range: range.clone(),
                    title: "first patch".into(),
                    edits: vec![
                        AssistantEdit {
                            path: "src/lib.rs".into(),
                            kind: AssistantEditKind::Update {
                                old_text: "1".into(),
                                new_text: "100".into(),
                                description: None,
                            },
                        },
                        AssistantEdit {
                            path: "src/lib.rs".into(),
                            kind: AssistantEditKind::Update {
                                old_text: "3".into(),
                                new_text: "300".into(),
                                description: None,
                            },
                        },
                    ]
                    .into(),
                    status: AssistantPatchStatus::Pending,
                },
                cx,
            )
            .unwrap();
    });

    cx.run_until_parked();
    let patch = patch_store
        .update(cx, |store, cx| store.resolve_patch(patch_id, cx))
        .await
        .unwrap();
    assert_apply_patch(
        &patch,
        cx,
        &[(
            Path::new("src/lib.rs").into(),
            "
            fn one() -> usize {
                100
            }
            fn two() -> usize {
                2
            }
            fn three() -> usize {
                300
            }
            "
            .unindent(),
        )],
    );
}

#[gpui::test]
fn test_resolve_location(cx: &mut AppContext) {
    assert_location_resolution(
        concat!(
            "    Lorem\n",
            "«    ipsum\n",
            "    dolor sit amet»\n",
            "    consecteur",
        ),
        "ipsum\ndolor",
        cx,
    );

    assert_location_resolution(
        &"
        «fn foo1(a: usize) -> usize {
            40
        }»

        fn foo2(b: usize) -> usize {
            42
        }
        "
        .unindent(),
        "fn foo1(b: usize) {\n40\n}",
        cx,
    );

    assert_location_resolution(
        &"
        fn main() {
        «    Foo
                .bar()
                .baz()
                .qux()»
        }

        fn foo2(b: usize) -> usize {
            42
        }
        "
        .unindent(),
        "Foo.bar.baz.qux()",
        cx,
    );

    assert_location_resolution(
        &"
        class Something {
            one() { return 1; }
        «    two() { return 2222; }
            three() { return 333; }
            four() { return 4444; }
            five() { return 5555; }
            six() { return 6666; }
        »    seven() { return 7; }
            eight() { return 8; }
        }
        "
        .unindent(),
        &"
            two() { return 2222; }
            four() { return 4444; }
            five() { return 5555; }
            six() { return 6666; }
        "
        .unindent(),
        cx,
    );
}

#[gpui::test]
async fn test_resolve_edits(cx: &mut TestAppContext) {
    cx.update(init_test);

    assert_edits(
        "
            /// A person
            struct Person {
                name: String,
                age: usize,
            }

            /// A dog
            struct Dog {
                weight: f32,
            }

            impl Person {
                fn name(&self) -> &str {
                    &self.name
                }
            }
        "
        .unindent(),
        vec![
            AssistantEditKind::Update {
                old_text: "
                    name: String,
                "
                .unindent(),
                new_text: "
                    first_name: String,
                    last_name: String,
                "
                .unindent(),
                description: None,
            },
            AssistantEditKind::Update {
                old_text: "
                    fn name(&self) -> &str {
                        &self.name
                    }
                "
                .unindent(),
                new_text: "
                    fn name(&self) -> String {
                        format!(\"{} {}\", self.first_name, self.last_name)
                    }
                "
                .unindent(),
                description: None,
            },
        ],
        "
            /// A person
            struct Person {
                first_name: String,
                last_name: String,
                age: usize,
            }

            /// A dog
            struct Dog {
                weight: f32,
            }

            impl Person {
                fn name(&self) -> String {
                    format!(\"{} {}\", self.first_name, self.last_name)
                }
            }
        "
        .unindent(),
        cx,
    )
    .await;

    // Ensure InsertBefore merges correctly with Update of the same text
    assert_edits(
        "
            fn foo() {

            }
        "
        .unindent(),
        vec![
            AssistantEditKind::InsertBefore {
                old_text: "
                    fn foo() {"
                    .unindent(),
                new_text: "
                    fn bar() {
                        qux();
                    }"
                .unindent(),
                description: Some("implement bar".into()),
            },
            AssistantEditKind::Update {
                old_text: "
                    fn foo() {

                    }"
                .unindent(),
                new_text: "
                    fn foo() {
                        bar();
                    }"
                .unindent(),
                description: Some("call bar in foo".into()),
            },
            AssistantEditKind::InsertAfter {
                old_text: "
                    fn foo() {

                    }
                "
                .unindent(),
                new_text: "
                    fn qux() {
                        // todo
                    }
                "
                .unindent(),
                description: Some("implement qux".into()),
            },
        ],
        "
            fn bar() {
                qux();
            }

            fn foo() {
                bar();
            }

            fn qux() {
                // todo
            }
        "
        .unindent(),
        cx,
    )
    .await;

    // Correctly indent new text when replacing multiple adjacent indented blocks.
    assert_edits(
        "
        impl Numbers {
            fn one() {
                1
            }

            fn two() {
                2
            }

            fn three() {
                3
            }
        }
        "
        .unindent(),
        vec![
            AssistantEditKind::Update {
                old_text: "
                    fn one() {
                        1
                    }
                "
                .unindent(),
                new_text: "
                    fn one() {
                        101
                    }
                "
                .unindent(),
                description: None,
            },
            AssistantEditKind::Update {
                old_text: "
                    fn two() {
                        2
                    }
                "
                .unindent(),
                new_text: "
                    fn two() {
                        102
                    }
                "
                .unindent(),
                description: None,
            },
            AssistantEditKind::Update {
                old_text: "
                    fn three() {
                        3
                    }
                "
                .unindent(),
                new_text: "
                    fn three() {
                        103
                    }
                "
                .unindent(),
                description: None,
            },
        ],
        "
            impl Numbers {
                fn one() {
                    101
                }

                fn two() {
                    102
                }

                fn three() {
                    103
                }
            }
        "
        .unindent(),
        cx,
    )
    .await;

    assert_edits(
        "
        impl Person {
            fn set_name(&mut self, name: String) {
                self.name = name;
            }

            fn name(&self) -> String {
                return self.name;
            }
        }
        "
        .unindent(),
        vec![
            AssistantEditKind::Update {
                old_text: "self.name = name;".unindent(),
                new_text: "self._name = name;".unindent(),
                description: None,
            },
            AssistantEditKind::Update {
                old_text: "return self.name;\n".unindent(),
                new_text: "return self._name;\n".unindent(),
                description: None,
            },
        ],
        "
            impl Person {
                fn set_name(&mut self, name: String) {
                    self._name = name;
                }

                fn name(&self) -> String {
                    return self._name;
                }
            }
        "
        .unindent(),
        cx,
    )
    .await;
}

fn init_test(cx: &mut AppContext) {
    let settings_store = SettingsStore::test(cx);
    cx.set_global(settings_store);
    language::init(cx);
    Project::init_settings(cx);
    cx.update_global::<SettingsStore, _>(|settings, cx| {
        settings.update_user_settings::<AllLanguageSettings>(cx, |_| {});
    });
}

#[track_caller]
fn assert_apply_patch(
    patch: &ResolvedPatch,
    cx: &mut TestAppContext,
    expected_output: &[(Arc<Path>, String)],
) {
    let mut actual_output = Vec::new();
    for (buffer, edit_groups) in &patch.edit_groups {
        let branch = buffer.update(cx, |buffer, cx| buffer.branch(cx));
        cx.update(|cx| {
            ResolvedPatch::apply_buffer_edits(&Vec::new(), edit_groups, &branch, cx);
            actual_output.push((
                buffer.read(cx).file().unwrap().path().clone(),
                branch.read(cx).text(),
            ));
        });
    }
    pretty_assertions::assert_eq!(actual_output, expected_output);
}

#[track_caller]
fn assert_location_resolution(text_with_expected_range: &str, query: &str, cx: &mut AppContext) {
    let (text, _) = marked_text_ranges(text_with_expected_range, false);
    let buffer = cx.new_model(|cx| Buffer::local(text.clone(), cx));
    let snapshot = buffer.read(cx).snapshot();
    let range = AssistantEditKind::resolve_location(snapshot.as_rope(), query).to_offset(&snapshot);
    let text_with_actual_range = generate_marked_text(&text, &[range], false);
    pretty_assertions::assert_eq!(text_with_actual_range, text_with_expected_range);
}

async fn assert_edits(
    old_text: String,
    edits: Vec<AssistantEditKind>,
    new_text: String,
    cx: &mut TestAppContext,
) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({"file.rs": old_text})).await;
    let project = Project::test(fs, [Path::new("/root")], cx).await;
    project.update(cx, |project, _| {
        project.languages().add(Arc::new(rust_lang()));
    });
    let patch_store = cx.new_model(|_| PatchStore::new(project));
    let patch_range = language::Anchor::MIN..language::Anchor::MAX;
    let patch_id = patch_store.update(cx, |patch_store, cx| {
        patch_store.insert(
            AssistantPatch {
                range: patch_range.clone(),
                title: "test-patch".into(),
                edits: edits
                    .into_iter()
                    .map(|kind| AssistantEdit {
                        path: "file.rs".into(),
                        kind,
                    })
                    .collect(),
                status: AssistantPatchStatus::Ready,
            },
            cx,
        )
    });
    cx.run_until_parked();
    let patch = patch_store
        .update(cx, |patch_store, cx| {
            patch_store.resolve_patch(patch_id, cx)
        })
        .await
        .unwrap();

    let (buffer, edit_groups) = patch.edit_groups.into_iter().next().unwrap();
    cx.update(|cx| ResolvedPatch::apply_buffer_edits(&Vec::new(), &edit_groups, &buffer, cx));
    let actual_new_text = buffer.read_with(cx, |buffer, _| buffer.text());
    pretty_assertions::assert_eq!(actual_new_text, new_text);
}

fn rust_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(language::tree_sitter_rust::LANGUAGE.into()),
    )
    .with_indents_query(
        r#"
        (call_expression) @indent
        (field_expression) @indent
        (_ "(" ")" @end) @indent
        (_ "{" "}" @end) @indent
        "#,
    )
    .unwrap()
}
