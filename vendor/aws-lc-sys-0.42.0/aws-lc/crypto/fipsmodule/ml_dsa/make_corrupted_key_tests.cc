// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
//
// Generates test vectors with intentionally corrupted ML-DSA private keys
//
// USAGE:
//   cd crypto/fipsmodule/ml_dsa
//   make generate
//
// This regenerates crypto/evp_extra/mldsa_corrupted_key_tests.txt

#include <openssl/evp.h>
#include <openssl/obj.h>

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <functional>
#include <iostream>
#include <vector>

// Need ML-DSA internal headers to manipulate the expanded private key
extern "C" {
#include "./ml_dsa_ref/packing.h"
#include "./ml_dsa_ref/params.h"
#include "./ml_dsa_ref/polyvec.h"
}

static void PrintHex(const std::vector<uint8_t> &data) {
  for (uint8_t byte : data) {
    printf("%02x", byte);
  }
}

struct MLDSAParamSet {
  const char name[20];
  const int nid;
};

static const struct MLDSAParamSet kMLDSAs[] = {{"MLDSA44", NID_MLDSA44},
                                               {"MLDSA65", NID_MLDSA65},
                                               {"MLDSA87", NID_MLDSA87}};


// Corruption function type: takes unpacked key components and corrupts them
using CorruptionFn =
    std::function<void(polyvecl *s1, polyveck *s2, polyveck *t0)>;


// Generates a corrupted private key with the provided corruption
static bool GenerateCorruptedKey(const MLDSAParamSet &ps, ml_dsa_params *params,
                                 const std::vector<uint8_t> &honest_key_bytes,
                                 const CorruptionFn &corruption) {
  polyvecl s1;
  polyveck s2, t0;
  uint8_t rho[ML_DSA_SEEDBYTES];
  uint8_t tr[ML_DSA_TRBYTES];
  uint8_t key[ML_DSA_SEEDBYTES];

  // Unpack the honest key
  ml_dsa_unpack_sk(params, rho, tr, key, &t0, &s1, &s2,
                   honest_key_bytes.data());

  // Apply the corruption
  corruption(&s1, &s2, &t0);

  // Repack the corrupted key
  std::vector<uint8_t> corrupted_key(params->secret_key_bytes);
  ml_dsa_pack_sk(params, corrupted_key.data(), rho, tr, key, &t0, &s1, &s2);

  // Verify the corrupted key differs from the honest key
  assert(std::memcmp(corrupted_key.data(), honest_key_bytes.data(),
                     params->secret_key_bytes) != 0);

  // Output the test vector
  printf("# corrupted private key with invalid s1 or s2, inconsistent\n");
  printf("CorruptedKey = ");
  PrintHex(corrupted_key);
  printf("\n\n");

  // Create a consistent version by recomputing the public key and tr
  std::vector<uint8_t> consistent_key = corrupted_key;

  // Recompute the public key. We cannot use ml_dsa_pack_pk_from_sk since we
  // fixed it to fail for invalid secret keys. Instead we adapt from
  // https://github.com/aws/aws-lc/blob/0336dd78a0f2623c1f9b209a98cd497026d9c779/crypto/fipsmodule/ml_dsa/ml_dsa_ref/packing.c#L7-L61
  ml_dsa_unpack_sk(params, rho, tr, key, &t0, &s1, &s2, consistent_key.data());
  polyvecl mat[ML_DSA_K_MAX];
  ml_dsa_polyvec_matrix_expand(params, mat, rho);
  ml_dsa_polyvecl_ntt(params, &s1);
  polyveck t1;
  ml_dsa_polyvec_matrix_pointwise_montgomery(params, &t1, mat, &s1);
  ml_dsa_polyveck_reduce(params, &t1);
  ml_dsa_polyveck_invntt_tomont(params, &t1);
  ml_dsa_polyveck_add(params, &t1, &t1, &s2);
  ml_dsa_polyveck_caddq(params, &t1);
  ml_dsa_polyveck_power2round(params, &t1, &t0, &t1);
  std::vector<uint8_t> consistent_pk(params->public_key_bytes);
  ml_dsa_pack_pk(params, consistent_pk.data(), rho, &t1);

  // Recompute tr = SHAKE256(pk, 64)
  std::vector<uint8_t> new_tr(ML_DSA_TRBYTES);
  bssl::ScopedEVP_MD_CTX md_ctx;
  if (!EVP_DigestInit_ex(md_ctx.get(), EVP_shake256(), nullptr) ||
      !EVP_DigestUpdate(md_ctx.get(), consistent_pk.data(),
                        params->public_key_bytes) ||
      !EVP_DigestFinalXOF(md_ctx.get(), new_tr.data(), new_tr.size())) {
    return false;
  }

  // Repack the consistent corrupted key
  ml_dsa_pack_sk(params, consistent_key.data(), rho, new_tr.data(),
                 consistent_pk.data(), &t0, &s1, &s2);

  // Verify the consistent key differs from the inconsistent one
  assert(std::memcmp(consistent_key.data(), corrupted_key.data(),
                     params->secret_key_bytes) != 0);

  // Output the test vector
  printf("# corrupted private key with invalid s1 or s2, consistent\n");
  printf("CorruptedKey = ");
  PrintHex(consistent_key);
  printf("\n\n");

  return true;
}

