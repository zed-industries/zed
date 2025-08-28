use super::*;
use crate::{
    LanguageConfig, LanguageMatcher,
    buffer_tests::{markdown_inline_lang, markdown_lang},
};
use gpui::App;
use rand::rngs::StdRng;
use std::{env, ops::Range, sync::Arc};
use text::{Buffer, BufferId};
use tree_sitter::Node;
use unindent::Unindent as _;
use util::test::marked_text_ranges;

#[test]
fn test_splice_included_ranges() {
    let ranges = vec![ts_range(20..30), ts_range(50..60), ts_range(80..90)];

    let (new_ranges, change) = splice_included_ranges(
        ranges.clone(),
        &[54..56, 58..68],
        &[ts_range(50..54), ts_range(59..67)],
    );
    assert_eq!(
        new_ranges,
        &[
            ts_range(20..30),
            ts_range(50..54),
            ts_range(59..67),
            ts_range(80..90),
        ]
    );
    assert_eq!(change, 1..3);

    let (new_ranges, change) = splice_included_ranges(ranges.clone(), &[70..71, 91..100], &[]);
    assert_eq!(
        new_ranges,
        &[ts_range(20..30), ts_range(50..60), ts_range(80..90)]
    );
    assert_eq!(change, 2..3);

    let (new_ranges, change) =
        splice_included_ranges(ranges.clone(), &[], &[ts_range(0..2), ts_range(70..75)]);
    assert_eq!(
        new_ranges,
        &[
            ts_range(0..2),
            ts_range(20..30),
            ts_range(50..60),
            ts_range(70..75),
            ts_range(80..90)
        ]
    );
    assert_eq!(change, 0..4);

    let (new_ranges, change) =
        splice_included_ranges(ranges.clone(), &[30..50], &[ts_range(25..55)]);
    assert_eq!(new_ranges, &[ts_range(25..55), ts_range(80..90)]);
    assert_eq!(change, 0..1);

    // does not create overlapping ranges
    let (new_ranges, change) = splice_included_ranges(ranges, &[0..18], &[ts_range(20..32)]);
    assert_eq!(
        new_ranges,
        &[ts_range(20..32), ts_range(50..60), ts_range(80..90)]
    );
    assert_eq!(change, 0..1);

    fn ts_range(range: Range<usize>) -> tree_sitter::Range {
        tree_sitter::Range {
            start_byte: range.start,
            start_point: tree_sitter::Point {
                row: 0,
                column: range.start,
            },
            end_byte: range.end,
            end_point: tree_sitter::Point {
                row: 0,
                column: range.end,
            },
        }
    }
}

#[gpui::test]
fn test_syntax_map_layers_for_range(cx: &mut App) {
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let language = Arc::new(rust_lang());
    registry.add(language.clone());

    let mut buffer = Buffer::new(
        0,
        BufferId::new(1).unwrap(),
        r#"
            fn a() {
                assert_eq!(
                    b(vec![C {}]),
                    vec![d.e],
                );
                println!("{}", f(|_| true));
            }
        "#
        .unindent(),
    );

    let mut syntax_map = SyntaxMap::new(&buffer);
    syntax_map.set_language_registry(registry);
    syntax_map.reparse(language.clone(), &buffer);

    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(2, 0)..Point::new(2, 0),
        &[
            "...(function_item ... (block (expression_statement (macro_invocation...",
            "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
        ],
    );
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(2, 14)..Point::new(2, 16),
        &[
            "...(function_item ...",
            "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
            "...(array_expression (struct_expression ...",
        ],
    );
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(3, 14)..Point::new(3, 16),
        &[
            "...(function_item ...",
            "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
            "...(array_expression (field_expression ...",
        ],
    );
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(5, 12)..Point::new(5, 16),
        &[
            "...(function_item ...",
            "...(call_expression ... (arguments (closure_expression ...",
        ],
    );

    // Replace a vec! macro invocation with a plain slice, removing a syntactic layer.
    let macro_name_range = range_for_text(&buffer, "vec!");
    buffer.edit([(macro_name_range, "&")]);
    syntax_map.interpolate(&buffer);
    syntax_map.reparse(language.clone(), &buffer);

    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(2, 14)..Point::new(2, 16),
        &[
            "...(function_item ...",
            "...(tuple_expression (call_expression ... arguments: (arguments (reference_expression value: (array_expression...",
        ],
    );

    // Put the vec! macro back, adding back the syntactic layer.
    buffer.undo();
    syntax_map.interpolate(&buffer);
    syntax_map.reparse(language, &buffer);

    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(2, 14)..Point::new(2, 16),
        &[
            "...(function_item ...",
            "...(tuple_expression (call_expression ... arguments: (arguments (macro_invocation...",
            "...(array_expression (struct_expression ...",
        ],
    );
}

