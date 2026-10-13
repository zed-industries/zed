//! What the person sees, for OMP (ARCHITECTURE §39, §40): a bounded snapshot
//! goes in front of each prompt, and the full detail is served from Zed
//! through `cedian://` reads OMP makes.
//!
//! Both read the same thing: a [`HostTools`] filled from the open buffers of
//! the workspace folder, the active editor's selection, and the diagnostics
//! Zed's language servers published on those buffers. Buffers outside the
//! folder and Zed's private files (`private_files`, e.g. `.env`) never enter
//! it.

use cedian_workspace::{CedianUri, Diagnostic, DiagnosticSeverity, HostTools, UriKind};
use editor::Editor;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::{FutureExt as _, StreamExt as _};
use gpui::{App, Entity, Task, WeakEntity};
use language::{Buffer, Location, Point, PointUtf16, ToPointUtf16 as _};
use omp_rpc::{HostUri, HostUriRead};
use project::{Project, WorktreeId};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::RecvTimeoutError;
use std::time::{Duration, Instant};
use util::paths::PathStyle;
use util::rel_path::RelPath;
use workspace::Workspace;

/// How long a `cedian://` read waits for the app.
const READ_TIMEOUT: Duration = Duration::from_secs(30);
/// How long the app works on one read before it answers an error: under
/// OMP's own 30 s, so OMP gets the reason rather than its timeout.
const APP_TIMEOUT: Duration = Duration::from_secs(20);

/// One `cedian://` read OMP asked for, answered on the GPUI thread.
pub(crate) struct Read {
    pub url: String,
    /// Set once OMP cancelled the read; the app then skips it.
    pub cancelled: Arc<AtomicBool>,
    pub reply: std::sync::mpsc::Sender<Result<HostUriRead, String>>,
}

/// The `cedian` scheme: each read crosses to the GPUI thread over `reads`.
pub(crate) fn scheme(reads: UnboundedSender<Read>) -> HostUri {
    HostUri::new("cedian", move |url, ctx| {
        let (reply, answer) = std::sync::mpsc::channel();
        let cancelled = Arc::new(AtomicBool::new(false));
        reads
            .unbounded_send(Read {
                url: url.to_string(),
                cancelled: cancelled.clone(),
                reply,
            })
            .map_err(|_| "the cedian panel is closed")?;
        let deadline = Instant::now() + READ_TIMEOUT;
        loop {
            match answer.recv_timeout(Duration::from_millis(50)) {
                Ok(read) => return read.map_err(Into::into),
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(format!("the app dropped the read of {url}").into());
                }
                Err(RecvTimeoutError::Timeout) if ctx.is_cancelled() => {
                    cancelled.store(true, Ordering::SeqCst);
                    return Err(format!("cancelled: {url}").into());
                }
                Err(RecvTimeoutError::Timeout) if Instant::now() >= deadline => {
                    return Err(format!("no answer from the app for {url}").into());
                }
                Err(RecvTimeoutError::Timeout) => {}
            }
        }
    })
    .expect("cedian scheme is valid")
}

/// Answer each `cedian://` read in its own task, with `answer` called when
/// it arrives, so a slow language server holds up only its own read. A read
/// gets an error after [`APP_TIMEOUT`]; a cancelled one is skipped. `None`
/// from `answer` (the panel is gone) stops.
pub(crate) fn serve(
    mut reads: UnboundedReceiver<Read>,
    answer: impl Fn(&str, &mut App) -> Option<Task<Result<HostUriRead, String>>> + 'static,
    cx: &mut App,
) -> Task<()> {
    cx.spawn(async move |cx| {
        while let Some(read) = reads.next().await {
            if read.cancelled.load(Ordering::SeqCst) {
                continue;
            }
            let Some(task) = cx.update(|cx| answer(&read.url, cx)) else {
                break;
            };
            let timeout = cx.background_executor().timer(APP_TIMEOUT);
            cx.spawn(async move |_| {
                let task = task.fuse();
                let timeout = timeout.fuse();
                futures::pin_mut!(task, timeout);
                let answer = futures::select_biased! {
                    answer = task => answer,
                    _ = timeout => Err(format!(
                        "no answer from Zed within {}s for {}",
                        APP_TIMEOUT.as_secs(),
                        read.url
                    )),
                };
                let _ = read.reply.send(answer);
            })
            .detach();
        }
    })
}

