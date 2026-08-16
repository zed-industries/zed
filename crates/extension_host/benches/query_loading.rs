use std::{borrow::Cow, path::Path, sync::Arc};

use anyhow::Result;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use extension_host::load_plugin_queries;
use fs::{Fs, RealFs};
use futures::{StreamExt as _, future::try_join_all};
use gpui::{TestAppContext, TestDispatcher};
use language::{LanguageQueries, QueryFile, QueryFileContents};
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

    criterion.bench_function("load_plugin_queries/read_dir", |bencher| {
        bencher.iter(|| {
            black_box(
                cx.foreground_executor()
                    .block_on(load_queries_from_directory(fs.clone(), &root_path))
                    .unwrap(),
            );
        });
    });

    criterion.bench_function("load_plugin_queries/direct", |bencher| {
        bencher.iter(|| {
            black_box(
                cx.foreground_executor()
                    .block_on(load_plugin_queries(fs.clone(), &root_path)),
            );
        });
    });
}

async fn load_queries_from_directory(fs: Arc<dyn Fs>, root_path: &Path) -> Result<LanguageQueries> {
    let mut entries = fs.read_dir(root_path).await?;
    let mut query_paths = Vec::new();
    while let Some(path) = entries.next().await {
        let path = path?;
        let Some(query_file) = path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .and_then(QueryFile::from_file_name)
        else {
            continue;
        };
        query_paths.push((query_file, path));
    }

    let files = try_join_all(query_paths.into_iter().map(|(query_file, path)| {
        let fs = fs.clone();
        async move {
            let contents = fs.load(&path).await?;
            anyhow::Ok(QueryFileContents::new(query_file, Cow::Owned(contents)))
        }
    }))
    .await?;
    Ok(LanguageQueries::from_files(files))
}

criterion_group!(benches, query_loading);
criterion_main!(benches);
