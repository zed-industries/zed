# Built-in tool guidance defaults

Each `<tool>.hbs` file here is embedded into the binary and rendered as the
default `Tool guidance` system-prompt section for that tool, gated on the tool
being available in the session. A `<tool>.hbs` file in the user-global
`tool_guidance` config directory (`~/.config/zed/tool_guidance/`) shadows the
built-in default of the same name — the same shadowing model as skills.

Files are Handlebars templates rendered with the rules-template context
(`available_tools`, `model_name`, `date`, `is_windows`, `is_linux`,
`sandboxing`). Text is emitted verbatim; `{{!-- ... --}}` comments are
stripped and never reach the model.

Every file (here and in the user override directory) is importable from the
others as a partial named by its relative path without the extension,
`/`-separated on every platform (`shared/tips.hbs` → `{{> shared/tips}}` —
use `/` even on Windows). Files in subdirectories never map to a tool name,
so shared guidance can be factored out without producing stray sections.

The contract/guidance tier split of the existing tool docs has intentionally
not been done yet — the files here are currently raw dumps of the tool docs.
The `extract_builtin_tool_docs` utility test in `../tool_guidance.rs`
regenerates them as the starting point for that curation pass.
