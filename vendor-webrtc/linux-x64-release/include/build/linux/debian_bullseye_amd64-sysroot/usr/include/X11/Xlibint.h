
/*

Copyright 1984, 1985, 1987, 1989, 1998  The Open Group

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

#ifndef _X11_XLIBINT_H_
#define _X11_XLIBINT_H_ 1

/*
 *	Xlibint.h - Header definition and support file for the internal
 *	support routines used by the C subroutine interface
 *	library (Xlib) to the X Window System.
 *
 *	Warning, there be dragons here....
 */

#include <stdint.h>
#include <X11/Xlib.h>
#include <X11/Xproto.h>		/* to declare xEvent */
#include <X11/XlibConf.h>	/* for configured options like XTHREADS */

/* The Xlib structs are full of implicit padding to properly align members.
   We can't clean that up without breaking ABI, so tell clang not to bother
   complaining about it. */
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wpadded"
#endif

#ifdef WIN32
#define _XFlush _XFlushIt
#endif

struct _XGC
{
    XExtData *ext_data;	/* hook for extension to hang data */
    GContext gid;	/* protocol ID for graphics context */
    Bool rects;		/* boolean: TRUE if clipmask is list of rectangles */
    Bool dashes;	/* boolean: TRUE if dash-list is really a list */
    unsigned long dirty;/* cache dirty bits */
    XGCValues values;	/* shadow structure of values */
};

struct _XDisplay
{
	XExtData *ext_data;	/* hook for extension to hang data */
	struct _XFreeFuncs *free_funcs; /* internal free functions */
	int fd;			/* Network socket. */
	int conn_checker;         /* ugly thing used by _XEventsQueued */
	int proto_major_version;/* maj. version of server's X protocol */
	int proto_minor_version;/* minor version of server's X protocol */
	char *vendor;		/* vendor of the server hardware */
        XID resource_base;	/* resource ID base */
	XID resource_mask;	/* resource ID mask bits */
	XID resource_id;	/* allocator current ID */
	int resource_shift;	/* allocator shift to correct bits */
	XID (*resource_alloc)(	/* allocator function */
		struct _XDisplay*
		);
	int byte_order;		/* screen byte order, LSBFirst, MSBFirst */
	int bitmap_unit;	/* padding and data requirements */
	int bitmap_pad;		/* padding requirements on bitmaps */
	int bitmap_bit_order;	/* LeastSignificant or MostSignificant */
	int nformats;		/* number of pixmap formats in list */
	ScreenFormat *pixmap_format;	/* pixmap format list */
	int vnumber;		/* Xlib's X protocol version number. */
	int release;		/* release of the server */
	struct _XSQEvent *head, *tail;	/* Input event queue. */
	int qlen;		/* Length of input event queue */
	unsigned long last_request_read; /* seq number of last event read */
	unsigned long request;	/* sequence number of last request. */
	char *last_req;		/* beginning of last request, or dummy */
	char *buffer;		/* Output buffer starting address. */
	char *bufptr;		/* Output buffer index pointer. */
	char *bufmax;		/* Output buffer maximum+1 address. */
	unsigned max_request_size; /* maximum number 32 bit words in request*/
	struct _XrmHashBucketRec *db;
	int (*synchandler)(	/* Synchronization handler */
		struct _XDisplay*
		);
	char *display_name;	/* "host:display" string used on this connect*/
	int default_screen;	/* default screen for operations */
	int nscreens;		/* number of screens on this server*/
	Screen *screens;	/* pointer to list of screens */
	unsigned long motion_buffer;	/* size of motion buffer */
	volatile unsigned long flags;	   /* internal connection flags */
	int min_keycode;	/* minimum defined keycode */
	int max_keycode;	/* maximum defined keycode */
	KeySym *keysyms;	/* This server's keysyms */
	XModifierKeymap *modifiermap;	/* This server's modifier keymap */
	int keysyms_per_keycode;/* number of rows */
	char *xdefaults;	/* contents of defaults from server */
	char *scratch_buffer;	/* place to hang scratch buffer */
	unsigned long scratch_length;	/* length of scratch buffer */
	int ext_number;		/* extension number on this display */
	struct _XExten *ext_procs; /* extensions initialized on this display */
	/*
	 * the following can be fixed size, as the protocol defines how
	 * much address space is available.
	 * While this could be done using the extension vector, there
	 * may be MANY events processed, so a search through the extension
	 * list to find the right procedure for each event might be
	 * expensive if many extensions are being used.
	 */
	Bool (*event_vec[128])(	/* vector for wire to event */
		Display *	/* dpy */,
		XEvent *	/* re */,
		xEvent *	/* event */
		);
	Status (*wire_vec[128])( /* vector for event to wire */
		Display *	/* dpy */,
		XEvent *	/* re */,
		xEvent *	/* event */
		);
	KeySym lock_meaning;	   /* for XLookupString */
	struct _XLockInfo *lock;   /* multi-thread state, display lock */
	struct _XInternalAsync *async_handlers; /* for internal async */
	unsigned long bigreq_size; /* max size of big requests */
	struct _XLockPtrs *lock_fns; /* pointers to threads functions */
	void (*idlist_alloc)(	   /* XID list allocator function */
		Display *	/* dpy */,
		XID *		/* ids */,
		int		/* count */
		);
	/* things above this line should not move, for binary compatibility */
	struct _XKeytrans *key_bindings; /* for XLookupString */
	Font cursor_font;	   /* for XCreateFontCursor */
	struct _XDisplayAtoms *atoms; /* for XInternAtom */
	unsigned int mode_switch;  /* keyboard group modifiers */
	unsigned int num_lock;  /* keyboard numlock modifiers */
	struct _XContextDB *context_db; /* context database */
	Bool (**error_vec)(	/* vector for wire to error */
		Display     *	/* display */,
		XErrorEvent *	/* he */,
		xError      *	/* we */
		);
	/*
	 * Xcms information
	 */
	struct {
	   XPointer defaultCCCs;  /* pointer to an array of default XcmsCCC */
	   XPointer clientCmaps;  /* pointer to linked list of XcmsCmapRec */
	   XPointer perVisualIntensityMaps;
				  /* linked list of XcmsIntensityMap */
	} cms;
	struct _XIMFilter *im_filters;
	struct _XSQEvent *qfree; /* unallocated event queue elements */
	unsigned long next_event_serial_num; /* inserted into next queue elt */
	struct _XExten *flushes; /* Flush hooks */
	struct _XConnectionInfo *im_fd_info; /* _XRegisterInternalConnection */
	int im_fd_length;	/* number of im_fd_info */
	struct _XConnWatchInfo *conn_watchers; /* XAddConnectionWatch */
	int watcher_count;	/* number of conn_watchers */
	XPointer filedes;	/* struct pollfd cache for _XWaitForReadable */
	int (*savedsynchandler)( /* user synchandler when Xlib usurps */
		Display *	/* dpy */
		);
	XID resource_max;	/* allocator max ID */
	int xcmisc_opcode;	/* major opcode for XC-MISC */
	struct _XkbInfoRec *xkb_info; /* XKB info */
	struct _XtransConnInfo *trans_conn; /* transport connection object */
	struct _X11XCBPrivate *xcb; /* XCB glue private data */

