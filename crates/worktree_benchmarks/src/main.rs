use std::{
    path::Path,
    sync::{Arc, atomic::AtomicUsize},
};

use fs::RealFs;
use gpui::Application;
use settings::WorktreeId;
use worktree::Worktree;

fn main() {
    let Some(worktree_root_path) = std::env::args().nth(1) else {
        println!(
            "Missing path to worktree root\nUsage: bench_background_scan PATH_TO_WORKTREE_ROOT"
        );
        return;
    };
    let app = Application::headless();

    app.run(|cx| {
        settings::init(cx);
        let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));

        cx.spawn(async move |cx| {
            let worktree = Worktree::local(
                Path::new(&worktree_root_path),
                true,
                fs,
                Arc::new(AtomicUsize::new(0)),
                true,
                WorktreeId::from_proto(0),
                cx,
            )
            .await
            .expect("Worktree initialization to succeed");
            let did_finish_scan =
                worktree.update(cx, |this, _| this.as_local().unwrap().scan_complete());
            let start = std::time::Instant::now();
            did_finish_scan.await;
            let elapsed = start.elapsed();
            let (files, directories) =
                worktree.read_with(cx, |this, _| (this.file_count(), this.dir_count()));
            println!(
                "{:?} for {directories} directories and {files} files",
                elapsed
            );
            cx.update(|cx| {
                cx.quit();
            })
        })
        .detach();
    })
}
