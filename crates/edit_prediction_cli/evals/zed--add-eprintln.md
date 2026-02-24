+++
repository_url = "git@github.com:zed-industries/zed"
revision = "780a87dd98f26816876d12e2728933b17faca78d"
+++

## Edit History

```diff
--- a/crates/edit_prediction_ui/src/rate_prediction_modal.rs
+++ b/crates/edit_prediction_ui/src/rate_prediction_modal.rs
@@ -206,6 +206,7 @@
         self.select_next_edit(&Default::default(), window, cx);
         self.confirm(&Default::default(), window, cx);

+        epr
         cx.notify();
     }

```

## Cursor Position

```crates/edit_prediction_ui/src/rate_prediction_modal.rs
        let current_completion = self
            .active_prediction
            .as_ref()
            .map(|completion| completion.prediction.clone());
        self.select_completion(current_completion, false, window, cx);
        self.select_next_edit(&Default::default(), window, cx);
        self.confirm(&Default::default(), window, cx);

        epr
        // ^[CURSOR_POSITION]
        cx.notify();
    }

    pub fn thumbs_down_active(
        &mut self,
        _: &ThumbsDownActivePrediction,
        window: &mut Window,
```

## Expected Patch

```diff
--- a/crates/edit_prediction_ui/src/rate_prediction_modal.rs
+++ b/crates/edit_prediction_ui/src/rate_prediction_modal.rs
@@ -201,16 +201,16 @@
         self.confirm(&Default::default(), window, cx);

-        epr
+        eprintln!("");
#                   ^[CURSOR_POSITION]
         cx.notify();
     }
```
