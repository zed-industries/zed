#
# Makefile to run all tests for Vim, on Amiga
#
# Requires "rm", "csh" and "diff"!

VIMPROG = /vim

default: nongui

include Make_all.mak

SCRIPTS = $(SCRIPTS_TINY_OUT)

.SUFFIXES: .in .out .res .vim

nongui:	/tmp $(SCRIPTS)
	csh -c echo ALL DONE

clean:
	csh -c \rm -rf *.out Xdir1 Xfind XfakeHOME Xdotest test.ok viminfo

.in.out:
	copy $*.ok test.ok
	$(VIMPROG) -u amiga.vim -U NONE --noplugin --not-a-term -s dotest.in $*.in
	diff test.out $*.ok
	rename test.out $*.out
	-delete X#? ALL QUIET
	-delete test.ok

# Create a directory for temp files
/tmp:
	makedir /tmp

# Manx requires all dependencies, but we stopped updating them.
# Delete the .out file(s) to run test(s).
