# Java

Java language support in Zed is provided by:

- Zed Java: [zed-extensions/java](https://github.com/zed-extensions/java)
- Tree-sitter: [tree-sitter/tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java)
- Language Server: [eclipse-jdtls/eclipse.jdt.ls](https://github.com/eclipse-jdtls/eclipse.jdt.ls)

## Install OpenJDK

You will need to install a Java runtime (OpenJDK).

- macOS: `brew install openjdk`
- Ubuntu: `sudo add-apt-repository ppa:openjdk-23 && sudo apt-get install openjdk-23`
- Windows: `choco install openjdk`
- Arch Linux: `sudo pacman -S jre-openjdk-headless`

Or manually download and install [OpenJDK 23](https://jdk.java.net/23/).

## Extension Install

You can install either by opening {#action zed::Extensions}({#kb zed::Extensions}) and searching for `java`.

## Settings / Initialization Options

The extension will automatically download the language server, see: [Manual JDTLS Install](#manual-jdts-install) below if you'd prefer to manage that yourself.

For available `initialization_options` please see the [Initialize Request section of the Eclipse.jdt.ls Wiki](https://github.com/eclipse-jdtls/eclipse.jdt.ls/wiki/Running-the-JAVA-LS-server-from-the-command-line#initialize-request).

You can add these customizations to your Zed Settings by launching {#action zed::OpenSettings}({#kb zed::OpenSettings}) or by using a `.zed/setting.json` inside your project.

### Zed Java Settings

```json [settings]
{
  "lsp": {
    "jdtls": {
      "initialization_options": {}
    }
  }
}
```

## Example Configs

### JDTLS Binary

By default, zed will look in your `PATH` for a `jdtls` binary, if you wish to specify an explicit binary you can do so via settings:

```json [settings]
  "lsp": {
    "jdtls": {
      "binary": {
        "path": "/path/to/java/bin/jdtls",
        // "arguments": [],
        // "env": {},
        "ignore_system_version": true
      }
    }
  }
```

### Zed Java Initialization Options

There are also many more options you can pass directly to the language server, for example:

```json [settings]
{
  "lsp": {
    "jdtls": {
      "initialization_options": {
        "bundles": [],
        "workspaceFolders": ["file:///home/snjeza/Project"],
        "settings": {
          "java": {
            "home": "/usr/local/jdk-9.0.1",
            "errors": {
              "incompleteClasspath": {
                "severity": "warning"
              }
            },
            "configuration": {
              "updateBuildConfiguration": "interactive",
              "maven": {
                "userSettings": null
              }
            },
            "trace": {
              "server": "verbose"
            },
            "import": {
              "gradle": {
                "enabled": true
              },
              "maven": {
                "enabled": true
              },
              "exclusions": [
                "**/node_modules/**",
                "**/.metadata/**",
                "**/archetype-resources/**",
                "**/META-INF/maven/**",
                "/**/test/**"
              ]
            },
            "jdt": {
              "ls": {
                "lombokSupport": {
                  "enabled": false // Set this to true to enable lombok support
                }
              }
            },
            "referencesCodeLens": {
              "enabled": false
            },
            "signatureHelp": {
              "enabled": false
            },
            "implementationsCodeLens": {
              "enabled": false
            },
            "format": {
              "enabled": true
            },
            "saveActions": {
              "organizeImports": false
            },
            "contentProvider": {
              "preferred": null
            },
            "autobuild": {
              "enabled": false
            },
            "completion": {
              "favoriteStaticMembers": [
                "org.junit.Assert.*",
                "org.junit.Assume.*",
                "org.junit.jupiter.api.Assertions.*",
                "org.junit.jupiter.api.Assumptions.*",
                "org.junit.jupiter.api.DynamicContainer.*",
                "org.junit.jupiter.api.DynamicTest.*"
              ],
              "importOrder": ["java", "javax", "com", "org"]
            }
          }
        }
      }
    }
  }
}
```

## Manual JDTLS Install

If you prefer, you can install JDTLS yourself and the extension can be configured to use that instead.

- macOS: `brew install jdtls`
- Arch: [`jdtls` from AUR](https://aur.archlinux.org/packages/jdtls)

Or manually download install:

- [JDTLS Milestone Builds](http://download.eclipse.org/jdtls/milestones/) (updated every two weeks)
- [JDTLS Snapshot Builds](https://download.eclipse.org/jdtls/snapshots/) (frequent updates)

## See also

- [Zed Java Repo](https://github.com/zed-extensions/java)
- [Zed Java Issues](https://github.com/zed-extensions/java/issues)
