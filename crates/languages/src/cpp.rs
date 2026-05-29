use settings::SemanticTokenRules;

pub(crate) fn semantic_token_rules() -> SemanticTokenRules {
    let content = grammars::get_file("cpp/semantic_token_rules.json")
        .expect("missing cpp/semantic_token_rules.json");
    let json = std::str::from_utf8(&content.data).expect("invalid utf-8 in semantic_token_rules");
    settings::parse_json_with_comments::<SemanticTokenRules>(json)
        .expect("failed to parse cpp semantic_token_rules.json")
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;
    use unindent::Unindent;

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
