#
# Makefile for Vim on OpenVMS
#
# Maintainer:   Zoltan Arpadffy <zoltan.arpadffy@gmail.com>
# Last change:  2024 Jan 03
#
# This script has been tested on VMS 6.2 to 9.2 on VAX, ALPHA, IA64 and X86_64
# with MMS and MMK
#
# The following could be built:
#	vim.exe:	standard (terminal, GUI/Motif, GUI/GTK)
#	dvim.exe:	debug
#
# Edit the lines in the Configuration section below for fine tuning.
#
# To build:    mms/descrip=Make_vms.mms /ignore=warning
# To clean up: mms/descrip=Make_vms.mms clean
#
# Hints and detailed description could be found in INSTALLVMS.TXT file.
#
######################################################################
# Configuration section.
######################################################################

# Compiler selection.
# Comment out if you use the VAXC compiler
DECC = YES

# Build model selection
# TINY   - No optional features enabled
# NORMAL - A default selection of features enabled
# HUGE   - All possible features enabled.
# Please select one of these alternatives above.
MODEL = HUGE

# GUI or terminal mode executable.
# Comment out if you want just the character terminal mode only.
# GUI with Motif
# GUI = YES

# GUI with GTK
# If you have GTK installed you might want to enable this option.
# NOTE: you will need to properly define GTK_DIR below
# NOTE: since Vim 7.3 GTK 2+ is used that is not ported to VMS,
#       therefore this option should not be used
# GTK = YES

# GUI/Motif with XPM
# If you have XPM installed you might want to build Motif version with toolbar
# XPM = YES

# Comment out if you want the compiler version with :ver command.
# NOTE: This part can make some complications if you're using some
# predefined symbols/flags for your compiler. If does, just leave behind
# the comment variable CCVER.
CCVER = YES

# Uncomment if want a debug version. Resulting executable is DVIM.EXE
# Development purpose only! Normally, it should not be defined. !!!
# DEBUG = YES

# Languages support for Perl, Python, TCL etc.
# If you don't need it really, leave them behind the comment.
# You will need related libraries, include files etc.
# VIM_TCL    = YES
# VIM_PERL   = YES
# VIM_PYTHON = YES
# VIM_PYTHON3= YES
# VIM_RUBY   = YES
# VIM_LUA    = YES

# X Input Method.  For entering special languages like chinese and
# Japanese.
# If you don't need it really, leave it behind the comment.
# VIM_XIM = YES

# Allow any white space to separate the fields in a tags file
# When not defined, only a TAB is allowed.
# VIM_TAG_ANYWHITE = YES

# Allow FEATURE_MZSCHEME
# VIM_MZSCHEME = YES

# Use ICONV
# VIM_ICONV = YES

# If you modified the source code and plan to distribute the build
# please, let the users know that.
# MODIFIED_BY = "name surname <your@email.com>"

######################################################################
# Directory, library and include files configuration section.
# Normally you need not to change anything below. !
# These may need to be defined if things are not in standard locations
#
# You can find some explanation in INSTALLVMS.TXT
######################################################################

# Compiler setup

.IFDEF MMSVAX
.IFDEF DECC	     # VAX with DECC
CC_DEF  = cc # /decc # some versions require /decc switch but when it is not required /ver might fail
PREFIX  = /prefix=all/name=(upper,short)
OPTIMIZE= /noopt     # do not optimize on VAX. The compiler has hard time with crypto functions
.ELSE		     # VAX with VAXC
CC_DEF	= cc
PREFIX	=
OPTIMIZE= /noopt
CCVER	=
.ENDIF
.ELSE		     # AXP, IA64, X86 with DECC
CC_DEF  = cc
PREFIX  = /prefix=all/name=(upper,short)
OPTIMIZE= /opt
.IFDEF MMSX86_64
ARCH_DEF=        # ,__CRTL_VER_OVERRIDE=80400000
.ENDIF
.ENDIF

LD_DEF  = link
C_INC   = [.proto]

.IFDEF DEBUG
DEBUG_DEF = ,"DEBUG"
TARGET    = dvim.exe
CFLAGS    = /debug/noopt$(PREFIX)
LDFLAGS   = /debug
.ELSE
TARGET    = vim.exe
CFLAGS    = $(OPTIMIZE)$(PREFIX)
LDFLAGS   =
.ENDIF

# Predefined VIM directories
# Please, use $VIM and $VIMRUNTIME logicals instead
VIMLOC  = ""
VIMRUN  = ""

CONFIG_H = os_vms_conf.h

# GTK or XPM but not both
.IFDEF GTK
.IFDEF GUI
.ELSE
GUI = YES
.ENDIF
.IFDEF XPM
XPM = ""
.ENDIF
.ENDIF

.IFDEF XPM
.IFDEF GUI
.ELSE
GUI = YES
.ENDIF
.IFDEF GTK
GTK = ""
.ENDIF
.ENDIF

