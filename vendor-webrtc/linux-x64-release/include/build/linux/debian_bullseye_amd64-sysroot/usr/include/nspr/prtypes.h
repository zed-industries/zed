/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:                prtypes.h
** Description: Definitions of NSPR's basic types
**
** Prototypes and macros used to make up for deficiencies that we have found
** in ANSI environments.
**
** Since we do not wrap <stdlib.h> and all the other standard headers, authors
** of portable code will not know in general that they need these definitions.
** Instead of requiring these authors to find the dependent uses in their code
** and take the following steps only in those C files, we take steps once here
** for all C files.
**/

#ifndef prtypes_h___
#define prtypes_h___

#ifdef MDCPUCFG
#include MDCPUCFG
#else
#include "prcpucfg.h"
#endif

#include <stddef.h>

/***********************************************************************
** MACROS:      PR_EXTERN
**              PR_IMPLEMENT
** DESCRIPTION:
**      These are only for externally visible routines and globals.  For
**      internal routines, just use "extern" for type checking and that
**      will not export internal cross-file or forward-declared symbols.
**      Define a macro for declaring procedures return types. We use this to
**      deal with windoze specific type hackery for DLL definitions. Use
**      PR_EXTERN when the prototype for the method is declared. Use
**      PR_IMPLEMENT for the implementation of the method.
**
** Example:
**   in dowhim.h
**     PR_EXTERN( void ) DoWhatIMean( void );
**   in dowhim.c
**     PR_IMPLEMENT( void ) DoWhatIMean( void ) { return; }
**
**
***********************************************************************/
#if defined(WIN32)

#define PR_EXPORT(__type) extern __declspec(dllexport) __type
#define PR_EXPORT_DATA(__type) extern __declspec(dllexport) __type
#define PR_IMPORT(__type) __declspec(dllimport) __type
#define PR_IMPORT_DATA(__type) __declspec(dllimport) __type

#define PR_EXTERN(__type) extern __declspec(dllexport) __type
#define PR_IMPLEMENT(__type) __declspec(dllexport) __type
#define PR_EXTERN_DATA(__type) extern __declspec(dllexport) __type
#define PR_IMPLEMENT_DATA(__type) __declspec(dllexport) __type

#define PR_CALLBACK
#define PR_CALLBACK_DECL
#define PR_STATIC_CALLBACK(__x) static __x

#elif defined(XP_OS2) && defined(__declspec)

#define PR_EXPORT(__type) extern __declspec(dllexport) __type
#define PR_EXPORT_DATA(__type) extern __declspec(dllexport) __type
#define PR_IMPORT(__type) extern  __declspec(dllimport) __type
#define PR_IMPORT_DATA(__type) extern __declspec(dllimport) __type

#define PR_EXTERN(__type) extern __declspec(dllexport) __type
#define PR_IMPLEMENT(__type) __declspec(dllexport) __type
#define PR_EXTERN_DATA(__type) extern __declspec(dllexport) __type
#define PR_IMPLEMENT_DATA(__type) __declspec(dllexport) __type

#define PR_CALLBACK
#define PR_CALLBACK_DECL
#define PR_STATIC_CALLBACK(__x) static __x

#else /* Unix */

/* GCC 3.3 and later support the visibility attribute. */
#if (__GNUC__ >= 4) || \
    (__GNUC__ == 3 && __GNUC_MINOR__ >= 3)
#define PR_VISIBILITY_DEFAULT __attribute__((visibility("default")))
#else
#define PR_VISIBILITY_DEFAULT
#endif

#define PR_EXPORT(__type) extern PR_VISIBILITY_DEFAULT __type
#define PR_EXPORT_DATA(__type) extern PR_VISIBILITY_DEFAULT __type
#define PR_IMPORT(__type) extern PR_VISIBILITY_DEFAULT __type
#define PR_IMPORT_DATA(__type) extern PR_VISIBILITY_DEFAULT __type

#define PR_EXTERN(__type) extern PR_VISIBILITY_DEFAULT __type
#define PR_IMPLEMENT(__type) PR_VISIBILITY_DEFAULT __type
#define PR_EXTERN_DATA(__type) extern PR_VISIBILITY_DEFAULT __type
#define PR_IMPLEMENT_DATA(__type) PR_VISIBILITY_DEFAULT __type
#define PR_CALLBACK
#define PR_CALLBACK_DECL
#define PR_STATIC_CALLBACK(__x) static __x

