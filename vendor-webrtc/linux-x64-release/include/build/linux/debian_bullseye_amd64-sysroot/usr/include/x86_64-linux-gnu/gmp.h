/* Definitions for GNU multiple precision functions.   -*- mode: c -*-

Copyright 1991, 1993-1997, 1999-2016, 2020 Free Software Foundation, Inc.

This file is part of the GNU MP Library.

The GNU MP Library is free software; you can redistribute it and/or modify
it under the terms of either:

  * the GNU Lesser General Public License as published by the Free
    Software Foundation; either version 3 of the License, or (at your
    option) any later version.

or

  * the GNU General Public License as published by the Free Software
    Foundation; either version 2 of the License, or (at your option) any
    later version.

or both in parallel, as here.

The GNU MP Library is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
for more details.

You should have received copies of the GNU General Public License and the
GNU Lesser General Public License along with the GNU MP Library.  If not,
see https://www.gnu.org/licenses/.  */

#ifndef __GMP_H__

#if defined (__cplusplus)
#include <iosfwd>   /* for std::istream, std::ostream, std::string */
#include <cstdio>
#endif


/* Instantiated by configure. */
#if ! defined (__GMP_WITHIN_CONFIGURE)
#define __GMP_HAVE_HOST_CPU_FAMILY_power   0
#define __GMP_HAVE_HOST_CPU_FAMILY_powerpc 0
#define GMP_LIMB_BITS                      64
#define GMP_NAIL_BITS                      0
#endif
#define GMP_NUMB_BITS     (GMP_LIMB_BITS - GMP_NAIL_BITS)
#define GMP_NUMB_MASK     ((~ __GMP_CAST (mp_limb_t, 0)) >> GMP_NAIL_BITS)
#define GMP_NUMB_MAX      GMP_NUMB_MASK
#define GMP_NAIL_MASK     (~ GMP_NUMB_MASK)


#ifndef __GNU_MP__
#define __GNU_MP__ 6

#include <stddef.h>    /* for size_t */
#include <limits.h>

/* Instantiated by configure. */
#if ! defined (__GMP_WITHIN_CONFIGURE)
/* #undef _LONG_LONG_LIMB */
#define __GMP_LIBGMP_DLL  0
#endif


/* __GMP_DECLSPEC supports Windows DLL versions of libgmp, and is empty in
   all other circumstances.

   When compiling objects for libgmp, __GMP_DECLSPEC is an export directive,
   or when compiling for an application it's an import directive.  The two
   cases are differentiated by __GMP_WITHIN_GMP defined by the GMP Makefiles
   (and not defined from an application).

   __GMP_DECLSPEC_XX is similarly used for libgmpxx.  __GMP_WITHIN_GMPXX
   indicates when building libgmpxx, and in that case libgmpxx functions are
   exports, but libgmp functions which might get called are imports.

   Libtool DLL_EXPORT define is not used.

   There's no attempt to support GMP built both static and DLL.  Doing so
   would mean applications would have to tell us which of the two is going
   to be used when linking, and that seems very tedious and error prone if
   using GMP by hand, and equally tedious from a package since autoconf and
   automake don't give much help.

   __GMP_DECLSPEC is required on all documented global functions and
   variables, the various internals in gmp-impl.h etc can be left unadorned.
   But internals used by the test programs or speed measuring programs
   should have __GMP_DECLSPEC, and certainly constants or variables must
   have it or the wrong address will be resolved.

   In gcc __declspec can go at either the start or end of a prototype.

   In Microsoft C __declspec must go at the start, or after the type like
   void __declspec(...) *foo()".  There's no __dllexport or anything to
   guard against someone foolish #defining dllexport.  _export used to be
   available, but no longer.

   In Borland C _export still exists, but needs to go after the type, like
   "void _export foo();".  Would have to change the __GMP_DECLSPEC syntax to
   make use of that.  Probably more trouble than it's worth.  */

#if defined (__GNUC__)
#define __GMP_DECLSPEC_EXPORT  __declspec(__dllexport__)
#define __GMP_DECLSPEC_IMPORT  __declspec(__dllimport__)
#endif
#if defined (_MSC_VER) || defined (__BORLANDC__)
#define __GMP_DECLSPEC_EXPORT  __declspec(dllexport)
#define __GMP_DECLSPEC_IMPORT  __declspec(dllimport)
#endif
#ifdef __WATCOMC__
#define __GMP_DECLSPEC_EXPORT  __export
#define __GMP_DECLSPEC_IMPORT  __import
#endif
#ifdef __IBMC__
#define __GMP_DECLSPEC_EXPORT  _Export
#define __GMP_DECLSPEC_IMPORT  _Import
#endif

#if __GMP_LIBGMP_DLL
#ifdef __GMP_WITHIN_GMP
/* compiling to go into a DLL libgmp */
#define __GMP_DECLSPEC  __GMP_DECLSPEC_EXPORT
#else
/* compiling to go into an application which will link to a DLL libgmp */
#define __GMP_DECLSPEC  __GMP_DECLSPEC_IMPORT
#endif
#else
/* all other cases */
#define __GMP_DECLSPEC
#endif


#ifdef __GMP_SHORT_LIMB
typedef unsigned int		mp_limb_t;
typedef int			mp_limb_signed_t;
#else
#ifdef _LONG_LONG_LIMB
typedef unsigned long long int	mp_limb_t;
typedef long long int		mp_limb_signed_t;
#else
typedef unsigned long int	mp_limb_t;
typedef long int		mp_limb_signed_t;
#endif
#endif
typedef unsigned long int	mp_bitcnt_t;

/* For reference, note that the name __mpz_struct gets into C++ mangled
   function names, which means although the "__" suggests an internal, we
   must leave this name for binary compatibility.  */
typedef struct
{
  int _mp_alloc;		/* Number of *limbs* allocated and pointed
				   to by the _mp_d field.  */
  int _mp_size;			/* abs(_mp_size) is the number of limbs the
				   last field points to.  If _mp_size is
				   negative this is a negative number.  */
  mp_limb_t *_mp_d;		/* Pointer to the limbs.  */
} __mpz_struct;

#endif /* __GNU_MP__ */


typedef __mpz_struct MP_INT;    /* gmp 1 source compatibility */
typedef __mpz_struct mpz_t[1];

typedef mp_limb_t *		mp_ptr;
typedef const mp_limb_t *	mp_srcptr;
#if defined (_CRAY) && ! defined (_CRAYMPP)
/* plain `int' is much faster (48 bits) */
#define __GMP_MP_SIZE_T_INT     1
typedef int			mp_size_t;
typedef int			mp_exp_t;
#else
#define __GMP_MP_SIZE_T_INT     0
typedef long int		mp_size_t;
typedef long int		mp_exp_t;
#endif

typedef struct
{
  __mpz_struct _mp_num;
  __mpz_struct _mp_den;
} __mpq_struct;

typedef __mpq_struct MP_RAT;    /* gmp 1 source compatibility */
typedef __mpq_struct mpq_t[1];

typedef struct
{
  int _mp_prec;			/* Max precision, in number of `mp_limb_t's.
				   Set by mpf_init and modified by
				   mpf_set_prec.  The area pointed to by the
				   _mp_d field contains `prec' + 1 limbs.  */
  int _mp_size;			/* abs(_mp_size) is the number of limbs the
				   last field points to.  If _mp_size is
				   negative this is a negative number.  */
  mp_exp_t _mp_exp;		/* Exponent, in the base of `mp_limb_t'.  */
  mp_limb_t *_mp_d;		/* Pointer to the limbs.  */
} __mpf_struct;

/* typedef __mpf_struct MP_FLOAT; */
typedef __mpf_struct mpf_t[1];

/* Available random number generation algorithms.  */
typedef enum
{
  GMP_RAND_ALG_DEFAULT = 0,
  GMP_RAND_ALG_LC = GMP_RAND_ALG_DEFAULT /* Linear congruential.  */
} gmp_randalg_t;

/* Random state struct.  */
typedef struct
{
  mpz_t _mp_seed;	  /* _mp_d member points to state of the generator. */
  gmp_randalg_t _mp_alg;  /* Currently unused. */
  union {
    void *_mp_lc;         /* Pointer to function pointers structure.  */
  } _mp_algdata;
} __gmp_randstate_struct;
typedef __gmp_randstate_struct gmp_randstate_t[1];

/* Types for function declarations in gmp files.  */
/* ??? Should not pollute user name space with these ??? */
typedef const __mpz_struct *mpz_srcptr;
typedef __mpz_struct *mpz_ptr;
typedef const __mpf_struct *mpf_srcptr;
typedef __mpf_struct *mpf_ptr;
typedef const __mpq_struct *mpq_srcptr;
typedef __mpq_struct *mpq_ptr;


#if __GMP_LIBGMP_DLL
#ifdef __GMP_WITHIN_GMPXX
/* compiling to go into a DLL libgmpxx */
#define __GMP_DECLSPEC_XX  __GMP_DECLSPEC_EXPORT
#else
/* compiling to go into a application which will link to a DLL libgmpxx */
#define __GMP_DECLSPEC_XX  __GMP_DECLSPEC_IMPORT
#endif
#else
/* all other cases */
#define __GMP_DECLSPEC_XX
#endif


#ifndef __MPN
#define __MPN(x) __gmpn_##x
#endif

/* For reference, "defined(EOF)" cannot be used here.  In g++ 2.95.4,
   <iostream> defines EOF but not FILE.  */
#if defined (FILE)                                              \
  || defined (H_STDIO)                                          \
  || defined (_H_STDIO)               /* AIX */                 \
  || defined (_STDIO_H)               /* glibc, Sun, SCO */     \
  || defined (_STDIO_H_)              /* BSD, OSF */            \
  || defined (__STDIO_H)              /* Borland */             \
  || defined (__STDIO_H__)            /* IRIX */                \
  || defined (_STDIO_INCLUDED)        /* HPUX */                \
  || defined (__dj_include_stdio_h_)  /* DJGPP */               \
  || defined (_FILE_DEFINED)          /* Microsoft */           \
  || defined (__STDIO__)              /* Apple MPW MrC */       \
  || defined (_MSL_STDIO_H)           /* Metrowerks */          \
  || defined (_STDIO_H_INCLUDED)      /* QNX4 */		\
  || defined (_ISO_STDIO_ISO_H)       /* Sun C++ */		\
  || defined (__STDIO_LOADED)         /* VMS */			\
  || defined (__DEFINED_FILE)         /* musl */
#define _GMP_H_HAVE_FILE 1
#endif

/* In ISO C, if a prototype involving "struct obstack *" is given without
   that structure defined, then the struct is scoped down to just the
   prototype, causing a conflict if it's subsequently defined for real.  So
   only give prototypes if we've got obstack.h.  */
#if defined (_OBSTACK_H)   /* glibc <obstack.h> */
#define _GMP_H_HAVE_OBSTACK 1
#endif

/* The prototypes for gmp_vprintf etc are provided only if va_list is defined,
   via an application having included <stdarg.h>.  Usually va_list is a typedef
   so can't be tested directly, but C99 specifies that va_start is a macro.

   <stdio.h> will define some sort of va_list for vprintf and vfprintf, but
   let's not bother trying to use that since it's not standard and since
   application uses for gmp_vprintf etc will almost certainly require the
   whole <stdarg.h> anyway.  */

#ifdef va_start
#define _GMP_H_HAVE_VA_LIST 1
#endif

/* Test for gcc >= maj.min, as per __GNUC_PREREQ in glibc */
#if defined (__GNUC__) && defined (__GNUC_MINOR__)
#define __GMP_GNUC_PREREQ(maj, min) \
  ((__GNUC__ << 16) + __GNUC_MINOR__ >= ((maj) << 16) + (min))
#else
#define __GMP_GNUC_PREREQ(maj, min)  0
#endif

/* "pure" is in gcc 2.96 and up, see "(gcc)Function Attributes".  Basically
   it means a function does nothing but examine its arguments and memory
   (global or via arguments) to generate a return value, but changes nothing
   and has no side-effects.  __GMP_NO_ATTRIBUTE_CONST_PURE lets
   tune/common.c etc turn this off when trying to write timing loops.  */
#if __GMP_GNUC_PREREQ (2,96) && ! defined (__GMP_NO_ATTRIBUTE_CONST_PURE)
#define __GMP_ATTRIBUTE_PURE   __attribute__ ((__pure__))
#else
#define __GMP_ATTRIBUTE_PURE
#endif


/* __GMP_CAST allows us to use static_cast in C++, so our macros are clean
   to "g++ -Wold-style-cast".

   Casts in "extern inline" code within an extern "C" block don't induce
   these warnings, so __GMP_CAST only needs to be used on documented
   macros.  */

#ifdef __cplusplus
#define __GMP_CAST(type, expr)  (static_cast<type> (expr))
#else
#define __GMP_CAST(type, expr)  ((type) (expr))
#endif


