// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#if !defined(_GNU_SOURCE)
#define _GNU_SOURCE  // needed for syscall() on Linux.
#endif

#include <openssl/crypto.h>

#include <stdlib.h>
#if defined(BORINGSSL_FIPS) && !defined(OPENSSL_WINDOWS)
#include <sys/mman.h>
#include <unistd.h>
#endif

// On Windows place the bcm code in a specific section that uses Grouped Sections
// to control the order. $b section will place bcm in between the start/end markers
// which are in $a and $z.
#if defined(BORINGSSL_FIPS) && defined(OPENSSL_WINDOWS)
#pragma code_seg(".fipstx$b")
#pragma data_seg(".fipsda$b")
#pragma const_seg(".fipsco$b")
#pragma bss_seg(".fipsbs$b")
// Explicitly declare the FIPS rodata section with correct attributes. This
// ensures the section is known to the compiler/linker even if #pragma const_seg
// is not fully supported (e.g. clang-cl on ARM64).
#pragma section(".fipsco$b", read)
#endif

#include <openssl/chacha.h>
#include <openssl/digest.h>
#include <openssl/hmac.h>
#include <openssl/sha.h>

#include "../internal.h"

#include "aes/aes.c"
#include "aes/aes_nohw.c"
#include "aes/key_wrap.c"
#include "aes/mode_wrappers.c"
#include "bn/add.c"
#include "bn/asm/x86_64-gcc.c"
#include "bn/bn.c"
#include "bn/bytes.c"
#include "bn/cmp.c"
#include "bn/ctx.c"
#include "bn/div.c"
#include "bn/div_extra.c"
#include "bn/exponentiation.c"
#include "bn/gcd.c"
#include "bn/gcd_extra.c"
#include "bn/generic.c"
#include "bn/jacobi.c"
#include "bn/montgomery.c"
#include "bn/montgomery_inv.c"
#include "bn/mul.c"
#include "bn/prime.c"
#include "bn/random.c"
#include "bn/rsaz_exp.c"
#include "bn/rsaz_exp_x2.c"
#include "bn/shift.c"
#include "bn/sqrt.c"
#include "cipher/aead.c"
#include "cipher/cipher.c"
#include "cipher/e_aes.c"
#include "cipher/e_aesccm.c"

#include "cpucap/internal.h"
#include "cpucap/cpu_aarch64.c"
#include "cpucap/cpu_aarch64_sysreg.c"
#include "cpucap/cpu_aarch64_apple.c"
#include "cpucap/cpu_aarch64_freebsd.c"
#include "cpucap/cpu_aarch64_linux.c"
#include "cpucap/cpu_aarch64_netbsd.c"
#include "cpucap/cpu_aarch64_openbsd.c"
#include "cpucap/cpu_aarch64_win.c"
#include "cpucap/cpu_arm_freebsd.c"
#include "cpucap/cpu_arm_linux.c"
#include "cpucap/cpu_intel.c"
#include "cpucap/cpu_ppc64le.c"

