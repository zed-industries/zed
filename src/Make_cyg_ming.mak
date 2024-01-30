# Makefile for VIM on Win32 (Cygwin and MinGW)
#
# This file contains common part for Cygwin and MinGW and it is included
# from Make_cyg.mak and Make_ming.mak.
#
# Info at http://www.mingw.org
# Alternative x86 and 64-builds: http://mingw-w64.sourceforge.net
# Also requires GNU make, which you can download from the same sites.
# Get missing libraries from http://gnuwin32.sf.net.
#
# Tested on Win32 NT 4 and Win95.
#
# To make everything, just 'make -f Make_ming.mak'.
# To make just e.g. gvim.exe, 'make -f Make_ming.mak gvim.exe'.
# After a run, you can 'make -f Make_ming.mak clean' to clean up.
#
# NOTE: Sometimes 'GNU Make' will stop after building vimrun.exe -- I think
# it's just run out of memory or something.  Run again, and it will continue
# with 'xxd'.
#
# "make upx" makes *compressed* versions of the 32 bit GUI and console EXEs,
# using the excellent UPX compressor:
#     https://upx.github.io/
# "make mpress" uses the MPRESS compressor for 32- and 64-bit EXEs:
#     http://www.matcode.com/mpress.htm
#
# Maintained by Ron Aaron <ronaharon@yahoo.com> et al.
# Updated 2014 Oct 13.

#>>>>> choose options:
# FEATURES=[TINY | NORMAL | HUGE]
# Set to TINY to make a minimal version (no optional features).
FEATURES=HUGE

# Set to yes for a debug build.
DEBUG=no

# Set to yes to create a mapfile.
#MAP=yes

# Set to yes to measure code coverage.
COVERAGE=no

# Better encryption support using libsodium.
# Set to yes or specify the path to the libsodium directory to enable it.
#SODIUM=yes

# Set to SIZE for size, SPEED for speed, MAXSPEED for maximum optimization.
OPTIMIZE=MAXSPEED

# Set to yes to make gvim, no for vim.
GUI=yes

# Set to yes to enable the DLL support (EXPERIMENTAL).
# Creates vim{32,64}.dll, and stub gvim.exe and vim.exe.
# "GUI" should be also set to "yes".
#VIMDLL=yes

# Set to no if you do not want to use DirectWrite (DirectX).
# MinGW-w64 is needed, and ARCH should be set to i686 or x86-64.
DIRECTX=yes

# Disable Color emoji support
# (default is yes if DIRECTX=yes, requires WinSDK 8.1 or later.)
#COLOR_EMOJI=no

# Set to one of i386, i486, i586, i686 as the minimum target processor.
# For amd64/x64 architecture set ARCH=x86-64 .
# If not set, it will be automatically detected. (Normally i686 or x86-64.)
#ARCH=i686
# Set to yes to cross-compile from unix; no=native Windows (and Cygwin).
CROSS=no

# Set to path to iconv.h and libiconv.a to enable using 'iconv.dll'.
# Use "yes" when the path does not need to be define.
#ICONV="."
ICONV=yes
GETTEXT=yes

# Set to yes to include IME support.
IME=yes
DYNAMIC_IME=yes

# Set to yes to enable writing a postscript file with :hardcopy.
POSTSCRIPT=no

# Set to yes to enable OLE support.
OLE=no

# Set the default $(WINVER).  Use 0x0601 to make it work with Windows 7.
ifndef WINVER
WINVER = 0x0601
endif

# Set to yes to enable Cscope support.
CSCOPE=yes

# Set to yes to enable Netbeans support (requires CHANNEL).
NETBEANS=$(GUI)

# Set to yes to enable inter process communication.
ifeq (HUGE, $(FEATURES))
CHANNEL=yes
else
CHANNEL=$(GUI)
endif

# Set to yes to enable terminal support.
ifeq (HUGE, $(FEATURES))
TERMINAL=yes
else
TERMINAL=no
endif

# Set to yes to enable sound support.
ifneq ($(findstring $(FEATURES),HUGE),)
SOUND=yes
else
SOUND=no
endif

ifndef CTAGS
# this assumes ctags is Exuberant ctags
CTAGS = ctags -I INIT+,INIT2+,INIT3+,INIT4+,INIT5+ --fields=+S
endif

# Link against the shared version of libstdc++ by default.  Set
# STATIC_STDCPLUS to "yes" to link against static version instead.
ifndef STATIC_STDCPLUS
STATIC_STDCPLUS=no
endif


# Link against the shared version of libwinpthread by default.  Set
# STATIC_WINPTHREAD to "yes" to link against static version instead.
ifndef STATIC_WINPTHREAD
STATIC_WINPTHREAD=$(STATIC_STDCPLUS)
endif
# If you use TDM-GCC(-64), change HAS_GCC_EH to "no".
# This is used when STATIC_STDCPLUS=yes.
HAS_GCC_EH=yes

# If the user doesn't want gettext, undefine it.
ifeq (no, $(GETTEXT))
GETTEXT=
endif
# Added by E.F. Amatria <eferna1@platea.ptic.mec.es> 2001 Feb 23
# Uncomment the first line and one of the following three if you want Native Language
# Support.  You'll need gnu_gettext.win32, a MINGW32 Windows PORT of gettext by
# Franco Bez <franco.bez@gmx.de>.  It may be found at
# http://home.a-city.de/franco.bez/gettext/gettext_win32_en.html
# Tested with mingw32 with GCC-2.95.2 on Win98
# Updated 2001 Jun 9
#GETTEXT=c:/gettext.win32.msvcrt
#STATIC_GETTEXT=USE_STATIC_GETTEXT
#DYNAMIC_GETTEXT=USE_GETTEXT_DLL
#DYNAMIC_GETTEXT=USE_SAFE_GETTEXT_DLL
SAFE_GETTEXT_DLL_OBJ = $(GETTEXT)/src/safe_gettext_dll/safe_gettext_dll.o
# Alternatively, if you uncomment the two following lines, you get a "safe" version
# without linking the safe_gettext_dll.o object file.
#DYNAMIC_GETTEXT=DYNAMIC_GETTEXT
#GETTEXT_DYNAMIC=gnu_gettext.dll
INTLPATH=$(GETTEXT)/lib/mingw32
INTLLIB=gnu_gettext

# If you are using gettext-0.10.35 from http://sourceforge.net/projects/gettext
# or gettext-0.10.37 from http://sourceforge.net/projects/mingwrep/
# uncomment the following, but I can't build a static version with them, ?-(|
#GETTEXT=c:/gettext-0.10.37-20010430
#STATIC_GETTEXT=USE_STATIC_GETTEXT
#DYNAMIC_GETTEXT=DYNAMIC_GETTEXT
#INTLPATH=$(GETTEXT)/lib
#INTLLIB=intl


# Command definitions (depends on cross-compiling and shell)
ifeq ($(CROSS),yes)
# cross-compiler prefix:
 ifndef CROSS_COMPILE
CROSS_COMPILE = i586-pc-mingw32msvc-
 endif
DEL = rm
MKDIR = mkdir -p
DIRSLASH = /
else
# normal (Windows) compilation:
 ifndef CROSS_COMPILE
CROSS_COMPILE =
 endif

# About the "sh.exe" condition, as explained by Ken Takata:
#
# If the makefile is executed with mingw32-make and sh.exe is not found in
# $PATH, then $SHELL is set to "sh.exe" (without any path). In this case,
# unix-like commands might not work and a dos-style path is needed.
# 
# If the makefile is executed with mingw32-make and sh.exe IS found in $PATH,
# then $SHELL is set with the actual path of sh.exe (e.g.
# "C:/msys64/usr/bin/sh.exe").  In this case, unix-like commands can be used.
# 
# If it is executed by the "make" command from cmd.exe, $SHELL is set to
# "/bin/sh". If the "make" command is in the $PATH, other unix-like commands
# might also work.
# 
# If it is executed by the "make" command from a unix-like shell,
# $SHELL is set with the unix-style path (e.g. "/bin/bash").
# In this case, unix-like commands can be used.
#
 ifneq (sh.exe, $(SHELL))
