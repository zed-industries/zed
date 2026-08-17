#!/bin/bash
set -e

# Bump `sea-orm-codegen` version
cd sea-orm-codegen
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
cd ..

# Bump `sea-orm-cli` version
cd sea-orm-cli
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
sed -i 's/^sea-orm-codegen [^,]*,/sea-orm-codegen = { version = "\='$1'",/' Cargo.toml
cd ..

# Bump `sea-orm-macros` version
cd sea-orm-macros
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
cd ..

# Bump `sea-orm` version
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
sed -i 's/^sea-orm-macros [^,]*,/sea-orm-macros = { version = "'~$1'",/' Cargo.toml

# Bump `sea-orm-migration` version
cd sea-orm-migration
sed -i 's/^version.*$/version = "'$1'"/' Cargo.toml
sed -i 's/^sea-orm-cli [^,]*,/sea-orm-cli = { version = "'~$1'",/' Cargo.toml
sed -i 's/^sea-orm [^,]*,/sea-orm = { version = "'~$1'",/' Cargo.toml
cd ..

git commit -am "$1"

# Bump examples' dependency version
cd examples
find . -depth -type f -name '*.toml' -exec sed -i 's/^version = ".*" # sea-orm version$/version = "'~$1'" # sea-orm version/' {} \;
find . -depth -type f -name '*.toml' -exec sed -i 's/^version = ".*" # sea-orm-migration version$/version = "'~$1'" # sea-orm-migration version/' {} \;
git add .
git commit -m "update examples"