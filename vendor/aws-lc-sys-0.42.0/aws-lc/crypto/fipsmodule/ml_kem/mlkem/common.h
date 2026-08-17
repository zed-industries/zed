/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLK_COMMON_H
#define MLK_COMMON_H

#ifndef __ASSEMBLER__
#include <stdint.h>
#endif

#define MLK_BUILD_INTERNAL

#if defined(MLK_CONFIG_FILE)
#include MLK_CONFIG_FILE
#else
#include "mlkem_native_config.h"
#endif

#include "params.h"
#include "sys.h"

/* Internal and public API have external linkage by default, but
 * this can be overwritten by the user, e.g. for single-CU builds. */
#if !defined(MLK_CONFIG_INTERNAL_API_QUALIFIER)
#define MLK_INTERNAL_API
#else
#define MLK_INTERNAL_API MLK_CONFIG_INTERNAL_API_QUALIFIER
#endif

#if !defined(MLK_CONFIG_EXTERNAL_API_QUALIFIER)
#define MLK_EXTERNAL_API
#else
#define MLK_EXTERNAL_API MLK_CONFIG_EXTERNAL_API_QUALIFIER
#endif

#define MLK_CONCAT_(x1, x2) x1##x2
#define MLK_CONCAT(x1, x2) MLK_CONCAT_(x1, x2)

#if (defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || \
     defined(MLK_CONFIG_MULTILEVEL_NO_SHARED))
#define MLK_ADD_PARAM_SET(s) MLK_CONCAT(s, MLK_CONFIG_PARAMETER_SET)
#else
#define MLK_ADD_PARAM_SET(s) s
#endif

#define MLK_NAMESPACE_PREFIX MLK_CONCAT(MLK_CONFIG_NAMESPACE_PREFIX, _)
#define MLK_NAMESPACE_PREFIX_K \
  MLK_CONCAT(MLK_ADD_PARAM_SET(MLK_CONFIG_NAMESPACE_PREFIX), _)

/* Functions are prefixed by MLK_CONFIG_NAMESPACE_PREFIX.
 *
 * If multiple parameter sets are used, functions depending on the parameter
 * set are additionally prefixed with 512/768/1024. See mlkem_native_config.h.
 *
 * Example: If MLK_CONFIG_NAMESPACE_PREFIX is mlkem, then
 * MLK_NAMESPACE_K(enc) becomes mlkem512_enc/mlkem768_enc/mlkem1024_enc.
 */
#define MLK_NAMESPACE(s) MLK_CONCAT(MLK_NAMESPACE_PREFIX, s)
#define MLK_NAMESPACE_K(s) MLK_CONCAT(MLK_NAMESPACE_PREFIX_K, s)

/* On Apple platforms, we need to emit leading underscore
 * in front of assembly symbols. We thus introducee a separate
 * namespace wrapper for ASM symbols. */
#if !defined(__APPLE__)
#define MLK_ASM_NAMESPACE(sym) MLK_NAMESPACE(sym)
#else
#define MLK_ASM_NAMESPACE(sym) MLK_CONCAT(_, MLK_NAMESPACE(sym))
#endif

/*
 * On X86_64 if control-flow protections (CET) are enabled (through
 * -fcf-protection=), we add an endbr64 instruction at every global function
 * label.  See sys.h for more details
 */
#if defined(MLK_SYS_X86_64)
#define MLK_ASM_FN_SYMBOL(sym) MLK_ASM_NAMESPACE(sym) : MLK_CET_ENDBR
#elif defined(MLK_SYS_ARMV81M_MVE)
/* clang-format off */
#define MLK_ASM_FN_SYMBOL(sym) \
  .type MLK_ASM_NAMESPACE(sym), %function; \
  MLK_ASM_NAMESPACE(sym) :
/* clang-format on */
#else /* !MLK_SYS_X86_64 && MLK_SYS_ARMV81M_MVE */
#define MLK_ASM_FN_SYMBOL(sym) MLK_ASM_NAMESPACE(sym) :
#endif /* !MLK_SYS_X86_64 && !MLK_SYS_ARMV81M_MVE */

/*
 * Output the size of an assembly function.
 */
#if defined(__ELF__)
#define MLK_ASM_FN_SIZE(sym) \
  .size MLK_ASM_NAMESPACE(sym), .- MLK_ASM_NAMESPACE(sym)
#else
#define MLK_ASM_FN_SIZE(sym)
#endif

