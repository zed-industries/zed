#!/bin/sh

TOP_DIR=.
OBJ_DIR=.

for x in "$@"
do
  case x"$x" in
  x--srcdir=* | x--topdir=* )
    TOP_DIR=`echo $x | sed 's/^--[a-z]*dir=//'`
    ;;
  x--builddir=* | x--objdir=* )
    OBJ_DIR=`echo $x | sed 's/^--[a-z]*dir=//'`
    ;;
  esac
done

mkdir -p ${OBJ_DIR}/builds/atari/tmp/orig

( cd ${TOP_DIR} && find . -name '*.[CHch]' -type f | fgrep -v builds/atari/tmp | cpio -o ) | \
( cd ${OBJ_DIR}/builds/atari/tmp/orig && cpio -idum )
cp ${TOP_DIR}/builds/atari/deflinejoiner.awk ${OBJ_DIR}/builds/atari/tmp

pushd ${OBJ_DIR}/builds/atari/tmp

  cp -pr orig purec
  for f in `cd orig && find . -type f`
  do
    echo filter $f
    env LANG=C awk -f deflinejoiner.awk < orig/$f > purec/$f
  done

  echo '#define FT2_BUILD_LIBRARY'  >  purec/include/ft2build.h
  echo '#include "ATARI.H"'         >> purec/include/ft2build.h
  env LANG=C awk -f deflinejoiner.awk < orig/include/ft2build.h >> purec/include/ft2build.h

  env LANG=C diff -ur orig purec > ../purec.diff

popd
rm -rf ${OBJ_DIR}/builds/atari/tmp
