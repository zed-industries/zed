// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/crypto.h>
#include <openssl/rand.h>

#include <assert.h>

#include "fipsmodule/cpucap/internal.h"
#include "internal.h"


OPENSSL_STATIC_ASSERT(sizeof(ossl_ssize_t) == sizeof(size_t),
              ossl_ssize_t_should_be_the_same_size_as_size_t)

#if !defined(OPENSSL_NO_ASM) && !defined(OPENSSL_STATIC_ARMCAP) && \
    (defined(OPENSSL_X86) || defined(OPENSSL_X86_64) || \
     defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64) || \
     defined(OPENSSL_PPC64LE))
// x86, x86_64, the ARMs and ppc64le need to record the result of a
// cpuid/getauxval call for the asm to work correctly, unless compiled without
// asm code.
#define NEED_CPUID

#else

// Otherwise, don't emit a static initialiser.

#if !defined(BORINGSSL_NO_STATIC_INITIALIZER)
#define BORINGSSL_NO_STATIC_INITIALIZER
#endif

#endif  // !NO_ASM && !STATIC_ARMCAP &&
        // (X86 || X86_64 || ARM || AARCH64 || PPC64LE)

#if defined(BORINGSSL_FIPS)
// In FIPS mode, the power-on self-test function calls |OPENSSL_cpuid_setup|
// because we have to ensure that CPUID detection occurs first.
#define BORINGSSL_NO_STATIC_INITIALIZER
#endif

#if defined(OPENSSL_WINDOWS) && !defined(BORINGSSL_NO_STATIC_INITIALIZER)
#define OPENSSL_CDECL __cdecl
#else
#define OPENSSL_CDECL
#endif

#if defined(BORINGSSL_NO_STATIC_INITIALIZER)
static CRYPTO_once_t once = CRYPTO_ONCE_INIT;
#elif defined(_MSC_VER)
#pragma section(".CRT$XCU", read)
static void __cdecl do_library_init(void);
__declspec(allocate(".CRT$XCU")) void(*library_init_constructor)(void) =
    do_library_init;
#else
static void do_library_init(void) __attribute__ ((constructor));
#endif

// do_library_init is the actual initialization function. If
// BORINGSSL_NO_STATIC_INITIALIZER isn't defined, this is set as a static
// initializer. Otherwise, it is called by CRYPTO_library_init.
static void OPENSSL_CDECL do_library_init(void) {
 // WARNING: this function may only configure the capability variables. See the
 // note above about the linker bug.
 // In the FIPS build the module itself has to call |OPENSSL_cpuid_setup|.
#if defined(NEED_CPUID) && !defined(BORINGSSL_FIPS)
  OPENSSL_cpuid_setup();
#endif
}

void CRYPTO_library_init(void) {
  // TODO(davidben): It would be tidier if this build knob could be replaced
  // with an internal lazy-init mechanism that would handle things correctly
  // in-library. https://crbug.com/542879
#if defined(BORINGSSL_NO_STATIC_INITIALIZER)
  CRYPTO_once(&once, do_library_init);
#endif
}

int CRYPTO_is_confidential_build(void) {
#if defined(BORINGSSL_CONFIDENTIAL)
  return 1;
#else
  return 0;
#endif
}

int CRYPTO_has_asm(void) {
#if defined(OPENSSL_NO_ASM)
  return 0;
#else
  return 1;
#endif
}

void CRYPTO_pre_sandbox_init(void) {
  // Read from /proc/cpuinfo if needed.
  CRYPTO_library_init();

  // The randomness generation subsystem has a few kernel touch points that
  // can be blocked when sandboxed. For example, /dev/urandom, MADV_WIPEONFORK
  // tagged state, and VM UBE allocated state. All this is implemented lazily.
  // Invoke the top-level function that will kick off the lazy work pre-sandbox.
  uint8_t buf[10];
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(buf, 10));
}

const char *SSLeay_version(int which) { return OpenSSL_version(which); }

const char *OpenSSL_version(int which) {
  switch (which) {
    case OPENSSL_VERSION:
      return AWSLC_VERSION_STRING;
    case OPENSSL_CFLAGS:
      return "compiler: n/a";
    case OPENSSL_BUILT_ON:
      return "built on: n/a";
    case OPENSSL_PLATFORM:
      return "platform: n/a";
    case OPENSSL_DIR:
      return "OPENSSLDIR: n/a";
    default:
      return "not available";
  }
}

unsigned long SSLeay(void) { return OPENSSL_VERSION_NUMBER; }

unsigned long OpenSSL_version_num(void) { return OPENSSL_VERSION_NUMBER; }

unsigned long awslc_api_version_num(void) { return AWSLC_API_VERSION; }

int CRYPTO_malloc_init(void) { return 1; }

int OPENSSL_malloc_init(void) { return 1; }

void ENGINE_load_builtin_engines(void) {}

void ENGINE_register_all_ciphers(void) {}

void ENGINE_register_all_digests(void) {}

int ENGINE_register_all_complete(void) { return 1; }

void OPENSSL_load_builtin_modules(void) {}

int OPENSSL_init_crypto(uint64_t opts, const OPENSSL_INIT_SETTINGS *settings) {
  CRYPTO_library_init();
  return 1;
}

void OPENSSL_init(void) {}

void OPENSSL_cleanup(void) {}
