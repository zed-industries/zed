# Agent Panel

The Agent Panel is where you interact with AI agents that can read, write, and run code in your project. Use it for code generation, refactoring, debugging, documentation, and general questions.

Open it with `agent: new thread` from [the Command Palette](../getting-started.md#command-palette) or click the ✨ icon in the status bar.

## Getting Started {#getting-started}

If you're using the Agent Panel for the first time, you need to have at least one LLM provider or external agent configured.
You can do that by:

1. [subscribing to our Pro plan](https://zed.dev/pricing), so you have access to our hosted models
2. [using your own API keys](./llm-providers.md#use-your-own-keys), either from model providers like Anthropic or model gateways like OpenRouter.
3. using an [external agent](./external-agents.md) like [Gemini CLI](./external-agents.md#gemini-cli) or [Claude Code](./external-agents.md#claude-code)

## Overview {#overview}

With an LLM provider or external agent configured, type in the message editor and press `enter` to submit. Expand the editor with {#kb agent::ExpandMessageEditor} if you need more room.

Responses stream in with indicators showing [which tools](./tools.md) the model is using. The sections below cover what you can do from here.

> Note that for external agents, like [Gemini CLI](./external-agents.md#gemini-cli) or [Claude Code](./external-agents.md#claude-code), some of the features outlined below may _not_ be supported—for example, _restoring threads from history_, _checkpoints_, _token usage display_, and others. Their availability varies depending on the agent.

### Creating New Threads {#new-thread}

By default, the Agent Panel uses Zed's first-party agent.

To choose another agent, go to the plus button in the top-right of the Agent Panel and pick either one of the [external agents](./external-agents.md) installed out of the box or a new [Text Thread](./text-threads.md).

### Editing Messages {#editing-messages}

Any message that you send to the model is editable.
You can click on the card that contains your message and re-submit it with an adjusted prompt and/or new pieces of context.

### Queueing Messages

Messages sent while the agent is in the generating state get, by default, queued.

For the Zed agent, queued messages get sent at the next turn boundary, which is usually between a tool call and a response, whereas for external agents, the message gets sent at the end of the generation.

You can edit or remove (an individual or all) queued messages.
You can also still interrupt the agent immediately if you want by either clicking on the stop button or by clicking the "Send Now" (double-enter) on a queued message.

### Checkpoints {#checkpoints}

Every time the model performs an edit, you should see a "Restore Checkpoint" button at the top of your message, allowing you to return your code base to the state it was in prior to that message.

The checkpoint button appears even if you interrupt the thread midway through an edit, as this is likely a moment when you've identified that the agent is not heading in the right direction and you want to revert back.

### Context Menu {#context-menu}

Right-click on any agent response in the thread view to access a context menu with the following actions:

- **Copy Selection**: Copies the currently selected text as Markdown (available when text is selected).
- **Copy This Agent Response**: Copies the full text of the agent response you right-clicked on.
- **Scroll to Top / Scroll to Bottom**: Scrolls to the beginning or end of the thread, depending on your current position.
- **Open Thread as Markdown**: Opens the entire thread as a Markdown file in a new tab.

### Navigating the Thread {#navigating-the-thread}

In long conversations, use the scroll button at the bottom of the panel to jump to your most recent prompt.

### Navigating History {#navigating-history}

To quickly navigate through recently updated threads, use the {#kb agent::ToggleNavigationMenu} binding when focused on the panel's editor, or click the menu icon button at the top right of the panel.
Doing that will open a dropdown that shows you your six most recently updated threads.

To view all historical conversations, reach for the `View All` option from within the same menu or via the {#kb agent::OpenHistory} binding.

Thread titles are auto-generated based on the conversation content. To regenerate a title, open the ellipsis menu in the top right of the panel and select "Regenerate Thread Title".

### Following the Agent {#following-the-agent}

Follow the agent as it reads and edits files by clicking the crosshair icon at the bottom left of the panel. Your editor will jump to each file the agent touches.

You can also hold `cmd`/`ctrl` when submitting a message to automatically follow.

### Get Notified {#get-notified}

If you send a prompt to the Agent and then put Zed in the background, you can choose to be notified when its generation wraps up via:

- a visual notification that appears in the top right of your screen
- a sound notification

These notifications can be used together or individually, and you can use the `agent.notify_when_agent_waiting` and `agent.play_sound_when_agent_done` settings keys to customize that, including turning both off entirely.

### Reviewing Changes {#reviewing-changes}

Once the agent has made changes to your project, the panel will surface which files, how many of them, and how many lines have been edited.

To see which files specifically have been edited, expand the accordion bar that shows up right above the message editor or click the `Review Changes` button ({#kb agent::OpenAgentDiff}), which opens a special multi-buffer tab with all changes.

You can accept or reject each individual change hunk, or the whole set of changes made by the agent.

Edit diffs also appear in singleton buffers.
If your active tab had edits made by the AI, you'll see diffs with the same accept/reject controls as in the multi-buffer.
You can turn this off, though, through the `agent.single_file_review` setting.

## Adding Context {#adding-context}

The agent can search your codebase to find relevant context, but providing it explicitly improves response quality and reduces latency.

Add context by typing `@` in the message editor. You can mention files, directories, symbols, previous threads, rules files, and diagnostics.

Copying images and pasting them in the panel's message editor is also supported.

When you paste multi-line code selections copied from a buffer, Zed automatically formats them as @-mentions with the file context.
To paste content without this automatic formatting, use {#kb agent::PasteRaw} to paste raw text directly.

### Selection as Context

Additionally, you can also select text in a buffer and add it as context by using the {#kb agent::AddSelectionToThread} keybinding, running the {#action agent::AddSelectionToThread} action, or choosing the "Selection" item in the `@` menu.

## Token Usage {#token-usage}

Zed surfaces how many tokens you are consuming for your currently active thread near the profile selector in the panel's message editor.

Once you approach the model's context window, a banner appears above the message editor suggesting to start a new thread with the current one summarized and added as context.
You can also do this at any time with an ongoing thread via the "Agent Options" menu on the top right, where you'll see a "New from Summary" button, as well as simply @-mentioning a past thread in a new one..

## Changing Models {#changing-models}

After you've configured your LLM providers—either via [a custom API key](./llm-providers.md) or through [Zed's hosted models](./models.md)—you can switch between their models by clicking on the model selector on the message editor or by using the {#kb agent::ToggleModelSelector} keybinding.

> The same model can be offered via multiple providers - for example, Claude Sonnet 4 is available via Zed Pro, OpenRouter, Anthropic directly, and more.
> Make sure you've selected the correct model **_provider_** for the model you'd like to use, delineated by the logo to the left of the model in the model selector.

### Favoriting Models

You can mark specific models as favorites either through the model selector, by clicking on the star icon button that appears as you hover the model, or through your settings via the `agent.favorite_models` settings key.

Cycle through your favorites with {#kb agent::CycleFavoriteModels} without opening the model selector.

## Using Tools {#using-tools}

The Agent Panel supports tool calling, which enables agentic editing.
Zed includes [built-in tools](./tools.md) for searching your codebase, editing files, running terminal commands, and fetching web content.

You can also extend the set of available tools via [MCP Servers](./mcp.md).

### Profiles {#profiles}

Profiles act as a way to group tools.
Zed offers three built-in profiles and you can create as many custom ones as you want.

#### Built-in Profiles {#built-in-profiles}

- `Write`: A profile with tools to allow the LLM to write to your files and run terminal commands. This one essentially has all built-in tools turned on.
- `Ask`: A profile with read-only tools. Best for asking questions about your code base without the concern of the agent making changes.
- `Minimal`: A profile with no tools. Best for general conversations with the LLM where no knowledge of your code base is necessary.

You can explore the exact tools enabled in each profile by clicking on the profile selector button > `Configure` button > the one you want to check out.

Alternatively, you can also use either the command palette, by running {#action agent::ManageProfiles}, or the keybinding directly, {#kb agent::ManageProfiles}, to have access to the profile management modal.

Use {#kb agent::CycleModeSelector} to switch between profiles without opening the modal.

#### Custom Profiles {#custom-profiles}

You can also create a custom profile through the Agent Profile modal.
From there, you can choose to `Add New Profile` or fork an existing one with a custom name and your preferred set of tools.

It's also possible to override built-in profiles.
In the Agent Profile modal, select a built-in profile, navigate to `Configure Tools`, and rearrange the tools you'd like to keep or remove.

Zed will store this profile in your settings using the same profile name as the default you overrode.

All custom profiles can be edited via the UI or by hand under the `agent.profiles` key in your `settings.json` file.

To delete a custom profile, open the Agent Profile modal, select the profile you want to remove, and click the delete button.

### Tool Approval

Zed's Agent Panel provides the `agent.tool_permissions.default` setting to control tool approval behavior:

- `"confirm"` (default) - Prompts for approval before running any tool action
- `"allow"` - Auto-approves tool actions without prompting
- `"deny"` - Blocks all tool actions

You can change this in either your `settings.json` or via the Agent Panel's settings view.

Even with `default: "allow"`, you can configure per-tool rules using `always_deny` and `always_confirm` patterns to maintain safety guardrails for specific commands. For example, you can auto-approve most actions while still requiring confirmation for `sudo` commands.

> For `copy_path` and `move_path` tools, patterns are matched independently against both the source and destination paths. A deny match on either path blocks the operation. See [Per-tool Permission Rules](./agent-settings.md#per-tool-permission-rules) for details and examples.

You can also give more granular permissions through the dropdown that appears in the UI whenever the agent requests authorization to run a tool call.

### Model Support {#model-support}

Tool calling needs to be individually supported by each model and model provider.
Therefore, despite the presence of tools, some models may not have the ability to pick them up yet in Zed.
You should see a "No tools" label if you select a model that falls into this case.

All [Zed's hosted models](./models.md) support tool calling out-of-the-box.

### MCP Servers {#mcp-servers}

Similarly to the built-in tools, some models may not support all tools included in a given MCP Server.
Zed's UI will inform you about this via a warning icon that appears close to the model selector.

## Text Threads {#text-threads}

["Text Threads"](./text-threads.md) present your conversation with the LLM in a different format—as raw text.
With text threads, you have full control over the conversation data.
You can remove and edit responses from the LLM, swap roles, and include more context earlier in the conversation.

Text threads are Zed's original assistant panel format, preserved for users who want direct control over conversation data.
Autonomous code editing (where the agent writes to files) is only available in the default thread format, not text threads.

## Errors and Debugging {#errors-and-debugging}

In case of any error or strange LLM response behavior, the best way to help the Zed team debug is by reaching for the `agent: open thread as markdown` action and attaching that data as part of your issue on GitHub.

You can also open threads as Markdown by clicking on the file icon button, to the right of the thumbs down button, when focused on the panel's editor.

## Feedback {#feedback}

You can rate agent responses to help improve Zed's system prompt and tools.

> Note that rating responses will send your data related to that response to Zed's servers.
> See [AI Improvement](./ai-improvement.md) and [Privacy and Security](./privacy-and-security.md) for more information about Zed's approach to AI improvement, privacy, and security.
> **_If you don't want data persisted on Zed's servers, don't rate_**. We will not collect data for improving our Agentic offering without you explicitly rating responses.

The best way you can help influence the next change to Zed's system prompt and tools is by rating the LLM's response via the thumbs up/down buttons at the end of every response. In case of a thumbs down, a new text area will show up where you can add more specifics about what happened.

You can provide feedback on the thread at any point after the agent responds, and multiple times within the same thread.