DEL = rm
MKDIR = mkdir -p
DIRSLASH = /
 else
DEL = del
MKDIR = mkdir
DIRSLASH = \\
 endif
endif
# set $CC to "gcc" unless it matches "clang"
ifeq ($(findstring clang,$(CC)),)
CC := $(CROSS_COMPILE)gcc
endif
# set $CXX to "g++" unless it matches "clang"
ifeq ($(findstring clang,$(CXX)),)
CXX := $(CROSS_COMPILE)g++
endif
ifeq ($(UNDER_CYGWIN),yes)
WINDRES := $(CROSS_COMPILE)windres
else ifeq ($(findstring clang,$(CC)),)
WINDRES := windres
else
WINDRES := llvm-windres
endif

# Get the default ARCH.
ifndef ARCH
ARCH := $(shell $(CC) -dumpmachine | sed -e 's/-.*//' -e 's/_/-/' -e 's/^mingw32$$/i686/')
endif


#	Perl interface:
#	  PERL=[Path to Perl directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  DYNAMIC_PERL=yes (to load the Perl DLL dynamically)
#	  PERL_VER=[Perl version, eg 56, 58, 510] (default is 524)
ifdef PERL
 ifndef PERL_VER
PERL_VER=524
 endif
 ifndef DYNAMIC_PERL
DYNAMIC_PERL=yes
 endif
# on Linux, for cross-compile, it's here:
#PERLLIB=/home/ron/ActivePerl/lib
# on NT, it's here:
PERLEXE=$(PERL)/bin/perl
PERLLIB=$(PERL)/lib
PERLLIBS=$(PERLLIB)/Core
 ifeq ($(UNDER_CYGWIN),yes)
PERLTYPEMAP:=$(shell cygpath -m $(PERLLIB)/ExtUtils/typemap)
XSUBPPTRY:=$(shell cygpath -m $(PERLLIB)/ExtUtils/xsubpp)
 else
PERLTYPEMAP=$(PERLLIB)/ExtUtils/typemap
XSUBPPTRY=$(PERLLIB)/ExtUtils/xsubpp
 endif
XSUBPP_EXISTS=$(shell $(PERLEXE) -e "print 1 unless -e '$(XSUBPPTRY)'")
 ifeq "$(XSUBPP_EXISTS)" ""
XSUBPP=$(PERLEXE) $(XSUBPPTRY)
 else
XSUBPP=xsubpp
 endif
endif

#	Lua interface:
#	  LUA=[Path to Lua directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  LUA_LIBDIR=[Path to Lua library directory] (default: $LUA/lib)
#	  LUA_INCDIR=[Path to Lua include directory] (default: $LUA/include)
#	  DYNAMIC_LUA=yes (to load the Lua DLL dynamically)
#	  LUA_VER=[Lua version, eg 51, 52] (default is 53)
ifdef LUA
 ifndef DYNAMIC_LUA
DYNAMIC_LUA=yes
 endif

 ifndef LUA_VER
LUA_VER=53
 endif

 ifeq (no,$(DYNAMIC_LUA))
LUA_LIBDIR = $(LUA)/lib
LUA_LIB = -L$(LUA_LIBDIR) -llua
 endif

endif

#	MzScheme interface:
#	  MZSCHEME=[Path to MzScheme directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  DYNAMIC_MZSCHEME=yes (to load the MzScheme DLL dynamically)
#	  MZSCHEME_VER=[MzScheme version] (default is 3m_a0solc (6.6))
#	  	Used for the DLL file name. E.g.:
#	  	C:\Program Files (x86)\Racket\lib\libracket3m_XXXXXX.dll
#	  MZSCHEME_DEBUG=no
ifdef MZSCHEME
 ifndef DYNAMIC_MZSCHEME
DYNAMIC_MZSCHEME=yes
 endif

 ifndef MZSCHEME_VER
MZSCHEME_VER=3m_a0solc
 endif

# for version 4.x we need to generate byte-code for Scheme base
 ifndef MZSCHEME_GENERATE_BASE
MZSCHEME_GENERATE_BASE=no
 endif

 ifneq ($(wildcard $(MZSCHEME)/lib/msvc/libmzsch$(MZSCHEME_VER).lib),)
MZSCHEME_MAIN_LIB=mzsch
 else
MZSCHEME_MAIN_LIB=racket
 endif

 ifndef MZSCHEME_PRECISE_GC
MZSCHEME_PRECISE_GC=no
  ifneq ($(wildcard $(MZSCHEME)\lib\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll),)
   ifeq ($(wildcard $(MZSCHEME)\lib\libmzgc$(MZSCHEME_VER).dll),)
MZSCHEME_PRECISE_GC=yes
   endif
  else
   ifneq ($(wildcard $(MZSCHEME)\lib\msvc\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib),)
    ifeq ($(wildcard $(MZSCHEME)\lib\msvc\libmzgc$(MZSCHEME_VER).lib),)
MZSCHEME_PRECISE_GC=yes
    endif
   endif
  endif
 endif

 ifeq (no,$(DYNAMIC_MZSCHEME))
  ifeq (yes,$(MZSCHEME_PRECISE_GC))
MZSCHEME_LIB=-l$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER)
  else
MZSCHEME_LIB=-l$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER) -lmzgc$(MZSCHEME_VER)
  endif
# the modern MinGW can dynamically link to dlls directly.
# point MZSCHEME_DLLS to where you put libmzschXXXXXXX.dll and libgcXXXXXXX.dll
  ifndef MZSCHEME_DLLS
MZSCHEME_DLLS=$(MZSCHEME)
  endif
MZSCHEME_LIBDIR=-L$(MZSCHEME_DLLS) -L$(MZSCHEME_DLLS)\lib
 endif

endif

#	Python interface:
#	  PYTHON=[Path to Python directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  DYNAMIC_PYTHON=yes (to load the Python DLL dynamically)
#	  PYTHON_VER=[Python version, eg 22, 23, ..., 27] (default is 27)
ifdef PYTHON
 ifndef DYNAMIC_PYTHON
DYNAMIC_PYTHON=yes
 endif

 ifndef PYTHON_VER
PYTHON_VER=27
 endif
 ifndef DYNAMIC_PYTHON_DLL
DYNAMIC_PYTHON_DLL=python$(PYTHON_VER).dll
 endif
 ifdef PYTHON_HOME
PYTHON_HOME_DEF=-DPYTHON_HOME=\"$(PYTHON_HOME)\"
 endif

 ifeq (no,$(DYNAMIC_PYTHON))
PYTHONLIB=-L$(PYTHON)/libs -lpython$(PYTHON_VER)
 endif
# my include files are in 'win32inc' on Linux, and 'include' in the standard
# NT distro (ActiveState)
 ifndef PYTHONINC
  ifeq ($(CROSS),no)
PYTHONINC=-I $(PYTHON)/include
  else
PYTHONINC=-I $(PYTHON)/win32inc
  endif
 endif
endif

#	Python3 interface:
#	  PYTHON3=[Path to Python3 directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  DYNAMIC_PYTHON3=yes (to load the Python3 DLL dynamically)
#	  PYTHON3_VER=[Python3 version, eg 31, 32] (default is 36)
ifdef PYTHON3
 ifndef DYNAMIC_PYTHON3
DYNAMIC_PYTHON3=yes
 endif

 ifndef PYTHON3_VER
PYTHON3_VER=36
 endif
 ifeq ($(DYNAMIC_PYTHON3_STABLE_ABI),yes)
PYTHON3_NAME=python3
 else
PYTHON3_NAME=python$(PYTHON3_VER)
 endif
 ifndef DYNAMIC_PYTHON3_DLL
DYNAMIC_PYTHON3_DLL=$(PYTHON3_NAME).dll
 endif
 ifdef PYTHON3_HOME
PYTHON3_HOME_DEF=-DPYTHON3_HOME=L\"$(PYTHON3_HOME)\"
 endif

 ifeq (no,$(DYNAMIC_PYTHON3))
PYTHON3LIB=-L$(PYTHON3)/libs -l$(PYTHON3_NAME)
 endif

 ifndef PYTHON3INC
  ifeq ($(CROSS),no)
PYTHON3INC=-I $(PYTHON3)/include
  else
PYTHON3INC=-I $(PYTHON3)/win32inc
  endif
  ifeq ($(DYNAMIC_PYTHON3_STABLE_ABI),yes)
PYTHON3INC += -DPy_LIMITED_API=0x3080000
  endif
 endif
endif

#	TCL interface:
#	  TCL=[Path to TCL directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  DYNAMIC_TCL=yes (to load the TCL DLL dynamically)
#	  TCL_VER=[TCL version, eg 83, 84] (default is 86)
#	  TCL_VER_LONG=[Tcl version, eg 8.3] (default is 8.6)
#	    You must set TCL_VER_LONG when you set TCL_VER.
#	  TCL_DLL=[TCL dll name, eg tcl86.dll] (default is tcl86.dll)
ifdef TCL
 ifndef DYNAMIC_TCL
DYNAMIC_TCL=yes
 endif
 ifndef TCL_VER
TCL_VER = 86
 endif
 ifndef TCL_VER_LONG
TCL_VER_LONG = 8.6
 endif
 ifndef TCL_DLL
TCL_DLL = tcl$(TCL_VER).dll
 endif
TCLINC += -I$(TCL)/include
endif


#	Ruby interface:
#	  RUBY=[Path to Ruby directory] (Set inside Make_cyg.mak or Make_ming.mak)
#	  DYNAMIC_RUBY=yes (to load the Ruby DLL dynamically, "no" for static)
#	  RUBY_VER=[Ruby version, eg 19, 22] (default is 22)
#	  RUBY_API_VER_LONG=[Ruby API version, eg 1.9.1, 2.2.0]
#			    (default is 2.2.0)
#	    You must set RUBY_API_VER_LONG when changing RUBY_VER.
#	    Note: If you use Ruby 1.9.3, set as follows:
#	      RUBY_VER=19
#	      RUBY_API_VER_LONG=1.9.1 (not 1.9.3, because the API version is 1.9.1.)
ifdef RUBY
 ifndef DYNAMIC_RUBY
DYNAMIC_RUBY=yes
 endif
#  Set default value
 ifndef RUBY_VER
RUBY_VER = 22
 endif
 ifndef RUBY_VER_LONG
RUBY_VER_LONG = 2.2.0
 endif
 ifndef RUBY_API_VER_LONG
RUBY_API_VER_LONG = $(RUBY_VER_LONG)
 endif
 ifndef RUBY_API_VER
RUBY_API_VER = $(subst .,,$(RUBY_API_VER_LONG))
 endif

 ifndef RUBY_PLATFORM
  ifneq ($(wildcard $(RUBY)/lib/ruby/$(RUBY_API_VER_LONG)/i386-mingw32),)
RUBY_PLATFORM = i386-mingw32
  else ifneq ($(wildcard $(RUBY)/lib/ruby/$(RUBY_API_VER_LONG)/x64-mingw32),)
RUBY_PLATFORM = x64-mingw32
  else ifneq ($(wildcard $(RUBY)/lib/ruby/$(RUBY_API_VER_LONG)/x64-mingw-ucrt),)
RUBY_PLATFORM = x64-mingw-ucrt
  else
RUBY_PLATFORM = i386-mswin32
  endif
 endif

 ifndef RUBY_INSTALL_NAME
  ifndef RUBY_MSVCRT_NAME
# Base name of msvcrXX.dll which is used by ruby's dll.
RUBY_MSVCRT_NAME = msvcrt
  endif
  ifeq ($(RUBY_PLATFORM),x64-mingw-ucrt)
RUBY_INSTALL_NAME = x64-ucrt-ruby$(RUBY_API_VER)
  else ifeq ($(ARCH),x86-64)
RUBY_INSTALL_NAME = x64-$(RUBY_MSVCRT_NAME)-ruby$(RUBY_API_VER)
  else
RUBY_INSTALL_NAME = $(RUBY_MSVCRT_NAME)-ruby$(RUBY_API_VER)
  endif
 endif

RUBYINC = -I $(RUBY)/include/ruby-$(RUBY_API_VER_LONG) -I $(RUBY)/include/ruby-$(RUBY_API_VER_LONG)/$(RUBY_PLATFORM)
 ifeq (no, $(DYNAMIC_RUBY))
RUBYLIB = -L$(RUBY)/lib -l$(RUBY_INSTALL_NAME)
 endif

endif # RUBY

# See feature.h for a list of options.
# Any other defines can be included here.
DEF_GUI=-DFEAT_GUI_MSWIN -DFEAT_CLIPBOARD
DEFINES=-DWIN32 -DWINVER=$(WINVER) -D_WIN32_WINNT=$(WINVER) \
	-DHAVE_PATHDEF -DFEAT_$(FEATURES) -DHAVE_STDINT_H

#>>>>> end of choices
###########################################################################

CFLAGS = -I. -Iproto $(DEFINES) -pipe -march=$(ARCH) -Wall
# To get additional compiler warnings
#CFLAGS += -Wextra -pedantic
CXXFLAGS = -std=gnu++11
# This used to have --preprocessor, but it's no longer supported
WINDRES_FLAGS =
EXTRA_LIBS =

ifdef GETTEXT
DEFINES += -DHAVE_GETTEXT -DHAVE_LOCALE_H
GETTEXTINCLUDE = $(GETTEXT)/include
GETTEXTLIB = $(INTLPATH)
 ifeq (yes, $(GETTEXT))
DEFINES += -DDYNAMIC_GETTEXT
 else ifdef DYNAMIC_GETTEXT
DEFINES += -D$(DYNAMIC_GETTEXT)
  ifdef GETTEXT_DYNAMIC
DEFINES += -DGETTEXT_DYNAMIC -DGETTEXT_DLL=\"$(GETTEXT_DYNAMIC)\"
  endif
 endif
endif

ifdef PERL
CFLAGS += -I$(PERLLIBS) -DFEAT_PERL -DPERL_IMPLICIT_CONTEXT -DPERL_IMPLICIT_SYS
 ifeq (yes, $(DYNAMIC_PERL))
CFLAGS += -DDYNAMIC_PERL -DDYNAMIC_PERL_DLL=\"perl$(PERL_VER).dll\"
EXTRA_LIBS += -L$(PERLLIBS) -lperl$(PERL_VER)
 endif
endif

ifdef LUA
LUA_INCDIR = $(LUA)/include
CFLAGS += -I$(LUA_INCDIR) -I$(LUA) -DFEAT_LUA
 ifeq (yes, $(DYNAMIC_LUA))
CFLAGS += -DDYNAMIC_LUA -DDYNAMIC_LUA_DLL=\"lua$(LUA_VER).dll\"
 endif
endif

ifdef MZSCHEME
 ifndef MZSCHEME_COLLECTS
