use gpui::{App, Task};
pub use language::*;
use language::{
    Buffer, ContextProvider, LanguageName, LanguageToolchainStore,
    language_settings::LanguageSettings,
};

use settings::SemanticTokenRules;
use std::{borrow::Cow, sync::Arc};
use task::{TaskTemplate, TaskTemplates, TaskVariables, VariableName};

pub(crate) fn semantic_token_rules() -> SemanticTokenRules {
    let content = grammars::get_file("cpp/semantic_token_rules.json")
        .expect("missing cpp/semantic_token_rules.json");
    let json = std::str::from_utf8(&content.data).expect("invalid utf-8 in semantic_token_rules");
    settings::parse_json_with_comments::<SemanticTokenRules>(json)
        .expect("failed to parse cpp semantic_token_rules.json")
}

pub(crate) struct CppContextProvider;

const CATCH2_TEST_PROGRAM: &str = "CATCH2_TEST_PROGRAM";
const CATCH2_TEST_CWD: &str = "CATCH2_TEST_CWD";
const CATCH2_TEST_ADDITIONAL_ARGS: &str = "CATCH2_TEST_ADDITIONAL_ARGS";
const CATCH2_TEST_BUILD_TASK: &str = "CATCH2_TEST_BUILD_TASK";
const CATCH2_TEST_NAME_VARIABLE: VariableName = VariableName::Custom(Cow::Borrowed("_test_name"));
const CATCH2_SECTION_NAME_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("_section_name"));

impl ContextProvider for CppContextProvider {
    fn build_context(
        &self,
        variables: &TaskVariables,
        _location: language::ContextLocation<'_>,
        _project_env: Option<collections::HashMap<String, String>>,
        _toolchains: Arc<dyn LanguageToolchainStore>,
        _cx: &mut App,
    ) -> Task<anyhow::Result<TaskVariables>> {
        let mut new_vars = TaskVariables::default();

        if let Some(raw) = variables.get(&CATCH2_TEST_NAME_VARIABLE) {
            new_vars.insert(
                VariableName::Custom(Cow::Borrowed("CATCH2_TEST_NAME")),
                raw.trim_matches('"').to_owned(),
            );
        }
        if let Some(raw) = variables.get(&CATCH2_SECTION_NAME_VARIABLE) {
            new_vars.insert(
                VariableName::Custom(Cow::Borrowed("CATCH2_SECTION_NAME")),
                raw.trim_matches('"').to_owned(),
            );
        }

        Task::ready(Ok(new_vars))
    }

    fn associated_tasks(
        &self,
        buffer: Option<gpui::Entity<Buffer>>,
        cx: &App,
    ) -> Task<Option<TaskTemplates>> {
        let language = LanguageName::new_static("C++");
        let settings = LanguageSettings::resolve(buffer.map(|b| b.read(cx)), Some(&language), cx);
        let Some(executable) = settings.tasks.variables.get(CATCH2_TEST_PROGRAM).cloned() else {
            let error_msg = format!(
                "Catch2 test executable not configured. \
                 Set the {CATCH2_TEST_PROGRAM} variable under \
                 languages.C++.tasks.variables in your settings.json."
            );
            return Task::ready(Some(TaskTemplates(vec![
                TaskTemplate {
                    label: "catch2 run test case (not configured)".to_owned(),
                    command: "echo".to_owned(),
                    args: vec![error_msg.clone()],
                    tags: vec!["catch2-test".to_owned()],
                    ..TaskTemplate::default()
                },
                TaskTemplate {
                    label: "catch2 run section (not configured)".to_owned(),
                    command: "echo".to_owned(),
                    args: vec![error_msg],
                    tags: vec!["catch2-section".to_owned()],
                    ..TaskTemplate::default()
                },
            ])));
        };

        let cwd = settings
            .tasks
            .variables
            .get(CATCH2_TEST_CWD)
            .cloned()
            .or(Some("$ZED_DIRNAME".to_owned()));

        let extra_args: Vec<String> = settings
            .tasks
            .variables
            .get(CATCH2_TEST_ADDITIONAL_ARGS)
            .map(|s| s.split_whitespace().map(str::to_owned).collect())
            .unwrap_or_default();

        let build_task_env: collections::HashMap<String, String> = settings
            .tasks
            .variables
            .get(CATCH2_TEST_BUILD_TASK)
            .map(|name| {
                let mut map = collections::HashMap::default();
                map.insert(CATCH2_TEST_BUILD_TASK.to_owned(), name.clone());
                map
            })
            .unwrap_or_default();

        let catch2_test_name = VariableName::Custom(Cow::Borrowed("CATCH2_TEST_NAME"))
            .template_value_with_whitespace();
        let catch2_section_name = VariableName::Custom(Cow::Borrowed("CATCH2_SECTION_NAME"))
            .template_value_with_whitespace();

        let mut test_args = vec![catch2_test_name.clone()];
        test_args.extend(extra_args.iter().cloned());

        let mut section_args = vec![
            catch2_test_name.clone(),
            "--section".to_owned(),
            catch2_section_name.clone(),
        ];
        section_args.extend(extra_args);

        Task::ready(Some(TaskTemplates(vec![
            TaskTemplate {
                label: format!("catch2 run test case {catch2_test_name}"),
                command: executable.clone(),
                args: test_args,
                tags: vec!["catch2-test".to_owned()],
                cwd: cwd.clone(),
                env: build_task_env.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("catch2 run section {catch2_section_name} in {catch2_test_name}"),
                command: executable.clone(),
                args: section_args,
                tags: vec!["catch2-section".to_owned()],
                cwd: cwd.clone(),
                env: build_task_env,
                ..TaskTemplate::default()
            },
        ])))
    }
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;
    use unindent::Unindent;

