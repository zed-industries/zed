/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:    pprio.h
**
** Description: Private definitions for I/O related structures
*/

#ifndef pprio_h___
#define pprio_h___

#include "prtypes.h"
#include "prio.h"

PR_BEGIN_EXTERN_C

/************************************************************************/
/************************************************************************/

#ifdef _WIN64
typedef __int64 PROsfd;
#else
typedef PRInt32 PROsfd;
#endif

/* Return the method tables for files, tcp sockets and udp sockets */
NSPR_API(const PRIOMethods*)    PR_GetFileMethods(void);
NSPR_API(const PRIOMethods*)    PR_GetTCPMethods(void);
NSPR_API(const PRIOMethods*)    PR_GetUDPMethods(void);
NSPR_API(const PRIOMethods*)    PR_GetPipeMethods(void);

/*
** Convert a NSPR socket handle to a native socket handle.
**
** Using this function makes your code depend on the properties of the
** current NSPR implementation, which may change (although extremely
** unlikely because of NSPR's backward compatibility requirement).  Avoid
** using it if you can.
**
** If you use this function, you need to understand what NSPR does to
** the native handle.  For example, NSPR puts native socket handles in
** non-blocking mode or associates them with an I/O completion port (the
** WINNT build configuration only).  Your use of the native handle should
** not interfere with NSPR's use of the native handle.  If your code
** changes the configuration of the native handle, (e.g., changes it to
** blocking or closes it), NSPR will not work correctly.
*/
NSPR_API(PROsfd)       PR_FileDesc2NativeHandle(PRFileDesc *);
NSPR_API(void)         PR_ChangeFileDescNativeHandle(PRFileDesc *, PROsfd);
NSPR_API(PRFileDesc*)  PR_AllocFileDesc(PROsfd osfd,
                                        const PRIOMethods *methods);
NSPR_API(void)         PR_FreeFileDesc(PRFileDesc *fd);
/*
** Import an existing OS file to NSPR.
*/
NSPR_API(PRFileDesc*)  PR_ImportFile(PROsfd osfd);
NSPR_API(PRFileDesc*)  PR_ImportPipe(PROsfd osfd);
NSPR_API(PRFileDesc*)  PR_ImportTCPSocket(PROsfd osfd);
NSPR_API(PRFileDesc*)  PR_ImportUDPSocket(PROsfd osfd);


/*
 *************************************************************************
 * FUNCTION: PR_CreateSocketPollFd
 * DESCRIPTION:
 *     Create a PRFileDesc wrapper for a native socket handle, for use with
 *     PR_Poll only
 * INPUTS:
 *     None
 * OUTPUTS:
 *     None
 * RETURN: PRFileDesc*
 *     Upon successful completion, PR_CreateSocketPollFd returns a pointer
 *     to the PRFileDesc created for the native socket handle
 *     Returns a NULL pointer if the create of a new PRFileDesc failed
 *
 **************************************************************************
 */

NSPR_API(PRFileDesc*)   PR_CreateSocketPollFd(PROsfd osfd);

/*
 *************************************************************************
 * FUNCTION: PR_DestroySocketPollFd
 * DESCRIPTION:
 *     Destroy the PRFileDesc wrapper created by PR_CreateSocketPollFd
 * INPUTS:
 *     None
 * OUTPUTS:
 *     None
 * RETURN: PRFileDesc*
 *     Upon successful completion, PR_DestroySocketPollFd returns
 *     PR_SUCCESS, else PR_FAILURE
 *
 **************************************************************************
 */

NSPR_API(PRStatus) PR_DestroySocketPollFd(PRFileDesc *fd);


/*
** Macros for PR_Socket
**
** Socket types: PR_SOCK_STREAM, PR_SOCK_DGRAM
*/

#ifdef WIN32

#define PR_SOCK_STREAM 1
#define PR_SOCK_DGRAM 2

#else /* WIN32 */

#define PR_SOCK_STREAM SOCK_STREAM
#define PR_SOCK_DGRAM SOCK_DGRAM

#endif /* WIN32 */

/*
** Create a new Socket; this function is obsolete.
*/
NSPR_API(PRFileDesc*)   PR_Socket(PRInt32 domain, PRInt32 type, PRInt32 proto);

/* FUNCTION: PR_LockFile
** DESCRIPTION:
**    Lock a file for exclusive access.
** RETURNS:
**    PR_SUCCESS when the lock is held
**    PR_FAILURE otherwise
*/
NSPR_API(PRStatus) PR_LockFile(PRFileDesc *fd);

