set -euxo pipefail

# Based on the Rust-Embedded WG's book CI
# https://github.com/rust-embedded/book/blob/HEAD/ci/install.sh

main() {
    # Note - this will only accept releases tagged with v0.3.x
    local tag=$(git ls-remote --tags --refs --exit-code \
                    https://github.com/rust-lang-nursery/mdbook \
                        | cut -d/ -f3 \
                        | grep -E '^v0\.3\.[0-9]+$' \
                        | sort --version-sort \
                        | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- --git rust-lang-nursery/mdbook --tag $tag
}

main
