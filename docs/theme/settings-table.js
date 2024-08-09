// - status/channel - None (stable) | Preview | Nightly | Experiemental | Unstable
// - namespace?/key?
// - name
// - type (boolean, string, ...)
// - default_value
// - values[] | "See {name} for more &rarr;"
// - short_description
// - long_description? / examples? -> these ones get pulled out into sections

// Every row should have a #permalink
// Create a table from an array of settings

const settings = [
  {
    status: "Preview",
    key: "ui",
    name: "font_weight",
    type: "string",
    default_value: 400,
    values: [100, 200, 300, 400, 500, 600, 700, 800, 900],
    short_description: "The weight of the ui font",
    description: "The weight of the ui font",
  },
  {
    status: "Nightly",
    key: "ui",
    name: "font_weight",
    type: "number",
    default_value: 400,
    values: [100, 200, 300, 400, 500, 600, 700, 800, 900],
    short_description: "The weight of the ui font",
    description: "The weight of the ui font",
  },
];

const render_tag = (text, variant = "default") => {
  let classes = "tag";

  if (variant !== "default") {
    switch (variant) {
      case "info":
        classes += "tag-info;";
        break;
      case "warning":
        classes += "tag-warning;";
        break;
      case "error":
        classes += "tag-error";
        break;
      default:
        classes;
    }
  }

  return `<span class="${classes}">${text}</span>`;
};

function parseSettingsSchema(schema) {
  const settings = [];

  function processProperty(key, value, parentKey = "") {
    const fullKey = parentKey ? `${parentKey}.${key}` : key;

    if (value.type === "object" && value.properties) {
      Object.entries(value.properties).forEach(([subKey, subValue]) => {
        processProperty(subKey, subValue, fullKey);
      });
    } else {
      settings.push({
        key: fullKey,
        name: key,
        type: Array.isArray(value.type) ? value.type.join(" | ") : value.type,
        default_value: value.default,
        values: value.enum,
        short_description: value.description,
      });
    }
  }

  Object.entries(schema.properties).forEach(([key, value]) => {
    processProperty(key, value);
  });

  return settings;
}

function createSettingsTable(settings) {
  const container = document.createElement("div");
  container.className = "settings-list";

  // Group settings by their top-level key
  const groupedSettings = settings.reduce((acc, setting) => {
    const [topLevelKey, ...rest] = setting.key.split(".");
    if (!acc[topLevelKey]) {
      acc[topLevelKey] = [];
    }
    acc[topLevelKey].push({ ...setting, subKey: rest.join(".") });
    return acc;
  }, {});

  Object.entries(groupedSettings).forEach(([topLevelKey, groupSettings]) => {
    let section;
    if (groupSettings.length > 1) {
      section = document.createElement("section");
      section.id = `settings-${topLevelKey}`;

      const title = document.createElement("h3");
      title.textContent = topLevelKey;
      section.appendChild(title);
    }

    groupSettings.forEach((setting) => {
      const settingContainer = document.createElement("div");
      settingContainer.className = "setting-container";
      settingContainer.id = `setting-${setting.key}`;

      // Line 1: name type [default]
      const line1 = document.createElement("div");
      line1.className = "setting-line";
      line1.innerHTML = `<span class="setting-name">${setting.subKey || setting.name}</span>`;
      if (setting.type !== null && setting.type !== undefined) {
        line1.innerHTML += ` <span class="setting-type">${setting.type}</span>`;
      }
      settingContainer.appendChild(line1);

      // Line 2: values (if present)
      if (setting.values && setting.values.length > 0) {
        const line2 = document.createElement("div");
        line2.className = "setting-line setting-values";
        line2.textContent = `Values: ${setting.values.join(", ")}`;
        settingContainer.appendChild(line2);
      }

      // Line 3: description (if present)
      if (setting.short_description) {
        const line3 = document.createElement("div");
        line3.className = "setting-line setting-description";
        line3.textContent = setting.short_description;
        settingContainer.appendChild(line3);
      }

      // Line 4: default value
      if (setting.default_value !== undefined) {
        const line4 = document.createElement("div");
        line4.className = "setting-line setting-default";
        line4.textContent = `Default value: ${setting.default_value}`;
        settingContainer.appendChild(line4);
      }

      if (section) {
        section.appendChild(settingContainer);
      } else {
        container.appendChild(settingContainer);
      }
    });

    if (section) {
      container.appendChild(section);
    }
  });

  return container;
}

// Usage
document.addEventListener("DOMContentLoaded", () => {
  const settingsContainer = document.getElementById("settings-container");
  if (settingsContainer) {
    const parsedSettings = parseSettingsSchema(schema);
    const settingsList = createSettingsTable(parsedSettings);
    settingsContainer.appendChild(settingsList);
  }
});

