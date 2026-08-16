use std::sync::Arc;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use extension_host::load_plugin_queries;
use fs::{Fs, RealFs};
use gpui::{TestAppContext, TestDispatcher};
use serde_json::json;
use util::test::TempTree;

fn query_loading(criterion: &mut Criterion) {
    let dispatcher = TestDispatcher::new(9999);
    let cx = TestAppContext::build(dispatcher, None);
    cx.executor().allow_parking();

    let queries = TempTree::new(json!({
        "brackets.scm": "brackets query",
        "highlights.scm": "highlights query",
        "indents.scm": "indents query",
        "injections.scm": "injections query",
        "outline.scm": "outline query",
        "overrides.scm": "overrides query",
        "textobjects.scm": "text objects query",
        "locals.scm": "unrelated query",
    }));
    let root_path = queries.path().to_path_buf();
    let fs: Arc<dyn Fs> = Arc::new(RealFs::new(None, cx.executor()));

    criterion.bench_function("load_plugin_queries/direct", |bencher| {
        bencher.iter(|| {
            black_box(
                cx.foreground_executor()
                    .block_on(load_plugin_queries(fs.clone(), &root_path)),
            );
        });
    });
}

criterion_group!(benches, query_loading);
criterion_main!(benches);
