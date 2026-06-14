//! The GPUI-backed [`Dispatcher`] implementation: this is where the wire
//! protocol meets Zed's editor state. Each `tools/call` is satisfied by reading
//! the active workspace, editor, and buffers and shaping the result into the
//! JSON the Claude Code CLI expects (mirroring the official extensions).

use crate::server::{Dispatcher, ProtocolError, ToolDescriptor, error_codes};
use editor::{Editor, SplittableEditor};
use gpui::{AnyWindowHandle, AsyncApp, Entity, WeakEntity};
use language::{Buffer, DiagnosticSeverity};
use serde_json::{Value, json};
use std::path::PathBuf;
use text::Point;
use workspace::{OpenOptions, SaveIntent, Workspace};

/// Satisfies tool calls against a single Zed workspace.
///
/// Holds a weak handle to the workspace (so the server task never keeps the
/// window alive) plus a cheap clone of the async app context, which lets each
/// call hop onto the foreground thread to read entity state.
pub struct WorkspaceDispatcher {
    workspace: WeakEntity<Workspace>,
    window: Option<AnyWindowHandle>,
    cx: AsyncApp,
}

impl WorkspaceDispatcher {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: Option<AnyWindowHandle>,
        cx: AsyncApp,
    ) -> Self {
        Self { workspace, window, cx }
    }

    fn get_current_selection(&self, cx: &mut AsyncApp) -> Result<Value, ProtocolError> {
        let payload = self
            .workspace
            .update(cx, |workspace, cx| {
                let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
                    return json!({ "success": false, "message": "No active editor found" });
                };

                editor.update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    let cursor = editor.selections.newest::<Point>(&display_snapshot);

                    let path = editor
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .and_then(|buffer| {
                            buffer
                                .read(cx)
                                .file()
                                .and_then(|file| file.as_local())
                                .map(|file| file.abs_path(cx))
                        })
                        .map(|path| path.to_string_lossy().into_owned())
                        .unwrap_or_default();

                    let text: String = editor
                        .buffer()
                        .read(cx)
                        .snapshot(cx)
                        .text_for_range(cursor.start..cursor.end)
                        .collect();

                    json!({
                        "success": true,
                        "text": text,
                        "filePath": path,
                        "fileUrl": format!("file://{path}"),
                        "selection": {
                            "start": { "line": cursor.start.row, "character": cursor.start.column },
                            "end": { "line": cursor.end.row, "character": cursor.end.column },
                            "isEmpty": cursor.start == cursor.end,
                        }
                    })
                })
            })
            .map_err(|error| ProtocolError::internal(error.to_string()))?;

        Ok(mcp_text(payload))
    }

    fn get_workspace_folders(&self, cx: &mut AsyncApp) -> Result<Value, ProtocolError> {
        let paths = self
            .workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
            })
            .map_err(|error| ProtocolError::internal(error.to_string()))?;

        let folders = paths
            .iter()
            .map(|path| {
                let name = path.rsplit('/').next().filter(|name| !name.is_empty()).unwrap_or(path);
                json!({ "name": name, "uri": format!("file://{path}"), "path": path })
            })
            .collect::<Vec<_>>();
        let root_path = paths.first().cloned().unwrap_or_default();

        Ok(mcp_text(json!({
            "success": true,
            "folders": folders,
            "rootPath": root_path,
        })))
    }

    /// Finds the open buffer for an absolute path, if any.
    fn buffer_for_path(&self, path: &str, cx: &mut AsyncApp) -> Option<Entity<Buffer>> {
        self.workspace
            .update(cx, |workspace, cx| {
                let project = workspace.project().read(cx);
                let project_path = project.find_project_path(path, cx)?;
                project.get_open_buffer(&project_path, cx)
            })
            .ok()
            .flatten()
    }

    fn get_open_editors(&self, cx: &mut AsyncApp) -> Result<Value, ProtocolError> {
        let tabs = self
            .workspace
            .update(cx, |workspace, cx| {
                let active = workspace.active_item_as::<Editor>(cx);
                workspace
                    .items_of_type::<Editor>(cx)
                    .filter_map(|editor| {
                        let is_active = active.as_ref() == Some(&editor);
                        let editor = editor.read(cx);
                        let buffer = editor.buffer().read(cx).as_singleton()?;
                        let buffer = buffer.read(cx);
                        let path = buffer
                            .file()
                            .and_then(|file| file.as_local())
                            .map(|file| file.abs_path(cx).to_string_lossy().into_owned())?;
                        let language = buffer
                            .language()
                            .map(|language| language.name().to_string())
                            .unwrap_or_else(|| "plaintext".to_owned());
                        let line_count = buffer.text_snapshot().max_point().row + 1;
                        let label = path.rsplit('/').next().unwrap_or(&path).to_owned();
                        Some(json!({
                            "uri": format!("file://{path}"),
                            "fileName": path,
                            "label": label,
                            "languageId": language,
                            "isActive": is_active,
                            "isDirty": buffer.is_dirty(),
                            "isPinned": false,
                            "isPreview": false,
                            "isUntitled": false,
                            "lineCount": line_count,
                            "groupIndex": 0,
                            "viewColumn": 1,
                            "isGroupActive": true,
                        }))
                    })
                    .collect::<Vec<_>>()
            })
            .map_err(|error| ProtocolError::internal(error.to_string()))?;
        Ok(mcp_text(json!({ "tabs": tabs })))
    }

    fn get_diagnostics(&self, arguments: &Value, cx: &mut AsyncApp) -> Result<Value, ProtocolError> {
        let target = arguments
            .get("uri")
            .and_then(Value::as_str)
            .map(|uri| uri.strip_prefix("file://").unwrap_or(uri).to_owned());

        let diagnostics = self
            .workspace
            .update(cx, |workspace, cx| {
                let buffers = workspace
                    .project()
                    .read(cx)
                    .buffer_store()
                    .read(cx)
                    .buffers()
                    .collect::<Vec<_>>();
                let mut out = Vec::new();
                for buffer in buffers {
                    let buffer = buffer.read(cx);
                    let Some(path) = buffer
                        .file()
                        .and_then(|file| file.as_local())
                        .map(|file| file.abs_path(cx).to_string_lossy().into_owned())
                    else {
                        continue;
                    };
                    if target.as_ref().is_some_and(|target| target != &path) {
                        continue;
                    }
                    let snapshot = buffer.snapshot();
                    for entry in snapshot.diagnostics_in_range::<Point, Point>(
                        Point::new(0, 0)..snapshot.max_point(),
                        false,
                    ) {
                        out.push(json!({
                            "filePath": path,
                            "line": entry.range.start.row + 1,
                            "character": entry.range.start.column + 1,
                            "severity": severity_to_number(entry.diagnostic.severity),
                            "message": entry.diagnostic.message.clone(),
                            "source": entry.diagnostic.source.clone(),
                        }));
                    }
                }
                out
            })
            .map_err(|error| ProtocolError::internal(error.to_string()))?;

        let content = diagnostics
            .into_iter()
            .map(|diagnostic| json!({ "type": "text", "text": diagnostic.to_string() }))
            .collect::<Vec<_>>();
        Ok(json!({ "content": content }))
    }

    fn check_document_dirty(
        &self,
        arguments: &Value,
        cx: &mut AsyncApp,
    ) -> Result<Value, ProtocolError> {
        let path = required_path_field(arguments, "filePath")?;
        match self.buffer_for_path(&path, cx) {
            Some(buffer) => {
                let is_dirty = buffer.update(cx, |buffer, _| buffer.is_dirty());
                Ok(mcp_text(json!({
                    "success": true,
                    "filePath": path,
                    "isDirty": is_dirty,
                    "isUntitled": false,
                })))
            }
            None => Ok(mcp_text(
                json!({ "success": false, "message": format!("Document not open: {path}") }),
            )),
        }
    }

    /// Closes all open diff tabs (our `openDiff` views), returning the count.
    fn close_diff_tabs(&self, cx: &mut AsyncApp) -> usize {
        let Some(window) = self.window else {
            return 0;
        };
        window
            .update(cx, |_root, window, cx| {
                let Some(workspace) = self.workspace.upgrade() else {
                    return 0;
                };
                workspace.update(cx, |workspace, cx| {
                    let ids = workspace
                        .items_of_type::<SplittableEditor>(cx)
                        .map(|editor| editor.entity_id())
                        .collect::<Vec<_>>();
                    let count = ids.len();
                    for pane in workspace.panes().to_vec() {
                        for id in &ids {
                            pane.update(cx, |pane, cx| {
                                pane.close_item_by_id(*id, SaveIntent::Skip, window, cx)
                            })
                            .detach();
                        }
                    }
                    count
                })
            })
            .unwrap_or(0)
    }

    async fn open_file(&self, arguments: Value, cx: &mut AsyncApp) -> Result<Value, ProtocolError> {
        let path = required_path_field(&arguments, "filePath")?;
        let start_line = arguments.get("startLine").and_then(Value::as_u64);
        let end_line = arguments.get("endLine").and_then(Value::as_u64);
        let window =
            self.window.ok_or_else(|| ProtocolError::internal("no window available"))?;
        let open_task = window
            .update(cx, |_root, window, cx| {
                self.workspace.upgrade().map(|workspace| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.open_abs_path(
                            PathBuf::from(&path),
                            OpenOptions::default(),
                            window,
                            cx,
                        )
                    })
                })
            })
            .map_err(|error| ProtocolError::internal(error.to_string()))?
            .ok_or_else(|| ProtocolError::internal("workspace unavailable"))?;
        let item = open_task.await.map_err(|error| ProtocolError::internal(error.to_string()))?;

        // Optionally select the requested line range (1-indexed in the protocol).
        if let (Some(start), Some(end)) = (start_line, end_line) {
            window
                .update(cx, |_root, window, cx| {
                    if let Some(editor) = item.downcast::<Editor>() {
                        editor.update(cx, |editor, cx| {
                            let start = Point::new(start.saturating_sub(1) as u32, 0);
                            let end = Point::new(end as u32, 0);
                            editor.change_selections(Default::default(), window, cx, |selections| {
                                selections.select_ranges([start..end]);
                            });
                        });
                    }
                })
                .ok();
        }

        let message = match (start_line, end_line) {
            (Some(start), Some(end)) => {
                format!("Opened file and selected lines {start} to {end}")
            }
            _ => format!("Opened file: {path}"),
        };
        Ok(json!({ "content": [{ "type": "text", "text": message }] }))
    }

    async fn save_document(
        &self,
        arguments: Value,
        cx: &mut AsyncApp,
    ) -> Result<Value, ProtocolError> {
        let path = required_path_field(&arguments, "filePath")?;
        let Some(buffer) = self.buffer_for_path(&path, cx) else {
            return Ok(mcp_text(
                json!({ "success": false, "message": format!("Document not open: {path}") }),
            ));
        };
        let save_task = self
            .workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .update(cx, |project, cx| project.save_buffer(buffer, cx))
            })
            .map_err(|error| ProtocolError::internal(error.to_string()))?;
        save_task.await.map_err(|error| ProtocolError::internal(error.to_string()))?;
        Ok(mcp_text(json!({ "success": true, "filePath": path })))
    }
}

