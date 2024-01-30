#
# Makefile to run all tests for Vim, on Dos-like machines.
#
# Requires a set of Unix tools: echo, diff, etc.

# Testing may be done with a debug build 
!IF EXIST(..\\vimd.exe) && !EXIST(..\\vim.exe)
VIMPROG = ..\\vimd
!ELSE
VIMPROG = ..\\vim
!ENDIF


default: nongui

!include Make_all.mak

# Explicit dependencies.
test_options.res test_alot.res: opt_test.vim

TEST_OUTFILES = $(SCRIPTS_TINY_OUT)
DOSTMP = dostmp
DOSTMP_OUTFILES = $(TEST_OUTFILES:test=dostmp\test)
DOSTMP_INFILES = $(DOSTMP_OUTFILES:.out=.in)

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
# 	nmake -f Make_mvc.mak test_largefile
$(NEW_TESTS):
	-if exist $@.res del $@.res
	-if exist test.log del test.log
	-if exist messages del messages
	-if exist starttime del starttime
	@$(MAKE) -nologo -f Make_mvc.mak $@.res VIMPROG=$(VIMPROG)
	@type messages
	@if exist test.log exit 1


# Delete files that may interfere with running tests.  This includes some files
# that may result from working on the tests, not only from running them.
clean:
	-if exist *.out del *.out
	-if exist *.failed del *.failed
	-if exist *.res del *.res
	-if exist $(DOSTMP) rd /s /q $(DOSTMP)
	-if exist test.in del test.in
	-if exist test.ok del test.ok
	-if exist Xdir1 rd /s /q Xdir1
	-if exist Xfind rd /s /q Xfind
	-if exist XfakeHOME rd /s /q XfakeHOME
	-if exist X* del X*
	-for /d %i in (X*) do @rd /s/q %i
	-if exist viminfo del viminfo
	-if exist test.log del test.log
	-if exist test_result.log del test_result.log
	-if exist messages del messages
	-if exist starttime del starttime
	-if exist benchmark.out del benchmark.out
	-if exist opt_test.vim del opt_test.vim
	-if exist guidialog del guidialog
	-if exist guidialogfile del guidialogfile

nolog:
	-if exist test.log del test.log
	-if exist test_result.log del test_result.log
	-if exist messages del messages
	-if exist starttime del starttime


# Tiny tests.  Works even without the +eval feature.
tinytests: $(SCRIPTS_TINY_OUT)

# Copy the input files to dostmp, changing the fileformat to dos.
$(DOSTMP_INFILES): $(*B).in
	if not exist $(DOSTMP)\NUL md $(DOSTMP)
	if exist $@ del $@
	$(VIMPROG) -u dos.vim $(COMMON_ARGS) "+set ff=dos|f $@|wq" $(*B).in

# For each input file dostmp/test99.in run the tests.
# This moves test99.in to test99.in.bak temporarily.
$(TEST_OUTFILES): $(DOSTMP)\$(*B).in
	-@if exist test.out DEL test.out
	-@if exist $(DOSTMP)\$(*B).out DEL $(DOSTMP)\$(*B).out
	move $(*B).in $(*B).in.bak > nul
	copy $(DOSTMP)\$(*B).in $(*B).in > nul
	copy $(*B).ok test.ok > nul
	$(VIMPROG) -u dos.vim $(COMMON_ARGS) -s dotest.in $(*B).in
	-@if exist test.out MOVE /y test.out $(DOSTMP)\$(*B).out > nul
	-@if exist $(*B).in.bak move /y $(*B).in.bak $(*B).in > nul
	-@if exist test.ok del test.ok
	-@if exist Xdir1 rd /s /q Xdir1
	-@if exist Xfind rd /s /q Xfind
	-@if exist XfakeHOME rd /s /q XfakeHOME
	-@del X*
	-@if exist viminfo del viminfo
	$(VIMPROG) -u dos.vim $(COMMON_ARGS) "+set ff=unix|f test.out|wq" \
		$(DOSTMP)\$(*B).out
	@diff test.out $*.ok & if errorlevel 1 \
		( move /y test.out $*.failed > nul \
		 & del $(DOSTMP)\$(*B).out \
		 & echo $* FAILED >> test.log ) \
		else ( move /y test.out $*.out > nul )


# New style of tests uses Vim script with assert calls.  These are easier
# to write and a lot easier to read and debug.
# Limitation: Only works with the +eval feature.

newtests: newtestssilent
	@if exist messages type messages

newtestssilent: $(NEW_TESTS_RES)

.vim.res:
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S runtest.vim $*.vim
	@del vimcmd

test_gui.res: test_gui.vim
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S runtest.vim $*.vim
	@del vimcmd

test_gui_init.res: test_gui_init.vim
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u gui_preinit.vim -U gui_init.vim $(NO_PLUGINS) -S runtest.vim $*.vim
	@del vimcmd

opt_test.vim: ../optiondefs.h gen_opt_test.vim
	$(VIMPROG) -u NONE -S gen_opt_test.vim --noplugin --not-a-term ../optiondefs.h

test_bench_regexp.res: test_bench_regexp.vim
	-if exist benchmark.out del benchmark.out
	@echo $(VIMPROG) > vimcmd
	$(VIMPROG) -u NONE $(COMMON_ARGS) -S runtest.vim $*.vim
	@del vimcmd
	@IF EXIST benchmark.out ( type benchmark.out )
