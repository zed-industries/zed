/* gcrypt.h -  GNU Cryptographic Library Interface              -*- c -*-
 * Copyright (C) 1998-2017 Free Software Foundation, Inc.
 * Copyright (C) 2012-2017 g10 Code GmbH
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 *
 * File: src/gcrypt.h.  Generated from gcrypt.h.in by configure.
 */

#ifndef _GCRYPT_H
#define _GCRYPT_H

#include <stdlib.h>
#include <stdarg.h>
#include <string.h>

#include <gpg-error.h>

#include <sys/types.h>

#if defined _WIN32 || defined __WIN32__
# include <winsock2.h>
# include <ws2tcpip.h>
# include <time.h>
# ifndef __GNUC__
  typedef long ssize_t;
  typedef int  pid_t;
# endif /*!__GNUC__*/
#else
# include <sys/socket.h>
# include <sys/time.h>
# include <sys/select.h>
#endif /*!_WIN32*/

typedef socklen_t gcry_socklen_t;

/* This is required for error code compatibility. */
#define _GCRY_ERR_SOURCE_DEFAULT GPG_ERR_SOURCE_GCRYPT

#ifdef __cplusplus
extern "C" {
#if 0 /* (Keep Emacsens' auto-indent happy.) */
}
#endif
#endif

/* The version of this header should match the one of the library. It
   should not be used by a program because gcry_check_version() should
   return the same version.  The purpose of this macro is to let
   autoconf (using the AM_PATH_GCRYPT macro) check that this header
   matches the installed library.  */
#define GCRYPT_VERSION "1.8.8"

/* The version number of this header.  It may be used to handle minor
   API incompatibilities.  */
#define GCRYPT_VERSION_NUMBER 0x010808


/* Internal: We can't use the convenience macros for the multi
   precision integer functions when building this library. */
#ifdef _GCRYPT_IN_LIBGCRYPT
#ifndef GCRYPT_NO_MPI_MACROS
#define GCRYPT_NO_MPI_MACROS 1
#endif
#endif

/* We want to use gcc attributes when possible.  Warning: Don't use
   these macros in your programs: As indicated by the leading
   underscore they are subject to change without notice. */
#ifdef __GNUC__

#define _GCRY_GCC_VERSION (__GNUC__ * 10000 \
                             + __GNUC_MINOR__ * 100 \
                             + __GNUC_PATCHLEVEL__)

#if _GCRY_GCC_VERSION >= 30100
#define _GCRY_GCC_ATTR_DEPRECATED __attribute__ ((__deprecated__))
#endif

#if _GCRY_GCC_VERSION >= 29600
#define _GCRY_GCC_ATTR_PURE  __attribute__ ((__pure__))
#endif

#if _GCRY_GCC_VERSION >= 30200
#define _GCRY_GCC_ATTR_MALLOC  __attribute__ ((__malloc__))
#endif

#define _GCRY_GCC_ATTR_PRINTF(f,a)  __attribute__ ((format (printf,f,a)))

#if _GCRY_GCC_VERSION >= 40000
#define _GCRY_GCC_ATTR_SENTINEL(a) __attribute__ ((sentinel(a)))
#endif

#endif /*__GNUC__*/

#ifndef _GCRY_GCC_ATTR_DEPRECATED
#define _GCRY_GCC_ATTR_DEPRECATED
#endif
#ifndef _GCRY_GCC_ATTR_PURE
#define _GCRY_GCC_ATTR_PURE
#endif
#ifndef _GCRY_GCC_ATTR_MALLOC
#define _GCRY_GCC_ATTR_MALLOC
#endif
#ifndef _GCRY_GCC_ATTR_PRINTF
#define _GCRY_GCC_ATTR_PRINTF(f,a)
#endif
#ifndef _GCRY_GCC_ATTR_SENTINEL
#define _GCRY_GCC_ATTR_SENTINEL(a)
#endif

/* Make up an attribute to mark functions and types as deprecated but
   allow internal use by Libgcrypt.  */
#ifdef _GCRYPT_IN_LIBGCRYPT
#define _GCRY_ATTR_INTERNAL
#else
#define _GCRY_ATTR_INTERNAL	_GCRY_GCC_ATTR_DEPRECATED
#endif

/* Wrappers for the libgpg-error library.  */

typedef gpg_error_t gcry_error_t;
typedef gpg_err_code_t gcry_err_code_t;
typedef gpg_err_source_t gcry_err_source_t;

static GPG_ERR_INLINE gcry_error_t
gcry_err_make (gcry_err_source_t source, gcry_err_code_t code)
{
  return gpg_err_make (source, code);
}

/* The user can define GPG_ERR_SOURCE_DEFAULT before including this
   file to specify a default source for gpg_error.  */
#ifndef GCRY_ERR_SOURCE_DEFAULT
#define GCRY_ERR_SOURCE_DEFAULT  GPG_ERR_SOURCE_USER_1
#endif

static GPG_ERR_INLINE gcry_error_t
gcry_error (gcry_err_code_t code)
{
  return gcry_err_make (GCRY_ERR_SOURCE_DEFAULT, code);
}

static GPG_ERR_INLINE gcry_err_code_t
gcry_err_code (gcry_error_t err)
{
  return gpg_err_code (err);
}


static GPG_ERR_INLINE gcry_err_source_t
gcry_err_source (gcry_error_t err)
{
  return gpg_err_source (err);
}

/* Return a pointer to a string containing a description of the error
   code in the error value ERR.  */
const char *gcry_strerror (gcry_error_t err);

/* Return a pointer to a string containing a description of the error
   source in the error value ERR.  */
const char *gcry_strsource (gcry_error_t err);

/* Retrieve the error code for the system error ERR.  This returns
   GPG_ERR_UNKNOWN_ERRNO if the system error is not mapped (report
   this).  */
gcry_err_code_t gcry_err_code_from_errno (int err);

/* Retrieve the system error for the error code CODE.  This returns 0
   if CODE is not a system error code.  */
int gcry_err_code_to_errno (gcry_err_code_t code);

/* Return an error value with the error source SOURCE and the system
   error ERR.  */
gcry_error_t gcry_err_make_from_errno (gcry_err_source_t source, int err);

/* Return an error value with the system error ERR.  */
gcry_error_t gcry_error_from_errno (int err);


/* NOTE: Since Libgcrypt 1.6 the thread callbacks are not anymore
   used.  However we keep it to allow for some source code
   compatibility if used in the standard way.  */

/* Constants defining the thread model to use.  Used with the OPTION
   field of the struct gcry_thread_cbs.  */
#define GCRY_THREAD_OPTION_DEFAULT  0
#define GCRY_THREAD_OPTION_USER     1
#define GCRY_THREAD_OPTION_PTH      2
#define GCRY_THREAD_OPTION_PTHREAD  3

/* The version number encoded in the OPTION field of the struct
   gcry_thread_cbs.  */
#define GCRY_THREAD_OPTION_VERSION  1

/* Wrapper for struct ath_ops.  */
struct gcry_thread_cbs
{
  /* The OPTION field encodes the thread model and the version number
     of this structure.
       Bits  7 - 0  are used for the thread model
       Bits 15 - 8  are used for the version number.  */
  unsigned int option;
} _GCRY_ATTR_INTERNAL;

#define GCRY_THREAD_OPTION_PTH_IMPL                                     \
  static struct gcry_thread_cbs gcry_threads_pth = {                    \
    (GCRY_THREAD_OPTION_PTH | (GCRY_THREAD_OPTION_VERSION << 8))}

#define GCRY_THREAD_OPTION_PTHREAD_IMPL                                 \
  static struct gcry_thread_cbs gcry_threads_pthread = {                \
    (GCRY_THREAD_OPTION_PTHREAD | (GCRY_THREAD_OPTION_VERSION << 8))}



/* A generic context object as used by some functions.  */
struct gcry_context;
typedef struct gcry_context *gcry_ctx_t;

/* The data objects used to hold multi precision integers.  */
struct gcry_mpi;
typedef struct gcry_mpi *gcry_mpi_t;
struct gcry_mpi_point;
typedef struct gcry_mpi_point *gcry_mpi_point_t;

#ifndef GCRYPT_NO_DEPRECATED
typedef struct gcry_mpi *GCRY_MPI _GCRY_GCC_ATTR_DEPRECATED;
typedef struct gcry_mpi *GcryMPI _GCRY_GCC_ATTR_DEPRECATED;
#endif

/* A structure used for scatter gather hashing.  */
typedef struct
{
  size_t size;  /* The allocated size of the buffer or 0.  */
  size_t off;   /* Offset into the buffer.  */
  size_t len;   /* The used length of the buffer.  */
  void *data;   /* The buffer.  */
} gcry_buffer_t;




/* Check that the library fulfills the version requirement.  */
const char *gcry_check_version (const char *req_version);

/* Codes for function dispatchers.  */

