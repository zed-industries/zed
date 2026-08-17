
/*--------------------------------------------------------------------*/
/*--- Header included by every tool C file.      pub_tool_basics.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2000-2017 Julian Seward 
      jseward@acm.org

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.
*/

#ifndef __PUB_TOOL_BASICS_H
#define __PUB_TOOL_BASICS_H

//--------------------------------------------------------------------
// PURPOSE: This header should be imported by every single C file in
// tools.  It contains the basic types and other things needed everywhere.
// There is no corresponding C file because this isn't a module
// containing executable code, it's all just declarations.
//--------------------------------------------------------------------

/* ---------------------------------------------------------------------
   Other headers to include
   ------------------------------------------------------------------ */

// VEX defines Char, UChar, Short, UShort, Int, UInt, Long, ULong, SizeT,
// Addr, Addr32, Addr64, HWord, HChar, Bool, False and True.
#include "libvex_basictypes.h"

// For varargs types
#include <stdarg.h>


/* ---------------------------------------------------------------------
   symbol prefixing
   ------------------------------------------------------------------ */
 
// All symbols externally visible from Valgrind are prefixed
// as specified here to avoid namespace conflict problems.
//
// VG_ is for symbols exported from modules.  ML_ (module-local) is
// for symbols which are not intended to be visible outside modules,
// but which cannot be declared as C 'static's since they need to be
// visible across C files within a given module.  It is a mistake for
// a ML_ name to appear in a pub_core_*.h or pub_tool_*.h file.
// Likewise it is a mistake for a VG_ name to appear in a priv_*.h
// file. 

#define VGAPPEND(str1,str2) str1##str2

#define VG_(str)    VGAPPEND(vgPlain_,          str)
#define ML_(str)    VGAPPEND(vgModuleLocal_,    str)


/* ---------------------------------------------------------------------
   builtin types
   ------------------------------------------------------------------ */

// By choosing the right types, we can get these right for 32-bit and 64-bit
// platforms without having to do any conditional compilation or anything.
// POSIX references:
// - http://www.opengroup.org/onlinepubs/009695399/basedefs/sys/types.h.html
// - http://www.opengroup.org/onlinepubs/009695399/basedefs/stddef.h.html
// 
// Size in bits on:                          32-bit archs   64-bit archs
//                                           ------------   ------------
typedef unsigned long          UWord;     // 32             64
typedef   signed long           Word;     // 32             64

// Our equivalent of POSIX 'ssize_t':
// - ssize_t is "used for a count of bytes or an error indication".
typedef  Word                 SSizeT;     // 32             64

// Our equivalent of POSIX 'ptrdiff_t':
// - ptrdiff_t is a "signed integer type of the result of subtracting two
//   pointers".
// We use it for memory offsets, eg. the offset into a memory block.
typedef  Word                 PtrdiffT;   // 32             64

// Our equivalent of POSIX 'off_t':
// - off_t is "used for file sizes".
// At one point we were using it for memory offsets, but PtrdiffT should be
// used in those cases.
// Nb: on Linux, off_t is a signed word-sized int.  On Darwin it's
// always a signed 64-bit int.  So we defined our own Off64T as well.
#if defined(VGO_linux) || defined(VGO_solaris)
typedef Word                   OffT;      // 32             64
#elif defined(VGO_darwin)
typedef Long                   OffT;      // 64             64
#else
#  error Unknown OS
#endif
typedef Long                 Off64T;      // 64             64

#if !defined(NULL)
#  define NULL ((void*)0)
#endif

/* This is just too useful to not have around the place somewhere. */
typedef  struct { UWord uw1; UWord uw2; }  UWordPair;


/* ---------------------------------------------------------------------
   non-builtin types
   ------------------------------------------------------------------ */

// These probably shouldn't be here, but moving them to their logical
// modules results in a lot more #includes...

/* ThreadIds are simply indices into the VG_(threads)[] array. */
typedef UInt ThreadId;


/* You need a debuginfo epoch in order to convert an address into any source
   level entity, since that conversion depends on what objects were mapped
   in at the time.  An epoch is simply a monotonically increasing counter,
   which we wrap up in a struct so as to enable the C type system to
   distinguish it from other kinds of numbers.  m_debuginfo holds and
   maintains the current epoch number. */
typedef  struct { UInt n; }  DiEpoch;

static inline DiEpoch DiEpoch_INVALID ( void ) {
   DiEpoch dep; dep.n = 0; return dep;
}

