/*
 * xdr.h, External Data Representation Serialization Routines.
 *
 * Copyright (c) 2010, 2012, Oracle America, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 *       copyright notice, this list of conditions and the following
 *       disclaimer in the documentation and/or other materials
 *       provided with the distribution.
 *     * Neither the name of the "Oracle America, Inc." nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *   FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *   COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 *   INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 *   DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 *   GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 *   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 *   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef _RPC_XDR_H
#define _RPC_XDR_H 1

#include <features.h>
#include <sys/types.h>
#include <rpc/types.h>

/* We need FILE.  */
#include <stdio.h>

__BEGIN_DECLS

/*
 * XDR provides a conventional way for converting between C data
 * types and an external bit-string representation.  Library supplied
 * routines provide for the conversion on built-in C data types.  These
 * routines and utility routines defined here are used to help implement
 * a type encode/decode routine for each user-defined type.
 *
 * Each data type provides a single procedure which takes two arguments:
 *
 *      bool_t
 *      xdrproc(xdrs, argresp)
 *              XDR *xdrs;
 *              <type> *argresp;
 *
 * xdrs is an instance of a XDR handle, to which or from which the data
 * type is to be converted.  argresp is a pointer to the structure to be
 * converted.  The XDR handle contains an operation field which indicates
 * which of the operations (ENCODE, DECODE * or FREE) is to be performed.
 *
 * XDR_DECODE may allocate space if the pointer argresp is null.  This
 * data can be freed with the XDR_FREE operation.
 *
 * We write only one procedure per data type to make it easy
 * to keep the encode and decode procedures for a data type consistent.
 * In many cases the same code performs all operations on a user defined type,
 * because all the hard work is done in the component type routines.
 * decode as a series of calls on the nested data types.
 */

/*
 * Xdr operations.  XDR_ENCODE causes the type to be encoded into the
 * stream.  XDR_DECODE causes the type to be extracted from the stream.
 * XDR_FREE can be used to release the space allocated by an XDR_DECODE
 * request.
 */
enum xdr_op {
  XDR_ENCODE = 0,
  XDR_DECODE = 1,
  XDR_FREE = 2
};

/*
 * This is the number of bytes per unit of external data.
 */
#define BYTES_PER_XDR_UNIT	(4)
/*
 * This only works if the above is a power of 2.  But it's defined to be
 * 4 by the appropriate RFCs.  So it will work.  And it's normally quicker
 * than the old routine.
 */
#if 1
#define RNDUP(x)  (((x) + BYTES_PER_XDR_UNIT - 1) & ~(BYTES_PER_XDR_UNIT - 1))
#else /* this is the old routine */
#define RNDUP(x)  ((((x) + BYTES_PER_XDR_UNIT - 1) / BYTES_PER_XDR_UNIT) \
		    * BYTES_PER_XDR_UNIT)
#endif

/*
 * The XDR handle.
 * Contains operation which is being applied to the stream,
 * an operations vector for the particular implementation (e.g. see xdr_mem.c),
 * and two private fields for the use of the particular implementation.
 */
typedef struct XDR XDR;
struct XDR
  {
    enum xdr_op x_op;		/* operation; fast additional param */
    struct xdr_ops
      {
	bool_t (*x_getlong) (XDR *__xdrs, long *__lp);
	/* get a long from underlying stream */
	bool_t (*x_putlong) (XDR *__xdrs, const long *__lp);
	/* put a long to " */
	bool_t (*x_getbytes) (XDR *__xdrs, caddr_t __addr, u_int __len);
	/* get some bytes from " */
	bool_t (*x_putbytes) (XDR *__xdrs, const char *__addr, u_int __len);
	/* put some bytes to " */
	u_int (*x_getpostn) (const XDR *__xdrs);
	/* returns bytes off from beginning */
	bool_t (*x_setpostn) (XDR *__xdrs, u_int __pos);
	/* lets you reposition the stream */
	int32_t *(*x_inline) (XDR *__xdrs, u_int __len);
	/* buf quick ptr to buffered data */
	void (*x_destroy) (XDR *__xdrs);
	/* free privates of this xdr_stream */
	bool_t (*x_getint32) (XDR *__xdrs, int32_t *__ip);
	/* get a int from underlying stream */
	bool_t (*x_putint32) (XDR *__xdrs, const int32_t *__ip);
	/* put a int to " */
      }
     *x_ops;
    caddr_t x_public;		/* users' data */
    caddr_t x_private;		/* pointer to private data */
    caddr_t x_base;		/* private used for position info */
    u_int x_handy;		/* extra private word */
  };

