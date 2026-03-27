+++
repository_url = "https://github.com/microsoft/vscode"
revision = "29e6da6efa2287aaa981635a475d425ff4fd5d5c"
+++

## Edit History

```diff
--- a/src/vs/workbench/contrib/debug/browser/debugCommands.ts
+++ b/src/vs/workbench/contrib/debug/browser/debugCommands.ts
@@ -304,8 +304,8 @@ CommandsRegistry.registerCommand({
 
 CommandsRegistry.registerCommand({
 	id: REVERSE_CONTINUE_ID,
-	handler: (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
-		getThreadAndRun(accessor, context, thread => thread.reverseContinue());
+	handler: async (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
+		await getThreadAndRun(accessor, context, thread => thread.reverseContinue());
 	}
 });
--- a/src/vs/workbench/contrib/debug/browser/debugCommands.ts
+++ b/src/vs/workbench/contrib/debug/browser/debugCommands.ts
@@ -311,11 +311,11 @@ CommandsRegistry.registerCommand({
 
 CommandsRegistry.registerCommand({
 	id: STEP_BACK_ID,
-	handler: (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
+	handler: async (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
 		const contextKeyService = accessor.get(IContextKeyService);
 		if (CONTEXT_DISASSEMBLY_VIEW_FOCUS.getValue(contextKeyService)) {
-			getThreadAndRun(accessor, context, (thread: IThread) => thread.stepBack('instruction'));
+			await getThreadAndRun(accessor, context, (thread: IThread) => thread.stepBack('instruction'));
 		} else {
-			getThreadAndRun(accessor, context, (thread: IThread) => thread.stepBack());
+			await getThreadAndRun(accessor, context, (thread: IThread) => thread.stepBack());
 		}
 	}
 });
--- a/src/vs/workbench/contrib/debug/browser/debugCommands.ts
+++ b/src/vs/workbench/contrib/debug/browser/debugCommands.ts
@@ -323,8 +323,8 @@ CommandsRegistry.registerCommand({
 
 CommandsRegistry.registerCommand({
 	id: TERMINATE_THREAD_ID,
-	handler: (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
-		getThreadAndRun(accessor, context, thread => thread.terminate());
+	handler: async (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
+		await getThreadAndRun(accessor, context, thread => thread.terminate());
 	}
 });
```

## Cursor Position

```src/vs/workbench/contrib/debug/browser/debugCommands.ts
	weight: KeybindingWeight.WorkbenchContrib,
	primary: isWeb ? (KeyMod.Alt | KeyCode.F10) : KeyCode.F10, // Browsers do not allow F10 to be binded so we have to bind an alternative
	when: CONTEXT_DEBUG_STATE.isEqualTo('stopped'),
	handler: (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
	//       ^[CURSOR_POSITION]
		const contextKeyService = accessor.get(IContextKeyService);
		if (CONTEXT_DISASSEMBLY_VIEW_FOCUS.getValue(contextKeyService)) {
			getThreadAndRun(accessor, context, (thread: IThread) => thread.next('instruction'));
		} else {
```

## Expected Patch

```diff
--- a/src/vs/workbench/contrib/debug/browser/debugCommands.ts
+++ b/src/vs/workbench/contrib/debug/browser/debugCommands.ts
@@ -467,10 +467,10 @@ KeybindingsRegistry.registerCommandAndKeybindingRule({
 	weight: KeybindingWeight.WorkbenchContrib,
 	primary: isWeb ? (KeyMod.Alt | KeyCode.F10) : KeyCode.F10, // Browsers do not allow F10 to be binded so we have to bind an alternative
 	when: CONTEXT_DEBUG_STATE.isEqualTo('stopped'),
-	handler: (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
+	handler: async (accessor: ServicesAccessor, _: string, context: CallStackContext | unknown) => {
 		const contextKeyService = accessor.get(IContextKeyService);
 		if (CONTEXT_DISASSEMBLY_VIEW_FOCUS.getValue(contextKeyService)) {
-			getThreadAndRun(accessor, context, (thread: IThread) => thread.next('instruction'));
+			await getThreadAndRun(accessor, context, (thread: IThread) => thread.next('instruction'));
 		} else {
-			getThreadAndRun(accessor, context, (thread: IThread) => thread.next());
+			await getThreadAndRun(accessor, context, (thread: IThread) => thread.next());
 		}
 	}
 });
```
