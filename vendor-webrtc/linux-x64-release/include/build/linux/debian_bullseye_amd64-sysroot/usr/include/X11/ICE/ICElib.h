/******************************************************************************


Copyright 1993, 1998  The Open Group

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

Author: Ralph Mor, X Consortium
******************************************************************************/

#ifndef _ICELIB_H_
#define _ICELIB_H_

#include <X11/ICE/ICE.h>
#include <X11/Xfuncproto.h>

#define Bool int
#define Status int
#define True 1
#define False 0

typedef void *IcePointer;

typedef enum {
    IcePoAuthHaveReply,
    IcePoAuthRejected,
    IcePoAuthFailed,
    IcePoAuthDoneCleanup
} IcePoAuthStatus;

typedef enum {
    IcePaAuthContinue,
    IcePaAuthAccepted,
    IcePaAuthRejected,
    IcePaAuthFailed
} IcePaAuthStatus;

typedef enum {
    IceConnectPending,
    IceConnectAccepted,
    IceConnectRejected,
    IceConnectIOError
} IceConnectStatus;

typedef enum {
    IceProtocolSetupSuccess,
    IceProtocolSetupFailure,
    IceProtocolSetupIOError,
    IceProtocolAlreadyActive
} IceProtocolSetupStatus;

typedef enum {
    IceAcceptSuccess,
    IceAcceptFailure,
    IceAcceptBadMalloc
} IceAcceptStatus;

typedef enum {
    IceClosedNow,
    IceClosedASAP,
    IceConnectionInUse,
    IceStartedShutdownNegotiation
} IceCloseStatus;

typedef enum {
    IceProcessMessagesSuccess,
    IceProcessMessagesIOError,
    IceProcessMessagesConnectionClosed
} IceProcessMessagesStatus;

typedef struct {
    unsigned long	sequence_of_request;
    int			major_opcode_of_request;
    int			minor_opcode_of_request;
    IcePointer		reply;
} IceReplyWaitInfo;

typedef struct _IceConn *IceConn;
typedef struct _IceListenObj *IceListenObj;

typedef void (*IceWatchProc) (
    IceConn		/* iceConn */,
    IcePointer		/* clientData */,
    Bool		/* opening */,
    IcePointer *	/* watchData */
);

typedef void (*IcePoProcessMsgProc) (
    IceConn 		/* iceConn */,
    IcePointer		/* clientData */,
    int			/* opcode */,
    unsigned long	/* length */,
    Bool		/* swap */,
    IceReplyWaitInfo *  /* replyWait */,
    Bool *		/* replyReadyRet */
);

typedef void (*IcePaProcessMsgProc) (
    IceConn 		/* iceConn */,
    IcePointer		/* clientData */,
    int			/* opcode */,
    unsigned long	/* length */,
    Bool		/* swap */
);

typedef struct {
    int			 major_version;
    int			 minor_version;
    IcePoProcessMsgProc  process_msg_proc;
} IcePoVersionRec;

typedef struct {
    int			 major_version;
    int			 minor_version;
    IcePaProcessMsgProc  process_msg_proc;
} IcePaVersionRec;

typedef IcePoAuthStatus (*IcePoAuthProc) (
    IceConn		/* iceConn */,
    IcePointer *	/* authStatePtr */,
    Bool		/* cleanUp */,
    Bool		/* swap */,
    int			/* authDataLen */,
    IcePointer		/* authData */,
    int *		/* replyDataLenRet */,
    IcePointer *	/* replyDataRet */,
    char **		/* errorStringRet */
);

typedef IcePaAuthStatus (*IcePaAuthProc) (
    IceConn		/* iceConn */,
    IcePointer *	/* authStatePtr */,
    Bool		/* swap */,
    int			/* authDataLen */,
    IcePointer		/* authData */,
    int *		/* replyDataLenRet */,
    IcePointer *	/* replyDataRet */,
    char **		/* errorStringRet */
);

typedef Bool (*IceHostBasedAuthProc) (
    char *		/* hostName */
);

typedef Status (*IceProtocolSetupProc) (
    IceConn 		/* iceConn */,
    int			/* majorVersion */,
    int			/* minorVersion */,
    char *		/* vendor */,
    char *		/* release */,
    IcePointer *	/* clientDataRet */,
    char **		/* failureReasonRet */
);

typedef void (*IceProtocolActivateProc) (
    IceConn 		/* iceConn */,
    IcePointer		/* clientData */
);

typedef void (*IceIOErrorProc) (
    IceConn 		/* iceConn */
);

typedef void (*IcePingReplyProc) (
    IceConn 		/* iceConn */,
    IcePointer		/* clientData */
);

typedef void (*IceErrorHandler) (
    IceConn 		/* iceConn */,
    Bool		/* swap */,
    int			/* offendingMinorOpcode */,
    unsigned long 	/* offendingSequence */,
    int 		/* errorClass */,
    int			/* severity */,
    IcePointer		/* values */
);

typedef void (*IceIOErrorHandler) (
    IceConn 		/* iceConn */
);


/*
 * Function prototypes
 */

_XFUNCPROTOBEGIN

extern int IceRegisterForProtocolSetup (
    const char *		/* protocolName */,
    const char *		/* vendor */,
    const char *		/* release */,
    int				/* versionCount */,
    IcePoVersionRec *		/* versionRecs */,
    int				/* authCount */,
    const char **		/* authNames */,
    IcePoAuthProc *		/* authProcs */,
    IceIOErrorProc		/* IOErrorProc */
);

extern int IceRegisterForProtocolReply (
    const char *		/* protocolName */,
    const char *		/* vendor */,
    const char *		/* release */,
    int				/* versionCount */,
    IcePaVersionRec *		/* versionRecs */,
    int				/* authCount */,
    const char **		/* authNames */,
    IcePaAuthProc *		/* authProcs */,
    IceHostBasedAuthProc	/* hostBasedAuthProc */,
    IceProtocolSetupProc	/* protocolSetupProc */,
    IceProtocolActivateProc	/* protocolActivateProc */,
    IceIOErrorProc		/* IOErrorProc */
);

extern IceConn IceOpenConnection (
    char *		/* networkIdsList */,
    IcePointer		/* context */,
    Bool		/* mustAuthenticate */,
    int			/* majorOpcodeCheck */,
    int			/* errorLength */,
    char *		/* errorStringRet */
);

extern IcePointer IceGetConnectionContext (
    IceConn		/* iceConn */
);

extern Status IceListenForConnections (
    int *		/* countRet */,
    IceListenObj **	/* listenObjsRet */,
    int			/* errorLength */,
    char *		/* errorStringRet */
);

extern Status IceListenForWellKnownConnections (
    char *		/* port */,
    int *		/* countRet */,
    IceListenObj **	/* listenObjsRet */,
    int			/* errorLength */,
    char *		/* errorStringRet */
);

extern int IceGetListenConnectionNumber (
    IceListenObj	/* listenObj */
);

extern char *IceGetListenConnectionString (
    IceListenObj	/* listenObj */
);

extern char *IceComposeNetworkIdList (
    int			/* count */,
    IceListenObj *	/* listenObjs */
);

extern void IceFreeListenObjs (
    int			/* count */,
    IceListenObj *	/* listenObjs */
);

extern void IceSetHostBasedAuthProc (
    IceListenObj		/* listenObj */,
    IceHostBasedAuthProc   	/* hostBasedAuthProc */
);

extern IceConn IceAcceptConnection (
    IceListenObj	/* listenObj */,
    IceAcceptStatus *	/* statusRet */
);

extern void IceSetShutdownNegotiation (
    IceConn		/* iceConn */,
    Bool		/* negotiate */
);

extern Bool IceCheckShutdownNegotiation (
    IceConn		/* iceConn */
);

extern IceCloseStatus IceCloseConnection (
    IceConn		/* iceConn */
);

extern Status IceAddConnectionWatch (
    IceWatchProc		/* watchProc */,
    IcePointer			/* clientData */
);

extern void IceRemoveConnectionWatch (
    IceWatchProc		/* watchProc */,
    IcePointer			/* clientData */
);

extern IceProtocolSetupStatus IceProtocolSetup (
    IceConn		/* iceConn */,
    int 		/* myOpcode */,
    IcePointer		/* clientData */,
    Bool		/* mustAuthenticate */,
    int	*		/* majorVersionRet */,
    int	*		/* minorVersionRet */,
    char **		/* vendorRet */,
    char **		/* releaseRet */,
    int			/* errorLength */,
    char *		/* errorStringRet */
);

extern Status IceProtocolShutdown (
    IceConn		/* iceConn */,
    int			/* majorOpcode */
);

extern IceProcessMessagesStatus IceProcessMessages (
    IceConn		/* iceConn */,
    IceReplyWaitInfo *	/* replyWait */,
    Bool *		/* replyReadyRet */
);

extern Status IcePing (
   IceConn		/* iceConn */,
   IcePingReplyProc	/* pingReplyProc */,
   IcePointer		/* clientData */
);

extern char *IceAllocScratch (
   IceConn		/* iceConn */,
   unsigned long	/* size */
);

extern int IceFlush (
   IceConn		/* iceConn */
);

extern int IceGetOutBufSize (
   IceConn		/* iceConn */
);

extern int IceGetInBufSize (
   IceConn		/* iceConn */
);

extern IceConnectStatus IceConnectionStatus (
    IceConn		/* iceConn */
);

extern char *IceVendor (
    IceConn		/* iceConn */
);

extern char *IceRelease (
    IceConn		/* iceConn */
);

extern int IceProtocolVersion (
    IceConn		/* iceConn */
);

extern int IceProtocolRevision (
    IceConn		/* iceConn */
);

extern int IceConnectionNumber (
    IceConn		/* iceConn */
);

extern char *IceConnectionString (
    IceConn		/* iceConn */
);

extern unsigned long IceLastSentSequenceNumber (
    IceConn		/* iceConn */
);

extern unsigned long IceLastReceivedSequenceNumber (
    IceConn		/* iceConn */
);

extern Bool IceSwapping (
    IceConn		/* iceConn */
);

extern IceErrorHandler IceSetErrorHandler (
    IceErrorHandler 	/* handler */
);

extern IceIOErrorHandler IceSetIOErrorHandler (
    IceIOErrorHandler 	/* handler */
);

extern char *IceGetPeerName (
    IceConn		/* iceConn */
);

/*
 * Multithread Routines
 */

extern Status IceInitThreads (
    void
);

extern void IceAppLockConn (
    IceConn		/* iceConn */
);

extern void IceAppUnlockConn (
    IceConn		/* iceConn */
);

_XFUNCPROTOEND

#endif /* _ICELIB_H_ */
