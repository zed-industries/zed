use crate::{
    db::dot,
    embedding::EmbeddingProvider,
    parsing::{CodeContextRetriever, Document},
    vector_store_settings::VectorStoreSettings,
    VectorStore,
};
use anyhow::Result;
use async_trait::async_trait;
use gpui::{Task, TestAppContext};
use language::{Language, LanguageConfig, LanguageRegistry};
use project::{project_settings::ProjectSettings, FakeFs, Fs, Project};
use rand::{rngs::StdRng, Rng};
use serde_json::json;
use settings::SettingsStore;
use std::{
    path::Path,
    sync::{
        atomic::{self, AtomicUsize},
        Arc,
    },
};
use unindent::Unindent;

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
async fn test_vector_store(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.set_global(SettingsStore::test(cx));
        settings::register::<VectorStoreSettings>(cx);
        settings::register::<ProjectSettings>(cx);
    });

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/the-root",
        json!({
            "src": {
                "file1.rs": "
                    fn aaa() {
                        println!(\"aaaa!\");
                    }

                    fn zzzzzzzzz() {
                        println!(\"SLEEPING\");
                    }
                ".unindent(),
                "file2.rs": "
                    fn bbb() {
                        println!(\"bbbb!\");
                    }
                ".unindent(),
                "file3.toml": "
                    ZZZZZZZ = 5
                    ".unindent(),
            }
        }),
    )
    .await;

    let languages = Arc::new(LanguageRegistry::new(Task::ready(())));
    let rust_language = rust_lang();
    let toml_language = toml_lang();
    languages.add(rust_language);
    languages.add(toml_language);

    let db_dir = tempdir::TempDir::new("vector-store").unwrap();
    let db_path = db_dir.path().join("db.sqlite");

    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let store = VectorStore::new(
        fs.clone(),
        db_path,
        embedding_provider.clone(),
        languages,
        cx.to_async(),
    )
    .await
    .unwrap();

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let file_count = store
        .update(cx, |store, cx| store.index_project(project.clone(), cx))
        .await
        .unwrap();
    assert_eq!(file_count, 3);
    cx.foreground().run_until_parked();
    store.update(cx, |store, _cx| {
        assert_eq!(
            store.remaining_files_to_index_for_project(&project),
            Some(0)
        );
    });

    let search_results = store
        .update(cx, |store, cx| {
            store.search_project(project.clone(), "aaaa".to_string(), 5, cx)
        })
        .await
        .unwrap();

    assert_eq!(search_results[0].byte_range.start, 0);
    assert_eq!(search_results[0].name, "aaa");
    assert_eq!(search_results[0].worktree_id, worktree_id);

    fs.save(
        "/the-root/src/file2.rs".as_ref(),
        &"
            fn dddd() { println!(\"ddddd!\"); }
            struct pqpqpqp {}
        "
        .unindent()
        .into(),
        Default::default(),
    )
    .await
    .unwrap();

    cx.foreground().run_until_parked();

    let prev_embedding_count = embedding_provider.embedding_count();
    let file_count = store
        .update(cx, |store, cx| store.index_project(project.clone(), cx))
        .await
        .unwrap();
    assert_eq!(file_count, 1);

    cx.foreground().run_until_parked();
    store.update(cx, |store, _cx| {
        assert_eq!(
            store.remaining_files_to_index_for_project(&project),
            Some(0)
        );
    });

    assert_eq!(
        embedding_provider.embedding_count() - prev_embedding_count,
        2
    );
}

#[gpui::test]
async fn test_code_context_retrieval_rust() {
    let language = rust_lang();
    let mut retriever = CodeContextRetriever::new();

    let text = "
        /// A doc comment
        /// that spans multiple lines
        fn a() {
            b
        }

        impl C for D {
        }
    "
    .unindent();

    let parsed_files = retriever
        .parse_file(Path::new("foo.rs"), &text, language)
        .unwrap();

    assert_eq!(
        parsed_files,
        &[
            Document {
                name: "a".into(),
                range: text.find("fn a").unwrap()..(text.find("}").unwrap() + 1),
                content: "
                    The below code snippet is from file 'foo.rs'

                    ```rust
                    /// A doc comment
                    /// that spans multiple lines
                    fn a() {
                        b
                    }
                    ```"
                .unindent(),
                embedding: vec![],
            },
            Document {
                name: "C for D".into(),
                range: text.find("impl C").unwrap()..(text.rfind("}").unwrap() + 1),
                content: "
                    The below code snippet is from file 'foo.rs'

                    ```rust
                    impl C for D {
                    }
                    ```"
                .unindent(),
                embedding: vec![],
            }
        ]
    );
}

#[gpui::test]
async fn test_code_context_retrieval_javascript() {
    let language = js_lang();
    let mut retriever = CodeContextRetriever::new();

    let text = "
        /* globals importScripts, backend */
        function _authorize() {}

        /**
         * Sometimes the frontend build is way faster than backend.
         */
        export async function authorizeBank() {
            _authorize(pushModal, upgradingAccountId, {});
        }

        export class SettingsPage {
            /* This is a test setting */
            constructor(page) {
                this.page = page;
            }
        }

        /* This is a test comment */
        class TestClass {}

        /* Schema for editor_events in Clickhouse. */
        export interface ClickhouseEditorEvent {
            installation_id: string
            operation: string
        }
        "
    .unindent();

    let parsed_files = retriever
        .parse_file(Path::new("foo.js"), &text, language)
        .unwrap();

    let test_documents = &[
        Document {
            name: "function _authorize".into(),
            range: text.find("function _authorize").unwrap()..(text.find("}").unwrap() + 1),
            content: "
                    The below code snippet is from file 'foo.js'

                    ```javascript
                    /* globals importScripts, backend */
                    function _authorize() {}
                    ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "async function authorizeBank".into(),
            range: text.find("export async").unwrap()..223,
            content: "
                    The below code snippet is from file 'foo.js'

                    ```javascript
                    /**
                     * Sometimes the frontend build is way faster than backend.
                     */
                    export async function authorizeBank() {
                        _authorize(pushModal, upgradingAccountId, {});
                    }
                    ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "class SettingsPage".into(),
            range: 225..343,
            content: "
                    The below code snippet is from file 'foo.js'

                    ```javascript
                    export class SettingsPage {
                        /* This is a test setting */
                        constructor(page) {
                            this.page = page;
                        }
                    }
                    ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "constructor".into(),
            range: 290..341,
            content: "
                The below code snippet is from file 'foo.js'

                ```javascript
                /* This is a test setting */
                constructor(page) {
                        this.page = page;
                    }
                ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "class TestClass".into(),
            range: 374..392,
            content: "
                    The below code snippet is from file 'foo.js'

                    ```javascript
                    /* This is a test comment */
                    class TestClass {}
                    ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "interface ClickhouseEditorEvent".into(),
            range: 440..532,
            content: "
                    The below code snippet is from file 'foo.js'

                    ```javascript
                    /* Schema for editor_events in Clickhouse. */
                    export interface ClickhouseEditorEvent {
                        installation_id: string
                        operation: string
                    }
                    ```"
            .unindent(),
            embedding: vec![],
        },
    ];

    for idx in 0..test_documents.len() {
        assert_eq!(test_documents[idx], parsed_files[idx]);
    }
}

#[gpui::test]
async fn test_code_context_retrieval_cpp() {
    let language = cpp_lang();
    let mut retriever = CodeContextRetriever::new();

    let text = "
    /**
     * @brief Main function
     * @returns 0 on exit
     */
    int main() { return 0; }

    /**
    * This is a test comment
    */
    class MyClass {       // The class
        public:             // Access specifier
        int myNum;        // Attribute (int variable)
        string myString;  // Attribute (string variable)
    };

    // This is a test comment
    enum Color { red, green, blue };

    /** This is a preceeding block comment
     * This is the second line
     */
    struct {           // Structure declaration
        int myNum;       // Member (int variable)
        string myString; // Member (string variable)
    } myStructure;

    /**
    * @brief Matrix class.
    */
    template <typename T,
              typename = typename std::enable_if<
                std::is_integral<T>::value || std::is_floating_point<T>::value,
                bool>::type>
    class Matrix2 {
        std::vector<std::vector<T>> _mat;

    public:
        /**
        * @brief Constructor
        * @tparam Integer ensuring integers are being evaluated and not other
        * data types.
        * @param size denoting the size of Matrix as size x size
        */
        template <typename Integer,
                  typename = typename std::enable_if<std::is_integral<Integer>::value,
                  Integer>::type>
        explicit Matrix(const Integer size) {
            for (size_t i = 0; i < size; ++i) {
                _mat.emplace_back(std::vector<T>(size, 0));
            }
        }
    }"
    .unindent();

    let parsed_files = retriever
        .parse_file(Path::new("foo.cpp"), &text, language)
        .unwrap();

    let test_documents = &[
        Document {
            name: "int main".into(),
            range: 54..78,
            content: "
                The below code snippet is from file 'foo.cpp'

                ```cpp
                /**
                 * @brief Main function
                 * @returns 0 on exit
                 */
                int main() { return 0; }
                ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "class MyClass".into(),
            range: 112..295,
            content: "
                The below code snippet is from file 'foo.cpp'

                ```cpp
                /**
                * This is a test comment
                */
                class MyClass {       // The class
                    public:             // Access specifier
                    int myNum;        // Attribute (int variable)
                    string myString;  // Attribute (string variable)
                }
                ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "enum Color".into(),
            range: 324..355,
            content: "
                The below code snippet is from file 'foo.cpp'

                ```cpp
                // This is a test comment
                enum Color { red, green, blue }
                ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "struct myStructure".into(),
            range: 428..581,
            content: "
                The below code snippet is from file 'foo.cpp'

                ```cpp
                /** This is a preceeding block comment
                 * This is the second line
                 */
                struct {           // Structure declaration
                    int myNum;       // Member (int variable)
                    string myString; // Member (string variable)
                } myStructure;
                ```"
            .unindent(),
            embedding: vec![],
        },
        Document {
            name: "class Matrix2".into(),
            range: 613..1342,
            content: "
                The below code snippet is from file 'foo.cpp'

                ```cpp
                /**
                * @brief Matrix class.
                */
                template <typename T,
                          typename = typename std::enable_if<
                            std::is_integral<T>::value || std::is_floating_point<T>::value,
                            bool>::type>
                class Matrix2 {
                    std::vector<std::vector<T>> _mat;

                public:
                    /**
                    * @brief Constructor
                    * @tparam Integer ensuring integers are being evaluated and not other
                    * data types.
                    * @param size denoting the size of Matrix as size x size
                    */
                    template <typename Integer,
                              typename = typename std::enable_if<std::is_integral<Integer>::value,
                              Integer>::type>
                    explicit Matrix(const Integer size) {
                        for (size_t i = 0; i < size; ++i) {
                            _mat.emplace_back(std::vector<T>(size, 0));
                        }
                    }
                }
                ```"
            .unindent(),
            embedding: vec![],
        },
    ];

    for idx in 0..test_documents.len() {
        assert_eq!(test_documents[idx], parsed_files[idx]);
    }
}

#[gpui::test]
fn test_dot_product(mut rng: StdRng) {
    assert_eq!(dot(&[1., 0., 0., 0., 0.], &[0., 1., 0., 0., 0.]), 0.);
    assert_eq!(dot(&[2., 0., 0., 0., 0.], &[3., 1., 0., 0., 0.]), 6.);

    for _ in 0..100 {
        let size = 1536;
        let mut a = vec![0.; size];
        let mut b = vec![0.; size];
        for (a, b) in a.iter_mut().zip(b.iter_mut()) {
            *a = rng.gen();
            *b = rng.gen();
        }

        assert_eq!(
            round_to_decimals(dot(&a, &b), 1),
            round_to_decimals(reference_dot(&a, &b), 1)
        );
    }

    fn round_to_decimals(n: f32, decimal_places: i32) -> f32 {
        let factor = (10.0 as f32).powi(decimal_places);
        (n * factor).round() / factor
    }

    fn reference_dot(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
    }
}

#[derive(Default)]
struct FakeEmbeddingProvider {
    embedding_count: AtomicUsize,
}

impl FakeEmbeddingProvider {
    fn embedding_count(&self) -> usize {
        self.embedding_count.load(atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl EmbeddingProvider for FakeEmbeddingProvider {
    async fn embed_batch(&self, spans: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        self.embedding_count
            .fetch_add(spans.len(), atomic::Ordering::SeqCst);
        Ok(spans
            .iter()
            .map(|span| {
                let mut result = vec![1.0; 26];
                for letter in span.chars() {
                    let letter = letter.to_ascii_lowercase();
                    if letter as u32 >= 'a' as u32 {
                        let ix = (letter as u32) - ('a' as u32);
                        if ix < 26 {
                            result[ix as usize] += 1.0;
                        }
                    }
                }

                let norm = result.iter().map(|x| x * x).sum::<f32>().sqrt();
                for x in &mut result {
                    *x /= norm;
                }

                result
            })
            .collect())
    }
}

fn js_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Javascript".into(),
                path_suffixes: vec!["js".into()],
                ..Default::default()
            },
            Some(tree_sitter_typescript::language_tsx()),
        )
        .with_embedding_query(
            &r#"

            (
                (comment)* @context
                .
                (export_statement
                    (function_declaration
                        "async"? @name
                        "function" @name
                        name: (_) @name)) @item
                    )

            (
                (comment)* @context
                .
                (function_declaration
                    "async"? @name
                    "function" @name
                    name: (_) @name) @item
                    )

            (
                (comment)* @context
                .
                (export_statement
                    (class_declaration
                        "class" @name
                        name: (_) @name)) @item
                    )

            (
                (comment)* @context
                .
                (class_declaration
                    "class" @name
                    name: (_) @name) @item
                    )

            (
                (comment)* @context
                .
                (method_definition
                    [
                        "get"
                        "set"
                        "async"
                        "*"
                        "static"
                    ]* @name
                    name: (_) @name) @item
                )

            (
                (comment)* @context
                .
                (export_statement
                    (interface_declaration
                        "interface" @name
                        name: (_) @name)) @item
                )

            (
                (comment)* @context
                .
                (interface_declaration
                    "interface" @name
                    name: (_) @name) @item
                )

            (
                (comment)* @context
                .
                (export_statement
                    (enum_declaration
                        "enum" @name
                        name: (_) @name)) @item
                )

            (
                (comment)* @context
                .
                (enum_declaration
                    "enum" @name
                    name: (_) @name) @item
                )

                    "#
            .unindent(),
        )
        .unwrap(),
    )
}

fn rust_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".into()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_embedding_query(
            r#"
            (
                (line_comment)* @context
                .
                (enum_item
                    name: (_) @name) @item
            )

            (
                (line_comment)* @context
                .
                (struct_item
                    name: (_) @name) @item
            )

            (
                (line_comment)* @context
                .
                (impl_item
                    trait: (_)? @name
                    "for"? @name
                    type: (_) @name) @item
            )

            (
                (line_comment)* @context
                .
                (trait_item
                    name: (_) @name) @item
            )

            (
                (line_comment)* @context
                .
                (function_item
                    name: (_) @name) @item
            )

            (
                (line_comment)* @context
                .
                (macro_definition
                    name: (_) @name) @item
            )

            (
                (line_comment)* @context
                .
                (function_signature_item
                    name: (_) @name) @item
            )
            "#,
        )
        .unwrap(),
    )
}

