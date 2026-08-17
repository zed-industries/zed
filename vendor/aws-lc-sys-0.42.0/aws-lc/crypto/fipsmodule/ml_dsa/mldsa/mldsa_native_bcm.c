/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*
 * WARNING: This file is auto-generated from scripts/autogen
 *          in the mldsa-native repository.
 *          Do not modify it directly.
 */

/******************************************************************************
 *
 * Single compilation unit (SCU) for fixed-level build of mldsa-native
 *
 * This compilation unit bundles together all source files for a build
 * of mldsa-native for a fixed security level (MLDSA-44/65/87).
 *
 * # API
 *
 * The API exposed by this file is described in mldsa_native.h.
 *
 * # Multi-level build
 *
 * If you want an SCU build of mldsa-native with support for multiple security
 * levels, you need to include this file multiple times, and set
 * MLD_CONFIG_MULTILEVEL_WITH_SHARED and MLD_CONFIG_MULTILEVEL_NO_SHARED
 * appropriately. This is exemplified in examples/monolithic_build_multilevel
 * and examples/monolithic_build_multilevel_native.
 *
 * # Configuration
 *
 * The following options from the mldsa-native configuration are relevant:
 *
 * - MLD_CONFIG_FIPS202_CUSTOM_HEADER
 *   Set this option if you use a custom FIPS202 implementation.
 *
 * - MLD_CONFIG_USE_NATIVE_BACKEND_ARITH
 *   Set this option if you want to include the native arithmetic backends
 *   in your build.
 *
 * - MLD_CONFIG_USE_NATIVE_BACKEND_FIPS202
 *   Set this option if you want to include the native FIPS202 backends
 *   in your build.
 *
 * - MLD_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS
 *   Set this option if you want to keep the directives defined in
 *   level-independent headers. This is needed for a multi-level build.
 */

/* If parts of the mldsa-native source tree are not used,
 * consider reducing this header via `unifdef`.
 *
 * Example:
 * ```bash
 * unifdef -UMLD_CONFIG_USE_NATIVE_BACKEND_ARITH mldsa_native.c
 * ```
 */

#include "common.h"

#include "ct.c"
#include "debug.c"
#include "packing.c"
#include "poly.c"
#include "poly_kl.c"
#include "polyvec.c"
#include "polyvec_lazy.c"
#include "sign.c"


#if defined(MLD_CONFIG_USE_NATIVE_BACKEND_ARITH)
#if defined(MLD_SYS_AARCH64)
#include "native/aarch64/src/aarch64_zetas.c"
#include "native/aarch64/src/polyz_unpack_table.c"
#include "native/aarch64/src/rej_uniform_eta_table.c"
#include "native/aarch64/src/rej_uniform_table.c"
#endif /* MLD_SYS_AARCH64 */
#if defined(MLD_SYS_X86_64)
#include "native/x86_64/src/consts.c"
#endif /* MLD_SYS_X86_64 */
#endif /* MLD_CONFIG_USE_NATIVE_BACKEND_ARITH */


/* Macro #undef's
 *
 * The following undefines macros from headers
 * included by the source files imported above.
 *
 * This is to allow building and linking multiple builds
 * of mldsa-native for varying parameter sets through concatenation
 * of this file, as if the files had been compiled separately.
 * If this is not relevant to you, you may remove the following.
 */

/*
 * Undefine macros from MLD_CONFIG_PARAMETER_SET-specific files
 */