.IFDEF GUI
# X/Motif/GTK executable  (also works in terminal mode )

.IFDEF GTK
# NOTE: you need to set up your GTK_DIR (GTK root directory), because it is
# unique on every system - logicals are not accepted
# please note: directory should end with . in order to /trans=conc work
# This value for GTK_DIR is an example.
GTK_DIR  = DKA0:[WORK.GTK1210.]
DEFS     = ,"HAVE_CONFIG_H","FEAT_GUI_GTK"
LIBS     = ,OS_VMS_GTK.OPT/OPT
GUI_FLAG = /float=ieee/ieee=denorm/WARNINGS=(DISABLE=MACROREDEF)
GUI_SRC  = gui.c gui_gtk.c gui_gtk_f.c gui_gtk_x11.c gui_beval.c pty.c
GUI_OBJ  = gui.obj gui_gtk.obj gui_gtk_f.obj gui_gtk_x11.obj gui_beval.obj pty.obj
GUI_INC  = ,"/gtk_root/gtk","/gtk_root/glib"
# GUI_INC_VER is used just for :ver information
# this string should escape from C and DCL in the same time
GUI_INC_VER= ,\""/gtk_root/gtk\"",\""/gtk_root/glib\""
.ELSE
MOTIF	 = YES
.IFDEF XPM
DEFS     = ,"HAVE_CONFIG_H","FEAT_GUI_MOTIF","HAVE_XPM"
XPM_INC  = ,[.xpm.include]
XPM_LIB  = ,OS_VMS_XPM.OPT/OPT
.ELSE
DEFS     = ,"HAVE_CONFIG_H","FEAT_GUI_MOTIF"
XPM_INC  =
.ENDIF
LIBS     = ,OS_VMS_MOTIF.OPT/OPT
GUI_FLAG = /WARNINGS=(DISABLE=MACROREDEF)
GUI_SRC  = gui.c gui_motif.c gui_x11.c gui_beval.c gui_xmdlg.c gui_xmebw.c
GUI_OBJ  = gui.obj gui_motif.obj gui_x11.obj gui_beval.obj gui_xmdlg.obj gui_xmebw.obj
GUI_INC  =
.ENDIF

# You need to define these variables if you do not have DECW files
# at standard location
GUI_INC_DIR = ,decw$include:
# GUI_LIB_DIR = ,sys$library:

.ELSE
# Character terminal only executable
DEFS	 = ,"HAVE_CONFIG_H"
LIBS	 =
.ENDIF

.IFDEF VIM_PERL
# Perl related setup.
PERL	 = perl
PERL_DEF = ,"FEAT_PERL"
PERL_SRC = if_perlsfio.c if_perl.xs
PERL_OBJ = if_perlsfio.obj if_perl.obj
PERL_LIB = ,OS_VMS_PERL.OPT/OPT
PERL_INC = ,dka0:[perlbuild.perl.lib.vms_axp.5_6_1.core]
.ENDIF

.IFDEF VIM_PYTHON
# Python related setup.
PYTHON_DEF = ,"FEAT_PYTHON"
PYTHON_SRC = if_python.c
PYTHON_OBJ = if_python.obj
PYTHON_LIB = ,OS_VMS_PYTHON.OPT/OPT
PYTHON_INC = ,PYTHON_INCLUDE
.ENDIF

.IFDEF VIM_PYTHON3
# Python related setup.
PYTHON3_DEF = ,"FEAT_PYTHON3"
PYTHON3_SRC = if_python3.c
PYTHON3_OBJ = if_python3.obj
PYTHON3_LIB = ,OS_VMS_PYTHON3.OPT/OPT
PYTHON3_INC = ,PYTHON3_INCLUDE
.ENDIF


.IFDEF VIM_TCL
# TCL related setup.
TCL_DEF = ,"FEAT_TCL"
TCL_SRC = if_tcl.c
TCL_OBJ = if_tcl.obj
TCL_LIB = ,OS_VMS_TCL.OPT/OPT
TCL_INC = ,dka0:[tcl80.generic]
.ENDIF

.IFDEF VIM_RUBY
# RUBY related setup.
RUBY_DEF = ,"FEAT_RUBY"
RUBY_SRC = if_ruby.c
RUBY_OBJ = if_ruby.obj
RUBY_LIB = ,OS_VMS_RUBY.OPT/OPT
RUBY_INC =
.ENDIF

.IFDEF VIM_LUA
# LUA related setup.
LUA_DEF = ,"FEAT_LUA"
LUA_SRC = if_lua.c
LUA_OBJ = if_lua.obj
LUA_LIB = ,OS_VMS_LUA.OPT/OPT
LUA_INC = ,LUA$ROOT:[INCLUDE]
.ENDIF

.IFDEF VIM_XIM
# XIM related setup.
.IFDEF GUI
XIM_DEF = ,"FEAT_XIM"
.ENDIF
.ENDIF

