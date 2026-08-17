# Hombrew

## Notes

- Install folders & symlinks documented here https://formulae.brew.sh/formula/python@3.9
- Homebrew install locations documented here https://docs.brew.sh/Installation

## Known Issues

- `sys.prefix` is not easy to identify. Hence left empty for now.
  If we can find a way to identify `sys.prefix` consistenly for all platforms, then we can populate this field.

## Pseduo code for algorithm

```rust
homebrew_dir = // find this folder (either "HOMEBREW_PREFIX" or default to directory defined here https://docs.brew.sh/Installation)
for file under "<homebrew_dir>/bin":
    if we have a python executable and its a symlink, then proceed
    if not, then skip this file

    resolve the symlink and verify the file is in one of the known homebrew directories.
    if not, then skip this file

    Extract the version from the file path.

    Compute the env_path by extracting the version information.
    The env_path is known directories on MacOS (Intel & Silicon) and Linux.

    Identify all known Symlinks for the python executable.
    There are a number of symlinks for each Python environment executable. Best to identify them all.
    As we have no idea which ones will be used by users.

    Note: Identifying `sys_prefix` is not easy, hence left empty for now.
```

</details>
</details>
