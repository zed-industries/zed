use std::{path::PathBuf, sync::Arc};

use benchmarks::bench_utils::random_rust_file;
use editor::{
    Editor, EditorMode, MultiBuffer,
    actions::{DeleteToPreviousWordStart, SelectAll, SplitSelectionIntoLines},
    scroll::ScrollAmount,
};
use gpui::{
    App, AppContext as _, BenchAppContext, BorrowAppContext as _, Focusable as _, UpdateGlobal as _,
};
use indoc::{formatdoc, indoc};
use language::{Buffer, Capability, DiskState, File, LocalFile, Rope};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::{
    DisplayIn, LocalSettingsKind, LocalSettingsPath, SettingsStore, ShowMinimap, WorktreeId,
};
use theme::ActiveTheme as _;
use util::{RandomCharIter, paths::PathStyle, rel_path::RelPath};
use zed_actions::editor::{MoveDown, MoveUp};

struct BenchFile {
    path: Arc<RelPath>,
}

impl File for BenchFile {
    fn as_local(&self) -> Option<&dyn LocalFile> {
        None
    }

    fn disk_state(&self) -> DiskState {
        DiskState::New
    }

    fn path(&self) -> &Arc<RelPath> {
        &self.path
    }

    fn full_path(&self, _: &App) -> PathBuf {
        PathBuf::from("root").join(self.path.as_std_path())
    }

    fn path_style(&self, _: &App) -> PathStyle {
        PathStyle::local()
    }

    fn file_name<'a>(&'a self, _: &'a App) -> &'a str {
        self.path.file_name().unwrap_or("root")
    }

    fn worktree_id(&self, _: &App) -> WorktreeId {
        WorktreeId::from_usize(0)
    }

    fn to_proto(&self, _: &App) -> rpc::proto::File {
        unimplemented!()
    }

    fn is_private(&self) -> bool {
        false
    }
}

#[gpui::bench(
    inputs = multi_cursor_line_counts(),
    group = "Multi-cursor input",
    input_name = "cursors",
    sample_size = 10
)]
fn editor_multi_cursor_input(line_count: &usize, cx: &mut BenchAppContext) {
    init_context(cx);

    let text = "line:\n".repeat(*line_count);
    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor.select_all(&SelectAll, window, cx);
            editor.split_selection_into_lines(
                &SplitSelectionIntoLines {
                    keep_selections: true,
                },
                window,
                cx,
            );
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    cx.bench_iter(|_| {
        window.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.handle_input("hello world", window, cx);
                editor.delete_to_previous_word_start(
                    &DeleteToPreviousWordStart {
                        ignore_newlines: false,
                        ignore_brackets: false,
                    },
                    window,
                    cx,
                );
                editor.delete_to_previous_word_start(
                    &DeleteToPreviousWordStart {
                        ignore_newlines: false,
                        ignore_brackets: false,
                    },
                    window,
                    cx,
                );
            });
        })
    });
}

#[gpui::bench]
fn open_editor_with_one_long_line(cx: &mut BenchAppContext) {
    init_context(cx);

    let text = String::from_iter(["char"; 1000]);
    cx.bench_iter(move |cx| {
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

        let mut window = cx.add_empty_window();
        window.update(|window, cx| {
            let editor = cx.new(|cx| {
                let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                editor.set_style(editor::EditorStyle::default(), window, cx);
                editor
            });
            window.focus(&editor.focus_handle(cx), cx);
            editor
        });
    });
}

