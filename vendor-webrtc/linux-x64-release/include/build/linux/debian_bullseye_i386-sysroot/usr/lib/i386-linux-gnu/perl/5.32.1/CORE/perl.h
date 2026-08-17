/*    perl.h
 *
 *    Copyright (C) 1993, 1994, 1995, 1996, 1997, 1998, 1999, 2000, 2001
 *    2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#ifndef H_PERL
#define H_PERL 1

#ifdef PERL_FOR_X2P
/*
 * This file is being used for x2p stuff.
 * Above symbol is defined via -D in 'x2p/Makefile.SH'
 * Decouple x2p stuff from some of perls more extreme eccentricities.
 */
#undef MULTIPLICITY
#undef USE_STDIO
#define USE_STDIO
#endif /* PERL_FOR_X2P */

#ifdef PERL_MICRO
#   include "uconfig.h"
#else
#   include "config.h"
#endif

/* this is used for functions which take a depth trailing
 * argument under debugging */
#ifdef DEBUGGING
#define _pDEPTH ,U32 depth
#define _aDEPTH ,depth
#else
#define _pDEPTH
#define _aDEPTH
#endif

/* NOTE 1: that with gcc -std=c89 the __STDC_VERSION__ is *not* defined
 * because the __STDC_VERSION__ became a thing only with C90.  Therefore,
 * with gcc, HAS_C99 will never become true as long as we use -std=c89.

 * NOTE 2: headers lie.  Do not expect that if HAS_C99 gets to be true,
 * all the C99 features are there and are correct. */
#if (defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L) || \
    defined(_STDC_C99) || defined(__c99)
#  define HAS_C99 1
#endif

/* See L<perlguts/"The Perl API"> for detailed notes on
 * PERL_IMPLICIT_CONTEXT and PERL_IMPLICIT_SYS */

/* XXX NOTE that from here --> to <-- the same logic is
 * repeated in makedef.pl, so be certain to update
 * both places when editing. */

#ifdef USE_ITHREADS
#  if !defined(MULTIPLICITY)
#    define MULTIPLICITY
#  endif
#endif

#ifdef PERL_GLOBAL_STRUCT_PRIVATE
#  ifndef PERL_GLOBAL_STRUCT
#    define PERL_GLOBAL_STRUCT
#  endif
#endif

#ifdef PERL_GLOBAL_STRUCT
#  ifndef MULTIPLICITY
#    define MULTIPLICITY
#  endif
#endif

#ifdef MULTIPLICITY
#  ifndef PERL_IMPLICIT_CONTEXT
#    define PERL_IMPLICIT_CONTEXT
#  endif
#endif

/* undef WIN32 when building on Cygwin (for libwin32) - gph */
#ifdef __CYGWIN__
#   undef WIN32
#   undef _WIN32
#endif

#if defined(__SYMBIAN32__) || (defined(__VC32__) && defined(WINS))
#   ifndef SYMBIAN
#       define SYMBIAN
#   endif
#endif

#ifdef __SYMBIAN32__
#  include "symbian/symbian_proto.h"
#endif

/* Any stack-challenged places.  The limit varies (and often
 * is configurable), but using more than a kilobyte of stack
 * is usually dubious in these systems. */
#if defined(__SYMBIAN32__)
/* Symbian: need to work around the SDK features. *
 * On WINS: MS VC5 generates calls to _chkstk,         *
 * if a "large" stack frame is allocated.              *
 * gcc on MARM does not generate calls like these.     */
#   define USE_HEAP_INSTEAD_OF_STACK
#endif

/* Use the reentrant APIs like localtime_r and getpwent_r */
/* Win32 has naturally threadsafe libraries, no need to use any _r variants.
 * XXX KEEP makedef.pl copy of this code in sync */
#if defined(USE_ITHREADS) && !defined(USE_REENTRANT_API) && !defined(NETWARE) && !defined(WIN32)
#   define USE_REENTRANT_API
#endif

/* <--- here ends the logic shared by perl.h and makedef.pl */

#undef START_EXTERN_C
#undef END_EXTERN_C
#undef EXTERN_C
#ifdef __cplusplus
#  define START_EXTERN_C extern "C" {
#  define END_EXTERN_C }
#  define EXTERN_C extern "C"
#else
#  define START_EXTERN_C
#  define END_EXTERN_C
#  define EXTERN_C extern
#endif

/* Fallback definitions in case we don't have definitions from config.h.
   This should only matter for systems that don't use Configure and
   haven't been modified to define PERL_STATIC_INLINE yet.
*/
#if !defined(PERL_STATIC_INLINE)
#  ifdef HAS_STATIC_INLINE
#    define PERL_STATIC_INLINE static inline
#  else
#    define PERL_STATIC_INLINE static
#  endif
#endif

#if defined(PERL_GLOBAL_STRUCT) && !defined(PERL_GET_VARS)
#    ifdef PERL_GLOBAL_STRUCT_PRIVATE
       EXTERN_C struct perl_vars* Perl_GetVarsPrivate();
#      define PERL_GET_VARS() Perl_GetVarsPrivate() /* see miniperlmain.c */
#    else
#      define PERL_GET_VARS() PL_VarsPtr
#    endif
#endif

/* this used to be off by default, now its on, see perlio.h */
#define PERLIO_FUNCS_CONST

#define pVAR    struct perl_vars* my_vars PERL_UNUSED_DECL

#ifdef PERL_GLOBAL_STRUCT
#  define dVAR		pVAR    = (struct perl_vars*)PERL_GET_VARS()
#else
#  define dVAR		dNOOP
#endif

#ifdef PERL_IMPLICIT_CONTEXT
#  ifndef MULTIPLICITY
#    define MULTIPLICITY
#  endif
#  define tTHX	PerlInterpreter*
#  define pTHX  tTHX my_perl PERL_UNUSED_DECL
#  define aTHX	my_perl
#  define aTHXa(a) aTHX = (tTHX)a
#  ifdef PERL_GLOBAL_STRUCT
#    define dTHXa(a)	dVAR; pTHX = (tTHX)a
#  else
#    define dTHXa(a)	pTHX = (tTHX)a
#  endif
#  ifdef PERL_GLOBAL_STRUCT
#    define dTHX		dVAR; pTHX = PERL_GET_THX
#  else
#    define dTHX		pTHX = PERL_GET_THX
#  endif
#  define pTHX_		pTHX,
#  define aTHX_		aTHX,
#  define pTHX_1	2
#  define pTHX_2	3
#  define pTHX_3	4
#  define pTHX_4	5
#  define pTHX_5	6
#  define pTHX_6	7
#  define pTHX_7	8
#  define pTHX_8	9
#  define pTHX_9	10
#  define pTHX_12	13
#  if defined(DEBUGGING) && !defined(PERL_TRACK_MEMPOOL)
#    define PERL_TRACK_MEMPOOL
#  endif
#else
#  undef PERL_TRACK_MEMPOOL
#endif

#ifdef DEBUGGING
#  define dTHX_DEBUGGING dTHX
#else
#  define dTHX_DEBUGGING dNOOP
#endif

#define STATIC static

#ifndef PERL_CORE
/* Do not use these macros. They were part of PERL_OBJECT, which was an
 * implementation of multiplicity using C++ objects. They have been left
 * here solely for the sake of XS code which has incorrectly
 * cargo-culted them.
 */
#define CPERLscope(x) x
#define CPERLarg void
#define CPERLarg_
#define _CPERLarg
#define PERL_OBJECT_THIS
#define _PERL_OBJECT_THIS
#define PERL_OBJECT_THIS_
#define CALL_FPTR(fptr) (*fptr)
#define MEMBER_TO_FPTR(name) name
#endif /* !PERL_CORE */

#define CALLRUNOPS  PL_runops

#define CALLREGCOMP(sv, flags) Perl_pregcomp(aTHX_ (sv),(flags))

#define CALLREGCOMP_ENG(prog, sv, flags) (prog)->comp(aTHX_ sv, flags)
#define CALLREGEXEC(prog,stringarg,strend,strbeg,minend,sv,data,flags) \
    RX_ENGINE(prog)->exec(aTHX_ (prog),(stringarg),(strend), \
        (strbeg),(minend),(sv),(data),(flags))
#define CALLREG_INTUIT_START(prog,sv,strbeg,strpos,strend,flags,data) \
    RX_ENGINE(prog)->intuit(aTHX_ (prog), (sv), (strbeg), (strpos), \
        (strend),(flags),(data))
#define CALLREG_INTUIT_STRING(prog) \
    RX_ENGINE(prog)->checkstr(aTHX_ (prog))

#define CALLREGFREE(prog) \
    Perl_pregfree(aTHX_ (prog))

#define CALLREGFREE_PVT(prog) \
    if(prog && RX_ENGINE(prog)) RX_ENGINE(prog)->rxfree(aTHX_ (prog))

#define CALLREG_NUMBUF_FETCH(rx,paren,usesv)                                \
    RX_ENGINE(rx)->numbered_buff_FETCH(aTHX_ (rx),(paren),(usesv))

#define CALLREG_NUMBUF_STORE(rx,paren,value) \
    RX_ENGINE(rx)->numbered_buff_STORE(aTHX_ (rx),(paren),(value))

#define CALLREG_NUMBUF_LENGTH(rx,sv,paren)                              \
    RX_ENGINE(rx)->numbered_buff_LENGTH(aTHX_ (rx),(sv),(paren))

#define CALLREG_NAMED_BUFF_FETCH(rx, key, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), (key), NULL, ((flags) | RXapif_FETCH))

#define CALLREG_NAMED_BUFF_STORE(rx, key, value, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), (key), (value), ((flags) | RXapif_STORE))

#define CALLREG_NAMED_BUFF_DELETE(rx, key, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx),(key), NULL, ((flags) | RXapif_DELETE))

#define CALLREG_NAMED_BUFF_CLEAR(rx, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), NULL, NULL, ((flags) | RXapif_CLEAR))

#define CALLREG_NAMED_BUFF_EXISTS(rx, key, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), (key), NULL, ((flags) | RXapif_EXISTS))

#define CALLREG_NAMED_BUFF_FIRSTKEY(rx, flags) \
    RX_ENGINE(rx)->named_buff_iter(aTHX_ (rx), NULL, ((flags) | RXapif_FIRSTKEY))

#define CALLREG_NAMED_BUFF_NEXTKEY(rx, lastkey, flags) \
    RX_ENGINE(rx)->named_buff_iter(aTHX_ (rx), (lastkey), ((flags) | RXapif_NEXTKEY))

#define CALLREG_NAMED_BUFF_SCALAR(rx, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), NULL, NULL, ((flags) | RXapif_SCALAR))

#define CALLREG_NAMED_BUFF_COUNT(rx) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), NULL, NULL, RXapif_REGNAMES_COUNT)

#define CALLREG_NAMED_BUFF_ALL(rx, flags) \
    RX_ENGINE(rx)->named_buff(aTHX_ (rx), NULL, NULL, flags)

#define CALLREG_PACKAGE(rx) \
    RX_ENGINE(rx)->qr_package(aTHX_ (rx))

#if defined(USE_ITHREADS)
#define CALLREGDUPE(prog,param) \
    Perl_re_dup(aTHX_ (prog),(param))

#define CALLREGDUPE_PVT(prog,param) \
    (prog ? RX_ENGINE(prog)->dupe(aTHX_ (prog),(param)) \
          : (REGEXP *)NULL)
#endif

/* some compilers impersonate gcc */
#if defined(__GNUC__) && !defined(__clang__) && !defined(__INTEL_COMPILER)
#  define PERL_IS_GCC 1
#endif

/* In case Configure was not used (we are using a "canned config"
 * such as Win32, or a cross-compilation setup, for example) try going
 * by the gcc major and minor versions.  One useful URL is
 * http://www.ohse.de/uwe/articles/gcc-attributes.html,
 * but contrary to this information warn_unused_result seems
 * not to be in gcc 3.3.5, at least. --jhi
 * Also, when building extensions with an installed perl, this allows
 * the user to upgrade gcc and get the right attributes, rather than
 * relying on the list generated at Configure time.  --AD
 * Set these up now otherwise we get confused when some of the <*thread.h>
 * includes below indirectly pull in <perlio.h> (which needs to know if we
 * have HASATTRIBUTE_FORMAT).
 */

#ifndef PERL_MICRO
#if defined __GNUC__ && !defined(__INTEL_COMPILER)
#  if __GNUC__ == 3 && __GNUC_MINOR__ >= 1 || __GNUC__ > 3 /* 3.1 -> */
#    define HASATTRIBUTE_DEPRECATED
#  endif
#  if __GNUC__ >= 3 /* 3.0 -> */ /* XXX Verify this version */
#    define HASATTRIBUTE_FORMAT
#    if defined __MINGW32__
#      define PRINTF_FORMAT_NULL_OK
#    endif
#  endif
#  if __GNUC__ >= 3 /* 3.0 -> */
#    define HASATTRIBUTE_MALLOC
#  endif
#  if __GNUC__ == 3 && __GNUC_MINOR__ >= 3 || __GNUC__ > 3 /* 3.3 -> */
#    define HASATTRIBUTE_NONNULL
#  endif
#  if __GNUC__ == 2 && __GNUC_MINOR__ >= 5 || __GNUC__ > 2 /* 2.5 -> */
#    define HASATTRIBUTE_NORETURN
#  endif
#  if __GNUC__ >= 3 /* gcc 3.0 -> */
#    define HASATTRIBUTE_PURE
#  endif
#  if __GNUC__ == 3 && __GNUC_MINOR__ >= 4 || __GNUC__ > 3 /* 3.4 -> */
#    define HASATTRIBUTE_UNUSED
#  endif
#  if __GNUC__ == 3 && __GNUC_MINOR__ == 3 && !defined(__cplusplus)
#    define HASATTRIBUTE_UNUSED /* gcc-3.3, but not g++-3.3. */
#  endif
#  if __GNUC__ == 3 && __GNUC_MINOR__ >= 4 || __GNUC__ > 3 /* 3.4 -> */
#    define HASATTRIBUTE_WARN_UNUSED_RESULT
#  endif
/* always_inline is buggy in gcc <= 4.6 and causes compilation errors */
#  if __GNUC__ == 4 && __GNUC_MINOR__ >= 7 || __GNUC__ > 4 /* 4.7 -> */
#    define HASATTRIBUTE_ALWAYS_INLINE
#  endif
#endif
#endif /* #ifndef PERL_MICRO */

#ifdef HASATTRIBUTE_DEPRECATED
#  define __attribute__deprecated__         __attribute__((deprecated))
#endif
#ifdef HASATTRIBUTE_FORMAT
#  define __attribute__format__(x,y,z)      __attribute__((format(x,y,z)))
#endif
#ifdef HASATTRIBUTE_MALLOC
#  define __attribute__malloc__             __attribute__((__malloc__))
#endif
#ifdef HASATTRIBUTE_NONNULL
#  define __attribute__nonnull__(a)         __attribute__((nonnull(a)))
#endif
#ifdef HASATTRIBUTE_NORETURN
#  define __attribute__noreturn__           __attribute__((noreturn))
#endif
#ifdef HASATTRIBUTE_PURE
#  define __attribute__pure__               __attribute__((pure))
#endif
#ifdef HASATTRIBUTE_UNUSED
#  define __attribute__unused__             __attribute__((unused))
#endif
#ifdef HASATTRIBUTE_WARN_UNUSED_RESULT
#  define __attribute__warn_unused_result__ __attribute__((warn_unused_result))
#endif
#ifdef HASATTRIBUTE_ALWAYS_INLINE
/* always_inline is buggy in gcc <= 4.6 and causes compilation errors */
#  if !defined(PERL_IS_GCC) || (__GNUC__ == 4 && __GNUC_MINOR__ >= 7 || __GNUC__ > 4)
#    define __attribute__always_inline__      __attribute__((always_inline))
#  endif
#endif

/* If we haven't defined the attributes yet, define them to blank. */
#ifndef __attribute__deprecated__
#  define __attribute__deprecated__
#endif
#ifndef __attribute__format__
#  define __attribute__format__(x,y,z)
#endif
#ifndef __attribute__malloc__
#  define __attribute__malloc__
#endif
#ifndef __attribute__nonnull__
#  define __attribute__nonnull__(a)
#endif
#ifndef __attribute__noreturn__
#  define __attribute__noreturn__
#endif
#ifndef __attribute__pure__
#  define __attribute__pure__
#endif
#ifndef __attribute__unused__
#  define __attribute__unused__
#endif
#ifndef __attribute__warn_unused_result__
#  define __attribute__warn_unused_result__
#endif
#ifndef __attribute__always_inline__
#  define __attribute__always_inline__
#endif

/* Some OS warn on NULL format to printf */
#ifdef PRINTF_FORMAT_NULL_OK
#  define __attribute__format__null_ok__(x,y,z)  __attribute__format__(x,y,z)
#else
#  define __attribute__format__null_ok__(x,y,z)
#endif

/*
 * Because of backward compatibility reasons the PERL_UNUSED_DECL
 * cannot be changed from postfix to PERL_UNUSED_DECL(x).  Sigh.
 *
 * Note that there are C compilers such as MetroWerks CodeWarrior
 * which do not have an "inlined" way (like the gcc __attribute__) of
 * marking unused variables (they need e.g. a #pragma) and therefore
 * cpp macros like PERL_UNUSED_DECL cannot work for this purpose, even
 * if it were PERL_UNUSED_DECL(x), which it cannot be (see above).
 *
 */

#ifndef PERL_UNUSED_DECL
#  define PERL_UNUSED_DECL __attribute__unused__
#endif

/* gcc -Wall:
 * for silencing unused variables that are actually used most of the time,
 * but we cannot quite get rid of, such as "ax" in PPCODE+noargs xsubs,
 * or variables/arguments that are used only in certain configurations.
 */
#ifndef PERL_UNUSED_ARG
#  define PERL_UNUSED_ARG(x) ((void)sizeof(x))
#endif
#ifndef PERL_UNUSED_VAR
#  define PERL_UNUSED_VAR(x) ((void)sizeof(x))
#endif

#if defined(USE_ITHREADS) || defined(PERL_GLOBAL_STRUCT)
#  define PERL_UNUSED_CONTEXT PERL_UNUSED_ARG(my_perl)
#else
#  define PERL_UNUSED_CONTEXT
#endif

/* gcc (-ansi) -pedantic doesn't allow gcc statement expressions,
 * g++ allows them but seems to have problems with them
 * (insane errors ensue).
 * g++ does not give insane errors now (RMB 2008-01-30, gcc 4.2.2).
 */
#if defined(PERL_GCC_PEDANTIC) || \
    (defined(__GNUC__) && defined(__cplusplus) && \
	((__GNUC__ < 4) || ((__GNUC__ == 4) && (__GNUC_MINOR__ < 2))))
#  ifndef PERL_GCC_BRACE_GROUPS_FORBIDDEN
#    define PERL_GCC_BRACE_GROUPS_FORBIDDEN
#  endif
#endif

/* Use PERL_UNUSED_RESULT() to suppress the warnings about unused results
 * of function calls, e.g. PERL_UNUSED_RESULT(foo(a, b)).
 *
 * The main reason for this is that the combination of gcc -Wunused-result
 * (part of -Wall) and the __attribute__((warn_unused_result)) cannot
 * be silenced with casting to void.  This causes trouble when the system
 * header files use the attribute.
 *
 * Use PERL_UNUSED_RESULT sparingly, though, since usually the warning
 * is there for a good reason: you might lose success/failure information,
 * or leak resources, or changes in resources.
 *
 * But sometimes you just want to ignore the return value, e.g. on
 * codepaths soon ending up in abort, or in "best effort" attempts,
 * or in situations where there is no good way to handle failures.
 *
 * Sometimes PERL_UNUSED_RESULT might not be the most natural way:
 * another possibility is that you can capture the return value
 * and use PERL_UNUSED_VAR on that.
 *
 * The __typeof__() is used instead of typeof() since typeof() is not
 * available under strict C89, and because of compilers masquerading
 * as gcc (clang and icc), we want exactly the gcc extension
 * __typeof__ and nothing else.
 */
#ifndef PERL_UNUSED_RESULT
#  if defined(__GNUC__) && defined(HASATTRIBUTE_WARN_UNUSED_RESULT)
#    define PERL_UNUSED_RESULT(v) STMT_START { __typeof__(v) z = (v); (void)sizeof(z); } STMT_END
#  else
#    define PERL_UNUSED_RESULT(v) ((void)(v))
#  endif
#endif

#if defined(_MSC_VER)
/* XXX older MSVC versions have a smallish macro buffer */
#define PERL_SMALL_MACRO_BUFFER
#endif

/* on gcc (and clang), specify that a warning should be temporarily
 * ignored; e.g.
 *
 *    GCC_DIAG_IGNORE_DECL(-Wmultichar);
 *    char b = 'ab';
 *    GCC_DIAG_RESTORE_DECL;
 *
 * based on http://dbp-consulting.com/tutorials/SuppressingGCCWarnings.html
 *
 * Note that "pragma GCC diagnostic push/pop" was added in GCC 4.6, Mar 2011;
 * clang only pretends to be GCC 4.2, but still supports push/pop.
 *
 * Note on usage: all macros must be used at a place where a declaration
 * or statement can occur, i.e., not in the middle of an expression.
 * *_DIAG_IGNORE() and *_DIAG_RESTORE can be used in any such place, but
 * must be used without a following semicolon.  *_DIAG_IGNORE_DECL() and
 * *_DIAG_RESTORE_DECL must be used with a following semicolon, and behave
 * syntactically as declarations (like dNOOP).  *_DIAG_IGNORE_STMT()
 * and *_DIAG_RESTORE_STMT must be used with a following semicolon,
 * and behave syntactically as statements (like NOOP).
 *
 */

#if defined(__clang__) || defined(__clang) || \
       (defined( __GNUC__) && ((__GNUC__ * 100) + __GNUC_MINOR__) >= 406)
#  define GCC_DIAG_PRAGMA(x) _Pragma (#x)
/* clang has "clang diagnostic" pragmas, but also understands gcc. */
#  define GCC_DIAG_IGNORE(x) _Pragma("GCC diagnostic push") \
                             GCC_DIAG_PRAGMA(GCC diagnostic ignored #x)
#  define GCC_DIAG_RESTORE   _Pragma("GCC diagnostic pop")
#else
#  define GCC_DIAG_IGNORE(w)
#  define GCC_DIAG_RESTORE
#endif
#define GCC_DIAG_IGNORE_DECL(x) GCC_DIAG_IGNORE(x) dNOOP
#define GCC_DIAG_RESTORE_DECL GCC_DIAG_RESTORE dNOOP
#define GCC_DIAG_IGNORE_STMT(x) GCC_DIAG_IGNORE(x) NOOP
#define GCC_DIAG_RESTORE_STMT GCC_DIAG_RESTORE NOOP
/* for clang specific pragmas */
#if defined(__clang__) || defined(__clang)
#  define CLANG_DIAG_PRAGMA(x) _Pragma (#x)
#  define CLANG_DIAG_IGNORE(x) _Pragma("clang diagnostic push") \
                               CLANG_DIAG_PRAGMA(clang diagnostic ignored #x)
#  define CLANG_DIAG_RESTORE   _Pragma("clang diagnostic pop")
#else
#  define CLANG_DIAG_IGNORE(w)
#  define CLANG_DIAG_RESTORE
#endif
#define CLANG_DIAG_IGNORE_DECL(x) CLANG_DIAG_IGNORE(x) dNOOP
#define CLANG_DIAG_RESTORE_DECL CLANG_DIAG_RESTORE dNOOP
#define CLANG_DIAG_IGNORE_STMT(x) CLANG_DIAG_IGNORE(x) NOOP
#define CLANG_DIAG_RESTORE_STMT CLANG_DIAG_RESTORE NOOP

#if defined(_MSC_VER) && (_MSC_VER >= 1300)
#  define MSVC_DIAG_IGNORE(x) __pragma(warning(push)) \
                              __pragma(warning(disable : x))
#  define MSVC_DIAG_RESTORE   __pragma(warning(pop))
#else
#  define MSVC_DIAG_IGNORE(x)
#  define MSVC_DIAG_RESTORE
#endif
#define MSVC_DIAG_IGNORE_DECL(x) MSVC_DIAG_IGNORE(x) dNOOP
#define MSVC_DIAG_RESTORE_DECL MSVC_DIAG_RESTORE dNOOP
#define MSVC_DIAG_IGNORE_STMT(x) MSVC_DIAG_IGNORE(x) NOOP
#define MSVC_DIAG_RESTORE_STMT MSVC_DIAG_RESTORE NOOP

#define NOOP /*EMPTY*/(void)0
#define dNOOP struct Perl___notused_struct

#ifndef pTHX
/* Don't bother defining tTHX ; using it outside
 * code guarded by PERL_IMPLICIT_CONTEXT is an error.
 */
#  define pTHX		void
#  define pTHX_
#  define aTHX
#  define aTHX_
#  define aTHXa(a)      NOOP
#  define dTHXa(a)	dNOOP
#  define dTHX		dNOOP
#  define pTHX_1	1	
#  define pTHX_2	2
#  define pTHX_3	3
#  define pTHX_4	4
#  define pTHX_5	5
#  define pTHX_6	6
#  define pTHX_7	7
#  define pTHX_8	8
#  define pTHX_9	9
#  define pTHX_12	12
#endif

#ifndef dVAR
#  define dVAR		dNOOP
#endif

/* these are only defined for compatibility; should not be used internally */
#if !defined(pTHXo) && !defined(PERL_CORE)
#  define pTHXo		pTHX
#  define pTHXo_	pTHX_
#  define aTHXo		aTHX
#  define aTHXo_	aTHX_
#  define dTHXo		dTHX
#  define dTHXoa(x)	dTHXa(x)
#endif

#ifndef pTHXx
#  define pTHXx		PerlInterpreter *my_perl
#  define pTHXx_	pTHXx,
#  define aTHXx		my_perl
#  define aTHXx_	aTHXx,
#  define dTHXx		dTHX
#endif

/* Under PERL_IMPLICIT_SYS (used in Windows for fork emulation)
 * PerlIO_foo() expands to PL_StdIO->pFOO(PL_StdIO, ...).
 * dTHXs is therefore needed for all functions using PerlIO_foo(). */
#ifdef PERL_IMPLICIT_SYS
#  ifdef PERL_GLOBAL_STRUCT_PRIVATE
#    define dTHXs		dVAR; dTHX
#  else
#    define dTHXs		dTHX
#  endif
#else
#  ifdef PERL_GLOBAL_STRUCT_PRIVATE
#    define dTHXs		dVAR
#  else
#    define dTHXs		dNOOP
#  endif
#endif

#if defined(__GNUC__) && !defined(PERL_GCC_BRACE_GROUPS_FORBIDDEN) && !defined(__cplusplus)
#  ifndef PERL_USE_GCC_BRACE_GROUPS
#    define PERL_USE_GCC_BRACE_GROUPS
#  endif
#endif

/*
=head1 Miscellaneous Functions

=for apidoc AmnUu|void|STMT_START

 STMT_START { statements; } STMT_END;

can be used as a single statement, as in

 if (x) STMT_START { ... } STMT_END; else ...

These are often used in macro definitions.  Note that you can't return a value
out of them.

=for apidoc AmnUhu|void|STMT_END

=cut

 Trying to select a version that gives no warnings...
*/
#if !(defined(STMT_START) && defined(STMT_END))
# ifdef PERL_USE_GCC_BRACE_GROUPS
#   define STMT_START	(void)(	/* gcc supports "({ STATEMENTS; })" */
#   define STMT_END	)
# else
#   define STMT_START	do
#   define STMT_END	while (0)
# endif
#endif

#ifndef BYTEORDER  /* Should never happen -- byteorder is in config.h */
#   define BYTEORDER 0x1234
#endif

#if 'A' == 65 && 'I' == 73 && 'J' == 74 && 'Z' == 90
#define ASCIIish
#else
#undef  ASCIIish
#endif

/*
 * The following contortions are brought to you on behalf of all the
 * standards, semi-standards, de facto standards, not-so-de-facto standards
 * of the world, as well as all the other botches anyone ever thought of.
 * The basic theory is that if we work hard enough here, the rest of the
 * code can be a lot prettier.  Well, so much for theory.  Sorry, Henry...
 */

/* define this once if either system, instead of cluttering up the src */
#if defined(MSDOS) || defined(WIN32) || defined(NETWARE)
#define DOSISH 1
#endif

/* These exist only for back-compat with XS modules. */
#ifndef PERL_CORE
#define VOL volatile
#define CAN_PROTOTYPE
#define _(args) args
#define I_LIMITS
#define I_STDARG
#define STANDARD_C
#endif

/* By compiling a perl with -DNO_TAINT_SUPPORT or -DSILENT_NO_TAINT_SUPPORT,
 * you get a perl without taint support, but doubtlessly with a lesser
 * degree of support. Do not do so unless you know exactly what it means
 * technically, have a good reason to do so, and know exactly how the
 * perl will be used. perls with -DSILENT_NO_TAINT_SUPPORT are considered
 * a potential security risk due to flat out ignoring the security-relevant
 * taint flags. This being said, a perl without taint support compiled in
 * has marginal run-time performance benefits.
 * SILENT_NO_TAINT_SUPPORT implies NO_TAINT_SUPPORT.
 * SILENT_NO_TAINT_SUPPORT is the same as NO_TAINT_SUPPORT except it
 * silently ignores -t/-T instead of throwing an exception.
 *
 * DANGER! Using NO_TAINT_SUPPORT or SILENT_NO_TAINT_SUPPORT
 *         voids your nonexistent warranty!
 */
#if defined(SILENT_NO_TAINT_SUPPORT) && !defined(NO_TAINT_SUPPORT)
#  define NO_TAINT_SUPPORT 1
#endif

/* NO_TAINT_SUPPORT can be set to transform virtually all taint-related
 * operations into no-ops for a very modest speed-up. Enable only if you
 * know what you're doing: tests and CPAN modules' tests are bound to fail.
 */
#ifdef NO_TAINT_SUPPORT
#   define TAINT		NOOP
#   define TAINT_NOT		NOOP
#   define TAINT_IF(c)		NOOP
#   define TAINT_ENV()		NOOP
#   define TAINT_PROPER(s)	NOOP
#   define TAINT_set(s)		NOOP
#   define TAINT_get		0
#   define TAINTING_get		0
#   define TAINTING_set(s)	NOOP
#   define TAINT_WARN_get       0
#   define TAINT_WARN_set(s)    NOOP
#else
    /* Set to tainted if we are running under tainting mode */
#   define TAINT		(PL_tainted = PL_tainting)

#   define TAINT_NOT	(PL_tainted = FALSE)        /* Untaint */
#   define TAINT_IF(c)	if (UNLIKELY(c)) { TAINT; } /* Conditionally taint */
#   define TAINT_ENV()	if (UNLIKELY(PL_tainting)) { taint_env(); }
                                /* croak or warn if tainting */
#   define TAINT_PROPER(s)	if (UNLIKELY(PL_tainting)) {                \
                                    taint_proper(NULL, s);                  \
                                }
#   define TAINT_set(s)		(PL_tainted = (s))
#   define TAINT_get		(cBOOL(UNLIKELY(PL_tainted)))    /* Is something tainted? */
#   define TAINTING_get		(cBOOL(UNLIKELY(PL_tainting)))   /* Is taint checking enabled? */
#   define TAINTING_set(s)	(PL_tainting = (s))
#   define TAINT_WARN_get       (PL_taint_warn) /* FALSE => tainting violations
                                                            are fatal
                                                   TRUE =>  they're just
                                                            warnings */
#   define TAINT_WARN_set(s)    (PL_taint_warn = (s))
#endif

/* flags used internally only within pp_subst and pp_substcont */
#ifdef PERL_CORE
#  define SUBST_TAINT_STR      1	/* string tainted */
#  define SUBST_TAINT_PAT      2	/* pattern tainted */
#  define SUBST_TAINT_REPL     4	/* replacement tainted */
#  define SUBST_TAINT_RETAINT  8	/* use re'taint' in scope */
#  define SUBST_TAINT_BOOLRET 16	/* return is boolean (don't taint) */
#endif

/* XXX All process group stuff is handled in pp_sys.c.  Should these
   defines move there?  If so, I could simplify this a lot. --AD  9/96.
*/
/* Process group stuff changed from traditional BSD to POSIX.
   perlfunc.pod documents the traditional BSD-style syntax, so we'll
   try to preserve that, if possible.
*/
#ifdef HAS_SETPGID
#  define BSD_SETPGRP(pid, pgrp)	setpgid((pid), (pgrp))
#elif defined(HAS_SETPGRP) && defined(USE_BSD_SETPGRP)
#  define BSD_SETPGRP(pid, pgrp)	setpgrp((pid), (pgrp))
#elif defined(HAS_SETPGRP2)
#  define BSD_SETPGRP(pid, pgrp)	setpgrp2((pid), (pgrp))
#endif
#if defined(BSD_SETPGRP) && !defined(HAS_SETPGRP)
#  define HAS_SETPGRP  /* Well, effectively it does . . . */
#endif

/* getpgid isn't POSIX, but at least Solaris and Linux have it, and it makes
    our life easier :-) so we'll try it.
*/
#ifdef HAS_GETPGID
#  define BSD_GETPGRP(pid)		getpgid((pid))
#elif defined(HAS_GETPGRP) && defined(USE_BSD_GETPGRP)
#  define BSD_GETPGRP(pid)		getpgrp((pid))
#elif defined(HAS_GETPGRP2)
#  define BSD_GETPGRP(pid)		getpgrp2((pid))
#endif
#if defined(BSD_GETPGRP) && !defined(HAS_GETPGRP)
#  define HAS_GETPGRP  /* Well, effectively it does . . . */
#endif

/* These are not exact synonyms, since setpgrp() and getpgrp() may
   have different behaviors, but perl.h used to define USE_BSDPGRP
   (prior to 5.003_05) so some extension might depend on it.
*/
#if defined(USE_BSD_SETPGRP) || defined(USE_BSD_GETPGRP)
#  ifndef USE_BSDPGRP
#    define USE_BSDPGRP
#  endif
#endif

/* HP-UX 10.X CMA (Common Multithreaded Architecture) insists that
   pthread.h must be included before all other header files.
*/
#if defined(USE_ITHREADS) && defined(PTHREAD_H_FIRST) && defined(I_PTHREAD)
#  include <pthread.h>
#endif

#include <sys/types.h>

/* EVC 4 SDK headers includes a bad definition of MB_CUR_MAX in stdlib.h
  which is included from stdarg.h. Bad definition not present in SD 2008
  SDK headers. wince.h is not yet included, so we cant fix this from there
  since by then MB_CUR_MAX will be defined from stdlib.h.
  cewchar.h includes a correct definition of MB_CUR_MAX and it is copied here
  since cewchar.h can't be included this early */
#if defined(UNDER_CE) && (_MSC_VER < 1300)
#  define MB_CUR_MAX 1uL
#endif

#  ifdef I_WCHAR
#    include <wchar.h>
#  endif

# include <stdarg.h>

#ifdef I_STDINT
# include <stdint.h>
#endif

#include <ctype.h>
#include <float.h>
#include <limits.h>

#ifdef METHOD 	/* Defined by OSF/1 v3.0 by ctype.h */
#undef METHOD
#endif

#ifdef PERL_MICRO
#   define NO_LOCALE
#endif

#ifdef I_LOCALE
#   include <locale.h>
#endif

#ifdef I_XLOCALE
#   include <xlocale.h>
#endif

/* If not forbidden, we enable locale handling if either 1) the POSIX 2008
 * functions are available, or 2) just the setlocale() function.  This logic is
 * repeated in t/loc_tools.pl and makedef.pl;  The three should be kept in
 * sync. */
#if   ! defined(NO_LOCALE)

#  if ! defined(NO_POSIX_2008_LOCALE)           \
   &&   defined(HAS_NEWLOCALE)                  \
   &&   defined(HAS_USELOCALE)                  \
   &&   defined(HAS_DUPLOCALE)                  \
   &&   defined(HAS_FREELOCALE)                 \
   &&   defined(LC_ALL_MASK)

    /* For simplicity, the code is written to assume that any platform advanced
     * enough to have the Posix 2008 locale functions has LC_ALL.  The final
     * test above makes sure that assumption is valid */

#    define HAS_POSIX_2008_LOCALE
#    define USE_LOCALE
#  elif defined(HAS_SETLOCALE)
#    define USE_LOCALE
#  endif
#endif

#ifdef USE_LOCALE
#   define HAS_SKIP_LOCALE_INIT /* Solely for XS code to test for this
                                   #define */
#   if !defined(NO_LOCALE_COLLATE) && defined(LC_COLLATE) \
       && defined(HAS_STRXFRM)
#	define USE_LOCALE_COLLATE
#   endif
#   if !defined(NO_LOCALE_CTYPE) && defined(LC_CTYPE)
#	define USE_LOCALE_CTYPE
#   endif
#   if !defined(NO_LOCALE_NUMERIC) && defined(LC_NUMERIC)
#	define USE_LOCALE_NUMERIC
#   endif
#   if !defined(NO_LOCALE_MESSAGES) && defined(LC_MESSAGES)
#	define USE_LOCALE_MESSAGES
#   endif
#   if !defined(NO_LOCALE_MONETARY) && defined(LC_MONETARY)
#	define USE_LOCALE_MONETARY
#   endif
#   if !defined(NO_LOCALE_TIME) && defined(LC_TIME)
#	define USE_LOCALE_TIME
#   endif
#   if !defined(NO_LOCALE_ADDRESS) && defined(LC_ADDRESS)
#	define USE_LOCALE_ADDRESS
#   endif
#   if !defined(NO_LOCALE_IDENTIFICATION) && defined(LC_IDENTIFICATION)
#	define USE_LOCALE_IDENTIFICATION
#   endif
#   if !defined(NO_LOCALE_MEASUREMENT) && defined(LC_MEASUREMENT)
#	define USE_LOCALE_MEASUREMENT
#   endif
#   if !defined(NO_LOCALE_PAPER) && defined(LC_PAPER)
#	define USE_LOCALE_PAPER
#   endif
#   if !defined(NO_LOCALE_TELEPHONE) && defined(LC_TELEPHONE)
#	define USE_LOCALE_TELEPHONE
#   endif

/* XXX The next few defines are unfortunately duplicated in makedef.pl, and
 * changes here MUST also be made there */

#  if ! defined(HAS_SETLOCALE) && defined(HAS_POSIX_2008_LOCALE)
#      define USE_POSIX_2008_LOCALE
#      ifndef USE_THREAD_SAFE_LOCALE
#        define USE_THREAD_SAFE_LOCALE
#      endif
                                   /* If compiled with
                                    * -DUSE_THREAD_SAFE_LOCALE, will do so even
                                    * on unthreaded builds */
#  elif   (defined(USE_ITHREADS) || defined(USE_THREAD_SAFE_LOCALE))         \
       && (    defined(HAS_POSIX_2008_LOCALE)                                \
           || (defined(WIN32) && defined(_MSC_VER) && _MSC_VER >= 1400))     \
       && ! defined(NO_THREAD_SAFE_LOCALE)
#    ifndef USE_THREAD_SAFE_LOCALE
#      define USE_THREAD_SAFE_LOCALE
#    endif
#    ifdef HAS_POSIX_2008_LOCALE
#      define USE_POSIX_2008_LOCALE
#    endif
#  endif
#endif

/*  Microsoft documentation reads in the change log for VS 2015:
 *     "The localeconv function declared in locale.h now works correctly when
 *     per-thread locale is enabled. In previous versions of the library, this
 *     function would return the lconv data for the global locale, not the
 *     thread's locale."
 */
#if defined(WIN32) && defined(USE_THREAD_SAFE_LOCALE) && _MSC_VER < 1900
#  define TS_W32_BROKEN_LOCALECONV
#endif

#include <setjmp.h>

#ifdef I_SYS_PARAM
#   ifdef PARAM_NEEDS_TYPES
#	include <sys/types.h>
#   endif
#   include <sys/param.h>
#endif

/* On BSD-derived systems, <sys/param.h> defines BSD to a year-month
   value something like 199306.  This may be useful if no more-specific
   feature test is available.
*/
#if defined(BSD)
#   ifndef BSDish
#       define BSDish
#   endif
#endif

/* Use all the "standard" definitions */
#include <stdlib.h>

/* If this causes problems, set i_unistd=undef in the hint file.  */
#ifdef I_UNISTD
#    if defined(__amigaos4__)
#        ifdef I_NETINET_IN
#            include <netinet/in.h>
#        endif
#   endif
#   include <unistd.h>
#   if defined(__amigaos4__)
/* Under AmigaOS 4 newlib.library provides an environ.  However using
 * it doesn't give us enough control over inheritance of variables by
 * subshells etc. so replace with custom version based on abc-shell
 * code. */
extern char **myenviron;
#       undef environ
#       define environ myenviron
#   endif
#endif

/* for WCOREDUMP */
#ifdef I_SYS_WAIT
#   include <sys/wait.h>
#endif

#ifdef __SYMBIAN32__
#   undef _SC_ARG_MAX /* Symbian has _SC_ARG_MAX but no sysconf() */
#endif

#if defined(HAS_SYSCALL) && !defined(HAS_SYSCALL_PROTO)
EXTERN_C int syscall(int, ...);
#endif

#if defined(HAS_USLEEP) && !defined(HAS_USLEEP_PROTO)
EXTERN_C int usleep(unsigned int);
#endif

/* macros for correct constant construction.  These are in C99 <stdint.h>
 * (so they will not be available in strict C89 mode), but they are nice, so
 * let's define them if necessary. */
#ifndef UINT16_C
#  if INTSIZE >= 2
#    define UINT16_C(x) ((U16_TYPE)x##U)
#  else
#    define UINT16_C(x) ((U16_TYPE)x##UL)
#  endif
#endif

#ifndef UINT32_C
#  if INTSIZE >= 4
#    define UINT32_C(x) ((U32_TYPE)x##U)
#  else
#    define UINT32_C(x) ((U32_TYPE)x##UL)
#  endif
#endif

#ifdef I_STDINT
    typedef intmax_t  PERL_INTMAX_T;
    typedef uintmax_t PERL_UINTMAX_T;
#endif

/* N.B.  We use QUADKIND here instead of HAS_QUAD here, because that doesn't
 * actually mean what it has always been documented to mean (see RT #119753)
 * and is explicitly turned off outside of core with dire warnings about
 * removing the undef. */

#if defined(QUADKIND)
#  undef PeRl_INT64_C
#  undef PeRl_UINT64_C
/* Prefer the native integer types (int and long) over long long
 * (which is not C89) and Win32-specific __int64. */
#  if QUADKIND == QUAD_IS_INT && INTSIZE == 8
#    define PeRl_INT64_C(c)	(c)
#    define PeRl_UINT64_C(c)	CAT2(c,U)
#  endif
#  if QUADKIND == QUAD_IS_LONG && LONGSIZE == 8
#    define PeRl_INT64_C(c)	CAT2(c,L)
#    define PeRl_UINT64_C(c)	CAT2(c,UL)
#  endif
#  if QUADKIND == QUAD_IS_LONG_LONG && defined(HAS_LONG_LONG)
#    define PeRl_INT64_C(c)	CAT2(c,LL)
#    define PeRl_UINT64_C(c)	CAT2(c,ULL)
#  endif
#  if QUADKIND == QUAD_IS___INT64
#    define PeRl_INT64_C(c)	CAT2(c,I64)
#    define PeRl_UINT64_C(c)	CAT2(c,UI64)
#  endif
#  ifndef PeRl_INT64_C
#    define PeRl_INT64_C(c)	((I64)(c)) /* last resort */
#    define PeRl_UINT64_C(c)	((U64TYPE)(c))
#  endif
/* In OS X the INT64_C/UINT64_C are defined with LL/ULL, which will
 * not fly with C89-pedantic gcc, so let's undefine them first so that
 * we can redefine them with our native integer preferring versions. */
#  if defined(PERL_DARWIN) && defined(PERL_GCC_PEDANTIC)
#    undef INT64_C
#    undef UINT64_C
#  endif
#  ifndef INT64_C
#    define INT64_C(c) PeRl_INT64_C(c)
#  endif
#  ifndef UINT64_C
#    define UINT64_C(c) PeRl_UINT64_C(c)
#  endif

#  ifndef I_STDINT
    typedef I64TYPE PERL_INTMAX_T;
    typedef U64TYPE PERL_UINTMAX_T;
#  endif
#  ifndef INTMAX_C
#    define INTMAX_C(c) INT64_C(c)
#  endif
#  ifndef UINTMAX_C
#    define UINTMAX_C(c) UINT64_C(c)
#  endif

#else  /* below QUADKIND is undefined */

/* Perl doesn't work on 16 bit systems, so must be 32 bit */
#  ifndef I_STDINT
    typedef I32TYPE PERL_INTMAX_T;
    typedef U32TYPE PERL_UINTMAX_T;
#  endif
#  ifndef INTMAX_C
#    define INTMAX_C(c) INT32_C(c)
#  endif
#  ifndef UINTMAX_C
#    define UINTMAX_C(c) UINT32_C(c)
#  endif

#endif  /* no QUADKIND */

#ifdef PERL_CORE

/* byte-swapping functions for big-/little-endian conversion */
# define _swab_16_(x) ((U16)( \
         (((U16)(x) & UINT16_C(0x00ff)) << 8) | \
         (((U16)(x) & UINT16_C(0xff00)) >> 8) ))

# define _swab_32_(x) ((U32)( \
         (((U32)(x) & UINT32_C(0x000000ff)) << 24) | \
         (((U32)(x) & UINT32_C(0x0000ff00)) <<  8) | \
         (((U32)(x) & UINT32_C(0x00ff0000)) >>  8) | \
         (((U32)(x) & UINT32_C(0xff000000)) >> 24) ))

# ifdef HAS_QUAD
#  define _swab_64_(x) ((U64)( \
          (((U64)(x) & UINT64_C(0x00000000000000ff)) << 56) | \
          (((U64)(x) & UINT64_C(0x000000000000ff00)) << 40) | \
          (((U64)(x) & UINT64_C(0x0000000000ff0000)) << 24) | \
          (((U64)(x) & UINT64_C(0x00000000ff000000)) <<  8) | \
          (((U64)(x) & UINT64_C(0x000000ff00000000)) >>  8) | \
          (((U64)(x) & UINT64_C(0x0000ff0000000000)) >> 24) | \
          (((U64)(x) & UINT64_C(0x00ff000000000000)) >> 40) | \
          (((U64)(x) & UINT64_C(0xff00000000000000)) >> 56) ))
# endif

/* The old value was hard coded at 1008. (4096-16) seems to be a bit faster,
   at least on FreeBSD.  YMMV, so experiment.  */
#ifndef PERL_ARENA_SIZE
#define PERL_ARENA_SIZE 4080
#endif

/* Maximum level of recursion */
#ifndef PERL_SUB_DEPTH_WARN
#define PERL_SUB_DEPTH_WARN 100
#endif

#endif /* PERL_CORE */

/* Maximum number of args that may be passed to an OP_MULTICONCAT op.
 * It determines the size of local arrays in S_maybe_multiconcat() and
 * pp_multiconcat().
 */
#define PERL_MULTICONCAT_MAXARG 64

/* The indexes of fields of a multiconcat aux struct.
 * The fixed fields are followed by nargs+1 const segment lengths,
 * and if utf8 and non-utf8 differ, a second nargs+1 set for utf8.
 */

#define PERL_MULTICONCAT_IX_NARGS     0 /* number of arguments */
#define PERL_MULTICONCAT_IX_PLAIN_PV  1 /* non-utf8 constant string */
#define PERL_MULTICONCAT_IX_PLAIN_LEN 2 /* non-utf8 constant string length */
#define PERL_MULTICONCAT_IX_UTF8_PV   3 /* utf8 constant string */
#define PERL_MULTICONCAT_IX_UTF8_LEN  4 /* utf8 constant string length */
#define PERL_MULTICONCAT_IX_LENGTHS   5 /* first of nargs+1 const segment lens */
#define PERL_MULTICONCAT_HEADER_SIZE 5 /* The number of fields of a
                                           multiconcat header */

/* We no longer default to creating a new SV for GvSV.
   Do this before embed.  */
#ifndef PERL_CREATE_GVSV
#  ifndef PERL_DONT_CREATE_GVSV
#    define PERL_DONT_CREATE_GVSV
#  endif
#endif

#if !defined(HAS_WAITPID) && !defined(HAS_WAIT4) || defined(HAS_WAITPID_RUNTIME)
#define PERL_USES_PL_PIDSTATUS
#endif

#if !defined(OS2) && !defined(WIN32) && !defined(DJGPP) && !defined(__SYMBIAN32__)
#define PERL_DEFAULT_DO_EXEC3_IMPLEMENTATION
#endif

#define MEM_SIZE Size_t

/* Round all values passed to malloc up, by default to a multiple of
   sizeof(size_t)
*/
#ifndef PERL_STRLEN_ROUNDUP_QUANTUM
#define PERL_STRLEN_ROUNDUP_QUANTUM Size_t_size
#endif

/* sv_grow() will expand strings by at least a certain percentage of
   the previously *used* length to avoid excessive calls to realloc().
   The default is 25% of the current length.
*/
#ifndef PERL_STRLEN_EXPAND_SHIFT
#  define PERL_STRLEN_EXPAND_SHIFT 2
#endif

/* This use of offsetof() requires /Zc:offsetof- for VS2017 (and presumably
 * onwards) when building Socket.xs, but we can just use a different definition
 * for STRUCT_OFFSET instead. */
#if defined(WIN32) && defined(_MSC_VER) && _MSC_VER >= 1910
#  define STRUCT_OFFSET(s,m)  (Size_t)(&(((s *)0)->m))
#else
#  include <stddef.h>
#  define STRUCT_OFFSET(s,m)  offsetof(s,m)
#endif

/* ptrdiff_t is C11, so undef it under pedantic builds.  (Actually it is
 * in C89, but apparently there are platforms where it doesn't exist.  See
 * thread beginning at http://nntp.perl.org/group/perl.perl5.porters/251541.)
 * */
#ifdef PERL_GCC_PEDANTIC
#   undef HAS_PTRDIFF_T
#endif

#ifdef HAS_PTRDIFF_T
#  define Ptrdiff_t ptrdiff_t
#else
#  define Ptrdiff_t SSize_t
#endif

#ifndef __SYMBIAN32__
#  include <string.h>
#endif

/* This comes after <stdlib.h> so we don't try to change the standard
 * library prototypes; we'll use our own in proto.h instead. */

#ifdef MYMALLOC
#  ifdef PERL_POLLUTE_MALLOC
#   ifndef PERL_EXTMALLOC_DEF
#    define Perl_malloc		malloc
#    define Perl_calloc		calloc
#    define Perl_realloc	realloc
#    define Perl_mfree		free
#   endif
#  else
#    define EMBEDMYMALLOC	/* for compatibility */
#  endif

#  define safemalloc  Perl_malloc
#  define safecalloc  Perl_calloc
#  define saferealloc Perl_realloc
#  define safefree    Perl_mfree
#  define CHECK_MALLOC_TOO_LATE_FOR_(code)	STMT_START {		\
	if (!TAINTING_get && MallocCfg_ptr[MallocCfg_cfg_env_read])	\
		code;							\
    } STMT_END
#  define CHECK_MALLOC_TOO_LATE_FOR(ch)				\
	CHECK_MALLOC_TOO_LATE_FOR_(MALLOC_TOO_LATE_FOR(ch))
#  define panic_write2(s)		write(2, s, strlen(s))
#  define CHECK_MALLOC_TAINT(newval)				\
	CHECK_MALLOC_TOO_LATE_FOR_(				\
		if (newval) {					\
		  PERL_UNUSED_RESULT(panic_write2("panic: tainting with $ENV{PERL_MALLOC_OPT}\n"));\
		  exit(1); })
#  define MALLOC_CHECK_TAINT(argc,argv,env)	STMT_START {	\
	if (doing_taint(argc,argv,env)) {			\
		MallocCfg_ptr[MallocCfg_skip_cfg_env] = 1;	\
    }} STMT_END;
#else  /* MYMALLOC */
#  define safemalloc  safesysmalloc
#  define safecalloc  safesyscalloc
#  define saferealloc safesysrealloc
#  define safefree    safesysfree
#  define CHECK_MALLOC_TOO_LATE_FOR(ch)		((void)0)
#  define CHECK_MALLOC_TAINT(newval)		((void)0)
#  define MALLOC_CHECK_TAINT(argc,argv,env)
#endif /* MYMALLOC */

/* diag_listed_as: "-T" is on the #! line, it must also be used on the command line */
#define TOO_LATE_FOR_(ch,what)	Perl_croak(aTHX_ "\"-%c\" is on the #! line, it must also be used on the command line%s", (char)(ch), what)
#define TOO_LATE_FOR(ch)	TOO_LATE_FOR_(ch, "")
#define MALLOC_TOO_LATE_FOR(ch)	TOO_LATE_FOR_(ch, " with $ENV{PERL_MALLOC_OPT}")
#define MALLOC_CHECK_TAINT2(argc,argv)	MALLOC_CHECK_TAINT(argc,argv,NULL)

#ifndef memzero
#   define memzero(d,l) memset(d,0,l)
#endif

#ifdef I_NETINET_IN
#   include <netinet/in.h>
#endif

#ifdef I_ARPA_INET
#   include <arpa/inet.h>
#endif

#ifdef I_SYS_STAT
#   include <sys/stat.h>
#endif

/* Microsoft VC's sys/stat.h defines all S_Ixxx macros except S_IFIFO.
   This definition should ideally go into win32/win32.h, but S_IFIFO is
   used later here in perl.h before win32/win32.h is being included. */
#if !defined(S_IFIFO) && defined(_S_IFIFO)
#   define S_IFIFO _S_IFIFO
#endif

/* The stat macros for Unisoft System V/88 (and derivatives
   like UTekV) are broken, sometimes giving false positives.  Undefine
   them here and let the code below set them to proper values.

   The ghs macro stands for GreenHills Software C-1.8.5 which
   is the C compiler for sysV88 and the various derivatives.
   This header file bug is corrected in gcc-2.5.8 and later versions.
   --Kaveh Ghazi (ghazi@noc.rutgers.edu) 10/3/94.  */

#if defined(m88k) && defined(ghs)
#   undef S_ISDIR
#   undef S_ISCHR
#   undef S_ISBLK
#   undef S_ISREG
#   undef S_ISFIFO
#   undef S_ISLNK
#endif

#include <time.h>

#ifdef I_SYS_TIME
#   ifdef I_SYS_TIME_KERNEL
#	define KERNEL
#   endif
#   include <sys/time.h>
#   ifdef I_SYS_TIME_KERNEL
#	undef KERNEL
#   endif
#endif

#if defined(HAS_TIMES) && defined(I_SYS_TIMES)
#    include <sys/times.h>
#endif

#include <errno.h>

#if defined(WIN32) && defined(PERL_IMPLICIT_SYS)
#  define WIN32SCK_IS_STDSCK		/* don't pull in custom wsock layer */
#endif

#if defined(HAS_SOCKET) && !defined(WIN32) /* WIN32 handles sockets via win32.h */
# include <sys/socket.h>
# if defined(USE_SOCKS) && defined(I_SOCKS)
#   if !defined(INCLUDE_PROTOTYPES)
#       define INCLUDE_PROTOTYPES /* for <socks.h> */
#       define PERL_SOCKS_NEED_PROTOTYPES
#   endif
#   include <socks.h>
#   ifdef PERL_SOCKS_NEED_PROTOTYPES /* keep cpp space clean */
#       undef INCLUDE_PROTOTYPES
#       undef PERL_SOCKS_NEED_PROTOTYPES
#   endif
# endif
# ifdef I_NETDB
#  ifdef NETWARE
#   include<stdio.h>
#  endif
#  include <netdb.h>
# endif
# ifndef ENOTSOCK
#  ifdef I_NET_ERRNO
#   include <net/errno.h>
#  endif
# endif
#endif

/* sockatmark() is so new (2001) that many places might have it hidden
 * behind some -D_BLAH_BLAH_SOURCE guard.  The __THROW magic is required
 * e.g. in Gentoo, see http://bugs.gentoo.org/show_bug.cgi?id=12605 */
#if defined(HAS_SOCKATMARK) && !defined(HAS_SOCKATMARK_PROTO)
# if defined(__THROW) && defined(__GLIBC__)
int sockatmark(int) __THROW;
# else
int sockatmark(int);
# endif
#endif

#if defined(__osf__) && defined(__cplusplus) && !defined(_XOPEN_SOURCE_EXTENDED) /* Tru64 "cxx" (C++), see hints/dec_osf.sh for why the _XOPEN_SOURCE_EXTENDED cannot be defined. */
EXTERN_C int fchdir(int);
EXTERN_C int flock(int, int);
EXTERN_C int fseeko(FILE *, off_t, int);
EXTERN_C off_t ftello(FILE *);
#endif

#if defined(__SUNPRO_CC) /* SUNWspro CC (C++) */
EXTERN_C char *crypt(const char *, const char *);
#endif

#if defined(__cplusplus) && defined(__CYGWIN__)
EXTERN_C char *crypt(const char *, const char *);
#endif

/*
=head1 Errno

=for apidoc m|void|SETERRNO|int errcode|int vmserrcode

Set C<errno>, and on VMS set C<vaxc$errno>.

=for apidoc mn|void|dSAVEDERRNO

Declare variables needed to save C<errno> and any operating system
specific error number.

=for apidoc mn|void|dSAVE_ERRNO

Declare variables needed to save C<errno> and any operating system
specific error number, and save them for optional later restoration
by C<RESTORE_ERRNO>.

=for apidoc mn|void|SAVE_ERRNO

Save C<errno> and any operating system specific error number for
optional later restoration by C<RESTORE_ERRNO>.  Requires
C<dSAVEDERRNO> or C<dSAVE_ERRNO> in scope.

=for apidoc mn|void|RESTORE_ERRNO

Restore C<errno> and any operating system specific error number that
was saved by C<dSAVE_ERRNO> or C<RESTORE_ERRNO>.

=cut
*/

#ifdef SETERRNO
# undef SETERRNO  /* SOCKS might have defined this */
#endif

#ifdef VMS
#   define SETERRNO(errcode,vmserrcode) \
	STMT_START {			\
	    set_errno(errcode);		\
	    set_vaxc_errno(vmserrcode);	\
	} STMT_END
#   define dSAVEDERRNO    int saved_errno; unsigned saved_vms_errno
#   define dSAVE_ERRNO    int saved_errno = errno; unsigned saved_vms_errno = vaxc$errno
#   define SAVE_ERRNO     ( saved_errno = errno, saved_vms_errno = vaxc$errno )
#   define RESTORE_ERRNO  SETERRNO(saved_errno, saved_vms_errno)

#   define LIB_INVARG 		LIB$_INVARG
#   define RMS_DIR    		RMS$_DIR
#   define RMS_FAC    		RMS$_FAC
#   define RMS_FEX    		RMS$_FEX
#   define RMS_FNF    		RMS$_FNF
#   define RMS_IFI    		RMS$_IFI
#   define RMS_ISI    		RMS$_ISI
#   define RMS_PRV    		RMS$_PRV
#   define SS_ACCVIO      	SS$_ACCVIO
#   define SS_DEVOFFLINE	SS$_DEVOFFLINE
#   define SS_IVCHAN  		SS$_IVCHAN
#   define SS_NORMAL  		SS$_NORMAL
#   define SS_NOPRIV  		SS$_NOPRIV
#   define SS_BUFFEROVF		SS$_BUFFEROVF
#else
#   define LIB_INVARG 		0
#   define RMS_DIR    		0
#   define RMS_FAC    		0
#   define RMS_FEX    		0
#   define RMS_FNF    		0
#   define RMS_IFI    		0
#   define RMS_ISI    		0
#   define RMS_PRV    		0
#   define SS_ACCVIO      	0
#   define SS_DEVOFFLINE	0
#   define SS_IVCHAN  		0
#   define SS_NORMAL  		0
#   define SS_NOPRIV  		0
#   define SS_BUFFEROVF		0
#endif

#ifdef WIN32
#   define dSAVEDERRNO  int saved_errno; DWORD saved_win32_errno
#   define dSAVE_ERRNO  int saved_errno = errno; DWORD saved_win32_errno = GetLastError()
#   define SAVE_ERRNO   ( saved_errno = errno, saved_win32_errno = GetLastError() )
#   define RESTORE_ERRNO ( errno = saved_errno, SetLastError(saved_win32_errno) )
#endif

#ifdef OS2
#   define dSAVEDERRNO  int saved_errno; unsigned long saved_os2_errno
#   define dSAVE_ERRNO  int saved_errno = errno; unsigned long saved_os2_errno = Perl_rc
#   define SAVE_ERRNO   ( saved_errno = errno, saved_os2_errno = Perl_rc )
#   define RESTORE_ERRNO ( errno = saved_errno, Perl_rc = saved_os2_errno )
#endif

#ifndef SETERRNO
#   define SETERRNO(errcode,vmserrcode) (errno = (errcode))
#endif

#ifndef dSAVEDERRNO
#   define dSAVEDERRNO    int saved_errno
#   define dSAVE_ERRNO    int saved_errno = errno
#   define SAVE_ERRNO     (saved_errno = errno)
#   define RESTORE_ERRNO  (errno = saved_errno)
#endif

/*
=head1 Warning and Dieing

=for apidoc Amn|SV *|ERRSV

Returns the SV for C<$@>, creating it if needed.

=for apidoc Am|void|CLEAR_ERRSV

Clear the contents of C<$@>, setting it to the empty string.

This replaces any read-only SV with a fresh SV and removes any magic.

=for apidoc Am|void|SANE_ERRSV

Clean up ERRSV so we can safely set it.

This replaces any read-only SV with a fresh writable copy and removes
any magic.

=cut
*/

#define ERRSV GvSVn(PL_errgv)

/* contains inlined gv_add_by_type */
#define CLEAR_ERRSV() STMT_START {					\
    SV ** const svp = &GvSV(PL_errgv);					\
    if (!*svp) {							\
        *svp = newSVpvs("");                                            \
    } else if (SvREADONLY(*svp)) {					\
	SvREFCNT_dec_NN(*svp);						\
	*svp = newSVpvs("");						\
    } else {								\
	SV *const errsv = *svp;						\
        SvPVCLEAR(errsv);                                               \
	SvPOK_only(errsv);						\
	if (SvMAGICAL(errsv)) {						\
	    mg_free(errsv);						\
	}								\
    }									\
    } STMT_END

/* contains inlined gv_add_by_type */
#define SANE_ERRSV() STMT_START {					\
    SV ** const svp = &GvSV(PL_errgv);					\
    if (!*svp) {							\
        *svp = newSVpvs("");                                            \
    } else if (SvREADONLY(*svp)) {					\
        SV *dupsv = newSVsv(*svp);					\
	SvREFCNT_dec_NN(*svp);						\
	*svp = dupsv;							\
    } else {								\
	SV *const errsv = *svp;						\
	if (SvMAGICAL(errsv)) {						\
	    mg_free(errsv);						\
	}								\
    }									\
    } STMT_END


#ifdef PERL_CORE
# define DEFSV (0 + GvSVn(PL_defgv))
# define DEFSV_set(sv) \
    (SvREFCNT_dec(GvSV(PL_defgv)), GvSV(PL_defgv) = SvREFCNT_inc(sv))
# define SAVE_DEFSV                \
    (                               \
	save_gp(PL_defgv, 0),        \
	GvINTRO_off(PL_defgv),        \
	SAVEGENERICSV(GvSV(PL_defgv)), \
	GvSV(PL_defgv) = NULL           \
    )
#else
# define DEFSV GvSVn(PL_defgv)
# define DEFSV_set(sv) (GvSV(PL_defgv) = (sv))
# define SAVE_DEFSV SAVESPTR(GvSV(PL_defgv))
#endif

#ifndef errno
	extern int errno;     /* ANSI allows errno to be an lvalue expr.
			       * For example in multithreaded environments
			       * something like this might happen:
			       * extern int *_errno(void);
			       * #define errno (*_errno()) */
#endif

#define UNKNOWN_ERRNO_MSG "(unknown)"

#ifdef VMS
#define Strerror(e) strerror((e), vaxc$errno)
#else
#define Strerror(e) strerror(e)
#endif

#ifdef I_SYS_IOCTL
#   ifndef _IOCTL_
#	include <sys/ioctl.h>
#   endif
#endif

#if defined(mc300) || defined(mc500) || defined(mc700) || defined(mc6000)
#   ifdef HAS_SOCKETPAIR
#	undef HAS_SOCKETPAIR
#   endif
#   ifdef I_NDBM
#	undef I_NDBM
#   endif
#endif

#ifndef HAS_SOCKETPAIR
#   ifdef HAS_SOCKET
#	define socketpair Perl_my_socketpair
#   endif
#endif

#if INTSIZE == 2
#   define htoni htons
#   define ntohi ntohs
#else
#   define htoni htonl
#   define ntohi ntohl
#endif

/* Configure already sets Direntry_t */
#if defined(I_DIRENT)
#  include <dirent.h>
#elif defined(I_SYS_NDIR)
#  include <sys/ndir.h>
#elif defined(I_SYS_DIR)
#  include <sys/dir.h>
#endif

/*
 * The following gobbledygook brought to you on behalf of __STDC__.
 * (I could just use #ifndef __STDC__, but this is more bulletproof
 * in the face of half-implementations.)
 */

#if defined(I_SYSMODE)
#include <sys/mode.h>
#endif

#ifndef S_IFMT
#   ifdef _S_IFMT
#	define S_IFMT _S_IFMT
#   else
#	define S_IFMT 0170000
#   endif
#endif

#ifndef S_ISDIR
#   define S_ISDIR(m) ((m & S_IFMT) == S_IFDIR)
#endif

#ifndef S_ISCHR
#   define S_ISCHR(m) ((m & S_IFMT) == S_IFCHR)
#endif

#ifndef S_ISBLK
#   ifdef S_IFBLK
#	define S_ISBLK(m) ((m & S_IFMT) == S_IFBLK)
#   else
#	define S_ISBLK(m) (0)
#   endif
#endif

#ifndef S_ISREG
#   define S_ISREG(m) ((m & S_IFMT) == S_IFREG)
#endif

#ifndef S_ISFIFO
#   ifdef S_IFIFO
#	define S_ISFIFO(m) ((m & S_IFMT) == S_IFIFO)
#   else
#	define S_ISFIFO(m) (0)
#   endif
#endif

#ifndef S_ISLNK
#  ifdef _S_ISLNK
#    define S_ISLNK(m) _S_ISLNK(m)
#  elif defined(_S_IFLNK)
#    define S_ISLNK(m) ((m & S_IFMT) == _S_IFLNK)
#  elif defined(S_IFLNK)
#    define S_ISLNK(m) ((m & S_IFMT) == S_IFLNK)
#  else
#    define S_ISLNK(m) (0)
#  endif
#endif

#ifndef S_ISSOCK
#  ifdef _S_ISSOCK
#    define S_ISSOCK(m) _S_ISSOCK(m)
#  elif defined(_S_IFSOCK)
#    define S_ISSOCK(m) ((m & S_IFMT) == _S_IFSOCK)
#  elif defined(S_IFSOCK)
#    define S_ISSOCK(m) ((m & S_IFMT) == S_IFSOCK)
#  else
#    define S_ISSOCK(m) (0)
#  endif
#endif

#ifndef S_IRUSR
#   ifdef S_IREAD
#	define S_IRUSR S_IREAD
#	define S_IWUSR S_IWRITE
#	define S_IXUSR S_IEXEC
#   else
#	define S_IRUSR 0400
#	define S_IWUSR 0200
#	define S_IXUSR 0100
#   endif
#endif

#ifndef S_IRGRP
#   ifdef S_IRUSR
#       define S_IRGRP (S_IRUSR>>3)
#       define S_IWGRP (S_IWUSR>>3)
#       define S_IXGRP (S_IXUSR>>3)
#   else
#       define S_IRGRP 0040
#       define S_IWGRP 0020
#       define S_IXGRP 0010
#   endif
#endif

#ifndef S_IROTH
#   ifdef S_IRUSR
#       define S_IROTH (S_IRUSR>>6)
#       define S_IWOTH (S_IWUSR>>6)
#       define S_IXOTH (S_IXUSR>>6)
#   else
#       define S_IROTH 0040
#       define S_IWOTH 0020
#       define S_IXOTH 0010
#   endif
#endif

#ifndef S_ISUID
#   define S_ISUID 04000
#endif

#ifndef S_ISGID
#   define S_ISGID 02000
#endif

#ifndef S_IRWXU
#   define S_IRWXU (S_IRUSR|S_IWUSR|S_IXUSR)
#endif

#ifndef S_IRWXG
#   define S_IRWXG (S_IRGRP|S_IWGRP|S_IXGRP)
#endif

#ifndef S_IRWXO
#   define S_IRWXO (S_IROTH|S_IWOTH|S_IXOTH)
#endif

/* Haiku R1 seems to define S_IREAD and S_IWRITE in <posix/fcntl.h>
 * which would get included through <sys/file.h >, but that is 3000
 * lines in the future.  --jhi */

#if !defined(S_IREAD) && !defined(__HAIKU__)
#   define S_IREAD S_IRUSR
#endif

#if !defined(S_IWRITE) && !defined(__HAIKU__)
#   define S_IWRITE S_IWUSR
#endif

#ifndef S_IEXEC
#   define S_IEXEC S_IXUSR
#endif

#if defined(cray) || defined(gould) || defined(i860) || defined(pyr)
#   define SLOPPYDIVIDE
#endif

#ifdef UV
#undef UV
#endif

/* This used to be conditionally defined based on whether we had a sprintf()
 * that correctly returns the string length (as required by C89), but we no
 * longer need that. XS modules can (and do) use this name, so it must remain
 * a part of the API that's visible to modules.

=head1 Miscellaneous Functions

=for apidoc ATmD|int|my_sprintf|NN char *buffer|NN const char *pat|...

Do NOT use this due to the possibility of overflowing C<buffer>.  Instead use
my_snprintf()

=cut
*/
#define my_sprintf sprintf

/*
 * If we have v?snprintf() and the C99 variadic macros, we can just
 * use just the v?snprintf().  It is nice to try to trap the buffer
 * overflow, however, so if we are DEBUGGING, and we cannot use the
 * gcc statement expressions, then use the function wrappers which try
 * to trap the overflow.  If we can use the gcc statement expressions,
 * we can try that even with the version that uses the C99 variadic
 * macros.
 */

/* Note that we do not check against snprintf()/vsnprintf() returning
 * negative values because that is non-standard behaviour and we use
 * snprintf/vsnprintf only iff HAS_VSNPRINTF has been defined, and
 * that should be true only if the snprintf()/vsnprintf() are true
 * to the standard. */

#define PERL_SNPRINTF_CHECK(len, max, api) STMT_START { if ((max) > 0 && (Size_t)len > (max)) Perl_croak_nocontext("panic: %s buffer overflow", STRINGIFY(api)); } STMT_END

#ifdef USE_QUADMATH
#  define my_snprintf Perl_my_snprintf
#  define PERL_MY_SNPRINTF_GUARDED
#elif defined(HAS_SNPRINTF) && defined(HAS_C99_VARIADIC_MACROS) && !(defined(DEBUGGING) && !defined(PERL_USE_GCC_BRACE_GROUPS)) && !defined(PERL_GCC_PEDANTIC)
#  ifdef PERL_USE_GCC_BRACE_GROUPS
#      define my_snprintf(buffer, max, ...) ({ int len = snprintf(buffer, max, __VA_ARGS__); PERL_SNPRINTF_CHECK(len, max, snprintf); len; })
#      define PERL_MY_SNPRINTF_GUARDED
#  else
#    define my_snprintf(buffer, max, ...) snprintf(buffer, max, __VA_ARGS__)
#  endif
#else
#  define my_snprintf  Perl_my_snprintf
#  define PERL_MY_SNPRINTF_GUARDED
#endif

/* There is no quadmath_vsnprintf, and therefore my_vsnprintf()
 * dies if called under USE_QUADMATH. */
#if defined(HAS_VSNPRINTF) && defined(HAS_C99_VARIADIC_MACROS) && !(defined(DEBUGGING) && !defined(PERL_USE_GCC_BRACE_GROUPS)) && !defined(PERL_GCC_PEDANTIC)
#  ifdef PERL_USE_GCC_BRACE_GROUPS
#      define my_vsnprintf(buffer, max, ...) ({ int len = vsnprintf(buffer, max, __VA_ARGS__); PERL_SNPRINTF_CHECK(len, max, vsnprintf); len; })
#      define PERL_MY_VSNPRINTF_GUARDED
#  else
#    define my_vsnprintf(buffer, max, ...) vsnprintf(buffer, max, __VA_ARGS__)
#  endif
#else
#  define my_vsnprintf Perl_my_vsnprintf
#  define PERL_MY_VSNPRINTF_GUARDED
#endif

/* You will definitely need to use the PERL_MY_SNPRINTF_POST_GUARD()
 * or PERL_MY_VSNPRINTF_POST_GUARD() if you otherwise decide to ignore
 * the result of my_snprintf() or my_vsnprintf().  (No, you should not
 * completely ignore it: otherwise you cannot know whether your output
 * was too long.)
 *
 * int len = my_sprintf(buf, max, ...);
 * PERL_MY_SNPRINTF_POST_GUARD(len, max);
 *
 * The trick is that in certain platforms [a] the my_sprintf() already
 * contains the sanity check, while in certain platforms [b] it needs
 * to be done as a separate step.  The POST_GUARD is that step-- in [a]
 * platforms the POST_GUARD actually does nothing since the check has
 * already been done.  Watch out for the max being the same in both calls.
 *
 * If you actually use the snprintf/vsnprintf return value already,
 * you assumedly are checking its validity somehow.  But you can
 * insert the POST_GUARD() also in that case. */

#ifndef PERL_MY_SNPRINTF_GUARDED
#  define PERL_MY_SNPRINTF_POST_GUARD(len, max) PERL_SNPRINTF_CHECK(len, max, snprintf)
#else
#  define PERL_MY_SNPRINTF_POST_GUARD(len, max) PERL_UNUSED_VAR(len)
#endif

#ifndef  PERL_MY_VSNPRINTF_GUARDED
#  define PERL_MY_VSNPRINTF_POST_GUARD(len, max) PERL_SNPRINTF_CHECK(len, max, vsnprintf)
#else
#  define PERL_MY_VSNPRINTF_POST_GUARD(len, max) PERL_UNUSED_VAR(len)
#endif

#ifdef HAS_STRLCAT
#  define my_strlcat    strlcat
#endif

#if defined(PERL_CORE) || defined(PERL_EXT)
#  ifdef HAS_MEMRCHR
#    define my_memrchr	memrchr
#  else
#    define my_memrchr	S_my_memrchr
#  endif
#endif

#ifdef HAS_STRLCPY
#  define my_strlcpy	strlcpy
#endif

#ifdef HAS_STRNLEN
#  define my_strnlen	strnlen
#endif

/*
    The IV type is supposed to be long enough to hold any integral
    value or a pointer.
    --Andy Dougherty	August 1996
*/

typedef IVTYPE IV;
typedef UVTYPE UV;

#if defined(USE_64_BIT_INT) && defined(HAS_QUAD)
#  if QUADKIND == QUAD_IS_INT64_T && defined(INT64_MAX)
#    define IV_MAX ((IV)INT64_MAX)
#    define IV_MIN ((IV)INT64_MIN)
#    define UV_MAX ((UV)UINT64_MAX)
#    ifndef UINT64_MIN
#      define UINT64_MIN 0
#    endif
#    define UV_MIN ((UV)UINT64_MIN)
#  else
#    define IV_MAX PERL_QUAD_MAX
#    define IV_MIN PERL_QUAD_MIN
#    define UV_MAX PERL_UQUAD_MAX
#    define UV_MIN PERL_UQUAD_MIN
#  endif
#  define IV_IS_QUAD
#  define UV_IS_QUAD
#else
#  if defined(INT32_MAX) && IVSIZE == 4
#    define IV_MAX ((IV)INT32_MAX)
#    define IV_MIN ((IV)INT32_MIN)
#    ifndef UINT32_MAX_BROKEN /* e.g. HP-UX with gcc messes this up */
#        define UV_MAX ((UV)UINT32_MAX)
#    else
#        define UV_MAX ((UV)4294967295U)
#    endif
#    ifndef UINT32_MIN
#      define UINT32_MIN 0
#    endif
#    define UV_MIN ((UV)UINT32_MIN)
#  else
#    define IV_MAX PERL_LONG_MAX
#    define IV_MIN PERL_LONG_MIN
#    define UV_MAX PERL_ULONG_MAX
#    define UV_MIN PERL_ULONG_MIN
#  endif
#  if IVSIZE == 8
#    define IV_IS_QUAD
#    define UV_IS_QUAD
#    ifndef HAS_QUAD
#      define HAS_QUAD
#    endif
#  else
#    undef IV_IS_QUAD
#    undef UV_IS_QUAD
#if !defined(PERL_CORE)
/* We think that removing this decade-old undef this will cause too much
   breakage on CPAN for too little gain. (See RT #119753)
   However, we do need HAS_QUAD in the core for use by the drand48 code. */
#    undef HAS_QUAD
#endif
#  endif
#endif

#define Size_t_MAX (~(Size_t)0)
#define SSize_t_MAX (SSize_t)(~(Size_t)0 >> 1)

#define IV_DIG (BIT_DIGITS(IVSIZE * 8))
#define UV_DIG (BIT_DIGITS(UVSIZE * 8))

#ifndef NO_PERL_PRESERVE_IVUV
#define PERL_PRESERVE_IVUV	/* We like our integers to stay integers. */
#endif

/*
 *  The macros INT2PTR and NUM2PTR are (despite their names)
 *  bi-directional: they will convert int/float to or from pointers.
 *  However the conversion to int/float are named explicitly:
 *  PTR2IV, PTR2UV, PTR2NV.
 *
 *  For int conversions we do not need two casts if pointers are
 *  the same size as IV and UV.   Otherwise we need an explicit
 *  cast (PTRV) to avoid compiler warnings.
 */
#if (IVSIZE == PTRSIZE) && (UVSIZE == PTRSIZE)
#  define PTRV			UV
#  define INT2PTR(any,d)	(any)(d)
#elif PTRSIZE == LONGSIZE
#  define PTRV			unsigned long
#  define PTR2ul(p)		(unsigned long)(p)
#else
#  define PTRV			unsigned
#endif

#ifndef INT2PTR
#  define INT2PTR(any,d)	(any)(PTRV)(d)
#endif

#ifndef PTR2ul
#  define PTR2ul(p)	INT2PTR(unsigned long,p)	
#endif

#define NUM2PTR(any,d)	(any)(PTRV)(d)
#define PTR2IV(p)	INT2PTR(IV,p)
#define PTR2UV(p)	INT2PTR(UV,p)
#define PTR2NV(p)	NUM2PTR(NV,p)
#define PTR2nat(p)	(PTRV)(p)	/* pointer to integer of PTRSIZE */

/* According to strict ANSI C89 one cannot freely cast between
 * data pointers and function (code) pointers.  There are at least
 * two ways around this.  One (used below) is to do two casts,
 * first the other pointer to an (unsigned) integer, and then
 * the integer to the other pointer.  The other way would be
 * to use unions to "overlay" the pointers.  For an example of
 * the latter technique, see union dirpu in struct xpvio in sv.h.
 * The only feasible use is probably temporarily storing
 * function pointers in a data pointer (such as a void pointer). */

#define DPTR2FPTR(t,p) ((t)PTR2nat(p))	/* data pointer to function pointer */
#define FPTR2DPTR(t,p) ((t)PTR2nat(p))	/* function pointer to data pointer */

#ifdef USE_LONG_DOUBLE
#  if LONG_DOUBLESIZE == DOUBLESIZE
#    define LONG_DOUBLE_EQUALS_DOUBLE
#    undef USE_LONG_DOUBLE /* Ouch! */
#  endif
#endif

/* The following is all to get LDBL_DIG, in order to pick a nice
   default value for printing floating point numbers in Gconvert.
   (see config.h)
*/
#ifndef HAS_LDBL_DIG
#  if LONG_DOUBLESIZE == 10
#    define LDBL_DIG 18 /* assume IEEE */
#  elif LONG_DOUBLESIZE == 12
#    define LDBL_DIG 18 /* gcc? */
#  elif LONG_DOUBLESIZE == 16
#    define LDBL_DIG 33 /* assume IEEE */
#  elif LONG_DOUBLESIZE == DOUBLESIZE
#    define LDBL_DIG DBL_DIG /* bummer */
#  endif
#endif

typedef NVTYPE NV;

#ifdef I_IEEEFP
#   include <ieeefp.h>
#endif

#if defined(__DECC) && defined(__osf__)
/* Also Tru64 cc has broken NaN comparisons. */
#  define NAN_COMPARE_BROKEN
#endif
#if defined(__sgi)
#  define NAN_COMPARE_BROKEN
#endif

#ifdef USE_LONG_DOUBLE
#   ifdef I_SUNMATH
#       include <sunmath.h>
#   endif
#   if defined(LDBL_DIG)
#       define NV_DIG LDBL_DIG
#       ifdef LDBL_MANT_DIG
#           define NV_MANT_DIG LDBL_MANT_DIG
#       endif
#       ifdef LDBL_MIN
#           define NV_MIN LDBL_MIN
#       endif
#       ifdef LDBL_MAX
#           define NV_MAX LDBL_MAX
#       endif
#       ifdef LDBL_MIN_EXP
#           define NV_MIN_EXP LDBL_MIN_EXP
#       endif
#       ifdef LDBL_MAX_EXP
#           define NV_MAX_EXP LDBL_MAX_EXP
#       endif
#       ifdef LDBL_MIN_10_EXP
#           define NV_MIN_10_EXP LDBL_MIN_10_EXP
#       endif
#       ifdef LDBL_MAX_10_EXP
#           define NV_MAX_10_EXP LDBL_MAX_10_EXP
#       endif
#       ifdef LDBL_EPSILON
#           define NV_EPSILON LDBL_EPSILON
#       endif
#       ifdef LDBL_MAX
#           define NV_MAX LDBL_MAX
/* Having LDBL_MAX doesn't necessarily mean that we have LDBL_MIN... -Allen */
#       elif defined(HUGE_VALL)
#           define NV_MAX HUGE_VALL
#       endif
#   endif
#   if defined(HAS_SQRTL)
#       define Perl_acos acosl
#       define Perl_asin asinl
#       define Perl_atan atanl
#       define Perl_atan2 atan2l
#       define Perl_ceil ceill
#       define Perl_cos cosl
#       define Perl_cosh coshl
#       define Perl_exp expl
/* no Perl_fabs, but there's PERL_ABS */
#       define Perl_floor floorl
#       define Perl_fmod fmodl
#       define Perl_log logl
#       define Perl_log10 log10l
#       define Perl_pow powl
#       define Perl_sin sinl
#       define Perl_sinh sinhl
#       define Perl_sqrt sqrtl
#       define Perl_tan tanl
#       define Perl_tanh tanhl
#   endif
/* e.g. libsunmath doesn't have modfl and frexpl as of mid-March 2000 */
#   ifndef Perl_modf
#       ifdef HAS_MODFL
#           define Perl_modf(x,y) modfl(x,y)
/* eg glibc 2.2 series seems to provide modfl on ppc and arm, but has no
   prototype in <math.h> */
#           ifndef HAS_MODFL_PROTO
EXTERN_C long double modfl(long double, long double *);
#	    endif
#       elif (defined(HAS_TRUNCL) || defined(HAS_AINTL)) && defined(HAS_COPYSIGNL)
        extern long double Perl_my_modfl(long double x, long double *ip);
#           define Perl_modf(x,y) Perl_my_modfl(x,y)
#       endif
#   endif
#   ifndef Perl_frexp
#       ifdef HAS_FREXPL
#           define Perl_frexp(x,y) frexpl(x,y)
#       elif defined(HAS_ILOGBL) && defined(HAS_SCALBNL)
extern long double Perl_my_frexpl(long double x, int *e);
#           define Perl_frexp(x,y) Perl_my_frexpl(x,y)
#       endif
#   endif
#   ifndef Perl_ldexp
#       ifdef HAS_LDEXPL
#           define Perl_ldexp(x, y) ldexpl(x,y)
#       elif defined(HAS_SCALBNL) && FLT_RADIX == 2
#           define Perl_ldexp(x,y) scalbnl(x,y)
#       endif
#   endif
#   ifndef Perl_isnan
#       if defined(HAS_ISNANL) && !(defined(isnan) && defined(HAS_C99))
#           define Perl_isnan(x) isnanl(x)
#       elif defined(__sgi) && defined(__c99)  /* XXX Configure test needed */
#           define Perl_isnan(x) isnan(x)
#       endif
#   endif
#   ifndef Perl_isinf
#       if defined(HAS_ISINFL) && !(defined(isinf) && defined(HAS_C99))
#           define Perl_isinf(x) isinfl(x)
#       elif defined(__sgi) && defined(__c99)  /* XXX Configure test needed */
#           define Perl_isinf(x) isinf(x)
#       elif defined(LDBL_MAX) && !defined(NAN_COMPARE_BROKEN)
#           define Perl_isinf(x) ((x) > LDBL_MAX || (x) < -LDBL_MAX)
#       endif
#   endif
#   ifndef Perl_isfinite
#       define Perl_isfinite(x) Perl_isfinitel(x)
#   endif
#elif defined(USE_QUADMATH) && defined(I_QUADMATH)
#   include <quadmath.h>
#   define NV_DIG FLT128_DIG
#   define NV_MANT_DIG FLT128_MANT_DIG
#   define NV_MIN FLT128_MIN
#   define NV_MAX FLT128_MAX
#   define NV_MIN_EXP FLT128_MIN_EXP
#   define NV_MAX_EXP FLT128_MAX_EXP
#   define NV_EPSILON FLT128_EPSILON
#   define NV_MIN_10_EXP FLT128_MIN_10_EXP
#   define NV_MAX_10_EXP FLT128_MAX_10_EXP
#   define Perl_acos acosq
#   define Perl_asin asinq
#   define Perl_atan atanq
#   define Perl_atan2 atan2q
#   define Perl_ceil ceilq
#   define Perl_cos cosq
#   define Perl_cosh coshq
#   define Perl_exp expq
/* no Perl_fabs, but there's PERL_ABS */
#   define Perl_floor floorq
#   define Perl_fmod fmodq
#   define Perl_log logq
#   define Perl_log10 log10q
#   define Perl_signbit signbitq
#   define Perl_pow powq
#   define Perl_sin sinq
#   define Perl_sinh sinhq
#   define Perl_sqrt sqrtq
#   define Perl_tan tanq
#   define Perl_tanh tanhq
#   define Perl_modf(x,y) modfq(x,y)
#   define Perl_frexp(x,y) frexpq(x,y)
#   define Perl_ldexp(x, y) ldexpq(x,y)
#   define Perl_isinf(x) isinfq(x)
#   define Perl_isnan(x) isnanq(x)
#   define Perl_isfinite(x) !(isnanq(x) || isinfq(x))
#   define Perl_fp_class(x) ((x) == 0.0Q ? 0 : isinfq(x) ? 3 : isnanq(x) ? 4 : PERL_ABS(x) < FLT128_MIN ? 2 : 1)
#   define Perl_fp_class_inf(x)    (Perl_fp_class(x) == 3)
#   define Perl_fp_class_nan(x)    (Perl_fp_class(x) == 4)
#   define Perl_fp_class_norm(x)   (Perl_fp_class(x) == 1)
#   define Perl_fp_class_denorm(x) (Perl_fp_class(x) == 2)
#   define Perl_fp_class_zero(x)   (Perl_fp_class(x) == 0)
#else
#   define NV_DIG DBL_DIG
#   define NV_MANT_DIG DBL_MANT_DIG
#   define NV_MIN DBL_MIN
#   define NV_MAX DBL_MAX
#   define NV_MIN_EXP DBL_MIN_EXP
#   define NV_MAX_EXP DBL_MAX_EXP
#   define NV_MIN_10_EXP DBL_MIN_10_EXP
#   define NV_MAX_10_EXP DBL_MAX_10_EXP
#   define NV_EPSILON DBL_EPSILON
#   define NV_MAX DBL_MAX
#   define NV_MIN DBL_MIN

/* These math interfaces are C89. */
#   define Perl_acos acos
#   define Perl_asin asin
#   define Perl_atan atan
#   define Perl_atan2 atan2
#   define Perl_ceil ceil
#   define Perl_cos cos
#   define Perl_cosh cosh
#   define Perl_exp exp
/* no Perl_fabs, but there's PERL_ABS */
#   define Perl_floor floor
#   define Perl_fmod fmod
#   define Perl_log log
#   define Perl_log10 log10
#   define Perl_pow pow
#   define Perl_sin sin
#   define Perl_sinh sinh
#   define Perl_sqrt sqrt
#   define Perl_tan tan
#   define Perl_tanh tanh

#   define Perl_modf(x,y) modf(x,y)
#   define Perl_frexp(x,y) frexp(x,y)
#   define Perl_ldexp(x,y) ldexp(x,y)

#   ifndef Perl_isnan
#       ifdef HAS_ISNAN
#           define Perl_isnan(x) isnan(x)
#       endif
#   endif
#   ifndef Perl_isinf
#       if defined(HAS_ISINF)
#           define Perl_isinf(x) isinf(x)
#       elif defined(DBL_MAX) && !defined(NAN_COMPARE_BROKEN)
#           define Perl_isinf(x) ((x) > DBL_MAX || (x) < -DBL_MAX)
#       endif
#   endif
#   ifndef Perl_isfinite
#     ifdef HAS_ISFINITE
#       define Perl_isfinite(x) isfinite(x)
#     elif defined(HAS_FINITE)
#       define Perl_isfinite(x) finite(x)
#     endif
#   endif
#endif

/* fpclassify(): C99.  It is supposed to be a macro that switches on
* the sizeof() of its argument, so there's no need for e.g. fpclassifyl().*/
#if !defined(Perl_fp_class) && defined(HAS_FPCLASSIFY)
#    include <math.h>
#    if defined(FP_INFINITE) && defined(FP_NAN)
#        define Perl_fp_class(x)	fpclassify(x)
#        define Perl_fp_class_inf(x)	(Perl_fp_class(x)==FP_INFINITE)
#        define Perl_fp_class_nan(x)	(Perl_fp_class(x)==FP_NAN)
#        define Perl_fp_class_norm(x)	(Perl_fp_class(x)==FP_NORMAL)
#        define Perl_fp_class_denorm(x)	(Perl_fp_class(x)==FP_SUBNORMAL)
#        define Perl_fp_class_zero(x)	(Perl_fp_class(x)==FP_ZERO)
#    elif defined(FP_PLUS_INF) && defined(FP_QNAN)
/* Some versions of HP-UX (10.20) have (only) fpclassify() but which is
 * actually not the C99 fpclassify, with its own set of return defines. */
#        define Perl_fp_class(x)	fpclassify(x)
#        define Perl_fp_class_pinf(x)	(Perl_fp_class(x)==FP_PLUS_INF)
#        define Perl_fp_class_ninf(x)	(Perl_fp_class(x)==FP_MINUS_INF)
#        define Perl_fp_class_snan(x)	(Perl_fp_class(x)==FP_SNAN)
#        define Perl_fp_class_qnan(x)	(Perl_fp_class(x)==FP_QNAN)
#        define Perl_fp_class_pnorm(x)	(Perl_fp_class(x)==FP_PLUS_NORM)
#        define Perl_fp_class_nnorm(x)	(Perl_fp_class(x)==FP_MINUS_NORM)
#        define Perl_fp_class_pdenorm(x)	(Perl_fp_class(x)==FP_PLUS_DENORM)
#        define Perl_fp_class_ndenorm(x)	(Perl_fp_class(x)==FP_MINUS_DENORM)
#        define Perl_fp_class_pzero(x)	(Perl_fp_class(x)==FP_PLUS_ZERO)
#        define Perl_fp_class_nzero(x)	(Perl_fp_class(x)==FP_MINUS_ZERO)
#    else
#        undef Perl_fp_class /* Unknown set of defines */
#    endif
#endif

/* fp_classify(): Legacy: VMS, maybe Unicos? The values, however,
 * are identical to the C99 fpclassify(). */
#if !defined(Perl_fp_class) && defined(HAS_FP_CLASSIFY)
#    include <math.h>
#    ifdef __VMS
     /* FP_INFINITE and others are here rather than in math.h as C99 stipulates */
#        include <fp.h>
     /* oh, and the isnormal macro has a typo in it! */
#    undef isnormal
#    define isnormal(x) Perl_fp_class_norm(x)
#    endif
#    if defined(FP_INFINITE) && defined(FP_NAN)
#        define Perl_fp_class(x)	fp_classify(x)
#        define Perl_fp_class_inf(x)	(Perl_fp_class(x)==FP_INFINITE)
#        define Perl_fp_class_nan(x)	(Perl_fp_class(x)==FP_NAN)
#        define Perl_fp_class_norm(x)	(Perl_fp_class(x)==FP_NORMAL)
#        define Perl_fp_class_denorm(x)	(Perl_fp_class(x)==FP_SUBNORMAL)
#        define Perl_fp_class_zero(x)	(Perl_fp_class(x)==FP_ZERO)
#    else
#        undef Perl_fp_class /* Unknown set of defines */
#    endif
#endif

/* Feel free to check with me for the SGI manpages, SGI testing,
 * etcetera, if you want to try getting this to work with IRIX.
 *
 * - Allen <allens@cpan.org> */

/* fpclass(): SysV, at least Solaris and some versions of IRIX. */
#if !defined(Perl_fp_class) && (defined(HAS_FPCLASS)||defined(HAS_FPCLASSL))
/* Solaris and IRIX have fpclass/fpclassl, but they are using
 * an enum typedef, not cpp symbols, and Configure doesn't detect that.
 * Define some symbols also as cpp symbols so we can detect them. */
#    if defined(__sun) || defined(__sgi) /* XXX Configure test instead */
#     define FP_PINF FP_PINF
#     define FP_QNAN FP_QNAN
#    endif
#    include <math.h>
#    ifdef I_IEEFP
#        include <ieeefp.h>
#    endif
#    ifdef I_FP
#        include <fp.h>
#    endif
#    if defined(USE_LONG_DOUBLE) && defined(HAS_FPCLASSL)
#        define Perl_fp_class(x)	fpclassl(x)
#    else
#        define Perl_fp_class(x)	fpclass(x)
#    endif
#    if defined(FP_CLASS_PINF) && defined(FP_CLASS_SNAN)
#        define Perl_fp_class_snan(x)	(Perl_fp_class(x)==FP_CLASS_SNAN)
#        define Perl_fp_class_qnan(x)	(Perl_fp_class(x)==FP_CLASS_QNAN)
#        define Perl_fp_class_ninf(x)	(Perl_fp_class(x)==FP_CLASS_NINF)
#        define Perl_fp_class_pinf(x)	(Perl_fp_class(x)==FP_CLASS_PINF)
#        define Perl_fp_class_nnorm(x)	(Perl_fp_class(x)==FP_CLASS_NNORM)
#        define Perl_fp_class_pnorm(x)	(Perl_fp_class(x)==FP_CLASS_PNORM)
#        define Perl_fp_class_ndenorm(x)	(Perl_fp_class(x)==FP_CLASS_NDENORM)
#        define Perl_fp_class_pdenorm(x)	(Perl_fp_class(x)==FP_CLASS_PDENORM)
#        define Perl_fp_class_nzero(x)	(Perl_fp_class(x)==FP_CLASS_NZERO)
#        define Perl_fp_class_pzero(x)	(Perl_fp_class(x)==FP_CLASS_PZERO)
#    elif defined(FP_PINF) && defined(FP_QNAN)
#        define Perl_fp_class_snan(x)	(Perl_fp_class(x)==FP_SNAN)
#        define Perl_fp_class_qnan(x)	(Perl_fp_class(x)==FP_QNAN)
#        define Perl_fp_class_ninf(x)	(Perl_fp_class(x)==FP_NINF)
#        define Perl_fp_class_pinf(x)	(Perl_fp_class(x)==FP_PINF)
#        define Perl_fp_class_nnorm(x)	(Perl_fp_class(x)==FP_NNORM)
#        define Perl_fp_class_pnorm(x)	(Perl_fp_class(x)==FP_PNORM)
#        define Perl_fp_class_ndenorm(x)	(Perl_fp_class(x)==FP_NDENORM)
#        define Perl_fp_class_pdenorm(x)	(Perl_fp_class(x)==FP_PDENORM)
#        define Perl_fp_class_nzero(x)	(Perl_fp_class(x)==FP_NZERO)
#        define Perl_fp_class_pzero(x)	(Perl_fp_class(x)==FP_PZERO)
#    else
#        undef Perl_fp_class /* Unknown set of defines */
#    endif
#endif

/* fp_class(): Legacy: at least Tru64, some versions of IRIX. */
#if !defined(Perl_fp_class) && (defined(HAS_FP_CLASS)||defined(HAS_FP_CLASSL))
#    include <math.h>
#    if !defined(FP_SNAN) && defined(I_FP_CLASS)
#        include <fp_class.h>
#    endif
#    if defined(FP_POS_INF) && defined(FP_QNAN)
#        ifdef __sgi /* XXX Configure test instead */
#            ifdef USE_LONG_DOUBLE
#                define Perl_fp_class(x)	fp_class_l(x)
#            else
#                define Perl_fp_class(x)	fp_class_d(x)
#            endif
#        else
#            if defined(USE_LONG_DOUBLE) && defined(HAS_FP_CLASSL)
#                define Perl_fp_class(x)	fp_classl(x)
#            else
#                define Perl_fp_class(x)	fp_class(x)
#            endif
#        endif
#        if defined(FP_POS_INF) && defined(FP_QNAN)
#            define Perl_fp_class_snan(x)	(Perl_fp_class(x)==FP_SNAN)
#            define Perl_fp_class_qnan(x)	(Perl_fp_class(x)==FP_QNAN)
#            define Perl_fp_class_ninf(x)	(Perl_fp_class(x)==FP_NEG_INF)
#            define Perl_fp_class_pinf(x)	(Perl_fp_class(x)==FP_POS_INF)
#            define Perl_fp_class_nnorm(x)	(Perl_fp_class(x)==FP_NEG_NORM)
#            define Perl_fp_class_pnorm(x)	(Perl_fp_class(x)==FP_POS_NORM)
#            define Perl_fp_class_ndenorm(x)	(Perl_fp_class(x)==FP_NEG_DENORM)
#            define Perl_fp_class_pdenorm(x)	(Perl_fp_class(x)==FP_POS_DENORM)
#            define Perl_fp_class_nzero(x)	(Perl_fp_class(x)==FP_NEG_ZERO)
#            define Perl_fp_class_pzero(x)	(Perl_fp_class(x)==FP_POS_ZERO)
#        else
#            undef Perl_fp_class /* Unknown set of defines */
#        endif
#    endif
#endif

/* class(), _class(): Legacy: AIX. */
#if !defined(Perl_fp_class) && defined(HAS_CLASS)
#    include <math.h>
#    if defined(FP_PLUS_NORM) && defined(FP_PLUS_INF)
#        ifndef _cplusplus
#            define Perl_fp_class(x)	class(x)
#        else
#            define Perl_fp_class(x)	_class(x)
#        endif
#        if defined(FP_PLUS_INF) && defined(FP_NANQ)
#            define Perl_fp_class_snan(x)	(Perl_fp_class(x)==FP_NANS)
#            define Perl_fp_class_qnan(x)	(Perl_fp_class(x)==FP_NANQ)
#            define Perl_fp_class_ninf(x)	(Perl_fp_class(x)==FP_MINUS_INF)
#            define Perl_fp_class_pinf(x)	(Perl_fp_class(x)==FP_PLUS_INF)
#            define Perl_fp_class_nnorm(x)	(Perl_fp_class(x)==FP_MINUS_NORM)
#            define Perl_fp_class_pnorm(x)	(Perl_fp_class(x)==FP_PLUS_NORM)
#            define Perl_fp_class_ndenorm(x)	(Perl_fp_class(x)==FP_MINUS_DENORM)
#            define Perl_fp_class_pdenorm(x)	(Perl_fp_class(x)==FP_PLUS_DENORM)
#            define Perl_fp_class_nzero(x)	(Perl_fp_class(x)==FP_MINUS_ZERO)
#            define Perl_fp_class_pzero(x)	(Perl_fp_class(x)==FP_PLUS_ZERO)
#        else
#            undef Perl_fp_class /* Unknown set of defines */
#        endif
#    endif
#endif

/* Win32: _fpclass(), _isnan(), _finite(). */
#ifdef _MSC_VER
#  ifndef Perl_isnan
#    define Perl_isnan(x) _isnan(x)
#  endif
#  ifndef Perl_isfinite
#    define Perl_isfinite(x) _finite(x)
#  endif
#  ifndef Perl_fp_class_snan
/* No simple way to #define Perl_fp_class because _fpclass()
 * returns a set of bits. */
#    define Perl_fp_class_snan(x) (_fpclass(x) & _FPCLASS_SNAN)
#    define Perl_fp_class_qnan(x) (_fpclass(x) & _FPCLASS_QNAN)
#    define Perl_fp_class_nan(x) (_fpclass(x) & (_FPCLASS_SNAN|_FPCLASS_QNAN))
#    define Perl_fp_class_ninf(x) (_fpclass(x) & _FPCLASS_NINF))
#    define Perl_fp_class_pinf(x) (_fpclass(x) & _FPCLASS_PINF))
#    define Perl_fp_class_inf(x) (_fpclass(x) & (_FPCLASS_NINF|_FPCLASS_PINF))
#    define Perl_fp_class_nnorm(x) (_fpclass(x) & _FPCLASS_NN)
#    define Perl_fp_class_pnorm(x) (_fpclass(x) & _FPCLASS_PN)
#    define Perl_fp_class_norm(x) (_fpclass(x) & (_FPCLASS_NN|_FPCLASS_PN))
#    define Perl_fp_class_ndenorm(x) (_fpclass(x) & _FPCLASS_ND)
#    define Perl_fp_class_pdenorm(x) (_fpclass(x) & _FPCLASS_PD)
#    define Perl_fp_class_denorm(x) (_fpclass(x) & (_FPCLASS_ND|_FPCLASS_PD))
#    define Perl_fp_class_nzero(x) (_fpclass(x) & _FPCLASS_NZ)
#    define Perl_fp_class_pzero(x) (_fpclass(x) & _FPCLASS_PZ)
#    define Perl_fp_class_zero(x) (_fpclass(x) & (_FPCLASS_NZ|_FPCLASS_PZ))
#  endif
#endif

#if !defined(Perl_fp_class_inf) && \
  defined(Perl_fp_class_pinf) && defined(Perl_fp_class_ninf)
#  define Perl_fp_class_inf(x) \
    (Perl_fp_class_pinf(x) || Perl_fp_class_ninf(x))
#endif

#if !defined(Perl_fp_class_nan) && \
  defined(Perl_fp_class_snan) && defined(Perl_fp_class_qnan)
#  define Perl_fp_class_nan(x) \
    (Perl_fp_class_snan(x) || Perl_fp_class_qnan(x))
#endif

#if !defined(Perl_fp_class_zero) && \
  defined(Perl_fp_class_pzero) && defined(Perl_fp_class_nzero)
#  define Perl_fp_class_zero(x) \
    (Perl_fp_class_pzero(x) || Perl_fp_class_nzero(x))
#endif

#if !defined(Perl_fp_class_norm) && \
  defined(Perl_fp_class_pnorm) && defined(Perl_fp_class_nnorm)
#  define Perl_fp_class_norm(x) \
    (Perl_fp_class_pnorm(x) || Perl_fp_class_nnorm(x))
#endif

#if !defined(Perl_fp_class_denorm) && \
  defined(Perl_fp_class_pdenorm) && defined(Perl_fp_class_ndenorm)
#  define Perl_fp_class_denorm(x) \
    (Perl_fp_class_pdenorm(x) || Perl_fp_class_ndenorm(x))
#endif

#ifndef Perl_isnan
#   ifdef Perl_fp_class_nan
#       define Perl_isnan(x) Perl_fp_class_nan(x)
#   elif defined(HAS_UNORDERED)
#       define Perl_isnan(x) unordered((x), 0.0)
#   else
#       define Perl_isnan(x) ((x)!=(x))
#   endif
#endif

#ifndef Perl_isinf
#   ifdef Perl_fp_class_inf
#       define Perl_isinf(x) Perl_fp_class_inf(x)
#   endif
#endif

#ifndef Perl_isfinite
#   if defined(HAS_ISFINITE) && !defined(isfinite)
#     define Perl_isfinite(x) isfinite((double)(x))
#   elif defined(HAS_FINITE)
#       define Perl_isfinite(x) finite((double)(x))
#   elif defined(Perl_fp_class_finite)
#     define Perl_isfinite(x) Perl_fp_class_finite(x)
#   else
/* For the infinities the multiplication returns nan,
 * for the nan the multiplication also returns nan,
 * for everything else (that is, finite) zero should be returned. */
#     define Perl_isfinite(x) (((x) * 0) == 0)
#   endif
#endif

#ifndef Perl_isinf
#   if defined(Perl_isfinite) && defined(Perl_isnan)
#       define Perl_isinf(x) !(Perl_isfinite(x)||Perl_isnan(x))
#   endif
#endif

/* We need Perl_isfinitel (ends with ell) (if available) even when
 * not USE_LONG_DOUBLE because the printf code (sv_catpvfn_flags)
 * needs that. */
#if defined(HAS_LONG_DOUBLE) && !defined(Perl_isfinitel)
/* If isfinite() is a macro and looks like we have C99,
 * we assume it's the type-aware C99 isfinite(). */
#    if defined(HAS_ISFINITE) && defined(isfinite) && defined(HAS_C99)
#        define Perl_isfinitel(x) isfinite(x)
#    elif defined(HAS_ISFINITEL)
#        define Perl_isfinitel(x) isfinitel(x)
#    elif defined(HAS_FINITEL)
#        define Perl_isfinitel(x) finitel(x)
#    elif defined(HAS_INFL) && defined(HAS_NANL)
#        define Perl_isfinitel(x) !(isinfl(x)||isnanl(x))
#    else
#        define Perl_isfinitel(x) ((x) * 0 == 0)  /* See Perl_isfinite. */
#    endif
#endif

/* The default is to use Perl's own atof() implementation (in numeric.c).
 * Usually that is the one to use but for some platforms (e.g. UNICOS)
 * it is however best to use the native implementation of atof.
 * You can experiment with using your native one by -DUSE_PERL_ATOF=0.
 * Some good tests to try out with either setting are t/base/num.t,
 * t/op/numconvert.t, and t/op/pack.t. Note that if using long doubles
 * you may need to be using a different function than atof! */

#ifndef USE_PERL_ATOF
#   ifndef _UNICOS
#       define USE_PERL_ATOF
#   endif
#else
#   if USE_PERL_ATOF == 0
#       undef USE_PERL_ATOF
#   endif
#endif

#ifdef USE_PERL_ATOF
#   define Perl_atof(s) Perl_my_atof(s)
#   define Perl_atof2(s, n) Perl_my_atof3(aTHX_ (s), &(n), 0)
#else
#   define Perl_atof(s) (NV)atof(s)
#   define Perl_atof2(s, n) ((n) = atof(s))
#endif
#define my_atof2(a,b) my_atof3(a,b,0)

/*
 * CHAR_MIN and CHAR_MAX are not included here, as the (char) type may be
 * ambiguous. It may be equivalent to (signed char) or (unsigned char)
 * depending on local options. Until Configure detects this (or at least
 * detects whether the "signed" keyword is available) the CHAR ranges
 * will not be included. UCHAR functions normally.
 *                                                           - kja
 */

#define PERL_UCHAR_MIN ((unsigned char)0)
#define PERL_UCHAR_MAX ((unsigned char)UCHAR_MAX)

#define PERL_USHORT_MIN ((unsigned short)0)
#define PERL_USHORT_MAX ((unsigned short)USHRT_MAX)

#define PERL_SHORT_MAX ((short)SHRT_MAX)
#define PERL_SHORT_MIN ((short)SHRT_MIN)

#define PERL_UINT_MAX ((unsigned int)UINT_MAX)
#define PERL_UINT_MIN ((unsigned int)0)

#define PERL_INT_MAX ((int)INT_MAX)
#define PERL_INT_MIN ((int)INT_MIN)

#define PERL_ULONG_MAX ((unsigned long)ULONG_MAX)
#define PERL_ULONG_MIN ((unsigned long)0L)

#define PERL_LONG_MAX ((long)LONG_MAX)
#define PERL_LONG_MIN ((long)LONG_MIN)

#ifdef UV_IS_QUAD
#    define PERL_UQUAD_MAX	(~(UV)0)
#    define PERL_UQUAD_MIN	((UV)0)
#    define PERL_QUAD_MAX 	((IV) (PERL_UQUAD_MAX >> 1))
#    define PERL_QUAD_MIN 	(-PERL_QUAD_MAX - ((3 & -1) == 3))
#endif

/*
=head1 Numeric functions

=for apidoc AmnUh||PERL_INT_MIN
=for apidoc AmnUh||PERL_LONG_MAX
=for apidoc AmnUh||PERL_LONG_MIN
=for apidoc AmnUh||PERL_QUAD_MAX
=for apidoc AmnUh||PERL_SHORT_MAX
=for apidoc AmnUh||PERL_SHORT_MIN
=for apidoc AmnUh||PERL_UCHAR_MAX
=for apidoc AmnUh||PERL_UCHAR_MIN
=for apidoc AmnUh||PERL_UINT_MAX
=for apidoc AmnUh||PERL_ULONG_MAX
=for apidoc AmnUh||PERL_ULONG_MIN
=for apidoc AmnUh||PERL_UQUAD_MAX
=for apidoc AmnUh||PERL_UQUAD_MIN
=for apidoc AmnUh||PERL_USHORT_MAX
=for apidoc AmnUh||PERL_USHORT_MIN
=for apidoc AmnUh||PERL_QUAD_MIN
=for apidoc AmnU||PERL_INT_MAX
This and
C<PERL_INT_MIN>,
C<PERL_LONG_MAX>,
C<PERL_LONG_MIN>,
C<PERL_QUAD_MAX>,
C<PERL_SHORT_MAX>,
C<PERL_SHORT_MIN>,
C<PERL_UCHAR_MAX>,
C<PERL_UCHAR_MIN>,
C<PERL_UINT_MAX>,
C<PERL_ULONG_MAX>,
C<PERL_ULONG_MIN>,
C<PERL_UQUAD_MAX>,
C<PERL_UQUAD_MIN>,
C<PERL_USHORT_MAX>,
C<PERL_USHORT_MIN>,
C<PERL_QUAD_MIN>
give the largest and smallest number representable in the current
platform in variables of the corresponding types.

For signed types, the smallest representable number is the most negative
number, the one furthest away from zero.

For C99 and later compilers, these correspond to things like C<INT_MAX>, which
are available to the C code.  But these constants, furnished by Perl,
allow code compiled on earlier compilers to portably have access to the same
constants.

=cut

*/

typedef MEM_SIZE STRLEN;

typedef struct op OP;
typedef struct cop COP;
typedef struct unop UNOP;
typedef struct unop_aux UNOP_AUX;
typedef struct binop BINOP;
typedef struct listop LISTOP;
typedef struct logop LOGOP;
typedef struct pmop PMOP;
typedef struct svop SVOP;
typedef struct padop PADOP;
typedef struct pvop PVOP;
typedef struct loop LOOP;
typedef struct methop METHOP;

#ifdef PERL_CORE
typedef struct opslab OPSLAB;
typedef struct opslot OPSLOT;
#endif

typedef struct block_hooks BHK;
typedef struct custom_op XOP;

typedef struct interpreter PerlInterpreter;

/* SGI's <sys/sema.h> has struct sv */
#if defined(__sgi)
#   define STRUCT_SV perl_sv
#else
#   define STRUCT_SV sv
#endif
typedef struct STRUCT_SV SV;
typedef struct av AV;
typedef struct hv HV;
typedef struct cv CV;
typedef struct p5rx REGEXP;
typedef struct gp GP;
typedef struct gv GV;
typedef struct io IO;
typedef struct context PERL_CONTEXT;
typedef struct block BLOCK;

typedef struct magic MAGIC;
typedef struct xpv XPV;
typedef struct xpviv XPVIV;
typedef struct xpvuv XPVUV;
typedef struct xpvnv XPVNV;
typedef struct xpvmg XPVMG;
typedef struct xpvlv XPVLV;
typedef struct xpvinvlist XINVLIST;
typedef struct xpvav XPVAV;
typedef struct xpvhv XPVHV;
typedef struct xpvgv XPVGV;
typedef struct xpvcv XPVCV;
typedef struct xpvbm XPVBM;
typedef struct xpvfm XPVFM;
typedef struct xpvio XPVIO;
typedef struct mgvtbl MGVTBL;
typedef union any ANY;
typedef struct ptr_tbl_ent PTR_TBL_ENT_t;
typedef struct ptr_tbl PTR_TBL_t;
typedef struct clone_params CLONE_PARAMS;

/* a pad is currently just an AV; but that might change,
 * so hide the type.  */
typedef struct padlist PADLIST;
typedef AV PAD;
typedef struct padnamelist PADNAMELIST;
typedef struct padname PADNAME;

/* always enable PERL_OP_PARENT  */
#if !defined(PERL_OP_PARENT)
#  define PERL_OP_PARENT
#endif

/* enable PERL_COPY_ON_WRITE by default */
#if !defined(PERL_COPY_ON_WRITE) && !defined(PERL_NO_COW)
#  define PERL_COPY_ON_WRITE
#endif

#ifdef PERL_COPY_ON_WRITE
#  define PERL_ANY_COW
#else
# define PERL_SAWAMPERSAND
#endif

#if defined(PERL_DEBUG_READONLY_OPS) && !defined(USE_ITHREADS)
# error PERL_DEBUG_READONLY_OPS only works with ithreads
#endif

#include "handy.h"
#include "charclass_invlists.h"

#if defined(USE_LARGE_FILES) && !defined(NO_64_BIT_RAWIO)
#   if LSEEKSIZE == 8 && !defined(USE_64_BIT_RAWIO)
#       define USE_64_BIT_RAWIO	/* implicit */
#   endif
#endif

/* Notice the use of HAS_FSEEKO: now we are obligated to always use
 * fseeko/ftello if possible.  Don't go #defining ftell to ftello yourself,
 * however, because operating systems like to do that themself. */
#ifndef FSEEKSIZE
#   ifdef HAS_FSEEKO
#       define FSEEKSIZE LSEEKSIZE
#   else
#       define FSEEKSIZE LONGSIZE
#   endif
#endif

#if defined(USE_LARGE_FILES) && !defined(NO_64_BIT_STDIO)
#   if FSEEKSIZE == 8 && !defined(USE_64_BIT_STDIO)
#       define USE_64_BIT_STDIO /* implicit */
#   endif
#endif

#ifdef USE_64_BIT_RAWIO
#   ifdef HAS_OFF64_T
#       undef Off_t
#       define Off_t off64_t
#       undef LSEEKSIZE
#       define LSEEKSIZE 8
#   endif
/* Most 64-bit environments have defines like _LARGEFILE_SOURCE that
 * will trigger defines like the ones below.  Some 64-bit environments,
 * however, do not.  Therefore we have to explicitly mix and match. */
#   if defined(USE_OPEN64)
#       define open open64
#   endif
#   if defined(USE_LSEEK64)
#       define lseek lseek64
#   else
#       if defined(USE_LLSEEK)
#           define lseek llseek
#       endif
#   endif
#   if defined(USE_STAT64)
#       define stat stat64
#   endif
#   if defined(USE_FSTAT64)
#       define fstat fstat64
#   endif
#   if defined(USE_LSTAT64)
#       define lstat lstat64
#   endif
#   if defined(USE_FLOCK64)
#       define flock flock64
#   endif
#   if defined(USE_LOCKF64)
#       define lockf lockf64
#   endif
#   if defined(USE_FCNTL64)
#       define fcntl fcntl64
#   endif
#   if defined(USE_TRUNCATE64)
#       define truncate truncate64
#   endif
#   if defined(USE_FTRUNCATE64)
#       define ftruncate ftruncate64
#   endif
#endif

#ifdef USE_64_BIT_STDIO
#   ifdef HAS_FPOS64_T
#       undef Fpos_t
#       define Fpos_t fpos64_t
#   endif
/* Most 64-bit environments have defines like _LARGEFILE_SOURCE that
 * will trigger defines like the ones below.  Some 64-bit environments,
 * however, do not. */
#   if defined(USE_FOPEN64)
#       define fopen fopen64
#   endif
#   if defined(USE_FSEEK64)
#       define fseek fseek64 /* don't do fseeko here, see perlio.c */
#   endif
#   if defined(USE_FTELL64)
#       define ftell ftell64 /* don't do ftello here, see perlio.c */
#   endif
#   if defined(USE_FSETPOS64)
#       define fsetpos fsetpos64
#   endif
#   if defined(USE_FGETPOS64)
#       define fgetpos fgetpos64
#   endif
#   if defined(USE_TMPFILE64)
#       define tmpfile tmpfile64
#   endif
#   if defined(USE_FREOPEN64)
#       define freopen freopen64
#   endif
#endif

#if defined(OS2)
#  include "iperlsys.h"
#endif

#ifdef DOSISH
#   if defined(OS2)
#       include "os2ish.h"
#   else
#       include "dosish.h"
#   endif
#elif defined(VMS)
#   include "vmsish.h"
#elif defined(PLAN9)
#   include "./plan9/plan9ish.h"
#elif defined(__VOS__)
#   ifdef __GNUC__
#     include "./vos/vosish.h"
#   else
#     include "vos/vosish.h"
#   endif
#elif defined(__SYMBIAN32__)
#   include "symbian/symbianish.h"
#elif defined(__HAIKU__)
#   include "haiku/haikuish.h"
#else
#   include "unixish.h"
#endif

#ifdef __amigaos4__
#    include "amigaos.h"
#    undef FD_CLOEXEC /* a lie in AmigaOS */
#endif

/* NSIG logic from Configure --> */
#ifndef NSIG
#  ifdef _NSIG
#    define NSIG (_NSIG)
#  elif defined(SIGMAX)
#    define NSIG (SIGMAX+1)
#  elif defined(SIG_MAX)
#    define NSIG (SIG_MAX+1)
#  elif defined(_SIG_MAX)
#    define NSIG (_SIG_MAX+1)
#  elif defined(MAXSIG)
#    define NSIG (MAXSIG+1)
#  elif defined(MAX_SIG)
#    define NSIG (MAX_SIG+1)
#  elif defined(SIGARRAYSIZE)
#    define NSIG SIGARRAYSIZE /* Assume ary[SIGARRAYSIZE] */
#  elif defined(_sys_nsig)
#    define NSIG (_sys_nsig) /* Solaris 2.5 */
#  else
     /* Default to some arbitrary number that's big enough to get most
      * of the common signals.  */
#    define NSIG 50
#  endif
#endif
/* <-- NSIG logic from Configure */

#ifndef NO_ENVIRON_ARRAY
#  define USE_ENVIRON_ARRAY
#endif

#ifdef USE_ITHREADS
   /* On some platforms it would be safe to use a read/write mutex with many
    * readers possible at the same time.  On other platforms, notably IBM ones,
    * subsequent getenv calls destroy earlier ones.  Those platforms would not
    * be able to handle simultaneous getenv calls */
#  define ENV_LOCK            MUTEX_LOCK(&PL_env_mutex)
#  define ENV_UNLOCK          MUTEX_UNLOCK(&PL_env_mutex)
#  define ENV_INIT            MUTEX_INIT(&PL_env_mutex);
#  define ENV_TERM            MUTEX_DESTROY(&PL_env_mutex);
#else
#  define ENV_LOCK       NOOP;
#  define ENV_UNLOCK     NOOP;
#  define ENV_INIT       NOOP;
#  define ENV_TERM       NOOP;
#endif

/* Some critical sections need to lock both the locale and the environment.
 * XXX khw intends to change this to lock both mutexes, but that brings up
 * issues of potential deadlock, so should be done at the beginning of a
 * development cycle.  So for now, it just locks the environment.  Note that
 * many modern platforms are locale-thread-safe anyway, so locking the locale
 * mutex is a no-op anyway */
#define ENV_LOCALE_LOCK     ENV_LOCK
#define ENV_LOCALE_UNLOCK   ENV_UNLOCK

/* And some critical sections care only that no one else is writing either the
 * locale nor the environment.  XXX Again this is for the future.  This can be
 * simulated with using COND_WAIT in thread.h */
#define ENV_LOCALE_READ_LOCK     ENV_LOCALE_LOCK
#define ENV_LOCALE_READ_UNLOCK   ENV_LOCALE_UNLOCK

#if defined(HAS_SIGACTION) && defined(SA_SIGINFO)
    /* having sigaction(2) means that the OS supports both 1-arg and 3-arg
     * signal handlers. But the perl core itself only fully supports 1-arg
     * handlers, so don't enable for now.
     * NB: POSIX::sigaction() supports both.
     *
     * # define PERL_USE_3ARG_SIGHANDLER
     */
#endif

/* Siginfo_t:
 * This is an alias for the OS's siginfo_t, except that where the OS
 * doesn't support it, declare a dummy version instead. This allows us to
 * have signal handler functions which always have a Siginfo_t parameter
 * regardless of platform, (and which will just be passed a NULL value
 * where the OS doesn't support HAS_SIGACTION).
 */

#if defined(HAS_SIGACTION) && defined(SA_SIGINFO)
    typedef siginfo_t Siginfo_t;
#else
#ifdef si_signo /* minix */
#undef si_signo
#endif
    typedef struct {
        int si_signo;
    } Siginfo_t;
#endif


/*
 * initialise to avoid floating-point exceptions from overflow, etc
 */
#ifndef PERL_FPU_INIT
#  ifdef HAS_FPSETMASK
#    if HAS_FLOATINGPOINT_H
#      include <floatingpoint.h>
#    endif
/* Some operating systems have this as a macro, which in turn expands to a comma
   expression, and the last sub-expression is something that gets calculated,
   and then they have the gall to warn that a value computed is not used. Hence
   cast to void.  */
#    define PERL_FPU_INIT (void)fpsetmask(0)
#  elif defined(SIGFPE) && defined(SIG_IGN) && !defined(PERL_MICRO)
#    define PERL_FPU_INIT       PL_sigfpe_saved = (Sighandler_t) signal(SIGFPE, SIG_IGN)
#    define PERL_FPU_PRE_EXEC   { Sigsave_t xfpe; rsignal_save(SIGFPE, PL_sigfpe_saved, &xfpe);
#    define PERL_FPU_POST_EXEC    rsignal_restore(SIGFPE, &xfpe); }
#  else
#    define PERL_FPU_INIT
#  endif
#endif
#ifndef PERL_FPU_PRE_EXEC
#  define PERL_FPU_PRE_EXEC   {
#  define PERL_FPU_POST_EXEC  }
#endif

/* In Tru64 the cc -ieee enables the IEEE math but disables traps.
 * We need to reenable the "invalid" trap because otherwise generation
 * of NaN values leaves the IEEE fp flags in bad state, leaving any further
 * fp ops behaving strangely (Inf + 1 resulting in zero, for example). */
#ifdef __osf__
#  include <machine/fpu.h>
#  define PERL_SYS_FPU_INIT \
     STMT_START { \
         ieee_set_fp_control(IEEE_TRAP_ENABLE_INV); \
         signal(SIGFPE, SIG_IGN); \
     } STMT_END
#endif
/* In IRIX the default for Flush to Zero bit is true,
 * which means that results going below the minimum of normal
 * floating points go to zero, instead of going denormal/subnormal.
 * This is unlike almost any other system running Perl, so let's clear it.
 * [perl #123767] IRIX64 blead (ddce084a) opbasic/arith.t failure, originally
 * [perl #120426] small numbers shouldn't round to zero if they have extra floating digits
 *
 * XXX The flush-to-zero behaviour should be a Configure scan.
 * To change the behaviour usually requires some system-specific
 * incantation, though, like the below. */
#ifdef __sgi
#  include <sys/fpu.h>
#  define PERL_SYS_FPU_INIT \
     STMT_START { \
         union fpc_csr csr; \
         csr.fc_word = get_fpc_csr(); \
         csr.fc_struct.flush = 0; \
         set_fpc_csr(csr.fc_word); \
     } STMT_END
#endif

#ifndef PERL_SYS_FPU_INIT
#  define PERL_SYS_FPU_INIT NOOP
#endif

#ifndef PERL_SYS_INIT3_BODY
#  define PERL_SYS_INIT3_BODY(argvp,argcp,envp) PERL_SYS_INIT_BODY(argvp,argcp)
#endif

/*
=head1 Miscellaneous Functions

=for apidoc Am|void|PERL_SYS_INIT|int *argc|char*** argv
Provides system-specific tune up of the C runtime environment necessary to
run Perl interpreters.  This should be called only once, before creating
any Perl interpreters.

=for apidoc Am|void|PERL_SYS_INIT3|int *argc|char*** argv|char*** env
Provides system-specific tune up of the C runtime environment necessary to
run Perl interpreters.  This should be called only once, before creating
any Perl interpreters.

=for apidoc Am|void|PERL_SYS_TERM|
Provides system-specific clean up of the C runtime environment after
running Perl interpreters.  This should be called only once, after
freeing any remaining Perl interpreters.

=cut
 */

#define PERL_SYS_INIT(argc, argv)	Perl_sys_init(argc, argv)
#define PERL_SYS_INIT3(argc, argv, env)	Perl_sys_init3(argc, argv, env)
#define PERL_SYS_TERM()			Perl_sys_term()

#ifndef PERL_WRITE_MSG_TO_CONSOLE
#  define PERL_WRITE_MSG_TO_CONSOLE(io, msg, len) PerlIO_write(io, msg, len)
#endif

#ifndef MAXPATHLEN
#  ifdef PATH_MAX
#    ifdef _POSIX_PATH_MAX
#       if PATH_MAX > _POSIX_PATH_MAX
/* POSIX 1990 (and pre) was ambiguous about whether PATH_MAX
 * included the null byte or not.  Later amendments of POSIX,
 * XPG4, the Austin Group, and the Single UNIX Specification
 * all explicitly include the null byte in the PATH_MAX.
 * Ditto for _POSIX_PATH_MAX. */
#         define MAXPATHLEN PATH_MAX
#       else
#         define MAXPATHLEN _POSIX_PATH_MAX
#       endif
#    else
#      define MAXPATHLEN (PATH_MAX+1)
#    endif
#  else
#    define MAXPATHLEN 1024	/* Err on the large side. */
#  endif
#endif

/* USE_5005THREADS needs to be after unixish.h as <pthread.h> includes
 * <sys/signal.h> which defines NSIG - which will stop inclusion of <signal.h>
 * this results in many functions being undeclared which bothers C++
 * May make sense to have threads after "*ish.h" anyway
 */

/* clang Thread Safety Analysis/Annotations/Attributes
 * http://clang.llvm.org/docs/ThreadSafetyAnalysis.html
 *
 * Available since clang 3.6-ish (appeared in 3.4, but shaky still in 3.5).
 * Apple XCode hijacks __clang_major__ and __clang_minor__
 * (6.1 means really clang 3.6), so needs extra hijinks
 * (could probably also test the contents of __apple_build_version__).
 */
#if defined(USE_ITHREADS) && defined(I_PTHREAD) && \
    defined(__clang__) && \
    !defined(PERL_GLOBAL_STRUCT) && \
    !defined(PERL_GLOBAL_STRUCT_PRIVATE) && \
    !defined(SWIG) && \
  ((!defined(__apple_build_version__) &&               \
    ((__clang_major__ == 3 && __clang_minor__ >= 6) || \
     (__clang_major__ >= 4))) || \
   (defined(__apple_build_version__) &&                \
    ((__clang_major__ == 6 && __clang_minor__ >= 1) || \
     (__clang_major__ >= 7))))
#  define PERL_TSA__(x)   __attribute__((x))
#  define PERL_TSA_ACTIVE
#else
#  define PERL_TSA__(x)   /* No TSA, make TSA attributes no-ops. */
#  undef PERL_TSA_ACTIVE
#endif

/* PERL_TSA_CAPABILITY() is used to annotate typedefs.
 * typedef old_type PERL_TSA_CAPABILITY("mutex") new_type;
 */
#define PERL_TSA_CAPABILITY(x) \
    PERL_TSA__(capability(x))

/* In the below examples the mutex must be lexically visible, usually
 * either as global variables, or as function arguments. */

/* PERL_TSA_GUARDED_BY() is used to annotate global variables.
 *
 * Foo foo PERL_TSA_GUARDED_BY(mutex);
 */
#define PERL_TSA_GUARDED_BY(x) \
    PERL_TSA__(guarded_by(x))

/* PERL_TSA_PT_GUARDED_BY() is used to annotate global pointers.
 * The data _behind_ the pointer is guarded.
 *
 * Foo* ptr PERL_TSA_PT_GUARDED_BY(mutex);
 */
#define PERL_TSA_PT_GUARDED_BY(x) \
    PERL_TSA__(pt_guarded_by(x))

/* PERL_TSA_REQUIRES() is used to annotate functions.
 * The caller MUST hold the resource when calling the function.
 *
 * void Foo() PERL_TSA_REQUIRES(mutex);
 */
#define PERL_TSA_REQUIRES(x) \
    PERL_TSA__(requires_capability(x))

/* PERL_TSA_EXCLUDES() is used to annotate functions.
 * The caller MUST NOT hold resource when calling the function.
 *
 * EXCLUDES should be used when the function first acquires
 * the resource and then releases it.  Use to avoid deadlock.
 *
 * void Foo() PERL_TSA_EXCLUDES(mutex);
 */
#define PERL_TSA_EXCLUDES(x) \
    PERL_TSA__(locks_excluded(x))

/* PERL_TSA_ACQUIRE() is used to annotate functions.
 * The caller MUST NOT hold the resource when calling the function,
 * and the function will acquire the resource.
 *
 * void Foo() PERL_TSA_ACQUIRE(mutex);
 */
#define PERL_TSA_ACQUIRE(x) \
    PERL_TSA__(acquire_capability(x))

/* PERL_TSA_RELEASE() is used to annotate functions.
 * The caller MUST hold the resource when calling the function,
 * and the function will release the resource.
 *
 * void Foo() PERL_TSA_RELEASE(mutex);
 */
#define PERL_TSA_RELEASE(x) \
    PERL_TSA__(release_capability(x))

/* PERL_TSA_NO_TSA is used to annotate functions.
 * Used when being intentionally unsafe, or when the code is too
 * complicated for the analysis.  Use sparingly.
 *
 * void Foo() PERL_TSA_NO_TSA;
 */
#define PERL_TSA_NO_TSA \
    PERL_TSA__(no_thread_safety_analysis)

/* There are more annotations/attributes available, see the clang
 * documentation for details. */

#if defined(USE_ITHREADS)
#  ifdef NETWARE
#    include <nw5thread.h>
#  elif defined(WIN32)
#    include <win32thread.h>
#  elif defined(OS2)
#    include "os2thread.h"
#  elif defined(I_MACH_CTHREADS)
#    include <mach/cthreads.h>
typedef cthread_t	perl_os_thread;
typedef mutex_t		perl_mutex;
typedef condition_t	perl_cond;
typedef void *		perl_key;
#  elif defined(I_PTHREAD) /* Posix threads */
#    include <pthread.h>
typedef pthread_t	perl_os_thread;
typedef pthread_mutex_t PERL_TSA_CAPABILITY("mutex") perl_mutex;
typedef pthread_cond_t	perl_cond;
typedef pthread_key_t	perl_key;
#  endif
#endif /* USE_ITHREADS */

#ifdef PERL_TSA_ACTIVE
/* Since most pthread mutex interfaces have not been annotated, we
 * need to have these wrappers. The NO_TSA annotation is quite ugly
 * but it cannot be avoided in plain C, unlike in C++, where one could
 * e.g. use ACQUIRE() with no arg on a mutex lock method.
 *
 * The bodies of these wrappers are in util.c
 *
 * TODO: however, some platforms are starting to get these clang
 * thread safety annotations for pthreads, for example FreeBSD.
 * Do we need a way to a bypass these wrappers? */
EXTERN_C int perl_tsa_mutex_lock(perl_mutex* mutex)
  PERL_TSA_ACQUIRE(*mutex)
  PERL_TSA_NO_TSA;
EXTERN_C int perl_tsa_mutex_unlock(perl_mutex* mutex)
  PERL_TSA_RELEASE(*mutex)
  PERL_TSA_NO_TSA;
#endif

#if defined(WIN32)
#  include "win32.h"
#endif

#ifdef NETWARE
#  include "netware.h"
#endif

#define STATUS_UNIX	PL_statusvalue
#ifdef VMS
#   define STATUS_NATIVE	PL_statusvalue_vms
/*
 * vaxc$errno is only guaranteed to be valid if errno == EVMSERR, otherwise
 * its contents can not be trusted.  Unfortunately, Perl seems to check
 * it on exit, so it when PL_statusvalue_vms is updated, vaxc$errno should
 * be updated also.
 */
#  include <stsdef.h>
#  include <ssdef.h>
/* Presume this because if VMS changes it, it will require a new
 * set of APIs for waiting on children for binary compatibility.
 */
#  define child_offset_bits (8)
#  ifndef C_FAC_POSIX
#  define C_FAC_POSIX 0x35A000
#  endif

/*  STATUS_EXIT - validates and returns a NATIVE exit status code for the
 * platform from the existing UNIX or Native status values.
 */

#   define STATUS_EXIT \
	(((I32)PL_statusvalue_vms == -1 ? SS$_ABORT : PL_statusvalue_vms) | \
	   (VMSISH_HUSHED ? STS$M_INHIB_MSG : 0))


/* STATUS_NATIVE_CHILD_SET - Calculate UNIX status that matches the child
 * exit code and shifts the UNIX value over the correct number of bits to
 * be a child status.  Usually the number of bits is 8, but that could be
 * platform dependent.  The NATIVE status code is presumed to have either
 * from a child process.
 */

/* This is complicated.  The child processes return a true native VMS
   status which must be saved.  But there is an assumption in Perl that
   the UNIX child status has some relationship to errno values, so
   Perl tries to translate it to text in some of the tests.
   In order to get the string translation correct, for the error, errno
   must be EVMSERR, but that generates a different text message
   than what the test programs are expecting.  So an errno value must
   be derived from the native status value when an error occurs.
   That will hide the true native status message.  With this version of
   perl, the true native child status can always be retrieved so that
   is not a problem.  But in this case, Pl_statusvalue and errno may
   have different values in them.
 */

#   define STATUS_NATIVE_CHILD_SET(n) \
	STMT_START {							\
	    I32 evalue = (I32)n;					\
	    if (evalue == EVMSERR) {					\
	      PL_statusvalue_vms = vaxc$errno;				\
	      PL_statusvalue = evalue;					\
	    } else {							\
	      PL_statusvalue_vms = evalue;				\
	      if (evalue == -1) {					\
		PL_statusvalue = -1;					\
		PL_statusvalue_vms = SS$_ABORT; /* Should not happen */ \
	      } else							\
		PL_statusvalue = Perl_vms_status_to_unix(evalue, 1);	\
	      set_vaxc_errno(evalue);					\
	      if ((PL_statusvalue_vms & C_FAC_POSIX) == C_FAC_POSIX)	\
		  set_errno(EVMSERR);					\
	      else set_errno(Perl_vms_status_to_unix(evalue, 0));	\
	      PL_statusvalue = PL_statusvalue << child_offset_bits;	\
	    }								\
	} STMT_END

#   ifdef VMSISH_STATUS
#	define STATUS_CURRENT	(VMSISH_STATUS ? STATUS_NATIVE : STATUS_UNIX)
#   else
#	define STATUS_CURRENT	STATUS_UNIX
#   endif

  /* STATUS_UNIX_SET - takes a UNIX/POSIX errno value and attempts to update
   * the NATIVE status to an equivalent value.  Can not be used to translate
   * exit code values as exit code values are not guaranteed to have any
   * relationship at all to errno values.
   * This is used when Perl is forcing errno to have a specific value.
   */
#   define STATUS_UNIX_SET(n)				\
	STMT_START {					\
	    I32 evalue = (I32)n;			\
	    PL_statusvalue = evalue;			\
	    if (PL_statusvalue != -1) {			\
		if (PL_statusvalue != EVMSERR) {	\
		  PL_statusvalue &= 0xFFFF;		\
		  if (MY_POSIX_EXIT)			\
		    PL_statusvalue_vms=PL_statusvalue ? SS$_ABORT : SS$_NORMAL;\
		  else PL_statusvalue_vms = Perl_unix_status_to_vms(evalue); \
		}					\
		else {					\
		  PL_statusvalue_vms = vaxc$errno;	\
		}					\
	    }						\
	    else PL_statusvalue_vms = SS$_ABORT;	\
	    set_vaxc_errno(PL_statusvalue_vms);		\
	} STMT_END

  /* STATUS_UNIX_EXIT_SET - Takes a UNIX/POSIX exit code and sets
   * the NATIVE error status based on it.
   *
   * When in the default mode to comply with the Perl VMS documentation,
   * 0 is a success and any other code sets the NATIVE status to a failure
   * code of SS$_ABORT.
   *
   * In the new POSIX EXIT mode, native status will be set so that the
   * actual exit code will can be retrieved by the calling program or
   * shell.
   *
   * If the exit code is not clearly a UNIX parent or child exit status,
   * it will be passed through as a VMS status.
   */

#   define STATUS_UNIX_EXIT_SET(n)			\
	STMT_START {					\
	    I32 evalue = (I32)n;			\
	    PL_statusvalue = evalue;			\
	    if (MY_POSIX_EXIT) { \
	      if (evalue <= 0xFF00) {		\
		  if (evalue > 0xFF)			\
		    evalue = (evalue >> child_offset_bits) & 0xFF; \
		  PL_statusvalue_vms =		\
		    (C_FAC_POSIX | (evalue << 3 ) |	\
		    ((evalue == 1) ? (STS$K_ERROR | STS$M_INHIB_MSG) : 1)); \
	      } else /* forgive them Perl, for they have sinned */ \
		PL_statusvalue_vms = evalue; \
	    } else { \
	      if (evalue == 0)			\
		PL_statusvalue_vms = SS$_NORMAL;	\
	      else if (evalue <= 0xFF00) \
		PL_statusvalue_vms = SS$_ABORT; \
	      else { /* forgive them Perl, for they have sinned */ \
		  if (evalue != EVMSERR) PL_statusvalue_vms = evalue; \
		  else PL_statusvalue_vms = vaxc$errno;	\
		  /* And obviously used a VMS status value instead of UNIX */ \
		  PL_statusvalue = EVMSERR;		\
	      } \
	      set_vaxc_errno(PL_statusvalue_vms);	\
	    }						\
	} STMT_END


  /* STATUS_EXIT_SET - Takes a NATIVE/UNIX/POSIX exit code
   * and sets the NATIVE error status based on it.  This special case
   * is needed to maintain compatibility with past VMS behavior.
   *
   * In the default mode on VMS, this number is passed through as
   * both the NATIVE and UNIX status.  Which makes it different
   * that the STATUS_UNIX_EXIT_SET.
   *
   * In the new POSIX EXIT mode, native status will be set so that the
   * actual exit code will can be retrieved by the calling program or
   * shell.
   *
   * A POSIX exit code is from 0 to 255.  If the exit code is higher
   * than this, it needs to be assumed that it is a VMS exit code and
   * passed through.
   */

#   define STATUS_EXIT_SET(n)				\
	STMT_START {					\
	    I32 evalue = (I32)n;			\
	    PL_statusvalue = evalue;			\
	    if (MY_POSIX_EXIT)				\
		if (evalue > 255) PL_statusvalue_vms = evalue; else {	\
		  PL_statusvalue_vms = \
		    (C_FAC_POSIX | (evalue << 3 ) |	\
		     ((evalue == 1) ? (STS$K_ERROR | STS$M_INHIB_MSG) : 1));} \
	    else					\
		PL_statusvalue_vms = evalue ? evalue : SS$_NORMAL; \
	    set_vaxc_errno(PL_statusvalue_vms);		\
	} STMT_END


 /* This macro forces a success status */
#   define STATUS_ALL_SUCCESS	\
	(PL_statusvalue = 0, PL_statusvalue_vms = SS$_NORMAL)

 /* This macro forces a failure status */
#   define STATUS_ALL_FAILURE	(PL_statusvalue = 1, \
     vaxc$errno = PL_statusvalue_vms = MY_POSIX_EXIT ? \
	(C_FAC_POSIX | (1 << 3) | STS$K_ERROR | STS$M_INHIB_MSG) : SS$_ABORT)

#elif defined(__amigaos4__)
 /* A somewhat experimental attempt to simulate posix return code values */
#   define STATUS_NATIVE	PL_statusvalue_posix
#   define STATUS_NATIVE_CHILD_SET(n)                      \
        STMT_START {                                       \
            PL_statusvalue_posix = (n);                    \
            if (PL_statusvalue_posix < 0) {                \
                PL_statusvalue = -1;                       \
            }                                              \
            else {                                         \
                PL_statusvalue = n << 8;                   \
            }                                              \
        } STMT_END
#   define STATUS_UNIX_SET(n)		\
	STMT_START {			\
	    PL_statusvalue = (n);		\
	    if (PL_statusvalue != -1)	\
		PL_statusvalue &= 0xFFFF;	\
	} STMT_END
#   define STATUS_UNIX_EXIT_SET(n) STATUS_UNIX_SET(n)
#   define STATUS_EXIT_SET(n) STATUS_UNIX_SET(n)
#   define STATUS_CURRENT STATUS_UNIX
#   define STATUS_EXIT STATUS_UNIX
#   define STATUS_ALL_SUCCESS	(PL_statusvalue = 0, PL_statusvalue_posix = 0)
#   define STATUS_ALL_FAILURE	(PL_statusvalue = 1, PL_statusvalue_posix = 1)

#else
#   define STATUS_NATIVE	PL_statusvalue_posix
#   if defined(WCOREDUMP)
#       define STATUS_NATIVE_CHILD_SET(n)                  \
            STMT_START {                                   \
                PL_statusvalue_posix = (n);                \
                if (PL_statusvalue_posix == -1)            \
                    PL_statusvalue = -1;                   \
                else {                                     \
                    PL_statusvalue =                       \
                        (WIFEXITED(PL_statusvalue_posix) ? (WEXITSTATUS(PL_statusvalue_posix) << 8) : 0) |  \
                        (WIFSIGNALED(PL_statusvalue_posix) ? (WTERMSIG(PL_statusvalue_posix) & 0x7F) : 0) | \
                        (WIFSIGNALED(PL_statusvalue_posix) && WCOREDUMP(PL_statusvalue_posix) ? 0x80 : 0);  \
                }                                          \
            } STMT_END
#   elif defined(WIFEXITED)
#       define STATUS_NATIVE_CHILD_SET(n)                  \
            STMT_START {                                   \
                PL_statusvalue_posix = (n);                \
                if (PL_statusvalue_posix == -1)            \
                    PL_statusvalue = -1;                   \
                else {                                     \
                    PL_statusvalue =                       \
                        (WIFEXITED(PL_statusvalue_posix) ? (WEXITSTATUS(PL_statusvalue_posix) << 8) : 0) |  \
                        (WIFSIGNALED(PL_statusvalue_posix) ? (WTERMSIG(PL_statusvalue_posix) & 0x7F) : 0);  \
                }                                          \
            } STMT_END
#   else
#       define STATUS_NATIVE_CHILD_SET(n)                  \
            STMT_START {                                   \
                PL_statusvalue_posix = (n);                \
                if (PL_statusvalue_posix == -1)            \
                    PL_statusvalue = -1;                   \
                else {                                     \
                    PL_statusvalue =                       \
                        PL_statusvalue_posix & 0xFFFF;     \
                }                                          \
            } STMT_END
#   endif
#   define STATUS_UNIX_SET(n)		\
	STMT_START {			\
	    PL_statusvalue = (n);		\
	    if (PL_statusvalue != -1)	\
		PL_statusvalue &= 0xFFFF;	\
	} STMT_END
#   define STATUS_UNIX_EXIT_SET(n) STATUS_UNIX_SET(n)
#   define STATUS_EXIT_SET(n) STATUS_UNIX_SET(n)
#   define STATUS_CURRENT STATUS_UNIX
#   define STATUS_EXIT STATUS_UNIX
#   define STATUS_ALL_SUCCESS	(PL_statusvalue = 0, PL_statusvalue_posix = 0)
#   define STATUS_ALL_FAILURE	(PL_statusvalue = 1, PL_statusvalue_posix = 1)
#endif

/* flags in PL_exit_flags for nature of exit() */
#define PERL_EXIT_EXPECTED	0x01
#define PERL_EXIT_DESTRUCT_END  0x02  /* Run END in perl_destruct */
#define PERL_EXIT_WARN		0x04  /* Warn if Perl_my_exit() or Perl_my_failure_exit() called */
#define PERL_EXIT_ABORT		0x08  /* Call abort() if Perl_my_exit() or Perl_my_failure_exit() called */

#ifndef PERL_CORE
/* format to use for version numbers in file/directory names */
/* XXX move to Configure? */
/* This was only ever used for the current version, and that can be done at
   compile time, as PERL_FS_VERSION, so should we just delete it?  */
#  ifndef PERL_FS_VER_FMT
#    define PERL_FS_VER_FMT	"%d.%d.%d"
#  endif
#endif

#ifndef PERL_FS_VERSION
#  define PERL_FS_VERSION	PERL_VERSION_STRING
#endif

/* This defines a way to flush all output buffers.  This may be a
 * performance issue, so we allow people to disable it.  Also, if
 * we are using stdio, there are broken implementations of fflush(NULL)
 * out there, Solaris being the most prominent.
 */
#ifndef PERL_FLUSHALL_FOR_CHILD
# if defined(USE_PERLIO) || defined(FFLUSH_NULL)
#  define PERL_FLUSHALL_FOR_CHILD	PerlIO_flush((PerlIO*)NULL)
# elif defined(FFLUSH_ALL)
#  define PERL_FLUSHALL_FOR_CHILD	my_fflush_all()
# else
#  define PERL_FLUSHALL_FOR_CHILD	NOOP
# endif
#endif

#ifndef PERL_WAIT_FOR_CHILDREN
#  define PERL_WAIT_FOR_CHILDREN	NOOP
#endif

/* the traditional thread-unsafe notion of "current interpreter". */
#ifndef PERL_SET_INTERP
#  define PERL_SET_INTERP(i)		(PL_curinterp = (PerlInterpreter*)(i))
#endif

#ifndef PERL_GET_INTERP
#  define PERL_GET_INTERP		(PL_curinterp)
#endif

#if defined(PERL_IMPLICIT_CONTEXT) && !defined(PERL_GET_THX)
#  ifdef MULTIPLICITY
#    define PERL_GET_THX		((PerlInterpreter *)PERL_GET_CONTEXT)
#  endif
#  define PERL_SET_THX(t)		PERL_SET_CONTEXT(t)
#endif

/*
    This replaces the previous %_ "hack" by the "%p" hacks.
    All that is required is that the perl source does not
    use "%-p" or "%-<number>p" or "%<number>p" formats.
    These formats will still work in perl code.
    See comments in sv.c for further details.

    Robin Barker 2005-07-14

    No longer use %1p for VDf = %vd.  RMB 2007-10-19
*/

#ifndef SVf_
#  define SVf_(n) "-" STRINGIFY(n) "p"
#endif

#ifndef SVf
#  define SVf "-p"
#endif

#ifndef SVf32
#  define SVf32 SVf_(32)
#endif

#ifndef SVf256
#  define SVf256 SVf_(256)
#endif

#define SVfARG(p) ((void*)(p))

#ifndef HEKf
#  define HEKf "2p"
#endif

/* Not ideal, but we cannot easily include a number in an already-numeric
 * format sequence. */
#ifndef HEKf256
#  define HEKf256 "3p"
#endif

#define HEKfARG(p) ((void*)(p))

/*
=for apidoc Amnh||UTF8f
=for apidoc Amh||UTF8fARG|bool is_utf8|Size_t byte_len|char *str

=cut
 * %4p is a custom format
 */
#ifndef UTF8f
#  define UTF8f "d%" UVuf "%4p"
#endif
#define UTF8fARG(u,l,p) (int)cBOOL(u), (UV)(l), (void*)(p)

#define PNf UTF8f
#define PNfARG(pn) (int)1, (UV)PadnameLEN(pn), (void *)PadnamePV(pn)

#ifdef PERL_CORE
/* not used; but needed for backward compatibility with XS code? - RMB */
#  undef UVf
#elif !defined(UVf)
#  define UVf UVuf
#endif

#if !defined(DEBUGGING) && !defined(NDEBUG)
#  define NDEBUG 1
#endif
#include <assert.h>

/* For functions that are marked as __attribute__noreturn__, it's not
   appropriate to call return.  In either case, include the lint directive.
 */
#ifdef HASATTRIBUTE_NORETURN
#  define NORETURN_FUNCTION_END NOT_REACHED;
#else
#  define NORETURN_FUNCTION_END NOT_REACHED; return 0
#endif

#ifdef HAS_BUILTIN_EXPECT
#  define EXPECT(expr,val)                  __builtin_expect(expr,val)
#else
#  define EXPECT(expr,val)                  (expr)
#endif

/*
=head1 Miscellaneous Functions

=for apidoc AmU|bool|LIKELY|const bool expr

Returns the input unchanged, but at the same time it gives a branch prediction
hint to the compiler that this condition is likely to be true.

=for apidoc AmU|bool|UNLIKELY|const bool expr

Returns the input unchanged, but at the same time it gives a branch prediction
hint to the compiler that this condition is likely to be false.

=cut
*/
#define LIKELY(cond)                        EXPECT(cBOOL(cond),TRUE)
#define UNLIKELY(cond)                      EXPECT(cBOOL(cond),FALSE)

#ifdef HAS_BUILTIN_CHOOSE_EXPR
/* placeholder */
#endif

/* STATIC_ASSERT_DECL/STATIC_ASSERT_STMT are like assert(), but for compile
   time invariants. That is, their argument must be a constant expression that
   can be verified by the compiler. This expression can contain anything that's
   known to the compiler, e.g. #define constants, enums, or sizeof (...). If
   the expression evaluates to 0, compilation fails.
   Because they generate no runtime code (i.e.  their use is "free"), they're
   always active, even under non-DEBUGGING builds.
   STATIC_ASSERT_DECL expands to a declaration and is suitable for use at
   file scope (outside of any function).
   STATIC_ASSERT_STMT expands to a statement and is suitable for use inside a
   function.
*/
#if (! defined(__IBMC__) || __IBMC__ >= 1210)                               \
 && ((   defined(static_assert) && (   defined(_ISOC11_SOURCE)              \
                                    || (__STDC_VERSION__ - 0) >= 201101L))  \
     || (defined(__cplusplus) && __cplusplus >= 201103L))
/* XXX static_assert is a macro defined in <assert.h> in C11 or a compiler
   builtin in C++11.  But IBM XL C V11 does not support _Static_assert, no
   matter what <assert.h> says.
*/
#  define STATIC_ASSERT_DECL(COND) static_assert(COND, #COND)
#else
/* We use a bit-field instead of an array because gcc accepts
   'typedef char x[n]' where n is not a compile-time constant.
   We want to enforce constantness.
*/
#  define STATIC_ASSERT_2(COND, SUFFIX) \
    typedef struct { \
        unsigned int _static_assertion_failed_##SUFFIX : (COND) ? 1 : -1; \
    } _static_assertion_failed_##SUFFIX PERL_UNUSED_DECL
#  define STATIC_ASSERT_1(COND, SUFFIX) STATIC_ASSERT_2(COND, SUFFIX)
#  define STATIC_ASSERT_DECL(COND)    STATIC_ASSERT_1(COND, __LINE__)
#endif
/* We need this wrapper even in C11 because 'case X: static_assert(...);' is an
   error (static_assert is a declaration, and only statements can have labels).
*/
#define STATIC_ASSERT_STMT(COND)      STMT_START { STATIC_ASSERT_DECL(COND); } STMT_END

#ifndef __has_builtin
#  define __has_builtin(x) 0 /* not a clang style compiler */
#endif

/* ASSUME is like assert(), but it has a benefit in a release build. It is a
   hint to a compiler about a statement of fact in a function call free
   expression, which allows the compiler to generate better machine code.
   In a debug build, ASSUME(x) is a synonym for assert(x). ASSUME(0) means
   the control path is unreachable. In a for loop, ASSUME can be used to hint
   that a loop will run at least X times. ASSUME is based off MSVC's __assume
   intrinsic function, see its documents for more details.
*/

#ifndef DEBUGGING
#  if __has_builtin(__builtin_unreachable) \
     || (__GNUC__ == 4 && __GNUC_MINOR__ >= 5 || __GNUC__ > 4) /* 4.5 -> */
#    define ASSUME(x) ((x) ? (void) 0 : __builtin_unreachable())
#  elif defined(_MSC_VER)
#    define ASSUME(x) __assume(x)
#  elif defined(__ARMCC_VERSION) /* untested */
#    define ASSUME(x) __promise(x)
#  else
/* a random compiler might define assert to its own special optimization token
   so pass it through to C lib as a last resort */
#    define ASSUME(x) assert(x)
#  endif
#else
#  define ASSUME(x) assert(x)
#endif

#if defined(__sun)      /* ASSUME() generates warnings on Solaris */
#  define NOT_REACHED
#elif defined(DEBUGGING) && (__has_builtin(__builtin_unreachable) \
     || (__GNUC__ == 4 && __GNUC_MINOR__ >= 5 || __GNUC__ > 4)) /* 4.5 -> */
#  define NOT_REACHED STMT_START { ASSUME(!"UNREACHABLE"); __builtin_unreachable(); } STMT_END
#else
#  define NOT_REACHED ASSUME(!"UNREACHABLE")
#endif

/* Some unistd.h's give a prototype for pause() even though
   HAS_PAUSE ends up undefined.  This causes the #define
   below to be rejected by the compiler.  Sigh.
*/
#ifdef HAS_PAUSE
#define Pause	pause
#else
#define Pause() sleep((32767<<16)+32767)
#endif

#ifndef IOCPARM_LEN
#   ifdef IOCPARM_MASK
	/* on BSDish systems we're safe */
#	define IOCPARM_LEN(x)  (((x) >> 16) & IOCPARM_MASK)
#   elif defined(_IOC_SIZE) && defined(__GLIBC__)
	/* on Linux systems we're safe; except when we're not [perl #38223] */
#	define IOCPARM_LEN(x) (_IOC_SIZE(x) < 256 ? 256 : _IOC_SIZE(x))
#   else
	/* otherwise guess at what's safe */
#	define IOCPARM_LEN(x)	256
#   endif
#endif

#if defined(__CYGWIN__)
/* USEMYBINMODE
 *   This symbol, if defined, indicates that the program should
 *   use the routine my_binmode(FILE *fp, char iotype, int mode) to insure
 *   that a file is in "binary" mode -- that is, that no translation
 *   of bytes occurs on read or write operations.
 */
#  define USEMYBINMODE /**/
#  include <io.h> /* for setmode() prototype */
#  define my_binmode(fp, iotype, mode) \
            cBOOL(PerlLIO_setmode(fileno(fp), mode) != -1)
#endif

#ifdef __CYGWIN__
void init_os_extras(void);
#endif

#ifdef UNION_ANY_DEFINITION
UNION_ANY_DEFINITION;
#else
union any {
    void*	any_ptr;
    SV*         any_sv;
    SV**        any_svp;
    GV*         any_gv;
    AV*         any_av;
    HV*         any_hv;
    OP*         any_op;
    char*       any_pv;
    char**      any_pvp;
    I32		any_i32;
    U32		any_u32;
    IV		any_iv;
    UV		any_uv;
    long	any_long;
    bool	any_bool;
    void	(*any_dptr) (void*);
    void	(*any_dxptr) (pTHX_ void*);
};
#endif

typedef I32 (*filter_t) (pTHX_ int, SV *, int);

#define FILTER_READ(idx, sv, len)  filter_read(idx, sv, len)
#define FILTER_DATA(idx) \
	    (PL_parser ? AvARRAY(PL_parser->rsfp_filters)[idx] : NULL)
#define FILTER_ISREADER(idx) \
	    (PL_parser && PL_parser->rsfp_filters \
		&& idx >= AvFILLp(PL_parser->rsfp_filters))
#define PERL_FILTER_EXISTS(i) \
	    (PL_parser && PL_parser->rsfp_filters \
		&& (i) <= av_tindex(PL_parser->rsfp_filters))

#if defined(_AIX) && !defined(_AIX43)
#if defined(USE_REENTRANT) || defined(_REENTRANT) || defined(_THREAD_SAFE)
/* We cannot include <crypt.h> to get the struct crypt_data
 * because of setkey prototype problems when threading */
typedef        struct crypt_data {     /* straight from /usr/include/crypt.h */
    /* From OSF, Not needed in AIX
       char C[28], D[28];
    */
    char E[48];
    char KS[16][48];
    char block[66];
    char iobuf[16];
} CRYPTD;
#endif /* threading */
#endif /* AIX */

#ifndef PERL_CALLCONV
#  ifdef __cplusplus
#    define PERL_CALLCONV extern "C"
#  else
#    define PERL_CALLCONV
#  endif
#endif
#ifndef PERL_CALLCONV_NO_RET
#    define PERL_CALLCONV_NO_RET PERL_CALLCONV
#endif

/* PERL_STATIC_NO_RET is supposed to be equivalent to STATIC on builds that
   dont have a noreturn as a declaration specifier
*/
#ifndef PERL_STATIC_NO_RET
#  define PERL_STATIC_NO_RET STATIC
#endif
/* PERL_STATIC_NO_RET is supposed to be equivalent to PERL_STATIC_INLINE on
   builds that dont have a noreturn as a declaration specifier
*/
#ifndef PERL_STATIC_INLINE_NO_RET
#  define PERL_STATIC_INLINE_NO_RET PERL_STATIC_INLINE
#endif

#ifndef PERL_STATIC_FORCE_INLINE
#  define PERL_STATIC_FORCE_INLINE PERL_STATIC_INLINE
#endif

#ifndef PERL_STATIC_FORCE_INLINE_NO_RET
#  define PERL_STATIC_FORCE_INLINE_NO_RET PERL_STATIC_INLINE
#endif

#if !defined(OS2)
#  include "iperlsys.h"
#endif

#ifdef __LIBCATAMOUNT__
#undef HAS_PASSWD  /* unixish.h but not unixish enough. */
#undef HAS_GROUP
#define FAKE_BIT_BUCKET
#endif

/* [perl #22371] Algorimic Complexity Attack on Perl 5.6.1, 5.8.0.
 * Note that the USE_HASH_SEED and similar defines are *NOT* defined by
 * Configure, despite their names being similar to other defines like
 * USE_ITHREADS.  Configure in fact knows nothing about the randomised
 * hashes.  Therefore to enable/disable the hash randomisation defines
 * use the Configure -Accflags=... instead. */
#if !defined(NO_HASH_SEED) && !defined(USE_HASH_SEED)
#  define USE_HASH_SEED
#endif

#include "perly.h"


/* macros to define bit-fields in structs. */
#ifndef PERL_BITFIELD8
#  define PERL_BITFIELD8 U8
#endif
#ifndef PERL_BITFIELD16
#  define PERL_BITFIELD16 U16
#endif
#ifndef PERL_BITFIELD32
#  define PERL_BITFIELD32 U32
#endif

#include "sv.h"
#include "regexp.h"
#include "util.h"
#include "form.h"
#include "gv.h"
#include "pad.h"
#include "cv.h"
#include "opnames.h"
#include "op.h"
#include "hv.h"
#include "cop.h"
#include "av.h"
#include "mg.h"
#include "scope.h"
#include "warnings.h"
#include "utf8.h"

/* these would be in doio.h if there was such a file */
#define my_stat()  my_stat_flags(SV_GMAGIC)
#define my_lstat() my_lstat_flags(SV_GMAGIC)

/* defined in sv.c, but also used in [ach]v.c */
#undef _XPV_HEAD
#undef _XPVMG_HEAD
#undef _XPVCV_COMMON

#include "parser.h"

typedef struct magic_state MGS;	/* struct magic_state defined in mg.c */

#if defined(PERL_IN_REGCOMP_C) || defined(PERL_IN_REGEXEC_C)

/* These have to be predeclared, as they are used in proto.h which is #included
 * before their definitions in regcomp.h. */

struct scan_data_t;
typedef struct regnode_charclass regnode_charclass;

/* A hopefully less confusing name.  The sub-classes are all Posix classes only
 * used under /l matching */
typedef struct regnode_charclass_posixl regnode_charclass_class;
typedef struct regnode_charclass_posixl regnode_charclass_posixl;

typedef struct regnode_ssc regnode_ssc;
typedef struct RExC_state_t RExC_state_t;
struct _reg_trie_data;

#endif

struct ptr_tbl_ent {
    struct ptr_tbl_ent*		next;
    const void*			oldval;
    void*			newval;
};

struct ptr_tbl {
    struct ptr_tbl_ent**	tbl_ary;
    UV				tbl_max;
    UV				tbl_items;
    struct ptr_tbl_arena	*tbl_arena;
    struct ptr_tbl_ent		*tbl_arena_next;
    struct ptr_tbl_ent		*tbl_arena_end;
};

#if defined(htonl) && !defined(HAS_HTONL)
#define HAS_HTONL
#endif
#if defined(htons) && !defined(HAS_HTONS)
#define HAS_HTONS
#endif
#if defined(ntohl) && !defined(HAS_NTOHL)
#define HAS_NTOHL
#endif
#if defined(ntohs) && !defined(HAS_NTOHS)
#define HAS_NTOHS
#endif
#ifndef HAS_HTONL
#define HAS_HTONS
#define HAS_HTONL
#define HAS_NTOHS
#define HAS_NTOHL
#  if (BYTEORDER & 0xffff) == 0x4321
/* Big endian system, so ntohl, ntohs, htonl and htons do not need to
   re-order their values. However, to behave identically to the alternative
   implementations, they should truncate to the correct size.  */
#    define ntohl(x)    ((x)&0xFFFFFFFF)
#    define htonl(x)    ntohl(x)
#    define ntohs(x)    ((x)&0xFFFF)
#    define htons(x)    ntohs(x)
#  elif BYTEORDER == 0x1234 || BYTEORDER == 0x12345678

/* Note that we can't straight out declare our own htonl and htons because
   the Win32 build process forcibly undefines HAS_HTONL etc for its miniperl,
   to avoid the overhead of initialising the socket subsystem, but the headers
   that *declare* the various functions are still seen. If we declare our own
   htonl etc they will clash with the declarations in the Win32 headers.  */

PERL_STATIC_INLINE U32
my_swap32(const U32 x) {
    return ((x & 0xFF) << 24) | ((x >> 24) & 0xFF)	
        | ((x & 0x0000FF00) << 8) | ((x & 0x00FF0000) >> 8);
}

PERL_STATIC_INLINE U16
my_swap16(const U16 x) {
    return ((x & 0xFF) << 8) | ((x >> 8) & 0xFF);
}

#    define htonl(x)    my_swap32(x)
#    define ntohl(x)    my_swap32(x)
#    define ntohs(x)    my_swap16(x)
#    define htons(x)    my_swap16(x)
#  else
#    error "Unsupported byteorder"
/* The C pre-processor doesn't let us return the value of BYTEORDER as part of
   the error message. Please check the value of the macro BYTEORDER, as defined
   in config.h. The values of BYTEORDER we expect are

	    big endian  little endian
   32 bit       0x4321  0x1234
   64 bit   0x87654321  0x12345678

   If you have a system with a different byte order, please see
   pod/perlhack.pod for how to submit a patch to add supporting code.
*/
#  endif
#endif

/*
 * Little-endian byte order functions - 'v' for 'VAX', or 'reVerse'.
 * -DWS
 */
#if BYTEORDER == 0x1234 || BYTEORDER == 0x12345678
/* Little endian system, so vtohl, vtohs, htovl and htovs do not need to
   re-order their values. However, to behave identically to the alternative
   implementations, they should truncate to the correct size.  */
#  define vtohl(x)      ((x)&0xFFFFFFFF)
#  define vtohs(x)      ((x)&0xFFFF)
#  define htovl(x)      vtohl(x)
#  define htovs(x)      vtohs(x)
#elif BYTEORDER == 0x4321 || BYTEORDER == 0x87654321
#  define vtohl(x)	((((x)&0xFF)<<24)	\
			+(((x)>>24)&0xFF)	\
			+(((x)&0x0000FF00)<<8)	\
			+(((x)&0x00FF0000)>>8)	)
#  define vtohs(x)	((((x)&0xFF)<<8) + (((x)>>8)&0xFF))
#  define htovl(x)	vtohl(x)
#  define htovs(x)	vtohs(x)
#else
#  error "Unsupported byteorder"
/* If you have need for current perl on PDP-11 or similar, and can help test
   that blead keeps working on a mixed-endian system, then see
   pod/perlhack.pod for how to submit patches to things working again.  */
#endif

/* *MAX Plus 1. A floating point value.
   Hopefully expressed in a way that dodgy floating point can't mess up.
   >> 2 rather than 1, so that value is safely less than I32_MAX after 1
   is added to it
   May find that some broken compiler will want the value cast to I32.
   [after the shift, as signed >> may not be as secure as unsigned >>]
*/
#define I32_MAX_P1 (2.0 * (1 + (((U32)I32_MAX) >> 1)))
#define U32_MAX_P1 (4.0 * (1 + ((U32_MAX) >> 2)))
/* For compilers that can't correctly cast NVs over 0x7FFFFFFF (or
   0x7FFFFFFFFFFFFFFF) to an unsigned integer. In the future, sizeof(UV)
   may be greater than sizeof(IV), so don't assume that half max UV is max IV.
*/
#define U32_MAX_P1_HALF (2.0 * (1 + ((U32_MAX) >> 2)))

#define UV_MAX_P1 (4.0 * (1 + ((UV_MAX) >> 2)))
#define IV_MAX_P1 (2.0 * (1 + (((UV)IV_MAX) >> 1)))
#define UV_MAX_P1_HALF (2.0 * (1 + ((UV_MAX) >> 2)))

/* This may look like unnecessary jumping through hoops, but converting
   out of range floating point values to integers *is* undefined behaviour,
   and it is starting to bite.
*/
#ifndef CAST_INLINE
#define I_32(what) (cast_i32((NV)(what)))
#define U_32(what) (cast_ulong((NV)(what)))
#define I_V(what) (cast_iv((NV)(what)))
#define U_V(what) (cast_uv((NV)(what)))
#else
#define I_32(n) ((n) < I32_MAX_P1 ? ((n) < I32_MIN ? I32_MIN : (I32) (n)) \
                  : ((n) < U32_MAX_P1 ? (I32)(U32) (n) \
                     : ((n) > 0 ? (I32) U32_MAX : 0 /* NaN */)))
#define U_32(n) ((n) < 0.0 ? ((n) < I32_MIN ? (UV) I32_MIN : (U32)(I32) (n)) \
                  : ((n) < U32_MAX_P1 ? (U32) (n) \
                     : ((n) > 0 ? U32_MAX : 0 /* NaN */)))
#define I_V(n) (LIKELY((n) < IV_MAX_P1) ? (UNLIKELY((n) < IV_MIN) ? IV_MIN : (IV) (n)) \
                  : (LIKELY((n) < UV_MAX_P1) ? (IV)(UV) (n) \
                     : ((n) > 0 ? (IV)UV_MAX : 0 /* NaN */)))
#define U_V(n) ((n) < 0.0 ? (UNLIKELY((n) < IV_MIN) ? (UV) IV_MIN : (UV)(IV) (n)) \
                  : (LIKELY((n) < UV_MAX_P1) ? (UV) (n) \
                     : ((n) > 0 ? UV_MAX : 0 /* NaN */)))
#endif

#define U_S(what) ((U16)U_32(what))
#define U_I(what) ((unsigned int)U_32(what))
#define U_L(what) U_32(what)

#ifdef HAS_SIGNBIT
#  ifndef Perl_signbit
#    define Perl_signbit signbit
#  endif
#endif

/* These do not care about the fractional part, only about the range. */
#define NV_WITHIN_IV(nv) (I_V(nv) >= IV_MIN && I_V(nv) <= IV_MAX)
#define NV_WITHIN_UV(nv) ((nv)>=0.0 && U_V(nv) >= UV_MIN && U_V(nv) <= UV_MAX)

/* Used with UV/IV arguments: */
					/* XXXX: need to speed it up */
#define CLUMP_2UV(iv)	((iv) < 0 ? 0 : (UV)(iv))
#define CLUMP_2IV(uv)	((uv) > (UV)IV_MAX ? IV_MAX : (IV)(uv))

#ifndef MAXSYSFD
#   define MAXSYSFD 2
#endif

#ifndef __cplusplus
#if !(defined(WIN32) || defined(SYMBIAN))
Uid_t getuid (void);
Uid_t geteuid (void);
Gid_t getgid (void);
Gid_t getegid (void);
#endif
#endif

#ifndef Perl_debug_log
#  define Perl_debug_log	PerlIO_stderr()
#endif

#ifndef Perl_error_log
#  define Perl_error_log	(PL_stderrgv			\
				 && isGV(PL_stderrgv)		\
				 && GvIOp(PL_stderrgv)          \
				 && IoOFP(GvIOp(PL_stderrgv))	\
				 ? IoOFP(GvIOp(PL_stderrgv))	\
				 : PerlIO_stderr())
#endif


#define DEBUG_p_FLAG		0x00000001 /*      1 */
#define DEBUG_s_FLAG		0x00000002 /*      2 */
#define DEBUG_l_FLAG		0x00000004 /*      4 */
#define DEBUG_t_FLAG		0x00000008 /*      8 */
#define DEBUG_o_FLAG		0x00000010 /*     16 */
#define DEBUG_c_FLAG		0x00000020 /*     32 */
#define DEBUG_P_FLAG		0x00000040 /*     64 */
#define DEBUG_m_FLAG		0x00000080 /*    128 */
#define DEBUG_f_FLAG		0x00000100 /*    256 */
#define DEBUG_r_FLAG		0x00000200 /*    512 */
#define DEBUG_x_FLAG		0x00000400 /*   1024 */
#define DEBUG_u_FLAG		0x00000800 /*   2048 */
/* U is reserved for Unofficial, exploratory hacking */
#define DEBUG_U_FLAG		0x00001000 /*   4096 */
/* spare                                        8192 */
#define DEBUG_X_FLAG		0x00004000 /*  16384 */
#define DEBUG_D_FLAG		0x00008000 /*  32768 */
#define DEBUG_S_FLAG		0x00010000 /*  65536 */
#define DEBUG_T_FLAG		0x00020000 /* 131072 */
#define DEBUG_R_FLAG		0x00040000 /* 262144 */
#define DEBUG_J_FLAG		0x00080000 /* 524288 */
#define DEBUG_v_FLAG		0x00100000 /*1048576 */
#define DEBUG_C_FLAG		0x00200000 /*2097152 */
#define DEBUG_A_FLAG		0x00400000 /*4194304 */
#define DEBUG_q_FLAG		0x00800000 /*8388608 */
#define DEBUG_M_FLAG		0x01000000 /*16777216*/
#define DEBUG_B_FLAG		0x02000000 /*33554432*/
#define DEBUG_L_FLAG		0x04000000 /*67108864*/
#define DEBUG_i_FLAG		0x08000000 /*134217728*/
#define DEBUG_y_FLAG		0x10000000 /*268435456*/
#define DEBUG_MASK		0x1FFFEFFF /* mask of all the standard flags */

#define DEBUG_DB_RECURSE_FLAG	0x40000000
#define DEBUG_TOP_FLAG		0x80000000 /* -D was given --> PL_debug |= FLAG */

#  define DEBUG_p_TEST_ UNLIKELY(PL_debug & DEBUG_p_FLAG)
#  define DEBUG_s_TEST_ UNLIKELY(PL_debug & DEBUG_s_FLAG)
#  define DEBUG_l_TEST_ UNLIKELY(PL_debug & DEBUG_l_FLAG)
#  define DEBUG_t_TEST_ UNLIKELY(PL_debug & DEBUG_t_FLAG)
#  define DEBUG_o_TEST_ UNLIKELY(PL_debug & DEBUG_o_FLAG)
#  define DEBUG_c_TEST_ UNLIKELY(PL_debug & DEBUG_c_FLAG)
#  define DEBUG_P_TEST_ UNLIKELY(PL_debug & DEBUG_P_FLAG)
#  define DEBUG_m_TEST_ UNLIKELY(PL_debug & DEBUG_m_FLAG)
#  define DEBUG_f_TEST_ UNLIKELY(PL_debug & DEBUG_f_FLAG)
#  define DEBUG_r_TEST_ UNLIKELY(PL_debug & DEBUG_r_FLAG)
#  define DEBUG_x_TEST_ UNLIKELY(PL_debug & DEBUG_x_FLAG)
#  define DEBUG_u_TEST_ UNLIKELY(PL_debug & DEBUG_u_FLAG)
#  define DEBUG_U_TEST_ UNLIKELY(PL_debug & DEBUG_U_FLAG)
#  define DEBUG_X_TEST_ UNLIKELY(PL_debug & DEBUG_X_FLAG)
#  define DEBUG_D_TEST_ UNLIKELY(PL_debug & DEBUG_D_FLAG)
#  define DEBUG_S_TEST_ UNLIKELY(PL_debug & DEBUG_S_FLAG)
#  define DEBUG_T_TEST_ UNLIKELY(PL_debug & DEBUG_T_FLAG)
#  define DEBUG_R_TEST_ UNLIKELY(PL_debug & DEBUG_R_FLAG)
#  define DEBUG_J_TEST_ UNLIKELY(PL_debug & DEBUG_J_FLAG)
#  define DEBUG_v_TEST_ UNLIKELY(PL_debug & DEBUG_v_FLAG)
#  define DEBUG_C_TEST_ UNLIKELY(PL_debug & DEBUG_C_FLAG)
#  define DEBUG_A_TEST_ UNLIKELY(PL_debug & DEBUG_A_FLAG)
#  define DEBUG_q_TEST_ UNLIKELY(PL_debug & DEBUG_q_FLAG)
#  define DEBUG_M_TEST_ UNLIKELY(PL_debug & DEBUG_M_FLAG)
#  define DEBUG_B_TEST_ UNLIKELY(PL_debug & DEBUG_B_FLAG)
#  define DEBUG_L_TEST_ UNLIKELY(PL_debug & DEBUG_L_FLAG)
#  define DEBUG_i_TEST_ UNLIKELY(PL_debug & DEBUG_i_FLAG)
#  define DEBUG_y_TEST_ UNLIKELY(PL_debug & DEBUG_y_FLAG)
#  define DEBUG_Xv_TEST_ (DEBUG_X_TEST_ && DEBUG_v_TEST_)
#  define DEBUG_Uv_TEST_ (DEBUG_U_TEST_ && DEBUG_v_TEST_)
#  define DEBUG_Pv_TEST_ (DEBUG_P_TEST_ && DEBUG_v_TEST_)
#  define DEBUG_Lv_TEST_ (DEBUG_L_TEST_ && DEBUG_v_TEST_)
#  define DEBUG_yv_TEST_ (DEBUG_y_TEST_ && DEBUG_v_TEST_)

#ifdef DEBUGGING

#  define DEBUG_p_TEST DEBUG_p_TEST_
#  define DEBUG_s_TEST DEBUG_s_TEST_
#  define DEBUG_l_TEST DEBUG_l_TEST_
#  define DEBUG_t_TEST DEBUG_t_TEST_
#  define DEBUG_o_TEST DEBUG_o_TEST_
#  define DEBUG_c_TEST DEBUG_c_TEST_
#  define DEBUG_P_TEST DEBUG_P_TEST_
#  define DEBUG_m_TEST DEBUG_m_TEST_
#  define DEBUG_f_TEST DEBUG_f_TEST_
#  define DEBUG_r_TEST DEBUG_r_TEST_
#  define DEBUG_x_TEST DEBUG_x_TEST_
#  define DEBUG_u_TEST DEBUG_u_TEST_
#  define DEBUG_U_TEST DEBUG_U_TEST_
#  define DEBUG_X_TEST DEBUG_X_TEST_
#  define DEBUG_D_TEST DEBUG_D_TEST_
#  define DEBUG_S_TEST DEBUG_S_TEST_
#  define DEBUG_T_TEST DEBUG_T_TEST_
#  define DEBUG_R_TEST DEBUG_R_TEST_
#  define DEBUG_J_TEST DEBUG_J_TEST_
#  define DEBUG_v_TEST DEBUG_v_TEST_
#  define DEBUG_C_TEST DEBUG_C_TEST_
#  define DEBUG_A_TEST DEBUG_A_TEST_
#  define DEBUG_q_TEST DEBUG_q_TEST_
#  define DEBUG_M_TEST DEBUG_M_TEST_
#  define DEBUG_B_TEST DEBUG_B_TEST_
#  define DEBUG_L_TEST DEBUG_L_TEST_
#  define DEBUG_i_TEST DEBUG_i_TEST_
#  define DEBUG_y_TEST DEBUG_y_TEST_
#  define DEBUG_Xv_TEST DEBUG_Xv_TEST_
#  define DEBUG_Uv_TEST DEBUG_Uv_TEST_
#  define DEBUG_Pv_TEST DEBUG_Pv_TEST_
#  define DEBUG_Lv_TEST DEBUG_Lv_TEST_
#  define DEBUG_yv_TEST DEBUG_yv_TEST_

#  define PERL_DEB(a)                  a
#  define PERL_DEB2(a,b)               a
#  define PERL_DEBUG(a) if (PL_debug)  a
#  define DEBUG_p(a) if (DEBUG_p_TEST) a
#  define DEBUG_s(a) if (DEBUG_s_TEST) a
#  define DEBUG_l(a) if (DEBUG_l_TEST) a
#  define DEBUG_t(a) if (DEBUG_t_TEST) a
#  define DEBUG_o(a) if (DEBUG_o_TEST) a
#  define DEBUG_c(a) if (DEBUG_c_TEST) a
#  define DEBUG_P(a) if (DEBUG_P_TEST) a

     /* Temporarily turn off memory debugging in case the a
      * does memory allocation, either directly or indirectly. */
#  define DEBUG_m(a)  \
    STMT_START {					                \
        if (PERL_GET_INTERP) {                                          \
                                dTHX;                                   \
                                if (DEBUG_m_TEST) {                     \
                                    PL_debug &= ~DEBUG_m_FLAG;          \
                                    a;                                  \
                                    PL_debug |= DEBUG_m_FLAG;           \
                                }                                       \
                              }                                         \
    } STMT_END

#  define DEBUG__(t, a)                                                 \
        STMT_START {                                                    \
                if (t) STMT_START {a;} STMT_END;                        \
        } STMT_END

#  define DEBUG_f(a) DEBUG__(DEBUG_f_TEST, a)

/* For re_comp.c, re_exec.c, assume -Dr has been specified */
#  ifdef PERL_EXT_RE_BUILD
#    define DEBUG_r(a) STMT_START {a;} STMT_END
#  else
#    define DEBUG_r(a) DEBUG__(DEBUG_r_TEST, a)
#  endif /* PERL_EXT_RE_BUILD */

#  define DEBUG_x(a) DEBUG__(DEBUG_x_TEST, a)
#  define DEBUG_u(a) DEBUG__(DEBUG_u_TEST, a)
#  define DEBUG_U(a) DEBUG__(DEBUG_U_TEST, a)
#  define DEBUG_X(a) DEBUG__(DEBUG_X_TEST, a)
#  define DEBUG_D(a) DEBUG__(DEBUG_D_TEST, a)
#  define DEBUG_Xv(a) DEBUG__(DEBUG_Xv_TEST, a)
#  define DEBUG_Uv(a) DEBUG__(DEBUG_Uv_TEST, a)
#  define DEBUG_Pv(a) DEBUG__(DEBUG_Pv_TEST, a)
#  define DEBUG_Lv(a) DEBUG__(DEBUG_Lv_TEST, a)
#  define DEBUG_yv(a) DEBUG__(DEBUG_yv_TEST, a)

#  define DEBUG_S(a) DEBUG__(DEBUG_S_TEST, a)
#  define DEBUG_T(a) DEBUG__(DEBUG_T_TEST, a)
#  define DEBUG_R(a) DEBUG__(DEBUG_R_TEST, a)
#  define DEBUG_v(a) DEBUG__(DEBUG_v_TEST, a)
#  define DEBUG_C(a) DEBUG__(DEBUG_C_TEST, a)
#  define DEBUG_A(a) DEBUG__(DEBUG_A_TEST, a)
#  define DEBUG_q(a) DEBUG__(DEBUG_q_TEST, a)
#  define DEBUG_M(a) DEBUG__(DEBUG_M_TEST, a)
#  define DEBUG_B(a) DEBUG__(DEBUG_B_TEST, a)
#  define DEBUG_L(a) DEBUG__(DEBUG_L_TEST, a)
#  define DEBUG_i(a) DEBUG__(DEBUG_i_TEST, a)
#  define DEBUG_y(a) DEBUG__(DEBUG_y_TEST, a)

#else /* ! DEBUGGING below */

#  define DEBUG_p_TEST (0)
#  define DEBUG_s_TEST (0)
#  define DEBUG_l_TEST (0)
#  define DEBUG_t_TEST (0)
#  define DEBUG_o_TEST (0)
#  define DEBUG_c_TEST (0)
#  define DEBUG_P_TEST (0)
#  define DEBUG_m_TEST (0)
#  define DEBUG_f_TEST (0)
#  define DEBUG_r_TEST (0)
#  define DEBUG_x_TEST (0)
#  define DEBUG_u_TEST (0)
#  define DEBUG_U_TEST (0)
#  define DEBUG_X_TEST (0)
#  define DEBUG_D_TEST (0)
#  define DEBUG_S_TEST (0)
#  define DEBUG_T_TEST (0)
#  define DEBUG_R_TEST (0)
#  define DEBUG_J_TEST (0)
#  define DEBUG_v_TEST (0)
#  define DEBUG_C_TEST (0)
#  define DEBUG_A_TEST (0)
#  define DEBUG_q_TEST (0)
#  define DEBUG_M_TEST (0)
#  define DEBUG_B_TEST (0)
#  define DEBUG_L_TEST (0)
#  define DEBUG_i_TEST (0)
#  define DEBUG_y_TEST (0)
#  define DEBUG_Xv_TEST (0)
#  define DEBUG_Uv_TEST (0)
#  define DEBUG_Pv_TEST (0)
#  define DEBUG_Lv_TEST (0)
#  define DEBUG_yv_TEST (0)

#  define PERL_DEB(a)
#  define PERL_DEB2(a,b)               b
#  define PERL_DEBUG(a)
#  define DEBUG_p(a)
#  define DEBUG_s(a)
#  define DEBUG_l(a)
#  define DEBUG_t(a)
#  define DEBUG_o(a)
#  define DEBUG_c(a)
#  define DEBUG_P(a)
#  define DEBUG_m(a)
#  define DEBUG_f(a)
#  define DEBUG_r(a)
#  define DEBUG_x(a)
#  define DEBUG_u(a)
#  define DEBUG_U(a)
#  define DEBUG_X(a)
#  define DEBUG_D(a)
#  define DEBUG_S(a)
#  define DEBUG_T(a)
#  define DEBUG_R(a)
#  define DEBUG_v(a)
#  define DEBUG_C(a)
#  define DEBUG_A(a)
#  define DEBUG_q(a)
#  define DEBUG_M(a)
#  define DEBUG_B(a)
#  define DEBUG_L(a)
#  define DEBUG_i(a)
#  define DEBUG_y(a)
#  define DEBUG_Xv(a)
#  define DEBUG_Uv(a)
#  define DEBUG_Pv(a)
#  define DEBUG_Lv(a)
#  define DEBUG_yv(a)
#endif /* DEBUGGING */


#define DEBUG_SCOPE(where) \
    DEBUG_l( \
    Perl_deb(aTHX_ "%s scope %ld (savestack=%ld) at %s:%d\n",	\
		    where, (long)PL_scopestack_ix, (long)PL_savestack_ix, \
		    __FILE__, __LINE__));

/* Keep the old croak based assert for those who want it, and as a fallback if
   the platform is so heretically non-ANSI that it can't assert.  */

#define Perl_assert(what)	PERL_DEB2( 				\
	((what) ? ((void) 0) :						\
	    (Perl_croak_nocontext("Assertion %s failed: file \"" __FILE__ \
			"\", line %d", STRINGIFY(what), __LINE__),	\
             (void) 0)), ((void)0))

/* assert() gets defined if DEBUGGING.
 * If no DEBUGGING, the <assert.h> has not been included. */
#ifndef assert
#  define assert(what)	Perl_assert(what)
#endif
#ifdef DEBUGGING
#  define assert_(what)	assert(what),
#else
#  define assert_(what)
#endif

struct ufuncs {
    I32 (*uf_val)(pTHX_ IV, SV*);
    I32 (*uf_set)(pTHX_ IV, SV*);
    IV uf_index;
};

/* In pre-5.7-Perls the PERL_MAGIC_uvar magic didn't get the thread context.
 * XS code wanting to be backward compatible can do something
 * like the following:

#ifndef PERL_MG_UFUNC
#define PERL_MG_UFUNC(name,ix,sv) I32 name(IV ix, SV *sv)
#endif

static PERL_MG_UFUNC(foo_get, index, val)
{
    sv_setsv(val, ...);
    return TRUE;
}

-- Doug MacEachern

*/

#ifndef PERL_MG_UFUNC
#define PERL_MG_UFUNC(name,ix,sv) I32 name(pTHX_ IV ix, SV *sv)
#endif

#include <math.h>
#ifdef __VMS
     /* isfinite and others are here rather than in math.h as C99 stipulates */
#    include <fp.h>
#endif

#ifndef __cplusplus
#  if !defined(WIN32) && !defined(VMS)
#ifndef crypt
char *crypt (const char*, const char*);
#endif
#  endif /* !WIN32 */
#  ifndef WIN32
#    ifndef getlogin
char *getlogin (void);
#    endif
#  endif /* !WIN32 */
#endif /* !__cplusplus */

/* Fixme on VMS.  This needs to be a run-time, not build time options */
/* Also rename() is affected by this */
#ifdef UNLINK_ALL_VERSIONS /* Currently only makes sense for VMS */
#define UNLINK unlnk
I32 unlnk (pTHX_ const char*);
#else
#define UNLINK PerlLIO_unlink
#endif

/* some versions of glibc are missing the setresuid() proto */
#if defined(HAS_SETRESUID) && !defined(HAS_SETRESUID_PROTO)
int setresuid(uid_t ruid, uid_t euid, uid_t suid);
#endif
/* some versions of glibc are missing the setresgid() proto */
#if defined(HAS_SETRESGID) && !defined(HAS_SETRESGID_PROTO)
int setresgid(gid_t rgid, gid_t egid, gid_t sgid);
#endif

#ifndef HAS_SETREUID
#  ifdef HAS_SETRESUID
#    define setreuid(r,e) setresuid(r,e,(Uid_t)-1)
#    define HAS_SETREUID
#  endif
#endif
#ifndef HAS_SETREGID
#  ifdef HAS_SETRESGID
#    define setregid(r,e) setresgid(r,e,(Gid_t)-1)
#    define HAS_SETREGID
#  endif
#endif

/* Sighandler_t defined in iperlsys.h */

#ifdef HAS_SIGACTION
typedef struct sigaction Sigsave_t;
#else
typedef Sighandler_t Sigsave_t;
#endif

#define SCAN_DEF 0
#define SCAN_TR 1
#define SCAN_REPL 2

#ifdef DEBUGGING
# ifndef register
#  define register
# endif
# define RUNOPS_DEFAULT Perl_runops_debug
#else
# define RUNOPS_DEFAULT Perl_runops_standard
#endif

#if defined(USE_PERLIO)
EXTERN_C void PerlIO_teardown(void);
# ifdef USE_ITHREADS
#  define PERLIO_INIT MUTEX_INIT(&PL_perlio_mutex)
#  define PERLIO_TERM 				\
	STMT_START {				\
		PerlIO_teardown();		\
		MUTEX_DESTROY(&PL_perlio_mutex);\
	} STMT_END
# else
#  define PERLIO_INIT
#  define PERLIO_TERM	PerlIO_teardown()
# endif
#else
#  define PERLIO_INIT
#  define PERLIO_TERM
#endif

#ifdef MYMALLOC
#  ifdef MUTEX_INIT_CALLS_MALLOC
#    define MALLOC_INIT					\
	STMT_START {					\
		PL_malloc_mutex = NULL;			\
		MUTEX_INIT(&PL_malloc_mutex);		\
	} STMT_END
#    define MALLOC_TERM					\
	STMT_START {					\
		perl_mutex tmp = PL_malloc_mutex;	\
		PL_malloc_mutex = NULL;			\
		MUTEX_DESTROY(&tmp);			\
	} STMT_END
#  else
#    define MALLOC_INIT MUTEX_INIT(&PL_malloc_mutex)
#    define MALLOC_TERM MUTEX_DESTROY(&PL_malloc_mutex)
#  endif
#else
#  define MALLOC_INIT
#  define MALLOC_TERM
#endif

#if defined(PERL_IMPLICIT_CONTEXT)

struct perl_memory_debug_header;
struct perl_memory_debug_header {
  tTHX	interpreter;
#  if defined(PERL_POISON) || defined(PERL_DEBUG_READONLY_COW)
  MEM_SIZE size;
#  endif
  struct perl_memory_debug_header *prev;
  struct perl_memory_debug_header *next;
#  ifdef PERL_DEBUG_READONLY_COW
  bool readonly;
#  endif
};

#elif defined(PERL_DEBUG_READONLY_COW)

struct perl_memory_debug_header;
struct perl_memory_debug_header {
  MEM_SIZE size;
};

#endif

#if defined (PERL_TRACK_MEMPOOL) || defined (PERL_DEBUG_READONLY_COW)

#  define PERL_MEMORY_DEBUG_HEADER_SIZE \
        (sizeof(struct perl_memory_debug_header) + \
	(MEM_ALIGNBYTES - sizeof(struct perl_memory_debug_header) \
	 %MEM_ALIGNBYTES) % MEM_ALIGNBYTES)

#else
#  define PERL_MEMORY_DEBUG_HEADER_SIZE	0
#endif

#ifdef PERL_TRACK_MEMPOOL
# ifdef PERL_DEBUG_READONLY_COW
#  define INIT_TRACK_MEMPOOL(header, interp)			\
	STMT_START {						\
		(header).interpreter = (interp);		\
		(header).prev = (header).next = &(header);	\
		(header).readonly = 0;				\
	} STMT_END
# else
#  define INIT_TRACK_MEMPOOL(header, interp)			\
	STMT_START {						\
		(header).interpreter = (interp);		\
		(header).prev = (header).next = &(header);	\
	} STMT_END
# endif
# else
#  define INIT_TRACK_MEMPOOL(header, interp)
#endif

#ifdef I_MALLOCMALLOC
/* Needed for malloc_size(), malloc_good_size() on some systems */
#  include <malloc/malloc.h>
#endif

#ifdef MYMALLOC
#  define Perl_safesysmalloc_size(where)	Perl_malloced_size(where)
#else
#  if defined(HAS_MALLOC_SIZE) && !defined(PERL_DEBUG_READONLY_COW)
#    ifdef PERL_TRACK_MEMPOOL
#	define Perl_safesysmalloc_size(where)			\
	    (malloc_size(((char *)(where)) - PERL_MEMORY_DEBUG_HEADER_SIZE) - PERL_MEMORY_DEBUG_HEADER_SIZE)
#    else
#	define Perl_safesysmalloc_size(where) malloc_size(where)
#    endif
#  endif
#  ifdef HAS_MALLOC_GOOD_SIZE
#    ifdef PERL_TRACK_MEMPOOL
#	define Perl_malloc_good_size(how_much)			\
	    (malloc_good_size((how_much) + PERL_MEMORY_DEBUG_HEADER_SIZE) - PERL_MEMORY_DEBUG_HEADER_SIZE)
#    else
#	define Perl_malloc_good_size(how_much) malloc_good_size(how_much)
#    endif
#  else
/* Having this as the identity operation makes some code simpler.  */
#	define Perl_malloc_good_size(how_much)	(how_much)
#  endif
#endif

typedef int (*runops_proc_t)(pTHX);
typedef void (*share_proc_t) (pTHX_ SV *sv);
typedef int  (*thrhook_proc_t) (pTHX);
typedef OP* (*PPADDR_t[]) (pTHX);
typedef bool (*destroyable_proc_t) (pTHX_ SV *sv);
typedef void (*despatch_signals_proc_t) (pTHX);

#if defined(__DYNAMIC__) && defined(PERL_DARWIN) && defined(PERL_CORE)
#  include <crt_externs.h>	/* for the env array */
#  define environ (*_NSGetEnviron())
#elif defined(USE_ENVIRON_ARRAY) && !defined(environ)
   /* VMS and some other platforms don't use the environ array */
EXTERN_C char **environ;  /* environment variables supplied via exec */
#endif

#define PERL_PATCHLEVEL_H_IMPLICIT
#include "patchlevel.h"
#undef PERL_PATCHLEVEL_H_IMPLICIT

#define PERL_VERSION_STRING	STRINGIFY(PERL_REVISION) "." \
				STRINGIFY(PERL_VERSION) "." \
				STRINGIFY(PERL_SUBVERSION)

#define PERL_API_VERSION_STRING	STRINGIFY(PERL_API_REVISION) "." \
				STRINGIFY(PERL_API_VERSION) "." \
				STRINGIFY(PERL_API_SUBVERSION)

START_EXTERN_C

/* handy constants */
EXTCONST char PL_warn_uninit[]
  INIT("Use of uninitialized value%s%s%s");
EXTCONST char PL_warn_uninit_sv[]
  INIT("Use of uninitialized value%" SVf "%s%s");
EXTCONST char PL_warn_nosemi[]
  INIT("Semicolon seems to be missing");
EXTCONST char PL_warn_reserved[]
  INIT("Unquoted string \"%s\" may clash with future reserved word");
EXTCONST char PL_warn_nl[]
  INIT("Unsuccessful %s on filename containing newline");
EXTCONST char PL_no_wrongref[]
  INIT("Can't use %s ref as %s ref");
/* The core no longer needs this here. If you require the string constant,
   please inline a copy into your own code.  */
EXTCONST char PL_no_symref[] __attribute__deprecated__
  INIT("Can't use string (\"%.32s\") as %s ref while \"strict refs\" in use");
EXTCONST char PL_no_symref_sv[]
  INIT("Can't use string (\"%" SVf32 "\"%s) as %s ref while \"strict refs\" in use");

EXTCONST char PL_no_usym[]
  INIT("Can't use an undefined value as %s reference");
EXTCONST char PL_no_aelem[]
  INIT("Modification of non-creatable array value attempted, subscript %d");
EXTCONST char PL_no_helem_sv[]
  INIT("Modification of non-creatable hash value attempted, subscript \"%" SVf "\"");
EXTCONST char PL_no_modify[]
  INIT("Modification of a read-only value attempted");
EXTCONST char PL_no_mem[sizeof("Out of memory!\n")]
  INIT("Out of memory!\n");
EXTCONST char PL_no_security[]
  INIT("Insecure dependency in %s%s");
EXTCONST char PL_no_sock_func[]
  INIT("Unsupported socket function \"%s\" called");
EXTCONST char PL_no_dir_func[]
  INIT("Unsupported directory function \"%s\" called");
EXTCONST char PL_no_func[]
  INIT("The %s function is unimplemented");
EXTCONST char PL_no_myglob[]
  INIT("\"%s\" %s %s can't be in a package");
EXTCONST char PL_no_localize_ref[]
  INIT("Can't localize through a reference");
EXTCONST char PL_memory_wrap[]
  INIT("panic: memory wrap");
EXTCONST char PL_extended_cp_format[]
  INIT("Code point 0x%" UVXf " is not Unicode, requires a Perl extension,"
                             " and so is not portable");
EXTCONST char PL_Yes[]
  INIT("1");
EXTCONST char PL_No[]
  INIT("");
EXTCONST char PL_Zero[]
  INIT("0");
EXTCONST char PL_hexdigit[]
  INIT("0123456789abcdef0123456789ABCDEF");

EXTCONST STRLEN PL_WARN_ALL
  INIT(0);
EXTCONST STRLEN PL_WARN_NONE
  INIT(0);

/* This is constant on most architectures, a global on OS/2 */
#ifndef OS2
EXTCONST char PL_sh_path[]
  INIT(SH_PATH); /* full path of shell */
#endif

#ifdef CSH
EXTCONST char PL_cshname[]
  INIT(CSH);
#  define PL_cshlen	(sizeof(CSH "") - 1)
#endif

/* These are baked at compile time into any shared perl library.
   In future releases this will allow us in main() to sanity test the
   library we're linking against.  */

EXTCONST U8 PL_revision
  INIT(PERL_REVISION);
EXTCONST U8 PL_version
  INIT(PERL_VERSION);
EXTCONST U8 PL_subversion
  INIT(PERL_SUBVERSION);

EXTCONST char PL_uuemap[65]
  INIT("`!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_");

/* a special string address whose value is "isa", but which perl knows
 * to treat as if it were really "DOES" when printing the method name in
 *  the "Can't call method '%s'" error message */
EXTCONST char PL_isa_DOES[]
  INIT("isa");

#ifdef DOINIT
EXTCONST char PL_uudmap[256] =
#  ifdef PERL_MICRO
#    include "uuudmap.h"
#  else
#    include "uudmap.h"
#  endif
;
EXTCONST char PL_bitcount[256] =
#  ifdef PERL_MICRO
#    include "ubitcount.h"
#else
#    include "bitcount.h"
#  endif
;
EXTCONST char* const PL_sig_name[] = { SIG_NAME };
EXTCONST int         PL_sig_num[]  = { SIG_NUM };
#else
EXTCONST char PL_uudmap[256];
EXTCONST char PL_bitcount[256];
EXTCONST char* const PL_sig_name[];
EXTCONST int         PL_sig_num[];
#endif

/* fast conversion and case folding tables.  The folding tables complement the
 * fold, so that 'a' maps to 'A' and 'A' maps to 'a', ignoring more complicated
 * folds such as outside the range or to multiple characters. */

#ifdef DOINIT
#ifndef EBCDIC

/* The EBCDIC fold table depends on the code page, and hence is found in
 * utfebcdic.h */

EXTCONST  unsigned char PL_fold[] = {
	0,	1,	2,	3,	4,	5,	6,	7,
	8,	9,	10,	11,	12,	13,	14,	15,
	16,	17,	18,	19,	20,	21,	22,	23,
	24,	25,	26,	27,	28,	29,	30,	31,
	32,	33,	34,	35,	36,	37,	38,	39,
	40,	41,	42,	43,	44,	45,	46,	47,
	48,	49,	50,	51,	52,	53,	54,	55,
	56,	57,	58,	59,	60,	61,	62,	63,
	64,	'a',	'b',	'c',	'd',	'e',	'f',	'g',
	'h',	'i',	'j',	'k',	'l',	'm',	'n',	'o',
	'p',	'q',	'r',	's',	't',	'u',	'v',	'w',
	'x',	'y',	'z',	91,	92,	93,	94,	95,
	96,	'A',	'B',	'C',	'D',	'E',	'F',	'G',
	'H',	'I',	'J',	'K',	'L',	'M',	'N',	'O',
	'P',	'Q',	'R',	'S',	'T',	'U',	'V',	'W',
	'X',	'Y',	'Z',	123,	124,	125,	126,	127,
	128,	129,	130,	131,	132,	133,	134,	135,
	136,	137,	138,	139,	140,	141,	142,	143,
	144,	145,	146,	147,	148,	149,	150,	151,
	152,	153,	154,	155,	156,	157,	158,	159,
	160,	161,	162,	163,	164,	165,	166,	167,
	168,	169,	170,	171,	172,	173,	174,	175,
	176,	177,	178,	179,	180,	181,	182,	183,
	184,	185,	186,	187,	188,	189,	190,	191,
	192,	193,	194,	195,	196,	197,	198,	199,
	200,	201,	202,	203,	204,	205,	206,	207,
	208,	209,	210,	211,	212,	213,	214,	215,
	216,	217,	218,	219,	220,	221,	222,	223,	
	224,	225,	226,	227,	228,	229,	230,	231,
	232,	233,	234,	235,	236,	237,	238,	239,
	240,	241,	242,	243,	244,	245,	246,	247,
	248,	249,	250,	251,	252,	253,	254,	255
};
EXTCONST  unsigned char PL_fold_latin1[] = {
    /* Full latin1 complement folding, except for three problematic code points:
     *	Micro sign (181 = 0xB5) and y with diearesis (255 = 0xFF) have their
     *	fold complements outside the Latin1 range, so can't match something
     *	that isn't in utf8.
     *	German lower case sharp s (223 = 0xDF) folds to two characters, 'ss',
     *	not one, so can't be represented in this table.
     *
     * All have to be specially handled */
	0,	1,	2,	3,	4,	5,	6,	7,
	8,	9,	10,	11,	12,	13,	14,	15,
	16,	17,	18,	19,	20,	21,	22,	23,
	24,	25,	26,	27,	28,	29,	30,	31,
	32,	33,	34,	35,	36,	37,	38,	39,
	40,	41,	42,	43,	44,	45,	46,	47,
	48,	49,	50,	51,	52,	53,	54,	55,
	56,	57,	58,	59,	60,	61,	62,	63,
	64,	'a',	'b',	'c',	'd',	'e',	'f',	'g',
	'h',	'i',	'j',	'k',	'l',	'm',	'n',	'o',
	'p',	'q',	'r',	's',	't',	'u',	'v',	'w',
	'x',	'y',	'z',	91,	92,	93,	94,	95,
	96,	'A',	'B',	'C',	'D',	'E',	'F',	'G',
	'H',	'I',	'J',	'K',	'L',	'M',	'N',	'O',
	'P',	'Q',	'R',	'S',	'T',	'U',	'V',	'W',
	'X',	'Y',	'Z',	123,	124,	125,	126,	127,
	128,	129,	130,	131,	132,	133,	134,	135,
	136,	137,	138,	139,	140,	141,	142,	143,
	144,	145,	146,	147,	148,	149,	150,	151,
	152,	153,	154,	155,	156,	157,	158,	159,
	160,	161,	162,	163,	164,	165,	166,	167,
	168,	169,	170,	171,	172,	173,	174,	175,
	176,	177,	178,	179,	180,	181 /*micro */,	182,	183,
	184,	185,	186,	187,	188,	189,	190,	191,
	192+32,	193+32,	194+32,	195+32,	196+32,	197+32,	198+32,	199+32,
	200+32,	201+32,	202+32,	203+32,	204+32,	205+32,	206+32,	207+32,
	208+32,	209+32,	210+32,	211+32,	212+32,	213+32,	214+32,	215,
	216+32,	217+32,	218+32,	219+32,	220+32,	221+32,	222+32,	223 /* ss */,
	224-32,	225-32,	226-32,	227-32,	228-32,	229-32,	230-32,	231-32,
	232-32,	233-32,	234-32,	235-32,	236-32,	237-32,	238-32,	239-32,
	240-32,	241-32,	242-32,	243-32,	244-32,	245-32,	246-32,	247,
	248-32,	249-32,	250-32,	251-32,	252-32,	253-32,	254-32,
	255 /* y with diaeresis */
};

/* If these tables are accessed through ebcdic, the access will be converted to
 * latin1 first */
EXTCONST  unsigned char PL_latin1_lc[] = {  /* lowercasing */
	0,	1,	2,	3,	4,	5,	6,	7,
	8,	9,	10,	11,	12,	13,	14,	15,
	16,	17,	18,	19,	20,	21,	22,	23,
	24,	25,	26,	27,	28,	29,	30,	31,
	32,	33,	34,	35,	36,	37,	38,	39,
	40,	41,	42,	43,	44,	45,	46,	47,
	48,	49,	50,	51,	52,	53,	54,	55,
	56,	57,	58,	59,	60,	61,	62,	63,
	64,	'a',	'b',	'c',	'd',	'e',	'f',	'g',
	'h',	'i',	'j',	'k',	'l',	'm',	'n',	'o',
	'p',	'q',	'r',	's',	't',	'u',	'v',	'w',
	'x',	'y',	'z',	91,	92,	93,	94,	95,
	96,	97,	98,	99,	100,	101,	102,	103,
	104,	105,	106,	107,	108,	109,	110,	111,
	112,	113,	114,	115,	116,	117,	118,	119,
	120,	121,	122,	123,	124,	125,	126,	127,
	128,	129,	130,	131,	132,	133,	134,	135,
	136,	137,	138,	139,	140,	141,	142,	143,
	144,	145,	146,	147,	148,	149,	150,	151,
	152,	153,	154,	155,	156,	157,	158,	159,
	160,	161,	162,	163,	164,	165,	166,	167,
	168,	169,	170,	171,	172,	173,	174,	175,
	176,	177,	178,	179,	180,	181,	182,	183,
	184,	185,	186,	187,	188,	189,	190,	191,
	192+32,	193+32,	194+32,	195+32,	196+32,	197+32,	198+32,	199+32,
	200+32,	201+32,	202+32,	203+32,	204+32,	205+32,	206+32,	207+32,
	208+32,	209+32,	210+32,	211+32,	212+32,	213+32,	214+32,	215,
	216+32,	217+32,	218+32,	219+32,	220+32,	221+32,	222+32,	223,	
	224,	225,	226,	227,	228,	229,	230,	231,
	232,	233,	234,	235,	236,	237,	238,	239,
	240,	241,	242,	243,	244,	245,	246,	247,
	248,	249,	250,	251,	252,	253,	254,	255
};

/* upper and title case of latin1 characters, modified so that the three tricky
 * ones are mapped to 255 (which is one of the three) */
EXTCONST  unsigned char PL_mod_latin1_uc[] = {
	0,	1,	2,	3,	4,	5,	6,	7,
	8,	9,	10,	11,	12,	13,	14,	15,
	16,	17,	18,	19,	20,	21,	22,	23,
	24,	25,	26,	27,	28,	29,	30,	31,
	32,	33,	34,	35,	36,	37,	38,	39,
	40,	41,	42,	43,	44,	45,	46,	47,
	48,	49,	50,	51,	52,	53,	54,	55,
	56,	57,	58,	59,	60,	61,	62,	63,
	64,	65,	66,	67,	68,	69,	70,	71,
	72,	73,	74,	75,	76,	77,	78,	79,
	80,	81,	82,	83,	84,	85,	86,	87,
	88,	89,	90,	91,	92,	93,	94,	95,
	96,	'A',	'B',	'C',	'D',	'E',	'F',	'G',
	'H',	'I',	'J',	'K',	'L',	'M',	'N',	'O',
	'P',	'Q',	'R',	'S',	'T',	'U',	'V',	'W',
	'X',	'Y',	'Z',	123,	124,	125,	126,	127,
	128,	129,	130,	131,	132,	133,	134,	135,
	136,	137,	138,	139,	140,	141,	142,	143,
	144,	145,	146,	147,	148,	149,	150,	151,
	152,	153,	154,	155,	156,	157,	158,	159,
	160,	161,	162,	163,	164,	165,	166,	167,
	168,	169,	170,	171,	172,	173,	174,	175,
	176,	177,	178,	179,	180,	255 /*micro*/,	182,	183,
	184,	185,	186,	187,	188,	189,	190,	191,
	192,	193,	194,	195,	196,	197,	198,	199,
	200,	201,	202,	203,	204,	205,	206,	207,
	208,	209,	210,	211,	212,	213,	214,	215,
	216,	217,	218,	219,	220,	221,	222,
#if    UNICODE_MAJOR_VERSION > 2                                        \
   || (UNICODE_MAJOR_VERSION == 2 && UNICODE_DOT_VERSION >= 1		\
                                  && UNICODE_DOT_DOT_VERSION >= 8)
	                                                        255 /*sharp s*/,
#else   /* uc(sharp s) is 'sharp s' itself in early unicode */
	                                                        223,
#endif
	224-32,	225-32,	226-32,	227-32,	228-32,	229-32,	230-32,	231-32,
	232-32,	233-32,	234-32,	235-32,	236-32,	237-32,	238-32,	239-32,
	240-32,	241-32,	242-32,	243-32,	244-32,	245-32,	246-32,	247,
	248-32,	249-32,	250-32,	251-32,	252-32,	253-32,	254-32,	255
};
#endif  /* !EBCDIC, but still in DOINIT */
#else	/* ! DOINIT */
#   ifndef EBCDIC
EXTCONST unsigned char PL_fold[];
EXTCONST unsigned char PL_fold_latin1[];
EXTCONST unsigned char PL_mod_latin1_uc[];
EXTCONST unsigned char PL_latin1_lc[];
#   endif
#endif

#ifndef PERL_GLOBAL_STRUCT /* or perlvars.h */
#ifdef DOINIT
EXT unsigned char PL_fold_locale[256] = { /* Unfortunately not EXTCONST. */
	0,	1,	2,	3,	4,	5,	6,	7,
	8,	9,	10,	11,	12,	13,	14,	15,
	16,	17,	18,	19,	20,	21,	22,	23,
	24,	25,	26,	27,	28,	29,	30,	31,
	32,	33,	34,	35,	36,	37,	38,	39,
	40,	41,	42,	43,	44,	45,	46,	47,
	48,	49,	50,	51,	52,	53,	54,	55,
	56,	57,	58,	59,	60,	61,	62,	63,
	64,	'a',	'b',	'c',	'd',	'e',	'f',	'g',
	'h',	'i',	'j',	'k',	'l',	'm',	'n',	'o',
	'p',	'q',	'r',	's',	't',	'u',	'v',	'w',
	'x',	'y',	'z',	91,	92,	93,	94,	95,
	96,	'A',	'B',	'C',	'D',	'E',	'F',	'G',
	'H',	'I',	'J',	'K',	'L',	'M',	'N',	'O',
	'P',	'Q',	'R',	'S',	'T',	'U',	'V',	'W',
	'X',	'Y',	'Z',	123,	124,	125,	126,	127,
	128,	129,	130,	131,	132,	133,	134,	135,
	136,	137,	138,	139,	140,	141,	142,	143,
	144,	145,	146,	147,	148,	149,	150,	151,
	152,	153,	154,	155,	156,	157,	158,	159,
	160,	161,	162,	163,	164,	165,	166,	167,
	168,	169,	170,	171,	172,	173,	174,	175,
	176,	177,	178,	179,	180,	181,	182,	183,
	184,	185,	186,	187,	188,	189,	190,	191,
	192,	193,	194,	195,	196,	197,	198,	199,
	200,	201,	202,	203,	204,	205,	206,	207,
	208,	209,	210,	211,	212,	213,	214,	215,
	216,	217,	218,	219,	220,	221,	222,	223,	
	224,	225,	226,	227,	228,	229,	230,	231,
	232,	233,	234,	235,	236,	237,	238,	239,
	240,	241,	242,	243,	244,	245,	246,	247,
	248,	249,	250,	251,	252,	253,	254,	255
};
#else
EXT unsigned char PL_fold_locale[256]; /* Unfortunately not EXTCONST. */
#endif
#endif /* !PERL_GLOBAL_STRUCT */

#ifdef DOINIT
#ifdef EBCDIC
EXTCONST unsigned char PL_freq[] = {/* EBCDIC frequencies for mixed English/C */
    1,      2,      84,     151,    154,    155,    156,    157,
    165,    246,    250,    3,      158,    7,      18,     29,
    40,     51,     62,     73,     85,     96,     107,    118,
    129,    140,    147,    148,    149,    150,    152,    153,
    255,      6,      8,      9,     10,     11,     12,     13,
     14,     15,     24,     25,     26,     27,     28,    226,
     29,     30,     31,     32,     33,     43,     44,     45,
     46,     47,     48,     49,     50,     76,     77,     78,
     79,     80,     81,     82,     83,     84,     85,     86,
     87,     94,     95,    234,    181,    233,    187,    190,
    180,     96,     97,     98,     99,    100,    101,    102,
    104,    112,    182,    174,    236,    232,    229,    103,
    228,    226,    114,    115,    116,    117,    118,    119,
    120,    121,    122,    235,    176,    230,    194,    162,
    130,    131,    132,    133,    134,    135,    136,    137,
    138,    139,    201,    205,    163,    217,    220,    224,
    5,      248,    227,    244,    242,    255,    241,    231,
    240,    253,    16,     197,    19,     20,     21,     187,
    23,     169,    210,    245,    237,    249,    247,    239,
    168,    252,    34,     196,    36,     37,     38,     39,
    41,     42,     251,    254,    238,    223,    221,    213,
    225,    177,    52,     53,     54,     55,     56,     57,
    58,     59,     60,     61,     63,     64,     65,     66,
    67,     68,     69,     70,     71,     72,     74,     75,
    205,    208,    186,    202,    200,    218,    198,    179,
    178,    214,    88,     89,     90,     91,     92,     93,
    217,    166,    170,    207,    199,    209,    206,    204,
    160,    212,    105,    106,    108,    109,    110,    111,
    203,    113,    216,    215,    192,    175,    193,    243,
    172,    161,    123,    124,    125,    126,    127,    128,
    222,    219,    211,    195,    188,    193,    185,    184,
    191,    183,    141,    142,    143,    144,    145,    146
};
#else  /* ascii rather than ebcdic */
EXTCONST unsigned char PL_freq[] = {	/* letter frequencies for mixed English/C */
	1,	2,	84,	151,	154,	155,	156,	157,
	165,	246,	250,	3,	158,	7,	18,	29,
	40,	51,	62,	73,	85,	96,	107,	118,
	129,	140,	147,	148,	149,	150,	152,	153,
	255,	182,	224,	205,	174,	176,	180,	217,
	233,	232,	236,	187,	235,	228,	234,	226,
	222,	219,	211,	195,	188,	193,	185,	184,
	191,	183,	201,	229,	181,	220,	194,	162,
	163,	208,	186,	202,	200,	218,	198,	179,
	178,	214,	166,	170,	207,	199,	209,	206,
	204,	160,	212,	216,	215,	192,	175,	173,
	243,	172,	161,	190,	203,	189,	164,	230,
	167,	248,	227,	244,	242,	255,	241,	231,
	240,	253,	169,	210,	245,	237,	249,	247,
	239,	168,	252,	251,	254,	238,	223,	221,
	213,	225,	177,	197,	171,	196,	159,	4,
	5,	6,	8,	9,	10,	11,	12,	13,
	14,	15,	16,	17,	19,	20,	21,	22,
	23,	24,	25,	26,	27,	28,	30,	31,
	32,	33,	34,	35,	36,	37,	38,	39,
	41,	42,	43,	44,	45,	46,	47,	48,
	49,	50,	52,	53,	54,	55,	56,	57,
	58,	59,	60,	61,	63,	64,	65,	66,
	67,	68,	69,	70,	71,	72,	74,	75,
	76,	77,	78,	79,	80,	81,	82,	83,
	86,	87,	88,	89,	90,	91,	92,	93,
	94,	95,	97,	98,	99,	100,	101,	102,
	103,	104,	105,	106,	108,	109,	110,	111,
	112,	113,	114,	115,	116,	117,	119,	120,
	121,	122,	123,	124,	125,	126,	127,	128,
	130,	131,	132,	133,	134,	135,	136,	137,
	138,	139,	141,	142,	143,	144,	145,	146
};
#endif
#else
EXTCONST unsigned char PL_freq[];
#endif

/* Although only used for debugging, these constants must be available in
 * non-debugging builds too, since they're used in ext/re/re_exec.c,
 * which has DEBUGGING enabled always */
#ifdef DOINIT
EXTCONST char* const PL_block_type[] = {
	"NULL",
	"WHEN",
	"BLOCK",
	"GIVEN",
	"LOOP_ARY",
	"LOOP_LAZYSV",
	"LOOP_LAZYIV",
	"LOOP_LIST",
	"LOOP_PLAIN",
	"SUB",
	"FORMAT",
	"EVAL",
	"SUBST"
};
#else
EXTCONST char* PL_block_type[];
#endif

/* These are all the compile time options that affect binary compatibility.
   Other compile time options that are binary compatible are in perl.c
   (in S_Internals_V()). Both are combined for the output of perl -V
   However, this string will be embedded in any shared perl library, which will
   allow us add a comparison check in perlmain.c in the near future.  */
#ifdef DOINIT
EXTCONST char PL_bincompat_options[] =
#  ifdef DEBUG_LEAKING_SCALARS
			     " DEBUG_LEAKING_SCALARS"
#  endif
#  ifdef DEBUG_LEAKING_SCALARS_FORK_DUMP
			     " DEBUG_LEAKING_SCALARS_FORK_DUMP"
#  endif
#  ifdef FCRYPT
			     " FCRYPT"
#  endif
#  ifdef HAS_TIMES
			     " HAS_TIMES"
#  endif
#  ifdef HAVE_INTERP_INTERN
			     " HAVE_INTERP_INTERN"
#  endif
#  ifdef MULTIPLICITY
			     " MULTIPLICITY"
#  endif
#  ifdef MYMALLOC
			     " MYMALLOC"
#  endif
#  ifdef PERLIO_LAYERS
			     " PERLIO_LAYERS"
#  endif
#  ifdef PERL_DEBUG_READONLY_COW
			     " PERL_DEBUG_READONLY_COW"
#  endif
#  ifdef PERL_DEBUG_READONLY_OPS
			     " PERL_DEBUG_READONLY_OPS"
#  endif
#  ifdef PERL_GLOBAL_STRUCT
			     " PERL_GLOBAL_STRUCT"
#  endif
#  ifdef PERL_GLOBAL_STRUCT_PRIVATE
			     " PERL_GLOBAL_STRUCT_PRIVATE"
#  endif
#  ifdef PERL_IMPLICIT_CONTEXT
			     " PERL_IMPLICIT_CONTEXT"
#  endif
#  ifdef PERL_IMPLICIT_SYS
			     " PERL_IMPLICIT_SYS"
#  endif
#  ifdef PERL_MICRO
			     " PERL_MICRO"
#  endif
#  ifdef PERL_NEED_APPCTX
			     " PERL_NEED_APPCTX"
#  endif
#  ifdef PERL_NEED_TIMESBASE
			     " PERL_NEED_TIMESBASE"
#  endif
#  ifdef PERL_POISON
			     " PERL_POISON"
#  endif
#  ifdef PERL_SAWAMPERSAND
			     " PERL_SAWAMPERSAND"
#  endif
#  ifdef PERL_TRACK_MEMPOOL
			     " PERL_TRACK_MEMPOOL"
#  endif
#  ifdef PERL_USES_PL_PIDSTATUS
			     " PERL_USES_PL_PIDSTATUS"
#  endif
#  ifdef USE_64_BIT_ALL
			     " USE_64_BIT_ALL"
#  endif
#  ifdef USE_64_BIT_INT
			     " USE_64_BIT_INT"
#  endif
#  ifdef USE_IEEE
			     " USE_IEEE"
#  endif
#  ifdef USE_ITHREADS
			     " USE_ITHREADS"
#  endif
#  ifdef USE_LARGE_FILES
			     " USE_LARGE_FILES"
#  endif
#  ifdef USE_LOCALE_COLLATE
			     " USE_LOCALE_COLLATE"
#  endif
#  ifdef USE_LOCALE_NUMERIC
			     " USE_LOCALE_NUMERIC"
#  endif
#  ifdef USE_LOCALE_TIME
			     " USE_LOCALE_TIME"
#  endif
#  ifdef USE_LONG_DOUBLE
			     " USE_LONG_DOUBLE"
#  endif
#  ifdef USE_PERLIO
			     " USE_PERLIO"
#  endif
#  ifdef USE_QUADMATH
			     " USE_QUADMATH"
#  endif
#  ifdef USE_REENTRANT_API
			     " USE_REENTRANT_API"
#  endif
#  ifdef USE_SOCKS
			     " USE_SOCKS"
#  endif
#  ifdef VMS_DO_SOCKETS
			     " VMS_DO_SOCKETS"
#  endif
#  ifdef VMS_SHORTEN_LONG_SYMBOLS
			     " VMS_SHORTEN_LONG_SYMBOLS"
#  endif
#  ifdef VMS_WE_ARE_CASE_SENSITIVE
			     " VMS_SYMBOL_CASE_AS_IS"
#  endif
  "";
#else
EXTCONST char PL_bincompat_options[];
#endif

#ifndef PERL_SET_PHASE
#  define PERL_SET_PHASE(new_phase) \
    PERL_DTRACE_PROBE_PHASE(new_phase); \
    PL_phase = new_phase;
#endif

/* The interpreter phases. If these ever change, PL_phase_names right below will
 * need to be updated accordingly. */
enum perl_phase {
    PERL_PHASE_CONSTRUCT	= 0,
    PERL_PHASE_START		= 1,
    PERL_PHASE_CHECK		= 2,
    PERL_PHASE_INIT		= 3,
    PERL_PHASE_RUN		= 4,
    PERL_PHASE_END		= 5,
    PERL_PHASE_DESTRUCT		= 6
};

#ifdef DOINIT
EXTCONST char *const PL_phase_names[] = {
    "CONSTRUCT",
    "START",
    "CHECK",
    "INIT",
    "RUN",
    "END",
    "DESTRUCT"
};
#else
EXTCONST char *const PL_phase_names[];
#endif

#ifndef PERL_CORE
/* Do not use this macro. It only exists for extensions that rely on PL_dirty
 * instead of using the newer PL_phase, which provides everything PL_dirty
 * provided, and more. */
#  define PL_dirty cBOOL(PL_phase == PERL_PHASE_DESTRUCT)

#  define PL_amagic_generation PL_na
#  define PL_encoding ((SV *)NULL)
#endif /* !PERL_CORE */

#define PL_hints PL_compiling.cop_hints
#define PL_maxo  MAXO

END_EXTERN_C

/*****************************************************************************/
/* This lexer/parser stuff is currently global since yacc is hard to reenter */
/*****************************************************************************/
/* XXX This needs to be revisited, since BEGIN makes yacc re-enter... */

#ifdef __Lynx__
/* LynxOS defines these in scsi.h which is included via ioctl.h */
#ifdef FORMAT
#undef FORMAT
#endif
#ifdef SPACE
#undef SPACE
#endif
#endif

#define LEX_NOTPARSING		11	/* borrowed from toke.c */

typedef enum {
    XOPERATOR,
    XTERM,
    XREF,
    XSTATE,
    XBLOCK,
    XATTRBLOCK, /* next token should be an attribute or block */
    XATTRTERM,  /* next token should be an attribute, or block in a term */
    XTERMBLOCK,
    XBLOCKTERM,
    XPOSTDEREF,
    XTERMORDORDOR /* evil hack */
    /* update exp_name[] in toke.c if adding to this enum */
} expectation;

#define KEY_sigvar 0xFFFF /* fake keyword representing a signature var */

/* Hints are now stored in a dedicated U32, so the bottom 8 bits are no longer
   special and there is no need for HINT_PRIVATE_MASK for COPs
   However, bitops store HINT_INTEGER in their op_private.

    NOTE: The typical module using these has the bit value hard-coded, so don't
    blindly change the values of these.

   If we run out of bits, the 2 locale ones could be combined.  The PARTIAL one
   is for "use locale 'FOO'" which excludes some categories.  It requires going
   to %^H to find out which are in and which are out.  This could be extended
   for the normal case of a plain HINT_LOCALE, so that %^H would be used for
   any locale form. */
#define HINT_INTEGER		0x00000001 /* integer pragma */
#define HINT_STRICT_REFS	0x00000002 /* strict pragma */
#define HINT_LOCALE		0x00000004 /* locale pragma */
#define HINT_BYTES		0x00000008 /* bytes pragma */
#define HINT_LOCALE_PARTIAL	0x00000010 /* locale, but a subset of categories */

#define HINT_EXPLICIT_STRICT_REFS	0x00000020 /* strict.pm */
#define HINT_EXPLICIT_STRICT_SUBS	0x00000040 /* strict.pm */
#define HINT_EXPLICIT_STRICT_VARS	0x00000080 /* strict.pm */

#define HINT_BLOCK_SCOPE	0x00000100
#define HINT_STRICT_SUBS	0x00000200 /* strict pragma */
#define HINT_STRICT_VARS	0x00000400 /* strict pragma */
#define HINT_UNI_8_BIT		0x00000800 /* unicode_strings feature */

/* The HINT_NEW_* constants are used by the overload pragma */
#define HINT_NEW_INTEGER	0x00001000
#define HINT_NEW_FLOAT		0x00002000
#define HINT_NEW_BINARY		0x00004000
#define HINT_NEW_STRING		0x00008000
#define HINT_NEW_RE		0x00010000
#define HINT_LOCALIZE_HH	0x00020000 /* %^H needs to be copied */
#define HINT_LEXICAL_IO_IN	0x00040000 /* ${^OPEN} is set for input */
#define HINT_LEXICAL_IO_OUT	0x00080000 /* ${^OPEN} is set for output */

#define HINT_RE_TAINT		0x00100000 /* re pragma */
#define HINT_RE_EVAL		0x00200000 /* re pragma */

#define HINT_FILETEST_ACCESS	0x00400000 /* filetest pragma */
#define HINT_UTF8		0x00800000 /* utf8 pragma */

#define HINT_NO_AMAGIC		0x01000000 /* overloading pragma */

#define HINT_RE_FLAGS		0x02000000 /* re '/xism' pragma */

#define HINT_FEATURE_MASK	0x1c000000 /* 3 bits for feature bundles */

				/* Note: Used for HINT_M_VMSISH_*,
				   currently defined by vms/vmsish.h:
				0x40000000
				0x80000000
				 */

/* The following are stored in $^H{sort}, not in PL_hints */
#define HINT_SORT_STABLE	0x00000100 /* sort styles */
#define HINT_SORT_UNSTABLE	0x00000200

/* flags for PL_sawampersand */

#define SAWAMPERSAND_LEFT       1   /* saw $` */
#define SAWAMPERSAND_MIDDLE     2   /* saw $& */
#define SAWAMPERSAND_RIGHT      4   /* saw $' */

#ifndef PERL_SAWAMPERSAND
# define PL_sawampersand \
	(SAWAMPERSAND_LEFT|SAWAMPERSAND_MIDDLE|SAWAMPERSAND_RIGHT)
#endif

/* Used for debugvar magic */
#define DBVARMG_SINGLE  0
#define DBVARMG_TRACE   1
#define DBVARMG_SIGNAL  2
#define DBVARMG_COUNT   3

#define PL_DBsingle_iv  (PL_DBcontrol[DBVARMG_SINGLE])
#define PL_DBtrace_iv   (PL_DBcontrol[DBVARMG_TRACE])
#define PL_DBsignal_iv  (PL_DBcontrol[DBVARMG_SIGNAL])

/* Various states of the input record separator SV (rs) */
#define RsSNARF(sv)   (! SvOK(sv))
#define RsSIMPLE(sv)  (SvOK(sv) && (! SvPOK(sv) || SvCUR(sv)))
#define RsPARA(sv)    (SvPOK(sv) && ! SvCUR(sv))
#define RsRECORD(sv)  (SvROK(sv) && (SvIV(SvRV(sv)) > 0))

/* A struct for keeping various DEBUGGING related stuff,
 * neatly packed.  Currently only scratch variables for
 * constructing debug output are included.  Needed always,
 * not just when DEBUGGING, though, because of the re extension. c*/
struct perl_debug_pad {
  SV pad[3];
};

#define PERL_DEBUG_PAD(i)	&(PL_debug_pad.pad[i])
#define PERL_DEBUG_PAD_ZERO(i)	(SvPVX(PERL_DEBUG_PAD(i))[0] = 0, \
	(((XPV*) SvANY(PERL_DEBUG_PAD(i)))->xpv_cur = 0), \
	PERL_DEBUG_PAD(i))

/* Enable variables which are pointers to functions */
typedef void (*peep_t)(pTHX_ OP* o);
typedef regexp* (*regcomp_t) (pTHX_ char* exp, char* xend, PMOP* pm);
typedef I32     (*regexec_t) (pTHX_ regexp* prog, char* stringarg,
				      char* strend, char* strbeg, I32 minend,
				      SV* screamer, void* data, U32 flags);
typedef char*   (*re_intuit_start_t) (pTHX_ regexp *prog, SV *sv,
						char *strpos, char *strend,
						U32 flags,
						re_scream_pos_data *d);
typedef SV*	(*re_intuit_string_t) (pTHX_ regexp *prog);
typedef void	(*regfree_t) (pTHX_ struct regexp* r);
typedef regexp* (*regdupe_t) (pTHX_ const regexp* r, CLONE_PARAMS *param);
typedef I32     (*re_fold_t)(const char *, char const *, I32);

typedef void (*DESTRUCTORFUNC_NOCONTEXT_t) (void*);
typedef void (*DESTRUCTORFUNC_t) (pTHX_ void*);
typedef void (*SVFUNC_t) (pTHX_ SV* const);
typedef I32  (*SVCOMPARE_t) (pTHX_ SV* const, SV* const);
typedef void (*XSINIT_t) (pTHX);
typedef void (*ATEXIT_t) (pTHX_ void*);
typedef void (*XSUBADDR_t) (pTHX_ CV *);

typedef OP* (*Perl_ppaddr_t)(pTHX);
typedef OP* (*Perl_check_t) (pTHX_ OP*);
typedef void(*Perl_ophook_t)(pTHX_ OP*);
typedef int (*Perl_keyword_plugin_t)(pTHX_ char*, STRLEN, OP**);
typedef void(*Perl_cpeep_t)(pTHX_ OP *, OP *);

typedef void(*globhook_t)(pTHX);

#define KEYWORD_PLUGIN_DECLINE 0
#define KEYWORD_PLUGIN_STMT    1
#define KEYWORD_PLUGIN_EXPR    2

/* Interpreter exitlist entry */
typedef struct exitlistentry {
    void (*fn) (pTHX_ void*);
    void *ptr;
} PerlExitListEntry;

/* if you only have signal() and it resets on each signal, FAKE_PERSISTENT_SIGNAL_HANDLERS fixes */
/* These have to be before perlvars.h */
#if !defined(HAS_SIGACTION) && defined(VMS)
#  define  FAKE_PERSISTENT_SIGNAL_HANDLERS
#endif
/* if we're doing kill() with sys$sigprc on VMS, FAKE_DEFAULT_SIGNAL_HANDLERS */
#if defined(KILL_BY_SIGPRC)
#  define  FAKE_DEFAULT_SIGNAL_HANDLERS
#endif

#if !defined(MULTIPLICITY)

struct interpreter {
    char broiled;
};

#else

/* If we have multiple interpreters define a struct
   holding variables which must be per-interpreter
   If we don't have threads anything that would have
   be per-thread is per-interpreter.
*/

/* Set up PERLVAR macros for populating structs */
#  define PERLVAR(prefix,var,type) type prefix##var;

/* 'var' is an array of length 'n' */
#  define PERLVARA(prefix,var,n,type) type prefix##var[n];

/* initialize 'var' to init' */
#  define PERLVARI(prefix,var,type,init) type prefix##var;

/* like PERLVARI, but make 'var' a const */
#  define PERLVARIC(prefix,var,type,init) type prefix##var;

struct interpreter {
#  include "intrpvar.h"
};

EXTCONST U16 PL_interp_size
  INIT(sizeof(struct interpreter));

#  define PERL_INTERPRETER_SIZE_UPTO_MEMBER(member)			\
    STRUCT_OFFSET(struct interpreter, member) +				\
    sizeof(((struct interpreter*)0)->member)

/* This will be useful for subsequent releases, because this has to be the
   same in your libperl as in main(), else you have a mismatch and must abort.
*/
EXTCONST U16 PL_interp_size_5_18_0
  INIT(PERL_INTERPRETER_SIZE_UPTO_MEMBER(PERL_LAST_5_18_0_INTERP_MEMBER));


#  ifdef PERL_GLOBAL_STRUCT
/* MULTIPLICITY is automatically defined when PERL_GLOBAL_STRUCT is defined,
   hence it's safe and sane to nest this within #ifdef MULTIPLICITY  */

struct perl_vars {
#    include "perlvars.h"
};

EXTCONST U16 PL_global_struct_size
  INIT(sizeof(struct perl_vars));

#    ifdef PERL_CORE
#      ifndef PERL_GLOBAL_STRUCT_PRIVATE
EXT struct perl_vars PL_Vars;
EXT struct perl_vars *PL_VarsPtr INIT(&PL_Vars);
#        undef PERL_GET_VARS
#        define PERL_GET_VARS() PL_VarsPtr
#      endif /* !PERL_GLOBAL_STRUCT_PRIVATE */
#    else /* PERL_CORE */
#      if !defined(__GNUC__) || !defined(WIN32)
EXT
#      endif /* WIN32 */
struct perl_vars *PL_VarsPtr;
#      define PL_Vars (*((PL_VarsPtr) \
		       ? PL_VarsPtr : (PL_VarsPtr = Perl_GetVars(aTHX))))
#    endif /* PERL_CORE */
#  endif /* PERL_GLOBAL_STRUCT */

/* Done with PERLVAR macros for now ... */
#  undef PERLVAR
#  undef PERLVARA
#  undef PERLVARI
#  undef PERLVARIC

#endif /* MULTIPLICITY */

struct tempsym; /* defined in pp_pack.c */

#include "thread.h"
#include "pp.h"

#undef PERL_CKDEF
#undef PERL_PPDEF
#define PERL_CKDEF(s)	PERL_CALLCONV OP *s (pTHX_ OP *o);
#define PERL_PPDEF(s)	PERL_CALLCONV OP *s (pTHX);

#ifdef MYMALLOC
#  include "malloc_ctl.h"
#endif

/*
 * This provides a layer of functions and macros to ensure extensions will
 * get to use the same RTL functions as the core.
 */
#if defined(WIN32)
#  include "win32iop.h"
#endif


#include "proto.h"

/* this has structure inits, so it cannot be included before here */
#include "opcode.h"

/* The following must follow proto.h as #defines mess up syntax */

#if !defined(PERL_FOR_X2P)
#  include "embedvar.h"
#endif

/* Now include all the 'global' variables
 * If we don't have threads or multiple interpreters
 * these include variables that would have been their struct-s
 */

#define PERLVAR(prefix,var,type) EXT type PL_##var;
#define PERLVARA(prefix,var,n,type) EXT type PL_##var[n];
#define PERLVARI(prefix,var,type,init) EXT type  PL_##var INIT(init);
#define PERLVARIC(prefix,var,type,init) EXTCONST type PL_##var INIT(init);

#if !defined(MULTIPLICITY)
START_EXTERN_C
#  include "intrpvar.h"
END_EXTERN_C
#  define PL_sv_yes   (PL_sv_immortals[0])
#  define PL_sv_undef (PL_sv_immortals[1])
#  define PL_sv_no    (PL_sv_immortals[2])
#  define PL_sv_zero  (PL_sv_immortals[3])
#endif

#ifdef PERL_CORE
/* All core uses now exterminated. Ensure no zombies can return:  */
#  undef PL_na
#endif

/* Now all the config stuff is setup we can include embed.h
   In particular, need the relevant *ish file included already, as it may
   define HAVE_INTERP_INTERN  */
#include "embed.h"

#ifndef PERL_GLOBAL_STRUCT
START_EXTERN_C

#  include "perlvars.h"

END_EXTERN_C
#endif

#undef PERLVAR
#undef PERLVARA
#undef PERLVARI
#undef PERLVARIC

#if !defined(MULTIPLICITY)
/* Set up PERLVAR macros for populating structs */
#  define PERLVAR(prefix,var,type) type prefix##var;
/* 'var' is an array of length 'n' */
#  define PERLVARA(prefix,var,n,type) type prefix##var[n];
/* initialize 'var' to init' */
#  define PERLVARI(prefix,var,type,init) type prefix##var;
/* like PERLVARI, but make 'var' a const */
#  define PERLVARIC(prefix,var,type,init) type prefix##var;

/* this is never instantiated, is it just used for sizeof(struct PerlHandShakeInterpreter) */
struct PerlHandShakeInterpreter {
#  include "intrpvar.h"
};
#  undef PERLVAR
#  undef PERLVARA
#  undef PERLVARI
#  undef PERLVARIC
#endif

START_EXTERN_C

/* dummy variables that hold pointers to both runops functions, thus forcing
 * them *both* to get linked in (useful for Peek.xs, debugging etc) */

EXTCONST runops_proc_t PL_runops_std
  INIT(Perl_runops_standard);
EXTCONST runops_proc_t PL_runops_dbg
  INIT(Perl_runops_debug);

#define EXT_MGVTBL EXTCONST MGVTBL

#define PERL_MAGIC_READONLY_ACCEPTABLE 0x40
#define PERL_MAGIC_VALUE_MAGIC 0x80
#define PERL_MAGIC_VTABLE_MASK 0x3F
#define PERL_MAGIC_TYPE_READONLY_ACCEPTABLE(t) \
    (PL_magic_data[(U8)(t)] & PERL_MAGIC_READONLY_ACCEPTABLE)
#define PERL_MAGIC_TYPE_IS_VALUE_MAGIC(t) \
    (PL_magic_data[(U8)(t)] & PERL_MAGIC_VALUE_MAGIC)

#include "mg_vtable.h"

#ifdef DOINIT
EXTCONST U8 PL_magic_data[256] =
#  ifdef PERL_MICRO
#    include "umg_data.h"
#  else
#    include "mg_data.h"
#  endif
;
#else
EXTCONST U8 PL_magic_data[256];
#endif

#ifdef DOINIT
		        /* NL IV NV PV INV PI PN MG RX GV LV AV HV CV FM IO */
EXTCONST bool
PL_valid_types_IVX[]    = { 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0 };
EXTCONST bool
PL_valid_types_NVX[]    = { 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0 };
EXTCONST bool
PL_valid_types_PVX[]    = { 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1 };
EXTCONST bool
PL_valid_types_RV[]     = { 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1 };
EXTCONST bool
PL_valid_types_IV_set[] = { 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1 };
EXTCONST bool
PL_valid_types_NV_set[] = { 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0 };

#else

EXTCONST bool PL_valid_types_IVX[];
EXTCONST bool PL_valid_types_NVX[];
EXTCONST bool PL_valid_types_PVX[];
EXTCONST bool PL_valid_types_RV[];
EXTCONST bool PL_valid_types_IV_set[];
EXTCONST bool PL_valid_types_NV_set[];

#endif

/* In C99 we could use designated (named field) union initializers.
 * In C89 we need to initialize the member declared first.
 * In C++ we need extern C initializers.
 *
 * With the U8_NV version you will want to have inner braces,
 * while with the NV_U8 use just the NV. */

#define INFNAN_U8_NV_DECL EXTCONST union { U8 u8[NVSIZE]; NV nv; }
#define INFNAN_NV_U8_DECL EXTCONST union { NV nv; U8 u8[NVSIZE]; }

/* if these never got defined, they need defaults */
#ifndef PERL_SET_CONTEXT
#  define PERL_SET_CONTEXT(i)		PERL_SET_INTERP(i)
#endif

#ifndef PERL_GET_CONTEXT
#  define PERL_GET_CONTEXT		PERL_GET_INTERP
#endif

#ifndef PERL_GET_THX
#  define PERL_GET_THX			((void*)NULL)
#endif

#ifndef PERL_SET_THX
#  define PERL_SET_THX(t)		NOOP
#endif

#ifndef EBCDIC

/* The tables below are adapted from
 * https://bjoern.hoehrmann.de/utf-8/decoder/dfa/, which requires this copyright
 * notice:

Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

#  ifdef DOINIT
#    if 0       /* This is the original table given in
                   https://bjoern.hoehrmann.de/utf-8/decoder/dfa/ */
static U8 utf8d_C9[] = {
  /* The first part of the table maps bytes to character classes that
   * to reduce the size of the transition table and create bitmasks. */
   0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /*-1F*/
   0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /*-3F*/
   0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /*-5F*/
   0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /*-7F*/
   1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,  9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9, /*-9F*/
   7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,  7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7, /*-BF*/
   8,8,2,2,2,2,2,2,2,2,2,2,2,2,2,2,  2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, /*-DF*/
  10,3,3,3,3,3,3,3,3,3,3,3,3,4,3,3, 11,6,6,6,5,8,8,8,8,8,8,8,8,8,8,8, /*-FF*/

  /* The second part is a transition table that maps a combination
   * of a state of the automaton and a character class to a state. */
   0,12,24,36,60,96,84,12,12,12,48,72, 12,12,12,12,12,12,12,12,12,12,12,12,
  12, 0,12,12,12,12,12, 0,12, 0,12,12, 12,24,12,12,12,12,12,24,12,24,12,12,
  12,12,12,12,12,12,12,24,12,12,12,12, 12,24,12,12,12,12,12,12,12,24,12,12,
  12,12,12,12,12,12,12,36,12,36,12,12, 12,36,12,12,12,12,12,36,12,36,12,12,
  12,36,12,12,12,12,12,12,12,12,12,12
};

#    endif

/* This is a version of the above table customized for Perl that doesn't
 * exclude surrogates and accepts start bytes up through FD (FE on 64-bit
 * machines).  The classes have been renumbered so that the patterns are more
 * evident in the table.  The class numbers for start bytes are constrained so
 * that they can be used as a shift count for masking off the leading one bits.
 * It would make the code simpler if start byte FF could also be handled, but
 * doing so would mean adding nodes for each of continuation bytes 6-12
 * remaining, and two more nodes for overlong detection (a total of 9), and
 * there is room only for 4 more nodes unless we make the array U16 instead of
 * U8.
 *
 * The classes are
 *      00-7F           0
 *      80-81           7   Not legal immediately after start bytes E0 F0 F8 FC
 *                          FE
 *      82-83           8   Not legal immediately after start bytes E0 F0 F8 FC
 *      84-87           9   Not legal immediately after start bytes E0 F0 F8
 *      88-8F          10   Not legal immediately after start bytes E0 F0
 *      90-9F          11   Not legal immediately after start byte E0
 *      A0-BF          12
 *      C0,C1           1
 *      C2-DF           2
 *      E0             13
 *      E1-EF           3
 *      F0             14
 *      F1-F7           4
 *      F8             15
 *      F9-FB           5
 *      FC             16
 *      FD              6
 *      FE             17  (or 1 on 32-bit machines, since it overflows)
 *      FF              1
 */

EXTCONST U8 PL_extended_utf8_dfa_tab[] = {
    /* The first part of the table maps bytes to character classes to reduce
     * the size of the transition table and create bitmasks. */
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*00-0F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*10-1F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*20-2F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*30-3F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*40-4F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*50-5F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*60-6F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*70-7F*/
   7, 7, 8, 8, 9, 9, 9, 9,10,10,10,10,10,10,10,10, /*80-8F*/
  11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11, /*90-9F*/
  12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12, /*A0-AF*/
  12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12, /*B0-BF*/
   1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, /*C0-CF*/
   2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, /*D0-DF*/
  13, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, /*E0-EF*/
  14, 4, 4, 4, 4, 4, 4, 4,15, 5, 5, 5,16, 6,       /*F0-FD*/
#    ifdef UV_IS_QUAD
                                            17,    /*FE*/
#    else
                                             1,    /*FE*/
#    endif
                                                1, /*FF*/

/* The second part is a transition table that maps a combination
 * of a state of the automaton and a character class to a new state, called a
 * node.  The nodes are:
 * N0     The initial state, and final accepting one.
 * N1     Any one continuation byte (80-BF) left.  This is transitioned to
 *        immediately when the start byte indicates a two-byte sequence
 * N2     Any two continuation bytes left.
 * N3     Any three continuation bytes left.
 * N4     Any four continuation bytes left.
 * N5     Any five continuation bytes left.
 * N6     Start byte is E0.  Continuation bytes 80-9F are illegal (overlong);
 *        the other continuations transition to N1
 * N7     Start byte is F0.  Continuation bytes 80-8F are illegal (overlong);
 *        the other continuations transition to N2
 * N8     Start byte is F8.  Continuation bytes 80-87 are illegal (overlong);
 *        the other continuations transition to N3
 * N9     Start byte is FC.  Continuation bytes 80-83 are illegal (overlong);
 *        the other continuations transition to N4
 * N10    Start byte is FE.  Continuation bytes 80-81 are illegal (overlong);
 *        the other continuations transition to N5
 * 1      Reject.  All transitions not mentioned above (except the single
 *        byte ones (as they are always legal) are to this state.
 */

#    define NUM_CLASSES 18
#    define N0 0
#    define N1 ((N0)   + NUM_CLASSES)
#    define N2 ((N1)   + NUM_CLASSES)
#    define N3 ((N2)   + NUM_CLASSES)
#    define N4 ((N3)   + NUM_CLASSES)
#    define N5 ((N4)   + NUM_CLASSES)
#    define N6 ((N5)   + NUM_CLASSES)
#    define N7 ((N6)   + NUM_CLASSES)
#    define N8 ((N7)   + NUM_CLASSES)
#    define N9 ((N8)   + NUM_CLASSES)
#    define N10 ((N9)  + NUM_CLASSES)

/*Class: 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17  */
/*N0*/   0, 1,N1,N2,N3,N4,N5, 1, 1, 1, 1, 1, 1,N6,N7,N8,N9,N10,
/*N1*/   1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
/*N2*/   1, 1, 1, 1, 1, 1, 1,N1,N1,N1,N1,N1,N1, 1, 1, 1, 1, 1,
/*N3*/   1, 1, 1, 1, 1, 1, 1,N2,N2,N2,N2,N2,N2, 1, 1, 1, 1, 1,
/*N4*/   1, 1, 1, 1, 1, 1, 1,N3,N3,N3,N3,N3,N3, 1, 1, 1, 1, 1,
/*N5*/   1, 1, 1, 1, 1, 1, 1,N4,N4,N4,N4,N4,N4, 1, 1, 1, 1, 1,

/*N6*/   1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,N1, 1, 1, 1, 1, 1,
/*N7*/   1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,N2,N2, 1, 1, 1, 1, 1,
/*N8*/   1, 1, 1, 1, 1, 1, 1, 1, 1, 1,N3,N3,N3, 1, 1, 1, 1, 1,
/*N9*/   1, 1, 1, 1, 1, 1, 1, 1, 1,N4,N4,N4,N4, 1, 1, 1, 1, 1,
/*N10*/  1, 1, 1, 1, 1, 1, 1, 1,N5,N5,N5,N5,N5, 1, 1, 1, 1, 1,
};

/* And below is a version of the above table that accepts only strict UTF-8.
 * Hence no surrogates nor non-characters, nor non-Unicode.  Thus, if the input
 * passes this dfa, it will be for a well-formed, non-problematic code point
 * that can be returned immediately.
 *
 * The "Implementation details" portion of
 * https://bjoern.hoehrmann.de/utf-8/decoder/dfa/ shows how
 * the first portion of the table maps each possible byte into a character
 * class.  And that the classes for those bytes which are start bytes have been
 * carefully chosen so they serve as well to be used as a shift value to mask
 * off the leading 1 bits of the start byte.  Unfortunately the addition of
 * being able to distinguish non-characters makes this not fully work.  This is
 * because, now, the start bytes E1-EF have to be broken into 3 classes instead
 * of 2:
 *  1) ED because it could be a surrogate
 *  2) EF because it could be a non-character
 *  3) the rest, which can never evaluate to a problematic code point.
 *
 * Each of E1-EF has three leading 1 bits, then a 0.  That means we could use a
 * shift (and hence class number) of either 3 or 4 to get a mask that works.
 * But that only allows two categories, and we need three.  khw made the
 * decision to therefore treat the ED start byte as an error, so that the dfa
 * drops out immediately for that.  In the dfa, classes 3 and 4 are used to
 * distinguish EF vs the rest.  Then special code is used to deal with ED,
 * that's executed only when the dfa drops out.  The code points started by ED
 * are half surrogates, and half hangul syllables.  This means that 2048 of
 * the hangul syllables (about 18%) take longer than all other non-problematic
 * code points to handle.
 *
 * The changes to handle non-characters requires the addition of states and
 * classes to the dfa.  (See the section on "Mapping bytes to character
 * classes" in the linked-to document for further explanation of the original
 * dfa.)
 *
 * The classes are
 *      00-7F           0
 *      80-8E           9
 *      8F             10
 *      90-9E          11
 *      9F             12
 *      A0-AE          13
 *      AF             14
 *      B0-B6          15
 *      B7             16
 *      B8-BD          15
 *      BE             17
 *      BF             18
 *      C0,C1           1
 *      C2-DF           2
 *      E0              7
 *      E1-EC           3
 *      ED              1
 *      EE              3
 *      EF              4
 *      F0              8
 *      F1-F3           6  (6 bits can be stripped)
 *      F4              5  (only 5 can be stripped)
 *      F5-FF           1
 */

EXTCONST U8 PL_strict_utf8_dfa_tab[] = {
    /* The first part of the table maps bytes to character classes to reduce
     * the size of the transition table and create bitmasks. */
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*00-0F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*10-1F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*20-2F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*30-3F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*40-4F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*50-5F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*60-6F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*70-7F*/
   9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,10, /*80-8F*/
  11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12, /*90-9F*/
  13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,14, /*A0-AF*/
  15,15,15,15,15,15,15,16,15,15,15,15,15,15,17,18, /*B0-BF*/
   1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, /*C0-CF*/
   2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, /*D0-DF*/
   7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 3, 4, /*E0-EF*/
   8, 6, 6, 6, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, /*F0-FF*/

/* The second part is a transition table that maps a combination
 * of a state of the automaton and a character class to a new state, called a
 * node.  The nodes are:
 * N0     The initial state, and final accepting one.
 * N1     Any one continuation byte (80-BF) left.  This is transitioned to
 *        immediately when the start byte indicates a two-byte sequence
 * N2     Any two continuation bytes left.
 * N3     Start byte is E0.  Continuation bytes 80-9F are illegal (overlong);
 *        the other continuations transition to state N1
 * N4     Start byte is EF.  Continuation byte B7 transitions to N8; BF to N9;
 *        the other continuations transitions to N1
 * N5     Start byte is F0.  Continuation bytes 80-8F are illegal (overlong);
 *        [9AB]F transition to N10; the other continuations to N2.
 * N6     Start byte is F[123].  Continuation bytes [89AB]F transition
 *        to N10; the other continuations to N2.
 * N7     Start byte is F4.  Continuation bytes 90-BF are illegal
 *        (non-unicode); 8F transitions to N10; the other continuations to N2
 * N8     Initial sequence is EF B7.  Continuation bytes 90-AF are illegal
 *        (non-characters); the other continuations transition to N0.
 * N9     Initial sequence is EF BF.  Continuation bytes BE and BF are illegal
 *        (non-characters); the other continuations transition to N0.
 * N10    Initial sequence is one of: F0 [9-B]F; F[123] [8-B]F; or F4 8F.
 *        Continuation byte BF transitions to N11; the other continuations to
 *        N1
 * N11    Initial sequence is the two bytes given in N10 followed by BF.
 *        Continuation bytes BE and BF are illegal (non-characters); the other
 *        continuations transition to N0.
 * 1      Reject.  All transitions not mentioned above (except the single
 *        byte ones (as they are always legal) are to this state.
 */

#    undef N0
#    undef N1
#    undef N2
#    undef N3
#    undef N4
#    undef N5
#    undef N6
#    undef N7
#    undef N8
#    undef N9
#    undef NUM_CLASSES
#    define NUM_CLASSES 19
#    define N0 0
#    define N1  ((N0)  + NUM_CLASSES)
#    define N2  ((N1)  + NUM_CLASSES)
#    define N3  ((N2)  + NUM_CLASSES)
#    define N4  ((N3)  + NUM_CLASSES)
#    define N5  ((N4)  + NUM_CLASSES)
#    define N6  ((N5)  + NUM_CLASSES)
#    define N7  ((N6)  + NUM_CLASSES)
#    define N8  ((N7)  + NUM_CLASSES)
#    define N9  ((N8)  + NUM_CLASSES)
#    define N10 ((N9)  + NUM_CLASSES)
#    define N11 ((N10) + NUM_CLASSES)

/*Class: 0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16  17  18 */
/*N0*/   0,  1, N1, N2, N4, N7, N6, N3, N5,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,
/*N1*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
/*N2*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N1, N1, N1, N1, N1, N1, N1, N1, N1, N1,

/*N3*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, N1, N1, N1, N1, N1, N1,
/*N4*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N1, N1, N1, N1, N1, N1, N1, N8, N1, N9,
/*N5*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, N2,N10, N2,N10, N2, N2, N2,N10,
/*N6*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N2,N10, N2,N10, N2,N10, N2, N2, N2,N10,
/*N7*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N2,N10,  1,  1,  1,  1,  1,  1,  1,  1,
/*N8*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  0,  0,  1,  1,  1,  1,  0,  0,  0,  0,
/*N9*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  0,  0,  0,  0,  0,  0,  0,  0,  1,  1,
/*N10*/  1,  1,  1,  1,  1,  1,  1,  1,  1, N1, N1, N1, N1, N1, N1, N1, N1, N1,N11,
/*N11*/  1,  1,  1,  1,  1,  1,  1,  1,  1,  0,  0,  0,  0,  0,  0,  0,  0,  1,  1,
};

/* And below is yet another version of the above tables that accepts only UTF-8
 * as defined by Corregidum #9.  Hence no surrogates nor non-Unicode, but
 * it allows non-characters.  This is isomorphic to the original table
 * in https://bjoern.hoehrmann.de/utf-8/decoder/dfa/
 *
 * The classes are
 *      00-7F           0
 *      80-8F           9
 *      90-9F          10
 *      A0-BF          11
 *      C0,C1           1
 *      C2-DF           2
 *      E0              7
 *      E1-EC           3
 *      ED              4
 *      EE-EF           3
 *      F0              8
 *      F1-F3           6  (6 bits can be stripped)
 *      F4              5  (only 5 can be stripped)
 *      F5-FF           1
 */

EXTCONST U8 PL_c9_utf8_dfa_tab[] = {
    /* The first part of the table maps bytes to character classes to reduce
     * the size of the transition table and create bitmasks. */
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*00-0F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*10-1F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*20-2F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*30-3F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*40-4F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*50-5F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*60-6F*/
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /*70-7F*/
   9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, /*80-8F*/
  10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10, /*90-9F*/
  11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11, /*A0-AF*/
  11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11, /*B0-BF*/
   1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, /*C0-CF*/
   2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, /*D0-DF*/
   7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, /*E0-EF*/
   8, 6, 6, 6, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, /*F0-FF*/

/* The second part is a transition table that maps a combination
 * of a state of the automaton and a character class to a new state, called a
 * node.  The nodes are:
 * N0     The initial state, and final accepting one.
 * N1     Any one continuation byte (80-BF) left.  This is transitioned to
 *        immediately when the start byte indicates a two-byte sequence
 * N2     Any two continuation bytes left.
 * N3     Any three continuation bytes left.
 * N4     Start byte is E0.  Continuation bytes 80-9F are illegal (overlong);
 *        the other continuations transition to state N1
 * N5     Start byte is ED.  Continuation bytes A0-BF all lead to surrogates,
 *        so are illegal.  The other continuations transition to state N1.
 * N6     Start byte is F0.  Continuation bytes 80-8F are illegal (overlong);
 *        the other continuations transition to N2
 * N7     Start byte is F4.  Continuation bytes 90-BF are illegal
 *        (non-unicode); the other continuations transition to N2
 * 1      Reject.  All transitions not mentioned above (except the single
 *        byte ones (as they are always legal) are to this state.
 */

#    undef N0
#    undef N1
#    undef N2
#    undef N3
#    undef N4
#    undef N5
#    undef N6
#    undef N7
#    undef NUM_CLASSES
#    define NUM_CLASSES 12
#    define N0 0
#    define N1  ((N0)  + NUM_CLASSES)
#    define N2  ((N1)  + NUM_CLASSES)
#    define N3  ((N2)  + NUM_CLASSES)
#    define N4  ((N3)  + NUM_CLASSES)
#    define N5  ((N4)  + NUM_CLASSES)
#    define N6  ((N5)  + NUM_CLASSES)
#    define N7  ((N6)  + NUM_CLASSES)

/*Class: 0   1   2   3   4   5   6   7   8   9  10  11 */
/*N0*/   0,  1, N1, N2, N5, N7, N3, N4, N6,  1,  1,  1,
/*N1*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  0,  0,  0,
/*N2*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N1, N1, N1,
/*N3*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N2, N2, N2,

/*N4*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1, N1,
/*N5*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N1, N1,  1,
/*N6*/   1,  1,  1,  1,  1,  1,  1,  1,  1,  1, N2, N2,
/*N7*/   1,  1,  1,  1,  1,  1,  1,  1,  1, N2,  1,  1,
};

#  else     /* End of is DOINIT */

EXTCONST U8 PL_extended_utf8_dfa_tab[];
EXTCONST U8 PL_strict_utf8_dfa_tab[];
EXTCONST U8 PL_c9_utf8_dfa_tab[];

#  endif
#endif    /* end of isn't EBCDIC */

#ifndef PERL_NO_INLINE_FUNCTIONS
/* Static inline funcs that depend on includes and declarations above.
   Some of these reference functions in the perl object files, and some
   compilers aren't smart enough to eliminate unused static inline
   functions, so including this file in source code can cause link errors
   even if the source code uses none of the functions. Hence including these
   can be suppressed by setting PERL_NO_INLINE_FUNCTIONS. Doing this will
   (obviously) result in unworkable XS code, but allows simple probing code
   to continue to work, because it permits tests to include the perl headers
   for definitions without creating a link dependency on the perl library
   (which may not exist yet).
*/

#  include "inline.h"
#endif

#include "overload.h"

END_EXTERN_C

struct am_table {
  U8 flags;
  U8 fallback;
  U16 spare;
  U32 was_ok_sub;
  CV* table[NofAMmeth];
};
struct am_table_short {
  U8 flags;
  U8 fallback;
  U16 spare;
  U32 was_ok_sub;
};
typedef struct am_table AMT;
typedef struct am_table_short AMTS;

#define AMGfallNEVER	1
#define AMGfallNO	2
#define AMGfallYES	3

#define AMTf_AMAGIC		1
#define AMT_AMAGIC(amt)		((amt)->flags & AMTf_AMAGIC)
#define AMT_AMAGIC_on(amt)	((amt)->flags |= AMTf_AMAGIC)
#define AMT_AMAGIC_off(amt)	((amt)->flags &= ~AMTf_AMAGIC)

#define StashHANDLER(stash,meth)	gv_handler((stash),CAT2(meth,_amg))

/*
 * some compilers like to redefine cos et alia as faster
 * (and less accurate?) versions called F_cos et cetera (Quidquid
 * latine dictum sit, altum viditur.)  This trick collides with
 * the Perl overloading (amg).  The following #defines fool both.
 */

#ifdef _FASTMATH
#   ifdef atan2
#       define F_atan2_amg  atan2_amg
#   endif
#   ifdef cos
#       define F_cos_amg    cos_amg
#   endif
#   ifdef exp
#       define F_exp_amg    exp_amg
#   endif
#   ifdef log
#       define F_log_amg    log_amg
#   endif
#   ifdef pow
#       define F_pow_amg    pow_amg
#   endif
#   ifdef sin
#       define F_sin_amg    sin_amg
#   endif
#   ifdef sqrt
#       define F_sqrt_amg   sqrt_amg
#   endif
#endif /* _FASTMATH */

#define PERLDB_ALL		(PERLDBf_SUB	| PERLDBf_LINE	|	\
				 PERLDBf_NOOPT	| PERLDBf_INTER	|	\
				 PERLDBf_SUBLINE| PERLDBf_SINGLE|	\
				 PERLDBf_NAMEEVAL| PERLDBf_NAMEANON |   \
				 PERLDBf_SAVESRC)
					/* No _NONAME, _GOTO */
#define PERLDBf_SUB		0x01	/* Debug sub enter/exit */
#define PERLDBf_LINE		0x02	/* Keep line # */
#define PERLDBf_NOOPT		0x04	/* Switch off optimizations */
#define PERLDBf_INTER		0x08	/* Preserve more data for
					   later inspections  */
#define PERLDBf_SUBLINE		0x10	/* Keep subr source lines */
#define PERLDBf_SINGLE		0x20	/* Start with single-step on */
#define PERLDBf_NONAME		0x40	/* For _SUB: no name of the subr */
#define PERLDBf_GOTO		0x80	/* Report goto: call DB::goto */
#define PERLDBf_NAMEEVAL	0x100	/* Informative names for evals */
#define PERLDBf_NAMEANON	0x200	/* Informative names for anon subs */
#define PERLDBf_SAVESRC  	0x400	/* Save source lines into @{"_<$filename"} */
#define PERLDBf_SAVESRC_NOSUBS	0x800	/* Including evals that generate no subroutines */
#define PERLDBf_SAVESRC_INVALID	0x1000	/* Save source that did not compile */

#define PERLDB_SUB		(PL_perldb & PERLDBf_SUB)
#define PERLDB_LINE		(PL_perldb & PERLDBf_LINE)
#define PERLDB_NOOPT		(PL_perldb & PERLDBf_NOOPT)
#define PERLDB_INTER		(PL_perldb & PERLDBf_INTER)
#define PERLDB_SUBLINE		(PL_perldb & PERLDBf_SUBLINE)
#define PERLDB_SINGLE		(PL_perldb & PERLDBf_SINGLE)
#define PERLDB_SUB_NN		(PL_perldb & PERLDBf_NONAME)
#define PERLDB_GOTO		(PL_perldb & PERLDBf_GOTO)
#define PERLDB_NAMEEVAL 	(PL_perldb & PERLDBf_NAMEEVAL)
#define PERLDB_NAMEANON 	(PL_perldb & PERLDBf_NAMEANON)
#define PERLDB_SAVESRC  	(PL_perldb & PERLDBf_SAVESRC)
#define PERLDB_SAVESRC_NOSUBS	(PL_perldb & PERLDBf_SAVESRC_NOSUBS)
#define PERLDB_SAVESRC_INVALID	(PL_perldb & PERLDBf_SAVESRC_INVALID)

#define PERLDB_LINE_OR_SAVESRC (PL_perldb & (PERLDBf_LINE | PERLDBf_SAVESRC))

#ifdef USE_ITHREADS
#  define KEYWORD_PLUGIN_MUTEX_INIT    MUTEX_INIT(&PL_keyword_plugin_mutex)
#  define KEYWORD_PLUGIN_MUTEX_LOCK    MUTEX_LOCK(&PL_keyword_plugin_mutex)
#  define KEYWORD_PLUGIN_MUTEX_UNLOCK  MUTEX_UNLOCK(&PL_keyword_plugin_mutex)
#  define KEYWORD_PLUGIN_MUTEX_TERM    MUTEX_DESTROY(&PL_keyword_plugin_mutex)
#  define USER_PROP_MUTEX_INIT    MUTEX_INIT(&PL_user_prop_mutex)
#  define USER_PROP_MUTEX_LOCK    MUTEX_LOCK(&PL_user_prop_mutex)
#  define USER_PROP_MUTEX_UNLOCK  MUTEX_UNLOCK(&PL_user_prop_mutex)
#  define USER_PROP_MUTEX_TERM    MUTEX_DESTROY(&PL_user_prop_mutex)
#else
#  define KEYWORD_PLUGIN_MUTEX_INIT    NOOP
#  define KEYWORD_PLUGIN_MUTEX_LOCK    NOOP
#  define KEYWORD_PLUGIN_MUTEX_UNLOCK  NOOP
#  define KEYWORD_PLUGIN_MUTEX_TERM    NOOP
#  define USER_PROP_MUTEX_INIT    NOOP
#  define USER_PROP_MUTEX_LOCK    NOOP
#  define USER_PROP_MUTEX_UNLOCK  NOOP
#  define USER_PROP_MUTEX_TERM    NOOP
#endif

#ifdef USE_LOCALE /* These locale things are all subject to change */

   /* Returns TRUE if the plain locale pragma without a parameter is in effect.
    * */
#  define IN_LOCALE_RUNTIME	(PL_curcop                                  \
                              && CopHINTS_get(PL_curcop) & HINT_LOCALE)

   /* Returns TRUE if either form of the locale pragma is in effect */
#  define IN_SOME_LOCALE_FORM_RUNTIME                                       \
        cBOOL(CopHINTS_get(PL_curcop) & (HINT_LOCALE|HINT_LOCALE_PARTIAL))

#  define IN_LOCALE_COMPILETIME	cBOOL(PL_hints & HINT_LOCALE)
#  define IN_SOME_LOCALE_FORM_COMPILETIME                                   \
                        cBOOL(PL_hints & (HINT_LOCALE|HINT_LOCALE_PARTIAL))

/*
=head1 Locale-related functions and macros

=for apidoc Amn|bool|IN_LOCALE

Evaluates to TRUE if the plain locale pragma without a parameter (S<C<use
locale>>) is in effect.

=for apidoc Amn|bool|IN_LOCALE_COMPILETIME

Evaluates to TRUE if, when compiling a perl program (including an C<eval>) if
the plain locale pragma without a parameter (S<C<use locale>>) is in effect.

=for apidoc Amn|bool|IN_LOCALE_RUNTIME

Evaluates to TRUE if, when executing a perl program (including an C<eval>) if
the plain locale pragma without a parameter (S<C<use locale>>) is in effect.

=cut
*/

#  define IN_LOCALE                                                         \
        (IN_PERL_COMPILETIME ? IN_LOCALE_COMPILETIME : IN_LOCALE_RUNTIME)
#  define IN_SOME_LOCALE_FORM                                               \
                    (IN_PERL_COMPILETIME ? IN_SOME_LOCALE_FORM_COMPILETIME  \
                                         : IN_SOME_LOCALE_FORM_RUNTIME)

#  define IN_LC_ALL_COMPILETIME   IN_LOCALE_COMPILETIME
#  define IN_LC_ALL_RUNTIME       IN_LOCALE_RUNTIME

#  define IN_LC_PARTIAL_COMPILETIME   cBOOL(PL_hints & HINT_LOCALE_PARTIAL)
#  define IN_LC_PARTIAL_RUNTIME                                             \
              (PL_curcop && CopHINTS_get(PL_curcop) & HINT_LOCALE_PARTIAL)

#  define IN_LC_COMPILETIME(category)                                       \
       (       IN_LC_ALL_COMPILETIME                                        \
        || (   IN_LC_PARTIAL_COMPILETIME                                    \
            && Perl__is_in_locale_category(aTHX_ TRUE, (category))))
#  define IN_LC_RUNTIME(category)                                           \
      (IN_LC_ALL_RUNTIME || (IN_LC_PARTIAL_RUNTIME                          \
                 && Perl__is_in_locale_category(aTHX_ FALSE, (category))))
#  define IN_LC(category)  \
                    (IN_LC_COMPILETIME(category) || IN_LC_RUNTIME(category))

#  if defined (PERL_CORE) || defined (PERL_IN_XSUB_RE)

     /* This internal macro should be called from places that operate under
      * locale rules.  If there is a problem with the current locale that
      * hasn't been raised yet, it will output a warning this time.  Because
      * this will so rarely  be true, there is no point to optimize for time;
      * instead it makes sense to minimize space used and do all the work in
      * the rarely called function */
#    ifdef USE_LOCALE_CTYPE
#      define _CHECK_AND_WARN_PROBLEMATIC_LOCALE                              \
                STMT_START {                                                  \
                    if (UNLIKELY(PL_warn_locale)) {                           \
                        Perl__warn_problematic_locale();                      \
                    }                                                         \
                }  STMT_END
#    else
#      define _CHECK_AND_WARN_PROBLEMATIC_LOCALE
#    endif


     /* These two internal macros are called when a warning should be raised,
      * and will do so if enabled.  The first takes a single code point
      * argument; the 2nd, is a pointer to the first byte of the UTF-8 encoded
      * string, and an end position which it won't try to read past */
#    define _CHECK_AND_OUTPUT_WIDE_LOCALE_CP_MSG(cp)                        \
	STMT_START {                                                        \
            if (! PL_in_utf8_CTYPE_locale && ckWARN(WARN_LOCALE)) {         \
                Perl_warner(aTHX_ packWARN(WARN_LOCALE),                    \
                                       "Wide character (U+%" UVXf ") in %s",\
                                       (UV) cp, OP_DESC(PL_op));            \
            }                                                               \
        }  STMT_END

#    define _CHECK_AND_OUTPUT_WIDE_LOCALE_UTF8_MSG(s, send)                 \
	STMT_START { /* Check if to warn before doing the conversion work */\
            if (! PL_in_utf8_CTYPE_locale && ckWARN(WARN_LOCALE)) {         \
                UV cp = utf8_to_uvchr_buf((U8 *) (s), (U8 *) (send), NULL); \
                Perl_warner(aTHX_ packWARN(WARN_LOCALE),                    \
                    "Wide character (U+%" UVXf ") in %s",                   \
                    (cp == 0)                                               \
                     ? UNICODE_REPLACEMENT                                  \
                     : (UV) cp,                                             \
                    OP_DESC(PL_op));                                        \
            }                                                               \
        }  STMT_END

#  endif   /* PERL_CORE or PERL_IN_XSUB_RE */
#else   /* No locale usage */
#  define IN_LOCALE_RUNTIME                0
#  define IN_SOME_LOCALE_FORM_RUNTIME      0
#  define IN_LOCALE_COMPILETIME            0
#  define IN_SOME_LOCALE_FORM_COMPILETIME  0
#  define IN_LOCALE                        0
#  define IN_SOME_LOCALE_FORM              0
#  define IN_LC_ALL_COMPILETIME            0
#  define IN_LC_ALL_RUNTIME                0
#  define IN_LC_PARTIAL_COMPILETIME        0
#  define IN_LC_PARTIAL_RUNTIME            0
#  define IN_LC_COMPILETIME(category)      0
#  define IN_LC_RUNTIME(category)          0
#  define IN_LC(category)                  0
#  define _CHECK_AND_WARN_PROBLEMATIC_LOCALE
#  define _CHECK_AND_OUTPUT_WIDE_LOCALE_UTF8_MSG(s, send)
#  define _CHECK_AND_OUTPUT_WIDE_LOCALE_CP_MSG(c)
#endif


/* Locale/thread synchronization macros.  These aren't needed if using
 * thread-safe locale operations, except if something is broken */
#if    defined(USE_LOCALE)                                                  \
 &&    defined(USE_ITHREADS)                                                \
 && (! defined(USE_THREAD_SAFE_LOCALE) || defined(TS_W32_BROKEN_LOCALECONV))

/* We have a locale object holding the 'C' locale for Posix 2008 */
#  ifndef USE_POSIX_2008_LOCALE
#    define _LOCALE_TERM_POSIX_2008  NOOP
#  else
#    define _LOCALE_TERM_POSIX_2008                                         \
                    STMT_START {                                            \
                        if (PL_C_locale_obj) {                              \
                            /* Make sure we aren't using the locale         \
                             * space we are about to free */                \
                            uselocale(LC_GLOBAL_LOCALE);                    \
                            freelocale(PL_C_locale_obj);                    \
                            PL_C_locale_obj = (locale_t) NULL;              \
                        }                                                   \
                    } STMT_END
#  endif

/* This is used as a generic lock for locale operations.  For example this is
 * used when calling nl_langinfo() so that another thread won't zap the
 * contents of its buffer before it gets saved; and it's called when changing
 * the locale of LC_MESSAGES.  On some systems the latter can cause the
 * nl_langinfo buffer to be zapped under a race condition.
 *
 * If combined with LC_NUMERIC_LOCK, calls to this and its corresponding unlock
 * should be contained entirely within the locked portion of LC_NUMERIC.  This
 * mutex should be used only in very short sections of code, while
 * LC_NUMERIC_LOCK may span more operations.  By always following this
 * convention, deadlock should be impossible.  But if necessary, the two
 * mutexes could be combined.
 *
 * Actually, the two macros just below with the '_V' suffixes are used in just
 * a few places where there is a broken localeconv(), but otherwise things are
 * thread safe, and hence don't need locking.  Just below LOCALE_LOCK and
 * LOCALE_UNLOCK are defined in terms of these for use everywhere else */
#  define LOCALE_LOCK_V                                                     \
        STMT_START {                                                        \
            DEBUG_Lv(PerlIO_printf(Perl_debug_log,                          \
                    "%s: %d: locking locale\n", __FILE__, __LINE__));       \
            MUTEX_LOCK(&PL_locale_mutex);                                   \
        } STMT_END
#  define LOCALE_UNLOCK_V                                                   \
        STMT_START {                                                        \
            DEBUG_Lv(PerlIO_printf(Perl_debug_log,                          \
                   "%s: %d: unlocking locale\n", __FILE__, __LINE__));      \
            MUTEX_UNLOCK(&PL_locale_mutex);                                 \
        } STMT_END

/* On windows, we just need the mutex for LOCALE_LOCK */
#  ifdef TS_W32_BROKEN_LOCALECONV
#    define LOCALE_LOCK     NOOP
#    define LOCALE_UNLOCK   NOOP
#    define LOCALE_INIT     MUTEX_INIT(&PL_locale_mutex);
#    define LOCALE_TERM     MUTEX_DESTROY(&PL_locale_mutex)
#    define LC_NUMERIC_LOCK(cond)
#    define LC_NUMERIC_UNLOCK
#  else
#    define LOCALE_LOCK     LOCALE_LOCK_V
#    define LOCALE_UNLOCK   LOCALE_UNLOCK_V

     /* We also need to lock LC_NUMERIC for non-windows (hence Posix 2008)
      * systems */
#    define LOCALE_INIT          STMT_START {                               \
                                    MUTEX_INIT(&PL_locale_mutex);           \
                                    MUTEX_INIT(&PL_lc_numeric_mutex);       \
                                } STMT_END

#    define LOCALE_TERM         STMT_START {                                \
                                    MUTEX_DESTROY(&PL_locale_mutex);        \
                                    MUTEX_DESTROY(&PL_lc_numeric_mutex);    \
                                    _LOCALE_TERM_POSIX_2008;                \
                                } STMT_END

    /* This mutex is used to create critical sections where we want the
     * LC_NUMERIC locale to be locked into either the C (standard) locale, or
     * the underlying locale, so that other threads interrupting this one don't
     * change it to the wrong state before we've had a chance to complete our
     * operation.  It can stay locked over an entire printf operation, for
     * example.  And so is made distinct from the LOCALE_LOCK mutex.
     *
     * This simulates kind of a general semaphore.  The current thread will
     * lock the mutex if the per-thread variable is zero, and then increments
     * that variable.  Each corresponding UNLOCK decrements the variable until
     * it is 0, at which point it actually unlocks the mutex.  Since the
     * variable is per-thread, there is no race with other threads.
     *
     * The single argument is a condition to test for, and if true, to panic,
     * as this would be an attempt to complement the LC_NUMERIC state, and
     * we're not supposed to because it's locked.
     *
     * Clang improperly gives warnings for this, if not silenced:
     * https://clang.llvm.org/docs/ThreadSafetyAnalysis.html#conditional-locks
     * */
#    define LC_NUMERIC_LOCK(cond_to_panic_if_already_locked)                \
        CLANG_DIAG_IGNORE(-Wthread-safety)	     	                    \
        STMT_START {                                                        \
            if (PL_lc_numeric_mutex_depth <= 0) {                           \
                MUTEX_LOCK(&PL_lc_numeric_mutex);                           \
                PL_lc_numeric_mutex_depth = 1;                              \
                DEBUG_Lv(PerlIO_printf(Perl_debug_log,                      \
                         "%s: %d: locking lc_numeric; depth=1\n",           \
                         __FILE__, __LINE__));                              \
            }                                                               \
            else {                                                          \
                PL_lc_numeric_mutex_depth++;                                \
                DEBUG_Lv(PerlIO_printf(Perl_debug_log,                      \
                        "%s: %d: avoided lc_numeric_lock; new depth=%d\n",  \
                        __FILE__, __LINE__, PL_lc_numeric_mutex_depth));    \
                if (cond_to_panic_if_already_locked) {                      \
                    Perl_croak_nocontext("panic: %s: %d: Trying to change"  \
                                         " LC_NUMERIC incompatibly",        \
                                         __FILE__, __LINE__);               \
                }                                                           \
            }                                                               \
        } STMT_END

#    define LC_NUMERIC_UNLOCK                                               \
        STMT_START {                                                        \
            if (PL_lc_numeric_mutex_depth <= 1) {                           \
                MUTEX_UNLOCK(&PL_lc_numeric_mutex);                         \
                PL_lc_numeric_mutex_depth = 0;                              \
                DEBUG_Lv(PerlIO_printf(Perl_debug_log,                      \
                         "%s: %d: unlocking lc_numeric; depth=0\n",         \
                         __FILE__, __LINE__));                              \
            }                                                               \
            else {                                                          \
                PL_lc_numeric_mutex_depth--;                                \
                DEBUG_Lv(PerlIO_printf(Perl_debug_log,                      \
                        "%s: %d: avoided lc_numeric_unlock; new depth=%d\n",\
                        __FILE__, __LINE__, PL_lc_numeric_mutex_depth));    \
            }                                                               \
        } STMT_END                                                          \
        CLANG_DIAG_RESTORE

#  endif    /* End of needs locking LC_NUMERIC */
#else   /* Below is no locale sync needed */
#  define LOCALE_INIT
#  define LOCALE_LOCK
#  define LOCALE_LOCK_V
#  define LOCALE_UNLOCK
#  define LOCALE_UNLOCK_V
#  define LC_NUMERIC_LOCK(cond)
#  define LC_NUMERIC_UNLOCK
#  define LOCALE_TERM
#endif

#ifdef USE_LOCALE_NUMERIC

/* These macros are for toggling between the underlying locale (UNDERLYING or
 * LOCAL) and the C locale (STANDARD).  (Actually we don't have to use the C
 * locale if the underlying locale is indistinguishable from it in the numeric
 * operations used by Perl, namely the decimal point, and even the thousands
 * separator.)

=head1 Locale-related functions and macros

=for apidoc Amn|void|DECLARATION_FOR_LC_NUMERIC_MANIPULATION

This macro should be used as a statement.  It declares a private variable
(whose name begins with an underscore) that is needed by the other macros in
this section.  Failing to include this correctly should lead to a syntax error.
For compatibility with C89 C compilers it should be placed in a block before
any executable statements.

=for apidoc Am|void|STORE_LC_NUMERIC_FORCE_TO_UNDERLYING

This is used by XS code that is C<LC_NUMERIC> locale-aware to force the
locale for category C<LC_NUMERIC> to be what perl thinks is the current
underlying locale.  (The perl interpreter could be wrong about what the
underlying locale actually is if some C or XS code has called the C library
function L<setlocale(3)> behind its back; calling L</sync_locale> before calling
this macro will update perl's records.)

A call to L</DECLARATION_FOR_LC_NUMERIC_MANIPULATION> must have been made to
declare at compile time a private variable used by this macro.  This macro
should be called as a single statement, not an expression, but with an empty
argument list, like this:

 {
    DECLARATION_FOR_LC_NUMERIC_MANIPULATION;
     ...
    STORE_LC_NUMERIC_FORCE_TO_UNDERLYING();
     ...
    RESTORE_LC_NUMERIC();
     ...
 }

The private variable is used to save the current locale state, so
that the requisite matching call to L</RESTORE_LC_NUMERIC> can restore it.

On threaded perls not operating with thread-safe functionality, this macro uses
a mutex to force a critical section.  Therefore the matching RESTORE should be
close by, and guaranteed to be called.

=for apidoc Am|void|STORE_LC_NUMERIC_SET_TO_NEEDED

This is used to help wrap XS or C code that is C<LC_NUMERIC> locale-aware.
This locale category is generally kept set to a locale where the decimal radix
character is a dot, and the separator between groups of digits is empty.  This
is because most XS code that reads floating point numbers is expecting them to
have this syntax.

This macro makes sure the current C<LC_NUMERIC> state is set properly, to be
aware of locale if the call to the XS or C code from the Perl program is
from within the scope of a S<C<use locale>>; or to ignore locale if the call is
instead from outside such scope.

This macro is the start of wrapping the C or XS code; the wrap ending is done
by calling the L</RESTORE_LC_NUMERIC> macro after the operation.  Otherwise
the state can be changed that will adversely affect other XS code.

A call to L</DECLARATION_FOR_LC_NUMERIC_MANIPULATION> must have been made to
declare at compile time a private variable used by this macro.  This macro
should be called as a single statement, not an expression, but with an empty
argument list, like this:

 {
    DECLARATION_FOR_LC_NUMERIC_MANIPULATION;
     ...
    STORE_LC_NUMERIC_SET_TO_NEEDED();
     ...
    RESTORE_LC_NUMERIC();
     ...
 }

On threaded perls not operating with thread-safe functionality, this macro uses
a mutex to force a critical section.  Therefore the matching RESTORE should be
close by, and guaranteed to be called; see L</WITH_LC_NUMERIC_SET_TO_NEEDED>
for a more contained way to ensure that.

=for apidoc Am|void|STORE_LC_NUMERIC_SET_TO_NEEDED_IN|bool in_lc_numeric

Same as L</STORE_LC_NUMERIC_SET_TO_NEEDED> with in_lc_numeric provided
as the precalculated value of C<IN_LC(LC_NUMERIC)>. It is the caller's
responsibility to ensure that the status of C<PL_compiling> and C<PL_hints>
cannot have changed since the precalculation.

=for apidoc Am|void|RESTORE_LC_NUMERIC

This is used in conjunction with one of the macros
L</STORE_LC_NUMERIC_SET_TO_NEEDED>
and L</STORE_LC_NUMERIC_FORCE_TO_UNDERLYING> to properly restore the
C<LC_NUMERIC> state.

A call to L</DECLARATION_FOR_LC_NUMERIC_MANIPULATION> must have been made to
declare at compile time a private variable used by this macro and the two
C<STORE> ones.  This macro should be called as a single statement, not an
expression, but with an empty argument list, like this:

 {
    DECLARATION_FOR_LC_NUMERIC_MANIPULATION;
     ...
    RESTORE_LC_NUMERIC();
     ...
 }

=for apidoc Am|void|WITH_LC_NUMERIC_SET_TO_NEEDED|block

This macro invokes the supplied statement or block within the context
of a L</STORE_LC_NUMERIC_SET_TO_NEEDED> .. L</RESTORE_LC_NUMERIC> pair
if required, so eg:

  WITH_LC_NUMERIC_SET_TO_NEEDED(
    SNPRINTF_G(fv, ebuf, sizeof(ebuf), precis)
  );

is equivalent to:

  {
#ifdef USE_LOCALE_NUMERIC
    DECLARATION_FOR_LC_NUMERIC_MANIPULATION;
    STORE_LC_NUMERIC_SET_TO_NEEDED();
#endif
    SNPRINTF_G(fv, ebuf, sizeof(ebuf), precis);
#ifdef USE_LOCALE_NUMERIC
    RESTORE_LC_NUMERIC();
#endif
  }

=for apidoc Am|void|WITH_LC_NUMERIC_SET_TO_NEEDED_IN|bool in_lc_numeric|block

Same as L</WITH_LC_NUMERIC_SET_TO_NEEDED> with in_lc_numeric provided
as the precalculated value of C<IN_LC(LC_NUMERIC)>. It is the caller's
responsibility to ensure that the status of C<PL_compiling> and C<PL_hints>
cannot have changed since the precalculation.

=cut

*/

/* If the underlying numeric locale has a non-dot decimal point or has a
 * non-empty floating point thousands separator, the current locale is instead
 * generally kept in the C locale instead of that underlying locale.  The
 * current status is known by looking at two words.  One is non-zero if the
 * current numeric locale is the standard C/POSIX one or is indistinguishable
 * from C.  The other is non-zero if the current locale is the underlying
 * locale.  Both can be non-zero if, as often happens, the underlying locale is
 * C or indistinguishable from it.
 *
 * khw believes the reason for the variables instead of the bits in a single
 * word is to avoid having to have masking instructions. */

#  define _NOT_IN_NUMERIC_STANDARD (! PL_numeric_standard)

/* We can lock the category to stay in the C locale, making requests to the
 * contrary be noops, in the dynamic scope by setting PL_numeric_standard to 2.
 * */
#  define _NOT_IN_NUMERIC_UNDERLYING                                        \
                    (! PL_numeric_underlying && PL_numeric_standard < 2)

#  define DECLARATION_FOR_LC_NUMERIC_MANIPULATION                           \
    void (*_restore_LC_NUMERIC_function)(pTHX) = NULL

#  define STORE_LC_NUMERIC_SET_TO_NEEDED_IN(in)                             \
        STMT_START {                                                        \
            bool _in_lc_numeric = (in);                                     \
            LC_NUMERIC_LOCK(                                                \
                    (   (  _in_lc_numeric && _NOT_IN_NUMERIC_UNDERLYING)    \
                     || (! _in_lc_numeric && _NOT_IN_NUMERIC_STANDARD)));   \
            if (_in_lc_numeric) {                                           \
                if (_NOT_IN_NUMERIC_UNDERLYING) {                           \
                    Perl_set_numeric_underlying(aTHX);                      \
                    _restore_LC_NUMERIC_function                            \
                                            = &Perl_set_numeric_standard;   \
                }                                                           \
            }                                                               \
            else {                                                          \
                if (_NOT_IN_NUMERIC_STANDARD) {                             \
                    Perl_set_numeric_standard(aTHX);                        \
                    _restore_LC_NUMERIC_function                            \
                                            = &Perl_set_numeric_underlying; \
                }                                                           \
            }                                                               \
        } STMT_END

#  define STORE_LC_NUMERIC_SET_TO_NEEDED() \
        STORE_LC_NUMERIC_SET_TO_NEEDED_IN(IN_LC(LC_NUMERIC))

#  define RESTORE_LC_NUMERIC()                                              \
        STMT_START {                                                        \
            if (_restore_LC_NUMERIC_function) {                             \
                _restore_LC_NUMERIC_function(aTHX);                         \
            }                                                               \
            LC_NUMERIC_UNLOCK;                                              \
        } STMT_END

/* The next two macros set unconditionally.  These should be rarely used, and
 * only after being sure that this is what is needed */
#  define SET_NUMERIC_STANDARD()                                            \
	STMT_START {                                                        \
            DEBUG_Lv(PerlIO_printf(Perl_debug_log,                          \
                               "%s: %d: lc_numeric standard=%d\n",          \
                                __FILE__, __LINE__, PL_numeric_standard));  \
            Perl_set_numeric_standard(aTHX);                                \
            DEBUG_Lv(PerlIO_printf(Perl_debug_log,                          \
                                 "%s: %d: lc_numeric standard=%d\n",        \
                                 __FILE__, __LINE__, PL_numeric_standard)); \
        } STMT_END

#  define SET_NUMERIC_UNDERLYING()                                          \
	STMT_START {                                                        \
            if (_NOT_IN_NUMERIC_UNDERLYING) {                               \
                Perl_set_numeric_underlying(aTHX);                          \
            }                                                               \
        } STMT_END

/* The rest of these LC_NUMERIC macros toggle to one or the other state, with
 * the RESTORE_foo ones called to switch back, but only if need be */
#  define STORE_LC_NUMERIC_SET_STANDARD()                                   \
        STMT_START {                                                        \
            LC_NUMERIC_LOCK(_NOT_IN_NUMERIC_STANDARD);                      \
            if (_NOT_IN_NUMERIC_STANDARD) {                                 \
                _restore_LC_NUMERIC_function = &Perl_set_numeric_underlying;\
                Perl_set_numeric_standard(aTHX);                            \
            }                                                               \
        } STMT_END

/* Rarely, we want to change to the underlying locale even outside of 'use
 * locale'.  This is principally in the POSIX:: functions */
#  define STORE_LC_NUMERIC_FORCE_TO_UNDERLYING()                            \
	STMT_START {                                                        \
            LC_NUMERIC_LOCK(_NOT_IN_NUMERIC_UNDERLYING);                    \
            if (_NOT_IN_NUMERIC_UNDERLYING) {                               \
                Perl_set_numeric_underlying(aTHX);                          \
                _restore_LC_NUMERIC_function = &Perl_set_numeric_standard;  \
            }                                                               \
        } STMT_END

/* Lock/unlock to the C locale until unlock is called.  This needs to be
 * recursively callable.  [perl #128207] */
#  define LOCK_LC_NUMERIC_STANDARD()                                        \
        STMT_START {                                                        \
            DEBUG_Lv(PerlIO_printf(Perl_debug_log,                          \
                      "%s: %d: lock lc_numeric_standard: new depth=%d\n",   \
                      __FILE__, __LINE__, PL_numeric_standard + 1));        \
            __ASSERT_(PL_numeric_standard)                                  \
            PL_numeric_standard++;                                          \
        } STMT_END

#  define UNLOCK_LC_NUMERIC_STANDARD()                                      \
        STMT_START {                                                        \
            if (PL_numeric_standard > 1) {                                  \
                PL_numeric_standard--;                                      \
            }                                                               \
            else {                                                          \
                assert(0);                                                  \
            }                                                               \
            DEBUG_Lv(PerlIO_printf(Perl_debug_log,                          \
            "%s: %d: lc_numeric_standard decrement lock, new depth=%d\n",   \
            __FILE__, __LINE__, PL_numeric_standard));                      \
        } STMT_END

#  define WITH_LC_NUMERIC_SET_TO_NEEDED_IN(in_lc_numeric, block)            \
        STMT_START {                                                        \
            DECLARATION_FOR_LC_NUMERIC_MANIPULATION;                        \
            STORE_LC_NUMERIC_SET_TO_NEEDED_IN(in_lc_numeric);               \
            block;                                                          \
            RESTORE_LC_NUMERIC();                                           \
        } STMT_END;

#  define WITH_LC_NUMERIC_SET_TO_NEEDED(block) \
        WITH_LC_NUMERIC_SET_TO_NEEDED_IN(IN_LC(LC_NUMERIC), block)

#else /* !USE_LOCALE_NUMERIC */

#  define SET_NUMERIC_STANDARD()
#  define SET_NUMERIC_UNDERLYING()
#  define IS_NUMERIC_RADIX(a, b)		(0)
#  define DECLARATION_FOR_LC_NUMERIC_MANIPULATION  dNOOP
#  define STORE_LC_NUMERIC_SET_STANDARD()
#  define STORE_LC_NUMERIC_FORCE_TO_UNDERLYING()
#  define STORE_LC_NUMERIC_SET_TO_NEEDED_IN(in_lc_numeric)
#  define STORE_LC_NUMERIC_SET_TO_NEEDED()
#  define RESTORE_LC_NUMERIC()
#  define LOCK_LC_NUMERIC_STANDARD()
#  define UNLOCK_LC_NUMERIC_STANDARD()
#  define WITH_LC_NUMERIC_SET_TO_NEEDED_IN(in_lc_numeric, block) \
    STMT_START { block; } STMT_END
#  define WITH_LC_NUMERIC_SET_TO_NEEDED(block) \
    STMT_START { block; } STMT_END

#endif /* !USE_LOCALE_NUMERIC */

#define Atof				my_atof

/*

=head1 Numeric functions

=for apidoc AmTR|NV|Strtod|NN const char * const s|NULLOK char ** e

This is a synonym for L</my_strtod>.

=for apidoc AmTR|NV|Strtol|NN const char * const s|NULLOK char ** e|int base

Platform and configuration independent C<strtol>.  This expands to the
appropriate C<strotol>-like function based on the platform and F<Configure>
options>.  For example it could expand to C<strtoll> or C<strtoq> instead of
C<strtol>.

=for apidoc AmTR|NV|Strtoul|NN const char * const s|NULLOK char ** e|int base

Platform and configuration independent C<strtoul>.  This expands to the
appropriate C<strotoul>-like function based on the platform and F<Configure>
options>.  For example it could expand to C<strtoull> or C<strtouq> instead of
C<strtoul>.

=cut

*/

#define Strtod                          my_strtod

#if    defined(HAS_STRTOD)                                          \
   ||  defined(USE_QUADMATH)                                        \
   || (defined(HAS_STRTOLD) && defined(HAS_LONG_DOUBLE)             \
                            && defined(USE_LONG_DOUBLE))
#  define Perl_strtod   Strtod
#endif

#if !defined(Strtol) && defined(USE_64_BIT_INT) && defined(IV_IS_QUAD) && \
	(QUADKIND == QUAD_IS_LONG_LONG || QUADKIND == QUAD_IS___INT64)
#    ifdef __hpux
#        define strtoll __strtoll	/* secret handshake */
#    endif
#    if defined(WIN64) && defined(_MSC_VER)
#        define strtoll _strtoi64	/* secret handshake */
#    endif
#   if !defined(Strtol) && defined(HAS_STRTOLL)
#       define Strtol	strtoll
#   endif
#    if !defined(Strtol) && defined(HAS_STRTOQ)
#       define Strtol	strtoq
#    endif
/* is there atoq() anywhere? */
#endif
#if !defined(Strtol) && defined(HAS_STRTOL)
#   define Strtol	strtol
#endif
#ifndef Atol
/* It would be more fashionable to use Strtol() to define atol()
 * (as is done for Atoul(), see below) but for backward compatibility
 * we just assume atol(). */
#   if defined(USE_64_BIT_INT) && defined(IV_IS_QUAD) && defined(HAS_ATOLL) && \
	(QUADKIND == QUAD_IS_LONG_LONG || QUADKIND == QUAD_IS___INT64)
#    ifdef WIN64
#       define atoll    _atoi64		/* secret handshake */
#    endif
#       define Atol	atoll
#   else
#       define Atol	atol
#   endif
#endif

#if !defined(Strtoul) && defined(USE_64_BIT_INT) && defined(UV_IS_QUAD) && \
	(QUADKIND == QUAD_IS_LONG_LONG || QUADKIND == QUAD_IS___INT64)
#    ifdef __hpux
#        define strtoull __strtoull	/* secret handshake */
#    endif
#    if defined(WIN64) && defined(_MSC_VER)
#        define strtoull _strtoui64	/* secret handshake */
#    endif
#    if !defined(Strtoul) && defined(HAS_STRTOULL)
#       define Strtoul	strtoull
#    endif
#    if !defined(Strtoul) && defined(HAS_STRTOUQ)
#       define Strtoul	strtouq
#    endif
/* is there atouq() anywhere? */
#endif
#if !defined(Strtoul) && defined(HAS_STRTOUL)
#   define Strtoul	strtoul
#endif
#if !defined(Strtoul) && defined(HAS_STRTOL) /* Last resort. */
#   define Strtoul(s, e, b)	strchr((s), '-') ? ULONG_MAX : (unsigned long)strtol((s), (e), (b))
#endif
#ifndef Atoul
#   define Atoul(s)	Strtoul(s, NULL, 10)
#endif

#define grok_bin(s,lp,fp,rp)                                                \
                    grok_bin_oct_hex(s, lp, fp, rp, 1, _CC_BINDIGIT, 'b')
#define grok_oct(s,lp,fp,rp)                                                \
                    (*(fp) |= PERL_SCAN_DISALLOW_PREFIX,                    \
                    grok_bin_oct_hex(s, lp, fp, rp, 3, _CC_OCTDIGIT, '\0'))
#define grok_hex(s,lp,fp,rp)                                                \
                    grok_bin_oct_hex(s, lp, fp, rp, 4, _CC_XDIGIT, 'x')

#ifndef PERL_SCRIPT_MODE
#define PERL_SCRIPT_MODE "r"
#endif

/* not used. Kept as a NOOP for backcompat */
#define PERL_STACK_OVERFLOW_CHECK()  NOOP

/*
 * Some nonpreemptive operating systems find it convenient to
 * check for asynchronous conditions after each op execution.
 * Keep this check simple, or it may slow down execution
 * massively.
 */

#ifndef PERL_MICRO
#	ifndef PERL_ASYNC_CHECK
#		define PERL_ASYNC_CHECK() if (UNLIKELY(PL_sig_pending)) PL_signalhook(aTHX)
#	endif
#endif

#ifndef PERL_ASYNC_CHECK
#   define PERL_ASYNC_CHECK()  NOOP
#endif

/*
 * On some operating systems, a memory allocation may succeed,
 * but put the process too close to the system's comfort limit.
 * In this case, PERL_ALLOC_CHECK frees the pointer and sets
 * it to NULL.
 */
#ifndef PERL_ALLOC_CHECK
#define PERL_ALLOC_CHECK(p)  NOOP
#endif

#ifdef HAS_SEM
#   include <sys/ipc.h>
#   include <sys/sem.h>
#   ifndef HAS_UNION_SEMUN	/* Provide the union semun. */
    union semun {
	int		val;
	struct semid_ds	*buf;
	unsigned short	*array;
    };
#   endif
#   ifdef USE_SEMCTL_SEMUN
#	ifdef IRIX32_SEMUN_BROKEN_BY_GCC
            union gccbug_semun {
		int             val;
		struct semid_ds *buf;
		unsigned short  *array;
		char            __dummy[5];
	    };
#           define semun gccbug_semun
#	endif
#       define Semctl(id, num, cmd, semun) semctl(id, num, cmd, semun)
#   elif defined(USE_SEMCTL_SEMID_DS)
#           ifdef EXTRA_F_IN_SEMUN_BUF
#               define Semctl(id, num, cmd, semun) semctl(id, num, cmd, semun.buff)
#           else
#               define Semctl(id, num, cmd, semun) semctl(id, num, cmd, semun.buf)
#           endif
#   endif
#endif

/*
 * Boilerplate macros for initializing and accessing interpreter-local
 * data from C.  All statics in extensions should be reworked to use
 * this, if you want to make the extension thread-safe.  See
 * ext/XS/APItest/APItest.xs for an example of the use of these macros,
 * and perlxs.pod for more.
 *
 * Code that uses these macros is responsible for the following:
 * 1. #define MY_CXT_KEY to a unique string, e.g.
 *    "DynaLoader::_guts" XS_VERSION
 *    XXX in the current implementation, this string is ignored.
 * 2. Declare a typedef named my_cxt_t that is a structure that contains
 *    all the data that needs to be interpreter-local.
 * 3. Use the START_MY_CXT macro after the declaration of my_cxt_t.
 * 4. Use the MY_CXT_INIT macro such that it is called exactly once
 *    (typically put in the BOOT: section).
 * 5. Use the members of the my_cxt_t structure everywhere as
 *    MY_CXT.member.
 * 6. Use the dMY_CXT macro (a declaration) in all the functions that
 *    access MY_CXT.
 */

#if defined(PERL_IMPLICIT_CONTEXT)

/* START_MY_CXT must appear in all extensions that define a my_cxt_t structure,
 * right after the definition (i.e. at file scope).  The non-threads
 * case below uses it to declare the data as static. */
#  ifdef PERL_GLOBAL_STRUCT_PRIVATE
#    define START_MY_CXT
#    define MY_CXT_INDEX Perl_my_cxt_index(aTHX_ MY_CXT_KEY)
#    define MY_CXT_INIT_ARG MY_CXT_KEY
#  else
#    define START_MY_CXT static int my_cxt_index = -1;
#    define MY_CXT_INDEX my_cxt_index
#    define MY_CXT_INIT_ARG &my_cxt_index
#  endif /* #ifdef PERL_GLOBAL_STRUCT_PRIVATE */

/* Creates and zeroes the per-interpreter data.
 * (We allocate my_cxtp in a Perl SV so that it will be released when
 * the interpreter goes away.) */
#  define MY_CXT_INIT \
	my_cxt_t *my_cxtp = \
	    (my_cxt_t*)Perl_my_cxt_init(aTHX_ MY_CXT_INIT_ARG, sizeof(my_cxt_t)); \
	PERL_UNUSED_VAR(my_cxtp)
#  define MY_CXT_INIT_INTERP(my_perl) \
	my_cxt_t *my_cxtp = \
	    (my_cxt_t*)Perl_my_cxt_init(my_perl, MY_CXT_INIT_ARG, sizeof(my_cxt_t)); \
	PERL_UNUSED_VAR(my_cxtp)

/* This declaration should be used within all functions that use the
 * interpreter-local data. */
#  define dMY_CXT	\
	my_cxt_t *my_cxtp = (my_cxt_t *)PL_my_cxt_list[MY_CXT_INDEX]
#  define dMY_CXT_INTERP(my_perl)	\
	my_cxt_t *my_cxtp = (my_cxt_t *)(my_perl)->Imy_cxt_list[MY_CXT_INDEX]

/* Clones the per-interpreter data. */
#  define MY_CXT_CLONE \
	my_cxt_t *my_cxtp = (my_cxt_t*)SvPVX(newSV(sizeof(my_cxt_t)-1));\
	void * old_my_cxtp = PL_my_cxt_list[MY_CXT_INDEX];		\
	PL_my_cxt_list[MY_CXT_INDEX] = my_cxtp;				\
	Copy(old_my_cxtp, my_cxtp, 1, my_cxt_t);



/* This macro must be used to access members of the my_cxt_t structure.
 * e.g. MY_CXT.some_data */
#  define MY_CXT		(*my_cxtp)

/* Judicious use of these macros can reduce the number of times dMY_CXT
 * is used.  Use is similar to pTHX, aTHX etc. */
#  define pMY_CXT	my_cxt_t *my_cxtp
#  define pMY_CXT_	pMY_CXT,
#  define _pMY_CXT	,pMY_CXT
#  define aMY_CXT	my_cxtp
#  define aMY_CXT_	aMY_CXT,
#  define _aMY_CXT	,aMY_CXT

#else /* PERL_IMPLICIT_CONTEXT */

#  define START_MY_CXT		static my_cxt_t my_cxt;
#  define dMY_CXT_SV	    	dNOOP
#  define dMY_CXT		dNOOP
#  define dMY_CXT_INTERP(my_perl) dNOOP
#  define MY_CXT_INIT		NOOP
#  define MY_CXT_CLONE		NOOP
#  define MY_CXT		my_cxt

#  define pMY_CXT		void
#  define pMY_CXT_
#  define _pMY_CXT
#  define aMY_CXT
#  define aMY_CXT_
#  define _aMY_CXT

#endif /* !defined(PERL_IMPLICIT_CONTEXT) */

#ifdef I_FCNTL
#  include <fcntl.h>
#endif

#ifdef __Lynx__
#  include <fcntl.h>
#endif

#ifdef __amigaos4__
#  undef FD_CLOEXEC /* a lie in AmigaOS */
#endif

#ifdef I_SYS_FILE
#  include <sys/file.h>
#endif

#if defined(HAS_FLOCK) && !defined(HAS_FLOCK_PROTO)
EXTERN_C int flock(int fd, int op);
#endif

#ifndef O_RDONLY
/* Assume UNIX defaults */
#    define O_RDONLY	0000
#    define O_WRONLY	0001
#    define O_RDWR	0002
#    define O_CREAT	0100
#endif

#ifndef O_BINARY
#  define O_BINARY 0
#endif

#ifndef O_TEXT
#  define O_TEXT 0
#endif

#if O_TEXT != O_BINARY
    /* If you have different O_TEXT and O_BINARY and you are a CRLF shop,
     * that is, you are somehow DOSish. */
#   if defined(__HAIKU__) || defined(__VOS__) || defined(__CYGWIN__)
    /* Haiku has O_TEXT != O_BINARY but O_TEXT and O_BINARY have no effect;
     * Haiku is always UNIXoid (LF), not DOSish (CRLF). */
    /* VOS has O_TEXT != O_BINARY, and they have effect,
     * but VOS always uses LF, never CRLF. */
    /* If you have O_TEXT different from your O_BINARY but you still are
     * not a CRLF shop. */
#       undef PERLIO_USING_CRLF
#   else
    /* If you really are DOSish. */
#      define PERLIO_USING_CRLF 1
#   endif
#endif

#ifdef I_LIBUTIL
#   include <libutil.h>		/* setproctitle() in some FreeBSDs */
#endif

#ifndef EXEC_ARGV_CAST
#define EXEC_ARGV_CAST(x) (char **)x
#endif

#define IS_NUMBER_IN_UV		      0x01 /* number within UV range (maybe not
					      int).  value returned in pointed-
					      to UV */
#define IS_NUMBER_GREATER_THAN_UV_MAX 0x02 /* pointed to UV undefined */
#define IS_NUMBER_NOT_INT	      0x04 /* saw . or E notation or infnan */
#define IS_NUMBER_NEG		      0x08 /* leading minus sign */
#define IS_NUMBER_INFINITY	      0x10 /* this is big */
#define IS_NUMBER_NAN                 0x20 /* this is not */
#define IS_NUMBER_TRAILING            0x40 /* number has trailing trash */

/*
=head1 Numeric functions

=for apidoc AmdR|bool|GROK_NUMERIC_RADIX|NN const char **sp|NN const char *send

A synonym for L</grok_numeric_radix>

=cut
*/
#define GROK_NUMERIC_RADIX(sp, send) grok_numeric_radix(sp, send)

/* Number scan flags.  All are used for input, the ones used for output are so
 * marked */
#define PERL_SCAN_ALLOW_UNDERSCORES   0x01 /* grok_??? accept _ in numbers */
#define PERL_SCAN_DISALLOW_PREFIX     0x02 /* grok_??? reject 0x in hex etc */

/* grok_??? input: ignored; output: found overflow */
#define PERL_SCAN_GREATER_THAN_UV_MAX 0x04

/* grok_??? don't warn about illegal digits.  To preserve total backcompat,
 * this isn't set on output if one is found.  Instead, see
 * PERL_SCAN_NOTIFY_ILLDIGIT. */
#define PERL_SCAN_SILENT_ILLDIGIT     0x08

#define PERL_SCAN_TRAILING            0x10 /* grok_number_flags() allow trailing
                                              and set IS_NUMBER_TRAILING */

/* These are considered experimental, so not exposed publicly */
#if defined(PERL_CORE) || defined(PERL_EXT)
/* grok_??? don't warn about very large numbers which are <= UV_MAX;
 * output: found such a number */
#  define PERL_SCAN_SILENT_NON_PORTABLE 0x20

/* If this is set on input, and no illegal digit is found, it will be cleared
 * on output; otherwise unchanged */
#  define PERL_SCAN_NOTIFY_ILLDIGIT 0x40

/* Don't warn on overflow; output flag still set */
#  define PERL_SCAN_SILENT_OVERFLOW 0x80

/* Forbid a leading underscore, which the other one doesn't */
#  define PERL_SCAN_ALLOW_MEDIAL_UNDERSCORES (0x100|PERL_SCAN_ALLOW_UNDERSCORES)
#endif


/* to let user control profiling */
#ifdef PERL_GPROF_CONTROL
extern void moncontrol(int);
#define PERL_GPROF_MONCONTROL(x) moncontrol(x)
#else
#define PERL_GPROF_MONCONTROL(x)
#endif

/* ISO 6429 NEL - C1 control NExt Line */
/* See https://www.unicode.org/unicode/reports/tr13/ */
#define NEXT_LINE_CHAR	NEXT_LINE_NATIVE

#ifndef PIPESOCK_MODE
#  define PIPESOCK_MODE
#endif

#ifndef SOCKET_OPEN_MODE
#  define SOCKET_OPEN_MODE	PIPESOCK_MODE
#endif

#ifndef PIPE_OPEN_MODE
#  define PIPE_OPEN_MODE	PIPESOCK_MODE
#endif

#define PERL_MAGIC_UTF8_CACHESIZE	2

#define PERL_UNICODE_STDIN_FLAG			0x0001
#define PERL_UNICODE_STDOUT_FLAG		0x0002
#define PERL_UNICODE_STDERR_FLAG		0x0004
#define PERL_UNICODE_IN_FLAG			0x0008
#define PERL_UNICODE_OUT_FLAG			0x0010
#define PERL_UNICODE_ARGV_FLAG			0x0020
#define PERL_UNICODE_LOCALE_FLAG		0x0040
#define PERL_UNICODE_WIDESYSCALLS_FLAG		0x0080 /* for Sarathy */
#define PERL_UNICODE_UTF8CACHEASSERT_FLAG	0x0100

#define PERL_UNICODE_STD_FLAG		\
	(PERL_UNICODE_STDIN_FLAG	| \
	 PERL_UNICODE_STDOUT_FLAG	| \
	 PERL_UNICODE_STDERR_FLAG)

#define PERL_UNICODE_INOUT_FLAG		\
	(PERL_UNICODE_IN_FLAG	| \
	 PERL_UNICODE_OUT_FLAG)

#define PERL_UNICODE_DEFAULT_FLAGS	\
	(PERL_UNICODE_STD_FLAG		| \
	 PERL_UNICODE_INOUT_FLAG	| \
	 PERL_UNICODE_LOCALE_FLAG)

#define PERL_UNICODE_ALL_FLAGS			0x01ff

#define PERL_UNICODE_STDIN			'I'
#define PERL_UNICODE_STDOUT			'O'
#define PERL_UNICODE_STDERR			'E'
#define PERL_UNICODE_STD			'S'
#define PERL_UNICODE_IN				'i'
#define PERL_UNICODE_OUT			'o'
#define PERL_UNICODE_INOUT			'D'
#define PERL_UNICODE_ARGV			'A'
#define PERL_UNICODE_LOCALE			'L'
#define PERL_UNICODE_WIDESYSCALLS		'W'
#define PERL_UNICODE_UTF8CACHEASSERT		'a'

#define PERL_SIGNALS_UNSAFE_FLAG	0x0001

/*
=head1 Numeric functions

=for apidoc Am|int|PERL_ABS|int

Typeless C<abs> or C<fabs>, I<etc>.  (The usage below indicates it is for
integers, but it works for any type.)  Use instead of these, since the C
library ones force their argument to be what it is expecting, potentially
leading to disaster.  But also beware that this evaluates its argument twice,
so no C<x++>.

=cut
*/

#define PERL_ABS(x) ((x) < 0 ? -(x) : (x))

#if defined(__DECC) && defined(__osf__)
#pragma message disable (mainparm) /* Perl uses the envp in main(). */
#endif

#define do_open(g, n, l, a, rm, rp, sf) \
	do_openn(g, n, l, a, rm, rp, sf, (SV **) NULL, 0)
#ifdef PERL_DEFAULT_DO_EXEC3_IMPLEMENTATION
#  define do_exec(cmd)			do_exec3(cmd,0,0)
#endif
#ifdef OS2
#  define do_aexec			Perl_do_aexec
#else
#  define do_aexec(really, mark,sp)	do_aexec5(really, mark, sp, 0, 0)
#endif


/*
=head1 Miscellaneous Functions

=for apidoc Am|bool|IS_SAFE_SYSCALL|NN const char *pv|STRLEN len|NN const char *what|NN const char *op_name

Same as L</is_safe_syscall>.

=cut

Allows one ending \0
*/
#define IS_SAFE_SYSCALL(p, len, what, op_name) (Perl_is_safe_syscall(aTHX_ (p), (len), (what), (op_name)))

#define IS_SAFE_PATHNAME(p, len, op_name) IS_SAFE_SYSCALL((p), (len), "pathname", (op_name))

#if defined(OEMVS) || defined(__amigaos4__)
#define NO_ENV_ARRAY_IN_MAIN
#endif

/* These are used by Perl_pv_escape() and Perl_pv_pretty()
 * are here so that they are available throughout the core
 * NOTE that even though some are for _escape and some for _pretty
 * there must not be any clashes as the flags from _pretty are
 * passed straight through to _escape.
 */

#define PERL_PV_ESCAPE_QUOTE        0x000001
#define PERL_PV_PRETTY_QUOTE        PERL_PV_ESCAPE_QUOTE

#define PERL_PV_PRETTY_ELLIPSES     0x000002
#define PERL_PV_PRETTY_LTGT         0x000004
#define PERL_PV_PRETTY_EXACTSIZE    0x000008

#define PERL_PV_ESCAPE_UNI          0x000100
#define PERL_PV_ESCAPE_UNI_DETECT   0x000200
#define PERL_PV_ESCAPE_NONASCII     0x000400
#define PERL_PV_ESCAPE_FIRSTCHAR    0x000800

#define PERL_PV_ESCAPE_ALL            0x001000
#define PERL_PV_ESCAPE_NOBACKSLASH  0x002000
#define PERL_PV_ESCAPE_NOCLEAR      0x004000
#define PERL_PV_PRETTY_NOCLEAR      PERL_PV_ESCAPE_NOCLEAR
#define PERL_PV_ESCAPE_RE           0x008000

#define PERL_PV_ESCAPE_DWIM         0x010000


/* used by pv_display in dump.c*/
#define PERL_PV_PRETTY_DUMP  PERL_PV_PRETTY_ELLIPSES|PERL_PV_PRETTY_QUOTE
#define PERL_PV_PRETTY_REGPROP PERL_PV_PRETTY_ELLIPSES|PERL_PV_PRETTY_LTGT|PERL_PV_ESCAPE_RE|PERL_PV_ESCAPE_NONASCII

#if DOUBLEKIND == DOUBLE_IS_VAX_F_FLOAT || \
    DOUBLEKIND == DOUBLE_IS_VAX_D_FLOAT || \
    DOUBLEKIND == DOUBLE_IS_VAX_G_FLOAT
#  define DOUBLE_IS_VAX_FLOAT
#else
#  define DOUBLE_IS_IEEE_FORMAT
#endif

#if DOUBLEKIND == DOUBLE_IS_IEEE_754_32_BIT_LITTLE_ENDIAN || \
    DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_LITTLE_ENDIAN || \
    DOUBLEKIND == DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#  define DOUBLE_LITTLE_ENDIAN
#endif

#if DOUBLEKIND == DOUBLE_IS_IEEE_754_32_BIT_BIG_ENDIAN || \
    DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_BIG_ENDIAN || \
    DOUBLEKIND == DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#  define DOUBLE_BIG_ENDIAN
#endif

#if DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_MIXED_ENDIAN_LE_BE || \
    DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_MIXED_ENDIAN_BE_LE
#  define DOUBLE_MIX_ENDIAN
#endif

/* The VAX fp formats are neither consistently little-endian nor
 * big-endian, and neither are they really IEEE-mixed endian like
 * the mixed-endian ARM IEEE formats (with swapped bytes).
 * Ultimately, the VAX format came from the PDP-11.
 *
 * The ordering of the parts in VAX floats is quite vexing.
 * In the below the fraction_n are the mantissa bits.
 *
 * The fraction_1 is the most significant (numbering as by DEC/Digital),
 * while the rightmost bit in each fraction is the least significant:
 * in other words, big-endian bit order within the fractions.
 *
 * The fraction segments themselves would be big-endianly, except that
 * within 32 bit segments the less significant half comes first, the more
 * significant after, except that in the format H (used for long doubles)
 * the first fraction segment is alone, because the exponent is wider.
 * This means for example that both the most and the least significant
 * bits can be in the middle of the floats, not at either end.
 *
 * References:
 * http://nssdc.gsfc.nasa.gov/nssdc/formats/VAXFloatingPoint.htm
 * http://www.quadibloc.com/comp/cp0201.htm
 * http://h71000.www7.hp.com/doc/82final/6443/6443pro_028.html
 * (somebody at HP should be fired for the URLs)
 *
 * F   fraction_2:16 sign:1 exp:8  fraction_1:7
 *     (exponent bias 128, hidden first one-bit)
 *
 * D   fraction_2:16 sign:1 exp:8  fraction_1:7
 *     fraction_4:16               fraction_3:16
 *     (exponent bias 128, hidden first one-bit)
 *
 * G   fraction_2:16 sign:1 exp:11 fraction_1:4
 *     fraction_4:16               fraction_3:16
 *     (exponent bias 1024, hidden first one-bit)
 *
 * H   fraction_1:16 sign:1 exp:15
 *     fraction_3:16               fraction_2:16
 *     fraction_5:16               fraction_4:16
 *     fraction_7:16               fraction_6:16
 *     (exponent bias 16384, hidden first one-bit)
 *     (available only on VAX, and only on Fortran?)
 *
 * The formats S, T and X are available on the Alpha (and Itanium,
 * also known as I64/IA64) and are equivalent with the IEEE-754 formats
 * binary32, binary64, and binary128 (commonly: float, double, long double).
 *
 * S   sign:1 exp:8 mantissa:23
 *     (exponent bias 127, hidden first one-bit)
 *
 * T   sign:1 exp:11 mantissa:52
 *     (exponent bias 1022, hidden first one-bit)
 *
 * X   sign:1 exp:15 mantissa:112
 *     (exponent bias 16382, hidden first one-bit)
 *
 */

#ifdef DOUBLE_IS_VAX_FLOAT
#  define DOUBLE_VAX_ENDIAN
#endif

#ifdef DOUBLE_IS_IEEE_FORMAT
/* All the basic IEEE formats have the implicit bit,
 * except for the x86 80-bit extended formats, which will undef this.
 * Also note that the IEEE 754 subnormals (formerly known as denormals)
 * do not have the implicit bit of one. */
#  define NV_IMPLICIT_BIT
#endif

#if defined(LONG_DOUBLEKIND) && LONG_DOUBLEKIND != LONG_DOUBLE_IS_DOUBLE

#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_LITTLE_ENDIAN || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_LE
#    define LONGDOUBLE_LITTLE_ENDIAN
#  endif

#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_BIG_ENDIAN || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_BE
#    define LONGDOUBLE_BIG_ENDIAN
#  endif

#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_BE || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_LE
#    define LONGDOUBLE_MIX_ENDIAN
#  endif

#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_LITTLE_ENDIAN || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_BIG_ENDIAN
#    define LONGDOUBLE_X86_80_BIT
#    ifdef USE_LONG_DOUBLE
#      undef NV_IMPLICIT_BIT
#      define NV_X86_80_BIT
#    endif
#  endif

#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_LE || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_BE || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_BE || \
      LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_LE
#    define LONGDOUBLE_DOUBLEDOUBLE
#  endif

#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_VAX_H_FLOAT
#    define LONGDOUBLE_VAX_ENDIAN
#  endif

#endif /* LONG_DOUBLEKIND */

#ifdef USE_QUADMATH /* assume quadmath endianness == native double endianness */
#  if defined(DOUBLE_LITTLE_ENDIAN)
#    define NV_LITTLE_ENDIAN
#  elif defined(DOUBLE_BIG_ENDIAN)
#    define NV_BIG_ENDIAN
#  elif defined(DOUBLE_MIX_ENDIAN) /* stretch */
#    define NV_MIX_ENDIAN
#  endif
#elif NVSIZE == DOUBLESIZE
#  ifdef DOUBLE_LITTLE_ENDIAN
#    define NV_LITTLE_ENDIAN
#  endif
#  ifdef DOUBLE_BIG_ENDIAN
#    define NV_BIG_ENDIAN
#  endif
#  ifdef DOUBLE_MIX_ENDIAN
#    define NV_MIX_ENDIAN
#  endif
#  ifdef DOUBLE_VAX_ENDIAN
#    define NV_VAX_ENDIAN
#  endif
#elif NVSIZE == LONG_DOUBLESIZE
#  ifdef LONGDOUBLE_LITTLE_ENDIAN
#    define NV_LITTLE_ENDIAN
#  endif
#  ifdef LONGDOUBLE_BIG_ENDIAN
#    define NV_BIG_ENDIAN
#  endif
#  ifdef LONGDOUBLE_MIX_ENDIAN
#    define NV_MIX_ENDIAN
#  endif
#  ifdef LONGDOUBLE_VAX_ENDIAN
#    define NV_VAX_ENDIAN
#  endif
#endif

/* We have somehow managed not to define the denormal/subnormal
 * detection.
 *
 * This may happen if the compiler doesn't expose the C99 math like
 * the fpclassify() without some special switches.  Perl tries to
 * stay C89, so for example -std=c99 is not an option.
 *
 * The Perl_isinf() and Perl_isnan() should have been defined even if
 * the C99 isinf() and isnan() are unavailable, and the NV_MIN becomes
 * from the C89 DBL_MIN or moral equivalent. */
#if !defined(Perl_fp_class_denorm) && defined(Perl_isinf) && defined(Perl_isnan) && defined(NV_MIN)
#  define Perl_fp_class_denorm(x) ((x) != 0.0 && !Perl_isinf(x) && !Perl_isnan(x) && PERL_ABS(x) < NV_MIN)
#endif

/* This is not a great fallback: subnormals tests will fail,
 * but at least Perl will link and 99.999% of tests will work. */
#if !defined(Perl_fp_class_denorm)
#  define Perl_fp_class_denorm(x) FALSE
#endif

#ifdef DOUBLE_IS_IEEE_FORMAT
#  define DOUBLE_HAS_INF
#  define DOUBLE_HAS_NAN
#endif

#ifdef DOUBLE_HAS_NAN

START_EXTERN_C

#ifdef DOINIT

/* PL_inf and PL_nan initialization.
 *
 * For inf and nan initialization the ultimate fallback is dividing
 * one or zero by zero: however, some compilers will warn or even fail
 * on divide-by-zero, but hopefully something earlier will work.
 *
 * If you are thinking of using HUGE_VAL for infinity, or using
 * <math.h> functions to generate NV_INF (e.g. exp(1e9), log(-1.0)),
 * stop.  Neither will work portably: HUGE_VAL can be just DBL_MAX,
 * and the math functions might be just generating DBL_MAX, or even zero.
 *
 * Also, do NOT try doing NV_NAN based on NV_INF and trying (NV_INF-NV_INF).
 * Though logically correct, some compilers (like Visual C 2003)
 * falsely misoptimize that to zero (x-x is always zero, right?)
 *
 * Finally, note that not all floating point formats define Inf (or NaN).
 * For the infinity a large number may be used instead.  Operations that
 * under the IEEE floating point would return Inf or NaN may return
 * either large numbers (positive or negative), or they may cause
 * a floating point exception or some other fault.
 */

/* The quadmath literals are anon structs which -Wc++-compat doesn't like. */
#  ifndef USE_CPLUSPLUS
GCC_DIAG_IGNORE_DECL(-Wc++-compat);
#  endif

#  ifdef USE_QUADMATH
/* Cannot use HUGE_VALQ for PL_inf because not a compile-time
 * constant. */
INFNAN_NV_U8_DECL PL_inf = { 1.0Q/0.0Q };
#  elif NVSIZE == LONG_DOUBLESIZE && defined(LONGDBLINFBYTES)
INFNAN_U8_NV_DECL PL_inf = { { LONGDBLINFBYTES } };
#  elif NVSIZE == DOUBLESIZE && defined(DOUBLEINFBYTES)
INFNAN_U8_NV_DECL PL_inf = { { DOUBLEINFBYTES } };
#  else
#    if NVSIZE == LONG_DOUBLESIZE && defined(USE_LONG_DOUBLE)
#      if defined(LDBL_INFINITY)
INFNAN_NV_U8_DECL PL_inf = { LDBL_INFINITY };
#      elif defined(LDBL_INF)
INFNAN_NV_U8_DECL PL_inf = { LDBL_INF };
#      elif defined(INFINITY)
INFNAN_NV_U8_DECL PL_inf = { (NV)INFINITY };
#      elif defined(INF)
INFNAN_NV_U8_DECL PL_inf = { (NV)INF };
#      else
INFNAN_NV_U8_DECL PL_inf = { 1.0L/0.0L }; /* keep last */
#      endif
#    else
#      if defined(DBL_INFINITY)
INFNAN_NV_U8_DECL PL_inf = { DBL_INFINITY };
#      elif defined(DBL_INF)
INFNAN_NV_U8_DECL PL_inf = { DBL_INF };
#      elif defined(INFINITY) /* C99 */
INFNAN_NV_U8_DECL PL_inf = { (NV)INFINITY };
#      elif defined(INF)
INFNAN_NV_U8_DECL PL_inf = { (NV)INF };
#      else
INFNAN_NV_U8_DECL PL_inf = { 1.0/0.0 }; /* keep last */
#      endif
#    endif
#  endif

#  ifdef USE_QUADMATH
/* Cannot use nanq("0") for PL_nan because not a compile-time
 * constant. */
INFNAN_NV_U8_DECL PL_nan = { 0.0Q/0.0Q };
#  elif NVSIZE == LONG_DOUBLESIZE && defined(LONGDBLNANBYTES)
INFNAN_U8_NV_DECL PL_nan = { { LONGDBLNANBYTES } };
#  elif NVSIZE == DOUBLESIZE && defined(DOUBLENANBYTES)
INFNAN_U8_NV_DECL PL_nan = { { DOUBLENANBYTES } };
#  else
#    if NVSIZE == LONG_DOUBLESIZE && defined(USE_LONG_DOUBLE)
#      if defined(LDBL_NAN)
INFNAN_NV_U8_DECL PL_nan = { LDBL_NAN };
#      elif defined(LDBL_QNAN)
INFNAN_NV_U8_DECL PL_nan = { LDBL_QNAN };
#      elif defined(NAN)
INFNAN_NV_U8_DECL PL_nan = { (NV)NAN };
#      else
INFNAN_NV_U8_DECL PL_nan = { 0.0L/0.0L }; /* keep last */
#      endif
#    else
#      if defined(DBL_NAN)
INFNAN_NV_U8_DECL PL_nan = { DBL_NAN };
#      elif defined(DBL_QNAN)
INFNAN_NV_U8_DECL PL_nan = { DBL_QNAN };
#      elif defined(NAN) /* C99 */
INFNAN_NV_U8_DECL PL_nan = { (NV)NAN };
#      else
INFNAN_NV_U8_DECL PL_nan = { 0.0/0.0 }; /* keep last */
#      endif
#    endif
#  endif

#  ifndef USE_CPLUSPLUS
GCC_DIAG_RESTORE_DECL;
#  endif

#else

INFNAN_NV_U8_DECL PL_inf;
INFNAN_NV_U8_DECL PL_nan;

#endif

END_EXTERN_C

/* If you have not defined NV_INF/NV_NAN (like for example win32/win32.h),
 * we will define NV_INF/NV_NAN as the nv part of the global const
 * PL_inf/PL_nan.  Note, however, that the preexisting NV_INF/NV_NAN
 * might not be a compile-time constant, in which case it cannot be
 * used to initialize PL_inf/PL_nan above. */
#ifndef NV_INF
#  define NV_INF PL_inf.nv
#endif
#ifndef NV_NAN
#  define NV_NAN PL_nan.nv
#endif

/* NaNs (not-a-numbers) can carry payload bits, in addition to
 * "nan-ness".  Part of the payload is the quiet/signaling bit.
 * To back up a bit (harhar):
 *
 * For IEEE 754 64-bit formats [1]:
 *
 * s 000 (mantissa all-zero)  zero
 * s 000 (mantissa non-zero)  subnormals (denormals)
 * s 001 ... 7fe              normals
 * s 7ff q                    nan
 *
 * For IEEE 754 128-bit formats:
 *
 * s 0000 (mantissa all-zero)  zero
 * s 0000 (mantissa non-zero)  subnormals (denormals)
 * s 0001 ... 7ffe             normals
 * s 7fff q                    nan
 *
 * [1] this looks like big-endian, but applies equally to little-endian.
 *
 * s = Sign bit.  Yes, zeros and nans can have negative sign,
 *     the interpretation is application-specific.
 *
 * q = Quietness bit, the interpretation is platform-specific.
 *     Most platforms have the most significant bit being one
 *     meaning quiet, but some (older mips, hppa) have the msb
 *     being one meaning signaling.  Note that the above means
 *     that on most platforms there cannot be signaling nan with
 *     zero payload because that is identical with infinity;
 *     while conversely on older mips/hppa there cannot be a quiet nan
 *     because that is identical with infinity.
 *
 *     Moreover, whether there is any behavioral difference
 *     between quiet and signaling NaNs, depends on the platform.
 *
 * x86 80-bit extended precision is different, the mantissa bits:
 *
 * 63 62 61   30387+    pre-387    visual c
 * --------   ----      --------   --------
 *  0  0  0   invalid   infinity
 *  0  0  1   invalid   snan
 *  0  1  0   invalid   snan
 *  0  1  1   invalid   snan
 *  1  0  0   infinity  snan        1.#INF
 *  1  0  1   snan                  1.#SNAN
 *  1  1  0   qnan                 -1.#IND  (x86 chooses this to negative)
 *  1  1  1   qnan                  1.#QNAN
 *
 * This means that in this format there are 61 bits available
 * for the nan payload.
 *
 * Note that the 32-bit x86 ABI cannot do signaling nans: the x87
 * simply cannot preserve the bit.  You can either use the 80-bit
 * extended precision (long double, -Duselongdouble), or use x86-64.
 *
 * In all platforms, the payload bytes (and bits, some of them are
 * often in a partial byte) themselves can be either all zero (x86),
 * all one (sparc or mips), or a mixture: in IEEE 754 128-bit double
 * or in a double-double, the first half of the payload can follow the
 * native double, while in the second half the payload can be all
 * zeros.  (Therefore the mask for payload bits is not necessarily
 * identical to bit complement of the NaN.)  Another way of putting
 * this: the payload for the default NaN might not be zero.
 *
 * For the x86 80-bit long doubles, the trailing bytes (the 80 bits
 * being 'packaged' in either 12 or 16 bytes) can be whatever random
 * garbage.
 *
 * Furthermore, the semantics of the sign bit on NaNs are platform-specific.
 * On normal floats, the sign bit being on means negative.  But this may,
 * or may not, be reverted on NaNs: in other words, the default NaN might
 * have the sign bit on, and therefore look like negative if you look
 * at it at the bit level.
 *
 * NaN payloads are not propagated even on copies, or in arithmetics.
 * They *might* be, according to some rules, on your particular
 * cpu/os/compiler/libraries, but no guarantees.
 *
 * To summarize, on most platforms, and for 64-bit doubles
 * (using big-endian ordering here):
 *
 * [7FF8000000000000..7FFFFFFFFFFFFFFF] quiet
 * [FFF8000000000000..FFFFFFFFFFFFFFFF] quiet
 * [7FF0000000000001..7FF7FFFFFFFFFFFF] signaling
 * [FFF0000000000001..FFF7FFFFFFFFFFFF] signaling
 *
 * The C99 nan() is supposed to generate *quiet* NaNs.
 *
 * Note the asymmetry:
 * The 7FF0000000000000 is positive infinity,
 * the FFF0000000000000 is negative infinity.
 */

/* NVMANTBITS is the number of _real_ mantissa bits in an NV.
 * For the standard IEEE 754 fp this number is usually one less that
 * *DBL_MANT_DIG because of the implicit (aka hidden) bit, which isn't
 * real.  For the 80-bit extended precision formats (x86*), the number
 * of mantissa bits... depends. For normal floats, it's 64.  But for
 * the inf/nan, it's different (zero for inf, 61 for nan).
 * NVMANTBITS works for normal floats. */

/* We do not want to include the quiet/signaling bit. */
#define NV_NAN_BITS (NVMANTBITS - 1)

#if defined(USE_LONG_DOUBLE) && NVSIZE > DOUBLESIZE
#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 13
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 2
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_LITTLE_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 7
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_BIG_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 2
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_LE
#    define NV_NAN_QS_BYTE_OFFSET 13
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_BE
#    define NV_NAN_QS_BYTE_OFFSET 1
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_BE
#    define NV_NAN_QS_BYTE_OFFSET 9
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_LE
#    define NV_NAN_QS_BYTE_OFFSET 6
#  else
#    error "Unexpected long double format"
#  endif
#else
#  ifdef USE_QUADMATH
#    ifdef NV_LITTLE_ENDIAN
#      define NV_NAN_QS_BYTE_OFFSET 13
#    elif defined(NV_BIG_ENDIAN)
#      define NV_NAN_QS_BYTE_OFFSET 2
#    else
#      error "Unexpected quadmath format"
#    endif
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_32_BIT_LITTLE_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 2
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_32_BIT_BIG_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 1
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_LITTLE_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 6
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_BIG_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 1
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 13
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#    define NV_NAN_QS_BYTE_OFFSET 2
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_MIXED_ENDIAN_LE_BE
#    define NV_NAN_QS_BYTE_OFFSET 2 /* bytes 4 5 6 7 0 1 2 3 (MSB 7) */
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_MIXED_ENDIAN_BE_LE
#    define NV_NAN_QS_BYTE_OFFSET 5 /* bytes 3 2 1 0 7 6 5 4 (MSB 7) */
#  else
/* For example the VAX formats should never
 * get here because they do not have NaN. */
#    error "Unexpected double format"
#  endif
#endif
/* NV_NAN_QS_BYTE is the byte to test for the quiet/signaling */
#define NV_NAN_QS_BYTE(nvp) (((U8*)(nvp))[NV_NAN_QS_BYTE_OFFSET])
/* NV_NAN_QS_BIT is the bit to test in the NV_NAN_QS_BYTE_OFFSET
 * for the quiet/signaling */
#if defined(USE_LONG_DOUBLE) && \
  (LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_LITTLE_ENDIAN || \
   LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_BIG_ENDIAN)
#  define NV_NAN_QS_BIT_SHIFT 6 /* 0x40 */
#elif defined(USE_LONG_DOUBLE) && \
  (LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_LE || \
   LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_BE || \
   LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_BE || \
   LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_LE)
#  define NV_NAN_QS_BIT_SHIFT 3 /* 0x08, but not via NV_NAN_BITS */
#else
#  define NV_NAN_QS_BIT_SHIFT ((NV_NAN_BITS) % 8) /* usually 3, or 0x08 */
#endif
#define NV_NAN_QS_BIT (1 << (NV_NAN_QS_BIT_SHIFT))
/* NV_NAN_QS_BIT_OFFSET is the bit offset from the beginning of a NV
 * (bytes ordered big-endianly) for the quiet/signaling bit
 * for the quiet/signaling */
#define NV_NAN_QS_BIT_OFFSET \
    (8 * (NV_NAN_QS_BYTE_OFFSET) + (NV_NAN_QS_BIT_SHIFT))
/* NV_NAN_QS_QUIET (always defined) is true if the NV_NAN_QS_QS_BIT being
 * on indicates quiet NaN.  NV_NAN_QS_SIGNALING (also always defined)
 * is true if the NV_NAN_QS_BIT being on indicates signaling NaN. */
#define NV_NAN_QS_QUIET \
    ((NV_NAN_QS_BYTE(PL_nan.u8) & NV_NAN_QS_BIT) == NV_NAN_QS_BIT)
#define NV_NAN_QS_SIGNALING (!(NV_NAN_QS_QUIET))
#define NV_NAN_QS_TEST(nvp) (NV_NAN_QS_BYTE(nvp) & NV_NAN_QS_BIT)
/* NV_NAN_IS_QUIET() returns true if the NV behind nvp is a NaN,
 * whether it is a quiet NaN, NV_NAN_IS_SIGNALING() if a signaling NaN.
 * Note however that these do not check whether the nvp is a NaN. */
#define NV_NAN_IS_QUIET(nvp) \
    (NV_NAN_QS_TEST(nvp) == (NV_NAN_QS_QUIET ? NV_NAN_QS_BIT : 0))
#define NV_NAN_IS_SIGNALING(nvp) \
    (NV_NAN_QS_TEST(nvp) == (NV_NAN_QS_QUIET ? 0 : NV_NAN_QS_BIT))
#define NV_NAN_SET_QUIET(nvp) \
    (NV_NAN_QS_QUIET ? \
     (NV_NAN_QS_BYTE(nvp) |= NV_NAN_QS_BIT) : \
     (NV_NAN_QS_BYTE(nvp) &= ~NV_NAN_QS_BIT))
#define NV_NAN_SET_SIGNALING(nvp) \
    (NV_NAN_QS_QUIET ? \
     (NV_NAN_QS_BYTE(nvp) &= ~NV_NAN_QS_BIT) : \
     (NV_NAN_QS_BYTE(nvp) |= NV_NAN_QS_BIT))
#define NV_NAN_QS_XOR(nvp) (NV_NAN_QS_BYTE(nvp) ^= NV_NAN_QS_BIT)

/* NV_NAN_PAYLOAD_MASK: masking the nan payload bits.
 *
 * NV_NAN_PAYLOAD_PERM: permuting the nan payload bytes.
 * 0xFF means "don't go here".*/

/* Shorthands to avoid typoses. */
#define NV_NAN_PAYLOAD_MASK_SKIP_EIGHT \
  0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
#define NV_NAN_PAYLOAD_PERM_SKIP_EIGHT \
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
#define NV_NAN_PAYLOAD_PERM_0_TO_7 \
  0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7
#define NV_NAN_PAYLOAD_PERM_7_TO_0 \
  0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0
#define NV_NAN_PAYLOAD_MASK_IEEE_754_128_LE \
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, \
  0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0x00, 0x00
#define NV_NAN_PAYLOAD_PERM_IEEE_754_128_LE \
  NV_NAN_PAYLOAD_PERM_0_TO_7, \
  0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xFF, 0xFF
#define NV_NAN_PAYLOAD_MASK_IEEE_754_128_BE \
  0x00, 0x00, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, \
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
#define NV_NAN_PAYLOAD_PERM_IEEE_754_128_BE \
  0xFF, 0xFF, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, \
  NV_NAN_PAYLOAD_PERM_7_TO_0
#define NV_NAN_PAYLOAD_MASK_IEEE_754_64_LE \
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x07, 0x00
#define NV_NAN_PAYLOAD_PERM_IEEE_754_64_LE \
  0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0xFF
#define NV_NAN_PAYLOAD_MASK_IEEE_754_64_BE \
  0x00, 0x07, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
#define NV_NAN_PAYLOAD_PERM_IEEE_754_64_BE \
  0xFF, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0

#if defined(USE_LONG_DOUBLE) && NVSIZE > DOUBLESIZE
#  if LONG_DOUBLEKIND == LONG_DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#    define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_128_LE
#    define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_128_LE
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#    define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_128_BE
#    define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_128_BE
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_LITTLE_ENDIAN
#    if LONG_DOUBLESIZE == 10
#      define NV_NAN_PAYLOAD_MASK \
         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1f, \
         0x00, 0x00
#      define NV_NAN_PAYLOAD_PERM \
         NV_NAN_PAYLOAD_PERM_0_TO_7, 0xFF, 0xFF
#    elif LONG_DOUBLESIZE == 12
#      define NV_NAN_PAYLOAD_MASK \
         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1f, \
         0x00, 0x00, 0x00, 0x00
#      define NV_NAN_PAYLOAD_PERM \
         NV_NAN_PAYLOAD_PERM_0_TO_7, 0xFF, 0xFF, 0xFF, 0xFF
#    elif LONG_DOUBLESIZE == 16
#      define NV_NAN_PAYLOAD_MASK \
         0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1f, \
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
#      define NV_NAN_PAYLOAD_PERM \
         NV_NAN_PAYLOAD_PERM_0_TO_7, \
         0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
#    else
#      error "Unexpected x86 80-bit little-endian long double format"
#    endif
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_X86_80_BIT_BIG_ENDIAN
#    if LONG_DOUBLESIZE == 10
#      define NV_NAN_PAYLOAD_MASK \
         0x00, 0x00, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, \
         0xff, 0xff
#      define NV_NAN_PAYLOAD_PERM \
         NV_NAN_PAYLOAD_PERM_7_TO_0, 0xFF, 0xFF
#    elif LONG_DOUBLESIZE == 12
#      define NV_NAN_PAYLOAD_MASK \
         0x00, 0x00, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, \
         0xff, 0xff, 0x00, 0x00
#      define NV_NAN_PAYLOAD_PERM \
         NV_NAN_PAYLOAD_PERM_7_TO_0, 0xFF, 0xFF, 0xFF, 0xFF
#    elif LONG_DOUBLESIZE == 16
#      define NV_NAN_PAYLOAD_MASK \
         0x00, 0x00, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, \
         0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
#      define NV_NAN_PAYLOAD_PERM \
         NV_NAN_PAYLOAD_PERM_7_TO_0, \
         0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
#    else
#      error "Unexpected x86 80-bit big-endian long double format"
#    endif
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_LE
/* For double-double we assume only the first double (in LE or BE terms)
 * is used for NaN. */
#    define NV_NAN_PAYLOAD_MASK \
       NV_NAN_PAYLOAD_MASK_SKIP_EIGHT, NV_NAN_PAYLOAD_MASK_IEEE_754_64_LE
#    define NV_NAN_PAYLOAD_PERM \
       NV_NAN_PAYLOAD_PERM_SKIP_EIGHT, NV_NAN_PAYLOAD_PERM_IEEE_754_64_LE
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_BE
#    define NV_NAN_PAYLOAD_MASK \
       NV_NAN_PAYLOAD_MASK_IEEE_754_64_BE
#    define NV_NAN_PAYLOAD_PERM \
       NV_NAN_PAYLOAD_PERM_IEEE_754_64_BE
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_LE_BE
#    define NV_NAN_PAYLOAD_MASK \
       NV_NAN_PAYLOAD_MASK_IEEE_754_64_LE
#    define NV_NAN_PAYLOAD_PERM \
       NV_NAN_PAYLOAD_PERM_IEEE_754_64_LE
#  elif LONG_DOUBLEKIND == LONG_DOUBLE_IS_DOUBLEDOUBLE_128_BIT_BE_LE
#    define NV_NAN_PAYLOAD_MASK \
       NV_NAN_PAYLOAD_MASK_SKIP_EIGHT, NV_NAN_PAYLOAD_MASK_IEEE_754_64_BE
#    define NV_NAN_PAYLOAD_PERM \
       NV_NAN_PAYLOAD_PERM_SKIP_EIGHT, NV_NAN_PAYLOAD_PERM_IEEE_754_64_BE
#  else
#    error "Unexpected long double format"
#  endif
#else
#  ifdef USE_QUADMATH /* quadmath is not long double */
#    ifdef NV_LITTLE_ENDIAN
#      define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_128_LE
#      define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_128_LE
#    elif defined(NV_BIG_ENDIAN)
#      define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_128_BE
#      define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_128_BE
#    else
#      error "Unexpected quadmath format"
#    endif
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_32_BIT_LITTLE_ENDIAN
#    define NV_NAN_PAYLOAD_MASK 0xff, 0xff, 0x07, 0x00
#    define NV_NAN_PAYLOAD_PERM 0x0, 0x1, 0x2, 0xFF
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_32_BIT_BIG_ENDIAN
#    define NV_NAN_PAYLOAD_MASK 0x00, 0x07, 0xff, 0xff
#    define NV_NAN_PAYLOAD_PERM 0xFF, 0x2, 0x1, 0x0
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_LITTLE_ENDIAN
#    define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_64_LE
#    define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_64_LE
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_BIG_ENDIAN
#    define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_64_BE
#    define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_64_BE
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_128_BIT_LITTLE_ENDIAN
#    define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_128_LE
#    define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_128_LE
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_128_BIT_BIG_ENDIAN
#    define NV_NAN_PAYLOAD_MASK NV_NAN_PAYLOAD_MASK_IEEE_754_128_BE
#    define NV_NAN_PAYLOAD_PERM NV_NAN_PAYLOAD_PERM_IEEE_754_128_BE
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_MIXED_ENDIAN_LE_BE
#    define NV_NAN_PAYLOAD_MASK 0xff, 0xff, 0x07, 0x00, 0xff, 0xff, 0xff, 0xff
#    define NV_NAN_PAYLOAD_PERM 0x4, 0x5, 0x6, 0xFF, 0x0, 0x1, 0x2, 0x3
#  elif DOUBLEKIND == DOUBLE_IS_IEEE_754_64_BIT_MIXED_ENDIAN_BE_LE
#    define NV_NAN_PAYLOAD_MASK 0xff, 0xff, 0xff, 0xff, 0x00, 0x07, 0xff, 0xff
#    define NV_NAN_PAYLOAD_PERM 0x3, 0x2, 0x1, 0x0, 0xFF, 0x6, 0x5, 0x4
#  else
#    error "Unexpected double format"
#  endif
#endif

#endif /* DOUBLE_HAS_NAN */


/*

   (KEEP THIS LAST IN perl.h!)

   Mention

   NV_PRESERVES_UV

   HAS_MKSTEMP
   HAS_MKSTEMPS
   HAS_MKDTEMP

   HAS_GETCWD

   HAS_MMAP
   HAS_MPROTECT
   HAS_MSYNC
   HAS_MADVISE
   HAS_MUNMAP
   I_SYSMMAN
   Mmap_t

   NVef
   NVff
   NVgf

   HAS_UALARM
   HAS_USLEEP

   HAS_SETITIMER
   HAS_GETITIMER

   HAS_SENDMSG
   HAS_RECVMSG
   HAS_READV
   HAS_WRITEV
   I_SYSUIO
   HAS_STRUCT_MSGHDR
   HAS_STRUCT_CMSGHDR

   HAS_NL_LANGINFO

   HAS_DIRFD

   so that Configure picks them up.

   (KEEP THIS LAST IN perl.h!)

*/

#endif /* Include guard */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
