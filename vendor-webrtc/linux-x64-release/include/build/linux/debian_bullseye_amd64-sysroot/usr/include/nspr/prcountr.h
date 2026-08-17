/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prcountr_h___
#define prcountr_h___

/*----------------------------------------------------------------------------
** prcountr.h -- NSPR Instrumentation counters
**
** The NSPR Counter Feature provides a means to "count
** something." Counters can be dynamically defined, incremented,
** decremented, set, and deleted under application program
** control.
**
** The Counter Feature is intended to be used as instrumentation,
** not as operational data. If you need a counter for operational
** data, use native integral types.
**
** Counters are 32bit unsigned intergers. On overflow, a counter
** will wrap. No exception is recognized or reported.
**
** A counter can be dynamically created using a two level naming
** convention. A "handle" is returned when the counter is
** created. The counter can subsequently be addressed by its
** handle. An API is provided to get an existing counter's handle
** given the names with  which it was originally created.
** Similarly, a counter's name can be retrieved given its handle.
**
** The counter naming convention is a two-level hierarchy. The
** QName is the higher level of the hierarchy; RName is the
** lower level. RNames can be thought of as existing within a
** QName. The same RName can exist within multiple QNames. QNames
** are unique. The NSPR Counter is not a near-zero overhead
** feature. Application designers should be aware of
** serialization issues when using the Counter API. Creating a
** counter locks a large asset, potentially causing a stall. This
** suggest that applications should create counters at component
** initialization, for example, and not create and destroy them
** willy-nilly. ... You have been warned.
**
** Incrementing and Adding to counters uses atomic operations.
** The performance of these operations will vary from platform
** to platform. On platforms where atomic operations are not
** supported the overhead may be substantial.
**
** When traversing the counter database with FindNext functions,
** the instantaneous values of any given counter is that at the
** moment of extraction. The state of the entire counter database
** may not be viewed as atomic.
**
** The counter interface may be disabled (No-Op'd) at compile
** time. When DEBUG is defined at compile time, the Counter
** Feature is compiled into NSPR and applications invoking it.
** When DEBUG is not defined, the counter macros compile to
** nothing. To force the Counter Feature to be compiled into an
** optimized build, define FORCE_NSPR_COUNTERS at compile time
** for both NSPR and the application intending to use it.
**
** Application designers should use the macro form of the Counter
** Feature methods to minimize performance impact in optimized
** builds. The macros normally compile to nothing on optimized
** builds.
**
** Application designers should be aware of the effects of
** debug and optimized build differences when using result of the
** Counter Feature macros in expressions.
**
** The Counter Feature is thread-safe and SMP safe.
**
** /lth. 09-Jun-1998.
*/

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/*
** Opaque counter handle type.
** ... don't even think of looking in here.
**
*/
typedef void *  PRCounterHandle;

#define PRCOUNTER_NAME_MAX 31
#define PRCOUNTER_DESC_MAX 255



/* -----------------------------------------------------------------------
** FUNCTION: PR_DEFINE_COUNTER() -- Define a PRCounterHandle
**
** DESCRIPTION: PR_DEFINE_COUNTER() is used to define a counter
** handle.
**
*/
#define PR_DEFINE_COUNTER(name) PRCounterHandle name

