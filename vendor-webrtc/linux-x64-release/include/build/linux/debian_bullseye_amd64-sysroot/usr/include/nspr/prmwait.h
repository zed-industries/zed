/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#if defined(_PRMWAIT_H)
#else
#define _PRMWAIT_H

#include "prio.h"
#include "prtypes.h"
#include "prclist.h"

PR_BEGIN_EXTERN_C

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/
/******************************       WARNING        ****************************/
/********************************************************************************/
/**************************** This is work in progress. *************************/
/************************** Do not make any assumptions *************************/
/************************** about the stability of this *************************/
/************************** API or the underlying imple- ************************/
/************************** mentation.                   ************************/
/********************************************************************************/
/********************************************************************************/

/*
** STRUCTURE:   PRWaitGroup
** DESCRIPTION:
**      The client may define several wait groups in order to semantically
**      tie a collection of file descriptors for a single purpose. This allows
**      easier dispatching of threads that returned with active file descriptors
**      from the wait function.
*/
typedef struct PRWaitGroup PRWaitGroup;

/*
** ENUMERATION: PRMWStatus
** DESCRIPTION:
**      This enumeration is used to indicate the completion status of
**      a receive wait object. Generally stated, a positive value indicates
**      that the operation is not yet complete. A zero value indicates
**      success (similar to PR_SUCCESS) and any negative value is an
**      indication of failure. The reason for the failure can be retrieved
**      by calling PR_GetError().
**
**  PR_MW_PENDING       The operation is still pending. None of the other
**                      fields of the object are currently valid.
**  PR_MW_SUCCESS       The operation is complete and it was successful.
**  PR_MW_FAILURE       The operation failed. The reason for the failure
**                      can be retrieved by calling PR_GetError().
**  PR_MW_TIMEOUT       The amount of time allowed for by the object's
**                      'timeout' field has expired w/o the operation
**                      otherwise coming to closure.
**  PR_MW_INTERRUPT     The operation was cancelled, either by the client
**                      calling PR_CancelWaitFileDesc() or destroying the
**                      entire wait group (PR_DestroyWaitGroup()).
*/
typedef enum PRMWStatus
{
    PR_MW_PENDING = 1,
    PR_MW_SUCCESS = 0,
    PR_MW_FAILURE = -1,
    PR_MW_TIMEOUT = -2,
    PR_MW_INTERRUPT = -3
} PRMWStatus;

/*
** STRUCTURE:   PRMemoryDescriptor
** DESCRIPTION:
**      THis is a descriptor for an interval of memory. It contains a
**      pointer to the first byte of that memory and the length (in
**      bytes) of the interval.
*/
typedef struct PRMemoryDescriptor
{
    void *start;                /* pointer to first byte of memory */
    PRSize length;              /* length (in bytes) of memory interval */
} PRMemoryDescriptor;

/*
** STRUCTURE:   PRMWaitClientData
** DESCRIPTION:
**      An opague stucture for which a client MAY give provide a concrete
**      definition and associate with a receive descriptor. The NSPR runtime
**      does not manage this field. It is completely up to the client.
*/
typedef struct PRMWaitClientData PRMWaitClientData;

/*
** STRUCTURE:   PRRecvWait
** DESCRIPTION:
**      A receive wait object contains the file descriptor that is subject
**      to the wait and the amount of time (beginning epoch established
**      when the object is presented to the runtime) the the channel should
**      block before abandoning the process.
**
**      The success of the wait operation will be noted in the object's
**      'outcome' field. The fields are not valid when the NSPR runtime
**      is in possession of the object.
**
**      The memory descriptor describes an interval of writable memory
**      in the caller's address space where data from an initial read
**      can be placed. The description may indicate a null interval.
*/
typedef struct PRRecvWait
{
    PRCList internal;           /* internal runtime linkages */

    PRFileDesc *fd;             /* file descriptor associated w/ object */
    PRMWStatus outcome;         /* outcome of the current/last operation */
    PRIntervalTime timeout;     /* time allowed for entire operation */

    PRInt32 bytesRecv;          /* number of bytes transferred into buffer */
    PRMemoryDescriptor buffer;  /* where to store first segment of input data */
    PRMWaitClientData *client;  /* pointer to arbitrary client defined data */
} PRRecvWait;

/*
** STRUCTURE:   PRMWaitEnumerator
** DESCRIPTION:
**      An enumeration object is used to store the state of an existing
**      enumeration over a wait group. The opaque object must be allocated
**      by the client and the reference presented on each call to the
**      pseudo-stateless enumerator. The enumeration objects are sharable
**      only in serial fashion.
*/
typedef struct PRMWaitEnumerator PRMWaitEnumerator;