fn required_path_field(arguments: &Value, field: &str) -> Result<String, ProtocolError> {
    arguments
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| ProtocolError::new(error_codes::INVALID_REQUEST, format!("missing {field}")))
}

/// Maps Zed/LSP diagnostic severities to the numeric scale the protocol uses
/// (1 = error, 2 = warning, 3 = information, 4 = hint).
fn severity_to_number(severity: DiagnosticSeverity) -> u8 {
    match severity {
        DiagnosticSeverity::ERROR => 1,
        DiagnosticSeverity::WARNING => 2,
        DiagnosticSeverity::INFORMATION => 3,
        DiagnosticSeverity::HINT => 4,
        _ => 0,
    }
}

impl Dispatcher for WorkspaceDispatcher {
    fn tools(&self) -> Vec<ToolDescriptor> {
        let empty_object_schema = json!({
            "type": "object",
            "additionalProperties": false,
            "$schema": "http://json-schema.org/draft-07/schema#",
        });
        vec![
            ToolDescriptor {
                name: "getCurrentSelection",
                description: "Get the current text selection in the editor",
                input_schema: empty_object_schema.clone(),
            },
            ToolDescriptor {
                name: "getLatestSelection",
                description: "Get the most recent text selection (even if not in the active editor)",
                input_schema: empty_object_schema.clone(),
            },
            ToolDescriptor {
                name: "getWorkspaceFolders",
                description: "Get all workspace folders currently open in the IDE",
                input_schema: empty_object_schema.clone(),
            },
            ToolDescriptor {
                name: "getOpenEditors",
                description: "Get list of currently open files",
                input_schema: empty_object_schema.clone(),
            },
            ToolDescriptor {
                name: "closeAllDiffTabs",
                description: "Close all diff tabs in the editor",
                input_schema: empty_object_schema,
            },
            ToolDescriptor {
                name: "getDiagnostics",
                description: "Get language diagnostics (errors, warnings) from the editor",
                input_schema: json!({
                    "type": "object",
                    "additionalProperties": false,
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "properties": {
                        "uri": { "type": "string" },
                    },
                }),
            },
            ToolDescriptor {
                name: "checkDocumentDirty",
                description: "Check if a document has unsaved changes (is dirty)",
                input_schema: json!({
                    "type": "object",
                    "additionalProperties": false,
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "properties": { "filePath": { "type": "string" } },
                    "required": ["filePath"],
                }),
            },
            ToolDescriptor {
                name: "saveDocument",
                description: "Save a document with unsaved changes",
                input_schema: json!({
                    "type": "object",
                    "additionalProperties": false,
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "properties": { "filePath": { "type": "string" } },
                    "required": ["filePath"],
                }),
            },
            ToolDescriptor {
                name: "openFile",
                description: "Open a file in the editor and optionally select a range of text",
                input_schema: json!({
                    "type": "object",
                    "additionalProperties": false,
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "properties": {
                        "filePath": { "type": "string" },
                        "preview": { "type": "boolean" },
                        "startLine": { "type": "integer" },
                        "endLine": { "type": "integer" },
                        "startText": { "type": "string" },
                        "endText": { "type": "string" },
                        "makeFrontmost": { "type": "boolean" },
                    },
                    "required": ["filePath"],
                }),
            },
            ToolDescriptor {
                name: "openDiff",
                description: "Open a diff view comparing old file content with new file content",
                input_schema: json!({
                    "type": "object",
                    "additionalProperties": false,
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "properties": {
                        "old_file_path": { "type": "string" },
                        "new_file_path": { "type": "string" },
                        "new_file_contents": { "type": "string" },
                        "tab_name": { "type": "string" },
                    },
                    "required": ["old_file_path", "new_file_path", "new_file_contents", "tab_name"],
                }),
            },
        ]
    }

