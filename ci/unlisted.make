# vim: ft=make
SHELL = /bin/bash

# List all files that are tracked in git but not listed in Filelist.
# Exit code is 2 ("Make encountered an error") if any such files exist.

# Filelist is a Makefile that defines many variables, so we use Make itself to
# query which variables it defines, then expand them all by wrapping each name
# in $(...), importing Filelist and using $(eval).

include Filelist
$(eval all_patterns := $(shell \
	make -f Filelist --question --print-data-base --no-builtin-rules \
		--no-builtin-variables 2>/dev/null \
	| sed -nre \
		'/^# makefile .from \x27Filelist\x27,/ { \
			n; \
			s/ = .*//; \
			T; \
			s/.*/$$(\0)/; \
			p; \
		}'))

# In Makefile's `prepeare` target, all the IN_README_DIR files are moved from
# READMEdir to the root, so add those files in their Git-tracked location:
all_patterns := $(all_patterns) \
	$(foreach readme, $(IN_README_DIR), READMEdir/$(readme))

# The result 'all_patterns' is a list of patterns (globs), which we expand with
# wildcard to get actual filenames.  Note this means Filelist can list a file
# that does not exist, and it will be omitted at this step.
listed_files := $(wildcard $(all_patterns))

# Default target to actually run the comparison:
.PHONY: check
check:
	@# There are too many files to list on the command line, so we write
	@# that to a temporary file, one per line.
	$(file > Filelist-listed-files)
	$(foreach filename, $(listed_files),\
		$(file >> Filelist-listed-files,$(filename)))
	@# Compare the sorted lists.  Delete that temporary file on both
	@# success and failure, but exit with diff's exit code.
	diff -u0 --label files-in-git <(git ls-files | sort) \
		--label Filelist <(sort --unique Filelist-listed-files); \
	RV=$$?; \
	rm Filelist-listed-files; \
	(($$RV != 0)) && echo "Add files to the right variable in Filelist."; \
	exit $$RV
