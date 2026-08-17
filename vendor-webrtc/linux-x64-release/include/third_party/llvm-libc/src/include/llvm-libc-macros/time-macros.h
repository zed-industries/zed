#ifndef LLVM_LIBC_MACROS_TIME_MACROS_H
#define LLVM_LIBC_MACROS_TIME_MACROS_H

#if defined(__AMDGPU__) || defined(__NVPTX__)
#include "gpu/time-macros.h"
#elif defined(__linux__)
#include "linux/time-macros.h"
#endif

#define TIME_UTC 1
#define TIME_MONOTONIC 2
#define TIME_ACTIVE 3
#define TIME_THREAD_ACTIVE 4

#endif // LLVM_LIBC_MACROS_TIME_MACROS_H
