+++
repository_url = "git@github.com:zed-industries/zed"
revision = "be5763632dccb33470ca233c36ccd9e5e790e3b2"
+++

This prediction requires the model to see the `project::Event` enum.

## Edit History

```diff
--- a/crates/edit_prediction/src/edit_prediction.rs
+++ b/crates/edit_prediction/src/edit_prediction.rs
@@ -1035,7 +1035,7 @@
                     project_state.recent_paths.push_front(path);
                 }
             }
-            project::Event::DiagnosticsUpdated { .. } => {
+            project::Event::Disk { .. } => {
                 if cx.has_flag::<EditPredictionJumpsFeatureFlag>() {
                     self.refresh_prediction_from_diagnostics(
                         project,
```

## Cursor Position

```crates/edit_prediction/src/edit_prediction.rs
                    {
                        project_state.recent_paths.remove(ix);
                    }
                    project_state.recent_paths.push_front(path);
                }
            }
            project::Event::Disk { .. } => {
                //              ^[CURSOR_POSITION]
                if cx.has_flag::<EditPredictionJumpsFeatureFlag>() {
                    self.refresh_prediction_from_diagnostics(
                        project,
```

## Expected Patch

```diff
--- a/crates/edit_prediction/src/edit_prediction.rs
+++ b/crates/edit_prediction/src/edit_prediction.rs
@@ -1032,10 +1032,10 @@
                     project_state.recent_paths.push_front(path);
                 }
             }
-            project::Event::Disk { .. } => {
+            project::Event::DiskBasedDiagnosticsFinished { .. } => {
                 if cx.has_flag::<EditPredictionJumpsFeatureFlag>() {
                     self.refresh_prediction_from_diagnostics(
                         project,
```

```diff
--- a/crates/edit_prediction/src/edit_prediction.rs
+++ b/crates/edit_prediction/src/edit_prediction.rs
@@ -1032,10 +1032,10 @@
                     project_state.recent_paths.push_front(path);
                 }
             }
-            project::Event::Disk { .. } => {
+            project::Event::DiskBasedDiagnosticsStarted { .. } => {
                 if cx.has_flag::<EditPredictionJumpsFeatureFlag>() {
                     self.refresh_prediction_from_diagnostics(
                         project,
```