/*
** FUNCTION:    PR_AddWaitFileDesc
** DESCRIPTION:
**      This function will effectively add a file descriptor to the
**      list of those waiting for network receive. The new descriptor
**      will be semantically tied to the wait group specified.
**
**      The ownership for the storage pointed to by 'desc' is temporarily
**      passed over the the NSPR runtime. It will be handed back by the
**      function PR_WaitRecvReady().
**
**  INPUTS
**      group       A reference to a PRWaitGroup or NULL. Wait groups are
**                  created by calling PR_CreateWaitGroup() and are used
**                  to semantically group various file descriptors by the
**                  client's application.
**      desc        A reference to a valid PRRecvWait. The object of the
**                  reference must be preserved and not be modified
**                  until its ownership is returned to the client.
**  RETURN
**      PRStatus    An indication of success. If equal to PR_FAILUE details
**                  of the failure are avaiable via PR_GetError().
**
**  ERRORS
**      PR_INVALID_ARGUMENT_ERROR
**                  Invalid 'group' identifier or duplicate 'desc' object.
**      PR_OUT_OF_MEMORY_ERROR
**                  Insuffient memory for internal data structures.
**      PR_INVALID_STATE_ERROR
**                  The group is being destroyed.
*/
NSPR_API(PRStatus) PR_AddWaitFileDesc(PRWaitGroup *group, PRRecvWait *desc);

/*
** FUNCTION:    PR_WaitRecvReady
** DESCRIPTION:
**      PR_WaitRecvReady will block the calling thread until one of the
**      file descriptors that have been added via PR_AddWaitFileDesc is
**      available for input I/O.
**  INPUT
**      group       A pointer to a valid PRWaitGroup or NULL (the null
**                  group. The function will block the caller until a
**                  channel from the wait group becomes ready for receive
**                  or there is some sort of error.
**  RETURN
**      PRReciveWait
**                  When the caller is resumed it is either returned a
**                  valid pointer to a previously added receive wait or
**                  a NULL. If the latter, the function has terminated
**                  for a reason that can be determined by calling
**                  PR_GetError().
**                  If a valid pointer is returned, the reference is to the
**                  file descriptor contained in the receive wait object.
**                  The outcome of the wait operation may still fail, and
**                  if it has, that fact will be noted in the object's
**                  outcome field. Details can be retrieved from PR_GetError().
**
**  ERRORS
**      PR_INVALID_ARGUMENT_ERROR
**                  The 'group' is not known by the runtime.
**      PR_PENDING_INTERRUPT_ERROR
                    The thread was interrupted.
**      PR_INVALID_STATE_ERROR
**                  The group is being destroyed.
*/
NSPR_API(PRRecvWait*) PR_WaitRecvReady(PRWaitGroup *group);

/*
** FUNCTION:    PR_CancelWaitFileDesc
** DESCRIPTION:
**      PR_CancelWaitFileDesc is provided as a means for cancelling operations
**      on objects previously submitted by use of PR_AddWaitFileDesc(). If
**      the runtime knows of the object, it will be marked as having failed
**      because it was interrupted (similar to PR_Interrupt()). The first
**      available thread waiting on the group will be made to return the
**      PRRecvWait object with the outcome noted.
**
**  INPUTS
**      group       The wait group under which the wait receive object was
**                  added.
**      desc        A pointer to the wait receive object that is to be
**                  cancelled.
**  RETURN
**      PRStatus    If the wait receive object was located and associated
**                  with the specified wait group, the status returned will
**                  be PR_SUCCESS. There is still a race condition that would
**                  permit the offected object to complete normally, but it
**                  is assured that it will complete in the near future.
**                  If the receive object or wait group are invalid, the
**                  function will return with a status of PR_FAILURE.
**
**  ERRORS
**      PR_INVALID_ARGUMENT_ERROR
**                  The 'group' argument is not recognized as a valid group.
**      PR_COLLECTION_EMPTY_ERROR
**                  There are no more receive wait objects in the group's
**                  collection.
**      PR_INVALID_STATE_ERROR
**                  The group is being destroyed.
*/
NSPR_API(PRStatus) PR_CancelWaitFileDesc(PRWaitGroup *group, PRRecvWait *desc);

/*
** FUNCTION:    PR_CancelWaitGroup
** DESCRIPTION:
**      PR_CancelWaitGroup is provided as a means for cancelling operations
**      on objects previously submitted by use of PR_AddWaitFileDesc(). Each
**      successive call will return a pointer to a PRRecvWait object that
**      was previously registered via PR_AddWaitFileDesc(). If no wait
**      objects are associated with the wait group, a NULL will be returned.
**      This function should be called in a loop until a NULL is returned
**      to reclaim all the wait objects prior to calling PR_DestroyWaitGroup().
**
**  INPUTS
**      group       The wait group under which the wait receive object was
**                  added.
**  RETURN
**      PRRecvWait* If the wait group is valid and at least one receive wait
**                  object is present in the group, that object will be
**                  marked as PR_MW_INTERRUPT'd and removed from the group's
**                  queues. Otherwise a NULL will be returned and the reason
**                  for the NULL may be retrieved by calling PR_GetError().
**
**  ERRORS
**      PR_INVALID_ARGUMENT_ERROR
**      PR_GROUP_EMPTY_ERROR
*/
NSPR_API(PRRecvWait*) PR_CancelWaitGroup(PRWaitGroup *group);

