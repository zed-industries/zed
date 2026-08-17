#
# FreeType build system -- top-level sub-Makefile
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# This file is designed for GNU Make, do not use it with another Make tool!
#
# It works as follows:
#
# - When invoked for the first time, this Makefile includes the rules found
#   in `PROJECT/builds/detect.mk'.  They are in charge of detecting the
#   current platform.
#
#   A summary of the detection is displayed, and the file `config.mk' is
#   created in the current directory.
#
# - When invoked later, this Makefile includes the rules found in
#   `config.mk'.  This sub-Makefile defines some system-specific variables
#   (like compiler, compilation flags, object suffix, etc.), then includes
#   the rules found in `PROJECT/builds/PROJECT.mk', used to build the
#   library.
#
# See the comments in `builds/detect.mk' and `builds/PROJECT.mk' for more
# details on host platform detection and library builds.


# First of all, check whether we have `$(value ...)'.  We do this by testing
# for `$(eval ...)' which has been introduced in the same GNU make version.

eval_available :=
$(eval eval_available := T)
ifneq ($(eval_available),T)
  $(error FreeType's build system needs a Make program which supports $$(value))
endif


.PHONY: all dist distclean modules setup


# The `space' variable is used to avoid trailing spaces in defining the
# `T' variable later.
#
empty :=
space := $(empty) $(empty)


# The main configuration file, defining the `XXX_MODULES' variables.  We
# prefer a `modules.cfg' file in OBJ_DIR over TOP_DIR.
#
ifndef MODULES_CFG
  MODULES_CFG := $(TOP_DIR)/modules.cfg
  ifneq ($(wildcard $(OBJ_DIR)/modules.cfg),)
    MODULES_CFG := $(OBJ_DIR)/modules.cfg
  endif
endif


# FTMODULE_H, as its name suggests, indicates where the FreeType module
# class file resides.
#
FTMODULE_H ?= $(OBJ_DIR)/ftmodule.h


include $(MODULES_CFG)


# The list of modules we are using.
#
MODULES := $(FONT_MODULES)    \
           $(HINTING_MODULES) \
           $(RASTER_MODULES)  \
           $(AUX_MODULES)


CONFIG_MK ?= config.mk

# If no configuration sub-makefile is present, or if `setup' is the target
# to be built, run the auto-detection rules to figure out which
# configuration rules file to use.
#
# Note that the configuration file is put in the current directory, which is
# not necessarily $(TOP_DIR).

# If `config.mk' is not present, set `check_platform'.
#
ifeq ($(wildcard $(CONFIG_MK)),)
  check_platform := 1
endif

# If `setup' is one of the targets requested, set `check_platform'.
#
ifneq ($(findstring setup,$(MAKECMDGOALS)),)
  check_platform := 1
endif


# Include the automatic host platform detection rules when we need to
# check the platform.
#
ifdef check_platform

  all modules: setup

  include $(TOP_DIR)/builds/detect.mk

  # For builds directly from the git repository we need to copy files
  # from `subprojects/dlg' to `src/dlg' and `include/dlg'.
  #
  ifeq ($(wildcard $(TOP_DIR)/src/dlg/dlg.*),)
    ifeq ($(wildcard $(TOP_DIR)/subprojects/dlg/*),)
      copy_submodule: check_out_submodule
    endif

    setup: copy_submodule
  endif

  # This rule makes sense for Unix only to remove files created by a run of
  # the configure script which hasn't been successful (so that no
  # `config.mk' has been created).  It uses the built-in $(RM) command of
  # GNU make.  Similarly, `nul' is created if e.g. `make setup windows' has
  # been erroneously used.
  #
  # Note: This test is duplicated in `builds/unix/detect.mk'.
  #
  is_unix := $(strip $(wildcard /sbin/init) \
                     $(wildcard /usr/sbin/init) \
                     $(wildcard /dev/null) \
                     $(wildcard /hurd/auth))
  ifneq ($(is_unix),)

    distclean:
	  $(RM) $(TOP_DIR)/builds/unix/config.cache
	  $(RM) $(TOP_DIR)/builds/unix/config.log
	  $(RM) $(TOP_DIR)/builds/unix/config.status
	  $(RM) $(TOP_DIR)/builds/unix/unix-def.mk
	  $(RM) $(TOP_DIR)/builds/unix/unix-cc.mk
	  $(RM) $(TOP_DIR)/builds/unix/freetype2.pc
	  $(RM) nul

  endif # test is_unix

  # IMPORTANT:
  #
  # `setup' must be defined by the host platform detection rules to create
  # the `config.mk' file in the current directory.

else

  # A configuration sub-Makefile is present -- simply run it.
  #
  all: single

  BUILD_PROJECT := yes
  include $(CONFIG_MK)

endif # test check_platform


.PHONY: check_out_submodule copy_submodule

check_out_submodule:
	$(info Checking out submodule in `subprojects/dlg')
	git --git-dir=$(TOP_DIR) submodule init
	git --git-dir=$(TOP_DIR) submodule update

copy_submodule:
	$(info Copying files from `subprojects/dlg' to `src/dlg' and `include/dlg')
  ifeq ($(wildcard $(TOP_DIR)/include/dlg),)
	mkdir $(subst /,$(SEP),$(TOP_DIR)/include/dlg)
  endif
	$(COPY) $(subst /,$(SEP),$(TOP_DIR)/subprojects/dlg/include/dlg/output.h $(TOP_DIR)/include/dlg)
	$(COPY) $(subst /,$(SEP),$(TOP_DIR)/subprojects/dlg/include/dlg/dlg.h $(TOP_DIR)/include/dlg)
	$(COPY) $(subst /,$(SEP),$(TOP_DIR)/subprojects/dlg/src/dlg/dlg.c $(TOP_DIR)/src/dlg)


# We always need the list of modules in ftmodule.h.
#
all setup: $(FTMODULE_H)


# The `modules' target unconditionally rebuilds the module list.
#
modules:
	$(FTMODULE_H_INIT)
	$(FTMODULE_H_CREATE)
	$(FTMODULE_H_DONE)

include $(TOP_DIR)/builds/modules.mk


# get FreeType version string using built-in string functions
#
hash := \#

work := $(strip $(shell $(CAT) \
                  $(subst /,$(SEP),$(TOP_DIR)/include/freetype/freetype.h)))

work := $(subst $(hash)define$(space)FREETYPE_MAJOR$(space),MAjOR=,$(work))
work := $(subst $(hash)define$(space)FREETYPE_MINOR$(space),MInOR=,$(work))
work := $(subst $(hash)define$(space)FREETYPE_PATCH$(space),PAtCH=,$(work))

major := $(subst MAjOR=,,$(filter MAjOR=%,$(work)))
minor := $(subst MInOR=,,$(filter MInOR=%,$(work)))
patch := $(subst PAtCH=,,$(filter PAtCH=%,$(work)))

work :=

# ifneq ($(findstring x0x,x$(patch)x),)
#   version := $(major).$(minor)
#   winversion := $(major)$(minor)
# else
  version := $(major).$(minor).$(patch)
  winversion := $(major)$(minor)$(patch)
  version_tag := VER-$(major)-$(minor)-$(patch)
# endif


# This target builds the tarballs.
#
# Not to be run by a normal user -- there are no attempts to make it
# generic.

dist:
	-rm -rf tmp
	rm -f freetype-$(version).tar.gz
	rm -f freetype-$(version).tar.xz
	rm -f ft$(winversion).zip

	for d in `find . -wholename '*/.git' -prune \
	                 -o -type f \
	                 -o -print` ; do \
	  mkdir -p tmp/$$d ; \
	done ;

	currdir=`pwd` ; \
	for f in `find . -wholename '*/.git' -prune \
	                 -o -name .gitattributes \
	                 -o -name .gitignore \
	                 -o -name .gitlab-ci.yml \
	                 -o -name .gitmodules \
	                 -o -name .mailmap \
	                 -o -type d \
	                 -o -print` ; do \
	  ln -s $$currdir/$$f tmp/$$f ; \
	done

	cd tmp ; \
	$(MAKE) devel ; \
	$(MAKE) do-dist

	mv tmp freetype-$(version)

	tar -H ustar -chf - freetype-$(version) \
	| gzip -9 -c > freetype-$(version).tar.gz
	tar -H ustar -chf - freetype-$(version) \
	| xz -c > freetype-$(version).tar.xz

	@# Use CR/LF for zip files.
	zip -lr9 ft$(winversion).zip freetype-$(version)

	rm -fr freetype-$(version)


# The locations of the latest `config.guess' and `config.sub' versions (from
# GNU `config' git repository), relative to the `tmp' directory used during
# `make dist'.
#
CONFIG_GUESS = ~/git/config/config.guess
CONFIG_SUB   = ~/git/config/config.sub

# We also use this repository to access the gnulib script that converts git
# commit messages to a ChangeLog file.
CHANGELOG_SCRIPT = ~/git/config/gitlog-to-changelog


# Don't say `make do-dist'.  Always use `make dist' instead.
#
.PHONY: do-dist

do-dist: distclean refdoc
	@# Without removing the files, `autoconf' and friends follow links.
	rm -f $(TOP_DIR)/builds/unix/aclocal.m4
	rm -f $(TOP_DIR)/builds/unix/configure.ac
	rm -f $(TOP_DIR)/builds/unix/configure

	sh autogen.sh
	rm -rf $(TOP_DIR)/builds/unix/autom4te.cache

	cp $(CONFIG_GUESS) $(TOP_DIR)/builds/unix
	cp $(CONFIG_SUB) $(TOP_DIR)/builds/unix

	@# Generate `ChangeLog' file with commits since release 2.11.0
	@# (when we stopped creating this file manually).
	$(CHANGELOG_SCRIPT) \
	  --format='%B%n' \
	  --no-cluster \
	  -- VER-2-11-0..$(version_tag) \
	> ChangeLog

	@# Remove intermediate files created by the `refdoc' target.
	rm -rf $(TOP_DIR)/docs/markdown
	rm -f $(TOP_DIR)/docs/mkdocs.yml

	@# Remove more stuff related to git.
	rm -rf $(TOP_DIR)/subprojects/dlg

# EOF
