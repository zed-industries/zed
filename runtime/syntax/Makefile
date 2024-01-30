# Portable Makefile for running syntax tests.

# Override this if needed, the default assumes Vim was build in the src dir.
#VIMPROG = vim
VIMPROG = ../../src/vim

# "runtime" relative to "runtime/syntax/testdir"
VIMRUNTIME = ../..

# Uncomment this line to use valgrind for memory leaks and extra warnings.
# VALGRIND = valgrind --tool=memcheck --leak-check=yes --num-callers=45 --log-file=valgrind.$*

# ENVVARS = LC_ALL=C LANG=C LANGUAGE=C

RUN_VIMTEST = VIMRUNTIME=$(VIMRUNTIME) $(VALGRIND) $(ENVVARS) ../$(VIMPROG) -f $(GUI_FLAG)

# Uncomment this line for debugging
# DEBUGLOG = --log testlog

# Run the tests that didn't run yet or failed previously.
# If a test succeeds a testdir/done/{name} file will be written.
# If a test fails a testdir/failed/{name}.dump file will be written.
# Progress and error messages can be found in "testdir/messages".
test:
	@# the "vimcmd" file is used by the screendump utils
	@echo "../$(VIMPROG)" > testdir/vimcmd
	@echo "$(RUN_VIMTEST)" >> testdir/vimcmd
	VIMRUNTIME=$(VIMRUNTIME) $(VIMPROG) --clean --not-a-term $(DEBUGLOG) -u testdir/runtest.vim
	@# FIXME: Temporarily show the whole file to find out what goes wrong
	@#if [ -f testdir/messages ]; then tail -n 6 testdir/messages; fi
	@if [ -f testdir/messages ]; then cat testdir/messages; fi


clean testclean:
	rm -f testdir/failed/* testdir/done/* testdir/vimcmd testdir/messages
