/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2016 Intel Corporation.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QGLOBAL_H
# include <QtCore/qglobal.h>
#endif

#ifndef QCOMPILERDETECTION_H
#define QCOMPILERDETECTION_H

/*
   The compiler, must be one of: (Q_CC_x)

     SYM      - Digital Mars C/C++ (used to be Symantec C++)
     MSVC     - Microsoft Visual C/C++, Intel C++ for Windows
     BOR      - Borland/Turbo C++
     WAT      - Watcom C++
     GNU      - GNU C++
     COMEAU   - Comeau C++
     EDG      - Edison Design Group C++
     OC       - CenterLine C++
     SUN      - Forte Developer, or Sun Studio C++
     MIPS     - MIPSpro C++
     DEC      - DEC C++
     HPACC    - HP aC++
     USLC     - SCO OUDK and UDK
     CDS      - Reliant C++
     KAI      - KAI C++
     INTEL    - Intel C++ for Linux, Intel C++ for Windows
     HIGHC    - MetaWare High C/C++
     PGI      - Portland Group C++
     GHS      - Green Hills Optimizing C++ Compilers
     RVCT     - ARM Realview Compiler Suite
     CLANG    - C++ front-end for the LLVM compiler


   Should be sorted most to least authoritative.
*/

/* Symantec C++ is now Digital Mars */
#if defined(__DMC__) || defined(__SC__)
#  define Q_CC_SYM
/* "explicit" semantics implemented in 8.1e but keyword recognized since 7.5 */
#  if defined(__SC__) && __SC__ < 0x750
#    error "Compiler not supported"
#  endif

#elif defined(_MSC_VER)
#  ifdef __clang__
#    define Q_CC_CLANG ((__clang_major__ * 100) + __clang_minor__)
#  endif
#  define Q_CC_MSVC (_MSC_VER)
#  define Q_CC_MSVC_NET
#  define Q_OUTOFLINE_TEMPLATE inline
#  define Q_COMPILER_MANGLES_RETURN_TYPE
#  define Q_FUNC_INFO __FUNCSIG__
#  define Q_ALIGNOF(type) __alignof(type)
#  define Q_DECL_ALIGN(n) __declspec(align(n))
#  define Q_ASSUME_IMPL(expr) __assume(expr)
#  define Q_UNREACHABLE_IMPL() __assume(0)
#  define Q_NORETURN __declspec(noreturn)
#  define Q_DECL_DEPRECATED __declspec(deprecated)
#  ifndef Q_CC_CLANG
#    define Q_DECL_DEPRECATED_X(text) __declspec(deprecated(text))
#  endif
#  define Q_DECL_EXPORT __declspec(dllexport)
#  define Q_DECL_IMPORT __declspec(dllimport)
#  define QT_MAKE_UNCHECKED_ARRAY_ITERATOR(x) stdext::make_unchecked_array_iterator(x) // Since _MSC_VER >= 1800
#  define QT_MAKE_CHECKED_ARRAY_ITERATOR(x, N) stdext::make_checked_array_iterator(x, size_t(N)) // Since _MSC_VER >= 1500
/* Intel C++ disguising as Visual C++: the `using' keyword avoids warnings */
#  if defined(__INTEL_COMPILER)
#    define Q_DECL_VARIABLE_DEPRECATED
#    define Q_CC_INTEL  __INTEL_COMPILER
#  endif

#elif defined(__BORLANDC__) || defined(__TURBOC__)
#  define Q_CC_BOR
#  define Q_INLINE_TEMPLATE
#  if __BORLANDC__ < 0x502
#    error "Compiler not supported"
#  endif

#elif defined(__WATCOMC__)
#  define Q_CC_WAT

/* ARM Realview Compiler Suite
   RVCT compiler also defines __EDG__ and __GNUC__ (if --gnu flag is given),
   so check for it before that */
#elif defined(__ARMCC__) || defined(__CC_ARM)
#  define Q_CC_RVCT
/* work-around for missing compiler intrinsics */
#  define __is_empty(X) false
#  define __is_pod(X) false
#  define Q_DECL_DEPRECATED __attribute__ ((__deprecated__))
#  ifdef Q_OS_LINUX
#    define Q_DECL_EXPORT     __attribute__((visibility("default")))
#    define Q_DECL_IMPORT     __attribute__((visibility("default")))
#    define Q_DECL_HIDDEN     __attribute__((visibility("hidden")))
#  else
#    define Q_DECL_EXPORT     __declspec(dllexport)
#    define Q_DECL_IMPORT     __declspec(dllimport)
#  endif

#elif defined(__GNUC__)
#  define Q_CC_GNU          (__GNUC__ * 100 + __GNUC_MINOR__)
#  if defined(__MINGW32__)
#    define Q_CC_MINGW
#  endif
#  if defined(__INTEL_COMPILER)
/* Intel C++ also masquerades as GCC */
#    define Q_CC_INTEL      (__INTEL_COMPILER)
#    ifdef __clang__
/* Intel C++ masquerades as Clang masquerading as GCC */
#      define Q_CC_CLANG    305
#    endif
#    define Q_ASSUME_IMPL(expr)  __assume(expr)
#    define Q_UNREACHABLE_IMPL() __builtin_unreachable()
#    if __INTEL_COMPILER >= 1300 && !defined(__APPLE__)
#      define Q_DECL_DEPRECATED_X(text) __attribute__ ((__deprecated__(text)))
#    endif
#  elif defined(__clang__)
/* Clang also masquerades as GCC */
#    if defined(__apple_build_version__)
#      /* http://en.wikipedia.org/wiki/Xcode#Toolchain_Versions */
#      if __apple_build_version__ >= 8020041
#        define Q_CC_CLANG 309
#      elif __apple_build_version__ >= 8000038
#        define Q_CC_CLANG 308
#      elif __apple_build_version__ >= 7000053
#        define Q_CC_CLANG 306
#      elif __apple_build_version__ >= 6000051
#        define Q_CC_CLANG 305
#      elif __apple_build_version__ >= 5030038
#        define Q_CC_CLANG 304
#      elif __apple_build_version__ >= 5000275
#        define Q_CC_CLANG 303
#      elif __apple_build_version__ >= 4250024
#        define Q_CC_CLANG 302
#      elif __apple_build_version__ >= 3180045
#        define Q_CC_CLANG 301
#      elif __apple_build_version__ >= 2111001
#        define Q_CC_CLANG 300
#      else
#        error "Unknown Apple Clang version"
#      endif
#    else
#      define Q_CC_CLANG ((__clang_major__ * 100) + __clang_minor__)
#    endif
#    if __has_builtin(__builtin_assume)
#      define Q_ASSUME_IMPL(expr)   __builtin_assume(expr)
#    else
#      define Q_ASSUME_IMPL(expr)  if (expr){} else __builtin_unreachable()
#    endif
#    define Q_UNREACHABLE_IMPL() __builtin_unreachable()
#    if !defined(__has_extension)
#      /* Compatibility with older Clang versions */
#      define __has_extension __has_feature
#    endif
#    if defined(__APPLE__)
     /* Apple/clang specific features */
#      define Q_DECL_CF_RETURNS_RETAINED __attribute__((cf_returns_retained))
#      ifdef __OBJC__
#        define Q_DECL_NS_RETURNS_AUTORELEASED __attribute__((ns_returns_autoreleased))
#      endif
#    endif
#    ifdef __EMSCRIPTEN__
#      define Q_CC_EMSCRIPTEN
#    endif
#  else
/* Plain GCC */
#    if Q_CC_GNU >= 405
#      define Q_ASSUME_IMPL(expr)  if (expr){} else __builtin_unreachable()
#      define Q_UNREACHABLE_IMPL() __builtin_unreachable()
#      define Q_DECL_DEPRECATED_X(text) __attribute__ ((__deprecated__(text)))
#    endif
#  endif

#  ifdef Q_OS_WIN
#    define Q_DECL_EXPORT     __declspec(dllexport)
#    define Q_DECL_IMPORT     __declspec(dllimport)
#  elif defined(QT_VISIBILITY_AVAILABLE)
#    define Q_DECL_EXPORT     __attribute__((visibility("default")))
#    define Q_DECL_IMPORT     __attribute__((visibility("default")))
#    define Q_DECL_HIDDEN     __attribute__((visibility("hidden")))
#  endif

