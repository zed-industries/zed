#! /bin/sh
#
# pathdef.sh: adjust pathdef.c for auto/link.sed, if it exists
#
if test -s auto/link.sed; then
  cp auto/pathdef.c auto/pathdef.tmp
  sed -f auto/link.sed <auto/pathdef.tmp >auto/pathdef.c
  rm -f auto/pathdef.tmp
fi

# vim:set sw=2 et:
