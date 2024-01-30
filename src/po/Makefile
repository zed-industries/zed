# Makefile for the Vim message translations.

# Include stuff found by configure.
include ../auto/config.mk

# get LANGUAGES, MOFILES, MOCONVERTED and others
include Make_all.mak

# Note: ja.sjis, *.cp1250 and zh_CN.cp936 are only for MS-Windows, they are
# not installed on Unix

PACKAGE = vim
SHELL = /bin/sh
VIM = ../vim

# MacOS sed is locale aware, set $LANG to avoid problems
SED = LANG=C sed

# The OLD_PO_FILE_INPUT and OLD_PO_FILE_OUTPUT are for the new GNU gettext
# tools 0.10.37, which use a slightly different .po file format that is not
# compatible with Solaris (and old gettext implementations) unless these are
# set.  gettext 0.10.36 will not work!
MSGFMTCMD = OLD_PO_FILE_INPUT=yes $(MSGFMT) -v
XGETTEXT = OLD_PO_FILE_INPUT=yes OLD_PO_FILE_OUTPUT=yes xgettext
MSGMERGE = OLD_PO_FILE_INPUT=yes OLD_PO_FILE_OUTPUT=yes msgmerge

.SUFFIXES:
.SUFFIXES: .po .mo .pot .ck
.PHONY: all install uninstall prefixcheck originals converted check clean \
	checkclean distclean update-po $(LANGUAGES)

all: $(MOFILES) $(MOCONVERTED) $(MSGFMT_DESKTOP)

originals: $(MOFILES)

converted: $(MOCONVERTED)

.po.mo:
	$(MSGFMTCMD) -o $@ $<

.po.ck:
	$(VIM) -u NONE --noplugins -e -s -X --cmd "set enc=utf-8" -S check.vim \
		-c "if error == 0 | q | else | num 2 | cq | endif" $<
	touch $@

check: $(CHECKFILES)

# installing for real
install: $(MOFILES) $(MOCONVERTED)
	@$(MAKE) prefixcheck
	for lang in $(LANGUAGES); do \
	  dir=$(LOCALEDIR)/$$lang/; \
	  if test ! -x "$$dir"; then \
	    mkdir $$dir; chmod 755 $$dir; \
	  fi; \
	  dir=$(LOCALEDIR)/$$lang/LC_MESSAGES; \
	  if test ! -x "$$dir"; then \
	    mkdir $$dir; chmod 755 $$dir; \
	  fi; \
	  if test -r $$lang.mo; then \
	    $(INSTALL_DATA) $$lang.mo $$dir/$(PACKAGE).mo; \
	    chmod $(FILEMOD) $$dir/$(PACKAGE).mo; \
	  fi; \
	done

uninstall:
	@$(MAKE) prefixcheck
	for cat in $(MOFILES) $(MOCONVERTED); do \
	  cat=`basename $$cat`; \
	  lang=`echo $$cat | $(SED) 's/\$(CATOBJEXT)$$//'`; \
	  rm -f $(LOCALEDIR)/$$lang/LC_MESSAGES/$(PACKAGE).mo; \
	done

# installing for local tryout into ../../runtime/lang
tryoutinstall: $(MOFILES) $(MOCONVERTED)
	@$(MAKE) prefixcheck
	for lang in $(LANGUAGES); do \
	  dir=../../runtime/lang/$$lang/; \
	  if test ! -x "$$dir"; then \
	    mkdir $$dir; chmod 755 $$dir; \
	  fi; \
	  dir=../../runtime/lang/$$lang/LC_MESSAGES; \
	  if test ! -x "$$dir"; then \
	    mkdir $$dir; chmod 755 $$dir; \
	  fi; \
	  if test -r $$lang.mo; then \
	    cp $$lang.mo $$dir/$(PACKAGE).mo; \
	    chmod 644 $$dir/$(PACKAGE).mo; \
	  fi; \
	done

