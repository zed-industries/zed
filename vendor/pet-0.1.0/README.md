# Server

This tool can be started as a JSON RPC server using stdin/stdout as the communication channel.
More details can be found in the [server](server.md) documentation.

# CLI

Currently the CLI is primarily used for testing purposes.

# Design Principles

- Avoid spawning processes
- Report environment as soon as they are discovered
- Identify environment information in one sweek (avoid incremental reporting)
  I.e. do not report an Environment first with the exe, then report again with the version, then again with prefix, etc.

# Miscellaneous

- /usr/bin, /bin, /usr/local/bin are the most common locations for Python on Unix.
  - This tool will list all Python executables in all of these directories.
  - /usr/bin/python, /bin/python are generally used & installed by tools.
  - /usr/local/bin/python is generally used by end users.

# Search Algorightm

- First we as each of the locators to find all environments by calling the `find` method on each locator.

  - The assumption is that the locators will only return environments that are guaranteed to be of the type that the locator is looking for.
  - E.g. `venv` locator will not return anything in the `find` method, as it cannot guarantee that the environment is a `venv` environment. It could be a `poetry`, `pipenv`, etc environment.

- The caller will collate a list of known environments in global locations.
  - E.g. `~/.virtualenvs` is a known location for virtual envs, but used by various tools.
  - For each of these environments, the caller will honour the priority of the locators and call the `try_from` method in the order of priority.
  - If any locator returns an environment, then the caller will not call the `try_from` method on the other locators.

## Priority

**Why do we need to prioritize the search?**

The priority of the lcoators matter.
For instance any virtual environment should not and cannot be treated as `venv`.
It could have been created by other tools like `poetry`, `pipenv`, etc. Hence we need to first check if a Python environment is a `poetry`, or `pipenv` or the other.

**Order of Priority**

<details>
<summary>Below is the order of priority for the search algorithm:</summary>

1. Search Windows Store
   Done before `Windows Registry` because `Windows Store` has entries in `Windows Registry` as well.
   Hence first identify store to ensure we eliminate the possibility of duplicates identified by `Windows Registry`
2. Search Windows Registry
   These are always stored in a custom (global) location.
   However, Anaconda (or other installs) can be identified as Python installations.
   Hence we need to check if a Python env found here is a conda env and if so, then pass that onto the conda locator.
   Similarly, if we find out in the future that there are other Python environments like `poetry`, `venv` or others, that for some
   reason end up being installed in the Windows Registry, then we can pass that onto the respective locators.
3. Pyenv

- These are always stored in a custom (global) location.
- Conda
  - However there are a number of flavours of Conda (Anaconda, Miniconda, Miniforge, etc) that can be installed via pyenv.
    Hence pass those conda install folders to the conda locator.
- VirtualEnvironments
  - The [pyenv-virtualenv](https://github.com/pyenv/pyenv-virtualenv) plugin creates virtual environments.

4. Homebrew
   These are always stored in a custom (global) lcoation.

5. Pixi
   Pixi should run before Conda as its environments could be falsely identified as Conda environments.

6. Conda
   - These are always stored in a custom (global) location.

- At this stage Conda envs cannot be treated as anything else

7. Poetry

- These are always stored in a custom (global) location.
- Environments are created with a special name based on the hash of the project folder.
- This needs to happen before others as these environments are general virtual environments.

8. Pipenv

- These are always stored in a custom (global) location.
- This needs to happen before others as these environments are general virtual environments.

9. Virtualenvwrapper

- These are always stored in a custom (global) location.
- This needs to happen before others as these environments are general virtual environments.

10. Vevn

- A virtual environment that has a `pyvenv.cfg` file.
- Note, this is a fallback for all other virtual environments.

11. Virtualenv

- A virtual environment that does not have a `pyvenv.cfg` file but has activation scripts such as `/bin/activate` or `Scripts/activate.bat` or the like.
- Note, this is a fallback for all other environments.
- This must happen after conda env discovery, as conda envs have activation scripts as well.

12. Mac XCode

- These are always stored in a custom (global) location.

13. Mac Command Line Tools

- These are always stored in a custom (global) location.

14. Mac Python Org

- These are always stored in a custom (global) location.

15. Linux Python

- These are always stored in a custom (global) location.
</details>