MZSCHEME_COLLECTS=$(MZSCHEME)/collects
  ifeq (yes, $(UNDER_CYGWIN))
MZSCHEME_COLLECTS:=$(shell cygpath -m $(MZSCHEME_COLLECTS) | sed -e 's/ /\\ /g')
  endif
 endif
CFLAGS += -I$(MZSCHEME)/include -DFEAT_MZSCHEME -DMZSCHEME_COLLECTS=\"$(MZSCHEME_COLLECTS)\"
 ifeq (yes, $(DYNAMIC_MZSCHEME))
  ifeq (yes, $(MZSCHEME_PRECISE_GC))
# Precise GC does not use separate dll
CFLAGS += -DDYNAMIC_MZSCHEME -DDYNAMIC_MZSCH_DLL=\"lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll\" -DDYNAMIC_MZGC_DLL=\"lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll\"
  else
CFLAGS += -DDYNAMIC_MZSCHEME -DDYNAMIC_MZSCH_DLL=\"lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll\" -DDYNAMIC_MZGC_DLL=\"libmzgc$(MZSCHEME_VER).dll\"
  endif
 endif
 ifeq (yes, "$(MZSCHEME_DEBUG)")
CFLAGS += -DMZSCHEME_FORCE_GC
 endif
endif

ifdef RUBY
CFLAGS += -DFEAT_RUBY $(RUBYINC)
 ifeq (yes, $(DYNAMIC_RUBY))
CFLAGS += -DDYNAMIC_RUBY -DDYNAMIC_RUBY_DLL=\"$(RUBY_INSTALL_NAME).dll\"
 endif
CFLAGS += -DRUBY_VERSION=$(RUBY_VER)
 ifneq ($(findstring w64-mingw32,$(CC)),)
# A workaround for MinGW-w64
CFLAGS += -DHAVE_STRUCT_TIMESPEC -DHAVE_STRUCT_TIMEZONE
 endif
endif

ifdef PYTHON
CFLAGS += -DFEAT_PYTHON
 ifeq (yes, $(DYNAMIC_PYTHON))
CFLAGS += -DDYNAMIC_PYTHON -DDYNAMIC_PYTHON_DLL=\"$(DYNAMIC_PYTHON_DLL)\"
 endif
endif

ifdef PYTHON3
CFLAGS += -DFEAT_PYTHON3
 ifeq (yes, $(DYNAMIC_PYTHON3))
CFLAGS += -DDYNAMIC_PYTHON3 -DDYNAMIC_PYTHON3_DLL=\"$(DYNAMIC_PYTHON3_DLL)\"
  ifeq (yes, $(DYNAMIC_PYTHON3_STABLE_ABI))
CFLAGS += -DDYNAMIC_PYTHON3_STABLE_ABI
  endif
 else
CFLAGS += -DPYTHON3_DLL=\"$(DYNAMIC_PYTHON3_DLL)\"
 endif
endif

ifdef TCL
CFLAGS += -DFEAT_TCL $(TCLINC)
 ifeq (yes, $(DYNAMIC_TCL))
CFLAGS += -DDYNAMIC_TCL -DDYNAMIC_TCL_DLL=\"$(TCL_DLL)\" -DDYNAMIC_TCL_VER=\"$(TCL_VER_LONG)\"
 endif
endif

ifeq ($(POSTSCRIPT),yes)
DEFINES += -DMSWINPS
endif

ifeq (yes, $(OLE))
DEFINES += -DFEAT_OLE
endif

ifeq ($(CSCOPE),yes)
DEFINES += -DFEAT_CSCOPE
endif

ifeq ($(NETBEANS),yes)
# Only allow NETBEANS for a GUI build.
 ifeq (yes, $(GUI))
DEFINES += -DFEAT_NETBEANS_INTG

  ifeq ($(NBDEBUG), yes)
DEFINES += -DNBDEBUG
NBDEBUG_INCL = nbdebug.h
NBDEBUG_SRC = nbdebug.c
  endif
 endif
endif

ifeq ($(CHANNEL),yes)
DEFINES += -DFEAT_JOB_CHANNEL -DFEAT_IPV6
 ifeq ($(shell expr "$$(($(WINVER)))" \>= "$$((0x600))"),1)
DEFINES += -DHAVE_INET_NTOP
 endif
endif

ifeq ($(TERMINAL),yes)
DEFINES += -DFEAT_TERMINAL
TERM_DEPS = \
	libvterm/include/vterm.h \
	libvterm/include/vterm_keycodes.h \
	libvterm/src/rect.h \
	libvterm/src/utf8.h \
	libvterm/src/vterm_internal.h
endif

ifeq ($(SOUND),yes)
DEFINES += -DFEAT_SOUND
endif

# DirectWrite (DirectX)
ifeq ($(DIRECTX),yes)
# Only allow DirectWrite for a GUI build.
 ifeq (yes, $(GUI))
DEFINES += -DFEAT_DIRECTX -DDYNAMIC_DIRECTX
  ifneq ($(COLOR_EMOJI),no)
DEFINES += -DFEAT_DIRECTX_COLOR_EMOJI
  endif
 endif
endif

ifdef SODIUM
DEFINES += -DHAVE_SODIUM
 ifeq ($(SODIUM),yes)
SODIUM_DLL = libsodium-23.dll
 else
SODIUM_DLL = libsodium.dll
CFLAGS += -I $(SODIUM)/include
 endif
 ifndef DYNAMIC_SODIUM
DYNAMIC_SODIUM=yes
 endif
 ifeq ($(DYNAMIC_SODIUM),yes)
DEFINES += -DDYNAMIC_SODIUM -DDYNAMIC_SODIUM_DLL=\"$(SODIUM_DLL)\"
 else
SODIUMLIB = -lsodium
 endif
endif

# Only allow XPM for a GUI build.
ifeq (yes, $(GUI))

 ifndef XPM
  ifeq ($(ARCH),i386)
XPM = xpm/x86
  endif
  ifeq ($(ARCH),i486)
XPM = xpm/x86
  endif
  ifeq ($(ARCH),i586)
XPM = xpm/x86
  endif
  ifeq ($(ARCH),i686)
XPM = xpm/x86
  endif
  ifeq ($(ARCH),x86-64)
XPM = xpm/x64
  endif
 endif
 ifdef XPM
  ifneq ($(XPM),no)
CFLAGS += -DFEAT_XPM_W32 -I $(XPM)/include -I $(XPM)/../include
  endif
 endif

endif

ifeq ($(DEBUG),yes)
CFLAGS += -g -fstack-check
DEBUG_SUFFIX=d
else
 ifeq ($(OPTIMIZE), SIZE)
CFLAGS += -Os
 else ifeq ($(OPTIMIZE), MAXSPEED)
CFLAGS += -O3
CFLAGS += -fomit-frame-pointer
  ifeq ($(findstring clang,$(CC)),)
# Only GCC supports the "reg-struct-return" option. Clang doesn't support this.
CFLAGS += -freg-struct-return
  endif
 else  # SPEED
CFLAGS += -O2
 endif
LFLAGS += -s
endif

ifeq ($(COVERAGE),yes)
CFLAGS += --coverage
LFLAGS += --coverage
endif

# If the ASAN=yes argument is supplied, then compile Vim with the address
# sanitizer (asan).  Only supported by MingW64 clang compiler.
# May make Vim twice as slow.  Errors are reported on stderr.
# More at: https://code.google.com/p/address-sanitizer/
# Useful environment variable:
#     set ASAN_OPTIONS=print_stacktrace=1 log_path=asan
ifeq ($(ASAN),yes)
#CFLAGS += -g -O0  -fsanitize-recover=all -fsanitize=address -fsanitize=undefined -fno-omit-frame-pointer
CFLAGS += -g -O0  -fsanitize-recover=all -fsanitize=address -fno-omit-frame-pointer
endif

