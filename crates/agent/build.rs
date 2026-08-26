// RustEmbed expands embedded files during proc-macro expansion, which cargo
// cannot track: editing only an embedded template or guidance file would not
// rebuild this crate, and the binary would keep serving stale content.
// Watch each embedded *.hbs individually so unrelated files in these
// directories don't trigger rebuilds.
//
// Limitation: a newly added *.hbs file is not tracked until the build script
// re-runs for another reason — after adding one, touch this file (or any
// already-tracked .hbs) once to register it. Deletions are detected, since
// losing a tracked file re-runs the build script.
fn main() {
    for dir in ["src/templates", "src/tool_guidance"] {
        watch_hbs_files(std::path::Path::new(dir));
    }
}

fn watch_hbs_files(dir: &std::path::Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            watch_hbs_files(&path);
        } else if path.extension().is_some_and(|ext| ext == "hbs") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
