+++
repository_url = "git@github.com:tree-sitter/tree-sitter"
revision = "24007727d42b4caceda3095ac685c463fae1ba1a"
+++

## Edit History

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -604,7 +604,7 @@

 pub struct Loader {
     pub parser_lib_path: PathBuf,
-    languages_by_id: Vec<(PathBuf, OnceCell<Language>, Option<Vec<PathBuf>>)>,
+    languages_by_id: Vec<LanguageEntry>,
     language_configurations: Vec<LanguageConfiguration<'static>>,
     language_configuration_ids_by_file_type: HashMap<String, Vec<usize>>,
     language_configuration_in_current_path: Option<usize>,
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -621,6 +621,8 @@
     wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
 }

+str
 pub struct CompileConfig<'a> {
     pub src_path: &'a Path,
     pub header_paths: Vec<&'a Path>,
```

## Cursor Position

```tree-sitter/crates/loader/src/loader.rs
    sanitize_build: bool,
    force_rebuild: bool,

    #[cfg(feature = "wasm")]
    wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
}

str
// ^[CURSOR_POSITION]
pub struct CompileConfig<'a> {
    pub src_path: &'a Path,
    pub header_paths: Vec<&'a Path>,
    pub parser_path: PathBuf,
    pub scanner_path: Option<PathBuf>,
    pub external_files: Option<&'a [PathBuf]>,
```

## Expected Patch

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -621,6 +621,8 @@
     wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
 }

-str
+struct LanguageEntry {
+    path: PathBuf,
+    language: OnceCell<Language>,
+    external_files: Option<Vec<PathBuf>>,
+}
+
 pub struct CompileConfig<'a> {
     pub src_path: &'a Path,
     pub header_paths: Vec<&'a Path>,
```

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -621,6 +621,8 @@
     wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
 }

-str
+struct LanguageEntry {
+    path: PathBuf,
+    language: OnceCell<Language>,
+    dependencies: Option<Vec<PathBuf>>,
+}
+
 pub struct CompileConfig<'a> {
     pub src_path: &'a Path,
     pub header_paths: Vec<&'a Path>,
```


```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -621,6 +621,8 @@
     wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
 }

-str
+struct LanguageEntry {
+    path: PathBuf,
+    language: OnceCell<Language>,
+    extra_files: Option<Vec<PathBuf>>,
+}
+
 pub struct CompileConfig<'a> {
     pub src_path: &'a Path,
     pub header_paths: Vec<&'a Path>,
```

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -621,6 +621,8 @@
     wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
 }

-str
+struct LanguageEntry(PathBuf, OnceCell<Language>, Option<Vec<PathBuf>>);
+
 pub struct CompileConfig<'a> {
     pub src_path: &'a Path,
     pub header_paths: Vec<&'a Path>,
```