LIB = -lkernel32 -luser32 -lgdi32 -ladvapi32 -lcomdlg32 -lcomctl32 -lnetapi32 -lversion
GUIOBJ =  $(OUTDIR)/gui.o $(OUTDIR)/gui_w32.o $(OUTDIR)/gui_beval.o
CUIOBJ = $(OUTDIR)/iscygpty.o
OBJ = \
	$(OUTDIR)/alloc.o \
	$(OUTDIR)/arabic.o \
	$(OUTDIR)/arglist.o \
	$(OUTDIR)/autocmd.o \
	$(OUTDIR)/beval.o \
	$(OUTDIR)/blob.o \
	$(OUTDIR)/blowfish.o \
	$(OUTDIR)/buffer.o \
	$(OUTDIR)/bufwrite.o \
	$(OUTDIR)/change.o \
	$(OUTDIR)/charset.o \
	$(OUTDIR)/cindent.o \
	$(OUTDIR)/clientserver.o \
	$(OUTDIR)/clipboard.o \
	$(OUTDIR)/cmdexpand.o \
	$(OUTDIR)/cmdhist.o \
	$(OUTDIR)/crypt.o \
	$(OUTDIR)/crypt_zip.o \
	$(OUTDIR)/debugger.o \
	$(OUTDIR)/dict.o \
	$(OUTDIR)/diff.o \
	$(OUTDIR)/digraph.o \
	$(OUTDIR)/drawline.o \
	$(OUTDIR)/drawscreen.o \
	$(OUTDIR)/edit.o \
	$(OUTDIR)/eval.o \
	$(OUTDIR)/evalbuffer.o \
	$(OUTDIR)/evalfunc.o \
	$(OUTDIR)/evalvars.o \
	$(OUTDIR)/evalwindow.o \
	$(OUTDIR)/ex_cmds.o \
	$(OUTDIR)/ex_cmds2.o \
	$(OUTDIR)/ex_docmd.o \
	$(OUTDIR)/ex_eval.o \
	$(OUTDIR)/ex_getln.o \
	$(OUTDIR)/fileio.o \
	$(OUTDIR)/filepath.o \
	$(OUTDIR)/findfile.o \
	$(OUTDIR)/float.o \
	$(OUTDIR)/fold.o \
	$(OUTDIR)/getchar.o \
	$(OUTDIR)/gui_xim.o \
	$(OUTDIR)/hardcopy.o \
	$(OUTDIR)/hashtab.o \
	$(OUTDIR)/help.o \
	$(OUTDIR)/highlight.o \
	$(OUTDIR)/if_cscope.o \
	$(OUTDIR)/indent.o \
	$(OUTDIR)/insexpand.o \
	$(OUTDIR)/json.o \
	$(OUTDIR)/list.o \
	$(OUTDIR)/locale.o \
	$(OUTDIR)/logfile.o \
	$(OUTDIR)/main.o \
	$(OUTDIR)/map.o \
	$(OUTDIR)/mark.o \
	$(OUTDIR)/match.o \
	$(OUTDIR)/memfile.o \
	$(OUTDIR)/memline.o \
	$(OUTDIR)/menu.o \
	$(OUTDIR)/message.o \
	$(OUTDIR)/misc1.o \
	$(OUTDIR)/misc2.o \
	$(OUTDIR)/mouse.o \
	$(OUTDIR)/move.o \
	$(OUTDIR)/mbyte.o \
	$(OUTDIR)/normal.o \
	$(OUTDIR)/ops.o \
	$(OUTDIR)/option.o \
	$(OUTDIR)/optionstr.o \
	$(OUTDIR)/os_mswin.o \
	$(OUTDIR)/os_win32.o \
	$(OUTDIR)/pathdef.o \
	$(OUTDIR)/popupmenu.o \
	$(OUTDIR)/popupwin.o \
	$(OUTDIR)/profiler.o \
	$(OUTDIR)/quickfix.o \
	$(OUTDIR)/regexp.o \
	$(OUTDIR)/register.o \
	$(OUTDIR)/scriptfile.o \
	$(OUTDIR)/screen.o \
	$(OUTDIR)/search.o \
	$(OUTDIR)/session.o \
	$(OUTDIR)/sha256.o \
	$(OUTDIR)/sign.o \
	$(OUTDIR)/spell.o \
	$(OUTDIR)/spellfile.o \
	$(OUTDIR)/spellsuggest.o \
	$(OUTDIR)/strings.o \
	$(OUTDIR)/syntax.o \
	$(OUTDIR)/tag.o \
	$(OUTDIR)/term.o \
	$(OUTDIR)/testing.o \
	$(OUTDIR)/textformat.o \
	$(OUTDIR)/textobject.o \
	$(OUTDIR)/textprop.o \
	$(OUTDIR)/time.o \
	$(OUTDIR)/typval.o \
	$(OUTDIR)/ui.o \
	$(OUTDIR)/undo.o \
	$(OUTDIR)/usercmd.o \
	$(OUTDIR)/userfunc.o \
	$(OUTDIR)/version.o \
	$(OUTDIR)/vim9class.o \
	$(OUTDIR)/vim9cmds.o \
	$(OUTDIR)/vim9compile.o \
	$(OUTDIR)/vim9execute.o \
	$(OUTDIR)/vim9expr.o \
	$(OUTDIR)/vim9instr.o \
	$(OUTDIR)/vim9script.o \
	$(OUTDIR)/vim9type.o \
	$(OUTDIR)/viminfo.o \
	$(OUTDIR)/winclip.o \
	$(OUTDIR)/window.o

ifeq ($(VIMDLL),yes)
OBJ += $(OUTDIR)/os_w32dll.o $(OUTDIR)/vimresd.o
EXEOBJC = $(OUTDIR)/os_w32exec.o $(OUTDIR)/vimresc.o
EXEOBJG = $(OUTDIR)/os_w32exeg.o $(OUTDIR)/vimresg.o
else
OBJ += $(OUTDIR)/os_w32exe.o $(OUTDIR)/vimres.o
endif

ifdef PERL
OBJ += $(OUTDIR)/if_perl.o
endif
ifdef LUA
OBJ += $(OUTDIR)/if_lua.o
endif
ifdef MZSCHEME
OBJ += $(OUTDIR)/if_mzsch.o
MZSCHEME_INCL = if_mzsch.h
 ifeq (yes,$(MZSCHEME_GENERATE_BASE))
CFLAGS += -DINCLUDE_MZSCHEME_BASE
MZ_EXTRA_DEP += mzscheme_base.c
 endif
 ifeq (yes,$(MZSCHEME_PRECISE_GC))
CFLAGS += -DMZ_PRECISE_GC
 endif
endif
ifdef PYTHON
OBJ += $(OUTDIR)/if_python.o
endif
ifdef PYTHON3
OBJ += $(OUTDIR)/if_python3.o
endif
ifdef RUBY
OBJ += $(OUTDIR)/if_ruby.o
endif
ifdef TCL
OBJ += $(OUTDIR)/if_tcl.o
endif

ifeq ($(NETBEANS),yes)
 ifneq ($(CHANNEL),yes)
# Cannot use Netbeans without CHANNEL
NETBEANS=no
 else ifneq (yes, $(GUI))
# Cannot use Netbeans without GUI.
NETBEANS=no
 else
OBJ += $(OUTDIR)/netbeans.o
 endif
endif

ifeq ($(CHANNEL),yes)
OBJ += $(OUTDIR)/job.o $(OUTDIR)/channel.o
LIB += -lws2_32
endif