#  define Q_FUNC_INFO       __PRETTY_FUNCTION__
#  define Q_ALIGNOF(type)   __alignof__(type)
#  define Q_TYPEOF(expr)    __typeof__(expr)
#  define Q_DECL_DEPRECATED __attribute__ ((__deprecated__))
#  define Q_DECL_ALIGN(n)   __attribute__((__aligned__(n)))
#  define Q_DECL_UNUSED     __attribute__((__unused__))
#  define Q_LIKELY(expr)    __builtin_expect(!!(expr), true)
#  define Q_UNLIKELY(expr)  __builtin_expect(!!(expr), false)
#  define Q_NORETURN        __attribute__((__noreturn__))
#  define Q_REQUIRED_RESULT __attribute__ ((__warn_unused_result__))
#  define Q_DECL_PURE_FUNCTION __attribute__((pure))
#  define Q_DECL_CONST_FUNCTION __attribute__((const))
#  define Q_DECL_COLD_FUNCTION __attribute__((cold))
#  if !defined(QT_MOC_CPP)
#    define Q_PACKED __attribute__ ((__packed__))
#    ifndef __ARM_EABI__
#      define QT_NO_ARM_EABI
#    endif
#  endif
#  if Q_CC_GNU >= 403 && !defined(Q_CC_CLANG)
#      define Q_ALLOC_SIZE(x) __attribute__((alloc_size(x)))
#  endif

/* IBM compiler versions are a bit messy. There are actually two products:
   the C product, and the C++ product. The C++ compiler is always packaged
   with the latest version of the C compiler. Version numbers do not always
   match. This little table (I'm not sure it's accurate) should be helpful:

   C++ product                C product

   C Set 3.1                  C Compiler 3.0
   ...                        ...
   C++ Compiler 3.6.6         C Compiler 4.3
   ...                        ...
   Visual Age C++ 4.0         ...
   ...                        ...
   Visual Age C++ 5.0         C Compiler 5.0
   ...                        ...
   Visual Age C++ 6.0         C Compiler 6.0

   Now:
   __xlC__    is the version of the C compiler in hexadecimal notation
              is only an approximation of the C++ compiler version
   __IBMCPP__ is the version of the C++ compiler in decimal notation
              but it is not defined on older compilers like C Set 3.1 */
#elif defined(__xlC__)
#  define Q_CC_XLC
#  define Q_FULL_TEMPLATE_INSTANTIATION
#  if __xlC__ < 0x400
#    error "Compiler not supported"
#  elif __xlC__ >= 0x0600
#    define Q_ALIGNOF(type)     __alignof__(type)
#    define Q_TYPEOF(expr)      __typeof__(expr)
#    define Q_DECL_ALIGN(n)     __attribute__((__aligned__(n)))
#    define Q_PACKED            __attribute__((__packed__))
#  endif

/* Older versions of DEC C++ do not define __EDG__ or __EDG - observed
   on DEC C++ V5.5-004. New versions do define  __EDG__ - observed on
   Compaq C++ V6.3-002.
   This compiler is different enough from other EDG compilers to handle
   it separately anyway. */
#elif defined(__DECCXX) || defined(__DECC)
#  define Q_CC_DEC
/* Compaq C++ V6 compilers are EDG-based but I'm not sure about older
   DEC C++ V5 compilers. */
#  if defined(__EDG__)
#    define Q_CC_EDG
#  endif
/* Compaq has disabled EDG's _BOOL macro and uses _BOOL_EXISTS instead
   - observed on Compaq C++ V6.3-002.
   In any case versions prior to Compaq C++ V6.0-005 do not have bool. */
#  if !defined(_BOOL_EXISTS)
#    error "Compiler not supported"
#  endif
/* Spurious (?) error messages observed on Compaq C++ V6.5-014. */
/* Apply to all versions prior to Compaq C++ V6.0-000 - observed on
   DEC C++ V5.5-004. */
#  if __DECCXX_VER < 60060000
#    define Q_BROKEN_TEMPLATE_SPECIALIZATION
#  endif
/* avoid undefined symbol problems with out-of-line template members */
#  define Q_OUTOFLINE_TEMPLATE inline

/* The Portland Group C++ compiler is based on EDG and does define __EDG__
   but the C compiler does not */
#elif defined(__PGI)
#  define Q_CC_PGI
#  if defined(__EDG__)
#    define Q_CC_EDG
#  endif

/* Compilers with EDG front end are similar. To detect them we test:
   __EDG documented by SGI, observed on MIPSpro 7.3.1.1 and KAI C++ 4.0b
   __EDG__ documented in EDG online docs, observed on Compaq C++ V6.3-002
   and PGI C++ 5.2-4 */
#elif !defined(Q_OS_HPUX) && (defined(__EDG) || defined(__EDG__))
#  define Q_CC_EDG
/* From the EDG documentation (does not seem to apply to Compaq C++ or GHS C):
   _BOOL
        Defined in C++ mode when bool is a keyword. The name of this
        predefined macro is specified by a configuration flag. _BOOL
        is the default.
   __BOOL_DEFINED
        Defined in Microsoft C++ mode when bool is a keyword. */
#  if !defined(_BOOL) && !defined(__BOOL_DEFINED) && !defined(__ghs)
#    error "Compiler not supported"
#  endif

/* The Comeau compiler is based on EDG and does define __EDG__ */
#  if defined(__COMO__)
#    define Q_CC_COMEAU

/* The `using' keyword was introduced to avoid KAI C++ warnings
   but it's now causing KAI C++ errors instead. The standard is
   unclear about the use of this keyword, and in practice every
   compiler is using its own set of rules. Forget it. */
#  elif defined(__KCC)
#    define Q_CC_KAI

/* Using the `using' keyword avoids Intel C++ for Linux warnings */
#  elif defined(__INTEL_COMPILER)
#    define Q_CC_INTEL      (__INTEL_COMPILER)

/* Uses CFront, make sure to read the manual how to tweak templates. */
#  elif defined(__ghs)
#    define Q_CC_GHS
#    define Q_DECL_DEPRECATED __attribute__ ((__deprecated__))
#    define Q_PACKED __attribute__ ((__packed__))
#    define Q_FUNC_INFO       __PRETTY_FUNCTION__
#    define Q_TYPEOF(expr)      __typeof__(expr)
#    define Q_ALIGNOF(type)     __alignof__(type)
#    define Q_UNREACHABLE_IMPL()
#    if defined(__cplusplus)
#      define Q_COMPILER_AUTO_TYPE
#      define Q_COMPILER_STATIC_ASSERT
#      define Q_COMPILER_RANGE_FOR
#      if __GHS_VERSION_NUMBER >= 201505
#        define Q_COMPILER_ALIGNAS
#        define Q_COMPILER_ALIGNOF
#        define Q_COMPILER_ATOMICS
#        define Q_COMPILER_ATTRIBUTES
#        define Q_COMPILER_AUTO_FUNCTION
#        define Q_COMPILER_CLASS_ENUM
#        define Q_COMPILER_DECLTYPE
#        define Q_COMPILER_DEFAULT_MEMBERS
#        define Q_COMPILER_DELETE_MEMBERS
#        define Q_COMPILER_DELEGATING_CONSTRUCTORS
#        define Q_COMPILER_EXPLICIT_CONVERSIONS
#        define Q_COMPILER_EXPLICIT_OVERRIDES
#        define Q_COMPILER_EXTERN_TEMPLATES
#        define Q_COMPILER_INHERITING_CONSTRUCTORS
#        define Q_COMPILER_INITIALIZER_LISTS
#        define Q_COMPILER_LAMBDA
#        define Q_COMPILER_NONSTATIC_MEMBER_INIT
#        define Q_COMPILER_NOEXCEPT
#        define Q_COMPILER_NULLPTR
#        define Q_COMPILER_RANGE_FOR
#        define Q_COMPILER_RAW_STRINGS
#        define Q_COMPILER_REF_QUALIFIERS
#        define Q_COMPILER_RVALUE_REFS
#        define Q_COMPILER_STATIC_ASSERT
#        define Q_COMPILER_TEMPLATE_ALIAS
#        define Q_COMPILER_THREAD_LOCAL
#        define Q_COMPILER_THREADSAFE_STATICS
#        define Q_COMPILER_UDL
#        define Q_COMPILER_UNICODE_STRINGS
#        define Q_COMPILER_UNIFORM_INIT
#        define Q_COMPILER_UNRESTRICTED_UNIONS
#        define Q_COMPILER_VARIADIC_MACROS
#        define Q_COMPILER_VARIADIC_TEMPLATES
#      endif
#    endif //__cplusplus

