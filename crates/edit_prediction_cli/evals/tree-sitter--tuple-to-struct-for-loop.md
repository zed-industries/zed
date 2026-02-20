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
@@ -767,7 +773,7 @@
     pub fn get_all_language_configurations(&self) -> Vec<(&LanguageConfiguration, &Path)> {
         self.language_configurations
             .iter()
-            .map(|c| (c, self.languages_by_id[c.language_id].0.as_ref()))
+            .map(|c| (c, self.languages_by_id[c.language_id].path.as_ref()))
             .collect()
     }

@@ -920,7 +926,11 @@
     }

     fn language_for_id(&self, id: usize) -> LoaderResult<Language> {
-        let (path, language, externals) = &self.languages_by_id[id];
+        let LanguageEntry {
+            path,
+            language,
+            external_files,
+        } = &self.languages_by_id[id];
         language
             .get_or_try_init(|| {
                 let src_path = path.join("src");
```

## Cursor Position

```tree-sitter/crates/loader/src/loader.rs
                    let mut language_id = None;
                    for (id, (path, _, _)) in
                    //       ^[CURSOR_POSITION]
                        self.languages_by_id.iter().enumerate().skip(language_count)
                    {
                        if language_path == *path {
                            language_id = Some(id);
                        }
                    }
```

## Expected Patch

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -1542,8 +1542,7 @@
                     // Determine if a previous language configuration in this package.json file
                     // already uses the same language.
                     let mut language_id = None;
-                    for (id, (path, _, _)) in
-                        self.languages_by_id.iter().enumerate().skip(language_count)
+                    for (id, entry) in self.languages_by_id.iter().enumerate().skip(language_count)
                     {
-                        if language_path == *path {
+                        if language_path == entry.path {
                             language_id = Some(id);
                         }
                     }
```

```diff
--- a/tree-sitter/crates/loader/src/loader.rs
+++ b/tree-sitter/crates/loader/src/loader.rs
@@ -1542,8 +1542,7 @@
                     // Determine if a previous language configuration in this package.json file
                     // already uses the same language.
                     let mut language_id = None;
-                    for (id, (path, _, _)) in
+                    for (id, LanguageEntry { path, .. }) in
                         self.languages_by_id.iter().enumerate().skip(language_count)
                     {
                         if language_path == *path {
                             language_id = Some(id);
                         }
                     }
```