#[gpui::bench]
fn editor_render(cx: &mut BenchAppContext) {
    init_context(cx);

    let buffer = cx.update(|cx| {
        let mut rng = StdRng::seed_from_u64(1);
        let text_len = rng.random_range(10000..90000);
        if rng.random() {
            let text = RandomCharIter::new(&mut rng)
                .take(text_len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        }
    });

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = window.replace_root(cx, |window, cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    let mut move_down = true;
    cx.bench_renderer(editor, move |editor, window, cx| {
        if move_down {
            editor.move_down(&MoveDown, window, cx);
        } else {
            editor.move_up(&MoveUp, window, cx);
        }
        move_down = !move_down;
    });
}

#[gpui::bench]
fn editor_render_with_editorconfig(cx: &mut BenchAppContext) {
    init_context(cx);

    let worktree_id = WorktreeId::from_usize(0);
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            let nested_configs = [
                ("", jetbrains_editorconfig()),
                (
                    "src",
                    indoc! {"
                        [*.{ts,tsx}]
                        indent_size = 2
                    "}
                    .to_string(),
                ),
                (
                    "src/app",
                    indoc! {"
                        [*]
                        trim_trailing_whitespace = false

                        [*.ts]
                        max_line_length = 100
                    "}
                    .to_string(),
                ),
                (
                    "src/app/components",
                    indoc! {"
                        [*.{ts,tsx}]
                        indent_style = space
                    "}
                    .to_string(),
                ),
            ];
            for (directory, content) in nested_configs {
                store
                    .set_local_settings(
                        worktree_id,
                        LocalSettingsPath::InWorktree(Arc::from(
                            RelPath::from_unix_str(directory).unwrap(),
                        )),
                        LocalSettingsKind::Editorconfig,
                        Some(&content),
                        cx,
                    )
                    .unwrap();
            }
        });
    });

    let buffer = cx.update(|cx| {
        let text = indented_code_text(3000);
        let file: Arc<dyn File> = Arc::new(BenchFile {
            path: RelPath::from_unix_str("src/app/components/editor_pane.ts")
                .unwrap()
                .into(),
        });
        let buffer = cx.new(|cx| {
            Buffer::build(
                text::Buffer::new(
                    text::ReplicaId::LOCAL,
                    cx.entity_id().as_non_zero_u64().into(),
                    text,
                ),
                Some(file),
                Capability::ReadWrite,
                cx,
            )
        });
        cx.new(|cx| MultiBuffer::singleton(buffer, cx))
    });

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = window.replace_root(cx, |window, cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    let mut insert = true;
    cx.bench_renderer(editor, move |editor, window, cx| {
        if insert {
            editor.handle_input("x", window, cx);
        } else {
            editor.backspace(&editor::actions::Backspace, window, cx);
        }
        insert = !insert;
        editor.move_down(&MoveDown, window, cx);
        editor.move_up(&MoveUp, window, cx);
    });
}

fn indented_code_text(line_count: usize) -> String {
    let mut text = String::new();
    for block in 0..line_count / 10 {
        text.push_str(&format!("export function component{block:04}() {{\n"));
        text.push_str("    const state = {\n");
        text.push_str("        items: [],\n");
        text.push_str("        selection: null,\n");
        text.push_str("    };\n");
        text.push_str("    if (state.items.length > 0) {\n");
        text.push_str("        for (const item of state.items) {\n");
        text.push_str("            console.log(item, state.selection);\n");
        text.push_str("        }\n");
        text.push_str("    }\n");
        text.push_str("}\n");
    }
    text
}

fn jetbrains_editorconfig() -> String {
    let mut content = indoc! {"
        [*]
        charset = utf-8
        end_of_line = lf
        indent_size = 4
        indent_style = space
        insert_final_newline = true
        max_line_length = 150
        tab_width = 4
        trim_trailing_whitespace = false
    "}
    .to_string();
    for key_index in 0..750 {
        content.push_str(&format!("ij_continuation_option_{key_index:04} = false\n"));
    }
    for key_index in 0..1500 {
        content.push_str(&format!(
            "dotnet_diagnostic.ca{key_index:04}.severity = warning\n"
        ));
    }
    for key_index in 0..750 {
        content.push_str(&format!("resharper_style_option_{key_index:04} = true\n"));
    }
    let sections = [
        "*.css",
        "*.feature",
        "*.less",
        "*.properties",
        "*.proto",
        "*.sass",
        "*.scss",
        "*.vue",
        ".editorconfig",
        "{*.ant,*.appxmanifest,*.axml,*.cscfg,*.csdef,*.disco,*.filelayout,*.fxml,*.jhm,*.jnlp,*.jrxml,*.manifest,*.myapp,*.nuspec,*.rng,*.stylecop,*.svcmap,*.tld,*.tps,*.wadcfgx,*.webref,*.wsdl,*.xml,*.xsd,*.xsl,*.xslt,*.xul,StyleCop.Cache}",
        "{*.ats,*.ts}",
        "{*.bash,*.sh,*.zsh}",
        "{*.cjs,*.js}",
        "{*.cjsx,*.coffee}",
        "{*.har,*.inputactions,*.jsb2,*.jsb3,*.json,.babelrc,.eslintrc,.stylelintrc,bowerrc,jest.config}",
        "{*.hcl,*.nomad}",
        "{*.htm,*.html,*.ng,*.sht,*.shtm,*.shtml}",
        "{*.markdown,*.md}",
        "{*.pb,*.textproto}",
        "{*.ps1,*.psd1,*.psm1}",
        "{*.tf,*.tfvars}",
        "{*.yaml,*.yml}",
        "*.js.map",
        "*.{appxmanifest,asax,ascx,aspx,axaml,build,cg,cginc,compute,cs,cshtml,dtd,fs,fsi,fsscript,fsx,hlsl,hlsli,hlslinc,master,ml,mli,nuspec,paml,razor,resw,resx,shader,skin,usf,ush,vb,xaml,xamlx,xoml,xsd}",
    ];
    for (section_index, section) in sections.iter().enumerate() {
        content.push_str(&formatdoc! {"

            [{section}]
            indent_size = 2
        "});
        for key_index in 0..40 {
            content.push_str(&format!(
                "ij_section_{section_index:02}_option_{key_index:02} = false\n"
            ));
        }
    }
    content
}

/// A workspace-shaped window: a 1,000-line editor between four panels of independent
/// rows. `update` selects which entities change per frame, so the modes span the range
/// from "one small view dirty" (`row`) to "every view dirty" (`full`, a full rebuild).
#[gpui::bench(
    inputs = ["row", "editor", "mixed", "full"],
    group = "Workbench",
    input_name = "update",
    sample_size = 20
)]
fn workbench_render(mode: &&str, cx: &mut BenchAppContext) {
    use gpui::{
        Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
        StatefulInteractiveElement, Styled, Window, div, px, rgb,
    };

    struct Row {
        index: usize,
        revision: usize,
    }
    impl Render for Row {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id(self.index)
                .h(px(26.))
                .px_2()
                .flex()
                .justify_between()
                .bg(rgb(if self.revision.is_multiple_of(2) {
                    0x202832
                } else {
                    0x384858
                }))
                .border_b_1()
                .border_color(rgb(0x465060))
                .child(format!("module_{}.rs", self.index))
                .child(format!("{} issues", self.revision % 10))
                .on_click(cx.listener(|row, _, _, cx| {
                    row.revision += 1;
                    cx.notify();
                }))
        }
    }
    struct Panel {
        title: &'static str,
        rows: Vec<Entity<Row>>,
    }
    impl Render for Panel {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .overflow_hidden()
                .child(div().h(px(28.)).child(self.title))
                .children(self.rows.iter().cloned())
        }
    }
    struct Workbench {
        editor: Entity<Editor>,
        panels: [Entity<Panel>; 4],
        step: usize,
    }
    impl Render for Workbench {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(rgb(0x18202a))
                .text_color(rgb(0xdde5ef))
                .child(
                    div()
                        .h(px(32.))
                        .flex_shrink_0()
                        .child("src/main.rs    Cargo.toml    README.md    |    Workbench"),
                )
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .w_full()
                        .min_h_0()
                        .child(
                            div()
                                .w(px(250.))
                                .flex_shrink_0()
                                .h_full()
                                .child(self.panels[0].clone()),
                        )
                        .child(div().flex_1().min_w_0().h_full().child(self.editor.clone()))
                        .child(
                            div()
                                .w(px(300.))
                                .flex_shrink_0()
                                .h_full()
                                .child(self.panels[1].clone()),
                        ),
                )
                .child(
                    div()
                        .h(px(210.))
                        .w_full()
                        .flex_shrink_0()
                        .flex()
                        .child(div().flex_1().min_w_0().child(self.panels[2].clone()))
                        .child(div().flex_1().min_w_0().child(self.panels[3].clone())),
                )
                .child(
                    div()
                        .h(px(24.))
                        .flex_shrink_0()
                        .child("main    Rust    UTF-8    |    48 independent result rows"),
                )
        }
    }

    init_context(cx);
    let mut window = cx.add_empty_window();
    let host = window.update(|window, cx| {
        window.resize(gpui::size(px(1600.), px(1000.)));
        // Test windows do not dispatch platform resize callbacks.
        window.bounds_changed(cx);
        let buffer = MultiBuffer::build_simple(
            &"fn example(value: usize) -> usize { value + 1 }\n".repeat(1000),
            cx,
        );
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        let panels = [
            "Project files",
            "Diagnostics",
            "Search results",
            "Background tasks",
        ]
        .map(|title| {
            cx.new(|cx| Panel {
                title,
                rows: (0..12)
                    .map(|index| cx.new(|_| Row { index, revision: 0 }))
                    .collect(),
            })
        });
        window.replace_root(cx, |_, _| Workbench {
            editor,
            panels,
            step: 0,
        })
    });
    let mode = (*mode).to_owned();
    let update = move |host: &mut Workbench, window: &mut Window, cx: &mut Context<Workbench>| {
        host.step += 1;
        if mode != "row" && (mode != "mixed" || host.step.is_multiple_of(2)) {
            host.editor.update(cx, |editor, cx| {
                if host.step % 4 < 2 {
                    editor.move_down(&MoveDown, window, cx);
                } else {
                    editor.move_up(&MoveUp, window, cx);
                }
            });
        }
        if mode != "editor" {
            for (panel_index, panel) in host.panels.iter().enumerate() {
                if mode == "full" || panel_index == host.step % 4 {
                    panel.update(cx, |panel, cx| {
                        for (index, row) in panel.rows.iter().enumerate() {
                            if mode == "full" || index == host.step % 6 {
                                row.update(cx, |row, cx| {
                                    row.revision += 1;
                                    cx.notify();
                                });
                            }
                        }
                        if mode == "mixed" && host.step.is_multiple_of(8) {
                            panel.rows.rotate_left(1);
                            cx.notify();
                        }
                    });
                }
            }
        }
        if mode == "mixed" && host.step.is_multiple_of(12) {
            window.resize(gpui::size(
                px(if host.step.is_multiple_of(24) {
                    1600.
                } else {
                    1400.
                }),
                px(1000.),
            ));
            window.bounds_changed(cx);
        }
        cx.notify();
    };
    for _ in 0..8 {
        cx.run_until_idle();
        window.update(|window, cx| host.update(cx, |host, cx| update(host, window, cx)));
    }
    cx.bench_renderer(host, update);
}

