// A shadow call stack runtime is not yet included with compiler-rt, provide a
// minimal runtime to allocate a shadow call stack and assign an
// architecture-specific register to point at it.

#pragma once

#include <stdlib.h>
#include <sys/mman.h>
#include <sys/prctl.h>

#include "libc_support.h"

__attribute__((no_sanitize("shadow-call-stack")))
static void __shadowcallstack_init() {
  void *stack = mmap(NULL, 8192, PROT_READ | PROT_WRITE,
                     MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
  if (stack == MAP_FAILED)
    abort();

#if defined(__aarch64__)
  __asm__ __volatile__("mov x18, %0" ::"r"(stack));
#else
#error Unsupported platform
#endif
}

int scs_main(void);

__attribute__((no_sanitize("shadow-call-stack"))) int main(void) {
  __shadowcallstack_init();

  // We can't simply return scs_main() because scs_main might have corrupted our
  // return address for testing purposes (see overflow.c), so we need to exit
  // ourselves.
  exit(scs_main());
}