	/* Generic event cookie handling */
	unsigned int next_cookie; /* next event cookie */
	/* vector for wire to generic event, index is (extension - 128) */
	Bool (*generic_event_vec[128])(
		Display *	/* dpy */,
		XGenericEventCookie *	/* Xlib event */,
		xEvent *	/* wire event */);
	/* vector for event copy, index is (extension - 128) */
	Bool (*generic_event_copy_vec[128])(
		Display *	/* dpy */,
		XGenericEventCookie *	/* in */,
		XGenericEventCookie *   /* out*/);
	void *cookiejar;  /* cookie events returned but not claimed */
#ifndef LONG64
	unsigned long last_request_read_upper32bit;
	unsigned long request_upper32bit;
#endif

	struct _XErrorThreadInfo *error_threads;

	XIOErrorExitHandler exit_handler;
	void *exit_handler_data;
};

#define XAllocIDs(dpy,ids,n) (*(dpy)->idlist_alloc)(dpy,ids,n)

/*
 * access "last_request_read" and "request" with 64bit
 * warning: the value argument of the SET-macros must not
 * have any side-effects because it may get called twice.
 */
#ifndef LONG64
/* accessors for 32-bit unsigned long */

#define X_DPY_GET_REQUEST(dpy) \
    ( \
        ((uint64_t)(((struct _XDisplay*)dpy)->request)) \
	+ (((uint64_t)(((struct _XDisplay*)dpy)->request_upper32bit)) << 32) \
    )

#define X_DPY_SET_REQUEST(dpy, value) \
    ( \
        (((struct _XDisplay*)dpy)->request = \
            (value) & 0xFFFFFFFFUL), \
        (((struct _XDisplay*)dpy)->request_upper32bit = \
            ((uint64_t)(value)) >> 32), \
	(void)0 /* don't use the result */ \
    )

#define X_DPY_GET_LAST_REQUEST_READ(dpy) \
    ( \
        ((uint64_t)(((struct _XDisplay*)dpy)->last_request_read)) \
        + ( \
            ((uint64_t)( \
                ((struct _XDisplay*)dpy)->last_request_read_upper32bit \
            )) << 32 \
        ) \
    )

#define X_DPY_SET_LAST_REQUEST_READ(dpy, value) \
    ( \
        (((struct _XDisplay*)dpy)->last_request_read = \
            (value) & 0xFFFFFFFFUL), \
        (((struct _XDisplay*)dpy)->last_request_read_upper32bit = \
            ((uint64_t)(value)) >> 32), \
	(void)0 /* don't use the result */ \
    )

/*
 * widen a 32-bit sequence number to a 64 sequence number.
 * This macro makes the following assumptions:
 * - ulseq refers to a sequence that has already been sent
 * - ulseq means the most recent possible sequence number
 *   with these lower 32 bits.
 *
 * The following optimization is used:
 * The comparison result is taken a 0 or 1 to avoid a branch.
 */
#define X_DPY_WIDEN_UNSIGNED_LONG_SEQ(dpy, ulseq) \
    ( \
        ((uint64_t)ulseq) \
        + \
        (( \
            ((uint64_t)(((struct _XDisplay*)dpy)->request_upper32bit)) \
            - (uint64_t)( \
                (ulseq) > (((struct _XDisplay*)dpy)->request) \
	    ) \
        ) << 32) \
    )

#define X_DPY_REQUEST_INCREMENT(dpy) \
    ( \
        ((struct _XDisplay*)dpy)->request++, \
        ( \
            (((struct _XDisplay*)dpy)->request == 0) ? ( \
                ((struct _XDisplay*)dpy)->request_upper32bit++ \
	    ) : 0 \
        ), \
	(void)0 /* don't use the result */ \
    )


#define X_DPY_REQUEST_DECREMENT(dpy) \
    ( \
	( \
            (((struct _XDisplay*)dpy)->request == 0) ? (\
                ((struct _XDisplay*)dpy)->request--, /* wrap */ \
                ((struct _XDisplay*)dpy)->request_upper32bit-- \
            ) : ( \
                ((struct _XDisplay*)dpy)->request-- \
            ) \
	), \
	(void)0 /* don't use the result */ \
    )

#else
/* accessors for 64-bit unsigned long */
#define X_DPY_GET_REQUEST(dpy) \
    (((struct _XDisplay*)dpy)->request)
