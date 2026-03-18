#[cfg(test)]
mod test {
    use gpui::AppContext;
    use gpui::TestAppContext;
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::language("diff", tree_sitter_diff::LANGUAGE.into());
        // Diff variants supported by
        // https://github.com/tree-sitter-grammars/tree-sitter-diff/blob/main/test/corpus/text.txt
        let text = r#"
            diff --git a/grammar.js b/grammar.js
            index dc36969..f37fde0 100644
            --- a/grammar.js
            +++ b/grammar.js
            @@ -6,6 +6,8 @@ module.exports = grammar({
               extras: ($) => [],

               rules: {
            -    source: ($) => "hello",
            +    source: ($) => repeat($._line),
            +
            +    _line: ($) => choice(),
               },
             });
            diff --git a/tmp.txt b/tmp.txt
            new file mode 100644
            index 0000000..e69de29
            diff --git a/tmp.txt b/tmp.txt
            deleted file mode 100644
            index e69de29..0000000
            diff --git a/tmp.txt b/tmp.md
            similarity index 100%
            rename from tmp.txt
            rename to tmp.md
            diff --git a/tmp.txt b/tmp.txt
            new file mode 100644
            index 00000000..ee9808dc
            --- /dev/null
            +++ b/tmp.txt
            @@ -0,0 +1 @@
            +aaa
            \ No newline at end of file
            --- /dev/null
            diff --git a/runtime/queries/elixir/highlights.scm b/runtime/queries/elixir/highlights.scm
            index 76fd2af..308ff34 100644
            --- a/runtime/queries/elixir/highlights.scm
            +++ b/runtime/queries/elixir/highlights.scm
            @@ -125,7 +125,8 @@
             (sigil
               (sigil_name) @__name__
               quoted_start: _ @string.special
            -  quoted_end: _ @string.special) @string.special
            +  quoted_end: _ @string.special
            +  (#not-eq? @__name__ "H")) @string.special

             ; Calls

            diff --git a/runtime/queries/elixir/injections.scm b/runtime/queries/elixir/injections.scm
            index 321c90a..b4a5cba 100644
            --- a/runtime/queries/elixir/injections.scm
            +++ b/runtime/queries/elixir/injections.scm
            @@ -1,2 +1,8 @@
             ((comment) @injection.content
              (#set! injection.language "comment"))
            +
            +((sigil
            +  (sigil_name) @_sigil_name
            +  (quoted_content) @injection.content)
            + (#eq? @_sigil_name "H")
            + (#set! injection.language "heex"))
            diff --git a/LICENSE b/LICENSE
            old mode 100644
            new mode 100755
        "#.unindent();

        let buffer = cx.new(|cx| language::Buffer::local(text, cx).with_language(language, cx));
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None));
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("diff --git a/grammar.js b/grammar.js", 0),
                ("a/grammar.js", 1),
                ("b/grammar.js", 1),
                ("diff --git a/tmp.txt b/tmp.txt", 0),
                ("new file mode 100644", 1),
                ("diff --git a/tmp.txt b/tmp.txt", 0),
                ("deleted file mode 100644", 1),
                ("diff --git a/tmp.txt b/tmp.md", 0),
                ("rename from tmp.txt", 1),
                ("rename to tmp.md", 1),
                ("diff --git a/tmp.txt b/tmp.txt", 0),
                ("new file mode 100644", 1),
                ("/dev/null", 1),
                ("b/tmp.txt", 1),
                (
                    "diff --git a/runtime/queries/elixir/highlights.scm b/runtime/queries/elixir/highlights.scm",
                    0
                ),
                ("a/runtime/queries/elixir/highlights.scm", 1),
                ("b/runtime/queries/elixir/highlights.scm", 1),
                (
                    "diff --git a/runtime/queries/elixir/injections.scm b/runtime/queries/elixir/injections.scm",
                    0
                ),
                ("a/runtime/queries/elixir/injections.scm", 1),
                ("b/runtime/queries/elixir/injections.scm", 1),
                ("diff --git a/LICENSE b/LICENSE", 0),
                ("old mode 100644", 1),
                ("new mode 100755", 1)
            ]
        );
    }
}