#  elif defined(__DCC__)
#    define Q_CC_DIAB
#    if !defined(__bool)
#      error "Compiler not supported"
#    endif

/* The UnixWare 7 UDK compiler is based on EDG and does define __EDG__ */
#  elif defined(__USLC__) && defined(__SCO_VERSION__)
#    define Q_CC_USLC
/* The latest UDK 7.1.1b does not need this, but previous versions do */
#    if !defined(__SCO_VERSION__) || (__SCO_VERSION__ < 302200010)
#      define Q_OUTOFLINE_TEMPLATE inline
#    endif

/* Never tested! */
#  elif defined(CENTERLINE_CLPP) || defined(OBJECTCENTER)
#    define Q_CC_OC

/* CDS++ defines __EDG__ although this is not documented in the Reliant
   documentation. It also follows conventions like _BOOL and this documented */
#  elif defined(sinix)
#    define Q_CC_CDS

/* The MIPSpro compiler defines __EDG */
#  elif defined(__sgi)
#    define Q_CC_MIPS
#    define Q_NO_TEMPLATE_FRIENDS
#    if defined(_COMPILER_VERSION) && (_COMPILER_VERSION >= 740)
#      define Q_OUTOFLINE_TEMPLATE inline
#      pragma set woff 3624,3625,3649 /* turn off some harmless warnings */
#    endif
#  endif

/* VxWorks' DIAB toolchain has an additional EDG type C++ compiler
   (see __DCC__ above). This one is for C mode files (__EDG is not defined) */
#elif defined(_DIAB_TOOL)
#  define Q_CC_DIAB
#  define Q_FUNC_INFO       __PRETTY_FUNCTION__

/* Never tested! */
#elif defined(__HIGHC__)
#  define Q_CC_HIGHC

#elif defined(__SUNPRO_CC) || defined(__SUNPRO_C)
#  define Q_CC_SUN
#  define Q_COMPILER_MANGLES_RETURN_TYPE
/* 5.0 compiler or better
    'bool' is enabled by default but can be disabled using -features=nobool
    in which case _BOOL is not defined
        this is the default in 4.2 compatibility mode triggered by -compat=4 */
#  if __SUNPRO_CC >= 0x500
#    define QT_NO_TEMPLATE_TEMPLATE_PARAMETERS
   /* see http://developers.sun.com/sunstudio/support/Ccompare.html */
#    if __SUNPRO_CC >= 0x590
#      define Q_ALIGNOF(type)   __alignof__(type)
#      define Q_TYPEOF(expr)    __typeof__(expr)
#      define Q_DECL_ALIGN(n)   __attribute__((__aligned__(n)))
#    endif
#    if __SUNPRO_CC >= 0x550
#      define Q_DECL_EXPORT     __global
#    endif
#    if __SUNPRO_CC < 0x5a0
#      define Q_NO_TEMPLATE_FRIENDS
#    endif
#    if !defined(_BOOL)
#      error "Compiler not supported"
#    endif
/* 4.2 compiler or older */
#  else
#    error "Compiler not supported"
#  endif

/* CDS++ does not seem to define __EDG__ or __EDG according to Reliant
   documentation but nevertheless uses EDG conventions like _BOOL */
#elif defined(sinix)
#  define Q_CC_EDG
#  define Q_CC_CDS
#  if !defined(_BOOL)
#    error "Compiler not supported"
#  endif
#  define Q_BROKEN_TEMPLATE_SPECIALIZATION

#elif defined(Q_OS_HPUX)
/* __HP_aCC was not defined in first aCC releases */
#  if defined(__HP_aCC) || __cplusplus >= 199707L
#    define Q_NO_TEMPLATE_FRIENDS
#    define Q_CC_HPACC
#    define Q_FUNC_INFO         __PRETTY_FUNCTION__
#    if __HP_aCC-0 < 060000
#      define QT_NO_TEMPLATE_TEMPLATE_PARAMETERS
#      define Q_DECL_EXPORT     __declspec(dllexport)
#      define Q_DECL_IMPORT     __declspec(dllimport)
#    endif
#    if __HP_aCC-0 >= 061200
#      define Q_DECL_ALIGN(n) __attribute__((aligned(n)))
#    endif
#    if __HP_aCC-0 >= 062000
#      define Q_DECL_EXPORT     __attribute__((visibility("default")))
#      define Q_DECL_HIDDEN     __attribute__((visibility("hidden")))
#      define Q_DECL_IMPORT     Q_DECL_EXPORT
#    endif
#  else
#    error "Compiler not supported"
#  endif

#else
#  error "Qt has not been tested with this compiler - see http://www.qt-project.org/"
#endif

/*
 * SG10's SD-6 feature detection and some useful extensions from Clang and GCC
 * https://isocpp.org/std/standing-documents/sd-6-sg10-feature-test-recommendations
 * http://clang.llvm.org/docs/LanguageExtensions.html#feature-checking-macros
 * Not using wrapper macros, per http://eel.is/c++draft/cpp.cond#7.sentence-2
 */
#ifndef __has_builtin
#  define __has_builtin(x)             0
#endif
#ifndef __has_feature
#  define __has_feature(x)             0
#endif
#ifndef __has_attribute
#  define __has_attribute(x)           0
#endif
#ifndef __has_cpp_attribute
#  define __has_cpp_attribute(x)       0
#endif
#ifndef __has_include
#  define __has_include(x)             0
#endif
#ifndef __has_include_next
#  define __has_include_next(x)        0
#endif

// Kept around until all submodules have transitioned
#define QT_HAS_BUILTIN(x)        __has_builtin(x)
#define QT_HAS_FEATURE(x)        __has_feature(x)
#define QT_HAS_ATTRIBUTE(x)      __has_attribute(x)
#define QT_HAS_CPP_ATTRIBUTE(x)  __has_cpp_attribute(x)
#define QT_HAS_INCLUDE(x)        __has_include(x)
#define QT_HAS_INCLUDE_NEXT(x)   __has_include_next(x)