#define X_DPY_SET_REQUEST(dpy, value) \
    ((struct _XDisplay*)dpy)->request = (value)

#define X_DPY_GET_LAST_REQUEST_READ(dpy) \
    (((struct _XDisplay*)dpy)->last_request_read)
#define X_DPY_SET_LAST_REQUEST_READ(dpy, value) \
    ((struct _XDisplay*)dpy)->last_request_read = (value)

#define X_DPY_WIDEN_UNSIGNED_LONG_SEQ(dpy, ulseq) ulseq

#define X_DPY_REQUEST_INCREMENT(dpy) ((struct _XDisplay*)dpy)->request++
#define X_DPY_REQUEST_DECREMENT(dpy) ((struct _XDisplay*)dpy)->request--
#endif


#ifndef _XEVENT_
/*
 * _QEvent datatype for use in input queueing.
 */
typedef struct _XSQEvent
{
    struct _XSQEvent *next;
    XEvent event;
    unsigned long qserial_num;	/* so multi-threaded code can find new ones */
} _XQEvent;
#endif

#include <X11/Xproto.h>
#ifdef __sgi
#define _SGI_MP_SOURCE  /* turn this on to get MP safe errno */
#endif
#include <errno.h>
#define _XBCOPYFUNC _Xbcopy
#include <X11/Xfuncs.h>
#include <X11/Xosdefs.h>

/* Utek leaves kernel macros around in include files (bleah) */
#ifdef dirty
#undef dirty
#endif

#include <stdlib.h>
#include <string.h>

#include <X11/Xfuncproto.h>

_XFUNCPROTOBEGIN

/*
 * The following definitions can be used for locking requests in multi-threaded
 * address spaces.
 */
#ifdef XTHREADS
/* Author: Stephen Gildea, MIT X Consortium
 *
 * declarations for C Threads locking
 */

typedef struct _LockInfoRec *LockInfoPtr;

/* interfaces for locking.c */
struct _XLockPtrs {
    /* used by all, including extensions; do not move */
    void (*lock_display)(
		Display *dpy
#if defined(XTHREADS_WARN) || defined(XTHREADS_FILE_LINE)
		, char *file
		, int line
#endif
	);
    void (*unlock_display)(
		Display *dpy
#if defined(XTHREADS_WARN) || defined(XTHREADS_FILE_LINE)
		, char *file
		, int line
#endif
	);
};

#if defined(WIN32) && !defined(_XLIBINT_)
#define _XCreateMutex_fn (*_XCreateMutex_fn_p)
#define _XFreeMutex_fn (*_XFreeMutex_fn_p)
#define _XLockMutex_fn (*_XLockMutex_fn_p)
#define _XUnlockMutex_fn (*_XUnlockMutex_fn_p)
#define _Xglobal_lock (*_Xglobal_lock_p)
#endif

/* in XlibInt.c */
extern void (*_XCreateMutex_fn)(
    LockInfoPtr /* lock */
);
extern void (*_XFreeMutex_fn)(
    LockInfoPtr /* lock */
);
extern void (*_XLockMutex_fn)(
    LockInfoPtr	/* lock */
#if defined(XTHREADS_WARN) || defined(XTHREADS_FILE_LINE)
    , char * /* file */
    , int /* line */
#endif
);
extern void (*_XUnlockMutex_fn)(
    LockInfoPtr	/* lock */
#if defined(XTHREADS_WARN) || defined(XTHREADS_FILE_LINE)
    , char * /* file */
    , int /* line */
#endif
);

extern LockInfoPtr _Xglobal_lock;

#if defined(XTHREADS_WARN) || defined(XTHREADS_FILE_LINE)
#define LockDisplay(d)	     if ((d)->lock_fns) (*(d)->lock_fns->lock_display)((d),__FILE__,__LINE__)
#define UnlockDisplay(d)     if ((d)->lock_fns) (*(d)->lock_fns->unlock_display)((d),__FILE__,__LINE__)
#define _XLockMutex(lock)		if (_XLockMutex_fn) (*_XLockMutex_fn)(lock,__FILE__,__LINE__)
#define _XUnlockMutex(lock)	if (_XUnlockMutex_fn) (*_XUnlockMutex_fn)(lock,__FILE__,__LINE__)
#else
/* used everywhere, so must be fast if not using threads */
#define LockDisplay(d)	     if ((d)->lock_fns) (*(d)->lock_fns->lock_display)(d)
#define UnlockDisplay(d)     if ((d)->lock_fns) (*(d)->lock_fns->unlock_display)(d)
#define _XLockMutex(lock)		if (_XLockMutex_fn) (*_XLockMutex_fn)(lock)
#define _XUnlockMutex(lock)	if (_XUnlockMutex_fn) (*_XUnlockMutex_fn)(lock)
#endif
#define _XCreateMutex(lock)	if (_XCreateMutex_fn) (*_XCreateMutex_fn)(lock);
#define _XFreeMutex(lock)	if (_XFreeMutex_fn) (*_XFreeMutex_fn)(lock);

#else /* XTHREADS */
#define LockDisplay(dis)
#define _XLockMutex(lock)
#define _XUnlockMutex(lock)
#define UnlockDisplay(dis)
#define _XCreateMutex(lock)
#define _XFreeMutex(lock)
#endif

#define Xfree(ptr) free((ptr))

/*
 * Note that some machines do not return a valid pointer for malloc(0), in
 * which case we provide an alternate under the control of the
 * define MALLOC_0_RETURNS_NULL.  This is necessary because some
 * Xlib code expects malloc(0) to return a valid pointer to storage.
 */
#if defined(MALLOC_0_RETURNS_NULL) || defined(__clang_analyzer__)