/// The view tree's worst case: many small views, every one of them dirty on every
/// update, so nothing is reused and each node pays its fixed bookkeeping in full.
#[gpui::bench(
    inputs = [64usize, 256, 1024],
    group = "Siblings",
    input_name = "all dirty",
    sample_size = 20
)]
fn siblings_all_dirty(count: &usize, cx: &mut BenchAppContext) {
    use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb};

    struct Leaf {
        index: usize,
        revision: usize,
    }
    impl Render for Leaf {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size(px(24.))
                .m(px(2.))
                .bg(rgb(if (self.index + self.revision).is_multiple_of(2) {
                    0x336699
                } else {
                    0x996633
                }))
                .child(format!("{}", self.revision % 10))
        }
    }
    struct Host {
        leaves: Vec<Entity<Leaf>>,
    }
    impl Render for Host {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .flex_wrap()
                .bg(rgb(0x18202a))
                .text_color(rgb(0xdde5ef))
                .children(self.leaves.iter().cloned())
        }
    }

    init_context(cx);
    let count = *count;
    let mut window = cx.add_empty_window();
    let host = window.update(|window, cx| {
        window.resize(gpui::size(px(1600.), px(1000.)));
        window.bounds_changed(cx);
        let leaves = (0..count)
            .map(|index| cx.new(|_| Leaf { index, revision: 0 }))
            .collect();
        window.replace_root(cx, |_, _| Host { leaves })
    });
    let update = move |host: &mut Host, _: &mut Window, cx: &mut Context<Host>| {
        for leaf in &host.leaves {
            leaf.update(cx, |leaf, cx| {
                leaf.revision += 1;
                cx.notify();
            });
        }
        cx.notify();
    };
    for _ in 0..4 {
        cx.run_until_idle();
        window.update(|window, cx| host.update(cx, |host, cx| update(host, window, cx)));
    }
    cx.bench_renderer(host, update);
}

