
SED := `command -v gsed || command -v sed`

help:
    @just --list

# git tasks to start next development version
prepare-dev-version v:
    git checkout trunk
    {{SED}} -zE -i 's/(name = "bigdecimal"\nversion )= [^\n]*/\1= "{{v}}+dev"/' Cargo.toml Cargo.lock
    git add Cargo.toml Cargo.lock
    git commit -m 'Begin v{{v}} development'


# git tasks to run to merge trunk into master
prepare-release v:
    git checkout trunk
    cargo clippy
    {{SED}} -zE -i 's/(name = "bigdecimal"\nversion )= [^\n]*/\1= "{{v}}"/' Cargo.toml Cargo.lock
    git add Cargo.toml Cargo.lock
    git commit -m 'Version {{v}}'
    git checkout master
    git merge trunk --no-ff -m 'v{{v}}'
    # git tag 'v{{v}}'

# enable and run benchmarks
benchmark *args:
  scripts/benchmark-bigdecimal {{args}}

# enable and run property-tests
run-property-tests:
  scripts/bigdecimal-property-tests test


# enable property test dependencies
enable-property-tests:
  scripts/bigdecimal-property-tests enable


# print decimals with various formatting rules
run-formatting-example:
  cargo run --example formatting-examples

