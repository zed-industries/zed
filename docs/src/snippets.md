# Snippets

Place your snippet files in `~/.config/zed/snippets` directory.

Ex: If you want to create snippets that targeted JavaScript files, your file would be `javascript.json` and it might contain:

```json
{
  "Log to the console": {
    "prefix": "log",
    "body": ["console.log($1);", "$0"],
    "description": "Log to the console"
  },
  "Log warning to console": {
    "prefix": "warn",
    "body": ["console.warn($1);", "$0"],
    "description": "Log warning to the console"
  },
  "Log error to console": {
    "prefix": "error",
    "body": ["console.error($1);", "$0"],
    "description": "Log error to the console"
  },
  "Throw Exception": {
    "prefix": "throw",
    "body": ["throw new Error(\"$1\");", "$0"],
    "description": "Throw Exception"
  }
}
```

For more configuration information, see the [`simple-completion-language-server` instructions](https://github.com/zed-industries/simple-completion-language-server/tree/main).