/// The view tree's proportional cost: one view (one node) rendering `count` plain
/// elements, re-rendered in full on every update. Nothing here is a node but the host, so
/// the difference from `main` is what each element pays to be recorded: its dispatch op,
/// its hitbox item, its primitives written into the node's scene as well as the frame's,
/// and its text line's handle. Sweeping `count` separates that slope from the fixed cost
/// `Siblings` measures.
#[gpui::bench(
    inputs = [256usize, 2048, 8192],
    group = "Elements",
    input_name = "all dirty",
    sample_size = 20
)]
fn elements_all_dirty(count: &usize, cx: &mut BenchAppContext) {
    bench_elements(*count, false, cx);
}

/// As `Elements/all dirty`, with one clean sibling view so the frame is incremental: the
/// retained layout tree is kept rather than cleared, and the host's previous tree is
/// retired subtree by subtree. This is the shape of a real window with one busy view.
#[gpui::bench(
    inputs = [256usize, 2048, 8192],
    group = "Elements",
    input_name = "incremental",
    sample_size = 20
)]
fn elements_incremental(count: &usize, cx: &mut BenchAppContext) {
    bench_elements(*count, true, cx);
}

fn bench_elements(count: usize, with_clean_sibling: bool, cx: &mut BenchAppContext) {
    use gpui::{
        AnyElement, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
        Styled, Window, div, px, rgb,
    };

    struct Sibling;
    impl Render for Sibling {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size(px(24.)).bg(rgb(0x334455))
        }
    }
    struct Host {
        count: usize,
        revision: usize,
        sibling: Option<Entity<Sibling>>,
    }
    impl Render for Host {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let revision = self.revision;
            div()
                .size_full()
                .flex()
                .flex_wrap()
                .bg(rgb(0x18202a))
                .text_color(rgb(0xdde5ef))
                .children(self.sibling.clone().map(IntoElement::into_any_element))
                .children((0..self.count).map(|index| -> AnyElement {
                    div()
                        .id(index)
                        .size(px(24.))
                        .m(px(2.))
                        .bg(rgb(if (index + revision).is_multiple_of(2) {
                            0x336699
                        } else {
                            0x996633
                        }))
                        .child(format!("{}", revision % 10))
                        .into_any_element()
                }))
        }
    }

    init_context(cx);
    let mut window = cx.add_empty_window();
    let host = window.update(|window, cx| {
        window.resize(gpui::size(px(1600.), px(1000.)));
        window.bounds_changed(cx);
        let sibling = with_clean_sibling.then(|| cx.new(|_| Sibling));
        window.replace_root(cx, |_, _| Host {
            count,
            revision: 0,
            sibling,
        })
    });
    let update = move |host: &mut Host, _: &mut Window, cx: &mut Context<Host>| {
        host.revision += 1;
        cx.notify();
    };
    for _ in 0..4 {
        cx.run_until_idle();
        window.update(|window, cx| host.update(cx, |host, cx| update(host, window, cx)));
    }
    cx.bench_renderer(host, update);
}

