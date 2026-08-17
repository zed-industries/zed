#
# FreeType 2 modules sub-Makefile
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# DO NOT INVOKE THIS MAKEFILE DIRECTLY!  IT IS MEANT TO BE INCLUDED BY
# OTHER MAKEFILES.


# This file is in charge of handling the generation of the modules list
# file.


# Build the modules list.
#
$(FTMODULE_H): $(MODULES_CFG)
	$(FTMODULE_H_INIT)
	$(FTMODULE_H_CREATE)
	$(FTMODULE_H_DONE)

ifneq ($(findstring $(PLATFORM),dos windows os2),)
  OPEN_MODULE   := @echo$(space)
  CLOSE_MODULE  :=  >> $(subst /,$(SEP),$(FTMODULE_H))
  REMOVE_MODULE := @-$(DELETE) $(subst /,$(SEP),$(FTMODULE_H))
else
  OPEN_MODULE   := @echo "
  CLOSE_MODULE  := " >> $(FTMODULE_H)
  REMOVE_MODULE := @-$(DELETE) $(FTMODULE_H)
endif


define FTMODULE_H_INIT
$(REMOVE_MODULE)
$(info Generating modules list in $(FTMODULE_H)...)
$(OPEN_MODULE)/* This is a generated file. */$(CLOSE_MODULE)
endef

# It is no mistake that the final closing parenthesis is on the
# next line -- it produces proper newlines during the expansion
# of `foreach'.
#
define FTMODULE_H_CREATE
$(foreach COMMAND,$(FTMODULE_H_COMMANDS),$($(COMMAND))
)
endef

define FTMODULE_H_DONE
$(OPEN_MODULE)/* EOF */$(CLOSE_MODULE)
$(info done.)
endef


# $(OPEN_DRIVER) & $(CLOSE_DRIVER) are used to specify a given font driver
# in the `module.mk' rules file.
#
OPEN_DRIVER  := $(OPEN_MODULE)FT_USE_MODULE(
CLOSE_DRIVER := )$(CLOSE_MODULE)

ECHO_DRIVER      := @echo "* module:$(space)
ECHO_DRIVER_DESC := (
ECHO_DRIVER_DONE := )"

# Each `module.mk' in the `src/*' subdirectories adds a variable with
# commands to $(FTMODULE_H_COMMANDS).  Note that we can't use SRC_DIR here.
#
-include $(patsubst %,$(TOP_DIR)/src/%/module.mk,$(MODULES))


# EOF
