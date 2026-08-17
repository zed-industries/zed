// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

#if defined(BORINGSSL_DISPATCH_TEST) && !defined(BORINGSSL_SHARED_LIBRARY)

#include <functional>
#include <utility>
#include <vector>

#include <openssl/aead.h>
#include <openssl/aes.h>
#include <openssl/sha.h>
#include <openssl/mem.h>

#include <gtest/gtest.h>

#include "internal.h"
#include "fipsmodule/cpucap/internal.h"
#include "fipsmodule/modes/internal.h"
#include "fipsmodule/bn/rsaz_exp.h"
#include "fipsmodule/sha/internal.h"

#include "test/file_test.h"

class ImplDispatchTest : public ::testing::Test {
 public:
  void SetUp() override {
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
    aes_hw_ = CRYPTO_is_AESNI_capable();
    avx_movbe_ = CRYPTO_is_AVX_capable() && CRYPTO_is_MOVBE_capable();
    aes_vpaes_ = CRYPTO_is_SSSE3_capable();
    is_avx2_ = CRYPTO_is_AVX2_capable();
    ifma_avx512 = CRYPTO_is_AVX512IFMA_capable();
    sha_ext_ =
    // sha_ext_ isn't enabled on 32-bit x86 architectures.
#if defined(OPENSSL_X86)
        false;
#else
        CRYPTO_is_SHAEXT_capable();
#endif

    vaes_vpclmulqdq_ =
#if !defined(OPENSSL_WINDOWS)
  // crypto_gcm_avx512_enabled excludes Windows
        CRYPTO_is_AVX512_capable() &&
        CRYPTO_is_VAES_capable() &&
        CRYPTO_is_VPCLMULQDQ_capable();
#else
        false;
#endif

    is_x86_64_ =
#if defined(OPENSSL_X86_64)
        true;
#else
        false;
#endif
    is_assembler_too_old =
#if defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
        true;
#else
        false;
#endif // MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX
    is_assembler_too_old_avx512 =
#if defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
        true;
#else
        false;
#endif // MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX
#elif defined(OPENSSL_AARCH64)
    aes_hw_ = CRYPTO_is_ARMv8_AES_capable();
    aes_vpaes_ = CRYPTO_is_NEON_capable();
    aes_gcm_pmull_ = CRYPTO_is_ARMv8_PMULL_capable();
    aes_gcm_8x_ = CRYPTO_is_ARMv8_GCM_8x_capable();
    sha_ext_ = OPENSSL_armcap_P & ARMV8_SHA256;
    sha_512_ext_ = OPENSSL_armcap_P & ARMV8_SHA512;
    sha3_ext_ = CRYPTO_is_ARMv8_SHA3_capable();
    neoverse_n1_ = CRYPTO_is_Neoverse_N1();
    neoverse_v1_ = CRYPTO_is_Neoverse_V1();
    neoverse_v2_ = CRYPTO_is_Neoverse_V2();

    assembler_has_neon_sha3_extension_ =
#if defined(MY_ASSEMBLER_SUPPORTS_NEON_SHA3_EXTENSION)
        true;
#else
        false;
#endif
#endif
    have_s2n_bignum_asm_ =
#if defined(KECCAK1600_S2N_BIGNUM_ASM)
        true;
#else
        false;
#endif
  }

 protected:
  // AssertFunctionsHit takes a list of pairs (flag index, boolean), and a
  // function to test. It runs the given function and asserts, for each flag
  // index, that the boolean reflects whether that flag index was written or
  // not, and that no other flagged functions were triggered.
  void AssertFunctionsHit(std::vector<std::pair<size_t, bool>> flags,
                          std::function<void()> f) {
    OPENSSL_memset(BORINGSSL_function_hit, 0, sizeof(BORINGSSL_function_hit));

    f();

    for (const auto& flag : flags) {
      SCOPED_TRACE(flag.first);
      ASSERT_LT(flag.first, sizeof(BORINGSSL_function_hit));
      EXPECT_EQ(flag.second, BORINGSSL_function_hit[flag.first] == 1);
      BORINGSSL_function_hit[flag.first] = 0;
    }

    for (size_t i = 0; i < sizeof(BORINGSSL_function_hit); i++) {
      EXPECT_EQ(0u, BORINGSSL_function_hit[i])
          << "Flag " << i << " unexpectedly hit";
    }
  }

