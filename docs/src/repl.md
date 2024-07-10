# REPL

Read. Eval. Print. Loop.

The built-in REPL for Zed allows you to run code interactively in your editor similarly to a notebook with your own text files.

(TODO: Include GIF in action)

To start using it the REPL, add the following to your Zed `settings.json` to bring the power of Jupyter kernels to your editor:

```json
{
  "jupyter": {
    "enabled": true
  }
}
```

After that, install any of the supported kernels:

* [Python](#python)
* [TypeScript via Deno](#deno)

## Python

### Global environment

To setup your current python to have an available kernel, run:

```
python -m ipykernel install --user
```

### Conda Environment

```
source activate myenv
conda install ipykernel
python -m ipykernel install --user --name myenv --display-name "Python (myenv)"
```


### Virtualenv with pip

```
source activate myenv
pip install ipykernel
python -m ipykernel install --user --name myenv --display-name "Python (myenv)"
```

## Deno

[Install Deno](https://docs.deno.com/runtime/manual/getting_started/installation/) and then install the Deno jupyter kernel:

```
deno jupyter --unstable --install
```

## Other languages

* [Julia](https://github.com/JuliaLang/IJulia.jl)
* R
  - [Ark Kernel from Positron, formerly RStudio](https://github.com/posit-dev/ark)
  - [Xeus-R](https://github.com/jupyter-xeus/xeus-r)
* [Scala](https://almond.sh/docs/quick-start-install)