static inline Bool is_DiEpoch_INVALID ( DiEpoch dep ) {
   return dep.n == 0;
}


/* Many data structures need to allocate and release memory.
   The allocation/release functions must be provided by the caller.
   The Alloc_Fn_t function must allocate a chunk of memory of size szB.
   cc is the Cost Centre for this allocated memory. This constant string
   is used to provide Valgrind's heap profiling, activated by
   --profile-heap=no|yes.
   The corresponding Free_Fn_t frees the memory chunk p. */

typedef void* (*Alloc_Fn_t)       ( const HChar* cc, SizeT szB );
typedef void  (*Free_Fn_t)        ( void* p );

/* An abstraction of syscall return values.
   Linux/MIPS32 and Linux/MIPS64:
      When _isError == False, 
         _val and possible _valEx hold the return value.  Whether
         _valEx actually holds a valid value depends on which syscall
         this SysRes holds of the result of.
      When _isError == True,  
         _val holds the error code.

   Linux/other:
      When _isError == False, 
         _val holds the return value.
      When _isError == True,  
         _val holds the error code.

   Darwin:
      Interpretation depends on _mode:
      MACH, MDEP:
         these can never 'fail' (apparently).  The result of the
         syscall is a single host word, _wLO.
      UNIX:
         Can record a double-word error or a double-word result:
         When _mode is SysRes_UNIX_OK,  _wHI:_wLO holds the result.
         When _mode is SysRes_UNIX_ERR, _wHI:_wLO holds the error code.
         Probably the high word of an error is always ignored by
         userspace, but we have to record it, so that we can correctly
         update both {R,E}DX and {R,E}AX (in guest state) given a SysRes,
         if we're required to.

   Solaris:
      When _isError == False,
         _val and _val2 hold the return value.
      When _isError == True,
         _val holds the error code.
*/
#if defined(VGP_mips32_linux) || defined(VGP_mips64_linux)
typedef
   struct {
      Bool  _isError;
      RegWord _val;
      UWord _valEx;
   }
   SysRes;

#elif defined(VGO_linux) \
      && !defined(VGP_mips32_linux) && !defined(VGP_mips64_linux)
typedef
   struct {
      Bool  _isError;
      UWord _val;
   }
   SysRes;

#elif defined(VGO_darwin)
typedef
   enum { 
      SysRes_MACH=40,  // MACH, result is _wLO
      SysRes_MDEP,     // MDEP, result is _wLO
      SysRes_UNIX_OK,  // UNIX, success, result is _wHI:_wLO
      SysRes_UNIX_ERR  // UNIX, error,   error  is _wHI:_wLO
   }
   SysResMode;
typedef
   struct {
      UWord _wLO;
      UWord _wHI;
      SysResMode _mode;
   }
   SysRes;

#elif defined(VGO_solaris)
typedef
   struct {
      UWord _val;
      UWord _val2;
      Bool  _isError;
   }
   SysRes;

#else
#  error "Unknown OS"
#endif


/* ---- And now some basic accessor functions for it. ---- */

#if defined(VGP_mips32_linux) || defined(VGP_mips64_linux)

static inline Bool sr_isError ( SysRes sr ) {
   return sr._isError;
}
static inline RegWord sr_Res ( SysRes sr ) {
   return sr._isError ? 0 : sr._val;
}
static inline UWord sr_ResEx ( SysRes sr ) {
   return sr._isError ? 0 : sr._valEx;
}
static inline UWord sr_Err ( SysRes sr ) {
   return sr._isError ? sr._val : 0;
}
static inline Bool sr_EQ ( UInt sysno, SysRes sr1, SysRes sr2 ) {
   /* This uglyness of hardcoding syscall numbers is necessary to
      avoid having this header file be dependent on
      include/vki/vki-scnums-mips{32,64}-linux.h.  It seems pretty
      safe given that it is inconceivable that the syscall numbers
      for such simple syscalls would ever change.  To make it 
      really safe, coregrind/m_vkiscnums.c static-asserts that these
      syscall numbers haven't changed, so that the build wil simply
      fail if they ever do. */
#  if defined(VGP_mips32_linux)
   const UInt __nr_Linux = 4000;
   const UInt __nr_pipe  = __nr_Linux + 42;
   const UInt __nr_pipe2 = __nr_Linux + 328;
#  elif defined(VGP_mips64_linux) && defined(VGABI_N32)
   const UInt __nr_Linux = 6000;
   const UInt __nr_pipe  = __nr_Linux + 21;
   const UInt __nr_pipe2 = __nr_Linux + 291;
#  else
   const UInt __nr_Linux = 5000;
   const UInt __nr_pipe  = __nr_Linux + 21;
   const UInt __nr_pipe2 = __nr_Linux + 287;
#  endif
   Bool useEx = sysno == __nr_pipe || sysno == __nr_pipe2;
   return sr1._val == sr2._val
          && (useEx ? (sr1._valEx == sr2._valEx) : True)
          && sr1._isError == sr2._isError;
}