  bool aes_hw_ = false;
  bool aes_vpaes_ = false;
  bool sha_ext_ = false;
  bool have_s2n_bignum_asm_ = false;
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
  bool vaes_vpclmulqdq_ = false;
  bool avx_movbe_ = false;
  bool is_x86_64_ = false;
  bool is_avx2_ = false;
  bool is_assembler_too_old = false;
  bool is_assembler_too_old_avx512 = false;
  bool ifma_avx512 = false;
#else // AARCH64
  bool aes_gcm_pmull_ = false;
  bool aes_gcm_8x_ = false;
  bool sha_512_ext_ = false;
  bool sha3_ext_ = false;
  bool neoverse_n1_ = false;
  bool neoverse_v1_ = false;
  bool neoverse_v2_ = false;
  bool assembler_has_neon_sha3_extension_ = false;
#endif

};

#if !defined(OPENSSL_NO_ASM) && (defined(OPENSSL_X86) || \
    defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64))

constexpr size_t kFlag_aes_hw_ctr32_encrypt_blocks = 0;
constexpr size_t kFlag_aes_hw_encrypt = 1;
constexpr size_t kFlag_aes_hw_set_encrypt_key = 3;
constexpr size_t kFlag_vpaes_encrypt = 4;
constexpr size_t kFlag_vpaes_set_encrypt_key = 5;
constexpr size_t kFlag_sha256_hw = 6;
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
constexpr size_t kFlag_aesni_gcm_encrypt = 2;
constexpr size_t kFlag_aes_gcm_encrypt_avx512 = 7;
constexpr size_t kFlag_RSAZ_mod_exp_avx512_x2 = 8;
constexpr size_t kFlag_sha3_keccak_f1600 = 9;
constexpr size_t kFlag_sha3_keccak4_f1600_alt = 10;
#else // AARCH64
constexpr size_t kFlag_aes_gcm_enc_kernel = 2;
constexpr size_t kFlag_aesv8_gcm_8x_enc_128 = 7;
constexpr size_t kFlag_sha512_hw = 8;
constexpr size_t kFlag_KeccakF1600_hw = 9;
constexpr size_t kFlag_sha3_keccak_f1600 = 10;
constexpr size_t kFlag_sha3_keccak_f1600_alt = 11;
constexpr size_t kFlag_sha3_keccak2_f1600 = 12;
constexpr size_t kFlag_sha3_keccak4_f1600_alt = 13;
constexpr size_t kFlag_sha3_keccak4_f1600_alt2 = 14;
#endif

TEST_F(ImplDispatchTest, AEAD_AES_GCM) {
  AssertFunctionsHit(
      {
          {kFlag_aes_hw_encrypt, aes_hw_},
          {kFlag_aes_hw_set_encrypt_key, aes_hw_},
          {kFlag_vpaes_encrypt, aes_vpaes_ && !aes_hw_},
          {kFlag_vpaes_set_encrypt_key, aes_vpaes_ && !aes_hw_},
#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
          {kFlag_aes_hw_ctr32_encrypt_blocks, aes_hw_ &&
           (!is_x86_64_ || is_assembler_too_old ||
            !(vaes_vpclmulqdq_ && !is_assembler_too_old_avx512))},
          {kFlag_aesni_gcm_encrypt,
           is_x86_64_ && aes_hw_ && avx_movbe_ &&
           !is_assembler_too_old &&
           !(vaes_vpclmulqdq_ && !is_assembler_too_old_avx512)},
          {kFlag_aes_gcm_encrypt_avx512,
           is_x86_64_ && aes_hw_ &&
           !is_assembler_too_old_avx512 &&
           vaes_vpclmulqdq_},
#else // AARCH64
          {kFlag_aes_hw_ctr32_encrypt_blocks, aes_hw_ &&
           !aes_gcm_pmull_ && !aes_gcm_8x_},
          {kFlag_aes_gcm_enc_kernel, aes_hw_ &&
           aes_gcm_pmull_ && !aes_gcm_8x_},
          {kFlag_aesv8_gcm_8x_enc_128, aes_hw_ &&
           aes_gcm_pmull_ && aes_gcm_8x_}
#endif
      },
      [] {
        const uint8_t kZeros[16] = {0};
        const uint8_t kPlaintext[256] = {1, 2, 3, 4, 0};
        uint8_t ciphertext[sizeof(kPlaintext) + 16];
        size_t ciphertext_len;
        bssl::ScopedEVP_AEAD_CTX ctx;
        ASSERT_TRUE(EVP_AEAD_CTX_init(ctx.get(), EVP_aead_aes_128_gcm(), kZeros,
                                      sizeof(kZeros),
                                      EVP_AEAD_DEFAULT_TAG_LENGTH, nullptr));
        ASSERT_TRUE(EVP_AEAD_CTX_seal(
            ctx.get(), ciphertext, &ciphertext_len, sizeof(ciphertext), kZeros,
            EVP_AEAD_nonce_length(EVP_aead_aes_128_gcm()), kPlaintext,
            sizeof(kPlaintext), nullptr, 0));
      });
}

