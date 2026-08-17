#
# FreeType 2 configuration file to detect a Win32 host platform.
#


# Copyright (C) 1996-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


.PHONY: setup


ifeq ($(PLATFORM),ansi)

  # Detecting Windows NT is easy, as the OS variable must be defined and
  # contains `Windows_NT'.  This also works with Windows 2000 and XP.
  #
  ifeq ($(OS),Windows_NT)

    PLATFORM := windows

  else

    # Detecting Windows 9X

    # We used to run the `ver' command to see if its output contains the
    # word `Windows'.  If this is true, we are running Windows 95 or later:
    #
    #   ifdef COMSPEC
    #     # First, check if we have the COMSPEC environment variable, which
    #     # indicates we can use COMMAND.COM's internal commands
    #     is_windows := $(findstring Windows,$(strip $(shell ver)))
    #   endif
    #
    # Unfortunately, this also detects the case when one is running
    # DOS 7.x (the MS-DOS version that lies below Windows) without actually
    # launching the GUI.
    #
    # A better test is to check whether there are both the environment
    # variables `winbootdir' and `windir'.  The first indicates an
    # underlying DOS 7.x, while the second is set only if windows is
    # available.
    #
    # Note that on Windows NT, such an environment variable will not be seen
    # from DOS-based tools like DJGPP's make; this is not actually a problem
    # since NT is detected independently above.  But do not try to be clever!
    #
    ifdef winbootdir
      ifdef windir

        PLATFORM := windows

      endif
    endif

  endif  # test NT

endif # test PLATFORM ansi

ifeq ($(PLATFORM),windows)

  DELETE := del
  CAT    := type
  SEP    := $(BACKSLASH)

  # Setting COPY is a bit trickier.  Plain COPY on NT will not work
  # correctly, because it will uppercase 8.3 filenames, creating a
  # `CONFIG.MK' file which isn't found later on by `make'.
  # Since we do not want that, we need to force execution of CMD.EXE.
  # Unfortunately, CMD.EXE is not available on Windows 9X.
  # So we need to hack.
  #
  # Kudos to Eli Zaretskii (DJGPP guru) that helped debug it.
  # Details are available in threads of the FreeType mailing list
  # (2004-11-11), and then in the devel mailing list (2004-11-20 to -23).
  #
  ifeq ($(OS),Windows_NT)
    COPY := >nul cmd.exe /c copy
  else
    COPY := >nul copy
  endif  # test NT


  # gcc Makefile by default
  CONFIG_FILE := w32-gcc.mk
  ifeq ($(firstword $(CC)),cc)
    CC        := gcc
  endif

  ifneq ($(findstring list,$(MAKECMDGOALS)),)  # test for the "list" target
    dump_target_list:
	    $(info )
	    $(info $(PROJECT_TITLE) build system -- supported compilers)
	    $(info )
	    $(info Several command-line compilers are supported on Win32:)
	    $(info )
	    $(info $(empty)  make setup                     gcc (with Mingw))
	    $(info $(empty)  make setup visualc             Microsoft Visual C++)
	    $(info $(empty)  make setup bcc32               Borland C/C++)
	    $(info $(empty)  make setup lcc                 Win32-LCC)
	    $(info $(empty)  make setup intelc              Intel C/C++)
	    $(info )

    setup: dump_target_list
    .PHONY: dump_target_list list
  else
    setup: std_setup
  endif

  # additionally, we provide hooks for various other compilers
  #
  ifneq ($(findstring visualc,$(MAKECMDGOALS)),)     # Visual C/C++
    CONFIG_FILE := w32-vcc.mk
    CC          := cl

    .PHONY: visualc
    visualc: setup
	    @cd .
  endif

  ifneq ($(findstring intelc,$(MAKECMDGOALS)),)      # Intel C/C++
    CONFIG_FILE := w32-intl.mk
    CC          := cl

    .PHONY: intelc
    visualc: setup
	    @cd .
  endif

  ifneq ($(findstring watcom,$(MAKECMDGOALS)),)      # Watcom C/C++
    CONFIG_FILE := w32-wat.mk
    CC          := wcc386

    .PHONY: watcom
    watcom: setup
	    @cd .
  endif

  ifneq ($(findstring visualage,$(MAKECMDGOALS)),)   # Visual Age C++
    CONFIG_FILE := w32-icc.mk
    CC          := icc

    .PHONY: visualage
    visualage: setup
	    @cd .
  endif

  ifneq ($(findstring lcc,$(MAKECMDGOALS)),)         # LCC-Win32
    CONFIG_FILE := w32-lcc.mk
    CC          := lcc

    .PHONY: lcc
    lcc: setup
	    @cd .
  endif

  ifneq ($(findstring mingw32,$(MAKECMDGOALS)),)     # mingw32
    CONFIG_FILE := w32-mingw32.mk
    CC          := gcc

    .PHONY: mingw32
    mingw32: setup
	    @cd .
  endif

  ifneq ($(findstring bcc32,$(MAKECMDGOALS)),)       # Borland C++
    CONFIG_FILE := w32-bcc.mk
    CC          := bcc32

    .PHONY: bcc32
    bcc32: setup
	    @cd .
  endif

  ifneq ($(findstring devel-bcc,$(MAKECMDGOALS)),)   # development target
    CONFIG_FILE := w32-bccd.mk
    CC          := bcc32

    .PHONY: devel-bcc
    devel-bcc: setup
	    @cd .
  endif

  ifneq ($(findstring devel-gcc,$(MAKECMDGOALS)),)   # development target
    CONFIG_FILE := w32-dev.mk
    CC          := gcc

    .PHONY: devel-gcc
    devel-gcc: setup
	    @cd .
  endif

endif   # test PLATFORM windows


# EOF
