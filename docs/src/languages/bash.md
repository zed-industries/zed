# Bash

Bash language support in Zed is provided by the community-maintained [Basher extension](https://github.com/d1y/bash.zed).
Report issues to: [https://github.com/d1y/bash.zed/issues](https://github.com/d1y/bash.zed/issues)

- Tree Sitter: [tree-sitter/tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash)
- Language Server: [bash-lsp/bash-language-server](https://github.com/bash-lsp/bash-language-server)

## Configuration

When `shellcheck` is available `bash-language-server` will use it internally to provide diagnostics.

### Install `shellcheck`:

```sh
brew install shellcheck             # macOS (HomeBrew)
apt-get install shellcheck          # Ubuntu/Debian
pacman -S shellcheck                # ArchLinux
dnf install shellcheck              # Fedora
yum install shellcheck              # CentOS/RHEL
zypper install shellcheck           # openSUSE
choco install shellcheck            # Windows (Chocolatey)
```

And verify it is available from your path:

```sh
which shellcheck
shellcheck --version
```

If you wish to customize the warnings/errors reported you just need to create a `.shellcheckrc` file. You can do this in the root of your project or in your home directory (`~/.shellcheckrc`). See: [shellcheck documentation](https://github.com/koalaman/shellcheck/wiki/Ignore#ignoring-one-or-more-types-of-errors-forever) for more.

### See also:

- [Zed Docs: Language Support: Shell Scripts](./sh.md)