#[gpui::test]
fn test_dynamic_language_injection(cx: &mut App) {
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let markdown = Arc::new(markdown_lang());
    let markdown_inline = Arc::new(markdown_inline_lang());
    registry.add(markdown.clone());
    registry.add(markdown_inline.clone());
    registry.add(Arc::new(rust_lang()));
    registry.add(Arc::new(ruby_lang()));

    let mut buffer = Buffer::new(
        0,
        BufferId::new(1).unwrap(),
        r#"
            This is a code block:

            ```rs
            fn foo() {}
            ```
        "#
        .unindent(),
    );

    let mut syntax_map = SyntaxMap::new(&buffer);
    syntax_map.set_language_registry(registry.clone());
    syntax_map.reparse(markdown.clone(), &buffer);
    syntax_map.reparse(markdown_inline.clone(), &buffer);
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(3, 0)..Point::new(3, 0),
        &[
            "(document (section (paragraph (inline)) (fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (block_continuation) (code_fence_content (block_continuation)) (fenced_code_block_delimiter))))",
            "(inline (code_span (code_span_delimiter) (code_span_delimiter)))",
            "...(function_item name: (identifier) parameters: (parameters) body: (block)...",
        ],
    );

    // Replace `rs` with a path to ending in `.rb` in code block.
    let macro_name_range = range_for_text(&buffer, "rs");
    buffer.edit([(macro_name_range, "foo/bar/baz.rb")]);
    syntax_map.interpolate(&buffer);
    syntax_map.reparse(markdown.clone(), &buffer);
    syntax_map.reparse(markdown_inline.clone(), &buffer);
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(3, 0)..Point::new(3, 0),
        &[
            "(document (section (paragraph (inline)) (fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (block_continuation) (code_fence_content (block_continuation)) (fenced_code_block_delimiter))))",
            "(inline (code_span (code_span_delimiter) (code_span_delimiter)))",
            "...(call method: (identifier) arguments: (argument_list (call method: (identifier) arguments: (argument_list) block: (block)...",
        ],
    );

    // Replace Ruby with a language that hasn't been loaded yet.
    let macro_name_range = range_for_text(&buffer, "foo/bar/baz.rb");
    buffer.edit([(macro_name_range, "html")]);
    syntax_map.interpolate(&buffer);
    syntax_map.reparse(markdown.clone(), &buffer);
    syntax_map.reparse(markdown_inline.clone(), &buffer);
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(3, 0)..Point::new(3, 0),
        &[
            "(document (section (paragraph (inline)) (fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (block_continuation) (code_fence_content (block_continuation)) (fenced_code_block_delimiter))))",
            "(inline (code_span (code_span_delimiter) (code_span_delimiter)))",
        ],
    );
    assert!(syntax_map.contains_unknown_injections());

    registry.add(Arc::new(html_lang()));
    syntax_map.reparse(markdown, &buffer);
    syntax_map.reparse(markdown_inline, &buffer);
    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(3, 0)..Point::new(3, 0),
        &[
            "(document (section (paragraph (inline)) (fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (block_continuation) (code_fence_content (block_continuation)) (fenced_code_block_delimiter))))",
            "(inline (code_span (code_span_delimiter) (code_span_delimiter)))",
            "(document (text))",
        ],
    );
    assert!(!syntax_map.contains_unknown_injections());
}