# define Xmalloc(size) malloc((size_t)((size) == 0 ? 1 : (size)))
# define Xrealloc(ptr, size) realloc((ptr), (size_t)((size) == 0 ? 1 : (size)))
# define Xcalloc(nelem, elsize) calloc((size_t)((nelem) == 0 ? 1 : (nelem)), (size_t)(elsize))

#else

# define Xmalloc(size) malloc((size_t)(size))
# define Xrealloc(ptr, size) realloc((ptr), (size_t)(size))
# define Xcalloc(nelem, elsize) calloc((size_t)(nelem), (size_t)(elsize))

#endif

#include <stddef.h>

#define LOCKED 1
#define UNLOCKED 0

#ifndef BUFSIZE
#define BUFSIZE 2048			/* X output buffer size. */
#endif
#ifndef PTSPERBATCH
#define PTSPERBATCH 1024		/* point batching */
#endif
#ifndef WLNSPERBATCH
#define WLNSPERBATCH 50			/* wide line batching */
#endif
#ifndef ZLNSPERBATCH
#define ZLNSPERBATCH 1024		/* thin line batching */
#endif
#ifndef WRCTSPERBATCH
#define WRCTSPERBATCH 10		/* wide line rectangle batching */
#endif
#ifndef ZRCTSPERBATCH
#define ZRCTSPERBATCH 256		/* thin line rectangle batching */
#endif
#ifndef FRCTSPERBATCH
#define FRCTSPERBATCH 256		/* filled rectangle batching */
#endif
#ifndef FARCSPERBATCH
#define FARCSPERBATCH 256		/* filled arc batching */
#endif
#ifndef CURSORFONT
#define CURSORFONT "cursor"		/* standard cursor fonts */
#endif

/*
 * Display flags
 */
#define XlibDisplayIOError	(1L << 0)
#define XlibDisplayClosing	(1L << 1)
#define XlibDisplayNoXkb	(1L << 2)
#define XlibDisplayPrivSync	(1L << 3)
#define XlibDisplayProcConni	(1L << 4) /* in _XProcessInternalConnection */
#define XlibDisplayReadEvents	(1L << 5) /* in _XReadEvents */
#define XlibDisplayReply	(1L << 5) /* in _XReply */
#define XlibDisplayWriting	(1L << 6) /* in _XFlushInt, _XSend */
#define XlibDisplayDfltRMDB     (1L << 7) /* mark if RM db from XGetDefault */

/*
 * X Protocol packetizing macros.
 */

/* Leftover from CRAY support - was defined empty on all non-Cray systems */
#define WORD64ALIGN

/**
 * Return a len-sized request buffer for the request type. This function may
 * flush the output queue.
 *
 * @param dpy The display connection
 * @param type The request type
 * @param len Length of the request in bytes
 *
 * @returns A pointer to the request buffer with a few default values
 * initialized.
 */
extern void *_XGetRequest(Display *dpy, CARD8 type, size_t len);

/* GetReqSized is the same as GetReq but allows the caller to specify the
 * size in bytes. 'sz' must be a multiple of 4! */

#define GetReqSized(name, sz, req) \
	req = (x##name##Req *) _XGetRequest(dpy, X_##name, sz)

/*
 * GetReq - Get the next available X request packet in the buffer and
 * return it.
 *
 * "name" is the name of the request, e.g. CreatePixmap, OpenFont, etc.
 * "req" is the name of the request pointer.
 *
 */

#define GetReq(name, req) \
	GetReqSized(name, SIZEOF(x##name##Req), req)

/* GetReqExtra is the same as GetReq, but allocates "n" additional
   bytes after the request. "n" must be a multiple of 4!  */

#define GetReqExtra(name, n, req) \
        GetReqSized(name, SIZEOF(x##name##Req) + n, req)

/*
 * GetResReq is for those requests that have a resource ID
 * (Window, Pixmap, GContext, etc.) as their single argument.
 * "rid" is the name of the resource.
 */

#define GetResReq(name, rid, req) \
	req = (xResourceReq *) _XGetRequest(dpy, X_##name, SIZEOF(xResourceReq)); \
	req->id = (rid)

/*
 * GetEmptyReq is for those requests that have no arguments
 * at all.
 */

#define GetEmptyReq(name, req) \
	req = (xReq *) _XGetRequest(dpy, X_##name, SIZEOF(xReq))

/*
 * MakeBigReq sets the CARD16 "req->length" to 0 and inserts a new CARD32
 * length, after req->length, before the data in the request.  The new length
 * includes the "n" extra 32-bit words.
 *
 * Do not use MakeBigReq if there is no data already in the request.
 * req->length must already be >= 2.
 */
#ifdef LONG64
#define MakeBigReq(req,n) \
    { \
    CARD64 _BRdat; \
    CARD32 _BRlen = req->length - 1; \
    req->length = 0; \
    _BRdat = ((CARD32 *)req)[_BRlen]; \
    memmove(((char *)req) + 8, ((char *)req) + 4, (_BRlen - 1) << 2); \
    ((CARD32 *)req)[1] = _BRlen + n + 2; \
    Data32(dpy, &_BRdat, 4); \
    }
#else
#define MakeBigReq(req,n) \
    { \
    CARD32 _BRdat; \
    CARD32 _BRlen = req->length - 1; \
    req->length = 0; \
    _BRdat = ((CARD32 *)req)[_BRlen]; \
    memmove(((char *)req) + 8, ((char *)req) + 4, (_BRlen - 1) << 2); \
    ((CARD32 *)req)[1] = _BRlen + n + 2; \
    Data32(dpy, &_BRdat, 4); \
    }
#endif

/*
 * SetReqLen increases the count of 32-bit words in the request by "n",
 * or by "badlen" if "n" is too large.
 *
 * Do not use SetReqLen if "req" does not already have data after the
 * xReq header.  req->length must already be >= 2.
 */
#ifndef __clang_analyzer__
#define SetReqLen(req,n,badlen) \
    if ((req->length + n) > (unsigned)65535) { \
	if (dpy->bigreq_size) { \
	    MakeBigReq(req,n) \
	} else { \
	    n = badlen; \
	    req->length += n; \
	} \
    } else \
	req->length += n
#else
#define SetReqLen(req,n,badlen) \
    req->length += n
#endif

#define SyncHandle() \
	if (dpy->synchandler) (*dpy->synchandler)(dpy)

extern void _XFlushGCCache(Display *dpy, GC gc);
#define FlushGC(dpy, gc) \
	if ((gc)->dirty) _XFlushGCCache((dpy), (gc))
/*
 * Data - Place data in the buffer and pad the end to provide
 * 32 bit word alignment.  Transmit if the buffer fills.
 *
 * "dpy" is a pointer to a Display.
 * "data" is a pointer to a data buffer.
 * "len" is the length of the data buffer.
 */
#ifndef DataRoutineIsProcedure
#define Data(dpy, data, len) {\
	if (dpy->bufptr + (len) <= dpy->bufmax) {\
		memcpy(dpy->bufptr, data, (int)(len));\
		dpy->bufptr += ((len) + 3) & ~3;\
	} else\
		_XSend(dpy, data, len);\
}
#endif /* DataRoutineIsProcedure */


/* Allocate bytes from the buffer.  No padding is done, so if
 * the length is not a multiple of 4, the caller must be
 * careful to leave the buffer aligned after sending the
 * current request.
 *
 * "type" is the type of the pointer being assigned to.
 * "ptr" is the pointer being assigned to.
 * "n" is the number of bytes to allocate.
 *
 * Example:
 *    xTextElt *elt;
 *    BufAlloc (xTextElt *, elt, nbytes)
 */

#define BufAlloc(type, ptr, n) \
    if (dpy->bufptr + (n) > dpy->bufmax) \
        _XFlush (dpy); \
    ptr = (type) dpy->bufptr; \
    memset(ptr, '\0', n); \
    dpy->bufptr += (n);

#define Data16(dpy, data, len) Data((dpy), (_Xconst char *)(data), (len))
#define _XRead16Pad(dpy, data, len) _XReadPad((dpy), (char *)(data), (len))
#define _XRead16(dpy, data, len) _XRead((dpy), (char *)(data), (len))
#ifdef LONG64
#define Data32(dpy, data, len) _XData32(dpy, (_Xconst long *)data, len)
extern int _XData32(
	     Display *dpy,
	     _Xconst long *data,
	     unsigned len
);
extern void _XRead32(
	     Display *dpy,
	     long *data,
	     long len
);
#else
#define Data32(dpy, data, len) Data((dpy), (_Xconst char *)(data), (len))
#define _XRead32(dpy, data, len) _XRead((dpy), (char *)(data), (len))
#endif

#define PackData16(dpy,data,len) Data16 (dpy, data, len)
#define PackData32(dpy,data,len) Data32 (dpy, data, len)

/* Xlib manual is bogus */
#define PackData(dpy,data,len) PackData16 (dpy, data, len)

#define min(a,b) (((a) < (b)) ? (a) : (b))
#define max(a,b) (((a) > (b)) ? (a) : (b))

#define CI_NONEXISTCHAR(cs) (((cs)->width == 0) && \
			     (((cs)->rbearing|(cs)->lbearing| \
			       (cs)->ascent|(cs)->descent) == 0))

/*
 * CI_GET_CHAR_INFO_1D - return the charinfo struct for the indicated 8bit
 * character.  If the character is in the column and exists, then return the
 * appropriate metrics (note that fonts with common per-character metrics will
 * return min_bounds).  If none of these hold true, try again with the default
 * char.
 */
#define CI_GET_CHAR_INFO_1D(fs,col,def,cs) \
{ \
    cs = def; \
    if (col >= fs->min_char_or_byte2 && col <= fs->max_char_or_byte2) { \
	if (fs->per_char == NULL) { \
	    cs = &fs->min_bounds; \
	} else { \
	    cs = &fs->per_char[(col - fs->min_char_or_byte2)]; \
	    if (CI_NONEXISTCHAR(cs)) cs = def; \
	} \
    } \
}

#define CI_GET_DEFAULT_INFO_1D(fs,cs) \
  CI_GET_CHAR_INFO_1D (fs, fs->default_char, NULL, cs)



/*
 * CI_GET_CHAR_INFO_2D - return the charinfo struct for the indicated row and
 * column.  This is used for fonts that have more than row zero.
 */
#define CI_GET_CHAR_INFO_2D(fs,row,col,def,cs) \
{ \
    cs = def; \
    if (row >= fs->min_byte1 && row <= fs->max_byte1 && \
	col >= fs->min_char_or_byte2 && col <= fs->max_char_or_byte2) { \
	if (fs->per_char == NULL) { \
	    cs = &fs->min_bounds; \
	} else { \
	    cs = &fs->per_char[((row - fs->min_byte1) * \
			        (fs->max_char_or_byte2 - \
				 fs->min_char_or_byte2 + 1)) + \
			       (col - fs->min_char_or_byte2)]; \
	    if (CI_NONEXISTCHAR(cs)) cs = def; \
        } \
    } \
}

#define CI_GET_DEFAULT_INFO_2D(fs,cs) \
{ \
    unsigned int r = (fs->default_char >> 8); \
    unsigned int c = (fs->default_char & 0xff); \
    CI_GET_CHAR_INFO_2D (fs, r, c, NULL, cs); \
}


/* srcvar must be a variable for large architecture version */
#define OneDataCard32(dpy,dstaddr,srcvar) \
  { *(CARD32 *)(dstaddr) = (srcvar); }


typedef struct _XInternalAsync {
    struct _XInternalAsync *next;
    /*
     * handler arguments:
     * rep is the generic reply that caused this handler
     * to be invoked.  It must also be passed to _XGetAsyncReply.
     * buf and len are opaque values that must be passed to
     * _XGetAsyncReply or _XGetAsyncData.
     * data is the closure stored in this struct.
     * The handler returns True iff it handled this reply.
     */
    Bool (*handler)(
		    Display*	/* dpy */,
		    xReply*	/* rep */,
		    char*	/* buf */,
		    int		/* len */,
		    XPointer	/* data */
		    );
    XPointer data;
} _XAsyncHandler;

/*
 * This struct is part of the ABI and is defined by value
 * in user-code. This means that we cannot make
 * the sequence-numbers 64bit.
 */
typedef struct _XAsyncEState {
    unsigned long min_sequence_number;
    unsigned long max_sequence_number;
    unsigned char error_code;
    unsigned char major_opcode;
    unsigned short minor_opcode;
    unsigned char last_error_received;
    int error_count;
} _XAsyncErrorState;

extern void _XDeqAsyncHandler(Display *dpy, _XAsyncHandler *handler);
#define DeqAsyncHandler(dpy,handler) { \
    if (dpy->async_handlers == (handler)) \
	dpy->async_handlers = (handler)->next; \
    else \
	_XDeqAsyncHandler(dpy, handler); \
    }

typedef void (*FreeFuncType) (
    Display*	/* display */
);

typedef int (*FreeModmapType) (
    XModifierKeymap*	/* modmap */
);

/*
 * This structure is private to the library.
 */
typedef struct _XFreeFuncs {
    FreeFuncType atoms;		/* _XFreeAtomTable */
    FreeModmapType modifiermap;	/* XFreeModifiermap */
    FreeFuncType key_bindings;	/* _XFreeKeyBindings */
    FreeFuncType context_db;	/* _XFreeContextDB */
    FreeFuncType defaultCCCs;	/* _XcmsFreeDefaultCCCs */
    FreeFuncType clientCmaps;	/* _XcmsFreeClientCmaps */
    FreeFuncType intensityMaps;	/* _XcmsFreeIntensityMaps */
    FreeFuncType im_filters;	/* _XFreeIMFilters */
    FreeFuncType xkb;		/* _XkbFreeInfo */
} _XFreeFuncRec;

/* types for InitExt.c */
typedef int (*CreateGCType) (
    Display*	/* display */,
    GC		/* gc */,
    XExtCodes*	/* codes */
);

typedef int (*CopyGCType)(
    Display*	/* display */,
    GC		/* gc */,
    XExtCodes*	/* codes */
);

typedef int (*FlushGCType) (
    Display*	/* display */,
    GC		/* gc */,
    XExtCodes*	/* codes */
);

typedef int (*FreeGCType) (
    Display*	/* display */,
    GC		/* gc */,
    XExtCodes*	/* codes */
);

typedef int (*CreateFontType) (
    Display*	/* display */,
    XFontStruct* /* fs */,
    XExtCodes*	/* codes */
);

typedef int (*FreeFontType) (
    Display*	/* display */,
    XFontStruct* /* fs */,
    XExtCodes*	/* codes */
);

typedef int (*CloseDisplayType) (
    Display*	/* display */,
    XExtCodes*	/* codes */
);

typedef int (*ErrorType) (
    Display*	/* display */,
    xError*	/* err */,
    XExtCodes*	/* codes */,
    int*	/* ret_code */
);

typedef char* (*ErrorStringType) (
    Display*	/* display */,
    int		/* code */,
    XExtCodes*	/* codes */,
    char*	/* buffer */,
    int		/* nbytes */
);

typedef void (*PrintErrorType)(
    Display*	/* display */,
    XErrorEvent* /* ev */,
    void*	/* fp */
);

typedef void (*BeforeFlushType)(
    Display*	/* display */,
    XExtCodes*	/* codes */,
    _Xconst char* /* data */,
    long	/* len */
);

/*
 * This structure is private to the library.
 */
typedef struct _XExten {		/* private to extension mechanism */
	struct _XExten *next;		/* next in list */
	XExtCodes codes;		/* public information, all extension told */
	CreateGCType create_GC;		/* routine to call when GC created */
	CopyGCType copy_GC;		/* routine to call when GC copied */
	FlushGCType flush_GC;		/* routine to call when GC flushed */
	FreeGCType free_GC;		/* routine to call when GC freed */
	CreateFontType create_Font;	/* routine to call when Font created */
	FreeFontType free_Font;		/* routine to call when Font freed */
	CloseDisplayType close_display;	/* routine to call when connection closed */
	ErrorType error;		/* who to call when an error occurs */
	ErrorStringType error_string;	/* routine to supply error string */
	char *name;			/* name of this extension */
	PrintErrorType error_values;	/* routine to supply error values */
	BeforeFlushType before_flush;	/* routine to call when sending data */
	struct _XExten *next_flush;	/* next in list of those with flushes */
} _XExtension;

/* extension hooks */

#ifdef DataRoutineIsProcedure
extern void Data(Display *dpy, char *data, long len);
#endif
extern int _XError(
    Display*	/* dpy */,
    xError*	/* rep */
);
extern int _XIOError(
    Display*	/* dpy */
);
extern int (*_XIOErrorFunction)(
    Display*	/* dpy */
);
extern int (*_XErrorFunction)(
    Display*		/* dpy */,
    XErrorEvent*	/* error_event */
);
extern void _XEatData(
    Display*		/* dpy */,
    unsigned long	/* n */
) _X_COLD;
extern void _XEatDataWords(
    Display*		/* dpy */,
    unsigned long	/* n */
) _X_COLD;
#if defined(__SUNPRO_C) /* Studio compiler alternative to "cold" attribute */
# pragma rarely_called(_XEatData, _XEatDataWords)
#endif
extern char *_XAllocScratch(
    Display*		/* dpy */,
    unsigned long	/* nbytes */
);
extern char *_XAllocTemp(
    Display*		/* dpy */,
    unsigned long	/* nbytes */
);
extern void _XFreeTemp(
    Display*		/* dpy */,
    char*		/* buf */,
    unsigned long	/* nbytes */
);
extern Visual *_XVIDtoVisual(
    Display*	/* dpy */,
    VisualID	/* id */
);
extern unsigned long _XSetLastRequestRead(
    Display*		/* dpy */,
    xGenericReply*	/* rep */
);
extern int _XGetHostname(
    char*	/* buf */,
    int		/* maxlen */
);
extern Screen *_XScreenOfWindow(
    Display*	/* dpy */,
    Window	/* w */
);
extern Bool _XAsyncErrorHandler(
    Display*	/* dpy */,
    xReply*	/* rep */,
    char*	/* buf */,
    int		/* len */,
    XPointer	/* data */
);
extern char *_XGetAsyncReply(
    Display*	/* dpy */,
    char*	/* replbuf */,
    xReply*	/* rep */,
    char*	/* buf */,
    int		/* len */,
    int		/* extra */,
    Bool	/* discard */
);
extern void _XGetAsyncData(
    Display*	/* dpy */,
    char *	/* data */,
    char *	/* buf */,
    int		/* len */,
    int		/* skip */,
    int		/* datalen */,
    int		/* discardtotal */
);
extern void _XFlush(
    Display*	/* dpy */
);
extern int _XEventsQueued(
    Display*	/* dpy */,
    int 	/* mode */
);
extern void _XReadEvents(
    Display*	/* dpy */
);
extern int _XRead(
    Display*	/* dpy */,
    char*	/* data */,
    long	/* size */
);
extern void _XReadPad(
    Display*	/* dpy */,
    char*	/* data */,
    long	/* size */
);
extern void _XSend(
    Display*		/* dpy */,
    _Xconst char*	/* data */,
    long		/* size */
);
extern Status _XReply(
    Display*	/* dpy */,
    xReply*	/* rep */,
    int		/* extra */,
    Bool	/* discard */
);
extern void _XEnq(
    Display*	/* dpy */,
    xEvent*	/* event */
);
extern void _XDeq(
    Display*	/* dpy */,
    _XQEvent*	/* prev */,
    _XQEvent*	/* qelt */
);

extern Bool _XUnknownWireEvent(
    Display*	/* dpy */,
    XEvent*	/* re */,
    xEvent*	/* event */
);

extern Bool _XUnknownWireEventCookie(
    Display*	/* dpy */,
    XGenericEventCookie*	/* re */,
    xEvent*	/* event */
);

extern Bool _XUnknownCopyEventCookie(
    Display*	/* dpy */,
    XGenericEventCookie*	/* in */,
    XGenericEventCookie*	/* out */
);

extern Status _XUnknownNativeEvent(
    Display*	/* dpy */,
    XEvent*	/* re */,
    xEvent*	/* event */
);

extern Bool _XWireToEvent(Display *dpy, XEvent *re, xEvent *event);
extern Bool _XDefaultWireError(Display *display, XErrorEvent *he, xError *we);
extern Bool _XPollfdCacheInit(Display *dpy);
extern void _XPollfdCacheAdd(Display *dpy, int fd);
extern void _XPollfdCacheDel(Display *dpy, int fd);
extern XID _XAllocID(Display *dpy);
extern void _XAllocIDs(Display *dpy, XID *ids, int count);

extern int _XFreeExtData(
    XExtData*	/* extension */
);

extern int (*XESetCreateGC(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
	      GC			/* gc */,
	      XExtCodes*		/* codes */
	    )		/* proc */
))(
    Display*, GC, XExtCodes*
);

extern int (*XESetCopyGC(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              GC			/* gc */,
              XExtCodes*		/* codes */
            )		/* proc */
))(
    Display*, GC, XExtCodes*
);

extern int (*XESetFlushGC(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              GC			/* gc */,
              XExtCodes*		/* codes */
            )		/* proc */
))(
    Display*, GC, XExtCodes*
);

extern int (*XESetFreeGC(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              GC			/* gc */,
              XExtCodes*		/* codes */
            )		/* proc */
))(
    Display*, GC, XExtCodes*
);