/// One point of the `Complexity` sweep: `views` child views (each its own node), each
/// rendering `elements` id'd `div`s that paint about `primitives` primitives each, with
/// `dirty_percent` of the views notified on every update.
#[derive(Clone, Copy)]
struct ComplexityPoint {
    views: usize,
    elements: usize,
    primitives: usize,
    dirty_percent: usize,
    /// Wrap every view in `.cached()`, the opt-in reuse `main` ships for docked panels and
    /// pane items; measures what a clean cached panel costs there.
    cached: bool,
}

impl std::fmt::Display for ComplexityPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "v{}-e{}-p{}-f{}{}",
            self.views,
            self.elements,
            self.primitives,
            self.dirty_percent,
            if self.cached { "-cached" } else { "" }
        )
    }
}

/// One-factor sweeps around a Zed-like baseline (16 views of 128 four-primitive elements,
/// a quarter of the views changing per frame), plus a few interactions, to fit the frame
/// cost model `t = c + dirty (a·views + b·elements + p·primitives) + clean (a'·views +
/// h·elements + r·primitives)`; `main` has only the dirty terms.
fn complexity_points() -> Vec<ComplexityPoint> {
    let point = |views, elements, primitives, dirty_percent| ComplexityPoint {
        views,
        elements,
        primitives,
        dirty_percent,
        cached: false,
    };
    let cached = |views, elements, primitives, dirty_percent| ComplexityPoint {
        cached: true,
        ..point(views, elements, primitives, dirty_percent)
    };
    vec![
        point(16, 128, 4, 25),
        point(4, 128, 4, 25),
        point(64, 128, 4, 25),
        point(16, 32, 4, 25),
        point(16, 512, 4, 25),
        point(16, 128, 1, 25),
        point(16, 128, 12, 25),
        point(16, 128, 4, 0),
        point(16, 128, 4, 6),
        point(16, 128, 4, 100),
        point(64, 128, 4, 6),
        point(16, 512, 4, 6),
        point(4, 512, 4, 100),
        point(64, 32, 4, 100),
        // Primitive and element counts at 0% and 100% pin the clean and dirty
        // per-primitive and per-element terms apart; at 25% alone they are collinear.
        point(16, 128, 1, 100),
        point(16, 128, 12, 100),
        point(16, 128, 1, 0),
        point(16, 128, 12, 0),
        point(16, 512, 4, 0),
        point(16, 512, 4, 100),
        // `.cached()` views: the clean per-element replay cost of `main`'s own caching, per
        // kind and scale, plus a few dirty shares to check that a notified cached view
        // costs what an uncached one does.
        cached(16, 128, 1, 0),
        cached(16, 128, 4, 0),
        cached(16, 128, 12, 0),
        cached(16, 512, 4, 0),
        cached(64, 128, 4, 0),
        cached(16, 128, 4, 6),
        cached(16, 128, 4, 25),
        cached(16, 128, 4, 100),
    ]
}

