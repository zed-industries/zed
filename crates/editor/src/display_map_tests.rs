#[cfg(test)]
mod syntax_token_tests {
    use crate::display_map::SyntaxTokenView;
    use crate::editor_tests::init_test;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use language::{tree_sitter_rust, Buffer, Language, LanguageConfig};
    use std::sync::Arc;
    use theme::ActiveTheme;

    #[gpui::test]
    async fn test_syntax_tokens_detect_variables_parameters_constants(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        // Initialize a Rust language
        let language = Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: Default::default(),
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_highlights_query(tree_sitter_rust::HIGHLIGHTS_QUERY)
        .unwrap();

        let language = Arc::new(language);

        // Create a buffer with various Rust constructs
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(
                indoc! {r#"
                    fn process_data(param1: u32, param2: &str) -> u32 {
                        let local_var = 42;
                        const CONSTANT: u32 = 100;
                        let result = param1 + local_var + CONSTANT;
                        result
                    }
                    
                    struct MyStruct {
                        field: u32,
                    }
                    
                    impl MyStruct {
                        fn method(&self, method_param: u32) -> u32 {
                            let temp = method_param + self.field;
                            temp
                        }
                    }
                    
                    macro_rules! my_macro {
                        ($macro_param:expr) => {
                            let macro_local = $macro_param;
                            macro_local
                        };
                    }
                "#},
                cx,
            );
            buffer.set_language(Some(language.clone()), cx);
            buffer
        });

        // Wait for parsing to complete
        cx.condition(&buffer, |buf: &mut Buffer, _| !buf.is_parsing()).await;

        let (snapshot, token_texts) = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot();
            let cache = Arc::new(crate::rainbow::VariableColorCache::new(
                crate::editor_settings::VariableColorMode::ThemePalette,
            ));
            let theme = cx.theme().syntax().clone();

            // Extract syntax tokens
            let tokens = SyntaxTokenView::new(snapshot.clone(), &cache, &theme);

            assert!(tokens.is_some(), "Should extract syntax tokens");

            let tokens = tokens.unwrap();

            // Check that we found tokens for various contexts
            let token_texts: Vec<String> = tokens
                .tokens
                .iter()
                .map(|token| snapshot.text_for_range(token.range.clone()).collect())
                .collect();

            (snapshot, token_texts)
        });

        // Function parameters
        assert!(
            token_texts.contains(&"param1".to_string()),
            "Should detect function parameter 'param1'. Found: {:?}",
            token_texts
        );
        assert!(
            token_texts.contains(&"param2".to_string()),
            "Should detect function parameter 'param2'. Found: {:?}",
            token_texts
        );

        // Local variables
        assert!(
            token_texts.contains(&"local_var".to_string()),
            "Should detect local variable 'local_var'. Found: {:?}",
            token_texts
        );
        assert!(
            token_texts.contains(&"result".to_string()),
            "Should detect local variable 'result'. Found: {:?}",
            token_texts
        );

        // Constants
        assert!(
            token_texts.contains(&"CONSTANT".to_string()),
            "Should detect constant 'CONSTANT'. Found: {:?}",
            token_texts
        );

        // Method parameters
        assert!(
            token_texts.contains(&"method_param".to_string()),
            "Should detect method parameter 'method_param'. Found: {:?}",
            token_texts
        );
        assert!(
            token_texts.contains(&"temp".to_string()),
            "Should detect local variable 'temp' in method. Found: {:?}",
            token_texts
        );

        // Macro parameters and locals
        assert!(
            token_texts.contains(&"macro_param".to_string()),
            "Should detect macro parameter 'macro_param'. Found: {:?}",
            token_texts
        );
        assert!(
            token_texts.contains(&"macro_local".to_string()),
            "Should detect macro local 'macro_local'. Found: {:?}",
            token_texts
        );

        // Should NOT include struct names, field names (unless in variable context), or keywords
        assert!(
            !token_texts.contains(&"MyStruct".to_string()),
            "Should NOT detect type name 'MyStruct'"
        );
        assert!(
            !token_texts.contains(&"process_data".to_string()),
            "Should NOT detect function name 'process_data'"
        );

        eprintln!("✓ Syntax tokens detected: {:?}", token_texts);
    }
}
