#!/bin/bash
set -euxo pipefail

PRETTIER_VERSION=3.5.0

if [[ "${1:-}" == "--write" ]]; then
    MODE="--write"
else
    MODE="--check"
fi

pnpm dlx "prettier@${PRETTIER_VERSION}" assets/settings/default.json --parser=jsonc $MODE || {
    echo "To fix, run from the root of the Zed repo:"
    echo "  pnpm dlx prettier@${PRETTIER_VERSION} assets/settings/default.json --parser=jsonc --write"
    false
}

cd docs
pnpm dlx "prettier@${PRETTIER_VERSION}" . $MODE || {
    echo "To fix, run from the root of the Zed repo:"
    echo "  cd docs && pnpm dlx prettier@${PRETTIER_VERSION} . --write && cd .."
    false
}