TEST_F(ImplDispatchTest, AES_set_encrypt_key) {
  AssertFunctionsHit(
      {
          {kFlag_aes_hw_set_encrypt_key, aes_hw_},
          {kFlag_vpaes_set_encrypt_key, aes_vpaes_ && !aes_hw_},
      },
      [] {
        AES_KEY key;
        static const uint8_t kZeros[16] = {0};
        AES_set_encrypt_key(kZeros, sizeof(kZeros) * 8, &key);
      });
}

TEST_F(ImplDispatchTest, AES_single_block) {
  AES_KEY key;
  static const uint8_t kZeros[16] = {0};
  AES_set_encrypt_key(kZeros, sizeof(kZeros) * 8, &key);

  AssertFunctionsHit(
      {
          {kFlag_aes_hw_encrypt, aes_hw_},
          {kFlag_vpaes_encrypt, aes_vpaes_ && !aes_hw_},
      },
      [&key] {
        uint8_t in[AES_BLOCK_SIZE] = {0};
        uint8_t out[AES_BLOCK_SIZE];
        AES_encrypt(in, out, &key);
      });
}

TEST_F(ImplDispatchTest, SHA256) {
  AssertFunctionsHit(
      {
          {kFlag_sha256_hw, sha_ext_},
      },
      [] {
        const uint8_t in[32] = {0};
        uint8_t out[SHA256_DIGEST_LENGTH];
        SHA256(in, 32, out);
      });
}

#ifdef OPENSSL_AARCH64
TEST_F(ImplDispatchTest, SHA512) {
  AssertFunctionsHit(
      {
          {kFlag_sha512_hw, sha_512_ext_},
      },
      [] {
        const uint8_t in[32] = {0};
        uint8_t out[SHA512_DIGEST_LENGTH];
        SHA512(in, 32, out);
      });
}

TEST_F(ImplDispatchTest, SHA3_512) {
  // Assembly dispatch logic for Keccak-x1 on AArch64:
  // - For Neoverse N1, V1, V2, we use scalar Keccak assembly from s2n-bignum
  //   (`sha3_keccak_f1600()`)
  //   leveraging lazy rotations from https://eprint.iacr.org/2022/1243.
  // - Otherwise, if the Neon SHA3 extension is supported, we use the Neon
  //   Keccak assembly from s2n-bignum (`sha3_keccak_f1600_alt()`),
  //   leveraging that extension.
  // - Otherwise, fall back to scalar Keccak implementation from OpenSSL,
  //   (`Keccak1600_hw()`), not using lazy rotations.
  AssertFunctionsHit(
      {
          {kFlag_sha3_keccak_f1600,
           have_s2n_bignum_asm_ &&
           (neoverse_n1_ || neoverse_v1_ || neoverse_v2_) },
          {kFlag_sha3_keccak_f1600_alt,
           have_s2n_bignum_asm_ &&
           !(neoverse_n1_ || neoverse_v1_ || neoverse_v2_) &&
           (assembler_has_neon_sha3_extension_ && sha3_ext_) },
          {kFlag_KeccakF1600_hw,
           !have_s2n_bignum_asm_ ||
           (
             !(neoverse_n1_ || neoverse_v1_ || neoverse_v2_) &&
             !(assembler_has_neon_sha3_extension_ && sha3_ext_)
           ) },
      },
      [] {
        const uint8_t in[32] = {0};
        uint8_t out[SHA3_512_DIGEST_LENGTH];
        SHA3_512(in, 32, out);
      });
}
#endif // OPENSSL_AARCH64

