/*
 * Copyright (c) The mldsa-native project authors
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_COMMON_H
#define MLD_COMMON_H

#ifndef __ASSEMBLER__
#include <stdint.h>
#endif


#define MLD_BUILD_INTERNAL

#if defined(MLD_CONFIG_FILE)
#include MLD_CONFIG_FILE
#else
#include "mldsa_native_config.h"
#endif

#include "params.h"
#include "sys.h"

/* Internal and public API have external linkage by default, but
 * this can be overwritten by the user, e.g. for single-CU builds. */
#if !defined(MLD_CONFIG_INTERNAL_API_QUALIFIER)
#define MLD_INTERNAL_API
#define MLD_INTERNAL_DATA_DECLARATION extern
#define MLD_INTERNAL_DATA_DEFINITION
#else
#define MLD_INTERNAL_API MLD_CONFIG_INTERNAL_API_QUALIFIER
#define MLD_INTERNAL_DATA_DECLARATION MLD_CONFIG_INTERNAL_API_QUALIFIER
#define MLD_INTERNAL_DATA_DEFINITION MLD_CONFIG_INTERNAL_API_QUALIFIER
#endif

#if !defined(MLD_CONFIG_EXTERNAL_API_QUALIFIER)
#define MLD_EXTERNAL_API
#else
#define MLD_EXTERNAL_API MLD_CONFIG_EXTERNAL_API_QUALIFIER
#endif

#if defined(MLD_CONFIG_MULTILEVEL_NO_SHARED) || \
    defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED)
#define MLD_MULTILEVEL_BUILD
#endif

#define MLD_CONCAT_(x1, x2) x1##x2
#define MLD_CONCAT(x1, x2) MLD_CONCAT_(x1, x2)

#if defined(MLD_MULTILEVEL_BUILD)
#define MLD_ADD_PARAM_SET(s) MLD_CONCAT(s, MLD_CONFIG_PARAMETER_SET)
#else
#define MLD_ADD_PARAM_SET(s) s
#endif

#define MLD_NAMESPACE_PREFIX MLD_CONCAT(MLD_CONFIG_NAMESPACE_PREFIX, _)
#define MLD_NAMESPACE_PREFIX_KL \
  MLD_CONCAT(MLD_ADD_PARAM_SET(MLD_CONFIG_NAMESPACE_PREFIX), _)

/* Functions are prefixed by MLD_CONFIG_NAMESPACE_PREFIX.
 *
 * If multiple parameter sets are used, functions depending on the parameter
 * set are additionally prefixed with 44/65/87. See mldsa_native_config.h.
 *
 * Example: If MLD_CONFIG_NAMESPACE_PREFIX is PQCP_MLDSA_NATIVE, then
 * MLD_NAMESPACE_KL(keypair) becomes PQCP_MLDSA_NATIVE44_keypair/
 * PQCP_MLDSA_NATIVE65_keypair/PQCP_MLDSA_NATIVE87_keypair.
 */
#define MLD_NAMESPACE(s) MLD_CONCAT(MLD_NAMESPACE_PREFIX, s)
#define MLD_NAMESPACE_KL(s) MLD_CONCAT(MLD_NAMESPACE_PREFIX_KL, s)

/* On Apple platforms, we need to emit leading underscore
 * in front of assembly symbols. We thus introducee a separate
 * namespace wrapper for ASM symbols. */
#if !defined(__APPLE__)
#define MLD_ASM_NAMESPACE(sym) MLD_NAMESPACE(sym)
#else
#define MLD_ASM_NAMESPACE(sym) MLD_CONCAT(_, MLD_NAMESPACE(sym))
#endif

/*
 * On X86_64 if control-flow protections (CET) are enabled (through
 * -fcf-protection=), we add an endbr64 instruction at every global function
 * label.  See sys.h for more details
 */
#if defined(MLD_SYS_X86_64)
#define MLD_ASM_FN_SYMBOL(sym) MLD_ASM_NAMESPACE(sym) : MLD_CET_ENDBR
#elif defined(MLD_SYS_ARMV81M_MVE)
/* clang-format off */
#define MLD_ASM_FN_SYMBOL(sym) \
  .type MLD_ASM_NAMESPACE(sym), %function; \
  MLD_ASM_NAMESPACE(sym) :