extern int (*XESetCreateFont(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              XFontStruct*		/* fs */,
              XExtCodes*		/* codes */
            )		/* proc */
))(
    Display*, XFontStruct*, XExtCodes*
);

extern int (*XESetFreeFont(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              XFontStruct*		/* fs */,
              XExtCodes*		/* codes */
            )		/* proc */
))(
    Display*, XFontStruct*, XExtCodes*
);

extern int (*XESetCloseDisplay(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              XExtCodes*		/* codes */
            )		/* proc */
))(
    Display*, XExtCodes*
);

extern int (*XESetError(
    Display*		/* display */,
    int			/* extension */,
    int (*) (
	      Display*			/* display */,
              xError*			/* err */,
              XExtCodes*		/* codes */,
              int*			/* ret_code */
            )		/* proc */
))(
    Display*, xError*, XExtCodes*, int*
);

extern char* (*XESetErrorString(
    Display*		/* display */,
    int			/* extension */,
    char* (*) (
	        Display*		/* display */,
                int			/* code */,
                XExtCodes*		/* codes */,
                char*			/* buffer */,
                int			/* nbytes */
              )		/* proc */
))(
    Display*, int, XExtCodes*, char*, int
);

extern void (*XESetPrintErrorValues (
    Display*		/* display */,
    int			/* extension */,
    void (*)(
	      Display*			/* display */,
	      XErrorEvent*		/* ev */,
	      void*			/* fp */
	     )		/* proc */
))(
    Display*, XErrorEvent*, void*
);

