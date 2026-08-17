/*
Copyright 1996, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.
*/

#ifndef _SECURPROTO_H
#define _SECURPROTO_H

#include <X11/extensions/secur.h>

#define X_SecurityQueryVersion		0
#define X_SecurityGenerateAuthorization 1
#define X_SecurityRevokeAuthorization   2

typedef struct {
    CARD8       reqType;
    CARD8       securityReqType;
    CARD16      length;
    CARD16      majorVersion;
    CARD16      minorVersion;
} xSecurityQueryVersionReq;
#define sz_xSecurityQueryVersionReq 	8

typedef struct {
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
 } xSecurityQueryVersionReply;
#define sz_xSecurityQueryVersionReply  	32

typedef struct {
    CARD8       reqType;
    CARD8       securityReqType;
    CARD16      length;
    CARD16      nbytesAuthProto;
    CARD16      nbytesAuthData;
    CARD32      valueMask;
    /* auth protocol name padded to 4 bytes */
    /* auth protocol data padded to 4 bytes */
    /* list of CARD32 values, if any */
} xSecurityGenerateAuthorizationReq;
#define sz_xSecurityGenerateAuthorizationReq 12

typedef struct {
    CARD8   type;
    CARD8   pad0;
    CARD16  sequenceNumber;
    CARD32  length;
    CARD32  authId;
    CARD16  dataLength;
    CARD16  pad1;
    CARD32  pad2;
    CARD32  pad3;
    CARD32  pad4;
    CARD32  pad5;
 } xSecurityGenerateAuthorizationReply;
#define sz_xSecurityGenerateAuthorizationReply  	32

typedef struct {
    CARD8       reqType;
    CARD8       securityReqType;
    CARD16      length;
    CARD32      authId;
} xSecurityRevokeAuthorizationReq;
#define sz_xSecurityRevokeAuthorizationReq 8

typedef struct _xSecurityAuthorizationRevokedEvent {
    BYTE	type;
    BYTE	detail;
    CARD16	sequenceNumber;
    CARD32	authId;
    CARD32	pad0;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xSecurityAuthorizationRevokedEvent;
#define sz_xSecurityAuthorizationRevokedEvent 32

#endif /* _SECURPROTO_H */
