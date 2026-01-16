# R

R support is available via multiple R Zed extensions:

- [ocsmit/zed-r](https://github.com/ocsmit/zed-r)
  - Tree-sitter: [r-lib/tree-sitter-r](https://github.com/r-lib/tree-sitter-r)
  - Language-Server: [REditorSupport/languageserver](https://github.com/REditorSupport/languageserver)

- [posit-dev/air](https://github.com/posit-dev/air/tree/main/editors/zed)
  - Formatter: [posit-dev/air](https://posit-dev.github.io/air/)

## Installation

1. [Download and Install R](https://cloud.r-project.org/).
2. Install the R packages `languageserver` and `lintr`:

```R
install.packages("languageserver")
install.packages("lintr")
```

3. Install the [R](https://github.com/ocsmit/zed-r) extension through Zed's extensions manager for basic R language support (syntax highlighting, tree-sitter support) and for [REditorSupport/languageserver](https://github.com/REditorSupport/languageserver) support.

4. Install the [Air](https://posit-dev.github.io/air/) extension through Zed's extensions manager for R code formatting via Air.

## Linting

`REditorSupport/languageserver` bundles support for [r-lib/lintr](https://github.com/r-lib/lintr) as a linter. This can be configured via the use of a `.lintr` inside your project (or in your home directory for global defaults).

```r
linters: linters_with_defaults(
    line_length_linter(120),
    commented_code_linter = NULL
  )
exclusions: list(
    "inst/doc/creating_linters.R" = 1,
    "inst/example/bad.R",
    "tests/testthat/exclusions-test"
  )
```

Or exclude it from linting anything,

```r
exclusions: list(".")
```

See [Using lintr](https://lintr.r-lib.org/articles/lintr.html) for a complete list of options,

## Formatting

### Air

[Air](https://posit-dev.github.io/air/) provides code formatting for R, including support for format-on-save. The [Air documentation for Zed](https://posit-dev.github.io/air/editor-zed.html) contains the most up-to-date advice for running Air in Zed.

Ensure that you have installed both the [ocsmit/zed-r](https://github.com/ocsmit/zed-r) extension (for general R language awareness in Zed) and the [Air](https://posit-dev.github.io/air/) extension.

Enable Air in your `settings.json`:

```json [settings]
{
  "languages": {
    "R": {
      "language_servers": ["air"]
    }
  }
}
```

If you use the `"r_language_server"` from `REditorSupport/languageserver`, but would still like to use Air for formatting, use the following configuration:

```json [settings]
{
  "languages": {
    "R": {
      "language_servers": ["air", "r_language_server"],
      "use_on_type_format": false
    }
  }
}
```

Note that `"air"` must come first in this list, otherwise [r-lib/styler](https://github.com/r-lib/styler) will be invoked via `"r_language_server"`.

`"r_language_server"` provides on-type-formatting that differs from Air's formatting rules. To avoid this entirely and let Air be fully in charge of formatting your R files, also set `"use_on_type_format": false` as shown above.

#### Configuring Air

Air is minimally configurable via an `air.toml` file placed in the root directory of your project:

```toml
[format]
line-width = 80
indent-width = 2
```

For more details, refer to the Air documentation about [configuration](https://posit-dev.github.io/air/configuration.html).

### Styler

`REditorSupport/languageserver` bundles support for [r-lib/styler](https://github.com/r-lib/styler) as a formatter. See [Customizing Styler](https://cran.r-project.org/web/packages/styler/vignettes/customizing_styler.html) for more information on how to customize its behavior.

<!--
TBD: Get this working

### REditorSupport/languageserver Configuration

You can configure the [R languageserver settings](https://github.com/REditorSupport/languageserver#settings) via Zed Project Settings `.zed/settings.json` or Zed User Settings `~/.config/zed/settings.json`:

For example to disable Lintr linting and suppress code snippet suggestions (both enabled by default):

```json [settings]
{
  "lsp": {
    "r_language_server": {
      "settings": {
        "r": {
          "lsp": {
            "diagnostics": false,
            "snippet_support": false
          }
        }
      }
    }
  }
}
```

-->

<!--
TBD: R REPL Docs

## REPL

### Ark Installation

To use the Zed REPL with R you need to install [Ark](https://github.com/posit-dev/ark), an R Kernel for Jupyter applications.
You can down the latest version from the [Ark GitHub Releases](https://github.com/posit-dev/ark/releases) and then extract the `ark` binary to a directory in your `PATH`.

For example to install the latest non-debug build:

```sh
# macOS
cd /tmp
curl -L -o ark-latest-darwin.zip \
    $(curl -s "https://api.github.com/repos/posit-dev/ark/releases/latest" | \
    jq -r '.assets[] | select(.name | contains("darwin-universal") and (contains("debug") | not)) | .browser_download_url')
unzip ark-latest-darwin.zip ark
sudo mv /tmp/ark /usr/local/bin/
```

```sh
# Linux X86_64
cd /tmp
curl -L -o ark-latest-linux.zip \
    $(curl -s "https://api.github.com/repos/posit-dev/ark/releases/latest" \
        | jq -r '.assets[] | select(.name | contains("linux-x64") and (contains("debug") | not)) | .browser_download_url'
    )
unzip ark-latest-linux.zip ark
sudo mv /tmp/ark /usr/local/bin/
```

-->
