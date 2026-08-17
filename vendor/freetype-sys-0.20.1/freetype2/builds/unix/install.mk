#
# FreeType 2 installation instructions for Unix systems
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.

# If you say
#
#   make install DESTDIR=/tmp/somewhere/
#
# don't forget the final backslash (this command is mainly for package
# maintainers).


.PHONY: install uninstall check

# Unix installation and deinstallation targets.
#
# Note that we remove any data found in `$(includedir)/freetype2' before
# installing new files to avoid interferences with files installed by
# previous FreeType versions (which use slightly different locations).
#
# We also remove `$(includedir)/ft2build.h' for the same reason.
#
# Note that some header files get handled twice for simplicity; a special,
# configured version overwrites the generic one.
#
install: $(PROJECT_LIBRARY)
	-$(DELDIR) $(DESTDIR)$(includedir)/freetype2
	-$(DELETE) $(DESTDIR)$(includedir)/ft2build.h
	$(MKINSTALLDIRS) $(DESTDIR)$(libdir)                               \
                         $(DESTDIR)$(libdir)/pkgconfig                     \
                         $(DESTDIR)$(includedir)/freetype2/freetype/config \
                         $(DESTDIR)$(datadir)/aclocal
ifeq ($(INSTALL_FT2_CONFIG),TRUE)
	$(MKINSTALLDIRS) $(DESTDIR)$(bindir)                               \
                         $(DESTDIR)$(mandir)/man1
endif
	$(LIBTOOL) --mode=install $(INSTALL)                             \
                                  $(PROJECT_LIBRARY) $(DESTDIR)$(libdir)
	-for P in $(PUBLIC_H) ; do                           \
          $(INSTALL_DATA)                                    \
            $$P $(DESTDIR)$(includedir)/freetype2/freetype ; \
        done
	-for P in $(CONFIG_H) ; do                                  \
          $(INSTALL_DATA)                                           \
            $$P $(DESTDIR)$(includedir)/freetype2/freetype/config ; \
        done
	$(INSTALL_DATA) $(TOP_DIR)/include/ft2build.h                  \
          $(DESTDIR)$(includedir)/freetype2/ft2build.h
	$(INSTALL_DATA) $(OBJ_BUILD)/ftconfig.h                        \
          $(DESTDIR)$(includedir)/freetype2/freetype/config/ftconfig.h
	$(INSTALL_DATA) $(OBJ_DIR)/ftmodule.h                          \
          $(DESTDIR)$(includedir)/freetype2/freetype/config/ftmodule.h
	$(INSTALL_DATA) $(OBJ_BUILD)/ftoption.h                        \
          $(DESTDIR)$(includedir)/freetype2/freetype/config/ftoption.h
	$(INSTALL_SCRIPT) -m 644 $(PLATFORM_DIR)/freetype2.m4          \
          $(DESTDIR)$(datadir)/aclocal/freetype2.m4
	$(INSTALL_SCRIPT) -m 644 $(OBJ_BUILD)/freetype2.pc             \
          $(DESTDIR)$(libdir)/pkgconfig/freetype2.pc
ifeq ($(INSTALL_FT2_CONFIG),TRUE)
	$(INSTALL_SCRIPT) -m 755 $(OBJ_BUILD)/freetype-config          \
          $(DESTDIR)$(bindir)/freetype-config
	$(INSTALL_DATA) $(TOP_DIR)/docs/freetype-config.1              \
          $(DESTDIR)$(mandir)/man1/freetype-config.1
endif


uninstall:
	-$(LIBTOOL) --mode=uninstall $(RM) $(DESTDIR)$(libdir)/$(LIBRARY).$A
	-$(DELDIR) $(DESTDIR)$(includedir)/freetype2
	-$(DELETE) $(DESTDIR)$(bindir)/freetype-config
	-$(DELETE) $(DESTDIR)$(datadir)/aclocal/freetype2.m4
	-$(DELETE) $(DESTDIR)$(libdir)/pkgconfig/freetype2.pc
	-$(DELETE) $(DESTDIR)$(mandir)/man1/freetype-config.1


check:
	$(info There is no validation suite for this package.)


.PHONY: clean_project_unix distclean_project_unix

# Unix cleaning and distclean rules.
#
clean_project_unix:
	-$(LIBTOOL) --mode=clean $(RM) $(OBJECTS_LIST)
	-$(DELETE) $(CLEAN)

distclean_project_unix: clean_project_unix
	-$(LIBTOOL) --mode=clean $(RM) $(PROJECT_LIBRARY)
	-$(DELETE) *.orig *~ core *.core $(DISTCLEAN)

# EOF