/* mldsa/mldsa_native.h */
#undef CRYPTO_BYTES
#undef CRYPTO_PUBLICKEYBYTES
#undef CRYPTO_SECRETKEYBYTES
#undef MLDSA44_BYTES
#undef MLDSA44_CRHBYTES
#undef MLDSA44_PUBLICKEYBYTES
#undef MLDSA44_RNDBYTES
#undef MLDSA44_SECRETKEYBYTES
#undef MLDSA44_SEEDBYTES
#undef MLDSA44_TRBYTES
#undef MLDSA65_BYTES
#undef MLDSA65_CRHBYTES
#undef MLDSA65_PUBLICKEYBYTES
#undef MLDSA65_RNDBYTES
#undef MLDSA65_SECRETKEYBYTES
#undef MLDSA65_SEEDBYTES
#undef MLDSA65_TRBYTES
#undef MLDSA87_BYTES
#undef MLDSA87_CRHBYTES
#undef MLDSA87_PUBLICKEYBYTES
#undef MLDSA87_RNDBYTES
#undef MLDSA87_SECRETKEYBYTES
#undef MLDSA87_SEEDBYTES
#undef MLDSA87_TRBYTES
#undef MLDSA_BYTES
#undef MLDSA_BYTES_
#undef MLDSA_CRHBYTES
#undef MLDSA_PUBLICKEYBYTES
#undef MLDSA_PUBLICKEYBYTES_
#undef MLDSA_RNDBYTES
#undef MLDSA_SECRETKEYBYTES
#undef MLDSA_SECRETKEYBYTES_
#undef MLDSA_SEEDBYTES
#undef MLDSA_TRBYTES
#undef MLD_API_CONCAT
#undef MLD_API_CONCAT_
#undef MLD_API_CONCAT_UNDERSCORE
#undef MLD_API_LEGACY_CONFIG
#undef MLD_API_MUST_CHECK_RETURN_VALUE
#undef MLD_API_NAMESPACE
#undef MLD_API_QUALIFIER
#undef MLD_CONFIG_API_CONSTANTS_ONLY
#undef MLD_CONFIG_API_NAMESPACE_PREFIX
#undef MLD_CONFIG_API_NO_SUPERCOP
#undef MLD_CONFIG_API_PARAMETER_SET
#undef MLD_CONFIG_API_QUALIFIER
#undef MLD_DOMAIN_SEPARATION_MAX_BYTES
#undef MLD_ERR_FAIL
#undef MLD_ERR_OUT_OF_MEMORY
#undef MLD_ERR_RNG_FAIL
#undef MLD_ERR_SIGN_ATTEMPTS_EXHAUSTED
#undef MLD_H
#undef MLD_MAX3_
#undef MLD_MAX4_
#undef MLD_PREHASH_NONE
#undef MLD_PREHASH_SHA2_224
#undef MLD_PREHASH_SHA2_256
#undef MLD_PREHASH_SHA2_384
#undef MLD_PREHASH_SHA2_512
#undef MLD_PREHASH_SHA2_512_224
#undef MLD_PREHASH_SHA2_512_256
#undef MLD_PREHASH_SHA3_224
#undef MLD_PREHASH_SHA3_256
#undef MLD_PREHASH_SHA3_384
#undef MLD_PREHASH_SHA3_512
#undef MLD_PREHASH_SHAKE_128
#undef MLD_PREHASH_SHAKE_256
#undef MLD_TOTAL_ALLOC_44
#undef MLD_TOTAL_ALLOC_44_KEYPAIR
#undef MLD_TOTAL_ALLOC_44_KEYPAIR_NO_PCT
#undef MLD_TOTAL_ALLOC_44_KEYPAIR_PCT
#undef MLD_TOTAL_ALLOC_44_PK_FROM_SK
#undef MLD_TOTAL_ALLOC_44_SIGN
#undef MLD_TOTAL_ALLOC_44_VERIFY
#undef MLD_TOTAL_ALLOC_65
#undef MLD_TOTAL_ALLOC_65_KEYPAIR
#undef MLD_TOTAL_ALLOC_65_KEYPAIR_NO_PCT
#undef MLD_TOTAL_ALLOC_65_KEYPAIR_PCT
#undef MLD_TOTAL_ALLOC_65_PK_FROM_SK
#undef MLD_TOTAL_ALLOC_65_SIGN
#undef MLD_TOTAL_ALLOC_65_VERIFY
#undef MLD_TOTAL_ALLOC_87
#undef MLD_TOTAL_ALLOC_87_KEYPAIR
#undef MLD_TOTAL_ALLOC_87_KEYPAIR_NO_PCT
#undef MLD_TOTAL_ALLOC_87_KEYPAIR_PCT
#undef MLD_TOTAL_ALLOC_87_PK_FROM_SK
#undef MLD_TOTAL_ALLOC_87_SIGN
#undef MLD_TOTAL_ALLOC_87_VERIFY
#undef crypto_sign
#undef crypto_sign_keypair
#undef crypto_sign_open
#undef crypto_sign_signature
#undef crypto_sign_verify
/* mldsa/src/common.h */
#undef MLD_ADD_PARAM_SET
#undef MLD_ALLOC
#undef MLD_ANY_ERROR
#undef MLD_APPLY
#undef MLD_ASM_FN_SIZE
#undef MLD_ASM_FN_SYMBOL
#undef MLD_ASM_NAMESPACE
#undef MLD_BUILD_INTERNAL
#undef MLD_COMMON_H
#undef MLD_CONCAT
#undef MLD_CONCAT_
#undef MLD_CONTEXT_PARAMETERS_0
#undef MLD_CONTEXT_PARAMETERS_1
#undef MLD_CONTEXT_PARAMETERS_2
#undef MLD_CONTEXT_PARAMETERS_3
#undef MLD_CONTEXT_PARAMETERS_4
#undef MLD_CONTEXT_PARAMETERS_5
#undef MLD_CONTEXT_PARAMETERS_6
#undef MLD_CONTEXT_PARAMETERS_7
#undef MLD_CONTEXT_PARAMETERS_8
#undef MLD_CONTEXT_PARAMETERS_9
#undef MLD_EMPTY_CU
#undef MLD_ERR_FAIL
#undef MLD_ERR_OUT_OF_MEMORY
#undef MLD_ERR_RNG_FAIL
#undef MLD_ERR_SIGN_ATTEMPTS_EXHAUSTED
#undef MLD_EXTERNAL_API
#undef MLD_FIPS202X4_HEADER_FILE
#undef MLD_FIPS202_HEADER_FILE
#undef MLD_FREE
#undef MLD_INTERNAL_API
#undef MLD_INTERNAL_DATA_DECLARATION
#undef MLD_INTERNAL_DATA_DEFINITION
#undef MLD_MULTILEVEL_BUILD
#undef MLD_NAMESPACE
#undef MLD_NAMESPACE_KL
#undef MLD_NAMESPACE_PREFIX
#undef MLD_NAMESPACE_PREFIX_KL
#undef mld_memcpy
#undef mld_memset
/* mldsa/src/packing.h */
#undef MLD_PACKING_H
#undef mld_pack_sig_c
#undef mld_pack_sig_h
#undef mld_pack_sig_z
#undef mld_pack_sk_rho_key_tr_s2
#undef mld_pack_sk_s1
#undef mld_sig_unpack_hints
#undef mld_unpack_pk_t1
#undef mld_unpack_sk
/* mldsa/src/params.h */
#undef MLDSA_BETA
#undef MLDSA_CRHBYTES
#undef MLDSA_CRYPTO_BYTES
#undef MLDSA_CRYPTO_PUBLICKEYBYTES
#undef MLDSA_CRYPTO_SECRETKEYBYTES
#undef MLDSA_CTILDEBYTES
#undef MLDSA_D
#undef MLDSA_ETA
#undef MLDSA_GAMMA1
#undef MLDSA_GAMMA2
#undef MLDSA_GAMMA2_32
#undef MLDSA_GAMMA2_88
#undef MLDSA_K
#undef MLDSA_L
#undef MLDSA_N
#undef MLDSA_OMEGA
#undef MLDSA_PK_END
#undef MLDSA_PK_RHO_BYTES
#undef MLDSA_PK_RHO_OFFSET
#undef MLDSA_PK_T1_BYTES
#undef MLDSA_PK_T1_OFFSET
#undef MLDSA_POLYETA_PACKEDBYTES
#undef MLDSA_POLYT0_PACKEDBYTES
#undef MLDSA_POLYT1_PACKEDBYTES
#undef MLDSA_POLYVECH_PACKEDBYTES
#undef MLDSA_POLYW1_PACKEDBYTES
#undef MLDSA_POLYW1_PACKEDBYTES_32
#undef MLDSA_POLYW1_PACKEDBYTES_88
#undef MLDSA_POLYZ_PACKEDBYTES
#undef MLDSA_Q
#undef MLDSA_Q_HALF
#undef MLDSA_RNDBYTES
#undef MLDSA_SEEDBYTES
#undef MLDSA_SIG_C_BYTES
#undef MLDSA_SIG_C_OFFSET
#undef MLDSA_SIG_END
#undef MLDSA_SIG_H_BYTES
#undef MLDSA_SIG_H_OFFSET
#undef MLDSA_SIG_Z_BYTES
#undef MLDSA_SIG_Z_OFFSET
#undef MLDSA_SK_END
#undef MLDSA_SK_KEY_BYTES
#undef MLDSA_SK_KEY_OFFSET
#undef MLDSA_SK_RHO_BYTES
#undef MLDSA_SK_RHO_OFFSET
#undef MLDSA_SK_S1_BYTES
#undef MLDSA_SK_S1_OFFSET
#undef MLDSA_SK_S2_BYTES
#undef MLDSA_SK_S2_OFFSET
#undef MLDSA_SK_T0_BYTES
#undef MLDSA_SK_T0_OFFSET
#undef MLDSA_SK_TR_BYTES
#undef MLDSA_SK_TR_OFFSET
#undef MLDSA_TAU
#undef MLDSA_TRBYTES
#undef MLD_PARAMS_H
/* mldsa/src/poly_kl.h */
#undef MLD_POLYETA_UNPACK_LOWER_BOUND
#undef MLD_POLY_KL_H
#undef mld_poly_challenge
#undef mld_poly_decompose
#undef mld_poly_uniform_eta
#undef mld_poly_uniform_eta_4x
#undef mld_poly_uniform_gamma1
#undef mld_poly_uniform_gamma1_4x
#undef mld_poly_use_hint
#undef mld_polyeta_pack
#undef mld_polyeta_unpack
#undef mld_polyw1_pack
#undef mld_polyz_pack
#undef mld_polyz_unpack
/* mldsa/src/polyvec.h */
#undef MLD_POLYVEC_H
#undef mld_polyveck
#undef mld_polyveck_caddq
#undef mld_polyveck_chknorm
#undef mld_polyveck_decompose
#undef mld_polyveck_invntt_tomont
#undef mld_polyveck_ntt
#undef mld_polyveck_pack_eta
#undef mld_polyveck_pack_w1
#undef mld_polyveck_reduce
#undef mld_polyveck_unpack_eta
#undef mld_polyvecl
#undef mld_polyvecl_chknorm
#undef mld_polyvecl_ntt
#undef mld_polyvecl_pack_eta
#undef mld_polyvecl_pointwise_acc_montgomery
#undef mld_polyvecl_uniform_gamma1
#undef mld_polyvecl_unpack_eta
#undef mld_polyvecl_unpack_z
/* mldsa/src/polyvec_lazy.h */
#undef MLD_POLYVEC_LAZY_H
#undef mld_poly_permute_bitrev_to_custom_optional
#undef mld_polymat
#undef mld_polymat_eager
#undef mld_polymat_lazy
#undef mld_polyvec_matrix_expand
#undef mld_polyvec_matrix_expand_eager
#undef mld_polyvec_matrix_expand_lazy
#undef mld_polyvec_matrix_pointwise_montgomery
#undef mld_polyvec_matrix_pointwise_montgomery_row
#undef mld_polyvec_matrix_pointwise_montgomery_row_eager
#undef mld_polyvec_matrix_pointwise_montgomery_row_lazy
#undef mld_polyvec_matrix_pointwise_montgomery_yvec
#undef mld_polyvec_matrix_pointwise_montgomery_yvec_eager
#undef mld_polyvec_matrix_pointwise_montgomery_yvec_lazy
#undef mld_sk_s1hat
#undef mld_sk_s1hat_eager
#undef mld_sk_s1hat_get_poly
#undef mld_sk_s1hat_get_poly_eager
#undef mld_sk_s1hat_get_poly_lazy
#undef mld_sk_s1hat_lazy
#undef mld_sk_s2hat
#undef mld_sk_s2hat_eager
#undef mld_sk_s2hat_get_poly
#undef mld_sk_s2hat_get_poly_eager
#undef mld_sk_s2hat_get_poly_lazy
#undef mld_sk_s2hat_lazy
#undef mld_sk_t0hat
#undef mld_sk_t0hat_eager
#undef mld_sk_t0hat_get_poly
#undef mld_sk_t0hat_get_poly_eager
#undef mld_sk_t0hat_get_poly_lazy
#undef mld_sk_t0hat_lazy
#undef mld_unpack_sk_s1hat
#undef mld_unpack_sk_s1hat_eager
#undef mld_unpack_sk_s1hat_lazy
#undef mld_unpack_sk_s2hat
#undef mld_unpack_sk_s2hat_eager
#undef mld_unpack_sk_s2hat_lazy
#undef mld_unpack_sk_t0hat
#undef mld_unpack_sk_t0hat_eager
#undef mld_unpack_sk_t0hat_lazy
#undef mld_yvec
#undef mld_yvec_eager
#undef mld_yvec_get_poly
#undef mld_yvec_get_poly_eager
#undef mld_yvec_get_poly_lazy
#undef mld_yvec_init
#undef mld_yvec_init_eager
#undef mld_yvec_init_lazy
#undef mld_yvec_lazy
/* mldsa/src/rounding.h */
#undef MLD_2_POW_D
#undef MLD_ROUNDING_H
#undef mld_decompose
#undef mld_make_hint
#undef mld_power2round
#undef mld_use_hint
/* mldsa/src/sign.h */
#undef MLD_DOMAIN_SEPARATION_MAX_BYTES
#undef MLD_PREHASH_NONE
#undef MLD_PREHASH_SHA2_224
#undef MLD_PREHASH_SHA2_256
#undef MLD_PREHASH_SHA2_384
#undef MLD_PREHASH_SHA2_512
#undef MLD_PREHASH_SHA2_512_224
#undef MLD_PREHASH_SHA2_512_256
#undef MLD_PREHASH_SHA3_224
#undef MLD_PREHASH_SHA3_256
#undef MLD_PREHASH_SHA3_384
#undef MLD_PREHASH_SHA3_512
#undef MLD_PREHASH_SHAKE_128
#undef MLD_PREHASH_SHAKE_256
#undef MLD_SIGN_H
#undef mld_prepare_domain_separation_prefix
#undef mld_sign
#undef mld_sign_keypair
#undef mld_sign_keypair_internal
#undef mld_sign_open
#undef mld_sign_pk_from_sk
#undef mld_sign_signature
#undef mld_sign_signature_extmu
#undef mld_sign_signature_internal
#undef mld_sign_signature_pre_hash_internal
#undef mld_sign_signature_pre_hash_shake256
#undef mld_sign_verify
#undef mld_sign_verify_extmu
#undef mld_sign_verify_internal
#undef mld_sign_verify_pre_hash_internal
#undef mld_sign_verify_pre_hash_shake256

