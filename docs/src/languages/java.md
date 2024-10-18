# Java

There are two extensions that provide Java language support for Zed:

- Zed Java: [zed-extensions/java](https://github.com/zed-extensions/java) and
- Java with Eclipse JDTLS: [zed-java-eclipse-jdtls](https://github.com/ABckh/zed-java-eclipse-jdtls).

Both use:

- Tree Sitter: [tree-sitter/tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java)
- Language Server: [eclipse-jdtls/eclipse.jdt.ls](https://github.com/eclipse-jdtls/eclipse.jdt.ls)

## Install OpenJDK

You will need to install a Java runtime (OpenJDK).

- MacOS: `brew install openjdk`
- Ubuntu: `sudo add-apt-repository ppa:openjdk-23 && sudo apt-get install openjdk-23`
- Windows: `choco install openjdk`
- Arch Linux: `sudo pacman -S jre-openjdk-headless`

Or manually download and install [OpenJDK 23](https://jdk.java.net/23/).

## Extension Install

You can install either by opening {#action zed::Extensions}({#kb zed::Extensions}) and searching for `java`.

We recommend you install one or the other and not both.

## Settings / Initialization Options

Both extensions will automatically download the language server, see: [Manual JDTLS Install](#manual-jdts-install) below if you'd prefer to manage that yourself.

For available `initialization_options` please see the [Initialize Request section of the Eclipse.jdt.ls Wiki](https://github.com/eclipse-jdtls/eclipse.jdt.ls/wiki/Running-the-JAVA-LS-server-from-the-command-line#initialize-request).

You can add these customizations to your Zed Settings by launching {#action zed::OpenSettings}({#kb zed::OpenSettings}) or by using a `.zed/setting.json` inside your project.

### Zed Java Settings

```json
{
  "lsp": {
    "jdtls": {
      "settings": {
        "version": "1.40.0", // jdtls version to download and use
        "classpath": "/path/to/classes.jar:/path/to/more/classes/"
      },
      "initialization_options": {}
    }
  }
}
```

### Java with Eclipse JDTLS settings

```json
{
  "lsp": {
    "java": {
      "settings": {},
      "initialization_options": {}
    }
  }
}
```

## Example Configs

### Zed Java Initialization Options

There are also many more options you can pass directly to the language server, for example:

```json
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

### Java with Eclipse JTDLS Configuration {#zed-java-eclipse-configuration}

Configuration options match those provided in the [redhat-developer/vscode-java extension](https://github.com/redhat-developer/vscode-java#supported-vs-code-settings).

For example, to enable [Lombok Support](https://github.com/redhat-developer/vscode-java/wiki/Lombok-support):

```json
{
  "lsp": {
    "java": {
      "settings": {
        "java.jdt.ls.lombokSupport.enabled:": true
      }
    }
  }
}
```

## Manual JDTLS Install

If you prefer, you can install JDTLS yourself and both extensions can be configured to use that instead.

- MacOS: `brew install jdtls`
- Arch: [`jdtls` from AUR](https://aur.archlinux.org/packages/jdtls)

Or manually download install:

- [JDTLS Milestone Builds](http://download.eclipse.org/jdtls/milestones/) (updated every two weeks)
- [JDTLS Snapshot Builds](https://download.eclipse.org/jdtls/snapshots/) (frequent updates)

## See also

- [Zed Java Readme](https://github.com/zed-extensions/java)
- [Java with Eclipse JDTLS Readme](https://github.com/ABckh/zed-java-eclipse-jdtls)

## Support

If you have issues with either of these plugins, please open issues on their respective repositories:

- [Zed Java Issues](https://github.com/zed-extensions/java/issues)
- [Java with Eclipse JDTLS Issues](https://github.com/ABckh/zed-java-eclipse-jdtls/issues)