#include "cmac/cmac.c"
#include "curve25519/curve25519.c"
#include "curve25519/curve25519_nohw.c"
#include "curve25519/curve25519_s2n_bignum_asm.c"
#include "dh/check.c"
#include "dh/dh.c"
#include "digest/digest.c"
#include "digest/digests.c"
#include "ecdh/ecdh.c"
#include "ecdsa/ecdsa.c"
#include "ec/ec.c"
#include "ec/ec_key.c"
#include "ec/ec_montgomery.c"
#include "ec/ec_nistp.c"
#include "ec/felem.c"
#include "ec/oct.c"
#include "ec/p224-64.c"
#include "ec/p256.c"
#include "ec/p256-nistz.c"
#include "ec/p384.c"
#include "ec/p521.c"
#include "ec/scalar.c"
#include "ec/simple.c"
#include "ec/simple_mul.c"
#include "ec/util.c"
#include "ec/wnaf.c"
#include "evp/digestsign.c"
#include "evp/evp.c"
#include "evp/evp_ctx.c"
#include "evp/p_ec.c"
#include "evp/p_ed25519.c"
#include "evp/p_ed25519ph.c"
#include "evp/p_hkdf.c"
#include "evp/p_hmac.c"
#include "evp/p_kem.c"
#include "evp/p_pqdsa.c"
#include "evp/p_rsa.c"
#include "hkdf/hkdf.c"
#include "hmac/hmac.c"
#include "kdf/kbkdf.c"
#include "kdf/sskdf.c"
#include "kem/kem.c"
#include "md5/md5.c"
#include "ml_dsa/ml_dsa.c"
#include "ml_kem/ml_kem.c"
#include "modes/cbc.c"
#include "modes/cfb.c"
#include "modes/ctr.c"
#include "modes/gcm.c"
#include "modes/gcm_nohw.c"
#include "modes/ofb.c"
#include "modes/xts.c"
#include "modes/polyval.c"
#include "pbkdf/pbkdf.c"
#include "pqdsa/pqdsa.c"
#include "rand/ctrdrbg.c"
#include "rand/rand.c"
#include "rand/entropy/entropy_sources.c"
#include "rand/entropy/tree_drbg_jitter_entropy.c"
#include "rsa/blinding.c"
#include "rsa/padding.c"
#include "rsa/rsa.c"
#include "rsa/rsa_impl.c"
#include "self_check/fips.c"
#include "self_check/self_check.c"
#include "service_indicator/service_indicator.c"
#include "sha/keccak1600.c"
#include "sha/sha1-altivec.c"
#include "sha/sha1.c"
#include "sha/sha256.c"
#include "sha/sha3.c"
#include "sha/sha512.c"
#include "sshkdf/sshkdf.c"
#include "tls/kdf.c"

#if defined(BORINGSSL_FIPS)

#if !defined(OPENSSL_ASAN) && !defined(OPENSSL_MSAN)

static const void* function_entry_ptr(const void* func_sym) {
#if defined(OPENSSL_PPC64BE)
  // Function pointers on ppc64be point to a function descriptor.
  // https://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi.html#FUNC-ADDRESS
  return (const void*)(((uint64_t *)func_sym)[0]);
#else
  return (const void*)func_sym;
#endif
}

// These symbols are filled in by delocate.go (in static builds) or a linker
// script (in shared builds). They point to the start and end of the module, and
// the location of the integrity hash, respectively.
extern const uint8_t BORINGSSL_bcm_text_start[];
extern const uint8_t BORINGSSL_bcm_text_end[];
extern const uint8_t BORINGSSL_bcm_text_hash[];
#if defined(BORINGSSL_SHARED_LIBRARY)
extern const uint8_t BORINGSSL_bcm_rodata_start[];
extern const uint8_t BORINGSSL_bcm_rodata_end[];
#endif

#define STRING_POINTER_LENGTH 18
#define MAX_FUNCTION_NAME 32
#define ASSERT_WITHIN_MSG "FIPS module doesn't span expected symbol (%s). Expected %p <= %p < %p\n"
#define MAX_WITHIN_MSG_LEN sizeof(ASSERT_WITHIN_MSG) + (3 * STRING_POINTER_LENGTH) + MAX_FUNCTION_NAME
#define ASSERT_OUTSIDE_MSG "FIPS module spans unexpected symbol (%s), expected %p < %p || %p > %p\n"
#define MAX_OUTSIDE_MSG_LEN sizeof(ASSERT_OUTSIDE_MSG) + (4 * STRING_POINTER_LENGTH) + MAX_FUNCTION_NAME
// assert_within is used to sanity check that certain symbols are within the
// bounds of the integrity check. It checks that start <= symbol < end and
// aborts otherwise.
static void assert_within(const void *start, const void *symbol,
                          const char *symbol_name, const void *end) {
  const uintptr_t start_val = (uintptr_t) start;
  const uintptr_t symbol_val = (uintptr_t) symbol;
  const uintptr_t end_val = (uintptr_t) end;

  if (start_val <= symbol_val && symbol_val < end_val) {
    return;
  }

  assert(strlen(symbol_name) < MAX_FUNCTION_NAME);
  char message[MAX_WITHIN_MSG_LEN] = {0};
  snprintf(message, sizeof(message), ASSERT_WITHIN_MSG, symbol_name, start, symbol, end);
  AWS_LC_FIPS_failure(message);
}

