use criterion::{Bencher, BenchmarkId};
use editor::{Editor, EditorMode, MultiBuffer, actions::MoveDown};
use gpui::{AppContext, Focusable as _, TestAppContext, TestDispatcher};
use project::Project;
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::SettingsStore;
use ui::IntoElement;
use util::RandomCharIter;

fn editor_with_one_long_line(_bencher: &mut Bencher<'_>, args: &(String, TestAppContext)) {
    let (text, cx) = args;
    let mut cx = cx.clone();
    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

    let cx = cx.add_empty_window();
    let _editor = cx.update(|window, cx| {
        let editor = cx.new(|cx| Editor::new(EditorMode::full(), buffer, None, window, cx));
        window.focus(&editor.focus_handle(cx));
        editor
    });
}

fn editor_render(bencher: &mut Bencher<'_>, cx: &TestAppContext) {
    let mut cx = cx.clone();
    let buffer = cx.update(|cx| {
        let mut rng = StdRng::seed_from_u64(1);
        let text_len = rng.gen_range(10000..90000);
        if rng.r#gen() {
            let text = RandomCharIter::new(&mut rng)
                .take(text_len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        }
    });

    let cx = cx.add_empty_window();
    let editor = cx.update(|window, cx| {
        let editor = cx.new(|cx| Editor::new(EditorMode::full(), buffer, None, window, cx));
        window.focus(&editor.focus_handle(cx));
        editor
    });

    bencher.iter(|| {
        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| editor.move_down(&MoveDown, window, cx));
            let mut view = editor.clone().into_any_element();
            let _ = view.request_layout(window, cx);
            let _ = view.prepaint(window, cx);
            view.paint(window, cx);
        });
    })
}

pub fn benches() {
    let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(1));
    let cx = gpui::TestAppContext::build(dispatcher, None);
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        // release_channel::init(SemanticVersion::default(), cx);
        client::init_settings(cx);
        language::init(cx);
        workspace::init_settings(cx);
        Project::init_settings(cx);
        editor::init(cx);
    });

    let mut criterion: criterion::Criterion<_> =
        (criterion::Criterion::default()).configure_from_args();

    // setup app context
    criterion.bench_with_input(
        BenchmarkId::new("editor_render", "TestAppContext"),
        &cx,
        editor_render,
    );

    let text = String::from_iter(["char"; 100000]);
    criterion.bench_with_input(
        BenchmarkId::new("editor_with_one_long_line", "(String, TestAppContext )"),
        &(text, cx),
        editor_with_one_long_line,
    );
}

fn main() {
    benches();
    criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