#if !defined(MLD_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS)
/*
 * Undefine macros from MLD_CONFIG_PARAMETER_SET-generic files
 */
/* mldsa/src/ct.h */
#undef MLD_CT_H
#undef MLD_USE_ASM_VALUE_BARRIER
#undef mld_ct_opt_blocker_u64
/* mldsa/src/debug.h */
#undef MLD_DEBUG_H
#undef mld_assert
#undef mld_assert_abs_bound
#undef mld_assert_abs_bound_2d
#undef mld_assert_bound
#undef mld_assert_bound_2d
#undef mld_debug_check_assert
#undef mld_debug_check_bounds
/* mldsa/src/poly.h */
#undef MLD_FQMUL_BOUND
#undef MLD_INTT_BOUND
#undef MLD_NTT_BOUND
#undef MLD_POLY_H
#undef mld_poly_add
#undef mld_poly_caddq
#undef mld_poly_chknorm
#undef mld_poly_invntt_tomont
#undef mld_poly_ntt
#undef mld_poly_pointwise_montgomery
#undef mld_poly_power2round
#undef mld_poly_reduce
#undef mld_poly_shiftl
#undef mld_poly_sub
#undef mld_poly_uniform
#undef mld_poly_uniform_4x
#undef mld_polyt0_pack
#undef mld_polyt0_unpack
#undef mld_polyt1_pack
#undef mld_polyt1_unpack
#undef mld_polyw1_pack_32
#undef mld_polyw1_pack_88
/* mldsa/src/randombytes.h */
#undef MLD_RANDOMBYTES_H
/* mldsa/src/reduce.h */
#undef MLD_MONT
#undef MLD_REDUCE32_DOMAIN_MAX
#undef MLD_REDUCE32_RANGE_MAX
#undef MLD_REDUCE_H
/* mldsa/src/symmetric.h */
#undef MLD_STREAM128_BLOCKBYTES
#undef MLD_STREAM256_BLOCKBYTES
#undef MLD_SYMMETRIC_H
#undef mld_xof128_absorb_once
#undef mld_xof128_ctx
#undef mld_xof128_init
#undef mld_xof128_release
#undef mld_xof128_squeezeblocks
#undef mld_xof128_x4_absorb
#undef mld_xof128_x4_ctx
#undef mld_xof128_x4_init
#undef mld_xof128_x4_release
#undef mld_xof128_x4_squeezeblocks
#undef mld_xof256_absorb_once
#undef mld_xof256_ctx
#undef mld_xof256_init
#undef mld_xof256_release
#undef mld_xof256_squeezeblocks
#undef mld_xof256_x4_absorb
#undef mld_xof256_x4_ctx
#undef mld_xof256_x4_init
#undef mld_xof256_x4_release
#undef mld_xof256_x4_squeezeblocks
/* mldsa/src/sys.h */
#undef MLD_ALIGN
#undef MLD_ALIGN_UP
#undef MLD_ALWAYS_INLINE
#undef MLD_CET_ENDBR
#undef MLD_CT_TESTING_DECLASSIFY
#undef MLD_CT_TESTING_SECRET
#undef MLD_DEFAULT_ALIGN
#undef MLD_HAVE_INLINE_ASM
#undef MLD_INLINE
#undef MLD_MUST_CHECK_RETURN_VALUE
#undef MLD_RESTRICT
#undef MLD_STATIC_TESTABLE
#undef MLD_SYS_AARCH64
#undef MLD_SYS_AARCH64_EB
#undef MLD_SYS_APPLE
#undef MLD_SYS_ARMV81M_MVE
#undef MLD_SYS_BIG_ENDIAN
#undef MLD_SYS_H
#undef MLD_SYS_LINUX
#undef MLD_SYS_LITTLE_ENDIAN
#undef MLD_SYS_PPC64LE
#undef MLD_SYS_RISCV32
#undef MLD_SYS_RISCV64
#undef MLD_SYS_RISCV64_RVV
#undef MLD_SYS_WINDOWS
#undef MLD_SYS_X86_64
#undef MLD_SYS_X86_64_AVX2
/* mldsa/src/cbmc.h */
#undef MLD_CBMC_H
#undef __contract__
#undef __loop__