/* FUNCTION: PR_TLockFile
** DESCRIPTION:
**    Test and Lock a file for exclusive access.  Do not block if the
**    file cannot be locked immediately.
** RETURNS:
**    PR_SUCCESS when the lock is held
**    PR_FAILURE otherwise
*/
NSPR_API(PRStatus) PR_TLockFile(PRFileDesc *fd);

/* FUNCTION: PR_UnlockFile
** DESCRIPTION:
**    Unlock a file which has been previously locked successfully by this
**    process.
** RETURNS:
**    PR_SUCCESS when the lock is released
**    PR_FAILURE otherwise
*/
NSPR_API(PRStatus) PR_UnlockFile(PRFileDesc *fd);

/*
** Emulate acceptread by accept and recv.
*/
NSPR_API(PRInt32) PR_EmulateAcceptRead(PRFileDesc *sd, PRFileDesc **nd,
                                       PRNetAddr **raddr, void *buf, PRInt32 amount, PRIntervalTime timeout);

/*
** Emulate sendfile by reading from the file and writing to the socket.
** The file is memory-mapped if memory-mapped files are supported.
*/
NSPR_API(PRInt32) PR_EmulateSendFile(
    PRFileDesc *networkSocket, PRSendFileData *sendData,
    PRTransmitFileFlags flags, PRIntervalTime timeout);

#ifdef WIN32
/* FUNCTION: PR_NTFast_AcceptRead
** DESCRIPTION:
**    NT has the notion of an "accept context", which is only needed in
**    order to make certain calls.  By default, a socket connected via
**    AcceptEx can only do a limited number of things without updating
**    the acceptcontext.  The generic version of PR_AcceptRead always
**    updates the accept context.  This version does not.
**/
NSPR_API(PRInt32) PR_NTFast_AcceptRead(PRFileDesc *sd, PRFileDesc **nd,
                                       PRNetAddr **raddr, void *buf, PRInt32 amount, PRIntervalTime t);

typedef void (*_PR_AcceptTimeoutCallback)(void *);

/* FUNCTION: PR_NTFast_AcceptRead_WithTimeoutCallback
** DESCRIPTION:
**    The AcceptEx call combines the accept with the read function.  However,
**    our daemon threads need to be able to wakeup and reliably flush their
**    log buffers if the Accept times out.  However, with the current blocking
**    interface to AcceptRead, there is no way for us to timeout the Accept;
**    this is because when we timeout the Read, we can close the newly
**    socket and continue; but when we timeout the accept itself, there is no
**    new socket to timeout.  So instead, this version of the function is
**    provided.  After the initial timeout period elapses on the accept()
**    portion of the function, it will call the callback routine and then
**    continue the accept.   If the timeout occurs on the read, it will
**    close the connection and return error.
*/
NSPR_API(PRInt32) PR_NTFast_AcceptRead_WithTimeoutCallback(
    PRFileDesc *sd,
    PRFileDesc **nd,
    PRNetAddr **raddr,
    void *buf,
    PRInt32 amount,
    PRIntervalTime t,
    _PR_AcceptTimeoutCallback callback,
    void *callback_arg);

/* FUNCTION: PR_NTFast_Accept
** DESCRIPTION:
**    NT has the notion of an "accept context", which is only needed in
**    order to make certain calls.  By default, a socket connected via
**    AcceptEx can only do a limited number of things without updating
**    the acceptcontext.  The generic version of PR_Accept always
**    updates the accept context.  This version does not.
**/
NSPR_API(PRFileDesc*)   PR_NTFast_Accept(PRFileDesc *fd, PRNetAddr *addr,
        PRIntervalTime timeout);

/* FUNCTION: PR_NTFast_Update
** DESCRIPTION:
**    For sockets accepted with PR_NTFast_Accept or PR_NTFastAcceptRead,
**    this function will update the accept context for those sockets,
**    so that the socket can make general purpose socket calls.
**    Without calling this, the only operations supported on the socket
**    Are PR_Read, PR_Write, PR_Transmitfile, and PR_Close.
*/
NSPR_API(void) PR_NTFast_UpdateAcceptContext(PRFileDesc *acceptSock,
        PRFileDesc *listenSock);


/* FUNCTION: PR_NT_CancelIo
** DESCRIPTION:
**    Cancel IO operations on fd.
*/
NSPR_API(PRStatus) PR_NT_CancelIo(PRFileDesc *fd);


#endif /* WIN32 */

PR_END_EXTERN_C

#endif /* pprio_h___ */
