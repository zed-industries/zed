/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** prshm.h -- NSPR Shared Memory
**
** NSPR Named Shared Memory API provides a cross-platform named
** shared-memory interface. NSPR Named Shared Memory is modeled on
** similar constructs in Unix and Windows operating systems. Shared
** memory allows multiple processes to access one or more common shared
** memory regions, using it as an inter-process communication channel.
**
** Notes on Platform Independence:
**   NSPR Named Shared Memory is built on the native services offered
**   by most platforms. The NSPR Named Shared Memory API tries to
**   provide a least common denominator interface so that it works
**   across all supported platforms. To ensure that it works everywhere,
**   some platform considerations must be accomodated and the protocol
**   for using NSPR Shared Memory API must be observed.
**
** Protocol:
**   Multiple shared memories can be created using NSPR's Shared Memory
**   feature. For each named shared memory, as defined by the name
**   given in the PR_OpenSharedMemory() call, a protocol for using the
**   shared memory API is required to ensure desired behavior. Failing
**   to follow the protocol may yield unpredictable results.
**
**   PR_OpenSharedMemory() will create the shared memory segment, if it
**   does not already exist, or open a connection that the existing
**   shared memory segment if it already exists.
**
**   PR_AttachSharedMemory() should be called following
**   PR_OpenSharedMemory() to map the memory segment to an address in
**   the application's address space.
**
**   PR_AttachSharedMemory() may be called to re-map a shared memory
**   segment after detaching the same PRSharedMemory object. Be
**   sure to detach it when done.
**
**   PR_DetachSharedMemory() should be called to un-map the shared
**   memory segment from the application's address space.
**
**   PR_CloseSharedMemory() should be called when no further use of the
**   PRSharedMemory object is required within a process. Following a
**   call to  PR_CloseSharedMemory() the PRSharedMemory object is
**   invalid and cannot be reused.
**
**   PR_DeleteSharedMemory() should be called before process
**   termination. After calling PR_DeleteSharedMemory() any further use
**   of the shared memory associated with the name may cause
**   unpredictable results.
**
** Files:
**   The name passed to PR_OpenSharedMemory() should be a valid filename
**   for a unix platform. PR_OpenSharedMemory() creates file using the
**   name passed in. Some platforms may mangle the name before creating
**   the file and the shared memory.
**
**   The unix implementation may use SysV IPC shared memory, Posix
**   shared memory, or memory mapped files; the filename may used to
**   define the namespace. On Windows, the name is significant, but
**   there is no file associated with name.
**
**   No assumptions about the persistence of data in the named file
**   should be made. Depending on platform, the shared memory may be
**   mapped onto system paging space and be discarded at process
**   termination.
**
**   All names provided to PR_OpenSharedMemory() should be valid
**   filename syntax or name syntax for shared memory for the target
**   platform. Referenced directories should have permissions
**   appropriate for writing.
**
** Limits:
**   Different platforms have limits on both the number and size of
**   shared memory resources. The default system limits on some
**   platforms may be smaller than your requirements. These limits may
**   be adjusted on some platforms either via boot-time options or by
**   setting the size of the system paging space to accomodate more
**   and/or larger shared memory segment(s).
**
** Security:
**   On unix platforms, depending on implementation, contents of the
**   backing store for the shared memory can be exposed via the file
**   system. Set permissions and or access controls at create and attach
**   time to ensure you get the desired security.
**
**   On windows platforms, no special security measures are provided.
**
** Example:
**   The test case pr/tests/nameshm1.c provides an example of use as
**   well as testing the operation of NSPR's Named Shared Memory.
**
** lth. 18-Aug-1999.
*/

#ifndef prshm_h___
#define prshm_h___

#include "prtypes.h"
#include "prio.h"

PR_BEGIN_EXTERN_C

/*
** Declare opaque type PRSharedMemory.
*/
typedef struct PRSharedMemory PRSharedMemory;

/*
** FUNCTION: PR_OpenSharedMemory()
**
** DESCRIPTION:
**   PR_OpenSharedMemory() creates a new shared-memory segment or
**   associates a previously created memory segment with name.
**
**   When parameter create is (PR_SHM_EXCL | PR_SHM_CREATE) and the
**   shared memory already exists, the function returns NULL with the
**   error set to PR_FILE_EXISTS_ERROR.
**
**   When parameter create is PR_SHM_CREATE and the shared memory
**   already exists, a handle to that memory segment is returned. If
**   the segment does not exist, it is created and a pointer to the
**   related PRSharedMemory structure is returned.
**
**   When parameter create is 0, and the shared memory exists, a
**   pointer to a PRSharedMemory is returned. If the shared memory does
**   not exist, NULL is returned with the error set to
**   PR_FILE_NOT_FOUND_ERROR.
**
** INPUTS:
**   name -- the name the shared-memory segment is known as.
**   size -- the size of the shared memory segment.
**   flags -- Options for creating the shared memory
**   mode -- Same as is passed to PR_Open()
**
** OUTPUTS:
**   The shared memory is allocated.
**
** RETURNS: Pointer to opaque structure PRSharedMemory or NULL.
**   NULL is returned on error. The reason for the error can be
**   retrieved via PR_GetError() and PR_GetOSError();
**
*/
NSPR_API( PRSharedMemory * )
PR_OpenSharedMemory(
    const char *name,
    PRSize      size,
    PRIntn      flags,
    PRIntn      mode
);
/* Define values for PR_OpenShareMemory(...,create) */
#define PR_SHM_CREATE 0x1  /* create if not exist */
#define PR_SHM_EXCL   0x2  /* fail if already exists */

/*
** FUNCTION: PR_AttachSharedMemory()
**
** DESCRIPTION:
** PR_AttachSharedMemory() maps the shared-memory described by
** shm to the current process.
**
** INPUTS:
**   shm -- The handle returned from PR_OpenSharedMemory().
**   flags -- options for mapping the shared memory.
**   PR_SHM_READONLY causes the memory to be attached
**   read-only.
**
** OUTPUTS:
**   On success, the shared memory segment represented by shm is mapped
**   into the process' address space.
**
** RETURNS: Address where shared memory is mapped, or NULL.
**   NULL is returned on error. The reason for the error can be
**   retrieved via PR_GetError() and PR_GetOSError();
**
**
*/
NSPR_API( void * )
PR_AttachSharedMemory(
    PRSharedMemory *shm,
    PRIntn  flags
);
/* Define values for PR_AttachSharedMemory(...,flags) */
#define PR_SHM_READONLY 0x01

/*
** FUNCTION: PR_DetachSharedMemory()
**
** DESCRIPTION:
**   PR_DetachSharedMemory() detaches the shared-memory described
**   by shm.
**
** INPUTS:
**   shm -- The handle returned from PR_OpenSharedMemory().
**   addr -- The address at which the memory was attached.
**
** OUTPUTS:
**   The shared memory mapped to an address via a previous call to
**   PR_AttachSharedMemory() is unmapped.
**
** RETURNS: PRStatus
**
*/
NSPR_API( PRStatus )
PR_DetachSharedMemory(
    PRSharedMemory *shm,
    void  *addr
);

/*
** FUNCTION: PR_CloseSharedMemory()
**
** DESCRIPTION:
**   PR_CloseSharedMemory() closes the shared-memory described by
**   shm.
**
** INPUTS:
**   shm -- The handle returned from PR_OpenSharedMemory().
**
** OUTPUTS:
**   the shared memory represented by shm is closed
**
** RETURNS: PRStatus
**
*/
NSPR_API( PRStatus )
PR_CloseSharedMemory(
    PRSharedMemory *shm
);

/*
** FUNCTION: PR_DeleteSharedMemory()
**
** DESCRIPTION:
**   The shared memory resource represented by name is released.
**
** INPUTS:
**   name -- the name the shared-memory segment
**
** OUTPUTS:
**   depending on platform, resources may be returned to the underlying
**   operating system.
**
** RETURNS: PRStatus
**
*/
NSPR_API( PRStatus )
PR_DeleteSharedMemory(
    const char *name
);

PR_END_EXTERN_C

#endif /* prshm_h___ */
