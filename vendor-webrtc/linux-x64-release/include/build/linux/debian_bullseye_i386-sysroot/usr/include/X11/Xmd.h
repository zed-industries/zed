/***********************************************************

Copyright 1987, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.


Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/
#ifndef XMD_H
# define XMD_H 1
/*
 *  Xmd.h: MACHINE DEPENDENT DECLARATIONS.
 */

/*
 * Special per-machine configuration flags.
 */
# if defined(__sun) && defined(__SVR4)
#  include <sys/isa_defs.h> /* Solaris: defines _LP64 if necessary */
# endif

# if defined (_LP64) || defined(__LP64__) || \
     defined(__alpha) || defined(__alpha__) || \
     defined(__ia64__) || defined(ia64) || \
     defined(__sparc64__) || \
     defined(__s390x__) || \
     defined(__amd64__) || defined(amd64) || \
     defined(__powerpc64__)
#  if !defined(__ILP32__) /* amd64-x32 is 32bit */
#   define LONG64				/* 32/64-bit architecture */
#  endif /* !__ILP32__ */
# endif

/*
 * Definition of macro used to set constants for size of network structures;
 * machines with preprocessors that can't handle all of the sz_ symbols
 * can define this macro to be sizeof(x) if and only if their compiler doesn't
 * pad out structures (esp. the xTextElt structure which contains only two
 * one-byte fields).  Network structures should always define sz_symbols.
 *
 * The sz_ prefix is used instead of something more descriptive so that the
 * symbols are no more than 32 characters long (which causes problems for some
 * compilers and preprocessors).
 *
 * The extra indirection is to get macro arguments to expand correctly before
 * the concatenation, rather than afterward.
 */
# define _SIZEOF(x) sz_##x
# define SIZEOF(x) _SIZEOF(x)

/*
 * Bitfield suffixes for the protocol structure elements, if you
 * need them.  Note that bitfields are not guaranteed to be signed
 * (or even unsigned) according to ANSI C.
 */
# define B32 /* bitfield not needed on architectures with native 32-bit type */
# define B16 /* bitfield not needed on architectures with native 16-bit type */
# ifdef LONG64
typedef long INT64;
typedef int INT32;
# else
typedef long INT32;
# endif
typedef short INT16;

typedef signed char    INT8;

# ifdef LONG64
typedef unsigned long CARD64;
typedef unsigned int CARD32;
# else
typedef unsigned long long CARD64;
typedef unsigned long CARD32;
# endif
typedef unsigned short CARD16;
typedef unsigned char  CARD8;

typedef CARD32		BITS32;
typedef CARD16		BITS16;

typedef CARD8		BYTE;
typedef CARD8		BOOL;

/*
 * was definitions for sign-extending bitfields on architectures without
 * native types smaller than 64-bit, now just backwards compatibility
 */
# define cvtINT8toInt(val) (val)
# define cvtINT16toInt(val) (val)
# define cvtINT32toInt(val) (val)
# define cvtINT8toShort(val) (val)
# define cvtINT16toShort(val) (val)
# define cvtINT32toShort(val) (val)
# define cvtINT8toLong(val) (val)
# define cvtINT16toLong(val) (val)
# define cvtINT32toLong(val) (val)

/*
 * this version should leave result of type (t *), but that should only be
 * used when not in MUSTCOPY
 */
# define NEXTPTR(p,t) (((t *)(p)) + 1)

#endif /* XMD_H */