/// `primitives` is approximate and composed as: 1 — a background quad; 4 — the quad and a
/// three-glyph label (three monochrome sprites); 12 — the quad, a small shadow and a
/// ten-glyph label. Elements are 16×8 px so every point fits in the 1600×1000 window and
/// nothing is culled; labels may overflow their box, which still paints them.
#[gpui::bench(
    inputs = complexity_points(),
    group = "Complexity",
    input_name = "scene",
    sample_size = 20
)]
fn complexity(point: &ComplexityPoint, cx: &mut BenchAppContext) {
    use gpui::{
        AnyElement, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
        StyleRefinement, Styled, Window, div, px, rgb,
    };

    struct View {
        index: usize,
        revision: usize,
        elements: usize,
        primitives: usize,
    }
    impl Render for View {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let revision = self.revision;
            let primitives = self.primitives;
            let view_index = self.index;
            div()
                .w_full()
                .flex()
                .flex_wrap()
                .children((0..self.elements).map(|index| -> AnyElement {
                    let element = div().id(index).w(px(16.)).h(px(8.)).m(px(0.5)).bg(rgb(
                        if (index + revision).is_multiple_of(2) {
                            0x336699
                        } else {
                            0x996633
                        },
                    ));
                    // Labels are fixed per element: a changing string would be reshaped
                    // every frame and the sweep would measure text shaping, not primitives.
                    match primitives {
                        1 => element.into_any_element(),
                        4 => element
                            .child(format!("{}{:02}", view_index % 10, index % 100))
                            .into_any_element(),
                        _ => element
                            .shadow_sm()
                            .child(format!(
                                "{:03}{:03}{:04}",
                                view_index % 1000,
                                index % 1000,
                                0
                            ))
                            .into_any_element(),
                    }
                }))
        }
    }
    struct Host {
        views: Vec<Entity<View>>,
        dirty_per_update: usize,
        next_dirty: usize,
        cached: bool,
    }
    impl Render for Host {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let cached = self.cached;
            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(rgb(0x18202a))
                .text_color(rgb(0xdde5ef))
                .text_size(px(5.))
                .children(self.views.iter().map(|view| -> AnyElement {
                    if cached {
                        view.clone()
                            .cached(StyleRefinement::default().w_full())
                            .into_any_element()
                    } else {
                        view.clone().into_any_element()
                    }
                }))
        }
    }

    init_context(cx);
    let point = *point;
    let mut window = cx.add_empty_window();
    let host = window.update(|window, cx| {
        window.resize(gpui::size(px(1600.), px(1000.)));
        window.bounds_changed(cx);
        let views = (0..point.views)
            .map(|index| {
                cx.new(|_| View {
                    index,
                    revision: 0,
                    elements: point.elements,
                    primitives: point.primitives,
                })
            })
            .collect();
        let dirty_per_update = if point.dirty_percent == 0 {
            0
        } else {
            (point.dirty_percent * point.views).div_ceil(100).max(1)
        };
        window.replace_root(cx, |_, _| Host {
            views,
            dirty_per_update,
            next_dirty: 0,
            cached: point.cached,
        })
    });
    // The dirty views rotate, so the same nodes are not the changing ones every frame; with
    // none dirty the host alone is notified and every view is reused.
    let update = move |host: &mut Host, _: &mut Window, cx: &mut Context<Host>| {
        for _ in 0..host.dirty_per_update {
            let view = &host.views[host.next_dirty % host.views.len()];
            view.update(cx, |view, cx| {
                view.revision += 1;
                cx.notify();
            });
            host.next_dirty += 1;
        }
        cx.notify();
    };
    for _ in 0..4 {
        cx.run_until_idle();
        window.update(|window, cx| host.update(cx, |host, cx| update(host, window, cx)));
    }
    cx.bench_renderer(host, update);
}