#elif defined(VGO_linux) \
      && !defined(VGP_mips32_linux) && !defined(VGP_mips64_linux)

static inline Bool sr_isError ( SysRes sr ) {
   return sr._isError;
}
static inline UWord sr_Res ( SysRes sr ) {
   return sr._isError ? 0 : sr._val;
}
static inline UWord sr_Err ( SysRes sr ) {
#if defined(VGP_nanomips_linux)
   return sr._isError ? -sr._val : 0;
#else
   return sr._isError ? sr._val : 0;
#endif
}
static inline Bool sr_EQ ( UInt sysno, SysRes sr1, SysRes sr2 ) {
   /* sysno is ignored for Linux/not-MIPS */
   return sr1._val == sr2._val
          && sr1._isError == sr2._isError;
}

#elif defined(VGO_darwin)

static inline Bool sr_isError ( SysRes sr ) {
   switch (sr._mode) {
      case SysRes_UNIX_ERR:
         return True;
      /* should check tags properly and assert here, but we can't here */
      case SysRes_MACH:
      case SysRes_MDEP:
      case SysRes_UNIX_OK:
      default:
         return False;
   }
}

static inline UWord sr_Res ( SysRes sr ) {
   switch (sr._mode) {
      case SysRes_MACH:
      case SysRes_MDEP:
      case SysRes_UNIX_OK:
         return sr._wLO;
      /* should assert, but we can't here */
      case SysRes_UNIX_ERR:
      default:
         return 0;
   }
}

static inline UWord sr_ResHI ( SysRes sr ) {
   switch (sr._mode) {
      case SysRes_UNIX_OK:
         return sr._wHI;
      /* should assert, but we can't here */
      case SysRes_MACH:
      case SysRes_MDEP:
      case SysRes_UNIX_ERR:
      default:
         return 0;
   }
}

static inline UWord sr_Err ( SysRes sr ) {
   switch (sr._mode) {
      case SysRes_UNIX_ERR:
         return sr._wLO;
      /* should assert, but we can't here */
      case SysRes_MACH:
      case SysRes_MDEP:
      case SysRes_UNIX_OK:
      default:
         return 0;
   }
}

static inline Bool sr_EQ ( UInt sysno, SysRes sr1, SysRes sr2 ) {
   /* sysno is ignored for Darwin */
   return sr1._mode == sr2._mode
          && sr1._wLO == sr2._wLO && sr1._wHI == sr2._wHI;
}

#elif defined(VGO_solaris)

static inline Bool sr_isError ( SysRes sr ) {
   return sr._isError;
}
static inline UWord sr_Res ( SysRes sr ) {
   return sr._isError ? 0 : sr._val;
}
static inline UWord sr_ResHI ( SysRes sr ) {
   return sr._isError ? 0 : sr._val2;
}
static inline UWord sr_Err ( SysRes sr ) {
   return sr._isError ? sr._val : 0;
}
static inline Bool sr_EQ ( UInt sysno, SysRes sr1, SysRes sr2 ) {
   /* sysno is ignored for Solaris */
   return sr1._val == sr2._val
       && sr1._isError == sr2._isError
       && (!sr1._isError) ? (sr1._val2 == sr2._val2) : True;
}

#else
#  error "Unknown OS"
#endif


/* ---------------------------------------------------------------------
   Miscellaneous (word size, endianness, regparmness, stringification)
   ------------------------------------------------------------------ */

/* Word size: this is going to be either 4 or 8. */
// It should probably be in m_machine.
#define VG_WORDSIZE VEX_HOST_WORDSIZE

/* Endianness */
#undef VG_BIGENDIAN
#undef VG_LITTLEENDIAN

#if defined(VGA_x86) || defined(VGA_amd64) || defined (VGA_arm) \
    || ((defined(VGA_mips32) || defined(VGA_mips64) || defined(VGA_nanomips)) \
    && defined (_MIPSEL)) || defined(VGA_arm64)  || defined(VGA_ppc64le)
