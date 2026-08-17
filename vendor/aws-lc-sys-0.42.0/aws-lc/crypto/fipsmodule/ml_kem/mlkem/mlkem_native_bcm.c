/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*
 * WARNING: This file is auto-generated from scripts/autogen
 *          in the mlkem-native repository.
 *          Do not modify it directly.
 */

/******************************************************************************
 *
 * Single compilation unit (SCU) for fixed-level build of mlkem-native
 *
 * This compilation unit bundles together all source files for a build
 * of mlkem-native for a fixed security level (MLKEM-512/768/1024).
 *
 * # API
 *
 * The API exposed by this file is described in mlkem_native.h.
 *
 * # Multi-level build
 *
 * If you want an SCU build of mlkem-native with support for multiple security
 * levels, you need to include this file multiple times, and set
 * MLK_CONFIG_MULTILEVEL_WITH_SHARED and MLK_CONFIG_MULTILEVEL_NO_SHARED
 * appropriately. This is exemplified in examples/monolithic_build_multilevel
 * and examples/monolithic_build_multilevel_native.
 *
 * # Configuration
 *
 * The following options from the mlkem-native configuration are relevant:
 *
 * - MLK_CONFIG_FIPS202_CUSTOM_HEADER
 *   Set this option if you use a custom FIPS202 implementation.
 *
 * - MLK_CONFIG_USE_NATIVE_BACKEND_ARITH
 *   Set this option if you want to include the native arithmetic backends
 *   in your build.
 *
 * - MLK_CONFIG_USE_NATIVE_BACKEND_FIPS202
 *   Set this option if you want to include the native FIPS202 backends
 *   in your build.
 *
 * - MLK_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS
 *   Set this option if you want to keep the directives defined in
 *   level-independent headers. This is needed for a multi-level build.
 */

/* If parts of the mlkem-native source tree are not used,
 * consider reducing this header via `unifdef`.
 *
 * Example:
 * ```bash
 * unifdef -UMLK_CONFIG_USE_NATIVE_BACKEND_ARITH mlkem_native.c
 * ```
 */

#include "common.h"

#include "compress.c"
#include "debug.c"
#include "indcpa.c"
#include "kem.c"
#include "poly.c"
#include "poly_k.c"
#include "sampling.c"
#include "verify.c"


#if defined(MLK_CONFIG_USE_NATIVE_BACKEND_ARITH)
#if defined(MLK_SYS_AARCH64)
#include "native/aarch64/src/aarch64_zetas.c"
#include "native/aarch64/src/rej_uniform_table.c"
#endif
#if defined(MLK_SYS_X86_64)
#include "native/x86_64/src/compress_consts.c"
#include "native/x86_64/src/consts.c"
#include "native/x86_64/src/rej_uniform_table.c"
#endif
#endif /* MLK_CONFIG_USE_NATIVE_BACKEND_ARITH */


/* Macro #undef's
 *
 * The following undefines macros from headers
 * included by the source files imported above.
 *
 * This is to allow building and linking multiple builds
 * of mlkem-native for varying parameter sets through concatenation
 * of this file, as if the files had been compiled separately.
 * If this is not relevant to you, you may remove the following.
 */

/*
 * Undefine macros from MLK_CONFIG_PARAMETER_SET-specific files
 */
