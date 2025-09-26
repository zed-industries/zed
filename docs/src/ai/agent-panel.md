# Agent Panel

The Agent Panel allows you to interact with many LLMs and coding agents that can help with in various types of tasks, such as generating code, codebase understanding, and other general inquiries like writing emails, documentation, and more.

To open it, use the `agent: new thread` action in [the Command Palette](../getting-started.md#command-palette) or click the ✨ (sparkles) icon in the status bar.

## Getting Started

If you're using the Agent Panel for the first time, you need to have at least one LLM provider or external agent configured.
You can do that by:

1. [subscribing to our Pro plan](https://zed.dev/pricing), so you have access to our hosted models
2. [using your own API keys](./llm-providers.md#use-your-own-keys), either from model providers like Anthropic or model gateways like OpenRouter.
3. using an external agent like [Gemini CLI](./external-agents.md#gemini-cli) or [Claude Code](./external-agents.md#claude-code)

## Overview {#overview}

With an LLM provider or an external agent configured, type at the message editor and hit `enter` to submit your prompt.
If you need extra room to type, you can expand the message editor with {#kb agent::ExpandMessageEditor}.

You should start to see the responses stream in with indications of [which tools](./tools.md) the model is using to fulfill your prompt.
From this point on, you can interact with the many supported features outlined below.

> Note that for external agents, like [Gemini CLI](./external-agents.md#gemini-cli) or [Claude Code](./external-agents.md#claude-code), some of the features outlined below are _not_ currently supported—for example, _restoring threads from history_, _checkpoints_, _token usage display_, _model selection_, and others. All of them should hopefully be supported in the future.

### Creating New Threads {#new-thread}

By default, the Agent Panel uses Zed's first-party agent.

To change that, go to the plus button in the top-right of the Agent Panel and choose another option.
You choose to create a new [Text Thread](./text-threads.md) or, if you have [external agents](./external-agents.md) connected, you can create new threads with them.

### Editing Messages {#editing-messages}

Any message that you send to the AI is editable.
You can click on the card that contains your message and re-submit it with an adjusted prompt and/or new pieces of context.

### Checkpoints {#checkpoints}

Every time the AI performs an edit, you should see a "Restore Checkpoint" button to the top of your message, allowing you to return your code base to the state it was in prior to that message.

The checkpoint button appears even if you interrupt the thread midway through an edit attempt, as this is likely a moment when you've identified that the agent is not heading in the right direction and you want to revert back.

### Navigating History {#navigating-history}

To quickly navigate through recently opened threads, use the {#kb agent::ToggleNavigationMenu} binding, when focused on the panel's editor, or click the menu icon button at the top right of the panel to open the dropdown that shows you the six most recent threads.

The items in this menu function similarly to tabs, and closing them doesn’t delete the thread; instead, it simply removes them from the recent list.

To view all historical conversations, reach for the `View All` option from within the same menu or via the {#kb agent::OpenHistory} binding.

### Following the Agent {#following-the-agent}

Zed is built with collaboration natively integrated, and this design pattern extends to collaboration with AI. To follow the agent as it reads and edits in your codebase, click on the "crosshair" icon button at the bottom left of the panel.

You can also do that with the keyboard by pressing the `cmd`/`ctrl` modifier with `enter` when submitting a message.

### Get Notified {#get-notified}

If you send a prompt to the Agent and then move elsewhere, putting Zed in the background, you can be notified when its response is finished via:

- a visual notification that appears in the top right of your screen
- a sound notification

These notifications can be used together or individually, according to your preference.

You can customize their behavior, including turning them off entirely, by using the `agent.notify_when_agent_waiting` and `agent.play_sound_when_agent_done` settings keys.

### Reviewing Changes {#reviewing-changes}

Once the agent has made changes to your project, the panel will surface which files, and how many of them, have been edited.

To see which files specifically have been edited, expand the accordion bar that shows up right above the message editor or click the `Review Changes` button ({#kb agent::OpenAgentDiff}), which opens a multi-buffer tab with all changes.

You're able to reject or accept each individual change hunk, or the whole set of changes made by the agent.

Edit diffs also appear in individual buffers. If your active tab had edits made by the AI, you'll see diffs with the same accept/reject controls as in the multi-buffer.

## Adding Context {#adding-context}

Although Zed's agent is very efficient at reading through your code base to autonomously pick up relevant files, directories, and other context, manually adding context is still encouraged as a way to speed up and improve the AI's response quality.

To add any file, directory, symbol, previous threads, rules files, or even web pages as context, type `@` to mention them in the editor.

Pasting images as context is also supported by the Agent Panel.

### Token Usage {#token-usage}

Zed surfaces how many tokens you are consuming for your currently active thread nearby the profile selector in the panel's message editor. Depending on how many pieces of context you add, your token consumption can grow rapidly.

Once you approach the model's context window, a banner appears below the message editor suggesting to start a new thread with the current one summarized and added as context.
You can also do this at any time with an ongoing thread via the "Agent Options" menu on the top right.

## Changing Models {#changing-models}

After you've configured your LLM providers—either via [a custom API key](./llm-providers.md) or through [Zed's hosted models](./models.md)—you can switch between them by clicking on the model selector on the message editor or by using the {#kb agent::ToggleModelSelector} keybinding.

> The same model can be offered via multiple providers - for example, Claude Sonnet 4 is available via Zed Pro, OpenRouter, Anthropic directly, and more. Make sure you've selected the correct model **_provider_** for the model you'd like to use, delineated by the logo to the left of the model in the model selector.

## Using Tools {#using-tools}

The new Agent Panel supports tool calling, which enables agentic editing.
Zed comes with [several built-in tools](./tools.md) that allow models to perform tasks such as searching through your codebase, editing files, running commands, and others.

You can also extend the set of available tools via [MCP Servers](./mcp.md).

### Profiles {#profiles}

Profiles act as a way to group tools.
Zed offers three built-in profiles and you can create as many custom ones as you want.

#### Built-in Profiles {#built-in-profiles}

- `Write`: A profile with tools to allow the LLM to write to your files and run terminal commands. This one essentially has all built-in tools turned on.
- `Ask`: A profile with read-only tools. Best for asking questions about your code base without the concern of the agent making changes.
- `Minimal`: A profile with no tools. Best for general conversations with the LLM where no knowledge of your code base is necessary.

You can explore the exact tools enabled in each profile by clicking on the profile selector button > `Configure Profiles…` > the one you want to check out.

#### Custom Profiles {#custom-profiles}

You can create a custom profile via the `Configure Profiles…` option in the profile selector.
From here, you can choose to `Add New Profile` or fork an existing one with a custom name and your preferred set of tools.

You can also override built-in profiles.
With a built-in profile selected, in the profile selector, navigate to `Configure Tools`, and select the tools you'd like.

Zed will store this profile in your settings using the same profile name as the default you overrode.

All custom profiles can be edited via the UI or by hand under the `assistant.profiles` key in your `settings.json` file.

### Tool Approval

Zed's Agent Panel surfaces the `agent.always_allow_tool_actions` setting that, if turned to `false`, will require you to give permission to any editing attempt as well as tool calls coming from MCP servers.

You can change that by setting this key to `true` in either your `settings.json` or via the Agent Panel's settings view.

### Model Support {#model-support}

Tool calling needs to be individually supported by each model and model provider.
Therefore, despite the presence of tools, some models may not have the ability to pick them up yet in Zed. You should see a "No tools" label if you select a model that falls into this case.

All [Zed's hosted models](./models.md) support tool calling out-of-the-box.

### MCP Servers {#mcp-servers}

Similarly to the built-in tools, some models may not support all tools included in a given MCP Server. Zed's UI will inform about this via a warning icon that appears close to the model selector.

## Text Threads {#text-threads}

["Text Threads"](./text-threads.md) present your conversation with the LLM in a different format—as raw text. With text threads, you have full control over the conversation data. You can remove and edit responses from the LLM, swap roles, and include more context earlier in the conversation.

For users who have been with us for some time, you'll notice that text threads are our original assistant panel—users love it for the control it offers. We do not plan to deprecate text threads, but it should be noted that if you want the AI to write to your code base autonomously, that's only available in the newer, and now default, "Threads".

## Errors and Debugging {#errors-and-debugging}

In case of any error or strange LLM response behavior, the best way to help the Zed team debug is by reaching for the `agent: open thread as markdown` action and attaching that data as part of your issue on GitHub.

You can also open threads as Markdown by clicking on the file icon button, to the right of the thumbs down button, when focused on the panel's editor.

## Feedback {#feedback}

Zed supports rating responses from the agent for feedback and improvement.

> Note that rating responses will send your data related to that response to Zed's servers.
> See [AI Improvement](./ai-improvement.md) and [Privacy and Security](./privacy-and-security.md) for more information about Zed's approach to AI improvement, privacy, and security.
> **_If you don't want data persisted on Zed's servers, don't rate_**. We will not collect data for improving our Agentic offering without you explicitly rating responses.

The best way you can help influence the next change to Zed's system prompt and tools is by rating the LLM's response via the thumbs up/down buttons at the end of every response. In case of a thumbs down, a new text area will show up where you can add more specifics about what happened.

You can provide feedback on the thread at any point after the agent responds, and multiple times within the same thread.