#endif

#if defined(_NSPR_BUILD_)
#define NSPR_API(__type) PR_EXPORT(__type)
#define NSPR_DATA_API(__type) PR_EXPORT_DATA(__type)
#else
#define NSPR_API(__type) PR_IMPORT(__type)
#define NSPR_DATA_API(__type) PR_IMPORT_DATA(__type)
#endif

/***********************************************************************
** MACROS:      PR_BEGIN_MACRO
**              PR_END_MACRO
** DESCRIPTION:
**      Macro body brackets so that macros with compound statement definitions
**      behave syntactically more like functions when called.
***********************************************************************/
#define PR_BEGIN_MACRO  do {
#define PR_END_MACRO    } while (0)

/***********************************************************************
** MACROS:      PR_BEGIN_EXTERN_C
**              PR_END_EXTERN_C
** DESCRIPTION:
**      Macro shorthands for conditional C++ extern block delimiters.
***********************************************************************/
#ifdef __cplusplus
#define PR_BEGIN_EXTERN_C       extern "C" {
#define PR_END_EXTERN_C         }
#else
#define PR_BEGIN_EXTERN_C
#define PR_END_EXTERN_C
#endif

/***********************************************************************
** MACROS:      PR_BIT
**              PR_BITMASK
** DESCRIPTION:
** Bit masking macros.  XXX n must be <= 31 to be portable
***********************************************************************/
#define PR_BIT(n)       ((PRUint32)1 << (n))
#define PR_BITMASK(n)   (PR_BIT(n) - 1)

/***********************************************************************
** MACROS:      PR_ROUNDUP
**              PR_MIN
**              PR_MAX
**              PR_ABS
** DESCRIPTION:
**      Commonly used macros for operations on compatible types.
***********************************************************************/
#define PR_ROUNDUP(x,y) ((((x)+((y)-1))/(y))*(y))
#define PR_MIN(x,y)     ((x)<(y)?(x):(y))
#define PR_MAX(x,y)     ((x)>(y)?(x):(y))
#define PR_ABS(x)       ((x)<0?-(x):(x))

/***********************************************************************
** MACROS:      PR_ARRAY_SIZE
** DESCRIPTION:
**  The number of elements in an array.
***********************************************************************/
#define PR_ARRAY_SIZE(a) (sizeof(a)/sizeof((a)[0]))

PR_BEGIN_EXTERN_C

/*
** Starting in NSPR 4.9.5, NSPR's exact-width integer types should match
** the exact-width integer types defined in <stdint.h>. This allows sloppy
** code to use PRInt{N} and int{N}_t interchangeably.
**
** The 8-bit and 16-bit integer types can only be defined using char and
** short. All platforms define the 32-bit integer types using int. So only
** the 64-bit integer types could be defined differently.
**
** NSPR's original strategy was to use the "shortest" 64-bit integer type:
** if long is 64-bit, then prefer it over long long. This strategy is also
** used by Linux/glibc, FreeBSD, and NetBSD.
**
** Other platforms use a different strategy: simply define the 64-bit
** integer types using long long. We define the PR_ALTERNATE_INT64_TYPEDEF
** macro on these platforms. Note that PR_ALTERNATE_INT64_TYPEDEF is for
** internal use by NSPR headers only. Do not define or test this macro in
** your code.
**
** NOTE: NSPR can't use <stdint.h> because C99 requires C++ code to define
** __STDC_LIMIT_MACROS and __STDC_CONSTANT_MACROS to make all the macros
** defined in <stdint.h> available. This strange requirement is gone in
** C11. When most platforms ignore this C99 requirement, NSPR will be able
** to use <stdint.h>. A patch to do that is in NSPR bug 634793.
*/

#if defined(__APPLE__) || defined(__OpenBSD__)
#define PR_ALTERNATE_INT64_TYPEDEF
#endif

