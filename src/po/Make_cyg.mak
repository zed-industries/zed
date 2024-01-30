# Makefile for the Vim message translations for Cygwin
# by Tony Mechelynck <antoine.mechelynck@skynet.be>
# after Make_ming.mak by
# Eduardo F. Amatria <eferna1@platea.pntic.mec.es>
#
# Read the README_ming.txt file before using it.
#
# Use at your own risk but with care, it could even kill your canary.
#

ifndef VIMRUNTIME
VIMRUNTIME = ../../runtime
endif

# get LANGUAGES, MOFILES and MOCONVERTED
include Make_all.mak

PACKAGE = vim
VIM = ../vim

# Uncomment one of the lines below or modify it to put the path to your
# gettext binaries
ifndef GETTEXT_PATH
#GETTEXT_PATH = C:/gettext.win32/bin/
#GETTEXT_PATH = C:/gettext-0.10.35-w32/win32/Release/
GETTEXT_PATH = /bin/
endif

# The OLD_PO_FILE_INPUT and OLD_PO_FILE_OUTPUT are for the new GNU gettext
# tools 0.10.37, which use a slightly different .po file format that is not
# compatible with Solaris (and old gettext implementations) unless these are
# set.  gettext 0.10.36 will not work!
MSGFMT = OLD_PO_FILE_INPUT=yes $(GETTEXT_PATH)msgfmt -v
XGETTEXT = OLD_PO_FILE_INPUT=yes OLD_PO_FILE_OUTPUT=yes $(GETTEXT_PATH)xgettext
MSGMERGE = OLD_PO_FILE_INPUT=yes OLD_PO_FILE_OUTPUT=yes $(GETTEXT_PATH)msgmerge

# MV = move
# CP = copy
# RM = del
# MKD = mkdir
MV = mv -f
CP = cp -f
RM = rm -f
MKD = mkdir -p

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
	@$(MAKE) -f Make_cyg.mak $(PACKAGE).pot GETTEXT_PATH=$(GETTEXT_PATH)
	$(CP) $@.po $@.po.orig
	$(MV) $@.po $@.po.old
	$(MSGMERGE) $@.po.old $(PACKAGE).pot -o $@.po
	$(RM) $@.po.old

install: $(MOFILES) $(MOCONVERTED)
	for TARGET in $(LANGUAGES); do \
		$(MKD) $(VIMRUNTIME)/lang/$$TARGET/LC_MESSAGES ; \
		$(CP) $$TARGET.mo $(VIMRUNTIME)/lang/$$TARGET/LC_MESSAGES/$(PACKAGE).mo ; \
	done

install-all: install

clean:
	$(RM) *.mo
	$(RM) *.pot