#  define VG_LITTLEENDIAN 1
#elif defined(VGA_ppc32) || defined(VGA_ppc64be) || defined(VGA_s390x) \
      || ((defined(VGA_mips32) || defined(VGA_mips64) || defined(VGA_nanomips)) \
      && defined (_MIPSEB))
#  define VG_BIGENDIAN 1
#else
#  error Unknown arch
#endif

/* Offsetof */
#if !defined(offsetof)
#   define offsetof(type,memb) ((SizeT)(HWord)&((type*)0)->memb)
#endif

#if !defined(container_of)
#   define container_of(ptr, type, member) ((type *)((char *)(ptr) - offsetof(type, member)))
#endif

/* Alignment */
/* We use a prefix vg_ for vg_alignof as its behaviour slightly
   differs from the standard alignof/gcc defined __alignof__

   vg_alignof returns a "safe" alignement.
   "safe" is defined as the alignment chosen by the compiler in
   a struct made of a char followed by this type.

      Note that this is not necessarily the "preferred" alignment
      for a platform. This preferred alignment is returned by the gcc
       __alignof__ and by the standard (in recent standard) alignof.
      Compared to __alignof__, vg_alignof gives on some platforms (e.g.
      amd64, ppc32, ppc64) a bigger alignment for long double (16 bytes
      instead of 8).
      On some platforms (e.g. x86), vg_alignof gives a smaller alignment
      than __alignof__ for long long and double (4 bytes instead of 8). 
      If we want to have the "preferred" alignment for the basic types,
      then either we need to depend on gcc __alignof__, or on a (too)
      recent standard and compiler (implementing <stdalign.h>).
*/
#define vg_alignof(_type) (sizeof(struct {char c;_type _t;})-sizeof(_type))

/* Regparmness */
#if defined(VGA_x86)
#  define VG_REGPARM(n)            __attribute__((regparm(n)))
#elif defined(VGA_amd64) || defined(VGA_ppc32) \
      || defined(VGA_ppc64be) || defined(VGA_ppc64le) \
      || defined(VGA_arm) || defined(VGA_s390x) \
      || defined(VGA_mips32) || defined(VGA_mips64) \
      || defined(VGA_arm64) || defined(VGA_nanomips)
#  define VG_REGPARM(n)            /* */
#else
#  error Unknown arch
#endif

/* Macro games */
#define VG_STRINGIFZ(__str)  #__str
#define VG_STRINGIFY(__str)  VG_STRINGIFZ(__str)

// Where to send bug reports to.
#define VG_BUGS_TO "www.valgrind.org"

/* Branch prediction hints. */
#if defined(__GNUC__)
#  define LIKELY(x)   __builtin_expect(!!(x), 1)
#  define UNLIKELY(x) __builtin_expect(!!(x), 0)
#else
#  define LIKELY(x)   (x)
#  define UNLIKELY(x) (x)
#endif

// printf format string checking for gcc.
// This feature has been supported since at least gcc version 2.95.
// For more information about the format attribute, see
// http://gcc.gnu.org/onlinedocs/gcc-4.3.0/gcc/Function-Attributes.html.
#if defined(__GNUC__)
#define PRINTF_CHECK(x, y) __attribute__((format(__printf__, x, y)))
#else
#define PRINTF_CHECK(x, y)
#endif

// Macro to "cast" away constness (from type const T to type T) without
// GCC complaining about it. This macro should be used RARELY. 
// x is expected to have type const T
#define CONST_CAST(T,x)    \
   ({                      \
      union {              \
         const T in;      \
         T out;           \
      } var = { .in = x }; var.out;  \
   })

/* Some architectures (eg. mips, arm) do not support unaligned memory access
   by hardware, so GCC warns about suspicious situations. This macro could
   be used to avoid these warnings but only after careful examination. */
#define ASSUME_ALIGNED(D, x)                 \
   ({                                        \
      union {                                \
         void *in;                           \
         D out;                              \
      } var;                                 \
      var.in = (void *) (x); var.out;        \
   })

// Poor man's static assert
#define STATIC_ASSERT(x)  extern int VG_(VG_(VG_(unused)))[(x) ? 1 : -1] \
                                     __attribute__((unused))

#define VG_MAX(a,b) ((a) > (b) ? a : b)
#define VG_MIN(a,b) ((a) < (b) ? a : b)

#endif /* __PUB_TOOL_BASICS_H */

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
