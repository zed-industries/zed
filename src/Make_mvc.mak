# Makefile for Vim on Win32 (Windows 7/8/10/11) and Win64, using the Microsoft
# Visual C++ compilers. Known to work with VC14 (VS2015), VC14.1 (VS2017),
# VC14.2 (VS2019) and VC14.3 (VS2022).
#
# To build using other Windows compilers, see INSTALLpc.txt
#
# This makefile can build the console, GUI, OLE-enable, Perl-enabled and
# Python-enabled versions of Vim for Win32 platforms.
#
# The basic command line to build Vim is:
#
#	nmake -f Make_mvc.mak
#
# This will build the console version of Vim with no additional interfaces.
# To add features, define any of the following:
#
#	!!!!  After changing any features do "nmake clean" first  !!!!
#
#	Feature Set: FEATURES=[TINY, NORMAL, HUGE] (default is HUGE)
#
#   	Name to add to the version: MODIFIED_BY=[name of modifier]
#
#	GUI interface: GUI=yes (default is no)
#
#	GUI with DirectWrite (DirectX): DIRECTX=yes
#	  (default is yes if GUI=yes, requires GUI=yes)
#
#	Color emoji support: COLOR_EMOJI=yes
#	  (default is yes if DIRECTX=yes, requires WinSDK 8.1 or later.)
#
#	OLE interface: OLE=yes (usually with GUI=yes)
#
#	IME support: IME=yes	(default is yes)
#	  DYNAMIC_IME=[yes or no]  (to load the imm32.dll dynamically, default
#	  is yes)
#
#	Terminal support: TERMINAL=yes (default is yes if FEATURES is HUGE)
#	  Will also enable CHANNEL
#
#	Sound support: SOUND=yes (default is yes)
#
#	Sodium support: SODIUM=[Path to Sodium directory]
#	  DYNAMIC_SODIUM=yes (to load the Sodium DLL dynamically)
#	  You need to install the msvc package from
#	  https://download.libsodium.org/libsodium/releases/
#	  and package the libsodium.dll with Vim
#
#
#	DLL support (EXPERIMENTAL): VIMDLL=yes (default is no)
#	  Creates vim{32,64}.dll, and stub gvim.exe and vim.exe.
#	  The shared codes between the GUI and the console are built into
#	  the DLL.  This reduces the total file size and memory usage.
#	  Also supports `vim -g` and the `:gui` command.
#
#	Lua interface:
#	  LUA=[Path to Lua directory]
#	  DYNAMIC_LUA=yes (to load the Lua DLL dynamically)
#	  LUA_VER=[Lua version]  (default is 53)
#
#	MzScheme interface:
#	  MZSCHEME=[Path to MzScheme directory]
#	  DYNAMIC_MZSCHEME=yes (to load the MzScheme DLLs dynamically)
#	  MZSCHEME_VER=[MzScheme version] (default is 3m_a0solc (6.6))
#	  	Used for the DLL file name. E.g.:
#	  	C:\Program Files (x86)\Racket\lib\libracket3m_XXXXXX.dll
#	  MZSCHEME_DEBUG=no
#
#	Perl interface:
#	  PERL=[Path to Perl directory]
#	  DYNAMIC_PERL=yes (to load the Perl DLL dynamically)
#	  PERL_VER=[Perl version, in the form 55 (5.005), 56 (5.6.x),
#		    510 (5.10.x), etc]
#	  (default is 524)
#
#	Python interface:
#	  PYTHON=[Path to Python directory]
#	  DYNAMIC_PYTHON=yes (to load the Python DLL dynamically)
#	  PYTHON_VER=[Python version, eg 22, 23, ..., 27]  (default is 27)
#
#	Python3 interface:
#	  PYTHON3=[Path to Python3 directory]
#	  DYNAMIC_PYTHON3=yes (to load the Python3 DLL dynamically)
#	  PYTHON3_VER=[Python3 version, eg 30, 31]  (default is 36)
#
#	Ruby interface:
#	  RUBY=[Path to Ruby directory]
#	  DYNAMIC_RUBY=yes (to load the Ruby DLL dynamically)
#	  RUBY_VER=[Ruby version, eg 19, 22] (default is 22)
#	  RUBY_API_VER_LONG=[Ruby API version, eg 1.9.1, 2.2.0]
#	  		    (default is 2.2.0)
#	    You must set RUBY_API_VER_LONG when change RUBY_VER.
#	    Note: If you use Ruby 1.9.3, set as follows:
#	      RUBY_VER=19
#	      RUBY_API_VER_LONG=1.9.1 (not 1.9.3, because the API version is 1.9.1.)
#
#	Tcl interface:
#	  TCL=[Path to Tcl directory]
#	  DYNAMIC_TCL=yes (to load the Tcl DLL dynamically)
#	  TCL_VER=[Tcl version, e.g. 80, 83]  (default is 86)
#	  TCL_VER_LONG=[Tcl version, eg 8.3] (default is 8.6)
#	    You must set TCL_VER_LONG when you set TCL_VER.
#	  TCL_DLL=[Tcl dll name, e.g. tcl86.dll]  (default is tcl86.dll)
#
#	Cscope support: CSCOPE=yes
#
#	Iconv library support (always dynamically loaded):
#	  ICONV=[yes or no]  (default is yes)
#
#	Intl library support (always dynamically loaded):
#	  GETTEXT=[yes or no]  (default is yes)
#	See http://sourceforge.net/projects/gettext/
#
#	PostScript printing: POSTSCRIPT=yes (default is no)
#
#	Netbeans Support: NETBEANS=[yes or no] (default is yes if GUI is yes)
#	  Requires CHANNEL.
#
#	Netbeans Debugging Support: NBDEBUG=[yes or no] (should be no, yes
#	doesn't work)
#
#	Inter process communication: CHANNEL=[yes or no] (default is yes if GUI
#	is yes or TERMINAL is yes)
#
#	XPM Image Support: XPM=[path to XPM directory]
#	Default is "xpm", using the files included in the distribution.
#	Use "no" to disable this feature.
#
#	Optimization: OPTIMIZE=[SPACE, SPEED, MAXSPEED] (default is MAXSPEED)
#
#	Processor Version: CPUNR=[any, i686, sse, sse2, avx, avx2] (default is
#	sse2)
#	  avx is available on Visual C++ 2010 and after.
#	  avx2 is available on Visual C++ 2013 Update 2 and after.
#
#	Version Support: WINVER=[0x0601, 0x0602, 0x0603, 0x0A00] (default is
#	0x0601)
#	Supported versions depends on your target SDK, check SDKDDKVer.h
#	See https://docs.microsoft.com/en-us/cpp/porting/modifying-winver-and-win32-winnt
#
#	Debug version: DEBUG=yes
#	Mapfile: MAP=[no, yes or lines] (default is yes)
#	  no:    Don't write a mapfile.
#	  yes:   Write a normal mapfile.
#	  lines: Write a mapfile with line numbers (only for VC6 and later)
#
#	Static Code Analysis: ANALYZE=yes (works with VS2012 or later)
#
#	Address Sanitizer: ASAN=yes (works with VS2019 or later)
#
# You can combine any of these interfaces
#
# Example: To build the non-debug, GUI version with Perl interface:
#	nmake -f Make_mvc.mak GUI=yes PERL=C:\Perl

### See feature.h for a list of optionals.
# If you want to build some optional features without modifying the source,
# you can set DEFINES on the command line, e.g.,
#	nmake -f Make_mvc.mvc "DEFINES=-DEMACS_TAGS"

# Build on Windows NT/XP

TARGETOS = WINNT

!if "$(VIMDLL)" == "yes"
GUI = yes
!endif

!ifndef DIRECTX
DIRECTX = $(GUI)
!endif

# Select a code directory, depends on GUI, OLE, DEBUG, interfaces and etc.
# If you change something else, do "make clean" first!
!if "$(VIMDLL)" == "yes"
OBJDIR = .\ObjD
!elseif "$(GUI)" == "yes"
OBJDIR = .\ObjG
!else
OBJDIR = .\ObjC
!endif
!if "$(DIRECTX)" == "yes" && "$(GUI)" == "yes"
OBJDIR = $(OBJDIR)X
!endif
!if "$(OLE)" == "yes"
OBJDIR = $(OBJDIR)O
!endif
!ifdef LUA
OBJDIR = $(OBJDIR)U
!endif
!ifdef PERL
OBJDIR = $(OBJDIR)L
!endif
!ifdef PYTHON
OBJDIR = $(OBJDIR)Y
!endif
!ifdef PYTHON3
OBJDIR = $(OBJDIR)H
!endif
!ifdef TCL
OBJDIR = $(OBJDIR)T
!endif
!ifdef RUBY
OBJDIR = $(OBJDIR)R
!endif
!ifdef MZSCHEME
OBJDIR = $(OBJDIR)Z
!endif
!ifdef USE_MSVCRT
OBJDIR = $(OBJDIR)V
!endif
!if "$(DEBUG)" == "yes"
OBJDIR = $(OBJDIR)d
!endif

!ifdef PROCESSOR_ARCHITECTURE
# We're on Windows NT or using VC 6+
! ifdef CPU
ASSEMBLY_ARCHITECTURE=$(CPU)
# Using I386 for $ASSEMBLY_ARCHITECTURE doesn't work for VC7.
!  if "$(CPU)" == "I386"
CPU = i386
!  endif
! else  # !CPU
CPU = i386
!  ifndef PLATFORM
!   ifdef TARGET_CPU
PLATFORM = $(TARGET_CPU)
!   elseif defined(VSCMD_ARG_TGT_ARCH)
PLATFORM = $(VSCMD_ARG_TGT_ARCH)
!   endif
!  endif
!  ifdef PLATFORM
!   if ("$(PLATFORM)" == "x64") || ("$(PLATFORM)" == "X64")
CPU = AMD64
!   elseif ("$(PLATFORM)" == "arm64") || ("$(PLATFORM)" == "ARM64")
CPU = ARM64
!   elseif ("$(PLATFORM)" != "x86") && ("$(PLATFORM)" != "X86")
!    error *** ERROR Unknown target platform "$(PLATFORM)". Make aborted.
!   endif
!  endif  # !PLATFORM
! endif
!else  # !PROCESSOR_ARCHITECTURE
# We're on Windows 95
CPU = i386
!endif # !PROCESSOR_ARCHITECTURE
ASSEMBLY_ARCHITECTURE=$(CPU)
OBJDIR = $(OBJDIR)$(CPU)

# Build a retail version by default

!if "$(DEBUG)" != "yes"
NODEBUG = 1
!else
! undef NODEBUG
MAKEFLAGS_GVIMEXT = DEBUG=yes
!endif

LINK = link

# Check VC version.
!if [echo MSVCVER=_MSC_VER> msvcver.c && $(CC) /EP msvcver.c > msvcver.~ 2> nul]
! message *** ERROR
! message Cannot run Visual C to determine its version. Make sure cl.exe is in your PATH.
! message This can usually be done by running "vcvarsall.bat", located in the bin directory where Visual Studio was installed.
! error Make aborted.
!else
! include msvcver.~
! if [del msvcver.c msvcver.~]
! endif
!endif

!if $(MSVCVER) < 1900
! message *** ERROR
! message Unsupported MSVC version.
! message Please use Visual C++ 2015 or later.
! error Make aborted.
!endif

MSVC_MAJOR = ($(MSVCVER) / 100 - 5)
MSVCRT_VER = ($(MSVCVER) / 100 * 10 - 50)

# Calculate MSVC_FULL.
!if [echo MSVC_FULL=_MSC_FULL_VER> msvcfullver.c && $(CC) /EP msvcfullver.c > msvcfullver.~ 2> nul]
! message *** ERROR
! message Cannot run Visual C to determine its version. Make sure cl.exe is in your PATH.
! message This can usually be done by running "vcvarsall.bat", located in the bin directory where Visual Studio was installed.
! error Make aborted.
!else
! include msvcfullver.~
! if [del msvcfullver.c msvcfullver.~]
! endif
!endif


# Calculate MSVCRT_VER
!if [(set /a MSVCRT_VER="$(MSVCRT_VER)" > nul) && set MSVCRT_VER > msvcrtver.~] == 0
! include msvcrtver.~
! if [del msvcrtver.~]
! endif
!endif

# Base name of the msvcrXX.dll (vcruntimeXXX.dll)
MSVCRT_NAME = vcruntime$(MSVCRT_VER)

### Set the default $(WINVER) to make it work with Windows 7
!ifndef WINVER
WINVER = 0x0601
!endif

# Use multiprocess build
USE_MP = yes

!if "$(FEATURES)"==""
FEATURES = HUGE
!endif

!ifndef CTAGS
# this assumes ctags is Exuberant ctags
CTAGS = ctags -I INIT+,INIT2+,INIT3+,INIT4+,INIT5+ --fields=+S
!endif

!ifndef CSCOPE
CSCOPE = yes
!endif

!if "$(CSCOPE)" == "yes"
# CSCOPE - Include support for Cscope
CSCOPE_DEFS  = -DFEAT_CSCOPE
!endif

!ifndef TERMINAL
! if "$(FEATURES)"=="HUGE"
TERMINAL = yes
! else
TERMINAL = no
! endif
!endif

!if "$(TERMINAL)" == "yes"
TERM_OBJ = \
	$(OBJDIR)/terminal.obj \
	$(OBJDIR)/vterm_encoding.obj \
	$(OBJDIR)/vterm_keyboard.obj \
	$(OBJDIR)/vterm_mouse.obj \
	$(OBJDIR)/vterm_parser.obj \
	$(OBJDIR)/vterm_pen.obj \
	$(OBJDIR)/vterm_screen.obj \
	$(OBJDIR)/vterm_state.obj \
	$(OBJDIR)/vterm_unicode.obj \
	$(OBJDIR)/vterm_vterm.obj
TERM_DEFS = -DFEAT_TERMINAL
TERM_DEPS = \
	libvterm/include/vterm.h \
	libvterm/include/vterm_keycodes.h \
	libvterm/src/rect.h \
	libvterm/src/utf8.h \
	libvterm/src/vterm_internal.h
!endif

!ifndef SOUND
! if "$(FEATURES)"=="HUGE"
SOUND = yes
! else
SOUND = no
! endif
!endif

!ifndef SODIUM
SODIUM = no
!endif
!ifndef DYNAMIC_SODIUM
DYNAMIC_SODIUM = yes
!endif

!if "$(SODIUM)" != "no"
! if "$(CPU)" == "AMD64"
SOD_LIB		= $(SODIUM)\x64\Release\v140\dynamic
! elseif "$(CPU)" == "i386"
SOD_LIB		= $(SODIUM)\Win32\Release\v140\dynamic
! else
SODIUM = no
! endif
!endif

!if "$(SODIUM)" != "no"
SOD_INC		= /I "$(SODIUM)\include"
! if "$(DYNAMIC_SODIUM)" == "yes"
SODIUM_DLL	= libsodium.dll
SOD_DEFS	= -DHAVE_SODIUM -DDYNAMIC_SODIUM -DDYNAMIC_SODIUM_DLL=\"$(SODIUM_DLL)\"
SOD_LIB		=
! else
SOD_DEFS	= -DHAVE_SODIUM
SOD_LIB		= $(SOD_LIB)\libsodium.lib
! endif
!endif

!ifndef NETBEANS
NETBEANS = $(GUI)
!endif

!ifndef CHANNEL
! if "$(FEATURES)"=="HUGE" || "$(TERMINAL)"=="yes"
CHANNEL = yes
! else
CHANNEL = $(GUI)
! endif
!endif

# GUI specific features.
!if "$(GUI)" == "yes"
# Only allow NETBEANS for a GUI build and CHANNEL.
! if "$(NETBEANS)" == "yes" && "$(CHANNEL)" == "yes"
# NETBEANS - Include support for Netbeans integration
NETBEANS_PRO	= proto/netbeans.pro
NETBEANS_OBJ	= $(OBJDIR)/netbeans.obj
NETBEANS_DEFS	= -DFEAT_NETBEANS_INTG

!  if "$(NBDEBUG)" == "yes"
NBDEBUG_DEFS	= -DNBDEBUG
NBDEBUG_INCL	= nbdebug.h
NBDEBUG_SRC	= nbdebug.c
!  endif
! endif

# DirectWrite (DirectX)
! if "$(DIRECTX)" == "yes"
DIRECTX_DEFS	= -DFEAT_DIRECTX -DDYNAMIC_DIRECTX
!  if "$(COLOR_EMOJI)" != "no"
DIRECTX_DEFS	= $(DIRECTX_DEFS) -DFEAT_DIRECTX_COLOR_EMOJI
!  endif
DIRECTX_INCL	= gui_dwrite.h
DIRECTX_OBJ	= $(OUTDIR)\gui_dwrite.obj
! endif

# Only allow XPM for a GUI build.
! ifndef XPM
!  ifndef USE_MSVCRT
# Both XPM and USE_MSVCRT are not set, use the included xpm files, depending
# on the architecture.
!   if "$(CPU)" == "AMD64"
XPM = xpm\x64
!   elseif "$(CPU)" == "ARM64"
XPM = xpm\arm64
!   elseif "$(CPU)" == "i386"
XPM = xpm\x86
!   else
XPM = no
!   endif
!  else # USE_MSVCRT
XPM = no
!  endif # USE_MSVCRT
! endif # XPM
! if "$(XPM)" != "no"
# XPM - Include support for XPM signs
# See the xpm directory for more information.
XPM_OBJ   = $(OBJDIR)/xpm_w32.obj
XPM_DEFS  = -DFEAT_XPM_W32
XPM_LIB   = $(XPM)\lib-vc14\libXpm.lib
XPM_INC	  = -I $(XPM)\include -I $(XPM)\..\include
! endif
!endif # GUI

!if "$(SOUND)" == "yes"
SOUND_PRO	= proto/sound.pro
SOUND_OBJ	= $(OBJDIR)/sound.obj
SOUND_DEFS	= -DFEAT_SOUND
SOUND_LIB	= winmm.lib
!endif

!if "$(CHANNEL)" == "yes"
CHANNEL_PRO	= proto/job.pro proto/channel.pro
CHANNEL_OBJ	= $(OBJDIR)/job.obj $(OBJDIR)/channel.obj
CHANNEL_DEFS	= -DFEAT_JOB_CHANNEL -DFEAT_IPV6 -DHAVE_INET_NTOP

NETBEANS_LIB	= Ws2_32.lib
!endif

# need advapi32.lib for GetUserName()
# need shell32.lib for ExtractIcon()
# need netapi32.lib for NetUserEnum()
# gdi32.lib and comdlg32.lib for printing support
# ole32.lib and uuid.lib are needed for FEAT_SHORTCUT
CON_LIB = oldnames.lib kernel32.lib advapi32.lib shell32.lib gdi32.lib \
	  comdlg32.lib ole32.lib netapi32.lib uuid.lib user32.lib \
	  /machine:$(CPU)
!if "$(DELAYLOAD)" == "yes"
CON_LIB = $(CON_LIB) /DELAYLOAD:comdlg32.dll /DELAYLOAD:ole32.dll DelayImp.lib
!endif

# If you have a fixed directory for $VIM or $VIMRUNTIME, other than the normal
# default, use these lines.
#VIMRCLOC = somewhere
#VIMRUNTIMEDIR = somewhere

CFLAGS = -c /W3 /GF /nologo -I. -Iproto -DHAVE_PATHDEF -DWIN32 -DHAVE_STDINT_H \
		$(CSCOPE_DEFS) $(TERM_DEFS) $(SOUND_DEFS) $(NETBEANS_DEFS) $(CHANNEL_DEFS) \
		$(NBDEBUG_DEFS) $(XPM_DEFS) $(SOD_DEFS) $(SOD_INC) \
		$(DEFINES) -DWINVER=$(WINVER) -D_WIN32_WINNT=$(WINVER) \
		/source-charset:utf-8

#>>>>> end of choices
###########################################################################

DEL_TREE = rmdir /s /q

INTDIR=$(OBJDIR)
OUTDIR=$(OBJDIR)

### Validate CPUNR
!ifndef CPUNR
# default to SSE2
CPUNR = sse2
!elseif "$(CPUNR)" == "i386" || "$(CPUNR)" == "i486" || "$(CPUNR)" == "i586"
# alias i386, i486 and i586 to i686
! message *** WARNING CPUNR=$(CPUNR) is not a valid target architecture.
! message Windows 7 is the minimum target OS, with a minimum target
! message architecture of i686.
! message Retargeting to i686
CPUNR = i686
!elseif "$(CPUNR)" == "pentium4"
# alias pentium4 to sse2
! message *** WARNING CPUNR=pentium4 is deprecated in favour of sse2.
! message Retargeting to sse2.
CPUNR = sse2
!elseif "$(CPUNR)" != "any" && "$(CPUNR)" != "i686" && "$(CPUNR)" != "sse" && "$(CPUNR)" != "sse2" && "$(CPUNR)" != "avx" && "$(CPUNR)" != "avx2"
! error *** ERROR Unknown target architecture "$(CPUNR)". Make aborted.
!endif

# Convert processor ID to MVC-compatible number
# IA32/SSE/SSE2 are only supported on x86
!if "$(ASSEMBLY_ARCHITECTURE)" == "i386" && ("$(CPUNR)" == "i686" || "$(CPUNR)" == "any")
CPUARG = /arch:IA32
!elseif "$(ASSEMBLY_ARCHITECTURE)" == "i386" && "$(CPUNR)" == "sse"
CPUARG = /arch:SSE
!elseif "$(ASSEMBLY_ARCHITECTURE)" == "i386" && "$(CPUNR)" == "sse2"
CPUARG = /arch:SSE2
!elseif "$(CPUNR)" == "avx"
CPUARG = /arch:AVX
!elseif "$(CPUNR)" == "avx2"
CPUARG = /arch:AVX2
!endif

