SHELL := bash

export ROOT := $(shell pwd)

export PATH := $(ROOT)/bin:$(PATH)

CONFIG_MK := $(ROOT)/../yaml-test-runtimes/Config.mk

ifneq (,$(wildcard $(CONFIG_MK)))
    include $(CONFIG_MK)
    YAML_TEST_RUNTIMES_VERSION := $(TAG_MAIN)
endif
ifneq (,$(YAML_TEST_RUNTIMES_VERSION))
    export YAML_TEST_RUNTIMES_VERSION
endif

BPAN := .bpan
COMMON := ../yaml-common

SRC ?= src/*.yaml

ifneq (,$(DOCKER))
  export RUN_OR_DOCKER := $(DOCKER)
endif

default:

docker:
	$(eval override export RUN_OR_DOCKER := force)
	@true

docker-build:
	RUN_OR_DOCKER=force-build suite-to-data

verbose:
	$(eval override export RUN_OR_DOCKER_VERBOSE := true)
	@true

test:
	! $$(git rev-parse --is-shallow-repository) || \
	    git fetch --unshallow
	make data
	make clean
	make data-update
	GIT_PAGER=cat make data-diff
	make data-status
	make clean
	make gh-pages
	make clean

add-new:
	for f in new/*; do cp "$$f" "src/$${f#*-}"; done

import: import.tsv
	./bin/tsv-to-new $<

import.tsv:
	$(error 'make import' requires a '$@' file)

export: export.tsv

run-tests:
ifndef YAML_TEST_RUNTIMES_VERSION
	$(error Set YAML_TEST_RUNTIMES_VERSION)
endif
	$(eval override export YTS_TEST_RUNNER := true)
	@true

export.tsv:
	time ./bin/suite-to-tsv $(SRC) > $@

new-test:
	new-test-file

testml:
	suite-to-testml $(SRC)

data:
	git branch --track $@ origin/$@ 2>/dev/null || true
	git worktree add -f $@ $@

data-update: data
	rm -fr $</*
	suite-to-data src/*.yaml

data-status: data
	@git -C $< add -Af . && \
	 git -C $< status --short

data-diff: data
	git -C $< add -Af . && \
	 git -C $< diff --cached

data-push: data data-update
	[[ $$(git -C $< status --short) ]] && \
	( \
	    git -C $< add -Af . && \
	    COMMIT=$$(git rev-parse --short HEAD) && \
	    git -C $< commit -m "Regenerated data from main $$COMMIT" && \
	    git -C $< push origin data \
	)

common:
	cp $(COMMON)/bpan/run-or-docker.bash $(BPAN)/

clean:
	rm -f export.tsv
	rm -fr data gh-pages new testml
	git worktree prune

docker-push: docker-build
	RUN_OR_DOCKER_PUSH=true suite-to-data

clean-docker:
	-docker images | \
	    grep -E '(suite-to-data|new-test-file)' | \
	    awk '{print $3}' | \
	    xargs docker rmi 2>/dev/null

#------------------------------------------------------------------------------

gh-pages:
	git branch --track $@ origin/$@ 2>/dev/null || true
	git worktree add $@ $@