ifeq ($(DIRECTX),yes)
# Only allow DIRECTX for a GUI build.
 ifeq (yes, $(GUI))
OBJ += $(OUTDIR)/gui_dwrite.o
LIB += -ld2d1 -ldwrite
USE_STDCPLUS = yes
 endif
endif
ifneq ($(XPM),no)
# Only allow XPM for a GUI build.
 ifeq (yes, $(GUI))
OBJ += $(OUTDIR)/xpm_w32.o
# You'll need libXpm.a from http://gnuwin32.sf.net
LIB += -L$(XPM)/lib -lXpm
 endif
endif

ifeq ($(TERMINAL),yes)
OBJ += $(OUTDIR)/terminal.o \
	$(OUTDIR)/vterm_encoding.o \
	$(OUTDIR)/vterm_keyboard.o \
	$(OUTDIR)/vterm_mouse.o \
	$(OUTDIR)/vterm_parser.o \
	$(OUTDIR)/vterm_pen.o \
	$(OUTDIR)/vterm_screen.o \
	$(OUTDIR)/vterm_state.o \
	$(OUTDIR)/vterm_unicode.o \
	$(OUTDIR)/vterm_vterm.o
endif

ifeq ($(SOUND),yes)
OBJ += $(OUTDIR)/sound.o
endif

# Include xdiff
OBJ +=  $(OUTDIR)/xdiffi.o \
	$(OUTDIR)/xemit.o \
	$(OUTDIR)/xprepare.o \
	$(OUTDIR)/xutils.o \
	$(OUTDIR)/xhistogram.o \
	$(OUTDIR)/xpatience.o

XDIFF_DEPS = \
	xdiff/xdiff.h \
	xdiff/xdiffi.h \
	xdiff/xemit.h \
	xdiff/xinclude.h \
	xdiff/xmacros.h \
	xdiff/xprepare.h \
	xdiff/xtypes.h \
	xdiff/xutils.h

ifdef MZSCHEME
MZSCHEME_SUFFIX = Z
endif

LFLAGS += -municode

ifeq ($(VIMDLL),yes)
VIMEXE := vim$(DEBUG_SUFFIX).exe
GVIMEXE := gvim$(DEBUG_SUFFIX).exe
 ifeq ($(ARCH),x86-64)
VIMDLLBASE := vim64$(DEBUG_SUFFIX)
 else
VIMDLLBASE := vim32$(DEBUG_SUFFIX)
 endif
TARGET = $(VIMDLLBASE).dll
LFLAGS += -shared
EXELFLAGS += -municode
 ifneq ($(DEBUG),yes)
EXELFLAGS += -s
 endif
 ifeq ($(COVERAGE),yes)
EXELFLAGS += --coverage
 endif
DEFINES += $(DEF_GUI) -DVIMDLL
OBJ += $(GUIOBJ) $(CUIOBJ)
OUTDIR = dobj$(DEBUG_SUFFIX)$(MZSCHEME_SUFFIX)$(ARCH)
MAIN_TARGET = $(GVIMEXE) $(VIMEXE) $(VIMDLLBASE).dll
else ifeq ($(GUI),yes)
TARGET := gvim$(DEBUG_SUFFIX).exe
DEFINES += $(DEF_GUI)
OBJ += $(GUIOBJ)
LFLAGS += -mwindows
OUTDIR = gobj$(DEBUG_SUFFIX)$(MZSCHEME_SUFFIX)$(ARCH)
MAIN_TARGET = $(TARGET)
else
OBJ += $(CUIOBJ)
TARGET := vim$(DEBUG_SUFFIX).exe
OUTDIR = obj$(DEBUG_SUFFIX)$(MZSCHEME_SUFFIX)$(ARCH)
MAIN_TARGET = $(TARGET)
endif

ifdef GETTEXT
 ifneq (yes, $(GETTEXT))
CFLAGS += -I$(GETTEXTINCLUDE)
  ifndef STATIC_GETTEXT
LIB += -L$(GETTEXTLIB) -l$(INTLLIB)
   ifeq (USE_SAFE_GETTEXT_DLL, $(DYNAMIC_GETTEXT))
OBJ+=$(SAFE_GETTEXT_DLL_OBJ)
   endif
  else
LIB += -L$(GETTEXTLIB) -lintl
  endif
 endif
endif

ifdef PERL
 ifeq (no, $(DYNAMIC_PERL))
LIB += -L$(PERLLIBS) -lperl$(PERL_VER)
 endif
endif

ifdef TCL
LIB += -L$(TCL)/lib
 ifeq (yes, $(DYNAMIC_TCL))
LIB += -ltclstub$(TCL_VER)
 else
LIB += -ltcl$(TCL_VER)
 endif
endif

ifeq (yes, $(OLE))
LIB += -loleaut32
OBJ += $(OUTDIR)/if_ole.o
USE_STDCPLUS = yes
endif

ifeq (yes, $(IME))
DEFINES += -DFEAT_MBYTE_IME
 ifeq (yes, $(DYNAMIC_IME))
DEFINES += -DDYNAMIC_IME
 else
LIB += -limm32
 endif
endif

ifdef ICONV
 ifneq (yes, $(ICONV))
LIB += -L$(ICONV)
CFLAGS += -I$(ICONV)
 endif
DEFINES+=-DDYNAMIC_ICONV
endif

ifeq (yes, $(SOUND))
LIB += -lwinmm
endif

ifeq (yes, $(USE_STDCPLUS))
LINK = $(CXX)
 ifeq (yes, $(STATIC_STDCPLUS))
#LIB += -static-libstdc++ -static-libgcc
LIB += -Wl,-Bstatic -lstdc++ -lgcc -Wl,-Bdynamic
 endif
else
LINK = $(CC)
endif

ifeq (yes, $(STATIC_WINPTHREAD))
 ifeq (yes, $(HAS_GCC_EH))
LIB += -lgcc_eh
 endif
LIB += -Wl,-Bstatic -lwinpthread -Wl,-Bdynamic
endif

ifeq (yes, $(MAP))
LFLAGS += -Wl,-Map=$(TARGET).map
endif

# The default stack size on Windows is 2 MB.  With the default stack size, the
# following tests fail with the clang address sanitizer:
#   Test_listdict_compare, Test_listdict_compare_complex, Test_deep_recursion,
#   Test_map_error, Test_recursive_define, Test_recursive_addstate
# To increase the stack size to 16MB, uncomment the following line:
#LFLAGS += -Wl,-stack -Wl,0x1000000

all: $(MAIN_TARGET) vimrun.exe xxd/xxd.exe tee/tee.exe install.exe uninstall.exe GvimExt/gvimext.dll

vimrun.exe: vimrun.c
	$(CC) $(CFLAGS) -o vimrun.exe vimrun.c $(LIB)

install.exe: dosinst.c dosinst.h version.h
	$(CC) $(CFLAGS) -o install.exe dosinst.c $(LIB) -lole32 -luuid

uninstall.exe: uninstall.c dosinst.h version.h
	$(CC) $(CFLAGS) -o uninstall.exe uninstall.c $(LIB) -lole32

$(OBJ): | $(OUTDIR)

$(EXEOBJG): | $(OUTDIR)

$(EXEOBJC): | $(OUTDIR)

ifeq ($(VIMDLL),yes)
$(TARGET): $(OBJ)
	$(LINK) $(CFLAGS) $(LFLAGS) -o $@ $(OBJ) $(LIB) -lole32 -luuid -lgdi32 $(LUA_LIB) $(MZSCHEME_LIBDIR) $(MZSCHEME_LIB) $(PYTHONLIB) $(PYTHON3LIB) $(RUBYLIB) $(SODIUMLIB)