/* We aim to simplify the user's life by supporting builds where
 * all source files are included, even those that are not needed.
 * Those files are appropriately guarded and will be empty when unneeded.
 * The following is to avoid compilers complaining about this. */
#define MLK_EMPTY_CU(s) extern int MLK_NAMESPACE_K(empty_cu_##s);

/* MLK_CONFIG_NO_ASM takes precedence over MLK_USE_NATIVE_XXX */
#if defined(MLK_CONFIG_NO_ASM)
#undef MLK_CONFIG_USE_NATIVE_BACKEND_ARITH
#undef MLK_CONFIG_USE_NATIVE_BACKEND_FIPS202
#endif

#if defined(MLK_CONFIG_USE_NATIVE_BACKEND_ARITH) && \
    !defined(MLK_CONFIG_ARITH_BACKEND_FILE)
#error Bad configuration: MLK_CONFIG_USE_NATIVE_BACKEND_ARITH is set, but MLK_CONFIG_ARITH_BACKEND_FILE is not.
#endif

#if defined(MLK_CONFIG_USE_NATIVE_BACKEND_FIPS202) && \
    !defined(MLK_CONFIG_FIPS202_BACKEND_FILE)
#error Bad configuration: MLK_CONFIG_USE_NATIVE_BACKEND_FIPS202 is set, but MLK_CONFIG_FIPS202_BACKEND_FILE is not.
#endif

#if defined(MLK_CONFIG_NO_RANDOMIZED_API) && defined(MLK_CONFIG_KEYGEN_PCT)
#error Bad configuration: MLK_CONFIG_NO_RANDOMIZED_API is incompatible with MLK_CONFIG_KEYGEN_PCT as the current PCT implementation requires crypto_kem_enc()
#endif

#if defined(MLK_CONFIG_USE_NATIVE_BACKEND_ARITH)
#include MLK_CONFIG_ARITH_BACKEND_FILE
/* Include to enforce consistency of API and implementation,
 * and conduct sanity checks on the backend.
 *
 * Keep this _after_ the inclusion of the backend; otherwise,
 * the sanity checks won't have an effect. */
#if defined(MLK_CHECK_APIS) && !defined(__ASSEMBLER__)
#include "native/api.h"
#endif
#endif /* MLK_CONFIG_USE_NATIVE_BACKEND_ARITH */

#if defined(MLK_CONFIG_USE_NATIVE_BACKEND_FIPS202)
#include MLK_CONFIG_FIPS202_BACKEND_FILE
/* Include to enforce consistency of API and implementation,
 * and conduct sanity checks on the backend.
 *
 * Keep this _after_ the inclusion of the backend; otherwise,
 * the sanity checks won't have an effect. */
#if defined(MLK_CHECK_APIS) && !defined(__ASSEMBLER__)
#include "fips202/native/api.h"
#endif
#endif /* MLK_CONFIG_USE_NATIVE_BACKEND_FIPS202 */

#if !defined(MLK_CONFIG_FIPS202_CUSTOM_HEADER)
#define MLK_FIPS202_HEADER_FILE "fips202/fips202.h"
#else
#define MLK_FIPS202_HEADER_FILE MLK_CONFIG_FIPS202_CUSTOM_HEADER
#endif

#if !defined(MLK_CONFIG_FIPS202X4_CUSTOM_HEADER)
#define MLK_FIPS202X4_HEADER_FILE "fips202/fips202x4.h"
#else
#define MLK_FIPS202X4_HEADER_FILE MLK_CONFIG_FIPS202X4_CUSTOM_HEADER
#endif

/* Standard library function replacements */
#if !defined(__ASSEMBLER__)
#if !defined(MLK_CONFIG_CUSTOM_MEMCPY)
#include <string.h>
#define mlk_memcpy memcpy
#endif

#if !defined(MLK_CONFIG_CUSTOM_MEMSET)
#include <string.h>
#define mlk_memset memset
#endif


/* Allocation macros for large local structures
 *
 * MLK_ALLOC(v, T, N) declares T *v and attempts to point it to an T[N]
 * MLK_FREE(v, T, N) zeroizes and frees the allocation
 *
 * Default implementation uses stack allocation.
 * Can be overridden by setting the config option MLK_CONFIG_CUSTOM_ALLOC_FREE
 * and defining MLK_CUSTOM_ALLOC and MLK_CUSTOM_FREE.
 */
#if defined(MLK_CONFIG_CUSTOM_ALLOC_FREE) != \
    (defined(MLK_CUSTOM_ALLOC) && defined(MLK_CUSTOM_FREE))