/* mlkem/mlkem_native.h */
#undef CRYPTO_BYTES
#undef CRYPTO_CIPHERTEXTBYTES
#undef CRYPTO_PUBLICKEYBYTES
#undef CRYPTO_SECRETKEYBYTES
#undef CRYPTO_SYMBYTES
#undef MLKEM1024_BYTES
#undef MLKEM1024_CIPHERTEXTBYTES
#undef MLKEM1024_PUBLICKEYBYTES
#undef MLKEM1024_SECRETKEYBYTES
#undef MLKEM1024_SYMBYTES
#undef MLKEM512_BYTES
#undef MLKEM512_CIPHERTEXTBYTES
#undef MLKEM512_PUBLICKEYBYTES
#undef MLKEM512_SECRETKEYBYTES
#undef MLKEM512_SYMBYTES
#undef MLKEM768_BYTES
#undef MLKEM768_CIPHERTEXTBYTES
#undef MLKEM768_PUBLICKEYBYTES
#undef MLKEM768_SECRETKEYBYTES
#undef MLKEM768_SYMBYTES
#undef MLKEM_BYTES
#undef MLKEM_CIPHERTEXTBYTES
#undef MLKEM_CIPHERTEXTBYTES_
#undef MLKEM_PUBLICKEYBYTES
#undef MLKEM_PUBLICKEYBYTES_
#undef MLKEM_SECRETKEYBYTES
#undef MLKEM_SECRETKEYBYTES_
#undef MLKEM_SYMBYTES
#undef MLK_API_CONCAT
#undef MLK_API_CONCAT_
#undef MLK_API_CONCAT_UNDERSCORE
#undef MLK_API_LEGACY_CONFIG
#undef MLK_API_MUST_CHECK_RETURN_VALUE
#undef MLK_API_NAMESPACE
#undef MLK_API_QUALIFIER
#undef MLK_CONFIG_API_CONSTANTS_ONLY
#undef MLK_CONFIG_API_NAMESPACE_PREFIX
#undef MLK_CONFIG_API_NO_SUPERCOP
#undef MLK_CONFIG_API_PARAMETER_SET
#undef MLK_CONFIG_API_QUALIFIER
#undef MLK_ERR_FAIL
#undef MLK_ERR_OUT_OF_MEMORY
#undef MLK_ERR_RNG_FAIL
#undef MLK_H
#undef MLK_MAX3_
#undef MLK_TOTAL_ALLOC_1024
#undef MLK_TOTAL_ALLOC_1024_DECAPS
#undef MLK_TOTAL_ALLOC_1024_ENCAPS
#undef MLK_TOTAL_ALLOC_1024_KEYPAIR
#undef MLK_TOTAL_ALLOC_1024_KEYPAIR_NO_PCT
#undef MLK_TOTAL_ALLOC_1024_KEYPAIR_PCT
#undef MLK_TOTAL_ALLOC_512
#undef MLK_TOTAL_ALLOC_512_DECAPS
#undef MLK_TOTAL_ALLOC_512_ENCAPS
#undef MLK_TOTAL_ALLOC_512_KEYPAIR
#undef MLK_TOTAL_ALLOC_512_KEYPAIR_NO_PCT
#undef MLK_TOTAL_ALLOC_512_KEYPAIR_PCT
#undef MLK_TOTAL_ALLOC_768
#undef MLK_TOTAL_ALLOC_768_DECAPS
#undef MLK_TOTAL_ALLOC_768_ENCAPS
#undef MLK_TOTAL_ALLOC_768_KEYPAIR
#undef MLK_TOTAL_ALLOC_768_KEYPAIR_NO_PCT
#undef MLK_TOTAL_ALLOC_768_KEYPAIR_PCT
#undef crypto_kem_check_pk
#undef crypto_kem_check_sk
#undef crypto_kem_dec
#undef crypto_kem_enc
#undef crypto_kem_enc_derand
#undef crypto_kem_keypair
#undef crypto_kem_keypair_derand
/* mlkem/src/common.h */
#undef MLK_ADD_PARAM_SET
#undef MLK_ALLOC
#undef MLK_APPLY
#undef MLK_ASM_FN_SIZE
#undef MLK_ASM_FN_SYMBOL
#undef MLK_ASM_NAMESPACE
#undef MLK_BUILD_INTERNAL
#undef MLK_COMMON_H
#undef MLK_CONCAT
#undef MLK_CONCAT_
#undef MLK_CONTEXT_PARAMETERS_0
#undef MLK_CONTEXT_PARAMETERS_1
#undef MLK_CONTEXT_PARAMETERS_2
#undef MLK_CONTEXT_PARAMETERS_3
#undef MLK_CONTEXT_PARAMETERS_4
#undef MLK_EMPTY_CU
#undef MLK_ERR_FAIL
#undef MLK_ERR_OUT_OF_MEMORY
#undef MLK_ERR_RNG_FAIL
#undef MLK_EXTERNAL_API
#undef MLK_FIPS202X4_HEADER_FILE
#undef MLK_FIPS202_HEADER_FILE
#undef MLK_FREE
#undef MLK_INTERNAL_API
#undef MLK_NAMESPACE
#undef MLK_NAMESPACE_K
#undef MLK_NAMESPACE_PREFIX
#undef MLK_NAMESPACE_PREFIX_K
#undef mlk_memcpy
#undef mlk_memset
/* mlkem/src/indcpa.h */
#undef MLK_INDCPA_H
#undef mlk_gen_matrix
#undef mlk_indcpa_dec
#undef mlk_indcpa_enc
#undef mlk_indcpa_keypair_derand
/* mlkem/src/kem.h */
#undef MLK_KEM_H
#undef mlk_kem_check_pk
#undef mlk_kem_check_sk
#undef mlk_kem_dec
#undef mlk_kem_enc
#undef mlk_kem_enc_derand
#undef mlk_kem_keypair
#undef mlk_kem_keypair_derand
/* mlkem/src/params.h */
#undef MLKEM_DU
#undef MLKEM_DV
#undef MLKEM_ETA1
#undef MLKEM_ETA2
#undef MLKEM_INDCCA_CIPHERTEXTBYTES
#undef MLKEM_INDCCA_PUBLICKEYBYTES
#undef MLKEM_INDCCA_SECRETKEYBYTES
#undef MLKEM_INDCPA_BYTES
#undef MLKEM_INDCPA_MSGBYTES
#undef MLKEM_INDCPA_PUBLICKEYBYTES
#undef MLKEM_INDCPA_SECRETKEYBYTES
#undef MLKEM_K
#undef MLKEM_N
#undef MLKEM_POLYBYTES
#undef MLKEM_POLYCOMPRESSEDBYTES_D10
#undef MLKEM_POLYCOMPRESSEDBYTES_D11
#undef MLKEM_POLYCOMPRESSEDBYTES_D4
#undef MLKEM_POLYCOMPRESSEDBYTES_D5
#undef MLKEM_POLYCOMPRESSEDBYTES_DU
#undef MLKEM_POLYCOMPRESSEDBYTES_DV
#undef MLKEM_POLYVECBYTES
#undef MLKEM_POLYVECCOMPRESSEDBYTES_DU
#undef MLKEM_Q
#undef MLKEM_Q_HALF
#undef MLKEM_SSBYTES
#undef MLKEM_SYMBYTES
#undef MLKEM_UINT12_LIMIT
#undef MLK_PARAMS_H
/* mlkem/src/poly_k.h */
#undef MLK_POLY_K_H
#undef mlk_poly_compress_du
#undef mlk_poly_compress_dv
#undef mlk_poly_decompress_du
#undef mlk_poly_decompress_dv
#undef mlk_poly_getnoise_eta1122_4x
#undef mlk_poly_getnoise_eta1_4x
#undef mlk_poly_getnoise_eta2
#undef mlk_poly_getnoise_eta2_4x
#undef mlk_polymat
#undef mlk_polyvec
#undef mlk_polyvec_add
#undef mlk_polyvec_basemul_acc_montgomery_cached
#undef mlk_polyvec_compress_du
#undef mlk_polyvec_decompress_du
#undef mlk_polyvec_frombytes
#undef mlk_polyvec_invntt_tomont
#undef mlk_polyvec_mulcache
#undef mlk_polyvec_mulcache_compute
#undef mlk_polyvec_ntt
#undef mlk_polyvec_reduce
#undef mlk_polyvec_tobytes
#undef mlk_polyvec_tomont

