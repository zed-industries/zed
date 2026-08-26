+++
repository_url = "git@github.com:zed-industries/zed"
revision = "b7090c9fae7390a82021b994994c0f587744d96c"
+++

This example shows the model's preference for making conservative predictions, and ability to place
the cursor within the predicted output.

## Edit History

```diff
--- a/crates/edit_prediction_ui/src/rate_prediction_modal.rs
+++ b/crates/edit_prediction_ui/src/rate_prediction_modal.rs
@@ -144,7 +144,7 @@
     fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {
+        epr
         let next_index = self
             .ep_store
             .read(cx)
```

## Cursor Position

```crates/edit_prediction_ui/src/rate_prediction_modal.rs
    fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {
        epr
        // ^[CURSOR_POSITION]
        let next_index = self
            .ep_store
            .read(cx)
            .shown_predictions()
            .skip(self.selected_index)
            .enumerate()
            .skip(1) // Skip straight to the next item
```

## Expected Patch

```diff
--- a/crates/edit_prediction_ui/src/rate_prediction_modal.rs
+++ b/crates/edit_prediction_ui/src/rate_prediction_modal.rs
@@ -144,14 +144,14 @@
     fn select_next_edit(&mut self, _: &NextEdit, _: &mut Window, cx: &mut Context<Self>) {
-        epr
+        eprintln!("");
#                   ^[CURSOR_POSITION]
         let next_index = self
             .ep_store
             .read(cx)
             .shown_predictions()
             .skip(self.selected_index)
             .enumerate()
             .skip(1) // Skip straight to the next item
```