/* Codes used with the gcry_control function. */
enum gcry_ctl_cmds
  {
    /* Note: 1 .. 2 are not anymore used. */
    GCRYCTL_CFB_SYNC = 3,
    GCRYCTL_RESET    = 4,   /* e.g. for MDs */
    GCRYCTL_FINALIZE = 5,
    GCRYCTL_GET_KEYLEN = 6,
    GCRYCTL_GET_BLKLEN = 7,
    GCRYCTL_TEST_ALGO = 8,
    GCRYCTL_IS_SECURE = 9,
    GCRYCTL_GET_ASNOID = 10,
    GCRYCTL_ENABLE_ALGO = 11,
    GCRYCTL_DISABLE_ALGO = 12,
    GCRYCTL_DUMP_RANDOM_STATS = 13,
    GCRYCTL_DUMP_SECMEM_STATS = 14,
    GCRYCTL_GET_ALGO_NPKEY    = 15,
    GCRYCTL_GET_ALGO_NSKEY    = 16,
    GCRYCTL_GET_ALGO_NSIGN    = 17,
    GCRYCTL_GET_ALGO_NENCR    = 18,
    GCRYCTL_SET_VERBOSITY     = 19,
    GCRYCTL_SET_DEBUG_FLAGS   = 20,
    GCRYCTL_CLEAR_DEBUG_FLAGS = 21,
    GCRYCTL_USE_SECURE_RNDPOOL= 22,
    GCRYCTL_DUMP_MEMORY_STATS = 23,
    GCRYCTL_INIT_SECMEM       = 24,
    GCRYCTL_TERM_SECMEM       = 25,
    GCRYCTL_DISABLE_SECMEM_WARN = 27,
    GCRYCTL_SUSPEND_SECMEM_WARN = 28,
    GCRYCTL_RESUME_SECMEM_WARN  = 29,
    GCRYCTL_DROP_PRIVS          = 30,
    GCRYCTL_ENABLE_M_GUARD      = 31,
    GCRYCTL_START_DUMP          = 32,
    GCRYCTL_STOP_DUMP           = 33,
    GCRYCTL_GET_ALGO_USAGE      = 34,
    GCRYCTL_IS_ALGO_ENABLED     = 35,
    GCRYCTL_DISABLE_INTERNAL_LOCKING = 36,
    GCRYCTL_DISABLE_SECMEM      = 37,
    GCRYCTL_INITIALIZATION_FINISHED = 38,
    GCRYCTL_INITIALIZATION_FINISHED_P = 39,
    GCRYCTL_ANY_INITIALIZATION_P = 40,
    GCRYCTL_SET_CBC_CTS = 41,
    GCRYCTL_SET_CBC_MAC = 42,
    /* Note: 43 is not anymore used. */
    GCRYCTL_ENABLE_QUICK_RANDOM = 44,
    GCRYCTL_SET_RANDOM_SEED_FILE = 45,
    GCRYCTL_UPDATE_RANDOM_SEED_FILE = 46,
    GCRYCTL_SET_THREAD_CBS = 47,
    GCRYCTL_FAST_POLL = 48,
    GCRYCTL_SET_RANDOM_DAEMON_SOCKET = 49,
    GCRYCTL_USE_RANDOM_DAEMON = 50,
    GCRYCTL_FAKED_RANDOM_P = 51,
    GCRYCTL_SET_RNDEGD_SOCKET = 52,
    GCRYCTL_PRINT_CONFIG = 53,
    GCRYCTL_OPERATIONAL_P = 54,
    GCRYCTL_FIPS_MODE_P = 55,
    GCRYCTL_FORCE_FIPS_MODE = 56,
    GCRYCTL_SELFTEST = 57,
    /* Note: 58 .. 62 are used internally.  */
    GCRYCTL_DISABLE_HWF = 63,
    GCRYCTL_SET_ENFORCED_FIPS_FLAG = 64,
    GCRYCTL_SET_PREFERRED_RNG_TYPE = 65,
    GCRYCTL_GET_CURRENT_RNG_TYPE = 66,
    GCRYCTL_DISABLE_LOCKED_SECMEM = 67,
    GCRYCTL_DISABLE_PRIV_DROP = 68,
    GCRYCTL_SET_CCM_LENGTHS = 69,
    GCRYCTL_CLOSE_RANDOM_DEVICE = 70,
    GCRYCTL_INACTIVATE_FIPS_FLAG = 71,
    GCRYCTL_REACTIVATE_FIPS_FLAG = 72,
    GCRYCTL_SET_SBOX = 73,
    GCRYCTL_DRBG_REINIT = 74,
    GCRYCTL_SET_TAGLEN = 75,
    GCRYCTL_GET_TAGLEN = 76,
    GCRYCTL_REINIT_SYSCALL_CLAMP = 77
  };

/* Perform various operations defined by CMD. */
gcry_error_t gcry_control (enum gcry_ctl_cmds CMD, ...);


/* S-expression management. */

/* The object to represent an S-expression as used with the public key
   functions.  */
struct gcry_sexp;
typedef struct gcry_sexp *gcry_sexp_t;

#ifndef GCRYPT_NO_DEPRECATED
typedef struct gcry_sexp *GCRY_SEXP _GCRY_GCC_ATTR_DEPRECATED;
typedef struct gcry_sexp *GcrySexp _GCRY_GCC_ATTR_DEPRECATED;
#endif

/* The possible values for the S-expression format. */
enum gcry_sexp_format
  {
    GCRYSEXP_FMT_DEFAULT   = 0,
    GCRYSEXP_FMT_CANON     = 1,
    GCRYSEXP_FMT_BASE64    = 2,
    GCRYSEXP_FMT_ADVANCED  = 3
  };

/* Create an new S-expression object from BUFFER of size LENGTH and
   return it in RETSEXP.  With AUTODETECT set to 0 the data in BUFFER
   is expected to be in canonized format.  */
gcry_error_t gcry_sexp_new (gcry_sexp_t *retsexp,
                            const void *buffer, size_t length,
                            int autodetect);

 /* Same as gcry_sexp_new but allows to pass a FREEFNC which has the
    effect to transfer ownership of BUFFER to the created object.  */
gcry_error_t gcry_sexp_create (gcry_sexp_t *retsexp,
                               void *buffer, size_t length,
                               int autodetect, void (*freefnc) (void *));

/* Scan BUFFER and return a new S-expression object in RETSEXP.  This
   function expects a printf like string in BUFFER.  */
gcry_error_t gcry_sexp_sscan (gcry_sexp_t *retsexp, size_t *erroff,
                              const char *buffer, size_t length);

/* Same as gcry_sexp_sscan but expects a string in FORMAT and can thus
   only be used for certain encodings.  */
gcry_error_t gcry_sexp_build (gcry_sexp_t *retsexp, size_t *erroff,
                              const char *format, ...);

/* Like gcry_sexp_build, but uses an array instead of variable
   function arguments.  */
gcry_error_t gcry_sexp_build_array (gcry_sexp_t *retsexp, size_t *erroff,
				    const char *format, void **arg_list);

/* Release the S-expression object SEXP */
void gcry_sexp_release (gcry_sexp_t sexp);

/* Calculate the length of an canonized S-expression in BUFFER and
   check for a valid encoding. */
size_t gcry_sexp_canon_len (const unsigned char *buffer, size_t length,
                            size_t *erroff, gcry_error_t *errcode);

/* Copies the S-expression object SEXP into BUFFER using the format
   specified in MODE.  */
size_t gcry_sexp_sprint (gcry_sexp_t sexp, int mode, void *buffer,
                         size_t maxlength);

/* Dumps the S-expression object A in a format suitable for debugging
   to Libgcrypt's logging stream.  */
void gcry_sexp_dump (const gcry_sexp_t a);

gcry_sexp_t gcry_sexp_cons (const gcry_sexp_t a, const gcry_sexp_t b);
gcry_sexp_t gcry_sexp_alist (const gcry_sexp_t *array);
gcry_sexp_t gcry_sexp_vlist (const gcry_sexp_t a, ...);
gcry_sexp_t gcry_sexp_append (const gcry_sexp_t a, const gcry_sexp_t n);
gcry_sexp_t gcry_sexp_prepend (const gcry_sexp_t a, const gcry_sexp_t n);

/* Scan the S-expression for a sublist with a type (the car of the
   list) matching the string TOKEN.  If TOKLEN is not 0, the token is
   assumed to be raw memory of this length.  The function returns a
   newly allocated S-expression consisting of the found sublist or
   `NULL' when not found.  */
gcry_sexp_t gcry_sexp_find_token (gcry_sexp_t list,
                                const char *tok, size_t toklen);
/* Return the length of the LIST.  For a valid S-expression this
   should be at least 1.  */
int gcry_sexp_length (const gcry_sexp_t list);

/* Create and return a new S-expression from the element with index
   NUMBER in LIST.  Note that the first element has the index 0.  If
   there is no such element, `NULL' is returned.  */
gcry_sexp_t gcry_sexp_nth (const gcry_sexp_t list, int number);

/* Create and return a new S-expression from the first element in
   LIST; this called the "type" and should always exist and be a
   string. `NULL' is returned in case of a problem.  */
gcry_sexp_t gcry_sexp_car (const gcry_sexp_t list);

/* Create and return a new list form all elements except for the first
   one.  Note, that this function may return an invalid S-expression
   because it is not guaranteed, that the type exists and is a string.
   However, for parsing a complex S-expression it might be useful for
   intermediate lists.  Returns `NULL' on error.  */
gcry_sexp_t gcry_sexp_cdr (const gcry_sexp_t list);

gcry_sexp_t gcry_sexp_cadr (const gcry_sexp_t list);


/* This function is used to get data from a LIST.  A pointer to the
   actual data with index NUMBER is returned and the length of this
   data will be stored to DATALEN.  If there is no data at the given
   index or the index represents another list, `NULL' is returned.
   *Note:* The returned pointer is valid as long as LIST is not
   modified or released.  */
const char *gcry_sexp_nth_data (const gcry_sexp_t list, int number,
                                size_t *datalen);

/* This function is used to get data from a LIST.  A malloced buffer to the
   data with index NUMBER is returned and the length of this
   data will be stored to RLENGTH.  If there is no data at the given
   index or the index represents another list, `NULL' is returned.  */
void *gcry_sexp_nth_buffer (const gcry_sexp_t list, int number,
                            size_t *rlength);

/* This function is used to get and convert data from a LIST.  The
   data is assumed to be a Nul terminated string.  The caller must
   release the returned value using `gcry_free'.  If there is no data
   at the given index, the index represents a list or the value can't
   be converted to a string, `NULL' is returned.  */
char *gcry_sexp_nth_string (gcry_sexp_t list, int number);

/* This function is used to get and convert data from a LIST. This
   data is assumed to be an MPI stored in the format described by
   MPIFMT and returned as a standard Libgcrypt MPI.  The caller must
   release this returned value using `gcry_mpi_release'.  If there is
   no data at the given index, the index represents a list or the
   value can't be converted to an MPI, `NULL' is returned.  */
gcry_mpi_t gcry_sexp_nth_mpi (gcry_sexp_t list, int number, int mpifmt);