static bool InitializeParams(int nid, ml_dsa_params *params) {
  if (nid == NID_MLDSA44) {
    ml_dsa_44_params_init(params);
  } else if (nid == NID_MLDSA65) {
    ml_dsa_65_params_init(params);
  } else if (nid == NID_MLDSA87) {
    ml_dsa_87_params_init(params);
  } else {
    std::cerr << "Unexpected NID: " << nid << "\n";
    return false;
  }
  return true;
}

static bool GenerateHonestKey(const MLDSAParamSet &ps,
                              const ml_dsa_params &params,
                              std::vector<uint8_t> *honest_key_bytes) {
  // Generate an honest private key from a fixed seed
  const std::vector<uint8_t> seed(32, 0x42);
  bssl::UniquePtr<EVP_PKEY> honest_pkey(
      EVP_PKEY_pqdsa_new_raw_private_key(ps.nid, seed.data(), seed.size()));
  if (!honest_pkey) {
    std::cerr << "Failed to generate honest key for " << ps.name << "\n";
    return false;
  }

  // Export the honest private key to bytes
  size_t key_len = params.secret_key_bytes;
  honest_key_bytes->resize(key_len);
  if (!EVP_PKEY_get_raw_private_key(honest_pkey.get(), honest_key_bytes->data(),
                                    &key_len)) {
    std::cerr << "Failed to export honest key for " << ps.name << "\n";
    return false;
  }
  return true;
}

static std::vector<CorruptionFn> CreateCorruptionFunctions(
    const ml_dsa_params &params, int vec_index, int coeff_index) {
  return {
      // Corrupt s1 with eta + 1
      [&params, vec_index, coeff_index](polyvecl *s1, polyveck *, polyveck *) {
        s1->vec[vec_index].coeffs[coeff_index] = params.eta + 1;
      },
      // Corrupt s1 with -(eta + 1)
      [&params, vec_index, coeff_index](polyvecl *s1, polyveck *, polyveck *) {
        s1->vec[vec_index].coeffs[coeff_index] = -(params.eta + 1);
      },
      // Corrupt s2 with eta + 1
      [&params, vec_index, coeff_index](polyvecl *, polyveck *s2, polyveck *) {
        s2->vec[vec_index].coeffs[coeff_index] = params.eta + 1;
      },
      // Corrupt s2 with -(eta + 1)
      [&params, vec_index, coeff_index](polyvecl *, polyveck *s2, polyveck *) {
        s2->vec[vec_index].coeffs[coeff_index] = -(params.eta + 1);
      },
  };
}

int main() {
  printf(
      "# Invalid ML-DSA extended private keys\n"
      "# This file was generated by "
      "crypto/fipsmodule/ml_dsa/make_corrupted_key_tests.cc\n\n");

  for (const auto &ps : kMLDSAs) {
    printf("[ParamSet = %s]\n", ps.name);

    ml_dsa_params params;
    if (!InitializeParams(ps.nid, &params)) {
      return 1;
    }

    std::vector<uint8_t> honest_key_bytes;
    if (!GenerateHonestKey(ps, params, &honest_key_bytes)) {
      return 1;
    }

    // Test coefficient indices: first, last, and some random ones
    const int coeff_indices[] = {0, 255, 127, 95, 42, 224};

    for (int vec_index = 0; vec_index < params.l; vec_index++) {
      for (int coeff_index : coeff_indices) {
        const std::vector<CorruptionFn> corruptions =
            CreateCorruptionFunctions(params, vec_index, coeff_index);

        for (const auto &corruption : corruptions) {
          if (!GenerateCorruptedKey(ps, &params, honest_key_bytes,
                                    corruption)) {
            return 1;
          }
        }
      }
    }
  }
  return 0;
}
