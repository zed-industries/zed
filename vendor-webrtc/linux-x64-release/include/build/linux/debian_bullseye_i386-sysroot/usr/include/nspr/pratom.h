/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* GLOBAL FUNCTIONS:
** DESCRIPTION:
**     PR Atomic operations
*/

#ifndef pratom_h___
#define pratom_h___

#include "prtypes.h"
#include "prlock.h"

PR_BEGIN_EXTERN_C

/*
** FUNCTION: PR_AtomicIncrement
** DESCRIPTION:
**    Atomically increment a 32 bit value.
** INPUTS:
**    val:  a pointer to the value to increment
** RETURN:
**    the returned value is the result of the increment
*/
NSPR_API(PRInt32)   PR_AtomicIncrement(PRInt32 *val);

/*
** FUNCTION: PR_AtomicDecrement
** DESCRIPTION:
**    Atomically decrement a 32 bit value.
** INPUTS:
**    val:  a pointer to the value to decrement
** RETURN:
**    the returned value is the result of the decrement
*/
NSPR_API(PRInt32)   PR_AtomicDecrement(PRInt32 *val);

/*
** FUNCTION: PR_AtomicSet
** DESCRIPTION:
**    Atomically set a 32 bit value.
** INPUTS:
**    val: A pointer to a 32 bit value to be set
**    newval: The newvalue to assign to val
** RETURN:
**    Returns the prior value
*/
NSPR_API(PRInt32) PR_AtomicSet(PRInt32 *val, PRInt32 newval);

/*
** FUNCTION: PR_AtomicAdd
** DESCRIPTION:
**    Atomically add a 32 bit value.
** INPUTS:
**    ptr:  a pointer to the value to increment
**    val:  value to be added
** RETURN:
**    the returned value is the result of the addition
*/
NSPR_API(PRInt32)   PR_AtomicAdd(PRInt32 *ptr, PRInt32 val);

/*
** MACRO: PR_ATOMIC_INCREMENT
** MACRO: PR_ATOMIC_DECREMENT
** MACRO: PR_ATOMIC_SET
** MACRO: PR_ATOMIC_ADD
** DESCRIPTION:
**    Macro versions of the atomic operations.  They may be implemented
**    as compiler intrinsics.
**
** IMPORTANT NOTE TO NSPR MAINTAINERS:
**    Implement these macros with compiler intrinsics only on platforms
**    where the PR_AtomicXXX functions are truly atomic (i.e., where the
**    configuration macro _PR_HAVE_ATOMIC_OPS is defined).  Otherwise,
**    the macros and functions won't be compatible and can't be used
**    interchangeably.
*/
#if defined(_WIN32) && !defined(_WIN32_WCE) && \
    (!defined(_MSC_VER) || (_MSC_VER >= 1310))

#include <intrin.h>

#ifdef _MSC_VER
#pragma intrinsic(_InterlockedIncrement)
#pragma intrinsic(_InterlockedDecrement)
#pragma intrinsic(_InterlockedExchange)
#pragma intrinsic(_InterlockedExchangeAdd)
#endif

#define PR_ATOMIC_INCREMENT(val) _InterlockedIncrement((long volatile *)(val))
#define PR_ATOMIC_DECREMENT(val) _InterlockedDecrement((long volatile *)(val))
#define PR_ATOMIC_SET(val, newval) \
        _InterlockedExchange((long volatile *)(val), (long)(newval))
#define PR_ATOMIC_ADD(ptr, val) \
        (_InterlockedExchangeAdd((long volatile *)(ptr), (long)(val)) + (val))

#elif ((__GNUC__ > 4) || (__GNUC__ == 4 && __GNUC_MINOR__ >= 1)) && \
      ((defined(__APPLE__) && \
           (defined(__ppc__) || defined(__i386__) || defined(__x86_64__))) || \
       (defined(__linux__) && \
           ((defined(__i386__) && \
           defined(__GCC_HAVE_SYNC_COMPARE_AND_SWAP_4)) || \
           defined(__ia64__) || defined(__x86_64__) || \
           defined(__powerpc__) || \
           (defined(__arm__) && \
           defined(__GCC_HAVE_SYNC_COMPARE_AND_SWAP_4)) || \
           defined(__aarch64__) || defined(__alpha) || \
           (defined(__mips__) && \
           defined(__GCC_HAVE_SYNC_COMPARE_AND_SWAP_4)))))

/*
 * Because the GCC manual warns that some processors may support
 * reduced functionality of __sync_lock_test_and_set, we test for the
 * processors that we believe support a full atomic exchange operation.
 */

#define PR_ATOMIC_INCREMENT(val) __sync_add_and_fetch(val, 1)
#define PR_ATOMIC_DECREMENT(val) __sync_sub_and_fetch(val, 1)
#define PR_ATOMIC_SET(val, newval) __sync_lock_test_and_set(val, newval)
#define PR_ATOMIC_ADD(ptr, val) __sync_add_and_fetch(ptr, val)

#else

#define PR_ATOMIC_INCREMENT(val) PR_AtomicIncrement(val)
#define PR_ATOMIC_DECREMENT(val) PR_AtomicDecrement(val)
#define PR_ATOMIC_SET(val, newval) PR_AtomicSet(val, newval)
#define PR_ATOMIC_ADD(ptr, val) PR_AtomicAdd(ptr, val)

#endif

/*
** LIFO linked-list (stack)
*/
typedef struct PRStackElemStr PRStackElem;

struct PRStackElemStr {
    PRStackElem *prstk_elem_next;   /* next pointer MUST be at offset 0;
                                      assembly language code relies on this */
};

typedef struct PRStackStr PRStack;

/*
** FUNCTION: PR_CreateStack
** DESCRIPTION:
**    Create a stack, a LIFO linked list
** INPUTS:
**    stack_name:  a pointer to string containing the name of the stack
** RETURN:
**    A pointer to the created stack, if successful, else NULL.
*/
NSPR_API(PRStack *) PR_CreateStack(const char *stack_name);

/*
** FUNCTION: PR_StackPush
** DESCRIPTION:
**    Push an element on the top of the stack
** INPUTS:
**    stack:        pointer to the stack
**    stack_elem:   pointer to the stack element
** RETURN:
**    None
*/
NSPR_API(void)          PR_StackPush(PRStack *stack, PRStackElem *stack_elem);

/*
** FUNCTION: PR_StackPop
** DESCRIPTION:
**    Remove the element on the top of the stack
** INPUTS:
**    stack:        pointer to the stack
** RETURN:
**    A pointer to the stack element removed from the top of the stack,
**    if non-empty,
**    else NULL
*/
NSPR_API(PRStackElem *) PR_StackPop(PRStack *stack);

/*
** FUNCTION: PR_DestroyStack
** DESCRIPTION:
**    Destroy the stack
** INPUTS:
**    stack:        pointer to the stack
** RETURN:
**    PR_SUCCESS - if successfully deleted
**    PR_FAILURE - if the stack is not empty
**                  PR_GetError will return
**                      PR_INVALID_STATE_ERROR - stack is not empty
*/
NSPR_API(PRStatus)      PR_DestroyStack(PRStack *stack);

PR_END_EXTERN_C

#endif /* pratom_h___ */