/*
 * A xdrproc_t exists for each data type which is to be encoded or decoded.
 *
 * The second argument to the xdrproc_t is a pointer to an opaque pointer.
 * The opaque pointer generally points to a structure of the data type
 * to be decoded.  If this pointer is 0, then the type routines should
 * allocate dynamic storage of the appropriate size and return it.
 * bool_t       (*xdrproc_t)(XDR *, caddr_t *);
 */
typedef bool_t (*xdrproc_t) (XDR *, void *,...);


/*
 * Operations defined on a XDR handle
 *
 * XDR          *xdrs;
 * int32_t      *int32p;
 * long         *longp;
 * caddr_t       addr;
 * u_int         len;
 * u_int         pos;
 */
#define XDR_GETINT32(xdrs, int32p)                      \
	(*(xdrs)->x_ops->x_getint32)(xdrs, int32p)
#define xdr_getint32(xdrs, int32p)                      \
	(*(xdrs)->x_ops->x_getint32)(xdrs, int32p)

#define XDR_PUTINT32(xdrs, int32p)                      \
	(*(xdrs)->x_ops->x_putint32)(xdrs, int32p)
#define xdr_putint32(xdrs, int32p)                      \
	(*(xdrs)->x_ops->x_putint32)(xdrs, int32p)

#define XDR_GETLONG(xdrs, longp)			\
	(*(xdrs)->x_ops->x_getlong)(xdrs, longp)
#define xdr_getlong(xdrs, longp)			\
	(*(xdrs)->x_ops->x_getlong)(xdrs, longp)

#define XDR_PUTLONG(xdrs, longp)			\
	(*(xdrs)->x_ops->x_putlong)(xdrs, longp)
#define xdr_putlong(xdrs, longp)			\
	(*(xdrs)->x_ops->x_putlong)(xdrs, longp)

#define XDR_GETBYTES(xdrs, addr, len)			\
	(*(xdrs)->x_ops->x_getbytes)(xdrs, addr, len)
#define xdr_getbytes(xdrs, addr, len)			\
	(*(xdrs)->x_ops->x_getbytes)(xdrs, addr, len)

#define XDR_PUTBYTES(xdrs, addr, len)			\
	(*(xdrs)->x_ops->x_putbytes)(xdrs, addr, len)
#define xdr_putbytes(xdrs, addr, len)			\
	(*(xdrs)->x_ops->x_putbytes)(xdrs, addr, len)

#define XDR_GETPOS(xdrs)				\
	(*(xdrs)->x_ops->x_getpostn)(xdrs)
#define xdr_getpos(xdrs)				\
	(*(xdrs)->x_ops->x_getpostn)(xdrs)

#define XDR_SETPOS(xdrs, pos)				\
	(*(xdrs)->x_ops->x_setpostn)(xdrs, pos)
#define xdr_setpos(xdrs, pos)				\
	(*(xdrs)->x_ops->x_setpostn)(xdrs, pos)

#define	XDR_INLINE(xdrs, len)				\
	(*(xdrs)->x_ops->x_inline)(xdrs, len)
#define	xdr_inline(xdrs, len)				\
	(*(xdrs)->x_ops->x_inline)(xdrs, len)

#define	XDR_DESTROY(xdrs)					\
	do {							\
		if ((xdrs)->x_ops->x_destroy)			\
			(*(xdrs)->x_ops->x_destroy)(xdrs);	\
	} while (0)
#define	xdr_destroy(xdrs)					\
	do {							\
		if ((xdrs)->x_ops->x_destroy)			\
			(*(xdrs)->x_ops->x_destroy)(xdrs);	\
	} while (0)

/*
 * Support struct for discriminated unions.
 * You create an array of xdrdiscrim structures, terminated with
 * an entry with a null procedure pointer.  The xdr_union routine gets
 * the discriminant value and then searches the array of structures
 * for a matching value.  If a match is found the associated xdr routine
 * is called to handle that part of the union.  If there is
 * no match, then a default routine may be called.
 * If there is no match and no default routine it is an error.
 */