# Pass CPUARG to GvimExt, to avoid using version-dependent defaults
MAKEFLAGS_GVIMEXT = $(MAKEFLAGS_GVIMEXT) CPUARG="$(CPUARG)"

!if "$(VIMDLL)" == "yes"
VIMDLLBASE = vim
! if "$(ASSEMBLY_ARCHITECTURE)" == "i386"
VIMDLLBASE = $(VIMDLLBASE)32
! else
VIMDLLBASE = $(VIMDLLBASE)64
! endif
! if "$(DEBUG)" == "yes"
VIMDLLBASE = $(VIMDLLBASE)d
! endif
!endif

LIBC =
DEBUGINFO = /Zi

# Use multiprocess build.
!if "$(USE_MP)" == "yes"
CFLAGS = $(CFLAGS) /MP
!endif

# Use static code analysis
!if "$(ANALYZE)" == "yes"
CFLAGS = $(CFLAGS) /analyze
!endif

# Address Sanitizer (ASAN) generally available starting with VS2019 version
# 16.9
!if ("$(ASAN)" == "yes") && ($(MSVC_FULL) >= 192829913)
CFLAGS = $(CFLAGS) /fsanitize=address
!endif

!ifdef NODEBUG

VIM = vim
! if "$(OPTIMIZE)" == "SPACE"
OPTFLAG = /O1
! elseif "$(OPTIMIZE)" == "SPEED"
OPTFLAG = /O2
! else # MAXSPEED
OPTFLAG = /Ox
! endif

# Use link time code generation if not worried about size
! if "$(OPTIMIZE)" != "SPACE"
OPTFLAG = $(OPTFLAG) /GL
! endif

CFLAGS = $(CFLAGS) $(OPTFLAG) -DNDEBUG $(CPUARG)
RCFLAGS = -DNDEBUG
! ifdef USE_MSVCRT
CFLAGS = $(CFLAGS) /MD
LIBC = msvcrt.lib
! else
CFLAGS = $(CFLAGS) /Zl /MT
LIBC = libcmt.lib
! endif

!else  # DEBUG

VIM = vimd
! if ("$(CPU)" == "i386") || ("$(CPU)" == "ix86")
DEBUGINFO = /ZI
! endif
CFLAGS = $(CFLAGS) -D_DEBUG -DDEBUG /Od
RCFLAGS = -D_DEBUG -DDEBUG
# The /fixed:no is needed for Quantify.
LIBC = /fixed:no
! ifdef USE_MSVCRT
CFLAGS = $(CFLAGS) /MDd
LIBC = $(LIBC) msvcrtd.lib
! else
CFLAGS = $(CFLAGS) /Zl /MTd
LIBC = $(LIBC) libcmtd.lib
! endif

!endif # DEBUG

# Visual Studio 2005 has 'deprecated' many of the standard CRT functions
CFLAGS_DEPR = /D_CRT_SECURE_NO_DEPRECATE /D_CRT_NONSTDC_NO_DEPRECATE
CFLAGS = $(CFLAGS) $(CFLAGS_DEPR)

!include Make_all.mak
!include testdir\Make_all.mak

INCL =	vim.h alloc.h ascii.h ex_cmds.h feature.h errors.h globals.h \
	keymap.h macros.h option.h os_dos.h os_win32.h proto.h regexp.h \
	spell.h structs.h termdefs.h beval.h $(NBDEBUG_INCL)

OBJ = \
	$(OUTDIR)\alloc.obj \
	$(OUTDIR)\arabic.obj \
	$(OUTDIR)\arglist.obj \
	$(OUTDIR)\autocmd.obj \
	$(OUTDIR)\beval.obj \
	$(OUTDIR)\blob.obj \
	$(OUTDIR)\blowfish.obj \
	$(OUTDIR)\buffer.obj \
	$(OUTDIR)\bufwrite.obj \
	$(OUTDIR)\change.obj \
	$(OUTDIR)\charset.obj \
	$(OUTDIR)\cindent.obj \
	$(OUTDIR)\clientserver.obj \
	$(OUTDIR)\clipboard.obj \
	$(OUTDIR)\cmdexpand.obj \
	$(OUTDIR)\cmdhist.obj \
	$(OUTDIR)\crypt.obj \
	$(OUTDIR)\crypt_zip.obj \
	$(OUTDIR)\debugger.obj \
	$(OUTDIR)\dict.obj \
	$(OUTDIR)\diff.obj \
	$(OUTDIR)\digraph.obj \
	$(OUTDIR)\drawline.obj \
	$(OUTDIR)\drawscreen.obj \
	$(OUTDIR)\edit.obj \
	$(OUTDIR)\eval.obj \
	$(OUTDIR)\evalbuffer.obj \
	$(OUTDIR)\evalfunc.obj \
	$(OUTDIR)\evalvars.obj \
	$(OUTDIR)\evalwindow.obj \
	$(OUTDIR)\ex_cmds.obj \
	$(OUTDIR)\ex_cmds2.obj \
	$(OUTDIR)\ex_docmd.obj \
	$(OUTDIR)\ex_eval.obj \
	$(OUTDIR)\ex_getln.obj \
	$(OUTDIR)\fileio.obj \
	$(OUTDIR)\filepath.obj \
	$(OUTDIR)\findfile.obj \
	$(OUTDIR)\float.obj \
	$(OUTDIR)\fold.obj \
	$(OUTDIR)\getchar.obj \
	$(OUTDIR)\gui_xim.obj \
	$(OUTDIR)\hardcopy.obj \
	$(OUTDIR)\hashtab.obj \
	$(OUTDIR)\help.obj \
	$(OUTDIR)\highlight.obj \
	$(OUTDIR)\if_cscope.obj \
	$(OUTDIR)\indent.obj \
	$(OUTDIR)\insexpand.obj \
	$(OUTDIR)\json.obj \
	$(OUTDIR)\list.obj \
	$(OUTDIR)\locale.obj \
	$(OUTDIR)\logfile.obj \
	$(OUTDIR)\main.obj \
	$(OUTDIR)\map.obj \
	$(OUTDIR)\mark.obj \
	$(OUTDIR)\match.obj \
	$(OUTDIR)\mbyte.obj \
	$(OUTDIR)\memfile.obj \
	$(OUTDIR)\memline.obj \
	$(OUTDIR)\menu.obj \
	$(OUTDIR)\message.obj \
	$(OUTDIR)\misc1.obj \
	$(OUTDIR)\misc2.obj \
	$(OUTDIR)\mouse.obj \
	$(OUTDIR)\move.obj \
	$(OUTDIR)\normal.obj \
	$(OUTDIR)\ops.obj \
	$(OUTDIR)\option.obj \
	$(OUTDIR)\optionstr.obj \
	$(OUTDIR)\os_mswin.obj \
	$(OUTDIR)\os_win32.obj \
	$(OUTDIR)\pathdef.obj \
	$(OUTDIR)\popupmenu.obj \
	$(OUTDIR)\popupwin.obj \
	$(OUTDIR)\profiler.obj \
	$(OUTDIR)\quickfix.obj \
	$(OUTDIR)\regexp.obj \
	$(OUTDIR)\register.obj \
	$(OUTDIR)\scriptfile.obj \
	$(OUTDIR)\screen.obj \
	$(OUTDIR)\search.obj \
	$(OUTDIR)\session.obj \
	$(OUTDIR)\sha256.obj \
	$(OUTDIR)\sign.obj \
	$(OUTDIR)\spell.obj \
	$(OUTDIR)\spellfile.obj \
	$(OUTDIR)\spellsuggest.obj \
	$(OUTDIR)\strings.obj \
	$(OUTDIR)\syntax.obj \
	$(OUTDIR)\tag.obj \
	$(OUTDIR)\term.obj \
	$(OUTDIR)\testing.obj \
	$(OUTDIR)\textformat.obj \
	$(OUTDIR)\textobject.obj \
	$(OUTDIR)\textprop.obj \
	$(OUTDIR)\time.obj \
	$(OUTDIR)\typval.obj \
	$(OUTDIR)\ui.obj \
	$(OUTDIR)\undo.obj \
	$(OUTDIR)\usercmd.obj \
	$(OUTDIR)\userfunc.obj \
	$(OUTDIR)\vim9class.obj \
	$(OUTDIR)\vim9cmds.obj \
	$(OUTDIR)\vim9compile.obj \
	$(OUTDIR)\vim9execute.obj \
	$(OUTDIR)\vim9expr.obj \
	$(OUTDIR)\vim9instr.obj \
	$(OUTDIR)\vim9script.obj \
	$(OUTDIR)\vim9type.obj \
	$(OUTDIR)\viminfo.obj \
	$(OUTDIR)\winclip.obj \
	$(OUTDIR)\window.obj \

!if "$(VIMDLL)" == "yes"
OBJ = $(OBJ) $(OUTDIR)\os_w32dll.obj $(OUTDIR)\vimd.res
EXEOBJC = $(OUTDIR)\os_w32exec.obj $(OUTDIR)\vimc.res
EXEOBJG = $(OUTDIR)\os_w32exeg.obj $(OUTDIR)\vimg.res
CFLAGS = $(CFLAGS) -DVIMDLL
!else
OBJ = $(OBJ) $(OUTDIR)\os_w32exe.obj $(OUTDIR)\vim.res
!endif

!if "$(OLE)" == "yes"
CFLAGS = $(CFLAGS) -DFEAT_OLE
RCFLAGS = $(RCFLAGS) -DFEAT_OLE
OLE_OBJ = $(OUTDIR)\if_ole.obj
OLE_IDL = if_ole.idl
OLE_LIB = oleaut32.lib
!endif

!ifndef IME
IME = yes
!endif
!if "$(IME)" == "yes"
CFLAGS = $(CFLAGS) -DFEAT_MBYTE_IME
! ifndef DYNAMIC_IME
DYNAMIC_IME = yes
! endif
! if "$(DYNAMIC_IME)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_IME
! else
IME_LIB = imm32.lib
! endif
!endif

!if "$(GUI)" == "yes"
SUBSYSTEM = windows
CFLAGS = $(CFLAGS) -DFEAT_GUI_MSWIN
RCFLAGS = $(RCFLAGS) -DFEAT_GUI_MSWIN
! if "$(VIMDLL)" == "yes"
SUBSYSTEM_CON = console
GVIM = g$(VIM)
CUI_INCL = iscygpty.h
CUI_OBJ = $(OUTDIR)\iscygpty.obj
RCFLAGS = $(RCFLAGS) -DVIMDLL
! else
VIM = g$(VIM)
! endif
GUI_INCL = \
	gui.h
