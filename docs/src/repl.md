# REPL

Read. Eval. Print. Loop.

<div class="warning">

This feature is in active development. Details may change. We're delighted to get feedback as the REPL feature evolves.

</div>

## Getting started

Bring the power of [Jupyter kernels](https://docs.jupyter.org/en/latest/projects/kernels.html) to your editor! The built-in REPL for Zed allows you to run code interactively in your editor similarly to a notebook with your own text files.

<!-- TODO: Include GIF in action -->

To start using the REPL, add the following to your Zed `settings.json`:

```json
{
  "jupyter": {
    "enabled": true
  }
}
```

## Installation

Zed supports running code in multiple languages. To get started, you need to install a kernel for the language you want to use.

**Currently supported languages:**

* [Python (ipykernel)](#python)
* [TypeScript (Deno)](#typescript-deno)


Once installed, you can start using the REPL in the respective language files, or other places those languages are supported, such as Markdown.

<!-- TODO: Make markdown a link with an example -->

## Using the REPL

To start the REPL, open a file with the language you want to use and use the `repl: run` command (defaults to CMD + Enter on macOS). You can also click on the REPL icon in the toolbar.

The `repl: run` command will be executed on your selection(s), and the result will be displayed below the selection.

Outputs can be cleared with the `repl: clear outputs` command, or from the REPL menu in the toolbar.

## Changing Kernels {#changing-kernels}

Work in Progress!

## Language specific instructions

### Python {#python}

#### Global environment

<div class="warning">

On MacOS, your system Python will _not_ work. Either set up [pyenv](https://github.com/pyenv/pyenv?tab=readme-ov-file#installation) or use a virtual environment.

</div>


To setup your current python to have an available kernel, run:

```
pip install ipykernel
python -m ipykernel install --user
```

#### Conda Environment

```
source activate myenv
conda install ipykernel
python -m ipykernel install --user --name myenv --display-name "Python (myenv)"
```


#### Virtualenv with pip

```
source activate myenv
pip install ipykernel
python -m ipykernel install --user --name myenv --display-name "Python (myenv)"
```

### Typescript: Deno {#typescript-deno}

[Install Deno](https://docs.deno.com/runtime/manual/getting_started/installation/) and then install the Deno jupyter kernel:

```
deno jupyter --unstable --install
```

### Other languages

The following languages and kernels are also supported. You can help us out by expanding their installation instructions and configuration:

* [Julia (IJulia)](https://github.com/JuliaLang/IJulia.jl)
* R
  - [Ark Kernel](https://github.com/posit-dev/ark) - via Positron, formerly RStudio
  - [Xeus-R](https://github.com/jupyter-xeus/xeus-r)
* [Scala (almond)](https://almond.sh/docs/quick-start-install)
