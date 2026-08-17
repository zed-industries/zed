/*

Copyright 1993, 1994, 1998  The Open Group

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

 * Copyright 1993, 1994 NCR Corporation - Dayton, Ohio, USA
 *
 * All Rights Reserved
 *
 * Permission to use, copy, modify, and distribute this software and its
 * documentation for any purpose and without fee is hereby granted, provided
 * that the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name NCR not be used in advertising
 * or publicity pertaining to distribution of the software without specific,
 * written prior permission.  NCR makes no representations about the
 * suitability of this software for any purpose.  It is provided "as is"
 * without express or implied warranty.
 *
 * NCR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN
 * NO EVENT SHALL NCR BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
 * OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
 * NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
 * CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

#ifndef _XTRANSINT_H_
#define _XTRANSINT_H_

/*
 * XTRANSDEBUG will enable the PRMSG() macros used in the X Transport
 * Interface code. Each use of the PRMSG macro has a level associated with
 * it. XTRANSDEBUG is defined to be a level. If the invocation level is =<
 * the value of XTRANSDEBUG, then the message will be printed out to stderr.
 * Recommended levels are:
 *
 *	XTRANSDEBUG=1	Error messages
 *	XTRANSDEBUG=2 API Function Tracing
 *	XTRANSDEBUG=3 All Function Tracing
 *	XTRANSDEBUG=4 printing of intermediate values
 *	XTRANSDEBUG=5 really detailed stuff
#define XTRANSDEBUG 2
 *
 * Defining XTRANSDEBUGTIMESTAMP will cause printing timestamps with each
 * message.
 */

#if !defined(XTRANSDEBUG) && defined(XTRANS_TRANSPORT_C)
#  define XTRANSDEBUG 1
#endif

#ifdef WIN32
# define _WILLWINSOCK_
#endif

#include "Xtrans.h"

#ifndef _X_UNUSED  /* Defined in Xfuncproto.h in xproto >= 7.0.22 */
# define _X_UNUSED  /* */
#endif

#ifdef XTRANSDEBUG
# include <stdio.h>
#endif /* XTRANSDEBUG */

#include <errno.h>

#ifndef WIN32
#  include <sys/socket.h>
# include <netinet/in.h>
# include <arpa/inet.h>

/*
 * Moved the setting of NEED_UTSNAME to this header file from Xtrans.c,
 * to avoid a race condition. JKJ (6/5/97)
 */

# if defined(_POSIX_SOURCE) || defined(USG) || defined(SVR4) || defined(__SVR4) || defined(__SCO__)
#  ifndef NEED_UTSNAME
#   define NEED_UTSNAME
#  endif
#  include <sys/utsname.h>
# endif

#  define ESET(val) errno = val
# define EGET() errno

#else /* WIN32 */

# include <limits.h>	/* for USHRT_MAX */

# define ESET(val) WSASetLastError(val)
# define EGET() WSAGetLastError()

#endif /* WIN32 */

#include <stddef.h>

#ifdef X11_t
#define X_TCP_PORT	6000
#endif

#if XTRANS_SEND_FDS

struct _XtransConnFd {
    struct _XtransConnFd   *next;
    int                    fd;
    int                    do_close;
};

#endif

struct _XtransConnInfo {
    struct _Xtransport     *transptr;
    int		index;
    char	*priv;
    int		flags;
    int		fd;
    char	*port;
    int		family;
    char	*addr;
    int		addrlen;
    char	*peeraddr;
    int		peeraddrlen;
    struct _XtransConnFd        *recv_fds;
    struct _XtransConnFd        *send_fds;
};

#define XTRANS_OPEN_COTS_CLIENT       1
#define XTRANS_OPEN_COTS_SERVER       2

typedef struct _Xtransport {
    const char	*TransName;
    int		flags;

#ifdef TRANS_CLIENT

    XtransConnInfo (*OpenCOTSClient)(
	struct _Xtransport *,	/* transport */
	const char *,		/* protocol */
	const char *,		/* host */
	const char *		/* port */
    );

#endif /* TRANS_CLIENT */

#ifdef TRANS_SERVER
    const char **	nolisten;
    XtransConnInfo (*OpenCOTSServer)(
	struct _Xtransport *,	/* transport */
	const char *,		/* protocol */
	const char *,		/* host */
	const char *		/* port */
    );

#endif /* TRANS_SERVER */

#ifdef TRANS_REOPEN

    XtransConnInfo (*ReopenCOTSServer)(
	struct _Xtransport *,	/* transport */
        int,			/* fd */
        const char *		/* port */
    );

#endif /* TRANS_REOPEN */


    int	(*SetOption)(
	XtransConnInfo,		/* connection */
	int,			/* option */
	int			/* arg */
    );

#ifdef TRANS_SERVER
/* Flags */
# define ADDR_IN_USE_ALLOWED	1

    int	(*CreateListener)(
	XtransConnInfo,		/* connection */
	const char *,		/* port */
	unsigned int		/* flags */
    );

    int	(*ResetListener)(
	XtransConnInfo		/* connection */
    );

    XtransConnInfo (*Accept)(
	XtransConnInfo,		/* connection */
        int *			/* status */
    );

#endif /* TRANS_SERVER */

#ifdef TRANS_CLIENT

    int	(*Connect)(
	XtransConnInfo,		/* connection */
	const char *,		/* host */
	const char *		/* port */
    );

#endif /* TRANS_CLIENT */

    int	(*BytesReadable)(
	XtransConnInfo,		/* connection */
	BytesReadable_t *	/* pend */
    );

    int	(*Read)(
	XtransConnInfo,		/* connection */
	char *,			/* buf */
	int			/* size */
    );

    int	(*Write)(
	XtransConnInfo,		/* connection */
	char *,			/* buf */
	int			/* size */
    );

    int	(*Readv)(
	XtransConnInfo,		/* connection */
	struct iovec *,		/* buf */
	int			/* size */
    );

    int	(*Writev)(
	XtransConnInfo,		/* connection */
	struct iovec *,		/* buf */
	int			/* size */
    );

#if XTRANS_SEND_FDS
    int (*SendFd)(
	XtransConnInfo,		/* connection */
        int,                    /* fd */
        int                     /* do_close */
    );

    int (*RecvFd)(
	XtransConnInfo		/* connection */
    );
#endif

    int	(*Disconnect)(
	XtransConnInfo		/* connection */
    );

    int	(*Close)(
	XtransConnInfo		/* connection */
    );

    int	(*CloseForCloning)(
	XtransConnInfo		/* connection */
    );

} Xtransport;


