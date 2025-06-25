use criterion::{
    BatchSize, Bencher, BenchmarkId, Criterion, Throughput, black_box, criterion_group,
    criterion_main,
};
use editor::{Editor, EditorElement, EditorMode, EditorSettings, MultiBuffer};
use gpui::{AppContext, Focusable as _, Render, TestAppContext, TestDispatcher};
use project::Project;
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::{Settings, SettingsStore};
use ui::{Element, IntoElement};
use util::RandomCharIter;

fn editor_render(bencher: &mut Bencher<'_>, cx: &TestAppContext) {
    let mut cx = cx.clone();
    let buffer = cx.update(|cx| {
        let mut rng = StdRng::seed_from_u64(1);
        let text_len = rng.gen_range(10000..100000);
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
    let mut editor = cx.update(|window, cx| {
        let editor = cx.new(|cx| Editor::new(EditorMode::full(), buffer, None, window, cx));
        window.focus(&editor.focus_handle(cx));
        editor
    });

    bencher.iter(|| {
        cx.update(|window, cx| {
            let (_, mut layout_state) = editor.request_layout(None, None, window, cx);
            let mut prepaint =
                editor.prepaint(None, None, window.bounds(), &mut layout_state, window, cx);
            editor.paint(
                None,
                None,
                window.bounds(),
                &mut layout_state,
                &mut prepaint,
                window,
                cx,
            );

            window.refresh();
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

    cx.dispatch_keystroke(window, keystroke);

    // setup app context
    criterion.bench_with_input(
        BenchmarkId::new("editor_render", "TestAppContext"),
        &cx,
        |bencher, cx| editor_render(bencher, cx),
    );
}

fn main() {
    benches();
    criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
