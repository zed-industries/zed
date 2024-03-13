use crate::{
    embedding_queue::EmbeddingQueue,
    parsing::{subtract_ranges, CodeContextRetriever, Span, SpanDigest},
    semantic_index_settings::SemanticIndexSettings,
    FileToEmbed, JobHandle, SearchResult, SemanticIndex, EMBEDDING_QUEUE_FLUSH_TIMEOUT,
};
use ai::test::FakeEmbeddingProvider;

use gpui::{Task, TestAppContext};
use language::{Language, LanguageConfig, LanguageMatcher, LanguageRegistry, ToOffset};
use parking_lot::Mutex;
use pretty_assertions::assert_eq;
use project::{FakeFs, Fs, Project};
use rand::{rngs::StdRng, Rng};
use serde_json::json;
use settings::{Settings, SettingsStore};
use std::{path::Path, sync::Arc, time::SystemTime};
use unindent::Unindent;
use util::{paths::PathMatcher, RandomCharIter};

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
async fn test_semantic_index(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/the-root",
        json!({
            "src": {
                "file1.rs": "
                    fn aaa() {
                        println!(\"aaaaaaaaaaaa!\");
                    }

                    fn zzzzz() {
                        println!(\"SLEEPING\");
                    }
                ".unindent(),
                "file2.rs": "
                    fn bbb() {
                        println!(\"bbbbbbbbbbbbb!\");
                    }
                    struct pqpqpqp {}
                ".unindent(),
                "file3.toml": "
                    ZZZZZZZZZZZZZZZZZZ = 5
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

    let db_dir = tempfile::Builder::new()
        .prefix("vector-store")
        .tempdir()
        .unwrap();
    let db_path = db_dir.path().join("db.sqlite");

    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let semantic_index = SemanticIndex::new(
        fs.clone(),
        db_path,
        embedding_provider.clone(),
        languages,
        cx.to_async(),
    )
    .await
    .unwrap();

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;

    let search_results = semantic_index.update(cx, |store, cx| {
        store.search_project(
            project.clone(),
            "aaaaaabbbbzz".to_string(),
            5,
            vec![],
            vec![],
            cx,
        )
    });
    let pending_file_count =
        semantic_index.read_with(cx, |index, _| index.pending_file_count(&project).unwrap());
    cx.background_executor.run_until_parked();
    assert_eq!(*pending_file_count.borrow(), 3);
    cx.background_executor
        .advance_clock(EMBEDDING_QUEUE_FLUSH_TIMEOUT);
    assert_eq!(*pending_file_count.borrow(), 0);

    let search_results = search_results.await.unwrap();
    assert_search_results(
        &search_results,
        &[
            (Path::new("src/file1.rs").into(), 0),
            (Path::new("src/file2.rs").into(), 0),
            (Path::new("src/file3.toml").into(), 0),
            (Path::new("src/file1.rs").into(), 45),
            (Path::new("src/file2.rs").into(), 45),
        ],
        cx,
    );

    // Test Include Files Functionality
    let include_files = vec![PathMatcher::new("*.rs").unwrap()];
    let exclude_files = vec![PathMatcher::new("*.rs").unwrap()];
    let rust_only_search_results = semantic_index
        .update(cx, |store, cx| {
            store.search_project(
                project.clone(),
                "aaaaaabbbbzz".to_string(),
                5,
                include_files,
                vec![],
                cx,
            )
        })
        .await
        .unwrap();

    assert_search_results(
        &rust_only_search_results,
        &[
            (Path::new("src/file1.rs").into(), 0),
            (Path::new("src/file2.rs").into(), 0),
            (Path::new("src/file1.rs").into(), 45),
            (Path::new("src/file2.rs").into(), 45),
        ],
        cx,
    );

    let no_rust_search_results = semantic_index
        .update(cx, |store, cx| {
            store.search_project(
                project.clone(),
                "aaaaaabbbbzz".to_string(),
                5,
                vec![],
                exclude_files,
                cx,
            )
        })
        .await
        .unwrap();

    assert_search_results(
        &no_rust_search_results,
        &[(Path::new("src/file3.toml").into(), 0)],
        cx,
    );

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

    cx.background_executor
        .advance_clock(EMBEDDING_QUEUE_FLUSH_TIMEOUT);

    let prev_embedding_count = embedding_provider.embedding_count();
    let index = semantic_index.update(cx, |store, cx| store.index_project(project.clone(), cx));
    cx.background_executor.run_until_parked();
    assert_eq!(*pending_file_count.borrow(), 1);
    cx.background_executor
        .advance_clock(EMBEDDING_QUEUE_FLUSH_TIMEOUT);
    assert_eq!(*pending_file_count.borrow(), 0);
    index.await.unwrap();

    assert_eq!(
        embedding_provider.embedding_count() - prev_embedding_count,
        1
    );
}

#[gpui::test(iterations = 10)]
async fn test_embedding_batching(cx: &mut TestAppContext, mut rng: StdRng) {
    let (outstanding_job_count, _) = postage::watch::channel_with(0);
    let outstanding_job_count = Arc::new(Mutex::new(outstanding_job_count));

    let files = (1..=3)
        .map(|file_ix| FileToEmbed {
            worktree_id: 5,
            path: Path::new(&format!("path-{file_ix}")).into(),
            mtime: SystemTime::now(),
            spans: (0..rng.gen_range(4..22))
                .map(|document_ix| {
                    let content_len = rng.gen_range(10..100);
                    let content = RandomCharIter::new(&mut rng)
                        .with_simple_text()
                        .take(content_len)
                        .collect::<String>();
                    let digest = SpanDigest::from(content.as_str());
                    Span {
                        range: 0..10,
                        embedding: None,
                        name: format!("document {document_ix}"),
                        content,
                        digest,
                        token_count: rng.gen_range(10..30),
                    }
                })
                .collect(),
            job_handle: JobHandle::new(&outstanding_job_count),
        })
        .collect::<Vec<_>>();

    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());

    let mut queue = EmbeddingQueue::new(embedding_provider.clone(), cx.background_executor.clone());
    for file in &files {
        queue.push(file.clone());
    }
    queue.flush();

    cx.background_executor.run_until_parked();
    let finished_files = queue.finished_files();
    let mut embedded_files: Vec<_> = files
        .iter()
        .map(|_| finished_files.try_recv().expect("no finished file"))
        .collect();

    let expected_files: Vec<_> = files
        .iter()
        .map(|file| {
            let mut file = file.clone();
            for doc in &mut file.spans {
                doc.embedding = Some(embedding_provider.embed_sync(doc.content.as_ref()));
            }
            file
        })
        .collect();

    embedded_files.sort_by_key(|f| f.path.clone());

    assert_eq!(embedded_files, expected_files);
}

#[track_caller]
fn assert_search_results(
    actual: &[SearchResult],
    expected: &[(Arc<Path>, usize)],
    cx: &TestAppContext,
) {
    let actual = actual
        .iter()
        .map(|search_result| {
            search_result.buffer.read_with(cx, |buffer, _cx| {
                (
                    buffer.file().unwrap().path().clone(),
                    search_result.range.start.to_offset(buffer),
                )
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[gpui::test]
async fn test_code_context_retrieval_rust() {
    let language = rust_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

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
                unimplemented!();
            }

            // This is a preceding comment
            fn function_2() -> Result<()> {
                unimplemented!();
            }
        }

        #[derive(Clone)]
        struct D {
            name: String
        }
    "
    .unindent();

    let documents = retriever.parse_file(&text, language).unwrap();

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
                    unimplemented!();
                }"
                .unindent(),
                text.find("pub fn function_1").unwrap(),
            ),
            (
                "
                // This is a preceding comment
                fn function_2() -> Result<()> {
                    unimplemented!();
                }"
                .unindent(),
                text.find("fn function_2").unwrap(),
            ),
            (
                "
                #[derive(Clone)]
                struct D {
                    name: String
                }"
                .unindent(),
                text.find("struct D").unwrap(),
            ),
        ],
    );
}

#[gpui::test]
async fn test_code_context_retrieval_json() {
    let language = json_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

    let text = r#"
        {
            "array": [1, 2, 3, 4],
            "string": "abcdefg",
            "nested_object": {
                "array_2": [5, 6, 7, 8],
                "string_2": "hijklmnop",
                "boolean": true,
                "none": null
            }
        }
    "#
    .unindent();

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[(
            r#"
                {
                    "array": [],
                    "string": "",
                    "nested_object": {
                        "array_2": [],
                        "string_2": "",
                        "boolean": true,
                        "none": null
                    }
                }"#
            .unindent(),
            text.find('{').unwrap(),
        )],
    );

    let text = r#"
        [
            {
                "name": "somebody",
                "age": 42
            },
            {
                "name": "somebody else",
                "age": 43
            }
        ]
    "#
    .unindent();

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[(
            r#"
            [{
                    "name": "",
                    "age": 42
                }]"#
            .unindent(),
            text.find('[').unwrap(),
        )],
    );
}

fn assert_documents_eq(
    documents: &[Span],
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

#[gpui::test]
async fn test_code_context_retrieval_javascript() {
    let language = js_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

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

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[
            (
                "
            /* globals importScripts, backend */
            function _authorize() {}"
                    .unindent(),
                37,
            ),
            (
                "
            /**
             * Sometimes the frontend build is way faster than backend.
             */
            export async function authorizeBank() {
                _authorize(pushModal, upgradingAccountId, {});
            }"
                .unindent(),
                131,
            ),
            (
                "
                export class SettingsPage {
                    /* This is a test setting */
                    constructor(page) {
                        this.page = page;
                    }
                }"
                .unindent(),
                225,
            ),
            (
                "
                /* This is a test setting */
                constructor(page) {
                    this.page = page;
                }"
                .unindent(),
                290,
            ),
            (
                "
                /* This is a test comment */
                class TestClass {}"
                    .unindent(),
                374,
            ),
            (
                "
                /* Schema for editor_events in Clickhouse. */
                export interface ClickhouseEditorEvent {
                    installation_id: string
                    operation: string
                }"
                .unindent(),
                440,
            ),
        ],
    )
}

#[gpui::test]
async fn test_code_context_retrieval_lua() {
    let language = lua_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

    let text = r#"
        -- Creates a new class
        -- @param baseclass The Baseclass of this class, or nil.
        -- @return A new class reference.
        function classes.class(baseclass)
            -- Create the class definition and metatable.
            local classdef = {}
            -- Find the super class, either Object or user-defined.
            baseclass = baseclass or classes.Object
            -- If this class definition does not know of a function, it will 'look up' to the Baseclass via the __index of the metatable.
            setmetatable(classdef, { __index = baseclass })
            -- All class instances have a reference to the class object.
            classdef.class = classdef
            --- Recursively allocates the inheritance tree of the instance.
            -- @param mastertable The 'root' of the inheritance tree.
            -- @return Returns the instance with the allocated inheritance tree.
            function classdef.alloc(mastertable)
                -- All class instances have a reference to a superclass object.
                local instance = { super = baseclass.alloc(mastertable) }
                -- Any functions this instance does not know of will 'look up' to the superclass definition.
                setmetatable(instance, { __index = classdef, __newindex = mastertable })
                return instance
            end
        end
        "#.unindent();

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[
            (r#"
                -- Creates a new class
                -- @param baseclass The Baseclass of this class, or nil.
                -- @return A new class reference.
                function classes.class(baseclass)
                    -- Create the class definition and metatable.
                    local classdef = {}
                    -- Find the super class, either Object or user-defined.
                    baseclass = baseclass or classes.Object
                    -- If this class definition does not know of a function, it will 'look up' to the Baseclass via the __index of the metatable.
                    setmetatable(classdef, { __index = baseclass })
                    -- All class instances have a reference to the class object.
                    classdef.class = classdef
                    --- Recursively allocates the inheritance tree of the instance.
                    -- @param mastertable The 'root' of the inheritance tree.
                    -- @return Returns the instance with the allocated inheritance tree.
                    function classdef.alloc(mastertable)
                        --[ ... ]--
                        --[ ... ]--
                    end
                end"#.unindent(),
            114),
            (r#"
            --- Recursively allocates the inheritance tree of the instance.
            -- @param mastertable The 'root' of the inheritance tree.
            -- @return Returns the instance with the allocated inheritance tree.
            function classdef.alloc(mastertable)
                -- All class instances have a reference to a superclass object.
                local instance = { super = baseclass.alloc(mastertable) }
                -- Any functions this instance does not know of will 'look up' to the superclass definition.
                setmetatable(instance, { __index = classdef, __newindex = mastertable })
                return instance
            end"#.unindent(), 810),
        ]
    );
}

#[gpui::test]
async fn test_code_context_retrieval_elixir() {
    let language = elixir_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

    let text = r#"
        defmodule File.Stream do
            @moduledoc """
            Defines a `File.Stream` struct returned by `File.stream!/3`.

            The following fields are public:

            * `path`          - the file path
            * `modes`         - the file modes
            * `raw`           - a boolean indicating if bin functions should be used
            * `line_or_bytes` - if reading should read lines or a given number of bytes
            * `node`          - the node the file belongs to

            """

            defstruct path: nil, modes: [], line_or_bytes: :line, raw: true, node: nil

            @type t :: %__MODULE__{}

            @doc false
            def __build__(path, modes, line_or_bytes) do
            raw = :lists.keyfind(:encoding, 1, modes) == false

            modes =
                case raw do
                true ->
                    case :lists.keyfind(:read_ahead, 1, modes) do
                    {:read_ahead, false} -> [:raw | :lists.keydelete(:read_ahead, 1, modes)]
                    {:read_ahead, _} -> [:raw | modes]
                    false -> [:raw, :read_ahead | modes]
                    end

                false ->
                    modes
                end

            %File.Stream{path: path, modes: modes, raw: raw, line_or_bytes: line_or_bytes, node: node()}

            end"#
    .unindent();

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[(
            r#"
        defmodule File.Stream do
            @moduledoc """
            Defines a `File.Stream` struct returned by `File.stream!/3`.

            The following fields are public:

            * `path`          - the file path
            * `modes`         - the file modes
            * `raw`           - a boolean indicating if bin functions should be used
            * `line_or_bytes` - if reading should read lines or a given number of bytes
            * `node`          - the node the file belongs to

            """

            defstruct path: nil, modes: [], line_or_bytes: :line, raw: true, node: nil

            @type t :: %__MODULE__{}

            @doc false
            def __build__(path, modes, line_or_bytes) do
            raw = :lists.keyfind(:encoding, 1, modes) == false

            modes =
                case raw do
                true ->
                    case :lists.keyfind(:read_ahead, 1, modes) do
                    {:read_ahead, false} -> [:raw | :lists.keydelete(:read_ahead, 1, modes)]
                    {:read_ahead, _} -> [:raw | modes]
                    false -> [:raw, :read_ahead | modes]
                    end

                false ->
                    modes
                end

            %File.Stream{path: path, modes: modes, raw: raw, line_or_bytes: line_or_bytes, node: node()}

            end"#
                .unindent(),
            0,
        ),(r#"
            @doc false
            def __build__(path, modes, line_or_bytes) do
            raw = :lists.keyfind(:encoding, 1, modes) == false

            modes =
                case raw do
                true ->
                    case :lists.keyfind(:read_ahead, 1, modes) do
                    {:read_ahead, false} -> [:raw | :lists.keydelete(:read_ahead, 1, modes)]
                    {:read_ahead, _} -> [:raw | modes]
                    false -> [:raw, :read_ahead | modes]
                    end

                false ->
                    modes
                end

            %File.Stream{path: path, modes: modes, raw: raw, line_or_bytes: line_or_bytes, node: node()}

            end"#.unindent(), 574)],
    );
}

#[gpui::test]
async fn test_code_context_retrieval_cpp() {
    let language = cpp_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

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
        public:           // Access specifier
        int myNum;        // Attribute (int variable)
        string myString;  // Attribute (string variable)
    };

    // This is a test comment
    enum Color { red, green, blue };

    /** This is a preceding block comment
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

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[
            (
                "
        /**
         * @brief Main function
         * @returns 0 on exit
         */
        int main() { return 0; }"
                    .unindent(),
                54,
            ),
            (
                "
                /**
                * This is a test comment
                */
                class MyClass {       // The class
                    public:           // Access specifier
                    int myNum;        // Attribute (int variable)
                    string myString;  // Attribute (string variable)
                }"
                .unindent(),
                112,
            ),
            (
                "
                // This is a test comment
                enum Color { red, green, blue }"
                    .unindent(),
                322,
            ),
            (
                "
                /** This is a preceding block comment
                 * This is the second line
                 */
                struct {           // Structure declaration
                    int myNum;       // Member (int variable)
                    string myString; // Member (string variable)
                } myStructure;"
                    .unindent(),
                425,
            ),
            (
                "
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
                .unindent(),
                612,
            ),
            (
                "
                explicit Matrix(const Integer size) {
                    for (size_t i = 0; i < size; ++i) {
                        _mat.emplace_back(std::vector<T>(size, 0));
                    }
                }"
                .unindent(),
                1226,
            ),
        ],
    );
}