fn toml_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "TOML".into(),
            path_suffixes: vec!["toml".into()],
            ..Default::default()
        },
        Some(tree_sitter_toml::language()),
    ))
}

fn cpp_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "CPP".into(),
                path_suffixes: vec!["cpp".into()],
                ..Default::default()
            },
            Some(tree_sitter_cpp::language()),
        )
        .with_embedding_query(
            r#"
            (
                (comment)* @context
                .
                (function_definition
                    (type_qualifier)? @name
                    type: (_)? @name
                    declarator: [
                        (function_declarator
                            declarator: (_) @name)
                        (pointer_declarator
                            "*" @name
                            declarator: (function_declarator
                            declarator: (_) @name))
                        (pointer_declarator
                            "*" @name
                            declarator: (pointer_declarator
                                "*" @name
                            declarator: (function_declarator
                                declarator: (_) @name)))
                        (reference_declarator
                            ["&" "&&"] @name
                            (function_declarator
                            declarator: (_) @name))
                    ]
                    (type_qualifier)? @name) @item
                )

            (
                (comment)* @context
                .
                (template_declaration
                    (class_specifier
                        "class" @name
                        name: (_) @name)
                        ) @item
            )

            (
                (comment)* @context
                .
                (class_specifier
                    "class" @name
                    name: (_) @name) @item
                )

            (
                (comment)* @context
                .
                (enum_specifier
                    "enum" @name
                    name: (_) @name) @item
                )

            (
                (comment)* @context
                .
                (declaration
                    type: (struct_specifier
                    "struct" @name)
                    declarator: (_) @name) @item
            )

            "#,
        )
        .unwrap(),
    )
}
