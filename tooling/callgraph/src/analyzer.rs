use crate::blocklist::Blocklist;
use crate::diagnostics::Warning;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;

/// The result of analyzing a single file: warnings plus the source text
/// (needed by the diagnostic renderer).
pub struct FileAnalysis {
    pub path: PathBuf,
    pub source: String,
    pub warnings: Vec<Warning>,
}

/// Discover all Rust source files for the given packages in a cargo workspace.
pub fn discover_source_files(manifest_path: &Path, packages: &[String]) -> Result<Vec<PathBuf>> {
    let manifest_path = manifest_path.join("Cargo.toml");
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&manifest_path)
        .exec()
        .with_context(|| {
            format!(
                "failed to run cargo metadata on {}",
                manifest_path.display()
            )
        })?;

    let mut source_files = Vec::new();

    let target_packages: Vec<_> = if packages.is_empty() {
        metadata.workspace_packages().into_iter().collect()
    } else {
        metadata
            .workspace_packages()
            .into_iter()
            .filter(|package| packages.iter().any(|name| &package.name == name))
            .collect()
    };

    for package in target_packages {
        let package_dir = package
            .manifest_path
            .parent()
            .expect("manifest path should have a parent");
        collect_rs_files(&PathBuf::from(package_dir.as_std_path()), &mut source_files)?;
    }

    Ok(source_files)
}

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

/// Analyze a single Rust source file for blocking calls in async contexts.
///
/// When `include_tests` is false, files under `tests/` directories are skipped
/// entirely, and `#[cfg(test)]` modules and `#[test]` functions within other
/// files are ignored.
pub fn analyze_file(
    path: &Path,
    blocklist: &Blocklist,
    include_tests: bool,
) -> Result<FileAnalysis> {
    // Layer 1: skip entire files under tests/ directories.
    if !include_tests && is_test_file(path) {
        return Ok(FileAnalysis {
            path: path.to_path_buf(),
            source: String::new(),
            warnings: Vec::new(),
        });
    }

    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let syntax =
        syn::parse_file(&source).with_context(|| format!("failed to parse {}", path.display()))?;

    let warnings = analyze_syntax(path, &source, &syntax, blocklist, include_tests);
    Ok(FileAnalysis {
        path: path.to_path_buf(),
        source,
        warnings,
    })
}

/// Analyze source provided as a string (for unit testing the analyzer itself).
pub fn analyze_source(
    path: &Path,
    source: &str,
    blocklist: &Blocklist,
    include_tests: bool,
) -> Result<Vec<Warning>> {
    let syntax =
        syn::parse_file(source).with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(analyze_syntax(path, source, &syntax, blocklist, include_tests))
}

/// Check if a file path looks like test code (lives under a `tests/` dir).
fn is_test_file(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "tests")
}

fn analyze_syntax(
    path: &Path,
    source: &str,
    syntax: &syn::File,
    blocklist: &Blocklist,
    include_tests: bool,
) -> Vec<Warning> {
    use syn::visit::Visit;

    let line_starts = build_line_starts(source);

    let mut visitor = AsyncBlockingVisitor {
        path: path.to_path_buf(),
        blocklist,
        warnings: Vec::new(),
        in_async_context: None,
        use_map: resolve_imports(syntax),
        in_safe_wrapper: false,
        include_tests,
        line_starts: &line_starts,
    };
    visitor.visit_file(syntax);
    visitor.warnings
}