#[gpui::test]
async fn test_code_context_retrieval_ruby() {
    let language = ruby_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

    let text = r#"
        # This concern is inspired by "sudo mode" on GitHub. It
        # is a way to re-authenticate a user before allowing them
        # to see or perform an action.
        #
        # Add `before_action :require_challenge!` to actions you
        # want to protect.
        #
        # The user will be shown a page to enter the challenge (which
        # is either the password, or just the username when no
        # password exists). Upon passing, there is a grace period
        # during which no challenge will be asked from the user.
        #
        # Accessing challenge-protected resources during the grace
        # period will refresh the grace period.
        module ChallengableConcern
            extend ActiveSupport::Concern

            CHALLENGE_TIMEOUT = 1.hour.freeze

            def require_challenge!
                return if skip_challenge?

                if challenge_passed_recently?
                    session[:challenge_passed_at] = Time.now.utc
                    return
                end

                @challenge = Form::Challenge.new(return_to: request.url)

                if params.key?(:form_challenge)
                    if challenge_passed?
                        session[:challenge_passed_at] = Time.now.utc
                    else
                        flash.now[:alert] = I18n.t('challenge.invalid_password')
                        render_challenge
                    end
                else
                    render_challenge
                end
            end

            def challenge_passed?
                current_user.valid_password?(challenge_params[:current_password])
            end
        end

        class Animal
            include Comparable

            attr_reader :legs

            def initialize(name, legs)
                @name, @legs = name, legs
            end

            def <=>(other)
                legs <=> other.legs
            end
        end

        # Singleton method for car object
        def car.wheels
            puts "There are four wheels"
        end"#
        .unindent();

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[
            (
                r#"
        # This concern is inspired by "sudo mode" on GitHub. It
        # is a way to re-authenticate a user before allowing them
        # to see or perform an action.
        #
        # Add `before_action :require_challenge!` to actions you
        # want to protect.
        #
        # The user will be shown a page to enter the challenge (which
        # is either the password, or just the username when no
        # password exists). Upon passing, there is a grace period
        # during which no challenge will be asked from the user.
        #
        # Accessing challenge-protected resources during the grace
        # period will refresh the grace period.
        module ChallengableConcern
            extend ActiveSupport::Concern

            CHALLENGE_TIMEOUT = 1.hour.freeze

            def require_challenge!
                # ...
            end

            def challenge_passed?
                # ...
            end
        end"#
                    .unindent(),
                558,
            ),
            (
                r#"
            def require_challenge!
                return if skip_challenge?

                if challenge_passed_recently?
                    session[:challenge_passed_at] = Time.now.utc
                    return
                end

                @challenge = Form::Challenge.new(return_to: request.url)

                if params.key?(:form_challenge)
                    if challenge_passed?
                        session[:challenge_passed_at] = Time.now.utc
                    else
                        flash.now[:alert] = I18n.t('challenge.invalid_password')
                        render_challenge
                    end
                else
                    render_challenge
                end
            end"#
                    .unindent(),
                663,
            ),
            (
                r#"
                def challenge_passed?
                    current_user.valid_password?(challenge_params[:current_password])
                end"#
                    .unindent(),
                1254,
            ),
            (
                r#"
                class Animal
                    include Comparable

                    attr_reader :legs

                    def initialize(name, legs)
                        # ...
                    end

                    def <=>(other)
                        # ...
                    end
                end"#
                    .unindent(),
                1363,
            ),
            (
                r#"
                def initialize(name, legs)
                    @name, @legs = name, legs
                end"#
                    .unindent(),
                1427,
            ),
            (
                r#"
                def <=>(other)
                    legs <=> other.legs
                end"#
                    .unindent(),
                1501,
            ),
            (
                r#"
                # Singleton method for car object
                def car.wheels
                    puts "There are four wheels"
                end"#
                    .unindent(),
                1591,
            ),
        ],
    );
}

