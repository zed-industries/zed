/***************************************************************************
 * Copyright 1995 Network Computing Devices
 *
 * Permission to use, copy, modify, distribute, and sell this software and
 * its documentation for any purpose is hereby granted without fee, provided
 * that the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of Network Computing Devices
 * not be used in advertising or publicity pertaining to distribution
 * of the software without specific, written prior permission.
 *
 * NETWORK COMPUTING DEVICES DISCLAIMs ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL NETWORK COMPUTING DEVICES BE LIABLE
 * FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 **************************************************************************/

#ifndef _RECORDPROTO_H_
#define _RECORDPROTO_H_

#include <X11/extensions/recordconst.h>

/* only difference between 1.12 and 1.13 is byte order of device events,
   which the library doesn't deal with. */

/*********************************************************
 *
 * Protocol request constants
 *
 */
#define X_RecordQueryVersion    0     /* First request from client */
#define X_RecordCreateContext   1     /* Create client RC */
#define X_RecordRegisterClients 2     /* Add to client RC */
#define X_RecordUnregisterClients 3   /* Delete from client RC */
#define X_RecordGetContext      4     /* Query client RC */
#define X_RecordEnableContext   5     /* Enable interception and reporting */
#define X_RecordDisableContext  6     /* Disable interception and reporting */
#define X_RecordFreeContext     7     /* Free client RC */

#define sz_XRecordRange		32
#define sz_XRecordClientInfo 	12
#define sz_XRecordState 	16
#define sz_XRecordDatum 	32


#define XRecordGlobaldef
#define XRecordGlobalref extern

#define RecordMaxEvent     	(128L-1L)
#define RecordMinDeviceEvent	(2L)
#define RecordMaxDeviceEvent	(6L)
#define RecordMaxError          (256L-1L)
#define RecordMaxCoreRequest    (128L-1L)
#define RecordMaxExtRequest     (256L-1L)
#define RecordMinExtRequest     (129L-1L)

#define RECORD_RC 		CARD32
#define RECORD_XIDBASE		CARD32
#define RECORD_CLIENTSPEC	CARD32
#define RECORD_ELEMENT_HEADER	CARD8

typedef RECORD_CLIENTSPEC RecordClientSpec, *RecordClientSpecPtr;

typedef struct
{
    CARD8	first;
    CARD8	last;
} RECORD_RANGE8;

typedef struct
{
    CARD16	first;
    CARD16	last;
} RECORD_RANGE16;

typedef struct
{
    RECORD_RANGE8	majorCode;
    RECORD_RANGE16	minorCode;
} RECORD_EXTRANGE;

typedef struct
{
    RECORD_RANGE8	coreRequests;
    RECORD_RANGE8	coreReplies;
    RECORD_EXTRANGE	extRequests;
    RECORD_EXTRANGE	extReplies;
    RECORD_RANGE8	deliveredEvents;
    RECORD_RANGE8	deviceEvents;
    RECORD_RANGE8	errors;
    BOOL		clientStarted;
    BOOL		clientDied;
} RECORDRANGE;
#define sz_RECORDRANGE 	24

/* typedef RECORDRANGE xRecordRange, *xRecordRangePtr;
#define sz_xRecordRange 24 */

/* Cannot have structures within structures going over the wire */
typedef struct
{
    CARD8       	coreRequestsFirst;
    CARD8       	coreRequestsLast;
    CARD8       	coreRepliesFirst;
    CARD8       	coreRepliesLast;
    CARD8  		extRequestsMajorFirst;
    CARD8		extRequestsMajorLast;
    CARD16  		extRequestsMinorFirst;
    CARD16  		extRequestsMinorLast;
    CARD8  		extRepliesMajorFirst;
    CARD8		extRepliesMajorLast;
    CARD16  		extRepliesMinorFirst;
    CARD16  		extRepliesMinorLast;
    CARD8       	deliveredEventsFirst;
    CARD8       	deliveredEventsLast;
    CARD8		deviceEventsFirst;
    CARD8		deviceEventsLast;
    CARD8       	errorsFirst;
    CARD8       	errorsLast;
    BOOL                clientStarted;
    BOOL		clientDied;
} xRecordRange;
#define sz_xRecordRange 24

typedef struct
{
    RECORD_CLIENTSPEC	clientResource;
    CARD32		nRanges;
/* LISTofRECORDRANGE */
} RECORD_CLIENT_INFO;

typedef RECORD_CLIENT_INFO xRecordClientInfo;

/*
 * Initialize
 */
typedef struct {
    CARD8       reqType;
    CARD8       recordReqType;
    CARD16      length;
    CARD16      majorVersion;
    CARD16      minorVersion;
} xRecordQueryVersionReq;
#define sz_xRecordQueryVersionReq 	8

typedef struct
{
    CARD8   type;
    CARD8   pad0;
    CARD16  sequenceNumber;
    CARD32  length;
    CARD16  majorVersion;
    CARD16  minorVersion;
    CARD32  pad1;
    CARD32  pad2;
    CARD32  pad3;
    CARD32  pad4;
    CARD32  pad5;
 } xRecordQueryVersionReply;
#define sz_xRecordQueryVersionReply  	32

/*
 * Create RC
 */
typedef struct
{
    CARD8     		reqType;
    CARD8     		recordReqType;
    CARD16    		length;
    RECORD_RC		context;
    RECORD_ELEMENT_HEADER elementHeader;
    CARD8		pad;
    CARD16		pad0;
    CARD32		nClients;
    CARD32		nRanges;
/* LISTofRECORD_CLIENTSPEC */
/* LISTofRECORDRANGE */
} xRecordCreateContextReq;
#define sz_xRecordCreateContextReq 	20

/*
 * Add to  RC
 */
typedef struct
{
    CARD8     		reqType;
    CARD8     		recordReqType;
    CARD16    		length;
    RECORD_RC		context;
    RECORD_ELEMENT_HEADER elementHeader;
    CARD8		pad;
    CARD16		pad0;
    CARD32		nClients;
    CARD32		nRanges;
/* LISTofRECORD_CLIENTSPEC */
/* LISTofRECORDRANGE */
} xRecordRegisterClientsReq;
#define sz_xRecordRegisterClientsReq 	20

/*
 * Delete from RC
 */
typedef struct
{
    CARD8     		reqType;
    CARD8     		recordReqType;
    CARD16    		length;
    RECORD_RC		context;
    CARD32		nClients;
/* LISTofRECORD_CLIENTSPEC */
} xRecordUnregisterClientsReq;
#define sz_xRecordUnregisterClientsReq 	12

/*
 * Query RC
 */
typedef struct
{
    CARD8     	reqType;
    CARD8     	recordReqType;
    CARD16    	length;
    RECORD_RC	context;
} xRecordGetContextReq;
#define sz_xRecordGetContextReq 		8

typedef struct
{
    CARD8   	type;
    BOOL    	enabled;
    CARD16  	sequenceNumber;
    CARD32  	length;
    RECORD_ELEMENT_HEADER  elementHeader;
    CARD8	pad;
    CARD16	pad0;
    CARD32  	nClients;
    CARD32  	pad1;
    CARD32  	pad2;
    CARD32  	pad3;
    CARD32  	pad4;
/* LISTofCLIENT_INFO */ 		/* intercepted-clients */
} xRecordGetContextReply;
#define sz_xRecordGetContextReply  	32

/*
 * Enable data interception
 */
typedef struct
{
    CARD8     	reqType;
    CARD8     	recordReqType;
    CARD16    	length;
    RECORD_RC	context;
} xRecordEnableContextReq;
#define sz_xRecordEnableContextReq 	8

typedef struct
{
    CARD8		type;
    CARD8		category;
    CARD16		sequenceNumber;
    CARD32		length;
    RECORD_ELEMENT_HEADER  elementHeader;
    BOOL		clientSwapped;
    CARD16		pad1;
    RECORD_XIDBASE 	idBase;
    CARD32		serverTime;
    CARD32		recordedSequenceNumber;
    CARD32		pad3;
    CARD32		pad4;
    /* BYTE		data; */
} xRecordEnableContextReply;
#define sz_xRecordEnableContextReply 	32

/*
 * Disable data interception
 */
typedef struct
{
    CARD8     	reqType;
    CARD8     	recordReqType;
    CARD16    	length;
    RECORD_RC 	context;
} xRecordDisableContextReq;
#define sz_xRecordDisableContextReq	8

/*
 * Free RC
 */
typedef struct
{
    CARD8     	reqType;
    CARD8     	recordReqType;
    CARD16    	length;
    RECORD_RC 	context;
} xRecordFreeContextReq;
#define sz_xRecordFreeContextReq 	8

#undef RECORD_RC
#undef RECORD_XIDBASE
#undef RECORD_ELEMENT_HEADER
#undef RECORD_CLIENTSPEC

#endif
