// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/rand.h>

#include "internal.h"
#include "../internal.h"

#if defined(OPENSSL_RAND_WINDOWS)

#include <limits.h>
#include <stdlib.h>

OPENSSL_MSVC_PRAGMA(warning(push, 3))

#include <windows.h>

// ProcessPrng (from `bcryptprimitives.dll`) is only available on Windows 8+.
// Fall back to BCryptGenRandom when targeting Windows 7 or earlier. This applies
// to both MSVC and MinGW-w64 toolchains; MinGW-w64 ships `bcrypt.h` and can link
// against `libbcrypt.a` / `bcrypt.dll`.
#if defined(_WIN32_WINNT) && _WIN32_WINNT <= _WIN32_WINNT_WIN7
#define AWSLC_WINDOWS_7_COMPAT
#endif

#if defined(AWSLC_WINDOWS_7_COMPAT) || \
    (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) && \
    !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP))
#include <bcrypt.h>
OPENSSL_MSVC_PRAGMA(comment(lib, "bcrypt.lib"))
#endif // AWSLC_WINDOWS_7_COMPAT || (WINAPI_PARTITION_APP && !WINAPI_PARTITION_DESKTOP)

OPENSSL_MSVC_PRAGMA(warning(pop))

#if defined(AWSLC_WINDOWS_7_COMPAT) || \
    (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) && \
    !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP))

void CRYPTO_sysrand(uint8_t *out, size_t requested) {
  while (requested > 0) {
    ULONG output_bytes_this_pass = ULONG_MAX;
    if (requested < output_bytes_this_pass) {
      output_bytes_this_pass = (ULONG)requested;
    }
    if (!BCRYPT_SUCCESS(BCryptGenRandom(
            /*hAlgorithm=*/NULL, out, output_bytes_this_pass,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG))) {
      abort();
    }
    requested -= output_bytes_this_pass;
    out += output_bytes_this_pass;
  }
}

#else

// See: https://learn.microsoft.com/en-us/windows/win32/seccng/processprng
typedef BOOL (WINAPI *ProcessPrngFunction)(PBYTE pbData, SIZE_T cbData);
static ProcessPrngFunction g_processprng_fn = NULL;
static CRYPTO_once_t once = CRYPTO_ONCE_INIT;

static void init_processprng(void) {
  HMODULE hmod = LoadLibraryW(L"bcryptprimitives");
  if (hmod == NULL) {
    abort();
  }
  g_processprng_fn = (ProcessPrngFunction)(void(*)(void))GetProcAddress(hmod, "ProcessPrng");
  if (g_processprng_fn == NULL) {
    abort();
  }
}

void CRYPTO_sysrand(uint8_t *out, size_t requested) {

  CRYPTO_once(&once, init_processprng);

  // On non-UWP configurations, use ProcessPrng instead of BCryptGenRandom
  // to avoid accessing resources that may be unavailable inside the
  // Chromium sandbox. See https://crbug.com/74242
  if (!g_processprng_fn(out, requested)) {
    abort();
  }
}

#endif  // AWSLC_WINDOWS_7_COMPAT || (WINAPI_PARTITION_APP && !WINAPI_PARTITION_DESKTOP)
#endif  // OPENSSL_RAND_WINDOWS