/*
 * C++11 support
 *
 *  Paper           Macro                               SD-6 macro
 *  N2341           Q_COMPILER_ALIGNAS
 *  N2341           Q_COMPILER_ALIGNOF
 *  N2427           Q_COMPILER_ATOMICS
 *  N2761           Q_COMPILER_ATTRIBUTES               __cpp_attributes = 200809
 *  N2541           Q_COMPILER_AUTO_FUNCTION
 *  N1984 N2546     Q_COMPILER_AUTO_TYPE
 *  N2437           Q_COMPILER_CLASS_ENUM
 *  N2235           Q_COMPILER_CONSTEXPR                __cpp_constexpr = 200704
 *  N2343 N3276     Q_COMPILER_DECLTYPE                 __cpp_decltype = 200707
 *  N2346           Q_COMPILER_DEFAULT_MEMBERS
 *  N2346           Q_COMPILER_DELETE_MEMBERS
 *  N1986           Q_COMPILER_DELEGATING_CONSTRUCTORS
 *  N2437           Q_COMPILER_EXPLICIT_CONVERSIONS
 *  N3206 N3272     Q_COMPILER_EXPLICIT_OVERRIDES
 *  N1987           Q_COMPILER_EXTERN_TEMPLATES
 *  N2540           Q_COMPILER_INHERITING_CONSTRUCTORS
 *  N2672           Q_COMPILER_INITIALIZER_LISTS
 *  N2658 N2927     Q_COMPILER_LAMBDA                   __cpp_lambdas = 200907
 *  N2756           Q_COMPILER_NONSTATIC_MEMBER_INIT
 *  N2855 N3050     Q_COMPILER_NOEXCEPT
 *  N2431           Q_COMPILER_NULLPTR
 *  N2930           Q_COMPILER_RANGE_FOR
 *  N2442           Q_COMPILER_RAW_STRINGS              __cpp_raw_strings = 200710
 *  N2439           Q_COMPILER_REF_QUALIFIERS
 *  N2118 N2844 N3053 Q_COMPILER_RVALUE_REFS            __cpp_rvalue_references = 200610
 *  N1720           Q_COMPILER_STATIC_ASSERT            __cpp_static_assert = 200410
 *  N2258           Q_COMPILER_TEMPLATE_ALIAS
 *  N2659           Q_COMPILER_THREAD_LOCAL
 *  N2660           Q_COMPILER_THREADSAFE_STATICS
 *  N2765           Q_COMPILER_UDL                      __cpp_user_defined_literals = 200809
 *  N2442           Q_COMPILER_UNICODE_STRINGS          __cpp_unicode_literals = 200710
 *  N2640           Q_COMPILER_UNIFORM_INIT
 *  N2544           Q_COMPILER_UNRESTRICTED_UNIONS
 *  N1653           Q_COMPILER_VARIADIC_MACROS
 *  N2242 N2555     Q_COMPILER_VARIADIC_TEMPLATES       __cpp_variadic_templates = 200704
 *
 * For any future version of the C++ standard, we use only the SD-6 macro.
 * For full listing, see
 *  http://isocpp.org/std/standing-documents/sd-6-sg10-feature-test-recommendations
 *
 * C++ extensions:
 *  Q_COMPILER_RESTRICTED_VLA       variable-length arrays, prior to __cpp_runtime_arrays
 */

#ifdef __cplusplus
#  if __cplusplus < 201103L && !defined(Q_CC_MSVC)
#    error Qt requires a C++11 compiler and yours does not seem to be that.
#  endif
#endif

#if defined(Q_CC_INTEL) && !defined(Q_CC_MSVC)
#  define Q_COMPILER_RESTRICTED_VLA
#  define Q_COMPILER_VARIADIC_MACROS // C++11 feature supported as an extension in other modes, too
#  define Q_COMPILER_THREADSAFE_STATICS
#  if __INTEL_COMPILER < 1200
#    define Q_NO_TEMPLATE_FRIENDS
#  endif
#  if __INTEL_COMPILER >= 1310 && !defined(_WIN32)
//    ICC supports C++14 binary literals in C, C++98, and C++11 modes
//    at least since 13.1, but I can't test further back
#     define Q_COMPILER_BINARY_LITERALS
#  endif
#  if __cplusplus >= 201103L || defined(__INTEL_CXX11_MODE__)
#    if __INTEL_COMPILER >= 1200
#      define Q_COMPILER_AUTO_TYPE
#      define Q_COMPILER_CLASS_ENUM
#      define Q_COMPILER_DECLTYPE
#      define Q_COMPILER_DEFAULT_MEMBERS
#      define Q_COMPILER_DELETE_MEMBERS
#      define Q_COMPILER_EXTERN_TEMPLATES
#      define Q_COMPILER_LAMBDA
#      define Q_COMPILER_RVALUE_REFS
#      define Q_COMPILER_STATIC_ASSERT
#      define Q_COMPILER_VARIADIC_MACROS
#    endif
#    if __INTEL_COMPILER >= 1210
#      define Q_COMPILER_ATTRIBUTES
#      define Q_COMPILER_AUTO_FUNCTION
#      define Q_COMPILER_NULLPTR
#      define Q_COMPILER_TEMPLATE_ALIAS
#      ifndef _CHAR16T      // MSVC headers
#        define Q_COMPILER_UNICODE_STRINGS
#      endif
#      define Q_COMPILER_VARIADIC_TEMPLATES
#    endif
#    if __INTEL_COMPILER >= 1300
#      define Q_COMPILER_ATOMICS
//       constexpr support is only partial
//#      define Q_COMPILER_CONSTEXPR
#      define Q_COMPILER_INITIALIZER_LISTS
#      define Q_COMPILER_UNIFORM_INIT
#      define Q_COMPILER_NOEXCEPT
#    endif
#    if __INTEL_COMPILER >= 1400
//       causes issues with QArrayData and QtPrivate::RefCount - Intel issue ID 6000056211, bug DPD200534796
//#      define Q_COMPILER_CONSTEXPR
#      define Q_COMPILER_DELEGATING_CONSTRUCTORS
#      define Q_COMPILER_EXPLICIT_CONVERSIONS
#      define Q_COMPILER_EXPLICIT_OVERRIDES
#      define Q_COMPILER_NONSTATIC_MEMBER_INIT
#      define Q_COMPILER_RANGE_FOR
#      define Q_COMPILER_RAW_STRINGS
#      define Q_COMPILER_REF_QUALIFIERS
#      define Q_COMPILER_UNICODE_STRINGS
#      define Q_COMPILER_UNRESTRICTED_UNIONS
#    endif
#    if __INTEL_COMPILER >= 1500
#      if __INTEL_COMPILER * 100 + __INTEL_COMPILER_UPDATE >= 150001
//       the bug mentioned above is fixed in 15.0.1
#        define Q_COMPILER_CONSTEXPR
#      endif
#      define Q_COMPILER_ALIGNAS
#      define Q_COMPILER_ALIGNOF
#      define Q_COMPILER_INHERITING_CONSTRUCTORS
#      define Q_COMPILER_THREAD_LOCAL
#      define Q_COMPILER_UDL
#    endif
#  elif defined(__STDC_VERSION__) && __STDC_VERSION__ > 199901L
//   C11 features supported. Only tested with ICC 17 and up.
#    define Q_COMPILER_STATIC_ASSERT
#    if __has_include(<threads.h>)
#      define Q_COMPILER_THREAD_LOCAL
#    endif
#  endif
#endif

#if defined(Q_CC_CLANG) && !defined(Q_CC_INTEL) && !defined(Q_CC_MSVC)
/* General C++ features */
#  define Q_COMPILER_RESTRICTED_VLA
#  define Q_COMPILER_THREADSAFE_STATICS
#  if __has_feature(attribute_deprecated_with_message)
#    define Q_DECL_DEPRECATED_X(text) __attribute__ ((__deprecated__(text)))
#  endif

// Clang supports binary literals in C, C++98 and C++11 modes
// It's been supported "since the dawn of time itself" (cf. commit 179883)
#  if __has_extension(cxx_binary_literals)
#    define Q_COMPILER_BINARY_LITERALS
#  endif

// Variadic macros are supported for gnu++98, c++11, c99 ... since 2.9
#  if Q_CC_CLANG >= 209
#    if !defined(__STRICT_ANSI__) || defined(__GXX_EXPERIMENTAL_CXX0X__) \
      || (defined(__cplusplus) && (__cplusplus >= 201103L)) \
      || (defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 199901L))
#      define Q_COMPILER_VARIADIC_MACROS
#    endif
#  endif

/* C++11 features, see http://clang.llvm.org/cxx_status.html */
#  if __cplusplus >= 201103L || defined(__GXX_EXPERIMENTAL_CXX0X__)
    /* Detect C++ features using __has_feature(), see http://clang.llvm.org/docs/LanguageExtensions.html#cxx11 */
