test *FILTER:
    cargo build
    cargo test -q {{FILTER}}

watch *FILTER:
    watchexec -e rs -rc reset -- just test {{FILTER}}

build:
    cargo build --all

watch-build:
    watchexec -- "reset && just build"

doc:
    cargo doc --all-features

open-doc:
    xdg-open target/doc/convert_case/index.html

watch-doc:
    watchexec -- "just doc && cargo test --all-features --doc"

tree:
    tree -I target

verify-nostd:
    cargo build --target thumbv6m-none-eabi
