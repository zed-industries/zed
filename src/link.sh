#! /bin/sh
#
# link.sh -- try linking Vim with different sets of libraries, finding the
# minimal set for fastest startup.  The problem is that configure adds a few
# libraries when they exist, but this doesn't mean they are needed for Vim.
#
#      Author: Bram Moolenaar
# Last change: 2010 Nov 03
#     License: Public domain
#
# Warning: This fails miserably if the linker doesn't return an error code!
#
# Otherwise this script is fail-safe, falling back to the original full link
# command if anything fails.

echo "$LINK " >link_$PROG.cmd
exit_value=0

if test "$LINK_AS_NEEDED" = yes; then
  echo "link.sh: \$LINK_AS_NEEDED set to 'yes': invoking linker directly."
  cat link_$PROG.cmd
  if sh link_$PROG.cmd; then
    exit_value=0
    echo "link.sh: Linked fine"
  else
    exit_value=$?
    echo "link.sh: Linking failed"
  fi
else
  if test -f auto/link.sed; then

#
# If auto/link.sed already exists, use it.  We assume a previous run of
# link.sh has found the correct set of libraries.
#
  echo "link.sh: The file 'auto/link.sed' exists, which is going to be used now."
  echo "link.sh: If linking fails, try deleting the auto/link.sed file."
  echo "link.sh: If this fails too, try creating an empty auto/link.sed file."
else

# If linking works with the full link command, try removing some libraries,
# that are known not to be needed on at least one system.
# Remove auto/pathdef.c if there is a new link command and compile it again.
# There is a loop to remove libraries that appear several times.
#
# Notes:
# - Can't remove Xext; It links fine but will give an error when running gvim
#   with Motif.
# - Don't remove the last -lm: On HP-UX Vim links OK but crashes when the GTK
#   GUI is started, because the "floor" symbol could not be resolved.
#
  cat link_$PROG.cmd
  if sh link_$PROG.cmd; then
    touch auto/link.sed
    cp link_$PROG.cmd linkit_$PROG.sh
    for libname in SM ICE nsl dnet dnet_stub inet socket dir elf iconv Xt Xmu Xp Xpm X11 Xdmcp x w perl dl pthread thread readline m crypt attr; do
      cont=yes
      while test -n "$cont"; do
        if grep "l$libname " linkit_$PROG.sh >/dev/null; then
          if test ! -f link1_$PROG.sed; then
            echo "link.sh: OK, linking works, let's try omitting a few libraries."
            echo "link.sh: See auto/link.log for details."
            rm -f auto/link.log
          fi
          echo "s/-l$libname  *//" >link1_$PROG.sed
          sed -f auto/link.sed <link_$PROG.cmd >linkit2_$PROG.sh
          sed -f link1_$PROG.sed <linkit2_$PROG.sh >linkit_$PROG.sh
          # keep the last -lm
          if test $libname != "m" || grep "lm " linkit_$PROG.sh >/dev/null; then
            echo "link.sh: Trying to omit the $libname library..."
            cat linkit_$PROG.sh >>auto/link.log
            # Redirect this link output, it may contain error messages which
            # should be ignored.
            if sh linkit_$PROG.sh >>auto/link.log 2>&1; then
              echo "link.sh: Vim doesn't need the $libname library!"
              cat link1_$PROG.sed >>auto/link.sed
              rm -f auto/pathdef.c
            else
              echo "link.sh: Vim DOES need the $libname library."
              cont=
              cp link_$PROG.cmd linkit_$PROG.sh
            fi
          else
            cont=
            cp link_$PROG.cmd linkit_$PROG.sh
          fi
        else
          cont=
          cp link_$PROG.cmd linkit_$PROG.sh
        fi
      done
    done
    if test ! -f auto/pathdef.c; then
      $MAKE objects/pathdef.o
    fi
    if test ! -f link1_$PROG.sed; then
      echo "link.sh: Linked fine, no libraries can be omitted"
      touch link3_$PROG.sed
    fi
  else
    exit_value=$?
  fi
fi

#
# Now do the real linking.
#
if test -s auto/link.sed; then
  echo "link.sh: Using auto/link.sed file to omit a few libraries"
  sed -f auto/link.sed <link_$PROG.cmd >linkit_$PROG.sh
  cat linkit_$PROG.sh
  if sh linkit_$PROG.sh; then
    exit_value=0
    echo "link.sh: Linked fine with a few libraries omitted"
  else
    exit_value=$?
    echo "link.sh: Linking failed, making auto/link.sed empty and trying again"
    mv -f auto/link.sed link2_$PROG.sed
    touch auto/link.sed
    rm -f auto/pathdef.c
    $MAKE objects/pathdef.o
  fi
fi
if test -f auto/link.sed -a ! -s auto/link.sed -a ! -f link3_$PROG.sed; then
  echo "link.sh: Using unmodified link command"
  cat link_$PROG.cmd
  if sh link_$PROG.cmd; then
    exit_value=0
    echo "link.sh: Linked OK"
  else
    exit_value=$?
    if test -f link2_$PROG.sed; then
      echo "link.sh: Linking doesn't work at all, removing auto/link.sed"
      rm -f auto/link.sed
    fi
  fi
fi

fi

#
# cleanup
#
rm -f link_$PROG.cmd linkit_$PROG.sh link1_$PROG.sed link2_$PROG.sed \
  link3_$PROG.sed linkit2_$PROG.sh

#
# return an error code if something went wrong
#
exit $exit_value

# vim:set sw=2 et:
