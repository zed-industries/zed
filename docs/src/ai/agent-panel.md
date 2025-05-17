# Agent Panel

The Agent Panel provides you with a way to interact with LLMs.
You can use it for various tasks, such as generating code, asking questions about your code base, and general inquiries such as emails and documentation.

To open the Agent Panel, use the `agent: new thread` action in [the Command Palette](../getting-started.md#command-palette) or click the ✨ (sparkles) icon in the status bar.

If you're using the Agent Panel for the first time, you'll need to [configure at least one LLM provider](./configuration.md).

## Overview {#overview}

After you've configured a LLM provider, type at the message editor and hit `enter` to submit your prompt.
If you need extra room to type, you can expand the message editor with {#kb agent::ExpandMessageEditor}.

You should start to see the responses stream in with indications of [which tools](./tools.md) the AI is using to fulfill your prompt.
For example, if the AI chooses to perform an edit, you will see a card with the diff.

### Editing Messages {#editing-messages}

Any message that you send to the AI is editable.
You can click on the card that contains your message and re-submit it with an adjusted prompt and/or new pieces of context.

### Checkpoints {#checkpoints}

Every time the AI performs an edit, you should see a "Restore Checkpoint" button to the top of your message.
This allows you to return your code base to the state it was in prior to that message.
This is usually valuable if the AI's edit doesn't go in the right direction.

### Navigating History {#navigating-history}

To quickly navigate through recently opened threads, use the {#kb agent::ToggleNavigationMenu} binding, when focused on the panel's editor, or click the hamburger icon button at the top left of the panel to open the dropdown that shows you the six most recent interactions with the LLM.

The items in this menu work similarly to tabs, and closing them from there doesn't delete the thread; just takes them out of the recent list.

You can also view all historical conversations with the `View All` option from within the same menu or by reaching for the {#kb agent::OpenHistory} binding.

### Following the Agent {#following-the-agent}

Zed is built with collaboration natively integrated into the product.
This approach extends to collaboration with AI as well.

As soon as you send a prompt to the Agent, click on the "crosshair" icon at the bottom left of the panel to follow along as it reads through your codebase and performs edits.

### Get Notified {#get-notified}

If you send a prompt to the Agent and then move elsewhere, putting Zed in the background, a notification will pop up at the top right of your monitor indicating that the Agent has completed its work.

You can customize the notification behavior or turn it off entirely by using the `agent.notify_when_agent_waiting` key.

### Reviewing Changes {#reviewing-changes}

If you are using a profile that includes write tools, and the agent has made changes to your project, you'll notice the Agent Panel surfaces the fact that edits have been applied.

You can click on the accordion bar that shows up right above the panel's editor see which files have been changed, or click `Review Changes` ({#kb agent::OpenAgentDiff}) to open a multi-buffer to review them.
Reviewing includes the option to accept or reject each edit, or accept or reject all edits.

Diffs with changes also appear in individual buffers.
So, if your active tab had changes added by the AI, you'll see diffs with the same accept/reject controls as in the multi-buffer.

## Adding Context {#adding-context}

Although Zed's agent is very efficient at reading through your code base to autonomously pick up relevant files, directories, and other context, manually adding context is still usually encouraged as a way to speed up and improve the AI's response quality.

If you have a tab open when triggering the Agent Panel, that tab will appear as a suggested context in form of a dashed button.
You can also add other forms of context, like files, rules, and directories, by either typing `@` or hitting the `+` icon button.

You can even add previous threads as context with the `@thread` command, or by selecting "Start new from summary" option from the top-right menu in the agent panel to continue a longer conversation and keep it within the size of context window.

Images are also supported, and pasting them over in the panel's editor works.

### Token Usage {#token-usage}

Zed surfaces how many tokens you are consuming for your currently active thread in the panel's toolbar.
Depending on how many pieces of context you add, your token consumption can grow rapidly.

With that in mind, once you get close to the model's context window, we'll display a banner on the bottom of the message editor offering to start a new thread with the current one summarized and added as context.
You can also do this at any time with an ongoing thread via the "Agent Options" menu on the top right.

## Changing Models {#changing-models}

After you've configured your LLM providers—either via [a custom API key](./configuration.md#use-your-own-keys) or through [Zed's hosted models](./models.md)—you can switch between them by clicking on the model selector on the message editor or by using the {#kb agent::ToggleModelSelector} keybinding.

## Using Tools {#using-tools}

The new Agent Panel supports tool calling, which enables agentic collaboration with AI.
Zed comes with [several built-in tools](./tools.md) that allow models to perform tasks such as searching through your codebase, editing files, running commands, and others.

You can also extend the set of available tools via [MCP Servers](./mcp.md).

### Profiles {#profiles}

Profiles introduce a way to group tools.
Zed offers three built-in profiles and you can create as many custom ones as you want.

#### Built-in Profiles {#built-in-profiles}

- `Write`: A profile with tools to allow the LLM to write to your files and run terminal commands. This one essentially has all built-in tools turned on.
- `Ask`: A profile with read-only tools. Best for asking questions about your code base without the fear of the agent making changes.
- `Minimal`: A profile with no tools. Best for general conversations with the LLM where no knowledge of your code is necessary.

You can explore the exact tools enabled in each profile by clicking on the profile selector button > `Configure Profiles…` > the one you want to check out.

#### Custom Profiles {#custom-profiles}

You may find yourself in a situation where the built-in profiles don't quite fit your specific needs.
Zed's Agent Panel allows for building custom profiles.

You can create new profile via the `Configure Profiles…` option in the profile selector.
From here, you can choose to `Add New Profile` or fork an existing one with your choice of tools and a custom profile name.

You can also override built-in profiles.
With a built-in profile selected, in the profile selector, navigate to `Configure Tools`, and select the tools you'd like.

Zed will store this profile in your settings using the same profile name as the default you overrode.

All custom profiles can be edited via the UI or by hand under the `assistant.profiles` key in your `settings.json` file.

### Model Support {#model-support}

Tool calling needs to be individually supported by each model and model provider.
Therefore, despite the presence of tools, some models may not have the ability to pick them up yet in Zed.
You should see a "No tools" disabled button if you select a model that falls into this case.

We want to support all of them, though!
We may prioritize which ones to focus on based on popularity and user feedback, so feel free to help and contribute.

All [Zed's hosted models](./models.md) support tool calling out-of-the-box.

### MCP Servers {#mcp-servers}

Similarly to the built-in tools, some models may not support all tools included in a given MCP Server.
Zed's UI will inform about this via a warning icon that appears close to the model selector.

## Text Threads {#text-threads}

["Text threads"](./text-threads.md) present your conversation with the LLM in a different format—as raw text.
With text threads, you have full control over the conversation data.
You can remove and edit responses from the LLM, swap roles, and include more context earlier in the conversation.

For users who have been with us for some time, you'll notice that text threads are our original assistant panel—users love it for the control it offers.
We do not plan to deprecate text threads, but it should be noted that if you want the AI to write to your code base autonomously, that's only available in the newer, and now default, "Threads".

### Text Thread History {#text-thread-history}

Content from text thread are saved to your file system.
Visit [the dedicated docs](./text-threads.md#history) for more info.

## Errors and Debugging {#errors-and-debugging}

In case of any error or strange LLM response behavior, the best way to help the Zed team debug is by reaching for the `agent: open thread as markdown` action and attaching that data as part of your issue on GitHub.

This action exposes the entire thread in the form of Markdown and allows for deeper understanding of what each tool call was doing.

You can also open threads as Markdown by clicking on the file icon button, to the right of the thumbs down button, when focused on the panel's editor.

## Feedback {#feedback}

Every change we make to Zed's system prompt and tool set, needs to be backed by an eval with good scores.

Every time the LLM performs a weird change or investigates a certain topic in your codebase completely incorrectly, it's an indication that there's an improvement opportunity.

> Note that rating responses will send your data related to that response to Zed's servers.
> See [AI Improvement](./ai-improvement.md) and [Privacy and Security](./privacy-and-security.md) for more information about Zed's approach to AI improvement, privacy, and security.
> **_If you don't want data persisted on Zed's servers, don't rate_**. We will not collect data for improving our Agentic offering without you explicitly rating responses.

The best way you can help influence the next change to Zed's system prompt and tools is by rating the LLM's response via the thumbs up/down buttons at the end of every response.
In case of a thumbs down, a new text area will show up where you can add more specifics about what happened.

You can provide feedback on the thread at any point after the agent responds, and multiple times within the same thread.
