#
#  Link instructions for Dos-like systems (Dos, Win32, OS/2)
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


ifdef BUILD_PROJECT

  .PHONY: clean_project distclean_project

  # Now include the main sub-makefile.  It contains all the rules used to
  # build the library with the previous variables defined.
  #
  include $(TOP_DIR)/builds/$(PROJECT).mk

  # The cleanup targets.
  #
  clean_project: clean_project_dos
  distclean_project: distclean_project_dos

  # This final rule is used to link all object files into a single library.
  # this is compiler-specific
  #
  $(PROJECT_LIBRARY): $(OBJECTS_LIST)
    ifdef CLEAN_LIBRARY
	  -$(CLEAN_LIBRARY) $(NO_OUTPUT)
    endif
	  $(LINK_LIBRARY)

endif


# EOF