GUI_OBJ = \
	$(OUTDIR)\gui.obj \
	$(OUTDIR)\gui_beval.obj \
	$(OUTDIR)\gui_w32.obj
GUI_LIB = \
	version.lib $(IME_LIB) winspool.lib comctl32.lib
!else
SUBSYSTEM = console
CUI_INCL = iscygpty.h
CUI_OBJ = $(OUTDIR)\iscygpty.obj
!endif
SUBSYSTEM_TOOLS = console

XDIFF_OBJ = $(OBJDIR)/xdiffi.obj \
	$(OBJDIR)/xemit.obj \
	$(OBJDIR)/xprepare.obj \
	$(OBJDIR)/xutils.obj \
	$(OBJDIR)/xhistogram.obj \
	$(OBJDIR)/xpatience.obj

XDIFF_DEPS = \
	xdiff/xdiff.h \
	xdiff/xdiffi.h \
	xdiff/xemit.h \
	xdiff/xinclude.h \
	xdiff/xmacros.h \
	xdiff/xprepare.h \
	xdiff/xtypes.h \
	xdiff/xutils.h


!if "$(SUBSYSTEM_VER)" != ""
SUBSYSTEM = $(SUBSYSTEM),$(SUBSYSTEM_VER)
SUBSYSTEM_TOOLS = $(SUBSYSTEM_TOOLS),$(SUBSYSTEM_VER)
! if "$(VIMDLL)" == "yes"
SUBSYSTEM_CON = $(SUBSYSTEM_CON),$(SUBSYSTEM_VER)
! endif
# Pass SUBSYSTEM_VER to GvimExt and other tools
MAKEFLAGS_GVIMEXT = $(MAKEFLAGS_GVIMEXT) SUBSYSTEM_VER=$(SUBSYSTEM_VER)
MAKEFLAGS_TOOLS = $(MAKEFLAGS_TOOLS) SUBSYSTEM_VER=$(SUBSYSTEM_VER)
!endif

!if "$(GUI)" == "yes" && "$(DIRECTX)" == "yes"
CFLAGS = $(CFLAGS) $(DIRECTX_DEFS)
GUI_INCL = $(GUI_INCL) $(DIRECTX_INCL)
GUI_OBJ = $(GUI_OBJ) $(DIRECTX_OBJ)
!endif

# iconv.dll library (dynamically loaded)
!ifndef ICONV
ICONV = yes
!endif
!if "$(ICONV)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_ICONV
!endif

# libintl.dll library
!ifndef GETTEXT
GETTEXT = yes
!endif
!if "$(GETTEXT)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_GETTEXT
!endif

# TCL interface
!ifdef TCL
! ifndef TCL_VER
TCL_VER = 86
TCL_VER_LONG = 8.6
! endif
! message Tcl requested (version $(TCL_VER)) - root dir is "$(TCL)"
! if "$(DYNAMIC_TCL)" == "yes"
!  message Tcl DLL will be loaded dynamically
!  ifndef TCL_DLL
TCL_DLL = tcl$(TCL_VER).dll
!  endif
CFLAGS  = $(CFLAGS) -DFEAT_TCL -DDYNAMIC_TCL -DDYNAMIC_TCL_DLL=\"$(TCL_DLL)\" \
		-DDYNAMIC_TCL_VER=\"$(TCL_VER_LONG)\"
TCL_OBJ	= $(OUTDIR)\if_tcl.obj
TCL_INC	= /I "$(TCL)\Include" /I "$(TCL)"
TCL_LIB = "$(TCL)\lib\tclstub$(TCL_VER).lib"
! else
CFLAGS  = $(CFLAGS) -DFEAT_TCL
TCL_OBJ	= $(OUTDIR)\if_tcl.obj
TCL_INC	= /I "$(TCL)\Include" /I "$(TCL)"
TCL_LIB = "$(TCL)\lib\tcl$(TCL_VER)vc.lib"
! endif
!endif

# Lua interface
!ifdef LUA
! ifndef LUA_VER
LUA_VER = 53
! endif
! message Lua requested (version $(LUA_VER)) - root dir is "$(LUA)"
! if "$(DYNAMIC_LUA)" == "yes"
!  message Lua DLL will be loaded dynamically
!  endif
CFLAGS = $(CFLAGS) -DFEAT_LUA
LUA_OBJ = $(OUTDIR)\if_lua.obj
LUA_INC = /I "$(LUA)\include" /I "$(LUA)"
! if "$(DYNAMIC_LUA)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_LUA \
		-DDYNAMIC_LUA_DLL=\"lua$(LUA_VER).dll\"
LUA_LIB = /nodefaultlib:lua$(LUA_VER).lib
! else
LUA_LIB = "$(LUA)\lib\lua$(LUA_VER).lib"
! endif
!endif

!ifdef PYTHON
! ifdef PYTHON3
DYNAMIC_PYTHON=yes
DYNAMIC_PYTHON3=yes
! endif
!endif

# PYTHON interface
!ifdef PYTHON
! ifndef PYTHON_VER
PYTHON_VER = 27
! endif
! message Python requested (version $(PYTHON_VER)) - root dir is "$(PYTHON)"
! if "$(DYNAMIC_PYTHON)" == "yes"
!  message Python DLL will be loaded dynamically
! endif
CFLAGS = $(CFLAGS) -DFEAT_PYTHON
PYTHON_OBJ = $(OUTDIR)\if_python.obj
PYTHON_INC = /I "$(PYTHON)\Include" /I "$(PYTHON)\PC"
! if "$(DYNAMIC_PYTHON)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_PYTHON \
		-DDYNAMIC_PYTHON_DLL=\"python$(PYTHON_VER).dll\"
PYTHON_LIB = /nodefaultlib:python$(PYTHON_VER).lib
! else
PYTHON_LIB = "$(PYTHON)\libs\python$(PYTHON_VER).lib"
! endif
!endif

# PYTHON3 interface
!ifdef PYTHON3
! ifndef PYTHON3_VER
PYTHON3_VER = 36
! endif
! if "$(DYNAMIC_PYTHON3_STABLE_ABI)" == "yes"
PYTHON3_NAME = python3
! else
PYTHON3_NAME = python$(PYTHON3_VER)
! endif
! ifndef DYNAMIC_PYTHON3_DLL
DYNAMIC_PYTHON3_DLL = $(PYTHON3_NAME).dll
! endif
! message Python3 requested (version $(PYTHON3_VER)) - root dir is "$(PYTHON3)"
! if "$(DYNAMIC_PYTHON3)" == "yes"
!  message Python3 DLL will be loaded dynamically
! endif
CFLAGS = $(CFLAGS) -DFEAT_PYTHON3
PYTHON3_OBJ = $(OUTDIR)\if_python3.obj
PYTHON3_INC = /I "$(PYTHON3)\Include" /I "$(PYTHON3)\PC"
! if "$(DYNAMIC_PYTHON3)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_PYTHON3 \
		-DDYNAMIC_PYTHON3_DLL=\"$(DYNAMIC_PYTHON3_DLL)\"
!  if "$(DYNAMIC_PYTHON3_STABLE_ABI)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_PYTHON3_STABLE_ABI
PYTHON3_INC = $(PYTHON3_INC) -DPy_LIMITED_API=0x3080000
!  endif
PYTHON3_LIB = /nodefaultlib:$(PYTHON3_NAME).lib
! else
CFLAGS = $(CFLAGS) -DPYTHON3_DLL=\"$(DYNAMIC_PYTHON3_DLL)\"
PYTHON3_LIB = "$(PYTHON3)\libs\$(PYTHON3_NAME).lib"
! endif
!endif

# MzScheme interface
!ifdef MZSCHEME
! message MzScheme requested - root dir is "$(MZSCHEME)"
! ifndef MZSCHEME_VER
MZSCHEME_VER = 3m_a0solc
! endif
! ifndef MZSCHEME_COLLECTS
MZSCHEME_COLLECTS=$(MZSCHEME)\collects
! endif
CFLAGS = $(CFLAGS) -DFEAT_MZSCHEME -I "$(MZSCHEME)\include"
! if EXIST("$(MZSCHEME)\lib\msvc\libmzsch$(MZSCHEME_VER).lib")
MZSCHEME_MAIN_LIB=mzsch
! else
MZSCHEME_MAIN_LIB=racket
! endif
! if (EXIST("$(MZSCHEME)\lib\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll") \
     && !EXIST("$(MZSCHEME)\lib\libmzgc$(MZSCHEME_VER).dll")) \
    || (EXIST("$(MZSCHEME)\lib\msvc\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib") \
        && !EXIST("$(MZSCHEME)\lib\msvc\libmzgc$(MZSCHEME_VER).lib"))
!  message Building with Precise GC
MZSCHEME_PRECISE_GC = yes
CFLAGS = $(CFLAGS) -DMZ_PRECISE_GC
! endif
! if "$(DYNAMIC_MZSCHEME)" == "yes"
!  message MzScheme DLLs will be loaded dynamically
CFLAGS = $(CFLAGS) -DDYNAMIC_MZSCHEME
!  if "$(MZSCHEME_PRECISE_GC)" == "yes"
# Precise GC does not use separate dll
CFLAGS = $(CFLAGS) \
	 -DDYNAMIC_MZSCH_DLL=\"lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll\" \
	 -DDYNAMIC_MZGC_DLL=\"lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll\"
!  else
CFLAGS = $(CFLAGS) \
	 -DDYNAMIC_MZSCH_DLL=\"lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).dll\" \
	 -DDYNAMIC_MZGC_DLL=\"libmzgc$(MZSCHEME_VER).dll\"
!  endif
! else
!  if "$(MZSCHEME_DEBUG)" == "yes"
CFLAGS = $(CFLAGS) -DMZSCHEME_FORCE_GC
!  endif
!  if "$(MZSCHEME_PRECISE_GC)" == "yes"
# Precise GC does not use separate dll
!   if EXIST("$(MZSCHEME)\lib\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).def")
# create .lib from .def
MZSCHEME_LIB = lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib
MZSCHEME_EXTRA_DEP = lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib
!   else
MZSCHEME_LIB = "$(MZSCHEME)\lib\msvc\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib"
!   endif
!  else
MZSCHEME_LIB = "$(MZSCHEME)\lib\msvc\libmzgc$(MZSCHEME_VER).lib" \
		"$(MZSCHEME)\lib\msvc\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib"
!  endif
! endif
MZSCHEME_OBJ = $(OUTDIR)\if_mzsch.obj
# increase stack size
MZSCHEME_LIB = $(MZSCHEME_LIB) /STACK:8388608
MZSCHEME_INCL = if_mzsch.h
!endif

