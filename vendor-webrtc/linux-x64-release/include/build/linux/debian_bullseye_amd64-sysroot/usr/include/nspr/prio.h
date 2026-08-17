/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * File:     prio.h
 *
 * Description:    PR i/o related stuff, such as file system access, file
 *         i/o, socket i/o, etc.
 */

#ifndef prio_h___
#define prio_h___

#include "prlong.h"
#include "prtime.h"
#include "prinrval.h"
#include "prinet.h"

PR_BEGIN_EXTERN_C

/* Typedefs */
typedef struct PRDir            PRDir;
typedef struct PRDirEntry       PRDirEntry;
#ifdef MOZ_UNICODE
typedef struct PRDirUTF16       PRDirUTF16;
typedef struct PRDirEntryUTF16  PRDirEntryUTF16;
#endif /* MOZ_UNICODE */
typedef struct PRFileDesc       PRFileDesc;
typedef struct PRFileInfo       PRFileInfo;
typedef struct PRFileInfo64     PRFileInfo64;
typedef union  PRNetAddr        PRNetAddr;
typedef struct PRIOMethods      PRIOMethods;
typedef struct PRPollDesc       PRPollDesc;
typedef struct PRFilePrivate    PRFilePrivate;
typedef struct PRSendFileData   PRSendFileData;

/*
***************************************************************************
** The file descriptor.
** This is the primary structure to represent any active open socket,
** whether it be a normal file or a network connection. Such objects
** are stackable (or layerable). Each layer may have its own set of
** method pointers and context private to that layer. All each layer
** knows about its neighbors is how to get to their method table.
***************************************************************************
*/

typedef PRIntn PRDescIdentity;          /* see: Layering file descriptors */

struct PRFileDesc {
    const PRIOMethods *methods;         /* the I/O methods table */
    PRFilePrivate *secret;              /* layer dependent data */
    PRFileDesc *lower, *higher;         /* pointers to adjacent layers */
    void (PR_CALLBACK *dtor)(PRFileDesc *fd);
    /* A destructor function for layer */
    PRDescIdentity identity;            /* Identity of this particular layer  */
};

/*
***************************************************************************
** PRTransmitFileFlags
**
** Flags for PR_TransmitFile.  Pass PR_TRANSMITFILE_CLOSE_SOCKET to
** PR_TransmitFile if the connection should be closed after the file
** is transmitted.
***************************************************************************
*/
typedef enum PRTransmitFileFlags {
    PR_TRANSMITFILE_KEEP_OPEN = 0,    /* socket is left open after file
                                       * is transmitted. */
    PR_TRANSMITFILE_CLOSE_SOCKET = 1  /* socket is closed after file
                                       * is transmitted. */
} PRTransmitFileFlags;

/*
**************************************************************************
** Macros for PRNetAddr
**
** Address families: PR_AF_INET, PR_AF_INET6, PR_AF_LOCAL
** IP addresses: PR_INADDR_ANY, PR_INADDR_LOOPBACK, PR_INADDR_BROADCAST
**************************************************************************
*/

#ifdef WIN32

#define PR_AF_INET 2
#define PR_AF_LOCAL 1
#define PR_INADDR_ANY (unsigned long)0x00000000
#define PR_INADDR_LOOPBACK 0x7f000001
#define PR_INADDR_BROADCAST (unsigned long)0xffffffff

#else /* WIN32 */

#define PR_AF_INET AF_INET
#define PR_AF_LOCAL AF_UNIX
#define PR_INADDR_ANY INADDR_ANY
#define PR_INADDR_LOOPBACK INADDR_LOOPBACK
#define PR_INADDR_BROADCAST INADDR_BROADCAST

#endif /* WIN32 */

/*
** Define PR_AF_INET6 in prcpucfg.h with the same
** value as AF_INET6 on platforms with IPv6 support.
** Otherwise define it here.
*/
#ifndef PR_AF_INET6
#define PR_AF_INET6 100
#endif

#define PR_AF_INET_SDP 101
#define PR_AF_INET6_SDP 102

#ifndef PR_AF_UNSPEC
#define PR_AF_UNSPEC 0
#endif

/*
**************************************************************************
** A network address
**
** Only Internet Protocol (IPv4 and IPv6) addresses are supported.
** The address family must always represent IPv4 (AF_INET, probably == 2)
** or IPv6 (AF_INET6).
**************************************************************************
*************************************************************************/

struct PRIPv6Addr {
    union {
        PRUint8  _S6_u8[16];
        PRUint16 _S6_u16[8];
        PRUint32 _S6_u32[4];
        PRUint64 _S6_u64[2];
    } _S6_un;
};
#define pr_s6_addr      _S6_un._S6_u8
#define pr_s6_addr16    _S6_un._S6_u16
#define pr_s6_addr32    _S6_un._S6_u32
#define pr_s6_addr64    _S6_un._S6_u64

typedef struct PRIPv6Addr PRIPv6Addr;

union PRNetAddr {
    struct {
        PRUint16 family;                /* address family (0x00ff maskable) */
        char data[14];                  /* raw address data */
    } raw;
    struct {
        PRUint16 family;                /* address family (AF_INET) */
        PRUint16 port;                  /* port number */
        PRUint32 ip;                    /* The actual 32 bits of address */
        char pad[8];
    } inet;
    struct {
        PRUint16 family;                /* address family (AF_INET6) */
        PRUint16 port;                  /* port number */
        PRUint32 flowinfo;              /* routing information */
        PRIPv6Addr ip;                  /* the actual 128 bits of address */
        PRUint32 scope_id;              /* set of interfaces for a scope */
    } ipv6;
#if defined(XP_UNIX) || defined(XP_OS2) || defined(XP_WIN)
    struct {                            /* Unix domain socket address */
        PRUint16 family;                /* address family (AF_UNIX) */
#ifdef XP_OS2
        char path[108];                 /* null-terminated pathname */
        /* bind fails if size is not 108. */
#else
        char path[104];                 /* null-terminated pathname */
#endif
    } local;
#endif
};

/*
***************************************************************************
** PRSockOption
**
** The file descriptors can have predefined options set after they file
** descriptor is created to change their behavior. Only the options in
** the following enumeration are supported.
***************************************************************************
*/
typedef enum PRSockOption
{
    PR_SockOpt_Nonblocking,     /* nonblocking io */
    PR_SockOpt_Linger,          /* linger on close if data present */
    PR_SockOpt_Reuseaddr,       /* allow local address reuse */
    PR_SockOpt_Keepalive,       /* keep connections alive */
    PR_SockOpt_RecvBufferSize,  /* receive buffer size */
    PR_SockOpt_SendBufferSize,  /* send buffer size */

    PR_SockOpt_IpTimeToLive,    /* time to live */
    PR_SockOpt_IpTypeOfService, /* type of service and precedence */

    PR_SockOpt_AddMember,       /* add an IP group membership */
    PR_SockOpt_DropMember,      /* drop an IP group membership */
    PR_SockOpt_McastInterface,  /* multicast interface address */
    PR_SockOpt_McastTimeToLive, /* multicast timetolive */
    PR_SockOpt_McastLoopback,   /* multicast loopback */

    PR_SockOpt_NoDelay,         /* don't delay send to coalesce packets */
    PR_SockOpt_MaxSegment,      /* maximum segment size */
    PR_SockOpt_Broadcast,       /* enable broadcast */
    PR_SockOpt_Reuseport,       /* allow local address & port reuse on
                                 * platforms that support it */
    PR_SockOpt_Last
} PRSockOption;

typedef struct PRLinger {
    PRBool polarity;            /* Polarity of the option's setting */
    PRIntervalTime linger;      /* Time to linger before closing */
} PRLinger;

typedef struct PRMcastRequest {
    PRNetAddr mcaddr;           /* IP multicast address of group */
    PRNetAddr ifaddr;           /* local IP address of interface */
} PRMcastRequest;

typedef struct PRSocketOptionData
{
    PRSockOption option;
    union
    {
        PRUintn ip_ttl;             /* IP time to live */
        PRUintn mcast_ttl;          /* IP multicast time to live */
        PRUintn tos;                /* IP type of service and precedence */
        PRBool non_blocking;        /* Non-blocking (network) I/O */
        PRBool reuse_addr;          /* Allow local address reuse */
        PRBool reuse_port;          /* Allow local address & port reuse on
                                     * platforms that support it */
        PRBool keep_alive;          /* Keep connections alive */
        PRBool mcast_loopback;      /* IP multicast loopback */
        PRBool no_delay;            /* Don't delay send to coalesce packets */
        PRBool broadcast;           /* Enable broadcast */
        PRSize max_segment;         /* Maximum segment size */
        PRSize recv_buffer_size;    /* Receive buffer size */
        PRSize send_buffer_size;    /* Send buffer size */
        PRLinger linger;            /* Time to linger on close if data present */
        PRMcastRequest add_member;  /* add an IP group membership */
        PRMcastRequest drop_member; /* Drop an IP group membership */
        PRNetAddr mcast_if;         /* multicast interface address */
    } value;
} PRSocketOptionData;

/*
***************************************************************************
** PRIOVec
**
** The I/O vector is used by the write vector method to describe the areas
** that are affected by the ouput operation.
***************************************************************************
*/
typedef struct PRIOVec {
    char *iov_base;
    int iov_len;
} PRIOVec;

/*
***************************************************************************
** Discover what type of socket is being described by the file descriptor.
***************************************************************************
*/
typedef enum PRDescType
{
    PR_DESC_FILE = 1,
    PR_DESC_SOCKET_TCP = 2,
    PR_DESC_SOCKET_UDP = 3,
    PR_DESC_LAYERED = 4,
    PR_DESC_PIPE = 5
} PRDescType;