#    if __has_feature(cxx_alignas)
#      define Q_COMPILER_ALIGNAS
#      define Q_COMPILER_ALIGNOF
#    endif
#    if __has_feature(cxx_atomic) && __has_include(<atomic>)
#     define Q_COMPILER_ATOMICS
#    endif
#    if __has_feature(cxx_attributes)
#      define Q_COMPILER_ATTRIBUTES
#    endif
#    if __has_feature(cxx_auto_type)
#      define Q_COMPILER_AUTO_FUNCTION
#      define Q_COMPILER_AUTO_TYPE
#    endif
#    if __has_feature(cxx_strong_enums)
#      define Q_COMPILER_CLASS_ENUM
#    endif
#    if __has_feature(cxx_constexpr) && Q_CC_CLANG > 302 /* CLANG 3.2 has bad/partial support */
#      define Q_COMPILER_CONSTEXPR
#    endif
#    if __has_feature(cxx_decltype) /* && __has_feature(cxx_decltype_incomplete_return_types) */
#      define Q_COMPILER_DECLTYPE
#    endif
#    if __has_feature(cxx_defaulted_functions)
#      define Q_COMPILER_DEFAULT_MEMBERS
#    endif
#    if __has_feature(cxx_deleted_functions)
#      define Q_COMPILER_DELETE_MEMBERS
#    endif
#    if __has_feature(cxx_delegating_constructors)
#      define Q_COMPILER_DELEGATING_CONSTRUCTORS
#    endif
#    if __has_feature(cxx_explicit_conversions)
#      define Q_COMPILER_EXPLICIT_CONVERSIONS
#    endif
#    if __has_feature(cxx_override_control)
#      define Q_COMPILER_EXPLICIT_OVERRIDES
#    endif
#    if __has_feature(cxx_inheriting_constructors)
#      define Q_COMPILER_INHERITING_CONSTRUCTORS
#    endif
#    if __has_feature(cxx_generalized_initializers)
#      define Q_COMPILER_INITIALIZER_LISTS
#      define Q_COMPILER_UNIFORM_INIT /* both covered by this feature macro, according to docs */
#    endif
#    if __has_feature(cxx_lambdas)
#      define Q_COMPILER_LAMBDA
#    endif
#    if __has_feature(cxx_noexcept)
#      define Q_COMPILER_NOEXCEPT
#    endif
#    if __has_feature(cxx_nonstatic_member_init)
#      define Q_COMPILER_NONSTATIC_MEMBER_INIT
#    endif
#    if __has_feature(cxx_nullptr)
#      define Q_COMPILER_NULLPTR
#    endif
#    if __has_feature(cxx_range_for)
#      define Q_COMPILER_RANGE_FOR
#    endif
#    if __has_feature(cxx_raw_string_literals)
#      define Q_COMPILER_RAW_STRINGS
#    endif
#    if __has_feature(cxx_reference_qualified_functions)
#      define Q_COMPILER_REF_QUALIFIERS
#    endif
#    if __has_feature(cxx_rvalue_references)
#      define Q_COMPILER_RVALUE_REFS
#    endif
#    if __has_feature(cxx_static_assert)
#      define Q_COMPILER_STATIC_ASSERT
#    endif
#    if __has_feature(cxx_alias_templates)
#      define Q_COMPILER_TEMPLATE_ALIAS
#    endif
#    if __has_feature(cxx_thread_local)
#      if !defined(__FreeBSD__) /* FreeBSD clang fails on __cxa_thread_atexit */
#        define Q_COMPILER_THREAD_LOCAL
#      endif
#    endif
#    if __has_feature(cxx_user_literals)
#      define Q_COMPILER_UDL
#    endif
#    if __has_feature(cxx_unicode_literals)
#      define Q_COMPILER_UNICODE_STRINGS
#    endif
#    if __has_feature(cxx_unrestricted_unions)
#      define Q_COMPILER_UNRESTRICTED_UNIONS
#    endif
#    if __has_feature(cxx_variadic_templates)
#      define Q_COMPILER_VARIADIC_TEMPLATES
#    endif
    /* Features that have no __has_feature() check */
#    if Q_CC_CLANG >= 209 /* since clang 2.9 */
#      define Q_COMPILER_EXTERN_TEMPLATES
#    endif
#  endif

/* C++1y features, deprecated macros. Do not update this list. */
#  if __cplusplus > 201103L
//#    if __has_feature(cxx_binary_literals)
//#      define Q_COMPILER_BINARY_LITERALS  // see above
//#    endif
#    if __has_feature(cxx_generic_lambda)
#      define Q_COMPILER_GENERIC_LAMBDA
#    endif
#    if __has_feature(cxx_init_capture)
#      define Q_COMPILER_LAMBDA_CAPTURES
#    endif
#    if __has_feature(cxx_relaxed_constexpr)
#      define Q_COMPILER_RELAXED_CONSTEXPR_FUNCTIONS
#    endif
#    if __has_feature(cxx_decltype_auto) && __has_feature(cxx_return_type_deduction)
#      define Q_COMPILER_RETURN_TYPE_DEDUCTION
#    endif
#    if __has_feature(cxx_variable_templates)
#      define Q_COMPILER_VARIABLE_TEMPLATES
#    endif
#    if __has_feature(cxx_runtime_array)
#      define Q_COMPILER_VLA
#    endif
#  endif

#  if defined(__STDC_VERSION__)
#    if __has_feature(c_static_assert)
#      define Q_COMPILER_STATIC_ASSERT
#    endif
#    if __has_feature(c_thread_local) && __has_include(<threads.h>)
#      if !defined(__FreeBSD__) /* FreeBSD clang fails on __cxa_thread_atexit */
#        define Q_COMPILER_THREAD_LOCAL
#      endif
#    endif
#  endif

#endif // Q_CC_CLANG &&  !Q_CC_INTEL && !Q_CC_MSVC

#if defined(Q_CC_CLANG) && !defined(Q_CC_INTEL)
#  ifndef Q_DECL_UNUSED
#    define Q_DECL_UNUSED __attribute__((__unused__))
#  endif
#  define Q_DECL_UNUSED_MEMBER Q_DECL_UNUSED
#endif

#if defined(Q_CC_GNU) && !defined(Q_CC_INTEL) && !defined(Q_CC_CLANG)
#  define Q_COMPILER_RESTRICTED_VLA
#  define Q_COMPILER_THREADSAFE_STATICS
#  if Q_CC_GNU >= 403
//   GCC supports binary literals in C, C++98 and C++11 modes
#    define Q_COMPILER_BINARY_LITERALS
#  endif
#  if !defined(__STRICT_ANSI__) || defined(__GXX_EXPERIMENTAL_CXX0X__) \
    || (defined(__cplusplus) && (__cplusplus >= 201103L)) \
    || (defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 199901L))
     // Variadic macros are supported for gnu++98, c++11, C99 ... since forever (gcc 2.97)
#    define Q_COMPILER_VARIADIC_MACROS
#  endif
#  if defined(__GXX_EXPERIMENTAL_CXX0X__) || __cplusplus >= 201103L
#    if Q_CC_GNU >= 403
       /* C++11 features supported in GCC 4.3: */
#      define Q_COMPILER_DECLTYPE
#      define Q_COMPILER_RVALUE_REFS
#      define Q_COMPILER_STATIC_ASSERT
#    endif
#    if Q_CC_GNU >= 404
       /* C++11 features supported in GCC 4.4: */
#      define Q_COMPILER_AUTO_FUNCTION
#      define Q_COMPILER_AUTO_TYPE
#      define Q_COMPILER_EXTERN_TEMPLATES
#      define Q_COMPILER_UNIFORM_INIT
#      define Q_COMPILER_UNICODE_STRINGS
#      define Q_COMPILER_VARIADIC_TEMPLATES
#    endif
#    if Q_CC_GNU >= 405
       /* C++11 features supported in GCC 4.5: */
#      define Q_COMPILER_EXPLICIT_CONVERSIONS
       /* GCC 4.4 implements initializer_list but does not define typedefs required
        * by the standard. */
#      define Q_COMPILER_INITIALIZER_LISTS
#      define Q_COMPILER_LAMBDA
#      define Q_COMPILER_RAW_STRINGS
#      define Q_COMPILER_CLASS_ENUM
#    endif
#    if Q_CC_GNU >= 406
       /* Pre-4.6 compilers implement a non-final snapshot of N2346, hence default and delete
        * functions are supported only if they are public. Starting from 4.6, GCC handles
        * final version - the access modifier is not relevant. */
#      define Q_COMPILER_DEFAULT_MEMBERS
#      define Q_COMPILER_DELETE_MEMBERS
       /* C++11 features supported in GCC 4.6: */