.IFDEF VIM_MZSCHEME
# MZSCHEME related setup
MZSCHEME_DEF = ,"FEAT_MZSCHEME"
MZSCHEME_SRC = if_mzsch.c
MZSCHEME_OBJ = if_mzsch.obj
.ENDIF

.IFDEF VIM_ICONV
# ICONV related setup
ICONV_DEF = ,"USE_ICONV"
.ENDIF

# XDIFF related setup.
XDIFF_SRC = xdiffi.c,xemit.c,xprepare.c,xutils.c,xhistogram.c,xpatience.c
XDIFF_OBJ = xdiffi.obj,xemit.obj,xprepare.obj,xutils.obj,xhistogram.obj,xpatience.obj
XDIFF_INC = ,[.xdiff]

.IFDEF MODIFIED_BY
DEF_MODIFIED = YES
.ELSE
DEF_MODIFIED = NO
.ENDIF

######################################################################
# End of configuration section.
# Please, do not change anything below without programming experience.
######################################################################

MODEL_DEF = "FEAT_$(MODEL)"

# These go into pathdef.c
VIMUSER = "''F$EDIT(F$GETJPI(" ","USERNAME"),"TRIM")'"
VIMHOST = "''F$TRNLNM("SYS$NODE")'''F$TRNLNM("UCX$INET_HOST")'.''F$TRNLNM("UCX$INET_DOMAIN")'"

.SUFFIXES : .obj .c

ALL_CFLAGS = /def=($(MODEL_DEF)$(DEFS)$(DEBUG_DEF)$(PERL_DEF)$(PYTHON_DEF)$(PYTHON3_DEF) -
 $(TCL_DEF)$(RUBY_DEF)$(LUA_DEF)$(XIM_DEF)$(TAG_DEF)$(MZSCHEME_DEF) -
 $(ICONV_DEF)$(ARCH_DEF)) -
 $(CFLAGS)$(GUI_FLAG) -
 /include=($(C_INC)$(GUI_INC_DIR)$(GUI_INC)$(PERL_INC)$(PYTHON_INC)$(PYTHON3_INC) -
 $(TCL_INC)$(XDIFF_INC)$(XPM_INC))

# CFLAGS displayed in :ver information
# It is specially formatted for correct display of unix like includes
# as $(GUI_INC) - replaced with $(GUI_INC_VER)
# Otherwise should not be any other difference.
ALL_CFLAGS_VER = /def=($(MODEL_DEF)$(DEFS)$(DEBUG_DEF)$(PERL_DEF)$(PYTHON_DEF)$(PYTHON3_DEF) -
 $(TCL_DEF)$(RUBY_DEF)$(LUA_DEF)$(XIM_DEF)$(TAG_DEF)$(MZSCHEME_DEF) -
 $(ICONV_DEF)$(ARCH_DEF)) -
 $(CFLAGS)$(GUI_FLAG) -
 /include=($(C_INC)$(GUI_INC_DIR)$(GUI_INC_VER)$(PERL_INC)$(PYTHON_INC)$(PYTHON3_INC) -
 $(TCL_INC)$(XDIFF_INC)$(XPM_INC))

ALL_LIBS = $(LIBS) $(GUI_LIB_DIR) $(GUI_LIB) $(XPM_LIB)\
	   $(PERL_LIB) $(PYTHON_LIB) $(PYTHON3_LIB) $(TCL_LIB) $(RUBY_LIB) $(LUA_LIB)

SRC = \
	alloc.c \
	arabic.c \
	arglist.c \
	autocmd.c \
	beval.c \
	blob.c \
	blowfish.c \
	buffer.c \
	bufwrite.c \
	change.c \
	channel.c \
	charset.c \
	cindent.c \
	clientserver.c \
	clipboard.c \
	cmdexpand.c \
	cmdhist.c \
	crypt.c \
	crypt_zip.c \
	debugger.c \
	dict.c \
	diff.c \
	digraph.c \
	drawline.c \
	drawscreen.c \
	edit.c \
	eval.c \
	evalbuffer.c \
	evalfunc.c \
	evalvars.c \
	evalwindow.c \
	ex_cmds.c \
	ex_cmds2.c \
	ex_docmd.c \
	ex_eval.c \
	ex_getln.c \
	fileio.c \
	filepath.c, \
	findfile.c \
	float.c \
	fold.c \
	getchar.c \
	gui_xim.c \
	hardcopy.c \
	hashtab.c \
	help.c \
	highlight.c \
	if_cscope.c \
	if_xcmdsrv.c \
	indent.c \
	insexpand.c \
	job.c \
	json.c \
	list.c \
	locale.c \
	logfile.c \
	main.c \
	map.c \
	mark.c \
	match.c \
	mbyte.c \
	memfile.c \
	memline.c \
	menu.c \
	message.c \
	misc1.c \
	misc2.c \
	mouse.c \
	move.c \
	normal.c \
	ops.c \
	option.c \
	optionstr.c \
	os_unix.c \
	os_vms.c \
	pathdef.c \
	popupmenu.c \
	popupwin.c \
	profiler.c \
	pty.c \
	quickfix.c \
	regexp.c \
	register.c \
	screen.c \
	scriptfile.c \
	search.c \
	session.c \
	sha256.c \
	sign.c \
	sound.c \
	spell.c \
	spellfile.c \
	spellsuggest.c \
	strings.c \
	syntax.c \
	tag.c \
	term.c \
	terminal.c \
	termlib.c \
	testing.c \
	textformat.c \
	textobject.c \
	textprop.c \
	time.c \
	typval.c \
	ui.c \
	undo.c \
	usercmd.c \
	userfunc.c \
	version.c \
	vim9class.c \
	vim9cmds.c \
	vim9compile.c \
	vim9execute.c \
	vim9expr.c \
	vim9instr.c \
	vim9script.c \
	vim9type.c \
	viminfo.c \
	window.c \
	$(GUI_SRC) \
	$(XDIFF_SRC) \
	$(LUA_SRC) \
	$(MZSCHEME_SRC) \
	$(PERL_SRC) \
	$(PYTHON_SRC) \
	$(PYTHON3_SRC) \
	$(TCL_SRC) \
	$(RUBY_SRC)

