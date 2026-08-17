/*
 * Copyright 1989 Network Computing Devices, Inc., Mountain View, California.
 *
 * Permission to use, copy, modify, and distribute this software and its
 * documentation for any purpose and without fee is hereby granted, provided
 * that the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of N.C.D. not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  N.C.D. makes no representations about the
 * suitability of this software for any purpose.  It is provided "as is"
 * without express or implied warranty.
 *
 */

#ifndef _XDMCP_H_
#define _XDMCP_H_

#include <X11/Xmd.h>

#include <X11/Xfuncproto.h>

_XFUNCPROTOBEGIN

#define XDM_PROTOCOL_VERSION	1
#define XDM_UDP_PORT		177

/* IANA has assigned FF0X:0:0:0:0:0:0:12B as the permanently assigned
 * multicast addresses for XDMCP, where X in the prefix may be replaced
 * by any valid scope identifier, such as 1 for Node-Local, 2 for Link-Local,
 * 5 for Site-Local, and so on.  We set the default here to the Link-Local
 * version to most closely match the old IPv4 subnet broadcast behavior.
 * Both xdm and X -query allow specifying a different address if a different
 * scope is defined.
 */
#define XDM_DEFAULT_MCAST_ADDR6	"ff02:0:0:0:0:0:0:12b"

#define XDM_MAX_MSGLEN		8192
#define XDM_MIN_RTX		2
#define XDM_MAX_RTX		32
#define XDM_RTX_LIMIT		7
#define XDM_KA_RTX_LIMIT	4
#define XDM_DEF_DORMANCY	(3 * 60)	/* 3 minutes */
#define XDM_MAX_DORMANCY	(24 * 60 * 60)	/* 24 hours */

typedef enum {
    BROADCAST_QUERY = 1, QUERY, INDIRECT_QUERY, FORWARD_QUERY,
    WILLING, UNWILLING, REQUEST, ACCEPT, DECLINE, MANAGE, REFUSE,
    FAILED, KEEPALIVE, ALIVE
} xdmOpCode;

typedef enum {
    XDM_QUERY, XDM_BROADCAST, XDM_INDIRECT, XDM_COLLECT_QUERY,
    XDM_COLLECT_BROADCAST_QUERY, XDM_COLLECT_INDIRECT_QUERY,
    XDM_START_CONNECTION, XDM_AWAIT_REQUEST_RESPONSE,
    XDM_AWAIT_MANAGE_RESPONSE, XDM_MANAGE, XDM_RUN_SESSION, XDM_OFF,
    XDM_AWAIT_USER_INPUT, XDM_KEEPALIVE, XDM_AWAIT_ALIVE_RESPONSE,
#if defined(IPv6) && defined(AF_INET6)
    XDM_MULTICAST, XDM_COLLECT_MULTICAST_QUERY,
#endif
    XDM_KEEP_ME_LAST
} xdmcp_states;

#ifdef NOTDEF
/* table of hosts */

#define XDM_MAX_STR_LEN 21
#define XDM_MAX_HOSTS 20
struct xdm_host_table {
  struct sockaddr_in sockaddr;
  char name[XDM_MAX_STR_LEN];
  char status[XDM_MAX_STR_LEN];
};
#endif /* NOTDEF */

typedef CARD8	*CARD8Ptr;
typedef CARD16	*CARD16Ptr;
typedef CARD32	*CARD32Ptr;

typedef struct _ARRAY8 {
    CARD16	length;
    CARD8Ptr	data;
} ARRAY8, *ARRAY8Ptr;

typedef struct _ARRAY16 {
    CARD8	length;
    CARD16Ptr	data;
} ARRAY16, *ARRAY16Ptr;

typedef struct _ARRAY32 {
    CARD8	length;
    CARD32Ptr	data;
} ARRAY32, *ARRAY32Ptr;

typedef struct _ARRAYofARRAY8 {
    CARD8	length;
    ARRAY8Ptr	data;
} ARRAYofARRAY8, *ARRAYofARRAY8Ptr;

typedef struct _XdmcpHeader {
    CARD16  version, opcode, length;
} XdmcpHeader, *XdmcpHeaderPtr;

typedef struct _XdmcpBuffer {
    BYTE    *data;
    int	    size;		/* size of buffer pointed by to data */
    int	    pointer;		/* current index into data */
    int	    count;		/* bytes read from network into data */
} XdmcpBuffer, *XdmcpBufferPtr;