#if defined(MLD_CONFIG_USE_NATIVE_BACKEND_ARITH)
/* mldsa/src/native/api.h */
#undef MLD_FQMUL_BOUND
#undef MLD_INTT_BOUND
#undef MLD_NATIVE_API_H
#undef MLD_NATIVE_FUNC_FALLBACK
#undef MLD_NATIVE_FUNC_SUCCESS
#undef MLD_NTT_BOUND
#undef MLD_REDUCE32_RANGE_MAX
/* mldsa/src/native/meta.h */
#undef MLD_NATIVE_META_H
#if defined(MLD_SYS_AARCH64)
/*
 * Undefine macros from native code (Arith, AArch64)
 */
/* mldsa/src/native/aarch64/meta.h */
#undef MLD_ARITH_BACKEND_AARCH64
#undef MLD_NATIVE_AARCH64_META_H
#undef MLD_USE_NATIVE_INTT
#undef MLD_USE_NATIVE_NTT
#undef MLD_USE_NATIVE_POINTWISE_MONTGOMERY
#undef MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L4
#undef MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L5
#undef MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L7
#undef MLD_USE_NATIVE_POLYZ_UNPACK_17
#undef MLD_USE_NATIVE_POLYZ_UNPACK_19
#undef MLD_USE_NATIVE_POLY_CADDQ
#undef MLD_USE_NATIVE_POLY_CHKNORM
#undef MLD_USE_NATIVE_POLY_DECOMPOSE_32
#undef MLD_USE_NATIVE_POLY_DECOMPOSE_88
#undef MLD_USE_NATIVE_POLY_USE_HINT_32
#undef MLD_USE_NATIVE_POLY_USE_HINT_88
#undef MLD_USE_NATIVE_REJ_UNIFORM
#undef MLD_USE_NATIVE_REJ_UNIFORM_ETA2
#undef MLD_USE_NATIVE_REJ_UNIFORM_ETA4
/* mldsa/src/native/aarch64/src/arith_native_aarch64.h */
#undef MLD_AARCH64_REJ_UNIFORM_ETA2_BUFLEN
#undef MLD_AARCH64_REJ_UNIFORM_ETA4_BUFLEN
#undef MLD_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H
#undef mld_aarch64_intt_zetas_layer123456
#undef mld_aarch64_intt_zetas_layer78
#undef mld_aarch64_ntt_zetas_layer123456
#undef mld_aarch64_ntt_zetas_layer78
#undef mld_intt_aarch64_asm
#undef mld_ntt_aarch64_asm
#undef mld_poly_caddq_aarch64_asm
#undef mld_poly_chknorm_aarch64_asm
#undef mld_poly_decompose_32_aarch64_asm
#undef mld_poly_decompose_88_aarch64_asm
#undef mld_poly_pointwise_montgomery_aarch64_asm
#undef mld_poly_use_hint_32_aarch64_asm
#undef mld_poly_use_hint_88_aarch64_asm
#undef mld_polyvecl_pointwise_acc_montgomery_l4_aarch64_asm
#undef mld_polyvecl_pointwise_acc_montgomery_l5_aarch64_asm
#undef mld_polyvecl_pointwise_acc_montgomery_l7_aarch64_asm
#undef mld_polyz_unpack_17_aarch64_asm
#undef mld_polyz_unpack_17_indices
#undef mld_polyz_unpack_19_aarch64_asm
#undef mld_polyz_unpack_19_indices
#undef mld_rej_uniform_aarch64_asm
#undef mld_rej_uniform_eta2_aarch64_asm
#undef mld_rej_uniform_eta4_aarch64_asm
#undef mld_rej_uniform_eta_table
#undef mld_rej_uniform_table
#endif /* MLD_SYS_AARCH64 */
#if defined(MLD_SYS_X86_64)
/*
 * Undefine macros from native code (Arith, X86_64)
 */
