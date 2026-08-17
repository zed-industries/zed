# Conda

## Notes

- Structure of conda envionment
  - A conda install folder will `generally` have a default environment with Python (except for `micromamba` and possibly others)
  - the `envs` sub directory in a conda installation contains all conda environments belonging to that conda installation
- Conda environments
  - This is computed from a list of hardcoded locations (for windows, linux, macos)
  - This includes locations in `.condarc` file(s)
  - This includes locations in `environments.txt` file
  - This includes all sub directories under the `env` directory in the all known conda installation folders
- `.condarc`
  - The `.condarc` file can be configured to store conda environments in other locations instead of `<conda install folder>/envs`
  - There can be multiple conda installations on a machine, each with its own set of environments
  - There can be multiple directories defined in `.condarc` file to store conda environments
  - There can be multiple `.condarc` files
- `environments.txt`
  - This contains all known conda environments on the current machine (including base env, which is the same as install location of conda)
  - The directories returned by `conda info --json` are the directories where conda environments are stored
- Version of conda can be found in `<conda install folder>/conda-meta/conda-<version>.json` file
- User can install multiple versions (installations) of conda,
  - Eg. install anaconda, miniconda, miniforge, micromamba, etc.
  - In usual (default) locations & also in custom locations
  - Also can be installed via pyenv, in which case the conda installations are located under `~/.pyenv/versions/<conda install version>`
- `conda-meta\<package name>-<version>.json` files
  - This contains information about all conda packages installed into a conda environment.
  - The root (base) conda environment too has such a directory.
  - Given `conda` is a package in its own right, the version of conda can also be found in the `conda-meta` directory.
    (after all, you can update conda using `conda update conda`).
  - Similarly if Python is installed in a conda environment, the version of Python can be found in the `conda-meta` directory.
- `conda-meta\history` file
  - This contains a history of all installations of this conda environment.
  - One key information is the command used to create the conda environment itself.
  - The entry looks like so `# cmd: /Users/donjayamanne/.pyenv/versions/miniconda3-latest/bin/conda create -n myenv python=3.10`
  - Thus using the `history` file we can find the conda installation folder.
    This is useful in cases where conda environments are created using `-p` option.

## Miscellanous

- What if conda is installed in some custom locations that we have no idea about?

In such cases the assumption is that the `environments.txt` file will contain an entry to the base env.
Using that information we can get the conda directory and get the conda exe and version info.

Even if `environments.txt` file is empty, we will look for environments in known locations and from there we can find the conda install folder (recall `history` file).

- What if we have a custom conda env created in current workspace folder, and we do not know where Conda is installed?

In such cases we can just inspect the `conda-meta/history` file in the conda env folder and get the conda installation folder.

- How do we generate command to run Python in an environment?

If the Conda env is in the `envs` folder, then use `<conda exe> -n <env name> python`.
If the Conda env is the root (base) folder, then use `<conda exe> -n base python`.
For all other cases use `<conda exe> -p <fully qualified path to folder> python`.

## Pseduo code for algorithm

```rust
// Step 1
// Get a list of all known directories where conda environments can be found
// 1. environments.txt file
// 2. .condarc file(s)
// 3. Other known locations

// Step 2
// We hardcode some of the commonly known install directories of conda, miniconda, miniforge, etc for all platforms.
for known_install_folder in [<home>/anaconda3, <home>/miniconda3, etc]:
    conda_exe = "<known_install_folder>/bin/conda" // (windows is slightly different)
    conda_version = "<known_install_folder>/conda_meta/conda-<version>.json"
    python_exe = "<known_install_folder>/bin/python" // (windows is slightly different)
    python_version = "<known_install_folder>/conda_meta/conda-<version>.json"

    // Step 2.1
    // We now have conda exe, version, default python information
    // Conda run command is computed as `[<fully qualified path to conda_exe>, run, -n, <name> python]`
    for env in `<known_install_folder>/envs`:
        // env is a conda environment
        // Find python exe and version in the conda-meta directory
        // If python is not found, then this is a conda env without Python.
        // These are named environments that are activated (run) using the `-n` option.

    // Previously we captured a list of all known conda envs
    // Go through those one by one and inspect the conda-meta/history file
    // And check whether that env was created by this current conda installation
    // If so, then associate that env with this conda installation
    // Next remove that env folder from the list captured in step 1.

// Step 3
// Finally go through all of the remaining conda envs that were captured in step 1
// & did not get removed by step 2.1.
// Go into the env folder one by one
// Inspect the conda-meta/history file and try to find the installation location of the conda by parsing the `cmd:` line.
// If we find a conda installation, then process that folder as we did inside step 2.1
```