    #[gpui::test]
    fn test_catch2_runnable_detection(cx: &mut TestAppContext) {
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        let source = r#"
            TEST_CASE("My test case") {
                SECTION("first section") {
                }
                SECTION("second section") {
                }
            }
        "#
        .unindent();

        let buffer = cx.new(|cx| Buffer::local(source.clone(), cx).with_language(language, cx));
        cx.executor().run_until_parked();

        let runnables: Vec<_> = buffer.update(cx, |buffer, _| {
            buffer.snapshot().runnable_ranges(0..source.len()).collect()
        });

        // Collect (tag, extra_captures) pairs for easier assertions.
        let mut entries: Vec<(String, Vec<(String, String)>)> = runnables
            .iter()
            .flat_map(|r| {
                r.runnable.tags.iter().map(|tag| {
                    let mut captures: Vec<(String, String)> = r
                        .extra_captures
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    captures.sort();
                    (tag.0.to_string(), captures)
                })
            })
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        // One catch2-test for the TEST_CASE.
        let test_entries: Vec<_> = entries
            .iter()
            .filter(|(tag, _)| tag == "catch2-test")
            .collect();
        assert_eq!(test_entries.len(), 1);
        assert!(
            test_entries[0]
                .1
                .iter()
                .any(|(k, v)| k == "_test_name" && v == "\"My test case\"")
        );

        // Two catch2-section entries, one per SECTION, each carrying both _section_name and _test_name.
        let section_entries: Vec<_> = entries
            .iter()
            .filter(|(tag, _)| tag == "catch2-section")
            .collect();
        assert_eq!(section_entries.len(), 2);

        let section0: Vec<(String, String)> = section_entries[0].1.clone();
        assert_eq!(section0.len(), 4);
        assert_eq!(section0[0].0, "_inner_macro");
        assert_eq!(section0[1].0, "_outer_macro");
        assert_eq!(section0[2].0, "_section_name");
        assert_eq!(section0[3].0, "_test_name");
        assert_eq!(section0[2].1, "\"first section\"");
        assert_eq!(section0[3].1, "\"My test case\"");

        let section1: Vec<(String, String)> = section_entries[1].1.clone();
        assert_eq!(section1.len(), 4);
        assert_eq!(section1[0].0, "_inner_macro");
        assert_eq!(section1[1].0, "_outer_macro");
        assert_eq!(section1[2].0, "_section_name");
        assert_eq!(section1[3].0, "_test_name");
        assert_eq!(section1[2].1, "\"second section\"");
        assert_eq!(section1[3].1, "\"My test case\"");
    }