#[gpui::test]
fn test_typing_multiple_new_injections(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "Rust",
        &[
            "fn a() { test_macro }",
            "fn a() { test_macro«!» }",
            "fn a() { test_macro!«()» }",
            "fn a() { test_macro!(«b») }",
            "fn a() { test_macro!(b«.») }",
            "fn a() { test_macro!(b.«c») }",
            "fn a() { test_macro!(b.c«()») }",
            "fn a() { test_macro!(b.c(«vec»)) }",
            "fn a() { test_macro!(b.c(vec«!»)) }",
            "fn a() { test_macro!(b.c(vec!«[]»)) }",
            "fn a() { test_macro!(b.c(vec![«d»])) }",
            "fn a() { test_macro!(b.c(vec![d«.»])) }",
            "fn a() { test_macro!(b.c(vec![d.«e»])) }",
        ],
        cx,
    );

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["field"],
        "fn a() { test_macro!(b.«c»(vec![d.«e»])) }",
    );
}

#[gpui::test]
fn test_pasting_new_injection_line_between_others(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "Rust",
        &[
            "
                fn a() {
                    b!(B {});
                    c!(C {});
                    d!(D {});
                    e!(E {});
                    f!(F {});
                    g!(G {});
                }
            ",
            "
                fn a() {
                    b!(B {});
                    c!(C {});
                    d!(D {});
                «    h!(H {});
                »    e!(E {});
                    f!(F {});
                    g!(G {});
                }
            ",
        ],
        cx,
    );

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["struct"],
        "
        fn a() {
            b!(«B {}»);
            c!(«C {}»);
            d!(«D {}»);
            h!(«H {}»);
            e!(«E {}»);
            f!(«F {}»);
            g!(«G {}»);
        }
        ",
    );
}

#[gpui::test]
fn test_joining_injections_with_child_injections(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "Rust",
        &[
            "
                fn a() {
                    b!(
                        c![one.two.three],
                        d![four.five.six],
                    );
                    e!(
                        f![seven.eight],
                    );
                }
            ",
            "
                fn a() {
                    b!(
                        c![one.two.three],
                        d![four.five.six],
                    ˇ    f![seven.eight],
                    );
                }
            ",
        ],
        cx,
    );

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["field"],
        "
        fn a() {
            b!(
                c![one.«two».«three»],
                d![four.«five».«six»],
                f![seven.«eight»],
            );
        }
        ",
    );
}

#[gpui::test]
fn test_editing_edges_of_injection(cx: &mut App) {
    test_edit_sequence(
        "Rust",
        &[
            "
                fn a() {
                    b!(c!())
                }
            ",
            "
                fn a() {
                    «d»!(c!())
                }
            ",
            "
                fn a() {
                    «e»d!(c!())
                }
            ",
            "
                fn a() {
                    ed!«[»c!()«]»
                }
            ",
        ],
        cx,
    );
}

#[gpui::test]
fn test_edits_preceding_and_intersecting_injection(cx: &mut App) {
    test_edit_sequence(
        "Rust",
        &[
            //
            "const aaaaaaaaaaaa: B = c!(d(e.f));",
            "const aˇa: B = c!(d(eˇ));",
        ],
        cx,
    );
}

#[gpui::test]
fn test_non_local_changes_create_injections(cx: &mut App) {
    test_edit_sequence(
        "Rust",
        &[
            "
                // a! {
                    static B: C = d;
                // }
            ",
            "
                ˇa! {
                    static B: C = d;
                ˇ}
            ",
        ],
        cx,
    );
}

#[gpui::test]
fn test_creating_many_injections_in_one_edit(cx: &mut App) {
    test_edit_sequence(
        "Rust",
        &[
            "
                fn a() {
                    one(Two::three(3));
                    four(Five::six(6));
                    seven(Eight::nine(9));
                }
            ",
            "
                fn a() {
                    one«!»(Two::three(3));
                    four«!»(Five::six(6));
                    seven«!»(Eight::nine(9));
                }
            ",
            "
                fn a() {
                    one!(Two::three«!»(3));
                    four!(Five::six«!»(6));
                    seven!(Eight::nine«!»(9));
                }
            ",
        ],
        cx,
    );
}

#[gpui::test]
fn test_editing_across_injection_boundary(cx: &mut App) {
    test_edit_sequence(
        "Rust",
        &[
            "
                fn one() {
                    two();
                    three!(
                        three.four,
                        five.six,
                    );
                }
            ",
            "
                fn one() {
                    two();
                    th«irty_five![»
                        three.four,
                        five.six,
                    «   seven.eight,
                    ];»
                }
            ",
        ],
        cx,
    );
}