#define NULL_xdrproc_t ((xdrproc_t)0)
struct xdr_discrim
{
  int value;
  xdrproc_t proc;
};

/*
 * Inline routines for fast encode/decode of primitive data types.
 * Caveat emptor: these use single memory cycles to get the
 * data from the underlying buffer, and will fail to operate
 * properly if the data is not aligned.  The standard way to use these
 * is to say:
 *      if ((buf = XDR_INLINE(xdrs, count)) == NULL)
 *              return (FALSE);
 *      <<< macro calls >>>
 * where ``count'' is the number of bytes of data occupied
 * by the primitive data types.
 *
 * N.B. and frozen for all time: each data type here uses 4 bytes
 * of external representation.
 */

#define IXDR_GET_INT32(buf)           ((int32_t)ntohl((uint32_t)*(buf)++))
#define IXDR_PUT_INT32(buf, v)        (*(buf)++ = (int32_t)htonl((uint32_t)(v)))
#define IXDR_GET_U_INT32(buf)         ((uint32_t)IXDR_GET_INT32(buf))
#define IXDR_PUT_U_INT32(buf, v)      IXDR_PUT_INT32(buf, (int32_t)(v))

/* WARNING: The IXDR_*_LONG defines are removed by Sun for new platforms
 * and shouldn't be used any longer. Code which use this defines or longs
 * in the RPC code will not work on 64bit Solaris platforms !
 */
#define IXDR_GET_LONG(buf) ((long)IXDR_GET_U_INT32(buf))
#define IXDR_PUT_LONG(buf, v) ((long)IXDR_PUT_INT32(buf, (long)(v)))
#define IXDR_GET_U_LONG(buf)	      ((u_long)IXDR_GET_LONG(buf))
#define IXDR_PUT_U_LONG(buf, v)	      IXDR_PUT_LONG(buf, (long)(v))


#define IXDR_GET_BOOL(buf)            ((bool_t)IXDR_GET_LONG(buf))
#define IXDR_GET_ENUM(buf, t)         ((t)IXDR_GET_LONG(buf))
#define IXDR_GET_SHORT(buf)           ((short)IXDR_GET_LONG(buf))
#define IXDR_GET_U_SHORT(buf)         ((u_short)IXDR_GET_LONG(buf))

#define IXDR_PUT_BOOL(buf, v)         IXDR_PUT_LONG(buf, (long)(v))
#define IXDR_PUT_ENUM(buf, v)         IXDR_PUT_LONG(buf, (long)(v))
#define IXDR_PUT_SHORT(buf, v)        IXDR_PUT_LONG(buf, (long)(v))
#define IXDR_PUT_U_SHORT(buf, v)      IXDR_PUT_LONG(buf, (long)(v))

/*
 * These are the "generic" xdr routines.
 * None of these can have const applied because it's not possible to
 * know whether the call is a read or a write to the passed parameter
 * also, the XDR structure is always updated by some of these calls.
 */
extern bool_t xdr_void (void) __THROW;
extern bool_t xdr_short (XDR *__xdrs, short *__sp) __THROW;
extern bool_t xdr_u_short (XDR *__xdrs, u_short *__usp) __THROW;
extern bool_t xdr_int (XDR *__xdrs, int *__ip) __THROW;
extern bool_t xdr_u_int (XDR *__xdrs, u_int *__up) __THROW;
extern bool_t xdr_long (XDR *__xdrs, long *__lp) __THROW;
extern bool_t xdr_u_long (XDR *__xdrs, u_long *__ulp) __THROW;
extern bool_t xdr_hyper (XDR *__xdrs, quad_t *__llp) __THROW;
extern bool_t xdr_u_hyper (XDR *__xdrs, u_quad_t *__ullp) __THROW;
extern bool_t xdr_longlong_t (XDR *__xdrs, quad_t *__llp) __THROW;
extern bool_t xdr_u_longlong_t (XDR *__xdrs, u_quad_t *__ullp) __THROW;
extern bool_t xdr_int8_t (XDR *__xdrs, int8_t *__ip) __THROW;
extern bool_t xdr_uint8_t (XDR *__xdrs, uint8_t *__up) __THROW;
extern bool_t xdr_int16_t (XDR *__xdrs, int16_t *__ip) __THROW;
extern bool_t xdr_uint16_t (XDR *__xdrs, uint16_t *__up) __THROW;
extern bool_t xdr_int32_t (XDR *__xdrs, int32_t *__ip) __THROW;
extern bool_t xdr_uint32_t (XDR *__xdrs, uint32_t *__up) __THROW;
extern bool_t xdr_int64_t (XDR *__xdrs, int64_t *__ip) __THROW;
extern bool_t xdr_uint64_t (XDR *__xdrs, uint64_t *__up) __THROW;
extern bool_t xdr_quad_t (XDR *__xdrs, quad_t *__ip) __THROW;
extern bool_t xdr_u_quad_t (XDR *__xdrs, u_quad_t *__up) __THROW;
extern bool_t xdr_bool (XDR *__xdrs, bool_t *__bp) __THROW;
extern bool_t xdr_enum (XDR *__xdrs, enum_t *__ep) __THROW;
extern bool_t xdr_array (XDR * _xdrs, caddr_t *__addrp, u_int *__sizep,
			 u_int __maxsize, u_int __elsize, xdrproc_t __elproc)
     __THROW;