/* clang-format on */
#else /* !MLD_SYS_X86_64 && MLD_SYS_ARMV81M_MVE */
#define MLD_ASM_FN_SYMBOL(sym) MLD_ASM_NAMESPACE(sym) :
#endif /* !MLD_SYS_X86_64 && !MLD_SYS_ARMV81M_MVE */

/*
 * Output the size of an assembly function.
 */
#if defined(__ELF__)
#define MLD_ASM_FN_SIZE(sym) \
  .size MLD_ASM_NAMESPACE(sym), .- MLD_ASM_NAMESPACE(sym)
#else
#define MLD_ASM_FN_SIZE(sym)
#endif

/* We aim to simplify the user's life by supporting builds where
 * all source files are included, even those that are not needed.
 * Those files are appropriately guarded and will be empty when unneeded.
 * The following is to avoid compilers complaining about this. */
#define MLD_EMPTY_CU(s) extern int MLD_NAMESPACE_KL(empty_cu_##s);

/* MLD_CONFIG_NO_ASM takes precedence over MLD_USE_NATIVE_XXX */
#if defined(MLD_CONFIG_NO_ASM)
#undef MLD_CONFIG_USE_NATIVE_BACKEND_ARITH
#undef MLD_CONFIG_USE_NATIVE_BACKEND_FIPS202
#endif

#if defined(MLD_CONFIG_USE_NATIVE_BACKEND_ARITH) && \
    !defined(MLD_CONFIG_ARITH_BACKEND_FILE)
#error Bad configuration: MLD_CONFIG_USE_NATIVE_BACKEND_ARITH is set, but MLD_CONFIG_ARITH_BACKEND_FILE is not.
#endif

#if defined(MLD_CONFIG_USE_NATIVE_BACKEND_FIPS202) && \
    !defined(MLD_CONFIG_FIPS202_BACKEND_FILE)
#error Bad configuration: MLD_CONFIG_USE_NATIVE_BACKEND_FIPS202 is set, but MLD_CONFIG_FIPS202_BACKEND_FILE is not.
#endif

#if defined(MLD_CONFIG_NO_RANDOMIZED_API) && defined(MLD_CONFIG_KEYGEN_PCT)
#error Bad configuration: MLD_CONFIG_NO_RANDOMIZED_API is incompatible with MLD_CONFIG_KEYGEN_PCT as the current PCT implementation requires crypto_sign_signature()
#endif

#if defined(MLD_CONFIG_NO_SIGN_API) && defined(MLD_CONFIG_KEYGEN_PCT)
#error Bad configuration: MLD_CONFIG_NO_SIGN_API is incompatible with MLD_CONFIG_KEYGEN_PCT as the current PCT implementation requires crypto_sign_signature()
#endif

#if defined(MLD_CONFIG_NO_VERIFY_API) && defined(MLD_CONFIG_KEYGEN_PCT)
#error Bad configuration: MLD_CONFIG_NO_VERIFY_API is incompatible with MLD_CONFIG_KEYGEN_PCT as the current PCT implementation requires crypto_sign_verify()
#endif

#if defined(MLD_CONFIG_USE_NATIVE_BACKEND_ARITH)
#include MLD_CONFIG_ARITH_BACKEND_FILE
/* Include to enforce consistency of API and implementation,
 * and conduct sanity checks on the backend.
 *
 * Keep this _after_ the inclusion of the backend; otherwise,
 * the sanity checks won't have an effect. */
#if defined(MLD_CHECK_APIS) && !defined(__ASSEMBLER__)
#include "native/api.h"
#endif
#endif /* MLD_CONFIG_USE_NATIVE_BACKEND_ARITH */

#if defined(MLD_CONFIG_USE_NATIVE_BACKEND_FIPS202)
#include MLD_CONFIG_FIPS202_BACKEND_FILE
/* Include to enforce consistency of API and implementation,
 * and conduct sanity checks on the backend.
 *
 * Keep this _after_ the inclusion of the backend; otherwise,
 * the sanity checks won't have an effect. */
#if defined(MLD_CHECK_APIS) && !defined(__ASSEMBLER__)
#include "fips202/native/api.h"
#endif
#endif /* MLD_CONFIG_USE_NATIVE_BACKEND_FIPS202 */

#if !defined(MLD_CONFIG_FIPS202_CUSTOM_HEADER)
#define MLD_FIPS202_HEADER_FILE "fips202/fips202.h"
#else
#define MLD_FIPS202_HEADER_FILE MLD_CONFIG_FIPS202_CUSTOM_HEADER
#endif

