# Agent Panel

The Agent Panel provides you with a way to interact with LLMs.
You can use it for various tasks, such as generating code, asking questions about your code base, and general inquiries such as emails and documentation.

To open the Agent Panel, reach for the `agent: new thread` action in [the Command Palette](./getting-started#command-palette) or click the ✨ (sparkles) icon in the status bar.

If you're using the Agent Panel for the first time, you'll need to set up at least one LLM provider.
Check [the Configuration page](./configuration.md) for more details about how to do it.

## Overview

After you've configured some LLM providers, you're ready to start working with the agent panel.

Pick the appropriate tool profile for your task and start a conversation by sending a message via the message editor.
If you need extra room to type, you can you can expand the message editor via ({#kb agent::ExpandMessageEditor}).

Past messages in the conversation history can be edited.
Both the included context and text itself can be removed or altered.
Editing a past message will cause all history after to be removed and new history to start forming.

If you are using a profile that includes write tools, and the agent has made changes to your project, you'll notice the agent panel surfaces the fact that edits have been applied.
You can click the dropdown to see which files have been changed or click `Review Changes` to open a multi-buffer to review them.
Reviewing includes the option to accept or reject each edit, or accept or reject all edits.

You'll also notice that the agent panel will start to use checkpoints as it alters the contents of your project. You can revert to these checkpoints to undo both changes to the project and the associated history in the agent panel, up to the checkpoint.

You can swap between recent threads via the Panel Menu, or view all historical conversations with the `View All` option from within the same menu.


- Tool cards
  - Edit tool card
  - Terminal tool card
- Feedback and rating
  - Cross link it to "model improvement"

## Adding Context

Although Zed's agent is very efficient at reading through your codebase to autonomously pick up context, manually adding context is usually encouraged as a way to speed and improve the AI's response quality.

If you have a tab open when triggering the Agent Panel, that tab will appear as a suggested context in form of a dotted pill.
You can also add other forms of context by either typing `@` or hitting the `+` icon button and then referring to files, rules, directories, and past threads.

Images are also supported as context, and pasting them over in the panel's message editor also works.

### Token Usage

Zed surfaces how many tokens you are consuming for your currently active thread in the panel's toolbar.
Depending on how many pieces of context you add, your token consumption can grow rapidly.

With that in mind, once you get close to the model's context window, we'll display a banner on the bottom of the message editor offering to start a new thread with the current one summarized and added as context.
You can also do this at any time—once with an ongoing thread—via the "Agent Options" menu on the top right.

## Changing Models

After you've configured your LLM providers—either via a custom API key or through Zed's hosted models—you can switch between them by clicking on the model selector on the message editor or by hitting the {#kb assistant::ToggleModelSelector} keybinding.

## Using Tools

The new Agent Panel, different from the previous one, introduces the ability to do tool calls, which is one of the things that enables an agentic flow with AI.
Zed comes with several tools built-in that enables the AI to do tasks such as edit files, read and search for files, run commands, and others.

You can also extend the set of available tools via MCP Servers.

### Profiles

Profiles are a way to bundle a set of tools.
Some tools will perform read-only tasks, and others have the capacity to edit files.

#### Built-in Profiles

- `Write`: Enables tools to allow the LLM to run terminal commands and to write to your code files.
- `Ask`: Enables read-only tools. Best for asking questions about your code base without the fear of the agent making changes.
- `Manual`: A configuration with no tools. Best for general conversations with the LLM where no knowledge of your code is necessary.

You can explore the exact tools enabled in each profile by clicking on the profile selector button (💬) > `Customize Current Profile` > `Tools...`

#### Custom Profiles

You may find yourself in a situation where the default profiles don't quite fit your specific needs. Zed's agent panel allows for building custom profiles.

You can create new profile via the `Configure Profiles...` option in the profile selector (💬).
From here, you can choose to `Add New Profile` or fork an existing one with your choice of tools and a custom profile name.

You can also override build-in profiles.
With a built-in profile selected, in the profile selector (💬), navigate to `Custom Current Profile` > `Tools...`, and select the tools you'd like.
Zed will store this profile in your settings using the same profile name as the default you overrode.

All custom profiles can be edited via the UI or by hand under the `assistant.profiles` key in your `settings.json` file.

### Model Support

### MCP Servers

## Text Threads

"Text threads" present your conversation with the LLM in a different format - as raw text.
With text threads, you have full control over the conversation data.
You can remove and edit responses from the LLM, swap roles, and include more context earlier in the conversation.
For users who have been with us for some time, you'll notice that text threads are our original assistant panel - users love it for the control it offers.
We do not plan to deprecate text threads, but it should be noted that if you want the assistant to write to your code base through tools, you'll want to use the newer "prompt" threads.

## Errors and Debugging

- Opening the thread as markdown
