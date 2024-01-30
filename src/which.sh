#! /bin/sh
#
# which.sh -- find where an executable is located.  It's here because the
# "which" command is not supported everywhere.  Used by Makefile.

IFS=":"
for ac_dir in $PATH; do
	if test -f "$ac_dir/$1"; then
		echo "$ac_dir/$1"
		break
	fi
done