extern Bool (*XESetWireToEvent(
    Display*		/* display */,
    int			/* event_number */,
    Bool (*) (
	       Display*			/* display */,
               XEvent*			/* re */,
               xEvent*			/* event */
             )		/* proc */
))(
    Display*, XEvent*, xEvent*
);

extern Bool (*XESetWireToEventCookie(
    Display*		/* display */,
    int			/* extension */,
    Bool (*) (
	       Display*			/* display */,
               XGenericEventCookie*	/* re */,
               xEvent*			/* event */
             )		/* proc */
))(
    Display*, XGenericEventCookie*, xEvent*
);

extern Bool (*XESetCopyEventCookie(
    Display*		/* display */,
    int			/* extension */,
    Bool (*) (
	       Display*			/* display */,
               XGenericEventCookie*	/* in */,
               XGenericEventCookie*	/* out */
             )		/* proc */
))(
    Display*, XGenericEventCookie*, XGenericEventCookie*
);


extern Status (*XESetEventToWire(
    Display*		/* display */,
    int			/* event_number */,
    Status (*) (
	      Display*			/* display */,
              XEvent*			/* re */,
              xEvent*			/* event */
            )		/* proc */
))(
    Display*, XEvent*, xEvent*
);

extern Bool (*XESetWireToError(
    Display*		/* display */,
    int			/* error_number */,
    Bool (*) (
	       Display*			/* display */,
	       XErrorEvent*		/* he */,
	       xError*			/* we */
            )		/* proc */
))(
    Display*, XErrorEvent*, xError*
);