#[gpui::test]
fn test_removing_injection_by_replacing_across_boundary(cx: &mut App) {
    test_edit_sequence(
        "Rust",
        &[
            "
                fn one() {
                    two!(
                        three.four,
                    );
                }
            ",
            "
                fn one() {
                    t«en
                        .eleven(
                        twelve,
                    »
                        three.four,
                    );
                }
            ",
        ],
        cx,
    );
}

#[gpui::test]
fn test_combined_injections_simple(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "ERB",
        &[
            "
                <body>
                    <% if @one %>
                        <div class=one>
                    <% else %>
                        <div class=two>
                    <% end %>
                    </div>
                </body>
            ",
            "
                <body>
                    <% if @one %>
                        <div class=one>
                    ˇ else ˇ
                        <div class=two>
                    <% end %>
                    </div>
                </body>
            ",
            "
                <body>
                    <% if @one «;» end %>
                    </div>
                </body>
            ",
        ],
        cx,
    );

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["tag", "ivar"],
        "
            <«body»>
                <% if «@one» ; end %>
                </«div»>
            </«body»>
        ",
    );
}

#[gpui::test]
fn test_combined_injections_empty_ranges(cx: &mut App) {
    test_edit_sequence(
        "ERB",
        &[
            "
                <% if @one %>
                <% else %>
                <% end %>
            ",
            "
                <% if @one %>
                ˇ<% end %>
            ",
        ],
        cx,
    );
}

#[gpui::test]
fn test_combined_injections_edit_edges_of_ranges(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "ERB",
        &[
            "
                <%= one @two %>
                <%= three @four %>
            ",
            "
                <%= one @two %ˇ
                <%= three @four %>
            ",
            "
                <%= one @two %«>»
                <%= three @four %>
            ",
        ],
        cx,
    );

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["tag", "ivar"],
        "
            <%= one «@two» %>
            <%= three «@four» %>
        ",
    );
}

#[gpui::test]
fn test_combined_injections_splitting_some_injections(cx: &mut App) {
    let (_buffer, _syntax_map) = test_edit_sequence(
        "ERB",
        &[
            r#"
                <%A if b(:c) %>
                d
                <% end %>
                eee
                <% f %>
            "#,
            r#"
                <%« AAAAAAA %>
                hhhhhhh
                <%=» if b(:c) %>
                d
                <% end %>
                eee
                <% f %>
            "#,
        ],
        cx,
    );
}

#[gpui::test]
fn test_combined_injections_editing_after_last_injection(cx: &mut App) {
    test_edit_sequence(
        "ERB",
        &[
            r#"
                <% foo %>
                <div></div>
                <% bar %>
            "#,
            r#"
                <% foo %>
                <div></div>
                <% bar %>«
                more text»
            "#,
        ],
        cx,
    );
}

#[gpui::test]
fn test_combined_injections_inside_injections(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "Markdown",
        &[
            r#"
                here is
                some
                ERB code:

                ```erb
                <ul>
                <% people.each do |person| %>
                    <li><%= person.name %></li>
                    <li><%= person.age %></li>
                <% end %>
                </ul>
                ```
            "#,
            r#"
                here is
                some
                ERB code:

                ```erb
                <ul>
                <% people«2».each do |person| %>
                    <li><%= person.name %></li>
                    <li><%= person.age %></li>
                <% end %>
                </ul>
                ```
            "#,
            // Inserting a comment character inside one code directive
            // does not cause the other code directive to become a comment,
            // because newlines are included in between each injection range.
            r#"
                here is
                some
                ERB code:

                ```erb
                <ul>
                <% people2.each do |person| %>
                    <li><%= «# »person.name %></li>
                    <li><%= person.age %></li>
                <% end %>
                </ul>
                ```
            "#,
        ],
        cx,
    );

    // Check that the code directive below the ruby comment is
    // not parsed as a comment.
    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["method"],
        "
            here is
            some
            ERB code:

            ```erb
            <ul>
            <% people2.«each» do |person| %>
                <li><%= # person.name %></li>
                <li><%= person.«age» %></li>
            <% end %>
            </ul>
            ```
        ",
    );
}