    async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value, ProtocolError> {
        // Reading entity state has to happen on the foreground thread; a cloned
        // `AsyncApp` lets `WeakEntity::update` marshal us there.
        let mut cx = self.cx.clone();
        match name {
            "getCurrentSelection" => self.get_current_selection(&mut cx),
            "getLatestSelection" => self.get_current_selection(&mut cx),
            "getWorkspaceFolders" => self.get_workspace_folders(&mut cx),
            "getOpenEditors" => self.get_open_editors(&mut cx),
            "getDiagnostics" => self.get_diagnostics(&arguments, &mut cx),
            "checkDocumentDirty" => self.check_document_dirty(&arguments, &mut cx),
            "openFile" => self.open_file(arguments, &mut cx).await,
            "saveDocument" => self.save_document(arguments, &mut cx).await,
            "close_tab" => {
                let closed = self.close_diff_tabs(&mut cx);
                Ok(mcp_text(json!({ "success": true, "closed": closed })))
            }
            "closeAllDiffTabs" => {
                let closed = self.close_diff_tabs(&mut cx);
                Ok(mcp_text(json!({ "closedCount": closed })))
            }
            "openDiff" => {
                crate::open_diff::open_diff(self.workspace.clone(), self.window, arguments, &mut cx)
                    .await
            }
            other => Err(ProtocolError::method_not_found(other)),
        }
    }
}

/// Wraps a tool's payload in the MCP result envelope: the payload is
/// JSON-stringified into a single text content block, exactly as the official
/// extensions do.
fn mcp_text(payload: Value) -> Value {
    json!({ "content": [{ "type": "text", "text": payload.to_string() }] })
}