/* An empty "throw ()" means the function doesn't throw any C++ exceptions,
   this can save some stack frame info in applications.

   Currently it's given only on functions which never divide-by-zero etc,
   don't allocate memory, and are expected to never need to allocate memory.
   This leaves open the possibility of a C++ throw from a future GMP
   exceptions scheme.

   mpz_set_ui etc are omitted to leave open the lazy allocation scheme
   described in doc/tasks.html.  mpz_get_d etc are omitted to leave open
   exceptions for float overflows.

   Note that __GMP_NOTHROW must be given on any inlines the same as on their
   prototypes (for g++ at least, where they're used together).  Note also
   that g++ 3.0 demands that __GMP_NOTHROW is before other attributes like
   __GMP_ATTRIBUTE_PURE.  */

#if defined (__cplusplus)
#if __cplusplus >= 201103L
#define __GMP_NOTHROW  noexcept
#else
#define __GMP_NOTHROW  throw ()
#endif
#else
#define __GMP_NOTHROW
#endif


/* PORTME: What other compilers have a useful "extern inline"?  "static
   inline" would be an acceptable substitute if the compiler (or linker)
   discards unused statics.  */

 /* gcc has __inline__ in all modes, including strict ansi.  Give a prototype
    for an inline too, so as to correctly specify "dllimport" on windows, in
    case the function is called rather than inlined.
    GCC 4.3 and above with -std=c99 or -std=gnu99 implements ISO C99
    inline semantics, unless -fgnu89-inline is used.  */
#ifdef __GNUC__
#if (defined __GNUC_STDC_INLINE__) || (__GNUC__ == 4 && __GNUC_MINOR__ == 2) \
  || (defined __GNUC_GNU_INLINE__ && defined __cplusplus)
#define __GMP_EXTERN_INLINE extern __inline__ __attribute__ ((__gnu_inline__))
#else
#define __GMP_EXTERN_INLINE      extern __inline__
#endif
#define __GMP_INLINE_PROTOTYPES  1
#endif

/* DEC C (eg. version 5.9) supports "static __inline foo()", even in -std1
   strict ANSI mode.  Inlining is done even when not optimizing (ie. -O0
   mode, which is the default), but an unnecessary local copy of foo is
   emitted unless -O is used.  "extern __inline" is accepted, but the
   "extern" appears to be ignored, ie. it becomes a plain global function
   but which is inlined within its file.  Don't know if all old versions of
   DEC C supported __inline, but as a start let's do the right thing for
   current versions.  */
#ifdef __DECC
#define __GMP_EXTERN_INLINE  static __inline
#endif

/* SCO OpenUNIX 8 cc supports "static inline foo()" but not in -Xc strict
   ANSI mode (__STDC__ is 1 in that mode).  Inlining only actually takes
   place under -O.  Without -O "foo" seems to be emitted whether it's used
   or not, which is wasteful.  "extern inline foo()" isn't useful, the
   "extern" is apparently ignored, so foo is inlined if possible but also
   emitted as a global, which causes multiple definition errors when
   building a shared libgmp.  */
#ifdef __SCO_VERSION__
#if __SCO_VERSION__ > 400000000 && __STDC__ != 1 \
  && ! defined (__GMP_EXTERN_INLINE)
#define __GMP_EXTERN_INLINE  static inline
#endif
#endif

/* Microsoft's C compiler accepts __inline */
#ifdef _MSC_VER
#define __GMP_EXTERN_INLINE  __inline
#endif

/* Recent enough Sun C compilers want "inline" */
#if defined (__SUNPRO_C) && __SUNPRO_C >= 0x560 \
  && ! defined (__GMP_EXTERN_INLINE)
#define __GMP_EXTERN_INLINE  inline
#endif

/* Somewhat older Sun C compilers want "static inline" */
#if defined (__SUNPRO_C) && __SUNPRO_C >= 0x540 \
  && ! defined (__GMP_EXTERN_INLINE)
#define __GMP_EXTERN_INLINE  static inline
#endif


/* C++ always has "inline" and since it's a normal feature the linker should
   discard duplicate non-inlined copies, or if it doesn't then that's a
   problem for everyone, not just GMP.  */
#if defined (__cplusplus) && ! defined (__GMP_EXTERN_INLINE)
#define __GMP_EXTERN_INLINE  inline
#endif

/* Don't do any inlining within a configure run, since if the compiler ends
   up emitting copies of the code into the object file it can end up
   demanding the various support routines (like mpn_popcount) for linking,
   making the "alloca" test and perhaps others fail.  And on hppa ia64 a
   pre-release gcc 3.2 was seen not respecting the "extern" in "extern
   __inline__", triggering this problem too.  */
#if defined (__GMP_WITHIN_CONFIGURE) && ! __GMP_WITHIN_CONFIGURE_INLINE
#undef __GMP_EXTERN_INLINE
#endif

/* By default, don't give a prototype when there's going to be an inline
   version.  Note in particular that Cray C++ objects to the combination of
   prototype and inline.  */
#ifdef __GMP_EXTERN_INLINE
#ifndef __GMP_INLINE_PROTOTYPES
#define __GMP_INLINE_PROTOTYPES  0
#endif
#else
#define __GMP_INLINE_PROTOTYPES  1
#endif


#define __GMP_ABS(x)   ((x) >= 0 ? (x) : -(x))
#define __GMP_MAX(h,i) ((h) > (i) ? (h) : (i))


/* __builtin_expect is in gcc 3.0, and not in 2.95. */
#if __GMP_GNUC_PREREQ (3,0)
#define __GMP_LIKELY(cond)    __builtin_expect ((cond) != 0, 1)
#define __GMP_UNLIKELY(cond)  __builtin_expect ((cond) != 0, 0)
#else
#define __GMP_LIKELY(cond)    (cond)
#define __GMP_UNLIKELY(cond)  (cond)
#endif

#ifdef _CRAY
#define __GMP_CRAY_Pragma(str)  _Pragma (str)
#else
#define __GMP_CRAY_Pragma(str)
#endif


/* Allow direct user access to numerator and denominator of an mpq_t object.  */
#define mpq_numref(Q) (&((Q)->_mp_num))
#define mpq_denref(Q) (&((Q)->_mp_den))


#if defined (__cplusplus)
extern "C" {
using std::FILE;
#endif

#define mp_set_memory_functions __gmp_set_memory_functions
__GMP_DECLSPEC void mp_set_memory_functions (void *(*) (size_t),
				      void *(*) (void *, size_t, size_t),
				      void (*) (void *, size_t)) __GMP_NOTHROW;

#define mp_get_memory_functions __gmp_get_memory_functions
__GMP_DECLSPEC void mp_get_memory_functions (void *(**) (size_t),
				      void *(**) (void *, size_t, size_t),
				      void (**) (void *, size_t)) __GMP_NOTHROW;

#define mp_bits_per_limb __gmp_bits_per_limb
__GMP_DECLSPEC extern const int mp_bits_per_limb;

#define gmp_errno __gmp_errno
__GMP_DECLSPEC extern int gmp_errno;

#define gmp_version __gmp_version
__GMP_DECLSPEC extern const char * const gmp_version;


/**************** Random number routines.  ****************/

/* obsolete */
#define gmp_randinit __gmp_randinit
__GMP_DECLSPEC void gmp_randinit (gmp_randstate_t, gmp_randalg_t, ...);

#define gmp_randinit_default __gmp_randinit_default
__GMP_DECLSPEC void gmp_randinit_default (gmp_randstate_t);

#define gmp_randinit_lc_2exp __gmp_randinit_lc_2exp
__GMP_DECLSPEC void gmp_randinit_lc_2exp (gmp_randstate_t, mpz_srcptr, unsigned long int, mp_bitcnt_t);

#define gmp_randinit_lc_2exp_size __gmp_randinit_lc_2exp_size
__GMP_DECLSPEC int gmp_randinit_lc_2exp_size (gmp_randstate_t, mp_bitcnt_t);

#define gmp_randinit_mt __gmp_randinit_mt
__GMP_DECLSPEC void gmp_randinit_mt (gmp_randstate_t);

#define gmp_randinit_set __gmp_randinit_set
__GMP_DECLSPEC void gmp_randinit_set (gmp_randstate_t, const __gmp_randstate_struct *);

#define gmp_randseed __gmp_randseed
__GMP_DECLSPEC void gmp_randseed (gmp_randstate_t, mpz_srcptr);

#define gmp_randseed_ui __gmp_randseed_ui
__GMP_DECLSPEC void gmp_randseed_ui (gmp_randstate_t, unsigned long int);

#define gmp_randclear __gmp_randclear
__GMP_DECLSPEC void gmp_randclear (gmp_randstate_t);

#define gmp_urandomb_ui __gmp_urandomb_ui
__GMP_DECLSPEC unsigned long gmp_urandomb_ui (gmp_randstate_t, unsigned long);

#define gmp_urandomm_ui __gmp_urandomm_ui
__GMP_DECLSPEC unsigned long gmp_urandomm_ui (gmp_randstate_t, unsigned long);


/**************** Formatted output routines.  ****************/

#define gmp_asprintf __gmp_asprintf
__GMP_DECLSPEC int gmp_asprintf (char **, const char *, ...);

#define gmp_fprintf __gmp_fprintf
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC int gmp_fprintf (FILE *, const char *, ...);
#endif

#define gmp_obstack_printf __gmp_obstack_printf
#if defined (_GMP_H_HAVE_OBSTACK)
__GMP_DECLSPEC int gmp_obstack_printf (struct obstack *, const char *, ...);
#endif

#define gmp_obstack_vprintf __gmp_obstack_vprintf
#if defined (_GMP_H_HAVE_OBSTACK) && defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_obstack_vprintf (struct obstack *, const char *, va_list);
#endif

#define gmp_printf __gmp_printf
__GMP_DECLSPEC int gmp_printf (const char *, ...);

#define gmp_snprintf __gmp_snprintf
__GMP_DECLSPEC int gmp_snprintf (char *, size_t, const char *, ...);

#define gmp_sprintf __gmp_sprintf
__GMP_DECLSPEC int gmp_sprintf (char *, const char *, ...);

#define gmp_vasprintf __gmp_vasprintf
#if defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vasprintf (char **, const char *, va_list);
#endif

#define gmp_vfprintf __gmp_vfprintf
#if defined (_GMP_H_HAVE_FILE) && defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vfprintf (FILE *, const char *, va_list);
#endif

#define gmp_vprintf __gmp_vprintf
#if defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vprintf (const char *, va_list);
#endif

#define gmp_vsnprintf __gmp_vsnprintf
#if defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vsnprintf (char *, size_t, const char *, va_list);
#endif

#define gmp_vsprintf __gmp_vsprintf
#if defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vsprintf (char *, const char *, va_list);
#endif


/**************** Formatted input routines.  ****************/

#define gmp_fscanf __gmp_fscanf
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC int gmp_fscanf (FILE *, const char *, ...);
#endif

#define gmp_scanf __gmp_scanf
__GMP_DECLSPEC int gmp_scanf (const char *, ...);

#define gmp_sscanf __gmp_sscanf
__GMP_DECLSPEC int gmp_sscanf (const char *, const char *, ...);

#define gmp_vfscanf __gmp_vfscanf
#if defined (_GMP_H_HAVE_FILE) && defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vfscanf (FILE *, const char *, va_list);
#endif

#define gmp_vscanf __gmp_vscanf
#if defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vscanf (const char *, va_list);
#endif

#define gmp_vsscanf __gmp_vsscanf
#if defined (_GMP_H_HAVE_VA_LIST)
__GMP_DECLSPEC int gmp_vsscanf (const char *, const char *, va_list);
#endif


/**************** Integer (i.e. Z) routines.  ****************/

#define _mpz_realloc __gmpz_realloc
#define mpz_realloc __gmpz_realloc
__GMP_DECLSPEC void *_mpz_realloc (mpz_ptr, mp_size_t);

#define mpz_abs __gmpz_abs
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_abs)
__GMP_DECLSPEC void mpz_abs (mpz_ptr, mpz_srcptr);
#endif

#define mpz_add __gmpz_add
__GMP_DECLSPEC void mpz_add (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_add_ui __gmpz_add_ui
__GMP_DECLSPEC void mpz_add_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_addmul __gmpz_addmul
__GMP_DECLSPEC void mpz_addmul (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_addmul_ui __gmpz_addmul_ui
__GMP_DECLSPEC void mpz_addmul_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_and __gmpz_and
__GMP_DECLSPEC void mpz_and (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_array_init __gmpz_array_init
__GMP_DECLSPEC void mpz_array_init (mpz_ptr, mp_size_t, mp_size_t);

#define mpz_bin_ui __gmpz_bin_ui
__GMP_DECLSPEC void mpz_bin_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_bin_uiui __gmpz_bin_uiui
__GMP_DECLSPEC void mpz_bin_uiui (mpz_ptr, unsigned long int, unsigned long int);

#define mpz_cdiv_q __gmpz_cdiv_q
__GMP_DECLSPEC void mpz_cdiv_q (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_cdiv_q_2exp __gmpz_cdiv_q_2exp
__GMP_DECLSPEC void mpz_cdiv_q_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_cdiv_q_ui __gmpz_cdiv_q_ui
__GMP_DECLSPEC unsigned long int mpz_cdiv_q_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_cdiv_qr __gmpz_cdiv_qr
__GMP_DECLSPEC void mpz_cdiv_qr (mpz_ptr, mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_cdiv_qr_ui __gmpz_cdiv_qr_ui
__GMP_DECLSPEC unsigned long int mpz_cdiv_qr_ui (mpz_ptr, mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_cdiv_r __gmpz_cdiv_r
__GMP_DECLSPEC void mpz_cdiv_r (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_cdiv_r_2exp __gmpz_cdiv_r_2exp
__GMP_DECLSPEC void mpz_cdiv_r_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_cdiv_r_ui __gmpz_cdiv_r_ui
__GMP_DECLSPEC unsigned long int mpz_cdiv_r_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_cdiv_ui __gmpz_cdiv_ui
__GMP_DECLSPEC unsigned long int mpz_cdiv_ui (mpz_srcptr, unsigned long int) __GMP_ATTRIBUTE_PURE;

#define mpz_clear __gmpz_clear
__GMP_DECLSPEC void mpz_clear (mpz_ptr);

#define mpz_clears __gmpz_clears
__GMP_DECLSPEC void mpz_clears (mpz_ptr, ...);

#define mpz_clrbit __gmpz_clrbit
__GMP_DECLSPEC void mpz_clrbit (mpz_ptr, mp_bitcnt_t);

#define mpz_cmp __gmpz_cmp
__GMP_DECLSPEC int mpz_cmp (mpz_srcptr, mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_cmp_d __gmpz_cmp_d
__GMP_DECLSPEC int mpz_cmp_d (mpz_srcptr, double) __GMP_ATTRIBUTE_PURE;

#define _mpz_cmp_si __gmpz_cmp_si
__GMP_DECLSPEC int _mpz_cmp_si (mpz_srcptr, signed long int) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define _mpz_cmp_ui __gmpz_cmp_ui
__GMP_DECLSPEC int _mpz_cmp_ui (mpz_srcptr, unsigned long int) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_cmpabs __gmpz_cmpabs
__GMP_DECLSPEC int mpz_cmpabs (mpz_srcptr, mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_cmpabs_d __gmpz_cmpabs_d
__GMP_DECLSPEC int mpz_cmpabs_d (mpz_srcptr, double) __GMP_ATTRIBUTE_PURE;

#define mpz_cmpabs_ui __gmpz_cmpabs_ui
__GMP_DECLSPEC int mpz_cmpabs_ui (mpz_srcptr, unsigned long int) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_com __gmpz_com
__GMP_DECLSPEC void mpz_com (mpz_ptr, mpz_srcptr);

#define mpz_combit __gmpz_combit
__GMP_DECLSPEC void mpz_combit (mpz_ptr, mp_bitcnt_t);

#define mpz_congruent_p __gmpz_congruent_p
__GMP_DECLSPEC int mpz_congruent_p (mpz_srcptr, mpz_srcptr, mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_congruent_2exp_p __gmpz_congruent_2exp_p
__GMP_DECLSPEC int mpz_congruent_2exp_p (mpz_srcptr, mpz_srcptr, mp_bitcnt_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_congruent_ui_p __gmpz_congruent_ui_p
__GMP_DECLSPEC int mpz_congruent_ui_p (mpz_srcptr, unsigned long, unsigned long) __GMP_ATTRIBUTE_PURE;

#define mpz_divexact __gmpz_divexact
__GMP_DECLSPEC void mpz_divexact (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_divexact_ui __gmpz_divexact_ui
__GMP_DECLSPEC void mpz_divexact_ui (mpz_ptr, mpz_srcptr, unsigned long);

#define mpz_divisible_p __gmpz_divisible_p
__GMP_DECLSPEC int mpz_divisible_p (mpz_srcptr, mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_divisible_ui_p __gmpz_divisible_ui_p
__GMP_DECLSPEC int mpz_divisible_ui_p (mpz_srcptr, unsigned long) __GMP_ATTRIBUTE_PURE;

#define mpz_divisible_2exp_p __gmpz_divisible_2exp_p
__GMP_DECLSPEC int mpz_divisible_2exp_p (mpz_srcptr, mp_bitcnt_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_dump __gmpz_dump
__GMP_DECLSPEC void mpz_dump (mpz_srcptr);

#define mpz_export __gmpz_export
__GMP_DECLSPEC void *mpz_export (void *, size_t *, int, size_t, int, size_t, mpz_srcptr);

#define mpz_fac_ui __gmpz_fac_ui
__GMP_DECLSPEC void mpz_fac_ui (mpz_ptr, unsigned long int);

#define mpz_2fac_ui __gmpz_2fac_ui
__GMP_DECLSPEC void mpz_2fac_ui (mpz_ptr, unsigned long int);

#define mpz_mfac_uiui __gmpz_mfac_uiui
__GMP_DECLSPEC void mpz_mfac_uiui (mpz_ptr, unsigned long int, unsigned long int);

#define mpz_primorial_ui __gmpz_primorial_ui
__GMP_DECLSPEC void mpz_primorial_ui (mpz_ptr, unsigned long int);

#define mpz_fdiv_q __gmpz_fdiv_q
__GMP_DECLSPEC void mpz_fdiv_q (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_fdiv_q_2exp __gmpz_fdiv_q_2exp
__GMP_DECLSPEC void mpz_fdiv_q_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_fdiv_q_ui __gmpz_fdiv_q_ui
__GMP_DECLSPEC unsigned long int mpz_fdiv_q_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_fdiv_qr __gmpz_fdiv_qr
__GMP_DECLSPEC void mpz_fdiv_qr (mpz_ptr, mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_fdiv_qr_ui __gmpz_fdiv_qr_ui
__GMP_DECLSPEC unsigned long int mpz_fdiv_qr_ui (mpz_ptr, mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_fdiv_r __gmpz_fdiv_r
__GMP_DECLSPEC void mpz_fdiv_r (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_fdiv_r_2exp __gmpz_fdiv_r_2exp
__GMP_DECLSPEC void mpz_fdiv_r_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_fdiv_r_ui __gmpz_fdiv_r_ui
__GMP_DECLSPEC unsigned long int mpz_fdiv_r_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_fdiv_ui __gmpz_fdiv_ui
__GMP_DECLSPEC unsigned long int mpz_fdiv_ui (mpz_srcptr, unsigned long int) __GMP_ATTRIBUTE_PURE;

#define mpz_fib_ui __gmpz_fib_ui
__GMP_DECLSPEC void mpz_fib_ui (mpz_ptr, unsigned long int);

#define mpz_fib2_ui __gmpz_fib2_ui
__GMP_DECLSPEC void mpz_fib2_ui (mpz_ptr, mpz_ptr, unsigned long int);

#define mpz_fits_sint_p __gmpz_fits_sint_p
__GMP_DECLSPEC int mpz_fits_sint_p (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_fits_slong_p __gmpz_fits_slong_p
__GMP_DECLSPEC int mpz_fits_slong_p (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_fits_sshort_p __gmpz_fits_sshort_p
__GMP_DECLSPEC int mpz_fits_sshort_p (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_fits_uint_p __gmpz_fits_uint_p
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_fits_uint_p)
__GMP_DECLSPEC int mpz_fits_uint_p (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_fits_ulong_p __gmpz_fits_ulong_p
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_fits_ulong_p)
__GMP_DECLSPEC int mpz_fits_ulong_p (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_fits_ushort_p __gmpz_fits_ushort_p
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_fits_ushort_p)
__GMP_DECLSPEC int mpz_fits_ushort_p (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_gcd __gmpz_gcd
__GMP_DECLSPEC void mpz_gcd (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_gcd_ui __gmpz_gcd_ui
__GMP_DECLSPEC unsigned long int mpz_gcd_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_gcdext __gmpz_gcdext
__GMP_DECLSPEC void mpz_gcdext (mpz_ptr, mpz_ptr, mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_get_d __gmpz_get_d
__GMP_DECLSPEC double mpz_get_d (mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_get_d_2exp __gmpz_get_d_2exp
__GMP_DECLSPEC double mpz_get_d_2exp (signed long int *, mpz_srcptr);

#define mpz_get_si __gmpz_get_si
__GMP_DECLSPEC /* signed */ long int mpz_get_si (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_get_str __gmpz_get_str
__GMP_DECLSPEC char *mpz_get_str (char *, int, mpz_srcptr);

#define mpz_get_ui __gmpz_get_ui
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_get_ui)
__GMP_DECLSPEC unsigned long int mpz_get_ui (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_getlimbn __gmpz_getlimbn
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_getlimbn)
__GMP_DECLSPEC mp_limb_t mpz_getlimbn (mpz_srcptr, mp_size_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_hamdist __gmpz_hamdist
__GMP_DECLSPEC mp_bitcnt_t mpz_hamdist (mpz_srcptr, mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_import __gmpz_import
__GMP_DECLSPEC void mpz_import (mpz_ptr, size_t, int, size_t, int, size_t, const void *);

#define mpz_init __gmpz_init
__GMP_DECLSPEC void mpz_init (mpz_ptr) __GMP_NOTHROW;

#define mpz_init2 __gmpz_init2
__GMP_DECLSPEC void mpz_init2 (mpz_ptr, mp_bitcnt_t);

#define mpz_inits __gmpz_inits
__GMP_DECLSPEC void mpz_inits (mpz_ptr, ...) __GMP_NOTHROW;

#define mpz_init_set __gmpz_init_set
__GMP_DECLSPEC void mpz_init_set (mpz_ptr, mpz_srcptr);

#define mpz_init_set_d __gmpz_init_set_d
__GMP_DECLSPEC void mpz_init_set_d (mpz_ptr, double);

#define mpz_init_set_si __gmpz_init_set_si
__GMP_DECLSPEC void mpz_init_set_si (mpz_ptr, signed long int);

#define mpz_init_set_str __gmpz_init_set_str
__GMP_DECLSPEC int mpz_init_set_str (mpz_ptr, const char *, int);

#define mpz_init_set_ui __gmpz_init_set_ui
__GMP_DECLSPEC void mpz_init_set_ui (mpz_ptr, unsigned long int);

#define mpz_inp_raw __gmpz_inp_raw
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpz_inp_raw (mpz_ptr, FILE *);
#endif

#define mpz_inp_str __gmpz_inp_str
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpz_inp_str (mpz_ptr, FILE *, int);
#endif

#define mpz_invert __gmpz_invert
__GMP_DECLSPEC int mpz_invert (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_ior __gmpz_ior
__GMP_DECLSPEC void mpz_ior (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_jacobi __gmpz_jacobi
__GMP_DECLSPEC int mpz_jacobi (mpz_srcptr, mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_kronecker mpz_jacobi  /* alias */

#define mpz_kronecker_si __gmpz_kronecker_si
__GMP_DECLSPEC int mpz_kronecker_si (mpz_srcptr, long) __GMP_ATTRIBUTE_PURE;

#define mpz_kronecker_ui __gmpz_kronecker_ui
__GMP_DECLSPEC int mpz_kronecker_ui (mpz_srcptr, unsigned long) __GMP_ATTRIBUTE_PURE;

#define mpz_si_kronecker __gmpz_si_kronecker
__GMP_DECLSPEC int mpz_si_kronecker (long, mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_ui_kronecker __gmpz_ui_kronecker
__GMP_DECLSPEC int mpz_ui_kronecker (unsigned long, mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_lcm __gmpz_lcm
__GMP_DECLSPEC void mpz_lcm (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_lcm_ui __gmpz_lcm_ui
__GMP_DECLSPEC void mpz_lcm_ui (mpz_ptr, mpz_srcptr, unsigned long);

#define mpz_legendre mpz_jacobi  /* alias */

#define mpz_lucnum_ui __gmpz_lucnum_ui
__GMP_DECLSPEC void mpz_lucnum_ui (mpz_ptr, unsigned long int);

#define mpz_lucnum2_ui __gmpz_lucnum2_ui
__GMP_DECLSPEC void mpz_lucnum2_ui (mpz_ptr, mpz_ptr, unsigned long int);

#define mpz_millerrabin __gmpz_millerrabin
__GMP_DECLSPEC int mpz_millerrabin (mpz_srcptr, int) __GMP_ATTRIBUTE_PURE;

#define mpz_mod __gmpz_mod
__GMP_DECLSPEC void mpz_mod (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_mod_ui mpz_fdiv_r_ui /* same as fdiv_r because divisor unsigned */

#define mpz_mul __gmpz_mul
__GMP_DECLSPEC void mpz_mul (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_mul_2exp __gmpz_mul_2exp
__GMP_DECLSPEC void mpz_mul_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_mul_si __gmpz_mul_si
__GMP_DECLSPEC void mpz_mul_si (mpz_ptr, mpz_srcptr, long int);

#define mpz_mul_ui __gmpz_mul_ui
__GMP_DECLSPEC void mpz_mul_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_neg __gmpz_neg
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_neg)
__GMP_DECLSPEC void mpz_neg (mpz_ptr, mpz_srcptr);
#endif

#define mpz_nextprime __gmpz_nextprime
__GMP_DECLSPEC void mpz_nextprime (mpz_ptr, mpz_srcptr);

#define mpz_out_raw __gmpz_out_raw
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpz_out_raw (FILE *, mpz_srcptr);
#endif

#define mpz_out_str __gmpz_out_str
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpz_out_str (FILE *, int, mpz_srcptr);
#endif

#define mpz_perfect_power_p __gmpz_perfect_power_p
__GMP_DECLSPEC int mpz_perfect_power_p (mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpz_perfect_square_p __gmpz_perfect_square_p
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_perfect_square_p)
__GMP_DECLSPEC int mpz_perfect_square_p (mpz_srcptr) __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_popcount __gmpz_popcount
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_popcount)
__GMP_DECLSPEC mp_bitcnt_t mpz_popcount (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_pow_ui __gmpz_pow_ui
__GMP_DECLSPEC void mpz_pow_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_powm __gmpz_powm
__GMP_DECLSPEC void mpz_powm (mpz_ptr, mpz_srcptr, mpz_srcptr, mpz_srcptr);

#define mpz_powm_sec __gmpz_powm_sec
__GMP_DECLSPEC void mpz_powm_sec (mpz_ptr, mpz_srcptr, mpz_srcptr, mpz_srcptr);

#define mpz_powm_ui __gmpz_powm_ui
__GMP_DECLSPEC void mpz_powm_ui (mpz_ptr, mpz_srcptr, unsigned long int, mpz_srcptr);

#define mpz_probab_prime_p __gmpz_probab_prime_p
__GMP_DECLSPEC int mpz_probab_prime_p (mpz_srcptr, int) __GMP_ATTRIBUTE_PURE;

#define mpz_random __gmpz_random
__GMP_DECLSPEC void mpz_random (mpz_ptr, mp_size_t);

#define mpz_random2 __gmpz_random2
__GMP_DECLSPEC void mpz_random2 (mpz_ptr, mp_size_t);

#define mpz_realloc2 __gmpz_realloc2
__GMP_DECLSPEC void mpz_realloc2 (mpz_ptr, mp_bitcnt_t);

#define mpz_remove __gmpz_remove
__GMP_DECLSPEC mp_bitcnt_t mpz_remove (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_root __gmpz_root
__GMP_DECLSPEC int mpz_root (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_rootrem __gmpz_rootrem
__GMP_DECLSPEC void mpz_rootrem (mpz_ptr, mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_rrandomb __gmpz_rrandomb
__GMP_DECLSPEC void mpz_rrandomb (mpz_ptr, gmp_randstate_t, mp_bitcnt_t);

#define mpz_scan0 __gmpz_scan0
__GMP_DECLSPEC mp_bitcnt_t mpz_scan0 (mpz_srcptr, mp_bitcnt_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_scan1 __gmpz_scan1
__GMP_DECLSPEC mp_bitcnt_t mpz_scan1 (mpz_srcptr, mp_bitcnt_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_set __gmpz_set
__GMP_DECLSPEC void mpz_set (mpz_ptr, mpz_srcptr);

#define mpz_set_d __gmpz_set_d
__GMP_DECLSPEC void mpz_set_d (mpz_ptr, double);

#define mpz_set_f __gmpz_set_f
__GMP_DECLSPEC void mpz_set_f (mpz_ptr, mpf_srcptr);

#define mpz_set_q __gmpz_set_q
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_set_q)
__GMP_DECLSPEC void mpz_set_q (mpz_ptr, mpq_srcptr);
#endif

#define mpz_set_si __gmpz_set_si
__GMP_DECLSPEC void mpz_set_si (mpz_ptr, signed long int);

#define mpz_set_str __gmpz_set_str
__GMP_DECLSPEC int mpz_set_str (mpz_ptr, const char *, int);

#define mpz_set_ui __gmpz_set_ui
__GMP_DECLSPEC void mpz_set_ui (mpz_ptr, unsigned long int);

#define mpz_setbit __gmpz_setbit
__GMP_DECLSPEC void mpz_setbit (mpz_ptr, mp_bitcnt_t);

#define mpz_size __gmpz_size
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpz_size)
__GMP_DECLSPEC size_t mpz_size (mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpz_sizeinbase __gmpz_sizeinbase
__GMP_DECLSPEC size_t mpz_sizeinbase (mpz_srcptr, int) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_sqrt __gmpz_sqrt
__GMP_DECLSPEC void mpz_sqrt (mpz_ptr, mpz_srcptr);

#define mpz_sqrtrem __gmpz_sqrtrem
__GMP_DECLSPEC void mpz_sqrtrem (mpz_ptr, mpz_ptr, mpz_srcptr);

#define mpz_sub __gmpz_sub
__GMP_DECLSPEC void mpz_sub (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_sub_ui __gmpz_sub_ui
__GMP_DECLSPEC void mpz_sub_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_ui_sub __gmpz_ui_sub
__GMP_DECLSPEC void mpz_ui_sub (mpz_ptr, unsigned long int, mpz_srcptr);

#define mpz_submul __gmpz_submul
__GMP_DECLSPEC void mpz_submul (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_submul_ui __gmpz_submul_ui
__GMP_DECLSPEC void mpz_submul_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_swap __gmpz_swap
__GMP_DECLSPEC void mpz_swap (mpz_ptr, mpz_ptr) __GMP_NOTHROW;

#define mpz_tdiv_ui __gmpz_tdiv_ui
__GMP_DECLSPEC unsigned long int mpz_tdiv_ui (mpz_srcptr, unsigned long int) __GMP_ATTRIBUTE_PURE;

#define mpz_tdiv_q __gmpz_tdiv_q
__GMP_DECLSPEC void mpz_tdiv_q (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_tdiv_q_2exp __gmpz_tdiv_q_2exp
__GMP_DECLSPEC void mpz_tdiv_q_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_tdiv_q_ui __gmpz_tdiv_q_ui
__GMP_DECLSPEC unsigned long int mpz_tdiv_q_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_tdiv_qr __gmpz_tdiv_qr
__GMP_DECLSPEC void mpz_tdiv_qr (mpz_ptr, mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_tdiv_qr_ui __gmpz_tdiv_qr_ui
__GMP_DECLSPEC unsigned long int mpz_tdiv_qr_ui (mpz_ptr, mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_tdiv_r __gmpz_tdiv_r
__GMP_DECLSPEC void mpz_tdiv_r (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_tdiv_r_2exp __gmpz_tdiv_r_2exp
__GMP_DECLSPEC void mpz_tdiv_r_2exp (mpz_ptr, mpz_srcptr, mp_bitcnt_t);

#define mpz_tdiv_r_ui __gmpz_tdiv_r_ui
__GMP_DECLSPEC unsigned long int mpz_tdiv_r_ui (mpz_ptr, mpz_srcptr, unsigned long int);

#define mpz_tstbit __gmpz_tstbit
__GMP_DECLSPEC int mpz_tstbit (mpz_srcptr, mp_bitcnt_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpz_ui_pow_ui __gmpz_ui_pow_ui
__GMP_DECLSPEC void mpz_ui_pow_ui (mpz_ptr, unsigned long int, unsigned long int);

#define mpz_urandomb __gmpz_urandomb
__GMP_DECLSPEC void mpz_urandomb (mpz_ptr, gmp_randstate_t, mp_bitcnt_t);

#define mpz_urandomm __gmpz_urandomm
__GMP_DECLSPEC void mpz_urandomm (mpz_ptr, gmp_randstate_t, mpz_srcptr);

#define mpz_xor __gmpz_xor
#define mpz_eor __gmpz_xor
__GMP_DECLSPEC void mpz_xor (mpz_ptr, mpz_srcptr, mpz_srcptr);

#define mpz_limbs_read __gmpz_limbs_read
__GMP_DECLSPEC mp_srcptr mpz_limbs_read (mpz_srcptr);

#define mpz_limbs_write __gmpz_limbs_write
__GMP_DECLSPEC mp_ptr mpz_limbs_write (mpz_ptr, mp_size_t);

#define mpz_limbs_modify __gmpz_limbs_modify
__GMP_DECLSPEC mp_ptr mpz_limbs_modify (mpz_ptr, mp_size_t);

#define mpz_limbs_finish __gmpz_limbs_finish
__GMP_DECLSPEC void mpz_limbs_finish (mpz_ptr, mp_size_t);

#define mpz_roinit_n __gmpz_roinit_n
__GMP_DECLSPEC mpz_srcptr mpz_roinit_n (mpz_ptr, mp_srcptr, mp_size_t);

#define MPZ_ROINIT_N(xp, xs) {{0, (xs),(xp) }}

/**************** Rational (i.e. Q) routines.  ****************/

#define mpq_abs __gmpq_abs
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpq_abs)
__GMP_DECLSPEC void mpq_abs (mpq_ptr, mpq_srcptr);
#endif

#define mpq_add __gmpq_add
__GMP_DECLSPEC void mpq_add (mpq_ptr, mpq_srcptr, mpq_srcptr);

#define mpq_canonicalize __gmpq_canonicalize
__GMP_DECLSPEC void mpq_canonicalize (mpq_ptr);

#define mpq_clear __gmpq_clear
__GMP_DECLSPEC void mpq_clear (mpq_ptr);

#define mpq_clears __gmpq_clears
__GMP_DECLSPEC void mpq_clears (mpq_ptr, ...);

#define mpq_cmp __gmpq_cmp
__GMP_DECLSPEC int mpq_cmp (mpq_srcptr, mpq_srcptr) __GMP_ATTRIBUTE_PURE;

#define _mpq_cmp_si __gmpq_cmp_si
__GMP_DECLSPEC int _mpq_cmp_si (mpq_srcptr, long, unsigned long) __GMP_ATTRIBUTE_PURE;

#define _mpq_cmp_ui __gmpq_cmp_ui
__GMP_DECLSPEC int _mpq_cmp_ui (mpq_srcptr, unsigned long int, unsigned long int) __GMP_ATTRIBUTE_PURE;

#define mpq_cmp_z __gmpq_cmp_z
__GMP_DECLSPEC int mpq_cmp_z (mpq_srcptr, mpz_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpq_div __gmpq_div
__GMP_DECLSPEC void mpq_div (mpq_ptr, mpq_srcptr, mpq_srcptr);

#define mpq_div_2exp __gmpq_div_2exp
__GMP_DECLSPEC void mpq_div_2exp (mpq_ptr, mpq_srcptr, mp_bitcnt_t);

#define mpq_equal __gmpq_equal
__GMP_DECLSPEC int mpq_equal (mpq_srcptr, mpq_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpq_get_num __gmpq_get_num
__GMP_DECLSPEC void mpq_get_num (mpz_ptr, mpq_srcptr);

#define mpq_get_den __gmpq_get_den
__GMP_DECLSPEC void mpq_get_den (mpz_ptr, mpq_srcptr);

#define mpq_get_d __gmpq_get_d
__GMP_DECLSPEC double mpq_get_d (mpq_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpq_get_str __gmpq_get_str
__GMP_DECLSPEC char *mpq_get_str (char *, int, mpq_srcptr);

#define mpq_init __gmpq_init
__GMP_DECLSPEC void mpq_init (mpq_ptr);

#define mpq_inits __gmpq_inits
__GMP_DECLSPEC void mpq_inits (mpq_ptr, ...);

#define mpq_inp_str __gmpq_inp_str
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpq_inp_str (mpq_ptr, FILE *, int);
#endif

#define mpq_inv __gmpq_inv
__GMP_DECLSPEC void mpq_inv (mpq_ptr, mpq_srcptr);

#define mpq_mul __gmpq_mul
__GMP_DECLSPEC void mpq_mul (mpq_ptr, mpq_srcptr, mpq_srcptr);

#define mpq_mul_2exp __gmpq_mul_2exp
__GMP_DECLSPEC void mpq_mul_2exp (mpq_ptr, mpq_srcptr, mp_bitcnt_t);

#define mpq_neg __gmpq_neg
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpq_neg)
__GMP_DECLSPEC void mpq_neg (mpq_ptr, mpq_srcptr);
#endif

#define mpq_out_str __gmpq_out_str
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpq_out_str (FILE *, int, mpq_srcptr);
#endif

#define mpq_set __gmpq_set
__GMP_DECLSPEC void mpq_set (mpq_ptr, mpq_srcptr);

#define mpq_set_d __gmpq_set_d
__GMP_DECLSPEC void mpq_set_d (mpq_ptr, double);

#define mpq_set_den __gmpq_set_den
__GMP_DECLSPEC void mpq_set_den (mpq_ptr, mpz_srcptr);

#define mpq_set_f __gmpq_set_f
__GMP_DECLSPEC void mpq_set_f (mpq_ptr, mpf_srcptr);

#define mpq_set_num __gmpq_set_num
__GMP_DECLSPEC void mpq_set_num (mpq_ptr, mpz_srcptr);

#define mpq_set_si __gmpq_set_si
__GMP_DECLSPEC void mpq_set_si (mpq_ptr, signed long int, unsigned long int);

#define mpq_set_str __gmpq_set_str
__GMP_DECLSPEC int mpq_set_str (mpq_ptr, const char *, int);

#define mpq_set_ui __gmpq_set_ui
__GMP_DECLSPEC void mpq_set_ui (mpq_ptr, unsigned long int, unsigned long int);

#define mpq_set_z __gmpq_set_z
__GMP_DECLSPEC void mpq_set_z (mpq_ptr, mpz_srcptr);

#define mpq_sub __gmpq_sub
__GMP_DECLSPEC void mpq_sub (mpq_ptr, mpq_srcptr, mpq_srcptr);

#define mpq_swap __gmpq_swap
__GMP_DECLSPEC void mpq_swap (mpq_ptr, mpq_ptr) __GMP_NOTHROW;


/**************** Float (i.e. F) routines.  ****************/

#define mpf_abs __gmpf_abs
__GMP_DECLSPEC void mpf_abs (mpf_ptr, mpf_srcptr);

#define mpf_add __gmpf_add
__GMP_DECLSPEC void mpf_add (mpf_ptr, mpf_srcptr, mpf_srcptr);

#define mpf_add_ui __gmpf_add_ui
__GMP_DECLSPEC void mpf_add_ui (mpf_ptr, mpf_srcptr, unsigned long int);
#define mpf_ceil __gmpf_ceil
__GMP_DECLSPEC void mpf_ceil (mpf_ptr, mpf_srcptr);

#define mpf_clear __gmpf_clear
__GMP_DECLSPEC void mpf_clear (mpf_ptr);

#define mpf_clears __gmpf_clears
__GMP_DECLSPEC void mpf_clears (mpf_ptr, ...);

#define mpf_cmp __gmpf_cmp
__GMP_DECLSPEC int mpf_cmp (mpf_srcptr, mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_cmp_z __gmpf_cmp_z
__GMP_DECLSPEC int mpf_cmp_z (mpf_srcptr, mpz_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_cmp_d __gmpf_cmp_d
__GMP_DECLSPEC int mpf_cmp_d (mpf_srcptr, double) __GMP_ATTRIBUTE_PURE;

#define mpf_cmp_si __gmpf_cmp_si
__GMP_DECLSPEC int mpf_cmp_si (mpf_srcptr, signed long int) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_cmp_ui __gmpf_cmp_ui
__GMP_DECLSPEC int mpf_cmp_ui (mpf_srcptr, unsigned long int) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_div __gmpf_div
__GMP_DECLSPEC void mpf_div (mpf_ptr, mpf_srcptr, mpf_srcptr);

#define mpf_div_2exp __gmpf_div_2exp
__GMP_DECLSPEC void mpf_div_2exp (mpf_ptr, mpf_srcptr, mp_bitcnt_t);

#define mpf_div_ui __gmpf_div_ui
__GMP_DECLSPEC void mpf_div_ui (mpf_ptr, mpf_srcptr, unsigned long int);

#define mpf_dump __gmpf_dump
__GMP_DECLSPEC void mpf_dump (mpf_srcptr);

#define mpf_eq __gmpf_eq
__GMP_DECLSPEC int mpf_eq (mpf_srcptr, mpf_srcptr, mp_bitcnt_t) __GMP_ATTRIBUTE_PURE;

#define mpf_fits_sint_p __gmpf_fits_sint_p
__GMP_DECLSPEC int mpf_fits_sint_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_fits_slong_p __gmpf_fits_slong_p
__GMP_DECLSPEC int mpf_fits_slong_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_fits_sshort_p __gmpf_fits_sshort_p
__GMP_DECLSPEC int mpf_fits_sshort_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_fits_uint_p __gmpf_fits_uint_p
__GMP_DECLSPEC int mpf_fits_uint_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_fits_ulong_p __gmpf_fits_ulong_p
__GMP_DECLSPEC int mpf_fits_ulong_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_fits_ushort_p __gmpf_fits_ushort_p
__GMP_DECLSPEC int mpf_fits_ushort_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_floor __gmpf_floor
__GMP_DECLSPEC void mpf_floor (mpf_ptr, mpf_srcptr);

#define mpf_get_d __gmpf_get_d
__GMP_DECLSPEC double mpf_get_d (mpf_srcptr) __GMP_ATTRIBUTE_PURE;

#define mpf_get_d_2exp __gmpf_get_d_2exp
__GMP_DECLSPEC double mpf_get_d_2exp (signed long int *, mpf_srcptr);

#define mpf_get_default_prec __gmpf_get_default_prec
__GMP_DECLSPEC mp_bitcnt_t mpf_get_default_prec (void) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_get_prec __gmpf_get_prec
__GMP_DECLSPEC mp_bitcnt_t mpf_get_prec (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_get_si __gmpf_get_si
__GMP_DECLSPEC long mpf_get_si (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_get_str __gmpf_get_str
__GMP_DECLSPEC char *mpf_get_str (char *, mp_exp_t *, int, size_t, mpf_srcptr);

#define mpf_get_ui __gmpf_get_ui
__GMP_DECLSPEC unsigned long mpf_get_ui (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_init __gmpf_init
__GMP_DECLSPEC void mpf_init (mpf_ptr);

#define mpf_init2 __gmpf_init2
__GMP_DECLSPEC void mpf_init2 (mpf_ptr, mp_bitcnt_t);

#define mpf_inits __gmpf_inits
__GMP_DECLSPEC void mpf_inits (mpf_ptr, ...);

#define mpf_init_set __gmpf_init_set
__GMP_DECLSPEC void mpf_init_set (mpf_ptr, mpf_srcptr);

#define mpf_init_set_d __gmpf_init_set_d
__GMP_DECLSPEC void mpf_init_set_d (mpf_ptr, double);

#define mpf_init_set_si __gmpf_init_set_si
__GMP_DECLSPEC void mpf_init_set_si (mpf_ptr, signed long int);

#define mpf_init_set_str __gmpf_init_set_str
__GMP_DECLSPEC int mpf_init_set_str (mpf_ptr, const char *, int);

#define mpf_init_set_ui __gmpf_init_set_ui
__GMP_DECLSPEC void mpf_init_set_ui (mpf_ptr, unsigned long int);

#define mpf_inp_str __gmpf_inp_str
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpf_inp_str (mpf_ptr, FILE *, int);
#endif

#define mpf_integer_p __gmpf_integer_p
__GMP_DECLSPEC int mpf_integer_p (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_mul __gmpf_mul
__GMP_DECLSPEC void mpf_mul (mpf_ptr, mpf_srcptr, mpf_srcptr);

#define mpf_mul_2exp __gmpf_mul_2exp
__GMP_DECLSPEC void mpf_mul_2exp (mpf_ptr, mpf_srcptr, mp_bitcnt_t);

#define mpf_mul_ui __gmpf_mul_ui
__GMP_DECLSPEC void mpf_mul_ui (mpf_ptr, mpf_srcptr, unsigned long int);

#define mpf_neg __gmpf_neg
__GMP_DECLSPEC void mpf_neg (mpf_ptr, mpf_srcptr);

#define mpf_out_str __gmpf_out_str
#ifdef _GMP_H_HAVE_FILE
__GMP_DECLSPEC size_t mpf_out_str (FILE *, int, size_t, mpf_srcptr);
#endif

#define mpf_pow_ui __gmpf_pow_ui
__GMP_DECLSPEC void mpf_pow_ui (mpf_ptr, mpf_srcptr, unsigned long int);

#define mpf_random2 __gmpf_random2
__GMP_DECLSPEC void mpf_random2 (mpf_ptr, mp_size_t, mp_exp_t);

#define mpf_reldiff __gmpf_reldiff
__GMP_DECLSPEC void mpf_reldiff (mpf_ptr, mpf_srcptr, mpf_srcptr);

#define mpf_set __gmpf_set
__GMP_DECLSPEC void mpf_set (mpf_ptr, mpf_srcptr);

#define mpf_set_d __gmpf_set_d
__GMP_DECLSPEC void mpf_set_d (mpf_ptr, double);

#define mpf_set_default_prec __gmpf_set_default_prec
__GMP_DECLSPEC void mpf_set_default_prec (mp_bitcnt_t) __GMP_NOTHROW;

#define mpf_set_prec __gmpf_set_prec
__GMP_DECLSPEC void mpf_set_prec (mpf_ptr, mp_bitcnt_t);

#define mpf_set_prec_raw __gmpf_set_prec_raw
__GMP_DECLSPEC void mpf_set_prec_raw (mpf_ptr, mp_bitcnt_t) __GMP_NOTHROW;

#define mpf_set_q __gmpf_set_q
__GMP_DECLSPEC void mpf_set_q (mpf_ptr, mpq_srcptr);

#define mpf_set_si __gmpf_set_si
__GMP_DECLSPEC void mpf_set_si (mpf_ptr, signed long int);

#define mpf_set_str __gmpf_set_str
__GMP_DECLSPEC int mpf_set_str (mpf_ptr, const char *, int);

#define mpf_set_ui __gmpf_set_ui
__GMP_DECLSPEC void mpf_set_ui (mpf_ptr, unsigned long int);

#define mpf_set_z __gmpf_set_z
__GMP_DECLSPEC void mpf_set_z (mpf_ptr, mpz_srcptr);

#define mpf_size __gmpf_size
__GMP_DECLSPEC size_t mpf_size (mpf_srcptr) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpf_sqrt __gmpf_sqrt
__GMP_DECLSPEC void mpf_sqrt (mpf_ptr, mpf_srcptr);

#define mpf_sqrt_ui __gmpf_sqrt_ui
__GMP_DECLSPEC void mpf_sqrt_ui (mpf_ptr, unsigned long int);

#define mpf_sub __gmpf_sub
__GMP_DECLSPEC void mpf_sub (mpf_ptr, mpf_srcptr, mpf_srcptr);

#define mpf_sub_ui __gmpf_sub_ui
__GMP_DECLSPEC void mpf_sub_ui (mpf_ptr, mpf_srcptr, unsigned long int);

#define mpf_swap __gmpf_swap
__GMP_DECLSPEC void mpf_swap (mpf_ptr, mpf_ptr) __GMP_NOTHROW;

#define mpf_trunc __gmpf_trunc
__GMP_DECLSPEC void mpf_trunc (mpf_ptr, mpf_srcptr);

#define mpf_ui_div __gmpf_ui_div
__GMP_DECLSPEC void mpf_ui_div (mpf_ptr, unsigned long int, mpf_srcptr);

#define mpf_ui_sub __gmpf_ui_sub
__GMP_DECLSPEC void mpf_ui_sub (mpf_ptr, unsigned long int, mpf_srcptr);

#define mpf_urandomb __gmpf_urandomb
__GMP_DECLSPEC void mpf_urandomb (mpf_t, gmp_randstate_t, mp_bitcnt_t);


/************ Low level positive-integer (i.e. N) routines.  ************/

/* This is ugly, but we need to make user calls reach the prefixed function. */

#define mpn_add __MPN(add)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_add)
__GMP_DECLSPEC mp_limb_t mpn_add (mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t);
#endif

#define mpn_add_1 __MPN(add_1)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_add_1)
__GMP_DECLSPEC mp_limb_t mpn_add_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t) __GMP_NOTHROW;
#endif

#define mpn_add_n __MPN(add_n)
__GMP_DECLSPEC mp_limb_t mpn_add_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);

#define mpn_addmul_1 __MPN(addmul_1)
__GMP_DECLSPEC mp_limb_t mpn_addmul_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_cmp __MPN(cmp)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_cmp)
__GMP_DECLSPEC int mpn_cmp (mp_srcptr, mp_srcptr, mp_size_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpn_zero_p __MPN(zero_p)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_zero_p)
__GMP_DECLSPEC int mpn_zero_p (mp_srcptr, mp_size_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;
#endif

#define mpn_divexact_1 __MPN(divexact_1)
__GMP_DECLSPEC void mpn_divexact_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_divexact_by3(dst,src,size) \
  mpn_divexact_by3c (dst, src, size, __GMP_CAST (mp_limb_t, 0))

#define mpn_divexact_by3c __MPN(divexact_by3c)
__GMP_DECLSPEC mp_limb_t mpn_divexact_by3c (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_divmod_1(qp,np,nsize,dlimb) \
  mpn_divrem_1 (qp, __GMP_CAST (mp_size_t, 0), np, nsize, dlimb)

#define mpn_divrem __MPN(divrem)
__GMP_DECLSPEC mp_limb_t mpn_divrem (mp_ptr, mp_size_t, mp_ptr, mp_size_t, mp_srcptr, mp_size_t);

#define mpn_divrem_1 __MPN(divrem_1)
__GMP_DECLSPEC mp_limb_t mpn_divrem_1 (mp_ptr, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_divrem_2 __MPN(divrem_2)
__GMP_DECLSPEC mp_limb_t mpn_divrem_2 (mp_ptr, mp_size_t, mp_ptr, mp_size_t, mp_srcptr);

#define mpn_div_qr_1 __MPN(div_qr_1)
__GMP_DECLSPEC mp_limb_t mpn_div_qr_1 (mp_ptr, mp_limb_t *, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_div_qr_2 __MPN(div_qr_2)
__GMP_DECLSPEC mp_limb_t mpn_div_qr_2 (mp_ptr, mp_ptr, mp_srcptr, mp_size_t, mp_srcptr);

#define mpn_gcd __MPN(gcd)
__GMP_DECLSPEC mp_size_t mpn_gcd (mp_ptr, mp_ptr, mp_size_t, mp_ptr, mp_size_t);

#define mpn_gcd_11 __MPN(gcd_11)
__GMP_DECLSPEC mp_limb_t mpn_gcd_11 (mp_limb_t, mp_limb_t) __GMP_ATTRIBUTE_PURE;

#define mpn_gcd_1 __MPN(gcd_1)
__GMP_DECLSPEC mp_limb_t mpn_gcd_1 (mp_srcptr, mp_size_t, mp_limb_t) __GMP_ATTRIBUTE_PURE;

#define mpn_gcdext_1 __MPN(gcdext_1)
__GMP_DECLSPEC mp_limb_t mpn_gcdext_1 (mp_limb_signed_t *, mp_limb_signed_t *, mp_limb_t, mp_limb_t);

#define mpn_gcdext __MPN(gcdext)
__GMP_DECLSPEC mp_size_t mpn_gcdext (mp_ptr, mp_ptr, mp_size_t *, mp_ptr, mp_size_t, mp_ptr, mp_size_t);

#define mpn_get_str __MPN(get_str)
__GMP_DECLSPEC size_t mpn_get_str (unsigned char *, int, mp_ptr, mp_size_t);

#define mpn_hamdist __MPN(hamdist)
__GMP_DECLSPEC mp_bitcnt_t mpn_hamdist (mp_srcptr, mp_srcptr, mp_size_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpn_lshift __MPN(lshift)
__GMP_DECLSPEC mp_limb_t mpn_lshift (mp_ptr, mp_srcptr, mp_size_t, unsigned int);

#define mpn_mod_1 __MPN(mod_1)
__GMP_DECLSPEC mp_limb_t mpn_mod_1 (mp_srcptr, mp_size_t, mp_limb_t) __GMP_ATTRIBUTE_PURE;

#define mpn_mul __MPN(mul)
__GMP_DECLSPEC mp_limb_t mpn_mul (mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t);

#define mpn_mul_1 __MPN(mul_1)
__GMP_DECLSPEC mp_limb_t mpn_mul_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_mul_n __MPN(mul_n)
__GMP_DECLSPEC void mpn_mul_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);

#define mpn_sqr __MPN(sqr)
__GMP_DECLSPEC void mpn_sqr (mp_ptr, mp_srcptr, mp_size_t);

#define mpn_neg __MPN(neg)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_neg)
__GMP_DECLSPEC mp_limb_t mpn_neg (mp_ptr, mp_srcptr, mp_size_t);
#endif

#define mpn_com __MPN(com)
__GMP_DECLSPEC void mpn_com (mp_ptr, mp_srcptr, mp_size_t);

#define mpn_perfect_square_p __MPN(perfect_square_p)
__GMP_DECLSPEC int mpn_perfect_square_p (mp_srcptr, mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_perfect_power_p __MPN(perfect_power_p)
__GMP_DECLSPEC int mpn_perfect_power_p (mp_srcptr, mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_popcount __MPN(popcount)
__GMP_DECLSPEC mp_bitcnt_t mpn_popcount (mp_srcptr, mp_size_t) __GMP_NOTHROW __GMP_ATTRIBUTE_PURE;

#define mpn_pow_1 __MPN(pow_1)
__GMP_DECLSPEC mp_size_t mpn_pow_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t, mp_ptr);

/* undocumented now, but retained here for upward compatibility */
#define mpn_preinv_mod_1 __MPN(preinv_mod_1)
__GMP_DECLSPEC mp_limb_t mpn_preinv_mod_1 (mp_srcptr, mp_size_t, mp_limb_t, mp_limb_t) __GMP_ATTRIBUTE_PURE;

#define mpn_random __MPN(random)
__GMP_DECLSPEC void mpn_random (mp_ptr, mp_size_t);

#define mpn_random2 __MPN(random2)
__GMP_DECLSPEC void mpn_random2 (mp_ptr, mp_size_t);

#define mpn_rshift __MPN(rshift)
__GMP_DECLSPEC mp_limb_t mpn_rshift (mp_ptr, mp_srcptr, mp_size_t, unsigned int);

#define mpn_scan0 __MPN(scan0)
__GMP_DECLSPEC mp_bitcnt_t mpn_scan0 (mp_srcptr, mp_bitcnt_t) __GMP_ATTRIBUTE_PURE;

#define mpn_scan1 __MPN(scan1)
__GMP_DECLSPEC mp_bitcnt_t mpn_scan1 (mp_srcptr, mp_bitcnt_t) __GMP_ATTRIBUTE_PURE;

#define mpn_set_str __MPN(set_str)
__GMP_DECLSPEC mp_size_t mpn_set_str (mp_ptr, const unsigned char *, size_t, int);

#define mpn_sizeinbase __MPN(sizeinbase)
__GMP_DECLSPEC size_t mpn_sizeinbase (mp_srcptr, mp_size_t, int);

#define mpn_sqrtrem __MPN(sqrtrem)
__GMP_DECLSPEC mp_size_t mpn_sqrtrem (mp_ptr, mp_ptr, mp_srcptr, mp_size_t);

#define mpn_sub __MPN(sub)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_sub)
__GMP_DECLSPEC mp_limb_t mpn_sub (mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t);
#endif

#define mpn_sub_1 __MPN(sub_1)
#if __GMP_INLINE_PROTOTYPES || defined (__GMP_FORCE_mpn_sub_1)
__GMP_DECLSPEC mp_limb_t mpn_sub_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t) __GMP_NOTHROW;
#endif

#define mpn_sub_n __MPN(sub_n)
__GMP_DECLSPEC mp_limb_t mpn_sub_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);

#define mpn_submul_1 __MPN(submul_1)
__GMP_DECLSPEC mp_limb_t mpn_submul_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t);

#define mpn_tdiv_qr __MPN(tdiv_qr)
__GMP_DECLSPEC void mpn_tdiv_qr (mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t);

#define mpn_and_n __MPN(and_n)
__GMP_DECLSPEC void mpn_and_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_andn_n __MPN(andn_n)
__GMP_DECLSPEC void mpn_andn_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_nand_n __MPN(nand_n)
__GMP_DECLSPEC void mpn_nand_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_ior_n __MPN(ior_n)
__GMP_DECLSPEC void mpn_ior_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_iorn_n __MPN(iorn_n)
__GMP_DECLSPEC void mpn_iorn_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_nior_n __MPN(nior_n)
__GMP_DECLSPEC void mpn_nior_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_xor_n __MPN(xor_n)
__GMP_DECLSPEC void mpn_xor_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_xnor_n __MPN(xnor_n)
__GMP_DECLSPEC void mpn_xnor_n (mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);

#define mpn_copyi __MPN(copyi)
__GMP_DECLSPEC void mpn_copyi (mp_ptr, mp_srcptr, mp_size_t);
#define mpn_copyd __MPN(copyd)
__GMP_DECLSPEC void mpn_copyd (mp_ptr, mp_srcptr, mp_size_t);
#define mpn_zero __MPN(zero)
__GMP_DECLSPEC void mpn_zero (mp_ptr, mp_size_t);

#define mpn_cnd_add_n __MPN(cnd_add_n)
__GMP_DECLSPEC mp_limb_t mpn_cnd_add_n (mp_limb_t, mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);
#define mpn_cnd_sub_n __MPN(cnd_sub_n)
__GMP_DECLSPEC mp_limb_t mpn_cnd_sub_n (mp_limb_t, mp_ptr, mp_srcptr, mp_srcptr, mp_size_t);

#define mpn_sec_add_1 __MPN(sec_add_1)
__GMP_DECLSPEC mp_limb_t mpn_sec_add_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t, mp_ptr);
#define mpn_sec_add_1_itch __MPN(sec_add_1_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_add_1_itch (mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_sec_sub_1 __MPN(sec_sub_1)
__GMP_DECLSPEC mp_limb_t mpn_sec_sub_1 (mp_ptr, mp_srcptr, mp_size_t, mp_limb_t, mp_ptr);
#define mpn_sec_sub_1_itch __MPN(sec_sub_1_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_sub_1_itch (mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_cnd_swap  __MPN(cnd_swap)
__GMP_DECLSPEC void mpn_cnd_swap (mp_limb_t, volatile mp_limb_t *, volatile mp_limb_t *, mp_size_t);

#define mpn_sec_mul __MPN(sec_mul)
__GMP_DECLSPEC void mpn_sec_mul (mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t, mp_ptr);
#define mpn_sec_mul_itch __MPN(sec_mul_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_mul_itch (mp_size_t, mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_sec_sqr __MPN(sec_sqr)
__GMP_DECLSPEC void mpn_sec_sqr (mp_ptr, mp_srcptr, mp_size_t, mp_ptr);
#define mpn_sec_sqr_itch __MPN(sec_sqr_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_sqr_itch (mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_sec_powm __MPN(sec_powm)
__GMP_DECLSPEC void mpn_sec_powm (mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_bitcnt_t, mp_srcptr, mp_size_t, mp_ptr);
#define mpn_sec_powm_itch __MPN(sec_powm_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_powm_itch (mp_size_t, mp_bitcnt_t, mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_sec_tabselect __MPN(sec_tabselect)
__GMP_DECLSPEC void mpn_sec_tabselect (volatile mp_limb_t *, volatile const mp_limb_t *, mp_size_t, mp_size_t, mp_size_t);

#define mpn_sec_div_qr __MPN(sec_div_qr)
__GMP_DECLSPEC mp_limb_t mpn_sec_div_qr (mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, mp_ptr);
#define mpn_sec_div_qr_itch __MPN(sec_div_qr_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_div_qr_itch (mp_size_t, mp_size_t) __GMP_ATTRIBUTE_PURE;
#define mpn_sec_div_r __MPN(sec_div_r)
__GMP_DECLSPEC void mpn_sec_div_r (mp_ptr, mp_size_t, mp_srcptr, mp_size_t, mp_ptr);
#define mpn_sec_div_r_itch __MPN(sec_div_r_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_div_r_itch (mp_size_t, mp_size_t) __GMP_ATTRIBUTE_PURE;

#define mpn_sec_invert __MPN(sec_invert)
__GMP_DECLSPEC int mpn_sec_invert (mp_ptr, mp_ptr, mp_srcptr, mp_size_t, mp_bitcnt_t, mp_ptr);
#define mpn_sec_invert_itch __MPN(sec_invert_itch)
__GMP_DECLSPEC mp_size_t mpn_sec_invert_itch (mp_size_t) __GMP_ATTRIBUTE_PURE;


/**************** mpz inlines ****************/

/* The following are provided as inlines where possible, but always exist as
   library functions too, for binary compatibility.

   Within gmp itself this inlining generally isn't relied on, since it
   doesn't get done for all compilers, whereas if something is worth
   inlining then it's worth arranging always.

   There are two styles of inlining here.  When the same bit of code is
   wanted for the inline as for the library version, then __GMP_FORCE_foo
   arranges for that code to be emitted and the __GMP_EXTERN_INLINE
   directive suppressed, eg. mpz_fits_uint_p.  When a different bit of code
   is wanted for the inline than for the library version, then
   __GMP_FORCE_foo arranges the inline to be suppressed, eg. mpz_abs.  */

#if defined (__GMP_EXTERN_INLINE) && ! defined (__GMP_FORCE_mpz_abs)
__GMP_EXTERN_INLINE void
mpz_abs (mpz_ptr __gmp_w, mpz_srcptr __gmp_u)
{
  if (__gmp_w != __gmp_u)
    mpz_set (__gmp_w, __gmp_u);
  __gmp_w->_mp_size = __GMP_ABS (__gmp_w->_mp_size);
}
#endif

#if GMP_NAIL_BITS == 0
#define __GMPZ_FITS_UTYPE_P(z,maxval)					\
  mp_size_t  __gmp_n = z->_mp_size;					\
  mp_ptr  __gmp_p = z->_mp_d;						\
  return (__gmp_n == 0 || (__gmp_n == 1 && __gmp_p[0] <= maxval));
#else
#define __GMPZ_FITS_UTYPE_P(z,maxval)					\
  mp_size_t  __gmp_n = z->_mp_size;					\
  mp_ptr  __gmp_p = z->_mp_d;						\
  return (__gmp_n == 0 || (__gmp_n == 1 && __gmp_p[0] <= maxval)	\
	  || (__gmp_n == 2 && __gmp_p[1] <= ((mp_limb_t) maxval >> GMP_NUMB_BITS)));
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_fits_uint_p)
#if ! defined (__GMP_FORCE_mpz_fits_uint_p)
__GMP_EXTERN_INLINE
#endif
int
mpz_fits_uint_p (mpz_srcptr __gmp_z) __GMP_NOTHROW
{
  __GMPZ_FITS_UTYPE_P (__gmp_z, UINT_MAX);
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_fits_ulong_p)
#if ! defined (__GMP_FORCE_mpz_fits_ulong_p)
__GMP_EXTERN_INLINE
#endif
int
mpz_fits_ulong_p (mpz_srcptr __gmp_z) __GMP_NOTHROW
{
  __GMPZ_FITS_UTYPE_P (__gmp_z, ULONG_MAX);
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_fits_ushort_p)
#if ! defined (__GMP_FORCE_mpz_fits_ushort_p)
__GMP_EXTERN_INLINE
#endif
int
mpz_fits_ushort_p (mpz_srcptr __gmp_z) __GMP_NOTHROW
{
  __GMPZ_FITS_UTYPE_P (__gmp_z, USHRT_MAX);
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_get_ui)
#if ! defined (__GMP_FORCE_mpz_get_ui)
__GMP_EXTERN_INLINE
#endif
unsigned long
mpz_get_ui (mpz_srcptr __gmp_z) __GMP_NOTHROW
{
  mp_ptr __gmp_p = __gmp_z->_mp_d;
  mp_size_t __gmp_n = __gmp_z->_mp_size;
  mp_limb_t __gmp_l = __gmp_p[0];
  /* This is a "#if" rather than a plain "if" so as to avoid gcc warnings
     about "<< GMP_NUMB_BITS" exceeding the type size, and to avoid Borland
     C++ 6.0 warnings about condition always true for something like
     "ULONG_MAX < GMP_NUMB_MASK".  */
#if GMP_NAIL_BITS == 0 || defined (_LONG_LONG_LIMB)
  /* limb==long and no nails, or limb==longlong, one limb is enough */
  return (__gmp_n != 0 ? __gmp_l : 0);
#else
  /* limb==long and nails, need two limbs when available */
  __gmp_n = __GMP_ABS (__gmp_n);
  if (__gmp_n <= 1)
    return (__gmp_n != 0 ? __gmp_l : 0);
  else
    return __gmp_l + (__gmp_p[1] << GMP_NUMB_BITS);
#endif
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_getlimbn)
#if ! defined (__GMP_FORCE_mpz_getlimbn)
__GMP_EXTERN_INLINE
#endif
mp_limb_t
mpz_getlimbn (mpz_srcptr __gmp_z, mp_size_t __gmp_n) __GMP_NOTHROW
{
  mp_limb_t  __gmp_result = 0;
  if (__GMP_LIKELY (__gmp_n >= 0 && __gmp_n < __GMP_ABS (__gmp_z->_mp_size)))
    __gmp_result = __gmp_z->_mp_d[__gmp_n];
  return __gmp_result;
}
#endif

#if defined (__GMP_EXTERN_INLINE) && ! defined (__GMP_FORCE_mpz_neg)
__GMP_EXTERN_INLINE void
mpz_neg (mpz_ptr __gmp_w, mpz_srcptr __gmp_u)
{
  if (__gmp_w != __gmp_u)
    mpz_set (__gmp_w, __gmp_u);
  __gmp_w->_mp_size = - __gmp_w->_mp_size;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_perfect_square_p)
#if ! defined (__GMP_FORCE_mpz_perfect_square_p)
__GMP_EXTERN_INLINE
#endif
int
mpz_perfect_square_p (mpz_srcptr __gmp_a)
{
  mp_size_t __gmp_asize;
  int       __gmp_result;

  __gmp_asize = __gmp_a->_mp_size;
  __gmp_result = (__gmp_asize >= 0);  /* zero is a square, negatives are not */
  if (__GMP_LIKELY (__gmp_asize > 0))
    __gmp_result = mpn_perfect_square_p (__gmp_a->_mp_d, __gmp_asize);
  return __gmp_result;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_popcount)
#if ! defined (__GMP_FORCE_mpz_popcount)
__GMP_EXTERN_INLINE
#endif
mp_bitcnt_t
mpz_popcount (mpz_srcptr __gmp_u) __GMP_NOTHROW
{
  mp_size_t      __gmp_usize;
  mp_bitcnt_t    __gmp_result;

  __gmp_usize = __gmp_u->_mp_size;
  __gmp_result = (__gmp_usize < 0 ? ~ __GMP_CAST (mp_bitcnt_t, 0) : __GMP_CAST (mp_bitcnt_t, 0));
  if (__GMP_LIKELY (__gmp_usize > 0))
    __gmp_result =  mpn_popcount (__gmp_u->_mp_d, __gmp_usize);
  return __gmp_result;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_set_q)
#if ! defined (__GMP_FORCE_mpz_set_q)
__GMP_EXTERN_INLINE
#endif
void
mpz_set_q (mpz_ptr __gmp_w, mpq_srcptr __gmp_u)
{
  mpz_tdiv_q (__gmp_w, mpq_numref (__gmp_u), mpq_denref (__gmp_u));
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpz_size)
#if ! defined (__GMP_FORCE_mpz_size)
__GMP_EXTERN_INLINE
#endif
size_t
mpz_size (mpz_srcptr __gmp_z) __GMP_NOTHROW
{
  return __GMP_ABS (__gmp_z->_mp_size);
}
#endif


/**************** mpq inlines ****************/

#if defined (__GMP_EXTERN_INLINE) && ! defined (__GMP_FORCE_mpq_abs)
__GMP_EXTERN_INLINE void
mpq_abs (mpq_ptr __gmp_w, mpq_srcptr __gmp_u)
{
  if (__gmp_w != __gmp_u)
    mpq_set (__gmp_w, __gmp_u);
  __gmp_w->_mp_num._mp_size = __GMP_ABS (__gmp_w->_mp_num._mp_size);
}
#endif

#if defined (__GMP_EXTERN_INLINE) && ! defined (__GMP_FORCE_mpq_neg)
__GMP_EXTERN_INLINE void
mpq_neg (mpq_ptr __gmp_w, mpq_srcptr __gmp_u)
{
  if (__gmp_w != __gmp_u)
    mpq_set (__gmp_w, __gmp_u);
  __gmp_w->_mp_num._mp_size = - __gmp_w->_mp_num._mp_size;
}
#endif


/**************** mpn inlines ****************/

/* The comments with __GMPN_ADD_1 below apply here too.

   The test for FUNCTION returning 0 should predict well.  If it's assumed
   {yp,ysize} will usually have a random number of bits then the high limb
   won't be full and a carry out will occur a good deal less than 50% of the
   time.

   ysize==0 isn't a documented feature, but is used internally in a few
   places.

   Producing cout last stops it using up a register during the main part of
   the calculation, though gcc (as of 3.0) on an "if (mpn_add (...))"
   doesn't seem able to move the true and false legs of the conditional up
   to the two places cout is generated.  */

#define __GMPN_AORS(cout, wp, xp, xsize, yp, ysize, FUNCTION, TEST)     \
  do {                                                                  \
    mp_size_t  __gmp_i;                                                 \
    mp_limb_t  __gmp_x;                                                 \
                                                                        \
    /* ASSERT ((ysize) >= 0); */                                        \
    /* ASSERT ((xsize) >= (ysize)); */                                  \
    /* ASSERT (MPN_SAME_OR_SEPARATE2_P (wp, xsize, xp, xsize)); */      \
    /* ASSERT (MPN_SAME_OR_SEPARATE2_P (wp, xsize, yp, ysize)); */      \
                                                                        \
    __gmp_i = (ysize);                                                  \
    if (__gmp_i != 0)                                                   \
      {                                                                 \
        if (FUNCTION (wp, xp, yp, __gmp_i))                             \
          {                                                             \
            do                                                          \
              {                                                         \
                if (__gmp_i >= (xsize))                                 \
                  {                                                     \
                    (cout) = 1;                                         \
                    goto __gmp_done;                                    \
                  }                                                     \
                __gmp_x = (xp)[__gmp_i];                                \
              }                                                         \
            while (TEST);                                               \
          }                                                             \
      }                                                                 \
    if ((wp) != (xp))                                                   \
      __GMPN_COPY_REST (wp, xp, xsize, __gmp_i);                        \
    (cout) = 0;                                                         \
  __gmp_done:                                                           \
    ;                                                                   \
  } while (0)

#define __GMPN_ADD(cout, wp, xp, xsize, yp, ysize)              \
  __GMPN_AORS (cout, wp, xp, xsize, yp, ysize, mpn_add_n,       \
               (((wp)[__gmp_i++] = (__gmp_x + 1) & GMP_NUMB_MASK) == 0))
#define __GMPN_SUB(cout, wp, xp, xsize, yp, ysize)              \
  __GMPN_AORS (cout, wp, xp, xsize, yp, ysize, mpn_sub_n,       \
               (((wp)[__gmp_i++] = (__gmp_x - 1) & GMP_NUMB_MASK), __gmp_x == 0))


/* The use of __gmp_i indexing is designed to ensure a compile time src==dst
   remains nice and clear to the compiler, so that __GMPN_COPY_REST can
   disappear, and the load/add/store gets a chance to become a
   read-modify-write on CISC CPUs.

   Alternatives:

   Using a pair of pointers instead of indexing would be possible, but gcc
   isn't able to recognise compile-time src==dst in that case, even when the
   pointers are incremented more or less together.  Other compilers would
   very likely have similar difficulty.

   gcc could use "if (__builtin_constant_p(src==dst) && src==dst)" or
   similar to detect a compile-time src==dst.  This works nicely on gcc
   2.95.x, it's not good on gcc 3.0 where __builtin_constant_p(p==p) seems
   to be always false, for a pointer p.  But the current code form seems
   good enough for src==dst anyway.

   gcc on x86 as usual doesn't give particularly good flags handling for the
   carry/borrow detection.  It's tempting to want some multi instruction asm
   blocks to help it, and this was tried, but in truth there's only a few
   instructions to save and any gain is all too easily lost by register
   juggling setting up for the asm.  */

#if GMP_NAIL_BITS == 0
#define __GMPN_AORS_1(cout, dst, src, n, v, OP, CB)		\
  do {								\
    mp_size_t  __gmp_i;						\
    mp_limb_t  __gmp_x, __gmp_r;                                \
								\
    /* ASSERT ((n) >= 1); */					\
    /* ASSERT (MPN_SAME_OR_SEPARATE_P (dst, src, n)); */	\
								\
    __gmp_x = (src)[0];						\
    __gmp_r = __gmp_x OP (v);                                   \
    (dst)[0] = __gmp_r;						\
    if (CB (__gmp_r, __gmp_x, (v)))                             \
      {								\
	(cout) = 1;						\
	for (__gmp_i = 1; __gmp_i < (n);)                       \
	  {							\
	    __gmp_x = (src)[__gmp_i];                           \
	    __gmp_r = __gmp_x OP 1;                             \
	    (dst)[__gmp_i] = __gmp_r;                           \
	    ++__gmp_i;						\
	    if (!CB (__gmp_r, __gmp_x, 1))                      \
	      {							\
		if ((src) != (dst))				\
		  __GMPN_COPY_REST (dst, src, n, __gmp_i);      \
		(cout) = 0;					\
		break;						\
	      }							\
	  }							\
      }								\
    else							\
      {								\
	if ((src) != (dst))					\
	  __GMPN_COPY_REST (dst, src, n, 1);			\
	(cout) = 0;						\
      }								\
  } while (0)
#endif

#if GMP_NAIL_BITS >= 1
#define __GMPN_AORS_1(cout, dst, src, n, v, OP, CB)		\
  do {								\
    mp_size_t  __gmp_i;						\
    mp_limb_t  __gmp_x, __gmp_r;				\
								\
    /* ASSERT ((n) >= 1); */					\
    /* ASSERT (MPN_SAME_OR_SEPARATE_P (dst, src, n)); */	\
								\
    __gmp_x = (src)[0];						\
    __gmp_r = __gmp_x OP (v);					\
    (dst)[0] = __gmp_r & GMP_NUMB_MASK;				\
    if (__gmp_r >> GMP_NUMB_BITS != 0)				\
      {								\
	(cout) = 1;						\
	for (__gmp_i = 1; __gmp_i < (n);)			\
	  {							\
	    __gmp_x = (src)[__gmp_i];				\
	    __gmp_r = __gmp_x OP 1;				\
	    (dst)[__gmp_i] = __gmp_r & GMP_NUMB_MASK;		\
	    ++__gmp_i;						\
	    if (__gmp_r >> GMP_NUMB_BITS == 0)			\
	      {							\
		if ((src) != (dst))				\
		  __GMPN_COPY_REST (dst, src, n, __gmp_i);	\
		(cout) = 0;					\
		break;						\
	      }							\
	  }							\
      }								\
    else							\
      {								\
	if ((src) != (dst))					\
	  __GMPN_COPY_REST (dst, src, n, 1);			\
	(cout) = 0;						\
      }								\
  } while (0)
#endif

#define __GMPN_ADDCB(r,x,y) ((r) < (y))
#define __GMPN_SUBCB(r,x,y) ((x) < (y))

#define __GMPN_ADD_1(cout, dst, src, n, v)	     \
  __GMPN_AORS_1(cout, dst, src, n, v, +, __GMPN_ADDCB)
#define __GMPN_SUB_1(cout, dst, src, n, v)	     \
  __GMPN_AORS_1(cout, dst, src, n, v, -, __GMPN_SUBCB)


/* Compare {xp,size} and {yp,size}, setting "result" to positive, zero or
   negative.  size==0 is allowed.  On random data usually only one limb will
   need to be examined to get a result, so it's worth having it inline.  */
#define __GMPN_CMP(result, xp, yp, size)                                \
  do {                                                                  \
    mp_size_t  __gmp_i;                                                 \
    mp_limb_t  __gmp_x, __gmp_y;                                        \
                                                                        \
    /* ASSERT ((size) >= 0); */                                         \
                                                                        \
    (result) = 0;                                                       \
    __gmp_i = (size);                                                   \
    while (--__gmp_i >= 0)                                              \
      {                                                                 \
        __gmp_x = (xp)[__gmp_i];                                        \
        __gmp_y = (yp)[__gmp_i];                                        \
        if (__gmp_x != __gmp_y)                                         \
          {                                                             \
            /* Cannot use __gmp_x - __gmp_y, may overflow an "int" */   \
            (result) = (__gmp_x > __gmp_y ? 1 : -1);                    \
            break;                                                      \
          }                                                             \
      }                                                                 \
  } while (0)


#if defined (__GMPN_COPY) && ! defined (__GMPN_COPY_REST)
#define __GMPN_COPY_REST(dst, src, size, start)                 \
  do {                                                          \
    /* ASSERT ((start) >= 0); */                                \
    /* ASSERT ((start) <= (size)); */                           \
    __GMPN_COPY ((dst)+(start), (src)+(start), (size)-(start)); \
  } while (0)
#endif

/* Copy {src,size} to {dst,size}, starting at "start".  This is designed to
   keep the indexing dst[j] and src[j] nice and simple for __GMPN_ADD_1,
   __GMPN_ADD, etc.  */
#if ! defined (__GMPN_COPY_REST)
#define __GMPN_COPY_REST(dst, src, size, start)                 \
  do {                                                          \
    mp_size_t __gmp_j;                                          \
    /* ASSERT ((size) >= 0); */                                 \
    /* ASSERT ((start) >= 0); */                                \
    /* ASSERT ((start) <= (size)); */                           \
    /* ASSERT (MPN_SAME_OR_SEPARATE_P (dst, src, size)); */     \
    __GMP_CRAY_Pragma ("_CRI ivdep");                           \
    for (__gmp_j = (start); __gmp_j < (size); __gmp_j++)        \
      (dst)[__gmp_j] = (src)[__gmp_j];                          \
  } while (0)
#endif

/* Enhancement: Use some of the smarter code from gmp-impl.h.  Maybe use
   mpn_copyi if there's a native version, and if we don't mind demanding
   binary compatibility for it (on targets which use it).  */

#if ! defined (__GMPN_COPY)
#define __GMPN_COPY(dst, src, size)   __GMPN_COPY_REST (dst, src, size, 0)
#endif


#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_add)
#if ! defined (__GMP_FORCE_mpn_add)
__GMP_EXTERN_INLINE
#endif
mp_limb_t
mpn_add (mp_ptr __gmp_wp, mp_srcptr __gmp_xp, mp_size_t __gmp_xsize, mp_srcptr __gmp_yp, mp_size_t __gmp_ysize)
{
  mp_limb_t  __gmp_c;
  __GMPN_ADD (__gmp_c, __gmp_wp, __gmp_xp, __gmp_xsize, __gmp_yp, __gmp_ysize);
  return __gmp_c;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_add_1)
#if ! defined (__GMP_FORCE_mpn_add_1)
__GMP_EXTERN_INLINE
#endif
mp_limb_t
mpn_add_1 (mp_ptr __gmp_dst, mp_srcptr __gmp_src, mp_size_t __gmp_size, mp_limb_t __gmp_n) __GMP_NOTHROW
{
  mp_limb_t  __gmp_c;
  __GMPN_ADD_1 (__gmp_c, __gmp_dst, __gmp_src, __gmp_size, __gmp_n);
  return __gmp_c;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_cmp)
#if ! defined (__GMP_FORCE_mpn_cmp)
__GMP_EXTERN_INLINE
#endif
int
mpn_cmp (mp_srcptr __gmp_xp, mp_srcptr __gmp_yp, mp_size_t __gmp_size) __GMP_NOTHROW
{
  int __gmp_result;
  __GMPN_CMP (__gmp_result, __gmp_xp, __gmp_yp, __gmp_size);
  return __gmp_result;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_zero_p)
#if ! defined (__GMP_FORCE_mpn_zero_p)
__GMP_EXTERN_INLINE
#endif
int
mpn_zero_p (mp_srcptr __gmp_p, mp_size_t __gmp_n) __GMP_NOTHROW
{
  /* if (__GMP_LIKELY (__gmp_n > 0)) */
    do {
      if (__gmp_p[--__gmp_n] != 0)
	return 0;
    } while (__gmp_n != 0);
  return 1;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_sub)
#if ! defined (__GMP_FORCE_mpn_sub)
__GMP_EXTERN_INLINE
#endif
mp_limb_t
mpn_sub (mp_ptr __gmp_wp, mp_srcptr __gmp_xp, mp_size_t __gmp_xsize, mp_srcptr __gmp_yp, mp_size_t __gmp_ysize)
{
  mp_limb_t  __gmp_c;
  __GMPN_SUB (__gmp_c, __gmp_wp, __gmp_xp, __gmp_xsize, __gmp_yp, __gmp_ysize);
  return __gmp_c;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_sub_1)
#if ! defined (__GMP_FORCE_mpn_sub_1)
__GMP_EXTERN_INLINE
#endif
mp_limb_t
mpn_sub_1 (mp_ptr __gmp_dst, mp_srcptr __gmp_src, mp_size_t __gmp_size, mp_limb_t __gmp_n) __GMP_NOTHROW
{
  mp_limb_t  __gmp_c;
  __GMPN_SUB_1 (__gmp_c, __gmp_dst, __gmp_src, __gmp_size, __gmp_n);
  return __gmp_c;
}
#endif

#if defined (__GMP_EXTERN_INLINE) || defined (__GMP_FORCE_mpn_neg)
#if ! defined (__GMP_FORCE_mpn_neg)
__GMP_EXTERN_INLINE
#endif
mp_limb_t
mpn_neg (mp_ptr __gmp_rp, mp_srcptr __gmp_up, mp_size_t __gmp_n)
{
  while (*__gmp_up == 0) /* Low zero limbs are unchanged by negation. */
    {
      *__gmp_rp = 0;
      if (!--__gmp_n) /* All zero */
	return 0;
      ++__gmp_up; ++__gmp_rp;
    }

  *__gmp_rp = (- *__gmp_up) & GMP_NUMB_MASK;

  if (--__gmp_n) /* Higher limbs get complemented. */
    mpn_com (++__gmp_rp, ++__gmp_up, __gmp_n);

  return 1;
}
#endif

#if defined (__cplusplus)
}
#endif


/* Allow faster testing for negative, zero, and positive.  */
#define mpz_sgn(Z) ((Z)->_mp_size < 0 ? -1 : (Z)->_mp_size > 0)
#define mpf_sgn(F) ((F)->_mp_size < 0 ? -1 : (F)->_mp_size > 0)
#define mpq_sgn(Q) ((Q)->_mp_num._mp_size < 0 ? -1 : (Q)->_mp_num._mp_size > 0)

/* When using GCC, optimize certain common comparisons.  */
#if defined (__GNUC__) && __GNUC__ >= 2
#define mpz_cmp_ui(Z,UI) \
  (__builtin_constant_p (UI) && (UI) == 0				\
   ? mpz_sgn (Z) : _mpz_cmp_ui (Z,UI))
#define mpz_cmp_si(Z,SI)						\
  (__builtin_constant_p ((SI) >= 0) && (SI) >= 0			\
   ? mpz_cmp_ui (Z, __GMP_CAST (unsigned long, SI))			\
   : _mpz_cmp_si (Z,SI))
#define mpq_cmp_ui(Q,NUI,DUI)					\
  (__builtin_constant_p (NUI) && (NUI) == 0 ? mpq_sgn (Q)	\
   : __builtin_constant_p ((NUI) == (DUI)) && (NUI) == (DUI)	\
   ? mpz_cmp (mpq_numref (Q), mpq_denref (Q))			\
   : _mpq_cmp_ui (Q,NUI,DUI))
#define mpq_cmp_si(q,n,d)				\
  (__builtin_constant_p ((n) >= 0) && (n) >= 0		\
   ? mpq_cmp_ui (q, __GMP_CAST (unsigned long, n), d)	\
   : _mpq_cmp_si (q, n, d))
#else
#define mpz_cmp_ui(Z,UI) _mpz_cmp_ui (Z,UI)
#define mpz_cmp_si(Z,UI) _mpz_cmp_si (Z,UI)
#define mpq_cmp_ui(Q,NUI,DUI) _mpq_cmp_ui (Q,NUI,DUI)
#define mpq_cmp_si(q,n,d)  _mpq_cmp_si(q,n,d)
#endif


/* Using "&" rather than "&&" means these can come out branch-free.  Every
   mpz_t has at least one limb allocated, so fetching the low limb is always
   allowed.  */
#define mpz_odd_p(z)   (((z)->_mp_size != 0) & __GMP_CAST (int, (z)->_mp_d[0]))
#define mpz_even_p(z)  (! mpz_odd_p (z))


/**************** C++ routines ****************/

#ifdef __cplusplus
__GMP_DECLSPEC_XX std::ostream& operator<< (std::ostream &, mpz_srcptr);
__GMP_DECLSPEC_XX std::ostream& operator<< (std::ostream &, mpq_srcptr);
__GMP_DECLSPEC_XX std::ostream& operator<< (std::ostream &, mpf_srcptr);
__GMP_DECLSPEC_XX std::istream& operator>> (std::istream &, mpz_ptr);
__GMP_DECLSPEC_XX std::istream& operator>> (std::istream &, mpq_ptr);
__GMP_DECLSPEC_XX std::istream& operator>> (std::istream &, mpf_ptr);
#endif


/* Source-level compatibility with GMP 2 and earlier. */
#define mpn_divmod(qp,np,nsize,dp,dsize) \
  mpn_divrem (qp, __GMP_CAST (mp_size_t, 0), np, nsize, dp, dsize)

/* Source-level compatibility with GMP 1.  */
#define mpz_mdiv	mpz_fdiv_q
#define mpz_mdivmod	mpz_fdiv_qr
#define mpz_mmod	mpz_fdiv_r
#define mpz_mdiv_ui	mpz_fdiv_q_ui
#define mpz_mdivmod_ui(q,r,n,d) \
  (((r) == 0) ? mpz_fdiv_q_ui (q,n,d) : mpz_fdiv_qr_ui (q,r,n,d))
#define mpz_mmod_ui(r,n,d) \
  (((r) == 0) ? mpz_fdiv_ui (n,d) : mpz_fdiv_r_ui (r,n,d))

/* Useful synonyms, but not quite compatible with GMP 1.  */
#define mpz_div		mpz_fdiv_q
#define mpz_divmod	mpz_fdiv_qr
#define mpz_div_ui	mpz_fdiv_q_ui
#define mpz_divmod_ui	mpz_fdiv_qr_ui
#define mpz_div_2exp	mpz_fdiv_q_2exp
#define mpz_mod_2exp	mpz_fdiv_r_2exp

enum
{
  GMP_ERROR_NONE = 0,
  GMP_ERROR_UNSUPPORTED_ARGUMENT = 1,
  GMP_ERROR_DIVISION_BY_ZERO = 2,
  GMP_ERROR_SQRT_OF_NEGATIVE = 4,
  GMP_ERROR_INVALID_ARGUMENT = 8
};

/* Define CC and CFLAGS which were used to build this version of GMP */
#define __GMP_CC "x86_64-linux-gnu-gcc"
#define __GMP_CFLAGS "-g -O2 -ffile-prefix-map=/root/mod20/gmp-6.2.1+dfsg=. -fstack-protector-strong -Wformat -Werror=format-security -O3"

/* Major version number is the value of __GNU_MP__ too, above. */
#define __GNU_MP_VERSION            6
#define __GNU_MP_VERSION_MINOR      2
#define __GNU_MP_VERSION_PATCHLEVEL 1
#define __GNU_MP_RELEASE (__GNU_MP_VERSION * 10000 + __GNU_MP_VERSION_MINOR * 100 + __GNU_MP_VERSION_PATCHLEVEL)

#define __GMP_H__
#endif /* __GMP_H__ */
