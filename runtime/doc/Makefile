#
# Makefile for the Vim documentation on Unix
#
# If you get "don't know how to make scratch", first run make in the source
# directory.  Or remove the include below.

AWK = awk

# Set to $(VIMTARGET) when executed from src/Makefile.
VIMEXE = vim

# include the config.mk from the source directory.  It's only needed to set
# AWK, used for "make html".  Comment this out if the include gives problems.
include ../../src/auto/config.mk

# Common components
include Make_all.mak

.SUFFIXES:
.SUFFIXES: .c .o .txt .html
.PHONY: all vimtags noerrors perlhtml clean test_urls

all: tags vim.man evim.man vimdiff.man vimtutor.man xxd.man $(CONVERTED)

# Use Vim to generate the tags file.  Can only be used when Vim has been
# compiled and installed.  Supports multiple languages.
vimtags: $(DOCS)
	@$(VIMEXE) --clean -esX -V1 -u doctags.vim

# Use "doctags" to generate the tags file.  Only works for English!
tags: doctags $(DOCS)
	./doctags $(DOCS) | LANG=C LC_ALL=C sort >$@
	uniq -d -2 tags

doctags: doctags.c
	$(CC) doctags.c -o doctags

vim.man: vim.1
	nroff -man $< | sed -e s/.//g > $@

evim.man: evim.1
	nroff -man $< | sed -e s/.//g > $@

vimdiff.man: vimdiff.1
	nroff -man $< | sed -e s/.//g > $@

vimtutor.man: vimtutor.1
	nroff -man $< | sed -e s/.//g > $@

xxd.man: xxd.1
	nroff -man $< | sed -e s/.//g > $@

uganda.nsis.txt: uganda.txt
	sed -e 's/[ 	]*\*[-a-zA-Z0-9.]*\*//g' -e 's/vim:tw=78:.*//' \
		uganda.txt | uniq >uganda.nsis.txt

# Awk version of .txt to .html conversion.
html: noerrors tags $(HTMLS)
	@if test -f errors.log; then more errors.log; fi

noerrors:
	-rm -f errors.log

$(HTMLS): tags.ref

.txt.html:
	$(AWK) -f makehtml.awk $< >$@

# index.html is the starting point for HTML, but for the help files it is
# help.txt.  Therefore use vimindex.html for index.txt.
index.html: help.txt
	$(AWK) -f makehtml.awk $< >$@

vimindex.html: index.txt
	$(AWK) -f makehtml.awk $< >$@

tags.ref tags.html: tags
	$(AWK) -f maketags.awk tags >tags.html

# Perl version of .txt to .html conversion.
# There can't be two rules to produce a .html from a .txt file.
# Just run over all .txt files each time one changes.  It's fast anyway.
perlhtml: tags $(DOCS)
	./vim2html.pl tags $(DOCS)

# Check URLs in the help with "curl".
test_urls:
	$(VIMEXE) --clean -S test_urls.vim

clean:
	-rm -f doctags *.html tags.ref

# These files are in the extra archive, skip if not present

arabic.txt:
	touch $@

farsi.txt:
	touch $@

hebrew.txt:
	touch $@

russian.txt:
	touch $@

gui_w32.txt:
	touch $@

if_ole.txt:
	touch $@

os_390.txt:
	touch $@

os_amiga.txt:
	touch $@

os_beos.txt:
	touch $@

os_dos.txt:
	touch $@

os_haiku.txt:
	touch $@

os_mac.txt:
	touch $@

os_mint.txt:
	touch $@

os_msdos.txt:
	touch $@

os_os2.txt:
	touch $@

os_qnx.txt:
	touch $@

os_risc.txt:
	touch $@

os_win32.txt:
	touch $@

# Note that $< works with GNU make while $> works for BSD make.
# Is there a solution that works for both??
vim-da.UTF-8.1: vim-da.1
	iconv -f latin1 -t utf-8 $< >$@

vimdiff-da.UTF-8.1: vimdiff-da.1
	iconv -f latin1 -t utf-8 $< >$@

vimtutor-da.UTF-8.1: vimtutor-da.1
	iconv -f latin1 -t utf-8 $< >$@

vim-de.UTF-8.1: vim-de.1
	iconv -f latin1 -t utf-8 $< >$@

vim-fr.UTF-8.1: vim-fr.1
	iconv -f latin1 -t utf-8 $< >$@

evim-fr.UTF-8.1: evim-fr.1
	iconv -f latin1 -t utf-8 $< >$@

vimdiff-fr.UTF-8.1: vimdiff-fr.1
	iconv -f latin1 -t utf-8 $< >$@

vimtutor-fr.UTF-8.1: vimtutor-fr.1
	iconv -f latin1 -t utf-8 $< >$@

xxd-fr.UTF-8.1: xxd-fr.1
	iconv -f latin1 -t utf-8 $< >$@

vim-it.UTF-8.1: vim-it.1
	iconv -f latin1 -t utf-8 $< >$@

evim-it.UTF-8.1: evim-it.1
	iconv -f latin1 -t utf-8 $< >$@

vimdiff-it.UTF-8.1: vimdiff-it.1
	iconv -f latin1 -t utf-8 $< >$@

vimtutor-it.UTF-8.1: vimtutor-it.1
	iconv -f latin1 -t utf-8 $< >$@

xxd-it.UTF-8.1: xxd-it.1
	iconv -f latin1 -t utf-8 $< >$@

vim-pl.UTF-8.1: vim-pl.1
	iconv -f latin2 -t utf-8 $< >$@

evim-pl.UTF-8.1: evim-pl.1
	iconv -f latin2 -t utf-8 $< >$@

vimdiff-pl.UTF-8.1: vimdiff-pl.1
	iconv -f latin2 -t utf-8 $< >$@

vimtutor-pl.UTF-8.1: vimtutor-pl.1
	iconv -f latin2 -t utf-8 $< >$@

xxd-pl.UTF-8.1: xxd-pl.1
	iconv -f latin2 -t utf-8 $< >$@

vim-ru.UTF-8.1: vim-ru.1
	iconv -f KOI8-R -t utf-8 $< >$@

evim-ru.UTF-8.1: evim-ru.1
	iconv -f KOI8-R -t utf-8 $< >$@

vimdiff-ru.UTF-8.1: vimdiff-ru.1
	iconv -f KOI8-R -t utf-8 $< >$@

vimtutor-ru.UTF-8.1: vimtutor-ru.1
	iconv -f KOI8-R -t utf-8 $< >$@

xxd-ru.UTF-8.1: xxd-ru.1
	iconv -f KOI8-R -t utf-8 $< >$@

vim-tr.UTF-8.1: vim-tr.1
	iconv -f latin5 -t utf-8 $< >$@

evim-tr.UTF-8.1: evim-tr.1
	iconv -f latin5 -t utf-8 $< >$@

vimdiff-tr.UTF-8.1: vimdiff-tr.1
	iconv -f latin5 -t utf-8 $< >$@

vimtutor-tr.UTF-8.1: vimtutor-tr.1
	iconv -f latin5 -t utf-8 $< >$@