# Perl interface
!ifdef PERL
! ifndef PERL_VER
PERL_VER = 524
! endif
! message Perl requested (version $(PERL_VER)) - root dir is "$(PERL)"
! if "$(DYNAMIC_PERL)" == "yes"
!  message Perl DLL will be loaded dynamically
! endif

# Is Perl installed in architecture-specific directories?
! if exist($(PERL)\Bin\MSWin32-x86)
PERL_ARCH = \MSWin32-x86
! endif

PERL_INCDIR = $(PERL)\Lib$(PERL_ARCH)\Core

# Version-dependent stuff
PERL_DLL = perl$(PERL_VER).dll
! if exist($(PERL_INCDIR)\perl$(PERL_VER).lib)
PERL_LIB = $(PERL_INCDIR)\perl$(PERL_VER).lib
! else
# For ActivePerl 5.18 and later
PERL_LIB = $(PERL_INCDIR)\libperl$(PERL_VER).a
! endif

CFLAGS = $(CFLAGS) -DFEAT_PERL -DPERL_IMPLICIT_CONTEXT -DPERL_IMPLICIT_SYS

# Do we want to load Perl dynamically?
! if "$(DYNAMIC_PERL)" == "yes"
CFLAGS = $(CFLAGS) -DDYNAMIC_PERL -DDYNAMIC_PERL_DLL=\"$(PERL_DLL)\"
!  undef PERL_LIB
! endif

PERL_EXE = $(PERL)\Bin$(PERL_ARCH)\perl
PERL_INC = /I $(PERL_INCDIR)
PERL_OBJ = $(OUTDIR)\if_perl.obj $(OUTDIR)\if_perlsfio.obj
XSUBPP = $(PERL)\lib\ExtUtils\xsubpp
! if exist($(XSUBPP))
XSUBPP = $(PERL_EXE) $(XSUBPP)
! else
XSUBPP = xsubpp
! endif
XSUBPP_TYPEMAP = $(PERL)\lib\ExtUtils\typemap

!endif

#
# Support Ruby interface
#
!ifdef RUBY
#  Set default value
! ifndef RUBY_VER
RUBY_VER = 22
! endif
! ifndef RUBY_VER_LONG
RUBY_VER_LONG = 2.2.0
! endif
! ifndef RUBY_API_VER_LONG
RUBY_API_VER_LONG = $(RUBY_VER_LONG)
! endif
! ifndef RUBY_API_VER
RUBY_API_VER = $(RUBY_API_VER_LONG:.=)
! endif

! ifndef RUBY_PLATFORM
!  if "$(CPU)" == "i386"
RUBY_PLATFORM = i386-mswin32
!  else # CPU
RUBY_PLATFORM = x64-mswin64
!  endif # CPU
RUBY_PLATFORM = $(RUBY_PLATFORM)_$(MSVCRT_VER)
! endif # RUBY_PLATFORM

! ifndef RUBY_INSTALL_NAME
!  ifndef RUBY_MSVCRT_NAME
# Base name of msvcrXX.dll which is used by ruby's dll.
RUBY_MSVCRT_NAME = $(MSVCRT_NAME)
!  endif # RUBY_MSVCRT_NAME
!  if "$(CPU)" == "i386"
RUBY_INSTALL_NAME = $(RUBY_MSVCRT_NAME)-ruby$(RUBY_API_VER)
!  else # CPU
!   if EXIST($(RUBY)/lib/ruby/$(RUBY_API_VER_LONG)/x64-mingw-ucrt)
RUBY_INSTALL_NAME = x64-ucrt-ruby$(RUBY_API_VER)
!   else
RUBY_INSTALL_NAME = x64-$(RUBY_MSVCRT_NAME)-ruby$(RUBY_API_VER)
!   endif
!  endif # CPU
! endif # RUBY_INSTALL_NAME

! message Ruby requested (version $(RUBY_VER)) - root dir is "$(RUBY)"
CFLAGS = $(CFLAGS) -DFEAT_RUBY
RUBY_OBJ = $(OUTDIR)\if_ruby.obj
RUBY_INC = /I "$(RUBY)\include\ruby-$(RUBY_API_VER_LONG)" /I "$(RUBY)\include\ruby-$(RUBY_API_VER_LONG)\$(RUBY_PLATFORM)"
RUBY_LIB = "$(RUBY)\lib\$(RUBY_INSTALL_NAME).lib"
# Do we want to load Ruby dynamically?
! if "$(DYNAMIC_RUBY)" == "yes"
!  message Ruby DLL will be loaded dynamically
CFLAGS = $(CFLAGS) -DDYNAMIC_RUBY \
		-DDYNAMIC_RUBY_DLL=\"$(RUBY_INSTALL_NAME).dll\"
!  undef RUBY_LIB
! endif
CFLAGS = $(CFLAGS) -DRUBY_VERSION=$(RUBY_VER)
!endif # RUBY

#
# Support PostScript printing
#
!if "$(POSTSCRIPT)" == "yes"
CFLAGS = $(CFLAGS) -DMSWINPS
!endif # POSTSCRIPT

#
# FEATURES: TINY, NORMAL, or HUGE
#
CFLAGS = $(CFLAGS) -DFEAT_$(FEATURES)

#
# MODIFIED_BY - Name of who modified a release version
#
!if "$(MODIFIED_BY)" != ""
CFLAGS = $(CFLAGS) -DMODIFIED_BY=\"$(MODIFIED_BY)\"
!endif

#
# Always generate the .pdb file, so that we get debug symbols that can be used
# on a crash (doesn't add overhead to the executable).
# Generate edit-and-continue debug info when no optimization - allows to
# debug more conveniently (able to look at variables which are in registers)
#
CFLAGS = $(CFLAGS) /Fd$(OUTDIR)/ $(DEBUGINFO)
!if "$(VIMDLL)" == "yes"
LINK_PDB = /PDB:$(VIMDLLBASE).pdb -debug
!else
LINK_PDB = /PDB:$(VIM).pdb -debug
!endif

#
# End extra feature include
#
!message

# CFLAGS with /Fo$(OUTDIR)/
CFLAGS_OUTDIR=$(CFLAGS) /Fo$(OUTDIR)/

PATHDEF_SRC = $(OUTDIR)\pathdef.c

LINKARGS1 = /nologo
LINKARGS2 = $(CON_LIB) $(GUI_LIB) $(LIBC) $(OLE_LIB) \
		$(LUA_LIB) $(MZSCHEME_LIB) $(PERL_LIB) $(PYTHON_LIB) $(PYTHON3_LIB) $(RUBY_LIB) \
		$(TCL_LIB) $(SOUND_LIB) $(NETBEANS_LIB) $(XPM_LIB) $(SOD_LIB) $(LINK_PDB)

!ifdef NODEBUG
# Add /opt:ref to remove unreferenced functions and data even when /DEBUG is
# added.
LINKARGS1 = $(LINKARGS1) /opt:ref
!else
LINKARGS1 = $(LINKARGS1) /opt:noref /opt:noicf
!endif

!if "$(MAP)" == "yes"
# "/map" is for debugging
LINKARGS1 = $(LINKARGS1) /map
!elseif "$(MAP)" == "lines"
# "/mapinfo:lines" is for debugging, only works for VC6 and later
LINKARGS1 = $(LINKARGS1) /map /mapinfo:lines
!endif

# Enable link time code generation if needed.
!ifdef NODEBUG
! if "$(OPTIMIZE)" != "SPACE"
!  if "$(CI)" == "true" || "$(CI)" == "True"
# Enable link time code generation, but do not show the progress.
LINKARGS1 = $(LINKARGS1) /LTCG
!  else
# Report link time code generation progress.
LINKARGS1 = $(LINKARGS1) /LTCG:STATUS
!  endif
! endif
!endif

!if "$(CPU)" == "AMD64" && "$(GUI)" == "yes"
# This option is required for VC2012 or later so that 64-bit gvim can
# accept D&D from 32-bit applications.  NOTE: This disables 64-bit ASLR,
# therefore the security level becomes as same as VC2010.
LINKARGS1 = $(LINKARGS1) /HIGHENTROPYVA:NO
!endif

!if "$(VIMDLL)" == "yes"
MAIN_TARGET = $(GVIM).exe $(VIM).exe $(VIMDLLBASE).dll
!else
MAIN_TARGET = $(VIM).exe
!endif

# Target to run individual tests.
VIMTESTTARGET = $(VIM).exe

all:	$(MAIN_TARGET) \
	vimrun.exe \
	install.exe \
	uninstall.exe \
	xxd/xxd.exe \
	tee/tee.exe \
	GvimExt/gvimext.dll

# To get around the command line limit: Make use of nmake's response files to
# capture the arguments for $(LINK) in a file  using the @<<ARGS<< syntax.

!if "$(VIMDLL)" == "yes"

$(VIMDLLBASE).dll: $(OUTDIR) $(OBJ) $(XDIFF_OBJ) $(GUI_OBJ) $(CUI_OBJ) $(OLE_OBJ) $(OLE_IDL) $(MZSCHEME_OBJ) \
		$(LUA_OBJ) $(PERL_OBJ) $(PYTHON_OBJ) $(PYTHON3_OBJ) $(RUBY_OBJ) $(TCL_OBJ) \
		$(TERM_OBJ) $(SOUND_OBJ) $(NETBEANS_OBJ) $(CHANNEL_OBJ) $(XPM_OBJ) \
		version.c version.h
	$(CC) $(CFLAGS_OUTDIR) version.c
	$(LINK) @<<
$(LINKARGS1) /dll -out:$(VIMDLLBASE).dll $(OBJ) $(XDIFF_OBJ) $(GUI_OBJ) $(CUI_OBJ) $(OLE_OBJ)
$(LUA_OBJ) $(MZSCHEME_OBJ) $(PERL_OBJ) $(PYTHON_OBJ) $(PYTHON3_OBJ) $(RUBY_OBJ)
$(TCL_OBJ) $(TERM_OBJ) $(SOUND_OBJ) $(NETBEANS_OBJ) $(CHANNEL_OBJ)
$(XPM_OBJ) $(OUTDIR)\version.obj $(LINKARGS2)
<<

$(GVIM).exe: $(OUTDIR) $(EXEOBJG) $(VIMDLLBASE).dll
	$(LINK) $(LINKARGS1) /subsystem:$(SUBSYSTEM) -out:$(GVIM).exe $(EXEOBJG) $(VIMDLLBASE).lib $(LIBC)

$(VIM).exe: $(OUTDIR) $(EXEOBJC) $(VIMDLLBASE).dll
	$(LINK) $(LINKARGS1) /subsystem:$(SUBSYSTEM_CON) -out:$(VIM).exe $(EXEOBJC) $(VIMDLLBASE).lib $(LIBC)

!else

