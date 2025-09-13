# Using Rules {#using-rules}

A rule is essentially a prompt that is inserted at the beginning of each interaction with the Agent.
Currently, Zed supports several types of rules: project-level `.rules` files, default rules from the Rules Library, and profile-specific rules that are automatically applied based on your active agent profile.

## `.rules` files

Zed supports including `.rules` files at the top level of worktrees, and they act as project-level instructions that are included in all of your interactions with the Agent Panel.
Other names for this file are also supported for compatibility with other agents, but note that the first file which matches in this list will be used:

- `.rules`
- `.cursorrules`
- `.windsurfrules`
- `.clinerules`
- `.github/copilot-instructions.md`
- `AGENT.md`
- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`

## Rules Library {#rules-library}

The Rules Library is an interface for writing and managing rules. Like other text-driven UIs in Zed, it is a full editor with syntax highlighting, keyboard shortcuts, etc.

You can use the inline assistant right in the rules editor, allowing you to automate and rewrite rules.

### Opening the Rules Library

1. Open the Agent Panel.
2. Click on the Agent menu (`...`) in the top right corner.
3. Select `Rules...` from the dropdown.

You can also use the `agent: open rules library` command while in the Agent Panel.

### Managing Rules

Once a rules file is selected, you can edit it directly in the built-in editor. Its title can be changed from the editor title bar as well.

Rules can be duplicated, deleted, or added to the default rules using the buttons in the rules editor.

### Creating Rules {#creating-rules}

To create a rule file, simply open the `Rules Library` and click the `+` button. Rules files are stored locally and can be accessed from the library at any time.

Having a series of rules files specifically tailored to prompt engineering can also help you write consistent and effective rules.

Here are a couple of helpful resources for writing better rules:

- [Anthropic: Prompt Engineering](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering/overview)
- [OpenAI: Prompt Engineering](https://platform.openai.com/docs/guides/prompt-engineering)

### Editing the Default Rules {#default-rules}

Zed allows you to customize the default rules used when interacting with LLMs.
Or to be more precise, it uses a series of rules that are combined to form the default rules.

Default rules are included in the context of every new thread automatically.
You can also manually add other rules (that are not flagged as default) as context using the `@rule` command.

## Profile Rules {#profile-rules}

In addition to default rules, Zed supports profile-specific rules that are automatically included when using a particular agent profile. These rules work alongside your default rules to provide specialized behavior for different use cases.

### How Profile Rules Work

When you start a conversation with the Agent Panel, Zed includes rules in this order of precedence:

1. **Project Rules**: `.rules` files from your project directory (always included)
2. **Default Rules**: Rules marked as default in your Rules Library (always included)
3. **Profile Rules**: Rules specifically enabled for your current profile (only when that profile is active)

This layered approach allows you to have:

- General rules that apply to all conversations
- Project-specific rules for the current codebase
- Profile-specific rules for specialized tasks

### Configuring Profile Rules

To set up profile rules:

1. Open the Agent Panel
2. Click on the profile selector dropdown
3. Choose `Configure Profiles...`
4. Select the profile you want to customize
5. Click `Configure Profile Rules`
6. Toggle the rules you want active for this profile

### Example Use Cases

Profile rules are particularly useful for specialized workflows:

**Code Review Profile**:

- Enable rules for security best practices
- Include rules for performance considerations
- Add rules for code documentation standards

**Testing Profile**:

- Enable rules for comprehensive test coverage
- Include rules for edge case testing
- Add rules for test naming conventions

**Refactoring Profile**:

- Enable rules for maintaining backward compatibility
- Include rules for incremental changes
- Add rules for code safety checks

### Viewing Active Rules

You can see which rules are currently active in your thread:

- **Thread View**: Look for the rules indicator that shows "Using X default rules" and "Using Y profile rules"
- **Rules Library**: Click on a rule to see if it's enabled for specific profiles
- **Profile Configuration**: View all rules enabled for each profile in the configuration interface

### Managing Profile Rules

Profile rules are stored in your Zed settings and can be managed through:

- **UI Configuration**: Use the profile configuration interface for easy toggling (recommended)
- **Settings File**: Edit the `assistant.profiles` section in your `settings.json`
- **Rules Library**: Create and edit rules that can be used across profiles

Rules enabled for profiles will show a profile indicator in the Rules Library, making it easy to see which rules are being used where.

> **Note**: When editing profile rules in your settings file, rules are referenced by their unique IDs (UUIDs for user-created rules, specific identifiers for built-in rules). It's recommended to use the UI configuration interface rather than editing these IDs manually, as the UI handles the ID mapping automatically and shows human-readable rule names.

## Migrating from Prompt Library

Previously, the Rules Library was called the "Prompt Library".
The new rules system replaces the Prompt Library except in a few specific cases, which are outlined below.

### Slash Commands in Rules

Previously, it was possible to use slash commands (now @-mentions) in custom prompts (now rules).
There is currently no support for using @-mentions in rules files, however, slash commands are supported in rules files when used with text threads.
See the documentation for using [slash commands in rules](./text-threads.md#slash-commands-in-rules) for more information.

### Prompt templates

Zed maintains backwards compatibility with its original template system, which allows you to customize prompts used throughout the application, including the inline assistant.
While the Rules Library is now the primary way to manage prompts, you can still use these legacy templates to override default prompts.
For more details, see the [Rules Templates](./text-threads.md#rule-templates) section under [Text Threads](./text-threads.md).