extern void (*XESetBeforeFlush(
    Display*		/* display */,
    int			/* error_number */,
    void (*) (
	       Display*			/* display */,
	       XExtCodes*		/* codes */,
	       _Xconst char*		/* data */,
	       long			/* len */
            )		/* proc */
))(
    Display*, XExtCodes*, _Xconst char*, long
);

/* internal connections for IMs */

typedef void (*_XInternalConnectionProc)(
    Display*			/* dpy */,
    int				/* fd */,
    XPointer			/* call_data */
);


extern Status _XRegisterInternalConnection(
    Display*			/* dpy */,
    int				/* fd */,
    _XInternalConnectionProc	/* callback */,
    XPointer			/* call_data */
);

extern void _XUnregisterInternalConnection(
    Display*			/* dpy */,
    int				/* fd */
);

extern void _XProcessInternalConnection(
    Display*			/* dpy */,
    struct _XConnectionInfo*	/* conn_info */
);

/* Display structure has pointers to these */

struct _XConnectionInfo {	/* info from _XRegisterInternalConnection */
    int fd;
    _XInternalConnectionProc read_callback;
    XPointer call_data;
    XPointer *watch_data;	/* set/used by XConnectionWatchProc */
    struct _XConnectionInfo *next;
};

struct _XConnWatchInfo {	/* info from XAddConnectionWatch */
    XConnectionWatchProc fn;
    XPointer client_data;
    struct _XConnWatchInfo *next;
};

#ifdef __UNIXOS2__
extern char* __XOS2RedirRoot(
    char*
);
#endif

extern int _XTextHeight(
    XFontStruct*	/* font_struct */,
    _Xconst char*	/* string */,
    int			/* count */
);

extern int _XTextHeight16(
    XFontStruct*	/* font_struct */,
    _Xconst XChar2b*	/* string */,
    int			/* count */
);

#if defined(WIN32)

extern int _XOpenFile(
    _Xconst char*	/* path */,
    int			/* flags */
);

extern int _XOpenFileMode(
    _Xconst char*	/* path */,
    int			/* flags */,
    mode_t              /* mode */
);

extern void* _XFopenFile(
    _Xconst char*	/* path */,
    _Xconst char*	/* mode */
);

extern int _XAccessFile(
    _Xconst char*	/* path */
);
#else
#define _XOpenFile(path,flags) open(path,flags)
#define _XOpenFileMode(path,flags,mode) open(path,flags,mode)
#define _XFopenFile(path,mode) fopen(path,mode)
#endif

/* EvToWire.c */
extern Status _XEventToWire(Display *dpy, XEvent *re, xEvent *event);

extern int _XF86LoadQueryLocaleFont(
    Display*		/* dpy */,
    _Xconst char*	/* name*/,
    XFontStruct**	/* xfp*/,
    Font*		/* fidp */
);

extern void _XProcessWindowAttributes (
    Display *dpy,
    xChangeWindowAttributesReq *req,
    unsigned long valuemask,
    XSetWindowAttributes *attributes);

extern int _XDefaultError(
        Display *dpy,
        XErrorEvent *event);

extern int _XDefaultIOError(
        Display *dpy);

extern void _XDefaultIOErrorExit(
    Display *dpy,
    void *user_data);

extern void _XSetClipRectangles (
    Display *dpy,
    GC gc,
    int clip_x_origin, int clip_y_origin,
    XRectangle *rectangles,
    int n,
    int ordering);

Status _XGetWindowAttributes(
    Display *dpy,
    Window w,
    XWindowAttributes *attr);

int _XPutBackEvent (
    Display *dpy,
    XEvent *event);

extern Bool _XIsEventCookie(
        Display *dpy,
        XEvent *ev);

extern void _XFreeEventCookies(
        Display *dpy);

extern void _XStoreEventCookie(
        Display *dpy,
        XEvent *ev);

extern Bool _XFetchEventCookie(
        Display *dpy,
        XGenericEventCookie *ev);

extern Bool _XCopyEventCookie(
        Display *dpy,
        XGenericEventCookie *in,
        XGenericEventCookie *out);

/* lcFile.c */

extern void xlocaledir(
    char *buf,
    int buf_len
);

#ifdef __clang__
#pragma clang diagnostic pop
#endif

_XFUNCPROTOEND

#endif /* _X11_XLIBINT_H_ */
