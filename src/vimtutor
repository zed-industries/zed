#!/bin/sh

# Start Vim on a copy of the tutor file.

# Usage: vimtutor [-g] [xx]
# Where optional argument -g starts vimtutor in gvim (GUI) instead of vim.
# and xx is a language code like "es" or "nl".
# When an argument is given, it tries loading that tutor.
# When this fails or no argument was given, it tries using 'v:lang'
# When that also fails, it uses the English version.

# Vim could be called "vim" or "vi".  Also check for "vimN", for people who
# have Vim installed with its version number.
# We anticipate up to a future Vim 8.1 version :-).
seq="vim vim81 vim80 vim8 vim74 vim73 vim72 vim71 vim70 vim7 vim6 vi"
if test "$1" = "-g"; then
    # Try to use the GUI version of Vim if possible, it will fall back
    # on Vim if Gvim is not installed.
    seq="gvim gvim81 gvim80 gvim8 gvim74 gvim73 gvim72 gvim71 gvim70 gvim7 gvim6 $seq"
    shift
fi

xx=$1
export xx

# We need a temp file for the copy.  First try using a standard command.
tmp="${TMPDIR-/tmp}"
TUTORCOPY=`mktemp $tmp/tutorXXXXXX || tempfile -p tutor || echo none`

# If the standard commands failed then create a directory to put the copy in.
# That is a secure way to make a temp file.
if test "$TUTORCOPY" = none; then
	tmpdir=$tmp/vimtutor$$
	OLD_UMASK=`umask`
	umask 077
	getout=no
	mkdir $tmpdir || getout=yes
	umask $OLD_UMASK
	if test $getout = yes; then
		echo "Could not create directory for tutor copy, exiting."
		exit 1
	fi
	TUTORCOPY=$tmpdir/tutorcopy
	touch $TUTORCOPY
	TODELETE=$tmpdir
else
	TODELETE=$TUTORCOPY
fi

export TUTORCOPY

# remove the copy of the tutor on exit
trap "rm -rf $TODELETE" 0 1 2 3 9 11 13 15

for i in $seq; do
    testvim=$(which $i 2>/dev/null)
    if test -f "$testvim"; then
        VIM=$i
        break
    fi
done

# When no Vim version was found fall back to "vim", you'll get an error message
# below.
if test -z "$VIM"; then
    VIM=vim
fi

# Use Vim to copy the tutor, it knows the value of $VIMRUNTIME
# The script tutor.vim tells Vim which file to copy
$VIM -f -u NONE -c 'so $VIMRUNTIME/tutor/tutor.vim'

# Start vim without any .vimrc, set 'nocompatible' and 'showcmd'
$VIM -f -u NONE -c "set nocp showcmd" "$TUTORCOPY"