#if !defined(MLD_CONFIG_FIPS202X4_CUSTOM_HEADER)
#define MLD_FIPS202X4_HEADER_FILE "fips202/fips202x4.h"
#else
#define MLD_FIPS202X4_HEADER_FILE MLD_CONFIG_FIPS202X4_CUSTOM_HEADER
#endif

/* Standard library function replacements */
#if !defined(__ASSEMBLER__)
#if !defined(MLD_CONFIG_CUSTOM_MEMCPY)
#include <string.h>
#define mld_memcpy memcpy
#endif

#if !defined(MLD_CONFIG_CUSTOM_MEMSET)
#include <string.h>
#define mld_memset memset
#endif

/* Allocation macros for large local structures
 *
 * MLD_ALLOC(v, T, N) declares T *v and attempts to point it to an T[N]
 * MLD_FREE(v, T, N) zeroizes and frees the allocation
 *
 * Default implementation uses stack allocation.
 * Can be overridden by setting the config option MLD_CONFIG_CUSTOM_ALLOC_FREE
 * and defining MLD_CUSTOM_ALLOC and MLD_CUSTOM_FREE.
 */
#if defined(MLD_CONFIG_CUSTOM_ALLOC_FREE) != \
    (defined(MLD_CUSTOM_ALLOC) && defined(MLD_CUSTOM_FREE))
#error Bad configuration: MLD_CONFIG_CUSTOM_ALLOC_FREE must be set together with MLD_CUSTOM_ALLOC and MLD_CUSTOM_FREE
#endif

/*
 * If the integration wants to provide a context parameter for use in
 * platform-specific hooks, then it should define this parameter.
 *
 * The MLD_CONTEXT_PARAMETERS_n macros are intended to be used with macros
 * defining the function names and expand to either pass or discard the context
 * argument as required by the current build.  If there is no context parameter
 * requested then these are removed from the prototypes and from all calls.
 */
#ifdef MLD_CONFIG_CONTEXT_PARAMETER
#define MLD_CONTEXT_PARAMETERS_0(context) (context)
#define MLD_CONTEXT_PARAMETERS_1(arg0, context) (arg0, context)
#define MLD_CONTEXT_PARAMETERS_2(arg0, arg1, context) (arg0, arg1, context)
#define MLD_CONTEXT_PARAMETERS_3(arg0, arg1, arg2, context) \
  (arg0, arg1, arg2, context)
#define MLD_CONTEXT_PARAMETERS_4(arg0, arg1, arg2, arg3, context) \
  (arg0, arg1, arg2, arg3, context)
#define MLD_CONTEXT_PARAMETERS_5(arg0, arg1, arg2, arg3, arg4, context) \
  (arg0, arg1, arg2, arg3, arg4, context)
#define MLD_CONTEXT_PARAMETERS_6(arg0, arg1, arg2, arg3, arg4, arg5, context) \
  (arg0, arg1, arg2, arg3, arg4, arg5, context)
#define MLD_CONTEXT_PARAMETERS_7(arg0, arg1, arg2, arg3, arg4, arg5, arg6, \
                                 context)                                  \
  (arg0, arg1, arg2, arg3, arg4, arg5, arg6, context)
#define MLD_CONTEXT_PARAMETERS_8(arg0, arg1, arg2, arg3, arg4, arg5, arg6, \
                                 arg7, context)                            \
  (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, context)
#define MLD_CONTEXT_PARAMETERS_9(arg0, arg1, arg2, arg3, arg4, arg5, arg6, \
                                 arg7, arg8, context)                      \
  (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, context)
#else /* MLD_CONFIG_CONTEXT_PARAMETER */
#define MLD_CONTEXT_PARAMETERS_0(context) ()
#define MLD_CONTEXT_PARAMETERS_1(arg0, context) (arg0)
#define MLD_CONTEXT_PARAMETERS_2(arg0, arg1, context) (arg0, arg1)
#define MLD_CONTEXT_PARAMETERS_3(arg0, arg1, arg2, context) (arg0, arg1, arg2)
#define MLD_CONTEXT_PARAMETERS_4(arg0, arg1, arg2, arg3, context) \
  (arg0, arg1, arg2, arg3)
#define MLD_CONTEXT_PARAMETERS_5(arg0, arg1, arg2, arg3, arg4, context) \
  (arg0, arg1, arg2, arg3, arg4)
