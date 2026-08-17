#
# FreeType 2 exports sub-Makefile
#


# Copyright (C) 2005-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


# DO NOT INVOKE THIS MAKEFILE DIRECTLY!  IT IS MEANT TO BE INCLUDED BY
# OTHER MAKEFILES.


# This sub-Makefile is used to compute the list of exported symbols whenever
# the EXPORTS_LIST variable is defined by one of the platform or compiler
# specific build files.
#
# EXPORTS_LIST contains the name of the `list' file, for example a Windows
# .DEF file.
#
ifneq ($(EXPORTS_LIST),)

  # CCexe is the compiler used to compile the `apinames' tool program
  # on the host machine.  This isn't necessarily the same as the compiler
  # which can be a cross-compiler for a different architecture, for example.
  #
  ifeq ($(CCexe),)
    CCexe := $(CC)
  endif

  # TE acts like T, but for executables instead of object files.
  ifeq ($(TE),)
    TE := $T
  endif

  # The list of public headers we're going to parse.
  PUBLIC_HEADERS := $(filter-out $(PUBLIC_DIR)/ftmac.h, \
                                 $(wildcard $(PUBLIC_DIR)/*.h))
  ifneq ($(ftmac_c),)
    PUBLIC_HEADERS += $(PUBLIC_DIR)/ftmac.h
  endif

  # The `apinames' source and executable.  We use $E_BUILD as the host
  # executable suffix, which *includes* the final dot.
  #
  # Note that $(APINAMES_OPTIONS) is empty, except for Windows compilers.
  #
  APINAMES_SRC := $(subst /,$(SEP),$(TOP_DIR)/src/tools/apinames.c)
  APINAMES_EXE := $(subst /,$(SEP),$(OBJ_DIR)/apinames$(E_BUILD))

  $(APINAMES_EXE): $(APINAMES_SRC)
	  $(CCexe) $(CCexe_CFLAGS) $(TE)$@ $< $(CCexe_LDFLAGS)

  .PHONY: symbols_list

  symbols_list: $(EXPORTS_LIST)

  # We manually add TT_New_Context and TT_RunIns, which are needed by TT
  # debuggers, to the EXPORTS_LIST.
  #
  $(EXPORTS_LIST): $(APINAMES_EXE) $(PUBLIC_HEADERS)
	  $(subst /,$(SEP),$(APINAMES_EXE)) -o$@ $(APINAMES_OPTIONS) $(PUBLIC_HEADERS)
	  @echo TT_New_Context >> $(EXPORTS_LIST)
	  @echo TT_RunIns >> $(EXPORTS_LIST)

  $(PROJECT_LIBRARY): $(EXPORTS_LIST)

  CLEAN += $(EXPORTS_LIST) \
           $(APINAMES_EXE)

endif


# EOF
