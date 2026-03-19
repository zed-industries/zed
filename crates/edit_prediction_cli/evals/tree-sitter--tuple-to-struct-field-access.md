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
@@ -619,6 +619,12 @@

     #[cfg(feature = "wasm")]
     wasm_store: Mutex<Option<tree_sitter::WasmStore>>,
 }
+
+struct LanguageEntry {
+    path: PathBuf,
+    language: OnceCell<Language>,
+    external_files: Option<Vec<PathBuf>>,
+}

 pub struct CompileConfig<'a> {
```

## Cursor Position

```tree-sitter/crates/loader/src/loader.rs
    #[must_use]
    pub fn get_all_language_configurations(&self) -> Vec<(&LanguageConfiguration, &Path)> {
        self.language_configurations
            .iter()
            .map(|c| (c, self.languages_by_id[c.language_id].0.as_ref()))
            //                                               ^[CURSOR_POSITION]
            .collect()
    }
```

## Expected Patch

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -773,7 +773,7 @@
     pub fn get_all_language_configurations(&self) -> Vec<(&LanguageConfiguration, &Path)> {
         self.language_configurations
             .iter()
-            .map(|c| (c, self.languages_by_id[c.language_id].0.as_ref()))
+            .map(|c| (c, self.languages_by_id[c.language_id].path.as_ref()))
             .collect()
     }

```
