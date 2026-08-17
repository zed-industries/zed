/*
 * Copyright 2018-2025 Yury Gribov
 *
 * The MIT License (MIT)
 *
 * Use of this source code is governed by MIT license that can be
 * found in the LICENSE.txt file.
 */

#ifndef _GNU_SOURCE
#define _GNU_SOURCE // For RTLD_DEFAULT
#endif

#define HAS_DLOPEN_CALLBACK 0
#define HAS_DLSYM_CALLBACK 0
#define NO_DLOPEN 0
#define LAZY_LOAD 1
#define THREAD_SAFE 1

#include <dlfcn.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>

#if THREAD_SAFE
#include <pthread.h>
#endif

// Sanity check for ARM to avoid puzzling runtime crashes
#ifdef __arm__
# if defined __thumb__ && ! defined __THUMB_INTERWORK__
#   error "ARM trampolines need -mthumb-interwork to work in Thumb mode"
# endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define CHECK(cond, fmt, ...) do { \
    if(!(cond)) { \
      fprintf(stderr, "implib-gen: libva.so.2: " fmt "\n", ##__VA_ARGS__); \
      assert(0 && "Assertion in generated code"); \
      abort(); \
    } \
  } while(0)

static void *lib_handle;
static int dlopened;

#if ! NO_DLOPEN

#if THREAD_SAFE

// We need to consider two cases:
// - different threads calling intercepted APIs in parallel
// - same thread calling 2 intercepted APIs recursively
//   due to dlopen calling library constructors
//   (usually happens only under IMPLIB_EXPORT_SHIMS)

// Current recursive mutex approach will deadlock
// if library constructor starts and joins a new thread
// which (directly or indirectly) calls another library function.
// Such situations should be very rare (although chances
// are higher when -DIMLIB_EXPORT_SHIMS are enabled).
//
// Similar issue is present in Glibc so hopefully it's
// not a big deal: // http://sourceware.org/bugzilla/show_bug.cgi?id=15686
// (also google for "dlopen deadlock).

static pthread_mutex_t mtx;
static int rec_count;

static void init_lock(void) {
  // We need recursive lock because dlopen will call library constructors
  // which may call other intercepted APIs that will call load_library again.
  // PTHREAD_RECURSIVE_MUTEX_INITIALIZER is not portable
  // so we do it hard way.

  pthread_mutexattr_t attr;
  CHECK(0 == pthread_mutexattr_init(&attr), "failed to init mutex");
  CHECK(0 == pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE), "failed to init mutex");

  CHECK(0 == pthread_mutex_init(&mtx, &attr), "failed to init mutex");
}

static int lock(void) {
  static pthread_once_t once = PTHREAD_ONCE_INIT;
  CHECK(0 == pthread_once(&once, init_lock), "failed to init lock");

  CHECK(0 == pthread_mutex_lock(&mtx), "failed to lock mutex");

  return 0 == __sync_fetch_and_add(&rec_count, 1);
}

static void unlock(void) {
  __sync_fetch_and_add(&rec_count, -1);
  CHECK(0 == pthread_mutex_unlock(&mtx), "failed to unlock mutex");
}
#else
static int lock(void) {
  return 1;
}
static void unlock(void) {}
#endif

static int load_library(void) {
  int publish = lock();

  if (lib_handle) {
    unlock();
    return publish;
  }

#if HAS_DLOPEN_CALLBACK
  extern void *(const char *lib_name);
  lib_handle = ("libva.so.2");
  CHECK(lib_handle, "failed to load library 'libva.so.2' via callback ''");
#else
  lib_handle = dlopen("libva.so.2", RTLD_LAZY | RTLD_GLOBAL);
  CHECK(lib_handle, "failed to load library 'libva.so.2' via dlopen: %s", dlerror());
#endif

  // With (non-default) IMPLIB_EXPORT_SHIMS we may call dlopen more than once
  // so dlclose it if we are not the first ones
  if (__sync_val_compare_and_swap(&dlopened, 0, 1)) {
    dlclose(lib_handle);
  }

  unlock();

  return publish;
}

// Run dtor as late as possible in case library functions are
// called in other global dtors
// FIXME: this may crash if one thread is calling into library
// while some other thread executes exit(). It's no clear
// how to fix this besides simply NOT dlclosing library at all.
static void __attribute__((destructor(101))) unload_lib(void) {
  if (dlopened) {
    dlclose(lib_handle);
    lib_handle = 0;
    dlopened = 0;
  }
}
#endif

#if ! NO_DLOPEN && ! LAZY_LOAD
static void __attribute__((constructor(101))) load_lib(void) {
  load_library();
}
#endif

