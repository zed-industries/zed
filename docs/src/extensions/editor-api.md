# Editor & UI Extension API

Starting in Zed extension API version 0.8.0, extensions can interact directly with Zed's editor, user interface, background processes, TCP streams, and persistent consoles. This enables interactive development tools, REPLs, diagnostics engines, and custom workflows.

## Manifest Configuration

Declare custom commands and subscriptions in your `extension.toml`:

```toml
id = "my-extension"
name = "My Extension"
version = "0.1.0"
schema_version = 1
authors = ["Author Name <author@example.com>"]
description = "Interactive workflow extension"

# Register commands executable from the command palette or keymaps
[commands.eval-selection]
title = "My Extension: Evaluate Selection"

[commands.restart-session]
title = "My Extension: Restart Session"

# Subscribe to editor and environment lifecycle events
events = [
    "active-editor-changed",
    "buffer-saved",
    "selection-changed",
    "settings-changed"
]
```

## Running Commands

Zed exposes registered commands through the command palette ("extensions: run command") and via keybindings using `zed::RunExtensionCommand`:

```json
[
  {
    "bindings": {
      "ctrl-alt-c e": ["zed::RunExtensionCommand", { "extension": "my-extension", "command": "eval-selection" }]
    }
  }
]
```

Implement the `run_command` method in your extension:

```rust
use zed_extension_api::{self as zed, CommandContext, Extension};

struct MyExtension;

impl Extension for MyExtension {
    fn new() -> Self {
        Self
    }

    fn run_command(&mut self, command_id: String, context: CommandContext) -> Result<(), String> {
        if command_id == "eval-selection" {
            if let Some(editor) = context.editor {
                zed::ui::notify(
                    zed::ui::NotificationLevel::Info,
                    &format!("Evaluating in {}", editor.path),
                )?;
            }
        }
        Ok(())
    }
}

zed::register_extension!(MyExtension);
```

## Editor API

Interact with active text editors, modify content, update cursor selections, and render inlays and decorations.

```rust
use zed_extension_api::editor::{self, Decoration, DecorationStyle, TextEdit};

// Query active editor state
if let Some(state) = editor::active_editor() {
    println!("Path: {}, Language: {}", state.path, state.language);
    for sel in &state.selections {
        println!("Selection: {}..{}", sel.start, sel.end);
    }

    // Apply text edits
    editor::apply_edits(
        &state.path,
        vec![TextEdit {
            start: 0,
            end: 0,
            text: "// Added header\n".to_string(),
        }],
    )?;

    // Set cursor selection
    editor::set_selections(&state.path, vec![0..0])?;

    // Render inline and block decorations
    editor::set_decorations(
        &state.path,
        "eval-results",
        vec![
            Decoration::InlineAfter {
                range: 0..10,
                text: " => 42".to_string(),
                style: DecorationStyle::Success,
            },
            Decoration::Block {
                row: 5,
                text: "Execution finished in 12ms".to_string(),
                style: DecorationStyle::Info,
            },
            Decoration::Highlight {
                range: 0..10,
                style: DecorationStyle::Warning,
            },
            Decoration::Gutter {
                row: 5,
                style: DecorationStyle::Success,
            },
        ],
    )?;

    // Clear decorations by key
    editor::clear_decorations(Some(&state.path), "eval-results")?;
}
```

## UI API

Display toasts, solicit user input, show status bar items, and copy to the system clipboard.

```rust
use zed_extension_api::ui::{self, NotificationLevel, QuickPickItem};

// Show a notification toast
ui::notify(NotificationLevel::Info, "Process completed successfully")?;

// Copy text to clipboard
ui::write_clipboard("Output result")?;

// Open a file at a specific line and column
ui::open_path("/path/to/project/src/main.rs", Some(42), Some(10))?;

// Update a status bar indicator (None clears the indicator)
ui::set_status("session-status", Some("Connected".to_string()))?;

// Request user selection via quick pick
ui::quick_pick(
    101, // Request ID
    vec![
        QuickPickItem {
            label: "Option 1".to_string(),
            detail: Some("First option detail".to_string()),
        },
        QuickPickItem {
            label: "Option 2".to_string(),
            detail: None,
        },
    ],
    Some("Choose an action".to_string()),
)?;

// Prompt user for string input
ui::input_box(102, "Enter connection port:", Some("7888".to_string()))?;
```