#if !defined(MLK_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS)
/*
 * Undefine macros from MLK_CONFIG_PARAMETER_SET-generic files
 */
/* mlkem/src/compress.h */
#undef MLK_COMPRESS_H
#undef mlk_poly_compress_d10
#undef mlk_poly_compress_d11
#undef mlk_poly_compress_d4
#undef mlk_poly_compress_d5
#undef mlk_poly_decompress_d10
#undef mlk_poly_decompress_d11
#undef mlk_poly_decompress_d4
#undef mlk_poly_decompress_d5
#undef mlk_poly_frombytes
#undef mlk_poly_frommsg
#undef mlk_poly_tobytes
#undef mlk_poly_tomsg
/* mlkem/src/debug.h */
#undef MLK_DEBUG_H
#undef mlk_assert
#undef mlk_assert_abs_bound
#undef mlk_assert_abs_bound_2d
#undef mlk_assert_bound
#undef mlk_assert_bound_2d
#undef mlk_debug_check_assert
#undef mlk_debug_check_bounds
/* mlkem/src/poly.h */
#undef MLK_INVNTT_BOUND
#undef MLK_NTT_BOUND
#undef MLK_POLY_H
#undef mlk_poly_add
#undef mlk_poly_invntt_tomont
#undef mlk_poly_mulcache_compute
#undef mlk_poly_ntt
#undef mlk_poly_reduce
#undef mlk_poly_sub
#undef mlk_poly_tomont
/* mlkem/src/randombytes.h */
#undef MLK_RANDOMBYTES_H
/* mlkem/src/sampling.h */
#undef MLK_SAMPLING_H
#undef mlk_poly_cbd2
#undef mlk_poly_cbd3
#undef mlk_poly_rej_uniform
#undef mlk_poly_rej_uniform_x4
/* mlkem/src/symmetric.h */
#undef MLK_SYMMETRIC_H
#undef MLK_XOF_RATE
#undef mlk_hash_g
#undef mlk_hash_h
#undef mlk_hash_j
#undef mlk_prf_eta
#undef mlk_prf_eta1
#undef mlk_prf_eta1_x4
#undef mlk_prf_eta2
#undef mlk_xof_absorb
#undef mlk_xof_ctx
#undef mlk_xof_init
#undef mlk_xof_release
#undef mlk_xof_squeezeblocks
#undef mlk_xof_x4_absorb
#undef mlk_xof_x4_ctx
#undef mlk_xof_x4_init
#undef mlk_xof_x4_release
#undef mlk_xof_x4_squeezeblocks
/* mlkem/src/sys.h */
#undef MLK_ALIGN
#undef MLK_ALIGN_UP
#undef MLK_ALWAYS_INLINE
#undef MLK_CET_ENDBR
#undef MLK_CT_TESTING_DECLASSIFY
#undef MLK_CT_TESTING_SECRET
#undef MLK_DEFAULT_ALIGN
#undef MLK_HAVE_INLINE_ASM
#undef MLK_INLINE
#undef MLK_MUST_CHECK_RETURN_VALUE
#undef MLK_RESTRICT
#undef MLK_STATIC_TESTABLE
#undef MLK_SYS_AARCH64
#undef MLK_SYS_AARCH64_EB
#undef MLK_SYS_APPLE
#undef MLK_SYS_ARMV81M_MVE
#undef MLK_SYS_BIG_ENDIAN
#undef MLK_SYS_H
#undef MLK_SYS_LINUX
#undef MLK_SYS_LITTLE_ENDIAN
#undef MLK_SYS_PPC64LE
#undef MLK_SYS_RISCV32
#undef MLK_SYS_RISCV64
#undef MLK_SYS_RISCV64_RVV
#undef MLK_SYS_WINDOWS
#undef MLK_SYS_X86_64
#undef MLK_SYS_X86_64_AVX2
/* mlkem/src/verify.h */
#undef MLK_USE_ASM_VALUE_BARRIER
#undef MLK_VERIFY_H
#undef mlk_ct_opt_blocker_u64
/* mlkem/src/cbmc.h */
#undef MLK_CBMC_H
#undef __contract__
#undef __loop__


