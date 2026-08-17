
/*--------------------------------------------------------------------*/
/*--- Standalone libc stuff.                   pub_tool_libcbase.h ---*/
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

#ifndef __PUB_TOOL_LIBCBASE_H
#define __PUB_TOOL_LIBCBASE_H

#include "pub_tool_basics.h"   // VG_ macro

/* ---------------------------------------------------------------------
   Char functions.
   ------------------------------------------------------------------ */

extern Bool  VG_(isspace) ( HChar c );
extern Bool  VG_(isdigit) ( HChar c );
extern HChar VG_(tolower) ( HChar c );

/* ---------------------------------------------------------------------
   Converting strings to numbers
   ------------------------------------------------------------------ */

// Convert strings to numbers according to various bases.  Leading
// whitespace is ignored.  A subsequent '-' or '+' is accepted.  For strtoll16,
// accepts an initial "0x" or "0X" prefix, but only if it's followed by a
// hex digit (if not, the '0' will be read and then it will stop on the
// "x"/"X".)  If 'endptr' isn't NULL, it gets filled in with the first
// non-digit char.  Returns 0 if no number could be converted, and 'endptr'
// is set to the start of the string.  None of them test that the number
// fits into 64 bits.
//
// Nb: we also don't provide VG_(atoll*);  these functions are worse than
// useless because they don't do any error checking and so accept malformed
// numbers and non-numbers -- eg. "123xyz" gives 123, and "foo" gives 0!
// If you really want that behaviour, you can use "VG_(strtoll10)(str, NULL)".
extern Long  VG_(strtoll10) ( const HChar* str, HChar** endptr );
extern Long  VG_(strtoll16) ( const HChar* str, HChar** endptr );
extern ULong  VG_(strtoull10) ( const HChar* str, HChar** endptr );
extern ULong  VG_(strtoull16) ( const HChar* str, HChar** endptr );

// Convert a string to a double.  After leading whitespace is ignored, a
// '+' or '-' is allowed, and then it accepts a non-empty sequence of
// decimal digits possibly containing a '.'.  Hexadecimal floats are not
// accepted, nor are "fancy" floats (eg. "3.4e-5", "NAN").
extern double VG_(strtod)  ( const HChar* str, HChar** endptr );

/* ---------------------------------------------------------------------
   String functions and macros
   ------------------------------------------------------------------ */

/* Use this for normal null-termination-style string comparison. */
#define VG_STREQ(s1,s2) ( (s1 != NULL && s2 != NULL \
                           && VG_(strcmp)((s1),(s2))==0) ? True : False )
#define VG_STREQN(n,s1,s2) ( (s1 != NULL && s2 != NULL \
                             && VG_(strncmp)((s1),(s2),(n))==0) ? True : False )

extern SizeT  VG_(strlen)         ( const HChar* str );
extern SizeT  VG_(strnlen)        ( const HChar* str, SizeT n );
extern HChar* VG_(strcat)         ( HChar* dest, const HChar* src );
extern HChar* VG_(strncat)        ( HChar* dest, const HChar* src, SizeT n );
extern HChar* VG_(strpbrk)        ( const HChar* s, const HChar* accpt );
extern HChar* VG_(strcpy)         ( HChar* dest, const HChar* src );
extern HChar* VG_(strncpy)        ( HChar* dest, const HChar* src, SizeT ndest );
extern SizeT  VG_(strlcpy)        ( HChar* dest, const HChar* src, SizeT n );
extern Int    VG_(strcmp)         ( const HChar* s1, const HChar* s2 );
extern Int    VG_(strcasecmp)     ( const HChar* s1, const HChar* s2 );
extern Int    VG_(strncmp)        ( const HChar* s1, const HChar* s2, SizeT nmax );
extern Int    VG_(strncasecmp)    ( const HChar* s1, const HChar* s2, SizeT nmax );
extern HChar* VG_(strstr)         ( const HChar* haystack, const HChar* needle );
extern HChar* VG_(strcasestr)     ( const HChar* haystack, const HChar* needle );
extern HChar* VG_(strchr)         ( const HChar* s, HChar c );
extern HChar* VG_(strrchr)        ( const HChar* s, HChar c );
extern SizeT  VG_(strspn)         ( const HChar* s, const HChar* accpt );
extern SizeT  VG_(strcspn)        ( const HChar* s, const HChar* reject );