/// Build a table mapping 1-based line numbers to byte offsets of line starts.
/// Index 0 is unused; line_starts[1] is the byte offset of line 1.
fn build_line_starts(source: &str) -> Vec<usize> {
    let mut starts = vec![0]; // index 0 placeholder
    starts.push(0); // line 1 starts at byte 0
    for (i, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

/// Convert a proc_macro2 line/column (1-based line, 0-based column) to a byte
/// offset in the source.
fn line_col_to_byte(line: usize, column: usize, line_starts: &[usize]) -> usize {
    if line < line_starts.len() {
        line_starts[line] + column
    } else {
        // Past end of file — return end of last known line.
        line_starts.last().copied().unwrap_or(0)
    }
}

/// Resolve `use` declarations to build a map from short names to full paths.
///
/// Examples:
/// - `use std::fs;` → `{"fs" → "std::fs"}`
/// - `use std::fs::read;` → `{"read" → "std::fs::read"}`
/// - `use std::fs as myfs;` → `{"myfs" → "std::fs"}`
fn resolve_imports(syntax: &syn::File) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for item in &syntax.items {
        if let syn::Item::Use(use_item) = item {
            collect_use_paths(&use_item.tree, &mut String::new(), &mut map);
        }
    }
    map
}

fn collect_use_paths(tree: &syn::UseTree, prefix: &mut String, map: &mut HashMap<String, String>) {
    match tree {
        syn::UseTree::Path(use_path) => {
            let old_len = prefix.len();
            if !prefix.is_empty() {
                prefix.push_str("::");
            }
            prefix.push_str(&use_path.ident.to_string());
            collect_use_paths(&use_path.tree, prefix, map);
            prefix.truncate(old_len);
        }
        syn::UseTree::Name(use_name) => {
            let full = if prefix.is_empty() {
                use_name.ident.to_string()
            } else {
                format!("{}::{}", prefix, use_name.ident)
            };
            map.insert(use_name.ident.to_string(), full);
        }
        syn::UseTree::Rename(use_rename) => {
            let full = if prefix.is_empty() {
                use_rename.ident.to_string()
            } else {
                format!("{}::{}", prefix, use_rename.ident)
            };
            map.insert(use_rename.rename.to_string(), full);
        }
        syn::UseTree::Glob(_) => {
            // For `use std::fs::*`, we can't resolve individual names statically.
            // We store the glob prefix so callers can check against it.
            if !prefix.is_empty() {
                map.insert(format!("{}::*", prefix), prefix.clone());
            }
        }
        syn::UseTree::Group(use_group) => {
            for item in &use_group.items {
                collect_use_paths(item, prefix, map);
            }
        }
    }
}

/// Try to extract a resolved call path from a function call expression.
///
/// Only handles path-based calls like `std::fs::read(...)` or `read(...)`.
/// Returns `None` for complex expressions (closures, field access, etc.).
fn extract_call_path(expr: &syn::Expr, use_map: &HashMap<String, String>) -> Option<String> {
    if let syn::Expr::Path(expr_path) = expr {
        let path_str = path_to_string(&expr_path.path);
        Some(resolve_path(&path_str, use_map))
    } else {
        None
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn resolve_path(path_str: &str, use_map: &HashMap<String, String>) -> String {
    // Multi-segment path: try resolving the first segment via use map.
    // e.g., `fs::read` with `use std::fs` → `std::fs::read`
    if let Some(first_sep) = path_str.find("::") {
        let first = &path_str[..first_sep];
        let rest = &path_str[first_sep..];
        if let Some(resolved) = use_map.get(first) {
            return format!("{resolved}{rest}");
        }
    }
    // Single-segment: try resolving the whole name.
    // e.g., `read` with `use std::fs::read` → `std::fs::read`
    if let Some(resolved) = use_map.get(path_str) {
        return resolved.clone();
    }
    path_str.to_string()
}

/// Compute the byte-offset range for the function path of a call expression.
/// For `std::fs::read("file")`, this spans `std::fs::read`.
fn call_byte_span(expr: &syn::Expr, line_starts: &[usize]) -> Range<usize> {
    match expr {
        syn::Expr::Call(call) => {
            let start = call.func.span().start();
            let end = call.func.span().end();
            let byte_start = line_col_to_byte(start.line, start.column, line_starts);
            let byte_end = line_col_to_byte(end.line, end.column, line_starts);
            byte_start..byte_end
        }
        syn::Expr::MethodCall(call) => {
            let start = call.method.span().start();
            let end = call.method.span().end();
            let byte_start = line_col_to_byte(start.line, start.column, line_starts);
            let byte_end = line_col_to_byte(end.line, end.column, line_starts);
            byte_start..byte_end
        }
        _ => 0..0,
    }
}

struct AsyncBlockingVisitor<'a> {
    path: PathBuf,
    blocklist: &'a Blocklist,
    warnings: Vec<Warning>,
    in_async_context: Option<String>,
    use_map: HashMap<String, String>,
    in_safe_wrapper: bool,
    include_tests: bool,
    line_starts: &'a [usize],
}

/// Check whether an attribute list contains a test-related attribute.
/// Matches `#[test]`, `#[tokio::test]`, `#[gpui::test]`, and any other
/// path ending in `test`.
fn has_test_attribute(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let path = attr.path();
        let last = path.segments.last();
        last.is_some_and(|seg| seg.ident == "test")
    })
}