    #[gpui::test]
    async fn test_cpp_autoindent_access_specifier(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    class Foo {
                    public:
                    void bar();
                    private:
                    int x;
                    };
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                class Foo {
                  public:
                    void bar();
                  private:
                    int x;
                };
                "#
                .unindent(),
                "members after access specifiers should be indented one level deeper than the specifier"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_access_specifier_next_line(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    class Foo {
                    public:
                      void bar();
                    void baz();
                    private:
                      int x;
                    };
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                class Foo {
                  public:
                    void bar();
                    void baz();
                  private:
                    int x;
                };
                "#
                .unindent(),
                "members after access specifiers should be indented one level deeper than the specifier"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_nested_class_access_specifiers(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    class Outer {
                    public:
                    class Inner {
                    public:
                    void inner_pub();
                    private:
                    int inner_priv;
                    };
                    private:
                    int outer_priv;
                    };
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                class Outer {
                  public:
                    class Inner {
                      public:
                        void inner_pub();
                      private:
                        int inner_priv;
                    };
                  private:
                    int outer_priv;
                };
                "#
                .unindent(),
                "nested class access specifiers should indent independently at each nesting level"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_consecutive_access_specifiers(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    class Foo {
                    public:
                    protected:
                    private:
                    int x;
                    };
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                class Foo {
                  public:
                  protected:
                  private:
                    int x;
                };
                "#
                .unindent(),
                "consecutive access specifiers with no members between them should all align at class level"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_indented_access_specifiers(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    class Foo {
                    int default_member;
                    public:
                    void pub_method();
                    private:
                    int priv_member;
                    };
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                class Foo {
                  int default_member;
                  public:
                    void pub_method();
                  private:
                    int priv_member;
                };
                "#
                .unindent(),
                "access specifiers should be indented one level inside class braces"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_access_specifier_with_method_bodies(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    class Foo {
                    public:
                    void bar() {
                    if (x)
                    y++;
                    }
                    private:
                    int get_x() {
                    return x;
                    }
                    int x;
                    };
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                class Foo {
                  public:
                    void bar() {
                      if (x)
                        y++;
                    }
                  private:
                    int get_x() {
                      return x;
                    }
                    int x;
                };
                "#
                .unindent(),
                "method bodies inside access specifier sections should compose brace and specifier indent"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_basic(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit([(0..0, "int main() {}")], None, cx);

            let ix = buffer.len() - 1;
            buffer.edit([(ix..ix, "\n\n")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                "int main() {\n  \n}",
                "content inside braces should be indented"
            );

            buffer
        });
    }

    #[gpui::test]
    async fn test_cpp_autoindent_if_else(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });
        let language = crate::language("cpp", tree_sitter_cpp::LANGUAGE.into());

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);

            buffer.edit(
                [(
                    0..0,
                    r#"
                    int main() {
                    if (a)
                    b;
                    }
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a)
                    b;
                }
                "#
                .unindent(),
                "body of if-statement without braces should be indented"
            );

            let ix = buffer.len() - 4;
            buffer.edit([(ix..ix, "\n.c")], Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a)
                    b
                      .c;
                }
                "#
                .unindent(),
                "field expression (.c) should be indented further than the statement body"
            );

            buffer.edit([(0..buffer.len(), "")], Some(AutoindentMode::EachLine), cx);
            buffer.edit(
                [(
                    0..0,
                    r#"
                    int main() {
                    if (a) a++;
                    else b++;
                    }
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a) a++;
                  else b++;
                }
                "#
                .unindent(),
                "single-line if/else without braces should align at the same level"
            );

            buffer.edit([(0..buffer.len(), "")], Some(AutoindentMode::EachLine), cx);
            buffer.edit(
                [(
                    0..0,
                    r#"
                    int main() {
                    if (a)
                    b++;
                    else
                    c++;
                    }
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a)
                    b++;
                  else
                    c++;
                }
                "#
                .unindent(),
                "multi-line if/else without braces should indent statement bodies"
            );

            buffer.edit([(0..buffer.len(), "")], Some(AutoindentMode::EachLine), cx);
            buffer.edit(
                [(
                    0..0,
                    r#"
                    int main() {
                    if (a)
                    if (b)
                    c++;
                    }
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a)
                    if (b)
                      c++;
                }
                "#
                .unindent(),
                "nested if statements without braces should indent properly"
            );

            buffer.edit([(0..buffer.len(), "")], Some(AutoindentMode::EachLine), cx);
            buffer.edit(
                [(
                    0..0,
                    r#"
                    int main() {
                    if (a)
                    b++;
                    else if (c)
                    d++;
                    else
                    f++;
                    }
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a)
                    b++;
                  else if (c)
                    d++;
                  else
                    f++;
                }
                "#
                .unindent(),
                "else-if chains should align all conditions at same level with indented bodies"
            );

            buffer.edit([(0..buffer.len(), "")], Some(AutoindentMode::EachLine), cx);
            buffer.edit(
                [(
                    0..0,
                    r#"
                    int main() {
                    if (a) {
                    b++;
                    } else
                    c++;
                    }
                    "#
                    .unindent(),
                )],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                r#"
                int main() {
                  if (a) {
                    b++;
                  } else
                    c++;
                }
                "#
                .unindent(),
                "mixed braces should indent properly"
            );

            buffer
        });
    }
}
