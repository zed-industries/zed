#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <processthreadsapi.h>

#ifndef PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE
#define PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE 34
#endif

static void CONSTRUCTOR_ATTRIBUTE init_have_lse_atomics(void) {
  if (IsProcessorFeaturePresent(PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE))
    __aarch64_have_lse_atomics = true;
}