# nl.po was added later, if it does not exist use a file with just a # in it
# (an empty file doesn't work with old msgfmt).
nl.po:
	@( echo \# >> nl.po )

# Norwegian/Bokmal: "nb" is an alias for "no".
# Copying the file is not efficient, but I don't know of another way to make
# this work.
nb.po: no.po
	cp no.po nb.po

# Convert ja.po to create ja.sjis.po.  Requires doubling backslashes in the
# second byte.  Don't depend on sjiscorr, it should only be compiled when
# ja.sjis.po is outdated.
ja.sjis.po: ja.po
	@$(MAKE) sjiscorr
	rm -f $@
	iconv -f UTF-8 -t CP932 $< | ./sjiscorr > $@

sjiscorr: sjiscorr.c
	$(CC) -o sjiscorr sjiscorr.c

ja.euc-jp.po: ja.po
	iconv -f UTF-8 -t EUC-JP $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=EUC-JP/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert cs.po to create cs.cp1250.po.
cs.cp1250.po: cs.po
	rm -f $@
	iconv -f ISO-8859-2 -t CP1250 $< | \
		$(SED) -e 's/charset=[iI][sS][oO]-8859-2/charset=CP1250/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert pl.po to create pl.cp1250.po.
pl.cp1250.po: pl.po
	rm -f $@
	iconv -f ISO-8859-2 -t CP1250 $< | \
		$(SED) -e 's/charset=[iI][sS][oO]-8859-2/charset=CP1250/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert pl.po to create pl.UTF-8.po.
pl.UTF-8.po: pl.po
	rm -f $@
	iconv -f ISO-8859-2 -t UTF-8 $< | \
		$(SED) -e 's/charset=[iI][sS][oO]-8859-2/charset=UTF-8/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert sk.po to create sk.cp1250.po.
sk.cp1250.po: sk.po
	rm -f $@
	iconv -f ISO-8859-2 -t CP1250 $< | \
		$(SED) -e 's/charset=[iI][sS][oO]-8859-2/charset=CP1250/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert zh_CN.UTF-8.po to create zh_CN.po.
zh_CN.po: zh_CN.UTF-8.po
	rm -f $@
	iconv -f UTF-8 -t GB2312 $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=GB2312/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert zh_CN.UTF-8.po to create zh_CN.cp936.po.
# Set 'charset' to gbk to avoid that msfmt generates a warning.
# This used to convert from zh_CN.po, but that results in a conversion error.
zh_CN.cp936.po: zh_CN.UTF-8.po
	rm -f $@
	iconv -f UTF-8 -t CP936 $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=GBK/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert zh_TW.UTF-8.po to create zh_TW.po
zh_TW.po: zh_TW.UTF-8.po
	rm -f $@
	iconv -f UTF-8 -t BIG5 $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=BIG5/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@


# Convert zh_TW.UTF-8.po to create zh_TW.po with backslash characters
# Requires doubling backslashes in the second byte.  Don't depend on big5corr,
# it should only be compiled when zh_TW.po is outdated.

#
#  06.11.23, added by Restorer
#  For more details, see:
#  https://github.com/vim/vim/pull/3261
#  https://github.com/vim/vim/pull/3476
#  https://github.com/vim/vim/pull/12153
#  (read all comments)
#
#  I checked the workability on the list of backslash characters
#  specified in zh_TW.UTF-8.po. It works.
#  But it is better to have someone native speaker check it.
#

#zh_TW.po: zh_TW.UTF-8.po
#	@$(MAKE) big5corr
#	rm -f $@
#	iconv -f UTF-8 -t BIG5 $< | ./big5corr > $@


# 06.11.23, added by Restorer
# see above in the zh_tw.po conversion section for backslashes.
#big5corr: big5corr.c
#	$(CC) -o big5corr big5corr.c


# Convert ko.UTF-8.po to create ko.po.
ko.po: ko.UTF-8.po
	rm -f $@
	iconv -f UTF-8 -t EUC-KR $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=EUC-KR/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert ru.po to create ru.cp1251.po.
ru.cp1251.po: ru.po
	rm -f $@
	iconv -f UTF-8 -t CP1251 $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=CP1251/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

# Convert uk.po to create uk.cp1251.po.
uk.cp1251.po: uk.po
	rm -f $@
	iconv -f UTF-8 -t CP1251 $< | \
		$(SED) -e 's/charset=[uU][tT][fF]-8/charset=CP1251/' \
			-e 's/# Original translations/# Generated from $<, DO NOT EDIT/' \
			> $@

prefixcheck:
	@if test "x" = "x$(prefix)"; then \
	  echo "******************************************"; \
	  echo "  Please use make from the src directory  "; \
	  echo "******************************************"; \
	  exit 1; \
	fi

clean: checkclean
	rm -f core core.* *.old.po *.mo *.pot sjiscorr
	rm -f LINGUAS vim.desktop gvim.desktop tmp_*desktop
#	rm -f big5corr

distclean: clean

checkclean:
	rm -f *.ck

PO_INPUTLIST = \
	../*.c \
	../if_perl.xs \
	../GvimExt/gvimext.cpp \
	../errors.h \
	../globals.h \
	../if_py_both.h \
	../vim.h \
	gvim.desktop.in \
	vim.desktop.in

$(PACKAGE).pot: $(PO_INPUTLIST) $(PO_VIM_INPUTLIST)
	# Convert the Vim scripts to (what looks like) Javascript
	$(VIM) -u NONE --not-a-term -S tojavascript.vim $(PACKAGE).pot $(PO_VIM_INPUTLIST)
	# create vim.pot
	$(XGETTEXT) --default-domain=$(PACKAGE) --add-comments \
		$(XGETTEXT_KEYWORDS) $(PO_INPUTLIST) $(PO_VIM_JSLIST)
	mv -f $(PACKAGE).po $(PACKAGE).pot
	# Fix Vim scripts names, so that "gf" works
	$(VIM) -u NONE --not-a-term -S fixfilenames.vim $(PACKAGE).pot $(PO_VIM_INPUTLIST)
	# Delete the temporary files
	rm *.js

vim.desktop: vim.desktop.in $(POFILES)
	echo $(LANGUAGES) | tr " " "\n" |$(SED) -e '/\./d' | sort > LINGUAS
	$(MSGFMT) --desktop -d . --template vim.desktop.in -o tmp_vim.desktop
	rm -f LINGUAS
	if command -v desktop-file-validate; then desktop-file-validate tmp_vim.desktop; fi
	mv tmp_vim.desktop vim.desktop

# The dependency on vim.desktop is only to avoid the two targets are build at
# the same time, which causes a race for the LINGUAS file.
gvim.desktop: gvim.desktop.in $(POFILES) vim.desktop
	echo $(LANGUAGES) | tr " " "\n" |$(SED) -e '/\./d' | sort > LINGUAS
	$(MSGFMT) --desktop -d . --template gvim.desktop.in -o tmp_gvim.desktop
	rm -f LINGUAS
	if command -v desktop-file-validate; then desktop-file-validate tmp_gvim.desktop; fi
	mv tmp_gvim.desktop gvim.desktop

# Only original translations with default encoding should be updated.
# The files that are converted to a different encoding clearly state "DO NOT EDIT".
update-po: $(MOFILES:.mo=)

# Don't add a dependency here, we only want to update the .po files manually
$(LANGUAGES):
	@$(MAKE) $(PACKAGE).pot
	if test ! -f $@.po.orig; then cp $@.po $@.po.orig; fi
	mv $@.po $@.po.old
	if $(MSGMERGE) $@.po.old $(PACKAGE).pot -o $@.po; then \
	    rm -f $@.po.old; \
	else \
	    echo "msgmerge for $@.po failed!"; mv $@.po.old $@.po; \
	fi
