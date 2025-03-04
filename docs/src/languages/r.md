# R

R support is available through the [R extension](https://github.com/ocsmit/zed-r).

- Tree-sitter: [r-lib/tree-sitter-r](https://github.com/r-lib/tree-sitter-r)
- Language-Server: [REditorSupport/languageserver](https://github.com/REditorSupport/languageserver)

## Installation

1. [Download and Install R](https://cloud.r-project.org/).
2. Install the R packages `languageserver` and `lintr`:

```R
install.packages("languageserver")
install.packages("lintr")
```

3. Install the [R Zed extension](https://github.com/ocsmit/zed-r) through Zed's extensions manager.

For example on macOS:

```sh
brew install --cask r
Rscript --version
Rscript -e 'options(repos = "https://cran.rstudio.com/"); install.packages("languageserver")'
Rscript -e 'options(repos = "https://cran.rstudio.com/"); install.packages("lintr")'
Rscript -e 'packageVersion("languageserver")'
Rscript -e 'packageVersion("lintr")'
```

## Ark Installation

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

<!--
TBD: R REPL Docs
-->
