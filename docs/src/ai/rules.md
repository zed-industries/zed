# Using Rules {#using-rules}

Rules are an essential part of interacting with AI assistants in Zed. They help guide the AI's responses and ensure you get the most relevant and useful information.

Every new chat will start with the [default rules](#default-rules), which can be customized and is where your model prompting will stored.

Remember that effective prompting is an iterative process. Experiment with different prompt structures and wordings to find what works best for your specific needs and the model you're using.

Here are some tips for creating effective rules:

1. Be specific: Clearly state what you want the AI to do or explain.
2. Provide context: Include relevant information about your project or problem.
3. Use examples: If applicable, provide examples to illustrate your request.
4. Break down complex tasks: For multi-step problems, consider breaking them into smaller, more manageable rules.

## Rules Library {#rules-library}

The Rules Library is an interface for writing and managing rules. Like other text-driven UIs in Zed, it is a full editor with syntax highlighting, keyboard shortcuts, etc.

You can use the inline assistant right in the rules editor, allowing you to automate and rewrite rules.

### Opening the Rules Library

1. Open the agent panel.
2. Click on the `Agent Menu` (`...`) in the top right corner.
3. Select `Rules...` from the dropdown.

You can also use the `assistant: open rules library` command while in the agent panel.

### Managing Rules

Once a rules file is selected, you can edit it directly in the built-in editor. Its title can be changed from the editor title bar as well.

Rules can be duplicated, deleted, or added to the default rules using the buttons in the rules editor.

## Creating Rules {#creating-rules}

To create a rule file, simply open the `Rules Library` and click the `+` button. Rules files are stored locally and can be accessed from the library at any time.

Having a series of rules files specifically tailored to prompt engineering can also help you write consistent and effective rules.

The process of writing and refining prompts is commonly referred to as "prompt engineering."

More on rule engineering:

- [Anthropic: Prompt Engineering](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview)
- [OpenAI: Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering)

## Editing the Default Rules {#default-rules}

Zed allows you to customize the default rules used when interacting with LLMs. Or to be more precise, it uses a series of rules that are combined to form the default rules.

To edit rules, select `Rules...` from the `Agent Menu` icon (`...`) in the upper right hand corner or using the {#kb assistant::OpenRulesLibrary} keyboard shortcut.

A default set of rules might look something like:

```plaintext
[-] Default
  [+] Today's date
  [+] You are an expert
  [+] Don't add comments
```

Default rules are included in the context of new threads automatically.

Default rules will show at the top of the rules list, and will be included with every new conversation.

You can manually add other rules as context using the `@rule` command.

> **Note:** Remember, commands are only evaluated when the context is created, so a command like `@file` won't continuously update.

## Migrating from Prompt Library

Previously, the Rules Library was called the Prompt Library. The new rules system replaces the Prompt Library except in a few specific cases, which are outlined below.

### Slash Commands in Rules

Previously, it was possible to use slash commands (now @-mentions) in custom prompts (now rules). There is currently no support for using @-mentions in rules files, however, slash commands are supported in rules files when used with text threads. See the documentation for using [slash commands in rules](./text-threads.md#slash-commands-in-rules) for more information.

### Prompt templates

Zed maintains backwards compatibility with its original template system, which allows you to customize prompts used throughout the application, including the inline assistant. While the Rules Library is now the primary way to manage prompts, you can still use these legacy templates to override default prompts. For more details, see the [Rules Templates](./text-threads.md#rule-templates) section under [Text Threads](./text-threads.md).
