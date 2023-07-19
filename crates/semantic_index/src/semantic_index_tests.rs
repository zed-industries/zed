use crate::{
    db::dot,
    embedding::EmbeddingProvider,
    parsing::{subtract_ranges, CodeContextRetriever, Document},
    semantic_index_settings::SemanticIndexSettings,
    SemanticIndex,
};
use anyhow::Result;
use async_trait::async_trait;
use gpui::{Task, TestAppContext};
use language::{Language, LanguageConfig, LanguageRegistry, ToOffset};
use pretty_assertions::assert_eq;
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
async fn test_semantic_index(cx: &mut TestAppContext) {
    cx.update(|cx| {
        cx.set_global(SettingsStore::test(cx));
        settings::register::<SemanticIndexSettings>(cx);
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
    let store = SemanticIndex::new(
        fs.clone(),
        db_path,
        embedding_provider.clone(),
        languages,
        cx.to_async(),
    )
    .await
    .unwrap();

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
    let (file_count, outstanding_file_count) = store
        .update(cx, |store, cx| store.index_project(project.clone(), cx))
        .await
        .unwrap();
    assert_eq!(file_count, 3);
    cx.foreground().run_until_parked();
    assert_eq!(*outstanding_file_count.borrow(), 0);

    let search_results = store
        .update(cx, |store, cx| {
            store.search_project(project.clone(), "aaaa".to_string(), 5, cx)
        })
        .await
        .unwrap();

    search_results[0].buffer.read_with(cx, |buffer, _cx| {
        assert_eq!(search_results[0].range.start.to_offset(buffer), 0);
        assert_eq!(
            buffer.file().unwrap().path().as_ref(),
            Path::new("src/file1.rs")
        );
    });

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
    let (file_count, outstanding_file_count) = store
        .update(cx, |store, cx| store.index_project(project.clone(), cx))
        .await
        .unwrap();
    assert_eq!(file_count, 1);

    cx.foreground().run_until_parked();
    assert_eq!(*outstanding_file_count.borrow(), 0);

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
        #[gpui::test]
        fn a() {
            b
        }

        impl C for D {
        }

        impl E {
            // This is also a preceding comment
            pub fn function_1() -> Option<()> {
                todo!();
            }

            // This is a preceding comment
            fn function_2() -> Result<()> {
                todo!();
            }
        }
    "
    .unindent();

    let documents = retriever
        .parse_file(Path::new("foo.rs"), &text, language)
        .unwrap();

    assert_documents_eq(
        &documents,
        &[
            (
                "
                /// A doc comment
                /// that spans multiple lines
                #[gpui::test]
                fn a() {
                    b
                }"
                .unindent(),
                text.find("fn a").unwrap(),
            ),
            (
                "
                impl C for D {
                }"
                .unindent(),
                text.find("impl C").unwrap(),
            ),
            (
                "
                impl E {
                    // This is also a preceding comment
                    pub fn function_1() -> Option<()> { /* ... */ }

                    // This is a preceding comment
                    fn function_2() -> Result<()> { /* ... */ }
                }"
                .unindent(),
                text.find("impl E").unwrap(),
            ),
            (
                "
                // This is also a preceding comment
                pub fn function_1() -> Option<()> {
                    todo!();
                }"
                .unindent(),
                text.find("pub fn function_1").unwrap(),
            ),
            (
                "
                // This is a preceding comment
                fn function_2() -> Result<()> {
                    todo!();
                }"
                .unindent(),
                text.find("fn function_2").unwrap(),
            ),
        ],
    );
}

fn assert_documents_eq(
    documents: &[Document],
    expected_contents_and_start_offsets: &[(String, usize)],
) {
    assert_eq!(
        documents
            .iter()
            .map(|document| (document.content.clone(), document.range.start))
            .collect::<Vec<_>>(),
        expected_contents_and_start_offsets
    );
}

// #[gpui::test]
// async fn test_code_context_retrieval_javascript() {
//     let language = js_lang();
//     let mut retriever = CodeContextRetriever::new();

//     let text = "
//         /* globals importScripts, backend */
//         function _authorize() {}

//         /**
//          * Sometimes the frontend build is way faster than backend.
//          */
//         export async function authorizeBank() {
//             _authorize(pushModal, upgradingAccountId, {});
//         }

//         export class SettingsPage {
//             /* This is a test setting */
//             constructor(page) {
//                 this.page = page;
//             }
//         }

//         /* This is a test comment */
//         class TestClass {}

//         /* Schema for editor_events in Clickhouse. */
//         export interface ClickhouseEditorEvent {
//             installation_id: string
//             operation: string
//         }
//         "
//     .unindent();

//     let parsed_files = retriever
//         .parse_file(Path::new("foo.js"), &text, language)
//         .unwrap();

//     let test_documents = &[
//         Document {
//             name: "function _authorize".into(),
//             range: text.find("function _authorize").unwrap()..(text.find("}").unwrap() + 1),
//             content: "
//                     The below code snippet is from file 'foo.js'

//                     ```javascript
//                     /* globals importScripts, backend */
//                     function _authorize() {}
//                     ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "async function authorizeBank".into(),
//             range: text.find("export async").unwrap()..223,
//             content: "
//                     The below code snippet is from file 'foo.js'

//                     ```javascript
//                     /**
//                      * Sometimes the frontend build is way faster than backend.
//                      */
//                     export async function authorizeBank() {
//                         _authorize(pushModal, upgradingAccountId, {});
//                     }
//                     ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "class SettingsPage".into(),
//             range: 225..343,
//             content: "
//                     The below code snippet is from file 'foo.js'

//                     ```javascript
//                     export class SettingsPage {
//                         /* This is a test setting */
//                         constructor(page) {
//                             this.page = page;
//                         }
//                     }
//                     ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "constructor".into(),
//             range: 290..341,
//             content: "
//                 The below code snippet is from file 'foo.js'

//                 ```javascript
//                 /* This is a test setting */
//                 constructor(page) {
//                         this.page = page;
//                     }
//                 ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "class TestClass".into(),
//             range: 374..392,
//             content: "
//                     The below code snippet is from file 'foo.js'

//                     ```javascript
//                     /* This is a test comment */
//                     class TestClass {}
//                     ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "interface ClickhouseEditorEvent".into(),
//             range: 440..532,
//             content: "
//                     The below code snippet is from file 'foo.js'

//                     ```javascript
//                     /* Schema for editor_events in Clickhouse. */
//                     export interface ClickhouseEditorEvent {
//                         installation_id: string
//                         operation: string
//                     }
//                     ```"
//             .unindent(),
//             embedding: vec![],
//         },
//     ];

//     for idx in 0..test_documents.len() {
//         assert_eq!(test_documents[idx], parsed_files[idx]);
//     }
// }

// #[gpui::test]
// async fn test_code_context_retrieval_elixir() {
//     let language = elixir_lang();
//     let mut retriever = CodeContextRetriever::new();

//     let text = r#"
// defmodule File.Stream do
//     @moduledoc """
//     Defines a `File.Stream` struct returned by `File.stream!/3`.

//     The following fields are public:

//     * `path`          - the file path
//     * `modes`         - the file modes
//     * `raw`           - a boolean indicating if bin functions should be used
//     * `line_or_bytes` - if reading should read lines or a given number of bytes
//     * `node`          - the node the file belongs to

//     """

//     defstruct path: nil, modes: [], line_or_bytes: :line, raw: true, node: nil

//     @type t :: %__MODULE__{}

//     @doc false
//     def __build__(path, modes, line_or_bytes) do
//     raw = :lists.keyfind(:encoding, 1, modes) == false

//     modes =
//         case raw do
//         true ->
//             case :lists.keyfind(:read_ahead, 1, modes) do
//             {:read_ahead, false} -> [:raw | :lists.keydelete(:read_ahead, 1, modes)]
//             {:read_ahead, _} -> [:raw | modes]
//             false -> [:raw, :read_ahead | modes]
//             end

//         false ->
//             modes
//         end

//     %File.Stream{path: path, modes: modes, raw: raw, line_or_bytes: line_or_bytes, node: node()}

//     end
// "#
//     .unindent();

//     let parsed_files = retriever
//         .parse_file(Path::new("foo.ex"), &text, language)
//         .unwrap();

//     let test_documents = &[
//         Document{
//             name: "defmodule File.Stream".into(),
//             range: 0..1132,
//             content: r#"
//                 The below code snippet is from file 'foo.ex'

//                 ```elixir
//                 defmodule File.Stream do
//                     @moduledoc """
//                     Defines a `File.Stream` struct returned by `File.stream!/3`.

//                     The following fields are public:

//                     * `path`          - the file path
//                     * `modes`         - the file modes
//                     * `raw`           - a boolean indicating if bin functions should be used
//                     * `line_or_bytes` - if reading should read lines or a given number of bytes
//                     * `node`          - the node the file belongs to

//                     """

//                     defstruct path: nil, modes: [], line_or_bytes: :line, raw: true, node: nil

//                     @type t :: %__MODULE__{}

//                     @doc false
//                     def __build__(path, modes, line_or_bytes) do
//                     raw = :lists.keyfind(:encoding, 1, modes) == false

//                     modes =
//                         case raw do
//                         true ->
//                             case :lists.keyfind(:read_ahead, 1, modes) do
//                             {:read_ahead, false} -> [:raw | :lists.keydelete(:read_ahead, 1, modes)]
//                             {:read_ahead, _} -> [:raw | modes]
//                             false -> [:raw, :read_ahead | modes]
//                             end

//                         false ->
//                             modes
//                         end

//                     %File.Stream{path: path, modes: modes, raw: raw, line_or_bytes: line_or_bytes, node: node()}

//                     end
//                 ```"#.unindent(),
//             embedding: vec![],
//         },
//         Document {
//         name: "def __build__".into(),
//         range: 574..1132,
//         content: r#"
// The below code snippet is from file 'foo.ex'

// ```elixir
// @doc false
// def __build__(path, modes, line_or_bytes) do
//     raw = :lists.keyfind(:encoding, 1, modes) == false

//     modes =
//         case raw do
//         true ->
//             case :lists.keyfind(:read_ahead, 1, modes) do
//             {:read_ahead, false} -> [:raw | :lists.keydelete(:read_ahead, 1, modes)]
//             {:read_ahead, _} -> [:raw | modes]
//             false -> [:raw, :read_ahead | modes]
//             end

//         false ->
//             modes
//         end

//     %File.Stream{path: path, modes: modes, raw: raw, line_or_bytes: line_or_bytes, node: node()}

//     end
// ```"#
//             .unindent(),
//         embedding: vec![],
//     }];

//     for idx in 0..test_documents.len() {
//         assert_eq!(test_documents[idx], parsed_files[idx]);
//     }
// }

// #[gpui::test]
// async fn test_code_context_retrieval_cpp() {
//     let language = cpp_lang();
//     let mut retriever = CodeContextRetriever::new();

//     let text = "
//     /**
//      * @brief Main function
//      * @returns 0 on exit
//      */
//     int main() { return 0; }

//     /**
//     * This is a test comment
//     */
//     class MyClass {       // The class
//         public:             // Access specifier
//         int myNum;        // Attribute (int variable)
//         string myString;  // Attribute (string variable)
//     };

//     // This is a test comment
//     enum Color { red, green, blue };

//     /** This is a preceding block comment
//      * This is the second line
//      */
//     struct {           // Structure declaration
//         int myNum;       // Member (int variable)
//         string myString; // Member (string variable)
//     } myStructure;

//     /**
//     * @brief Matrix class.
//     */
//     template <typename T,
//               typename = typename std::enable_if<
//                 std::is_integral<T>::value || std::is_floating_point<T>::value,
//                 bool>::type>
//     class Matrix2 {
//         std::vector<std::vector<T>> _mat;

//     public:
//         /**
//         * @brief Constructor
//         * @tparam Integer ensuring integers are being evaluated and not other
//         * data types.
//         * @param size denoting the size of Matrix as size x size
//         */
//         template <typename Integer,
//                   typename = typename std::enable_if<std::is_integral<Integer>::value,
//                   Integer>::type>
//         explicit Matrix(const Integer size) {
//             for (size_t i = 0; i < size; ++i) {
//                 _mat.emplace_back(std::vector<T>(size, 0));
//             }
//         }
//     }"
//     .unindent();

//     let parsed_files = retriever
//         .parse_file(Path::new("foo.cpp"), &text, language)
//         .unwrap();

//     let test_documents = &[
//         Document {
//             name: "int main".into(),
//             range: 54..78,
//             content: "
//                 The below code snippet is from file 'foo.cpp'

//                 ```cpp
//                 /**
//                  * @brief Main function
//                  * @returns 0 on exit
//                  */
//                 int main() { return 0; }
//                 ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "class MyClass".into(),
//             range: 112..295,
//             content: "
//                 The below code snippet is from file 'foo.cpp'

//                 ```cpp
//                 /**
//                 * This is a test comment
//                 */
//                 class MyClass {       // The class
//                     public:             // Access specifier
//                     int myNum;        // Attribute (int variable)
//                     string myString;  // Attribute (string variable)
//                 }
//                 ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "enum Color".into(),
//             range: 324..355,
//             content: "
//                 The below code snippet is from file 'foo.cpp'

//                 ```cpp
//                 // This is a test comment
//                 enum Color { red, green, blue }
//                 ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "struct myStructure".into(),
//             range: 428..581,
//             content: "
//                 The below code snippet is from file 'foo.cpp'

//                 ```cpp
//                 /** This is a preceding block comment
//                  * This is the second line
//                  */
//                 struct {           // Structure declaration
//                     int myNum;       // Member (int variable)
//                     string myString; // Member (string variable)
//                 } myStructure;
//                 ```"
//             .unindent(),
//             embedding: vec![],
//         },
//         Document {
//             name: "class Matrix2".into(),
//             range: 613..1342,
//             content: "
//                 The below code snippet is from file 'foo.cpp'

//                 ```cpp
//                 /**
//                 * @brief Matrix class.
//                 */
//                 template <typename T,
//                           typename = typename std::enable_if<
//                             std::is_integral<T>::value || std::is_floating_point<T>::value,
//                             bool>::type>
//                 class Matrix2 {
//                     std::vector<std::vector<T>> _mat;

//                 public:
//                     /**
//                     * @brief Constructor
//                     * @tparam Integer ensuring integers are being evaluated and not other
//                     * data types.
//                     * @param size denoting the size of Matrix as size x size
//                     */
//                     template <typename Integer,
//                               typename = typename std::enable_if<std::is_integral<Integer>::value,
//                               Integer>::type>
//                     explicit Matrix(const Integer size) {
//                         for (size_t i = 0; i < size; ++i) {
//                             _mat.emplace_back(std::vector<T>(size, 0));
//                         }
//                     }
//                 }
//                 ```"
//             .unindent(),
//             embedding: vec![],
//         },
//     ];

//     for idx in 0..test_documents.len() {
//         assert_eq!(test_documents[idx], parsed_files[idx]);
//     }
// }

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
                collapsed_placeholder: " /* ... */ ".to_string(),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_embedding_query(
            r#"
            (
                [(line_comment) (attribute_item)]* @context
                .
                [
                    (struct_item
                        name: (_) @name)

                    (enum_item
                        name: (_) @name)

                    (impl_item
                        trait: (_)? @name
                        "for"? @name
                        type: (_) @name)

                    (trait_item
                        name: (_) @name)

                    (function_item
                        name: (_) @name
                        body: (block
                            "{" @keep
                            "}" @keep) @collapse)

                    (macro_definition
                        name: (_) @name)
                ] @item
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

fn elixir_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Elixir".into(),
                path_suffixes: vec!["rs".into()],
                ..Default::default()
            },
            Some(tree_sitter_elixir::language()),
        )
        .with_embedding_query(
            r#"
            (
                (unary_operator
                    operator: "@"
                    operand: (call
                        target: (identifier) @unary
                        (#match? @unary "^(doc)$"))
                    ) @context
                .
                (call
                target: (identifier) @name
                (arguments
                [
                (identifier) @name
                (call
                target: (identifier) @name)
                (binary_operator
                left: (call
                target: (identifier) @name)
                operator: "when")
                ])
                (#match? @name "^(def|defp|defdelegate|defguard|defguardp|defmacro|defmacrop|defn|defnp)$")) @item
                )

            (call
                target: (identifier) @name
                (arguments (alias) @name)
                (#match? @name "^(defmodule|defprotocol)$")) @item
            "#,
        )
        .unwrap(),
    )
}

#[gpui::test]
fn test_subtract_ranges() {
    // collapsed_ranges: Vec<Range<usize>>, keep_ranges: Vec<Range<usize>>

    assert_eq!(
        subtract_ranges(&[0..5, 10..21], &[0..1, 4..5]),
        vec![1..4, 10..21]
    );

    assert_eq!(subtract_ranges(&[0..5], &[1..2]), &[0..1, 2..5]);
}