/// Check whether an attribute list contains `#[cfg(test)]`.
fn has_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        // Parse the token content inside #[cfg(...)]
        attr.parse_args::<syn::Ident>()
            .is_ok_and(|ident| ident == "test")
    })
}

impl<'a> syn::visit::Visit<'_> for AsyncBlockingVisitor<'a> {
    // Layer 2: skip #[cfg(test)] modules entirely.
    fn visit_item_mod(&mut self, node: &syn::ItemMod) {
        if !self.include_tests && has_cfg_test(&node.attrs) {
            return;
        }
        syn::visit::visit_item_mod(self, node);
    }

    fn visit_item_fn(&mut self, node: &syn::ItemFn) {
        // Layer 3: skip #[test] / #[gpui::test] / etc. functions.
        if !self.include_tests && has_test_attribute(&node.attrs) {
            return;
        }

        let was_async = self.in_async_context.take();

        if node.sig.asyncness.is_some() {
            let fn_name = node.sig.ident.to_string();
            self.in_async_context = Some(format!("async fn `{fn_name}`"));
        }

        syn::visit::visit_item_fn(self, node);
        self.in_async_context = was_async;
    }

    fn visit_impl_item_fn(&mut self, node: &syn::ImplItemFn) {
        if !self.include_tests && has_test_attribute(&node.attrs) {
            return;
        }

        let was_async = self.in_async_context.take();

        if node.sig.asyncness.is_some() {
            let fn_name = node.sig.ident.to_string();
            self.in_async_context = Some(format!("async fn `{fn_name}`"));
        }

        syn::visit::visit_impl_item_fn(self, node);
        self.in_async_context = was_async;
    }

    fn visit_trait_item_fn(&mut self, node: &syn::TraitItemFn) {
        let was_async = self.in_async_context.take();

        if node.sig.asyncness.is_some() {
            let fn_name = node.sig.ident.to_string();
            self.in_async_context = Some(format!("async fn `{fn_name}`"));
        }

        syn::visit::visit_trait_item_fn(self, node);
        self.in_async_context = was_async;
    }

    fn visit_expr(&mut self, node: &syn::Expr) {
        // Inside a safe wrapper, skip all checking.
        if self.in_safe_wrapper {
            syn::visit::visit_expr(self, node);
            return;
        }

        if let Some(ref context) = self.in_async_context {
            if let syn::Expr::Call(call) = node {
                if let Some(path) = extract_call_path(&call.func, &self.use_map) {
                    // Check for safe wrappers first — skip their closure args.
                    if self.blocklist.is_safe_wrapper(&path) {
                        let was_safe = self.in_safe_wrapper;
                        self.in_safe_wrapper = true;
                        syn::visit::visit_expr(self, node);
                        self.in_safe_wrapper = was_safe;
                        return;
                    }
                    if let Some(entry) = self.blocklist.matches(&path) {
                        self.warnings.push(Warning {
                            path: self.path.clone(),
                            span: call_byte_span(node, self.line_starts),
                            call_path: path,
                            category: entry.category.clone(),
                            help: entry.help.clone(),
                            context: context.clone(),
                        });
                    }
                }
            }
            // Method calls like `.lock()`, `.read()`, `.write()` are NOT
            // checked in Phase 1. Without type information we can't distinguish
            // `mutex.lock()` from `some_other_type.lock()`, which would produce
            // too many false positives. This is a known gap addressed in Phase 2
            // (GPUI-aware analysis) and Phase 4 (rustc driver with type info).
        }

        syn::visit::visit_expr(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocklist::Blocklist;
    use std::path::PathBuf;

    fn test_blocklist() -> Blocklist {
        Blocklist::load(false).expect("blocklist should load")
    }

    fn analyze(source: &str) -> Vec<Warning> {
        let blocklist = test_blocklist();
        analyze_source(&PathBuf::from("test.rs"), source, &blocklist, true)
            .expect("analysis should succeed")
    }

    fn analyze_skip_tests(source: &str) -> Vec<Warning> {
        let blocklist = test_blocklist();
        analyze_source(&PathBuf::from("test.rs"), source, &blocklist, false)
            .expect("analysis should succeed")
    }

    #[test]
    fn detects_direct_fs_call_in_async_fn() {
        let warnings = analyze(
            r#"
            async fn bad() {
                std::fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].call_path.contains("std::fs::read"));
        assert_eq!(warnings[0].category, "filesystem");
    }

    #[test]
    fn detects_imported_fs_call() {
        let warnings = analyze(
            r#"
            use std::fs;
            async fn bad() {
                fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].call_path.contains("std::fs::read"));
    }

    #[test]
    fn detects_direct_import_call() {
        let warnings = analyze(
            r#"
            use std::fs::read_to_string;
            async fn bad() {
                read_to_string("file.txt").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].call_path.contains("std::fs::read_to_string"));
    }

    #[test]
    fn detects_thread_sleep() {
        let warnings = analyze(
            r#"
            async fn bad() {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].call_path.contains("std::thread::sleep"));
        assert_eq!(warnings[0].category, "thread");
    }

    #[test]
    fn detects_block_on() {
        let warnings = analyze(
            r#"
            async fn bad() {
                pollster::block_on(async { 42 });
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].call_path.contains("pollster::block_on"));
    }

    #[test]
    fn no_warning_in_sync_fn() {
        let warnings = analyze(
            r#"
            fn sync_fn() {
                std::fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn no_warning_inside_smol_unblock() {
        let warnings = analyze(
            r#"
            async fn good() {
                smol::unblock(|| {
                    std::fs::read("file.txt").unwrap();
                }).await;
            }
            "#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn no_warning_inside_thread_spawn() {
        let warnings = analyze(
            r#"
            async fn good() {
                std::thread::spawn(|| {
                    std::fs::read("file.txt").unwrap();
                });
            }
            "#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn detects_renamed_import() {
        let warnings = analyze(
            r#"
            use std::fs as myfs;
            async fn bad() {
                myfs::read("file.txt").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].call_path.contains("std::fs::read"));
    }

    #[test]
    fn detects_multiple_violations() {
        let warnings = analyze(
            r#"
            async fn bad() {
                std::fs::read("a.txt").unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
                std::fs::write("b.txt", "data").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 3);
    }

    #[test]
    fn detects_in_impl_async_fn() {
        let warnings = analyze(
            r#"
            struct Foo;
            impl Foo {
                async fn bad(&self) {
                    std::fs::read("file.txt").unwrap();
                }
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].context.contains("async fn `bad`"));
    }

    #[test]
    fn context_describes_function() {
        let warnings = analyze(
            r#"
            async fn load_config() {
                std::fs::read("config.toml").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].context, "async fn `load_config`");
    }

    #[test]
    fn blocklist_wildcard_matching() {
        let blocklist = test_blocklist();
        assert!(blocklist.matches("std::fs::read").is_some());
        assert!(blocklist.matches("std::fs::write").is_some());
        assert!(blocklist.matches("std::fs::metadata").is_some());
        assert!(blocklist.matches("std::thread::sleep").is_some());
        assert!(blocklist.matches("std::vec::Vec::push").is_none());
        assert!(blocklist.matches("tokio::fs::read").is_none());
    }

    #[test]
    fn blocklist_pedantic_filtering() {
        let non_pedantic = Blocklist::load(false).expect("load");
        let pedantic = Blocklist::load(true).expect("load");
        assert!(non_pedantic.matches("parking_lot::Mutex::lock").is_none());
        assert!(pedantic.matches("parking_lot::Mutex::lock").is_some());
    }

    #[test]
    fn safe_wrapper_detection() {
        let blocklist = test_blocklist();
        assert!(blocklist.is_safe_wrapper("smol::unblock"));
        assert!(blocklist.is_safe_wrapper("std::thread::spawn"));
        assert!(!blocklist.is_safe_wrapper("std::fs::read"));
    }

    #[test]
    fn use_resolution() {
        let source = r#"
            use std::fs;
            use std::thread::sleep;
            use std::net::TcpStream as Tcp;
        "#;
        let syntax = syn::parse_file(source).expect("parse");
        let map = resolve_imports(&syntax);
        assert_eq!(map.get("fs").map(String::as_str), Some("std::fs"));
        assert_eq!(
            map.get("sleep").map(String::as_str),
            Some("std::thread::sleep")
        );
        assert_eq!(
            map.get("Tcp").map(String::as_str),
            Some("std::net::TcpStream")
        );
    }

    #[test]
    fn grouped_use_resolution() {
        let source = r#"
            use std::fs::{read, write, metadata};
        "#;
        let syntax = syn::parse_file(source).expect("parse");
        let map = resolve_imports(&syntax);
        assert_eq!(map.get("read").map(String::as_str), Some("std::fs::read"));
        assert_eq!(
            map.get("write").map(String::as_str),
            Some("std::fs::write")
        );
        assert_eq!(
            map.get("metadata").map(String::as_str),
            Some("std::fs::metadata")
        );
    }

    #[test]
    fn span_points_at_call_path() {
        let source = "async fn f() {\n    std::fs::read(\"x\");\n}\n";
        let warnings = analyze(source);
        assert_eq!(warnings.len(), 1);
        let span = &warnings[0].span;
        assert_eq!(&source[span.clone()], "std::fs::read");
    }

    #[test]
    fn skips_test_attributed_fn() {
        let warnings = analyze_skip_tests(
            r#"
            #[test]
            async fn my_test() {
                std::fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn skips_gpui_test_attributed_fn() {
        let warnings = analyze_skip_tests(
            r#"
            #[gpui::test]
            async fn my_test() {
                std::fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn skips_cfg_test_module() {
        let warnings = analyze_skip_tests(
            r#"
            #[cfg(test)]
            mod tests {
                async fn helper() {
                    std::fs::read("file.txt").unwrap();
                }
            }
            "#,
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn include_tests_flag_overrides_skip() {
        let warnings = analyze(
            r#"
            #[test]
            async fn my_test() {
                std::fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn skips_test_file_path() {
        assert!(is_test_file(Path::new("crates/fs/tests/integration/fs.rs")));
        assert!(is_test_file(Path::new("crates/project/tests/project_tests.rs")));
        assert!(!is_test_file(Path::new("crates/fs/src/fs.rs")));
        assert!(!is_test_file(Path::new("crates/editor/src/editor.rs")));
    }

    #[test]
    fn non_test_async_fn_still_flagged_when_skipping_tests() {
        let warnings = analyze_skip_tests(
            r#"
            async fn production_code() {
                std::fs::read("file.txt").unwrap();
            }

            #[test]
            async fn my_test() {
                std::fs::read("file.txt").unwrap();
            }
            "#,
        );
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].context, "async fn `production_code`");
    }
}