#      define Q_COMPILER_NULLPTR
#      define Q_COMPILER_UNRESTRICTED_UNIONS
#      define Q_COMPILER_RANGE_FOR
#    endif
#    if Q_CC_GNU >= 407
       /* GCC 4.4 implemented <atomic> and std::atomic using its old intrinsics.
        * However, the implementation is incomplete for most platforms until GCC 4.7:
        * instead, std::atomic would use an external lock. Since we need an std::atomic
        * that is behavior-compatible with QBasicAtomic, we only enable it here */
#      define Q_COMPILER_ATOMICS
       /* GCC 4.6.x has problems dealing with noexcept expressions,
        * so turn the feature on for 4.7 and above, only */
#      define Q_COMPILER_NOEXCEPT
       /* C++11 features supported in GCC 4.7: */
#      define Q_COMPILER_NONSTATIC_MEMBER_INIT
#      define Q_COMPILER_DELEGATING_CONSTRUCTORS
#      define Q_COMPILER_EXPLICIT_OVERRIDES
#      define Q_COMPILER_TEMPLATE_ALIAS
#      define Q_COMPILER_UDL
#    endif
#    if Q_CC_GNU >= 408
#      define Q_COMPILER_ATTRIBUTES
#      define Q_COMPILER_ALIGNAS
#      define Q_COMPILER_ALIGNOF
#      define Q_COMPILER_INHERITING_CONSTRUCTORS
#      define Q_COMPILER_THREAD_LOCAL
#      if Q_CC_GNU > 408 || __GNUC_PATCHLEVEL__ >= 1
#         define Q_COMPILER_REF_QUALIFIERS
#      endif
#    endif
#    if Q_CC_GNU >= 500
       /* GCC 4.6 introduces constexpr, but it's bugged (at least) in the whole
        * 4.x series, see e.g. https://gcc.gnu.org/bugzilla/show_bug.cgi?id=57694 */
#      define Q_COMPILER_CONSTEXPR
#    endif
#  endif
#  if __cplusplus > 201103L
#    if Q_CC_GNU >= 409
     /* C++1y features in GCC 4.9 - deprecated, do not update this list */
//#    define Q_COMPILER_BINARY_LITERALS   // already supported since GCC 4.3 as an extension
#      define Q_COMPILER_LAMBDA_CAPTURES
#      define Q_COMPILER_RETURN_TYPE_DEDUCTION
#    endif
#  endif
#  if defined(__STDC_VERSION__) && __STDC_VERSION__ > 199901L
#    if Q_CC_GNU >= 407
       /* C11 features supported in GCC 4.7: */
#      define Q_COMPILER_STATIC_ASSERT
#    endif
#    if Q_CC_GNU >= 409 && defined(__has_include)
       /* C11 features supported in GCC 4.9: */
#      if __has_include(<threads.h>)
#        define Q_COMPILER_THREAD_LOCAL
#      endif
#    endif
#  endif
#endif

#if defined(Q_CC_MSVC)
#  if defined(__cplusplus)
       /* C++11 features supported in VC8 = VC2005: */
#      define Q_COMPILER_VARIADIC_MACROS

       /* 2005 supports the override and final contextual keywords, in
        the same positions as the C++11 variants, but 'final' is
        called 'sealed' instead:
        http://msdn.microsoft.com/en-us/library/0w2w91tf%28v=vs.80%29.aspx
        The behavior is slightly different in C++/CLI, which requires the
        "virtual" keyword to be present too, so don't define for that.
        So don't define Q_COMPILER_EXPLICIT_OVERRIDES (since it's not
        the same as the C++11 version), but define the Q_DECL_* flags
        accordingly. */
       /* C++11 features supported in VC10 = VC2010: */
#      define Q_COMPILER_AUTO_FUNCTION
#      define Q_COMPILER_AUTO_TYPE
#      define Q_COMPILER_DECLTYPE
#      define Q_COMPILER_EXTERN_TEMPLATES
#      define Q_COMPILER_LAMBDA
#      define Q_COMPILER_NULLPTR
#      define Q_COMPILER_RVALUE_REFS
#      define Q_COMPILER_STATIC_ASSERT
       /* C++11 features supported in VC11 = VC2012: */
#      define Q_COMPILER_EXPLICIT_OVERRIDES /* ...and use std C++11 now   */
#      define Q_COMPILER_CLASS_ENUM
#      define Q_COMPILER_ATOMICS
       /* C++11 features in VC12 = VC2013 */
#      define Q_COMPILER_DELETE_MEMBERS
#      define Q_COMPILER_DELEGATING_CONSTRUCTORS
#      define Q_COMPILER_EXPLICIT_CONVERSIONS
#      define Q_COMPILER_NONSTATIC_MEMBER_INIT
#      define Q_COMPILER_RAW_STRINGS
#      define Q_COMPILER_TEMPLATE_ALIAS
#      define Q_COMPILER_VARIADIC_TEMPLATES
#      define Q_COMPILER_INITIALIZER_LISTS // VC 12 SP 2 RC
       /* C++11 features in VC14 = VC2015 */
#      define Q_COMPILER_DEFAULT_MEMBERS
#      define Q_COMPILER_ALIGNAS
#      define Q_COMPILER_ALIGNOF
#      define Q_COMPILER_INHERITING_CONSTRUCTORS
#      define Q_COMPILER_NOEXCEPT
#      define Q_COMPILER_RANGE_FOR
#      define Q_COMPILER_REF_QUALIFIERS
#      define Q_COMPILER_THREAD_LOCAL
#      define Q_COMPILER_UDL
#      define Q_COMPILER_UNICODE_STRINGS
#      define Q_COMPILER_UNRESTRICTED_UNIONS
#    if _MSC_FULL_VER >= 190023419
#      define Q_COMPILER_ATTRIBUTES
// Almost working, see https://connect.microsoft.com/VisualStudio/feedback/details/2011648
//#      define Q_COMPILER_CONSTEXPR
#      define Q_COMPILER_THREADSAFE_STATICS
#      define Q_COMPILER_UNIFORM_INIT
#    endif
#    if _MSC_VER >= 1910
#      define Q_COMPILER_CONSTEXPR
#    endif
#  endif /* __cplusplus */
#endif /* Q_CC_MSVC */

#ifdef Q_COMPILER_UNICODE_STRINGS
#  define Q_STDLIB_UNICODE_STRINGS
#endif

#ifdef __cplusplus
# include <utility>
# if defined(Q_OS_QNX)
// By default, QNX 7.0 uses libc++ (from LLVM) and
// QNX 6.X uses Dinkumware's libcpp. In all versions,
// it is also possible to use GNU libstdc++.

// For Dinkumware, some features must be disabled
// (mostly because of library problems).
// Dinkumware is assumed when __GLIBCXX__ (GNU libstdc++)
// and _LIBCPP_VERSION (LLVM libc++) are both absent.
#  if !defined(__GLIBCXX__) && !defined(_LIBCPP_VERSION)

// Older versions of libcpp (QNX 650) do not support C++11 features
// _HAS_* macros are set to 1 by toolchains that actually include
// Dinkum C++11 libcpp.

#   if !defined(_HAS_CPP0X) || !_HAS_CPP0X
// Disable C++11 features that depend on library support
#    undef Q_COMPILER_INITIALIZER_LISTS
#    undef Q_COMPILER_RVALUE_REFS
#    undef Q_COMPILER_REF_QUALIFIERS
#    undef Q_COMPILER_NOEXCEPT
// Disable C++11 library features:
#    undef Q_STDLIB_UNICODE_STRINGS
#   endif // !_HAS_CPP0X
#   if !defined(_HAS_NULLPTR_T) || !_HAS_NULLPTR_T
#    undef Q_COMPILER_NULLPTR
#   endif //!_HAS_NULLPTR_T
#   if !defined(_HAS_CONSTEXPR) || !_HAS_CONSTEXPR
// The libcpp is missing constexpr keywords on important functions like std::numeric_limits<>::min()
// Disable constexpr support on QNX even if the compiler supports it
#    undef Q_COMPILER_CONSTEXPR
#   endif // !_HAS_CONSTEXPR
#  endif // !__GLIBCXX__ && !_LIBCPP_VERSION
# endif // Q_OS_QNX
# if (defined(Q_CC_CLANG) || defined(Q_CC_INTEL)) && defined(Q_OS_MAC) && defined(__GNUC_LIBSTD__) \
    && ((__GNUC_LIBSTD__-0) * 100 + __GNUC_LIBSTD_MINOR__-0 <= 402)
