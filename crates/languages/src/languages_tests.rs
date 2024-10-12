use super::*;
use crate::language_settings::{
    AllLanguageSettings, AllLanguageSettingsContent, LanguageSettingsContent,
};
use crate::Buffer;
use language::LanguageRegistry;
use regex::RegexBuilder;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
async fn test_real_first_line_pattern(cx: &mut TestAppContext) {
    use languages::load_config;
    cx.update(|cx| init_settings(cx, |_| {}));

    let languages = LanguageRegistry::test(cx.executor());
    let languages = Arc::new(languages);
    languages.register_test_language(load_config("bash"));

    let shebang_lang = vec![
        ("#!/usr/bin/env bash", "Shell Script"),
        ("#!/bin/bash", "Shell Script"),
        ("#!/bin/bash -e", "Shell Script"),
        ("#!/usr/bin/bash", "Shell Script"),
        ("#!/bin/dash", "Shell Script"),
        ("#!/bin/ash", "Shell Script"),
        ("#!/bin/zsh", "Shell Script"),
        ("#!/bin/env node", "JavaScript"),
        ("#!/usr/bin/env python", "Python"),
    ];
    let f = file("some/script");
    for (shebang, lang) in shebang_lang {
        dbg!(&shebang, &lang);
        let rope = Rope::from(shebang.into());
        assert_eq!(
            cx.read(|cx| languages.language_for_file(&f, Some(&rope), cx))
                .unwrap()
                .name(),
            lang.into()
        );
    }
}