/// One point of the editors sweep: `editors` full editors of code in a grid, `dirty` of
/// them changed every update — by moving the cursor, which re-renders the editor with
/// its shaped lines still cached, or by scrolling a page, which brings in lines that
/// have to be shaped (`reshape`).
#[derive(Clone, Copy)]
struct EditorsPoint {
    editors: usize,
    dirty: usize,
    reshape: bool,
    /// Wrap every editor in `.cached()`, as `main`'s pane items are.
    cached: bool,
}

impl std::fmt::Display for EditorsPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "k{}-d{}-{}{}",
            self.editors,
            self.dirty,
            if self.reshape { "scroll" } else { "cursor" },
            if self.cached { "-cached" } else { "" }
        )
    }
}

fn editors_points() -> Vec<EditorsPoint> {
    [
        (1, 0, false, false),
        (1, 1, false, false),
        (1, 1, true, false),
        (4, 0, false, false),
        (4, 1, false, false),
        (4, 1, true, false),
        (4, 4, false, false),
        (4, 4, true, false),
        (1, 0, false, true),
        (4, 0, false, true),
        (4, 1, true, true),
    ]
    .into_iter()
    .map(|(editors, dirty, reshape, cached)| EditorsPoint {
        editors,
        dirty,
        reshape,
        cached,
    })
    .collect()
}