#[gpui::test]
async fn test_code_context_retrieval_php() {
    let language = php_lang();
    let embedding_provider = Arc::new(FakeEmbeddingProvider::default());
    let mut retriever = CodeContextRetriever::new(embedding_provider);

    let text = r#"
        <?php

        namespace LevelUp\Experience\Concerns;

        /*
        This is a multiple-lines comment block
        that spans over multiple
        lines
        */
        function functionName() {
            echo "Hello world!";
        }

        trait HasAchievements
        {
            /**
            * @throws \Exception
            */
            public function grantAchievement(Achievement $achievement, $progress = null): void
            {
                if ($progress > 100) {
                    throw new Exception(message: 'Progress cannot be greater than 100');
                }

                if ($this->achievements()->find($achievement->id)) {
                    throw new Exception(message: 'User already has this Achievement');
                }

                $this->achievements()->attach($achievement, [
                    'progress' => $progress ?? null,
                ]);

                $this->when(value: ($progress === null) || ($progress === 100), callback: fn (): ?array => event(new AchievementAwarded(achievement: $achievement, user: $this)));
            }

            public function achievements(): BelongsToMany
            {
                return $this->belongsToMany(related: Achievement::class)
                ->withPivot(columns: 'progress')
                ->where('is_secret', false)
                ->using(AchievementUser::class);
            }
        }

        interface Multiplier
        {
            public function qualifies(array $data): bool;

            public function setMultiplier(): int;
        }

        enum AuditType: string
        {
            case Add = 'add';
            case Remove = 'remove';
            case Reset = 'reset';
            case LevelUp = 'level_up';
        }

        ?>"#
    .unindent();

    let documents = retriever.parse_file(&text, language.clone()).unwrap();

    assert_documents_eq(
        &documents,
        &[
            (
                r#"
        /*
        This is a multiple-lines comment block
        that spans over multiple
        lines
        */
        function functionName() {
            echo "Hello world!";
        }"#
                .unindent(),
                123,
            ),
            (
                r#"
        trait HasAchievements
        {
            /**
            * @throws \Exception
            */
            public function grantAchievement(Achievement $achievement, $progress = null): void
            {/* ... */}

            public function achievements(): BelongsToMany
            {/* ... */}
        }"#
                .unindent(),
                177,
            ),
            (r#"
            /**
            * @throws \Exception
            */
            public function grantAchievement(Achievement $achievement, $progress = null): void
            {
                if ($progress > 100) {
                    throw new Exception(message: 'Progress cannot be greater than 100');
                }

                if ($this->achievements()->find($achievement->id)) {
                    throw new Exception(message: 'User already has this Achievement');
                }

                $this->achievements()->attach($achievement, [
                    'progress' => $progress ?? null,
                ]);

                $this->when(value: ($progress === null) || ($progress === 100), callback: fn (): ?array => event(new AchievementAwarded(achievement: $achievement, user: $this)));
            }"#.unindent(), 245),
            (r#"
                public function achievements(): BelongsToMany
                {
                    return $this->belongsToMany(related: Achievement::class)
                    ->withPivot(columns: 'progress')
                    ->where('is_secret', false)
                    ->using(AchievementUser::class);
                }"#.unindent(), 902),
            (r#"
                interface Multiplier
                {
                    public function qualifies(array $data): bool;

                    public function setMultiplier(): int;
                }"#.unindent(),
                1146),
            (r#"
                enum AuditType: string
                {
                    case Add = 'add';
                    case Remove = 'remove';
                    case Reset = 'reset';
                    case LevelUp = 'level_up';
                }"#.unindent(), 1265)
        ],
    );
}

fn js_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Javascript".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["js".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_typescript::language_tsx()),
        )
        .with_embedding_query(
            &r#"

            (
                (comment)* @context
                .
                [
                (export_statement
                    (function_declaration
                        "async"? @name
                        "function" @name
                        name: (_) @name))
                (function_declaration
                    "async"? @name
                    "function" @name
                    name: (_) @name)
                ] @item
            )

            (
                (comment)* @context
                .
                [
                (export_statement
                    (class_declaration
                        "class" @name
                        name: (_) @name))
                (class_declaration
                    "class" @name
                    name: (_) @name)
                ] @item
            )

            (
                (comment)* @context
                .
                [
                (export_statement
                    (interface_declaration
                        "interface" @name
                        name: (_) @name))
                (interface_declaration
                    "interface" @name
                    name: (_) @name)
                ] @item
            )

            (
                (comment)* @context
                .
                [
                (export_statement
                    (enum_declaration
                        "enum" @name
                        name: (_) @name))
                (enum_declaration
                    "enum" @name
                    name: (_) @name)
                ] @item
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
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".into()],
                    ..Default::default()
                },
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

            (attribute_item) @collapse
            (use_declaration) @collapse
            "#,
        )
        .unwrap(),
    )
}

