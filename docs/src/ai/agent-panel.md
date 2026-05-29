---
title: AI Coding Agent - Zed Agent Panel
description: Use Zed's AI coding agent to generate, refactor, and debug code with tool calling, checkpoints, and multi-model support.
---

# Agent Panel

The Agent Panel is where you interact with AI agents that can read, write, and run code in your project.
It's the core of Zed's AI code editing experience — use it for code generation, refactoring, debugging, documentation, and general questions.

Open it with {#action agent::NewThread} from [the Command Palette](../getting-started.md#command-palette) or click the ✨ icon in the status bar.

## Getting Started {#getting-started}

If you're using the Agent Panel for the first time, you need to have at least one LLM provider or external agent configured.
You can do that by:

1. [subscribing to our Pro plan](https://zed.dev/pricing), so you have access to our hosted models
2. [using your own API keys](./llm-providers.md#use-your-own-keys), either from model providers like Anthropic or model gateways like OpenRouter.
3. using an [external agent](./external-agents.md) like [Gemini CLI](./external-agents.md#gemini-cli) or [Claude Agent](./external-agents.md#claude-agent)

## Overview {#overview}

With an LLM provider or external agent configured, type in the message editor and press `enter` to submit.
Expand the editor with {#kb agent::ExpandMessageEditor} if you need more room.

Responses stream in with indicators showing [which tools](./tools.md) the model is using.
The sections below cover what you can do from here.

> Note that for external agents, like [Gemini CLI](./external-agents.md#gemini-cli) or [Claude Agent](./external-agents.md#claude-agent), some of the features outlined below may _not_ be supported—for example, _restoring threads from history_, _checkpoints_, _token usage display_, and others.
> Their availability varies depending on the agent.

### Creating New Threads {#new-thread}

By default, the Agent Panel uses Zed's first-party agent.

Start a new thread with {#kb agent::NewThread}, or open the "New Thread…" menu using the agent selector button on the left (in the empty state) or the `+` icon in the top-right of the panel toolbar. You can also open that menu with {#kb agent::ToggleNewThreadMenu}.

From the "New Thread…" menu you can:

- Pick **Zed Agent** or any installed [external agent](./external-agents.md) to start a new thread with that agent.
- Choose **New From Summary** to start a fresh Zed Agent thread seeded with a summary of the current conversation — useful for compacting long threads as you approach the context window limit.
- Choose **Terminal** to open a terminal thread directly in the Agent Panel — see [Terminal Threads](#terminal-threads) for details.

{#action agent::NewExternalAgentThread} creates a new thread with the specified external agent id.

You can also start a new thread from the [Threads Sidebar](./parallel-agents.md#threads-sidebar), scoped to a specific project — see [Running Multiple Threads](./parallel-agents.md#running-multiple-threads).

### Managing Multiple Threads {#multiple-threads}

You can run multiple agent threads at once, each working independently with its own agent, context window, and conversation history. Open the Threads Sidebar with {#kb multi_workspace::ToggleWorkspaceSidebar} to see all your threads grouped by project. Click any thread to switch to it, or use the thread switcher ({#kb agents_sidebar::ToggleThreadSwitcher}) to cycle between recent threads without opening the sidebar.

Threads you're no longer working on can be archived by hovering over them in the sidebar and clicking the archive icon, or selecting them and pressing {#kb agent::ArchiveSelectedThread}. The Thread History holds all your threads across all projects, sorted chronologically, and you can restore them at any time.

If two threads might edit the same files, you can isolate one in a new Git worktree. Use the worktree picker in the title bar to pick which worktree the agent runs in, or create a new one. See [Worktree Isolation](./parallel-agents.md#worktree-isolation) for details.

For more details on the Threads Sidebar and managing multiple projects, see [Parallel Agents](./parallel-agents.md).

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

In long conversations, use the scroll arrow buttons at the bottom of the panel to jump to your most recent prompt or to the very beginning of the thread. You can also scroll the thread using arrow keys, Page Up/Down, Home/End, and Shift+Page Up/Down to jump between messages, when the thread pane is focused.

When focus is in the message editor, you can also use {#kb agent::ScrollOutputPageUp}, {#kb agent::ScrollOutputPageDown}, {#kb agent::ScrollOutputToTop}, {#kb agent::ScrollOutputToBottom}, {#kb agent::ScrollOutputLineUp}, and {#kb agent::ScrollOutputLineDown} to navigate the thread, or {#kb agent::ScrollOutputToPreviousMessage} and {#kb agent::ScrollOutputToNextMessage} to jump between your prompts.

### Thread titles {#thread-titles}

Thread titles are auto-generated based on the content of the conversation.
But you can also edit them manually by clicking the title and typing, or regenerate them by clicking the "Regenerate Thread Title" button in the ellipsis menu in the top right of the panel.

### Following the Agent {#following-the-agent}

Follow the agent as it reads and edits files by clicking the crosshair icon at the bottom left of the panel.
Your editor will jump to each file the agent touches.

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

Edit diffs can also appear inline in individual files with the same
keep/reject hunk controls as the multi-buffer review pane. This temporarily overrides the buffer's git diff while review is active. Enable it by setting `agent.single_file_review` to `true` in your settings.

## Terminal Threads {#terminal-threads}

The Agent Panel can host terminal threads alongside your agent threads. Each terminal thread appears as its own entry in the [Threads Sidebar](./parallel-agents.md#threads-sidebar) with a terminal icon, letting you switch between conversations and shell sessions from the same list.

External agents like Claude Agent and Codex can also run as terminal threads. Some support terminal signals — such as bell notifications or title updates — that Zed uses to show useful context in the sidebar.

### Opening a Terminal Thread {#opening-a-terminal-thread}

Open the menu using the agent selector button on the left (in the empty state) or the `+` icon in the top-right of the panel toolbar, and choose **Terminal**. The terminal thread opens in the panel body, just like switching to a thread. You can open as many as you like — each gets its own sidebar entry.

### Terminal Thread Titles {#terminal-thread-titles}

The terminal title in the toolbar updates automatically to reflect the running shell or process. You can also set a custom name by clicking the title or the pencil icon that appears on hover.

### Notifications {#terminal-thread-notifications}

When a terminal produces a bell character while not in focus, Zed notifies you the same way it does when an agent finishes — with a visual pop-up and an optional sound. Clicking the notification brings the terminal into focus and clears the indicator. The same `agent.notify_when_agent_waiting` and `agent.play_sound_when_agent_done` settings apply.

### Closing Terminal Threads {#closing-terminal-threads}

Unlike agent threads, terminal threads are closed rather than archived — they don't go to Thread History. To close one, hover over it in the Threads Sidebar and click the **×** button, or select it and press {#kb agent::ArchiveSelectedThread}.

### Claude Code Notifications {#claude-code-notifications}

Claude Code can notify you when it finishes a task or pauses for permission. To enable this, set `preferredNotifChannel` to `"terminal_bell"` in your Claude Code user settings:

```json
{
  "preferredNotifChannel": "terminal_bell"
}
```

You can also set this from within Claude Code by running `/config`, selecting `Local Notifications`, and choosing `Terminal Bell`.

> If you run Claude Code inside tmux, bell notifications may not reach the outer terminal unless passthrough is enabled. Add this to `~/.tmux.conf`:
>
> ```
> set -g allow-passthrough on
> ```

For more, see the [Claude Code documentation](https://code.claude.com/docs/en/terminal-config).

### Amp Notifications {#amp-notifications}

Amp updates terminal titles automatically and can also notify you when it needs your attention. To enable notifications in Zed terminal threads, add `AMP_FORCE_BEL=1` to your terminal environment settings:

```json [settings]
{
  "terminal": {
    "env": {
      "AMP_FORCE_BEL": "1"
    }
  }
}
```

Restart Amp after adding the environment variable.

### OpenCode Notifications {#opencode-notifications}

OpenCode can update terminal titles automatically. For Zed notifications, add an OpenCode plugin that emits a terminal bell when OpenCode needs your attention.

Create `.opencode/plugins/zed-bell.js` in your project, or `~/.config/opencode/plugins/zed-bell.js` to use it globally:

```js
export const ZedBell = async () => {
  return {
    event: async ({ event }) => {
      if (event.type === "session.idle" || event.type === "permission.asked") {
        process.stdout.write("\x07");
      }
    },
  };
};
```

Restart OpenCode after adding the plugin.

### Pi Notifications {#pi-notifications}

Pi can use an extension to emit a notification when it finishes a turn. Create `.pi/extensions/zed-bell.ts` in your project, or `~/.pi/agent/extensions/zed-bell.ts` to use it globally:

```ts
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

export default function (pi: ExtensionAPI) {
  pi.on("agent_end", async () => {
    process.stdout.write("\x07");
  });
}
```

Restart Pi after adding the extension, or run `/reload` if the extension is in one of Pi's auto-discovered extension locations.

### Codex Terminal Titles {#codex-terminal-titles}

Codex can update the terminal title as it works, which Zed uses to show useful context for Codex terminal threads in the sidebar — such as the project, current status, branch, model, or task progress.

To configure this from within Codex, run `/title` and use the picker to choose which fields appear and in what order. Codex saves the selection to `tui.terminal_title` in `~/.codex/config.toml`. You can also edit it directly:

```toml
[tui]
terminal_title = ["spinner", "project-name", "run-state", "thread-title"]
```

## Adding Context {#adding-context}

The agent can search your codebase to find relevant context, but providing it explicitly improves response quality and reduces latency.

Add context by typing `@` in the message editor.
You can mention files, directories, symbols, previous threads, skills, and diagnostics.

When you paste multi-line code selections copied from a buffer, Zed automatically formats them as @-mentions with the file context.
To paste content without this automatic formatting, use {#kb agent::PasteRaw} to paste raw text directly.

### Selection as Context

Additionally, you can also select text in a buffer or terminal and add it as context by using the {#kb agent::AddSelectionToThread} keybinding, running the {#action agent::AddSelectionToThread} action, or choosing the "Selection" item in the `+` menu in the message editor.

### Images as Context

It's also possible to attach images in your prompt for providers that support vision models.
OpenAI GPT-4o and later, Anthropic Claude 3 and later, Google Gemini 1.5 and 2.0, and Bedrock vision models (Claude 3+, Amazon Nova Pro and Lite, Meta Llama 3.2 Vision, Mistral Pixtral) all support image inputs.

To add an image, you can either search in your project's directory by @-mentioning it, or drag it from your file system directly into the agent panel message editor.
Copying an image and pasting it is also supported.

## Token Usage {#token-usage}

Zed surfaces how many tokens you are consuming for your currently active thread near the profile selector in the panel's message editor.

Once you approach the model's context window, a banner appears above the message editor suggesting to start a new thread with the current one summarized and added as context.
You can also do this at any time with an ongoing thread via the "Agent Options" menu on the top right, where you'll see a "New from Summary" button, as well as simply @-mentioning a past thread in a new one..

## Changing Models {#changing-models}

After you've configured your LLM providers—either via [a custom API key](./llm-providers.md) or through [Zed's hosted models](./models.md)—you can switch between their models by clicking on the model selector on the message editor or by using the {#kb agent::ToggleModelSelector} keybinding.

> The same model can be offered via multiple providers - for example, Claude Sonnet 4.5 is available via Zed Pro, OpenRouter, Anthropic directly, and more.
> Make sure you've selected the correct model **_provider_** for the model you'd like to use, delineated by the logo to the left of the model in the model selector.

### Favoriting Models

You can mark specific models as favorites either through the model selector, by clicking on the star icon button that appears as you hover the model, or through your settings via the `agent.favorite_models` settings key.

Cycle through your favorites with {#kb agent::CycleFavoriteModels} without opening the model selector.

## Using Tools {#using-tools}

The Agent Panel supports tool calling, which enables agentic editing.
Zed includes several [built-in tools](./tools.md) for searching your codebase, editing files, running terminal commands, and more.

You can also extend the set of available tools via [MCP Servers](./mcp.md).

### Profiles {#profiles}

Profiles act as a way to group tools.
Zed offers three built-in profiles and you can create as many custom ones as you want.

#### Built-in Profiles {#built-in-profiles}

- `Write`: A profile with tools to allow the LLM to write to your files and run terminal commands.
  This one essentially has all built-in tools turned on.
- `Ask`: A profile with read-only tools.
  Best for asking questions about your code base without the concern of the agent making changes.
- `Minimal`: A profile with no tools.
  Best for general conversations with the LLM where no knowledge of your code base is necessary.

You can explore the exact tools enabled in each profile by clicking on the profile selector button > `Configure` button > the one you want to check out.

Alternatively, you can also use either the command palette, by running {#action agent::ManageProfiles}, or the keybinding directly, {#kb agent::ManageProfiles}, to have access to the profile management modal.

Use {#kb agent::CycleModeSelector} to cycle through available profiles without opening the modal.

#### Custom Profiles {#custom-profiles}

You can also create a custom profile through the Agent Profile modal.
From there, you can choose to `Add New Profile` or fork an existing one with a custom name and your preferred set of tools.

It's also possible to override built-in profiles.
In the Agent Profile modal, select a built-in profile, navigate to `Configure Tools`, and rearrange the tools you'd like to keep or remove.

Zed will store this profile in your settings using the same profile name as the default you overrode.

All custom profiles can be edited via the UI or by hand under the `agent.profiles` key in your settings file.

To delete a custom profile, open the Agent Profile modal, select the profile you want to remove, and click the delete button.

### Tool Permissions

> **Note:** In Zed v0.224.0 and above, tool approval is controlled by `agent.tool_permissions.default`.
> In earlier versions, it was controlled by the `agent.always_allow_tool_actions` boolean (default `false`).

Zed's Agent Panel provides the `agent.tool_permissions.default` setting to control tool approval behavior:

- `"confirm"` (default) — Prompts for approval before running any tool action
- `"allow"` — Auto-approves tool actions without prompting
- `"deny"` — Blocks all tool actions

When the agent requests permission for an action, the confirmation menu includes options to allow or deny once, plus "Always for <tool>" choices that set a tool-level default.
When Zed can extract a safe pattern from the input, it also offers pattern-based "Always for ..." choices that add `always_allow`/`always_deny` rules.
MCP tools only support tool-level defaults.

Even with `"default": "allow"`, per-tool `always_deny` and `always_confirm` patterns are still respected — so you can auto-approve most actions while blocking or gating specific ones.

Learn more about [how tool permissions work](./tool-permissions.md), how to further customize them, and other details.

### Model Support {#model-support}

Tool calling needs to be individually supported by each model and model provider.
Therefore, despite the presence of built-in tools, some models may not have the ability to pick them up.
You should see a "No tools" label if you select a model that falls into this case.

All [Zed's hosted models](./models.md) support tool calling out-of-the-box.

### MCP Servers {#mcp-servers}

Similarly to the built-in tools, some models may not support all tools included in a given MCP Server.
Zed's UI will inform you about this via a warning icon that appears close to the model selector.

## Errors and Debugging {#errors-and-debugging}

If you hit an error or unusual LLM behavior, open the thread as Markdown with {#action agent::OpenActiveThreadAsMarkdown} and attach it to your GitHub issue.

You can also open threads as Markdown by clicking on the file icon button, to the right of the thumbs down button, when focused on the panel's editor.

## Feedback {#feedback}

You can rate agent responses to help improve Zed's system prompt and tools.

> Note that rating responses will send your data related to that response to Zed's servers.
> See [AI Improvement](./ai-improvement.md) and [Privacy and Security](./privacy-and-security.md) for more information about Zed's approach to AI improvement, privacy, and security.
> **_If you don't want data persisted on Zed's servers, don't rate_**.
> We will not collect data for improving our Agentic offering without you explicitly rating responses.

To help improve Zed's system prompt and tools, rate responses with the thumbs up/down controls at the end of each response.
In case of a thumbs down, a new text area will show up where you can add more specifics about what happened.

You can provide feedback on the thread at any point after the agent responds, and multiple times within the same thread.
