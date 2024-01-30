# Makefile for the Vim message translations for mingw32
#
# Eduardo F. Amatria <eferna1@platea.pntic.mec.es>
#
# Read the README_ming.txt file before using it.
#
# Use at your own risk but with care, it could even kill your canary.
#
# Previous to all you must have the environment variable LANGUAGE set to your
# language (xx) and add it to the next three lines.
#

ifndef VIMRUNTIME
ifeq (sh.exe, $(SHELL))
VIMRUNTIME = ..\..\runtime
else
VIMRUNTIME = ../../runtime
endif
endif

# get LANGUAGES, MOFILES, MOCONVERTED and others
include Make_all.mak

PACKAGE = vim
ifeq (sh.exe, $(SHELL))
VIM = ..\vim
else
VIM = ../vim
endif

# Uncomment one of the lines below or modify it to put the path to your
# gettex binaries; I use the first
#GETTEXT_PATH = C:/gettext.win32/bin/
#GETTEXT_PATH = C:/gettext-0.10.35-w32/win32/Release/
#GETTEXT_PATH = C:/cygwin/bin/

ifeq (sh.exe, $(SHELL))
MSGFMT = set OLD_PO_FILE_INPUT=yes && $(GETTEXT_PATH)msgfmt -v
XGETTEXT = set OLD_PO_FILE_INPUT=yes && set OLD_PO_FILE_OUTPUT=yes && $(GETTEXT_PATH)xgettext
MSGMERGE = set OLD_PO_FILE_INPUT=yes && set OLD_PO_FILE_OUTPUT=yes && $(GETTEXT_PATH)msgmerge
else
MSGFMT = LANG=C OLD_PO_FILE_INPUT=yes $(GETTEXT_PATH)msgfmt -v
XGETTEXT = LANG=C OLD_PO_FILE_INPUT=yes OLD_PO_FILE_OUTPUT=yes $(GETTEXT_PATH)xgettext
MSGMERGE = LANG=C OLD_PO_FILE_INPUT=yes OLD_PO_FILE_OUTPUT=yes $(GETTEXT_PATH)msgmerge
endif

ifeq (sh.exe, $(SHELL))
MV = move
CP = copy
RM = del
MKD = mkdir
else
MV = mv -f
CP = cp -f
RM = rm -f
MKD = mkdir -p
endif

.SUFFIXES:
.SUFFIXES: .po .mo .pot
.PHONY: first_time all install install-all clean $(LANGUAGES)

.po.mo:
	$(MSGFMT) -o $@ $<

all: $(MOFILES) $(MOCONVERTED)

PO_INPUTLIST = \
	$(wildcard ../*.c) \
	../if_perl.xs \
	../GvimExt/gvimext.cpp \
	../errors.h \
	../globals.h \
	../if_py_both.h \
	../vim.h \
	gvim.desktop.in \
	vim.desktop.in

first_time: $(PO_INPUTLIST) $(PO_VIM_INPUTLIST)
	$(VIM) -u NONE --not-a-term -S tojavascript.vim $(LANGUAGE).pot $(PO_VIM_INPUTLIST)
	$(XGETTEXT) --default-domain=$(LANGUAGE) \
		--add-comments $(XGETTEXT_KEYWORDS) $(PO_INPUTLIST) $(PO_VIM_JSLIST)
	$(VIM) -u NONE --not-a-term -S fixfilenames.vim $(LANGUAGE).pot $(PO_VIM_INPUTLIST)
	$(RM) *.js

$(PACKAGE).pot: $(PO_INPUTLIST) $(PO_VIM_INPUTLIST)
	$(VIM) -u NONE --not-a-term -S tojavascript.vim $(PACKAGE).pot $(PO_VIM_INPUTLIST)
	$(XGETTEXT) --default-domain=$(PACKAGE) \
		--add-comments $(XGETTEXT_KEYWORDS) $(PO_INPUTLIST) $(PO_VIM_JSLIST)
	$(MV) $(PACKAGE).po $(PACKAGE).pot
	$(VIM) -u NONE --not-a-term -S fixfilenames.vim $(PACKAGE).pot $(PO_VIM_INPUTLIST)
	$(RM) *.js

# Don't add a dependency here, we only want to update the .po files manually
$(LANGUAGES):
	@$(MAKE) -f Make_ming.mak $(PACKAGE).pot GETTEXT_PATH=$(GETTEXT_PATH)
	$(CP) $@.po $@.po.orig
	$(MV) $@.po $@.po.old
	$(MSGMERGE) $@.po.old $(PACKAGE).pot -o $@.po
	$(RM) $@.po.old

install:
	$(MKD) $(VIMRUNTIME)\lang\$(LANGUAGE)
	$(MKD) $(VIMRUNTIME)\lang\$(LANGUAGE)\LC_MESSAGES
	$(CP) $(LANGUAGE).mo $(VIMRUNTIME)\lang\$(LANGUAGE)\LC_MESSAGES\$(PACKAGE).mo

ifeq (sh.exe, $(SHELL))
install-all: all
	FOR %%l IN ($(LANGUAGES)) DO @IF NOT EXIST $(VIMRUNTIME)\lang\%%l $(MKD) $(VIMRUNTIME)\lang\%%l
	FOR %%l IN ($(LANGUAGES)) DO @IF NOT EXIST $(VIMRUNTIME)\lang\%%l\LC_MESSAGES $(MKD) $(VIMRUNTIME)\lang\%%l\LC_MESSAGES
	FOR %%l IN ($(LANGUAGES)) DO @$(CP) %%l.mo $(VIMRUNTIME)\lang\%%l\LC_MESSAGES\$(PACKAGE).mo
else
install-all: all
	for TARGET in $(LANGUAGES); do \
		$(MKD) $(VIMRUNTIME)/lang/$$TARGET/LC_MESSAGES ; \
		$(CP) $$TARGET.mo $(VIMRUNTIME)/lang/$$TARGET/LC_MESSAGES/$(PACKAGE).mo ; \
	done
endif

clean:
	$(RM) *.mo
	$(RM) *.pot