#[gpui::test]
fn test_empty_combined_injections_inside_injections(cx: &mut App) {
    let (buffer, syntax_map) = test_edit_sequence(
        "Markdown",
        &[r#"
            ```erb
            hello
            ```

            goodbye
        "#],
        cx,
    );

    assert_layers_for_range(
        &syntax_map,
        &buffer,
        Point::new(0, 0)..Point::new(5, 0),
        &[
            // Markdown document
            "(document (section (fenced_code_block (fenced_code_block_delimiter) (info_string (language)) (block_continuation) (code_fence_content (block_continuation)) (fenced_code_block_delimiter)) (paragraph (inline))))",
            // ERB template in the code block
            "(template...",
            // Markdown inline content
            "(inline)",
            // The ruby syntax tree should be empty, since there are
            // no interpolations in the ERB template.
            "(program)",
            // HTML within the ERB
            "(document (text))",
        ],
    );
}

#[gpui::test]
fn test_syntax_map_languages_loading_with_erb(cx: &mut App) {
    let text = r#"
        <body>
            <% if @one %>
                <div class=one>
            <% else %>
                <div class=two>
            <% end %>
            </div>
        </body>
    "#
    .unindent();

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), text);

    let mut syntax_map = SyntaxMap::new(&buffer);
    syntax_map.set_language_registry(registry.clone());

    let language = Arc::new(erb_lang());

    log::info!("parsing");
    registry.add(language.clone());
    syntax_map.reparse(language.clone(), &buffer);

    log::info!("loading html");
    registry.add(Arc::new(html_lang()));
    syntax_map.reparse(language.clone(), &buffer);

    log::info!("loading ruby");
    registry.add(Arc::new(ruby_lang()));
    syntax_map.reparse(language.clone(), &buffer);

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["tag", "ivar"],
        "
            <«body»>
                <% if «@one» %>
                    <«div» class=one>
                <% else %>
                    <«div» class=two>
                <% end %>
                </«div»>
            </«body»>
        ",
    );

    let text = r#"
        <body>
            <% if @one«_hundred» %>
                <div class=one>
            <% else %>
                <div class=two>
            <% end %>
            </div>
        </body>
    "#
    .unindent();

    log::info!("editing");
    buffer.edit_via_marked_text(&text);
    syntax_map.interpolate(&buffer);
    syntax_map.reparse(language, &buffer);

    assert_capture_ranges(
        &syntax_map,
        &buffer,
        &["tag", "ivar"],
        "
            <«body»>
                <% if «@one_hundred» %>
                    <«div» class=one>
                <% else %>
                    <«div» class=two>
                <% end %>
                </«div»>
            </«body»>
        ",
    );
}

#[gpui::test(iterations = 50)]
fn test_random_syntax_map_edits_rust_macros(rng: StdRng, cx: &mut App) {
    let text = r#"
        fn test_something() {
            let vec = vec![5, 1, 3, 8];
            assert_eq!(
                vec
                    .into_iter()
                    .map(|i| i * 2)
                    .collect::<Vec<usize>>(),
                vec![
                    5 * 2, 1 * 2, 3 * 2, 8 * 2
                ],
            );
        }
    "#
    .unindent()
    .repeat(2);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let language = Arc::new(rust_lang());
    registry.add(language.clone());

    test_random_edits(text, registry, language, rng);
}

#[gpui::test(iterations = 50)]
fn test_random_syntax_map_edits_with_erb(rng: StdRng, cx: &mut App) {
    let text = r#"
        <div id="main">
        <% if one?(:two) %>
            <p class="three" four>
            <%= yield :five %>
            </p>
        <% elsif Six.seven(8) %>
            <p id="three" four>
            <%= yield :five %>
            </p>
        <% else %>
            <span>Ok</span>
        <% end %>
        </div>
    "#
    .unindent()
    .repeat(5);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let language = Arc::new(erb_lang());
    registry.add(language.clone());
    registry.add(Arc::new(ruby_lang()));
    registry.add(Arc::new(html_lang()));

    test_random_edits(text, registry, language, rng);
}

