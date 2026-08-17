/*
 * Declarations of request structures for the BIGFONT extension.
 *
 * Copyright (c) 1999-2000  Bruno Haible
 * Copyright (c) 1999-2000  The XFree86 Project, Inc.
 */

/* THIS IS NOT AN X CONSORTIUM STANDARD */

#ifndef _XF86BIGFPROTO_H_
#define _XF86BIGFPROTO_H_

#include <X11/extensions/xf86bigfont.h>

#define XF86BIGFONTNAME			"XFree86-Bigfont"

#define XF86BIGFONT_MAJOR_VERSION	1	/* current version numbers */
#define XF86BIGFONT_MINOR_VERSION	1

typedef struct _XF86BigfontQueryVersion {
    CARD8	reqType;		/* always XF86BigfontReqCode */
    CARD8	xf86bigfontReqType;	/* always X_XF86BigfontQueryVersion */
    CARD16	length;
} xXF86BigfontQueryVersionReq;
#define sz_xXF86BigfontQueryVersionReq	4

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	capabilities;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	majorVersion;		/* major version of XFree86-Bigfont */
    CARD16	minorVersion;		/* minor version of XFree86-Bigfont */
    CARD32	uid;
    CARD32	gid;
    CARD32	signature;
    CARD32	pad1;
    CARD32	pad2;
} xXF86BigfontQueryVersionReply;
#define sz_xXF86BigfontQueryVersionReply 32

/* Bit masks that can be set in the capabilities */
#define XF86Bigfont_CAP_LocalShm 1

typedef struct _XF86BigfontQueryFont {
    CARD8	reqType;		/* always XF86BigfontReqCode */
    CARD8	xf86bigfontReqType;	/* always X_XF86BigfontQueryFont */
    CARD16	length;
    CARD32	id;
    CARD32	flags;
} xXF86BigfontQueryFontReq;
#define sz_xXF86BigfontQueryFontReq	12

typedef struct {
    BYTE	type;			/* X_Reply */
    CARD8	pad1;
    CARD16	sequenceNumber;
    CARD32	length;
    xCharInfo	minBounds;
#ifndef WORD64
    CARD32	walign1;
#endif
    xCharInfo	maxBounds;
#ifndef WORD64
    CARD32	walign2;
#endif
    CARD16	minCharOrByte2;
    CARD16	maxCharOrByte2;
    CARD16	defaultChar;
    CARD16	nFontProps;
    CARD8	drawDirection;
    CARD8	minByte1;
    CARD8	maxByte1;
    BOOL	allCharsExist;
    INT16	fontAscent;
    INT16	fontDescent;
    CARD32	nCharInfos;
    CARD32	nUniqCharInfos;
    CARD32	shmid;
    CARD32	shmsegoffset;
    /* followed by nFontProps xFontProp structures */
    /* and if nCharInfos > 0 && shmid == -1,
       followed by nUniqCharInfos xCharInfo structures
       and then by nCharInfos CARD16 indices (each >= 0, < nUniqCharInfos)
       and then, if nCharInfos is odd, one more CARD16 for padding. */
} xXF86BigfontQueryFontReply;
#define sz_xXF86BigfontQueryFontReply	72

/* Bit masks that can be set in the flags */
#define XF86Bigfont_FLAGS_Shm 1

#endif /* _XF86BIGFPROTO_H_ */