OBJ = \
	alloc.obj \
	arabic.obj \
	arglist.obj \
	autocmd.obj \
	beval.obj \
	blob.obj \
	blowfish.obj \
	buffer.obj \
	bufwrite.obj \
	change.obj \
	channel.obj \
	charset.obj \
	cindent.obj \
	clientserver.obj \
	clipboard.obj \
	cmdexpand.obj \
	cmdhist.obj \
	crypt.obj \
	crypt_zip.obj \
	debugger.obj \
	dict.obj \
	diff.obj \
	digraph.obj \
	drawline.obj \
	drawscreen.obj \
	edit.obj \
	eval.obj \
	evalbuffer.obj \
	evalfunc.obj \
	evalvars.obj \
	evalwindow.obj \
	ex_cmds.obj \
	ex_cmds2.obj \
	ex_docmd.obj \
	ex_eval.obj \
	ex_getln.obj \
	fileio.obj \
	filepath.obj \
	findfile.obj \
	float.obj \
	fold.obj \
	getchar.obj \
	gui_xim.obj \
	hardcopy.obj \
	hashtab.obj \
	help.obj \
	highlight.obj \
	if_cscope.obj \
	if_mzsch.obj \
	if_xcmdsrv.obj \
	indent.obj \
	insexpand.obj \
	job.obj \
	json.obj \
	list.obj \
	locale.obj \
	logfile.obj \
	main.obj \
	map.obj \
	mark.obj \
	match.obj \
	mbyte.obj \
	memfile.obj \
	memline.obj \
	menu.obj \
	message.obj \
	misc1.obj \
	misc2.obj \
	mouse.obj \
	move.obj \
	normal.obj \
	ops.obj \
	option.obj \
	optionstr.obj \
	os_unix.obj \
	os_vms.obj \
	pathdef.obj \
	popupmenu.obj \
	popupwin.obj \
	profiler.obj \
	pty.obj \
	quickfix.obj \
	regexp.obj \
	register.obj \
	screen.obj \
	scriptfile.obj \
	search.obj \
	session.obj \
	sha256.obj \
	sign.obj \
	sound.obj \
	spell.obj \
	spellfile.obj \
	spellsuggest.obj \
	strings.obj \
	syntax.obj \
	tag.obj \
	term.obj \
	terminal.obj \
	termlib.obj \
	testing.obj \
	textformat.obj \
	textobject.obj \
	textprop.obj \
	time.obj \
	typval.obj \
	ui.obj \
	undo.obj \
	usercmd.obj \
	userfunc.obj \
	version.obj \
	vim9class.obj \
	vim9cmds.obj \
	vim9compile.obj \
	vim9execute.obj \
	vim9expr.obj \
	vim9instr.obj \
	vim9script.obj \
	vim9type.obj \
	viminfo.obj \
	window.obj \
	$(GUI_OBJ) \
	$(XDIFF_OBJ) \
	$(LUA_OBJ) \
	$(MZSCHEME_OBJ) \
	$(PERL_OBJ) \
	$(PYTHON_OBJ) \
	$(PYTHON3_OBJ) \
	$(TCL_OBJ) \
	$(RUBY_OBJ)

# Default target is making the executable
all : [.auto]config.h mmk_compat motif_env gtk_env perl_env python_env tcl_env ruby_env lua_env $(TARGET)
	! $@

[.auto]config.h : $(CONFIG_H)
	copy/nolog $(CONFIG_H) [.auto]config.h
	-@ open/append ac [.auto]config.h
        -@ hash[0,8]=35
	-@ quotes[0,8]=34
        -@ if ""$(DEF_MODIFIED)"" .EQS. "YES" then write ac ''hash',"define MODIFIED_BY ",''quotes',$(MODIFIED_BY),''quotes'
	-@ close ac

