/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef NSSBASE_H
#define NSSBASE_H

/*
 * nssbase.h
 *
 * This header file contains the prototypes of the basic public
 * NSS routines.
 */

#ifndef NSSBASET_H
#include "nssbaset.h"
#endif /* NSSBASET_H */

PR_BEGIN_EXTERN_C

/*
 * NSSArena
 *
 * The public methods relating to this type are:
 *
 *  NSSArena_Create  -- constructor
 *  NSSArena_Destroy
 *  NSS_ZAlloc
 *  NSS_ZRealloc
 *  NSS_ZFreeIf
 */

/*
 * NSSArena_Create
 *
 * This routine creates a new memory arena.  This routine may return
 * NULL upon error, in which case it will have created an error stack.
 *
 * The top-level error may be one of the following values:
 *  NSS_ERROR_NO_MEMORY
 *
 * Return value:
 *  NULL upon error
 *  A pointer to an NSSArena upon success
 */

NSS_EXTERN NSSArena *NSSArena_Create(void);

extern const NSSError NSS_ERROR_NO_MEMORY;

/*
 * NSSArena_Destroy
 *
 * This routine will destroy the specified arena, freeing all memory
 * allocated from it.  This routine returns a PRStatus value; if
 * successful, it will return PR_SUCCESS.  If unsuccessful, it will
 * create an error stack and return PR_FAILURE.
 *
 * The top-level error may be one of the following values:
 *  NSS_ERROR_INVALID_ARENA
 *
 * Return value:
 *  PR_SUCCESS upon success
 *  PR_FAILURE upon failure
 */

NSS_EXTERN PRStatus NSSArena_Destroy(NSSArena *arena);

extern const NSSError NSS_ERROR_INVALID_ARENA;

/*
 * The error stack
 *
 * The public methods relating to the error stack are:
 *
 *  NSS_GetError
 *  NSS_GetErrorStack
 */

/*
 * NSS_GetError
 *
 * This routine returns the highest-level (most general) error set
 * by the most recent NSS library routine called by the same thread
 * calling this routine.
 *
 * This routine cannot fail.  It may return NSS_ERROR_NO_ERROR, which
 * indicates that the previous NSS library call did not set an error.
 *
 * Return value:
 *  0 if no error has been set
 *  A nonzero error number
 */

NSS_EXTERN NSSError NSS_GetError(void);

extern const NSSError NSS_ERROR_NO_ERROR;

/*
 * NSS_GetErrorStack
 *
 * This routine returns a pointer to an array of NSSError values,
 * containingthe entire sequence or "stack" of errors set by the most
 * recent NSS library routine called by the same thread calling this
 * routine.  NOTE: the caller DOES NOT OWN the memory pointed to by
 * the return value.  The pointer will remain valid until the calling
 * thread calls another NSS routine.  The lowest-level (most specific)
 * error is first in the array, and the highest-level is last.  The
 * array is zero-terminated.  This routine may return NULL upon error;
 * this indicates a low-memory situation.
 *
 * Return value:
 *  NULL upon error, which is an implied NSS_ERROR_NO_MEMORY
 *  A NON-caller-owned pointer to an array of NSSError values
 */

NSS_EXTERN NSSError *NSS_GetErrorStack(void);

/*
 * NSS_ZNEW
 *
 * This preprocessor macro will allocate memory for a new object
 * of the specified type with nss_ZAlloc, and will cast the
 * return value appropriately.  If the optional arena argument is
 * non-null, the memory will be obtained from that arena; otherwise,
 * the memory will be obtained from the heap.  This routine may
 * return NULL upon error, in which case it will have set an error
 * upon the error stack.
 *
 * The error may be one of the following values:
 *  NSS_ERROR_INVALID_ARENA
 *  NSS_ERROR_NO_MEMORY
 *
 * Return value:
 *  NULL upon error
 *  A pointer to the new segment of zeroed memory
 */

#define NSS_ZNEW(arenaOpt, type) ((type *)NSS_ZAlloc((arenaOpt), sizeof(type)))

/*
 * NSS_ZNEWARRAY
 *
 * This preprocessor macro will allocate memory for an array of
 * new objects, and will cast the return value appropriately.
 * If the optional arena argument is non-null, the memory will
 * be obtained from that arena; otherwise, the memory will be
 * obtained from the heap.  This routine may return NULL upon
 * error, in which case it will have set an error upon the error
 * stack.  The array size may be specified as zero.
 *
 * The error may be one of the following values:
 *  NSS_ERROR_INVALID_ARENA
 *  NSS_ERROR_NO_MEMORY
 *
 * Return value:
 *  NULL upon error
 *  A pointer to the new segment of zeroed memory
 */

#define NSS_ZNEWARRAY(arenaOpt, type, quantity) \
    ((type *)NSS_ZAlloc((arenaOpt), sizeof(type) * (quantity)))

/*
 * NSS_ZAlloc
 *
 * This routine allocates and zeroes a section of memory of the
 * size, and returns to the caller a pointer to that memory.  If
 * the optional arena argument is non-null, the memory will be
 * obtained from that arena; otherwise, the memory will be obtained
 * from the heap.  This routine may return NULL upon error, in
 * which case it will have set an error upon the error stack.  The
 * value specified for size may be zero; in which case a valid
 * zero-length block of memory will be allocated.  This block may
 * be expanded by calling NSS_ZRealloc.
 *
 * The error may be one of the following values:
 *  NSS_ERROR_INVALID_ARENA
 *  NSS_ERROR_NO_MEMORY
 *  NSS_ERROR_ARENA_MARKED_BY_ANOTHER_THREAD
 *
 * Return value:
 *  NULL upon error
 *  A pointer to the new segment of zeroed memory
 */

NSS_EXTERN void *NSS_ZAlloc(NSSArena *arenaOpt, PRUint32 size);

/*
 * NSS_ZRealloc
 *
 * This routine reallocates a block of memory obtained by calling
 * nss_ZAlloc or nss_ZRealloc.  The portion of memory
 * between the new and old sizes -- which is either being newly
 * obtained or released -- is in either case zeroed.  This routine
 * may return NULL upon failure, in which case it will have placed
 * an error on the error stack.
 *
 * The error may be one of the following values:
 *  NSS_ERROR_INVALID_POINTER
 *  NSS_ERROR_NO_MEMORY
 *  NSS_ERROR_ARENA_MARKED_BY_ANOTHER_THREAD
 *
 * Return value:
 *  NULL upon error
 *  A pointer to the replacement segment of memory
 */

NSS_EXTERN void *NSS_ZRealloc(void *pointer, PRUint32 newSize);

/*
 * NSS_ZFreeIf
 *
 * If the specified pointer is non-null, then the region of memory
 * to which it points -- which must have been allocated with
 * nss_ZAlloc -- will be zeroed and released.  This routine
 * returns a PRStatus value; if successful, it will return PR_SUCCESS.
 * If unsuccessful, it will set an error on the error stack and return
 * PR_FAILURE.
 *
 * The error may be one of the following values:
 *  NSS_ERROR_INVALID_POINTER
 *
 * Return value:
 *  PR_SUCCESS
 *  PR_FAILURE
 */

NSS_EXTERN PRStatus NSS_ZFreeIf(void *pointer);

PR_END_EXTERN_C

#endif /* NSSBASE_H */