typedef struct _XdmAuthKey {
    BYTE    data[8];
} XdmAuthKeyRec, *XdmAuthKeyPtr;


/* implementation-independent network address structure.
   Equiv to sockaddr* for sockets. */

typedef char *XdmcpNetaddr;

extern int XdmcpWriteARRAY16(XdmcpBufferPtr buffer, const ARRAY16Ptr array);
extern int XdmcpWriteARRAY32(XdmcpBufferPtr buffer, const ARRAY32Ptr array);
extern int XdmcpWriteARRAY8(XdmcpBufferPtr buffer, const ARRAY8Ptr array);
extern int XdmcpWriteARRAYofARRAY8(XdmcpBufferPtr buffer, const ARRAYofARRAY8Ptr array);
extern int XdmcpWriteCARD16(XdmcpBufferPtr buffer, unsigned value);
extern int XdmcpWriteCARD32(XdmcpBufferPtr buffer, unsigned value);
extern int XdmcpWriteCARD8(XdmcpBufferPtr buffer, unsigned value);
extern int XdmcpWriteHeader(XdmcpBufferPtr  buffer, const XdmcpHeaderPtr  header);

extern int XdmcpFlush(int fd, XdmcpBufferPtr buffer, XdmcpNetaddr to, int tolen);

extern int XdmcpReadARRAY16(XdmcpBufferPtr buffer, ARRAY16Ptr array);
extern int XdmcpReadARRAY32(XdmcpBufferPtr buffer, ARRAY32Ptr array);
extern int XdmcpReadARRAY8(XdmcpBufferPtr buffer, ARRAY8Ptr array);
extern int XdmcpReadARRAYofARRAY8(XdmcpBufferPtr buffer, ARRAYofARRAY8Ptr array);
extern int XdmcpReadCARD16(XdmcpBufferPtr buffer, CARD16Ptr valuep);
extern int XdmcpReadCARD32(XdmcpBufferPtr buffer, CARD32Ptr valuep);
extern int XdmcpReadCARD8(XdmcpBufferPtr buffer, CARD8Ptr valuep);
extern int XdmcpReadHeader(XdmcpBufferPtr buffer, XdmcpHeaderPtr header);

extern int XdmcpFill(int fd, XdmcpBufferPtr buffer, XdmcpNetaddr from, int *fromlen);

extern int XdmcpReadRemaining(const XdmcpBufferPtr buffer);

extern void XdmcpDisposeARRAY8(ARRAY8Ptr array);
extern void XdmcpDisposeARRAY16(ARRAY16Ptr array);
extern void XdmcpDisposeARRAY32(ARRAY32Ptr array);
extern void XdmcpDisposeARRAYofARRAY8(ARRAYofARRAY8Ptr array);

extern int XdmcpCopyARRAY8(const ARRAY8Ptr src, ARRAY8Ptr dst);

extern int XdmcpARRAY8Equal(const ARRAY8Ptr array1, const ARRAY8Ptr array2);

extern void XdmcpGenerateKey (XdmAuthKeyPtr key);
extern void XdmcpIncrementKey (XdmAuthKeyPtr key);
extern void XdmcpDecrementKey (XdmAuthKeyPtr key);
#ifdef HASXDMAUTH
extern void XdmcpWrap(unsigned char *input, unsigned char *wrapper, unsigned char *output, int bytes);
extern void XdmcpUnwrap(unsigned char *input, unsigned char *wrapper, unsigned char *output, int bytes);
#endif

#ifndef TRUE
#define TRUE	1
#define FALSE	0
#endif

extern int XdmcpCompareKeys (const XdmAuthKeyPtr a, const XdmAuthKeyPtr b);

extern int XdmcpAllocARRAY16 (ARRAY16Ptr array, int length);
extern int XdmcpAllocARRAY32 (ARRAY32Ptr array, int length);
extern int XdmcpAllocARRAY8 (ARRAY8Ptr array, int length);
extern int XdmcpAllocARRAYofARRAY8 (ARRAYofARRAY8Ptr array, int length);

extern int XdmcpReallocARRAY16 (ARRAY16Ptr array, int length);
extern int XdmcpReallocARRAY32 (ARRAY32Ptr array, int length);
extern int XdmcpReallocARRAY8 (ARRAY8Ptr array, int length);
extern int XdmcpReallocARRAYofARRAY8 (ARRAYofARRAY8Ptr array, int length);

_XFUNCPROTOEND

#endif /* _XDMCP_H_ */