static void assert_not_within(const void *start, const void *symbol,
                          const char *symbol_name, const void *end) {
  const uintptr_t start_val = (uintptr_t) start;
  const uintptr_t symbol_val = (uintptr_t) symbol;
  const uintptr_t end_val = (uintptr_t) end;

  if (start_val >= symbol_val || symbol_val > end_val) {
    return;
  }

  assert(strlen(symbol_name) < MAX_FUNCTION_NAME);
  char message[MAX_OUTSIDE_MSG_LEN] = {0};
  snprintf(message, sizeof(message), ASSERT_OUTSIDE_MSG, symbol_name, symbol, start, symbol, end);
  AWS_LC_FIPS_failure(message);
}

// TODO: Re-enable once all data has been moved out of .text segments CryptoAlg-2360
#if 0
//#if defined(OPENSSL_ANDROID) && defined(OPENSSL_AARCH64)
static void BORINGSSL_maybe_set_module_text_permissions(int permission) {
  // Android may be compiled in execute-only-memory mode, in which case the
  // .text segment cannot be read. That conflicts with the need for a FIPS
  // module to hash its own contents, therefore |mprotect| is used to make
  // the module's .text readable for the duration of the hashing process. In
  // other build configurations this is a no-op.
  const uintptr_t page_size = getpagesize();
  const uintptr_t page_start =
      ((uintptr_t)BORINGSSL_bcm_text_start) & ~(page_size - 1);

  if (mprotect((void *)page_start,
               ((uintptr_t)BORINGSSL_bcm_text_end) - page_start,
               permission) != 0) {
    perror("BoringSSL: mprotect");
  }
}
#elif !defined(OPENSSL_WINDOWS)
static void BORINGSSL_maybe_set_module_text_permissions(int _permission) {}
#endif  // !ANDROID

#endif  // !ASAN && !MSAN

#if defined(AWSLC_FIPS_FAILURE_CALLBACK)
#if defined(__ELF__) && defined(__GNUC__)
WEAK_SYMBOL_FUNC(void, AWS_LC_fips_failure_callback, (const char* message))
#else
#error AWSLC_FIPS_FAILURE_CALLBACK not supported on this platform
#endif
#endif

#if defined(_MSC_VER)
#pragma section(".CRT$XCU", read)
static void BORINGSSL_bcm_power_on_self_test(void);
__declspec(allocate(".CRT$XCU")) void(*fips_library_init_constructor)(void) =
    BORINGSSL_bcm_power_on_self_test;
#else
static void BORINGSSL_bcm_power_on_self_test(void) __attribute__ ((constructor));
#endif

static void BORINGSSL_bcm_power_on_self_test(void) {
#if defined(HAS_OPENSSL_CPUID_SETUP) && !defined(OPENSSL_NO_ASM)
  OPENSSL_cpuid_setup();
#endif

  if (jent_entropy_init()) {
    AWS_LC_FIPS_failure("CPU Jitter entropy RNG initialization failed");
  }

#if !defined(OPENSSL_ASAN) && !defined(OPENSSL_MSAN)
  // Integrity tests cannot run under ASAN or MSAN because it involves reading
  // the full .text section, which triggers the global-buffer overflow detection
  // (ASAN) or use of uninstrumented code (MSAN).
  if (!BORINGSSL_integrity_test()) {
    AWS_LC_FIPS_failure("Integrity test failed");
  }
#endif  // !ASAN && !MSAN

  if (!boringssl_self_test_startup()) {
    AWS_LC_FIPS_failure("Power on self test failed");
  }
}