typedef struct _Xtransport_table {
    Xtransport	*transport;
    int		transport_id;
} Xtransport_table;


/*
 * Flags for the flags member of Xtransport.
 */

#define TRANS_ALIAS	(1<<0)	/* record is an alias, don't create server */
#define TRANS_LOCAL	(1<<1)	/* local transport */
#define TRANS_DISABLED	(1<<2)	/* Don't open this one */
#define TRANS_NOLISTEN  (1<<3)  /* Don't listen on this one */
#define TRANS_NOUNLINK	(1<<4)	/* Don't unlink transport endpoints */
#define TRANS_ABSTRACT	(1<<5)	/* Use abstract sockets if available */
#define TRANS_NOXAUTH	(1<<6)	/* Don't verify authentication (because it's secure some other way at the OS layer) */
#define TRANS_RECEIVED	(1<<7)  /* The fd for this has already been opened by someone else. */

/* Flags to preserve when setting others */
#define TRANS_KEEPFLAGS	(TRANS_NOUNLINK|TRANS_ABSTRACT)

#ifdef XTRANS_TRANSPORT_C /* only provide static function prototypes when
			     building the transport.c file that has them in */

#ifdef __clang__
/* Not all clients make use of all provided statics */
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunused-function"
#endif

/*
 * readv() and writev() don't exist or don't work correctly on some
 * systems, so they may be emulated.
 */

#ifdef WIN32

#define READV(ciptr, iov, iovcnt)	TRANS(ReadV)(ciptr, iov, iovcnt)

static	int TRANS(ReadV)(
    XtransConnInfo,	/* ciptr */
    struct iovec *,	/* iov */
    int			/* iovcnt */
);

#else

#define READV(ciptr, iov, iovcnt)	readv(ciptr->fd, iov, iovcnt)

#endif /* WIN32 */


#ifdef WIN32

#define WRITEV(ciptr, iov, iovcnt)	TRANS(WriteV)(ciptr, iov, iovcnt)

static int TRANS(WriteV)(
    XtransConnInfo,	/* ciptr */
    struct iovec *,	/* iov */
    int 		/* iovcnt */
);

#else

#define WRITEV(ciptr, iov, iovcnt)	writev(ciptr->fd, iov, iovcnt)

#endif /* WIN32 */


static int is_numeric (
    const char *	/* str */
);

#ifdef TRANS_SERVER
static int trans_mkdir (
    const char *,	/* path */
    int			/* mode */
);
#endif

#ifdef __clang__
#pragma clang diagnostic pop
#endif

/*
 * Some XTRANSDEBUG stuff
 */

#ifdef XTRANSDEBUG
#include <stdarg.h>

/*
 * The X server provides ErrorF() & VErrorF(), for other software that uses
 * xtrans, we provide our own simple versions.
 */
# if defined(XSERV_t) && defined(TRANS_SERVER)
#  include "os.h"
# else
static inline void _X_ATTRIBUTE_PRINTF(1, 0)
VErrorF(const char *f, va_list args)
{
    vfprintf(stderr, f, args);
    fflush(stderr);
}

static inline void  _X_ATTRIBUTE_PRINTF(1, 2)
ErrorF(const char *f, ...)
{
    va_list args;

    va_start(args, f);
    VErrorF(f, args);
    va_end(args);
}
# endif /* xserver */
#endif /* XTRANSDEBUG */

static inline void  _X_ATTRIBUTE_PRINTF(2, 3)
prmsg(int lvl, const char *f, ...)
{
#ifdef XTRANSDEBUG
    va_list args;

    va_start(args, f);
    if (lvl <= XTRANSDEBUG) {
	int saveerrno = errno;

	ErrorF("%s", __xtransname);
	VErrorF(f, args);

# ifdef XTRANSDEBUGTIMESTAMP
	{
	    struct timeval tp;
	    gettimeofday(&tp, 0);
	    ErrorF("timestamp (ms): %d\n",
		   tp.tv_sec * 1000 + tp.tv_usec / 1000);
	}
# endif
	errno = saveerrno;
    }
    va_end(args);
#endif /* XTRANSDEBUG */
}

#endif /* XTRANS_TRANSPORT_C */

#endif /* _XTRANSINT_H_ */