$(VIM).exe: $(OUTDIR) $(OBJ) $(XDIFF_OBJ) $(GUI_OBJ) $(CUI_OBJ) $(OLE_OBJ) $(OLE_IDL) $(MZSCHEME_OBJ) \
		$(LUA_OBJ) $(PERL_OBJ) $(PYTHON_OBJ) $(PYTHON3_OBJ) $(RUBY_OBJ) $(TCL_OBJ) \
		$(TERM_OBJ) $(SOUND_OBJ) $(NETBEANS_OBJ) $(CHANNEL_OBJ) $(XPM_OBJ) \
		version.c version.h
	$(CC) $(CFLAGS_OUTDIR) version.c
	$(LINK) @<<
$(LINKARGS1) /subsystem:$(SUBSYSTEM) -out:$(VIM).exe $(OBJ) $(XDIFF_OBJ) $(GUI_OBJ) $(CUI_OBJ) $(OLE_OBJ)
$(LUA_OBJ) $(MZSCHEME_OBJ) $(PERL_OBJ) $(PYTHON_OBJ) $(PYTHON3_OBJ) $(RUBY_OBJ)
$(TCL_OBJ) $(TERM_OBJ) $(SOUND_OBJ) $(NETBEANS_OBJ) $(CHANNEL_OBJ)
$(XPM_OBJ) $(OUTDIR)\version.obj $(LINKARGS2)
<<

!endif

$(VIM): $(VIM).exe

$(OUTDIR):
	if not exist $(OUTDIR)/nul  mkdir $(OUTDIR:/=\)

CFLAGS_INST = /nologo /O2 -DNDEBUG -DWIN32 -DWINVER=$(WINVER) -D_WIN32_WINNT=$(WINVER) $(CFLAGS_DEPR)

install.exe: dosinst.c dosinst.h version.h
	$(CC) $(CFLAGS_INST) dosinst.c kernel32.lib shell32.lib \
		user32.lib ole32.lib advapi32.lib uuid.lib \
		-link -subsystem:$(SUBSYSTEM_TOOLS)
	- if exist install.exe del install.exe
	ren dosinst.exe install.exe

uninstall.exe: uninstall.c dosinst.h version.h
	$(CC) $(CFLAGS_INST) uninstall.c shell32.lib advapi32.lib \
		-link -subsystem:$(SUBSYSTEM_TOOLS)

vimrun.exe: vimrun.c
	$(CC) /nologo -DNDEBUG vimrun.c -link -subsystem:$(SUBSYSTEM_TOOLS)

xxd/xxd.exe: xxd/xxd.c
	cd xxd
	$(MAKE) /NOLOGO -f Make_mvc.mak $(MAKEFLAGS_TOOLS)
	cd ..

tee/tee.exe: tee/tee.c
	cd tee
	$(MAKE) /NOLOGO -f Make_mvc.mak $(MAKEFLAGS_TOOLS)
	cd ..

GvimExt/gvimext.dll: GvimExt/gvimext.cpp GvimExt/gvimext.rc GvimExt/gvimext.h
	cd GvimExt
	$(MAKE) /NOLOGO -f Make_mvc.mak $(MAKEFLAGS_GVIMEXT)
	cd ..


tags: notags
	$(CTAGS) $(TAGS_FILES)

notags:
	- if exist tags del tags

clean: testclean
	- if exist $(OUTDIR)/nul $(DEL_TREE) $(OUTDIR)
	- if exist *.obj del *.obj
	- if exist $(VIM).exe del $(VIM).exe
	- if exist $(VIM).ilk del $(VIM).ilk
	- if exist $(VIM).pdb del $(VIM).pdb
	- if exist $(VIM).map del $(VIM).map
	- if exist $(VIM).ncb del $(VIM).ncb
!if "$(VIMDLL)" == "yes"
	- if exist $(GVIM).exe del $(GVIM).exe
	- if exist $(GVIM).map del $(GVIM).map
	- if exist $(VIMDLLBASE).dll del $(VIMDLLBASE).dll
	- if exist $(VIMDLLBASE).ilk del $(VIMDLLBASE).ilk
	- if exist $(VIMDLLBASE).lib del $(VIMDLLBASE).lib
	- if exist $(VIMDLLBASE).exp del $(VIMDLLBASE).exp
	- if exist $(VIMDLLBASE).pdb del $(VIMDLLBASE).pdb
	- if exist $(VIMDLLBASE).map del $(VIMDLLBASE).map
!endif
	- if exist vimrun.exe del vimrun.exe
	- if exist install.exe del install.exe
	- if exist uninstall.exe del uninstall.exe
	- if exist if_perl.c del if_perl.c
	- if exist auto\if_perl.c del auto\if_perl.c
	- if exist dosinst.exe del dosinst.exe
	cd xxd
	$(MAKE) /NOLOGO -f Make_mvc.mak clean
	cd ..
	cd tee
	$(MAKE) /NOLOGO -f Make_mvc.mak clean
	cd ..
	cd GvimExt
	$(MAKE) /NOLOGO -f Make_mvc.mak clean
	cd ..

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
	$(CC) /nologo -I. -Iproto -DNDEBUG create_nvcmdidxs.c -link -subsystem:$(SUBSYSTEM_TOOLS)
	vim --clean -N -X --not-a-term -u create_nvcmdidxs.vim -c quit
	-del create_nvcmdidxs.exe

test:
	cd testdir
	$(MAKE) /NOLOGO -f Make_mvc.mak
	cd ..

testgvim testgui:
	cd testdir
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\gvim
	cd ..

testtiny:
	cd testdir
	$(MAKE) /NOLOGO -f Make_mvc.mak tiny
	cd ..

testgvimtiny:
	cd testdir
	$(MAKE) /NOLOGO -f Make_mvc.mak tiny VIMPROG=..\gvim
	cd ..

testclean:
	cd testdir
	$(MAKE) /NOLOGO -f Make_mvc.mak clean
	cd ..

# Run individual OLD style test.
# These do not depend on the executable, compile it when needed.
$(SCRIPTS_TINY):
	cd testdir
	- if exist $@.out del $@.out
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) nolog
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) $@.out
	@ if exist test.log ( type test.log & exit /b 1 )
	cd ..

# Run individual NEW style test.
# These do not depend on the executable, compile it when needed.
$(NEW_TESTS):
	cd testdir
	- if exist $@.res del $@.res
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) nolog
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) $@.res
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) report
	cd ..

# Run Vim9 tests.
# These do not depend on the executable, compile it when needed.
test_vim9:
	cd testdir
	-del test_vim9_*.res
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) nolog
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) $(TEST_VIM9_RES)
	$(MAKE) /NOLOGO -f Make_mvc.mak VIMPROG=..\$(VIMTESTTARGET) report
	cd ..

###########################################################################

# Create a default rule for transforming .c files to .obj files in $(OUTDIR)
.c{$(OUTDIR)/}.obj::
	$(CC) $(CFLAGS_OUTDIR) $<

# Create a default rule for xdiff.
{xdiff/}.c{$(OUTDIR)/}.obj::
	$(CC) $(CFLAGS_OUTDIR) $<

# Create a default rule for transforming .cpp files to .obj files in $(OUTDIR)
.cpp{$(OUTDIR)/}.obj::
	$(CC) $(CFLAGS_OUTDIR) $<

$(OUTDIR)/alloc.obj:	$(OUTDIR) alloc.c  $(INCL)

$(OUTDIR)/arabic.obj:	$(OUTDIR) arabic.c  $(INCL)

$(OUTDIR)/arglist.obj:	$(OUTDIR) arglist.c  $(INCL)

$(OUTDIR)/autocmd.obj:	$(OUTDIR) autocmd.c  $(INCL)

$(OUTDIR)/beval.obj:	$(OUTDIR) beval.c  $(INCL)

$(OUTDIR)/blob.obj:	$(OUTDIR) blob.c  $(INCL)

$(OUTDIR)/blowfish.obj:	$(OUTDIR) blowfish.c  $(INCL)

$(OUTDIR)/buffer.obj:	$(OUTDIR) buffer.c  $(INCL) version.h

$(OUTDIR)/bufwrite.obj:	$(OUTDIR) bufwrite.c  $(INCL)

$(OUTDIR)/change.obj:	$(OUTDIR) change.c  $(INCL)

$(OUTDIR)/charset.obj:	$(OUTDIR) charset.c  $(INCL)

$(OUTDIR)/cindent.obj:	$(OUTDIR) cindent.c  $(INCL)

$(OUTDIR)/clientserver.obj:	$(OUTDIR) clientserver.c  $(INCL)

$(OUTDIR)/clipboard.obj:	$(OUTDIR) clipboard.c  $(INCL)

$(OUTDIR)/cmdexpand.obj:	$(OUTDIR) cmdexpand.c  $(INCL)

$(OUTDIR)/cmdhist.obj:	$(OUTDIR) cmdhist.c  $(INCL)

$(OUTDIR)/crypt.obj:	$(OUTDIR) crypt.c  $(INCL)

$(OUTDIR)/crypt_zip.obj: $(OUTDIR) crypt_zip.c  $(INCL)

$(OUTDIR)/debugger.obj:	$(OUTDIR) debugger.c  $(INCL)

$(OUTDIR)/dict.obj:	$(OUTDIR) dict.c  $(INCL)

$(OUTDIR)/diff.obj:	$(OUTDIR) diff.c  $(INCL)

$(OUTDIR)/xdiffi.obj:	$(OUTDIR) xdiff/xdiffi.c  $(XDIFF_DEPS)

$(OUTDIR)/xemit.obj:	$(OUTDIR) xdiff/xemit.c  $(XDIFF_DEPS)

$(OUTDIR)/xprepare.obj:	$(OUTDIR) xdiff/xprepare.c  $(XDIFF_DEPS)

$(OUTDIR)/xutils.obj:	$(OUTDIR) xdiff/xutils.c  $(XDIFF_DEPS)

$(OUTDIR)/xhistogram.obj:	$(OUTDIR) xdiff/xhistogram.c  $(XDIFF_DEPS)

$(OUTDIR)/xpatience.obj:	$(OUTDIR) xdiff/xpatience.c  $(XDIFF_DEPS)

$(OUTDIR)/digraph.obj:	$(OUTDIR) digraph.c  $(INCL)

$(OUTDIR)/drawline.obj:	$(OUTDIR) drawline.c  $(INCL)

$(OUTDIR)/drawscreen.obj:	$(OUTDIR) drawscreen.c  $(INCL)

$(OUTDIR)/edit.obj:	$(OUTDIR) edit.c  $(INCL)

$(OUTDIR)/eval.obj:	$(OUTDIR) eval.c  $(INCL)

$(OUTDIR)/evalbuffer.obj:	$(OUTDIR) evalbuffer.c  $(INCL)

$(OUTDIR)/evalfunc.obj:	$(OUTDIR) evalfunc.c  $(INCL) version.h

$(OUTDIR)/evalvars.obj:	$(OUTDIR) evalvars.c  $(INCL) version.h

$(OUTDIR)/evalwindow.obj:	$(OUTDIR) evalwindow.c  $(INCL)

