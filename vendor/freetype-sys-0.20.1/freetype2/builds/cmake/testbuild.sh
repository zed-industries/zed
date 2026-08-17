#!/bin/sh -e

# Copyright (C) 2015-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.

# This script tests the CMake build. Simply run
#
#     builds/cmake/testbuild.sh
#
# or
#
#     BUILD_SHARED_LIBS=1 builds/cmake/testbuild.sh
#
# The script:
#
# - builds the main CMakeLists.txt
# - builds and runs a small test app in a separate build tree so
#   the config-module is tested, too
#
# Options (environment variables):
#
# - The variable BUILD_SHARED_LIBS will be forwarded to the CMake project
#   that builds the library.
#


# prepare temporary dir

cd `dirname $0`/../..
ftdir=`pwd`
tmpdir=/tmp/freetype-cmake-testbuild
rm -rf $tmpdir
mkdir -p $tmpdir


# build and install freetype

if test -n "$BUILD_SHARED_LIBS"; then
  bsl=-DBUILD_SHARED_LIBS=$BUILD_SHARED_LIBS
else
  bsl=-UBUILD_SHARED_LIBS
fi

build_opts="-DWITH_ZLIB=0 \
            -DWITH_BZip2=0 \
            -DWITH_PNG=0 \
            -DWITH_HarfBuzz=0 \
            $bsl \
            -DCMAKE_INSTALL_PREFIX=$tmpdir/out"

(set -x; cmake -H$ftdir \
               -B$tmpdir/ftb \
               -DCMAKE_BUILD_TYPE=Debug \
               $build_opts)
(set -x; cmake --build $tmpdir/ftb \
               --config Debug \
               --target install)

(set -x; cmake $tmpdir/ftb \
               -DCMAKE_BUILD_TYPE=Release)
(set -x; cmake --build $tmpdir/ftb \
               --config Release \
               --target install \
               --clean-first)


# create test project CMakeLists.txt

cat >$tmpdir/CMakeLists.txt << END
cmake_minimum_required(VERSION 2.6)
project(freetype-cmake-testbuild)

find_package(Freetype REQUIRED CONFIG)

add_executable(freetype-cmake-test main.c)
target_link_libraries(freetype-cmake-test freetype)

enable_testing()
add_test(freetype-cmake-test freetype-cmake-test)
END


# create test project main.c

cat >$tmpdir/main.c << END
#include <stdio.h>
#include <stdlib.h>

#include <ft2build.h>
#include <freetype/freetype.h>


FT_Library library;


int main(int argc,
         char*argv[])
{
  FT_Error error;
  FT_Int major = 0;
  FT_Int minor = 0;
  FT_Int patch = 0;

  error = FT_Init_FreeType(&library);
  if (error)
    return EXIT_FAILURE;

  FT_Library_Version(library, &major, &minor, &patch);
  if (major != FREETYPE_MAJOR
      || minor != FREETYPE_MINOR
      || patch != FREETYPE_PATCH)
    return EXIT_FAILURE;

  printf("FT_Library_Version: %d.%d.%d\n", major, minor, patch);

  error = FT_Done_FreeType(library);
  if (error)
    return EXIT_FAILURE;

  return EXIT_SUCCESS;
}
END


# build and test

mkdir -p $tmpdir/tb
cd $tmpdir/tb

LD_LIBRARY_PATH=$tmpdir/out/lib:$LD_LIBRARY_PATH
DYLD_LIBRARY_PATH=$tmpdir/out/lib:$DYLD_LIBRARY_PATH
export LD_LIBRARY_PATH
export DYLD_LIBRARY_PATH

(set -x; cmake $tmpdir \
               -DCMAKE_BUILD_TYPE=Debug \
               -DCMAKE_PREFIX_PATH=$tmpdir/out)
(set -x; cmake --build . \
               --config Debug)
(set -x; ctest -V -C Debug)

(set -x; cmake . \
               -DCMAKE_BUILD_TYPE=Release)
(set -x; cmake --build . \
               --config Release \
               --clean-first)
(set -x; ctest -V -C Release)

rm -rf $tmpdir

# EOF
