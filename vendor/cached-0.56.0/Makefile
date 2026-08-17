################################################################################
# Author: Altair Bueno <business.altair.bueno@outlook.com>
# Date: 22/06/2022
# Source: https://github.com/jaemk/cached
# Copyright: MIT License (see LICENSE)
# Description: GNU Makefile for `cached`
################################################################################
# Configuration variables

# List with all basic examples. An example is considered basic if it can be
# run using `cargo run --example=$EXAMPLE` and run standalone. All features are
# **enabled**
CACHED_BASIC_EXAMPLES = async_std \
                        basic \
                        basic_proc_macro \
                        kitchen_sink \
                        kitchen_sink_proc_macro \
                        tokio \
                        expiring_sized_cache
# Same as `CACHED_BASIC_EXAMPLES`, but these examples require the `docker/redis`
# goal
CACHED_REDIS_EXAMPLES = redis \
                        redis-async
# Custom commands. NOTE: You'll need to specify the goal manually. See
# `examples/cargo/wasm` for an example
CACHED_CARGO_EXAMPLES = wasm

# Cargo command used to run `run`, `build`, `test`... Useful if you keep
# multiple cargo versions installed on your machine
CARGO_COMMAND         = cargo

# Compiler program and flags used to generate README.md
README_CC             = $(CARGO_COMMAND) readme
README_CCFLAGS        = --no-indent-headings

# Compiler program and flags used to generate format the crate
FMT_CC                = $(CARGO_COMMAND) fmt
FMT_CCFLAGS           =

# Docker configuration. Set DOCKER_COMMAND on your shell to override the
# container engine used
#
# ```sh
# # Using containerd to run `docker/redis`
# make DOCKER_COMMAND=containerd docker/redis
# ```
DOCKER_COMMAND                        = docker
DOCKER_REDIS_CONTAINER_NAME           = cached-tests
DOCKER_REDIS_CONTAINER_LOCAL_PORT     = 6399

################################################################################
# Exported variables
export CACHED_REDIS_CONNECTION_STRING = redis://127.0.0.1:$(DOCKER_REDIS_CONTAINER_LOCAL_PORT)
export RUST_BACKTRACE                 = 1

################################################################################
# GitHub Actions goal. Run this to test your changes before summiting your final
# pull request
ci: check tests examples

################################################################################
# Runs all examples
examples: examples/basic examples/cargo examples/redis
# Runs all basic examples
examples/basic: $(addprefix examples/basic/, $(CACHED_BASIC_EXAMPLES))
# Runs all the project based examples
examples/cargo: $(addprefix examples/cargo/, $(CACHED_CARGO_EXAMPLES))
# Runs `redis` related examples. NOTE: depends on `docker/redis`
examples/redis: $(addprefix examples/redis/, $(CACHED_REDIS_EXAMPLES))

examples/basic/%:
	@echo [$@]: Running example $*...
	$(CARGO_COMMAND) run --example $* --all-features

# Only builds the `wasm` example. Running this example requires a browser
examples/cargo/wasm:
	@echo [$@]: Building example $*...
	cd examples/wasm ; $(CARGO_COMMAND) build --target=wasm32-unknown-unknown

examples/redis/%: docker/redis
	@echo [$@]: Running example $*...
	$(CARGO_COMMAND) run --example $* --all-features

################################################################################
# Runs `cached` tests. NOTE: Depends on `docker/redis`
tests: docker/redis
	@echo [$@]: Running tests...
	$(CARGO_COMMAND) test --all-features -- --nocapture

################################################################################
# Starts a Redis server using `DOCKER_COMMAND`
docker/redis: docker/status
	@echo [$@]: Starting Redis container...
	-$(DOCKER_COMMAND) run --rm --name $(DOCKER_REDIS_CONTAINER_NAME) \
 		-p $(DOCKER_REDIS_CONTAINER_LOCAL_PORT):6379 -d redis

docker/status:
	@echo [$@]: Checking the Docker engine
	@docker info > /dev/null || (>&2 echo 'Is the Docker engine running?' && exit 42)

################################################################################
# Syncs all docs
docs: docs/readme

# Updates README.md using `README_CC`
docs/readme: README.md

README.md: src/lib.rs
	@echo [$@]: Updating $@...
	$(README_CC) $(README_CCFLAGS) > $@

################################################################################
# Formats `cached` crate
fmt:
	@echo [$@]: Formatting code...
	$(FMT_CC) $(FMT_CCFLAGS)

################################################################################
# Runs all checks
check: check/fmt check/readme check/clippy

# Checks if `cached` crate is well formatted
check/fmt: FMT_CCFLAGS += --check
check/fmt:
	@echo [$@]: Checking code format...
	$(FMT_CC) $(FMT_CCFLAGS)

# Checks if the README.md file is up-to-date
check/readme:
	@echo [$@]: Checking README.md...
	$(README_CC) $(README_CCFLAGS) | cmp README.md

# Runs clippy linter on `cached` crate
check/clippy:
	@echo [$@]: Running clippy...
	$(CARGO_COMMAND) clippy --all-features --all-targets --examples --tests

################################################################################
# Cleans all generated artifacts and deletes all docker containers
clean: clean/docker clean/cargo

# Runs `cargo clean`
clean/cargo:
	@echo [$@]: Removing cargo artifacts...
	$(CARGO_COMMAND) clean

# Removes all docker containers
clean/docker: clean/docker/$(DOCKER_REDIS_CONTAINER_NAME)

# Removes a docker container with the given name
clean/docker/%:
	@echo [$@]: Removing container called $*...
	$(DOCKER_COMMAND) rm -f $*

################################################################################
# Special targets.

.PHONY: ci \
		$(filter examples%, $(MAKECMDGOALS)) \
		$(filter tests%, $(MAKECMDGOALS)) \
		$(filter docker%, $(MAKECMDGOALS)) \
		$(filter docs%, $(MAKECMDGOALS)) \
		$(filter fmt%, $(MAKECMDGOALS)) \
		$(filter check%, $(MAKECMDGOALS)) \
		$(filter clean%, $(MAKECMDGOALS))