/* Extract MPIs from an s-expression using a list of parameters.  The
 * names of these parameters are given by the string LIST.  Some
 * special characters may be given to control the conversion:
 *
 *    + :: Switch to unsigned integer format (default).
 *    - :: Switch to standard signed format.
 *    / :: Switch to opaque format.
 *    & :: Switch to buffer descriptor mode - see below.
 *    ? :: The previous parameter is optional.
 *
 * In general parameter names are single letters.  To use a string for
 * a parameter name, enclose the name in single quotes.
 *
 * Unless in gcry_buffer_t mode for each parameter name a pointer to
 * an MPI variable is expected that must be set to NULL prior to
 * invoking this function, and finally a NULL is expected.  Example:
 *
 *   _gcry_sexp_extract_param (key, NULL, "n/x+ed",
 *                             &mpi_n, &mpi_x, &mpi_e, NULL)
 *
 * This stores the parameter "N" from KEY as an unsigned MPI into
 * MPI_N, the parameter "X" as an opaque MPI into MPI_X, and the
 * parameter "E" again as an unsigned MPI into MPI_E.
 *
 * If in buffer descriptor mode a pointer to gcry_buffer_t descriptor
 * is expected instead of a pointer to an MPI.  The caller may use two
 * different operation modes: If the DATA field of the provided buffer
 * descriptor is NULL, the function allocates a new buffer and stores
 * it at DATA; the other fields are set accordingly with OFF being 0.
 * If DATA is not NULL, the function assumes that DATA, SIZE, and OFF
 * describe a buffer where to but the data; on return the LEN field
 * receives the number of bytes copied to that buffer; if the buffer
 * is too small, the function immediately returns with an error code
 * (and LEN set to 0).
 *
 * PATH is an optional string used to locate a token.  The exclamation
 * mark separated tokens are used to via gcry_sexp_find_token to find
 * a start point inside SEXP.
 *
 * The function returns 0 on success.  On error an error code is
 * returned, all passed MPIs that might have been allocated up to this
 * point are deallocated and set to NULL, and all passed buffers are
 * either truncated if the caller supplied the buffer, or deallocated
 * if the function allocated the buffer.
 */
gpg_error_t gcry_sexp_extract_param (gcry_sexp_t sexp,
                                     const char *path,
                                     const char *list,
                                     ...) _GCRY_GCC_ATTR_SENTINEL(0);


/*******************************************
 *                                         *
 *  Multi Precision Integer Functions      *
 *                                         *
 *******************************************/

/* Different formats of external big integer representation. */
enum gcry_mpi_format
  {
    GCRYMPI_FMT_NONE= 0,
    GCRYMPI_FMT_STD = 1,    /* Twos complement stored without length.  */
    GCRYMPI_FMT_PGP = 2,    /* As used by OpenPGP (unsigned only).  */
    GCRYMPI_FMT_SSH = 3,    /* As used by SSH (like STD but with length).  */
    GCRYMPI_FMT_HEX = 4,    /* Hex format. */
    GCRYMPI_FMT_USG = 5,    /* Like STD but unsigned. */
    GCRYMPI_FMT_OPAQUE = 8  /* Opaque format (some functions only).  */
  };

/* Flags used for creating big integers.  */
enum gcry_mpi_flag
  {
    GCRYMPI_FLAG_SECURE = 1,  /* Allocate the number in "secure" memory.  */
    GCRYMPI_FLAG_OPAQUE = 2,  /* The number is not a real one but just
                                 a way to store some bytes.  This is
                                 useful for encrypted big integers.  */
    GCRYMPI_FLAG_IMMUTABLE = 4, /* Mark the MPI as immutable.  */
    GCRYMPI_FLAG_CONST     = 8, /* Mark the MPI as a constant.  */
    GCRYMPI_FLAG_USER1 = 0x0100,/* User flag 1.  */
    GCRYMPI_FLAG_USER2 = 0x0200,/* User flag 2.  */
    GCRYMPI_FLAG_USER3 = 0x0400,/* User flag 3.  */
    GCRYMPI_FLAG_USER4 = 0x0800 /* User flag 4.  */
  };


/* Macros to return pre-defined MPI constants.  */
#define GCRYMPI_CONST_ONE   (_gcry_mpi_get_const (1))
#define GCRYMPI_CONST_TWO   (_gcry_mpi_get_const (2))
#define GCRYMPI_CONST_THREE (_gcry_mpi_get_const (3))
#define GCRYMPI_CONST_FOUR  (_gcry_mpi_get_const (4))
#define GCRYMPI_CONST_EIGHT (_gcry_mpi_get_const (8))

/* Allocate a new big integer object, initialize it with 0 and
   initially allocate memory for a number of at least NBITS. */
gcry_mpi_t gcry_mpi_new (unsigned int nbits);

/* Same as gcry_mpi_new() but allocate in "secure" memory. */
gcry_mpi_t gcry_mpi_snew (unsigned int nbits);

/* Release the number A and free all associated resources. */
void gcry_mpi_release (gcry_mpi_t a);

/* Create a new number with the same value as A. */
gcry_mpi_t gcry_mpi_copy (const gcry_mpi_t a);

/* Store the big integer value U in W and release U.  */
void gcry_mpi_snatch (gcry_mpi_t w, gcry_mpi_t u);

/* Store the big integer value U in W. */
gcry_mpi_t gcry_mpi_set (gcry_mpi_t w, const gcry_mpi_t u);

/* Store the unsigned integer value U in W. */
gcry_mpi_t gcry_mpi_set_ui (gcry_mpi_t w, unsigned long u);

/* Swap the values of A and B. */
void gcry_mpi_swap (gcry_mpi_t a, gcry_mpi_t b);

/* Return 1 if A is negative; 0 if zero or positive.  */
int gcry_mpi_is_neg (gcry_mpi_t a);

/* W = - U */
void gcry_mpi_neg (gcry_mpi_t w, gcry_mpi_t u);

/* W = [W] */
void gcry_mpi_abs (gcry_mpi_t w);

/* Compare the big integer number U and V returning 0 for equality, a
   positive value for U > V and a negative for U < V. */
int gcry_mpi_cmp (const gcry_mpi_t u, const gcry_mpi_t v);

/* Compare the big integer number U with the unsigned integer V
   returning 0 for equality, a positive value for U > V and a negative
   for U < V. */
int gcry_mpi_cmp_ui (const gcry_mpi_t u, unsigned long v);

/* Convert the external representation of an integer stored in BUFFER
   with a length of BUFLEN into a newly create MPI returned in
   RET_MPI.  If NSCANNED is not NULL, it will receive the number of
   bytes actually scanned after a successful operation. */
gcry_error_t gcry_mpi_scan (gcry_mpi_t *ret_mpi, enum gcry_mpi_format format,
                            const void *buffer, size_t buflen,
                            size_t *nscanned);

/* Convert the big integer A into the external representation
   described by FORMAT and store it in the provided BUFFER which has
   been allocated by the user with a size of BUFLEN bytes.  NWRITTEN
   receives the actual length of the external representation unless it
   has been passed as NULL. */
gcry_error_t gcry_mpi_print (enum gcry_mpi_format format,
                             unsigned char *buffer, size_t buflen,
                             size_t *nwritten,
                             const gcry_mpi_t a);

/* Convert the big integer A into the external representation described
   by FORMAT and store it in a newly allocated buffer which address
   will be put into BUFFER.  NWRITTEN receives the actual lengths of the
   external representation. */
gcry_error_t gcry_mpi_aprint (enum gcry_mpi_format format,
                              unsigned char **buffer, size_t *nwritten,
                              const gcry_mpi_t a);

/* Dump the value of A in a format suitable for debugging to
   Libgcrypt's logging stream.  Note that one leading space but no
   trailing space or linefeed will be printed.  It is okay to pass
   NULL for A. */
void gcry_mpi_dump (const gcry_mpi_t a);


/* W = U + V.  */
void gcry_mpi_add (gcry_mpi_t w, gcry_mpi_t u, gcry_mpi_t v);

/* W = U + V.  V is an unsigned integer. */
void gcry_mpi_add_ui (gcry_mpi_t w, gcry_mpi_t u, unsigned long v);

/* W = U + V mod M. */
void gcry_mpi_addm (gcry_mpi_t w, gcry_mpi_t u, gcry_mpi_t v, gcry_mpi_t m);

/* W = U - V. */
void gcry_mpi_sub (gcry_mpi_t w, gcry_mpi_t u, gcry_mpi_t v);

/* W = U - V.  V is an unsigned integer. */
void gcry_mpi_sub_ui (gcry_mpi_t w, gcry_mpi_t u, unsigned long v );

/* W = U - V mod M */
void gcry_mpi_subm (gcry_mpi_t w, gcry_mpi_t u, gcry_mpi_t v, gcry_mpi_t m);

/* W = U * V. */
void gcry_mpi_mul (gcry_mpi_t w, gcry_mpi_t u, gcry_mpi_t v);

/* W = U * V.  V is an unsigned integer. */
void gcry_mpi_mul_ui (gcry_mpi_t w, gcry_mpi_t u, unsigned long v );

/* W = U * V mod M. */
void gcry_mpi_mulm (gcry_mpi_t w, gcry_mpi_t u, gcry_mpi_t v, gcry_mpi_t m);

/* W = U * (2 ^ CNT). */
void gcry_mpi_mul_2exp (gcry_mpi_t w, gcry_mpi_t u, unsigned long cnt);

/* Q = DIVIDEND / DIVISOR, R = DIVIDEND % DIVISOR,
   Q or R may be passed as NULL.  ROUND should be negative or 0. */
void gcry_mpi_div (gcry_mpi_t q, gcry_mpi_t r,
                   gcry_mpi_t dividend, gcry_mpi_t divisor, int round);

/* R = DIVIDEND % DIVISOR */
void gcry_mpi_mod (gcry_mpi_t r, gcry_mpi_t dividend, gcry_mpi_t divisor);

/* W = B ^ E mod M. */
void gcry_mpi_powm (gcry_mpi_t w,
                    const gcry_mpi_t b, const gcry_mpi_t e,
                    const gcry_mpi_t m);

/* Set G to the greatest common divisor of A and B.
   Return true if the G is 1. */
int gcry_mpi_gcd (gcry_mpi_t g, gcry_mpi_t a, gcry_mpi_t b);

/* Set X to the multiplicative inverse of A mod M.
   Return true if the value exists. */
int gcry_mpi_invm (gcry_mpi_t x, gcry_mpi_t a, gcry_mpi_t m);

/* Create a new point object.  NBITS is usually 0.  */
gcry_mpi_point_t gcry_mpi_point_new (unsigned int nbits);

/* Release the object POINT.  POINT may be NULL. */
void gcry_mpi_point_release (gcry_mpi_point_t point);

/* Return a copy of POINT. */
gcry_mpi_point_t gcry_mpi_point_copy (gcry_mpi_point_t point);

/* Store the projective coordinates from POINT into X, Y, and Z.  */
void gcry_mpi_point_get (gcry_mpi_t x, gcry_mpi_t y, gcry_mpi_t z,
                         gcry_mpi_point_t point);

/* Store the projective coordinates from POINT into X, Y, and Z and
   release POINT.  */
void gcry_mpi_point_snatch_get (gcry_mpi_t x, gcry_mpi_t y, gcry_mpi_t z,
                                gcry_mpi_point_t point);

/* Store the projective coordinates X, Y, and Z into POINT.  */
gcry_mpi_point_t gcry_mpi_point_set (gcry_mpi_point_t point,
                                     gcry_mpi_t x, gcry_mpi_t y, gcry_mpi_t z);

/* Store the projective coordinates X, Y, and Z into POINT and release
   X, Y, and Z.  */
gcry_mpi_point_t gcry_mpi_point_snatch_set (gcry_mpi_point_t point,
                                            gcry_mpi_t x, gcry_mpi_t y,
                                            gcry_mpi_t z);

/* Allocate a new context for elliptic curve operations based on the
   parameters given by KEYPARAM or using CURVENAME.  */
gpg_error_t gcry_mpi_ec_new (gcry_ctx_t *r_ctx,
                             gcry_sexp_t keyparam, const char *curvename);

/* Get a named MPI from an elliptic curve context.  */
gcry_mpi_t gcry_mpi_ec_get_mpi (const char *name, gcry_ctx_t ctx, int copy);

/* Get a named point from an elliptic curve context.  */
gcry_mpi_point_t gcry_mpi_ec_get_point (const char *name,
                                        gcry_ctx_t ctx, int copy);

/* Store a named MPI into an elliptic curve context.  */
gpg_error_t gcry_mpi_ec_set_mpi (const char *name, gcry_mpi_t newvalue,
                                 gcry_ctx_t ctx);

/* Store a named point into an elliptic curve context.  */
gpg_error_t gcry_mpi_ec_set_point (const char *name, gcry_mpi_point_t newvalue,
                                   gcry_ctx_t ctx);

/* Decode and store VALUE into RESULT.  */
gpg_error_t gcry_mpi_ec_decode_point (gcry_mpi_point_t result,
                                      gcry_mpi_t value, gcry_ctx_t ctx);

/* Store the affine coordinates of POINT into X and Y.  */
int gcry_mpi_ec_get_affine (gcry_mpi_t x, gcry_mpi_t y, gcry_mpi_point_t point,
                            gcry_ctx_t ctx);

/* W = 2 * U.  */
void gcry_mpi_ec_dup (gcry_mpi_point_t w, gcry_mpi_point_t u, gcry_ctx_t ctx);

/* W = U + V.  */
void gcry_mpi_ec_add (gcry_mpi_point_t w,
                      gcry_mpi_point_t u, gcry_mpi_point_t v, gcry_ctx_t ctx);

/* W = U - V.  */
void gcry_mpi_ec_sub (gcry_mpi_point_t w,
                      gcry_mpi_point_t u, gcry_mpi_point_t v, gcry_ctx_t ctx);

/* W = N * U.  */
void gcry_mpi_ec_mul (gcry_mpi_point_t w, gcry_mpi_t n, gcry_mpi_point_t u,
                      gcry_ctx_t ctx);

/* Return true if POINT is on the curve described by CTX.  */
int gcry_mpi_ec_curve_point (gcry_mpi_point_t w, gcry_ctx_t ctx);

/* Return the number of bits required to represent A. */
unsigned int gcry_mpi_get_nbits (gcry_mpi_t a);

/* Return true when bit number N (counting from 0) is set in A. */
int      gcry_mpi_test_bit (gcry_mpi_t a, unsigned int n);

/* Set bit number N in A. */
void     gcry_mpi_set_bit (gcry_mpi_t a, unsigned int n);

/* Clear bit number N in A. */
void     gcry_mpi_clear_bit (gcry_mpi_t a, unsigned int n);

/* Set bit number N in A and clear all bits greater than N. */
void     gcry_mpi_set_highbit (gcry_mpi_t a, unsigned int n);

/* Clear bit number N in A and all bits greater than N. */
void     gcry_mpi_clear_highbit (gcry_mpi_t a, unsigned int n);

/* Shift the value of A by N bits to the right and store the result in X. */
void     gcry_mpi_rshift (gcry_mpi_t x, gcry_mpi_t a, unsigned int n);

/* Shift the value of A by N bits to the left and store the result in X. */
void     gcry_mpi_lshift (gcry_mpi_t x, gcry_mpi_t a, unsigned int n);

/* Store NBITS of the value P points to in A and mark A as an opaque
   value.  On success A received the the ownership of the value P.
   WARNING: Never use an opaque MPI for anything thing else than
   gcry_mpi_release, gcry_mpi_get_opaque. */
gcry_mpi_t gcry_mpi_set_opaque (gcry_mpi_t a, void *p, unsigned int nbits);

/* Store NBITS of the value P points to in A and mark A as an opaque
   value.  The function takes a copy of the provided value P.
   WARNING: Never use an opaque MPI for anything thing else than
   gcry_mpi_release, gcry_mpi_get_opaque. */
gcry_mpi_t gcry_mpi_set_opaque_copy (gcry_mpi_t a,
                                     const void *p, unsigned int nbits);

/* Return a pointer to an opaque value stored in A and return its size
   in NBITS.  Note that the returned pointer is still owned by A and
   that the function should never be used for an non-opaque MPI. */
void *gcry_mpi_get_opaque (gcry_mpi_t a, unsigned int *nbits);

/* Set the FLAG for the big integer A.  Currently only the flag
   GCRYMPI_FLAG_SECURE is allowed to convert A into an big intger
   stored in "secure" memory. */
void gcry_mpi_set_flag (gcry_mpi_t a, enum gcry_mpi_flag flag);

/* Clear FLAG for the big integer A.  Note that this function is
   currently useless as no flags are allowed. */
void gcry_mpi_clear_flag (gcry_mpi_t a, enum gcry_mpi_flag flag);

/* Return true if the FLAG is set for A. */
int gcry_mpi_get_flag (gcry_mpi_t a, enum gcry_mpi_flag flag);

/* Private function - do not use.  */
gcry_mpi_t _gcry_mpi_get_const (int no);

/* Unless the GCRYPT_NO_MPI_MACROS is used, provide a couple of
   convenience macros for the big integer functions. */
#ifndef GCRYPT_NO_MPI_MACROS
#define mpi_new(n)          gcry_mpi_new( (n) )
#define mpi_secure_new( n ) gcry_mpi_snew( (n) )
#define mpi_release(a)      \
  do \
    { \
      gcry_mpi_release ((a)); \
      (a) = NULL; \
    } \
  while (0)

#define mpi_copy( a )          gcry_mpi_copy( (a) )
#define mpi_snatch( w, u)      gcry_mpi_snatch( (w), (u) )
#define mpi_set( w, u)         gcry_mpi_set( (w), (u) )
#define mpi_set_ui( w, u)      gcry_mpi_set_ui( (w), (u) )
#define mpi_abs( w )           gcry_mpi_abs( (w) )
#define mpi_neg( w, u)         gcry_mpi_neg( (w), (u) )
#define mpi_cmp( u, v )        gcry_mpi_cmp( (u), (v) )
#define mpi_cmp_ui( u, v )     gcry_mpi_cmp_ui( (u), (v) )
#define mpi_is_neg( a )        gcry_mpi_is_neg ((a))

#define mpi_add_ui(w,u,v)      gcry_mpi_add_ui((w),(u),(v))
#define mpi_add(w,u,v)         gcry_mpi_add ((w),(u),(v))
#define mpi_addm(w,u,v,m)      gcry_mpi_addm ((w),(u),(v),(m))
#define mpi_sub_ui(w,u,v)      gcry_mpi_sub_ui ((w),(u),(v))
#define mpi_sub(w,u,v)         gcry_mpi_sub ((w),(u),(v))
#define mpi_subm(w,u,v,m)      gcry_mpi_subm ((w),(u),(v),(m))
#define mpi_mul_ui(w,u,v)      gcry_mpi_mul_ui ((w),(u),(v))
#define mpi_mul_2exp(w,u,v)    gcry_mpi_mul_2exp ((w),(u),(v))
#define mpi_mul(w,u,v)         gcry_mpi_mul ((w),(u),(v))
#define mpi_mulm(w,u,v,m)      gcry_mpi_mulm ((w),(u),(v),(m))
#define mpi_powm(w,b,e,m)      gcry_mpi_powm ( (w), (b), (e), (m) )
#define mpi_tdiv(q,r,a,m)      gcry_mpi_div ( (q), (r), (a), (m), 0)
#define mpi_fdiv(q,r,a,m)      gcry_mpi_div ( (q), (r), (a), (m), -1)
#define mpi_mod(r,a,m)         gcry_mpi_mod ((r), (a), (m))
#define mpi_gcd(g,a,b)         gcry_mpi_gcd ( (g), (a), (b) )
#define mpi_invm(g,a,b)        gcry_mpi_invm ( (g), (a), (b) )

#define mpi_point_new(n)              gcry_mpi_point_new((n))
#define mpi_point_release(p)                    \
  do                                            \
    {                                           \
      gcry_mpi_point_release ((p));             \
      (p) = NULL;                               \
    }                                           \
  while (0)
#define mpi_point_copy(p)             gcry_mpi_point_copy((p))
#define mpi_point_get(x,y,z,p)        gcry_mpi_point_get((x),(y),(z),(p))
#define mpi_point_snatch_get(x,y,z,p) gcry_mpi_point_snatch_get((x),(y),(z),(p))
#define mpi_point_set(p,x,y,z)        gcry_mpi_point_set((p),(x),(y),(z))
#define mpi_point_snatch_set(p,x,y,z) gcry_mpi_point_snatch_set((p),(x),(y),(z))

#define mpi_get_nbits(a)       gcry_mpi_get_nbits ((a))
#define mpi_test_bit(a,b)      gcry_mpi_test_bit ((a),(b))
#define mpi_set_bit(a,b)       gcry_mpi_set_bit ((a),(b))
#define mpi_set_highbit(a,b)   gcry_mpi_set_highbit ((a),(b))
#define mpi_clear_bit(a,b)     gcry_mpi_clear_bit ((a),(b))
#define mpi_clear_highbit(a,b) gcry_mpi_clear_highbit ((a),(b))
#define mpi_rshift(a,b,c)      gcry_mpi_rshift ((a),(b),(c))
#define mpi_lshift(a,b,c)      gcry_mpi_lshift ((a),(b),(c))

#define mpi_set_opaque(a,b,c)  gcry_mpi_set_opaque( (a), (b), (c) )
#define mpi_get_opaque(a,b)    gcry_mpi_get_opaque( (a), (b) )
#endif /* GCRYPT_NO_MPI_MACROS */



