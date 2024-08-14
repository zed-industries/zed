# Luau

[Luau](https://luau-lang.org/) is a fast, small, safe, gradually typed embeddable scripting language derived from Lua. Luau was developed by Roblox and available under the MIT license.

Luau language support in Zed is provided by the community-maintained [Luau extension](https://github.com/4teapo/zed-luau).
Report issues to: [https://github.com/4teapo/zed-luau/issues](https://github.com/4teapo/zed-luau/issues)

- Tree Sitter: [tree-sitter-grammars/tree-sitter-luau](https://github.com/tree-sitter-grammars/tree-sitter-luau)
- Language Server: [JohnnyMorganz/luau-lsp](https://github.com/JohnnyMorganz/luau-lsp)

## Configuration

Configuration instructions are available in the [Luau Zed Extension README](https://github.com/4teapo/zed-luau).

## Formatting

To support automatically formatting your code, you can use [JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua), a Lua code formatter.

Install with:

```sh
# MacOS via HomeBrew
brew install stylua
# Or via Cargo
cargo install stylua --features lua52,lua53,lua54,luau
```

Then add the following to your Zed `settings.json`:

```json
  "languages": {
		"Luau": {
			"formatter": {
				"external": {
					"command": "stylua",
					"arguments": ["-"]
				}
			}
		}
	}
```