typedef enum PRSeekWhence {
    PR_SEEK_SET = 0,
    PR_SEEK_CUR = 1,
    PR_SEEK_END = 2
} PRSeekWhence;

NSPR_API(PRDescType) PR_GetDescType(PRFileDesc *file);

/*
***************************************************************************
** PRIOMethods
**
** The I/O methods table provides procedural access to the functions of
** the file descriptor. It is the responsibility of a layer implementor
** to provide suitable functions at every entry point. If a layer provides
** no functionality, it should call the next lower(higher) function of the
** same name (e.g., return fd->lower->method->close(fd->lower));
**
** Not all functions are implemented for all types of files. In cases where
** that is true, the function will return a error indication with an error
** code of PR_INVALID_METHOD_ERROR.
***************************************************************************
*/

typedef PRStatus (PR_CALLBACK *PRCloseFN)(PRFileDesc *fd);
typedef PRInt32 (PR_CALLBACK *PRReadFN)(PRFileDesc *fd, void *buf, PRInt32 amount);
typedef PRInt32 (PR_CALLBACK *PRWriteFN)(PRFileDesc *fd, const void *buf, PRInt32 amount);
typedef PRInt32 (PR_CALLBACK *PRAvailableFN)(PRFileDesc *fd);
typedef PRInt64 (PR_CALLBACK *PRAvailable64FN)(PRFileDesc *fd);
typedef PRStatus (PR_CALLBACK *PRFsyncFN)(PRFileDesc *fd);
typedef PROffset32 (PR_CALLBACK *PRSeekFN)(PRFileDesc *fd, PROffset32 offset, PRSeekWhence how);
typedef PROffset64 (PR_CALLBACK *PRSeek64FN)(PRFileDesc *fd, PROffset64 offset, PRSeekWhence how);
typedef PRStatus (PR_CALLBACK *PRFileInfoFN)(PRFileDesc *fd, PRFileInfo *info);
typedef PRStatus (PR_CALLBACK *PRFileInfo64FN)(PRFileDesc *fd, PRFileInfo64 *info);
typedef PRInt32 (PR_CALLBACK *PRWritevFN)(
    PRFileDesc *fd, const PRIOVec *iov, PRInt32 iov_size,
    PRIntervalTime timeout);
typedef PRStatus (PR_CALLBACK *PRConnectFN)(
    PRFileDesc *fd, const PRNetAddr *addr, PRIntervalTime timeout);
typedef PRFileDesc* (PR_CALLBACK *PRAcceptFN) (
    PRFileDesc *fd, PRNetAddr *addr, PRIntervalTime timeout);
typedef PRStatus (PR_CALLBACK *PRBindFN)(PRFileDesc *fd, const PRNetAddr *addr);
typedef PRStatus (PR_CALLBACK *PRListenFN)(PRFileDesc *fd, PRIntn backlog);
typedef PRStatus (PR_CALLBACK *PRShutdownFN)(PRFileDesc *fd, PRIntn how);
typedef PRInt32 (PR_CALLBACK *PRRecvFN)(
    PRFileDesc *fd, void *buf, PRInt32 amount,
    PRIntn flags, PRIntervalTime timeout);
typedef PRInt32 (PR_CALLBACK *PRSendFN) (
    PRFileDesc *fd, const void *buf, PRInt32 amount,
    PRIntn flags, PRIntervalTime timeout);
typedef PRInt32 (PR_CALLBACK *PRRecvfromFN)(
    PRFileDesc *fd, void *buf, PRInt32 amount,
    PRIntn flags, PRNetAddr *addr, PRIntervalTime timeout);
typedef PRInt32 (PR_CALLBACK *PRSendtoFN)(
    PRFileDesc *fd, const void *buf, PRInt32 amount,
    PRIntn flags, const PRNetAddr *addr, PRIntervalTime timeout);
typedef PRInt16 (PR_CALLBACK *PRPollFN)(
    PRFileDesc *fd, PRInt16 in_flags, PRInt16 *out_flags);
typedef PRInt32 (PR_CALLBACK *PRAcceptreadFN)(
    PRFileDesc *sd, PRFileDesc **nd, PRNetAddr **raddr,
    void *buf, PRInt32 amount, PRIntervalTime t);
typedef PRInt32 (PR_CALLBACK *PRTransmitfileFN)(
    PRFileDesc *sd, PRFileDesc *fd, const void *headers,
    PRInt32 hlen, PRTransmitFileFlags flags, PRIntervalTime t);
typedef PRStatus (PR_CALLBACK *PRGetsocknameFN)(PRFileDesc *fd, PRNetAddr *addr);
typedef PRStatus (PR_CALLBACK *PRGetpeernameFN)(PRFileDesc *fd, PRNetAddr *addr);
typedef PRStatus (PR_CALLBACK *PRGetsocketoptionFN)(
    PRFileDesc *fd, PRSocketOptionData *data);
typedef PRStatus (PR_CALLBACK *PRSetsocketoptionFN)(
    PRFileDesc *fd, const PRSocketOptionData *data);
typedef PRInt32 (PR_CALLBACK *PRSendfileFN)(
    PRFileDesc *networkSocket, PRSendFileData *sendData,
    PRTransmitFileFlags flags, PRIntervalTime timeout);
typedef PRStatus (PR_CALLBACK *PRConnectcontinueFN)(
    PRFileDesc *fd, PRInt16 out_flags);
typedef PRIntn (PR_CALLBACK *PRReservedFN)(PRFileDesc *fd);

struct PRIOMethods {
    PRDescType file_type;           /* Type of file represented (tos)           */
    PRCloseFN close;                /* close file and destroy descriptor        */
    PRReadFN read;                  /* read up to specified bytes into buffer   */
    PRWriteFN write;                /* write specified bytes from buffer        */
    PRAvailableFN available;        /* determine number of bytes available      */
    PRAvailable64FN available64;    /*          ditto, 64 bit                   */
    PRFsyncFN fsync;                /* flush all buffers to permanent store     */
    PRSeekFN seek;                  /* position the file to the desired place   */
    PRSeek64FN seek64;              /*           ditto, 64 bit                  */
    PRFileInfoFN fileInfo;          /* Get information about an open file       */
    PRFileInfo64FN fileInfo64;      /*           ditto, 64 bit                  */
    PRWritevFN writev;              /* Write segments as described by iovector  */
    PRConnectFN connect;            /* Connect to the specified (net) address   */
    PRAcceptFN accept;              /* Accept a connection for a (net) peer     */
    PRBindFN bind;                  /* Associate a (net) address with the fd    */
    PRListenFN listen;              /* Prepare to listen for (net) connections  */
    PRShutdownFN shutdown;          /* Shutdown a (net) connection              */
    PRRecvFN recv;                  /* Solicit up the the specified bytes       */
    PRSendFN send;                  /* Send all the bytes specified             */
    PRRecvfromFN recvfrom;          /* Solicit (net) bytes and report source    */
    PRSendtoFN sendto;              /* Send bytes to (net) address specified    */
    PRPollFN poll;                  /* Test the fd to see if it is ready        */
    PRAcceptreadFN acceptread;      /* Accept and read on a new (net) fd        */
    PRTransmitfileFN transmitfile;  /* Transmit at entire file                  */
    PRGetsocknameFN getsockname;    /* Get (net) address associated with fd     */
    PRGetpeernameFN getpeername;    /* Get peer's (net) address                 */
    PRReservedFN reserved_fn_6;     /* reserved for future use */
    PRReservedFN reserved_fn_5;     /* reserved for future use */
    PRGetsocketoptionFN getsocketoption;
    /* Get current setting of specified option  */
    PRSetsocketoptionFN setsocketoption;
    /* Set value of specified option            */
    PRSendfileFN sendfile;          /* Send a (partial) file with header/trailer*/
    PRConnectcontinueFN connectcontinue;
    /* Continue a nonblocking connect */
    PRReservedFN reserved_fn_3;     /* reserved for future use */
    PRReservedFN reserved_fn_2;     /* reserved for future use */
    PRReservedFN reserved_fn_1;     /* reserved for future use */
    PRReservedFN reserved_fn_0;     /* reserved for future use */
};

/*
 **************************************************************************
 * FUNCTION: PR_GetSpecialFD
 * DESCRIPTION: Get the file descriptor that represents the standard input,
 *              output, or error stream.
 * INPUTS:
 *     PRSpecialFD id
 *         A value indicating the type of stream desired:
 *             PR_StandardInput: standard input
 *             PR_StandardOuput: standard output
 *             PR_StandardError: standard error
 * OUTPUTS: none
 * RETURNS: PRFileDesc *
 *     If the argument is valid, PR_GetSpecialFD returns a file descriptor
 *     that represents the corresponding standard I/O stream.  Otherwise,
 *     PR_GetSpecialFD returns NULL and sets error PR_INVALID_ARGUMENT_ERROR.
 **************************************************************************
 */

typedef enum PRSpecialFD
{
    PR_StandardInput,          /* standard input */
    PR_StandardOutput,         /* standard output */
    PR_StandardError           /* standard error */
} PRSpecialFD;

NSPR_API(PRFileDesc*) PR_GetSpecialFD(PRSpecialFD id);

#define PR_STDIN    PR_GetSpecialFD(PR_StandardInput)
#define PR_STDOUT   PR_GetSpecialFD(PR_StandardOutput)
#define PR_STDERR   PR_GetSpecialFD(PR_StandardError)