// Apple has not updated libstdc++ since 2007, which means it does not have
// <initializer_list> or std::move. Let's disable these features
#  undef Q_COMPILER_INITIALIZER_LISTS
#  undef Q_COMPILER_RVALUE_REFS
#  undef Q_COMPILER_REF_QUALIFIERS
// Also disable <atomic>, since it's clearly not there
#  undef Q_COMPILER_ATOMICS
# endif
# if defined(Q_CC_CLANG) && defined(Q_CC_INTEL) && Q_CC_INTEL >= 1500
// ICC 15.x and 16.0 have their own implementation of std::atomic, which is activated when in Clang mode
// (probably because libc++'s <atomic> on OS X failed to compile), but they're missing some
// critical definitions. (Reported as Intel Issue ID 6000117277)
#  define __USE_CONSTEXPR 1
#  define __USE_NOEXCEPT 1
# endif
# if defined(Q_COMPILER_THREADSAFE_STATICS) && defined(Q_OS_MAC)
// Apple's low-level implementation of the C++ support library
// (libc++abi.dylib, shared between libstdc++ and libc++) has deadlocks. The
// C++11 standard requires the deadlocks to be removed, so this will eventually
// be fixed; for now, let's disable this.
#  undef Q_COMPILER_THREADSAFE_STATICS
# endif
#endif

/*
 * C++11 keywords and expressions
 */
#ifdef Q_COMPILER_NULLPTR
# define Q_NULLPTR         nullptr
#else
# define Q_NULLPTR         NULL
#endif

#ifdef Q_COMPILER_DEFAULT_MEMBERS
#  define Q_DECL_EQ_DEFAULT = default
#else
#  define Q_DECL_EQ_DEFAULT
#endif

#ifdef Q_COMPILER_DELETE_MEMBERS
# define Q_DECL_EQ_DELETE = delete
#else
# define Q_DECL_EQ_DELETE
#endif

// Don't break code that is already using Q_COMPILER_DEFAULT_DELETE_MEMBERS
#if defined(Q_COMPILER_DEFAULT_MEMBERS) && defined(Q_COMPILER_DELETE_MEMBERS)
#  define Q_COMPILER_DEFAULT_DELETE_MEMBERS
#endif

#if defined Q_COMPILER_CONSTEXPR
# if defined(__cpp_constexpr) && __cpp_constexpr-0 >= 201304
#  define Q_DECL_CONSTEXPR constexpr
#  define Q_DECL_RELAXED_CONSTEXPR constexpr
#  define Q_CONSTEXPR constexpr
#  define Q_RELAXED_CONSTEXPR constexpr
# else
#  define Q_DECL_CONSTEXPR constexpr
#  define Q_DECL_RELAXED_CONSTEXPR
#  define Q_CONSTEXPR constexpr
#  define Q_RELAXED_CONSTEXPR const
# endif
#else
# define Q_DECL_CONSTEXPR
# define Q_DECL_RELAXED_CONSTEXPR
# define Q_CONSTEXPR const
# define Q_RELAXED_CONSTEXPR const
#endif

#ifdef Q_COMPILER_EXPLICIT_OVERRIDES
# define Q_DECL_OVERRIDE override
# define Q_DECL_FINAL final
#else
# ifndef Q_DECL_OVERRIDE
#  define Q_DECL_OVERRIDE
# endif
# ifndef Q_DECL_FINAL
#  define Q_DECL_FINAL
# endif
#endif

#ifdef Q_COMPILER_NOEXCEPT
# define Q_DECL_NOEXCEPT noexcept
# define Q_DECL_NOEXCEPT_EXPR(x) noexcept(x)
#else
# define Q_DECL_NOEXCEPT
# define Q_DECL_NOEXCEPT_EXPR(x)
#endif
#define Q_DECL_NOTHROW Q_DECL_NOEXCEPT

#if defined(Q_COMPILER_ALIGNOF)
#  undef Q_ALIGNOF
#  define Q_ALIGNOF(x)  alignof(x)
#endif

#if defined(Q_COMPILER_ALIGNAS)
#  undef Q_DECL_ALIGN
#  define Q_DECL_ALIGN(n)   alignas(n)
#endif

#if __has_cpp_attribute(nodiscard) && !defined(Q_CC_CLANG)         // P0188R1
// Can't use [[nodiscard]] with Clang, see https://bugs.llvm.org/show_bug.cgi?id=33518
#  undef Q_REQUIRED_RESULT
#  define Q_REQUIRED_RESULT [[nodiscard]]
#endif

#if defined(__cpp_enumerator_attributes) && __cpp_enumerator_attributes >= 201411
#if defined(Q_CC_MSVC)
// Can't mark enum values as __declspec(deprecated) with MSVC, also can't move
// everything to [[deprecated]] because MSVC gives a compilation error when marking
// friend methods of a class as [[deprecated("text")]], breaking qstring.h
#  define Q_DECL_ENUMERATOR_DEPRECATED [[deprecated]]
#  define Q_DECL_ENUMERATOR_DEPRECATED_X(x) [[deprecated(x)]]
#else
#  define Q_DECL_ENUMERATOR_DEPRECATED Q_DECL_DEPRECATED
#  define Q_DECL_ENUMERATOR_DEPRECATED_X(x) Q_DECL_DEPRECATED_X(x)
#endif
#endif

/*
 * Fallback macros to certain compiler features
 */

#ifndef Q_NORETURN
# define Q_NORETURN
#endif
#ifndef Q_LIKELY
#  define Q_LIKELY(x) (x)
#endif
#ifndef Q_UNLIKELY
#  define Q_UNLIKELY(x) (x)
#endif
#ifndef Q_ASSUME_IMPL
#  define Q_ASSUME_IMPL(expr) qt_noop()
#endif
#ifndef Q_UNREACHABLE_IMPL
#  define Q_UNREACHABLE_IMPL() qt_noop()
#endif
#ifndef Q_ALLOC_SIZE
#  define Q_ALLOC_SIZE(x)
#endif
#ifndef Q_REQUIRED_RESULT
#  define Q_REQUIRED_RESULT
#endif
#ifndef Q_DECL_DEPRECATED
#  define Q_DECL_DEPRECATED
#endif
#ifndef Q_DECL_VARIABLE_DEPRECATED
#  define Q_DECL_VARIABLE_DEPRECATED Q_DECL_DEPRECATED
#endif
#ifndef Q_DECL_DEPRECATED_X
#  define Q_DECL_DEPRECATED_X(text) Q_DECL_DEPRECATED
#endif
#ifndef Q_DECL_ENUMERATOR_DEPRECATED
#  define Q_DECL_ENUMERATOR_DEPRECATED
#endif
#ifndef Q_DECL_ENUMERATOR_DEPRECATED_X
#  define Q_DECL_ENUMERATOR_DEPRECATED_X(x)
#endif
#ifndef Q_DECL_EXPORT
#  define Q_DECL_EXPORT
#endif
#ifndef Q_DECL_IMPORT
#  define Q_DECL_IMPORT
#endif
#ifndef Q_DECL_HIDDEN
#  define Q_DECL_HIDDEN
#endif
#ifndef Q_DECL_UNUSED
#  define Q_DECL_UNUSED
#endif
#ifndef Q_DECL_UNUSED_MEMBER
#  define Q_DECL_UNUSED_MEMBER
#endif
#ifndef Q_FUNC_INFO
#  if defined(Q_OS_SOLARIS) || defined(Q_CC_XLC)
#    define Q_FUNC_INFO __FILE__ "(line number unavailable)"
#  else
#    define Q_FUNC_INFO __FILE__ ":" QT_STRINGIFY(__LINE__)
#  endif
#endif
#ifndef Q_DECL_CF_RETURNS_RETAINED
#  define Q_DECL_CF_RETURNS_RETAINED
#endif
#ifndef Q_DECL_NS_RETURNS_AUTORELEASED
#  define Q_DECL_NS_RETURNS_AUTORELEASED
#endif
#ifndef Q_DECL_PURE_FUNCTION
#  define Q_DECL_PURE_FUNCTION
#endif
#ifndef Q_DECL_CONST_FUNCTION
#  define Q_DECL_CONST_FUNCTION Q_DECL_PURE_FUNCTION
#endif
#ifndef Q_DECL_COLD_FUNCTION
#  define Q_DECL_COLD_FUNCTION
#endif
#ifndef QT_MAKE_UNCHECKED_ARRAY_ITERATOR
#  define QT_MAKE_UNCHECKED_ARRAY_ITERATOR(x) (x)
#endif
#ifndef QT_MAKE_CHECKED_ARRAY_ITERATOR
#  define QT_MAKE_CHECKED_ARRAY_ITERATOR(x, N) (x)
#endif

