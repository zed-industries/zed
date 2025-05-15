use std::{collections::BTreeMap, sync::Arc};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use extension::{
    ExtensionCapability, ExtensionHostProxy, ExtensionManifest, GrammarManifestEntry,
    LanguageServerManifestEntry, SchemaVersion,
};
use extension_host::wasm_host::WasmHost;
use fs::RealFs;
use gpui::{SemanticVersion, TestAppContext, TestDispatcher};
use http_client::{FakeHttpClient, Response};
use node_runtime::NodeRuntime;
use rand::{SeedableRng, rngs::StdRng};
use serde_json::json;
use settings::SettingsStore;
use util::test::TempTree;

fn extension_benchmarks(c: &mut Criterion) {
    let cx = init();

    let mut group = c.benchmark_group("load");

    let extensions_dir = TempTree::new(json!({
        "installed": {},
        "work": {}
    }));
    let wasm_host = wasm_host(&cx, &extensions_dir);
    let manifest = manifest();

    group.bench_function(BenchmarkId::new("init", 1), |b| {
        b.iter(|| {
            let task = wasm_host.load_extension(vec![], &manifest, cx.executor());

            let _extension = cx.executor().block(task).unwrap();
        });
    });
}

fn init() -> TestAppContext {
    const SEED: u64 = 9999;
    let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(SEED));
    let cx = TestAppContext::build(dispatcher, None);
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        release_channel::init(SemanticVersion::default(), cx);
    });
    cx
}

fn wasm_host(cx: &TestAppContext, extensions_dir: &TempTree) -> Arc<WasmHost> {
    let work_dir = extensions_dir.path().canonicalize().unwrap().join("work");
    let fs = Arc::new(RealFs::new(None, cx.executor()));
    let extension_client = FakeHttpClient::create(async |_| {
        Ok(Response::builder().status(404).body("not found".into())?)
    });
    cx.update(|cx| {
        WasmHost::new(
            fs,
            extension_client,
            NodeRuntime::unavailable(),
            Arc::new(ExtensionHostProxy::new()),
            work_dir,
            cx,
        )
    })
}

fn manifest() -> Arc<ExtensionManifest> {
    Arc::new(ExtensionManifest {
        id: "test-extension".into(),
        name: "Test Extension".into(),
        version: "0.1.0".into(),
        schema_version: SchemaVersion(1),
        description: Some("An extension for use in tests.".into()),
        authors: Vec::new(),
        repository: None,
        themes: Default::default(),
        icon_themes: Vec::new(),
        lib: Default::default(),
        languages: Vec::new(),
        grammars: [("gleam".into(), GrammarManifestEntry::default())]
            .into_iter()
            .collect(),
        language_servers: [("gleam".into(), LanguageServerManifestEntry::default())]
            .into_iter()
            .collect(),
        context_servers: BTreeMap::default(),
        slash_commands: BTreeMap::default(),
        indexed_docs_providers: BTreeMap::default(),
        snippets: None,
        capabilities: vec![ExtensionCapability::ProcessExec {
            command: "echo".into(),
            args: vec!["hello!".into()],
        }],
    })
}

criterion_group!(benches, extension_benchmarks);
criterion_main!(benches);