$(GVIMEXE): $(EXEOBJG) $(VIMDLLBASE).dll
	$(CC) -L. $(EXELFLAGS) -mwindows -o $@ $(EXEOBJG) -l$(VIMDLLBASE)

$(VIMEXE): $(EXEOBJC) $(VIMDLLBASE).dll
	$(CC) -L. $(EXELFLAGS) -o $@ $(EXEOBJC) -l$(VIMDLLBASE)
else
$(TARGET): $(OBJ)
	$(LINK) $(CFLAGS) $(LFLAGS) -o $@ $(OBJ) $(LIB) -lole32 -luuid $(LUA_LIB) $(MZSCHEME_LIBDIR) $(MZSCHEME_LIB) $(PYTHONLIB) $(PYTHON3LIB) $(RUBYLIB) $(SODIUMLIB)
endif

upx: exes
	upx gvim.exe
	upx vim.exe

mpress: exes
	mpress gvim.exe
	mpress vim.exe

xxd/xxd.exe: xxd/xxd.c
	$(MAKE) -C xxd -f Make_ming.mak CC='$(CC)'

tee/tee.exe: tee/tee.c
	$(MAKE) -C tee -f Make_ming.mak CC='$(CC)'

GvimExt/gvimext.dll: GvimExt/gvimext.cpp GvimExt/gvimext.rc GvimExt/gvimext.h
	$(MAKE) -C GvimExt -f Make_ming.mak CROSS=$(CROSS) CROSS_COMPILE=$(CROSS_COMPILE) CXX='$(CXX)' STATIC_STDCPLUS=$(STATIC_STDCPLUS)

tags: notags
	$(CTAGS) $(TAGS_FILES)

notags:
	-$(DEL) tags

clean:
	-$(DEL) $(OUTDIR)$(DIRSLASH)*.o
	-$(DEL) $(OUTDIR)$(DIRSLASH)*.res
	-$(DEL) $(OUTDIR)$(DIRSLASH)pathdef.c
	-rmdir $(OUTDIR)
	-$(DEL) $(MAIN_TARGET) vimrun.exe install.exe uninstall.exe
	-$(DEL) *.map
ifdef PERL
	-$(DEL) if_perl.c
	-$(DEL) auto$(DIRSLASH)if_perl.c
endif
ifdef MZSCHEME
	-$(DEL) mzscheme_base.c
endif
	$(MAKE) -C GvimExt -f Make_ming.mak clean
	$(MAKE) -C xxd -f Make_ming.mak clean
	$(MAKE) -C tee -f Make_ming.mak clean

# Run vim script to generate the Ex command lookup table.
# This only needs to be run when a command name has been added or changed.
# If this fails because you don't have Vim yet, first build and install Vim
# without changes.
cmdidxs: ex_cmds.h
	vim --clean -N -X --not-a-term -u create_cmdidxs.vim -c quit

# Run vim script to generate the normal/visual mode command lookup table.
# This only needs to be run when a new normal/visual mode command has been
# added.  If this fails because you don't have Vim yet:
#   - change nv_cmds[] in nv_cmds.h to add the new normal/visual mode command.
#   - run "make nvcmdidxs" to generate nv_cmdidxs.h
nvcmdidxs: nv_cmds.h
	$(CC) $(CFLAGS) -o create_nvcmdidxs.exe create_nvcmdidxs.c $(LIB)
	vim --clean -N -X --not-a-term -u create_nvcmdidxs.vim -c quit
	-$(DEL) create_nvcmdidxs.exe

###########################################################################
INCL =	vim.h alloc.h ascii.h ex_cmds.h feature.h errors.h globals.h \
	keymap.h macros.h option.h os_dos.h os_win32.h proto.h regexp.h \
	spell.h structs.h termdefs.h beval.h $(NBDEBUG_INCL)
GUI_INCL = gui.h
ifeq ($(DIRECTX),yes)
GUI_INCL += gui_dwrite.h
endif
CUI_INCL = iscygpty.h

PATHDEF_SRC = $(OUTDIR)/pathdef.c

$(OUTDIR)/if_python.o:	if_python.c if_py_both.h $(INCL)
	$(CC) -c $(CFLAGS) $(PYTHONINC) $(PYTHON_HOME_DEF) $< -o $@

$(OUTDIR)/if_python3.o:	if_python3.c if_py_both.h $(INCL)
	$(CC) -c $(CFLAGS) $(PYTHON3INC) $(PYTHON3_HOME_DEF) $< -o $@

$(OUTDIR)/%.o : %.c $(INCL)
	$(CC) -c $(CFLAGS) $< -o $@

ifeq ($(VIMDLL),yes)
$(OUTDIR)/vimresc.o:	vim.rc vim.manifest version.h gui_w32_rc.h vim.ico
	$(WINDRES) $(WINDRES_FLAGS) $(DEFINES) -UFEAT_GUI_MSWIN \
	    --input-format=rc --output-format=coff -i vim.rc -o $@

$(OUTDIR)/vimresg.o:	vim.rc vim.manifest version.h gui_w32_rc.h vim.ico
	$(WINDRES) $(WINDRES_FLAGS) $(DEFINES) \
	    --input-format=rc --output-format=coff -i vim.rc -o $@

$(OUTDIR)/vimresd.o:	vim.rc version.h gui_w32_rc.h \
			tools.bmp tearoff.bmp vim.ico vim_error.ico \
			vim_alert.ico vim_info.ico vim_quest.ico
	$(WINDRES) $(WINDRES_FLAGS) $(DEFINES) -DRCDLL -DVIMDLLBASE=\\\"$(VIMDLLBASE)\\\" \
	    --input-format=rc --output-format=coff -i vim.rc -o $@
else
$(OUTDIR)/vimres.o:	vim.rc vim.manifest version.h gui_w32_rc.h \
			tools.bmp tearoff.bmp vim.ico vim_error.ico \
			vim_alert.ico vim_info.ico vim_quest.ico
	$(WINDRES) $(WINDRES_FLAGS) $(DEFINES) \
	    --input-format=rc --output-format=coff -i vim.rc -o $@
endif

$(OUTDIR):
	$(MKDIR) $(OUTDIR)

$(OUTDIR)/buffer.o: buffer.c $(INCL) version.h

$(OUTDIR)/evalfunc.o: evalfunc.c $(INCL) version.h

$(OUTDIR)/evalvars.o: evalvars.c $(INCL) version.h

$(OUTDIR)/ex_cmds.o: ex_cmds.c $(INCL) version.h

$(OUTDIR)/ex_cmds2.o: ex_cmds2.c $(INCL) version.h

$(OUTDIR)/ex_docmd.o: ex_docmd.c $(INCL) ex_cmdidxs.h

$(OUTDIR)/hardcopy.o: hardcopy.c $(INCL) version.h

$(OUTDIR)/misc1.o: misc1.c $(INCL) version.h

$(OUTDIR)/normal.o: normal.c $(INCL) nv_cmdidxs.h nv_cmds.h

$(OUTDIR)/netbeans.o: netbeans.c $(INCL) version.h

$(OUTDIR)/version.o: version.c $(INCL) version.h

$(OUTDIR)/vim9class.o: vim9class.c $(INCL) vim9.h

$(OUTDIR)/vim9cmds.o: vim9cmds.c $(INCL) vim9.h

$(OUTDIR)/vim9compile.o: vim9compile.c $(INCL) vim9.h

$(OUTDIR)/vim9execute.o: vim9execute.c $(INCL) vim9.h

$(OUTDIR)/vim9expr.o: vim9expr.c $(INCL) vim9.h

$(OUTDIR)/vim9instr.o: vim9instr.c $(INCL) vim9.h

$(OUTDIR)/vim9script.o: vim9script.c $(INCL) vim9.h