/*
 **************************************************************************
 * Layering file descriptors
 *
 * File descriptors may be layered. Each layer has it's own identity.
 * Identities are allocated by the runtime and are to be associated
 * (by the layer implementor) with all layers that are of that type.
 * It is then possible to scan the chain of layers and find a layer
 * that one recongizes and therefore predict that it will implement
 * a desired protocol.
 *
 * There are three well-known identities:
 *      PR_INVALID_IO_LAYER => an invalid layer identity, for error return
 *      PR_TOP_IO_LAYER     => the identity of the top of the stack
 *      PR_NSPR_IO_LAYER    => the identity used by NSPR proper
 * PR_TOP_IO_LAYER may be used as a shorthand for identifying the topmost
 * layer of an existing stack. Ie., the following two constructs are
 * equivalent.
 *
 *      rv = PR_PushIOLayer(stack, PR_TOP_IO_LAYER, my_layer);
 *      rv = PR_PushIOLayer(stack, PR_GetLayersIdentity(stack), my_layer)
 *
 * A string may be associated with the creation of the identity. It
 * will be copied by the runtime. If queried the runtime will return
 * a reference to that copied string (not yet another copy). There
 * is no facility for deleting an identity.
 **************************************************************************
 */

#define PR_IO_LAYER_HEAD (PRDescIdentity)-3
#define PR_INVALID_IO_LAYER (PRDescIdentity)-1
#define PR_TOP_IO_LAYER (PRDescIdentity)-2
#define PR_NSPR_IO_LAYER (PRDescIdentity)0

NSPR_API(PRDescIdentity) PR_GetUniqueIdentity(const char *layer_name);
NSPR_API(const char*) PR_GetNameForIdentity(PRDescIdentity ident);
NSPR_API(PRDescIdentity) PR_GetLayersIdentity(PRFileDesc* fd);
NSPR_API(PRFileDesc*) PR_GetIdentitiesLayer(PRFileDesc* fd_stack, PRDescIdentity id);

/*
 **************************************************************************
 * PR_GetDefaultIOMethods: Accessing the default methods table.
 * You may get a pointer to the default methods table by calling this function.
 * You may then select any elements from that table with which to build your
 * layer's methods table. You may NOT modify the table directly.
 **************************************************************************
 */
NSPR_API(const PRIOMethods *) PR_GetDefaultIOMethods(void);

/*
 **************************************************************************
 * Creating a layer
 *
 * A new layer may be allocated by calling PR_CreateIOLayerStub(). The
 * file descriptor returned will contain the pointer to the methods table
 * provided. The runtime will not modify the table nor test its correctness.
 **************************************************************************
 */
NSPR_API(PRFileDesc*) PR_CreateIOLayerStub(
    PRDescIdentity ident, const PRIOMethods *methods);

/*
 **************************************************************************
 * Creating a layer
 *
 * A new stack may be created by calling PR_CreateIOLayer(). The
 * file descriptor returned will point to the top of the stack, which has
 * the layer 'fd' as the topmost layer.
 *
 * NOTE: This function creates a new style stack, which has a fixed, dummy
 * header. The old style stack, created by a call to PR_PushIOLayer,
 * results in modifying contents of the top layer of the stack, when
 * pushing and popping layers of the stack.
 **************************************************************************
 */
NSPR_API(PRFileDesc*) PR_CreateIOLayer(PRFileDesc* fd);

/*
 **************************************************************************
 * Pushing a layer
 *
 * A file descriptor (perhaps allocated using PR_CreateIOLayerStub()) may
 * be pushed into an existing stack of file descriptors at any point the
 * caller deems appropriate. The new layer will be inserted into the stack
 * just above the layer with the indicated identity.
 *
 * Note: Even if the identity parameter indicates the top-most layer of
 * the stack, the value of the file descriptor describing the original
 * stack will not change.
 **************************************************************************
 */
NSPR_API(PRStatus) PR_PushIOLayer(
    PRFileDesc *fd_stack, PRDescIdentity id, PRFileDesc *layer);

/*
 **************************************************************************
 * Popping a layer
 *
 * A layer may be popped from a stack by indicating the identity of the
 * layer to be removed. If found, a pointer to the removed object will
 * be returned to the caller. The object then becomes the responsibility
 * of the caller.
 *
 * Note: Even if the identity indicates the top layer of the stack, the
 * reference returned will not be the file descriptor for the stack and
 * that file descriptor will remain valid.
 **************************************************************************
 */
NSPR_API(PRFileDesc*) PR_PopIOLayer(PRFileDesc *fd_stack, PRDescIdentity id);

/*
 **************************************************************************
 * FUNCTION:    PR_Open
 * DESCRIPTION:    Open a file for reading, writing, or both.
 * INPUTS:
 *     const char *name
 *         The path name of the file to be opened
 *     PRIntn flags
 *         The file status flags.
 *         It is a bitwise OR of the following bit flags (only one of
 *         the first three flags below may be used):
 *      PR_RDONLY        Open for reading only.
 *      PR_WRONLY        Open for writing only.
 *      PR_RDWR          Open for reading and writing.
 *      PR_CREATE_FILE   If the file does not exist, the file is created
 *                              If the file exists, this flag has no effect.
 *      PR_SYNC          If set, each write will wait for both the file data
 *                              and file status to be physically updated.
 *      PR_APPEND        The file pointer is set to the end of
 *                              the file prior to each write.
 *      PR_TRUNCATE      If the file exists, its length is truncated to 0.
 *      PR_EXCL          With PR_CREATE_FILE, if the file does not exist,
 *                              the file is created. If the file already
 *                              exists, no action and NULL is returned
 *
 *     PRIntn mode
 *         The access permission bits of the file mode, if the file is
 *         created when PR_CREATE_FILE is on.
 * OUTPUTS:    None
 * RETURNS:    PRFileDesc *
 *     If the file is successfully opened,
 *     returns a pointer to the PRFileDesc
 *     created for the newly opened file.
 *     Returns a NULL pointer if the open
 *     failed.
 * SIDE EFFECTS:
 * RESTRICTIONS:
 * MEMORY:
 *     The return value, if not NULL, points to a dynamically allocated
 *     PRFileDesc object.
 * ALGORITHM:
 **************************************************************************
 */

/* Open flags */
#define PR_RDONLY       0x01
#define PR_WRONLY       0x02
#define PR_RDWR         0x04
#define PR_CREATE_FILE  0x08
#define PR_APPEND       0x10
#define PR_TRUNCATE     0x20
#define PR_SYNC         0x40
#define PR_EXCL         0x80

/*
** File modes ....
**
** CAVEAT: 'mode' is currently only applicable on UNIX platforms.
** The 'mode' argument may be ignored by PR_Open on other platforms.
**
**   00400   Read by owner.
**   00200   Write by owner.
**   00100   Execute (search if a directory) by owner.
**   00040   Read by group.
**   00020   Write by group.
**   00010   Execute by group.
**   00004   Read by others.
**   00002   Write by others
**   00001   Execute by others.
**
*/

NSPR_API(PRFileDesc*) PR_Open(const char *name, PRIntn flags, PRIntn mode);

/*
 **************************************************************************
 * FUNCTION: PR_OpenFile
 * DESCRIPTION:
 *     Open a file for reading, writing, or both.
 *     PR_OpenFile has the same prototype as PR_Open but implements
 *     the specified file mode where possible.
 **************************************************************************
 */

/* File mode bits */
#define PR_IRWXU 00700  /* read, write, execute/search by owner */
#define PR_IRUSR 00400  /* read permission, owner */
#define PR_IWUSR 00200  /* write permission, owner */
#define PR_IXUSR 00100  /* execute/search permission, owner */
#define PR_IRWXG 00070  /* read, write, execute/search by group */
#define PR_IRGRP 00040  /* read permission, group */
#define PR_IWGRP 00020  /* write permission, group */
#define PR_IXGRP 00010  /* execute/search permission, group */
#define PR_IRWXO 00007  /* read, write, execute/search by others */
#define PR_IROTH 00004  /* read permission, others */
#define PR_IWOTH 00002  /* write permission, others */
#define PR_IXOTH 00001  /* execute/search permission, others */

NSPR_API(PRFileDesc*) PR_OpenFile(
    const char *name, PRIntn flags, PRIntn mode);

#ifdef MOZ_UNICODE
/*
 * EXPERIMENTAL: This function may be removed in a future release.
 */
NSPR_API(PRFileDesc*) PR_OpenFileUTF16(
    const PRUnichar *name, PRIntn flags, PRIntn mode);
#endif /* MOZ_UNICODE */

/*
 **************************************************************************
 * FUNCTION: PR_Close
 * DESCRIPTION:
 *     Close a file or socket.
 * INPUTS:
 *     PRFileDesc *fd
 *         a pointer to a PRFileDesc.
 * OUTPUTS:
 *     None.
 * RETURN:
 *     PRStatus
 * SIDE EFFECTS:
 * RESTRICTIONS:
 *     None.
 * MEMORY:
 *     The dynamic memory pointed to by the argument fd is freed.
 **************************************************************************
 */

NSPR_API(PRStatus)    PR_Close(PRFileDesc *fd);

/*
 **************************************************************************
 * FUNCTION: PR_Read
 * DESCRIPTION:
 *     Read bytes from a file or socket.
 *     The operation will block until either an end of stream indication is
 *     encountered, some positive number of bytes are transferred, or there
 *     is an error. No more than 'amount' bytes will be transferred.
 * INPUTS:
 *     PRFileDesc *fd
 *         pointer to the PRFileDesc object for the file or socket
 *     void *buf
 *         pointer to a buffer to hold the data read in.
 *     PRInt32 amount
 *         the size of 'buf' (in bytes)
 * OUTPUTS:
 * RETURN:
 *     PRInt32
 *         a positive number indicates the number of bytes actually read in.
 *         0 means end of file is reached or the network connection is closed.
 *         -1 indicates a failure. The reason for the failure is obtained
 *         by calling PR_GetError().
 * SIDE EFFECTS:
 *     data is written into the buffer pointed to by 'buf'.
 * RESTRICTIONS:
 *     None.
 * MEMORY:
 *     N/A
 * ALGORITHM:
 *     N/A
 **************************************************************************
 */

