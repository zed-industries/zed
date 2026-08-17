/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prtrace_h___
#define prtrace_h___
/*
** prtrace.h -- NSPR's Trace Facility.
**
** The Trace Facility provides a means to trace application
** program events within a process. When implementing an
** application program an engineer may insert a "Trace" function
** call, passing arguments to be traced. The "Trace" function
** combines the user trace data with identifying data and
** writes this data in time ordered sequence into a circular
** in-memory buffer; when the buffer fills, it wraps.
**
** Functions are provided to set and/or re-configure the size of
** the trace buffer, control what events are recorded in the
** buffer, enable and disable tracing based on specific user
** supplied data and other control functions. Methods are provided
** to record the trace entries in the in-memory trace buffer to
** a file.
**
** Tracing may cause a performance degredation to the application
** depending on the number and placement of calls to the tracing
** facility. When tracing is compiled in and all tracing is
** disabled via the runtime controls, the overhead should be
** minimal. ... Famous last words, eh?
**
** When DEBUG is defined at compile time, the Trace Facility is
** compiled as part of NSPR and any application using NSPR's
** header files will have tracing compiled in. When DEBUG is not
** defined, the Trace Facility is not compiled into NSPR nor
** exported in its header files.  If the Trace Facility is
** desired in a non-debug build, then FORCE_NSPR_TRACE may be
** defined at compile time for both the optimized build of NSPR
** and the application. NSPR and any application using  NSPR's
** Trace Facility must be compiled with the same level of trace
** conditioning or unresolved references may be realized at link
** time.
**
** For any of the Trace Facility methods that requires a trace
** handle as an input argument, the caller must ensure that the
** trace handle argument is valid. An invalid trace handle
** argument may cause unpredictable results.
**
** Trace Facility methods are thread-safe and SMP safe.
**
** Users of the Trace Facility should use the defined macros to
** invoke trace methods, not the function calls directly. e.g.
** PR_TRACE( h1,0,1,2, ...); not PR_Trace(h1,0,1,2, ...);
**
** Application designers should be aware of the effects of
** debug and optimized build differences when using result of the
** Trace Facility macros in expressions.
**
** See Also: prcountr.h
**
** /lth. 08-Jun-1998.
*/

#include "prtypes.h"
#include "prthread.h"
#include "prtime.h"

PR_BEGIN_EXTERN_C

/*
** Opaque type for the trace handle
** ... Don't even think about looking in here.
**
*/
typedef void *  PRTraceHandle;

/*
** PRTraceEntry -- A trace entry in the in-memory trace buffer
** looks like this.
**
*/
typedef struct PRTraceEntry
{
    PRThread        *thread;        /* The thread creating the trace entry */
    PRTraceHandle   handle;         /* PRTraceHandle creating the trace entry */
    PRTime          time;           /* Value of PR_Now() at time of trace entry */
    PRUint32        userData[8];    /* user supplied trace data */
} PRTraceEntry;

/*
** PRTraceOption -- command operands to
** PR_[Set|Get]TraceOption(). See descriptive meanings there.
**
*/
typedef enum PRTraceOption
{
    PRTraceBufSize,
    PRTraceEnable,
    PRTraceDisable,
    PRTraceSuspend,
    PRTraceResume,
    PRTraceSuspendRecording,
    PRTraceResumeRecording,
    PRTraceLockHandles,
    PRTraceUnLockHandles,
    PRTraceStopRecording
} PRTraceOption;

/* -----------------------------------------------------------------------
** FUNCTION: PR_DEFINE_TRACE() -- Define a PRTraceHandle
**
** DESCRIPTION: PR_DEFINE_TRACE() is used to define a trace
** handle.
**
*/
#define PR_DEFINE_TRACE(name) PRTraceHandle name

/* -----------------------------------------------------------------------
** FUNCTION: PR_INIT_TRACE_HANDLE() -- Set the value of a PRTraceHandle
**
** DESCRIPTION:
** PR_INIT_TRACE_HANDLE() sets the value of a PRTraceHandle
** to value. e.g. PR_INIT_TRACE_HANDLE( myHandle, NULL );
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_INIT_TRACE_HANDLE(handle,value)\
    (handle) = (PRCounterHandle)(value)
#else
#define PR_INIT_TRACE_HANDLE(handle,value)
#endif


/* -----------------------------------------------------------------------
** FUNCTION: PR_CreateTrace() -- Create a trace handle
**
** DESCRIPTION:
**  PR_CreateTrace() creates a new trace handle. Tracing is
**  enabled for this handle when it is created. The trace handle
**  is intended for use in other Trace Facility calls.
**
**  PR_CreateTrace() registers the QName, RName and description
**  data so that this data can be retrieved later.
**
** INPUTS:
**  qName: pointer to string. QName for this trace handle.
**
**  rName: pointer to string. RName for this trace handle.
**
**  description: pointer to string. Descriptive data about this
**  trace handle.
**
** OUTPUTS:
**  Creates the trace handle.
**  Registers the QName and RName with the trace facility.
**
** RETURNS:
**  PRTraceHandle
**
** RESTRICTIONS:
**  qName is limited to 31 characters.
**  rName is limited to 31 characters.
**  description is limited to 255 characters.
**
*/
#define PRTRACE_NAME_MAX 31
#define PRTRACE_DESC_MAX 255

#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_CREATE_TRACE(handle,qName,rName,description)\
    (handle) = PR_CreateTrace((qName),(rName),(description))
#else
#define PR_CREATE_TRACE(handle,qName,rName,description)
#endif

NSPR_API(PRTraceHandle)
PR_CreateTrace(
    const char *qName,          /* QName for this trace handle */
    const char *rName,          /* RName for this trace handle */
    const char *description     /* description for this trace handle */
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_DestroyTrace() -- Destroy a trace handle
**
** DESCRIPTION:
**  PR_DestroyTrace() removes the referenced trace handle and
** associated QName, RName and description data from the Trace
** Facility.
**
** INPUTS: handle. A PRTraceHandle
**
** OUTPUTS:
**  The trace handle is unregistered.
**  The QName, RName and description are removed.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_DESTROY_TRACE(handle)\
    PR_DestroyTrace((handle))
#else
#define PR_DESTROY_TRACE(handle)
#endif

NSPR_API(void)
PR_DestroyTrace(
    PRTraceHandle handle    /* Handle to be destroyed */
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_Trace() -- Make a trace entry in the in-memory trace
**
** DESCRIPTION:
** PR_Trace() makes an entry in the in-memory trace buffer for
** the referenced trace handle. The next logically available
** PRTraceEntry is used; when the next trace entry would overflow
** the trace table, the table wraps.
**
** PR_Trace() for a specific trace handle may be disabled by
** calling PR_SetTraceOption() specifying PRTraceDisable for the
** trace handle to be disabled.
**
** INPUTS:
** handle: PRTraceHandle. The trace handle for this trace.
**
** userData[0..7]: unsigned 32bit integers. user supplied data
** that is copied into the PRTraceEntry
**
** OUTPUTS:
**  A PRTraceEntry is (conditionally) formatted in the in-memory
** trace buffer.
**
** RETURNS: void.
**
** RESTRICTIONS:
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_TRACE(handle,ud0,ud1,ud2,ud3,ud4,ud5,ud6,ud7)\
    PR_Trace((handle),(ud0),(ud1),(ud2),(ud3),(ud4),(ud5),(ud6),(ud7))
#else
#define PR_TRACE(handle,ud0,ud1,ud2,ud3,ud4,ud5,ud6,ud7)
#endif

NSPR_API(void)
PR_Trace(
    PRTraceHandle handle,       /* use this trace handle */
    PRUint32    userData0,      /* User supplied data word 0 */
    PRUint32    userData1,      /* User supplied data word 1 */
    PRUint32    userData2,      /* User supplied data word 2 */
    PRUint32    userData3,      /* User supplied data word 3 */
    PRUint32    userData4,      /* User supplied data word 4 */
    PRUint32    userData5,      /* User supplied data word 5 */
    PRUint32    userData6,      /* User supplied data word 6 */
    PRUint32    userData7       /* User supplied data word 7 */
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_SetTraceOption() -- Control the Trace Facility
**
** DESCRIPTION:
** PR_SetTraceOption() controls the Trace Facility. Depending on
** command and value, attributes of the Trace Facility may be
** changed.
**
** INPUTS:
**  command: An enumerated value in the set of PRTraceOption.
**  value: pointer to the data to be set. Type of the data is
**  dependent on command; for each value of command, the type
**  and meaning of dereferenced value is shown.
**
**  PRTraceBufSize: unsigned long: the size of the trace buffer,
** in bytes.
**
**  PRTraceEnable: PRTraceHandle. The trace handle to be
** enabled.
**
**  PRTraceDisable: PRTraceHandle. The trace handle to be
** disabled.
**
**  PRTraceSuspend: void. value must be NULL. All tracing is
** suspended.
**
**  PRTraceResume: void. value must be NULL. Tracing for all
** previously enabled, prior to a PRTraceSuspend, is resumed.
**
**  PRTraceStopRecording: void. value must be NULL. If recording
** (see: ** PR_RecordTraceEntries()) is being done,
** PRTraceStopRecording causes PR_RecordTraceEntries() to return
** to its caller. If recording is not being done, this function
** has no effect.
**
**  PRTraceSuspendRecording: void. Must be NULL. If recording is
** being done, PRTraceSuspendRecording causes further writes to
** the trace file to be suspended. Data in the in-memory
** trace buffer that would ordinarily be written to the
** trace file will not be written. Trace entries will continue
** to be entered in the in-memory buffer. If the Trace Facility
** recording is already in a suspended state, the call has no
** effect.
**
**  PRTraceResumeRecording: void. value must be NULL. If
** recording for the Trace Facility has been previously been
** suspended, this causes recording to resume. Recording resumes
** with the next in-memory buffer segment that would be written
** if trace recording had not been suspended. If recording is
** not currently suspended, the call has no effect.
**
**  PRTraceLockHandles: void. value must be NULL. Locks the
** trace handle lock. While the trace handle lock is held,
** calls to PR_CreateTrace() will block until the lock is
** released.
**
**  PRTraceUnlockHandles: void. value must be NULL. Unlocks the
** trace handle lock.
**
** OUTPUTS:
**  The operation of the Trace Facility may be changed.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_SET_TRACE_OPTION(command,value)\
    PR_SetTraceOption((command),(value))
#else
#define PR_SET_TRACE_OPTION(command,value)
#endif

NSPR_API(void)
PR_SetTraceOption(
    PRTraceOption command,  /* One of the enumerated values */
    void *value             /* command value or NULL */
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_GetTraceOption() -- Retrieve settings from the Trace Facility
**
** DESCRIPTION:
** PR_GetTraceOption() retrieves the current setting of the
** Trace Facility control depending on command.
**
**
**  PRTraceBufSize: unsigned long: the size of the trace buffer,
** in bytes.
**
**
** INPUTS:
**  command: one of the enumerated values in PRTraceOptions
** valid for PR_GetTraceOption().
**
** OUTPUTS:
**  dependent on command.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_GET_TRACE_OPTION(command,value)\
    PR_GetTraceOption((command),(value))
#else
#define PR_GET_TRACE_OPTION(command,value)
#endif

NSPR_API(void)
PR_GetTraceOption(
    PRTraceOption command,  /* One of the enumerated values */
    void *value             /* command value or NULL */
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_GetTraceHandleFromName() -- Retrieve an existing
** handle by name.
**
** DESCRIPTION:
** PR_GetTraceHandleFromName() retreives an existing tracehandle
** using the name specified by qName and rName.
**
** INPUTS:
**  qName: pointer to string. QName for this trace handle.
**
**  rName: pointer to string. RName for this trace handle.
**
**
** OUTPUTS: returned.
**
** RETURNS:
**  PRTraceHandle associated with qName and rName or NULL when
** there is no match.
**
** RESTRICTIONS:
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_GET_TRACE_HANDLE_FROM_NAME(handle,qName,rName)\
    (handle) = PR_GetTraceHandleFromName((qName),(rName))
#else
#define PR_GET_TRACE_HANDLE_FROM_NAME(handle,qName,rName)
#endif

NSPR_API(PRTraceHandle)
PR_GetTraceHandleFromName(
    const char *qName,      /* QName search argument */
    const char *rName       /* RName search argument */
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_GetTraceNameFromHandle() -- Retreive trace name
** by bandle.
**
** DESCRIPTION:
** PR_GetTraceNameFromHandle() retreives the existing qName,
** rName, and description for the referenced trace handle.
**
** INPUTS: handle: PRTraceHandle.
**
** OUTPUTS: pointers to the Trace Facility's copy of qName,
** rName and description. ... Don't mess with these values.
** They're mine.
**
** RETURNS: void
**
** RESTRICTIONS:
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_GET_TRACE_NAME_FROM_HANDLE(handle,qName,rName,description)\
    PR_GetTraceNameFromHandle((handle),(qName),(rName),(description))
#else
#define PR_GET_TRACE_NAME_FROM_HANDLE(handle,qName,rName,description)
#endif

NSPR_API(void)
PR_GetTraceNameFromHandle(
    PRTraceHandle handle,       /* handle as search argument */
    const char **qName,         /* pointer to associated QName */
    const char **rName,         /* pointer to associated RName */
    const char **description    /* pointer to associated description */
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_FindNextTraceQname() -- Retrieive a QName handle
** iterator.
**
** DESCRIPTION:
** PR_FindNextTraceQname() retreives the first or next trace
** QName handle, depending on the value of handle, from the trace
** database. The PRTraceHandle returned can be used as an
** iterator to traverse the QName handles in the Trace database.
**
** INPUTS:
**  handle: When NULL, PR_FindNextQname() returns the first QName
** handle. When a handle is a valid PRTraceHandle previously
** retreived using PR_FindNextQname() the next QName handle is
** retreived.
**
** OUTPUTS: returned.
**
** RETURNS:
**  PRTraceHandle or NULL when there are no trace handles.
**
** RESTRICTIONS:
**  Iterating thru the trace handles via FindFirst/FindNext
** should be done under protection of the trace handle lock.
** See: PR_SetTraceOption( PRLockTraceHandles ).
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_FIND_NEXT_TRACE_QNAME(next,handle)\
    (next) = PR_FindNextTraceQname((handle))
#else
#define PR_FIND_NEXT_TRACE_QNAME(next,handle)
#endif

NSPR_API(PRTraceHandle)
PR_FindNextTraceQname(
    PRTraceHandle handle
);


/* -----------------------------------------------------------------------
** FUNCTION: PR_FindNextTraceRname() -- Retrieive an RName handle
** iterator.
**
** DESCRIPTION:
** PR_FindNextTraceRname() retreives the first or next trace
** RName handle, depending on the value of handle, from the trace
** database. The PRTraceHandle returned can be used as an
** iterator to traverse the RName handles in the Trace database.
**
** INPUTS:
**  rhandle: When NULL, PR_FindNextRname() returns the first
** RName handle. When a handle is a valid PRTraceHandle
** previously retreived using PR_FindNextRname() the next RName
** handle is retreived.
**  qhandle: A valid PRTraceHandle retruned from a previous call
** to PR_FIND_NEXT_TRACE_QNAME().
**
** OUTPUTS: returned.
**
** RETURNS:
**  PRTraceHandle or NULL when there are no trace handles.
**
** RESTRICTIONS:
**  Iterating thru the trace handles via FindNext should be done
** under protection of the trace handle lock. See: (
** PR_SetTraceOption( PRLockTraceHandles ).
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_FIND_NEXT_TRACE_RNAME(next,rhandle,qhandle)\
    (next) = PR_FindNextTraceRname((rhandle),(qhandle))
#else
#define PR_FIND_NEXT_TRACE_RNAME(next,rhandle,qhandle)
#endif

NSPR_API(PRTraceHandle)
PR_FindNextTraceRname(
    PRTraceHandle rhandle,
    PRTraceHandle qhandle
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_RecordTraceEntries() -- Write trace entries to external media
**
** DESCRIPTION:
** PR_RecordTraceEntries() causes entries in the in-memory trace
** buffer to be written to external media.
**
** When PR_RecordTraceEntries() is called from an application
** thread, the function appears to block until another thread
** calls PR_SetTraceOption() with the PRTraceStopRecording
** option. This suggests that PR_RecordTraceEntries() should be
** called from a user supplied thread whose only job is to
** record trace entries.
**
** The environment variable NSPR_TRACE_LOG controls the operation
** of this function. When NSPR_TRACE_LOG is not defined in the
** environment, no recording of trace entries occurs. When
** NSPR_TRACE_LOG is defined, the value of its definition must be
** the filename of the file to receive the trace entry buffer.
**
** PR_RecordTraceEntries() attempts to record the in-memory
** buffer to a file, subject to the setting of the environment
** variable NSPR_TRACE_LOG. It is possible because of system
** load, the thread priority of the recording thread, number of
** active trace records being written over time, and other
** variables that some trace records can be lost. ... In other
** words: don't bet the farm on getting everything.
**
** INPUTS: none
**
** OUTPUTS: none
**
** RETURNS: PR_STATUS
**    PR_SUCCESS no errors were found.
**    PR_FAILURE errors were found.
**
** RESTRICTIONS:
** Only one thread can call PR_RecordTraceEntries() within a
** process.
**
** On error, PR_RecordTraceEntries() may return prematurely.
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_RECORD_TRACE_ENTRIES()\
    PR_RecordTraceEntries()
#else
#define PR_RECORD_TRACE_ENTRIES()
#endif

NSPR_API(void)
PR_RecordTraceEntries(
    void
);

/* -----------------------------------------------------------------------
** FUNCTION: PR_GetTraceEntries() -- Retreive trace entries from
** the Trace Facility
**
** DESCRIPTION:
** PR_GetTraceEntries() retreives trace entries from the Trace
** Facility. Up to count trace entries are copied from the Trace
** Facility into buffer. Only those trace entries that have not
** been copied via a previous call to PR_GetTraceEntries() are
** copied. The actual number copied is placed in the PRInt32
** variable pointed to by found.
**
** If more than count trace entries have entered the Trace
** Facility since the last call to PR_GetTraceEntries()
** a lost data condition is returned. In this case, the most
** recent count trace entries are copied into buffer and found is
** set to count.
**
** INPUTS:
**  count. The number of trace entries to be copied into buffer.
**
**
** OUTPUTS:
**  buffer. An array of PRTraceEntries. The buffer is supplied
** by the caller.
**
** found: 32bit signed integer. The number of PRTraceEntries
** actually copied. found is always less than or equal to count.
**
** RETURNS:
**  zero when there is no lost data.
**  non-zero when some PRTraceEntries have been lost.
**
** RESTRICTIONS:
** This is a real performance pig. The copy out operation is bad
** enough, but depending on then frequency of calls to the
** function, serious performance impact to the operating
** application may be realized. ... YMMV.
**
*/
#if defined (DEBUG) || defined (FORCE_NSPR_TRACE)
#define PR_GET_TRACE_ENTRIES(buffer,count,found)\
        PR_GetTraceEntries((buffer),(count),(found))
#else
#define PR_GET_TRACE_ENTRIES(buffer,count,found)
#endif

NSPR_API(PRIntn)
PR_GetTraceEntries(
    PRTraceEntry    *buffer,    /* where to write output */
    PRInt32         count,      /* number to get */
    PRInt32         *found      /* number you got */
);

PR_END_EXTERN_C

#endif /* prtrace_h___ */

