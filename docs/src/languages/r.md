# R

R support is available via multiple R Zed extensions:

- [ocsmit/zed-r](https://github.com/ocsmit/zed-r)

  - Tree-sitter: [r-lib/tree-sitter-r](https://github.com/r-lib/tree-sitter-r)
  - Language-Server: [REditorSupport/languageserver](https://github.com/REditorSupport/languageserver)

- [posit-dev/air](https://github.com/posit-dev/air/tree/main/editors/zed)
  - Language-Server: [posit-dev/air](https://github.com/posit-dev/air)

## Installation

1. [Download and Install R](https://cloud.r-project.org/).
2. Install the R packages `languageserver` and `lintr`:

```R
install.packages("languageserver")
install.packages("lintr")
```

3. Install the [ocsmit/zed-r](https://github.com/ocsmit/zed-r) through Zed's extensions manager.

For example on macOS:

```sh
brew install --cask r
Rscript --version
Rscript -e 'options(repos = "https://cran.rstudio.com/"); install.packages("languageserver")'
Rscript -e 'options(repos = "https://cran.rstudio.com/"); install.packages("lintr")'
Rscript -e 'packageVersion("languageserver")'
Rscript -e 'packageVersion("lintr")'
```

## Configuration

### Linting

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

### Formatting

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
