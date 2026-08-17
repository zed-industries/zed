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

#ifndef _SECUR_H
#define _SECUR_H

#define SECURITY_EXTENSION_NAME		"SECURITY"
#define SECURITY_MAJOR_VERSION		1
#define SECURITY_MINOR_VERSION		0

#define XSecurityNumberEvents		1
#define XSecurityNumberErrors		2
#define XSecurityBadAuthorization	0
#define XSecurityBadAuthorizationProtocol 1

/* trust levels */
#define XSecurityClientTrusted		0
#define XSecurityClientUntrusted	1

/* authorization attribute masks */
#define XSecurityTimeout		(1<<0)
#define XSecurityTrustLevel		(1<<1)
#define XSecurityGroup  		(1<<2)
#define XSecurityEventMask		(1<<3)
#define XSecurityAllAuthorizationAttributes \
 (XSecurityTimeout | XSecurityTrustLevel | XSecurityGroup | XSecurityEventMask)

/* event masks */
#define XSecurityAuthorizationRevokedMask (1<<0)
#define XSecurityAllEventMasks XSecurityAuthorizationRevokedMask

/* event offsets */
#define XSecurityAuthorizationRevoked 0

#define XSecurityAuthorizationName	"XC-QUERY-SECURITY-1"
#define XSecurityAuthorizationNameLen	19

#endif /* _SECUR_H */