/************************************
 *                                  *
 *   Symmetric Cipher Functions     *
 *                                  *
 ************************************/

/* The data object used to hold a handle to an encryption object.  */
struct gcry_cipher_handle;
typedef struct gcry_cipher_handle *gcry_cipher_hd_t;

#ifndef GCRYPT_NO_DEPRECATED
typedef struct gcry_cipher_handle *GCRY_CIPHER_HD _GCRY_GCC_ATTR_DEPRECATED;
typedef struct gcry_cipher_handle *GcryCipherHd _GCRY_GCC_ATTR_DEPRECATED;
#endif

/* All symmetric encryption algorithms are identified by their IDs.
   More IDs may be registered at runtime. */
enum gcry_cipher_algos
  {
    GCRY_CIPHER_NONE        = 0,
    GCRY_CIPHER_IDEA        = 1,
    GCRY_CIPHER_3DES        = 2,
    GCRY_CIPHER_CAST5       = 3,
    GCRY_CIPHER_BLOWFISH    = 4,
    GCRY_CIPHER_SAFER_SK128 = 5,
    GCRY_CIPHER_DES_SK      = 6,
    GCRY_CIPHER_AES         = 7,
    GCRY_CIPHER_AES192      = 8,
    GCRY_CIPHER_AES256      = 9,
    GCRY_CIPHER_TWOFISH     = 10,

    /* Other cipher numbers are above 300 for OpenPGP reasons. */
    GCRY_CIPHER_ARCFOUR     = 301,  /* Fully compatible with RSA's RC4 (tm). */
    GCRY_CIPHER_DES         = 302,  /* Yes, this is single key 56 bit DES. */
    GCRY_CIPHER_TWOFISH128  = 303,
    GCRY_CIPHER_SERPENT128  = 304,
    GCRY_CIPHER_SERPENT192  = 305,
    GCRY_CIPHER_SERPENT256  = 306,
    GCRY_CIPHER_RFC2268_40  = 307,  /* Ron's Cipher 2 (40 bit). */
    GCRY_CIPHER_RFC2268_128 = 308,  /* Ron's Cipher 2 (128 bit). */
    GCRY_CIPHER_SEED        = 309,  /* 128 bit cipher described in RFC4269. */
    GCRY_CIPHER_CAMELLIA128 = 310,
    GCRY_CIPHER_CAMELLIA192 = 311,
    GCRY_CIPHER_CAMELLIA256 = 312,
    GCRY_CIPHER_SALSA20     = 313,
    GCRY_CIPHER_SALSA20R12  = 314,
    GCRY_CIPHER_GOST28147   = 315,
    GCRY_CIPHER_CHACHA20    = 316
  };

/* The Rijndael algorithm is basically AES, so provide some macros. */
#define GCRY_CIPHER_AES128      GCRY_CIPHER_AES
#define GCRY_CIPHER_RIJNDAEL    GCRY_CIPHER_AES
#define GCRY_CIPHER_RIJNDAEL128 GCRY_CIPHER_AES128
#define GCRY_CIPHER_RIJNDAEL192 GCRY_CIPHER_AES192
#define GCRY_CIPHER_RIJNDAEL256 GCRY_CIPHER_AES256

/* The supported encryption modes.  Note that not all of them are
   supported for each algorithm. */
enum gcry_cipher_modes
  {
    GCRY_CIPHER_MODE_NONE     = 0,   /* Not yet specified. */
    GCRY_CIPHER_MODE_ECB      = 1,   /* Electronic codebook. */
    GCRY_CIPHER_MODE_CFB      = 2,   /* Cipher feedback. */
    GCRY_CIPHER_MODE_CBC      = 3,   /* Cipher block chaining. */
    GCRY_CIPHER_MODE_STREAM   = 4,   /* Used with stream ciphers. */
    GCRY_CIPHER_MODE_OFB      = 5,   /* Outer feedback. */
    GCRY_CIPHER_MODE_CTR      = 6,   /* Counter. */
    GCRY_CIPHER_MODE_AESWRAP  = 7,   /* AES-WRAP algorithm.  */
    GCRY_CIPHER_MODE_CCM      = 8,   /* Counter with CBC-MAC.  */
    GCRY_CIPHER_MODE_GCM      = 9,   /* Galois Counter Mode. */
    GCRY_CIPHER_MODE_POLY1305 = 10,  /* Poly1305 based AEAD mode. */
    GCRY_CIPHER_MODE_OCB      = 11,  /* OCB3 mode.  */
    GCRY_CIPHER_MODE_CFB8     = 12,  /* Cipher feedback (8 bit mode). */
    GCRY_CIPHER_MODE_XTS      = 13  /* XTS mode.  */
  };

/* Flags used with the open function. */
enum gcry_cipher_flags
  {
    GCRY_CIPHER_SECURE      = 1,  /* Allocate in secure memory. */
    GCRY_CIPHER_ENABLE_SYNC = 2,  /* Enable CFB sync mode. */
    GCRY_CIPHER_CBC_CTS     = 4,  /* Enable CBC cipher text stealing (CTS). */
    GCRY_CIPHER_CBC_MAC     = 8   /* Enable CBC message auth. code (MAC). */
  };

/* GCM works only with blocks of 128 bits */
#define GCRY_GCM_BLOCK_LEN  (128 / 8)

/* CCM works only with blocks of 128 bits.  */
#define GCRY_CCM_BLOCK_LEN  (128 / 8)

/* OCB works only with blocks of 128 bits.  */
#define GCRY_OCB_BLOCK_LEN  (128 / 8)

/* XTS works only with blocks of 128 bits.  */
#define GCRY_XTS_BLOCK_LEN  (128 / 8)

/* Create a handle for algorithm ALGO to be used in MODE.  FLAGS may
   be given as an bitwise OR of the gcry_cipher_flags values. */
gcry_error_t gcry_cipher_open (gcry_cipher_hd_t *handle,
                              int algo, int mode, unsigned int flags);

/* Close the cipher handle H and release all resource. */
void gcry_cipher_close (gcry_cipher_hd_t h);

/* Perform various operations on the cipher object H. */
gcry_error_t gcry_cipher_ctl (gcry_cipher_hd_t h, int cmd, void *buffer,
                             size_t buflen);

/* Retrieve various information about the cipher object H. */
gcry_error_t gcry_cipher_info (gcry_cipher_hd_t h, int what, void *buffer,
                              size_t *nbytes);

/* Retrieve various information about the cipher algorithm ALGO. */
gcry_error_t gcry_cipher_algo_info (int algo, int what, void *buffer,
                                   size_t *nbytes);

/* Map the cipher algorithm whose ID is contained in ALGORITHM to a
   string representation of the algorithm name.  For unknown algorithm
   IDs this function returns "?".  */
const char *gcry_cipher_algo_name (int algorithm) _GCRY_GCC_ATTR_PURE;

/* Map the algorithm name NAME to an cipher algorithm ID.  Return 0 if
   the algorithm name is not known. */
int gcry_cipher_map_name (const char *name) _GCRY_GCC_ATTR_PURE;

/* Given an ASN.1 object identifier in standard IETF dotted decimal
   format in STRING, return the encryption mode associated with that
   OID or 0 if not known or applicable. */
int gcry_cipher_mode_from_oid (const char *string) _GCRY_GCC_ATTR_PURE;

/* Encrypt the plaintext of size INLEN in IN using the cipher handle H
   into the buffer OUT which has an allocated length of OUTSIZE.  For
   most algorithms it is possible to pass NULL for in and 0 for INLEN
   and do a in-place decryption of the data provided in OUT.  */
gcry_error_t gcry_cipher_encrypt (gcry_cipher_hd_t h,
                                  void *out, size_t outsize,
                                  const void *in, size_t inlen);

/* The counterpart to gcry_cipher_encrypt.  */
gcry_error_t gcry_cipher_decrypt (gcry_cipher_hd_t h,
                                  void *out, size_t outsize,
                                  const void *in, size_t inlen);

/* Set KEY of length KEYLEN bytes for the cipher handle HD.  */
gcry_error_t gcry_cipher_setkey (gcry_cipher_hd_t hd,
                                 const void *key, size_t keylen);


/* Set initialization vector IV of length IVLEN for the cipher handle HD. */
gcry_error_t gcry_cipher_setiv (gcry_cipher_hd_t hd,
                                const void *iv, size_t ivlen);

/* Provide additional authentication data for AEAD modes/ciphers.  */
gcry_error_t gcry_cipher_authenticate (gcry_cipher_hd_t hd, const void *abuf,
                                       size_t abuflen);

/* Get authentication tag for AEAD modes/ciphers.  */
gcry_error_t gcry_cipher_gettag (gcry_cipher_hd_t hd, void *outtag,
                                 size_t taglen);

/* Check authentication tag for AEAD modes/ciphers.  */
gcry_error_t gcry_cipher_checktag (gcry_cipher_hd_t hd, const void *intag,
                                   size_t taglen);

/* Reset the handle to the state after open.  */
#define gcry_cipher_reset(h)  gcry_cipher_ctl ((h), GCRYCTL_RESET, NULL, 0)

/* Perform the OpenPGP sync operation if this is enabled for the
   cipher handle H. */
#define gcry_cipher_sync(h)  gcry_cipher_ctl( (h), GCRYCTL_CFB_SYNC, NULL, 0)

/* Enable or disable CTS in future calls to gcry_encrypt(). CBC mode only. */
#define gcry_cipher_cts(h,on)  gcry_cipher_ctl( (h), GCRYCTL_SET_CBC_CTS, \
                                                                   NULL, on )

#define gcry_cipher_set_sbox(h,oid) gcry_cipher_ctl( (h), GCRYCTL_SET_SBOX, \
                                                     (void *) oid, 0);

/* Indicate to the encrypt and decrypt functions that the next call
   provides the final data.  Only used with some modes.  */
#define gcry_cipher_final(a) \
            gcry_cipher_ctl ((a), GCRYCTL_FINALIZE, NULL, 0)

/* Set counter for CTR mode.  (CTR,CTRLEN) must denote a buffer of
   block size length, or (NULL,0) to set the CTR to the all-zero block. */
gpg_error_t gcry_cipher_setctr (gcry_cipher_hd_t hd,
                                const void *ctr, size_t ctrlen);