NSPR_API(PRInt32) PR_Read(PRFileDesc *fd, void *buf, PRInt32 amount);

/*
 ***************************************************************************
 * FUNCTION: PR_Write
 * DESCRIPTION:
 *     Write a specified number of bytes to a file or socket.  The thread
 *     invoking this function blocks until all the data is written.
 * INPUTS:
 *     PRFileDesc *fd
 *         pointer to a PRFileDesc object that refers to a file or socket
 *     const void *buf
 *         pointer to the buffer holding the data
 *     PRInt32 amount
 *         amount of data in bytes to be written from the buffer
 * OUTPUTS:
 *     None.
 * RETURN: PRInt32
 *     A positive number indicates the number of bytes successfully written.
 *     A -1 is an indication that the operation failed. The reason
 *     for the failure is obtained by calling PR_GetError().
 ***************************************************************************
 */

NSPR_API(PRInt32) PR_Write(PRFileDesc *fd,const void *buf,PRInt32 amount);

/*
 ***************************************************************************
 * FUNCTION: PR_Writev
 * DESCRIPTION:
 *     Write data to a socket.  The data is organized in a PRIOVec array. The
 *     operation will block until all the data is written or the operation
 *     fails.
 * INPUTS:
 *     PRFileDesc *fd
 *         Pointer that points to a PRFileDesc object for a socket.
 *     const PRIOVec *iov
 *         An array of PRIOVec.  PRIOVec is a struct with the following
 *         two fields:
 *             char *iov_base;
 *             int iov_len;
 *     PRInt32 iov_size
 *         Number of elements in the iov array. The value of this
 *         argument must not be greater than PR_MAX_IOVECTOR_SIZE.
 *         If it is, the method will fail (PR_BUFFER_OVERFLOW_ERROR).
 *     PRIntervalTime timeout
 *       Time limit for completion of the entire write operation.
 * OUTPUTS:
 *     None
 * RETURN:
 *     A positive number indicates the number of bytes successfully written.
 *     A -1 is an indication that the operation failed. The reason
 *     for the failure is obtained by calling PR_GetError().
 ***************************************************************************
 */

#define PR_MAX_IOVECTOR_SIZE 16   /* 'iov_size' must be <= */

NSPR_API(PRInt32) PR_Writev(
    PRFileDesc *fd, const PRIOVec *iov, PRInt32 iov_size,
    PRIntervalTime timeout);

/*
 ***************************************************************************
 * FUNCTION: PR_Delete
 * DESCRIPTION:
 *     Delete a file from the filesystem. The operation may fail if the
 *     file is open.
 * INPUTS:
 *     const char *name
 *         Path name of the file to be deleted.
 * OUTPUTS:
 *     None.
 * RETURN: PRStatus
 *     The function returns PR_SUCCESS if the file is successfully
 *     deleted, otherwise it returns PR_FAILURE.
 ***************************************************************************
 */

NSPR_API(PRStatus) PR_Delete(const char *name);

/**************************************************************************/

typedef enum PRFileType
{
    PR_FILE_FILE = 1,
    PR_FILE_DIRECTORY = 2,
    PR_FILE_OTHER = 3
} PRFileType;

struct PRFileInfo {
    PRFileType type;        /* Type of file */
    PROffset32 size;        /* Size, in bytes, of file's contents */
    PRTime creationTime;    /* Creation time per definition of PRTime */
    PRTime modifyTime;      /* Last modification time per definition of PRTime */
};

struct PRFileInfo64 {
    PRFileType type;        /* Type of file */
    PROffset64 size;        /* Size, in bytes, of file's contents */
    PRTime creationTime;    /* Creation time per definition of PRTime */
    PRTime modifyTime;      /* Last modification time per definition of PRTime */
};

/****************************************************************************
 * FUNCTION: PR_GetFileInfo, PR_GetFileInfo64
 * DESCRIPTION:
 *     Get the information about the file with the given path name. This is
 *     applicable only to NSFileDesc describing 'file' types (see
 * INPUTS:
 *     const char *fn
 *         path name of the file
 * OUTPUTS:
 *     PRFileInfo *info
 *         Information about the given file is written into the file
 *         information object pointer to by 'info'.
 * RETURN: PRStatus
 *     PR_GetFileInfo returns PR_SUCCESS if file information is successfully
 *     obtained, otherwise it returns PR_FAILURE.
 ***************************************************************************
 */

NSPR_API(PRStatus) PR_GetFileInfo(const char *fn, PRFileInfo *info);
NSPR_API(PRStatus) PR_GetFileInfo64(const char *fn, PRFileInfo64 *info);

#ifdef MOZ_UNICODE
/*
 * EXPERIMENTAL: This function may be removed in a future release.
 */
NSPR_API(PRStatus) PR_GetFileInfo64UTF16(const PRUnichar *fn, PRFileInfo64 *info);
#endif /* MOZ_UNICODE */

/*
 **************************************************************************
 * FUNCTION: PR_GetOpenFileInfo, PR_GetOpenFileInfo64
 * DESCRIPTION:
 *     Get information about an open file referred to by the
 *     given PRFileDesc object.
 * INPUTS:
 *     const PRFileDesc *fd
 *          A reference to a valid, open file.
 * OUTPUTS:
 *     Same as PR_GetFileInfo, PR_GetFileInfo64
 * RETURN: PRStatus
 *     PR_GetFileInfo returns PR_SUCCESS if file information is successfully
 *     obtained, otherwise it returns PR_FAILURE.
 ***************************************************************************
 */

NSPR_API(PRStatus) PR_GetOpenFileInfo(PRFileDesc *fd, PRFileInfo *info);
NSPR_API(PRStatus) PR_GetOpenFileInfo64(PRFileDesc *fd, PRFileInfo64 *info);

/*
 **************************************************************************
 * FUNCTION: PR_Rename
 * DESCRIPTION:
 *     Rename a file from the old name 'from' to the new name 'to'.
 * INPUTS:
 *     const char *from
 *         The old name of the file to be renamed.
 *     const char *to
 *         The new name of the file.
 * OUTPUTS:
 *     None.
 * RETURN: PRStatus
 **************************************************************************
 */

NSPR_API(PRStatus)    PR_Rename(const char *from, const char *to);

/*
 *************************************************************************
 * FUNCTION: PR_Access
 * DESCRIPTION:
 *     Determine accessibility of a file.
 * INPUTS:
 *     const char *name
 *         path name of the file
 *     PRAccessHow how
 *         specifies which access permission to check for.
 *         It can be one of the following values:
 *             PR_ACCESS_READ_OK       Test for read permission
 *             PR_ACCESS_WRITE_OK      Test for write permission
 *             PR_ACCESS_EXISTS        Check existence of file
 * OUTPUTS:
 *     None.
 * RETURN: PRStatus
 *     PR_SUCCESS is returned if the requested access is permitted.
 *     Otherwise, PR_FAILURE is returned. Additional information
 *     regarding the reason for the failure may be retrieved from
 *     PR_GetError().
 *************************************************************************
 */

typedef enum PRAccessHow {
    PR_ACCESS_EXISTS = 1,
    PR_ACCESS_WRITE_OK = 2,
    PR_ACCESS_READ_OK = 3
} PRAccessHow;

NSPR_API(PRStatus) PR_Access(const char *name, PRAccessHow how);

/*
 *************************************************************************
 * FUNCTION: PR_Seek, PR_Seek64
 * DESCRIPTION:
 *     Moves read-write file offset
 * INPUTS:
 *     PRFileDesc *fd
 *         Pointer to a PRFileDesc object.
 *     PROffset32, PROffset64 offset
 *         Specifies a value, in bytes, that is used in conjunction
 *         with the 'whence' parameter to set the file pointer.  A
 *         negative value causes seeking in the reverse direction.
 *     PRSeekWhence whence
 *         Specifies how to interpret the 'offset' parameter in setting
 *         the file pointer associated with the 'fd' parameter.
 *         Values for the 'whence' parameter are:
 *             PR_SEEK_SET  Sets the file pointer to the value of the
 *                          'offset' parameter
 *             PR_SEEK_CUR  Sets the file pointer to its current location
 *                          plus the value of the offset parameter.
 *             PR_SEEK_END  Sets the file pointer to the size of the
 *                          file plus the value of the offset parameter.
 * OUTPUTS:
 *     None.
 * RETURN: PROffset32, PROffset64
 *     Upon successful completion, the resulting pointer location,
 *     measured in bytes from the beginning of the file, is returned.
 *     If the PR_Seek() function fails, the file offset remains
 *     unchanged, and the returned value is -1. The error code can
 *     then be retrieved via PR_GetError().
 *************************************************************************
 */

NSPR_API(PROffset32) PR_Seek(PRFileDesc *fd, PROffset32 offset, PRSeekWhence whence);
NSPR_API(PROffset64) PR_Seek64(PRFileDesc *fd, PROffset64 offset, PRSeekWhence whence);

