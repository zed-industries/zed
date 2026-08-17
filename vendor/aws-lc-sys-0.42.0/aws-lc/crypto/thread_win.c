// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

// Ensure we can't call OPENSSL_malloc circularly.
#define _BORINGSSL_PROHIBIT_OPENSSL_MALLOC
#include "internal.h"

#if defined(OPENSSL_WINDOWS_THREADS)

OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <windows.h>
OPENSSL_MSVC_PRAGMA(warning(pop))

#include <stdlib.h>
#include <string.h>

#include <openssl/type_check.h>


OPENSSL_STATIC_ASSERT(sizeof(CRYPTO_MUTEX) >= sizeof(SRWLOCK),
                      CRYPTO_MUTEX_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(CRYPTO_MUTEX) >= alignof(SRWLOCK),
                      CRYPTO_MUTEX_has_insufficient_alignment)

static BOOL CALLBACK call_once_init(INIT_ONCE *once, void *arg, void **out) {
  void (**init)(void) = (void (**)(void))arg;
  (**init)();
  return TRUE;
}

void CRYPTO_once(CRYPTO_once_t *once, void (*init)(void)) {
  if (!InitOnceExecuteOnce(once, call_once_init, &init, NULL)) {
    abort();
  }
}

void CRYPTO_MUTEX_init(CRYPTO_MUTEX *lock) {
  InitializeSRWLock((SRWLOCK *) lock);
}

void CRYPTO_MUTEX_lock_read(CRYPTO_MUTEX *lock) {
  AcquireSRWLockShared((SRWLOCK *) lock);
}

void CRYPTO_MUTEX_lock_write(CRYPTO_MUTEX *lock) {
  AcquireSRWLockExclusive((SRWLOCK *) lock);
}

void CRYPTO_MUTEX_unlock_read(CRYPTO_MUTEX *lock) {
  ReleaseSRWLockShared((SRWLOCK *) lock);
}

void CRYPTO_MUTEX_unlock_write(CRYPTO_MUTEX *lock) {
  ReleaseSRWLockExclusive((SRWLOCK *) lock);
}

void CRYPTO_MUTEX_cleanup(CRYPTO_MUTEX *lock) {
  // SRWLOCKs require no cleanup.
}

void CRYPTO_STATIC_MUTEX_lock_read(struct CRYPTO_STATIC_MUTEX *lock) {
  AcquireSRWLockShared(&lock->lock);
}

void CRYPTO_STATIC_MUTEX_lock_write(struct CRYPTO_STATIC_MUTEX *lock) {
  AcquireSRWLockExclusive(&lock->lock);
}

void CRYPTO_STATIC_MUTEX_unlock_read(struct CRYPTO_STATIC_MUTEX *lock) {
  ReleaseSRWLockShared(&lock->lock);
}

void CRYPTO_STATIC_MUTEX_unlock_write(struct CRYPTO_STATIC_MUTEX *lock) {
  ReleaseSRWLockExclusive(&lock->lock);
}

#if !defined(NDEBUG)
int CRYPTO_STATIC_MUTEX_is_write_locked(struct CRYPTO_STATIC_MUTEX *lock) {
  assert(lock != NULL);

  if (TryAcquireSRWLockShared(&lock->lock)) {
    // If successful, the lock is not write-locked
    // Release it immediately and return false (0)
    ReleaseSRWLockShared(&lock->lock);
    return 0;
  }

  return 1;
}
#endif

static SRWLOCK g_destructors_lock = SRWLOCK_INIT;
static thread_local_destructor_t g_destructors[NUM_OPENSSL_THREAD_LOCALS];

static CRYPTO_once_t g_thread_local_init_once = CRYPTO_ONCE_INIT;
static DWORD g_thread_local_key;
static int g_thread_local_failed;

static void thread_local_init(void) {
  g_thread_local_key = TlsAlloc();
  g_thread_local_failed = (g_thread_local_key == TLS_OUT_OF_INDEXES);
}

static void NTAPI thread_local_destructor(PVOID module, DWORD reason,
                                          PVOID reserved) {
  // Only free memory on |DLL_THREAD_DETACH|, not |DLL_PROCESS_DETACH|. In
  // VS2015's debug runtime, the C runtime has been unloaded by the time
  // |DLL_PROCESS_DETACH| runs. See https://crbug.com/575795. This is consistent
  // with |pthread_key_create| which does not call destructors on process exit,
  // only thread exit.
  if (reason != DLL_THREAD_DETACH) {
    return;
  }

  CRYPTO_once(&g_thread_local_init_once, thread_local_init);
  if (g_thread_local_failed) {
    return;
  }

  void **pointers = (void**) TlsGetValue(g_thread_local_key);
  if (pointers == NULL) {
    return;
  }

  thread_local_destructor_t destructors[NUM_OPENSSL_THREAD_LOCALS];

  AcquireSRWLockExclusive(&g_destructors_lock);
  OPENSSL_memcpy(destructors, g_destructors, sizeof(destructors));
  ReleaseSRWLockExclusive(&g_destructors_lock);

  for (unsigned i = 0; i < NUM_OPENSSL_THREAD_LOCALS; i++) {
    if (destructors[i] != NULL) {
      destructors[i](pointers[i]);
    }
  }

  free(pointers);
}

