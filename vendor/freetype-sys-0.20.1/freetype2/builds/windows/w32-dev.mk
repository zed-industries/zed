#
# FreeType 2 configuration rules for Win32 + GCC
#
#   Development version without optimizations.
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# NOTE: This version requires that GNU Make is invoked from the Windows
#       Shell (_not_ Cygwin BASH)!
#

DEVEL_DIR := $(TOP_DIR)/devel

include $(TOP_DIR)/builds/windows/win32-def.mk

include $(TOP_DIR)/builds/compiler/gcc-dev.mk

# include linking instructions
include $(TOP_DIR)/builds/link_dos.mk


# EOF