/*
 * Warning/diagnostic handling
 */

#define QT_DO_PRAGMA(text)                      _Pragma(#text)
#if defined(Q_CC_INTEL) && defined(Q_CC_MSVC)
/* icl.exe: Intel compiler on Windows */
#  undef QT_DO_PRAGMA                           /* not needed */
#  define QT_WARNING_PUSH                       __pragma(warning(push))
#  define QT_WARNING_POP                        __pragma(warning(pop))
#  define QT_WARNING_DISABLE_MSVC(number)
#  define QT_WARNING_DISABLE_INTEL(number)      __pragma(warning(disable: number))
#  define QT_WARNING_DISABLE_CLANG(text)
#  define QT_WARNING_DISABLE_GCC(text)
#  define QT_WARNING_DISABLE_DEPRECATED         QT_WARNING_DISABLE_INTEL(1478 1786)
#elif defined(Q_CC_INTEL)
/* icc: Intel compiler on Linux or OS X */
#  define QT_WARNING_PUSH                       QT_DO_PRAGMA(warning(push))
#  define QT_WARNING_POP                        QT_DO_PRAGMA(warning(pop))
#  define QT_WARNING_DISABLE_INTEL(number)      QT_DO_PRAGMA(warning(disable: number))
#  define QT_WARNING_DISABLE_MSVC(number)
#  define QT_WARNING_DISABLE_CLANG(text)
#  define QT_WARNING_DISABLE_GCC(text)
#  define QT_WARNING_DISABLE_DEPRECATED         QT_WARNING_DISABLE_INTEL(1478 1786)
#elif defined(Q_CC_MSVC) && !defined(Q_CC_CLANG)
#  undef QT_DO_PRAGMA                           /* not needed */
#  define QT_WARNING_PUSH                       __pragma(warning(push))
#  define QT_WARNING_POP                        __pragma(warning(pop))
#  define QT_WARNING_DISABLE_MSVC(number)       __pragma(warning(disable: number))
#  define QT_WARNING_DISABLE_INTEL(number)
#  define QT_WARNING_DISABLE_CLANG(text)
#  define QT_WARNING_DISABLE_GCC(text)
#  define QT_WARNING_DISABLE_DEPRECATED         QT_WARNING_DISABLE_MSVC(4996)
#elif defined(Q_CC_CLANG)
#  define QT_WARNING_PUSH                       QT_DO_PRAGMA(clang diagnostic push)
#  define QT_WARNING_POP                        QT_DO_PRAGMA(clang diagnostic pop)
#  define QT_WARNING_DISABLE_CLANG(text)        QT_DO_PRAGMA(clang diagnostic ignored text)
#  define QT_WARNING_DISABLE_GCC(text)
#  define QT_WARNING_DISABLE_INTEL(number)
#  define QT_WARNING_DISABLE_MSVC(number)
#  define QT_WARNING_DISABLE_DEPRECATED         QT_WARNING_DISABLE_CLANG("-Wdeprecated-declarations")
#elif defined(Q_CC_GNU) && (__GNUC__ * 100 + __GNUC_MINOR__ >= 406)
#  define QT_WARNING_PUSH                       QT_DO_PRAGMA(GCC diagnostic push)
#  define QT_WARNING_POP                        QT_DO_PRAGMA(GCC diagnostic pop)
#  define QT_WARNING_DISABLE_GCC(text)          QT_DO_PRAGMA(GCC diagnostic ignored text)
#  define QT_WARNING_DISABLE_CLANG(text)
#  define QT_WARNING_DISABLE_INTEL(number)
#  define QT_WARNING_DISABLE_MSVC(number)
#  define QT_WARNING_DISABLE_DEPRECATED         QT_WARNING_DISABLE_GCC("-Wdeprecated-declarations")
#else       // All other compilers, GCC < 4.6 and MSVC < 2008
#  define QT_WARNING_DISABLE_GCC(text)
#  define QT_WARNING_PUSH
#  define QT_WARNING_POP
#  define QT_WARNING_DISABLE_INTEL(number)
#  define QT_WARNING_DISABLE_MSVC(number)
#  define QT_WARNING_DISABLE_CLANG(text)
#  define QT_WARNING_DISABLE_GCC(text)
#  define QT_WARNING_DISABLE_DEPRECATED
#endif

#ifndef QT_IGNORE_DEPRECATIONS
#define QT_IGNORE_DEPRECATIONS(statement) \
    QT_WARNING_PUSH \
    QT_WARNING_DISABLE_DEPRECATED \
    statement \
    QT_WARNING_POP
#endif

/*
   Proper for-scoping in MIPSpro CC
*/
#ifndef QT_NO_KEYWORDS
#  if defined(Q_CC_MIPS) || (defined(Q_CC_HPACC) && defined(__ia64))
#    define for if (0) {} else for
#  endif
#endif

#ifdef Q_COMPILER_RVALUE_REFS
#define qMove(x) std::move(x)
#else
#define qMove(x) (x)
#endif

#define Q_UNREACHABLE() \
    do {\
        Q_ASSERT_X(false, "Q_UNREACHABLE()", "Q_UNREACHABLE was reached");\
        Q_UNREACHABLE_IMPL();\
    } while (false)

#define Q_ASSUME(Expr) \
    do {\
        const bool valueOfExpression = Expr;\
        Q_ASSERT_X(valueOfExpression, "Q_ASSUME()", "Assumption in Q_ASSUME(\"" #Expr "\") was not correct");\
        Q_ASSUME_IMPL(valueOfExpression);\
    } while (false)

#if defined(__cplusplus)
#if __has_cpp_attribute(clang::fallthrough)
#    define Q_FALLTHROUGH() [[clang::fallthrough]]
#elif __has_cpp_attribute(gnu::fallthrough)
#    define Q_FALLTHROUGH() [[gnu::fallthrough]]
#elif __has_cpp_attribute(fallthrough)
#  define Q_FALLTHROUGH() [[fallthrough]]
#endif
#endif
#ifndef Q_FALLTHROUGH
#  if (defined(Q_CC_GNU) && Q_CC_GNU >= 700) && !defined(Q_CC_INTEL)
#    define Q_FALLTHROUGH() __attribute__((fallthrough))
#  else
#    define Q_FALLTHROUGH() (void)0
#endif
#endif


/*
    Sanitize compiler feature availability
*/
#if !defined(Q_PROCESSOR_X86)
#  undef QT_COMPILER_SUPPORTS_SSE2
#  undef QT_COMPILER_SUPPORTS_SSE3
#  undef QT_COMPILER_SUPPORTS_SSSE3
#  undef QT_COMPILER_SUPPORTS_SSE4_1
#  undef QT_COMPILER_SUPPORTS_SSE4_2
#  undef QT_COMPILER_SUPPORTS_AVX
#  undef QT_COMPILER_SUPPORTS_AVX2
#endif
#if !defined(Q_PROCESSOR_ARM)
#  undef QT_COMPILER_SUPPORTS_NEON
#endif
#if !defined(Q_PROCESSOR_MIPS)
#  undef QT_COMPILER_SUPPORTS_MIPS_DSP
#  undef QT_COMPILER_SUPPORTS_MIPS_DSPR2
#endif

#endif // QCOMPILERDETECTION_H
