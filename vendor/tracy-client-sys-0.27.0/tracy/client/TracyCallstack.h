#ifndef __TRACYCALLSTACK_H__
#define __TRACYCALLSTACK_H__

#ifndef TRACY_NO_CALLSTACK

#  if !defined _WIN32
#    include <sys/param.h>
#  endif

#  if defined _WIN32
#    include "../common/TracyWinFamily.hpp"
#    if !defined TRACY_WIN32_NO_DESKTOP
#      define TRACY_HAS_CALLSTACK 1
#    endif
#  elif defined __ANDROID__
#    if !defined __arm__ || __ANDROID_API__ >= 21
#      define TRACY_HAS_CALLSTACK 2
#    else
#      define TRACY_HAS_CALLSTACK 5
#    endif
#  elif defined __linux
#    if defined _GNU_SOURCE && defined __GLIBC__
#      define TRACY_HAS_CALLSTACK 3
#    else
#      define TRACY_HAS_CALLSTACK 2
#    endif
#  elif defined __APPLE__
#    define TRACY_HAS_CALLSTACK 4
#  elif defined BSD
#    define TRACY_HAS_CALLSTACK 6
#  endif

#if TRACY_HAS_CALLSTACK == 2 || TRACY_HAS_CALLSTACK == 3 || TRACY_HAS_CALLSTACK == 4 || TRACY_HAS_CALLSTACK == 6
#define TRACY_USE_LIBBACKTRACE
#endif

#endif

#endif
