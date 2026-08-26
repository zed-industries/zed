+++
repository_url = "https://github.com/microsoft/vscode"
revision = "e28a92fc1fbe9de11eca2f8ad19899334bff8525"
+++

This prediction requires the model to see the `IDiffComputationResult` type definition.

## Edit History

```diff
--- a/src/vs/editor/browser/widget/diffEditorWidget.ts
+++ b/src/vs/editor/browser/widget/diffEditorWidget.ts
@@ -1117,6 +1117,7 @@
 				&& currentModifiedModel === this._modifiedEditor.getModel()
 			) {
 				this._setState(editorBrowser.DiffEditorState.DiffComputed);
+				console.log("did quit:")
 				this._diffComputationResult = result;
 				this._updateDecorationsRunner.schedule();
 				this._onDidUpdateDiff.fire();
```

## Cursor Position

```src/vs/editor/browser/widget/diffEditorWidget.ts
			if (currentToken === this._diffComputationToken
				&& currentOriginalModel === this._originalEditor.getModel()
				&& currentModifiedModel === this._modifiedEditor.getModel()
			) {
				this._setState(editorBrowser.DiffEditorState.DiffComputed);
				console.log("did quit:")
				//                    ^[CURSOR_POSITION]
				this._diffComputationResult = result;
				this._updateDecorationsRunner.schedule();
				this._onDidUpdateDiff.fire();
			}
```

## Expected Patch

```diff
--- a/src/vs/editor/browser/widget/diffEditorWidget.ts
+++ b/src/vs/editor/browser/widget/diffEditorWidget.ts
@@ -1115,10 +1115,10 @@
 			if (currentToken === this._diffComputationToken
 				&& currentOriginalModel === this._originalEditor.getModel()
 				&& currentModifiedModel === this._modifiedEditor.getModel()
 			) {
 				this._setState(editorBrowser.DiffEditorState.DiffComputed);
-				console.log("did quit:")
+				console.log("did quit:", result.quitEarly)
 				this._diffComputationResult = result;
 				this._updateDecorationsRunner.schedule();
 				this._onDidUpdateDiff.fire();
 			}
```
