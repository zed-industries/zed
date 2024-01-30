# This Makefile has two purposes:
# 1. Starting the compilation of Vim for Unix.
# 2. Creating the various distribution files.


#########################################################################
# 1. Starting the compilation of Vim for Unix.
#
# Using this Makefile without an argument will compile Vim for Unix.
# "make install" is also possible.
#
# NOTE: If this doesn't work properly, first change directory to "src" and use
# the Makefile there:
#	cd src
#	make [arguments]
# Noticed on AIX systems when using this Makefile: Trying to run "cproto" or
# something else after Vim has been compiled.  Don't know why...
# Noticed on OS/390 Unix: Restarts configure.
#
# The first (default) target is "first".  This will result in running
# "make first", so that the target from "src/auto/config.mk" is picked
# up properly when config didn't run yet.  Doing "make all" before configure
# has run can result in compiling with $(CC) empty.

first:
	@if test ! -f src/auto/config.mk; then \
		cp src/config.mk.dist src/auto/config.mk; \
	fi
	@echo "Starting make in the src directory."
	@echo "If there are problems, cd to the src directory and run make there"
	cd src && $(MAKE) $@

# Some make programs use the last target for the $@ default; put the other
# targets separately to always let $@ expand to "first" by default.
all install uninstall tools config configure reconfig proto depend lint tags types test scripttests testtiny test_libvterm unittests testclean clean distclean:
	@if test ! -f src/auto/config.mk; then \
		cp src/config.mk.dist src/auto/config.mk; \
	fi
	@echo "Starting make in the src directory."
	@echo "If there are problems, cd to the src directory and run make there"
	cd src && $(MAKE) $@
	@# When the target is "test" also run the indent and syntax tests.
	@if test "$@" = "test" -o "$@" = "testtiny"; then \
		$(MAKE) indenttest; \
		$(MAKE) syntaxtest; \
	fi
	@# When the target is "clean" also clean for the indent and syntax tests.
	@if test "$@" = "clean" -o "$@" = "distclean" -o "$@" = "testclean"; then \
		(cd runtime/indent && $(MAKE) clean); \
		(cd runtime/syntax && $(MAKE) clean); \
	fi

# Executable used for running the indent tests.
VIM_FOR_INDENTTEST = ../../src/vim

indenttest:
	cd runtime/indent && \
		$(MAKE) clean && \
		$(MAKE) test VIM="$(VIM_FOR_INDENTTEST)"

# Executable used for running the syntax tests.
VIM_FOR_SYNTAXTEST = ../../src/vim

syntaxtest:
	cd runtime/syntax && \
		$(MAKE) clean && \
		$(MAKE) test VIMPROG="$(VIM_FOR_SYNTAXTEST)"


#########################################################################
# 2. Creating the various distribution files.
#
# TARGET	PRODUCES		CONTAINS
# unixall	vim-#.#.tar.bz2		All runtime files and sources, for Unix
#
# html		vim##html.zip		HTML docs
#
# dossrc	vim##src.zip		sources for MS-DOS
# dosrt		vim##rt.zip		runtime for MS-DOS
# dosbin	vim##w32.zip		binary for Win32
#		gvim##.zip		binary for GUI Win32
#		gvim##ole.zip		OLE exe for Win32 GUI
#
# OBSOLETE
# amisrc	vim##src.tgz		sources for Amiga
# amirt		vim##rt.tgz		runtime for Amiga
# amibin	vim##bin.tgz		binary for Amiga
#
# farsi		farsi##.zip		Farsi fonts
#
#    All output files are created in the "dist" directory.  Existing files are
#    overwritten!
#    To do all this you need the Unix archive and compiled binaries.
#    Before creating an archive first delete all backup files, *.orig, etc.

MAJOR = 9
MINOR = 1

