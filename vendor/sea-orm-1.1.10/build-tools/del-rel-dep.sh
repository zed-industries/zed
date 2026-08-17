#!/bin/bash
set -e

find examples/ -depth -type f -name '*.toml' -exec sed -i '/^path = "..\/..\/..\/sea-orm-migration"/d' {} \;
find examples/ -depth -type f -name '*.toml' -exec sed -i '/^path = "..\/..\/..\/"/d' {} \;