#if !defined(OPENSSL_ASAN) && !defined(OPENSSL_MSAN)
int BORINGSSL_integrity_test(void) {
  const uint8_t *const start = BORINGSSL_bcm_text_start;
  const uint8_t *const end = BORINGSSL_bcm_text_end;

  assert_within(start, function_entry_ptr(AES_encrypt), "AES_encrypt", end);
  assert_within(start, function_entry_ptr(RSA_sign), "RSA_sign", end);
  assert_within(start, function_entry_ptr(RAND_bytes), "RAND_bytes", end);
  assert_within(start, function_entry_ptr(EC_GROUP_cmp), "EC_GROUP_cmp", end);
  assert_within(start, function_entry_ptr(SHA256_Update), "SHA256_Update", end);
  assert_within(start, function_entry_ptr(ECDSA_do_verify), "ECDSA_do_verify", end);
  assert_within(start, function_entry_ptr(EVP_AEAD_CTX_seal), "EVP_AEAD_CTX_seal", end);
  assert_not_within(start, function_entry_ptr(OPENSSL_cleanse), "OPENSSL_cleanse", end);
  assert_not_within(start, function_entry_ptr(CRYPTO_chacha_20), "CRYPTO_chacha_20", end);
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
  assert_not_within(start, OPENSSL_ia32cap_P, "OPENSSL_ia32cap_P", end);
#elif defined(OPENSSL_AARCH64)
  assert_not_within(start, &OPENSSL_armcap_P, "OPENSSL_armcap_P", end);
#endif

#if defined(BORINGSSL_SHARED_LIBRARY)
  const uint8_t *const rodata_start = BORINGSSL_bcm_rodata_start;
  const uint8_t *const rodata_end = BORINGSSL_bcm_rodata_end;
#else
  // In the static build, read-only data is placed within the .text segment.
  const uint8_t *const rodata_start = BORINGSSL_bcm_text_start;
  const uint8_t *const rodata_end = BORINGSSL_bcm_text_end;
#endif

  assert_within(rodata_start, kPrimes, "kPrimes", rodata_end);
  assert_within(rodata_start, kP256Field, "kP256Field", rodata_end);
  assert_within(rodata_start, kPKCS1SigPrefixes, "kPKCS1SigPrefixes", rodata_end);
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
  assert_not_within(rodata_start, OPENSSL_ia32cap_P, "OPENSSL_ia32cap_P", rodata_end);
#elif defined(OPENSSL_AARCH64)
  assert_not_within(rodata_start, &OPENSSL_armcap_P, "OPENSSL_armcap_P", rodata_end);
#endif

  // Per FIPS 140-3 we have to perform the CAST of the HMAC used for integrity
  // check before the integrity check itself. So we first call
  // SHA-256 and HMAC-SHA256
  // before we calculate the hash of the module.

  uint8_t result[SHA256_DIGEST_LENGTH];
  const EVP_MD *const kHashFunction = EVP_sha256();
  if (!boringssl_self_test_sha256() ||
      !boringssl_self_test_hmac_sha256()) {
    return 0;
  }

  static const uint8_t kHMACKey[64] = {0};
  unsigned result_len;
  HMAC_CTX hmac_ctx;
  HMAC_CTX_init(&hmac_ctx);
  if (!HMAC_Init_ex(&hmac_ctx, kHMACKey, sizeof(kHMACKey), kHashFunction,
                    NULL /* no ENGINE */)) {
    fprintf(stderr, "HMAC_Init_ex failed.\n");
    return 0;
  }
#if !defined(OPENSSL_WINDOWS)
  BORINGSSL_maybe_set_module_text_permissions(PROT_READ | PROT_EXEC);
#endif
#if defined(BORINGSSL_SHARED_LIBRARY)
  uint64_t length = end - start;
  uint8_t buffer[sizeof(length)];
  CRYPTO_store_u64_le(buffer, length);
  HMAC_Update(&hmac_ctx, buffer, sizeof(length));
  HMAC_Update(&hmac_ctx, start, length);

  length = rodata_end - rodata_start;
  CRYPTO_store_u64_le(buffer, length);
  HMAC_Update(&hmac_ctx, buffer, sizeof(length));
  HMAC_Update(&hmac_ctx, rodata_start, length);
#else
  HMAC_Update(&hmac_ctx, start, end - start);
#endif
#if !defined(OPENSSL_WINDOWS)
  BORINGSSL_maybe_set_module_text_permissions(PROT_EXEC);
#endif

  if (!HMAC_Final(&hmac_ctx, result, &result_len) ||
      result_len != sizeof(result)) {
    fprintf(stderr, "HMAC failed.\n");
    return 0;
  }
  HMAC_CTX_cleanse(&hmac_ctx); // FIPS 140-3, AS05.10.

  const uint8_t *expected = BORINGSSL_bcm_text_hash;

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
  // Check the integrity but don't call AWS_LC_FIPS_failure or return 0
  check_test_optional_abort(expected, result, sizeof(result), "FIPS integrity test", false);
#else
  // Check the integrity, call AWS_LC_FIPS_failure if it doesn't match which will
  // result in an abort
  check_test_optional_abort(expected, result, sizeof(result), "FIPS integrity test", true);
#endif

  OPENSSL_cleanse(result, sizeof(result)); // FIPS 140-3, AS05.10.
  return 1;
}
#endif  // !ASAN && !MSAN

