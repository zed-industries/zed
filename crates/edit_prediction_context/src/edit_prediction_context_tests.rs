use super::*;
use crate::assemble_excerpts::assemble_excerpt_ranges;
use futures::channel::mpsc::UnboundedReceiver;
use gpui::TestAppContext;
use indoc::indoc;
use language::{Point, ToPoint as _, rust_lang};
use lsp::FakeLanguageServer;
use project::{FakeFs, LocationLink, Project};
use serde_json::json;
use settings::SettingsStore;
use std::fmt::Write as _;
use util::{path, test::marked_text_ranges};

#[gpui::test]
async fn test_edit_prediction_context(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), test_project_1()).await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let mut servers = setup_fake_lsp(&project, cx);

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();

    let _server = servers.next().await.unwrap();
    cx.run_until_parked();

    let related_excerpt_store = cx.new(|cx| RelatedExcerptStore::new(&project, cx));
    related_excerpt_store.update(cx, |store, cx| {
        let position = {
            let buffer = buffer.read(cx);
            let offset = buffer.text().find("todo").unwrap();
            buffer.anchor_before(offset)
        };

        store.set_identifier_line_count(0);
        store.refresh(buffer.clone(), position, cx);
    });

    cx.executor().advance_clock(DEBOUNCE_DURATION);
    related_excerpt_store.update(cx, |store, cx| {
        let excerpts = store.related_files(cx);
        assert_related_files(
            &excerpts,
            &[
                (
                    "root/src/person.rs",
                    &[
                        indoc! {"
                        pub struct Person {
                            first_name: String,
                            last_name: String,
                            email: String,
                            age: u32,
                        }

                        impl Person {
                            pub fn get_first_name(&self) -> &str {
                                &self.first_name
                            }"},
                        "}",
                    ],
                ),
                (
                    "root/src/company.rs",
                    &[indoc! {"
                        pub struct Company {
                            owner: Arc<Person>,
                            address: Address,
                        }"}],
                ),
                (
                    "root/src/main.rs",
                    &[
                        indoc! {"
                        pub struct Session {
                            company: Arc<Company>,
                        }

                        impl Session {
                            pub fn set_company(&mut self, company: Arc<Company>) {"},
                        indoc! {"
                            }
                        }"},
                    ],
                ),
            ],
        );
    });

    let company_buffer = related_excerpt_store.update(cx, |store, cx| {
        store
            .related_files_with_buffers(cx)
            .find(|(file, _)| file.path.to_str() == Some("root/src/company.rs"))
            .map(|(_, buffer)| buffer)
            .expect("company.rs buffer not found")
    });

    company_buffer.update(cx, |buffer, cx| {
        let text = buffer.text();
        let insert_pos = text.find("address: Address,").unwrap() + "address: Address,".len();
        buffer.edit([(insert_pos..insert_pos, "\n    name: String,")], None, cx);
    });

    related_excerpt_store.update(cx, |store, cx| {
        let excerpts = store.related_files(cx);
        assert_related_files(
            &excerpts,
            &[
                (
                    "root/src/person.rs",
                    &[
                        indoc! {"
                        pub struct Person {
                            first_name: String,
                            last_name: String,
                            email: String,
                            age: u32,
                        }

                        impl Person {
                            pub fn get_first_name(&self) -> &str {
                                &self.first_name
                            }"},
                        "}",
                    ],
                ),
                (
                    "root/src/company.rs",
                    &[indoc! {"
                        pub struct Company {
                            owner: Arc<Person>,
                            address: Address,
                            name: String,
                        }"}],
                ),
                (
                    "root/src/main.rs",
                    &[
                        indoc! {"
                        pub struct Session {
                            company: Arc<Company>,
                        }

                        impl Session {
                            pub fn set_company(&mut self, company: Arc<Company>) {"},
                        indoc! {"
                            }
                        }"},
                    ],
                ),
            ],
        );
    });
}

#[gpui::test]
fn test_assemble_excerpts(cx: &mut TestAppContext) {
    let table = [
        (
            indoc! {r#"
                struct User {
                    first_name: String,
                    «last_name»: String,
                    age: u32,
                    email: String,
                    create_at: Instant,
                }

                impl User {
                    pub fn first_name(&self) -> String {
                        self.first_name.clone()
                    }

                    pub fn full_name(&self) -> String {
                «        format!("{} {}", self.first_name, self.last_name)
                »    }
                }
            "#},
            indoc! {r#"
                struct User {
                    first_name: String,
                    last_name: String,
                …
                }

                impl User {
                …
                    pub fn full_name(&self) -> String {
                        format!("{} {}", self.first_name, self.last_name)
                    }
                }
            "#},
        ),
        (
            indoc! {r#"
                struct «User» {
                    first_name: String,
                    last_name: String,
                    age: u32,
                }

                impl User {
                    // methods
                }
            "#},
            indoc! {r#"
                struct User {
                    first_name: String,
                    last_name: String,
                    age: u32,
                }
                …
            "#},
        ),
        (
            indoc! {r#"
                trait «FooProvider» {
                    const NAME: &'static str;

                    fn provide_foo(&self, id: usize) -> Foo;

                    fn provide_foo_batched(&self, ids: &[usize]) -> Vec<Foo> {
                            ids.iter()
                            .map(|id| self.provide_foo(*id))
                            .collect()
                    }

                    fn sync(&self);
                }
                "#
            },
            indoc! {r#"
                trait FooProvider {
                    const NAME: &'static str;

                    fn provide_foo(&self, id: usize) -> Foo;

                    fn provide_foo_batched(&self, ids: &[usize]) -> Vec<Foo> {
                …
                    }

                    fn sync(&self);
                }
            "#},
        ),
        (
            indoc! {r#"
                trait «Something» {
                    fn method1(&self, id: usize) -> Foo;

                    fn method2(&self, ids: &[usize]) -> Vec<Foo> {
                            struct Helper1 {
                            field1: usize,
                            }

                            struct Helper2 {
                            field2: usize,
                            }

                            struct Helper3 {
                            filed2: usize,
                        }
                    }

                    fn sync(&self);
                }
                "#
            },
            indoc! {r#"
                trait Something {
                    fn method1(&self, id: usize) -> Foo;

                    fn method2(&self, ids: &[usize]) -> Vec<Foo> {
                …
                    }

                    fn sync(&self);
                }
            "#},
        ),
    ];

    for (input, expected_output) in table {
        let (input, ranges) = marked_text_ranges(&input, false);
        let buffer = cx.new(|cx| Buffer::local(input, cx).with_language(rust_lang(), cx));
        buffer.read_with(cx, |buffer, _cx| {
            let ranges: Vec<(Range<Point>, usize)> = ranges
                .into_iter()
                .map(|range| (range.to_point(&buffer), 0))
                .collect();

            let assembled = assemble_excerpt_ranges(&buffer.snapshot(), ranges);
            let excerpts: Vec<RelatedExcerpt> = assembled
                .into_iter()
                .map(|(row_range, order)| {
                    let start = Point::new(row_range.start, 0);
                    let end = Point::new(row_range.end, buffer.line_len(row_range.end));
                    RelatedExcerpt {
                        row_range,
                        text: buffer.text_for_range(start..end).collect::<String>().into(),
                        order,
                    }
                })
                .collect();

            let output = format_excerpts(buffer, &excerpts);
            assert_eq!(output, expected_output);
        });
    }
}

#[gpui::test]
async fn test_fake_definition_lsp(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), test_project_1()).await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let mut servers = setup_fake_lsp(&project, cx);

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();

    let _server = servers.next().await.unwrap();
    cx.run_until_parked();

    let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());

    let definitions = project
        .update(cx, |project, cx| {
            let offset = buffer_text.find("Address {").unwrap();
            project.definitions(&buffer, offset, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_definitions(&definitions, &["pub struct Address {"], cx);

    let definitions = project
        .update(cx, |project, cx| {
            let offset = buffer_text.find("State::CA").unwrap();
            project.definitions(&buffer, offset, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_definitions(&definitions, &["pub enum State {"], cx);

    let definitions = project
        .update(cx, |project, cx| {
            let offset = buffer_text.find("to_string()").unwrap();
            project.definitions(&buffer, offset, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_definitions(&definitions, &["pub fn to_string(&self) -> String {"], cx);
}

#[gpui::test]
async fn test_fake_type_definition_lsp(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/root"), test_project_1()).await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let mut servers = setup_fake_lsp(&project, cx);

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();

    let _server = servers.next().await.unwrap();
    cx.run_until_parked();

    let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());

    // Type definition on a type name returns its own definition
    // (same as regular definition)
    let type_defs = project
        .update(cx, |project, cx| {
            let offset = buffer_text.find("Address {").expect("Address { not found");
            project.type_definitions(&buffer, offset, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_definitions(&type_defs, &["pub struct Address {"], cx);

    // Type definition on a field resolves through the type annotation.
    // company.rs has `owner: Arc<Person>`, so type-def of `owner` → Person.
    let (company_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/company.rs"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let company_text = company_buffer.read_with(cx, |buffer, _| buffer.text());
    let type_defs = project
        .update(cx, |project, cx| {
            let offset = company_text.find("owner").expect("owner not found");
            project.type_definitions(&company_buffer, offset, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_definitions(&type_defs, &["pub struct Person {"], cx);

    // Type definition on another field: `address: Address` → Address.
    let type_defs = project
        .update(cx, |project, cx| {
            let offset = company_text.find("address").expect("address not found");
            project.type_definitions(&company_buffer, offset, cx)
        })
        .await
        .unwrap()
        .unwrap();
    assert_definitions(&type_defs, &["pub struct Address {"], cx);

    // Type definition on a lowercase name with no type annotation returns empty.
    let type_defs = project
        .update(cx, |project, cx| {
            let offset = buffer_text.find("main").expect("main not found");
            project.type_definitions(&buffer, offset, cx)
        })
        .await;
    let is_empty = match &type_defs {
        Ok(Some(defs)) => defs.is_empty(),
        Ok(None) => true,
        Err(_) => false,
    };
    assert!(is_empty, "expected no type definitions for `main`");
}

#[gpui::test]
async fn test_type_definitions_in_related_files(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "src": {
                "config.rs": indoc! {r#"
                    pub struct Config {
                        debug: bool,
                        verbose: bool,
                    }
                "#},
                "widget.rs": indoc! {r#"
                    use super::config::Config;

                    pub struct Widget {
                        config: Config,
                        name: String,
                    }

                    impl Widget {
                        pub fn render(&self) {
                            if self.config.debug {
                                println!("debug mode");
                            }
                        }
                    }
                "#},
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let mut servers = setup_fake_lsp(&project, cx);

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/widget.rs"), cx)
        })
        .await
        .unwrap();

    let _server = servers.next().await.unwrap();
    cx.run_until_parked();

    let related_excerpt_store = cx.new(|cx| RelatedExcerptStore::new(&project, cx));
    related_excerpt_store.update(cx, |store, cx| {
        let position = {
            let buffer = buffer.read(cx);
            let offset = buffer
                .text()
                .find("self.config.debug")
                .expect("self.config.debug not found");
            buffer.anchor_before(offset)
        };

        store.set_identifier_line_count(0);
        store.refresh(buffer.clone(), position, cx);
    });

    cx.executor().advance_clock(DEBOUNCE_DURATION);
    // config.rs appears ONLY because the fake LSP resolves the type annotation
    // `config: Config` to `pub struct Config` via GotoTypeDefinition.
    // widget.rs appears from regular definitions of Widget / render.
    related_excerpt_store.update(cx, |store, cx| {
        let excerpts = store.related_files(cx);
        assert_related_files(
            &excerpts,
            &[
                (
                    "root/src/config.rs",
                    &[indoc! {"
                        pub struct Config {
                            debug: bool,
                            verbose: bool,
                        }"}],
                ),
                (
                    "root/src/widget.rs",
                    &[
                        indoc! {"
                        pub struct Widget {
                            config: Config,
                            name: String,
                        }

                        impl Widget {
                            pub fn render(&self) {"},
                        indoc! {"
                            }
                        }"},
                    ],
                ),
            ],
        );
    });
}

#[gpui::test]
async fn test_type_definition_deduplication(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // In this project the only identifier near the cursor whose type definition
    // resolves is `TypeA`, and its GotoTypeDefinition returns the exact same
    // location as GotoDefinition. After deduplication the CacheEntry for `TypeA`
    // should have an empty `type_definitions` vec, meaning the type-definition
    // path contributes nothing extra to the related-file output.
    fs.insert_tree(
        path!("/root"),
        json!({
            "src": {
                "types.rs": indoc! {r#"
                    pub struct TypeA {
                        value: i32,
                    }

                    pub struct TypeB {
                        label: String,
                    }
                "#},
                "main.rs": indoc! {r#"
                    use super::types::TypeA;

                    fn work() {
                        let item: TypeA = unimplemented!();
                        println!("{}", item.value);
                    }
                "#},
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let mut servers = setup_fake_lsp(&project, cx);

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();

    let _server = servers.next().await.unwrap();
    cx.run_until_parked();

    let related_excerpt_store = cx.new(|cx| RelatedExcerptStore::new(&project, cx));
    related_excerpt_store.update(cx, |store, cx| {
        let position = {
            let buffer = buffer.read(cx);
            let offset = buffer.text().find("let item").expect("let item not found");
            buffer.anchor_before(offset)
        };

        store.set_identifier_line_count(0);
        store.refresh(buffer.clone(), position, cx);
    });

    cx.executor().advance_clock(DEBOUNCE_DURATION);
    // types.rs appears because `TypeA` has a regular definition there.
    // `item`'s type definition also resolves to TypeA in types.rs, but
    // deduplication removes it since it points to the same location.
    // TypeB should NOT appear because nothing references it.
    related_excerpt_store.update(cx, |store, cx| {
        let excerpts = store.related_files(cx);
        assert_related_files(
            &excerpts,
            &[
                (
                    "root/src/types.rs",
                    &[indoc! {"
                        pub struct TypeA {
                            value: i32,
                        }"}],
                ),
                ("root/src/main.rs", &["fn work() {", "}"]),
            ],
        );
    });
}

#[gpui::test]
async fn test_definitions_ranked_by_cursor_proximity(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    // helpers.rs has an impl block whose body exceeds the test
    // MAX_OUTLINE_ITEM_BODY_SIZE (24 bytes), so assemble_excerpt_ranges
    // splits it into header + individual children + closing brace. main.rs
    // references two of the three methods on separate lines at varying
    // distances from the cursor. This exercises:
    //   1. File ordering by closest identifier rank.
    //   2. Per-excerpt ordering within a file — child excerpts carry the rank
    //      of the identifier that discovered them.
    //   3. Parent excerpt (impl header / closing brace) inheriting the minimum
    //      order of its children.
    fs.insert_tree(
        path!("/root"),
        json!({
            "src": {
                "helpers.rs": indoc! {r#"
                    pub struct Helpers {
                        value: i32,
                    }

                    impl Helpers {
                        pub fn alpha(&self) -> i32 {
                            let intermediate = self.value;
                            intermediate + 1
                        }

                        pub fn beta(&self) -> i32 {
                            let intermediate = self.value;
                            intermediate + 2
                        }

                        pub fn gamma(&self) -> i32 {
                            let intermediate = self.value;
                            intermediate + 3
                        }
                    }
                "#},
                "main.rs": indoc! {r#"
                    use super::helpers::Helpers;

                    fn process(h: Helpers) {
                        let a = h.alpha();
                        let b = h.gamma();
                    }
                "#},
            },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let mut servers = setup_fake_lsp(&project, cx);

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();

    let _server = servers.next().await.unwrap();
    cx.run_until_parked();

    // Place cursor on "h.alpha()". `alpha` is at distance 0, `gamma` is
    // farther below. Both resolve to methods inside `impl Helpers` in
    // helpers.rs. The impl header and closing brace excerpts should inherit
    // the min order of their children (alpha's order).
    let related_excerpt_store = cx.new(|cx| RelatedExcerptStore::new(&project, cx));
    related_excerpt_store.update(cx, |store, cx| {
        let position = {
            let buffer = buffer.read(cx);
            let offset = buffer.text().find("h.alpha()").unwrap();
            buffer.anchor_before(offset)
        };

        store.set_identifier_line_count(1);
        store.refresh(buffer.clone(), position, cx);
    });

    cx.executor().advance_clock(DEBOUNCE_DURATION);
    related_excerpt_store.update(cx, |store, cx| {
        let files = store.related_files(cx);

        // helpers.rs has 4 excerpts: the struct+impl header merged with
        // the alpha method header (order 1 from alpha), alpha's closing
        // brace (order 1), gamma's method header (order 6), and the
        // gamma+impl closing brace (order 1, inherited from alpha which
        // is also a child of the impl).
        let alpha_order = 1;
        let gamma_order = 6;
        assert_related_files_with_orders(
            &files,
            &[
                (
                    "root/src/helpers.rs",
                    &[
                        (
                            indoc! {"
                            pub struct Helpers {
                                value: i32,
                            }

                            impl Helpers {
                                pub fn alpha(&self) -> i32 {"},
                            alpha_order,
                        ),
                        ("    }", alpha_order),
                        ("    pub fn gamma(&self) -> i32 {", gamma_order),
                        (
                            indoc! {"
                                }
                            }"},
                            alpha_order,
                        ),
                    ],
                ),
                (
                    "root/src/main.rs",
                    &[("fn process(h: Helpers) {", 8), ("}", 8)],
                ),
            ],
        );
    });

    // Now move cursor to "h.gamma()" — gamma becomes closest, reranking the
    // excerpts so that the gamma method excerpt has the best order and the
    // alpha method excerpt has a worse order.
    related_excerpt_store.update(cx, |store, cx| {
        let position = {
            let buffer = buffer.read(cx);
            let offset = buffer.text().find("h.gamma()").unwrap();
            buffer.anchor_before(offset)
        };

        store.set_identifier_line_count(1);
        store.refresh(buffer.clone(), position, cx);
    });

    cx.executor().advance_clock(DEBOUNCE_DURATION);
    related_excerpt_store.update(cx, |store, cx| {
        let files = store.related_files(cx);

        // Now gamma is closest. The alpha method excerpts carry alpha's
        // rank (3), and the gamma method excerpts carry gamma's rank (1).
        // The impl closing brace merges with gamma's closing brace and
        // inherits gamma's order (the best child).
        let alpha_order = 3;
        let gamma_order = 1;
        assert_related_files_with_orders(
            &files,
            &[
                (
                    "root/src/helpers.rs",
                    &[
                        (
                            indoc! {"
                            pub struct Helpers {
                                value: i32,
                            }

                            impl Helpers {
                                pub fn alpha(&self) -> i32 {"},
                            alpha_order,
                        ),
                        ("    }", alpha_order),
                        ("    pub fn gamma(&self) -> i32 {", gamma_order),
                        (
                            indoc! {"
                                }
                            }"},
                            gamma_order,
                        ),
                    ],
                ),
                (
                    "root/src/main.rs",
                    &[("fn process(h: Helpers) {", 8), ("}", 8)],
                ),
            ],
        );
    });
}

fn init_test(cx: &mut TestAppContext) {
    let settings_store = cx.update(|cx| SettingsStore::test(cx));
    cx.set_global(settings_store);
    env_logger::try_init().ok();
}

fn setup_fake_lsp(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> UnboundedReceiver<FakeLanguageServer> {
    let (language_registry, fs) = project.read_with(cx, |project, _| {
        (project.languages().clone(), project.fs().clone())
    });
    let language = rust_lang();
    language_registry.add(language.clone());
    fake_definition_lsp::register_fake_definition_server(&language_registry, language, fs)
}

fn test_project_1() -> serde_json::Value {
    let person_rs = indoc! {r#"
        pub struct Person {
            first_name: String,
            last_name: String,
            email: String,
            age: u32,
        }

        impl Person {
            pub fn get_first_name(&self) -> &str {
                &self.first_name
            }

            pub fn get_last_name(&self) -> &str {
                &self.last_name
            }

            pub fn get_email(&self) -> &str {
                &self.email
            }

            pub fn get_age(&self) -> u32 {
                self.age
            }
        }
    "#};

    let address_rs = indoc! {r#"
        pub struct Address {
            street: String,
            city: String,
            state: State,
            zip: u32,
        }

        pub enum State {
            CA,
            OR,
            WA,
            TX,
            // ...
        }

        impl Address {
            pub fn get_street(&self) -> &str {
                &self.street
            }

            pub fn get_city(&self) -> &str {
                &self.city
            }

            pub fn get_state(&self) -> State {
                self.state
            }

            pub fn get_zip(&self) -> u32 {
                self.zip
            }
        }
    "#};

    let company_rs = indoc! {r#"
        use super::person::Person;
        use super::address::Address;

        pub struct Company {
            owner: Arc<Person>,
            address: Address,
        }

        impl Company {
            pub fn get_owner(&self) -> &Person {
                &self.owner
            }

            pub fn get_address(&self) -> &Address {
                &self.address
            }

            pub fn to_string(&self) -> String {
                format!("{} ({})", self.owner.first_name, self.address.city)
            }
        }
    "#};

    let main_rs = indoc! {r#"
        use std::sync::Arc;
        use super::person::Person;
        use super::address::Address;
        use super::company::Company;

        pub struct Session {
            company: Arc<Company>,
        }

        impl Session {
            pub fn set_company(&mut self, company: Arc<Company>) {
                self.company = company;
                if company.owner != self.company.owner {
                    log("new owner", company.owner.get_first_name()); todo();
                }
            }
        }

        fn main() {
            let company = Company {
                owner: Arc::new(Person {
                    first_name: "John".to_string(),
                    last_name: "Doe".to_string(),
                    email: "john@example.com".to_string(),
                    age: 30,
                }),
                address: Address {
                    street: "123 Main St".to_string(),
                    city: "Anytown".to_string(),
                    state: State::CA,
                    zip: 12345,
                },
            };

            println!("Company: {}", company.to_string());
        }
    "#};

    json!({
        "src": {
            "person.rs": person_rs,
            "address.rs": address_rs,
            "company.rs": company_rs,
            "main.rs": main_rs,
        },
    })
}

fn assert_related_files(actual_files: &[RelatedFile], expected_files: &[(&str, &[&str])]) {
    let expected_with_orders: Vec<(&str, Vec<(&str, usize)>)> = expected_files
        .iter()
        .map(|(path, texts)| (*path, texts.iter().map(|text| (*text, 0)).collect()))
        .collect();
    let expected_refs: Vec<(&str, &[(&str, usize)])> = expected_with_orders
        .iter()
        .map(|(path, excerpts)| (*path, excerpts.as_slice()))
        .collect();
    assert_related_files_impl(actual_files, &expected_refs, false)
}

fn assert_related_files_with_orders(
    actual_files: &[RelatedFile],
    expected_files: &[(&str, &[(&str, usize)])],
) {
    assert_related_files_impl(actual_files, expected_files, true)
}

fn assert_related_files_impl(
    actual_files: &[RelatedFile],
    expected_files: &[(&str, &[(&str, usize)])],
    check_orders: bool,
) {
    let actual: Vec<(&str, Vec<(String, usize)>)> = actual_files
        .iter()
        .map(|file| {
            let excerpts = file
                .excerpts
                .iter()
                .map(|excerpt| {
                    let order = if check_orders { excerpt.order } else { 0 };
                    (excerpt.text.to_string(), order)
                })
                .collect();
            (file.path.to_str().unwrap(), excerpts)
        })
        .collect();
    let expected: Vec<(&str, Vec<(String, usize)>)> = expected_files
        .iter()
        .map(|(path, excerpts)| {
            (
                *path,
                excerpts
                    .iter()
                    .map(|(text, order)| (text.to_string(), *order))
                    .collect(),
            )
        })
        .collect();
    pretty_assertions::assert_eq!(actual, expected)
}

fn assert_definitions(definitions: &[LocationLink], first_lines: &[&str], cx: &mut TestAppContext) {
    let actual_first_lines = definitions
        .iter()
        .map(|definition| {
            definition.target.buffer.read_with(cx, |buffer, _| {
                let mut start = definition.target.range.start.to_point(&buffer);
                start.column = 0;
                let end = Point::new(start.row, buffer.line_len(start.row));
                buffer
                    .text_for_range(start..end)
                    .collect::<String>()
                    .trim()
                    .to_string()
            })
        })
        .collect::<Vec<String>>();

    assert_eq!(actual_first_lines, first_lines);
}

fn format_excerpts(buffer: &Buffer, excerpts: &[RelatedExcerpt]) -> String {
    let mut output = String::new();
    let file_line_count = buffer.max_point().row;
    let mut current_row = 0;
    for excerpt in excerpts {
        if excerpt.text.is_empty() {
            continue;
        }
        if current_row < excerpt.row_range.start {
            writeln!(&mut output, "…").unwrap();
        }
        current_row = excerpt.row_range.start;

        for line in excerpt.text.to_string().lines() {
            output.push_str(line);
            output.push('\n');
            current_row += 1;
        }
    }
    if current_row < file_line_count {
        writeln!(&mut output, "…").unwrap();
    }
    output
}
