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

#ifndef _ICEUTIL_H_
#define _ICEUTIL_H_

#include <X11/Xfuncproto.h>

#include <stdio.h>

_XFUNCPROTOBEGIN

/*
 * Data structure for entry in ICE authority file
 */

typedef struct {
    char    	    *protocol_name;
    unsigned short  protocol_data_length;
    char   	    *protocol_data;
    char    	    *network_id;
    char    	    *auth_name;
    unsigned short  auth_data_length;
    char   	    *auth_data;
} IceAuthFileEntry;


/*
 * Authentication data maintained in memory.
 */

typedef struct {
    char    	    *protocol_name;
    char	    *network_id;
    char    	    *auth_name;
    unsigned short  auth_data_length;
    char   	    *auth_data;
} IceAuthDataEntry;


/*
 * Return values from IceLockAuthFile
 */

#define IceAuthLockSuccess	0   /* lock succeeded */
#define IceAuthLockError	1   /* lock unexpectely failed, check errno */
#define IceAuthLockTimeout	2   /* lock failed, timeouts expired */


/*
 * Function Prototypes
 */

extern char *IceAuthFileName (
    void
);

extern int IceLockAuthFile (
    const char *	/* file_name */,
    int			/* retries */,
    int			/* timeout */,
    long		/* dead */
);

extern void IceUnlockAuthFile (
    const char *	/* file_name */
);

extern IceAuthFileEntry *IceReadAuthFileEntry (
    FILE *		/* auth_file */
);

extern void IceFreeAuthFileEntry (
    IceAuthFileEntry *	/* auth */
);

extern Status IceWriteAuthFileEntry (
    FILE *		/* auth_file */,
    IceAuthFileEntry *	/* auth */
);

extern IceAuthFileEntry *IceGetAuthFileEntry (
    const char *	/* protocol_name */,
    const char *	/* network_id */,
    const char *	/* auth_name */
);

extern char *IceGenerateMagicCookie (
    int			/* len */
);

extern void IceSetPaAuthData (
    int			/* numEntries */,
    IceAuthDataEntry *	/* entries */
);

_XFUNCPROTOEND

#endif /* _ICEUTIL_H_ */
