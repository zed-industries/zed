#!/bin/sh
# toolcheck -- check for tools that have severe bugs. Good that all the buggy
#	       tools identify by version numbers. This is the spirit of GNU :-)
#
# 24.7.95 jw.

retval=0
reply="`sh -version -c exit 2>&1 < /dev/null`"
case "$reply" in
	GNU*1.14.3*)
		echo "- sh is	'$reply'";
		echo "  CAUTION: This shell has a buggy 'trap' command.";
		echo "           The configure script may fail silently.";
		retval=1;
		;;
	GNU*)
		echo "- sh is	'$reply' - probably OK.";
		;;
	*)	;;
esac

reply="`sed --version 2>&1 < /dev/null`"
case "$reply" in
	GNU\ sed\ version\ 2.0[34])
		echo "- sed is	'$reply'";
		echo "  CAUTION: This sed cannot configure screen properly."
		retval=1;
		;;
	GNU\ sed\ version\ 2.05|GNU\ sed\ version\ 2.03\ kevin)
		echo "- sed is	'$reply' - good.";
		;;
	GNU*)	echo "- sed is	'$reply'.";
		;;
	*)	;;
esac
exit $retval