/// How long a code-intelligence read waits for language servers to start,
/// inside [`APP_TIMEOUT`].
const SERVER_START_WAIT: Duration = Duration::from_secs(15);

/// Answer one `cedian://` read from Zed.
pub(crate) fn answer(
    url: &str,
    workspace: Option<&WeakEntity<Workspace>>,
    project: &Entity<Project>,
    cx: &mut App,
) -> Task<Result<HostUriRead, String>> {
    let Some(host) = host(workspace, project, cx) else {
        return Task::ready(Err("no folder is open".to_string()));
    };
    match cedian_workspace::parse_cedian_uri(url) {
        Ok(
            uri @ CedianUri {
                kind: UriKind::Definitions | UriKind::References | UriKind::Symbols,
                ..
            },
        ) => lsp_read(&host, project, &uri, cx),
        Ok(CedianUri {
            kind: UriKind::Diagnostics,
            path,
        }) => Task::ready(Ok(host.render_diagnostics(&path).into())),
        Ok(CedianUri {
            kind: UriKind::Buffer,
            path,
        }) => buffer_read(&host, project, &path, cx),
        Ok(CedianUri {
            kind: UriKind::Corrections,
            ..
        }) => Task::ready(corrections_read(project, cx)),
        _ => Task::ready(host.read_uri(url).map_err(|e| e.to_string())),
    }
}

/// ADR-0032: the correction ledger for this workspace's state dir.
fn corrections_read(project: &Entity<Project>, cx: &App) -> Result<HostUriRead, String> {
    let worktree = project
        .read(cx)
        .visible_worktrees(cx)
        .next()
        .ok_or_else(|| "no folder is open".to_string())?;
    let root = worktree.read(cx).abs_path();
    let dir = cedian_shell::state::dir(&root)?;
    cedian_shell::corrections::render_uri(&dir).map(|text| text.into())
}

/// The text Zed holds for `file`, unsaved edits included.
fn buffer_read(
    host: &HostTools,
    project: &Entity<Project>,
    file: &str,
    cx: &mut App,
) -> Task<Result<HostUriRead, String>> {
    let path = inside(host, project, file, cx);
    let project = project.clone();
    cx.spawn(async move |cx| {
        let path = path.await?;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(&path, cx))
            .await
            .map_err(|e| e.to_string())?;
        Ok(cx.update(|cx| buffer.read(cx).text()).into())
    })
}

/// The absolute path of `file` in the folder. Refused when it is private
/// (`private_files`), has no entry in Zed's worktree, or resolves,
/// symlinks followed, outside the folder.
fn inside(
    host: &HostTools,
    project: &Entity<Project>,
    file: &str,
    cx: &mut App,
) -> Task<Result<PathBuf, String>> {
    let Some(tree) = project.read(cx).visible_worktrees(cx).next() else {
        return Task::ready(Err("no folder is open".to_string()));
    };
    let key = match host.resolve(Path::new(file)) {
        Ok(key) => key,
        Err(e) => return Task::ready(Err(e)),
    };
    let root = tree.read(cx).abs_path().to_path_buf();
    let path = root.join(key.strip_prefix("/").unwrap_or(&key));
    let fs = project.read(cx).fs().clone();
    let file = file.to_string();
    cx.spawn(async move |cx| {
        // Only Zed's own entries are answered: they carry on-disk case and
        // `is_private`, and a case-folding fs would open any other spelling.
        let entry_private = |rel: &Path, cx: &mut gpui::AsyncApp| {
            cx.update(|cx| {
                let rel = RelPath::new(rel, PathStyle::local()).ok()?;
                tree.read(cx).entry_for_path(&rel).map(|e| e.is_private)
            })
        };
        let canonical = fs
            .canonicalize(&path)
            .await
            .map_err(|_| format!("no such file in the folder: {file}"))?;
        let canonical_root = fs.canonicalize(&root).await.map_err(|e| e.to_string())?;
        let Ok(rel) = canonical.strip_prefix(&canonical_root) else {
            return Err(format!("{file} resolves outside the folder"));
        };
        let given = path.strip_prefix(&root).unwrap_or(&path);
        for rel in [given, rel] {
            match entry_private(rel, cx) {
                Some(false) => {}
                Some(true) => return Err(format!("{file} is private (private_files)")),
                None => return Err(format!("{file} is not in the folder Zed has open")),
            }
        }
        Ok(path)
    })
}

