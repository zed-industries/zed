+++
repository_url = "https://github.com/microsoft/vscode"
revision = "b64eaf598008e2d600a81d846108f72cb37b48e2"
+++

## Edit History

```diff
--- a/src/vs/platform/window/electron-main/window.ts
+++ b/src/vs/platform/window/electron-main/window.ts
@@ -1,49 +1,50 @@
 export interface ICodeWindow extends IDisposable {
 
 	readonly onWillLoad: Event<ILoadEvent>;
 	readonly onDidSignalReady: Event<void>;
+	readonly onDidTriggerSystemContextMenu: Event<{ x: number; y: number }>;
 	readonly onDidClose: Event<void>;
 	readonly onDidDestroy: Event<void>;
 
 	readonly whenClosedOrLoaded: Promise<void>;
--- a/src/vs/platform/windows/electron-main/window.ts
+++ b/src/vs/platform/windows/electron-main/window.ts
@@ -63,60 +63,63 @@ const enum ReadyState {
 export class CodeWindow extends Disposable implements ICodeWindow {
 
 	//#region Events
 
 	private readonly _onWillLoad = this._register(new Emitter<ILoadEvent>());
 	readonly onWillLoad = this._onWillLoad.event;
 
 	private readonly _onDidSignalReady = this._register(new Emitter<void>());
 	readonly onDidSignalReady = this._onDidSignalReady.event;
 
+	private readonly _onDidTriggerSystemContextMenu = this._register(new Emitter<{ x: number; y: number }>());
+	readonly onDidTriggerSystemContextMenu = this._onDidTriggerSystemContextMenu.event;
+
 	private readonly _onDidClose = this._register(new Emitter<void>());
 	readonly onDidClose = this._onDidClose.event;
 
 	private readonly _onDidDestroy = this._register(new Emitter<void>());
 	readonly onDidDestroy = this._onDidDestroy.event;
 
 	//#endregion
--- a/src/vs/platform/windows/electron-main/windows.ts
+++ b/src/vs/platform/windows/electron-main/windows.ts
@@ -1,54 +1,55 @@
 export interface IWindowsMainService {
 
 	readonly _serviceBrand: undefined;
 
 	readonly onDidChangeWindowsCount: Event<IWindowsCountChangedEvent>;
 
 	readonly onDidOpenWindow: Event<ICodeWindow>;
 	readonly onDidSignalReadyWindow: Event<ICodeWindow>;
+	readonly onDidTriggerSystemContextMenu: Event<{ window: ICodeWindow; x: number; y: number }>;
 	readonly onDidDestroyWindow: Event<ICodeWindow>;
--- a/src/vs/platform/windows/electron-main/windowsMainService.ts
+++ b/src/vs/platform/windows/electron-main/windowsMainService.ts
@@ -160,60 +160,63 @@ interface ISingleFolderWorkspacePathToOpen extends IPathToOpen {
 export class WindowsMainService extends Disposable implements IWindowsMainService {
 
 	declare readonly _serviceBrand: undefined;
 
 	private static readonly WINDOWS: ICodeWindow[] = [];
 
 	private readonly _onDidOpenWindow = this._register(new Emitter<ICodeWindow>());
 	readonly onDidOpenWindow = this._onDidOpenWindow.event;
 
 	private readonly _onDidSignalReadyWindow = this._register(new Emitter<ICodeWindow>());
 	readonly onDidSignalReadyWindow = this._onDidSignalReadyWindow.event;
 
 	private readonly _onDidDestroyWindow = this._register(new Emitter<ICodeWindow>());
 	readonly onDidDestroyWindow = this._onDidDestroyWindow.event;
 
 	private readonly _onDidChangeWindowsCount = this._register(new Emitter<IWindowsCountChangedEvent>());
 	readonly onDidChangeWindowsCount = this._onDidChangeWindowsCount.event;
 
+	private readonly _onDidTriggerSystemContextMenu = this._register(new Emitter<{ window: ICodeWindow; x: number; y: number }>());
+	readonly onDidTriggerSystemContextMenu = this._onDidTriggerSystemContextMenu.event;
+
 	private readonly windowsStateHandler = this._register(new WindowsStateHandler(this, this.stateMainService, this.lifecycleMainService, this.logService, this.configurationService));
```

## Cursor Position

```src/vs/platform/windows/test/electron-main/windowsFinder.test.ts
	function createTestCodeWindow(options: { lastFocusTime: number; openedFolderUri?: URI; openedWorkspace?: IWorkspaceIdentifier }): ICodeWindow {
		return new class implements ICodeWindow {
			onWillLoad: Event<ILoadEvent> = Event.None;
			onDidSignalReady: Event<void> = Event.None;
			// <[CURSOR_POSITION]
			onDidClose: Event<void> = Event.None;
			onDidDestroy: Event<void> = Event.None;
			whenClosedOrLoaded: Promise<void> = Promise.resolve();
			id: number = -1;
```

## Expected Patch

```diff
--- a/src/vs/platform/windows/test/electron-main/windowsFinder.test.ts
+++ b/src/vs/platform/windows/test/electron-main/windowsFinder.test.ts
@@ -7,60 +7,61 @@ import * as assert from 'assert';
 	function createTestCodeWindow(options: { lastFocusTime: number; openedFolderUri?: URI; openedWorkspace?: IWorkspaceIdentifier }): ICodeWindow {
 		return new class implements ICodeWindow {
 			onWillLoad: Event<ILoadEvent> = Event.None;
+			onDidTriggerSystemContextMenu: Event<{ x: number; y: number }> = Event.None;
 			onDidSignalReady: Event<void> = Event.None;
 			onDidClose: Event<void> = Event.None;
 			onDidDestroy: Event<void> = Event.None;
 			whenClosedOrLoaded: Promise<void> = Promise.resolve();
 			id: number = -1;
```