/* strtok* functions and some parsing utilities. */
extern HChar* VG_(strtok_r)       (HChar* s, const HChar* delim, HChar** saveptr);
extern HChar* VG_(strtok)         (HChar* s, const HChar* delim);

/* Parse a 32- or 64-bit hex number, including leading 0x, from string
   starting at *ppc, putting result in *result, advance *ppc past the
   characters used, and return True.  Or fail, in which case *ppc and
   *result are undefined, and return False. */
extern Bool VG_(parse_Addr) ( const HChar** ppc, Addr* result );

/* Parse an unsigned 32 bit number, written using decimals only.
   Calling conventions are the same as for VG_(parse_Addr). */
extern Bool VG_(parse_UInt) ( const HChar** ppc, UInt* result );

/* Parse an "enum set" made of one or more words comma separated.
   The allowed word values are given in 'tokens', separated by comma.
   If a word in 'tokens' is found in 'input', the corresponding bit
   will be set in *enum_set (words in 'tokens' are numbered starting from 0).
   Using in 'tokens' the special token "-" (a minus character) indicates that
   the corresponding bit position cannot be set.
   In addition to the words specified in 'tokens', VG_(parse_enum_set)
   automatically accept the word "none" to indicate an empty enum_set (0).
   If allow_all, VG_(parse_enum_set) automatically accept the word "all"
   to indicate an enum_set with all bits corresponding to the words in tokens
    set.
   If "none" or "all" is present in 'input', no other word can be given
   in 'input'.
   If parsing is successful, returns True and sets *enum_set.
   If parsing fails, returns False. */
extern Bool VG_(parse_enum_set) ( const HChar *tokens,
                                  Bool  allow_all,
                                  const HChar *input,
                                  UInt *enum_set);

/* ---------------------------------------------------------------------
   mem* functions
   ------------------------------------------------------------------ */

extern void* VG_(memcpy) ( void *d, const void *s, SizeT sz );
extern void* VG_(memmove)( void *d, const void *s, SizeT sz );
extern void* VG_(memset) ( void *s, Int c, SizeT sz );
extern Int   VG_(memcmp) ( const void* s1, const void* s2, SizeT n );

/* Zero out up to 12 words quickly in-line.  Do not use this for blocks
   of size which are unknown at compile time, since the whole point is
   for it to be inlined, and then for gcc to remove all code except
   for the relevant 'sz' case. */
inline __attribute__((always_inline))
static void VG_(bzero_inline) ( void* s, SizeT sz )
{
   if (LIKELY(0 == (((Addr)sz) & (Addr)(sizeof(UWord)-1)))
       && LIKELY(0 == (((Addr)s) & (Addr)(sizeof(UWord)-1)))) {
      UWord* p = (UWord*)s;
      switch (sz / (SizeT)sizeof(UWord)) {
          case 12: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = p[6] = p[7] 
                  = p[8] = p[9] = p[10] = p[11] = 0UL; return;
          case 11: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = p[6] = p[7] 
                  = p[8] = p[9] = p[10] = 0UL; return;
          case 10: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = p[6] = p[7] 
                  = p[8] = p[9] = 0UL; return;
          case 9: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = p[6] = p[7] 
                  = p[8] = 0UL; return;
          case 8: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = p[6] = p[7] = 0UL; return;
          case 7: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = p[6] = 0UL; return;
          case 6: p[0] = p[1] = p[2] = p[3]
                  = p[4] = p[5] = 0UL; return;
          case 5: p[0] = p[1] = p[2] = p[3] = p[4] = 0UL; return;
          case 4: p[0] = p[1] = p[2] = p[3] = 0UL; return;
          case 3: p[0] = p[1] = p[2] = 0UL; return;
          case 2: p[0] = p[1] = 0UL; return;
          case 1: p[0] = 0UL; return;
          case 0: return;
          default: break;
      }
   }
   VG_(memset)(s, 0, sz);
}