/* Retrieve the key length in bytes used with algorithm A. */
size_t gcry_cipher_get_algo_keylen (int algo);

/* Retrieve the block length in bytes used with algorithm A. */
size_t gcry_cipher_get_algo_blklen (int algo);

/* Return 0 if the algorithm A is available for use. */
#define gcry_cipher_test_algo(a) \
            gcry_cipher_algo_info( (a), GCRYCTL_TEST_ALGO, NULL, NULL )


/************************************
 *                                  *
 *    Asymmetric Cipher Functions   *
 *                                  *
 ************************************/

/* The algorithms and their IDs we support.  */
enum gcry_pk_algos
  {
    GCRY_PK_RSA   = 1,      /* RSA */
    GCRY_PK_RSA_E = 2,      /* (deprecated: use 1).  */
    GCRY_PK_RSA_S = 3,      /* (deprecated: use 1).  */
    GCRY_PK_ELG_E = 16,     /* (deprecated: use 20). */
    GCRY_PK_DSA   = 17,     /* Digital Signature Algorithm.  */
    GCRY_PK_ECC   = 18,     /* Generic ECC.  */
    GCRY_PK_ELG   = 20,     /* Elgamal       */
    GCRY_PK_ECDSA = 301,    /* (only for external use).  */
    GCRY_PK_ECDH  = 302,    /* (only for external use).  */
    GCRY_PK_EDDSA = 303     /* (only for external use).  */
  };

/* Flags describing usage capabilities of a PK algorithm. */
#define GCRY_PK_USAGE_SIGN 1   /* Good for signatures. */
#define GCRY_PK_USAGE_ENCR 2   /* Good for encryption. */
#define GCRY_PK_USAGE_CERT 4   /* Good to certify other keys. */
#define GCRY_PK_USAGE_AUTH 8   /* Good for authentication. */
#define GCRY_PK_USAGE_UNKN 128 /* Unknown usage flag. */

/* Modes used with gcry_pubkey_get_sexp.  */
#define GCRY_PK_GET_PUBKEY 1
#define GCRY_PK_GET_SECKEY 2

/* Encrypt the DATA using the public key PKEY and store the result as
   a newly created S-expression at RESULT. */
gcry_error_t gcry_pk_encrypt (gcry_sexp_t *result,
                              gcry_sexp_t data, gcry_sexp_t pkey);

/* Decrypt the DATA using the private key SKEY and store the result as
   a newly created S-expression at RESULT. */
gcry_error_t gcry_pk_decrypt (gcry_sexp_t *result,
                              gcry_sexp_t data, gcry_sexp_t skey);

/* Sign the DATA using the private key SKEY and store the result as
   a newly created S-expression at RESULT. */
gcry_error_t gcry_pk_sign (gcry_sexp_t *result,
                           gcry_sexp_t data, gcry_sexp_t skey);

/* Check the signature SIGVAL on DATA using the public key PKEY. */
gcry_error_t gcry_pk_verify (gcry_sexp_t sigval,
                             gcry_sexp_t data, gcry_sexp_t pkey);

/* Check that private KEY is sane. */
gcry_error_t gcry_pk_testkey (gcry_sexp_t key);

/* Generate a new key pair according to the parameters given in
   S_PARMS.  The new key pair is returned in as an S-expression in
   R_KEY. */
gcry_error_t gcry_pk_genkey (gcry_sexp_t *r_key, gcry_sexp_t s_parms);

/* Catch all function for miscellaneous operations. */
gcry_error_t gcry_pk_ctl (int cmd, void *buffer, size_t buflen);

/* Retrieve information about the public key algorithm ALGO. */
gcry_error_t gcry_pk_algo_info (int algo, int what,
                                void *buffer, size_t *nbytes);

/* Map the public key algorithm whose ID is contained in ALGORITHM to
   a string representation of the algorithm name.  For unknown
   algorithm IDs this functions returns "?". */
const char *gcry_pk_algo_name (int algorithm) _GCRY_GCC_ATTR_PURE;

/* Map the algorithm NAME to a public key algorithm Id.  Return 0 if
   the algorithm name is not known. */
int gcry_pk_map_name (const char* name) _GCRY_GCC_ATTR_PURE;

/* Return what is commonly referred as the key length for the given
   public or private KEY.  */
unsigned int gcry_pk_get_nbits (gcry_sexp_t key) _GCRY_GCC_ATTR_PURE;

/* Return the so called KEYGRIP which is the SHA-1 hash of the public
   key parameters expressed in a way depending on the algorithm.  */
unsigned char *gcry_pk_get_keygrip (gcry_sexp_t key, unsigned char *array);

/* Return the name of the curve matching KEY.  */
const char *gcry_pk_get_curve (gcry_sexp_t key, int iterator,
                               unsigned int *r_nbits);

/* Return an S-expression with the parameters of the named ECC curve
   NAME.  ALGO must be set to an ECC algorithm.  */
gcry_sexp_t gcry_pk_get_param (int algo, const char *name);

/* Return 0 if the public key algorithm A is available for use. */
#define gcry_pk_test_algo(a) \
            gcry_pk_algo_info( (a), GCRYCTL_TEST_ALGO, NULL, NULL )

/* Return an S-expression representing the context CTX.  */
gcry_error_t gcry_pubkey_get_sexp (gcry_sexp_t *r_sexp,
                                   int mode, gcry_ctx_t ctx);



/************************************
 *                                  *
 *   Cryptograhic Hash Functions    *
 *                                  *
 ************************************/

/* Algorithm IDs for the hash functions we know about. Not all of them
   are implemented. */
enum gcry_md_algos
  {
    GCRY_MD_NONE    = 0,
    GCRY_MD_MD5     = 1,
    GCRY_MD_SHA1    = 2,
    GCRY_MD_RMD160  = 3,
    GCRY_MD_MD2     = 5,
    GCRY_MD_TIGER   = 6,   /* TIGER/192 as used by gpg <= 1.3.2. */
    GCRY_MD_HAVAL   = 7,   /* HAVAL, 5 pass, 160 bit. */
    GCRY_MD_SHA256  = 8,
    GCRY_MD_SHA384  = 9,
    GCRY_MD_SHA512  = 10,
    GCRY_MD_SHA224  = 11,

    GCRY_MD_MD4           = 301,
    GCRY_MD_CRC32         = 302,
    GCRY_MD_CRC32_RFC1510 = 303,
    GCRY_MD_CRC24_RFC2440 = 304,
    GCRY_MD_WHIRLPOOL     = 305,
    GCRY_MD_TIGER1        = 306, /* TIGER fixed.  */
    GCRY_MD_TIGER2        = 307, /* TIGER2 variant.   */
    GCRY_MD_GOSTR3411_94  = 308, /* GOST R 34.11-94.  */
    GCRY_MD_STRIBOG256    = 309, /* GOST R 34.11-2012, 256 bit.  */
    GCRY_MD_STRIBOG512    = 310, /* GOST R 34.11-2012, 512 bit.  */
    GCRY_MD_GOSTR3411_CP  = 311, /* GOST R 34.11-94 with CryptoPro-A S-Box.  */
    GCRY_MD_SHA3_224      = 312,
    GCRY_MD_SHA3_256      = 313,
    GCRY_MD_SHA3_384      = 314,
    GCRY_MD_SHA3_512      = 315,
    GCRY_MD_SHAKE128      = 316,
    GCRY_MD_SHAKE256      = 317,
    GCRY_MD_BLAKE2B_512   = 318,
    GCRY_MD_BLAKE2B_384   = 319,
    GCRY_MD_BLAKE2B_256   = 320,
    GCRY_MD_BLAKE2B_160   = 321,
    GCRY_MD_BLAKE2S_256   = 322,
    GCRY_MD_BLAKE2S_224   = 323,
    GCRY_MD_BLAKE2S_160   = 324,
    GCRY_MD_BLAKE2S_128   = 325
  };

/* Flags used with the open function.  */
enum gcry_md_flags
  {
    GCRY_MD_FLAG_SECURE = 1,  /* Allocate all buffers in "secure" memory.  */
    GCRY_MD_FLAG_HMAC   = 2,  /* Make an HMAC out of this algorithm.  */
    GCRY_MD_FLAG_BUGEMU1 = 0x0100
  };

/* (Forward declaration.)  */
struct gcry_md_context;

/* This object is used to hold a handle to a message digest object.
   This structure is private - only to be used by the public gcry_md_*
   macros.  */
typedef struct gcry_md_handle
{
  /* Actual context.  */
  struct gcry_md_context *ctx;

  /* Buffer management.  */
  int  bufpos;
  int  bufsize;
  unsigned char buf[1];
} *gcry_md_hd_t;

/* Compatibility types, do not use them.  */
#ifndef GCRYPT_NO_DEPRECATED
typedef struct gcry_md_handle *GCRY_MD_HD _GCRY_GCC_ATTR_DEPRECATED;
typedef struct gcry_md_handle *GcryMDHd _GCRY_GCC_ATTR_DEPRECATED;
#endif

/* Create a message digest object for algorithm ALGO.  FLAGS may be
   given as an bitwise OR of the gcry_md_flags values.  ALGO may be
   given as 0 if the algorithms to be used are later set using
   gcry_md_enable.  */
gcry_error_t gcry_md_open (gcry_md_hd_t *h, int algo, unsigned int flags);

/* Release the message digest object HD.  */
void gcry_md_close (gcry_md_hd_t hd);

/* Add the message digest algorithm ALGO to the digest object HD.  */
gcry_error_t gcry_md_enable (gcry_md_hd_t hd, int algo);

/* Create a new digest object as an exact copy of the object HD.  */
gcry_error_t gcry_md_copy (gcry_md_hd_t *bhd, gcry_md_hd_t ahd);

/* Reset the digest object HD to its initial state.  */
void gcry_md_reset (gcry_md_hd_t hd);

/* Perform various operations on the digest object HD. */
gcry_error_t gcry_md_ctl (gcry_md_hd_t hd, int cmd,
                          void *buffer, size_t buflen);

/* Pass LENGTH bytes of data in BUFFER to the digest object HD so that
   it can update the digest values.  This is the actual hash
   function. */
void gcry_md_write (gcry_md_hd_t hd, const void *buffer, size_t length);

/* Read out the final digest from HD return the digest value for
   algorithm ALGO. */
unsigned char *gcry_md_read (gcry_md_hd_t hd, int algo);

/* Read more output from algorithm ALGO to BUFFER of size LENGTH from
 * digest object HD. Algorithm needs to be 'expendable-output function'. */
