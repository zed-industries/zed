#include <stdint.h>
#include <stdlib.h>

#define CONCAT2(a, b) a##b
#define CONCAT(a, b) CONCAT2(a, b)
#define VERSIONED_SYMBOL(a) CONCAT(a, VERSIONED_SUFFIX)

#ifdef CFG_TARGET_OS_windows
// export required for external access.
__declspec(dllexport)
#else
// Note the `weak` linkage here, though, which is intended to let other code
// override this symbol if it's defined elsewhere, since this definition doesn't
// matter.
// Just in case cross-language LTO is enabled we set the `noinline` attribute
// and also try to have some sort of side effect in this function with a dummy
// `asm` statement.
__attribute__((weak, noinline))
#endif
    void __jit_debug_register_code() {
#ifndef CFG_TARGET_OS_windows
  __asm__("");
#endif
}

struct JITDescriptor {
  uint32_t version_;
  uint32_t action_flag_;
  void *relevant_entry_;
  void *first_entry_;
};

#ifdef CFG_TARGET_OS_windows
// export required for external access.
__declspec(dllexport)
#else
// Note the `weak` linkage here which is the same purpose as above. We want to
// let other runtimes be able to override this since our own definition isn't
// important.
__attribute__((weak))
#endif
struct JITDescriptor __jit_debug_descriptor = {1, 0, NULL, NULL};

struct JITDescriptor *VERSIONED_SYMBOL(wasmtime_jit_debug_descriptor)() {
  return &__jit_debug_descriptor;
}
