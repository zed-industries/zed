use super::*;
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
    related_excerpt_store.update(cx, |store, _| {
        let excerpts = store.related_files();
        assert_related_files(
            &excerpts,
            &[
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
                (
                    "root/src/person.rs",
                    &[
                        indoc! {"
                        impl Person {
                            pub fn get_first_name(&self) -> &str {
                                &self.first_name
                            }"},
                        "}",
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
            let ranges: Vec<Range<Point>> = ranges
                .into_iter()
                .map(|range| range.to_point(&buffer))
                .collect();

            let excerpts = assemble_excerpts(&buffer.snapshot(), ranges);

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
    let actual_files = actual_files
        .iter()
        .map(|file| {
            let excerpts = file
                .excerpts
                .iter()
                .map(|excerpt| excerpt.text.to_string())
                .collect::<Vec<_>>();
            (file.path.to_str().unwrap(), excerpts)
        })
        .collect::<Vec<_>>();
    let expected_excerpts = expected_files
        .iter()
        .map(|(path, texts)| {
            (
                *path,
                texts
                    .iter()
                    .map(|line| line.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    pretty_assertions::assert_eq!(actual_files, expected_excerpts)
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
