# Language Servers in Zed

TBD: Explain how Language servers are used in zed

- differentiate between [tree-sitter](/docs/tree-sitter)
- explain how we download them
- how they can be found locally
  - https://zed.dev/docs/configuring-zed#direnv-integration

TBD: Explain how to choose between multiple language servers
TBD: Cross link explanation to Python, TypeScript, Ruby, PHP, etc.

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "..."]
    }
  }
}
```

## inlayHints

TBD: Explain what inlay hints are.
Link: https://zed.dev/docs/configuring-zed#inlay-hints

## Other Actions:

TBD: Document the type of actions for language servers

- Code Completion
- Hover
- Jump to Def
- Workplace Symbols
- Find References
- Diagnostics

https://langserver.org/