const schema = {
  type: "object",
  properties: {
    active_pane_magnification: {
      description:
        "Scale by which to zoom the active pane. When set to 1.0, the active pane has the same size as others, but when set to a larger value, the active pane takes up more space.\n\nDefault: `1.0`",
      type: "number",
      format: "float",
    },
    always_treat_brackets_as_autoclosed: {
      description: "Default: false",
      type: "boolean",
    },
    assistant: {
      oneOf: [
        {
          type: "object",
          required: ["version"],
          properties: {
            button: {
              description:
                "Whether to show the assistant panel button in the status bar.\n\nDefault: true",
              type: "boolean",
            },
            default_height: {
              description:
                "Default height in pixels when the assistant is docked to the bottom.\n\nDefault: 320",
              type: "number",
              format: "float",
            },
            default_width: {
              description:
                "Default width in pixels when the assistant is docked to the left or right.\n\nDefault: 640",
              type: "number",
              format: "float",
            },
            dock: {
              description: "Where to dock the assistant.\n\nDefault: right",
              allOf: [
                {
                  $ref: "#/definitions/AssistantDockPosition",
                },
              ],
            },
            enabled: {
              description: "Whether the Assistant is enabled.\n\nDefault: true",
              type: "boolean",
            },
            provider: {
              description:
                'The provider of the assistant service.\n\nThis can be "openai", "anthropic", "ollama", "zed.dev" each with their respective default models and configurations.',
              allOf: [
                {
                  $ref: "#/definitions/AssistantProviderContentV1",
                },
              ],
            },
            version: {
              type: "string",
              enum: ["1"],
            },
          },
        },
        {
          type: "object",
          required: ["version"],
          properties: {
            button: {
              description:
                "Whether to show the assistant panel button in the status bar.\n\nDefault: true",
              type: "boolean",
            },
            default_height: {
              description:
                "Default height in pixels when the assistant is docked to the bottom.\n\nDefault: 320",
              type: "number",
              format: "float",
            },
            default_model: {
              description:
                "The default model to use when creating new contexts.",
              allOf: [
                {
                  $ref: "#/definitions/LanguageModelSelection",
                },
              ],
            },
            default_width: {
              description:
                "Default width in pixels when the assistant is docked to the left or right.\n\nDefault: 640",
              type: "number",
              format: "float",
            },
            dock: {
              description: "Where to dock the assistant.\n\nDefault: right",
              allOf: [
                {
                  $ref: "#/definitions/AssistantDockPosition",
                },
              ],
            },
            enabled: {
              description: "Whether the Assistant is enabled.\n\nDefault: true",
              type: "boolean",
            },
            version: {
              type: "string",
              enum: ["2"],
            },
          },
        },
      ],
    },
    auto_install_extensions: {
      description:
        "The extensions that should be automatically installed by Zed.\n\nThis is used to make functionality provided by extensions (e.g., language support) available out-of-the-box.",
      default: {},
      type: "object",
      additionalProperties: {
        type: "boolean",
      },
    },
    auto_signature_help: {
      description:
        "Whether to automatically show a signature help pop-up or not.\n\nDefault: false",
      type: "boolean",
    },
    auto_update: {
      type: "boolean",
    },
    auto_update_extensions: {
      default: {},
      type: "object",
      additionalProperties: {
        type: "boolean",
      },
    },
    autosave: {
      description: "When to automatically save edited buffers.\n\nDefault: off",
      allOf: [
        {
          $ref: "#/definitions/AutosaveSetting",
        },
      ],
    },
    base_keymap: {
      allOf: [
        {
          $ref: "#/definitions/BaseKeymap",
        },
      ],
    },
    buffer_font_fallbacks: {
      description: "The font fallbacks to use for rendering in text buffers.",
      default: null,
      type: "array",
      items: {
        type: "string",
      },
      $ref: "#/definitions/FontFallbacks",
    },
    buffer_font_family: {
      description: "The name of a font to use for rendering in text buffers.",
      default: null,
      type: "string",
      $ref: "#/definitions/FontFamilies",
    },
    buffer_font_features: {
      description:
        "The OpenType features to enable for rendering in text buffers.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/FontFeatures",
        },
      ],
    },
    buffer_font_size: {
      description: "The default font size for rendering in text buffers.",
      default: null,
      type: "number",
      format: "float",
    },
    buffer_font_weight: {
      description:
        "The weight of the editor font in CSS units from 100 to 900.",
      default: null,
      type: "number",
      format: "float",
    },
    buffer_line_height: {
      description: "The buffer's line height.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/BufferLineHeight",
        },
      ],
    },
    calls: {
      type: "object",
      properties: {
        mute_on_join: {
          description:
            "Whether the microphone should be muted when joining a channel or a call.\n\nDefault: false",
          type: "boolean",
        },
        share_on_join: {
          description:
            "Whether your current project should be shared when joining an empty channel.\n\nDefault: true",
          type: "boolean",
        },
      },
    },
    centered_layout: {
      $ref: "#/definitions/CenteredLayoutSettings",
    },
    chat_panel: {
      type: "object",
      properties: {
        button: {
          description:
            "Whether to show the panel button in the status bar.\n\nDefault: true",
          type: "boolean",
        },
        default_width: {
          description: "Default width of the panel in pixels.\n\nDefault: 240",
          type: "number",
          format: "float",
        },
        dock: {
          description: "Where to dock the panel.\n\nDefault: left",
          allOf: [
            {
              $ref: "#/definitions/DockPosition",
            },
          ],
        },
      },
    },
    code_actions_on_format: {
      description:
        'Which code actions to run on save after the formatter. These are not run if formatting is off.\n\nDefault: {} (or {"source.organizeImports": true} for Go).',
      type: "object",
      additionalProperties: {
        type: "boolean",
      },
    },
    collaboration_panel: {
      type: "object",
      properties: {
        button: {
          description:
            "Whether to show the panel button in the status bar.\n\nDefault: true",
          type: "boolean",
        },
        default_width: {
          description: "Default width of the panel in pixels.\n\nDefault: 240",
          type: "number",
          format: "float",
        },
        dock: {
          description: "Where to dock the panel.\n\nDefault: left",
          allOf: [
            {
              $ref: "#/definitions/DockPosition",
            },
          ],
        },
      },
    },
    command_aliases: {
      description:
        "Aliases for the command palette. When you type a key in this map, it will be assumed to equal the value.\n\nDefault: true",
      type: "object",
      additionalProperties: {
        type: "string",
      },
    },
    completion_documentation_secondary_query_debounce: {
      description:
        "The debounce delay before re-querying the language server for completion documentation when not included in original completion list.\n\nDefault: 300 ms",
      type: "integer",
      format: "uint64",
      minimum: 0.0,
    },
    confirm_quit: {
      description:
        "Whether or not to prompt the user to confirm before closing the application.\n\nDefault: false",
      type: "boolean",
    },
    current_line_highlight: {
      description:
        "How to highlight the current line in the editor.\n\nDefault: all",
      allOf: [
        {
          $ref: "#/definitions/CurrentLineHighlight",
        },
      ],
    },
    cursor_blink: {
      description: "Whether the cursor blinks in the editor.\n\nDefault: true",
      type: "boolean",
    },
    diagnostics: {
      type: "object",
      properties: {
        include_warnings: {
          description:
            "Whether to show warnings or not by default.\n\nDefault: true",
          type: "boolean",
        },
      },
    },
    double_click_in_multibuffer: {
      description:
        "What to do when multibuffer is double clicked in some of its excerpts (parts of singleton buffers).\n\nDefault: select",
      allOf: [
        {
          $ref: "#/definitions/DoubleClickInMultibuffer",
        },
      ],
    },
    drop_target_size: {
      description:
        "The size of the workspace split drop targets on the outer edges. Given as a fraction that will be multiplied by the smaller dimension of the workspace.\n\nDefault: `0.2` (20% of the smaller dimension of the workspace)",
      type: "number",
      format: "float",
    },
    enable_language_server: {
      description:
        "Whether to use language servers to provide code intelligence.\n\nDefault: true",
      default: null,
      type: "boolean",
    },
    ensure_final_newline_on_save: {
      description:
        "Whether or not to ensure there's a single newline at the end of a buffer when saving it.\n\nDefault: true",
      default: null,
      type: "boolean",
    },
    expand_excerpt_lines: {
      description:
        "How many lines to expand the multibuffer excerpts by default\n\nDefault: 3",
      type: "integer",
      format: "uint32",
      minimum: 0.0,
    },
    "experimental.theme_overrides": {
      description:
        "EXPERIMENTAL: Overrides for the current theme.\n\nThese values will override the ones on the current theme specified in `theme`.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/ThemeStyleContent",
        },
      ],
    },
    extend_comment_on_newline: {
      description:
        "Whether to start a new line with a comment when a previous line is a comment as well.\n\nDefault: true",
      default: null,
      type: "boolean",
    },
    features: {
      description: "The settings for enabling/disabling features.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/FeaturesContent",
        },
      ],
    },
    file_scan_exclusions: {
      description:
        'Completely ignore files matching globs from `file_scan_exclusions`\n\nDefault: [ "**/.git", "**/.svn", "**/.hg", "**/CVS", "**/.DS_Store", "**/Thumbs.db", "**/.classpath", "**/.settings" ]',
      default: null,
      type: "array",
      items: {
        type: "string",
      },
    },
    file_types: {
      description:
        "Settings for associating file extensions and filenames with languages.",
      default: {},
      type: "object",
      additionalProperties: {
        type: "array",
        items: {
          type: "string",
        },
      },
    },
    format_on_save: {
      description:
        "Whether or not to perform a buffer format before saving.\n\nDefault: on",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/OnSaveFormatter",
        },
      ],
    },
    formatter: {
      description: "How to perform a buffer format.\n\nDefault: auto",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/Formatter",
        },
      ],
    },
    git: {
      description: "Configuration for Git-related features",
      default: {
        git_gutter: null,
        gutter_debounce: null,
        inline_blame: null,
      },
      allOf: [
        {
          $ref: "#/definitions/GitSettings",
        },
      ],
    },
    gutter: {
      description: "Gutter related settings",
      allOf: [
        {
          $ref: "#/definitions/GutterContent",
        },
      ],
    },
    hard_tabs: {
      description:
        "Whether to indent lines using tab characters, as opposed to multiple spaces.\n\nDefault: false",
      default: null,
      type: "boolean",
    },
    hover_popover_enabled: {
      description:
        "Whether to show the informational hover box when moving the mouse over symbols in the editor.\n\nDefault: true",
      type: "boolean",
    },
    indent_guides: {
      description: "Indent guide related settings.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/IndentGuideSettings",
        },
      ],
    },
    inlay_hints: {
      description: "Inlay hint related settings.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/InlayHintSettings",
        },
      ],
    },
    inline_completions: {
      description: "The inline completion settings.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/InlineCompletionSettingsContent",
        },
      ],
    },
    journal: {
      type: "object",
      properties: {
        hour_format: {
          description:
            "What format to display the hours in.\n\nDefault: hour12",
          allOf: [
            {
              $ref: "#/definitions/HourFormat",
            },
          ],
        },
        path: {
          description:
            "The path of the directory where journal entries are stored.\n\nDefault: `~`",
          type: "string",
        },
      },
    },
    jupyter: {
      type: "object",
      allOf: [
        {
          $ref: "#/definitions/JupyterContent",
        },
      ],
      properties: {
        kernel_selections: {
          description:
            "Default kernels to select for each language.\n\nDefault: `{}`",
          type: "object",
          additionalProperties: {
            type: "string",
          },
        },
      },
    },
    language_models: {
      type: "object",
      properties: {
        anthropic: {
          $ref: "#/definitions/AnthropicSettingsContent",
        },
        copilot_chat: {
          $ref: "#/definitions/CopilotChatSettingsContent",
        },
        google: {
          $ref: "#/definitions/GoogleSettingsContent",
        },
        ollama: {
          $ref: "#/definitions/OllamaSettingsContent",
        },
        openai: {
          $ref: "#/definitions/OpenAiSettingsContent",
        },
        "zed.dev": {
          $ref: "#/definitions/ZedDotDevSettingsContent",
        },
      },
    },
    language_servers: {
      description:
        'The list of language servers to use (or disable) for this language.\n\nThis array should consist of language server IDs, as well as the following special tokens: - `"!<language_server_id>"` - A language server ID prefixed with a `!` will be disabled. - `"..."` - A placeholder to refer to the **rest** of the registered language servers for this language.\n\nDefault: ["..."]',
      default: null,
      type: "array",
      items: {
        type: "string",
      },
    },
    languages: {
      description: "The settings for individual languages.",
      default: {},
      type: "object",
      additionalProperties: {
        $ref: "#/definitions/LanguageSettingsContent",
      },
      $ref: "#/definitions/Languages",
    },
    line_indicator_format: {
      allOf: [
        {
          $ref: "#/definitions/LineIndicatorFormat",
        },
      ],
    },
    linked_edits: {
      description:
        "Whether to perform linked edits of associated ranges, if the language server supports it. For example, when editing opening <html> tag, the contents of the closing </html> tag will be edited as well.\n\nDefault: true",
      type: "boolean",
    },
    load_direnv: {
      description:
        "Configuration for how direnv configuration should be loaded",
      default: "shell_hook",
      allOf: [
        {
          $ref: "#/definitions/DirenvSettings",
        },
      ],
    },
    lsp: {
      description:
        "Configuration for language servers.\n\nThe following settings can be overridden for specific language servers: - initialization_options To override settings for a language, add an entry for that language server's name to the lsp value. Default: null",
      default: {},
      type: "object",
      additionalProperties: {
        $ref: "#/definitions/LspSettings",
      },
    },
    message_editor: {
      type: "object",
      properties: {
        auto_replace_emoji_shortcode: {
          description:
            "Whether to automatically replace emoji shortcodes with emoji characters. For example: typing `:wave:` gets replaced with `👋`.\n\nDefault: false",
          type: "boolean",
        },
      },
    },
    multi_cursor_modifier: {
      description: "The key to use for adding multiple cursors\n\nDefault: alt",
      allOf: [
        {
          $ref: "#/definitions/MultiCursorModifier",
        },
      ],
    },
    notification_panel: {
      type: "object",
      properties: {
        button: {
          description:
            "Whether to show the panel button in the status bar.\n\nDefault: true",
          type: "boolean",
        },
        default_width: {
          description: "Default width of the panel in pixels.\n\nDefault: 240",
          type: "number",
          format: "float",
        },
        dock: {
          description: "Where to dock the panel.\n\nDefault: left",
          allOf: [
            {
              $ref: "#/definitions/DockPosition",
            },
          ],
        },
      },
    },
    outline_panel: {
      type: "object",
      properties: {
        auto_fold_dirs: {
          description:
            "Whether to fold directories automatically when directory has only one directory inside.\n\nDefault: true",
          type: "boolean",
        },
        auto_reveal_entries: {
          description:
            "Whether to reveal it in the outline panel automatically, when a corresponding project entry becomes active. Gitignored entries are never auto revealed.\n\nDefault: true",
          type: "boolean",
        },
        button: {
          description:
            "Whether to show the outline panel button in the status bar.\n\nDefault: true",
          type: "boolean",
        },
        default_width: {
          description:
            "Customize default width (in pixels) taken by outline panel\n\nDefault: 240",
          type: "number",
          format: "float",
        },
        dock: {
          description: "The position of outline panel\n\nDefault: left",
          allOf: [
            {
              $ref: "#/definitions/OutlinePanelDockPosition",
            },
          ],
        },
        file_icons: {
          description:
            "Whether to show file icons in the outline panel.\n\nDefault: true",
          type: "boolean",
        },
        folder_icons: {
          description:
            "Whether to show folder icons or chevrons for directories in the outline panel.\n\nDefault: true",
          type: "boolean",
        },
        git_status: {
          description:
            "Whether to show the git status in the outline panel.\n\nDefault: true",
          type: "boolean",
        },
        indent_size: {
          description:
            "Amount of indentation (in pixels) for nested items.\n\nDefault: 20",
          type: "number",
          format: "float",
        },
      },
    },
    preferred_line_length: {
      description:
        "The column at which to soft-wrap lines, for buffers where soft-wrap is enabled.\n\nDefault: 80",
      default: null,
      type: "integer",
      format: "uint32",
      minimum: 0.0,
    },
    prettier: {
      description:
        "Zed's Prettier integration settings. Allows to enable/disable formatting with Prettier and configure default Prettier, used when no project-level Prettier installation is found.\n\nDefault: off",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/PrettierSettings",
        },
      ],
    },
    preview_tabs: {
      type: "object",
      properties: {
        enable_preview_from_code_navigation: {
          description:
            "Whether a preview tab gets replaced when code navigation is used to navigate away from the tab.\n\nDefault: false",
          type: "boolean",
        },
        enable_preview_from_file_finder: {
          description:
            "Whether to open tabs in preview mode when selected from the file finder.\n\nDefault: false",
          type: "boolean",
        },
        enabled: {
          description:
            "Whether to show opened editors as preview tabs. Preview tabs do not stay open, are reused until explicitly set to be kept open opened (via double-click or editing) and show file names in italic.\n\nDefault: true",
          type: "boolean",
        },
      },
    },
    private_files: {
      description:
        'Treat the files matching these globs as `.env` files. Default: [ "**/.env*" ]',
      type: "array",
      items: {
        type: "string",
      },
    },
    project_panel: {
      type: "object",
      properties: {
        auto_fold_dirs: {
          description:
            "Whether to fold directories automatically when directory has only one directory inside.\n\nDefault: false",
          type: "boolean",
        },
        auto_reveal_entries: {
          description:
            "Whether to reveal it in the project panel automatically, when a corresponding project entry becomes active. Gitignored entries are never auto revealed.\n\nDefault: true",
          type: "boolean",
        },
        button: {
          description:
            "Whether to show the project panel button in the status bar.\n\nDefault: true",
          type: "boolean",
        },
        default_width: {
          description:
            "Customize default width (in pixels) taken by project panel\n\nDefault: 240",
          type: "number",
          format: "float",
        },
        dock: {
          description: "The position of project panel\n\nDefault: left",
          allOf: [
            {
              $ref: "#/definitions/ProjectPanelDockPosition",
            },
          ],
        },
        file_icons: {
          description:
            "Whether to show file icons in the project panel.\n\nDefault: true",
          type: "boolean",
        },
        folder_icons: {
          description:
            "Whether to show folder icons or chevrons for directories in the project panel.\n\nDefault: true",
          type: "boolean",
        },
        git_status: {
          description:
            "Whether to show the git status in the project panel.\n\nDefault: true",
          type: "boolean",
        },
        indent_size: {
          description:
            "Amount of indentation (in pixels) for nested items.\n\nDefault: 20",
          type: "number",
          format: "float",
        },
        scrollbar: {
          description: "Scrollbar-related settings",
          allOf: [
            {
              $ref: "#/definitions/ScrollbarSettingsContent",
            },
          ],
        },
      },
    },
    proxy: {
      type: "string",
    },
    redact_private_values: {
      description:
        "Hide the values of variables in `private` files, as defined by the private_files setting. This only changes the visual representation, the values are still present in the file and can be selected / copied / pasted\n\nDefault: false",
      type: "boolean",
    },
    relative_line_numbers: {
      description:
        "Whether the line numbers on editors gutter are relative or not.\n\nDefault: false",
      type: "boolean",
    },
    remove_trailing_whitespace_on_save: {
      description:
        "Whether or not to remove any trailing whitespace from lines of a buffer before saving it.\n\nDefault: true",
      default: null,
      type: "boolean",
    },
    restore_on_startup: {
      description:
        "Controls previous session restoration in freshly launched Zed instance. Values: none, last_workspace, last_session Default: last_session",
      allOf: [
        {
          $ref: "#/definitions/RestoreOnStartupBehavior",
        },
      ],
    },
    scroll_beyond_last_line: {
      description:
        "Whether the editor will scroll beyond the last line.\n\nDefault: one_page",
      allOf: [
        {
          $ref: "#/definitions/ScrollBeyondLastLine",
        },
      ],
    },
    scroll_sensitivity: {
      description:
        "Scroll sensitivity multiplier. This multiplier is applied to both the horizontal and vertical delta values while scrolling.\n\nDefault: 1.0",
      type: "number",
      format: "float",
    },
    scrollbar: {
      description: "Scrollbar related settings",
      allOf: [
        {
          $ref: "#/definitions/ScrollbarContent",
        },
      ],
    },
    search_wrap: {
      description:
        "Whether the editor search results will loop\n\nDefault: true",
      type: "boolean",
    },
    seed_search_query_from_cursor: {
      description:
        "When to populate a new search's query based on the text under the cursor.\n\nDefault: always",
      allOf: [
        {
          $ref: "#/definitions/SeedQuerySetting",
        },
      ],
    },
    server_url: {
      type: "string",
    },
    session: {
      description: "Configuration for session-related features",
      default: {
        restore_unsaved_buffers: true,
      },
      allOf: [
        {
          $ref: "#/definitions/SessionSettings",
        },
      ],
    },
    show_call_status_icon: {
      description:
        "Whether or not to show the call status icon in the status bar.\n\nDefault: true",
      type: "boolean",
    },
    show_completion_documentation: {
      description:
        "Whether to display inline and alongside documentation for items in the completions menu.\n\nDefault: true",
      type: "boolean",
    },
    show_completions_on_input: {
      description:
        "Whether to pop the completions menu while typing in an editor without explicitly requesting it.\n\nDefault: true",
      type: "boolean",
    },
    show_inline_completions: {
      description:
        "Controls whether inline completions are shown immediately (true) or manually by triggering `editor::ShowInlineCompletion` (false).\n\nDefault: true",
      default: null,
      type: "boolean",
    },
    show_signature_help_after_edits: {
      description:
        "Whether to show the signature help pop-up after completions or bracket pairs inserted.\n\nDefault: true",
      type: "boolean",
    },
    show_whitespaces: {
      description: "Whether to show tabs and spaces in the editor.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/ShowWhitespaceSetting",
        },
      ],
    },
    show_wrap_guides: {
      description:
        "Whether to show wrap guides in the editor. Setting this to true will show a guide at the 'preferred_line_length' value if softwrap is set to 'preferred_line_length', and will show any additional guides as specified by the 'wrap_guides' setting.\n\nDefault: true",
      default: null,
      type: "boolean",
    },
    soft_wrap: {
      description: "How to soft-wrap long lines of text.\n\nDefault: none",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/SoftWrap",
        },
      ],
    },
    ssh_connections: {
      type: "array",
      items: {
        $ref: "#/definitions/SshConnection",
      },
    },
    tab_bar: {
      type: "object",
      properties: {
        show: {
          description:
            "Whether or not to show the tab bar in the editor.\n\nDefault: true",
          type: "boolean",
        },
        show_nav_history_buttons: {
          description:
            "Whether or not to show the navigation history buttons in the tab bar.\n\nDefault: true",
          type: "boolean",
        },
      },
    },
    tab_size: {
      description: "How many columns a tab should occupy.\n\nDefault: 4",
      default: null,
      type: "integer",
      format: "uint32",
      minimum: 1.0,
    },
    tabs: {
      type: "object",
      properties: {
        close_position: {
          description:
            "Position of the close button in a tab.\n\nDefault: right",
          allOf: [
            {
              $ref: "#/definitions/ClosePosition",
            },
          ],
        },
        file_icons: {
          description:
            "Whether to show the file icon for a tab.\n\nDefault: true",
          type: "boolean",
        },
        git_status: {
          description:
            "Whether to show the Git file status on a tab item.\n\nDefault: false",
          type: "boolean",
        },
      },
    },
    task: {
      type: "object",
      properties: {
        show_status_indicator: {
          description:
            "Whether to show task status indicator in the status bar. Default: true",
          type: "boolean",
        },
      },
    },
    tasks: {
      description: "Task configuration for this language.\n\nDefault: {}",
      allOf: [
        {
          $ref: "#/definitions/LanguageTaskConfig",
        },
      ],
    },
    telemetry: {
      type: "object",
      properties: {
        diagnostics: {
          description: "Send debug info like crash reports.\n\nDefault: true",
          type: "boolean",
        },
        metrics: {
          description:
            "Send anonymized usage data like what languages you're using Zed with.\n\nDefault: true",
          type: "boolean",
        },
      },
    },
    terminal: {
      type: "object",
      properties: {
        alternate_scroll: {
          description:
            "Sets whether Alternate Scroll mode (code: ?1007) is active by default. Alternate Scroll mode converts mouse scroll events into up / down key presses when in the alternate screen (e.g. when running applications like vim or  less). The terminal can still set and unset this mode.\n\nDefault: off",
          allOf: [
            {
              $ref: "#/definitions/AlternateScroll",
            },
          ],
        },
        blinking: {
          description:
            "Sets the cursor blinking behavior in the terminal.\n\nDefault: terminal_controlled",
          allOf: [
            {
              $ref: "#/definitions/TerminalBlink",
            },
          ],
        },
        button: {
          description:
            "Whether to show the terminal button in the status bar.\n\nDefault: true",
          type: "boolean",
        },
        copy_on_select: {
          description:
            "Whether or not selecting text in the terminal will automatically copy to the system clipboard.\n\nDefault: false",
          type: "boolean",
        },
        default_height: {
          description:
            "Default height when the terminal is docked to the bottom.\n\nDefault: 320",
          type: "number",
          format: "float",
        },
        default_width: {
          description:
            "Default width when the terminal is docked to the left or right.\n\nDefault: 640",
          type: "number",
          format: "float",
        },
        detect_venv: {
          description:
            'Activates the python virtual environment, if one is found, in the terminal\'s working directory (as resolved by the working_directory setting). Set this to "off" to disable this behavior.\n\nDefault: on',
          allOf: [
            {
              $ref: "#/definitions/VenvSettings",
            },
          ],
        },
        dock: {
          $ref: "#/definitions/TerminalDockPosition",
        },
        env: {
          description:
            "Any key-value pairs added to this list will be added to the terminal's environment. Use `:` to separate multiple values.\n\nDefault: {}",
          type: "object",
          additionalProperties: {
            type: "string",
          },
        },
        font_fallbacks: {
          description:
            "Sets the terminal's font fallbacks.\n\nIf this option is not included, the terminal will default to matching the buffer's font fallbacks.",
          type: "array",
          items: {
            type: "string",
          },
          $ref: "#/definitions/FontFallbacks",
        },
        font_family: {
          description:
            "Sets the terminal's font family.\n\nIf this option is not included, the terminal will default to matching the buffer's font family.",
          type: "string",
          $ref: "#/definitions/FontFamilies",
        },
        font_features: {
          $ref: "#/definitions/FontFeatures",
        },
        font_size: {
          description:
            "Sets the terminal's font size.\n\nIf this option is not included, the terminal will default to matching the buffer's font size.",
          type: "number",
          format: "float",
        },
        font_weight: {
          description:
            "Sets the terminal's font weight in CSS weight units 0-900.",
          type: "number",
          format: "float",
        },
        line_height: {
          description:
            "Sets the terminal's line height.\n\nDefault: comfortable",
          allOf: [
            {
              $ref: "#/definitions/TerminalLineHeight",
            },
          ],
        },
        max_scroll_history_lines: {
          description:
            'The maximum number of lines to keep in the scrollback history. Maximum allowed value is 100_000, all values above that will be treated as 100_000. 0 disables the scrolling. Existing terminals will not pick up this change until they are recreated. See <a href="https://github.com/alacritty/alacritty/blob/cb3a79dbf6472740daca8440d5166c1d4af5029e/extra/man/alacritty.5.scd?plain=1#L207-L213">Alacritty documentation</a> for more information.\n\nDefault: 10_000',
          type: "integer",
          format: "uint",
          minimum: 0.0,
        },
        option_as_meta: {
          description:
            "Sets whether the option key behaves as the meta key.\n\nDefault: true",
          type: "boolean",
        },
        shell: {
          description:
            "What shell to use when opening a terminal.\n\nDefault: system",
          allOf: [
            {
              $ref: "#/definitions/Shell",
            },
          ],
        },
        toolbar: {
          description: "Toolbar related settings",
          allOf: [
            {
              $ref: "#/definitions/ToolbarContent",
            },
          ],
        },
        working_directory: {
          description:
            "What working directory to use when launching the terminal\n\nDefault: current_project_directory",
          allOf: [
            {
              $ref: "#/definitions/WorkingDirectory",
            },
          ],
        },
      },
    },
    theme: {
      description: "The name of the Zed theme to use.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/ThemeSelection",
        },
      ],
    },
    toolbar: {
      description: "Toolbar related settings",
      allOf: [
        {
          $ref: "#/definitions/ToolbarContent2",
        },
      ],
    },
    ui_font_fallbacks: {
      description: "The font fallbacks to use for rendering in the UI.",
      default: null,
      type: "array",
      items: {
        type: "string",
      },
      $ref: "#/definitions/FontFallbacks",
    },
    ui_font_family: {
      description: "The name of a font to use for rendering in the UI.",
      default: null,
      type: "string",
      $ref: "#/definitions/FontFamilies",
    },
    ui_font_features: {
      description: "The OpenType features to enable for text in the UI.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/FontFeatures",
        },
      ],
    },
    ui_font_size: {
      description: "The default font size for text in the UI.",
      default: null,
      type: "number",
      format: "float",
    },
    ui_font_weight: {
      description: "The weight of the UI font in CSS units from 100 to 900.",
      default: null,
      type: "number",
      format: "float",
    },
    "unstable.ui_density": {
      description: "UNSTABLE: Expect many elements to be broken.",
      default: null,
      allOf: [
        {
          $ref: "#/definitions/UiDensity",
        },
      ],
    },
    use_auto_surround: {
      description:
        "Whether to automatically surround text with characters for you. For example, when you select text and type (, Zed will automatically surround text with ().\n\nDefault: true",
      type: "boolean",
    },
    use_autoclose: {
      description:
        "Whether to automatically type closing characters for you. For example, when you type (, Zed will automatically add a closing ) at the correct position.\n\nDefault: true",
      type: "boolean",
    },
    use_on_type_format: {
      description:
        'Whether to use additional LSP queries to format (and amend) the code after every "trigger" symbol input, defined by LSP server capabilities.\n\nDefault: true',
      type: "boolean",
    },
    use_system_path_prompts: {
      description:
        "Whether to use the system provided dialogs for Open and Save As. When set to false, Zed will use the built-in keyboard-first pickers.\n\nDefault: true",
      type: "boolean",
    },
    vertical_scroll_margin: {
      description:
        "The number of lines to keep above/below the cursor when auto-scrolling.\n\nDefault: 3.",
      type: "number",
      format: "float",
    },
    vim: {
      type: "object",
      properties: {
        custom_digraphs: {
          type: "object",
          additionalProperties: {
            type: "string",
          },
        },
        use_multiline_find: {
          type: "boolean",
        },
        use_smartcase_find: {
          type: "boolean",
        },
        use_system_clipboard: {
          $ref: "#/definitions/UseSystemClipboard",
        },
      },
    },
    vim_mode: {
      type: "boolean",
    },
    when_closing_with_no_tabs: {
      description:
        'Whether to close the window when using \'close active item\' on a workspace with no tabs\n\nDefault: auto ("on" on macOS, "off" otherwise)',
      allOf: [
        {
          $ref: "#/definitions/CloseWindowWhenNoItems",
        },
      ],
    },
    wrap_guides: {
      description:
        "Character counts at which to show wrap guides in the editor.\n\nDefault: []",
      default: null,
      type: "array",
      items: {
        type: "integer",
        format: "uint",
        minimum: 0.0,
      },
    },
  },
};