// Thread Termination Callbacks.
//
// Windows doesn't support a per-thread destructor with its TLS primitives.
// So, we build it manually by inserting a function to be called on each
// thread's exit. This magic is from http://www.codeproject.com/threads/tls.asp
// and it works for VC++ 7.0 and later.
//
// Force a reference to _tls_used to make the linker create the TLS directory
// if it's not already there. (E.g. if __declspec(thread) is not used). Force
// a reference to p_thread_callback_boringssl to prevent whole program
// optimization from discarding the variable.
//
// Note, in the prefixed build, |p_thread_callback_boringssl| may be a macro.
#ifdef _MSC_VER
#define STRINGIFY(x) #x
#define EXPAND_AND_STRINGIFY(x) STRINGIFY(x)
#ifdef _WIN64
__pragma(comment(linker, "/INCLUDE:_tls_used"))
__pragma(comment(
    linker, "/INCLUDE:" EXPAND_AND_STRINGIFY(p_thread_callback_boringssl)))
#else
__pragma(comment(linker, "/INCLUDE:__tls_used"))
__pragma(comment(
    linker, "/INCLUDE:_" EXPAND_AND_STRINGIFY(p_thread_callback_boringssl)))
#endif
#endif

// .CRT$XLA to .CRT$XLZ is an array of PIMAGE_TLS_CALLBACK pointers that are
// called automatically by the OS loader code (not the CRT) when the module is
// loaded and on thread creation. They are NOT called if the module has been
// loaded by a LoadLibrary() call. It must have implicitly been loaded at
// process startup.
//
// By implicitly loaded, I mean that it is directly referenced by the main EXE
// or by one of its dependent DLLs. Delay-loaded DLL doesn't count as being
// implicitly loaded.
//
// See VC\crt\src\tlssup.c for reference.

// The linker must not discard p_thread_callback_boringssl. (We force a
// reference to this variable with a linker /INCLUDE:symbol pragma to ensure
// that.) If this variable is discarded, the OnThreadExit function will never
// be called.
#ifdef _WIN64

// .CRT section is merged with .rdata on x64 so it must be constant data.
#pragma const_seg(".CRT$XLC")
// When defining a const variable, it must have external linkage to be sure the
// linker doesn't discard it.
extern const PIMAGE_TLS_CALLBACK p_thread_callback_boringssl;
const PIMAGE_TLS_CALLBACK p_thread_callback_boringssl = thread_local_destructor;
// Reset the default section.
#pragma const_seg()

#else

#pragma data_seg(".CRT$XLC")
PIMAGE_TLS_CALLBACK p_thread_callback_boringssl = thread_local_destructor;
// Reset the default section.
#pragma data_seg()

#endif  // _WIN64

static void **get_thread_locals(void) {
  // |TlsGetValue| clears the last error even on success, so that callers may
  // distinguish it successfully returning NULL or failing. It is documented to
  // never fail if the argument is a valid index from |TlsAlloc|, so we do not
  // need to handle this.
  //
  // However, this error-mangling behavior interferes with the caller's use of
  // |GetLastError|. In particular |SSL_get_error| queries the error queue to
  // determine whether the caller should look at the OS's errors. To avoid
  // destroying state, save and restore the Windows error.
  //
  // https://msdn.microsoft.com/en-us/library/windows/desktop/ms686812(v=vs.85).aspx
  DWORD last_error = GetLastError();
  void **ret = TlsGetValue(g_thread_local_key);
  SetLastError(last_error);
  return ret;
}

void *CRYPTO_get_thread_local(thread_local_data_t index) {
  CRYPTO_once(&g_thread_local_init_once, thread_local_init);
  if (g_thread_local_failed) {
    return NULL;
  }

  void **pointers = get_thread_locals();
  if (pointers == NULL) {
    return NULL;
  }
  return pointers[index];
}

int CRYPTO_set_thread_local(thread_local_data_t index, void *value,
                            thread_local_destructor_t destructor) {
  CRYPTO_once(&g_thread_local_init_once, thread_local_init);
  if (g_thread_local_failed) {
    destructor(value);
    return 0;
  }

  void **pointers = get_thread_locals();
  if (pointers == NULL) {
    pointers = malloc(sizeof(void *) * NUM_OPENSSL_THREAD_LOCALS);
    if (pointers == NULL) {
      destructor(value);
      return 0;
    }
    OPENSSL_memset(pointers, 0, sizeof(void *) * NUM_OPENSSL_THREAD_LOCALS);
    if (TlsSetValue(g_thread_local_key, pointers) == 0) {
      free(pointers);
      destructor(value);
      return 0;
    }
  }

  AcquireSRWLockExclusive(&g_destructors_lock);
  g_destructors[index] = destructor;
  ReleaseSRWLockExclusive(&g_destructors_lock);

  pointers[index] = value;
  return 1;
}

#endif  // OPENSSL_WINDOWS_THREADS