/************************************************************************
** TYPES:       PRUint8
**              PRInt8
** DESCRIPTION:
**  The int8 types are known to be 8 bits each. There is no type that
**      is equivalent to a plain "char".
************************************************************************/
#if PR_BYTES_PER_BYTE == 1
typedef unsigned char PRUint8;
/*
** There are two scenarios that require us to define PRInt8 as type 'char'.
** (1)
** Some cfront-based C++ compilers do not like 'signed char' and
** issue the warning message:
**     warning: "signed" not implemented (ignored)
** For these compilers, we have to define PRInt8 as plain 'char'.
** Make sure that plain 'char' is indeed signed under these compilers.
** (2)
** Mozilla C++ code expects the PRInt{N} and int{N}_t types to match (see bug
** 634793). If a platform defines int8_t as 'char', but NSPR defines it as
** 'signed char', it results in a type mismatch.
** On such platforms we define PRInt8 as 'char' to avoid the mismatch.
*/
#if (defined(HPUX) && defined(__cplusplus) /* reason 1*/ \
        && !defined(__GNUC__) && __cplusplus < 199707L) \
    || (defined(SCO) && defined(__cplusplus) /* reason 1 */ \
        && !defined(__GNUC__) && __cplusplus == 1L) \
    || (defined(__sun) && defined(__cplusplus)) /* reason 2 */
typedef char PRInt8;
#else
typedef signed char PRInt8;
#endif
#else
#error No suitable type for PRInt8/PRUint8
#endif

/************************************************************************
 * MACROS:      PR_INT8_MAX
 *              PR_INT8_MIN
 *              PR_UINT8_MAX
 * DESCRIPTION:
 *  The maximum and minimum values of a PRInt8 or PRUint8.
************************************************************************/

#define PR_INT8_MAX 127
#define PR_INT8_MIN (-128)
#define PR_UINT8_MAX 255U

/************************************************************************
** TYPES:       PRUint16
**              PRInt16
** DESCRIPTION:
**  The int16 types are known to be 16 bits each.
************************************************************************/
#if PR_BYTES_PER_SHORT == 2
typedef unsigned short PRUint16;
typedef short PRInt16;
#else
#error No suitable type for PRInt16/PRUint16
#endif

/************************************************************************
 * MACROS:      PR_INT16_MAX
 *              PR_INT16_MIN
 *              PR_UINT16_MAX
 * DESCRIPTION:
 *  The maximum and minimum values of a PRInt16 or PRUint16.
************************************************************************/

#define PR_INT16_MAX 32767
#define PR_INT16_MIN (-32768)
#define PR_UINT16_MAX 65535U

/************************************************************************
** TYPES:       PRUint32
**              PRInt32
** DESCRIPTION:
**  The int32 types are known to be 32 bits each.
************************************************************************/
#if PR_BYTES_PER_INT == 4
typedef unsigned int PRUint32;
typedef int PRInt32;
#define PR_INT32(x)  x
#define PR_UINT32(x) x ## U
#elif PR_BYTES_PER_LONG == 4
typedef unsigned long PRUint32;
typedef long PRInt32;
#define PR_INT32(x)  x ## L
#define PR_UINT32(x) x ## UL
#else
#error No suitable type for PRInt32/PRUint32
#endif

/************************************************************************
 * MACROS:      PR_INT32_MAX
 *              PR_INT32_MIN
 *              PR_UINT32_MAX
 * DESCRIPTION:
 *  The maximum and minimum values of a PRInt32 or PRUint32.
************************************************************************/

#define PR_INT32_MAX PR_INT32(2147483647)
#define PR_INT32_MIN (-PR_INT32_MAX - 1)
#define PR_UINT32_MAX PR_UINT32(4294967295)