#define MLD_CONTEXT_PARAMETERS_6(arg0, arg1, arg2, arg3, arg4, arg5, context) \
  (arg0, arg1, arg2, arg3, arg4, arg5)
#define MLD_CONTEXT_PARAMETERS_7(arg0, arg1, arg2, arg3, arg4, arg5, arg6, \
                                 context)                                  \
  (arg0, arg1, arg2, arg3, arg4, arg5, arg6)
#define MLD_CONTEXT_PARAMETERS_8(arg0, arg1, arg2, arg3, arg4, arg5, arg6, \
                                 arg7, context)                            \
  (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
#define MLD_CONTEXT_PARAMETERS_9(arg0, arg1, arg2, arg3, arg4, arg5, arg6, \
                                 arg7, arg8, context)                      \
  (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)
#endif /* !MLD_CONFIG_CONTEXT_PARAMETER */

#if defined(MLD_CONFIG_CONTEXT_PARAMETER_TYPE) != \
    defined(MLD_CONFIG_CONTEXT_PARAMETER)
#error MLD_CONFIG_CONTEXT_PARAMETER_TYPE must be defined if and only if MLD_CONFIG_CONTEXT_PARAMETER is defined
#endif

#if !defined(MLD_CONFIG_CUSTOM_ALLOC_FREE)
/* Default: stack allocation */

#define MLD_ALLOC(v, T, N, context) \
  MLD_ALIGN T mld_alloc_##v[N];     \
  T *v = mld_alloc_##v

/* TODO: This leads to a circular dependency between common and ct.h
 * It just works out before we're at the end of the file, but it's still
 * prone to issues in the future. */
#include "ct.h"
#define MLD_FREE(v, T, N, context)                     \
  do                                                   \
  {                                                    \
    mld_zeroize(mld_alloc_##v, sizeof(mld_alloc_##v)); \
    (v) = NULL;                                        \
  } while (0)

#else /* !MLD_CONFIG_CUSTOM_ALLOC_FREE */

/* Custom allocation */

/*
 * The indirection here is necessary to use MLD_CONTEXT_PARAMETERS_3 here.
 */
#define MLD_APPLY(f, args) f args

#define MLD_ALLOC(v, T, N, context) \
  MLD_APPLY(MLD_CUSTOM_ALLOC, MLD_CONTEXT_PARAMETERS_3(v, T, N, context))

#define MLD_FREE(v, T, N, context)                                            \
  do                                                                          \
  {                                                                           \
    if (v != NULL)                                                            \
    {                                                                         \
      mld_zeroize(v, sizeof(T) * (N));                                        \
      MLD_APPLY(MLD_CUSTOM_FREE, MLD_CONTEXT_PARAMETERS_3(v, T, N, context)); \
      v = NULL;                                                               \
    }                                                                         \
  } while (0)

#endif /* MLD_CONFIG_CUSTOM_ALLOC_FREE */

/****************************** Error codes ***********************************/

/* Generic failure condition */
#define MLD_ERR_FAIL -1
/* An allocation failed. This can only happen if MLD_CONFIG_CUSTOM_ALLOC_FREE
 * is defined and the provided MLD_CUSTOM_ALLOC can fail. */
#define MLD_ERR_OUT_OF_MEMORY -2
/* An rng failure occured. Might be due to insufficient entropy or
 * system misconfiguration. */
#define MLD_ERR_RNG_FAIL -3
/* The signing rejection-sampling loop exceeded
 * MLD_CONFIG_MAX_SIGNING_ATTEMPTS iterations without producing a valid
 * signature. With a FIPS 204 Appendix C compliant bound (>= 814) this
 * has probability < 2^-256. */
#define MLD_ERR_SIGN_ATTEMPTS_EXHAUSTED -4

/* Disjunction over the full set of MLD_ERR_XXX failure codes.
 *
 * Intended for use in top-level `ensures` clauses that admit every
 * possible error. Narrower contracts should enumerate only the
 * specific errors they can actually return. */
#define MLD_ANY_ERROR(err)                                    \
  ((err) == MLD_ERR_FAIL || (err) == MLD_ERR_OUT_OF_MEMORY || \
   (err) == MLD_ERR_RNG_FAIL || (err) == MLD_ERR_SIGN_ATTEMPTS_EXHAUSTED)


#endif /* !__ASSEMBLER__ */

#endif /* !MLD_COMMON_H */
