+++
repository_url = "git@github.com:tree-sitter/tree-sitter"
revision = "17e3c7a5c56527a179fa6e37ce7ee934493e5047"
+++

## Edit History

```diff
--- a/crates/loader/src/loader.rs
+++ b/crates/loader/src/loader.rs
@@ -729,15 +729,16 @@
             ));
         }
         for parser_container_dir in &config.parser_directories {
-            if let Ok(entries) = fs::read_dir(parser_container_dir) {
-                for entry in entries {
-                    let entry = entry.map_err(|e| LoaderError::IO(IoError::new(e, None)))?;
-                    if let Some(parser_dir_name) = entry.file_name().to_str() {
-                        if parser_dir_name.starts_with("tree-sitter-") {
-                            self.find_language_configurations_at_path(
-                                &parser_container_dir.join(parser_dir_name),
-                                false,
-                            )
+            match fs::read_dir(parser_container_dir) {
+                Ok(entries) => {
+                    for entry in entries {
+                        let entry = entry.map_err(|e| LoaderError::IO(IoError::new(e, None)))?;
+                        if let Some(parser_dir_name) = entry.file_name().to_str() {
+                            if parser_dir_name.starts_with("tree-sitter-") {
+                                self.find_language_configurations_at_path(
+                                    &parser_container_dir.join(parser_dir_name),
+                                    false,
+                                )
                             .ok();
                         }
                     }
--- a/crates/loader/src/loader.rs
+++ b/crates/loader/src/loader.rs
@@ -739,7 +739,8 @@
                                     &parser_container_dir.join(parser_dir_name),
                                     false,
                                 )
-                            .ok();
+                                .ok();
+                            }
                         }
                     }
                 }
```

## Cursor Position

```crates/loader/src/loader.rs
                        if let Some(parser_dir_name) = entry.file_name().to_str() {
                            if parser_dir_name.starts_with("tree-sitter-") {
                                self.find_language_configurations_at_path(
                                    &parser_container_dir.join(parser_dir_name),
                                    false,
                                )
                                .ok();
                            }
//                           ^[CURSOR_POSITION]
                        }
                    }
                }
            }
        }
```

## Expected Patch

```diff
--- a/crates/loader/src/loader.rs
+++ b/crates/loader/src/loader.rs
@@ -736,13 +736,13 @@
                         if let Some(parser_dir_name) = entry.file_name().to_str() {
                             if parser_dir_name.starts_with("tree-sitter-") {
                                 self.find_language_configurations_at_path(
                                     &parser_container_dir.join(parser_dir_name),
                                     false,
                                 )
                                 .ok();
                             }
                         }
                     }
                 }
+                Err(error) => {}
#                               ^[CURSOR_POSITION]
             }
         }
```

```diff
--- a/crates/loader/src/loader.rs
+++ b/crates/loader/src/loader.rs
@@ -736,13 +736,13 @@
                         if let Some(parser_dir_name) = entry.file_name().to_str() {
                             if parser_dir_name.starts_with("tree-sitter-") {
                                 self.find_language_configurations_at_path(
                                     &parser_container_dir.join(parser_dir_name),
                                     false,
                                 )
                                 .ok();
                             }
                         }
                     }
                 }
+                Err(_) => {}
#                           ^[CURSOR_POSITION]
             }
         }
```


```diff
--- a/crates/loader/src/loader.rs
+++ b/crates/loader/src/loader.rs
@@ -736,13 +736,13 @@
                         if let Some(parser_dir_name) = entry.file_name().to_str() {
                             if parser_dir_name.starts_with("tree-sitter-") {
                                 self.find_language_configurations_at_path(
                                     &parser_container_dir.join(parser_dir_name),
                                     false,
                                 )
                                 .ok();
                             }
                         }
                     }
                 }
+                Err(e) => {
+                    
#                    ^[CURSOR_POSITION]
+                }
             }
         }
```
