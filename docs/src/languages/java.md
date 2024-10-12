# Java

Java language support in Zed is provided the [zed Java extension](https://github.com/zed-extensions/java) and [zed-java-eclipse-jdtls](https://github.com/ABckh/zed-java-eclipse-jdtls).

## Java extension for Zed

- Tree Sitter: [tree-sitter/tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java)
- Language Server: [eclipse-jdtls/eclipse.jdt.ls](https://github.com/eclipse-jdtls/eclipse.jdt.ls)

Report issues to: [https://github.com/zed-extensions/java/issues](https://github.com/zed-extensions/java/issues).

### Configuration

You can optionally configure the Java home that JDTLS (the language server) uses
in your Zed settings like so:

```json
{
  "lsp": {
    "jdtls": {
      "settings": {
        "java_home": "/path/to/jdk"
      }
    }
  }
}
```

For any updates, please refer to the [README.md](https://github.com/zed-extensions/java).

## zed-java-eclipse-jdtls

- Tree Sitter: [tree-sitter/tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java)
- Language Server: [Eclipse JDTLS](https://github.com/eclipse-jdtls/eclipse.jdt.ls)

Report issues to: [https://github.com/ABckh/zed-java-eclipse-jdtls/issues](https://github.com/ABckh/zed-java-eclipse-jdtls/issues).

### Configuration

#### Configuring LSP Options

To configure the LSP options, you can utilize the same fields provided by VSCode's Java support documentation. These fields are documented in detail and cover a wide range of customization options. Follow these steps to configure your LSP:

1. **Open the Zed Configuration File:**
   Open configuration file for Zed (`CMD + ,`). Edit the configuration to include your desired LSP options.

2. **Set Custom Options:**
   Add the relevant fields in the configuration file to match the options provided in the [VSCode Java support documentation](https://github.com/redhat-developer/vscode-java#configuration).

```json
{
  "lsp": {
    "java": {
        "java.jdt.ls.lombokSupport.enabled:": true
    }
  }
}
```

#### Using a Custom JDTLS Binary

If you have a custom JDTLS binary that you would like to use with the Zed extension, follow these steps:

1. **Locate the JDTLS Binary:**
   By default, the Zed extension uses the JDTLS binary located at:
   - **Linux:** `~/.local/share/zed/extensions/work/java-eclipse-jdtls/eclipse.jdt.ls/bin/jdtls`
   - **macOS:** `~/Library/Application Support/Zed/extensions/work/java-eclipse-jdtls/eclipse.jdt.ls/bin/jdtls`
   - **Windows:** `%APPDATA%/Zed/extensions/work/java-eclipse-jdtls/eclipse.jdt.ls/bin/jdtls`

2. **Create a Symlink:**
   Symlink the default JDTLS binary path to your custom JDTLS binary. This ensures that when Zed starts the JDTLS, it will use your custom version.

Example for Linux:
```bash
ln -s /path/to/your/custom/jdtls ~/.local/share/zed/extensions/work/java-eclipse-jdtls/eclipse.jdt.ls/bin/jdtls
```

For any updates. please refer to the [README.md](https://github.com/ABckh/zed-java-eclipse-jdtls).
