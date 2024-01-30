#
# Makefile to run all tests for Vim, on Dos-like machines
# with sh.exe or zsh.exe in the path or not.
#
# Author: Bill McCarthy
#
# Requires a set of Unix tools: echo, diff, etc.

# Don't use unix-like shell.
SHELL = cmd.exe

DEL = del
DELDIR = rd /s /q
MV = move /y
CP = copy /y
CAT = type

VIMPROG = ..\\vim

default: nongui

include Make_all.mak

# Explicit dependencies.
test_options.res test_alot.res: opt_test.vim

TEST_OUTFILES = $(SCRIPTS_TINY_OUT)
DOSTMP = dostmp
# Keep $(DOSTMP)/*.in
.PRECIOUS: $(patsubst %.out, $(DOSTMP)/%.in, $(TEST_OUTFILES))

.SUFFIXES: .in .out .res .vim

# Add --gui-dialog-file to avoid getting stuck in a dialog.
COMMON_ARGS = $(NO_INITS) --gui-dialog-file guidialog

nongui:	nolog tinytests newtests report

gui:	nolog tinytests newtests report

tiny:	nolog tinytests report

benchmark: $(SCRIPTS_BENCH)

report:
	@rem without the +eval feature test_result.log is a copy of test.log
	@if exist test.log ( copy /y test.log test_result.log > nul ) \
		else ( echo No failures reported > test_result.log )
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S summarize.vim messages
	-if exist starttime del starttime
	@echo.
	@echo Test results:
	@cmd /c type test_result.log
	@if exist test.log ( echo TEST FAILURE & exit /b 1 ) \
		else ( echo ALL DONE )


# Execute an individual new style test, e.g.:
# 	mingw32-make -f Make_ming.mak test_largefile
$(NEW_TESTS):
	-if exist $@.res del $@.res
	-if exist test.log del test.log
	-if exist messages del messages
	-if exist starttime del starttime
	@$(MAKE) -f Make_ming.mak $@.res VIMPROG=$(VIMPROG) --no-print-directory
	@type messages
	@if exist test.log exit 1


# Delete files that may interfere with running tests.  This includes some files
# that may result from working on the tests, not only from running them.
clean:
	-@if exist *.out $(DEL) *.out
	-@if exist *.failed $(DEL) *.failed
	-@if exist *.res $(DEL) *.res
	-@if exist $(DOSTMP) rd /s /q $(DOSTMP)
	-@if exist test.in $(DEL) test.in
	-@if exist test.ok $(DEL) test.ok
	-@if exist Xdir1 $(DELDIR) Xdir1
	-@if exist Xfind $(DELDIR) Xfind
	-@if exist XfakeHOME $(DELDIR) XfakeHOME
	-@if exist X* $(DEL) X*
	-@for /d %%i in (X*) do @rd /s/q %%i
	-@if exist viminfo $(DEL) viminfo
	-@if exist test.log $(DEL) test.log
	-@if exist test_result.log del test_result.log
	-@if exist messages $(DEL) messages
	-@if exist starttime $(DEL) starttime
	-@if exist benchmark.out del benchmark.out
	-@if exist opt_test.vim $(DEL) opt_test.vim
	-@if exist guidialog $(DEL) guidialog
	-@if exist guidialogfile $(DEL) guidialogfile

nolog:
	-@if exist test.log $(DEL) test.log
	-@if exist test_result.log del test_result.log
	-@if exist messages $(DEL) messages
	-@if exist starttime $(DEL) starttime


# Tiny tests.  Works even without the +eval feature.
tinytests: $(SCRIPTS_TINY_OUT)

# Copy the input files to dostmp, changing the fileformat to dos.
$(DOSTMP)/%.in : %.in
	if not exist $(DOSTMP)\nul mkdir $(DOSTMP)
	if exist $(DOSTMP)\$< $(DEL) $(DOSTMP)\$<
	$(VIMPROG) -u dos.vim $(COMMON_ARGS) "+set ff=dos|f $@|wq" $<

# For each input file dostmp/test99.in run the tests.
# This moves test99.in to test99.in.bak temporarily.
%.out : $(DOSTMP)/%.in
	-@if exist test.out $(DEL) test.out
	-@if exist $(DOSTMP)\$@ $(DEL) $(DOSTMP)\$@
	$(MV) $(notdir $<) $(notdir $<).bak > NUL
	$(CP) $(DOSTMP)\$(notdir $<) $(notdir $<) > NUL
	$(CP) $(basename $@).ok test.ok > NUL
	$(VIMPROG) -u dos.vim $(COMMON_ARGS) -s dotest.in $(notdir $<)
	-@if exist test.out $(MV) test.out $(DOSTMP)\$@ > NUL
	-@if exist $(notdir $<).bak $(MV) $(notdir $<).bak $(notdir $<) > NUL
	-@if exist test.ok $(DEL) test.ok
	-@if exist Xdir1 $(DELDIR) /s /q Xdir1
	-@if exist Xfind $(DELDIR) Xfind
	-@if exist XfakeHOME $(DELDIR) XfakeHOME
	-@del X*
	-@if exist viminfo del viminfo
	$(VIMPROG) -u dos.vim $(COMMON_ARGS) "+set ff=unix|f test.out|wq" \
		$(DOSTMP)\$@
	@diff test.out $(basename $@).ok & if errorlevel 1 \
		( $(MV) test.out $(basename $@).failed > NUL \
		 & del $(DOSTMP)\$@ \
		 & echo $(basename $@) FAILED >> test.log ) \
		else ( $(MV) test.out $(basename $@).out > NUL )


# New style of tests uses Vim script with assert calls.  These are easier
# to write and a lot easier to read and debug.
# Limitation: Only works with the +eval feature.

newtests: newtestssilent
	@if exist messages type messages

newtestssilent: $(NEW_TESTS_RES)

.vim.res:
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S runtest.vim $*.vim
	@$(DEL) vimcmd

test_gui.res: test_gui.vim
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S runtest.vim $<
	@$(DEL) vimcmd

test_gui_init.res: test_gui_init.vim
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u gui_preinit.vim -U gui_init.vim $(NO_PLUGINS) -S runtest.vim $<
	@$(DEL) vimcmd

opt_test.vim: ../optiondefs.h gen_opt_test.vim
	$(VIMPROG) -u NONE -S gen_opt_test.vim --noplugin --not-a-term ../optiondefs.h

test_bench_regexp.res: test_bench_regexp.vim
	-$(DEL) benchmark.out
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S runtest.vim $*.vim
	@$(DEL) vimcmd
	$(CAT) benchmark.out
