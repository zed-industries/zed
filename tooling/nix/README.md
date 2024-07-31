# Nix Package

## How it works

1. There is a patch that completely removes the download code from the node runtime.
2. The path to NodeJS is hardcoded via the `NODE_PATH` environment variable (at build time), which is set inside
   `./package.nix`.