mmk_compat :
	-@ open/write pd pathdef.c
	-@ write pd "/* Empty file to satisfy MMK depend.  */"
	-@ write pd "/* It will be overwritten later on... */"
	-@ close pd
clean :
	-@ if "''F$SEARCH("*.exe")'" .NES. "" then delete/noconfirm/nolog *.exe;*
	-@ if "''F$SEARCH("*.obj")'" .NES. "" then delete/noconfirm/nolog *.obj;*
	-@ if "''F$SEARCH("[.auto]config.h")'" .NES. "" then delete/noconfirm/nolog [.auto]config.h;*
	-@ if "''F$SEARCH("pathdef.c")'" .NES. "" then delete/noconfirm/nolog pathdef.c;*
	-@ if "''F$SEARCH("if_perl.c")'" .NES. "" then delete/noconfirm/nolog if_perl.c;*
	-@ if "''F$SEARCH("*.opt")'" .NES. "" then delete/noconfirm/nolog *.opt;*
	-@ if "''F$SEARCH("*.dmp")'" .NES. "" then delete/noconfirm/nolog *.dmp;*

# Link the target
$(TARGET) : $(OBJ)
#     make an OPT file - as the obj file list is too long for one command line
	-@ DIRECTORY *.OBJ. /BRIEF/COLUMNS=1/NOHEADING/NOTRAILING /SELECT=FILE=(NONODE,NODEVICE,NODIRECTORY,NOVERSION)/OUTPUT=ALL_OBJS_LIST.OPT
	$(LD_DEF) $(LDFLAGS) /exe=$(TARGET) ALL_OBJS_LIST.OPT/OPT $(ALL_LIBS)

.c.obj :
	$(CC_DEF) $(ALL_CFLAGS) $<

pathdef.c : check_ccver $(CONFIG_H)
	-@ write sys$output "creating PATHDEF.C file."
	-@ open/write pd pathdef.c
	-@ write pd "/* pathdef.c -- DO NOT EDIT! */"
	-@ write pd "/* This file is automatically created by MAKE_VMS.MMS"
	-@ write pd " * Change the file MAKE_VMS.MMS Only. */"
	-@ write pd "typedef unsigned char   char_u;"
	-@ write pd "char_u *default_vim_dir = (char_u *)"$(VIMLOC)";"
	-@ write pd "char_u *default_vimruntime_dir = (char_u *)"$(VIMRUN)";"
	-@ write pd "char_u *all_cflags = (char_u *)""$(CC_DEF)$(ALL_CFLAGS_VER)"";"
	-@ write pd "char_u *all_lflags = (char_u *)""$(LD_DEF)$(LDFLAGS) /exe=$(TARGET) ALL_OBJS_LIST.OPT/OPT $(ALL_LIBS)"";"
	-@ write pd "char_u *compiler_version = (char_u *) ""''CC_VER'"";"
	-@ write pd "char_u *compiled_user = (char_u *) "$(VIMUSER)";"
	-@ write pd "char_u *compiled_sys  = (char_u *) "$(VIMHOST)";"
	-@ write pd "char_u *compiled_arch = (char_u *) ""$(MMSARCH_NAME)"";"
	-@ close pd

if_perl.c : if_perl.xs
	-@ $(PERL) PERL_ROOT:[LIB.ExtUtils]xsubpp -prototypes -typemap - PERL_ROOT:[LIB.ExtUtils]typemap if_perl.xs >> $@

make_vms.mms :
	-@ write sys$output "The name of the makefile MUST be <MAKE_VMS.MMS> !!!"

.IFDEF CCVER
# This part can make some complications if you're using some predefined
# symbols/flags for your compiler. If does, just comment out CCVER variable
check_ccver :
	-@ define sys$output cc_ver.tmp
	-@ $(CC_DEF)/version
	-@ deassign sys$output
	-@ open/read file cc_ver.tmp
	-@ read file CC_VER
	-@ close file
	-@ delete/noconfirm/nolog cc_ver.tmp.*
.ELSE
check_ccver :
	-@ !
.ENDIF

.IFDEF MOTIF
motif_env :
.IFDEF XPM
	-@ write sys$output "using DECW/Motif/XPM environment."
        -@ write sys$output "creating OS_VMS_XPM.OPT file."
	-@ open/write opt_file OS_VMS_XPM.OPT
.IFDEF MMSVAX
	-@ write opt_file "[.xpm.vms.vax]libxpm.olb/lib"
.ENDIF
.IFDEF MMSALPHA
	-@ write opt_file "[.xpm.vms.axp]libxpm.olb/lib"
.ENDIF
.IFDEF MMSIA64
	-@ write opt_file "[.xpm.vms.ia64]libxpm.olb/lib"
.ENDIF
	-@ close opt_file
.ELSE
	-@ write sys$output "using DECW/Motif environment."