/*
** FUNCTION:    PR_CreateWaitGroup
** DESCRIPTION:
**      A wait group is an opaque object that a client may create in order
**      to semantically group various wait requests. Each wait group is
**      unique, including the default wait group (NULL). A wait request
**      that was added under a wait group will only be serviced by a caller
**      that specified the same wait group.
**
**  INPUT
**      size        The size of the hash table to be used to contain the
**                  receive wait objects. This is just the initial size.
**                  It will grow as it needs to, but to avoid that hassle
**                  one can suggest a suitable size initially. It should
**                  be ~30% larger than the maximum number of receive wait
**                  objects expected.
**  RETURN
**      PRWaitGroup If successful, the function will return a pointer to an
**                  object that was allocated by and owned by the runtime.
**                  The reference remains valid until it is explicitly destroyed
**                  by calling PR_DestroyWaitGroup().
**
**  ERRORS
**      PR_OUT_OF_MEMORY_ERROR
*/
NSPR_API(PRWaitGroup*) PR_CreateWaitGroup(PRInt32 size);

/*
** FUNCTION:    PR_DestroyWaitGroup
** DESCRIPTION:
**      Undo the effects of PR_CreateWaitGroup(). Any receive wait operations
**      on the group will be treated as if the each had been the target of a
**      PR_CancelWaitFileDesc().
**
**  INPUT
**      group       Reference to a wait group previously allocated using
**                  PR_CreateWaitGroup().
**  RETURN
**      PRStatus    Will be PR_SUCCESS if the wait group was valid and there
**                  are no receive wait objects in that group. Otherwise
**                  will indicate PR_FAILURE.
**
**  ERRORS
**      PR_INVALID_ARGUMENT_ERROR
**                  The 'group' argument does not reference a known object.
**      PR_INVALID_STATE_ERROR
**                  The group still contains receive wait objects.
*/
NSPR_API(PRStatus) PR_DestroyWaitGroup(PRWaitGroup *group);

/*
** FUNCTION:    PR_CreateMWaitEnumerator
** DESCRIPTION:
**      The PR_CreateMWaitEnumerator() function returns a reference to an
**      opaque PRMWaitEnumerator object. The enumerator object is required
**      as an argument for each successive call in the stateless enumeration
**      of the indicated wait group.
**
**      group       The wait group that the enumeration is intended to
**                  process. It may be be the default wait group (NULL).
** RETURN
**      PRMWaitEnumerator* group
**                  A reference to an object that will be used to store
**                  intermediate state of enumerations.
** ERRORS
**      Errors are indicated by the function returning a NULL.
**      PR_INVALID_ARGUMENT_ERROR
**                  The 'group' argument does not reference a known object.
**      PR_OUT_OF_MEMORY_ERROR
*/
NSPR_API(PRMWaitEnumerator*) PR_CreateMWaitEnumerator(PRWaitGroup *group);

/*
** FUNCTION:    PR_DestroyMWaitEnumerator
** DESCRIPTION:
**      Destroys the object created by PR_CreateMWaitEnumerator(). The reference
**      used as an argument becomes invalid.
**
** INPUT
**      PRMWaitEnumerator* enumerator
**          The PRMWaitEnumerator object to destroy.
** RETURN
**      PRStatus
**          PR_SUCCESS if successful, PR_FAILURE otherwise.
** ERRORS
**      PR_INVALID_ARGUMENT_ERROR
**                  The enumerator is invalid.
*/
NSPR_API(PRStatus) PR_DestroyMWaitEnumerator(PRMWaitEnumerator* enumerator);

/*
** FUNCTION:    PR_EnumerateWaitGroup
** DESCRIPTION:
**      PR_EnumerateWaitGroup is a thread safe enumerator over a wait group.
**      Each call to the enumerator must present a valid PRMWaitEnumerator
**      rererence and a pointer to the "previous" element returned from the
**      enumeration process or a NULL.
**
**      An enumeration is started by passing a NULL as the "previous" value.
**      Subsequent calls to the enumerator must pass in the result of the
**      previous call. The enumeration end is signaled by the runtime returning
**      a NULL as the result.
**
**      Modifications to the content of the wait group are allowed during
**      an enumeration. The effect is that the enumeration may have to be
**      "reset" and that may result in duplicates being returned from the
**      enumeration.
**
**      An enumeration may be abandoned at any time. The runtime is not
**      keeping any state, so there are no issues in that regard.
*/
NSPR_API(PRRecvWait*) PR_EnumerateWaitGroup(
    PRMWaitEnumerator *enumerator, const PRRecvWait *previous);

PR_END_EXTERN_C

#endif /* defined(_PRMWAIT_H) */

/* prmwait.h */