$(OUTDIR)/ex_cmds.obj:	$(OUTDIR) ex_cmds.c  $(INCL) version.h

$(OUTDIR)/ex_cmds2.obj:	$(OUTDIR) ex_cmds2.c  $(INCL) version.h

$(OUTDIR)/ex_docmd.obj:	$(OUTDIR) ex_docmd.c  $(INCL) ex_cmdidxs.h

$(OUTDIR)/ex_eval.obj:	$(OUTDIR) ex_eval.c  $(INCL)

$(OUTDIR)/ex_getln.obj:	$(OUTDIR) ex_getln.c  $(INCL)

$(OUTDIR)/fileio.obj:	$(OUTDIR) fileio.c  $(INCL)

$(OUTDIR)/filepath.obj:	$(OUTDIR) filepath.c  $(INCL)

$(OUTDIR)/findfile.obj:	$(OUTDIR) findfile.c  $(INCL)

$(OUTDIR)/float.obj:	$(OUTDIR) float.c  $(INCL)

$(OUTDIR)/fold.obj:	$(OUTDIR) fold.c  $(INCL)

$(OUTDIR)/getchar.obj:	$(OUTDIR) getchar.c  $(INCL)

$(OUTDIR)/gui_xim.obj:	$(OUTDIR) gui_xim.c  $(INCL)

$(OUTDIR)/hardcopy.obj:	$(OUTDIR) hardcopy.c  $(INCL) version.h

$(OUTDIR)/hashtab.obj:	$(OUTDIR) hashtab.c  $(INCL)

$(OUTDIR)/help.obj:	$(OUTDIR) help.c  $(INCL)

$(OUTDIR)/highlight.obj:	$(OUTDIR) highlight.c  $(INCL)

$(OUTDIR)/indent.obj:	$(OUTDIR) indent.c  $(INCL)

$(OUTDIR)/insexpand.obj:	$(OUTDIR) insexpand.c  $(INCL)

$(OUTDIR)/gui.obj:	$(OUTDIR) gui.c  $(INCL) $(GUI_INCL)

$(OUTDIR)/gui_beval.obj:	$(OUTDIR) gui_beval.c $(INCL) $(GUI_INCL)

$(OUTDIR)/gui_w32.obj:	$(OUTDIR) gui_w32.c $(INCL) $(GUI_INCL) version.h

$(OUTDIR)/gui_dwrite.obj:	$(OUTDIR) gui_dwrite.cpp gui_dwrite.h

$(OUTDIR)/if_cscope.obj: $(OUTDIR) if_cscope.c  $(INCL)

$(OUTDIR)/if_lua.obj: $(OUTDIR) if_lua.c  $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(LUA_INC) if_lua.c

auto/if_perl.c : if_perl.xs typemap
	-if not exist auto/nul mkdir auto
	$(XSUBPP) -prototypes -typemap $(XSUBPP_TYPEMAP) \
		-typemap typemap if_perl.xs -output $@

$(OUTDIR)/if_perl.obj: $(OUTDIR) auto/if_perl.c  $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(PERL_INC) auto/if_perl.c

$(OUTDIR)/if_perlsfio.obj: $(OUTDIR) if_perlsfio.c  $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(PERL_INC) if_perlsfio.c

$(OUTDIR)/if_mzsch.obj: $(OUTDIR) if_mzsch.c $(MZSCHEME_INCL) $(INCL) $(MZSCHEME_EXTRA_DEP)
	$(CC) $(CFLAGS_OUTDIR) if_mzsch.c \
		-DMZSCHEME_COLLECTS="\"$(MZSCHEME_COLLECTS:\=\\)\""

lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).lib:
	lib /DEF:"$(MZSCHEME)\lib\lib$(MZSCHEME_MAIN_LIB)$(MZSCHEME_VER).def"

$(OUTDIR)/if_python.obj: $(OUTDIR) if_python.c if_py_both.h $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(PYTHON_INC) if_python.c

$(OUTDIR)/if_python3.obj: $(OUTDIR) if_python3.c if_py_both.h $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(PYTHON3_INC) if_python3.c

$(OUTDIR)/if_ole.obj: $(OUTDIR) if_ole.cpp  $(INCL) if_ole.h

$(OUTDIR)/if_ruby.obj: $(OUTDIR) if_ruby.c  $(INCL) version.h
	$(CC) $(CFLAGS_OUTDIR) $(RUBY_INC) if_ruby.c

$(OUTDIR)/if_tcl.obj: $(OUTDIR) if_tcl.c  $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(TCL_INC) if_tcl.c

$(OUTDIR)/iscygpty.obj:	$(OUTDIR) iscygpty.c $(CUI_INCL)
	$(CC) $(CFLAGS_OUTDIR) iscygpty.c

$(OUTDIR)/job.obj:	$(OUTDIR) job.c $(INCL)

$(OUTDIR)/json.obj:	$(OUTDIR) json.c  $(INCL)

$(OUTDIR)/list.obj:	$(OUTDIR) list.c  $(INCL)

$(OUTDIR)/locale.obj:	$(OUTDIR) locale.c  $(INCL)

$(OUTDIR)/logfile.obj:	$(OUTDIR) logfile.c  $(INCL)

$(OUTDIR)/main.obj:	$(OUTDIR) main.c  $(INCL) $(CUI_INCL)

$(OUTDIR)/map.obj:	$(OUTDIR) map.c  $(INCL)

$(OUTDIR)/mark.obj:	$(OUTDIR) mark.c  $(INCL)

$(OUTDIR)/match.obj:	$(OUTDIR) match.c  $(INCL)

$(OUTDIR)/memfile.obj:	$(OUTDIR) memfile.c  $(INCL)

$(OUTDIR)/memline.obj:	$(OUTDIR) memline.c  $(INCL)

$(OUTDIR)/menu.obj:	$(OUTDIR) menu.c  $(INCL)

$(OUTDIR)/message.obj:	$(OUTDIR) message.c  $(INCL)

$(OUTDIR)/misc1.obj:	$(OUTDIR) misc1.c  $(INCL) version.h

$(OUTDIR)/misc2.obj:	$(OUTDIR) misc2.c  $(INCL)

$(OUTDIR)/mouse.obj:	$(OUTDIR) mouse.c  $(INCL)

$(OUTDIR)/move.obj:	$(OUTDIR) move.c  $(INCL)

$(OUTDIR)/mbyte.obj:	$(OUTDIR) mbyte.c  $(INCL)

$(OUTDIR)/netbeans.obj:	$(OUTDIR) netbeans.c $(NBDEBUG_SRC) $(INCL) version.h

$(OUTDIR)/channel.obj:	$(OUTDIR) channel.c $(INCL)

$(OUTDIR)/normal.obj:	$(OUTDIR) normal.c  $(INCL) nv_cmdidxs.h nv_cmds.h

$(OUTDIR)/option.obj:	$(OUTDIR) option.c  $(INCL) optiondefs.h

$(OUTDIR)/optionstr.obj:	$(OUTDIR) optionstr.c  $(INCL)

$(OUTDIR)/ops.obj:	$(OUTDIR) ops.c  $(INCL)

$(OUTDIR)/os_mswin.obj:	$(OUTDIR) os_mswin.c  $(INCL)

$(OUTDIR)/terminal.obj:	$(OUTDIR) terminal.c  $(INCL) $(TERM_DEPS)

$(OUTDIR)/winclip.obj:	$(OUTDIR) winclip.c  $(INCL)

$(OUTDIR)/os_win32.obj:	$(OUTDIR) os_win32.c  $(INCL) $(MZSCHEME_INCL)

$(OUTDIR)/os_w32dll.obj:	$(OUTDIR) os_w32dll.c

$(OUTDIR)/os_w32exe.obj:	$(OUTDIR) os_w32exe.c  $(INCL)

$(OUTDIR)/os_w32exec.obj:	$(OUTDIR) os_w32exe.c  $(INCL)
	$(CC) $(CFLAGS:-DFEAT_GUI_MSWIN=) /Fo$@ os_w32exe.c

$(OUTDIR)/os_w32exeg.obj:	$(OUTDIR) os_w32exe.c  $(INCL)
	$(CC) $(CFLAGS) /Fo$@ os_w32exe.c

$(OUTDIR)/pathdef.obj:	$(OUTDIR) $(PATHDEF_SRC) $(INCL)
	$(CC) $(CFLAGS_OUTDIR) $(PATHDEF_SRC)

$(OUTDIR)/popupmenu.obj:	$(OUTDIR) popupmenu.c  $(INCL)

$(OUTDIR)/popupwin.obj:	$(OUTDIR) popupwin.c  $(INCL)

$(OUTDIR)/profiler.obj:	$(OUTDIR) profiler.c  $(INCL)

$(OUTDIR)/quickfix.obj:	$(OUTDIR) quickfix.c  $(INCL)

$(OUTDIR)/regexp.obj:	$(OUTDIR) regexp.c regexp_bt.c regexp_nfa.c  $(INCL)

$(OUTDIR)/register.obj:	$(OUTDIR) register.c $(INCL)

$(OUTDIR)/scriptfile.obj:	$(OUTDIR) scriptfile.c  $(INCL)

$(OUTDIR)/screen.obj:	$(OUTDIR) screen.c  $(INCL)

$(OUTDIR)/search.obj:	$(OUTDIR) search.c  $(INCL)

$(OUTDIR)/session.obj:	$(OUTDIR) session.c  $(INCL)

$(OUTDIR)/sha256.obj:	$(OUTDIR) sha256.c  $(INCL)

$(OUTDIR)/sign.obj:	$(OUTDIR) sign.c  $(INCL)

$(OUTDIR)/spell.obj:	$(OUTDIR) spell.c  $(INCL)

$(OUTDIR)/spellfile.obj:	$(OUTDIR) spellfile.c  $(INCL)

$(OUTDIR)/spellsuggest.obj:	$(OUTDIR) spellsuggest.c  $(INCL)

$(OUTDIR)/strings.obj:	$(OUTDIR) strings.c  $(INCL)

$(OUTDIR)/syntax.obj:	$(OUTDIR) syntax.c  $(INCL)

$(OUTDIR)/tag.obj:	$(OUTDIR) tag.c  $(INCL)

$(OUTDIR)/term.obj:	$(OUTDIR) term.c  $(INCL)

$(OUTDIR)/term.obj:	$(OUTDIR) testing.c  $(INCL)

$(OUTDIR)/textformat.obj:	$(OUTDIR) textformat.c  $(INCL)

$(OUTDIR)/textobject.obj:	$(OUTDIR) textobject.c  $(INCL)

$(OUTDIR)/textprop.obj:	$(OUTDIR) textprop.c  $(INCL)

$(OUTDIR)/time.obj:	$(OUTDIR) time.c  $(INCL)

$(OUTDIR)/typval.obj:	$(OUTDIR) typval.c  $(INCL)