fn json_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "JSON".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["json".into()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_json::language()),
        )
        .with_embedding_query(
            r#"
            (document) @item

            (array
                "[" @keep
                .
                (object)? @keep
                "]" @keep) @collapse

            (pair value: (string
                "\"" @keep
                "\"" @keep) @collapse)
            "#,
        )
        .unwrap(),
    )
}

fn toml_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "TOML".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["toml".into()],
                ..Default::default()
            },
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
                matcher: LanguageMatcher {
                    path_suffixes: vec!["cpp".into()],
                    ..Default::default()
                },
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

fn lua_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Lua".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["lua".into()],
                    ..Default::default()
                },
                collapsed_placeholder: "--[ ... ]--".to_string(),
                ..Default::default()
            },
            Some(tree_sitter_lua::language()),
        )
        .with_embedding_query(
            r#"
            (
                (comment)* @context
                .
                (function_declaration
                    "function" @name
                    name: (_) @name
                    (comment)* @collapse
                    body: (block) @collapse
                ) @item
            )
        "#,
        )
        .unwrap(),
    )
}

fn php_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "PHP".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["php".into()],
                    ..Default::default()
                },
                collapsed_placeholder: "/* ... */".into(),
                ..Default::default()
            },
            Some(tree_sitter_php::language_php()),
        )
        .with_embedding_query(
            r#"
            (
                (comment)* @context
                .
                [
                    (function_definition
                        "function" @name
                        name: (_) @name
                        body: (_
                            "{" @keep
                            "}" @keep) @collapse
                        )

                    (trait_declaration
                        "trait" @name
                        name: (_) @name)

                    (method_declaration
                        "function" @name
                        name: (_) @name
                        body: (_
                            "{" @keep
                            "}" @keep) @collapse
                        )

                    (interface_declaration
                        "interface" @name
                        name: (_) @name
                        )

                    (enum_declaration
                        "enum" @name
                        name: (_) @name
                        )

                ] @item
            )
            "#,
        )
        .unwrap(),
    )
}

fn ruby_lang() -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Ruby".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rb".into()],
                    ..Default::default()
                },
                collapsed_placeholder: "# ...".to_string(),
                ..Default::default()
            },
            Some(tree_sitter_ruby::language()),
        )
        .with_embedding_query(
            r#"
            (
                (comment)* @context
                .
                [
                (module
                    "module" @name
                    name: (_) @name)
                (method
                    "def" @name
                    name: (_) @name
                    body: (body_statement) @collapse)
                (class
                    "class" @name
                    name: (_) @name)
                (singleton_method
                    "def" @name
                    object: (_) @name
                    "." @name
                    name: (_) @name
                    body: (body_statement) @collapse)
                ] @item
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
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".into()],
                    ..Default::default()
                },
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
                (#any-match? @name "^(def|defp|defdelegate|defguard|defguardp|defmacro|defmacrop|defn|defnp)$")) @item
                )

            (call
                target: (identifier) @name
                (arguments (alias) @name)
                (#any-match? @name "^(defmodule|defprotocol)$")) @item
            "#,
        )
        .unwrap(),
    )
}

#[gpui::test]
fn test_subtract_ranges() {
    assert_eq!(
        subtract_ranges(&[0..5, 10..21], &[0..1, 4..5]),
        vec![1..4, 10..21]
    );

    assert_eq!(subtract_ranges(&[0..5], &[1..2]), &[0..1, 2..5]);
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        SemanticIndexSettings::register(cx);
        Project::init_settings(cx);
    });
}
