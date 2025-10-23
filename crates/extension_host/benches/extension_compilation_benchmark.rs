use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use extension::{
    ExtensionCapability, ExtensionHostProxy, ExtensionLibraryKind, ExtensionManifest,
    LanguageServerManifestEntry, LibManifestEntry, SchemaVersion,
    extension_builder::{CompileExtensionOptions, ExtensionBuilder},
};
use extension_host::wasm_host::WasmHost;
use fs::RealFs;
use gpui::{SemanticVersion, TestAppContext, TestDispatcher};
use http_client::{FakeHttpClient, Response};
use node_runtime::NodeRuntime;
use rand::{SeedableRng, rngs::StdRng};
use reqwest_client::ReqwestClient;
use serde_json::json;
use settings::SettingsStore;
use util::test::TempTree;

fn extension_benchmarks(c: &mut Criterion) {
    let cx = init();
    cx.update(gpui_tokio::init);

    let mut group = c.benchmark_group("load");

    let mut manifest = manifest();
    let wasm_bytes = wasm_bytes(&cx, &mut manifest);
    let manifest = Arc::new(manifest);
    let extensions_dir = TempTree::new(json!({
        "installed": {},
        "work": {}
    }));
    let wasm_host = wasm_host(&cx, &extensions_dir);

    group.bench_function(BenchmarkId::from_parameter(1), |b| {
        b.iter_batched(
            || wasm_bytes.clone(),
            |wasm_bytes| {
                let _extension = cx
                    .executor()
                    .block(wasm_host.load_extension(wasm_bytes, &manifest, &cx.to_async()))
                    .unwrap();
            },
            BatchSize::SmallInput,
        );
    });
}

fn init() -> TestAppContext {
    const SEED: u64 = 9999;
    let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(SEED));
    let cx = TestAppContext::build(dispatcher, None);
    cx.executor().allow_parking();
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        release_channel::init(SemanticVersion::default(), cx);
    });

    cx
}

fn wasm_bytes(cx: &TestAppContext, manifest: &mut ExtensionManifest) -> Vec<u8> {
    let extension_builder = extension_builder();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("extensions/test-extension");
    cx.executor()
        .block(extension_builder.compile_extension(
            &path,
            manifest,
            CompileExtensionOptions { release: true },
        ))
        .unwrap();
    std::fs::read(path.join("extension.wasm")).unwrap()
}

fn extension_builder() -> ExtensionBuilder {
    let user_agent = format!(
        "Zed Extension CLI/{} ({}; {})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let http_client = Arc::new(ReqwestClient::user_agent(&user_agent).unwrap());
    // Local dir so that we don't have to download it on every run
    let build_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/.build");
    ExtensionBuilder::new(http_client, build_dir)
}

fn wasm_host(cx: &TestAppContext, extensions_dir: &TempTree) -> Arc<WasmHost> {
    let http_client = FakeHttpClient::create(async |_| {
        Ok(Response::builder().status(404).body("not found".into())?)
    });
    let extensions_dir = extensions_dir.path().canonicalize().unwrap();
    let work_dir = extensions_dir.join("work");
    let fs = Arc::new(RealFs::new(None, cx.executor()));

    cx.update(|cx| {
        WasmHost::new(
            fs,
            http_client,
            NodeRuntime::unavailable(),
            Arc::new(ExtensionHostProxy::new()),
            work_dir,
            cx,
        )
    })
}

fn manifest() -> ExtensionManifest {
    ExtensionManifest {
        id: "test-extension".into(),
        name: "Test Extension".into(),
        version: "0.1.0".into(),
        schema_version: SchemaVersion(1),
        description: Some("An extension for use in tests.".into()),
        authors: Vec::new(),
        repository: None,
        themes: Default::default(),
        icon_themes: Vec::new(),
        lib: LibManifestEntry {
            kind: Some(ExtensionLibraryKind::Rust),
            version: Some(SemanticVersion::new(0, 1, 0)),
        },
        languages: Vec::new(),
        grammars: BTreeMap::default(),
        language_servers: [("gleam".into(), LanguageServerManifestEntry::default())]
            .into_iter()
            .collect(),
        context_servers: BTreeMap::default(),
        slash_commands: BTreeMap::default(),
        snippets: None,
        capabilities: vec![ExtensionCapability::ProcessExec(
            extension::ProcessExecCapability {
                command: "echo".into(),
                args: vec!["hello!".into()],
            },
        )],
        debug_adapters: Default::default(),
        debug_locators: Default::default(),
    }
}

criterion_group!(benches, extension_benchmarks);
criterion_main!(benches);