.ENDIF
	-@ write sys$output "creating OS_VMS_MOTIF.OPT file."
	-@ open/write opt_file OS_VMS_MOTIF.OPT
	-@ write opt_file "sys$share:decw$xmlibshr12.exe/share,-"
	-@ write opt_file "sys$share:decw$xtlibshrr5.exe/share,-"
	-@ write opt_file "sys$share:decw$xlibshr.exe/share"
	-@ close opt_file
.ELSE
motif_env :
	-@ !
.ENDIF


.IFDEF GTK
gtk_env :
	-@ write sys$output "using GTK environment:"
	-@ define/nolog gtk_root /trans=conc $(GTK_DIR)
	-@ show logical gtk_root
	-@ write sys$output "    include path: "$(GUI_INC)""
	-@ write sys$output "creating OS_VMS_GTK.OPT file."
	-@ open/write opt_file OS_VMS_GTK.OPT
	-@ write opt_file "gtk_root:[glib]libglib.exe /share,-"
	-@ write opt_file "gtk_root:[glib.gmodule]libgmodule.exe /share,-"
	-@ write opt_file "gtk_root:[gtk.gdk]libgdk.exe /share,-"
	-@ write opt_file "gtk_root:[gtk.gtk]libgtk.exe /share,-"
	-@ write opt_file "sys$share:decw$xmlibshr12.exe/share,-"
	-@ write opt_file "sys$share:decw$xtlibshrr5.exe/share,-"
	-@ write opt_file "sys$share:decw$xlibshr.exe/share"
	-@ close opt_file
.ELSE
gtk_env :
	-@ !
.ENDIF

.IFDEF VIM_PERL
perl_env :
	-@ write sys$output "using PERL environment:"
	-@ show logical PERLSHR
	-@ write sys$output "    include path: ""$(PERL_INC)"""
	-@ show symbol perl
	-@ open/write pd if_perl.c
	-@ write pd "/* Empty file to satisfy MMK depend.  */"
	-@ write pd "/* It will be overwritten later on... */"
	-@ close pd
	-@ write sys$output "creating OS_VMS_PERL.OPT file."
	-@ open/write opt_file OS_VMS_PERL.OPT
	-@ write opt_file "PERLSHR /share"
	-@ close opt_file
.ELSE
perl_env :
	-@ !
.ENDIF

.IFDEF VIM_PYTHON
python_env :
	-@ write sys$output "using PYTHON environment:"
	-@ show logical PYTHON_INCLUDE
	-@ show logical PYTHON_OLB
	-@ write sys$output "creating OS_VMS_PYTHON.OPT file."
	-@ open/write opt_file OS_VMS_PYTHON.OPT
	-@ write opt_file "PYTHON_OLB:PYTHON.OLB /share"
	-@ close opt_file
.ELSE
python_env :
	-@ !
.ENDIF

.IFDEF VIM_PYTHON3
python3_env :
	-@ write sys$output "using PYTHON3 environment:"
	-@ show logical PYTHON3_INCLUDE
	-@ show logical PYTHON3_OLB
	-@ write sys$output "creating OS_VMS_PYTHON3.OPT file."
	-@ open/write opt_file OS_VMS_PYTHON3.OPT
	-@ write opt_file "PYTHON3_OLB:PYTHON3.OLB /share"
	-@ close opt_file
.ELSE
python3_env :
	-@ !
.ENDIF

.IFDEF VIM_TCL
tcl_env :
	-@ write sys$output "using TCL environment:"
	-@ show logical TCLSHR
	-@ write sys$output "    include path: ""$(TCL_INC)"""
	-@ write sys$output "creating OS_VMS_TCL.OPT file."
	-@ open/write opt_file OS_VMS_TCL.OPT
	-@ write opt_file "TCLSHR /share"
	-@ close opt_file
.ELSE
tcl_env :
	-@ !
.ENDIF

.IFDEF VIM_RUBY
ruby_env :
	-@ write sys$output "using RUBY environment:"
	-@ write sys$output "    include path: ""$(RUBY_INC)"""
	-@ write sys$output "creating OS_VMS_RUBY.OPT file."
	-@ open/write opt_file OS_VMS_RUBY.OPT
	-@ write opt_file "RUBYSHR /share"
	-@ close opt_file
.ELSE
ruby_env :
	-@ !
.ENDIF

.IFDEF VIM_LUA
lua_env :
	-@ write sys$output "using LUA environment:"
	-@ write sys$output "    include path: ""$(LUA_INC)"""
	-@ write sys$output "creating OS_VMS_LUA.OPT file."
	-@ open/write opt_file OS_VMS_LUA.OPT
	-@ write opt_file "LUA$ROOT:[LIB]LUA$SHR.EXE /share"
	-@ close opt_file
.ELSE
lua_env :
	-@ !
.ENDIF

