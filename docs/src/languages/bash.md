# Bash

Bash language support in Zed is provided by the community-maintained [Basher extension](https://github.com/d1y/bash.zed).
Report issues to: [https://github.com/d1y/bash.zed/issues](https://github.com/d1y/bash.zed/issues)

- Tree Sitter: [tree-sitter/tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash)
- Language Server: [bash-lsp/bash-language-server](https://github.com/bash-lsp/bash-language-server)

## Configuration

The bash-language-server support shellcheck. But you need to install it manually:

```sh
# macOS
brew install shellcheck

# Ubuntu/Debian
sudo apt-get install shellcheck

# Arch Linux
pacman -S shellcheck
```

If you wish to customize the warnings/errors reported you just need to create a `.shellcheckrc` file. You can do this in the root of your project or in your home directory (`~/.shellcheckrc`). See: [shellcheck documentation](https://github.com/koalaman/shellcheck/wiki/Ignore#ignoring-one-or-more-types-of-errors-forever) for more.