# CHECKLIST for creating a new version:
#
# - Update Vim version number.  For a test version in: src/version.h,
#   READMEdir/Contents, MAJOR/MINOR above, VIMMAJOR and VIMMINOR in
#   src/Makefile, README.txt, README.md, src/README.md, READMEdir/README*.txt,
#   runtime/doc/*.txt and make nsis/gvim_version.nsh.
#   For a minor/major version: src/GvimExt/GvimExt.reg, src/vim.manifest.
# - Compile Vim with GTK, Perl, Python, Python3, TCL, Ruby, Lua, Cscope and
#   "huge" features.  Add MZscheme if you can make it work.
#   Use "make reconfig" after selecting the configure arguments.
# - With these features: "make proto" (requires cproto and Motif installed;
#   ignore warnings for missing include files, fix problems for syntax errors).
# - With these features: "make depend" (works best with gcc).
# - If you have a lint program: "make lint" and check the output (ignore GTK
#   warnings).
# - If you have valgrind, enable it in src/testdir/Makefile and run "make
#   test".  Enable EXITFREE, disable GUI, scheme and tcl to avoid false alarms.
#   Check the valgrind output.
# - Adjust the date and other info in src/version.h.
# - Correct included_patches[] in src/version.c.
# - Check for missing entries in runtime/makemenu.vim (with checkmenu script).
# - Check for missing options in runtime/optwin.vim et al. (with check.vim).
# - Do "make menu" to update the runtime/synmenu.vim file.
# - Add remarks for changes to runtime/doc/version9.txt.
# - Check that runtime/doc/help.txt doesn't contain entries in "LOCAL
#   ADDITIONS".
# - In runtime/doc run "make" and "make html" to check for errors.
# - Check if src/Makefile, src/testdir/Makefile and src/feature.h don't contain
#   any personal preferences or the changes mentioned above.
# - Check file protections to be "644" for text and "755" for executables (run
#   the "check" script).
# - Check compiling on Amiga, MS-DOS and MS-Windows.
# - Delete all *~, *.sw?, *.orig, *.rej files
# - "make unixall", "make html"
# - Make diff files against the previous release: "makediff7 7.1 7.2"
#
# Amiga: (OBSOLETE, Amiga files are no longer distributed)
# - "make amisrc", move the archive to the Amiga and compile:
#   "make -f Make_manx.mak" (will use "big" features by default).
# - Run the tests: "make -f Make_manx.mak test"
# - Place the executables Vim and Xxd in this directory (set the executable
#   flag).
# - "make amirt", "make amibin".
#
# MS-Windows:
# - Run make on Unix to update the ".mo" files.
# - Get 32 bit libintl-8.dll, libiconv-2.dll and libgcc_s_sjlj-1.dll. E.g. from
#   https://mlocati.github.io/gettext-iconv-windows/ .
#   Use the "shared-32.zip file and extract the archive to get the files.
#   Put them in the gettext32 directory, "make dosrt" uses them.
# - Get 64 bit libintl-8.dll and libiconv-2.dll. E.g. from
#   https://mlocati.github.io/gettext-iconv-windows/ .
#   Use the "shared-64.zip file and extract the archive to get the files.
#   Put them in the gettext64 directory, "make dosrt" uses them.
# - > make dossrc
#   > make dosrt
#   Unpack dist/vim##rt.zip and dist/vim##src.zip on an MS-Windows PC.
#   This creates the directory vim/vim90 and puts all files in there.
# Win32 console version build:
# - See src/INSTALLpc.txt for installing the compiler and SDK.
# - Set environment for Visual C++ 2015:
#   > cd src
#   > msvc2015.bat
# - Build the console binary:
#   > nmake -f Make_mvc.mak
# - Run the tests and check the output:
#   > nmake -f Make_mvc.mak testclean
#   > nmake -f Make_mvc.mak test
# - Rename (using ../tools/rename.bat):
#           vim.exe to vimw32.exe
#           tee/tee.exe to teew32.exe
#           xxd/xxd.exe to xxdw32.exe
#           vim.pdb to vimw32.pdb
#           install.exe to installw32.exe
#           uninstall.exe to uninstallw32.exe
# Win32 GUI version build:
# - > cd src
#   > nmake -f Make_mvc.mak GUI=yes
# - Run the tests and check the output:
#   > nmake -f Make_mvc.mak testclean
#   > nmake -f Make_mvc.mak testgvim
# - move "gvim.exe" to here (otherwise the OLE version will overwrite it).
# - Move gvim.pdb to here.
# - Copy "GvimExt/gvimext.dll" to here.
# - Delete vimrun.exe, install.exe and uninstall.exe.
# Win32 GUI version with OLE, PERL, Ruby, TCL, PYTHON and dynamic IME:
# - Install the interfaces you want, see src/INSTALLpc.txt
#   Adjust bigvim.bat to match the version of each interface you want.
# - Build:
#   > cd src
#   > bigvim.bat
# - Run the tests:
#   > nmake -f Make_mvc.mak testclean
#   > nmake -f Make_mvc.mak testgvim
#   - check the output.
# - Rename "gvim.exe" to "gvim_ole.exe".
# - Rename gvim.pdb to "gvim_ole.pdb".
# - Delete install.exe and uninstall.exe.
# Create the archives:
# - Copy all the "*.exe" files to where this Makefile is.
# - Copy all the "*.pdb" files to where this Makefile is.
# - in this directory:
#   > make dosbin
# NSIS self installing exe:
# - To get NSIS see http://nsis.sourceforge.net
# - Make sure gvim_ole.exe, vimw32.exe, installw32.exe,
#   uninstallw32.exe, teew32.exe and xxdw32.exe have been build as mentioned
#   above.
# - copy these files (get them from a binary archive or build them):
#	gvimext.dll in src/GvimExt
#	gvimext64.dll in src/GvimExt
#   gvimext64.dll can be obtained from:
#   https://github.com/vim/vim-win32-installer/releases
#	It is part of gvim_9.0.*_x64.zip as vim/vim90/GvimExt/gvimext64.dll.
# - Make sure there is a diff.exe two levels up (get it from a previous Vim
#   version).  Also put winpty32.dll and winpty-agent.exe there.
# - go to ../nsis and do:
#   > unzip icons.zip
#   > makensis gvim.nsi  (takes a few minutes).
#      ignore warning for libwinpthread-1.dll
# - Copy gvim##.exe to the dist directory.
#
# 64 bit builds (these are not in the normal distribution, the 32 bit build
# works just fine on 64 bit systems).
# Like the console and GUI version, but first run vcvars64.bat or
#   "..\VC\vcvarsall.bat x86_amd64".
# - Build the console version:
#   > nmake -f Make_mvc.mak
# - Build the GUI version:
#   > nmake -f Make_mvc.mak GUI=yes
# - Build the OLE version with interfaces:
#   > bigvim64.bat
#
#
# OBSOLETE systems: You can build these if you have an appropriate system.
#
# 16 bit DOS version: You need to get a very old version of Vim, for several
# years even the tiny build is too big to fit in DOS memory.
#
# 32 bit DOS version: Support was removed in 7.4.1399.  When syncing to before
# that it probably won't build.
#
# Win32s GUI version: Support was removed in patch 7.4.1364.
#
# OS/2 support was removed in patch 7.4.1008.  If you want to give it a try
# sync to before that and check the old version of this Makefile for
# instructions.