alloc.obj : alloc.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
arabic.obj : arabic.c vim.h
arglist.obj : arglist.c vim.h [.auto]config.h feature.h os_unix.h
autocmd.obj : autocmd.c vim.h [.auto]config.h feature.h os_unix.h
blowfish.obj : blowfish.c vim.h [.auto]config.h feature.h os_unix.h
blob.obj : blob.c vim.h [.auto]config.h feature.h os_unix.h
buffer.obj : buffer.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
bufwrite.obj : bufwrite.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
change.obj : change.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
charset.obj : charset.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
channel.obj : channel.c vim.h [.auto]config.h feature.h
cindent.obj : cindent.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
clientserver.obj : clientserver.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
clipboard.obj : clipboard.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
cmdexpand.obj : cmdexpand.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
cmdhist.obj : cmdhist.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
crypt.obj : crypt.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
crypt_zip.obj : crypt_zip.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
debugger.obj : debugger.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
dict.obj : dict.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
diff.obj : diff.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
digraph.obj : digraph.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
drawline.obj : drawline.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
drawscreen.obj : drawscreen.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
edit.obj : edit.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
eval.obj : eval.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
evalbuffer.obj : evalbuffer.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
evalfunc.obj : evalfunc.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h version.h
evalvars.obj : evalvars.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h version.h
evalwindow.obj : evalwindow.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
ex_cmds.obj : ex_cmds.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
ex_cmds2.obj : ex_cmds2.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
ex_docmd.obj : ex_docmd.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h ex_cmdidxs.h
ex_eval.obj : ex_eval.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
ex_getln.obj : ex_getln.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
fileio.obj : fileio.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
filepath.obj : filepath.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
findfile.obj : findfile.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
float.obj : float.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
fold.obj : fold.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
getchar.obj : getchar.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
gui_xim.obj : gui_xim.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
hardcopy.obj : hardcopy.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
hashtab.obj : hashtab.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
help.obj : help.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
highlight.obj : highlight.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
if_cscope.obj : if_cscope.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
if_xcmdsrv.obj : if_xcmdsrv.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
if_mzsch.obj : if_mzsch.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro ex_cmds.h proto.h \
 errors.h globals.h if_mzsch.h
indent.obj : indent.c vim.h [.auto]config.h feature.h os_unix.h
insexpand.obj : insexpand.c vim.h [.auto]config.h feature.h os_unix.h
job.obj : job.c vim.h [.auto]config.h feature.h os_unix.h
json.obj : json.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
list.obj : list.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
locale.obj : locale.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
logfile.obj : logfile.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
main.obj : main.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h \
 arabic.c
map.obj : map.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
mark.obj : mark.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
match.obj : match.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
memfile.obj : memfile.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
memline.obj : memline.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
menu.obj : menu.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
message.obj : message.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
misc1.obj : misc1.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h \
 version.h
misc2.obj : misc2.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
mouse.obj : mouse.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
move.obj : move.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
mbyte.obj : mbyte.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
normal.obj : normal.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h nv_cmdidxs.h nv_cmds.h
ops.obj : ops.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
option.obj : option.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h optiondefs.h
optionstr.obj : optionstr.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
os_unix.obj : os_unix.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h os_unixx.h
os_vms.obj : os_vms.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h os_unixx.h
pathdef.obj : pathdef.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
popupmenu.obj : popupmenu.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
popupwin.obj : popupwin.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
pty.obj : pty.c vim.h [.auto]config.h feature.h os_unix.h
profiler.obj : profiler.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
quickfix.obj : quickfix.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
regexp.obj : regexp.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
register.obj : register.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
scriptfile.obj : scriptfile.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
screen.obj : screen.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
search.obj : search.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
session.obj : session.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
sha256.obj : sha256.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
sign.obj : sign.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h regexp.h gui.h \
 beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h proto.h \
 errors.h globals.h
sound.obj : sound.c vim.h [.auto]config.h feature.h
spell.obj : spell.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
spellfile.obj : spellfile.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
spellsuggest.obj : spellsuggest.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
strings.obj : strings.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
syntax.obj : syntax.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
tag.obj : tag.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
term.obj : term.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
terminal.obj : terminal.c vim.h [.auto]config.h feature.h os_unix.h
termlib.obj : termlib.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
testing.obj : testing.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
textformat.obj : textformat.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
textobject.obj : textobject.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
textprop.obj : textprop.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
time.obj : time.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
typval.obj : typval.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
ui.obj : ui.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
undo.obj : undo.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
usercmd.obj : usercmd.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
userfunc.obj : userfunc.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h option.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro alloc.h ex_cmds.h spell.h \
 proto.h errors.h globals.h
version.obj : version.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
viminfo.obj : viminfo.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9class.obj : vim9class.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9cmds.obj : vim9cmds.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9compile.obj : vim9compile.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9execute.obj : vim9execute.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9expr.obj : vim9expr.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9instr.obj : vim9instr.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9script.obj : vim9script.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
vim9type.obj : vim9type.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
window.obj : window.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
gui.obj : gui.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
gui_gtk.obj : gui_gtk.c gui_gtk_f.h vim.h [.auto]config.h feature.h \
 os_unix.h   ascii.h keymap.h termdefs.h macros.h structs.h \
 regexp.h gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h \
 proto.h errors.h globals.h [-.pixmaps]stock_icons.h