$(OUTDIR)/ui.obj:	$(OUTDIR) ui.c  $(INCL)

$(OUTDIR)/undo.obj:	$(OUTDIR) undo.c  $(INCL)

$(OUTDIR)/usercmd.obj:	$(OUTDIR) usercmd.c  $(INCL)

$(OUTDIR)/userfunc.obj:	$(OUTDIR) userfunc.c  $(INCL)

$(OUTDIR)/version.obj:	$(OUTDIR) version.c  $(INCL) version.h

$(OUTDIR)/vim9class.obj:	$(OUTDIR) vim9class.c  $(INCL) vim9.h

$(OUTDIR)/vim9cmds.obj:	$(OUTDIR) vim9cmds.c  $(INCL) vim9.h

$(OUTDIR)/vim9compile.obj:	$(OUTDIR) vim9compile.c  $(INCL) vim9.h

$(OUTDIR)/vim9execute.obj:	$(OUTDIR) vim9execute.c  $(INCL) vim9.h

$(OUTDIR)/vim9expr.obj:	$(OUTDIR) vim9expr.c  $(INCL) vim9.h

$(OUTDIR)/vim9instr.obj:	$(OUTDIR) vim9instr.c  $(INCL) vim9.h

$(OUTDIR)/vim9script.obj:	$(OUTDIR) vim9script.c  $(INCL) vim9.h

$(OUTDIR)/vim9type.obj:	$(OUTDIR) vim9type.c  $(INCL) vim9.h

$(OUTDIR)/viminfo.obj:	$(OUTDIR) viminfo.c  $(INCL) version.h

$(OUTDIR)/window.obj:	$(OUTDIR) window.c  $(INCL)

$(OUTDIR)/xpm_w32.obj: $(OUTDIR) xpm_w32.c
	$(CC) $(CFLAGS_OUTDIR) $(XPM_INC) xpm_w32.c

!if "$(VIMDLL)" == "yes"
$(OUTDIR)/vimc.res:	$(OUTDIR) vim.rc vim.manifest version.h gui_w32_rc.h \
				vim.ico
	$(RC) /nologo /l 0x409 /Fo$@ $(RCFLAGS:-DFEAT_GUI_MSWIN=) vim.rc

$(OUTDIR)/vimg.res:	$(OUTDIR) vim.rc vim.manifest version.h gui_w32_rc.h \
				vim.ico
	$(RC) /nologo /l 0x409 /Fo$@ $(RCFLAGS) vim.rc

$(OUTDIR)/vimd.res:	$(OUTDIR) vim.rc version.h gui_w32_rc.h \
				tools.bmp tearoff.bmp vim.ico vim_error.ico \
				vim_alert.ico vim_info.ico vim_quest.ico
	$(RC) /nologo /l 0x409 /Fo$@ $(RCFLAGS) -DRCDLL -DVIMDLLBASE=\"$(VIMDLLBASE)\" vim.rc
!else
$(OUTDIR)/vim.res:	$(OUTDIR) vim.rc vim.manifest version.h gui_w32_rc.h \
				tools.bmp tearoff.bmp vim.ico vim_error.ico \
				vim_alert.ico vim_info.ico vim_quest.ico
	$(RC) /nologo /l 0x409 /Fo$@ $(RCFLAGS) vim.rc
!endif

iid_ole.c if_ole.h vim.tlb: if_ole.idl
	midl /nologo /error none /proxy nul /iid iid_ole.c /tlb vim.tlb \
		/header if_ole.h if_ole.idl


CCCTERM = $(CC) $(CFLAGS) -Ilibvterm/include -DINLINE="" \
	-DVSNPRINTF=vim_vsnprintf \
	-DSNPRINTF=vim_snprintf \
	-DIS_COMBINING_FUNCTION=utf_iscomposing_uint \
	-DWCWIDTH_FUNCTION=utf_uint2cells \
	-DGET_SPECIAL_PTY_TYPE_FUNCTION=get_special_pty_type \
	-D_CRT_SECURE_NO_WARNINGS

$(OUTDIR)/vterm_encoding.obj: $(OUTDIR) libvterm/src/encoding.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/encoding.c

$(OUTDIR)/vterm_keyboard.obj: $(OUTDIR) libvterm/src/keyboard.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/keyboard.c

$(OUTDIR)/vterm_mouse.obj: $(OUTDIR) libvterm/src/mouse.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/mouse.c

$(OUTDIR)/vterm_parser.obj: $(OUTDIR) libvterm/src/parser.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/parser.c

$(OUTDIR)/vterm_pen.obj: $(OUTDIR) libvterm/src/pen.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/pen.c

$(OUTDIR)/vterm_screen.obj: $(OUTDIR) libvterm/src/screen.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/screen.c

$(OUTDIR)/vterm_state.obj: $(OUTDIR) libvterm/src/state.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/state.c

$(OUTDIR)/vterm_unicode.obj: $(OUTDIR) libvterm/src/unicode.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/unicode.c

$(OUTDIR)/vterm_vterm.obj: $(OUTDIR) libvterm/src/vterm.c $(TERM_DEPS)
	$(CCCTERM) /Fo$@ libvterm/src/vterm.c


# $CFLAGS may contain backslashes, quotes and chevrons, escape them all.
E0_CFLAGS = $(CFLAGS:\=\\)
E00_CFLAGS = $(E0_CFLAGS:"=\")
# ") stop the string
E000_CFLAGS = $(E00_CFLAGS:<=^^<)
E_CFLAGS = $(E000_CFLAGS:>=^^>)
# $LINKARGS2 may contain backslashes, quotes and chevrons, escape them all.
E0_LINKARGS2 = $(LINKARGS2:\=\\)
E00_LINKARGS2 = $(E0_LINKARGS2:"=\")
# ") stop the string
E000_LINKARGS2 = $(E00_LINKARGS2:<=^^<)
E_LINKARGS2 = $(E000_LINKARGS2:>=^^>)

$(PATHDEF_SRC): Make_mvc.mak
	@echo creating $(PATHDEF_SRC)
	@echo /* pathdef.c */ > $(PATHDEF_SRC)
	@echo #include "vim.h" >> $(PATHDEF_SRC)
	@echo char_u *default_vim_dir = (char_u *)"$(VIMRCLOC:\=\\)"; >> $(PATHDEF_SRC)
	@echo char_u *default_vimruntime_dir = (char_u *)"$(VIMRUNTIMEDIR:\=\\)"; >> $(PATHDEF_SRC)
	@echo char_u *all_cflags = (char_u *)"$(CC:\=\\) $(E_CFLAGS)"; >> $(PATHDEF_SRC)
	@echo char_u *all_lflags = (char_u *)"$(LINK:\=\\) $(LINKARGS1:\=\\) $(E_LINKARGS2)"; >> $(PATHDEF_SRC)
	@echo char_u *compiled_user = (char_u *)"$(USERNAME)"; >> $(PATHDEF_SRC)
	@echo char_u *compiled_sys = (char_u *)"$(USERDOMAIN)"; >> $(PATHDEF_SRC)

# End Custom Build
proto.h: \
	proto/alloc.pro \
	proto/arabic.pro \
	proto/arglist.pro \
	proto/autocmd.pro \
	proto/blob.pro \
	proto/blowfish.pro \
	proto/buffer.pro \
	proto/bufwrite.pro \
	proto/change.pro \
	proto/charset.pro \
	proto/cindent.pro \
	proto/clientserver.pro \
	proto/clipboard.pro \
	proto/cmdexpand.pro \
	proto/cmdhist.pro \
	proto/crypt.pro \
	proto/crypt_zip.pro \
	proto/debugger.pro \
	proto/dict.pro \
	proto/diff.pro \
	proto/digraph.pro \
	proto/drawline.pro \
	proto/drawscreen.pro \
	proto/edit.pro \
	proto/eval.pro \
	proto/evalbuffer.pro \
	proto/evalfunc.pro \
	proto/evalvars.pro \
	proto/evalwindow.pro \
	proto/ex_cmds.pro \
	proto/ex_cmds2.pro \
	proto/ex_docmd.pro \
	proto/ex_eval.pro \
	proto/ex_getln.pro \
	proto/fileio.pro \
	proto/filepath.pro \
	proto/findfile.pro \
	proto/float.pro \
	proto/getchar.pro \
	proto/gui_xim.pro \
	proto/hardcopy.pro \
	proto/hashtab.pro \
	proto/help.pro \
	proto/highlight.pro \
	proto/indent.pro \
	proto/insexpand.pro \
	proto/json.pro \
	proto/list.pro \
	proto/locale.pro \
	proto/logfile.pro \
	proto/main.pro \
	proto/map.pro \
	proto/mark.pro \
	proto/match.pro \
	proto/memfile.pro \
	proto/memline.pro \
	proto/menu.pro \
	proto/message.pro \
	proto/misc1.pro \
	proto/misc2.pro \
	proto/mouse.pro \
	proto/move.pro \
	proto/mbyte.pro \
	proto/normal.pro \
	proto/ops.pro \
	proto/option.pro \
	proto/optionstr.pro \
	proto/os_mswin.pro \
	proto/winclip.pro \
	proto/os_win32.pro \
	proto/popupmenu.pro \
	proto/popupwin.pro \
	proto/profiler.pro \
	proto/quickfix.pro \
	proto/regexp.pro \
	proto/register.pro \
	proto/scriptfile.pro \
	proto/screen.pro \
	proto/search.pro \
	proto/session.pro \
	proto/sha256.pro \
	proto/sign.pro \
	proto/spell.pro \
	proto/spellfile.pro \
	proto/spellsuggest.pro \
	proto/strings.pro \
	proto/syntax.pro \
	proto/tag.pro \
	proto/term.pro \
	proto/testing.pro \
	proto/textformat.pro \
	proto/textobject.pro \
	proto/textprop.pro \
	proto/time.pro \
	proto/typval.pro \
	proto/ui.pro \
	proto/undo.pro \
	proto/usercmd.pro \
	proto/userfunc.pro \
	proto/vim9class.pro \
	proto/vim9cmds.pro \
	proto/vim9compile.pro \
	proto/vim9execute.pro \
	proto/vim9expr.pro \
	proto/vim9instr.pro \
	proto/vim9script.pro \
	proto/vim9type.pro \
	proto/viminfo.pro \
	proto/window.pro \
	$(SOUND_PRO) \
	$(NETBEANS_PRO) \
	$(CHANNEL_PRO)

.SUFFIXES: .cod .i

# Generate foo.cod (mixed source and assembly listing) from foo.c via "nmake
# foo.cod"
.c.cod:
	$(CC) $(CFLAGS) /FAcs $<

# Generate foo.i (preprocessor listing) from foo.c via "nmake foo.i"
.c.i:
	$(CC) $(CFLAGS) /P /C $<

# vim: set noet sw=8 ts=8 sts=0 wm=0 tw=0:
