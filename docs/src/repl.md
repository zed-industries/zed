# REPL

## Getting started

Bring the power of [Jupyter kernels](https://docs.jupyter.org/en/latest/projects/kernels.html) to your editor! The built-in REPL for Zed allows you to run code interactively in your editor similarly to a notebook with your own text files.

<figure style="overflow: hidden; border-top-left-radius: 2px; border-top-right-radius: 2px;">
    <video loop controls playsinline>
        <source
            src="https://customer-snccc0j9v3kfzkif.cloudflarestream.com/aec66e79f23d6d1a0bee5e388a3f17cc/downloads/default.mp4"
            type='video/webm; codecs="vp8.0, vorbis"'
        />
        <source
            src="https://customer-snccc0j9v3kfzkif.cloudflarestream.com/aec66e79f23d6d1a0bee5e388a3f17cc/downloads/default.mp4"
            type='video/mp4; codecs="avc1.4D401E, mp4a.40.2"'
        />
        <source
          src="https://zed.dev/img/post/repl/typescript-deno-kernel-markdown.png"
          type="image/png"
        />
    </video>
</figure>

## Installation

Zed supports running code in multiple languages. To get started, you need to install a kernel for the language you want to use.

**Currently supported languages:**

- [Python (ipykernel)](#python)
- [R (Ark)](#r)
- [TypeScript (Deno)](#typescript-deno)

Once installed, you can start using the REPL in the respective language files, or other places those languages are supported, such as Markdown. If you recently added the kernels, run the `repl: refresh kernelspecs` command to make them available in the editor.

## Using the REPL

To start the REPL, open a file with the language you want to use and use the `repl: run` command (defaults to `ctrl-shift-enter` on macOS) to run a block, selection, or line. You can also click on the REPL icon in the toolbar.

The `repl: run` command will be executed on your selection(s), and the result will be displayed below the selection.

Outputs can be cleared with the `repl: clear outputs` command, or from the REPL menu in the toolbar.

### Cell mode

Zed supports [notebooks as scripts](https://jupytext.readthedocs.io/en/latest/formats-scripts.html) using the `# %%` cell separator in Python and `// %%` in TypeScript. This allows you to write code in a single file and run it as if it were a notebook, cell by cell.

The `repl: run` command will run each block of code between the `# %%` markers as a separate cell.

```python
# %% Cell 1
import time
import numpy as np

# %% Cell 2
import matplotlib.pyplot as plt
import matplotlib.pyplot as plt
from matplotlib import style
style.use('ggplot')
```

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

### R

Install [Ark](https://github.com/posit-dev/ark/releases) by downloading the release for your operating system. E.g. for macOS just unpack `ark` binary and put it into `/usr/local/bin`. Then run:

```
ark --install
```

### Typescript: Deno {#typescript-deno}

[Install Deno](https://docs.deno.com/runtime/manual/getting_started/installation/) and then install the Deno jupyter kernel:

```
deno jupyter --install
```

### Other languages

The following languages and kernels are also supported. You can help us out by expanding their installation instructions and configuration:

- [Julia (IJulia)](https://github.com/JuliaLang/IJulia.jl)
- R
  - [Ark Kernel](https://github.com/posit-dev/ark) - via Positron, formerly RStudio
  - [Xeus-R](https://github.com/jupyter-xeus/xeus-r)
- [Scala (almond)](https://almond.sh/docs/quick-start-install)

## Changing which kernel is used per language {#changing-kernels}

Zed automatically detects the available kernels on your system. If you need to configure a different default kernel for a
language, you can assign a kernel for any supported language in your `settings.json`.

```json
{
  "jupyter": {
    "kernel_selections": {
      "python": "conda-env",
      "typescript": "deno",
      "javascript": "deno",
      "r": "ark"
    }
  }
}
```

## Debugging Kernelspecs

Available kernels are shown via the `repl: sessions` command. To refresh the kernels you can run, use the `repl: refresh kernelspecs` command.

If you have `jupyter` installed, you can run `jupyter kernelspec list` to see the available kernels.

```
$ jupyter kernelspec list
Available kernels:
  ark                   /Users/z/Library/Jupyter/kernels/ark
  conda-base            /Users/z/Library/Jupyter/kernels/conda-base
  deno                  /Users/z/Library/Jupyter/kernels/deno
  python-chatlab-dev    /Users/z/Library/Jupyter/kernels/python-chatlab-dev
  python3               /Users/z/Library/Jupyter/kernels/python3
  ruby                  /Users/z/Library/Jupyter/kernels/ruby
  rust                  /Users/z/Library/Jupyter/kernels/rust
```

Note: Zed makes best effort usage of `sys.prefix` and `CONDA_PREFIX` to find kernels in Python environments. If you want explicitly control run `python -m ipykernel install --user --name myenv --display-name "Python (myenv)"` to install the kernel directly while in the environment.