#[gpui::test(iterations = 50)]
fn test_random_syntax_map_edits_with_heex(rng: StdRng, cx: &mut App) {
    let text = r#"
        defmodule TheModule do
            def the_method(assigns) do
                ~H"""
                <%= if @empty do %>
                    <div class="h-4"></div>
                <% else %>
                    <div class="max-w-2xl w-full animate-pulse">
                    <div class="flex-1 space-y-4">
                        <div class={[@bg_class, "h-4 rounded-lg w-3/4"]}></div>
                        <div class={[@bg_class, "h-4 rounded-lg"]}></div>
                        <div class={[@bg_class, "h-4 rounded-lg w-5/6"]}></div>
                    </div>
                    </div>
                <% end %>
                """
            end
        end
    "#
    .unindent()
    .repeat(3);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let language = Arc::new(elixir_lang());
    registry.add(language.clone());
    registry.add(Arc::new(heex_lang()));
    registry.add(Arc::new(html_lang()));

    test_random_edits(text, registry, language, rng);
}

fn test_random_edits(
    text: String,
    registry: Arc<LanguageRegistry>,
    language: Arc<Language>,
    mut rng: StdRng,
) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), text);

    let mut syntax_map = SyntaxMap::new(&buffer);
    syntax_map.set_language_registry(registry.clone());
    syntax_map.reparse(language.clone(), &buffer);

    let mut reference_syntax_map = SyntaxMap::new(&buffer);
    reference_syntax_map.set_language_registry(registry);

    log::info!("initial text:\n{}", buffer.text());

    for _ in 0..operations {
        let prev_buffer = buffer.snapshot();
        let prev_syntax_map = syntax_map.snapshot();

        buffer.randomly_edit(&mut rng, 3);
        log::info!("text:\n{}", buffer.text());

        syntax_map.interpolate(&buffer);
        check_interpolation(&prev_syntax_map, &syntax_map, &prev_buffer, &buffer);

        syntax_map.reparse(language.clone(), &buffer);

        reference_syntax_map.clear(&buffer);
        reference_syntax_map.reparse(language.clone(), &buffer);
    }

    for i in 0..operations {
        let i = operations - i - 1;
        buffer.undo();
        log::info!("undoing operation {}", i);
        log::info!("text:\n{}", buffer.text());

        syntax_map.interpolate(&buffer);
        syntax_map.reparse(language.clone(), &buffer);

        reference_syntax_map.clear(&buffer);
        reference_syntax_map.reparse(language.clone(), &buffer);
        assert_eq!(
            syntax_map.layers(&buffer).len(),
            reference_syntax_map.layers(&buffer).len(),
            "wrong number of layers after undoing edit {i}"
        );
    }

    let layers = syntax_map.layers(&buffer);
    let reference_layers = reference_syntax_map.layers(&buffer);
    for (edited_layer, reference_layer) in layers.into_iter().zip(reference_layers.into_iter()) {
        assert_eq!(
            edited_layer.node().to_sexp(),
            reference_layer.node().to_sexp()
        );
        assert_eq!(edited_layer.node().range(), reference_layer.node().range());
    }
}

