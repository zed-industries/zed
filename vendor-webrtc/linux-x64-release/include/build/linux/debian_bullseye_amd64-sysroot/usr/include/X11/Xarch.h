#ifndef _XARCH_H_
# define _XARCH_H_

/*
 * Copyright 1997 Metro Link Incorporated
 *
 *                           All Rights Reserved
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the names of the above listed copyright holder(s)
 * not be used in advertising or publicity pertaining to distribution of
 * the software without specific, written prior permission.  The above listed
 * copyright holder(s) make(s) no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express or
 * implied warranty.
 *
 * THE ABOVE LISTED COPYRIGHT HOLDER(S) DISCLAIM(S) ALL WARRANTIES WITH REGARD
 * TO THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE ABOVE LISTED COPYRIGHT HOLDER(S) BE
 * LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY
 * DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER
 * IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */


/*
 * Determine the machine's byte order.
 */

/* See if it is set in the imake config first */
# ifdef X_BYTE_ORDER

#  define X_BIG_ENDIAN 4321
#  define X_LITTLE_ENDIAN 1234

# else

#  if defined(SVR4) || defined(__SVR4)
#   include <sys/types.h>
#   include <sys/byteorder.h>
#  elif defined(CSRG_BASED)
#   if defined(__NetBSD__) || defined(__OpenBSD__)
#    include <sys/types.h>
#   endif
#   include <machine/endian.h>
#  elif defined(linux)
#   if defined __STRICT_ANSI__
#    undef __STRICT_ANSI__
#    include <endian.h>
#    define __STRICT_ANSI__
#   else
#    include <endian.h>
#   endif
/* 'endian.h' might have been included before 'Xarch.h' */
#   if !defined(LITTLE_ENDIAN) && defined(__LITTLE_ENDIAN)
#    define LITTLE_ENDIAN __LITTLE_ENDIAN
#   endif
#   if !defined(BIG_ENDIAN) && defined(__BIG_ENDIAN)
#    define BIG_ENDIAN __BIG_ENDIAN
#   endif
#   if !defined(PDP_ENDIAN) && defined(__PDP_ENDIAN)
#    define PDP_ENDIAN __PDP_ENDIAN
#   endif
#   if !defined(BYTE_ORDER) && defined(__BYTE_ORDER)
#    define BYTE_ORDER __BYTE_ORDER
#   endif
#  endif

#  ifndef BYTE_ORDER
#   define LITTLE_ENDIAN 1234
#   define BIG_ENDIAN    4321

#   if defined(__sun) && defined(__SVR4)
#    include <sys/isa_defs.h>
#    ifdef _LITTLE_ENDIAN
#     define BYTE_ORDER LITTLE_ENDIAN
#    endif
#    ifdef _BIG_ENDIAN
#     define BYTE_ORDER BIG_ENDIAN
#    endif
#   endif /* sun */
#  endif /* BYTE_ORDER */

#  define X_BYTE_ORDER BYTE_ORDER
#  define X_BIG_ENDIAN BIG_ENDIAN
#  define X_LITTLE_ENDIAN LITTLE_ENDIAN

# endif /* not in imake config */

#endif /* _XARCH_H_ */