#if defined(MLK_CONFIG_USE_NATIVE_BACKEND_ARITH)
/* mlkem/src/native/api.h */
#undef MLK_INVNTT_BOUND
#undef MLK_NATIVE_API_H
#undef MLK_NATIVE_FUNC_FALLBACK
#undef MLK_NATIVE_FUNC_SUCCESS
#undef MLK_NTT_BOUND
/* mlkem/src/native/meta.h */
#undef MLK_NATIVE_META_H
#if defined(MLK_SYS_AARCH64)
/*
 * Undefine macros from native code (Arith, AArch64)
 */
/* mlkem/src/native/aarch64/meta.h */
#undef MLK_ARITH_BACKEND_AARCH64
#undef MLK_NATIVE_AARCH64_META_H
#undef MLK_USE_NATIVE_INTT
#undef MLK_USE_NATIVE_NTT
#undef MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED
#undef MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE
#undef MLK_USE_NATIVE_POLY_REDUCE
#undef MLK_USE_NATIVE_POLY_TOBYTES
#undef MLK_USE_NATIVE_POLY_TOMONT
#undef MLK_USE_NATIVE_REJ_UNIFORM
/* mlkem/src/native/aarch64/src/arith_native_aarch64.h */
#undef MLK_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H
#undef mlk_aarch64_invntt_zetas_layer12345
#undef mlk_aarch64_invntt_zetas_layer67
#undef mlk_aarch64_ntt_zetas_layer12345
#undef mlk_aarch64_ntt_zetas_layer67
#undef mlk_aarch64_zetas_mulcache_native
#undef mlk_aarch64_zetas_mulcache_twisted_native
#undef mlk_intt_asm
#undef mlk_ntt_asm
#undef mlk_poly_mulcache_compute_asm
#undef mlk_poly_reduce_asm
#undef mlk_poly_tobytes_asm
#undef mlk_poly_tomont_asm
#undef mlk_polyvec_basemul_acc_montgomery_cached_asm_k2
#undef mlk_polyvec_basemul_acc_montgomery_cached_asm_k3
#undef mlk_polyvec_basemul_acc_montgomery_cached_asm_k4
#undef mlk_rej_uniform_asm
#undef mlk_rej_uniform_table
#endif /* MLK_SYS_AARCH64 */
#if defined(MLK_SYS_X86_64)
/*
 * Undefine macros from native code (Arith, X86_64)
 */
