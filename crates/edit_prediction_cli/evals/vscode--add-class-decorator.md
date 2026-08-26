+++
repository_url = "https://github.com/microsoft/vscode"
revision = "6f6e26fcdf0a7ca5084e0da284cd7a5b2d41ae4d"
+++

## Edit History

```diff
--- a/src/vs/workbench/api/common/extHostTypes.ts
+++ b/src/vs/workbench/api/common/extHostTypes.ts
@@ -18,6 +18,14 @@ import { FileSystemProviderErrorCode, markAsFileSystemProviderError } from 'vs/
 import type * as vscode from 'vscode';

+function es5ClassCompat(target: Function): any {
+	///@ts-expect-error
+	function _() { return Reflect.construct(target, arguments, this.constructor); }
+	Object.defineProperty(_, 'name', Object.getOwnPropertyDescriptor(target, 'name')!);
+	Object.setPrototypeOf(_, target);
+	Object.setPrototypeOf(_.prototype, target.prototype);
+	return _;
+}
+
+@es5ClassCompat
 export class Disposable {
--- a/src/vs/workbench/api/common/extHostTypes.ts
+++ b/src/vs/workbench/api/common/extHostTypes.ts
@@ -50,6 +58,7 @@ export class Disposable {
 	}
 }

+@es5ClassCompat
 export class Position {

 	static Min(...positions: Position[]): Position {
--- a/src/vs/workbench/api/common/extHostTypes.ts
+++ b/src/vs/workbench/api/common/extHostTypes.ts
@@ -220,6 +229,7 @@ export class Position {
 	}
 }

+@es5ClassCompat
 export class Range {

 	static isRange(thing: any): thing is vscode.Range {
```

## Cursor Position

```src/vs/workbench/api/common/extHostTypes.ts
	Prepend = 3
}

export class TextEdit {
// <[CURSOR_POSITION]

	static isTextEdit(thing: any): thing is TextEdit {
		if (thing instanceof TextEdit) {
			return true;
```

## Expected Patch

```diff
--- a/src/vs/workbench/api/common/extHostTypes.ts
+++ b/src/vs/workbench/api/common/extHostTypes.ts
@@ -475,6 +485,7 @@ export enum EnvironmentVariableMutatorType {
 	Prepend = 3
 }

+@es5ClassCompat
 export class TextEdit {

 	static isTextEdit(thing: any): thing is TextEdit {
```