/// The editor's weight for the cost model: each editor is one view painting ~40 lines of
/// code directly, not element by element. A clean editor is replayed; a dirty one renders
/// again, cheaply when its visible lines are still shaped (cursor moves, typing within
/// a line) and dearly when they are not (scrolling, new text). Editors are 780×470 px so
/// four fill the 1600×1000 window.
#[gpui::bench(
    inputs = editors_points(),
    group = "Complexity",
    input_name = "editors",
    sample_size = 20
)]
fn complexity_editors(point: &EditorsPoint, cx: &mut BenchAppContext) {
    use gpui::{
        AnyElement, Context, Entity, IntoElement, ParentElement, Render, StyleRefinement, Styled,
        Window, div, px, rgb,
    };

    struct Host {
        editors: Vec<Entity<Editor>>,
        dirty: usize,
        reshape: bool,
        move_down: bool,
        cached: bool,
    }
    impl Render for Host {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let cached = self.cached;
            div()
                .size_full()
                .flex()
                .flex_wrap()
                .bg(rgb(0x18202a))
                .children(self.editors.iter().map(|editor| {
                    let editor: AnyElement = if cached {
                        editor
                            .clone()
                            .cached(StyleRefinement::default().flex().flex_col().size_full())
                            .into_any_element()
                    } else {
                        editor.clone().into_any_element()
                    };
                    div().w(px(780.)).h(px(470.)).m(px(5.)).child(editor)
                }))
        }
    }

    init_context(cx);
    let point = *point;
    let mut window = cx.add_empty_window();
    let host = window.update(|window, cx| {
        window.resize(gpui::size(px(1600.), px(1000.)));
        window.bounds_changed(cx);
        let editors = (0..point.editors)
            .map(|_| {
                let buffer = MultiBuffer::build_simple(&indented_code_text(400), cx);
                cx.new(|cx| {
                    let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                    editor.set_style(editor::EditorStyle::default(), window, cx);
                    editor
                })
            })
            .collect();
        window.replace_root(cx, |_, _| Host {
            editors,
            dirty: point.dirty,
            reshape: point.reshape,
            move_down: true,
            cached: point.cached,
        })
    });
    let update = move |host: &mut Host, window: &mut Window, cx: &mut Context<Host>| {
        let move_down = host.move_down;
        let reshape = host.reshape;
        for editor in host.editors.iter().take(host.dirty) {
            editor.update(cx, |editor, cx| {
                if reshape {
                    let amount = ScrollAmount::Page(if move_down { 1. } else { -1. });
                    editor.scroll_screen(&amount, window, cx);
                } else if move_down {
                    editor.move_down(&MoveDown, window, cx);
                } else {
                    editor.move_up(&MoveUp, window, cx);
                }
            });
        }
        host.move_down = !move_down;
        cx.notify();
    };
    for _ in 0..4 {
        cx.run_until_idle();
        window.update(|window, cx| host.update(cx, |host, cx| update(host, window, cx)));
    }
    cx.bench_renderer(host, update);
}

#[gpui::bench]
fn editor_render_highlighted(cx: &mut BenchAppContext) {
    init_context(cx);
    render_highlighted_editor(cx);
}

#[gpui::bench]
fn editor_render_highlighted_minimap(cx: &mut BenchAppContext) {
    init_context(cx);
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                let minimap = settings.editor.minimap.get_or_insert_default();
                minimap.show = Some(ShowMinimap::Always);
                minimap.display_in = Some(DisplayIn::AllEditors);
            });
        });
    });
    render_highlighted_editor(cx);
}

fn render_highlighted_editor(cx: &mut BenchAppContext) {
    let mut rng = StdRng::seed_from_u64(1);
    let text = random_rust_file(&mut rng, 10_000).join("\n");
    let language = language::rust_lang();
    let syntax_theme = cx.update(|cx| {
        let syntax_theme = cx.theme().syntax().clone();
        language.set_theme(&syntax_theme);
        syntax_theme
    });
    let probe = "fn main() {}";
    assert!(
        !language
            .highlight_text(&Rope::from(probe), 0..probe.len())
            .is_empty(),
        "the benchmark language must resolve syntax highlights against the theme"
    );
    let buffer = cx.update(|cx| {
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.new(|cx| MultiBuffer::singleton(buffer, cx))
    });
    cx.run_until_idle();

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = window.replace_root(cx, |window, cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(
                editor::EditorStyle {
                    syntax: syntax_theme.clone(),
                    ..editor::EditorStyle::default()
                },
                window,
                cx,
            );
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    let mut move_down = true;
    cx.bench_renderer(editor, move |editor, window, cx| {
        if move_down {
            editor.move_down(&MoveDown, window, cx);
        } else {
            editor.move_up(&MoveUp, window, cx);
        }
        move_down = !move_down;
    });
}

fn init_context(cx: &mut BenchAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
}

fn multi_cursor_line_counts() -> Vec<usize> {
    let mut line_counts = vec![1000, 10000];
    if std::env::var("ZED_BENCH_HUGE").is_ok() {
        line_counts.push(100000);
    }
    line_counts
}

gpui::bench_group!(
    benches,
    editor_multi_cursor_input,
    open_editor_with_one_long_line,
    editor_render,
    editor_render_with_editorconfig,
    workbench_render,
    siblings_all_dirty,
    elements_all_dirty,
    elements_incremental,
    complexity,
    complexity_editors,
    editor_render_highlighted,
    editor_render_highlighted_minimap
);
gpui::bench_main!(benches);