TEST_F(ImplDispatchTest, SHAKE256_Batched) {
#if defined(OPENSSL_X86_64) || defined(OPENSSL_X86)
  // Assembly dispatch logic for Keccak-x4 on x86:
  // - For platforms with AVX2 support, we use batched Keccak assembly from s2n-bignum
  //   (`sha3_keccak4_f1600_alt()`).
  // - Otherwise, fall back to scalar Keccak implementation from s2n-bignum,
  //   (`sha3_keccak_f1600()`).
  AssertFunctionsHit(
      {
          {kFlag_sha3_keccak4_f1600_alt,
           have_s2n_bignum_asm_ &&
           is_avx2_ },
           {kFlag_sha3_keccak_f1600,
           have_s2n_bignum_asm_ && is_x86_64_ && !is_avx2_ },
      },
#else // AARCH64
  // Assembly dispatch logic for Keccak-x4 on AArch64:
  // - For Neoverse N1, we use scalar batched hybrid Keccak assembly from s2n-bignum
  //   (`sha3_keccak4_f1600_alt()`) leveraging Neon and scalar assembly with
  //   lazy rotations.
  // - For Neoverse V1, V2, we use SIMD batched hybrid Keccak assembly from s2n-bignum
  //   (`sha3_keccak4_f1600_alt2()`) leveraging Neon, Neon SHA3 extension,
  //   and scalar assembly with lazy rotations.
  // - Otherwise, if the Neon SHA3 extension is supported, we use the 2-fold
  //   Keccak assembly from s2n-bignum (`sha3_keccak2_f1600()`) twice,
  //   which is a straightforward implementation using the SHA3 extension.
  // - Otherwise, fall back to four times the 1-fold Keccak implementation
  //   with its own dispatch logic.
  AssertFunctionsHit(
      {
          {kFlag_sha3_keccak4_f1600_alt,
           have_s2n_bignum_asm_ && neoverse_n1_},
          {kFlag_sha3_keccak4_f1600_alt2,
           have_s2n_bignum_asm_ &&
           (neoverse_v1_ || neoverse_v2_) &&
           assembler_has_neon_sha3_extension_},
          {kFlag_sha3_keccak2_f1600,
           have_s2n_bignum_asm_ &&
           !(neoverse_n1_ || neoverse_v1_ || neoverse_v2_) &&
           (assembler_has_neon_sha3_extension_ && sha3_ext_)},
          // If we don't have assembly batched Keccak available,
          // we fall back to the dispatch logic in KeccakF1600().
          // Under the assumption that no batched Keccak assembly
          // was chosen, this simplifies as follows:
          // 1. If we run on Neoverse-V1 and Neoverse-V2 and there is
          //    no compiler support for SHA3 (otherwise, we would have
          //    have chosen the batched hybrid with SHA3 extension),
          //    we use the scalar assembly with lazy rotation.
          // 2. Otherwise, we fall back to the OpenSSL assembly.
          {kFlag_sha3_keccak_f1600,
           have_s2n_bignum_asm_ && (neoverse_v1_ || neoverse_v2_) &&
           !(assembler_has_neon_sha3_extension_ && sha3_ext_) },
          {kFlag_KeccakF1600_hw,
           !have_s2n_bignum_asm_ ||
           (
	       !neoverse_n1_ && !neoverse_v1_ && !neoverse_v2_ &&
	       !(assembler_has_neon_sha3_extension_ && sha3_ext_)
	   ) },
      },
#endif
      [] {
        const uint8_t in[32] = {0};
        uint8_t out0[32], out1[32], out2[32], out3[32];
        SHAKE256_x4(in, in, in, in, 32, out0, out1, out2, out3, 32);
      });
}

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
static bssl::UniquePtr<BIGNUM> GetBIGNUM(FileTest *t, const char *attr);

static bssl::UniquePtr<BIGNUM> GetBIGNUM(FileTest *t, const char *attr) {
  std::string hex;
  if (!t->GetAttribute(&hex, attr)) {
    return nullptr;
  }

  BIGNUM *raw = NULL;
  int size = BN_hex2bn(&raw, hex.c_str());
  if (size != static_cast<int>(hex.size())) {
    t->PrintLine("Could not decode '%s'.", hex.c_str());
    return nullptr;
  }

  bssl::UniquePtr<BIGNUM> ret;
  (&ret)->reset(raw);
  return ret;
}

TEST_F(ImplDispatchTest, BN_mod_exp_mont_consttime_x2) {
  FileTestGTest(
    "crypto/fipsmodule/bn/test/mod_exp_x2_tests.txt",
    [&](FileTest *t) {
    AssertFunctionsHit(
      {
        {kFlag_RSAZ_mod_exp_avx512_x2,
         is_x86_64_ &&
         !is_assembler_too_old_avx512 &&
         ifma_avx512},
      },
      [&]() {
        BN_CTX *ctx = BN_CTX_new();
        BN_CTX_start(ctx);
        bssl::UniquePtr<BIGNUM> a1 = GetBIGNUM(t, "A1");
        bssl::UniquePtr<BIGNUM> e1 = GetBIGNUM(t, "E1");
        bssl::UniquePtr<BIGNUM> m1 = GetBIGNUM(t, "M1");
        bssl::UniquePtr<BIGNUM> mod_exp1 = GetBIGNUM(t, "ModExp1");
        ASSERT_TRUE(a1);
        ASSERT_TRUE(e1);
        ASSERT_TRUE(m1);
        ASSERT_TRUE(mod_exp1);

        bssl::UniquePtr<BIGNUM> a2 = GetBIGNUM(t, "A2");
        bssl::UniquePtr<BIGNUM> e2 = GetBIGNUM(t, "E2");
        bssl::UniquePtr<BIGNUM> m2 = GetBIGNUM(t, "M2");
        bssl::UniquePtr<BIGNUM> mod_exp2 = GetBIGNUM(t, "ModExp2");
        ASSERT_TRUE(a2);
        ASSERT_TRUE(e2);
        ASSERT_TRUE(m2);
        ASSERT_TRUE(mod_exp2);

        bssl::UniquePtr<BIGNUM> ret1(BN_new());
        ASSERT_TRUE(ret1);

        bssl::UniquePtr<BIGNUM> ret2(BN_new());
        ASSERT_TRUE(ret2);

        ASSERT_TRUE(BN_nnmod(a1.get(), a1.get(), m1.get(), ctx));
        ASSERT_TRUE(BN_nnmod(a2.get(), a2.get(), m2.get(), ctx));

        BN_MONT_CTX *mont1 = NULL;
        BN_MONT_CTX *mont2 = NULL;

        ASSERT_TRUE(mont1 = BN_MONT_CTX_new());
        ASSERT_TRUE(BN_MONT_CTX_set(mont1, m1.get(), ctx));
        ASSERT_TRUE(mont2 = BN_MONT_CTX_new());
        ASSERT_TRUE(BN_MONT_CTX_set(mont2, m2.get(), ctx));

        BN_mod_exp_mont_consttime_x2(ret1.get(), a1.get(), e1.get(), m1.get(), mont1,
                                     ret2.get(), a2.get(), e2.get(), m2.get(), mont2,
                                     ctx);

        BN_MONT_CTX_free(mont1);
        BN_MONT_CTX_free(mont2);
        BN_CTX_end(ctx);
        BN_CTX_free(ctx);
      });
    });
}

#endif // x86[_64]

#endif  // !OPENSSL_NO_ASM && (OPENSSL_X86 || OPENSSL_X86_64 || OPENSSL_AARCH64)

#endif  // DISPATCH_TEST && !SHARED_LIBRARY
