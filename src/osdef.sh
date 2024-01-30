#! /bin/sh
#
# osdef.sh -- copy osdef.h.in to osdef.h while removing declarations
# found in the system header files. Caution: weird sed magic going on here.
# Warnings are printed if sed did not survive.
#
# (C) Michael Schroeder, Juergen Weigert
#
# osdef.h.in has been split into osdef1.h.in and osdef2.h.in, because some
# sed's could not handle the amount of commands (is 50 commands the limit?).
#
# 31.10.95 jw.

if test -z "$CC"; then
  CC=cc
fi
if test -z "$srcdir"; then
  srcdir=.
fi

# Make sure collation works as expected
# swedish range [a-z] does not match 'w'
export LC_COLLATE=C
export LC_ALL=

rm -f core* *.core

cat << EOF > osdef0.c
#ifndef __APPLE__
# define select select_declared_wrong
#endif
#define tgetstr tgetstr_declared_wrong
#include "auto/config.h"
#include "os_unix.h"	/* bring in most header files, more follow below */
#include "os_unixx.h"	/* bring in header files for os_unix.c */

#ifdef HAVE_TERMCAP_H
# include <termcap.h>	/* only for term.c */
#endif

#ifdef HAVE_FCNTL_H
# include <fcntl.h>		/* only used in a few files */
#endif

#ifdef HAVE_SYS_STATFS_H
# include <sys/types.h>
# include <sys/statfs.h>	/* only for memfile.c */
#endif

#ifdef HAVE_X11
# include <X11/Intrinsic.h>
#endif
EOF

$CC -I. -I$srcdir -E osdef0.c >osdef0.cc

# insert a space in front of each line, so that a function name at the
# start of the line is matched with "[)*, 	]\1[ 	(]"
sed < osdef0.cc -e '/\(..*\)/s// \1/' > osdef0.ccc

sed < $srcdir/osdef1.h.in -n -e '/^extern/s@.*[)* 	][)* 	]*\([a-zA-Z_][a-zA-Z0-9_]*\)(.*@/[)*, 	][(]*\1[)]*[ 	(]/i\\\
\\/\\[^a-zA-Z_\\]\1(\\/d@p' > osdef11.sed

sed < $srcdir/osdef2.h.in -n -e '/^extern/s@.*[)* 	][)* 	]*\([a-zA-Z_][a-zA-Z0-9_]*\)(.*@/[)*, 	][(]*\1[)]*[ 	(]/i\\\
\\/\\[^a-zA-Z_\\]\1(\\/d@p' > osdef21.sed

cat << EOF > osdef2.sed
1i\\
/*
1i\\
 * osdef.h is automagically created from osdef?.h.in by osdef.sh -- DO NOT EDIT
1i\\
 */
EOF

cat osdef0.ccc | sed -n -f osdef11.sed >> osdef2.sed
sed -f osdef2.sed < $srcdir/osdef1.h.in > auto/osdef.h

cat osdef0.ccc | sed -n -f osdef21.sed > osdef2.sed
sed -f osdef2.sed < $srcdir/osdef2.h.in >> auto/osdef.h

rm osdef0.c osdef0.cc osdef0.ccc osdef11.sed osdef21.sed osdef2.sed

if test -f core*; then
  file core*
  echo "  Sorry, your sed is broken. Call the system administrator."
  echo "  Meanwhile, you may try to compile Vim with an empty osdef.h file."
  echo "  If you compiler complains about missing prototypes, move the needed"
  echo "  ones from osdef1.h.in and osdef2.h.in to osdef.h."
  exit 1
fi
cat $srcdir/osdef1.h.in $srcdir/osdef2.h.in >osdefX.h.in
if eval test "`diff auto/osdef.h osdefX.h.in | wc -l`" -eq 4; then
  echo "  Hmm, sed is very pessimistic about your system header files."
  echo "  But it did not dump core -- strange! Let's continue carefully..."
  echo "  If this fails, you may want to remove offending lines from osdef.h"
  echo "  or try with an empty osdef.h file, if your compiler can do without"
  echo "  function declarations."
fi
rm osdefX.h.in