VIMVER	= vim-$(MAJOR).$(MINOR)
VERSION = $(MAJOR)$(MINOR)
VDOT	= $(MAJOR).$(MINOR)
VIMRTDIR = vim$(VERSION)

# Vim used for conversion from "unix" to "dos"
VIM	= vim

# How to include Filelist depends on the version of "make" you have.
# If the current choice doesn't work, try the other one.

include Filelist
#.include "Filelist"


# All output is put in the "dist" directory.
dist:
	mkdir dist

# Clean up some files to avoid they are included.
# Copy README files to the top directory.
prepare:
	if test -f runtime/doc/uganda.nsis.txt; then \
		rm runtime/doc/uganda.nsis.txt; fi
	for name in $(IN_README_DIR); do \
	  cp READMEdir/"$$name" .; \
	  done

# For the zip files we need to create a file with the comment line
dist/comment:
	mkdir dist/comment

COMMENT_RT = comment/$(VERSION)-rt
COMMENT_W32 = comment/$(VERSION)-bin-w32
COMMENT_GVIM = comment/$(VERSION)-bin-gvim
COMMENT_OLE = comment/$(VERSION)-bin-ole
COMMENT_SRC = comment/$(VERSION)-src
COMMENT_HTML = comment/$(VERSION)-html
COMMENT_FARSI = comment/$(VERSION)-farsi

