Basic idea:

Run the `copilot-node-server` as an LSP
Reuse our LSP code to use it

Issues:
- Re-use our github authentication for copilot - ??
- Integrate Copilot suggestions with `SuggestionMap`



THE PLAN:
- Copilot crate.
- Instantiated with a project / listens to them
- Listens to events from the project about adding worktrees
- Manages the copilot language servers per worktree
- Editor <-?-> Copilot


From anotonio in Slack:
- soooo regarding copilot i was thinking… if it doesn’t really behave like a language server (but they implemented like that because of the protocol, etc.), it might be nice to just have a singleton that is not even set when we’re signed out. when we sign in, we set the global. then, the editor can access the global (e.g. cx.global::<Option<Copilot>>) after typing some character (and with some debouncing mechanism). the Copilot struct could hold a lsp::LanguageServer and then our job is to write an adapter that can then be used to start the language server, but it’s kinda orthogonal to the language servers we store in the project. what do you think?
