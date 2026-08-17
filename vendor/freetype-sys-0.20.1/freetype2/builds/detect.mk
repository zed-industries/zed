#
# FreeType 2 host platform detection rules
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# This sub-Makefile is in charge of detecting the current platform.  It sets
# the following variables:
#
#   PLATFORM_DIR The configuration and system-specific directory.  Usually
#                `builds/$(PLATFORM)' but can be different for custom builds
#                of the library.
#
# The following variables must be defined in system specific `detect.mk'
# files:
#
#   PLATFORM     The detected platform.  This will default to `ansi' if
#                auto-detection fails.
#   CONFIG_FILE  The configuration sub-makefile to use.  This usually depends
#                on the compiler defined in the `CC' environment variable.
#   DELETE       The shell command used to remove a given file.
#   COPY         The shell command used to copy one file.
#   SEP          The platform-specific directory separator.
#   COMPILER_SEP The separator used in arguments of the compilation tools.
#   CC           The compiler to use.
#
# You need to set the following variable(s) before calling it:
#
#   TOP_DIR      The top-most directory in the FreeType library source
#                hierarchy.  If not defined, it will default to `.'.

# Set auto-detection default to `ansi' resp. UNIX-like operating systems.
#
PLATFORM     := ansi
DELETE       := $(RM)
COPY         := cp
CAT          := cat
SEP          := /

BUILD_CONFIG := $(TOP_DIR)/builds

# These two assignments must be delayed.
PLATFORM_DIR = $(BUILD_CONFIG)/$(PLATFORM)
CONFIG_RULES = $(PLATFORM_DIR)/$(CONFIG_FILE)

# We define the BACKSLASH variable to hold a single back-slash character.
# This is needed because a line like
#
#   SEP := \
#
# does not work with GNU Make (the backslash is interpreted as a line
# continuation).  While a line like
#
#   SEP := \\
#
# really defines $(SEP) as `\' on Unix, and `\\' on Dos and Windows!
#
BACKSLASH := $(strip \ )

# Find all auto-detectable platforms.
#
PLATFORMS := $(notdir $(subst /detect.mk,,$(wildcard $(BUILD_CONFIG)/*/detect.mk)))
.PHONY: $(PLATFORMS) ansi

# Filter out platform specified as setup target.
#
PLATFORM := $(firstword $(filter $(MAKECMDGOALS),$(PLATFORMS)))

# If no setup target platform was specified, enable auto-detection/
# default platform.
#
ifeq ($(PLATFORM),)
  PLATFORM := ansi
endif

# If the user has explicitly asked for `ansi' on the command line,
# disable auto-detection.
#
ifeq ($(findstring ansi,$(MAKECMDGOALS)),)
  # Now, include all detection rule files found in the `builds/<system>'
  # directories.  Note that the calling order of the various `detect.mk'
  # files isn't predictable.
  #
  include $(wildcard $(BUILD_CONFIG)/*/detect.mk)
endif

# In case no detection rule file was successful, use the default.
#
ifndef CONFIG_FILE
  CONFIG_FILE := ansi.mk
  setup: std_setup
  .PHONY: setup
endif

# Flash out and copy rules.
#
.PHONY: std_setup

std_setup:
	$(info )
	$(info $(PROJECT_TITLE) build system -- automatic system detection)
	$(info )
	$(info The following settings are used:)
	$(info )
	$(info $(empty)  platform                    $(PLATFORM))
	$(info $(empty)  compiler                    $(CC))
	$(info $(empty)  configuration directory     $(subst /,$(SEP),$(PLATFORM_DIR)))
	$(info $(empty)  configuration rules         $(subst /,$(SEP),$(CONFIG_RULES)))
	$(info )
	$(info If this does not correspond to your system or settings please remove the file)
	$(info `$(CONFIG_MK)' from this directory then read the INSTALL file for help.)
	$(info )
	$(info Otherwise, simply type `$(MAKE)' again to build the library,)
	$(info or `$(MAKE) refdoc' to build the API reference (this needs Python >= 3.5).)
	$(info )
	@$(COPY) $(subst /,$(SEP),$(CONFIG_RULES) $(CONFIG_MK))


# EOF