dist/$(COMMENT_RT): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) runtime files for MS-DOS and MS-Windows" > dist/$(COMMENT_RT)

dist/$(COMMENT_W32): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) binaries for MS-Windows NT/95" > dist/$(COMMENT_W32)

dist/$(COMMENT_GVIM): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) GUI binaries for MS-Windows NT/95" > dist/$(COMMENT_GVIM)

dist/$(COMMENT_OLE): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) MS-Windows GUI binaries with OLE support" > dist/$(COMMENT_OLE)

dist/$(COMMENT_SRC): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) sources for MS-DOS and MS-Windows" > dist/$(COMMENT_SRC)

dist/$(COMMENT_HTML): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) documentation in HTML" > dist/$(COMMENT_HTML)

dist/$(COMMENT_FARSI): dist/comment
	echo "Vim - Vi IMproved - v$(VDOT) Farsi language files" > dist/$(COMMENT_FARSI)

unixall: dist prepare
	-rm -f dist/$(VIMVER).tar.bz2
	-rm -rf dist/$(VIMRTDIR)
	mkdir dist/$(VIMRTDIR)
	tar cf - \
		$(RT_ALL) \
		$(RT_ALL_BIN) \
		$(RT_UNIX) \
		$(RT_UNIX_DOS_BIN) \
		$(RT_SCRIPTS) \
		$(LANG_GEN) \
		$(LANG_GEN_BIN) \
		$(SRC_ALL) \
		$(SRC_UNIX) \
		$(SRC_DOS_UNIX) \
		$(EXTRA) \
		$(LANG_SRC) \
		| (cd dist/$(VIMRTDIR); tar xf -)
	-rm $(IN_README_DIR)
# Need to use a "distclean" config.mk file
# Note: this file is not included in the repository to avoid problems, but it's
# OK to put it in the archive.
	cp -f src/config.mk.dist dist/$(VIMRTDIR)/src/auto/config.mk
# Create an empty config.h file, make dependencies require it
	touch dist/$(VIMRTDIR)/src/auto/config.h
# Make sure configure is newer than config.mk to force it to be generated
	touch dist/$(VIMRTDIR)/src/configure
# Make sure ja.sjis.po is newer than ja.po to avoid it being regenerated.
# Same for cs.cp1250.po, pl.cp1250.po and sk.cp1250.po.
	touch dist/$(VIMRTDIR)/src/po/ja.sjis.po
	touch dist/$(VIMRTDIR)/src/po/cs.cp1250.po
	touch dist/$(VIMRTDIR)/src/po/pl.cp1250.po
	touch dist/$(VIMRTDIR)/src/po/sk.cp1250.po
	touch dist/$(VIMRTDIR)/src/po/zh_CN.cp936.po
	touch dist/$(VIMRTDIR)/src/po/ru.cp1251.po
	touch dist/$(VIMRTDIR)/src/po/uk.cp1251.po
# Create the archive.
	cd dist && tar cf $(VIMVER).tar $(VIMRTDIR)
	bzip2 dist/$(VIMVER).tar