// TODO: convert to single 0-separated string
static const char *const sym_names[] = {
  "disabled_va_TraceInit",
  "vaAcquireBufferHandle",
  "vaAssociateSubpicture",
  "vaAttachProtectedSession",
  "vaBeginPicture",
  "vaBufferInfo",
  "vaBufferSetNumElements",
  "vaBufferTypeStr",
  "vaConfigAttribTypeStr",
  "vaCopy",
  "vaCreateBuffer",
  "vaCreateBuffer2",
  "vaCreateConfig",
  "vaCreateContext",
  "vaCreateImage",
  "vaCreateMFContext",
  "vaCreateProtectedSession",
  "vaCreateSubpicture",
  "vaCreateSurfaces",
  "vaDeassociateSubpicture",
  "vaDeriveImage",
  "vaDestroyBuffer",
  "vaDestroyConfig",
  "vaDestroyContext",
  "vaDestroyImage",
  "vaDestroyProtectedSession",
  "vaDestroySubpicture",
  "vaDestroySurfaces",
  "vaDetachProtectedSession",
  "vaDisplayIsValid",
  "vaEndPicture",
  "vaEntrypointStr",
  "vaErrorStr",
  "vaExportSurfaceHandle",
  "vaGetConfigAttributes",
  "vaGetDisplayAttributes",
  "vaGetImage",
  "vaGetLibFunc",
  "vaInitialize",
  "vaLockSurface",
  "vaMFAddContext",
  "vaMFReleaseContext",
  "vaMFSubmit",
  "vaMapBuffer",
  "vaMapBuffer2",
  "vaMaxNumConfigAttributes",
  "vaMaxNumDisplayAttributes",
  "vaMaxNumEntrypoints",
  "vaMaxNumImageFormats",
  "vaMaxNumProfiles",
  "vaMaxNumSubpictureFormats",
  "vaProfileStr",
  "vaProtectedSessionExecute",
  "vaPutImage",
  "vaQueryConfigAttributes",
  "vaQueryConfigEntrypoints",
  "vaQueryConfigProfiles",
  "vaQueryDisplayAttributes",
  "vaQueryImageFormats",
  "vaQueryProcessingRate",
  "vaQuerySubpictureFormats",
  "vaQuerySurfaceAttributes",
  "vaQuerySurfaceError",
  "vaQuerySurfaceStatus",
  "vaQueryVendorString",
  "vaQueryVideoProcFilterCaps",
  "vaQueryVideoProcFilters",
  "vaQueryVideoProcPipelineCaps",
  "vaReleaseBufferHandle",
  "vaRenderPicture",
  "vaSetDisplayAttributes",
  "vaSetDriverName",
  "vaSetErrorCallback",
  "vaSetImagePalette",
  "vaSetInfoCallback",
  "vaSetSubpictureChromakey",
  "vaSetSubpictureGlobalAlpha",
  "vaSetSubpictureImage",
  "vaStatusStr",
  "vaSyncBuffer",
  "vaSyncSurface",
  "vaSyncSurface2",
  "vaTerminate",
  "vaUnlockSurface",
  "vaUnmapBuffer",
  "va_TracePutSurface",
  "va_TraceStatus",
  "va_newDisplayContext",
  "va_newDriverContext",
  0
};

#define SYM_COUNT (sizeof(sym_names)/sizeof(sym_names[0]) - 1)

extern void *_libva_so_tramp_table[];

// Can be sped up by manually parsing library symtab...
void *_libva_so_tramp_resolve(size_t i) {
  assert(i < SYM_COUNT);

  int publish = 1;

  void *h = 0;
#if NO_DLOPEN
  // Library with implementations must have already been loaded.
  if (lib_handle) {
    // User has specified loaded library
    h = lib_handle;
  } else {
    // User hasn't provided us the loaded library so search the global namespace.
#   ifndef IMPLIB_EXPORT_SHIMS
    // If shim symbols are hidden we should search
    // for first available definition of symbol in library list
    h = RTLD_DEFAULT;
#   else
    // Otherwise look for next available definition
    h = RTLD_NEXT;
#   endif
  }
#else
  publish = load_library();
  h = lib_handle;
  CHECK(h, "failed to resolve symbol '%s', library failed to load", sym_names[i]);
#endif

  void *addr;
#if HAS_DLSYM_CALLBACK
  extern void *(void *handle, const char *sym_name);
  addr = (h, sym_names[i]);
  CHECK(addr, "failed to resolve symbol '%s' via callback ", sym_names[i]);
#else
  // Dlsym is thread-safe so don't need to protect it.
  addr = dlsym(h, sym_names[i]);
  CHECK(addr, "failed to resolve symbol '%s' via dlsym: %s", sym_names[i], dlerror());
#endif

  if (publish) {
    // Use atomic to please Tsan and ensure that preceeding writes
    // in library ctors have been delivered before publishing address
    (void)__sync_val_compare_and_swap(&_libva_so_tramp_table[i], 0, addr);
  }

  return addr;
}

// Below APIs are not thread-safe
// and it's not clear how make them such
// (we can not know if some other thread is
// currently executing library code).

// Helper for user to resolve all symbols
void _libva_so_tramp_resolve_all(void) {
  size_t i;
  for(i = 0; i < SYM_COUNT; ++i)
    _libva_so_tramp_resolve(i);
}

// Allows user to specify manually loaded implementation library.
void _libva_so_tramp_set_handle(void *handle) {
  // TODO: call unload_lib ?
  lib_handle = handle;
  dlopened = 0;
}

// Resets all resolved symbols. This is needed in case
// client code wants to reload interposed library multiple times.
void _libva_so_tramp_reset(void) {
  // TODO: call unload_lib ?
  memset(_libva_so_tramp_table, 0, SYM_COUNT * sizeof(_libva_so_tramp_table[0]));
  lib_handle = 0;
  dlopened = 0;
}

#ifdef __cplusplus
}  // extern "C"
#endif