/* mldsa/src/native/x86_64/meta.h */
#undef MLD_ARITH_BACKEND_X86_64_DEFAULT
#undef MLD_NATIVE_X86_64_META_H
#undef MLD_USE_NATIVE_INTT
#undef MLD_USE_NATIVE_NTT
#undef MLD_USE_NATIVE_NTT_CUSTOM_ORDER
#undef MLD_USE_NATIVE_POINTWISE_MONTGOMERY
#undef MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L4
#undef MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L5
#undef MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L7
#undef MLD_USE_NATIVE_POLYZ_UNPACK_17
#undef MLD_USE_NATIVE_POLYZ_UNPACK_19
#undef MLD_USE_NATIVE_POLY_CADDQ
#undef MLD_USE_NATIVE_POLY_CHKNORM
#undef MLD_USE_NATIVE_POLY_DECOMPOSE_32
#undef MLD_USE_NATIVE_POLY_DECOMPOSE_88
#undef MLD_USE_NATIVE_POLY_USE_HINT_32
#undef MLD_USE_NATIVE_POLY_USE_HINT_88
#undef MLD_USE_NATIVE_REJ_UNIFORM
#undef MLD_USE_NATIVE_REJ_UNIFORM_ETA2
#undef MLD_USE_NATIVE_REJ_UNIFORM_ETA4
/* mldsa/src/native/x86_64/src/arith_native_x86_64.h */
#undef MLD_AVX2_REJ_UNIFORM_BUFLEN
#undef MLD_AVX2_REJ_UNIFORM_ETA2_BUFLEN
#undef MLD_AVX2_REJ_UNIFORM_ETA4_BUFLEN
#undef MLD_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H
#undef mld_invntt_avx2_asm
#undef mld_ntt_avx2_asm
#undef mld_nttunpack_avx2_asm
#undef mld_pointwise_acc_l4_avx2_asm
#undef mld_pointwise_acc_l5_avx2_asm
#undef mld_pointwise_acc_l7_avx2_asm
#undef mld_pointwise_avx2_asm
#undef mld_poly_caddq_avx2_asm
#undef mld_poly_chknorm_avx2
#undef mld_poly_decompose_32_avx2
#undef mld_poly_decompose_88_avx2
#undef mld_poly_use_hint_32_avx2
#undef mld_poly_use_hint_88_avx2
#undef mld_polyz_unpack_17_avx2
#undef mld_polyz_unpack_19_avx2
#undef mld_rej_uniform_avx2
#undef mld_rej_uniform_eta2_avx2
#undef mld_rej_uniform_eta4_avx2
#undef mld_rej_uniform_table
/* mldsa/src/native/x86_64/src/consts.h */
#undef MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV
#undef MLD_AVX2_BACKEND_DATA_OFFSET_8XDIV_QINV
#undef MLD_AVX2_BACKEND_DATA_OFFSET_8XQ
#undef MLD_AVX2_BACKEND_DATA_OFFSET_8XQINV
#undef MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS
#undef MLD_AVX2_BACKEND_DATA_OFFSET_ZETAS_QINV
#undef MLD_NATIVE_X86_64_SRC_CONSTS_H
#undef mld_qdata
#endif /* MLD_SYS_X86_64 */
#endif /* MLD_CONFIG_USE_NATIVE_BACKEND_ARITH */
#endif /* !MLD_CONFIG_MONOBUILD_KEEP_SHARED_HEADERS */