$(OUTDIR)/vim9type.o: vim9type.c $(INCL) vim9.h

$(OUTDIR)/viminfo.o: viminfo.c $(INCL) version.h

$(OUTDIR)/gui_dwrite.o:	gui_dwrite.cpp gui_dwrite.h
	$(CC) -c $(CFLAGS) $(CXXFLAGS) gui_dwrite.cpp -o $@

$(OUTDIR)/gui.o:	gui.c $(INCL) $(GUI_INCL)
	$(CC) -c $(CFLAGS) gui.c -o $@

$(OUTDIR)/beval.o:	beval.c $(INCL) $(GUI_INCL)
	$(CC) -c $(CFLAGS) beval.c -o $@

$(OUTDIR)/gui_beval.o:	gui_beval.c $(INCL) $(GUI_INCL)
	$(CC) -c $(CFLAGS) gui_beval.c -o $@

$(OUTDIR)/gui_w32.o:	gui_w32.c $(INCL) $(GUI_INCL) version.h
	$(CC) -c $(CFLAGS) gui_w32.c -o $@

$(OUTDIR)/if_cscope.o:	if_cscope.c $(INCL)
	$(CC) -c $(CFLAGS) if_cscope.c -o $@

$(OUTDIR)/if_mzsch.o:	if_mzsch.c $(INCL) $(MZSCHEME_INCL) $(MZ_EXTRA_DEP)
	$(CC) -c $(CFLAGS) if_mzsch.c -o $@

mzscheme_base.c:
	$(MZSCHEME)/mzc --c-mods mzscheme_base.c ++lib scheme/base

# Remove -D__IID_DEFINED__ for newer versions of the w32api
$(OUTDIR)/if_ole.o:	if_ole.cpp $(INCL) if_ole.h
	$(CC) -c $(CFLAGS) $(CXXFLAGS) if_ole.cpp -o $@

auto/if_perl.c:		if_perl.xs typemap
	$(XSUBPP) -prototypes -typemap \
	     $(PERLTYPEMAP) if_perl.xs -output $@

$(OUTDIR)/if_perl.o:	auto/if_perl.c $(INCL)
	$(CC) -c $(CFLAGS) auto/if_perl.c -o $@


$(OUTDIR)/if_ruby.o:	if_ruby.c $(INCL) version.h
ifeq (16, $(RUBY))
	$(CC) $(CFLAGS) -U_WIN32 -c -o $@ if_ruby.c
endif

$(OUTDIR)/iscygpty.o:	iscygpty.c $(CUI_INCL)
	$(CC) -c $(CFLAGS) iscygpty.c -o $@

$(OUTDIR)/main.o:	main.c $(INCL) $(CUI_INCL)
	$(CC) -c $(CFLAGS) main.c -o $@

$(OUTDIR)/netbeans.o:	netbeans.c $(INCL) $(NBDEBUG_INCL) $(NBDEBUG_SRC)
	$(CC) -c $(CFLAGS) netbeans.c -o $@

$(OUTDIR)/os_w32exec.o:	os_w32exe.c $(INCL)
	$(CC) -c $(CFLAGS) -UFEAT_GUI_MSWIN os_w32exe.c -o $@

$(OUTDIR)/os_w32exeg.o:	os_w32exe.c $(INCL)
	$(CC) -c $(CFLAGS) os_w32exe.c -o $@

$(OUTDIR)/os_win32.o:	os_win32.c $(INCL) $(MZSCHEME_INCL)
	$(CC) -c $(CFLAGS) os_win32.c -o $@

$(OUTDIR)/regexp.o:	regexp.c regexp_bt.c regexp_nfa.c $(INCL)
	$(CC) -c $(CFLAGS) regexp.c -o $@

$(OUTDIR)/register.o:	register.c $(INCL)
	$(CC) -c $(CFLAGS) register.c -o $@

$(OUTDIR)/terminal.o:	terminal.c $(INCL) $(TERM_DEPS)
	$(CC) -c $(CFLAGS) terminal.c -o $@

$(OUTDIR)/pathdef.o:	$(PATHDEF_SRC) $(INCL)
	$(CC) -c $(CFLAGS) $(PATHDEF_SRC) -o $@


CCCTERM = $(CC) -c $(CFLAGS) -Ilibvterm/include -DINLINE="" \
	  -DVSNPRINTF=vim_vsnprintf \
	  -DSNPRINTF=vim_snprintf \
	  -DIS_COMBINING_FUNCTION=utf_iscomposing_uint \
	  -DWCWIDTH_FUNCTION=utf_uint2cells \
	  -DGET_SPECIAL_PTY_TYPE_FUNCTION=get_special_pty_type

$(OUTDIR)/vterm_%.o : libvterm/src/%.c $(TERM_DEPS)
	$(CCCTERM) $< -o $@


$(OUTDIR)/%.o : xdiff/%.c $(XDIFF_DEPS)
	$(CC) -c $(CFLAGS) $< -o $@


$(PATHDEF_SRC): Make_cyg_ming.mak Make_cyg.mak Make_ming.mak | $(OUTDIR)
ifneq (sh.exe, $(SHELL))
	@echo creating $(PATHDEF_SRC)
	@echo '/* pathdef.c */' > $(PATHDEF_SRC)
	@echo '#include "vim.h"' >> $(PATHDEF_SRC)
	@echo 'char_u *default_vim_dir = (char_u *)"$(VIMRCLOC)";' >> $(PATHDEF_SRC)
	@echo 'char_u *default_vimruntime_dir = (char_u *)"$(VIMRUNTIMEDIR)";' >> $(PATHDEF_SRC)
	@echo 'char_u *all_cflags = (char_u *)"$(CC) $(CFLAGS)";' >> $(PATHDEF_SRC)
	@echo 'char_u *all_lflags = (char_u *)"$(LINK) $(CFLAGS) $(LFLAGS) -o $(TARGET) $(LIB) -lole32 -luuid $(LUA_LIB) $(MZSCHEME_LIBDIR) $(MZSCHEME_LIB) $(PYTHONLIB) $(PYTHON3LIB) $(RUBYLIB)";' >> $(PATHDEF_SRC)
	@echo 'char_u *compiled_user = (char_u *)"$(USERNAME)";' >> $(PATHDEF_SRC)
	@echo 'char_u *compiled_sys = (char_u *)"$(USERDOMAIN)";' >> $(PATHDEF_SRC)
else
	@echo creating $(PATHDEF_SRC)
	@echo /* pathdef.c */ > $(PATHDEF_SRC)
	@echo #include "vim.h" >> $(PATHDEF_SRC)
	@echo char_u *default_vim_dir = (char_u *)"$(VIMRCLOC)"; >> $(PATHDEF_SRC)
	@echo char_u *default_vimruntime_dir = (char_u *)"$(VIMRUNTIMEDIR)"; >> $(PATHDEF_SRC)
	@echo char_u *all_cflags = (char_u *)"$(CC) $(CFLAGS)"; >> $(PATHDEF_SRC)
	@echo char_u *all_lflags = (char_u *)"$(CC) $(CFLAGS) $(LFLAGS) -o $(TARGET) $(LIB) -lole32 -luuid $(LUA_LIB) $(MZSCHEME_LIBDIR) $(MZSCHEME_LIB) $(PYTHONLIB) $(PYTHON3LIB) $(RUBYLIB)"; >> $(PATHDEF_SRC)
	@echo char_u *compiled_user = (char_u *)"$(USERNAME)"; >> $(PATHDEF_SRC)
	@echo char_u *compiled_sys = (char_u *)"$(USERDOMAIN)"; >> $(PATHDEF_SRC)
endif

# vim: set noet sw=8 ts=8 sts=0 wm=0 tw=0:
