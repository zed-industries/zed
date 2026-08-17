#
# FreeType 2 dlg logging library configuration rules
#


# Copyright (C) 2020-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# dlg logging library directory
#
DLG_DIR := $(SRC_DIR)/dlg


# compilation flags for the library
#
DLG_COMPILE := $(CC) $(ANSIFLAGS)                            \
                     $I$(subst /,$(COMPILER_SEP),$(DLG_DIR)) \
                     $(INCLUDE_FLAGS)                        \
                     $(FT_CFLAGS)


# dlg logging library sources (i.e., C files)
#
DLG_SRC := $(DLG_DIR)/dlgwrap.c

# dlg logging library headers
#
DLG_H := $(TOP_DIR)/include/dlg/dlg.h \
         $(TOP_DIR)/include/dlg/output.h


# dlg logging library object(s)
#
#   DLG_OBJ_M is used during `multi' builds
#   DLG_OBJ_S is used during `single' builds
#
DLG_OBJ_M := $(DLG_SRC:$(DLG_DIR)/%.c=$(OBJ_DIR)/%.$O)
DLG_OBJ_S := $(OBJ_DIR)/dlg.$O

# dlg logging library source file for single build
#
DLG_SRC_S := $(DLG_DIR)/dlgwrap.c


# dlg logging library - single object
#
$(DLG_OBJ_S): $(DLG_SRC_S) $(DLG_SRC) $(FREETYPE_H) $(DLG_H)
	$(DLG_COMPILE) $T$(subst /,$(COMPILER_SEP),$@ $(DLG_SRC_S))


# dlg logging library - multiple objects
#
$(OBJ_DIR)/%.$O: $(DLG_DIR)/%.c $(FREETYPE_H) $(DLG_H)
	$(DLG_COMPILE) $T$(subst /,$(COMPILER_SEP),$@ $<)


# update main object lists
#
DLG_OBJS_S += $(DLG_OBJ_S)
DLG_OBJS_M += $(DLG_OBJ_M)


# EOF