gui_gtk_f.obj : gui_gtk_f.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h gui_gtk_f.h
gui_motif.obj : gui_motif.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h [-.pixmaps]alert.xpm [-.pixmaps]error.xpm \
 [-.pixmaps]generic.xpm [-.pixmaps]info.xpm [-.pixmaps]quest.xpm
gui_athena.obj : gui_athena.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h gui_at_sb.h
gui_gtk_x11.obj : gui_gtk_x11.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h gui_gtk_f.h [-.runtime]vim32x32.xpm \
 [-.runtime]vim16x16.xpm [-.runtime]vim48x48.xpm version.h
gui_x11.obj : gui_x11.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h [-.runtime]vim32x32.xpm \
 [-.runtime]vim16x16.xpm [-.runtime]vim48x48.xpm [-.pixmaps]tb_new.xpm \
 [-.pixmaps]tb_open.xpm [-.pixmaps]tb_close.xpm [-.pixmaps]tb_save.xpm \
 [-.pixmaps]tb_print.xpm [-.pixmaps]tb_cut.xpm [-.pixmaps]tb_copy.xpm \
 [-.pixmaps]tb_paste.xpm [-.pixmaps]tb_find.xpm \
 [-.pixmaps]tb_find_next.xpm [-.pixmaps]tb_find_prev.xpm \
 [-.pixmaps]tb_find_help.xpm [-.pixmaps]tb_exit.xpm \
 [-.pixmaps]tb_undo.xpm [-.pixmaps]tb_redo.xpm [-.pixmaps]tb_help.xpm \
 [-.pixmaps]tb_macro.xpm [-.pixmaps]tb_make.xpm \
 [-.pixmaps]tb_save_all.xpm [-.pixmaps]tb_jump.xpm \
 [-.pixmaps]tb_ctags.xpm [-.pixmaps]tb_load_session.xpm \
 [-.pixmaps]tb_save_session.xpm [-.pixmaps]tb_new_session.xpm \
 [-.pixmaps]tb_blank.xpm [-.pixmaps]tb_maximize.xpm \
 [-.pixmaps]tb_split.xpm [-.pixmaps]tb_minimize.xpm \
 [-.pixmaps]tb_shell.xpm [-.pixmaps]tb_replace.xpm \
 [-.pixmaps]tb_vsplit.xpm [-.pixmaps]tb_maxwidth.xpm \
 [-.pixmaps]tb_minwidth.xpm
gui_at_sb.obj : gui_at_sb.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h gui_at_sb.h
gui_at_fs.obj : gui_at_fs.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h gui_at_sb.h
pty.obj : pty.c vim.h [.auto]config.h feature.h os_unix.h   \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h gui.h beval.h \
 [.proto]gui_beval.pro option.h ex_cmds.h proto.h errors.h globals.h
if_perl.obj : [.auto]if_perl.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
if_perlsfio.obj : if_perlsfio.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
if_python.obj : if_python.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
if_tcl.obj : if_tcl.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
if_ruby.obj : if_ruby.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
if_lua.obj : if_lua.c vim.h [.auto]config.h feature.h os_unix.h \
 errors.h globals.h version.h
beval.obj : beval.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h option.h ex_cmds.h proto.h \
 errors.h globals.h
gui_beval.obj : gui_beval.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h
netbeans.obj : netbeans.c vim.h [.auto]config.h feature.h os_unix.h \
 ascii.h keymap.h termdefs.h macros.h structs.h regexp.h \
 gui.h beval.h [.proto]gui_beval.pro option.h ex_cmds.h proto.h \
 errors.h globals.h version.h
gui_xmdlg.obj : gui_xmdlg.c [.auto]config.h vim.h feature.h os_unix.h
gui_xmebw.obj : gui_xmebw.c [.auto]config.h vim.h feature.h os_unix.h
xdiffi.obj : [.xdiff]xdiffi.c [.xdiff]xinclude.h [.auto]config.h vim.h feature.h os_unix.h
xemit.obj : [.xdiff]xemit.c [.xdiff]xinclude.h [.auto]config.h vim.h feature.h os_unix.h
xprepare.obj : [.xdiff]xprepare.c [.xdiff]xinclude.h [.auto]config.h vim.h feature.h os_unix.h
xutils.obj : [.xdiff]xutils.c [.xdiff]xinclude.h [.auto]config.h vim.h feature.h os_unix.h
xhistogram.obj : [.xdiff]xhistogram.c [.xdiff]xinclude.h [.auto]config.h vim.h feature.h os_unix.h
xpatience.obj : [.xdiff]xpatience.c [.xdiff]xinclude.h [.auto]config.h vim.h feature.h os_unix.h