gpg_error_t gcry_md_extract (gcry_md_hd_t hd, int algo, void *buffer,
                             size_t length);

/* Convenience function to calculate the hash from the data in BUFFER
   of size LENGTH using the algorithm ALGO avoiding the creation of a
   hash object.  The hash is returned in the caller provided buffer
   DIGEST which must be large enough to hold the digest of the given
   algorithm. */
void gcry_md_hash_buffer (int algo, void *digest,
                          const void *buffer, size_t length);

/* Convenience function to hash multiple buffers.  */
gpg_error_t gcry_md_hash_buffers (int algo, unsigned int flags, void *digest,
                                  const gcry_buffer_t *iov, int iovcnt);

/* Retrieve the algorithm used with HD.  This does not work reliable
   if more than one algorithm is enabled in HD. */
int gcry_md_get_algo (gcry_md_hd_t hd);

/* Retrieve the length in bytes of the digest yielded by algorithm
   ALGO. */
unsigned int gcry_md_get_algo_dlen (int algo);

/* Return true if the the algorithm ALGO is enabled in the digest
   object A. */
int gcry_md_is_enabled (gcry_md_hd_t a, int algo);

/* Return true if the digest object A is allocated in "secure" memory. */
int gcry_md_is_secure (gcry_md_hd_t a);

/* Deprecated: Use gcry_md_is_enabled or gcry_md_is_secure.  */
gcry_error_t gcry_md_info (gcry_md_hd_t h, int what, void *buffer,
                          size_t *nbytes) _GCRY_ATTR_INTERNAL;

/* Retrieve various information about the algorithm ALGO.  */
gcry_error_t gcry_md_algo_info (int algo, int what, void *buffer,
                               size_t *nbytes);

/* Map the digest algorithm id ALGO to a string representation of the
   algorithm name.  For unknown algorithms this function returns
   "?". */
const char *gcry_md_algo_name (int algo) _GCRY_GCC_ATTR_PURE;

/* Map the algorithm NAME to a digest algorithm Id.  Return 0 if
   the algorithm name is not known. */
int gcry_md_map_name (const char* name) _GCRY_GCC_ATTR_PURE;

/* For use with the HMAC feature, the set MAC key to the KEY of
   KEYLEN bytes. */
gcry_error_t gcry_md_setkey (gcry_md_hd_t hd, const void *key, size_t keylen);

/* Start or stop debugging for digest handle HD; i.e. create a file
   named dbgmd-<n>.<suffix> while hashing.  If SUFFIX is NULL,
   debugging stops and the file will be closed. */
void gcry_md_debug (gcry_md_hd_t hd, const char *suffix);


/* Update the hash(s) of H with the character C.  This is a buffered
   version of the gcry_md_write function. */
#define gcry_md_putc(h,c)  \
            do {                                          \
                gcry_md_hd_t h__ = (h);                   \
                if( (h__)->bufpos == (h__)->bufsize )     \
                    gcry_md_write( (h__), NULL, 0 );      \
                (h__)->buf[(h__)->bufpos++] = (c) & 0xff; \
            } while(0)

/* Finalize the digest calculation.  This is not really needed because
   gcry_md_read() does this implicitly. */
#define gcry_md_final(a) \
            gcry_md_ctl ((a), GCRYCTL_FINALIZE, NULL, 0)

/* Return 0 if the algorithm A is available for use. */
#define gcry_md_test_algo(a) \
            gcry_md_algo_info( (a), GCRYCTL_TEST_ALGO, NULL, NULL )

/* Return an DER encoded ASN.1 OID for the algorithm A in buffer B. N
   must point to size_t variable with the available size of buffer B.
   After return it will receive the actual size of the returned
   OID. */
#define gcry_md_get_asnoid(a,b,n) \
            gcry_md_algo_info((a), GCRYCTL_GET_ASNOID, (b), (n))



/**********************************************
 *                                            *
 *   Message Authentication Code Functions    *
 *                                            *
 **********************************************/

/* The data object used to hold a handle to an encryption object.  */
struct gcry_mac_handle;
typedef struct gcry_mac_handle *gcry_mac_hd_t;

/* Algorithm IDs for the hash functions we know about. Not all of them
   are implemented. */
enum gcry_mac_algos
  {
    GCRY_MAC_NONE               = 0,

    GCRY_MAC_HMAC_SHA256        = 101,
    GCRY_MAC_HMAC_SHA224        = 102,
    GCRY_MAC_HMAC_SHA512        = 103,
    GCRY_MAC_HMAC_SHA384        = 104,
    GCRY_MAC_HMAC_SHA1          = 105,
    GCRY_MAC_HMAC_MD5           = 106,
    GCRY_MAC_HMAC_MD4           = 107,
    GCRY_MAC_HMAC_RMD160        = 108,
    GCRY_MAC_HMAC_TIGER1        = 109, /* The fixed TIGER variant */
    GCRY_MAC_HMAC_WHIRLPOOL     = 110,
    GCRY_MAC_HMAC_GOSTR3411_94  = 111,
    GCRY_MAC_HMAC_STRIBOG256    = 112,
    GCRY_MAC_HMAC_STRIBOG512    = 113,
    GCRY_MAC_HMAC_MD2           = 114,
    GCRY_MAC_HMAC_SHA3_224      = 115,
    GCRY_MAC_HMAC_SHA3_256      = 116,
    GCRY_MAC_HMAC_SHA3_384      = 117,
    GCRY_MAC_HMAC_SHA3_512      = 118,

    GCRY_MAC_CMAC_AES           = 201,
    GCRY_MAC_CMAC_3DES          = 202,
    GCRY_MAC_CMAC_CAMELLIA      = 203,
    GCRY_MAC_CMAC_CAST5         = 204,
    GCRY_MAC_CMAC_BLOWFISH      = 205,
    GCRY_MAC_CMAC_TWOFISH       = 206,
    GCRY_MAC_CMAC_SERPENT       = 207,
    GCRY_MAC_CMAC_SEED          = 208,
    GCRY_MAC_CMAC_RFC2268       = 209,
    GCRY_MAC_CMAC_IDEA          = 210,
    GCRY_MAC_CMAC_GOST28147     = 211,

    GCRY_MAC_GMAC_AES           = 401,
    GCRY_MAC_GMAC_CAMELLIA      = 402,
    GCRY_MAC_GMAC_TWOFISH       = 403,
    GCRY_MAC_GMAC_SERPENT       = 404,
    GCRY_MAC_GMAC_SEED          = 405,

    GCRY_MAC_POLY1305           = 501,
    GCRY_MAC_POLY1305_AES       = 502,
    GCRY_MAC_POLY1305_CAMELLIA  = 503,
    GCRY_MAC_POLY1305_TWOFISH   = 504,
    GCRY_MAC_POLY1305_SERPENT   = 505,
    GCRY_MAC_POLY1305_SEED      = 506
  };

/* Flags used with the open function.  */
enum gcry_mac_flags
  {
    GCRY_MAC_FLAG_SECURE = 1   /* Allocate all buffers in "secure" memory.  */
  };

/* Create a MAC handle for algorithm ALGO.  FLAGS may be given as an bitwise OR
   of the gcry_mac_flags values.  CTX maybe NULL or gcry_ctx_t object to be
   associated with HANDLE.  */
gcry_error_t gcry_mac_open (gcry_mac_hd_t *handle, int algo,
                            unsigned int flags, gcry_ctx_t ctx);

/* Close the MAC handle H and release all resource. */
void gcry_mac_close (gcry_mac_hd_t h);

/* Perform various operations on the MAC object H. */
gcry_error_t gcry_mac_ctl (gcry_mac_hd_t h, int cmd, void *buffer,
                           size_t buflen);

/* Retrieve various information about the MAC algorithm ALGO. */
gcry_error_t gcry_mac_algo_info (int algo, int what, void *buffer,
                                 size_t *nbytes);

/* Set KEY of length KEYLEN bytes for the MAC handle HD.  */
gcry_error_t gcry_mac_setkey (gcry_mac_hd_t hd, const void *key,
                              size_t keylen);

/* Set initialization vector IV of length IVLEN for the MAC handle HD. */
gcry_error_t gcry_mac_setiv (gcry_mac_hd_t hd, const void *iv,
                             size_t ivlen);

/* Pass LENGTH bytes of data in BUFFER to the MAC object HD so that
   it can update the MAC values.  */
gcry_error_t gcry_mac_write (gcry_mac_hd_t hd, const void *buffer,
                             size_t length);

/* Read out the final authentication code from the MAC object HD to BUFFER. */
gcry_error_t gcry_mac_read (gcry_mac_hd_t hd, void *buffer, size_t *buflen);

/* Verify the final authentication code from the MAC object HD with BUFFER. */
gcry_error_t gcry_mac_verify (gcry_mac_hd_t hd, const void *buffer,
                              size_t buflen);

/* Retrieve the algorithm used with MAC. */
int gcry_mac_get_algo (gcry_mac_hd_t hd);

/* Retrieve the length in bytes of the MAC yielded by algorithm ALGO. */
unsigned int gcry_mac_get_algo_maclen (int algo);

/* Retrieve the default key length in bytes used with algorithm A. */
unsigned int gcry_mac_get_algo_keylen (int algo);

/* Map the MAC algorithm whose ID is contained in ALGORITHM to a
   string representation of the algorithm name.  For unknown algorithm
   IDs this function returns "?".  */
const char *gcry_mac_algo_name (int algorithm) _GCRY_GCC_ATTR_PURE;

/* Map the algorithm name NAME to an MAC algorithm ID.  Return 0 if
   the algorithm name is not known. */
int gcry_mac_map_name (const char *name) _GCRY_GCC_ATTR_PURE;

/* Reset the handle to the state after open/setkey.  */
#define gcry_mac_reset(h)  gcry_mac_ctl ((h), GCRYCTL_RESET, NULL, 0)

/* Return 0 if the algorithm A is available for use. */
#define gcry_mac_test_algo(a) \
            gcry_mac_algo_info( (a), GCRYCTL_TEST_ALGO, NULL, NULL )


/******************************
 *                            *
 *  Key Derivation Functions  *
 *                            *
 ******************************/