/* ---------------------------------------------------------------------
   Address computation helpers
   ------------------------------------------------------------------ */

// Check if an address/whatever is aligned
#define VG_IS_2_ALIGNED(aaa_p)    (0 == (((Addr)(aaa_p)) & ((Addr)0x1)))
#define VG_IS_4_ALIGNED(aaa_p)    (0 == (((Addr)(aaa_p)) & ((Addr)0x3)))
#define VG_IS_8_ALIGNED(aaa_p)    (0 == (((Addr)(aaa_p)) & ((Addr)0x7)))
#define VG_IS_16_ALIGNED(aaa_p)   (0 == (((Addr)(aaa_p)) & ((Addr)0xf)))
#define VG_IS_32_ALIGNED(aaa_p)   (0 == (((Addr)(aaa_p)) & ((Addr)0x1f)))
#define VG_IS_64_ALIGNED(aaa_p)   (0 == (((Addr)(aaa_p)) & ((Addr)0x3f)))
#define VG_IS_WORD_ALIGNED(aaa_p) (0 == (((Addr)(aaa_p)) & ((Addr)(sizeof(Addr)-1))))
#define VG_IS_PAGE_ALIGNED(aaa_p) (0 == (((Addr)(aaa_p)) & ((Addr)(VKI_PAGE_SIZE-1))))

// 'a' -- the alignment -- must be a power of 2.
// The latter two require the vki-*.h header to be imported also.
#define VG_ROUNDDN(p, a)   ((Addr)(p) & ~((Addr)(a)-1))
#define VG_ROUNDUP(p, a)   VG_ROUNDDN((p)+(a)-1, (a))
#define VG_PGROUNDDN(p)    VG_ROUNDDN(p, VKI_PAGE_SIZE)
#define VG_PGROUNDUP(p)    VG_ROUNDUP(p, VKI_PAGE_SIZE)

/* Converts `Device ID` given as pair of 32-bit values (dev_major, dev_minor)
 * to 64-bit dev_t using MMMM Mmmm mmmM MMmm encoding. This is
 * downward compatible with legacy systems where dev_t is 16 bits wide,
 * encoded as MMmm. It is also downward compatible with the Linux kernel,
 * which uses 32-bit dev_t, encoded as mmmM MMmm.
 * Original macro can be found in bits/sysmacros.h. */
#define VG_MAKEDEV(__major, __minor)            \
   ((((ULong) (__major & 0x00000fffu)) <<  8) | \
    (((ULong) (__major & 0xfffff000u)) << 32) | \
    (((ULong) (__minor & 0x000000ffu)) <<  0) | \
    (((ULong) (__minor & 0xffffff00u)) << 12))

/* ---------------------------------------------------------------------
   Misc useful functions
   ------------------------------------------------------------------ */

/* Like qsort().  The name VG_(ssort) is for historical reasons -- it used
 * to be a shell sort, but is now a quicksort. */
extern void VG_(ssort)( void* base, SizeT nmemb, SizeT size,
                        Int (*compar)(const void*, const void*) );

/* Returns the base-2 logarithm of a 32 bit unsigned number.  Returns
 -1 if it is not a power of two.  Nb: VG_(log2)(1) == 0. */
extern Int VG_(log2) ( UInt x );

/* Ditto for 64 bit unsigned numbers. */
extern Int VG_(log2_64)( ULong x );

// A pseudo-random number generator returning a random UInt.  If pSeed
// is NULL, it uses its own seed, which starts at zero.  If pSeed is
// non-NULL, it uses and updates whatever pSeed points at.
extern UInt VG_(random) ( /*MOD*/UInt* pSeed );

/* Update a running Adler-32 checksum with the bytes buf[0..len-1] and
   return the updated checksum. If buf is NULL, this function returns
   the required initial value for the checksum. An Adler-32 checksum is
   almost as reliable as a CRC32 but can be computed much faster. */
extern UInt VG_(adler32)( UInt adler, const UChar* buf, UInt len);

#endif   // __PUB_TOOL_LIBCBASE_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
