## Using Rules {#using-rules}

Rules are an essential part of interacting with AI assistants in Zed. They help guide the AI's responses and ensure you get the most relevant and useful information.

Every new chat will start with the [default rules](#default-rules), which can be customized and is where your model prompting will stored

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

Once a rule is selected, you can edit it directly in the editor. Its title can be changed from the editor title bar as well.

Rules can be duplicated, deleted, or added to the default rules using the buttons in the rules editor.

## Creating Rules {#creating-rules}

To create a rule, simply open the Rules Library and click the `+` button. Rules are stored locally and can be accessed from the library at any time.

Having a series of rules specifically tailored to prompt engineering can also help you write consistent and effective rules.

The process of writing and refining prompts commonly called "prompt engineering".

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

Default rules are included in the context of new threads automatically. If using a Text Thread, @-mentioned rules can be edited directly—edits here will not propagate to the saved rules.

Default rules will show at the top of the rules list, and will be included with every new conversation.

You can manually add other rules as context using the `@rule` command, or `/prompt` in Text Threads.

> **Note:** Remember, commands are only evaluated when the context is created, so a command like `@file` won't continuously update.

## Commands in Rules

[Commands](./commands.md) can be used in rules to insert dynamic content or perform actions. For example, if you want to create a rule where it is important for the model to know the date, you can use the `/now` command to insert the current date.

> **Note:** Slash commands in rules **must** be on their own line.

See the [Commands](./commands.md) docs for more information on commands, and what slash commands are available.

### Example:

```plaintext
You are an expert Rust engineer. The user has asked you to review their project and answer some questions.

Here is some information about their project:

@file Cargo.toml
```

In the above example, the `@file` command is used to insert the contents of the `Cargo.toml` file (or all `Cargo.toml` files present in the project) into the rule.

## Nesting Rules

Similar to adding rules to the default rules, you can nest rules within other rules with the `/prompt` command (only supported in Text Threads currently).

You might want to nest rules to:

- Create templates on the fly
- Break collections like docs or references into smaller, mix-and-matchable parts
- Create variants of a similar rule (e.g., `Async Rust - Tokio` vs. `Async Rust - Async-std`)

### Example:

```plaintext
Title: Zed-Flavored Rust

## About Zed

/prompt Zed: Zed (a rule about what Zed is)

## Rust - Zed Style

/prompt Rust: Async - Async-std (zed doesn't use tokio)
/prompt Rust: Zed-style Crates (we have some unique conventions)
/prompt Rust - Workspace deps (bias towards reusing deps from the workspace)
```

_The (text) above are comments and are not part of the rule._

> **Note:** While you technically _can_ nest a rule within itself, we wouldn't recommend it (in the strongest of terms.) Use at your own risk!

By using nested rules, you can create modular and reusable rule components that can be combined in various ways to suit different scenarios.

## Advanced Concepts

### Rule Templates

Zed uses rule templates to power internal assistant features, like the terminal assistant, or the content rules used in the inline assistant.

Zed has the following internal rule templates:

- `content_prompt.hbs`: Used for generating content in the editor.
- `terminal_assistant_prompt.hbs`: Used for the terminal assistant feature.
- `suggest_edits.hbs`: Used for generating the model instructions for the XML Suggest Edits should return.

At this point it is unknown if we will expand templates further to be user-creatable.

### Overriding Templates

> **Note:** It is not recommended to override templates unless you know what you are doing. Editing templates will break your assistant if done incorrectly.

Zed allows you to override the default rules used for various assistant features by placing custom Handlebars (.hbs) templates in your `~/.config/zed/prompt_overrides` directory.

The following templates can be overridden:

1. [`content_prompt.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/content_prompt.hbs): Used for generating content in the editor.

2. [`terminal_assistant_prompt.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/terminal_assistant_prompt.hbs): Used for the terminal assistant feature.

3. [`suggest_edits.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/suggest_edits.hbs): Used for generating the model instructions for the XML Suggest Edits should return.

> **Note:** Be sure you want to override these, as you'll miss out on iteration on our built-in features. This should be primarily used when developing Zed.

You can customize these templates to better suit your needs while maintaining the core structure and variables used by Zed. Zed will automatically reload your prompt overrides when they change on disk.

Consult Zed's [assets/prompts](https://github.com/zed-industries/zed/tree/main/assets/prompts) directory for current versions you can play with.

- todo! replace more instances of "prompt" with "rules"
- todo! - needs more tweaks, not done
- todo! - for old docs covering features that were in old assistant, but not new, do we we mention, or omit?