/* Algorithm IDs for the KDFs.  */
enum gcry_kdf_algos
  {
    GCRY_KDF_NONE = 0,
    GCRY_KDF_SIMPLE_S2K = 16,
    GCRY_KDF_SALTED_S2K = 17,
    GCRY_KDF_ITERSALTED_S2K = 19,
    GCRY_KDF_PBKDF1 = 33,
    GCRY_KDF_PBKDF2 = 34,
    GCRY_KDF_SCRYPT = 48
  };

/* Derive a key from a passphrase.  */
gpg_error_t gcry_kdf_derive (const void *passphrase, size_t passphraselen,
                             int algo, int subalgo,
                             const void *salt, size_t saltlen,
                             unsigned long iterations,
                             size_t keysize, void *keybuffer);




/************************************
 *                                  *
 *   Random Generating Functions    *
 *                                  *
 ************************************/

/* The type of the random number generator.  */
enum gcry_rng_types
  {
    GCRY_RNG_TYPE_STANDARD   = 1, /* The default CSPRNG generator.  */
    GCRY_RNG_TYPE_FIPS       = 2, /* The FIPS X9.31 AES generator.  */
    GCRY_RNG_TYPE_SYSTEM     = 3  /* The system's native generator. */
  };

/* The possible values for the random quality.  The rule of thumb is
   to use STRONG for session keys and VERY_STRONG for key material.
   WEAK is usually an alias for STRONG and should not be used anymore
   (except with gcry_mpi_randomize); use gcry_create_nonce instead. */
typedef enum gcry_random_level
  {
    GCRY_WEAK_RANDOM = 0,
    GCRY_STRONG_RANDOM = 1,
    GCRY_VERY_STRONG_RANDOM = 2
  }
gcry_random_level_t;

/* Fill BUFFER with LENGTH bytes of random, using random numbers of
   quality LEVEL. */
void gcry_randomize (void *buffer, size_t length,
                     enum gcry_random_level level);

/* Add the external random from BUFFER with LENGTH bytes into the
   pool. QUALITY should either be -1 for unknown or in the range of 0
   to 100 */
gcry_error_t gcry_random_add_bytes (const void *buffer, size_t length,
                                    int quality);

/* If random numbers are used in an application, this macro should be
   called from time to time so that new stuff gets added to the
   internal pool of the RNG.  */
#define gcry_fast_random_poll()  gcry_control (GCRYCTL_FAST_POLL, NULL)


/* Return NBYTES of allocated random using a random numbers of quality
   LEVEL. */
void *gcry_random_bytes (size_t nbytes, enum gcry_random_level level)
                         _GCRY_GCC_ATTR_MALLOC;

/* Return NBYTES of allocated random using a random numbers of quality
   LEVEL.  The random numbers are created returned in "secure"
   memory. */
void *gcry_random_bytes_secure (size_t nbytes, enum gcry_random_level level)
                                _GCRY_GCC_ATTR_MALLOC;


/* Set the big integer W to a random value of NBITS using a random
   generator with quality LEVEL.  Note that by using a level of
   GCRY_WEAK_RANDOM gcry_create_nonce is used internally. */
void gcry_mpi_randomize (gcry_mpi_t w,
                         unsigned int nbits, enum gcry_random_level level);


/* Create an unpredicable nonce of LENGTH bytes in BUFFER. */
void gcry_create_nonce (void *buffer, size_t length);





/*******************************/
/*                             */
/*    Prime Number Functions   */
/*                             */
/*******************************/

/* Mode values passed to a gcry_prime_check_func_t. */
#define GCRY_PRIME_CHECK_AT_FINISH      0
#define GCRY_PRIME_CHECK_AT_GOT_PRIME   1
#define GCRY_PRIME_CHECK_AT_MAYBE_PRIME 2

/* The function should return 1 if the operation shall continue, 0 to
   reject the prime candidate. */
typedef int (*gcry_prime_check_func_t) (void *arg, int mode,
                                        gcry_mpi_t candidate);

/* Flags for gcry_prime_generate():  */

/* Allocate prime numbers and factors in secure memory.  */
#define GCRY_PRIME_FLAG_SECRET         (1 << 0)

/* Make sure that at least one prime factor is of size
   `FACTOR_BITS'.  */
#define GCRY_PRIME_FLAG_SPECIAL_FACTOR (1 << 1)

/* Generate a new prime number of PRIME_BITS bits and store it in
   PRIME.  If FACTOR_BITS is non-zero, one of the prime factors of
   (prime - 1) / 2 must be FACTOR_BITS bits long.  If FACTORS is
   non-zero, allocate a new, NULL-terminated array holding the prime
   factors and store it in FACTORS.  FLAGS might be used to influence
   the prime number generation process.  */
gcry_error_t gcry_prime_generate (gcry_mpi_t *prime,
                                  unsigned int prime_bits,
                                  unsigned int factor_bits,
                                  gcry_mpi_t **factors,
                                  gcry_prime_check_func_t cb_func,
                                  void *cb_arg,
                                  gcry_random_level_t random_level,
                                  unsigned int flags);

/* Find a generator for PRIME where the factorization of (prime-1) is
   in the NULL terminated array FACTORS. Return the generator as a
   newly allocated MPI in R_G.  If START_G is not NULL, use this as
   the start for the search. */
gcry_error_t gcry_prime_group_generator (gcry_mpi_t *r_g,
                                         gcry_mpi_t prime,
                                         gcry_mpi_t *factors,
                                         gcry_mpi_t start_g);


/* Convenience function to release the FACTORS array. */
void gcry_prime_release_factors (gcry_mpi_t *factors);


/* Check whether the number X is prime.  */
gcry_error_t gcry_prime_check (gcry_mpi_t x, unsigned int flags);



/************************************
 *                                  *
 *     Miscellaneous Stuff          *
 *                                  *
 ************************************/

/* Release the context object CTX.  */
void gcry_ctx_release (gcry_ctx_t ctx);

/* Log data using Libgcrypt's own log interface.  */
void gcry_log_debug (const char *fmt, ...) _GCRY_GCC_ATTR_PRINTF(1,2);
void gcry_log_debughex (const char *text, const void *buffer, size_t length);
void gcry_log_debugmpi (const char *text, gcry_mpi_t mpi);
void gcry_log_debugpnt (const char *text,
                        gcry_mpi_point_t point, gcry_ctx_t ctx);
void gcry_log_debugsxp (const char *text, gcry_sexp_t sexp);

char *gcry_get_config (int mode, const char *what);

/* Log levels used by the internal logging facility. */
enum gcry_log_levels
  {
    GCRY_LOG_CONT   = 0,    /* (Continue the last log line.) */
    GCRY_LOG_INFO   = 10,
    GCRY_LOG_WARN   = 20,
    GCRY_LOG_ERROR  = 30,
    GCRY_LOG_FATAL  = 40,
    GCRY_LOG_BUG    = 50,
    GCRY_LOG_DEBUG  = 100
  };

/* Type for progress handlers.  */
typedef void (*gcry_handler_progress_t) (void *, const char *, int, int, int);

/* Type for memory allocation handlers.  */
typedef void *(*gcry_handler_alloc_t) (size_t n);

/* Type for secure memory check handlers.  */
typedef int (*gcry_handler_secure_check_t) (const void *);

/* Type for memory reallocation handlers.  */
typedef void *(*gcry_handler_realloc_t) (void *p, size_t n);

/* Type for memory free handlers.  */
typedef void (*gcry_handler_free_t) (void *);

/* Type for out-of-memory handlers.  */
typedef int (*gcry_handler_no_mem_t) (void *, size_t, unsigned int);

/* Type for fatal error handlers.  */
typedef void (*gcry_handler_error_t) (void *, int, const char *);

/* Type for logging handlers.  */
typedef void (*gcry_handler_log_t) (void *, int, const char *, va_list);

/* Certain operations can provide progress information.  This function
   is used to register a handler for retrieving these information. */
void gcry_set_progress_handler (gcry_handler_progress_t cb, void *cb_data);


/* Register a custom memory allocation functions. */
void gcry_set_allocation_handler (
                             gcry_handler_alloc_t func_alloc,
                             gcry_handler_alloc_t func_alloc_secure,
                             gcry_handler_secure_check_t func_secure_check,
                             gcry_handler_realloc_t func_realloc,
                             gcry_handler_free_t func_free);

/* Register a function used instead of the internal out of memory
   handler. */
void gcry_set_outofcore_handler (gcry_handler_no_mem_t h, void *opaque);

/* Register a function used instead of the internal fatal error
   handler. */
void gcry_set_fatalerror_handler (gcry_handler_error_t fnc, void *opaque);

/* Register a function used instead of the internal logging
   facility. */
void gcry_set_log_handler (gcry_handler_log_t f, void *opaque);

/* Reserved for future use. */
void gcry_set_gettext_handler (const char *(*f)(const char*));

/* Libgcrypt uses its own memory allocation.  It is important to use
   gcry_free () to release memory allocated by libgcrypt. */
void *gcry_malloc (size_t n) _GCRY_GCC_ATTR_MALLOC;
void *gcry_calloc (size_t n, size_t m) _GCRY_GCC_ATTR_MALLOC;
void *gcry_malloc_secure (size_t n) _GCRY_GCC_ATTR_MALLOC;
void *gcry_calloc_secure (size_t n, size_t m) _GCRY_GCC_ATTR_MALLOC;
void *gcry_realloc (void *a, size_t n);
char *gcry_strdup (const char *string) _GCRY_GCC_ATTR_MALLOC;
void *gcry_xmalloc (size_t n) _GCRY_GCC_ATTR_MALLOC;
void *gcry_xcalloc (size_t n, size_t m) _GCRY_GCC_ATTR_MALLOC;
void *gcry_xmalloc_secure (size_t n) _GCRY_GCC_ATTR_MALLOC;
void *gcry_xcalloc_secure (size_t n, size_t m) _GCRY_GCC_ATTR_MALLOC;
void *gcry_xrealloc (void *a, size_t n);
char *gcry_xstrdup (const char * a) _GCRY_GCC_ATTR_MALLOC;
void  gcry_free (void *a);

/* Return true if A is allocated in "secure" memory. */
int gcry_is_secure (const void *a) _GCRY_GCC_ATTR_PURE;

/* Return true if Libgcrypt is in FIPS mode.  */
#define gcry_fips_mode_active()  !!gcry_control (GCRYCTL_FIPS_MODE_P, 0)


#if 0 /* (Keep Emacsens' auto-indent happy.) */
{
#endif
#ifdef __cplusplus
}
#endif
#endif /* _GCRYPT_H */
/*
Local Variables:
buffer-read-only: t
End:
*/
