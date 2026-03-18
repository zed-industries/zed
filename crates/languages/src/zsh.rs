use project::ContextProviderWithTasks;
use task::{TaskTemplate, TaskTemplates, VariableName};

pub(super) fn zsh_task_context() -> ContextProviderWithTasks {
    ContextProviderWithTasks::new(TaskTemplates(vec![
        TaskTemplate {
            label: "execute selection".to_owned(),
            command: VariableName::SelectedText.template_value(),
            ..TaskTemplate::default()
        },
        TaskTemplate {
            label: format!("run '{}'", VariableName::File.template_value()),
            command: VariableName::File.template_value(),
            tags: vec!["zsh-script".to_owned()],
            ..TaskTemplate::default()
        },
    ]))
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, Context, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;
    use unindent::Unindent;
    use util::test::marked_text_offsets;

    #[gpui::test]
    async fn test_zsh_autoindent(cx: &mut TestAppContext) {
        cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        let language = crate::language("zsh", tree_sitter_zsh::LANGUAGE.into());
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2)
                });
            });
        });

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            let expect_indents_to =
                |buffer: &mut Buffer, cx: &mut Context<Buffer>, input: &str, expected: &str| {
                    buffer.edit(
                        [(0..buffer.len(), input)],
                        Some(AutoindentMode::EachLine),
                        cx,
                    );
                    assert_eq!(buffer.text(), expected);
                };

            expect_indents_to(
                &mut buffer,
                cx,
                "#!/usr/bin/env zsh\n#",
                "#!/usr/bin/env zsh\n#",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "function name() {\necho \"Hello, World!\"\n}",
                "function name() {\n  echo \"Hello, World!\"\n}",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "if true;then\nfoo\nelse\nbar\nfi",
                "if true;then\n  foo\nelse\n  bar\nfi",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "if true;then\nfoo\nelif true;then\nbar\nelse\nbar\nfi",
                "if true;then\n  foo\nelif true;then\n  bar\nelse\n  bar\nfi",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "case $1 in\nfoo) echo \"Hello, World!\";;\n*) echo \"Unknown argument\";;\nesac",
                "case $1 in\n  foo) echo \"Hello, World!\";;\n  *) echo \"Unknown argument\";;\nesac",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "for i in {1..10};do\nfoo\ndone",
                "for i in {1..10};do\n  foo\ndone",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "while true; do\nfoo\ndone",
                "while true; do\n  foo\ndone",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "array=(\n1\n2\n3\n)",
                "array=(\n  1\n  2\n  3\n)",
            );

            expect_indents_to(
                &mut buffer,
                cx,
                "foo() {\necho \"Hello, World!\"\n}",
                "foo() {\n  echo \"Hello, World!\"\n}",
            );

            let (input, offsets) = marked_text_offsets(
                &r#"
                if foo; then
                  1ˇ
                else
                  3
                fi
                "#
                .unindent(),
            );

            buffer.edit([(0..buffer.len(), input)], None, cx);
            buffer.edit(
                [(offsets[0]..offsets[0], "\n")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            buffer.edit(
                [(offsets[0] + 3..offsets[0] + 3, "elif")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            let expected = r#"
                if foo; then
                  1
                elif
                else
                  3
                fi
                "#
            .unindent();

            pretty_assertions::assert_eq!(buffer.text(), expected);

            buffer
        });
    }
}