fn check_interpolation(
    old_syntax_map: &SyntaxSnapshot,
    new_syntax_map: &SyntaxSnapshot,
    old_buffer: &BufferSnapshot,
    new_buffer: &BufferSnapshot,
) {
    let edits = new_buffer
        .edits_since::<usize>(old_buffer.version())
        .collect::<Vec<_>>();

    for (old_layer, new_layer) in old_syntax_map
        .layers
        .iter()
        .zip(new_syntax_map.layers.iter())
    {
        assert_eq!(old_layer.range, new_layer.range);
        let Some(old_tree) = old_layer.content.tree() else {
            continue;
        };
        let Some(new_tree) = new_layer.content.tree() else {
            continue;
        };
        let old_start_byte = old_layer.range.start.to_offset(old_buffer);
        let new_start_byte = new_layer.range.start.to_offset(new_buffer);
        let old_start_point = old_layer.range.start.to_point(old_buffer).to_ts_point();
        let new_start_point = new_layer.range.start.to_point(new_buffer).to_ts_point();
        let old_node = old_tree.root_node_with_offset(old_start_byte, old_start_point);
        let new_node = new_tree.root_node_with_offset(new_start_byte, new_start_point);
        check_node_edits(
            old_layer.depth,
            &old_layer.range,
            old_node,
            new_node,
            old_buffer,
            new_buffer,
            &edits,
        );
    }

    fn check_node_edits(
        depth: usize,
        range: &Range<Anchor>,
        old_node: Node,
        new_node: Node,
        old_buffer: &BufferSnapshot,
        new_buffer: &BufferSnapshot,
        edits: &[text::Edit<usize>],
    ) {
        assert_eq!(old_node.kind(), new_node.kind());

        let old_range = old_node.byte_range();
        let new_range = new_node.byte_range();

        let is_edited = edits
            .iter()
            .any(|edit| edit.new.start < new_range.end && edit.new.end > new_range.start);
        if is_edited {
            assert!(
                new_node.has_changes(),
                concat!(
                    "failed to mark node as edited.\n",
                    "layer depth: {}, old layer range: {:?}, new layer range: {:?},\n",
                    "node kind: {}, old node range: {:?}, new node range: {:?}",
                ),
                depth,
                range.to_offset(old_buffer),
                range.to_offset(new_buffer),
                new_node.kind(),
                old_range,
                new_range,
            );
        }

        if !new_node.has_changes() {
            assert_eq!(
                old_buffer
                    .text_for_range(old_range.clone())
                    .collect::<String>(),
                new_buffer
                    .text_for_range(new_range.clone())
                    .collect::<String>(),
                concat!(
                    "mismatched text for node\n",
                    "layer depth: {}, old layer range: {:?}, new layer range: {:?},\n",
                    "node kind: {}, old node range:{:?}, new node range:{:?}",
                ),
                depth,
                range.to_offset(old_buffer),
                range.to_offset(new_buffer),
                new_node.kind(),
                old_range,
                new_range,
            );
        }

        for i in 0..new_node.child_count() {
            check_node_edits(
                depth,
                range,
                old_node.child(i).unwrap(),
                new_node.child(i).unwrap(),
                old_buffer,
                new_buffer,
                edits,
            )
        }
    }
}

fn test_edit_sequence(language_name: &str, steps: &[&str], cx: &mut App) -> (Buffer, SyntaxMap) {
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    registry.add(Arc::new(elixir_lang()));
    registry.add(Arc::new(heex_lang()));
    registry.add(Arc::new(rust_lang()));
    registry.add(Arc::new(ruby_lang()));
    registry.add(Arc::new(html_lang()));
    registry.add(Arc::new(erb_lang()));
    registry.add(Arc::new(markdown_lang()));
    registry.add(Arc::new(markdown_inline_lang()));

    let language = registry
        .language_for_name(language_name)
        .now_or_never()
        .unwrap()
        .unwrap();
    let mut buffer = Buffer::new(0, BufferId::new(1).unwrap(), "");

    let mut mutated_syntax_map = SyntaxMap::new(&buffer);
    mutated_syntax_map.set_language_registry(registry.clone());
    mutated_syntax_map.reparse(language.clone(), &buffer);

    for (i, marked_string) in steps.iter().enumerate() {
        let marked_string = marked_string.unindent();
        log::info!("incremental parse {i}: {marked_string:?}");
        buffer.edit_via_marked_text(&marked_string);

        // Reparse the syntax map
        mutated_syntax_map.interpolate(&buffer);
        mutated_syntax_map.reparse(language.clone(), &buffer);

        // Create a second syntax map from scratch
        log::info!("fresh parse {i}: {marked_string:?}");
        let mut reference_syntax_map = SyntaxMap::new(&buffer);
        reference_syntax_map.set_language_registry(registry.clone());
        reference_syntax_map.reparse(language.clone(), &buffer);

        // Compare the mutated syntax map to the new syntax map
        let mutated_layers = mutated_syntax_map.layers(&buffer);
        let reference_layers = reference_syntax_map.layers(&buffer);
        assert_eq!(
            mutated_layers.len(),
            reference_layers.len(),
            "wrong number of layers at step {i}"
        );
        for (edited_layer, reference_layer) in
            mutated_layers.into_iter().zip(reference_layers.into_iter())
        {
            assert_eq!(
                edited_layer.node().to_sexp(),
                reference_layer.node().to_sexp(),
                "different layer at step {i}"
            );
            assert_eq!(
                edited_layer.node().range(),
                reference_layer.node().range(),
                "different layer at step {i}"
            );
        }
    }

    (buffer, mutated_syntax_map)
}

fn html_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "HTML".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["html".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_html::LANGUAGE.into()),
    )
    .with_highlights_query(
        r#"
            (tag_name) @tag
            (erroneous_end_tag_name) @tag
            (attribute_name) @property
        "#,
    )
    .unwrap()
}