/************************************************************************
** TYPES:       PRUint64
**              PRInt64
** DESCRIPTION:
**  The int64 types are known to be 64 bits each. Care must be used when
**      declaring variables of type PRUint64 or PRInt64. Different hardware
**      architectures and even different compilers have varying support for
**      64 bit values. The only guaranteed portability requires the use of
**      the LL_ macros (see prlong.h).
**
** MACROS:      PR_INT64
**              PR_UINT64
** DESCRIPTION:
**  The PR_INT64 and PR_UINT64 macros provide a portable way for
**      specifying 64-bit integer constants. They can only be used if
**      PRInt64 and PRUint64 are defined as compiler-supported 64-bit
**      integer types (i.e., if HAVE_LONG_LONG is defined, which is true
**      for all the supported compilers topday). If PRInt64 and PRUint64
**      are defined as structs, the LL_INIT macro defined in prlong.h has
**      to be used.
**
** MACROS:      PR_INT64_MAX
**              PR_INT64_MIN
**              PR_UINT64_MAX
** DESCRIPTION:
**  The maximum and minimum values of a PRInt64 or PRUint64.
************************************************************************/
#ifdef HAVE_LONG_LONG
/* Keep this in sync with prlong.h. */
#if PR_BYTES_PER_LONG == 8 && !defined(PR_ALTERNATE_INT64_TYPEDEF)
typedef long PRInt64;
typedef unsigned long PRUint64;
#define PR_INT64(x)  x ## L
#define PR_UINT64(x) x ## UL
#elif defined(WIN32) && !defined(__GNUC__)
typedef __int64  PRInt64;
typedef unsigned __int64 PRUint64;
#define PR_INT64(x)  x ## i64
#define PR_UINT64(x) x ## ui64
#else
typedef long long PRInt64;
typedef unsigned long long PRUint64;
#define PR_INT64(x)  x ## LL
#define PR_UINT64(x) x ## ULL
#endif /* PR_BYTES_PER_LONG == 8 */

#define PR_INT64_MAX PR_INT64(0x7fffffffffffffff)
#define PR_INT64_MIN (-PR_INT64_MAX - 1)
#define PR_UINT64_MAX PR_UINT64(-1)
#else  /* !HAVE_LONG_LONG */
typedef struct {
#ifdef IS_LITTLE_ENDIAN
    PRUint32 lo, hi;
#else
    PRUint32 hi, lo;
#endif
} PRInt64;
typedef PRInt64 PRUint64;

#define PR_INT64_MAX (PRInt64){0x7fffffff, 0xffffffff}
#define PR_INT64_MIN (PRInt64){0xffffffff, 0xffffffff}
#define PR_UINT64_MAX (PRUint64){0xffffffff, 0xffffffff}

#endif /* !HAVE_LONG_LONG */

/************************************************************************
** TYPES:       PRUintn
**              PRIntn
** DESCRIPTION:
**  The PRIntn types are most appropriate for automatic variables. They are
**      guaranteed to be at least 16 bits, though various architectures may
**      define them to be wider (e.g., 32 or even 64 bits). These types are
**      never valid for fields of a structure.
************************************************************************/
#if PR_BYTES_PER_INT >= 2
typedef int PRIntn;
typedef unsigned int PRUintn;
#else
#error 'sizeof(int)' not sufficient for platform use
#endif

/************************************************************************
** TYPES:       PRFloat64
** DESCRIPTION:
**  NSPR's floating point type is always 64 bits.
************************************************************************/
typedef double          PRFloat64;

/************************************************************************
** TYPES:       PRSize
** DESCRIPTION:
**  A type for representing the size of objects.
************************************************************************/
typedef size_t PRSize;


/************************************************************************
** TYPES:       PROffset32, PROffset64
** DESCRIPTION:
**  A type for representing byte offsets from some location.
************************************************************************/
typedef PRInt32 PROffset32;
typedef PRInt64 PROffset64;

/************************************************************************
** TYPES:       PRPtrDiff
** DESCRIPTION:
**  A type for pointer difference. Variables of this type are suitable
**      for storing a pointer or pointer subtraction.
************************************************************************/
typedef ptrdiff_t PRPtrdiff;

/************************************************************************
** TYPES:       PRUptrdiff
** DESCRIPTION:
**  A type for pointer difference. Variables of this type are suitable
**      for storing a pointer or pointer sutraction.
************************************************************************/
#ifdef _WIN64
typedef PRUint64 PRUptrdiff;
#else
typedef unsigned long PRUptrdiff;
#endif