/*
 ************************************************************************
 * FUNCTION: PR_Available
 * DESCRIPTION:
 *     Determine the amount of data in bytes available for reading
 *     in the given file or socket.
 * INPUTS:
 *     PRFileDesc *fd
 *         Pointer to a PRFileDesc object that refers to a file or
 *         socket.
 * OUTPUTS:
 *     None
 * RETURN: PRInt32, PRInt64
 *     Upon successful completion, PR_Available returns the number of
 *     bytes beyond the current read pointer that is available for
 *     reading.  Otherwise, it returns a -1 and the reason for the
 *     failure can be retrieved via PR_GetError().
 ************************************************************************
 */

NSPR_API(PRInt32) PR_Available(PRFileDesc *fd);
NSPR_API(PRInt64) PR_Available64(PRFileDesc *fd);

/*
 ************************************************************************
 * FUNCTION: PR_Sync
 * DESCRIPTION:
 *     Sync any buffered data for a fd to its backing device (disk).
 * INPUTS:
 *     PRFileDesc *fd
 *         Pointer to a PRFileDesc object that refers to a file or
 *         socket
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *     PR_SUCCESS is returned if the requested access is permitted.
 *     Otherwise, PR_FAILURE is returned.
 ************************************************************************
 */

NSPR_API(PRStatus)  PR_Sync(PRFileDesc *fd);

/************************************************************************/

struct PRDirEntry {
    const char *name;        /* name of entry, relative to directory name */
};

#ifdef MOZ_UNICODE
struct PRDirEntryUTF16 {
    const PRUnichar *name;   /* name of entry in UTF16, relative to
                              * directory name */
};
#endif /* MOZ_UNICODE */

#if !defined(NO_NSPR_10_SUPPORT)
#define PR_DirName(dirEntry)    (dirEntry->name)
#endif

/*
 *************************************************************************
 * FUNCTION: PR_OpenDir
 * DESCRIPTION:
 *     Open the directory by the given name
 * INPUTS:
 *     const char *name
 *         path name of the directory to be opened
 * OUTPUTS:
 *     None
 * RETURN: PRDir *
 *     If the directory is sucessfully opened, a PRDir object is
 *     dynamically allocated and a pointer to it is returned.
 *     If the directory cannot be opened, a NULL pointer is returned.
 * MEMORY:
 *     Upon successful completion, the return value points to
 *     dynamically allocated memory.
 *************************************************************************
 */

NSPR_API(PRDir*) PR_OpenDir(const char *name);

#ifdef MOZ_UNICODE
/*
 * EXPERIMENTAL: This function may be removed in a future release.
 */
NSPR_API(PRDirUTF16*) PR_OpenDirUTF16(const PRUnichar *name);
#endif /* MOZ_UNICODE */

/*
 *************************************************************************
 * FUNCTION: PR_ReadDir
 * DESCRIPTION:
 * INPUTS:
 *     PRDir *dir
 *         pointer to a PRDir object that designates an open directory
 *     PRDirFlags flags
 *           PR_SKIP_NONE     Do not skip any files
 *           PR_SKIP_DOT      Skip the directory entry "." that
 *                            represents the current directory
 *           PR_SKIP_DOT_DOT  Skip the directory entry ".." that
 *                            represents the parent directory.
 *           PR_SKIP_BOTH     Skip both '.' and '..'
 *           PR_SKIP_HIDDEN   Skip hidden files
 * OUTPUTS:
 * RETURN: PRDirEntry*
 *     Returns a pointer to the next entry in the directory.  Returns
 *     a NULL pointer upon reaching the end of the directory or when an
 *     error occurs. The actual reason can be retrieved via PR_GetError().
 *************************************************************************
 */

typedef enum PRDirFlags {
    PR_SKIP_NONE = 0x0,
    PR_SKIP_DOT = 0x1,
    PR_SKIP_DOT_DOT = 0x2,
    PR_SKIP_BOTH = 0x3,
    PR_SKIP_HIDDEN = 0x4
} PRDirFlags;

NSPR_API(PRDirEntry*) PR_ReadDir(PRDir *dir, PRDirFlags flags);

#ifdef MOZ_UNICODE
/*
 * EXPERIMENTAL: This function may be removed in a future release.
 */
NSPR_API(PRDirEntryUTF16*) PR_ReadDirUTF16(PRDirUTF16 *dir, PRDirFlags flags);
#endif /* MOZ_UNICODE */

/*
 *************************************************************************
 * FUNCTION: PR_CloseDir
 * DESCRIPTION:
 *     Close the specified directory.
 * INPUTS:
 *     PRDir *dir
 *        The directory to be closed.
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *        If successful, will return a status of PR_SUCCESS. Otherwise
 *        a value of PR_FAILURE. The reason for the failure may be re-
 *        trieved using PR_GetError().
 *************************************************************************
 */

NSPR_API(PRStatus) PR_CloseDir(PRDir *dir);

#ifdef MOZ_UNICODE
/*
 * EXPERIMENTAL: This function may be removed in a future release.
 */
NSPR_API(PRStatus) PR_CloseDirUTF16(PRDirUTF16 *dir);
#endif /* MOZ_UNICODE */

/*
 *************************************************************************
 * FUNCTION: PR_MkDir
 * DESCRIPTION:
 *     Create a new directory with the given name and access mode.
 * INPUTS:
 *     const char *name
 *        The name of the directory to be created. All the path components
 *        up to but not including the leaf component must already exist.
 *     PRIntn mode
 *        See 'mode' definiton in PR_Open().
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *        If successful, will return a status of PR_SUCCESS. Otherwise
 *        a value of PR_FAILURE. The reason for the failure may be re-
 *        trieved using PR_GetError().
 *************************************************************************
 */

NSPR_API(PRStatus) PR_MkDir(const char *name, PRIntn mode);

/*
 *************************************************************************
 * FUNCTION: PR_MakeDir
 * DESCRIPTION:
 *     Create a new directory with the given name and access mode.
 *     PR_MakeDir has the same prototype as PR_MkDir but implements
 *     the specified access mode where possible.
 *************************************************************************
 */

NSPR_API(PRStatus) PR_MakeDir(const char *name, PRIntn mode);

/*
 *************************************************************************
 * FUNCTION: PR_RmDir
 * DESCRIPTION:
 *     Remove a directory by the given name.
 * INPUTS:
 *     const char *name
 *        The name of the directory to be removed. All the path components
 *        must already exist. Only the leaf component will be removed.
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *        If successful, will return a status of PR_SUCCESS. Otherwise
 *        a value of PR_FAILURE. The reason for the failure may be re-
 *        trieved using PR_GetError().
 **************************************************************************
 */

NSPR_API(PRStatus) PR_RmDir(const char *name);

/*
 *************************************************************************
 * FUNCTION: PR_NewUDPSocket
 * DESCRIPTION:
 *     Create a new UDP socket.
 * INPUTS:
 *     None
 * OUTPUTS:
 *     None
 * RETURN: PRFileDesc*
 *     Upon successful completion, PR_NewUDPSocket returns a pointer
 *     to the PRFileDesc created for the newly opened UDP socket.
 *     Returns a NULL pointer if the creation of a new UDP socket failed.
 *
 **************************************************************************
 */

NSPR_API(PRFileDesc*)    PR_NewUDPSocket(void);

/*
 *************************************************************************
 * FUNCTION: PR_NewTCPSocket
 * DESCRIPTION:
 *     Create a new TCP socket.
 * INPUTS:
 *     None
 * OUTPUTS:
 *     None
 * RETURN: PRFileDesc*
 *     Upon successful completion, PR_NewTCPSocket returns a pointer
 *     to the PRFileDesc created for the newly opened TCP socket.
 *     Returns a NULL pointer if the creation of a new TCP socket failed.
 *
 **************************************************************************
 */

NSPR_API(PRFileDesc*)    PR_NewTCPSocket(void);

/*
 *************************************************************************
 * FUNCTION: PR_OpenUDPSocket
 * DESCRIPTION:
 *     Create a new UDP socket of the specified address family.
 * INPUTS:
 *     PRIntn af
 *       Address family
 * OUTPUTS:
 *     None
 * RETURN: PRFileDesc*
 *     Upon successful completion, PR_OpenUDPSocket returns a pointer
 *     to the PRFileDesc created for the newly opened UDP socket.
 *     Returns a NULL pointer if the creation of a new UDP socket failed.
 *
 **************************************************************************
 */

NSPR_API(PRFileDesc*)    PR_OpenUDPSocket(PRIntn af);

/*
 *************************************************************************
 * FUNCTION: PR_OpenTCPSocket
 * DESCRIPTION:
 *     Create a new TCP socket of the specified address family.
 * INPUTS:
 *     PRIntn af
 *       Address family
 * OUTPUTS:
 *     None
 * RETURN: PRFileDesc*
 *     Upon successful completion, PR_NewTCPSocket returns a pointer
 *     to the PRFileDesc created for the newly opened TCP socket.
 *     Returns a NULL pointer if the creation of a new TCP socket failed.
 *
 **************************************************************************
 */

NSPR_API(PRFileDesc*)    PR_OpenTCPSocket(PRIntn af);

/*
 *************************************************************************
 * FUNCTION: PR_Connect
 * DESCRIPTION:
 *     Initiate a connection on a socket.
 * INPUTS:
 *     PRFileDesc *fd
 *       Points to a PRFileDesc object representing a socket
 *     PRNetAddr *addr
 *       Specifies the address of the socket in its own communication
 *       space.
 *     PRIntervalTime timeout
 *       The function uses the lesser of the provided timeout and
 *       the OS's connect timeout.  In particular, if you specify
 *       PR_INTERVAL_NO_TIMEOUT as the timeout, the OS's connection
 *       time limit will be used.
 *
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *     Upon successful completion of connection initiation, PR_Connect
 *     returns PR_SUCCESS.  Otherwise, it returns PR_FAILURE.  Further
 *     failure information can be obtained by calling PR_GetError().
 **************************************************************************
 */