#error Bad configuration: MLK_CONFIG_CUSTOM_ALLOC_FREE must be set together with MLK_CUSTOM_ALLOC and MLK_CUSTOM_FREE
#endif

/*
 * If the integration wants to provide a context parameter for use in
 * platform-specific hooks, then it should define this parameter.
 *
 * The MLK_CONTEXT_PARAMETERS_n macros are intended to be used with macros
 * defining the function names and expand to either pass or discard the context
 * argument as required by the current build.  If there is no context parameter
 * requested then these are removed from the prototypes and from all calls.
 */
#ifdef MLK_CONFIG_CONTEXT_PARAMETER
#define MLK_CONTEXT_PARAMETERS_0(context) (context)
#define MLK_CONTEXT_PARAMETERS_1(arg0, context) (arg0, context)
#define MLK_CONTEXT_PARAMETERS_2(arg0, arg1, context) (arg0, arg1, context)
#define MLK_CONTEXT_PARAMETERS_3(arg0, arg1, arg2, context) \
  (arg0, arg1, arg2, context)
#define MLK_CONTEXT_PARAMETERS_4(arg0, arg1, arg2, arg3, context) \
  (arg0, arg1, arg2, arg3, context)
#else /* MLK_CONFIG_CONTEXT_PARAMETER */
#define MLK_CONTEXT_PARAMETERS_0(context) ()
#define MLK_CONTEXT_PARAMETERS_1(arg0, context) (arg0)
#define MLK_CONTEXT_PARAMETERS_2(arg0, arg1, context) (arg0, arg1)
#define MLK_CONTEXT_PARAMETERS_3(arg0, arg1, arg2, context) (arg0, arg1, arg2)
#define MLK_CONTEXT_PARAMETERS_4(arg0, arg1, arg2, arg3, context) \
  (arg0, arg1, arg2, arg3)
#endif /* !MLK_CONFIG_CONTEXT_PARAMETER */

#if defined(MLK_CONFIG_CONTEXT_PARAMETER_TYPE) != \
    defined(MLK_CONFIG_CONTEXT_PARAMETER)
#error MLK_CONFIG_CONTEXT_PARAMETER_TYPE must be defined if and only if MLK_CONFIG_CONTEXT_PARAMETER is defined
#endif

#if !defined(MLK_CONFIG_CUSTOM_ALLOC_FREE)
/* Default: stack allocation */

#define MLK_ALLOC(v, T, N, context) \
  MLK_ALIGN T mlk_alloc_##v[N];     \
  T *v = mlk_alloc_##v

/* TODO: This leads to a circular dependency between common and verify.h
 * It just works out before we're at the end of the file, but it's still
 * prone to issues in the future. */
#include "verify.h"
#define MLK_FREE(v, T, N, context)                     \
  do                                                   \
  {                                                    \
    mlk_zeroize(mlk_alloc_##v, sizeof(mlk_alloc_##v)); \
    (v) = NULL;                                        \
  } while (0)

#else /* !MLK_CONFIG_CUSTOM_ALLOC_FREE */

/* Custom allocation */

/*
 * The indirection here is necessary to use MLK_CONTEXT_PARAMETERS_3 here.
 */
#define MLK_APPLY(f, args) f args

#define MLK_ALLOC(v, T, N, context) \
  MLK_APPLY(MLK_CUSTOM_ALLOC, MLK_CONTEXT_PARAMETERS_3(v, T, N, context))

#define MLK_FREE(v, T, N, context)                                            \
  do                                                                          \
  {                                                                           \
    if (v != NULL)                                                            \
    {                                                                         \
      mlk_zeroize(v, sizeof(T) * (N));                                        \
      MLK_APPLY(MLK_CUSTOM_FREE, MLK_CONTEXT_PARAMETERS_3(v, T, N, context)); \
      v = NULL;                                                               \
    }                                                                         \
  } while (0)

#endif /* MLK_CONFIG_CUSTOM_ALLOC_FREE */

/****************************** Error codes ***********************************/

/* Generic failure condition */
#define MLK_ERR_FAIL -1
/* An allocation failed. This can only happen if MLK_CONFIG_CUSTOM_ALLOC_FREE
 * is defined and the provided MLK_CUSTOM_ALLOC can fail. */
#define MLK_ERR_OUT_OF_MEMORY -2
/* An rng failure occured. Might be due to insufficient entropy or
 * system misconfiguration. */
#define MLK_ERR_RNG_FAIL -3

#endif /* !__ASSEMBLER__ */

#endif /* !MLK_COMMON_H */