Handle quick pick and input box responses by implementing `ui_response`:

```rust
impl Extension for MyExtension {
    fn ui_response(&mut self, request_id: u64, selected: Option<String>) -> Result<(), String> {
        if let Some(value) = selected {
            println!("User selected or entered: {value} for request {request_id}");
        }
        Ok(())
    }
}
```

## Interactive Console API

Open dedicated interactive REPL/console tabs with read-only output buffers and an interactive prompt input.

```rust
use zed_extension_api::console::{self, ConsoleOutputKind};

// Open a console pane
console::open_console("repl-1", "Interactive Session")?;

// Set input prompt placeholder text
console::set_prompt("repl-1", "user=> ")?;

// Append styled output
console::append("repl-1", "Session started on port 7888\n", ConsoleOutputKind::Output)?;
console::append("repl-1", "Warning: deprecated API\n", ConsoleOutputKind::Error)?;
console::append("repl-1", "=> 42\n", ConsoleOutputKind::Result)?;

// Clear console history
console::clear("repl-1")?;
```

Handle inputs sent from the console input field:

```rust
impl Extension for MyExtension {
    fn console_input(&mut self, id: String, text: String) -> Result<(), String> {
        println!("Input received from console {id}: {text}");
        // Echo back result
        console::append(&id, &format!("=> echoed: {text}\n"), ConsoleOutputKind::Result)?;
        Ok(())
    }
}
```

## Background Processes

Spawn external processes with streaming output callbacks and cancellation support. Requires the `process-exec` capability in `extension.toml`.

```rust
use zed_extension_api::process;

let handle = process::spawn(
    "node".to_string(),
    vec!["server.js".to_string()],
    vec![("PORT".to_string(), "3000".to_string())],
    Some("/path/to/project".to_string()),
)?;

// Later, terminate the process if needed
process::kill(handle)?;
```

Process outputs and exits are received asynchronously:

```rust
impl Extension for MyExtension {
    fn process_output(&mut self, handle: u64, is_stderr: bool, text: String) -> Result<(), String> {
        if is_stderr {
            eprintln!("[{handle}] STDERR: {text}");
        } else {
            println!("[{handle}] STDOUT: {text}");
        }
        Ok(())
    }

    fn process_exit(&mut self, handle: u64, exit_code: Option<i32>) -> Result<(), String> {
        println!("Process {handle} exited with code: {exit_code:?}");
        Ok(())
    }
}
```

## Asynchronous TCP Streaming

Connect to local TCP servers (e.g. background services or language environments) without blocking the extension runner. Requires the `tcp-connect` capability.

```rust
use zed_extension_api::tcp::TcpStream;

let stream = TcpStream::connect("127.0.0.1:7888")?;
// Subscribe to background async reads
let stream_id = stream.subscribe()?;

// Send data to the stream
stream.write(b"(:eval \"(+ 1 2)\")\n")?;
```

Data arriving on the TCP socket is delivered to `tcp_data`:

```rust
impl Extension for MyExtension {
    fn tcp_data(&mut self, stream_id: u64, data: Vec<u8>) -> Result<(), String> {
        println!("Received {} bytes on stream {stream_id}", data.len());
        Ok(())
    }

    fn tcp_closed(&mut self, stream_id: u64, error: Option<String>) -> Result<(), String> {
        println!("Stream {stream_id} closed: {error:?}");
        Ok(())
    }
}
```

## Lifecycle Events

When events are declared in `extension.toml`, Zed notifies the extension of relevant state changes:

```rust
impl Extension for MyExtension {
    fn on_event(&mut self, kind: String, payload_json: String) -> Result<(), String> {
        match kind.as_str() {
            "active-editor-changed" => println!("Active editor changed: {payload_json}"),
            "buffer-saved" => println!("Buffer saved: {payload_json}"),
            "selection-changed" => println!("Cursor moved: {payload_json}"),
            "settings-changed" => println!("Settings updated: {payload_json}"),
            _ => {}
        }
        Ok(())
    }
}
```

## Extension Settings

Retrieve custom extension settings configured under `"extensions_settings"` in Zed's `settings.json`:

```rust
use zed_extension_api::settings;

let settings_json = settings::extension_settings()?;
println!("Extension settings JSON: {settings_json}");
```