NSPR_API(PRStatus) PR_Connect(
    PRFileDesc *fd, const PRNetAddr *addr, PRIntervalTime timeout);

/*
 *************************************************************************
 * FUNCTION: PR_ConnectContinue
 * DESCRIPTION:
 *     Continue a nonblocking connect.  After a nonblocking connect
 *     is initiated with PR_Connect() (which fails with
 *     PR_IN_PROGRESS_ERROR), one should call PR_Poll() on the socket,
 *     with the in_flags PR_POLL_WRITE | PR_POLL_EXCEPT.  When
 *     PR_Poll() returns, one calls PR_ConnectContinue() on the
 *     socket to determine whether the nonblocking connect has
 *     completed or is still in progress.  Repeat the PR_Poll(),
 *     PR_ConnectContinue() sequence until the nonblocking connect
 *     has completed.
 * INPUTS:
 *     PRFileDesc *fd
 *         the file descriptor representing a socket
 *     PRInt16 out_flags
 *         the out_flags field of the poll descriptor returned by
 *         PR_Poll()
 * RETURN: PRStatus
 *     If the nonblocking connect has successfully completed,
 *     PR_ConnectContinue returns PR_SUCCESS.  If PR_ConnectContinue()
 *     returns PR_FAILURE, call PR_GetError():
 *     - PR_IN_PROGRESS_ERROR: the nonblocking connect is still in
 *       progress and has not completed yet.  The caller should poll
 *       on the file descriptor for the in_flags
 *       PR_POLL_WRITE|PR_POLL_EXCEPT and retry PR_ConnectContinue
 *       later when PR_Poll() returns.
 *     - Other errors: the nonblocking connect has failed with this
 *       error code.
 */

NSPR_API(PRStatus)    PR_ConnectContinue(PRFileDesc *fd, PRInt16 out_flags);

/*
 *************************************************************************
 * THIS FUNCTION IS DEPRECATED.  USE PR_ConnectContinue INSTEAD.
 *
 * FUNCTION: PR_GetConnectStatus
 * DESCRIPTION:
 *     Get the completion status of a nonblocking connect.  After
 *     a nonblocking connect is initiated with PR_Connect() (which
 *     fails with PR_IN_PROGRESS_ERROR), one should call PR_Poll()
 *     on the socket, with the in_flags PR_POLL_WRITE | PR_POLL_EXCEPT.
 *     When PR_Poll() returns, one calls PR_GetConnectStatus on the
 *     PRPollDesc structure to determine whether the nonblocking
 *     connect has succeeded or failed.
 * INPUTS:
 *     const PRPollDesc *pd
 *         Pointer to a PRPollDesc whose fd member is the socket,
 *         and in_flags must contain PR_POLL_WRITE and PR_POLL_EXCEPT.
 *         PR_Poll() should have been called and set the out_flags.
 * RETURN: PRStatus
 *     If the nonblocking connect has successfully completed,
 *     PR_GetConnectStatus returns PR_SUCCESS.  If PR_GetConnectStatus()
 *     returns PR_FAILURE, call PR_GetError():
 *     - PR_IN_PROGRESS_ERROR: the nonblocking connect is still in
 *       progress and has not completed yet.
 *     - Other errors: the nonblocking connect has failed with this
 *       error code.
 */

NSPR_API(PRStatus)    PR_GetConnectStatus(const PRPollDesc *pd);

/*
 *************************************************************************
 * FUNCTION: PR_Accept
 * DESCRIPTION:
 *     Accept a connection on a socket.
 * INPUTS:
 *     PRFileDesc *fd
 *       Points to a PRFileDesc object representing the rendezvous socket
 *       on which the caller is willing to accept new connections.
 *     PRIntervalTime timeout
 *       Time limit for completion of the accept operation.
 * OUTPUTS:
 *     PRNetAddr *addr
 *       Returns the address of the connecting entity in its own
 *       communication space. It may be NULL.
 * RETURN: PRFileDesc*
 *     Upon successful acceptance of a connection, PR_Accept
 *     returns a valid file descriptor. Otherwise, it returns NULL.
 *     Further failure information can be obtained by calling PR_GetError().
 **************************************************************************
 */

NSPR_API(PRFileDesc*) PR_Accept(
    PRFileDesc *fd, PRNetAddr *addr, PRIntervalTime timeout);

/*
 *************************************************************************
 * FUNCTION: PR_Bind
 * DESCRIPTION:
 *    Bind an address to a socket.
 * INPUTS:
 *     PRFileDesc *fd
 *       Points to a PRFileDesc object representing a socket.
 *     PRNetAddr *addr
 *       Specifies the address to which the socket will be bound.
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *     Upon successful binding of an address to a socket, PR_Bind
 *     returns PR_SUCCESS.  Otherwise, it returns PR_FAILURE.  Further
 *     failure information can be obtained by calling PR_GetError().
 **************************************************************************
 */

NSPR_API(PRStatus) PR_Bind(PRFileDesc *fd, const PRNetAddr *addr);

/*
 *************************************************************************
 * FUNCTION: PR_Listen
 * DESCRIPTION:
 *    Listen for connections on a socket.
 * INPUTS:
 *     PRFileDesc *fd
 *       Points to a PRFileDesc object representing a socket that will be
 *       used to listen for new connections.
 *     PRIntn backlog
 *       Specifies the maximum length of the queue of pending connections.
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *     Upon successful completion of listen request, PR_Listen
 *     returns PR_SUCCESS.  Otherwise, it returns PR_FAILURE.  Further
 *     failure information can be obtained by calling PR_GetError().
 **************************************************************************
 */

NSPR_API(PRStatus) PR_Listen(PRFileDesc *fd, PRIntn backlog);

/*
 *************************************************************************
 * FUNCTION: PR_Shutdown
 * DESCRIPTION:
 *    Shut down part of a full-duplex connection on a socket.
 * INPUTS:
 *     PRFileDesc *fd
 *       Points to a PRFileDesc object representing a connected socket.
 *     PRIntn how
 *       Specifies the kind of disallowed operations on the socket.
 *           PR_SHUTDOWN_RCV - Further receives will be disallowed
 *           PR_SHUTDOWN_SEND - Further sends will be disallowed
 *           PR_SHUTDOWN_BOTH - Further sends and receives will be disallowed
 * OUTPUTS:
 *     None
 * RETURN: PRStatus
 *     Upon successful completion of shutdown request, PR_Shutdown
 *     returns PR_SUCCESS.  Otherwise, it returns PR_FAILURE.  Further
 *     failure information can be obtained by calling PR_GetError().
 **************************************************************************
 */

typedef enum PRShutdownHow
{
    PR_SHUTDOWN_RCV = 0,      /* disallow further receives */
    PR_SHUTDOWN_SEND = 1,     /* disallow further sends */
    PR_SHUTDOWN_BOTH = 2      /* disallow further receives and sends */
} PRShutdownHow;

NSPR_API(PRStatus)    PR_Shutdown(PRFileDesc *fd, PRShutdownHow how);

/*
 *************************************************************************
 * FUNCTION: PR_Recv
 * DESCRIPTION:
 *    Receive a specified number of bytes from a connected socket.
 *     The operation will block until some positive number of bytes are
 *     transferred, a time out has occurred, or there is an error.
 *     No more than 'amount' bytes will be transferred.
 * INPUTS:
 *     PRFileDesc *fd
 *       points to a PRFileDesc object representing a socket.
 *     void *buf
 *       pointer to a buffer to hold the data received.
 *     PRInt32 amount
 *       the size of 'buf' (in bytes)
 *     PRIntn flags
 *       must be zero or PR_MSG_PEEK.
 *     PRIntervalTime timeout
 *       Time limit for completion of the receive operation.
 * OUTPUTS:
 *     None
 * RETURN: PRInt32
 *         a positive number indicates the number of bytes actually received.
 *         0 means the network connection is closed.
 *         -1 indicates a failure. The reason for the failure is obtained
 *         by calling PR_GetError().
 **************************************************************************
 */

#define PR_MSG_PEEK 0x2

NSPR_API(PRInt32)    PR_Recv(PRFileDesc *fd, void *buf, PRInt32 amount,
                             PRIntn flags, PRIntervalTime timeout);

/*
 *************************************************************************
 * FUNCTION: PR_Send
 * DESCRIPTION:
 *    Send a specified number of bytes from a connected socket.
 *     The operation will block until all bytes are
 *     processed, a time out has occurred, or there is an error.
 * INPUTS:
 *     PRFileDesc *fd
 *       points to a PRFileDesc object representing a socket.
 *     void *buf
 *       pointer to a buffer from where the data is sent.
 *     PRInt32 amount
 *       the size of 'buf' (in bytes)
 *     PRIntn flags
 *        (OBSOLETE - must always be zero)
 *     PRIntervalTime timeout
 *       Time limit for completion of the send operation.
 * OUTPUTS:
 *     None
 * RETURN: PRInt32
 *     A positive number indicates the number of bytes successfully processed.
 *     This number must always equal 'amount'. A -1 is an indication that the
 *     operation failed. The reason for the failure is obtained by calling
 *     PR_GetError().
 **************************************************************************
 */

NSPR_API(PRInt32)    PR_Send(PRFileDesc *fd, const void *buf, PRInt32 amount,
                             PRIntn flags, PRIntervalTime timeout);

/*
 *************************************************************************
 * FUNCTION: PR_RecvFrom
 * DESCRIPTION:
 *     Receive up to a specified number of bytes from socket which may
 *     or may not be connected.
 *     The operation will block until one or more bytes are
 *     transferred, a time out has occurred, or there is an error.
 *     No more than 'amount' bytes will be transferred.
 * INPUTS:
 *     PRFileDesc *fd
 *       points to a PRFileDesc object representing a socket.
 *     void *buf
 *       pointer to a buffer to hold the data received.
 *     PRInt32 amount
 *       the size of 'buf' (in bytes)
 *     PRIntn flags
 *        (OBSOLETE - must always be zero)
 *     PRNetAddr *addr
 *       Specifies the address of the sending peer. It may be NULL.
 *     PRIntervalTime timeout
 *       Time limit for completion of the receive operation.
 * OUTPUTS:
 *     None
 * RETURN: PRInt32
 *         a positive number indicates the number of bytes actually received.
 *         0 means the network connection is closed.
 *         -1 indicates a failure. The reason for the failure is obtained
 *         by calling PR_GetError().
 **************************************************************************
 */

NSPR_API(PRInt32) PR_RecvFrom(
    PRFileDesc *fd, void *buf, PRInt32 amount, PRIntn flags,
    PRNetAddr *addr, PRIntervalTime timeout);

/*
 *************************************************************************
 * FUNCTION: PR_SendTo
 * DESCRIPTION:
 *    Send a specified number of bytes from an unconnected socket.
 *    The operation will block until all bytes are
 *    sent, a time out has occurred, or there is an error.
 * INPUTS:
 *     PRFileDesc *fd
 *       points to a PRFileDesc object representing an unconnected socket.
 *     void *buf
 *       pointer to a buffer from where the data is sent.
 *     PRInt32 amount
 *       the size of 'buf' (in bytes)
 *     PRIntn flags
 *        (OBSOLETE - must always be zero)
 *     PRNetAddr *addr
 *       Specifies the address of the peer.
.*     PRIntervalTime timeout
 *       Time limit for completion of the send operation.
 * OUTPUTS:
 *     None
 * RETURN: PRInt32
 *     A positive number indicates the number of bytes successfully sent.
 *     -1 indicates a failure. The reason for the failure is obtained
 *     by calling PR_GetError().
 **************************************************************************
 */

NSPR_API(PRInt32) PR_SendTo(
    PRFileDesc *fd, const void *buf, PRInt32 amount, PRIntn flags,
    const PRNetAddr *addr, PRIntervalTime timeout);

/*
*************************************************************************
** FUNCTION: PR_TransmitFile
** DESCRIPTION:
**    Transmitfile sends a complete file (sourceFile) across a socket
**    (networkSocket).  If headers is non-NULL, the headers will be sent across
**    the socket prior to sending the file.
**
**    Optionally, the PR_TRANSMITFILE_CLOSE_SOCKET flag may be passed to
**    transmitfile.  This flag specifies that transmitfile should close the
**    socket after sending the data.
**
** INPUTS:
**    PRFileDesc *networkSocket
**        The socket to send data over
**    PRFileDesc *sourceFile
**        The file to send
**    const void *headers
**        A pointer to headers to be sent before sending data
**    PRInt32       hlen
**        length of header buffers in bytes.
**    PRTransmitFileFlags       flags
**        If the flags indicate that the connection should be closed,
**        it will be done immediately after transferring the file, unless
**        the operation is unsuccessful.
.*     PRIntervalTime timeout
 *        Time limit for completion of the transmit operation.
**
** RETURNS:
**    Returns the number of bytes written or -1 if the operation failed.
**    If an error occurs while sending the file, the PR_TRANSMITFILE_CLOSE_
**    SOCKET flag is ignored. The reason for the failure is obtained
**    by calling PR_GetError().
**************************************************************************
*/

NSPR_API(PRInt32) PR_TransmitFile(
    PRFileDesc *networkSocket, PRFileDesc *sourceFile,
    const void *headers, PRInt32 hlen, PRTransmitFileFlags flags,
    PRIntervalTime timeout);

/*
*************************************************************************
** FUNCTION: PR_SendFile
** DESCRIPTION:
**    PR_SendFile sends data from a file (sendData->fd) across a socket
**    (networkSocket).  If specified, a header and/or trailer buffer are sent
**    before and after the file, respectively. The file offset, number of bytes
**    of file data to send, the header and trailer buffers are specified in the
**    sendData argument.
**
**    Optionally, if the PR_TRANSMITFILE_CLOSE_SOCKET flag is passed, the
**    socket is closed after successfully sending the data.
**
** INPUTS:
**    PRFileDesc *networkSocket
**        The socket to send data over
**    PRSendFileData *sendData
**        Contains the FD, file offset and length, header and trailer
**        buffer specifications.
**    PRTransmitFileFlags       flags
**        If the flags indicate that the connection should be closed,
**        it will be done immediately after transferring the file, unless
**        the operation is unsuccessful.
.*     PRIntervalTime timeout
 *        Time limit for completion of the send operation.
**
** RETURNS:
**    Returns the number of bytes written or -1 if the operation failed.
**    If an error occurs while sending the file, the PR_TRANSMITFILE_CLOSE_
**    SOCKET flag is ignored. The reason for the failure is obtained
**    by calling PR_GetError().
**************************************************************************
*/

struct PRSendFileData {
    PRFileDesc  *fd;            /* file to send                         */
    PRUint32    file_offset;    /* file offset                          */
    PRSize      file_nbytes;    /* number of bytes of file data to send */
    /* if 0, send data from file_offset to  */
    /* end-of-file.                         */
    const void  *header;        /* header buffer                        */
    PRInt32     hlen;           /* header len                           */
    const void  *trailer;       /* trailer buffer                       */
    PRInt32     tlen;           /* trailer len                          */
};


NSPR_API(PRInt32) PR_SendFile(
    PRFileDesc *networkSocket, PRSendFileData *sendData,
    PRTransmitFileFlags flags, PRIntervalTime timeout);

/*
*************************************************************************
** FUNCTION: PR_AcceptRead
** DESCRIPTION:
**    AcceptRead accepts a new connection, returns the newly created
**    socket's descriptor and also returns the connecting peer's address.
**    AcceptRead, as its name suggests, also receives the first block of data
**    sent by the peer.
**
** INPUTS:
**    PRFileDesc *listenSock
**        A socket descriptor that has been called with the PR_Listen()
**        function, also known as the rendezvous socket.
**    void *buf
**        A pointer to a buffer to receive data sent by the client.  This
**        buffer must be large enough to receive <amount> bytes of data
**        and two PRNetAddr structures, plus an extra 32 bytes. See:
**        PR_ACCEPT_READ_BUF_OVERHEAD.
**    PRInt32 amount
**        The number of bytes of client data to receive.  Does not include
**        the size of the PRNetAddr structures.  If 0, no data will be read
**        from the client.
**    PRIntervalTime timeout
**        The timeout interval only applies to the read portion of the
**        operation.  PR_AcceptRead will block indefinitely until the
**        connection is accepted; the read will timeout after the timeout
**        interval elapses.
** OUTPUTS:
**    PRFileDesc **acceptedSock
**        The file descriptor for the newly connected socket.  This parameter
**        will only be valid if the function return does not indicate failure.
**    PRNetAddr  **peerAddr,
**        The address of the remote socket.  This parameter will only be
**        valid if the function return does not indicate failure.  The
**        returned address is not guaranteed to be properly aligned.
**
** RETURNS:
**     The number of bytes read from the client or -1 on failure.  The reason
**     for the failure is obtained by calling PR_GetError().
**************************************************************************
**/
/* define buffer overhead constant. Add this value to the user's
** data length when allocating a buffer to accept data.
**    Example:
**    #define USER_DATA_SIZE 10
**    char buf[USER_DATA_SIZE + PR_ACCEPT_READ_BUF_OVERHEAD];
**    bytesRead = PR_AcceptRead( s, fd, &a, &p, USER_DATA_SIZE, ...);
*/
#define PR_ACCEPT_READ_BUF_OVERHEAD (32+(2*sizeof(PRNetAddr)))

NSPR_API(PRInt32) PR_AcceptRead(
    PRFileDesc *listenSock, PRFileDesc **acceptedSock,
    PRNetAddr **peerAddr, void *buf, PRInt32 amount, PRIntervalTime timeout);

/*
*************************************************************************
** FUNCTION: PR_NewTCPSocketPair
** DESCRIPTION:
**    Create a new TCP socket pair. The returned descriptors can be used
**    interchangeably; they are interconnected full-duplex descriptors: data
**    written to one can be read from the other and vice-versa.
**
** INPUTS:
**    None
** OUTPUTS:
**    PRFileDesc *fds[2]
**        The file descriptor pair for the newly created TCP sockets.
** RETURN: PRStatus
**     Upon successful completion of TCP socket pair, PR_NewTCPSocketPair
**     returns PR_SUCCESS.  Otherwise, it returns PR_FAILURE.  Further
**     failure information can be obtained by calling PR_GetError().
** XXX can we implement this on windoze and mac?
**************************************************************************
**/
NSPR_API(PRStatus) PR_NewTCPSocketPair(PRFileDesc *fds[2]);

/*
*************************************************************************
** FUNCTION: PR_GetSockName
** DESCRIPTION:
**    Get socket name.  Return the network address for this socket.
**
** INPUTS:
**     PRFileDesc *fd
**       Points to a PRFileDesc object representing the socket.
** OUTPUTS:
**     PRNetAddr *addr
**       Returns the address of the socket in its own communication space.
** RETURN: PRStatus
**     Upon successful completion, PR_GetSockName returns PR_SUCCESS.
**     Otherwise, it returns PR_FAILURE.  Further failure information can
**     be obtained by calling PR_GetError().
**************************************************************************
**/
NSPR_API(PRStatus)  PR_GetSockName(PRFileDesc *fd, PRNetAddr *addr);

