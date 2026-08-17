#
# Maintenance Makefile
#

# Enforce bash with fatal errors.
SHELL			:= /bin/bash -eo pipefail

# Keep intermediates around on failures for better caching.
.SECONDARY:

# Default build and source directories.
BUILDDIR		?= ./build
SRCDIR			?= .

#
# Target: help
#

.PHONY: help
help:
	@# 80-width marker:
	@#     01234567012345670123456701234567012345670123456701234567012345670123456701234567
	@echo "make [TARGETS...]"
	@echo
	@echo "The following targets are provided by this maintenance makefile:"
	@echo
	@echo "    help:               Print this usage information"
	@echo
	@echo "    publish-github:     Publish a release to GitHub"

#
# Target: BUILDDIR
#

$(BUILDDIR)/:
	mkdir -p "$@"

$(BUILDDIR)/%/:
	mkdir -p "$@"

#
# Target: FORCE
#
# Used as alternative to `.PHONY` if the target is not fixed.
#

.PHONY: FORCE
FORCE:

#
# Target: publish-*
#

PUBLISH_REPO		?= r-efi/r-efi
PUBLISH_VERSION		?=

define PUBLISH_RELNOTES_PY
with open('NEWS.md', 'r') as f:
	notes = f.read().split("\n## CHANGES WITH ")[1:]
	notes = dict(map(lambda v: (v[:v.find(":")], v), notes))
	notes = notes["$(PUBLISH_VERSION)"].strip()
print("    # r-efi - UEFI Reference Specification Protocol Constants and Definitions\n")
print("    ## CHANGES WITH", notes)
endef

export PUBLISH_RELNOTES_PY
export PUBLISH_REPO
export PUBLISH_VERSION

.PHONY: publish-github
publish-github:
	test ! -z "$${PUBLISH_REPO}"
	test ! -z "$${PUBLISH_VERSION}"
	python \
		- \
		<<<"$${PUBLISH_RELNOTES_PY}" \
		| gh \
			release \
			--repo "$${PUBLISH_REPO}" \
			create \
			--verify-tag \
			--title \
			"r-efi-$${PUBLISH_VERSION}" \
			--notes-file - \
			"v$${PUBLISH_VERSION}"