/************************************************************************
** TYPES:       PRBool
** DESCRIPTION:
**  Use PRBool for variables and parameter types. Use PR_FALSE and PR_TRUE
**      for clarity of target type in assignments and actual arguments. Use
**      'if (bool)', 'while (!bool)', '(bool) ? x : y' etc., to test booleans
**      just as you would C int-valued conditions.
************************************************************************/
typedef PRIntn PRBool;
#define PR_TRUE 1
#define PR_FALSE 0

/************************************************************************
** TYPES:       PRPackedBool
** DESCRIPTION:
**  Use PRPackedBool within structs where bitfields are not desirable
**      but minimum and consistant overhead matters.
************************************************************************/
typedef PRUint8 PRPackedBool;

/*
** Status code used by some routines that have a single point of failure or
** special status return.
*/
typedef enum { PR_FAILURE = -1, PR_SUCCESS = 0 } PRStatus;

#ifndef __PRUNICHAR__
#define __PRUNICHAR__
#ifdef WIN32
typedef wchar_t PRUnichar;
#else
typedef PRUint16 PRUnichar;
#endif
#endif

/*
** WARNING: The undocumented data types PRWord and PRUword are
** only used in the garbage collection and arena code.  Do not
** use PRWord and PRUword in new code.
**
** A PRWord is an integer that is the same size as a void*.
** It implements the notion of a "word" in the Java Virtual
** Machine.  (See Sec. 3.4 "Words", The Java Virtual Machine
** Specification, Addison-Wesley, September 1996.
** http://java.sun.com/docs/books/vmspec/index.html.)
*/
#ifdef _WIN64
typedef PRInt64 PRWord;
typedef PRUint64 PRUword;
#else
typedef long PRWord;
typedef unsigned long PRUword;
#endif

/*
 * PR_PRETEND_NORETURN, specified at the end of a function declaration,
 * indicates that for the purposes of static analysis, this function does not
 * return.  (The function definition does not need to be annotated.)
 *
 * void PR_Assert(const char *s, const char *file, PRIntn ln)
 *     PR_PRETEND_NORETURN;
 *
 * Some static analyzers, like scan-build from clang, can use this information
 * to eliminate false positives.  From the upstream documentation of
 * scan-build:
 *     This attribute is useful for annotating assertion handlers that actually
 *     can return, but for the purpose of using the analyzer we want to pretend
 *     that such functions do not return.
 */
#ifdef __clang_analyzer__
#if __has_extension(attribute_analyzer_noreturn)
#define PR_PRETEND_NORETURN __attribute__((analyzer_noreturn))
#endif
#endif

#ifndef PR_PRETEND_NORETURN
#define PR_PRETEND_NORETURN /* no support */
#endif

#if defined(NO_NSPR_10_SUPPORT)
#else
/********* ???????????????? FIX ME       ??????????????????????????? *****/
/********************** Some old definitions until pr=>ds transition is done ***/
/********************** Also, we are still using NSPR 1.0. GC ******************/
/*
** Fundamental NSPR macros, used nearly everywhere.
*/

#define PR_PUBLIC_API       PR_IMPLEMENT

/*
** Macro body brackets so that macros with compound statement definitions
** behave syntactically more like functions when called.
*/
#define NSPR_BEGIN_MACRO        do {
#define NSPR_END_MACRO          } while (0)

/*
** Macro shorthands for conditional C++ extern block delimiters.
*/
#ifdef NSPR_BEGIN_EXTERN_C
#undef NSPR_BEGIN_EXTERN_C
#endif
#ifdef NSPR_END_EXTERN_C
#undef NSPR_END_EXTERN_C
#endif

#ifdef __cplusplus
#define NSPR_BEGIN_EXTERN_C     extern "C" {
#define NSPR_END_EXTERN_C       }
#else
#define NSPR_BEGIN_EXTERN_C
#define NSPR_END_EXTERN_C
#endif

#include "obsolete/protypes.h"

/********* ????????????? End Fix me ?????????????????????????????? *****/
#endif /* NO_NSPR_10_SUPPORT */

/*
** Compile-time assert. "condition" must be a constant expression.
** The macro can be used only in places where an "extern" declaration is
** allowed.
*/
#define PR_STATIC_ASSERT(condition) \
    extern void pr_static_assert(int arg[(condition) ? 1 : -1])

PR_END_EXTERN_C

#endif /* prtypes_h___ */