# Amiga runtime - OBSOLETE
amirt: dist prepare
	-rm -f dist/vim$(VERSION)rt.tar.gz
	-rm -rf dist/Vim
	mkdir dist/Vim
	mkdir dist/Vim/$(VIMRTDIR)
	tar cf - \
		$(ROOT_AMI) \
		$(RT_ALL) \
		$(RT_ALL_BIN) \
		$(RT_SCRIPTS) \
		$(RT_AMI) \
		$(RT_NO_UNIX) \
		$(RT_AMI_DOS) \
		| (cd dist/Vim/$(VIMRTDIR); tar xf -)
	-rm $(IN_README_DIR)
	mv dist/Vim/$(VIMRTDIR)/vimdir.info dist/Vim.info
	mv dist/Vim/$(VIMRTDIR)/runtime.info dist/Vim/$(VIMRTDIR).info
	mv dist/Vim/$(VIMRTDIR)/runtime/* dist/Vim/$(VIMRTDIR)
	rmdir dist/Vim/$(VIMRTDIR)/runtime
	cd dist && tar cf vim$(VERSION)rt.tar Vim Vim.info
	gzip -9 dist/vim$(VERSION)rt.tar
	mv dist/vim$(VERSION)rt.tar.gz dist/vim$(VERSION)rt.tgz

# Amiga binaries - OBSOLETE
amibin: dist prepare
	-rm -f dist/vim$(VERSION)bin.tar.gz
	-rm -rf dist/Vim
	mkdir dist/Vim
	mkdir dist/Vim/$(VIMRTDIR)
	tar cf - \
		$(ROOT_AMI) \
		$(BIN_AMI) \
		Vim \
		Xxd \
		| (cd dist/Vim/$(VIMRTDIR); tar xf -)
	-rm $(IN_README_DIR)
	mv dist/Vim/$(VIMRTDIR)/vimdir.info dist/Vim.info
	mv dist/Vim/$(VIMRTDIR)/runtime.info dist/Vim/$(VIMRTDIR).info
	cd dist && tar cf vim$(VERSION)bin.tar Vim Vim.info
	gzip -9 dist/vim$(VERSION)bin.tar
	mv dist/vim$(VERSION)bin.tar.gz dist/vim$(VERSION)bin.tgz

# Amiga sources - OBSOLETE
amisrc: dist prepare
	-rm -f dist/vim$(VERSION)src.tar.gz
	-rm -rf dist/Vim
	mkdir dist/Vim
	mkdir dist/Vim/$(VIMRTDIR)
	tar cf - \
		$(ROOT_AMI) \
		$(SRC_ALL) \
		$(SRC_AMI) \
		$(SRC_AMI_DOS) \
		| (cd dist/Vim/$(VIMRTDIR); tar xf -)
	-rm $(IN_README_DIR)
	mv dist/Vim/$(VIMRTDIR)/vimdir.info dist/Vim.info
	mv dist/Vim/$(VIMRTDIR)/runtime.info dist/Vim/$(VIMRTDIR).info
	cd dist && tar cf vim$(VERSION)src.tar Vim Vim.info
	gzip -9 dist/vim$(VERSION)src.tar
	mv dist/vim$(VERSION)src.tar.gz dist/vim$(VERSION)src.tgz

no_title.vim: Makefile
	echo "set notitle noicon nocp nomodeline viminfo=" >no_title.vim

# MS-DOS sources
dossrc: dist no_title.vim dist/$(COMMENT_SRC) \
	runtime/doc/uganda.nsis.txt \
	nsis/gvim_version.nsh
	-rm -rf dist/vim$(VERSION)src.zip
	-rm -rf dist/vim
	mkdir dist/vim
	mkdir dist/vim/$(VIMRTDIR)
	tar cf - \
		$(SRC_ALL) \
		$(SRC_DOS) \
		$(SRC_DOS_BIN) \
		$(SRC_AMI_DOS) \
		$(SRC_DOS_UNIX) \
		runtime/doc/uganda.nsis.txt \
		nsis/gvim_version.nsh \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	mv dist/vim/$(VIMRTDIR)/runtime/* dist/vim/$(VIMRTDIR)
	rmdir dist/vim/$(VIMRTDIR)/runtime
	# This file needs to be in dos fileformat for NSIS.
	$(VIM) -e -X -u no_title.vim -c ":set tx|wq" dist/vim/$(VIMRTDIR)/doc/uganda.nsis.txt
	cd dist && zip -9 -rD -z vim$(VERSION)src.zip vim <$(COMMENT_SRC)

runtime/doc/uganda.nsis.txt: runtime/doc/uganda.txt
	cd runtime/doc && $(MAKE) uganda.nsis.txt

nsis/gvim_version.nsh: Makefile
	echo "# Generated from Makefile: define the version numbers" > $@
	echo "!ifndef __GVIM_VER__NSH__"  >> $@
	echo "!define __GVIM_VER__NSH__"  >> $@
	echo "!define VER_MAJOR $(MAJOR)" >> $@
	echo "!define VER_MINOR $(MINOR)" >> $@
	echo "!endif" >> $@

dosrt: dist dist/$(COMMENT_RT) dosrt_files
	-rm -rf dist/vim$(VERSION)rt.zip
	cd dist && zip -9 -rD -z vim$(VERSION)rt.zip vim <$(COMMENT_RT)

# Split in two parts to avoid an "argument list too long" error.
# We no longer convert the files from unix to dos fileformat.
dosrt_files: dist prepare no_title.vim
	-rm -rf dist/vim
	mkdir dist/vim
	mkdir dist/vim/$(VIMRTDIR)
	mkdir dist/vim/$(VIMRTDIR)/lang
	cd src && MAKEMO=yes $(MAKE) languages
	tar cf - \
		$(RT_ALL) \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	tar cf - \
		$(RT_SCRIPTS) \
		$(RT_DOS) \
		$(RT_NO_UNIX) \
		$(RT_AMI_DOS) \
		$(LANG_GEN) \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	tar cf - \
		$(RT_UNIX_DOS_BIN) \
		$(RT_ALL_BIN) \
		$(RT_DOS_BIN) \
		$(LANG_GEN_BIN) \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	-rm $(IN_README_DIR)
	mv dist/vim/$(VIMRTDIR)/runtime/* dist/vim/$(VIMRTDIR)
	rmdir dist/vim/$(VIMRTDIR)/runtime
# Add the message translations.  Trick: skip ja.mo/ja.euc-jp.mo and use
# ja.sjis.mo instead.  Same for cs.mo / cs.cp1250.mo, pl.mo / pl.cp1250.mo,
# sk.mo / sk.cp1250.mo, zh_CN.mo / zh_CN.cp936.mo, uk.mo / uk.cp1251.mo and
# ru.mo / ru.cp1251.mo.
	for i in $(LANG_DOS); do \
	      if test "$$i" != "src/po/ja.mo" -a "$$i" != "src/po/ja.euc-jp.mo" -a "$$i" != "src/po/pl.mo" -a "$$i" != "src/po/cs.mo" -a "$$i" != "src/po/sk.mo" -a "$$i" != "src/po/zh_CN.mo" -a "$$i" != "src/po/ru.mo" -a "$$i" != "src/po/uk.mo"; then \
		n=`echo $$i | sed -e "s+src/po/\([-a-zA-Z0-9_]*\(.UTF-8\)*\)\(.sjis\)*\(.cp1250\)*\(.cp1251\)*\(.cp936\)*.mo+\1+"`; \
		mkdir dist/vim/$(VIMRTDIR)/lang/$$n; \
		mkdir dist/vim/$(VIMRTDIR)/lang/$$n/LC_MESSAGES; \
		cp $$i dist/vim/$(VIMRTDIR)/lang/$$n/LC_MESSAGES/vim.mo; \
	      fi \
	    done
	mkdir dist/vim/$(VIMRTDIR)/gettext32
	cp gettext32/libintl-8.dll dist/vim/$(VIMRTDIR)/gettext32/
	cp gettext32/libiconv-2.dll dist/vim/$(VIMRTDIR)/gettext32/
	cp gettext32/libgcc_s_sjlj-1.dll dist/vim/$(VIMRTDIR)/gettext32/
	mkdir dist/vim/$(VIMRTDIR)/gettext64
	cp gettext64/libintl-8.dll dist/vim/$(VIMRTDIR)/gettext64/
	cp gettext64/libiconv-2.dll dist/vim/$(VIMRTDIR)/gettext64/


# Used before uploading.  Don't delete the AAPDIR/sign files!
runtime_unix2dos: dosrt_files
	-rm -rf `find runtime/dos -type f -print | sed -e /AAPDIR/d`
	cd dist/vim/$(VIMRTDIR); tar cf - * \
		| (cd ../../../runtime/dos; tar xf -)

dosbin: prepare dosbin_gvim dosbin_w32 dosbin_ole $(DOSBIN_S)
	-rm $(IN_README_DIR)

# make Win32 gvim
dosbin_gvim: dist no_title.vim dist/$(COMMENT_GVIM)
	-rm -rf dist/gvim$(VERSION).zip
	-rm -rf dist/vim
	mkdir dist/vim
	mkdir dist/vim/$(VIMRTDIR)
	tar cf - \
		$(BIN_DOS) \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	cp gvim.exe dist/vim/$(VIMRTDIR)/gvim.exe
	cp teew32.exe dist/vim/$(VIMRTDIR)/tee.exe
	cp xxdw32.exe dist/vim/$(VIMRTDIR)/xxd.exe
	cp vimrun.exe dist/vim/$(VIMRTDIR)/vimrun.exe
	cp installw32.exe dist/vim/$(VIMRTDIR)/install.exe
	cp uninstallw32.exe dist/vim/$(VIMRTDIR)/uninstall.exe
	mkdir dist/vim/$(VIMRTDIR)/GvimExt32
	cp gvimext.dll dist/vim/$(VIMRTDIR)/GvimExt32/gvimext.dll
	mkdir dist/vim/$(VIMRTDIR)/GvimExt64
	cp gvimext64.dll dist/vim/$(VIMRTDIR)/GvimExt64/gvimext.dll
	cd dist && zip -9 -rD -z gvim$(VERSION).zip vim <$(COMMENT_GVIM)
	cp gvim.pdb dist/gvim$(VERSION).pdb

# make Win32 console
dosbin_w32: dist no_title.vim dist/$(COMMENT_W32)
	-rm -rf dist/vim$(VERSION)w32.zip
	-rm -rf dist/vim
	mkdir dist/vim
	mkdir dist/vim/$(VIMRTDIR)
	tar cf - \
		$(BIN_DOS) \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	cp vimw32.exe dist/vim/$(VIMRTDIR)/vim.exe
	cp teew32.exe dist/vim/$(VIMRTDIR)/tee.exe
	cp xxdw32.exe dist/vim/$(VIMRTDIR)/xxd.exe
	cp installw32.exe dist/vim/$(VIMRTDIR)/install.exe
	cp uninstallw32.exe dist/vim/$(VIMRTDIR)/uninstall.exe
	cd dist && zip -9 -rD -z vim$(VERSION)w32.zip vim <$(COMMENT_W32)
	cp vimw32.pdb dist/vim$(VERSION)w32.pdb

# make Win32 gvim with OLE
dosbin_ole: dist no_title.vim dist/$(COMMENT_OLE)
	-rm -rf dist/gvim$(VERSION)ole.zip
	-rm -rf dist/vim
	mkdir dist/vim
	mkdir dist/vim/$(VIMRTDIR)
	tar cf - \
		$(BIN_DOS) \
		| (cd dist/vim/$(VIMRTDIR); tar xf -)
	cp gvim_ole.exe dist/vim/$(VIMRTDIR)/gvim.exe
	cp teew32.exe dist/vim/$(VIMRTDIR)/tee.exe
	cp xxdw32.exe dist/vim/$(VIMRTDIR)/xxd.exe
	cp vimrun.exe dist/vim/$(VIMRTDIR)/vimrun.exe
	cp installw32.exe dist/vim/$(VIMRTDIR)/install.exe
	cp uninstallw32.exe dist/vim/$(VIMRTDIR)/uninstall.exe
	cp gvimext.dll dist/vim/$(VIMRTDIR)/gvimext.dll
	cp README_ole.txt dist/vim/$(VIMRTDIR)
	cd dist && zip -9 -rD -z gvim$(VERSION)ole.zip vim <$(COMMENT_OLE)
	cp gvim_ole.pdb dist/gvim$(VERSION)ole.pdb

html: dist dist/$(COMMENT_HTML)
	-rm -rf dist/vim$(VERSION)html.zip
	cd runtime/doc && zip -9 -z ../../dist/vim$(VERSION)html.zip *.html <../../dist/$(COMMENT_HTML)

farsi: dist dist/$(COMMENT_FARSI)
	-rm -f dist/farsi$(VERSION).zip
	zip -9 -rD -z dist/farsi$(VERSION).zip farsi < dist/$(COMMENT_FARSI)
