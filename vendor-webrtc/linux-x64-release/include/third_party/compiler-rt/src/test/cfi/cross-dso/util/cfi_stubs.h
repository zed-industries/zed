// This is a hack to access CFI interface that Android has in libdl.so on
// device, but not in the NDK.
#include <dlfcn.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*cfi_slowpath_ty)(uint64_t, void *);
typedef void (*cfi_slowpath_diag_ty)(uint64_t, void *, void *);

static cfi_slowpath_ty cfi_slowpath;
static cfi_slowpath_diag_ty cfi_slowpath_diag;

__attribute__((constructor(0), no_sanitize("cfi"))) static void init() {
  cfi_slowpath = (cfi_slowpath_ty)dlsym(RTLD_NEXT, "__cfi_slowpath");
  cfi_slowpath_diag =
      (cfi_slowpath_diag_ty)dlsym(RTLD_NEXT, "__cfi_slowpath_diag");
  if (!cfi_slowpath || !cfi_slowpath_diag) abort();
}

extern "C" {
__attribute__((visibility("hidden"), no_sanitize("cfi"))) void __cfi_slowpath(
    uint64_t Type, void *Addr) {
  cfi_slowpath(Type, Addr);
}

__attribute__((visibility("hidden"), no_sanitize("cfi"))) void
__cfi_slowpath_diag(uint64_t Type, void *Addr, void *Diag) {
  cfi_slowpath_diag(Type, Addr, Diag);
}
}