void AWS_LC_FIPS_failure(const char* message) {
#if defined(AWSLC_FIPS_FAILURE_CALLBACK)
  if (AWS_LC_fips_failure_callback != NULL) {
    AWS_LC_fips_failure_callback(message);
    return;
  }
  // Fallback to the default behavior if the callback is not defined
#endif
  fprintf(stderr, "AWS-LC FIPS failure caused by:\n%s\n", message);
  fflush(stderr);
  for (;;) {
    abort();
    exit(1);
  }
}
#else  // BORINGSSL_FIPS
void AWS_LC_FIPS_failure(const char* message) {
  fprintf(stderr, "AWS-LC FIPS failure caused by:\n%s\n", message); // NOLINT(clang-analyzer-security.insecureAPI.DeprecatedOrUnsafeBufferHandling)
  fflush(stderr);
}
#endif  // BORINGSSL_FIPS
#if !defined(AWSLC_FIPS) && !defined(BORINGSSL_SHARED_LIBRARY)
// When linking with a static library, if no symbols in an object file are
// referenced then the object file is discarded, even if it has a constructor
// function. For example, if an application is linking with libcrypto.a and
// not referencing any symbol from crypto.o file, then crypto.o will be
// discarded. This is an issue because we define the library constructor
// in crypto.o so if the file is discarded then the library is not initialized.
// Note that this is not a problem for the FIPS build because when building the
// FIPS mode we have to ensure the initialization is done before the power-on
// self-tests so the test function itself calls |OPENSSL_cpuid_setup|.
//
// So, the issue manifests only in the static non-FIPS build. The work around
// is we add a dummy function |dummy_func_for_constructor| in |bcm.c| that
// calls |CRYPTO_library_init| function defined in |crypto.c|. This will ensure
// that, when linking with libcrypto.a, crypto.o will not be discarded as long
// as bcm.o is not discarded.
//
// This workaround is partial in a sense that if the application that's linking
// to libcrypto.a is not using any of the symbols from bcm.o or crypto.o then
// both object files will be discarded. But this would be an edge case that we
// don't expect to happen with significant probability. In case it happens, the
// application would have to call the |CRYPTO_library_init| function itself to
// ensure the initialization is done.
void dummy_func_for_constructor(void) {
    CRYPTO_library_init();
}
#endif