/* -----------------------------------------------------------------------
** FUNCTION: PR_INIT_COUNTER_HANDLE() -- Set the value of a PRCounterHandle
**
** DESCRIPTION:
** PR_INIT_COUNTER_HANDLE() sets the value of a PRCounterHandle
** to value.
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_INIT_COUNTER_HANDLE(handle,value)\
    (handle) = (PRCounterHandle)(value)
#else
#define PR_INIT_COUNTER_HANDLE(handle,value)
#endif

/* -----------------------------------------------------------------------
** FUNCTION: PR_CreateCounter() -- Create a counter
**
** DESCRIPTION: PR_CreateCounter() creates a counter object and
** initializes it to zero.
**
** The macro form takes as its first argument the name of the
** PRCounterHandle to receive the handle returned from
** PR_CreateCounter().
**
** INPUTS:
**  qName: The QName for the counter object. The maximum length
** of qName is defined by PRCOUNTER_NAME_MAX
**
**  rName: The RName for the counter object. The maximum length
** of qName is defined by PRCOUNTER_NAME_MAX
**
**  descrioption: The description of the counter object. The
** maximum length of description is defined by
** PRCOUNTER_DESC_MAX.
**
** OUTPUTS:
**
** RETURNS:
**  PRCounterHandle.
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_CREATE_COUNTER(handle,qName,rName,description)\
   (handle) = PR_CreateCounter((qName),(rName),(description))
#else
#define PR_CREATE_COUNTER(handle,qName,rName,description)
#endif

NSPR_API(PRCounterHandle)
PR_CreateCounter(
    const char *qName,
    const char *rName,
    const char *description
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_DestroyCounter() -- Destroy a counter object.
**
** DESCRIPTION: PR_DestroyCounter() removes a counter and
** unregisters its handle from the counter database.
**
** INPUTS:
**  handle: the PRCounterHandle of the counter to be destroyed.
**
** OUTPUTS:
**  The counter is destroyed.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_DESTROY_COUNTER(handle) PR_DestroyCounter((handle))
#else
#define PR_DESTROY_COUNTER(handle)
#endif

NSPR_API(void)
PR_DestroyCounter(
    PRCounterHandle handle
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_GetCounterHandleFromName() -- Retreive a
** counter's handle give its name.
**
** DESCRIPTION: PR_GetCounterHandleFromName() retreives a
** counter's handle from the counter database, given the name
** the counter was originally created with.
**
** INPUTS:
**  qName: Counter's original QName.
**  rName: Counter's original RName.
**
** OUTPUTS:
**
** RETURNS:
**  PRCounterHandle or PRCounterError.
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_GET_COUNTER_HANDLE_FROM_NAME(handle,qName,rName)\
    (handle) = PR_GetCounterHandleFromName((qName),(rName))
#else
#define PR_GET_COUNTER_HANDLE_FROM_NAME(handle,qName,rName)
#endif

NSPR_API(PRCounterHandle)
PR_GetCounterHandleFromName(
    const char *qName,
    const char *rName
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_GetCounterNameFromHandle() -- Retreive a
** counter's name, given its handle.
**
** DESCRIPTION: PR_GetCounterNameFromHandle() retreives a
** counter's name given its handle.
**
** INPUTS:
**  qName: Where to store a pointer to qName.
**  rName: Where to store a pointer to rName.
**  description: Where to store a pointer to description.
**
** OUTPUTS: Pointers to the Counter Feature's copies of the names
** used when the counters were created.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_GET_COUNTER_NAME_FROM_HANDLE(handle,qName,rName,description)\
    PR_GetCounterNameFromHandle((handle),(qName),(rName),(description))
#else
#define PR_GET_COUNTER_NAME_FROM_HANDLE(handle,qName,rName,description )
#endif

NSPR_API(void)
PR_GetCounterNameFromHandle(
    PRCounterHandle handle,
    const char **qName,
    const char **rName,
    const char **description
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_IncrementCounter() -- Add one to the referenced
** counter.
**
** DESCRIPTION: Add one to the referenced counter.
**
** INPUTS:
**  handle: The PRCounterHandle of the counter to be incremented
**
** OUTPUTS: The counter is incrementd.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_INCREMENT_COUNTER(handle) PR_IncrementCounter(handle)
#else
#define PR_INCREMENT_COUNTER(handle)
#endif

NSPR_API(void)
PR_IncrementCounter(
    PRCounterHandle handle
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_DecrementCounter() -- Subtract one from the
** referenced counter
**
** DESCRIPTION: Subtract one from the referenced counter.
**
** INPUTS:
**  handle: The PRCounterHandle of the coutner to be
** decremented.
**
** OUTPUTS: the counter is decremented.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_DECREMENT_COUNTER(handle) PR_DecrementCounter(handle)
#else
#define PR_DECREMENT_COUNTER(handle)
#endif

NSPR_API(void)
PR_DecrementCounter(
    PRCounterHandle handle
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_AddToCounter() -- Add a value to a counter.
**
** DESCRIPTION: Add value to the counter referenced by handle.
**
** INPUTS:
**  handle: the PRCounterHandle of the counter to be added to.
**
**  value: the value to be added to the counter.
**
** OUTPUTS: new value for counter.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_ADD_TO_COUNTER(handle,value)\
    PR_AddToCounter((handle),(value))
#else
#define PR_ADD_TO_COUNTER(handle,value)
#endif

NSPR_API(void)
PR_AddToCounter(
    PRCounterHandle handle,
    PRUint32 value
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_SubtractFromCounter() -- A value is subtracted
** from a counter.
**
** DESCRIPTION:
** Subtract a value from a counter.
**
** INPUTS:
**  handle: the PRCounterHandle of the counter to be subtracted
** from.
**
**  value: the value to be subtracted from the counter.
**
** OUTPUTS: new value for counter
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_SUBTRACT_FROM_COUNTER(handle,value)\
    PR_SubtractFromCounter((handle),(value))
#else
#define PR_SUBTRACT_FROM_COUNTER(handle,value)
#endif

NSPR_API(void)
PR_SubtractFromCounter(
    PRCounterHandle handle,
    PRUint32 value
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_GetCounter() -- Retreive the value of a counter
**
** DESCRIPTION:
** Retreive the value of a counter.
**
** INPUTS:
**  handle: the PR_CounterHandle of the counter to be retreived
**
** OUTPUTS:
**
** RETURNS: The value of the referenced counter
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_GET_COUNTER(counter,handle)\
    (counter) = PR_GetCounter((handle))
#else
#define PR_GET_COUNTER(counter,handle) 0
#endif

NSPR_API(PRUint32)
PR_GetCounter(
    PRCounterHandle handle
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_SetCounter() -- Replace the content of counter
** with value.
**
** DESCRIPTION: The contents of the referenced counter are
** replaced by value.
**
** INPUTS:
**  handle: the PRCounterHandle of the counter whose contents
** are to be replaced.
**
**  value: the new value of the counter.
**
** OUTPUTS:
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_SET_COUNTER(handle,value) PR_SetCounter((handle),(value))
#else
#define PR_SET_COUNTER(handle,value)
#endif

NSPR_API(void)
PR_SetCounter(
    PRCounterHandle handle,
    PRUint32 value
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_FindNextCounterQname() -- Retreive the next QName counter
** handle iterator
**
** DESCRIPTION:
** PR_FindNextCounterQname() retreives the first or next Qname
** the counter data base, depending on the value of handle. When
** handle is NULL, the function attempts to retreive the first
** QName handle in the database. When handle is a handle previosly
** retreived QName handle, then the function attempts to retreive
** the next QName handle.
**
** INPUTS:
**  handle: PRCounterHandle or NULL.
**
** OUTPUTS: returned
**
** RETURNS: PRCounterHandle or NULL when no more QName counter
** handles are present.
**
** RESTRICTIONS:
**  A concurrent PR_CreateCounter() or PR_DestroyCounter() may
** cause unpredictable results.
**
** A PRCounterHandle returned from this function may only be used
** in another PR_FindNextCounterQname() function call; other
** operations may cause unpredictable results.
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_FIND_NEXT_COUNTER_QNAME(next,handle)\
    (next) = PR_FindNextCounterQname((handle))
#else
#define PR_FIND_NEXT_COUNTER_QNAME(next,handle) NULL
#endif

NSPR_API(PRCounterHandle)
PR_FindNextCounterQname(
    PRCounterHandle handle
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_FindNextCounterRname() -- Retreive the next RName counter
** handle iterator
**
** DESCRIPTION:
** PR_FindNextCounterRname() retreives the first or next RNname
** handle from the counter data base, depending on the
** value of handle. When handle is NULL, the function attempts to
** retreive the first RName handle in the database. When handle is
** a handle previosly retreived RName handle, then the function
** attempts to retreive the next RName handle.
**
** INPUTS:
**  handle: PRCounterHandle or NULL.
**  qhandle: PRCounterHandle of a previously aquired via
** PR_FIND_NEXT_QNAME_HANDLE()
**
** OUTPUTS: returned
**
** RETURNS: PRCounterHandle or NULL when no more RName counter
** handles are present.
**
** RESTRICTIONS:
**  A concurrent PR_CreateCounter() or PR_DestroyCounter() may
** cause unpredictable results.
**
** A PRCounterHandle returned from this function may only be used
** in another PR_FindNextCounterRname() function call; other
** operations may cause unpredictable results.
**
*/
#if defined(DEBUG) || defined(FORCE_NSPR_COUNTERS)
#define PR_FIND_NEXT_COUNTER_RNAME(next,rhandle,qhandle)\
    (next) = PR_FindNextCounterRname((rhandle),(qhandle))
#else
#define PR_FIND_NEXT_COUNTER_RNAME(next,rhandle,qhandle)
#endif

NSPR_API(PRCounterHandle)
PR_FindNextCounterRname(
    PRCounterHandle rhandle,
    PRCounterHandle qhandle
);

PR_END_EXTERN_C

#endif /* prcountr_h___ */
