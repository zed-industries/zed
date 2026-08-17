// This header provides replacements for certain libc functions. It is necessary
// in order to safely run the tests on aarch64, because the system libc might
// not have been compiled with -ffixed-x18.

#pragma once

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#ifdef __aarch64__

size_t scs_strlen(const char *p) {
  size_t retval = 0;
  while (*p++)
    retval++;
  return retval;
}

// We mark this function as noinline to make sure that its callers do not
// become leaf functions as a result of inlining. This is because we want to
// make sure that we generate the correct code for non-leaf functions.

__attribute__((noinline)) void scs_fputs_stdout(const char *p) {
  __asm__ __volatile__(
      "mov x0, #1\n"  // stdout
      "mov x1, %0\n"
      "mov x2, %1\n"
      "mov x8, #64\n"  // write
      "svc #0\n" ::"r"(p),
      "r"(scs_strlen(p))
      : "x0", "x1", "x2", "x8");
}

#else
#error Unsupported platform
#endif