/// A buffer's `/rel` key in the folder, unless it is outside it or private.
fn key(buffer: &Entity<Buffer>, root: WorktreeId, cx: &App) -> Option<PathBuf> {
    let file = project::File::from_dyn(buffer.read(cx).file())?;
    (file.worktree_id(cx) == root && !file.is_private)
        .then(|| PathBuf::from(format!("/{}", file.path.as_unix_str())))
}

/// Answer a code-intelligence read (`definitions`, `references`, `symbols`)
/// from Zed's language servers, through its LspStore (ADR-0048). Results in
/// a private file or outside the folder are dropped.
pub(crate) fn lsp_read(
    host: &HostTools,
    project: &Entity<Project>,
    uri: &CedianUri,
    cx: &mut App,
) -> Task<Result<HostUriRead, String>> {
    let Some(root) = project.read(cx).visible_worktrees(cx).next() else {
        return Task::ready(Err("no folder is open".to_string()));
    };
    let (file, position) = match uri.kind {
        UriKind::Symbols => match uri.path.strip_prefix("file/") {
            Some(file) => (file, None),
            None => {
                let symbols = project.update(cx, |p, cx| p.symbols(&uri.path, cx));
                let query = uri.path.clone();
                return cx.spawn(async move |cx| {
                    let symbols = symbols.await.map_err(|e| e.to_string())?;
                    let lines = cx.update(|cx| {
                        let root = root.read(cx);
                        symbols
                            .iter()
                            .filter_map(|s| workspace_symbol(s, root))
                            .collect::<Vec<_>>()
                    });
                    Ok(answer_lines(lines, &format!("no symbols match {query:?}")).into())
                });
            }
        },
        _ => match parse_position(&uri.path) {
            Some((file, position)) => (file, Some(position)),
            None => {
                return Task::ready(Err(format!(
                    "{} wants <path>:<line>:<column>, got {:?}",
                    uri_kind_name(&uri.kind),
                    uri.path
                )));
            }
        },
    };
    let root_id = root.read(cx).id();
    let path = inside(host, project, file, cx);
    let file = file.to_string();
    let kind = uri.kind.clone();
    let project = project.clone();
    cx.spawn(async move |cx| {
        let path = path.await?;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(&path, cx))
            .await
            .map_err(|e| e.to_string())?;
        // A file nobody has open has no language server until registered.
        let _lsp = project.update(cx, |p, cx| {
            p.register_buffer_with_language_servers(&buffer, cx)
        });
        let store = project.read_with(cx, |p, _| p.lsp_store());
        // Registering only starts the buffer's servers; wait for them, within
        // the read bound, before deciding the file has none.
        let started = std::time::Instant::now();
        let has_server = loop {
            let (running, starting) = store.update(cx, |store, cx| {
                let ids = buffer.update(cx, |buffer, cx| {
                    store.language_servers_for_local_buffer(buffer, cx)
                });
                let running = ids
                    .iter()
                    .filter(|id| store.language_server_for_id(**id).is_some())
                    .count();
                (running, ids.len() - running)
            });
            if starting == 0 || started.elapsed() >= SERVER_START_WAIT {
                break running > 0;
            }
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
        };
        if !has_server {
            return Err(format!("no language server for {file}"));
        }
        let located = |locations: Vec<Location>, cx: &mut gpui::AsyncApp| {
            cx.update(|cx| {
                locations
                    .iter()
                    .filter_map(|l| location(l, root_id, cx))
                    .collect::<Vec<_>>()
            })
        };
        let (lines, none) = match (kind, position) {
            (UriKind::Definitions, Some(at)) => {
                let links = project
                    .update(cx, |p, cx| p.definitions(&buffer, at, cx))
                    .await
                    .map_err(|e| e.to_string())?
                    .unwrap_or_default();
                let targets = links.into_iter().map(|link| link.target).collect();
                (located(targets, cx), "no definitions found")
            }
            (UriKind::References, Some(at)) => {
                let locations = project
                    .update(cx, |p, cx| p.references(&buffer, at, cx))
                    .await
                    .map_err(|e| e.to_string())?
                    .unwrap_or_default();
                (located(locations, cx), "no references found")
            }
            _ => {
                let symbols = project
                    .update(cx, |p, cx| p.document_symbols(&buffer, cx))
                    .await
                    .map_err(|e| e.to_string())?;
                let mut lines = Vec::new();
                document_symbols(&symbols, &mut lines);
                (lines, "no symbols found")
            }
        };
        Ok(answer_lines(lines, none).into())
    })
}

