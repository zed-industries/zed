# Pixi

## Notes

- Pixi environments are detected by:
  - Searching for Python interpreters in `.pixi/envs` subdirectories within workspace folders
  - Checking for a `conda-meta/pixi` file in potential Pixi environment directories (`.pixi/envs/{env_name}`)
  - Determining the version of the Python interpreter from the `conda-meta/python-{version}.json` file

This process ensures fast detection without spawning processes.
Note that the Pixi locator should run before Conda since Conda could incorrectly identify Pixi environments as Conda environments.
