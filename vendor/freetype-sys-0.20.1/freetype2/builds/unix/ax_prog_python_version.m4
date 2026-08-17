# ===========================================================================
#  https://www.gnu.org/software/autoconf-archive/ax_prog_python_version.html
# ===========================================================================
#
# SYNOPSIS
#
#   AX_PROG_PYTHON_VERSION([VERSION],[ACTION-IF-TRUE],[ACTION-IF-FALSE])
#
# DESCRIPTION
#
#   Makes sure that python supports the version indicated. If true the shell
#   commands in ACTION-IF-TRUE are executed. If not the shell commands in
#   ACTION-IF-FALSE are run. Note if $PYTHON is not set (for example by
#   running AC_CHECK_PROG or AC_PATH_PROG) the macro will fail.
#
#   Example:
#
#     AC_PATH_PROG([PYTHON],[python])
#     AX_PROG_PYTHON_VERSION([2.4.4],[ ... ],[ ... ])
#
#   This will check to make sure that the python you have supports at least
#   version 2.4.4.
#
#   NOTE: This macro uses the $PYTHON variable to perform the check.
#   AX_WITH_PYTHON can be used to set that variable prior to running this
#   macro. The $PYTHON_VERSION variable will be valorized with the detected
#   version.
#
# LICENSE
#
#   Copyright (c) 2009 Francesco Salvestrini <salvestrini@users.sourceforge.net>
#
#   Copying and distribution of this file, with or without modification, are
#   permitted in any medium without royalty provided the copyright notice
#   and this notice are preserved. This file is offered as-is, without any
#   warranty.

#serial 12

AC_DEFUN([AX_PROG_PYTHON_VERSION],[
    AC_REQUIRE([AC_PROG_SED])
    AC_REQUIRE([AC_PROG_GREP])

    AS_IF([test -n "$PYTHON"],[
        ax_python_version="$1"

        AC_MSG_CHECKING([for python version])
        changequote(<<,>>)
        python_version=`$PYTHON -V 2>&1 | $GREP "^Python " | $SED -e 's/^.* \([0-9]*\.[0-9]*\.[0-9]*\)/\1/'`
        changequote([,])
        AC_MSG_RESULT($python_version)

	AC_SUBST([PYTHON_VERSION],[$python_version])

        AX_COMPARE_VERSION([$ax_python_version],[le],[$python_version],[
	    :
            $2
        ],[
	    :
            $3
        ])
    ],[
        AC_MSG_WARN([could not find the python interpreter])
        $3
    ])
])
