#
# FreeType 2 configuration file to detect a DOS host platform.
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

  # Test for DJGPP by checking the DJGPP environment variable, which must be
  # set in order to use the system (ie. it will always be present when the
  # `make' utility is run).
  #
  # We test for the COMSPEC environment variable, then run the `ver'
  # command-line program to see if its output contains the word `Dos' or
  # `DOS'.
  #
  # If this is true, we are running a Dos-ish platform (or an emulation).
  #
  ifdef DJGPP
    PLATFORM := dos
  else
    ifdef COMSPEC
      is_dos := $(findstring DOS,$(subst Dos,DOS,$(shell ver)))

      # We try to recognize a Dos session under OS/2.  The `ver' command
      # returns `Operating System/2 ...' there, so `is_dos' should be empty.
      #
      # To recognize a Dos session under OS/2, we check COMSPEC for the
      # substring `MDOS\COMMAND'
      #
      ifeq ($(is_dos),)
        is_dos := $(findstring MDOS\COMMAND,$(COMSPEC))
      endif

      # We also try to recognize Dos 7.x without Windows 9X launched.
      # See builds/windows/detect.mk for explanations about the logic.
      #
      ifeq ($(is_dos),)
        ifdef winbootdir
#ifneq ($(OS),Windows_NT)
          # If windows is available, do not trigger this test.
          ifndef windir
            is_dos := $(findstring Windows,$(strip $(shell ver)))
          endif
#endif
        endif
      endif

    endif # test COMSPEC

    ifneq ($(is_dos),)

      PLATFORM := dos

    endif # test Dos
  endif # test DJGPP
endif # test PLATFORM ansi

ifeq ($(PLATFORM),dos)

  # Use DJGPP (i.e. gcc) by default.
  #
  CONFIG_FILE := dos-gcc.mk
  CC          ?= gcc

  # additionally, we provide hooks for various other compilers
  #
  ifneq ($(findstring emx,$(MAKECMDGOALS)),)        # EMX gcc
    CONFIG_FILE := dos-emx.mk
    CC          := gcc

    .PHONY: emx
    emx: setup
	    @cd .
  endif

  ifneq ($(findstring turboc,$(MAKECMDGOALS)),)     # Turbo C
    CONFIG_FILE := dos-tcc.mk
    CC          := tcc

    .PHONY: turboc
    turboc: setup
	    @cd .
  endif

  ifneq ($(findstring watcom,$(MAKECMDGOALS)),)     # Watcom C/C++
    CONFIG_FILE := dos-wat.mk
    CC          := wcc386

    .PHONY: watcom
    watcom: setup
	    @cd .
  endif

  ifneq ($(findstring borlandc,$(MAKECMDGOALS)),)   # Borland C/C++ 32-bit
    CONFIG_FILE := dos-bcc.mk
    CC          := bcc32

    .PHONY: borlandc
    borlandc: setup
	    @cd .
  endif

  ifneq ($(findstring borlandc16,$(MAKECMDGOALS)),) # Borland C/C++ 16-bit
    CONFIG_FILE := dos-bcc.mk
    CC          := bcc

    .PHONY: borlandc16
    borlandc16: setup
	    @cd .
  endif

  ifneq ($(findstring bash,$(SHELL)),)              # check for bash
    SEP    := /
    DELETE := rm
    COPY   := cp
    CAT    := cat
    setup: std_setup
  else
    SEP    := $(BACKSLASH)
    DELETE := del
    CAT    := type

    # Setting COPY is a bit trickier.  We can be running DJGPP on some
    # Windows NT derivatives, like XP.  See builds/windows/detect.mk for
    # explanations why we need hacking here.
    #
    ifeq ($(OS),Windows_NT)
      COPY := cmd.exe /c copy
    else
      COPY := copy
    endif  # test NT

    setup: std_setup
  endif

endif     # test PLATFORM dos


# EOF