/// One result per line, or what an empty answer means.
fn answer_lines(lines: Vec<String>, none: &str) -> String {
    if lines.is_empty() {
        none.to_string()
    } else {
        lines.join("\n")
    }
}

/// `name (Kind) /rel:line:column`, for a symbol in a file of the folder
/// that is not private.
fn workspace_symbol(symbol: &project::Symbol, root: &worktree::Worktree) -> Option<String> {
    use project::lsp_store::SymbolLocation;
    let SymbolLocation::InProject(p) = &symbol.path else {
        return None;
    };
    let private = root.entry_for_path(&p.path).is_some_and(|e| e.is_private)
        || root.as_local().is_some_and(|t| t.is_path_private(&p.path));
    if p.worktree_id != root.id() || private {
        return None;
    }
    let start = symbol.range.start.0;
    Some(format!(
        "{} ({:?}) /{}:{}:{}",
        symbol.name,
        symbol.kind,
        p.path.as_unix_str(),
        start.row,
        start.column
    ))
}

fn uri_kind_name(kind: &UriKind) -> &'static str {
    match kind {
        UriKind::Definitions => "cedian://definitions",
        UriKind::References => "cedian://references",
        _ => "cedian://symbols",
    }
}

/// `<path>:<line>:<column>`, both 0-based.
fn parse_position(path: &str) -> Option<(&str, PointUtf16)> {
    let mut parts = path.rsplitn(3, ':');
    let column = parts.next()?.parse().ok()?;
    let line = parts.next()?.parse().ok()?;
    Some((parts.next()?, PointUtf16::new(line, column)))
}

/// `/rel:line:column` of a location's start (UTF-16 column, as the input),
/// unless it is outside the folder or private.
fn location(location: &Location, root: WorktreeId, cx: &App) -> Option<String> {
    let path = key(&location.buffer, root, cx)?;
    let snapshot = location.buffer.read(cx).snapshot();
    let start = location.range.start.to_point_utf16(&snapshot);
    Some(format!("{}:{}:{}", path.display(), start.row, start.column))
}

/// `name (Kind) line:column`, children indented under their parent.
fn document_symbols(symbols: &[project::DocumentSymbol], lines: &mut Vec<String>) {
    fn walk(symbols: &[project::DocumentSymbol], depth: usize, lines: &mut Vec<String>) {
        for s in symbols {
            let start = s.range.start.0;
            lines.push(format!(
                "{}{} ({:?}) {}:{}",
                "  ".repeat(depth),
                s.name,
                s.kind,
                start.row,
                start.column
            ));
            walk(&s.children, depth + 1, lines);
        }
    }
    walk(symbols, 0, lines);
}