/*
*************************************************************************
** FUNCTION: PR_GetPeerName
** DESCRIPTION:
**    Get name of the connected peer.  Return the network address for the
**    connected peer socket.
**
** INPUTS:
**     PRFileDesc *fd
**       Points to a PRFileDesc object representing the connected peer.
** OUTPUTS:
**     PRNetAddr *addr
**       Returns the address of the connected peer in its own communication
**       space.
** RETURN: PRStatus
**     Upon successful completion, PR_GetPeerName returns PR_SUCCESS.
**     Otherwise, it returns PR_FAILURE.  Further failure information can
**     be obtained by calling PR_GetError().
**************************************************************************
**/
NSPR_API(PRStatus)  PR_GetPeerName(PRFileDesc *fd, PRNetAddr *addr);

NSPR_API(PRStatus)  PR_GetSocketOption(
    PRFileDesc *fd, PRSocketOptionData *data);

NSPR_API(PRStatus)  PR_SetSocketOption(
    PRFileDesc *fd, const PRSocketOptionData *data);

/*
 *********************************************************************
 *
 * File descriptor inheritance
 *
 *********************************************************************
 */

/*
 ************************************************************************
 * FUNCTION: PR_SetFDInheritable
 * DESCRIPTION:
 *    Set the inheritance attribute of a file descriptor.
 *
 * INPUTS:
 *     PRFileDesc *fd
 *       Points to a PRFileDesc object.
 *     PRBool inheritable
 *       If PR_TRUE, the file descriptor fd is set to be inheritable
 *       by a child process.  If PR_FALSE, the file descriptor is set
 *       to be not inheritable by a child process.
 * RETURN: PRStatus
 *     Upon successful completion, PR_SetFDInheritable returns PR_SUCCESS.
 *     Otherwise, it returns PR_FAILURE.  Further failure information can
 *     be obtained by calling PR_GetError().
 *************************************************************************
 */
NSPR_API(PRStatus) PR_SetFDInheritable(
    PRFileDesc *fd,
    PRBool inheritable);

/*
 ************************************************************************
 * FUNCTION: PR_GetInheritedFD
 * DESCRIPTION:
 *    Get an inherited file descriptor with the specified name.
 *
 * INPUTS:
 *     const char *name
 *       The name of the inherited file descriptor.
 * RETURN: PRFileDesc *
 *     Upon successful completion, PR_GetInheritedFD returns the
 *     inherited file descriptor with the specified name.  Otherwise,
 *     it returns NULL.  Further failure information can be obtained
 *     by calling PR_GetError().
 *************************************************************************
 */
NSPR_API(PRFileDesc *) PR_GetInheritedFD(const char *name);

/*
 *********************************************************************
 *
 * Memory-mapped files
 *
 *********************************************************************
 */

typedef struct PRFileMap PRFileMap;

/*
 * protection options for read and write accesses of a file mapping
 */
typedef enum PRFileMapProtect {
    PR_PROT_READONLY,     /* read only */
    PR_PROT_READWRITE,    /* readable, and write is shared */
    PR_PROT_WRITECOPY     /* readable, and write is private (copy-on-write) */
} PRFileMapProtect;

NSPR_API(PRFileMap *) PR_CreateFileMap(
    PRFileDesc *fd,
    PRInt64 size,
    PRFileMapProtect prot);

/*
 * return the alignment (in bytes) of the offset argument to PR_MemMap
 */
NSPR_API(PRInt32) PR_GetMemMapAlignment(void);

NSPR_API(void *) PR_MemMap(
    PRFileMap *fmap,
    PROffset64 offset,  /* must be aligned and sized according to the
                         * return value of PR_GetMemMapAlignment() */
    PRUint32 len);

NSPR_API(PRStatus) PR_MemUnmap(void *addr, PRUint32 len);

NSPR_API(PRStatus) PR_CloseFileMap(PRFileMap *fmap);

/*
 * Synchronously flush the given memory-mapped address range of the given open
 * file to disk. The function does not return until all modified data have
 * been written to disk.
 *
 * On some platforms, the function will call PR_Sync(fd) internally if it is
 * necessary for flushing modified data to disk synchronously.
 */
NSPR_API(PRStatus) PR_SyncMemMap(
    PRFileDesc *fd,
    void *addr,
    PRUint32 len);

/*
 ******************************************************************
 *
 * Interprocess communication
 *
 ******************************************************************
 */

/*
 * Creates an anonymous pipe and returns file descriptors for the
 * read and write ends of the pipe.
 */

NSPR_API(PRStatus) PR_CreatePipe(
    PRFileDesc **readPipe,
    PRFileDesc **writePipe
);

/************************************************************************/
/************** The following definitions are for poll ******************/
/************************************************************************/

struct PRPollDesc {
    PRFileDesc* fd;
    PRInt16 in_flags;
    PRInt16 out_flags;
};

/*
** Bit values for PRPollDesc.in_flags or PRPollDesc.out_flags. Binary-or
** these together to produce the desired poll request.
*/

#if defined(_PR_POLL_BACKCOMPAT)

#include <poll.h>
#define PR_POLL_READ    POLLIN
#define PR_POLL_WRITE   POLLOUT
#define PR_POLL_EXCEPT  POLLPRI
#define PR_POLL_ERR     POLLERR     /* only in out_flags */
#define PR_POLL_NVAL    POLLNVAL    /* only in out_flags when fd is bad */
#define PR_POLL_HUP     POLLHUP     /* only in out_flags */

#else  /* _PR_POLL_BACKCOMPAT */

#define PR_POLL_READ    0x1
#define PR_POLL_WRITE   0x2
#define PR_POLL_EXCEPT  0x4
#define PR_POLL_ERR     0x8         /* only in out_flags */
#define PR_POLL_NVAL    0x10        /* only in out_flags when fd is bad */
#define PR_POLL_HUP     0x20        /* only in out_flags */

#endif  /* _PR_POLL_BACKCOMPAT */

/*
*************************************************************************
** FUNCTION:    PR_Poll
** DESCRIPTION:
**
** The call returns as soon as I/O is ready on one or more of the underlying
** socket objects. A count of the number of ready descriptors is
** returned unless a timeout occurs in which case zero is returned.
**
** PRPollDesc.fd should be set to a pointer to a PRFileDesc object
** representing a socket. This field can be set to NULL to indicate to
** PR_Poll that this PRFileDesc object should be ignored.
** PRPollDesc.in_flags should be set to the desired request
** (read/write/except or some combination). Upon successful return from
** this call PRPollDesc.out_flags will be set to indicate what kind of
** i/o can be performed on the respective descriptor. PR_Poll() uses the
** out_flags fields as scratch variables during the call. If PR_Poll()
** returns 0 or -1, the out_flags fields do not contain meaningful values
** and must not be used.
**
** INPUTS:
**      PRPollDesc *pds         A pointer to an array of PRPollDesc
**
**      PRIntn npds             The number of elements in the array
**                              If this argument is zero PR_Poll is
**                              equivalent to a PR_Sleep(timeout).
**
**      PRIntervalTime timeout  Amount of time the call will block waiting
**                              for I/O to become ready. If this time expires
**                              w/o any I/O becoming ready, the result will
**                              be zero.
**
** OUTPUTS:    None
** RETURN:
**      PRInt32                 Number of PRPollDesc's with events or zero
**                              if the function timed out or -1 on failure.
**                              The reason for the failure is obtained by
**                              calling PR_GetError().
**************************************************************************
*/
NSPR_API(PRInt32) PR_Poll(
    PRPollDesc *pds, PRIntn npds, PRIntervalTime timeout);

/*
**************************************************************************
**
** Pollable events
**
** A pollable event is a special kind of file descriptor.
** The only I/O operation you can perform on a pollable event
** is to poll it with the PR_POLL_READ flag.  You can't
** read from or write to a pollable event.
**
** The purpose of a pollable event is to combine event waiting
** with I/O waiting in a single PR_Poll call.  Pollable events
** are implemented using a pipe or a pair of TCP sockets
** connected via the loopback address, therefore setting and
** waiting for pollable events are expensive operating system
** calls.  Do not use pollable events for general thread
** synchronization. Use condition variables instead.
**
** A pollable event has two states: set and unset.  Events
** are not queued, so there is no notion of an event count.
** A pollable event is either set or unset.
**
** A new pollable event is created by a PR_NewPollableEvent
** call and is initially in the unset state.
**
** PR_WaitForPollableEvent blocks the calling thread until
** the pollable event is set, and then it atomically unsets
** the pollable event before it returns.
**
** To set a pollable event, call PR_SetPollableEvent.
**
** One can call PR_Poll with the PR_POLL_READ flag on a pollable
** event.  When the pollable event is set, PR_Poll returns with
** the PR_POLL_READ flag set in the out_flags.
**
** To close a pollable event, call PR_DestroyPollableEvent
** (not PR_Close).
**
**************************************************************************
*/

NSPR_API(PRFileDesc *) PR_NewPollableEvent(void);

NSPR_API(PRStatus) PR_DestroyPollableEvent(PRFileDesc *event);

NSPR_API(PRStatus) PR_SetPollableEvent(PRFileDesc *event);

NSPR_API(PRStatus) PR_WaitForPollableEvent(PRFileDesc *event);

PR_END_EXTERN_C

#endif /* prio_h___ */