fn ruby_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Ruby".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rb".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_ruby::LANGUAGE.into()),
    )
    .with_highlights_query(
        r#"
            ["if" "do" "else" "end"] @keyword
            (instance_variable) @ivar
            (call method: (identifier) @method)
        "#,
    )
    .unwrap()
}

fn erb_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "ERB".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["erb".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_embedded_template::LANGUAGE.into()),
    )
    .with_highlights_query(
        r#"
            ["<%" "%>"] @keyword
        "#,
    )
    .unwrap()
    .with_injection_query(
        r#"
            (
                (code) @injection.content
                (#set! injection.language "ruby")
                (#set! injection.combined)
            )

            (
                (content) @injection.content
                (#set! injection.language "html")
                (#set! injection.combined)
            )
        "#,
    )
    .unwrap()
}

fn rust_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )
    .with_highlights_query(
        r#"
            (field_identifier) @field
            (struct_expression) @struct
        "#,
    )
    .unwrap()
    .with_injection_query(
        r#"
            (macro_invocation
                (token_tree) @injection.content
                (#set! injection.language "rust"))
        "#,
    )
    .unwrap()
}

fn elixir_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Elixir".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ex".into()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_elixir::LANGUAGE.into()),
    )
    .with_highlights_query(
        r#"

        "#,
    )
    .unwrap()
}

fn heex_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "HEEx".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["heex".into()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_heex::LANGUAGE.into()),
    )
    .with_injection_query(
        r#"
        (
          (directive
            [
              (partial_expression_value)
              (expression_value)
              (ending_expression_value)
            ] @injection.content)
          (#set! injection.language "elixir")
          (#set! injection.combined)
        )

        ((expression (expression_value) @injection.content)
         (#set! injection.language "elixir"))
        "#,
    )
    .unwrap()
}

fn range_for_text(buffer: &Buffer, text: &str) -> Range<usize> {
    let start = buffer.as_rope().to_string().find(text).unwrap();
    start..start + text.len()
}

#[track_caller]
fn assert_layers_for_range(
    syntax_map: &SyntaxMap,
    buffer: &BufferSnapshot,
    range: Range<Point>,
    expected_layers: &[&str],
) {
    let layers = syntax_map
        .layers_for_range(range, buffer, true)
        .collect::<Vec<_>>();
    assert_eq!(
        layers.len(),
        expected_layers.len(),
        "wrong number of layers"
    );
    for (i, (layer, expected_s_exp)) in layers.iter().zip(expected_layers.iter()).enumerate() {
        let actual_s_exp = layer.node().to_sexp();
        assert!(
            string_contains_sequence(
                &actual_s_exp,
                &expected_s_exp.split("...").collect::<Vec<_>>()
            ),
            "layer {i}:\n\nexpected: {expected_s_exp}\nactual:   {actual_s_exp}",
        );
    }
}

#[track_caller]
fn assert_capture_ranges(
    syntax_map: &SyntaxMap,
    buffer: &BufferSnapshot,
    highlight_query_capture_names: &[&str],
    marked_string: &str,
) {
    let mut actual_ranges = Vec::<Range<usize>>::new();
    let captures = syntax_map.captures(0..buffer.len(), buffer, |grammar| {
        grammar.highlights_query.as_ref()
    });
    let queries = captures
        .grammars()
        .iter()
        .map(|grammar| grammar.highlights_query.as_ref().unwrap())
        .collect::<Vec<_>>();
    for capture in captures {
        let name = &queries[capture.grammar_index].capture_names()[capture.index as usize];
        if highlight_query_capture_names.contains(name) {
            actual_ranges.push(capture.node.byte_range());
        }
    }

    let (text, expected_ranges) = marked_text_ranges(&marked_string.unindent(), false);
    assert_eq!(text, buffer.text());
    assert_eq!(actual_ranges, expected_ranges);
}

pub fn string_contains_sequence(text: &str, parts: &[&str]) -> bool {
    let mut last_part_end = 0;
    for part in parts {
        if let Some(start_ix) = text[last_part_end..].find(part) {
            last_part_end = start_ix + part.len();
        } else {
            return false;
        }
    }
    true
}
