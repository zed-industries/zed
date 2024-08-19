# Prompting & Prompt Library

### Adding Prompts

You can customize the default prompts used in new context editors by opening the `Prompt Library`.

Open the `Prompt Library` using either the menu in the top right of the assistant panel and choosing the `Prompt Library` option, or by using the `assistant: deploy prompt library` command when the assistant panel is focused.

## Advanced: Overriding Prompt Templates

Zed allows you to override the default prompts used for various assistant features by placing custom Handlebars (.hbs) templates in your `~/.config/zed/prompts/templates` directory. The following templates can be overridden:

1. `content_prompt.hbs`: Used for generating content in the editor.
   Format:

   ```handlebars
   You are an AI programming assistant. Your task is to
   {{#if is_insert}}insert{{else}}rewrite{{/if}}
   {{content_type}}{{#if language_name}} in {{language_name}}{{/if}}
   based on the following context and user request. Context:
   {{#if is_truncated}}
     [Content truncated...]
   {{/if}}
   {{document_content}}
   {{#if is_truncated}}
     [Content truncated...]
   {{/if}}

   User request:
   {{user_prompt}}

   {{#if rewrite_section}}
     Please rewrite the section enclosed in
     <rewrite_this></rewrite_this>
     tags.
   {{else}}
     Please insert your response at the
     <insert_here></insert_here>
     tag.
   {{/if}}

   Provide only the
   {{content_type}}
   content in your response, without any additional explanation.
   ```

2. `terminal_assistant_prompt.hbs`: Used for the terminal assistant feature.
   Format:

   ```handlebars
   You are an AI assistant for a terminal emulator. Provide helpful responses to
   user queries about terminal commands, file systems, and general computer
   usage. System information: - Operating System:
   {{os}}
   - Architecture:
   {{arch}}
   {{#if shell}}
     - Shell:
     {{shell}}
   {{/if}}
   {{#if working_directory}}
     - Current Working Directory:
     {{working_directory}}
   {{/if}}

   Latest terminal output:
   {{#each latest_output}}
     {{this}}
   {{/each}}

   User query:
   {{user_prompt}}

   Provide a clear and concise response to the user's query, considering the
   given system information and latest terminal output if relevant.
   ```

3. `edit_workflow.hbs`: Used for generating the edit workflow prompt.

4. `step_resolution.hbs`: Used for generating the step resolution prompt.

You can customize these templates to better suit your needs while maintaining the core structure and variables used by Zed. Zed will automatically reload your prompt overrides when they change on disk. Consult Zed's assets/prompts directory for current versions you can play with.

Be sure you want to override these, as you'll miss out on iteration on our built-in features. This should be primarily used when developing Zed.

Previous: [Using Commands](commands.md) | Next: [Inline Assistant](inline-assistant.md)
