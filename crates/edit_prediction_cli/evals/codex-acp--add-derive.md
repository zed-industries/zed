+++
repository_url = "https://github.com/zed-industries/codex-acp"
revision = "c3d24ee70928fc9da08c131fc632d624413ccc43"
+++

## Edit History

```diff
--- a/src/prompt_args.rs
+++ b/src/prompt_args.rs
@@ -28,7 +28,7 @@ impl PromptArgsError {
     }
 }

-#[derive(Debug)]
+#[derive(Debug, Serialize)]
 pub enum PromptExpansionError {
     Args {
         command: String,
```

## Cursor Position

```src/prompt_args.rs
#[derive(Debug)]
pub enum PromptArgsError {
//                      ^[CURSOR_POSITION]
    MissingAssignment { token: String },
    MissingKey { token: String },
}
```

## Expected Patch

```diff
--- a/src/prompt_args.rs
+++ b/src/prompt_args.rs
@@ -9,7 +9,7 @@ use std::sync::LazyLock;
 static PROMPT_ARG_REGEX: LazyLock<Regex> =
     LazyLock::new(|| Regex::new(r"\$[A-Z][A-Z0-9_]*").unwrap_or_else(|_| std::process::abort()));

-#[derive(Debug)]
+#[derive(Debug, Serialize)]
 pub enum PromptArgsError {
     MissingAssignment { token: String },
     MissingKey { token: String },
```

```diff
--- a/src/prompt_args.rs
+++ b/src/prompt_args.rs
@@ -3,19 +3,20 @@
 use regex_lite::Regex;
+use serde::Serialize;
 use shlex::Shlex;
 use std::collections::HashMap;
 use std::collections::HashSet;
 use std::sync::LazyLock;

 static PROMPT_ARG_REGEX: LazyLock<Regex> =
     LazyLock::new(|| Regex::new(r"\$[A-Z][A-Z0-9_]*").unwrap_or_else(|_| std::process::abort()));

-#[derive(Debug)]
+#[derive(Debug, Serialize)]
 pub enum PromptArgsError {
     MissingAssignment { token: String },
     MissingKey { token: String },
 }
```
