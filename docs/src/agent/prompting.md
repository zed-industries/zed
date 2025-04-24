# Prompting & Prompt Library

## Using Prompts {#using-prompts}

Prompts are an essential part of interacting with AI assistants in Zed. They help guide the AI's responses and ensure you get the most relevant and useful information.

Every new chat will start with the [default prompt](#default-prompt), which can be customized.

Remember that effective prompting is an iterative process. Experiment with different prompt structures and wordings to find what works best for your specific needs and the model you're using.

Here are some tips for using prompts effectively:

1. Be specific: Clearly state what you want the AI to do or explain.
2. Provide context: Include relevant information about your project or problem.
3. Use examples: If applicable, provide examples to illustrate your request.
4. Break down complex tasks: For multi-step problems, consider breaking them into smaller, more manageable prompts.

## Prompt Library {#prompt-library}

The Prompt Library is an interface for writing and managing prompts. Like other text-driven UIs in Zed, it is a full editor with syntax highlighting, keyboard shortcuts, etc.

You can use Inline Assist right in the prompt editor, allowing you to automate and rewrite prompts.

### Opening the Prompt Library

1. Open the assistant panel.
2. Click on the menu in the top right corner.
3. Select "Prompt Library" from the dropdown.

You can also use the `assistant: deploy prompt library` command while in the assistant panel.

### Managing Prompts

Once a prompt is selected, you can edit it directly in the editor. Its title can be changed from the editor title bar as well.

Prompts can be duplicated, deleted, or added to the default prompt using the buttons in the prompt editor.

## Creating a Prompt {#creating-a-prompt}

To create a prompt, simply open the Prompt Library and click the "+" button. Prompts are stored locally and can be accessed from the library at any time.

Having a series of prompts specifically tailored to prompt engineering can also help you write consistent and effective prompts.

The process of writing and refining prompts commonly called "prompt engineering".

More on prompt engineering:

- [Anthropic: Prompt Engineering](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview)
- [OpenAI: Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering)

## Editing the Default Prompt {#default-prompt}

Zed allows you to customize the default prompt used when interacting with LLMs. Or to be more precise, it uses a series of prompts that are combined to form the default prompt.

To edit prompts, select "Prompt Library" from the menu icon (three horizontal lines) in the upper right hand corner or using the `cmd-k l` keyboard shortcut.

A default prompt might look something like:

```plaintext
[-] Default
  [+] Today's date
  [+] You are an expert
  [+] Don't add comments
```

Each of the above prompts can be individually expanded, and since Zed's assistant is all text, they can also be edited directly. Edits here will not propagate to the saved prompts.

You can add prompts to the default prompt by clicking the icon in the top right (the "sparkle" icon) of the prompt editor. This will add the prompt to the default prompt.

_Changes to the default prompt will not affect existing contexts. You can remove the default prompt and manually re-add it with `/default` to update an existing context._

Default prompts will show at the top of the prompt list, and will be included with every new chat.

You can manually add the default prompt using the `/default` command.

> **Note:** Remember, commands are only evaluated when the context is created, so a command like `/now` won't continuously update, or `/file` commands won't keep their contents up to date.

## Commands in Prompts

[Commands](./commands.md) can be used in prompts to insert dynamic content or perform actions. For example, if you want to create a prompt where it is important for the model to know the date, you can use the `/now` command to insert the current date.

> **Note:** Slash commands in prompts **must** be on their own line.

See the [Commands](./commands.md) docs for more information on commands, and what slash commands are available.

### Example:

```plaintext
You are an expert Rust engineer. The user has asked you to review their project and answer some questions.

Here is some information about their project:

/file Cargo.toml
```

In the above example, the `/file` command is used to insert the contents of the `Cargo.toml` file (or all `Cargo.toml` files present in the project) into the prompt.

## Nesting Prompts

Similar to adding prompts to the default prompt, you can nest prompts within other prompts with the `/prompt` command.

You might want to nest prompts to:

- Create templates on the fly
- Break collections like docs or references into smaller, mix-and-matchable parts
- Create variants of a similar prompt (e.g., `Async Rust - Tokio` vs. `Async Rust - Async-std`)

### Example:

```plaintext
Title: Zed-Flavored Rust

## About Zed

/prompt Zed: Zed (a prompt about what Zed is)

## Rust - Zed Style

/prompt Rust: Async - Async-std (zed doesn't use tokio)
/prompt Rust: Zed-style Crates (we have some unique conventions)
/prompt Rust - Workspace deps (bias towards reusing deps from the workspace)
```

_The (text) above are comments and are not part of the prompt._

> **Note:** While you technically _can_ nest a prompt within itself, we wouldn't recommend it (in the strongest of terms.) Use at your own risk!

By using nested prompts, you can create modular and reusable prompt components that can be combined in various ways to suit different scenarios.

## Advanced Concepts

### Prompt Templates

Zed uses prompt templates to power internal assistant features, like the terminal assistant, or the content prompt used in Inline Assist.

Zed has the following internal prompt templates:

- `content_prompt.hbs`: Used for generating content in the editor.
- `terminal_assistant_prompt.hbs`: Used for the terminal assistant feature.
- `suggest_edits.hbs`: Used for generating the model instructions for the XML Suggest Edits should return.

At this point it is unknown if we will expand templates further to be user-creatable.

### Overriding Templates

> **Note:** It is not recommended to override templates unless you know what you are doing. Editing templates will break your assistant if done incorrectly.

Zed allows you to override the default prompts used for various assistant features by placing custom Handlebars (.hbs) templates in your `~/.config/zed/prompt_overrides` directory.

The following templates can be overridden:

1. [`content_prompt.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/content_prompt.hbs): Used for generating content in the editor.

2. [`terminal_assistant_prompt.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/terminal_assistant_prompt.hbs): Used for the terminal assistant feature.

3. [`suggest_edits.hbs`](https://github.com/zed-industries/zed/tree/main/assets/prompts/suggest_edits.hbs): Used for generating the model instructions for the XML Suggest Edits should return.

> **Note:** Be sure you want to override these, as you'll miss out on iteration on our built-in features. This should be primarily used when developing Zed.

You can customize these templates to better suit your needs while maintaining the core structure and variables used by Zed. Zed will automatically reload your prompt overrides when they change on disk.

Consult Zed's [assets/prompts](https://github.com/zed-industries/zed/tree/main/assets/prompts) directory for current versions you can play with.