extern bool_t xdr_bytes (XDR *__xdrs, char **__cpp, u_int *__sizep,
			 u_int __maxsize) __THROW;
extern bool_t xdr_opaque (XDR *__xdrs, caddr_t __cp, u_int __cnt) __THROW;
extern bool_t xdr_string (XDR *__xdrs, char **__cpp, u_int __maxsize) __THROW;
extern bool_t xdr_union (XDR *__xdrs, enum_t *__dscmp, char *__unp,
			 const struct xdr_discrim *__choices,
			 xdrproc_t __dfault) __THROW;
extern bool_t xdr_char (XDR *__xdrs, char *__cp) __THROW;
extern bool_t xdr_u_char (XDR *__xdrs, u_char *__cp) __THROW;
extern bool_t xdr_vector (XDR *__xdrs, char *__basep, u_int __nelem,
			  u_int __elemsize, xdrproc_t __xdr_elem) __THROW;
extern bool_t xdr_float (XDR *__xdrs, float *__fp) __THROW;
extern bool_t xdr_double (XDR *__xdrs, double *__dp) __THROW;
extern bool_t xdr_reference (XDR *__xdrs, caddr_t *__xpp, u_int __size,
			     xdrproc_t __proc) __THROW;
extern bool_t xdr_pointer (XDR *__xdrs, char **__objpp,
			   u_int __obj_size, xdrproc_t __xdr_obj) __THROW;
extern bool_t xdr_wrapstring (XDR *__xdrs, char **__cpp) __THROW;
extern u_long xdr_sizeof (xdrproc_t, void *) __THROW;

/*
 * Common opaque bytes objects used by many rpc protocols;
 * declared here due to commonality.
 */
#define MAX_NETOBJ_SZ 1024
struct netobj
{
  u_int n_len;
  char *n_bytes;
};
typedef struct netobj netobj;
extern bool_t xdr_netobj (XDR *__xdrs, struct netobj *__np) __THROW;

/*
 * These are the public routines for the various implementations of
 * xdr streams.
 */

/* XDR using memory buffers */
extern void xdrmem_create (XDR *__xdrs, const caddr_t __addr,
			   u_int __size, enum xdr_op __xop) __THROW;

/* XDR using stdio library */
extern void xdrstdio_create (XDR *__xdrs, FILE *__file, enum xdr_op __xop)
     __THROW;

/* XDR pseudo records for tcp */
extern void xdrrec_create (XDR *__xdrs, u_int __sendsize,
			   u_int __recvsize, caddr_t __tcp_handle,
			   int (*__readit) (char *, char *, int),
			   int (*__writeit) (char *, char *, int)) __THROW;

/* make end of xdr record */
extern bool_t xdrrec_endofrecord (XDR *__xdrs, bool_t __sendnow) __THROW;

/* move to beginning of next record */
extern bool_t xdrrec_skiprecord (XDR *__xdrs) __THROW;

/* true if no more input */
extern bool_t xdrrec_eof (XDR *__xdrs) __THROW;

/* free memory buffers for xdr */
extern void xdr_free (xdrproc_t __proc, char *__objp) __THROW;

__END_DECLS

#endif /* rpc/xdr.h */
