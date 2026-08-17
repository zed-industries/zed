/*

Copyright 1995, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.

*/
/*
 * The purpose of this header is to define the macros ALLOCATE_LOCAL and
 * DEALLOCATE_LOCAL appropriately for the platform being compiled on.
 * These macros are used to make fast, function-local memory allocations.
 * Their characteristics are as follows:
 *
 * void *ALLOCATE_LOCAL(size_t size)
 *    Returns a pointer to size bytes of memory, or NULL if the allocation
 *    failed.  The memory must be freed with DEALLOCATE_LOCAL before the
 *    function that made the allocation returns.  You should not ask for
 *    large blocks of memory with this function, since on many platforms
 *    the memory comes from the stack, which may have limited size.
 *
 * void DEALLOCATE_LOCAL(void *)
 *    Frees the memory allocated by ALLOCATE_LOCAL.  Omission of this
 *    step may be harmless on some platforms, but will result in
 *    memory leaks or worse on others.
 *
 * Before including this file, you should define two macros,
 * ALLOCATE_LOCAL_FALLBACK and DEALLOCATE_LOCAL_FALLBACK, that have the
 * same characteristics as ALLOCATE_LOCAL and DEALLOCATE_LOCAL.  The
 * header uses the fallbacks if it doesn't know a "better" way to define
 * ALLOCATE_LOCAL and DEALLOCATE_LOCAL.  Typical usage would be:
 *
 *    #define ALLOCATE_LOCAL_FALLBACK(_size) malloc(_size)
 *    #define DEALLOCATE_LOCAL_FALLBACK(_ptr) free(_ptr)
 *    #include "Xalloca.h"
 */

#ifndef XALLOCA_H
#define XALLOCA_H 1

#ifndef INCLUDE_ALLOCA_H
/* Need to add more here to match Imake *.cf's */
# if defined(HAVE_ALLOCA_H) || defined(__SUNPRO_C) || defined(__SUNPRO_CC)
#  define INCLUDE_ALLOCA_H
# endif
#endif

#ifdef INCLUDE_ALLOCA_H
#  include <alloca.h>
#endif

#ifndef NO_ALLOCA
/*
 * os-dependent definition of local allocation and deallocation
 * If you want something other than (DE)ALLOCATE_LOCAL_FALLBACK
 * for ALLOCATE/DEALLOCATE_LOCAL then you add that in here.
 */


#  ifdef __GNUC__
#    ifndef alloca
#      define alloca __builtin_alloca
#    endif /* !alloca */
#    define ALLOCATE_LOCAL(size) alloca((size_t)(size))
#  else /* ! __GNUC__ */

/*
 * warning: old mips alloca (pre 2.10) is unusable, new one is built in
 * Test is easy, the new one is named __builtin_alloca and comes
 * from alloca.h which #defines alloca.
 */
#      if defined(__sun) || defined(alloca)
/*
 * Some System V boxes extract alloca.o from /lib/libPW.a; if you
 * decide that you don't want to use alloca, you might want to fix it here.
 */
/* alloca might be a macro taking one arg (hi, Sun!), so give it one. */
#        if !defined(__cplusplus)
#          define __Xnullarg		/* as nothing */
           extern void *alloca(__Xnullarg);
#        endif
#        define ALLOCATE_LOCAL(size) alloca((size_t)(size))
#      endif /* who does alloca */
#  endif /* __GNUC__ */

#endif /* NO_ALLOCA */

#if !defined(ALLOCATE_LOCAL)
#  if defined(ALLOCATE_LOCAL_FALLBACK) && defined(DEALLOCATE_LOCAL_FALLBACK)
#    define ALLOCATE_LOCAL(_size)  ALLOCATE_LOCAL_FALLBACK(_size)
#    define DEALLOCATE_LOCAL(_ptr) DEALLOCATE_LOCAL_FALLBACK(_ptr)
#  else /* no fallbacks supplied; error */
#    define ALLOCATE_LOCAL(_size)  ALLOCATE_LOCAL_FALLBACK undefined!
#    define DEALLOCATE_LOCAL(_ptr) DEALLOCATE_LOCAL_FALLBACK undefined!
#  endif /* defined(ALLOCATE_LOCAL_FALLBACK && DEALLOCATE_LOCAL_FALLBACK) */
#else
#  if !defined(DEALLOCATE_LOCAL)
#    define DEALLOCATE_LOCAL(_ptr) do {} while(0)
#  endif
#endif /* defined(ALLOCATE_LOCAL) */

#endif /* XALLOCA_H */
