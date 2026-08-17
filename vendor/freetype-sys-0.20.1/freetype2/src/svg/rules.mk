#
# FreeType 2 SVG renderer module build rules
#


# Copyright (C) 2022-2023 by
# David Turner, Robert Wilhelm, Werner Lemberg, and Moazin Khatti.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# SVG renderer driver directory
#
SVG_DIR := $(SRC_DIR)/svg

# compilation flags for the driver
#
SVG_COMPILE := $(CC) $(ANSIFLAGS)                            \
                     $I$(subst /,$(COMPILER_SEP),$(SVG_DIR)) \
                     $(INCLUDE_FLAGS)                        \
                     $(FT_CFLAGS)

# SVG renderer sources (i.e., C files)
#
SVG_DRV_SRC := $(SVG_DIR)/ftsvg.c


# SVG renderer headers
#
SVG_DRV_H := $(SVG_DIR)/ftsvg.h    \
             $(SVG_DIR)/svgtypes.h


# SVG renderer object(s)
#
#   SVG_DRV_OBJ_M is used during `multi' builds.
#   SVG_DRV_OBJ_S is used during `single' builds.
#
SVG_DRV_OBJ_M := $(SVG_DRV_SRC:$(SVG_DIR)/%.c=$(OBJ_DIR)/%.$O)
SVG_DRV_OBJ_S := $(OBJ_DIR)/svg.$O

# SVG renderer source file for single build
#
SVG_DRV_SRC_S := $(SVG_DIR)/svg.c


# SVG renderer - single object
#
$(SVG_DRV_OBJ_S): $(SVG_DRV_SRC_S) $(SVG_DRV_SRC) \
                  $(FREETYPE_H) $(SVG_DRV_H)
	$(SVG_COMPILE) $T$(subst /,$(COMPILER_SEP),$@ $(SVG_DRV_SRC_S))


# SVG renderer - multiple objects
#
$(OBJ_DIR)/%.$O: $(SVG_DIR)/%.c $(FREETYPE_H) $(SVG_DRV_H)
	$(SVG_COMPILE) $T$(subst /,$(COMPILER_SEP),$@ $<)


# update main driver object lists
#
DRV_OBJS_S += $(SVG_DRV_OBJ_S)
DRV_OBJS_M += $(SVG_DRV_OBJ_M)


# EOF