/// The open buffers, selection and diagnostics of `project`'s folder.
pub(crate) fn host(
    workspace: Option<&WeakEntity<Workspace>>,
    project: &Entity<Project>,
    cx: &App,
) -> Option<HostTools> {
    let worktree = project.read(cx).visible_worktrees(cx).next()?;
    let root_id = worktree.read(cx).id();
    let host = HostTools::new(&worktree.read(cx).abs_path());
    let key = |buffer: &Entity<Buffer>| key(buffer, root_id, cx);
    for buffer in project.read(cx).opened_buffers(cx) {
        let Some(path) = key(&buffer) else { continue };
        let snapshot = buffer.read(cx).snapshot();
        host.open(&path, &snapshot.text());
        let diagnostics = snapshot
            .diagnostics_in_range::<_, Point>(0..snapshot.len(), false)
            .filter(|entry| entry.diagnostic.is_primary)
            .map(|entry| Diagnostic {
                path: path.clone(),
                line: entry.range.start.row as usize,
                severity: match entry.diagnostic.severity {
                    language::DiagnosticSeverity::ERROR => DiagnosticSeverity::Error,
                    language::DiagnosticSeverity::WARNING => DiagnosticSeverity::Warning,
                    _ => DiagnosticSeverity::Info,
                },
                message: entry.diagnostic.message.to_string(),
            })
            .collect::<Vec<_>>();
        if !diagnostics.is_empty() {
            host.publish_diagnostics(&path, diagnostics);
        }
    }
    let editor = workspace
        .and_then(|w| w.upgrade())
        .and_then(|w| w.read(cx).active_item_as::<Editor>(cx));
    if let Some(editor) = editor {
        let editor = editor.read(cx);
        let multi = editor.buffer().read(cx);
        if let Some(path) = multi.as_singleton().and_then(|b| key(&b)) {
            host.set_active_file(Some(path.clone()));
            let snapshot = multi.snapshot(cx);
            let selection = editor.selections.newest_anchor();
            let start = snapshot.point_to_buffer_offset(selection.start);
            let end = snapshot.point_to_buffer_offset(selection.end);
            if let (Some((_, start)), Some((_, end))) = (start, end)
                && start != end
            {
                host.set_selection(Some((path, start.0, end.0)));
            }
        }
    }
    Some(host)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use language::{FakeLspAdapter, rust_lang};
    use settings::SettingsStore;

    const MAIN: &str = "fn helper() {}\nfn main() { helper(); }\n";

    fn at(line: u32, column: u32) -> lsp::Location {
        at_in("/ws/src/main.rs", line, column)
    }

    fn at_in(path: &str, line: u32, column: u32) -> lsp::Location {
        lsp::Location::new(
            lsp::Uri::from_file_path(path).unwrap(),
            lsp::Range::new(
                lsp::Position::new(line, column),
                lsp::Position::new(line, column + 6),
            ),
        )
    }

    fn init(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    /// A real folder: `src/main.rs`, a private `.env`, and `link.txt`, a
    /// symlink to a file outside the folder.
    async fn real_folder(name: &str, cx: &mut TestAppContext) -> (PathBuf, Entity<Project>) {
        cx.executor().allow_parking();
        init(cx);
        let dir = std::env::temp_dir().join(format!("cedian-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("ws/src")).unwrap();
        std::fs::create_dir_all(dir.join("outside")).unwrap();
        std::fs::write(dir.join("ws/src/main.rs"), MAIN).unwrap();
        std::fs::write(dir.join("ws/.env"), "SECRET=1\n").unwrap();
        std::fs::create_dir_all(dir.join("ws/.git")).unwrap();
        std::fs::write(dir.join("ws/.git/config"), "[core]\n").unwrap();
        std::fs::write(dir.join("outside/secret.txt"), "secret\n").unwrap();
        std::os::unix::fs::symlink(dir.join("outside/secret.txt"), dir.join("ws/link.txt"))
            .unwrap();
        let dir = dir.canonicalize().unwrap();
        let project = Project::test(
            fs::RealFs::new(None, cx.executor()),
            [dir.join("ws").as_path()],
            cx,
        )
        .await;
        let scan = project.read_with(cx, |p, cx| {
            let tree = p.visible_worktrees(cx).next().unwrap();
            tree.read(cx).as_local().unwrap().scan_complete()
        });
        scan.await;
        (dir, project)
    }

    /// `cedian://buffer` is answered from Zed only: never a private file,
    /// never a file a symlink in the folder points to outside it.
    #[gpui::test]
    async fn buffer_reads_refuse_private_files_and_links_outside(cx: &mut TestAppContext) {
        let (dir, project) = real_folder("u6-buffer-guard", cx).await;
        let read = |url: &'static str, cx: &mut TestAppContext| {
            let project = project.clone();
            cx.update(move |cx| answer(url, None, &project, cx))
        };
        let main = read("cedian://buffer/src/main.rs", cx).await.unwrap();
        assert_eq!(main.content, MAIN);
        let private = read("cedian://buffer/.env", cx).await.unwrap_err();
        assert!(private.contains("private"), "{private}");
        read("cedian://buffer/.ENV", cx).await.unwrap_err();
        let sym = read("cedian://symbols/file/.ENV", cx).await.unwrap_err();
        assert!(!sym.contains("no language server"), "{sym}");
        read("cedian://buffer//.ENV", cx).await.unwrap_err();
        let unscanned = read("cedian://buffer/.git/config", cx).await.unwrap_err();
        assert!(unscanned.contains("not in the folder"), "{unscanned}");
        let link = read("cedian://buffer/link.txt", cx).await.unwrap_err();
        assert!(link.contains("escapes"), "{link}");
        let link_key = read("cedian://buffer//link.txt", cx).await.unwrap_err();
        assert!(link_key.contains("outside"), "{link_key}");
        let _ = std::fs::remove_dir_all(dir);
    }

    /// A folder on Zed's fake fs, a private `.env` and `/outside`, with a
    /// fake Rust language server started on `src/main.rs`.
    async fn fake_folder(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Project>,
        lsp::FakeLanguageServer,
        project::lsp_store::OpenLspBufferHandle,
    ) {
        init(cx);
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/ws",
            serde_json::json!({
                "src": {"main.rs": MAIN, "lib.rs": LIB},
                "notes.txt": "notes\n",
                ".env": "SECRET=1\n",
            }),
        )
        .await;
        fs.insert_tree("/outside", serde_json::json!({"x.rs": "fn x() {}\n"}))
            .await;
        let project = Project::test(fs, [Path::new("/ws")], cx).await;
        let languages = project.read_with(cx, |p, _| p.languages().clone());
        languages.add(rust_lang());
        let mut servers = languages.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    definition_provider: Some(lsp::OneOf::Left(true)),
                    references_provider: Some(lsp::OneOf::Left(true)),
                    workspace_symbol_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        let (_buffer, handle) = project
            .update(cx, |p, cx| {
                p.open_local_buffer_with_lsp("/ws/src/main.rs", cx)
            })
            .await
            .unwrap();
        let server = servers.next().await.unwrap();
        (project, server, handle)
    }

    /// A file nobody has open gets its language server started by the read,
    /// and the read waits for it instead of answering there is none.
    #[gpui::test]
    async fn lsp_reads_wait_for_a_starting_server(cx: &mut TestAppContext) {
        init(cx);
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/ws", serde_json::json!({"src": {"main.rs": MAIN}}))
            .await;
        let project = Project::test(fs, [Path::new("/ws")], cx).await;
        let languages = project.read_with(cx, |p, _| p.languages().clone());
        languages.add(rust_lang());
        let mut servers = languages.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    document_symbol_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                initializer: Some(Box::new(|server| {
                    server.set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                        |_, _| async move {
                            #[allow(deprecated)]
                            Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                                lsp::DocumentSymbol {
                                    name: "helper".into(),
                                    detail: None,
                                    kind: lsp::SymbolKind::FUNCTION,
                                    tags: None,
                                    deprecated: None,
                                    range: lsp::Range::new(
                                        lsp::Position::new(0, 0),
                                        lsp::Position::new(0, 14),
                                    ),
                                    selection_range: lsp::Range::new(
                                        lsp::Position::new(0, 3),
                                        lsp::Position::new(0, 9),
                                    ),
                                    children: None,
                                },
                            ])))
                        },
                    );
                })),
                ..Default::default()
            },
        );
        let read = cx.update(|cx| answer("cedian://symbols/file/src/main.rs", None, &project, cx));
        let _server = servers.next().await.unwrap();
        let answer = read.await.unwrap();
        assert!(answer.content.contains("helper"), "{}", answer.content);
    }

    const LIB: &str = "fn a() { /* éé */ helper(); }\n";

    /// Code-intelligence reads never take a private input and never answer
    /// a private file or one outside the folder; columns are UTF-16, as
    /// the input; an empty answer says why.
    #[gpui::test]
    async fn lsp_reads_stay_inside_the_folder(cx: &mut TestAppContext) {
        let (project, server, _handle) = fake_folder(cx).await;
        server.set_request_handler::<lsp::request::GotoDefinition, _, _>(|params, _| async move {
            let uri = params.text_document_position_params.text_document.uri;
            Ok(uri
                .as_str()
                .ends_with("lib.rs")
                .then(|| lsp::GotoDefinitionResponse::Scalar(at_in("/ws/src/lib.rs", 0, 18))))
        });
        server.set_request_handler::<lsp::request::References, _, _>(|_, _| async move {
            Ok(Some(vec![
                at(0, 3),
                at_in("/ws/.env", 0, 0),
                at_in("/outside/x.rs", 0, 3),
            ]))
        });
        #[allow(deprecated)]
        server.set_request_handler::<lsp::request::WorkspaceSymbolRequest, _, _>(
            |_, _| async move {
                let symbol = |name: &str, location| lsp::SymbolInformation {
                    name: name.to_string(),
                    kind: lsp::SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    location,
                    container_name: None,
                };
                Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
                    symbol("helper", at(0, 3)),
                    symbol("SECRET", at_in("/ws/.env", 0, 0)),
                    symbol("x", at_in("/outside/x.rs", 0, 3)),
                ])))
            },
        );
        cx.run_until_parked();
        let read = |url: &'static str, cx: &mut TestAppContext| {
            let project = project.clone();
            cx.update(move |cx| answer(url, None, &project, cx))
        };
        let private = read("cedian://references/.env:0:0", cx).await.unwrap_err();
        assert!(private.contains("private"), "{private}");
        let references = read("cedian://references/src/main.rs:1:12", cx).await;
        assert_eq!(references.unwrap().content, "/src/main.rs:0:3");
        let symbols = read("cedian://symbols/x", cx).await;
        assert_eq!(
            symbols.unwrap().content,
            "helper (Function) /src/main.rs:0:3"
        );
        let utf16 = read("cedian://definitions/src/lib.rs:0:18", cx).await;
        assert_eq!(utf16.unwrap().content, "/src/lib.rs:0:18");
        let none = read("cedian://definitions/src/main.rs:1:12", cx).await;
        assert_eq!(none.unwrap().content, "no definitions found");
        let no_server = read("cedian://definitions/notes.txt:0:0", cx)
            .await
            .unwrap_err();
        assert!(
            no_server.contains("no language server for notes.txt"),
            "{no_server}"
        );
    }

    /// A read whose language server never answers neither blocks the reads
    /// after it nor waits past the app's own timeout.
    #[gpui::test]
    async fn a_stuck_read_blocks_nothing(cx: &mut TestAppContext) {
        let (project, server, _handle) = fake_folder(cx).await;
        server.set_request_handler::<lsp::request::GotoDefinition, _, _>(|_, _| async move {
            futures::future::pending::<anyhow::Result<Option<lsp::GotoDefinitionResponse>>>().await
        });
        // Let the server finish initializing, so the request reaches it.
        cx.run_until_parked();
        let (reads, rx) = futures::channel::mpsc::unbounded();
        let _serve = cx.update(|cx| {
            let project = project.clone();
            serve(rx, move |url, cx| Some(answer(url, None, &project, cx)), cx)
        });
        let send = |url: &str, cancelled: bool| {
            let (reply, answer) = std::sync::mpsc::channel();
            reads
                .unbounded_send(Read {
                    url: url.to_string(),
                    cancelled: Arc::new(AtomicBool::new(cancelled)),
                    reply,
                })
                .unwrap();
            answer
        };
        let stuck = send("cedian://definitions/src/main.rs:1:12", false);
        let cancelled = send("cedian://active-file", true);
        let selection = send("cedian://selection", false);
        cx.run_until_parked();
        assert_eq!(selection.try_recv().unwrap().unwrap().content, "");
        assert_eq!(
            cancelled.try_recv(),
            Err(std::sync::mpsc::TryRecvError::Disconnected),
            "a cancelled read is skipped"
        );
        assert!(stuck.try_recv().is_err(), "still waiting on the server");
        cx.executor().advance_clock(APP_TIMEOUT);
        cx.run_until_parked();
        let timed_out = stuck.try_recv().unwrap().unwrap_err();
        assert!(timed_out.contains("no answer"), "{timed_out}");
    }

    /// Definitions, references and symbols are Zed's language server's
    /// answers, asked through the LspStore for the position the URI names.
    #[gpui::test]
    async fn code_intelligence_reads_come_from_zeds_language_server(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree("/ws", serde_json::json!({"src": {"main.rs": MAIN}}))
            .await;
        let project = Project::test(fs, [Path::new("/ws")], cx).await;
        let languages = project.read_with(cx, |p, _| p.languages().clone());
        languages.add(rust_lang());
        let mut servers = languages.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    definition_provider: Some(lsp::OneOf::Left(true)),
                    references_provider: Some(lsp::OneOf::Left(true)),
                    workspace_symbol_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        let read = |url: &'static str, cx: &mut TestAppContext| {
            let project = project.clone();
            cx.update(move |cx| {
                let host = host(None, &project, cx).unwrap();
                let uri = cedian_workspace::parse_cedian_uri(url).unwrap();
                lsp_read(&host, &project, &uri, cx)
            })
        };
        let (_buffer, _handle) = project
            .update(cx, |p, cx| {
                p.open_local_buffer_with_lsp("/ws/src/main.rs", cx)
            })
            .await
            .unwrap();
        let server = servers.next().await.unwrap();
        server.set_request_handler::<lsp::request::GotoDefinition, _, _>(|params, _| async move {
            let at_position = params.text_document_position_params.position;
            assert_eq!(at_position, lsp::Position::new(1, 12));
            Ok(Some(lsp::GotoDefinitionResponse::Scalar(at(0, 3))))
        });
        server.set_request_handler::<lsp::request::References, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(1, 12)
            );
            Ok(Some(vec![at(0, 3), at(1, 12)]))
        });
        #[allow(deprecated)]
        server.set_request_handler::<lsp::request::WorkspaceSymbolRequest, _, _>(
            |params, _| async move {
                assert_eq!(params.query, "help");
                Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
                    lsp::SymbolInformation {
                        name: "helper".to_string(),
                        kind: lsp::SymbolKind::FUNCTION,
                        tags: None,
                        deprecated: None,
                        location: at(0, 3),
                        container_name: None,
                    },
                ])))
            },
        );
        cx.run_until_parked();
        let definitions = read("cedian://definitions/src/main.rs:1:12", cx);
        assert_eq!(definitions.await.unwrap().content, "/src/main.rs:0:3");
        let references = read("cedian://references/src/main.rs:1:12", cx);
        assert_eq!(
            references.await.unwrap().content,
            "/src/main.rs:0:3\n/src/main.rs:1:12"
        );
        let symbols = read("cedian://symbols/help", cx);
        assert_eq!(
            symbols.await.unwrap().content,
            "helper (Function) /src/main.rs:0:3"
        );
        let outside = read("cedian://definitions/../etc/passwd:0:0", cx);
        assert!(outside.await.unwrap_err().contains("escapes"));
        let shapeless = read("cedian://references/src/main.rs", cx);
        assert!(
            shapeless
                .await
                .unwrap_err()
                .contains("<path>:<line>:<column>")
        );
    }
}
