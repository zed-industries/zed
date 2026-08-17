## FreeType specific autoconf tests
#
# Copyright (C) 2002-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.

# serial 2

AC_DEFUN([FT_MUNMAP_PARAM],
  [AC_MSG_CHECKING([for munmap's first parameter type])
   AC_COMPILE_IFELSE([
       AC_LANG_SOURCE([[

#include <unistd.h>
#include <sys/mman.h>
int munmap(void *, size_t);

       ]])
     ],
     [AC_MSG_RESULT([void *])
      AC_DEFINE([MUNMAP_USES_VOIDP],
        [],
        [Define to 1 if the first argument of munmap is of type void *])],
     [AC_MSG_RESULT([char *])])
  ])

# end of ft-munmap.m4
