use std::sync::LazyLock;

use buffer_diff::BufferDiff;
use client::{Collaborator, ParticipantIndex};
use collections::HashMap;
use editor::{
    CollaborationHub, Editor, EditorMode, MultiBuffer, MultiBufferOffset, PathKey,
    actions::Backspace,
};
use gpui::{
    App, AppContext as _, BenchAppContext, BenchWindowContext, Entity, Focusable as _, SharedString,
};
use language::{Buffer, Capability, Point};
use rpc::proto::PeerId;
use settings::SettingsStore;
use zed_actions::editor::{MoveDown, MoveUp};

const LINES_PER_FILE: usize = 20;
const CHANGED_FILES_PER_BATCH: usize = 16;
const CHURN_LINE: &str = "// churn\n";

/// Stands in for the project of a real diff view. Without a collaboration hub the editor
/// skips the remote cursor scan that a project-backed editor runs on every frame.
struct EmptyCollaborationHub;

impl CollaborationHub for EmptyCollaborationHub {
    fn collaborators<'a>(&self, _: &'a App) -> &'a HashMap<PeerId, Collaborator> {
        static COLLABORATORS: LazyLock<HashMap<PeerId, Collaborator>> =
            LazyLock::new(HashMap::default);
        &COLLABORATORS
    }

    fn user_participant_indices<'a>(&self, _: &'a App) -> &'a HashMap<u64, ParticipantIndex> {
        static PARTICIPANT_INDICES: LazyLock<HashMap<u64, ParticipantIndex>> =
            LazyLock::new(HashMap::default);
        &PARTICIPANT_INDICES
    }

    fn user_names(&self, _: &App) -> HashMap<u64, SharedString> {
        HashMap::default()
    }

    fn should_broadcast_selections(&self, _: &App) -> bool {
        false
    }
}

struct DiffEditor<'a, 'measurement> {
    // Keeps the window alive for the duration of the benchmark.
    _window: BenchWindowContext<'a, 'measurement>,
    editor: Entity<Editor>,
    files: Vec<(Entity<Buffer>, Entity<BufferDiff>)>,
}

fn file_marker(file_index: usize) -> String {
    format!("// file {file_index:05}")
}

fn file_text(file_index: usize, changed: bool) -> String {
    let mut text = file_marker(file_index);
    text.push('\n');
    for line in 0..LINES_PER_FILE {
        if changed && line % 8 == 3 {
            text.push_str(&format!(
                "pub fn function_{file_index:05}_{line:02}() -> usize {{ {line} + 1 }}\n"
            ));
        } else {
            text.push_str(&format!(
                "pub fn function_{file_index:05}_{line:02}() -> usize {{ {line} }}\n"
            ));
        }
    }
    text
}

/// Builds an editor shaped like the project diff: one excerpt per changed file, each with
/// expanded diff hunks, and the cursor in the file in the middle.
fn build_diff_editor<'a, 'measurement>(
    file_count: usize,
    cx: &mut BenchAppContext<'a, 'measurement>,
) -> DiffEditor<'a, 'measurement> {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });

    let middle_index = file_count / 2;
    let (multibuffer, files) = cx.update(|cx| {
        let language = language::rust_lang();
        let mut files = Vec::with_capacity(file_count);
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            for file_index in 0..file_count {
                let buffer = cx.new(|cx| {
                    Buffer::local(file_text(file_index, true), cx)
                        .with_language(language.clone(), cx)
                });
                let diff = cx.new(|cx| {
                    BufferDiff::new_with_base_text(
                        &file_text(file_index, false),
                        &buffer.read(cx).text_snapshot(),
                        cx,
                    )
                });
                multibuffer.set_excerpts_for_path(
                    PathKey::sorted(file_index as u64),
                    buffer.clone(),
                    [Point::zero()..buffer.read(cx).max_point()],
                    0,
                    cx,
                );
                multibuffer.add_diff(diff.clone(), cx);
                files.push((buffer, diff));
            }
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        (multibuffer, files)
    });
    cx.run_until_idle();

    let cursor_offset = cx.update(|cx| {
        let marker = file_marker(middle_index);
        let text = multibuffer.read(cx).snapshot(cx).text();
        let marker_start = text
            .find(&marker)
            .expect("the middle file must be in the multibuffer");
        MultiBufferOffset(marker_start + marker.len())
    });

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = window.replace_root(cx, |window, cx| {
            let mut editor = Editor::new(EditorMode::full(), multibuffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor.set_collaboration_hub(Box::new(EmptyCollaborationHub));
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges([cursor_offset..cursor_offset])
            });
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });
    window.run_until_idle();

    DiffEditor {
        _window: window,
        editor,
        files,
    }
}

/// One frame after a cursor move, with no buffer changes: the cost of scrolling and
/// navigating in a large diff.
#[gpui::bench(
    inputs = file_counts(),
    group = "Multibuffer diff frame",
    input_name = "files",
    sample_size = 10
)]
fn multibuffer_diff_frame(file_count: &usize, cx: &mut BenchAppContext) {
    let diff_editor = build_diff_editor(*file_count, cx);

    let mut move_down = true;
    cx.bench_renderer(diff_editor.editor.clone(), move |editor, window, cx| {
        if move_down {
            editor.move_down(&MoveDown, window, cx);
        } else {
            editor.move_up(&MoveUp, window, cx);
        }
        move_down = !move_down;
    });
}

/// One edit in a file in the middle of the diff, its diff recalculation, and the frame
/// that follows: the cost of typing, and of one file changing on disk.
#[gpui::bench(
    inputs = file_counts(),
    group = "Multibuffer diff edit",
    input_name = "files",
    sample_size = 10
)]
fn multibuffer_diff_edit(file_count: &usize, cx: &mut BenchAppContext) {
    let diff_editor = build_diff_editor(*file_count, cx);
    let (buffer, diff) = diff_editor.files[*file_count / 2].clone();

    let mut insert = true;
    cx.bench_renderer(diff_editor.editor.clone(), move |editor, window, cx| {
        if insert {
            editor.handle_input("x", window, cx);
        } else {
            editor.backspace(&Backspace, window, cx);
        }
        insert = !insert;
        let buffer_snapshot = buffer.read(cx).text_snapshot();
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(&buffer_snapshot, cx)
        });
    });
}

/// A batch of files spread over the whole diff change outside the editor, each gets its
/// diff recalculated, and one frame follows: the cost of files changing on disk while
/// the diff is open.
#[gpui::bench(
    inputs = file_counts(),
    group = "Multibuffer diff files changed",
    input_name = "files",
    sample_size = 10
)]
fn multibuffer_diff_files_changed(file_count: &usize, cx: &mut BenchAppContext) {
    let diff_editor = build_diff_editor(*file_count, cx);
    let changed_files = (0..CHANGED_FILES_PER_BATCH)
        .map(|batch_index| {
            diff_editor.files[batch_index * *file_count / CHANGED_FILES_PER_BATCH].clone()
        })
        .collect::<Vec<_>>();

    let mut append = true;
    cx.bench_renderer(diff_editor.editor.clone(), move |_, _, cx| {
        for (buffer, diff) in &changed_files {
            let buffer_snapshot = buffer.update(cx, |buffer, cx| {
                let end = buffer.len();
                if append {
                    buffer.edit([(end..end, CHURN_LINE)], None, cx);
                } else {
                    buffer.edit([(end - CHURN_LINE.len()..end, "")], None, cx);
                }
                buffer.text_snapshot()
            });
            diff.update(cx, |diff, cx| {
                diff.recalculate_diff_sync(&buffer_snapshot, cx)
            });
        }
        append = !append;
    });
}

fn file_counts() -> Vec<usize> {
    let mut file_counts = vec![1000, 4000];
    if std::env::var("ZED_BENCH_HUGE").is_ok() {
        file_counts.push(16000);
    }
    file_counts
}

gpui::bench_group!(
    benches,
    multibuffer_diff_frame,
    multibuffer_diff_edit,
    multibuffer_diff_files_changed
);
gpui::bench_main!(benches);