/* mlkem/src/native/x86_64/meta.h */
#undef MLK_ARITH_BACKEND_X86_64_DEFAULT
#undef MLK_NATIVE_X86_64_META_H
#undef MLK_USE_NATIVE_INTT
#undef MLK_USE_NATIVE_NTT
#undef MLK_USE_NATIVE_NTT_CUSTOM_ORDER
#undef MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED
#undef MLK_USE_NATIVE_POLY_COMPRESS_D10
#undef MLK_USE_NATIVE_POLY_COMPRESS_D11
#undef MLK_USE_NATIVE_POLY_COMPRESS_D4
#undef MLK_USE_NATIVE_POLY_COMPRESS_D5
#undef MLK_USE_NATIVE_POLY_DECOMPRESS_D10
#undef MLK_USE_NATIVE_POLY_DECOMPRESS_D11
#undef MLK_USE_NATIVE_POLY_DECOMPRESS_D4
#undef MLK_USE_NATIVE_POLY_DECOMPRESS_D5
#undef MLK_USE_NATIVE_POLY_FROMBYTES
#undef MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE
#undef MLK_USE_NATIVE_POLY_REDUCE
#undef MLK_USE_NATIVE_POLY_TOBYTES
#undef MLK_USE_NATIVE_POLY_TOMONT
#undef MLK_USE_NATIVE_REJ_UNIFORM
/* mlkem/src/native/x86_64/src/arith_native_x86_64.h */
#undef MLK_AVX2_REJ_UNIFORM_BUFLEN
#undef MLK_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H
#undef mlk_invntt_avx2
#undef mlk_ntt_avx2
#undef mlk_nttfrombytes_avx2
#undef mlk_ntttobytes_avx2
#undef mlk_nttunpack_avx2
#undef mlk_poly_compress_d10_avx2
#undef mlk_poly_compress_d11_avx2
#undef mlk_poly_compress_d4_avx2
#undef mlk_poly_compress_d5_avx2
#undef mlk_poly_decompress_d10_avx2
#undef mlk_poly_decompress_d11_avx2
#undef mlk_poly_decompress_d4_avx2
#undef mlk_poly_decompress_d5_avx2
#undef mlk_poly_mulcache_compute_avx2
#undef mlk_polyvec_basemul_acc_montgomery_cached_asm_k2
#undef mlk_polyvec_basemul_acc_montgomery_cached_asm_k3
#undef mlk_polyvec_basemul_acc_montgomery_cached_asm_k4
#undef mlk_reduce_avx2
#undef mlk_rej_uniform_asm
#undef mlk_rej_uniform_table
#undef mlk_tomont_avx2
/* mlkem/src/native/x86_64/src/compress_consts.h */
#undef MLK_NATIVE_X86_64_SRC_COMPRESS_CONSTS_H
#undef mlk_compress_d10_data
#undef mlk_compress_d11_data
#undef mlk_compress_d4_data
#undef mlk_compress_d5_data
#undef mlk_decompress_d10_data
#undef mlk_decompress_d11_data
#undef mlk_decompress_d4_data
#undef mlk_decompress_d5_data
/* mlkem/src/native/x86_64/src/consts.h */
#undef MLK_AVX2_BACKEND_DATA_OFFSET_MULCACHE_TWIDDLES
#undef MLK_AVX2_BACKEND_DATA_OFFSET_REVIDXB
#undef MLK_AVX2_BACKEND_DATA_OFFSET_REVIDXD
#undef MLK_AVX2_BACKEND_DATA_OFFSET_ZETAS_EXP
#undef MLK_NATIVE_X86_64_SRC_CONSTS_H
#undef mlk_qdata
#endif /* MLK_SYS_X86_64 */
#endif /* MLK_CONFIG_USE_NATIVE_BACKEND_ARITH */
#endif /* !MLK_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS */
