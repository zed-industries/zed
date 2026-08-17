/* libFLAC - Free Lossless Audio Codec library
 * Copyright (C) 2004-2009  Josh Coalson
 * Copyright (C) 2011-2016  Xiph.Org Foundation
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * - Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * - Redistributions in binary form must reproduce the above copyright
 * notice, this list of conditions and the following disclaimer in the
 * documentation and/or other materials provided with the distribution.
 *
 * - Neither the name of the Xiph.org Foundation nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef FLAC__CALLBACK_H
#define FLAC__CALLBACK_H

#include "ordinals.h"
#include <stdlib.h> /* for size_t */

/** \file include/FLAC/callback.h
 *
 *  \brief
 *  This module defines the structures for describing I/O callbacks
 *  to the other FLAC interfaces.
 *
 *  See the detailed documentation for callbacks in the
 *  \link flac_callbacks callbacks \endlink module.
 */

/** \defgroup flac_callbacks FLAC/callback.h: I/O callback structures
 *  \ingroup flac
 *
 *  \brief
 *  This module defines the structures for describing I/O callbacks
 *  to the other FLAC interfaces.
 *
 *  The purpose of the I/O callback functions is to create a common way
 *  for the metadata interfaces to handle I/O.
 *
 *  Originally the metadata interfaces required filenames as the way of
 *  specifying FLAC files to operate on.  This is problematic in some
 *  environments so there is an additional option to specify a set of
 *  callbacks for doing I/O on the FLAC file, instead of the filename.
 *
 *  In addition to the callbacks, a FLAC__IOHandle type is defined as an
 *  opaque structure for a data source.
 *
 *  The callback function prototypes are similar (but not identical) to the
 *  stdio functions fread, fwrite, fseek, ftell, feof, and fclose.  If you use
 *  stdio streams to implement the callbacks, you can pass fread, fwrite, and
 *  fclose anywhere a FLAC__IOCallback_Read, FLAC__IOCallback_Write, or
 *  FLAC__IOCallback_Close is required, and a FILE* anywhere a FLAC__IOHandle
 *  is required.  \warning You generally CANNOT directly use fseek or ftell
 *  for FLAC__IOCallback_Seek or FLAC__IOCallback_Tell since on most systems
 *  these use 32-bit offsets and FLAC requires 64-bit offsets to deal with
 *  large files.  You will have to find an equivalent function (e.g. ftello),
 *  or write a wrapper.  The same is true for feof() since this is usually
 *  implemented as a macro, not as a function whose address can be taken.
 *
 * \{
 */

#ifdef __cplusplus
extern "C" {
#endif

/** This is the opaque handle type used by the callbacks.  Typically
 *  this is a \c FILE* or address of a file descriptor.
 */
typedef void* FLAC__IOHandle;

/** Signature for the read callback.
 *  The signature and semantics match POSIX fread() implementations
 *  and can generally be used interchangeably.
 *
 * \param  ptr      The address of the read buffer.
 * \param  size     The size of the records to be read.
 * \param  nmemb    The number of records to be read.
 * \param  handle   The handle to the data source.
 * \retval size_t
 *    The number of records read.
 */
typedef size_t (*FLAC__IOCallback_Read) (void *ptr, size_t size, size_t nmemb, FLAC__IOHandle handle);

/** Signature for the write callback.
 *  The signature and semantics match POSIX fwrite() implementations
 *  and can generally be used interchangeably.
 *
 * \param  ptr      The address of the write buffer.
 * \param  size     The size of the records to be written.
 * \param  nmemb    The number of records to be written.
 * \param  handle   The handle to the data source.
 * \retval size_t
 *    The number of records written.
 */
typedef size_t (*FLAC__IOCallback_Write) (const void *ptr, size_t size, size_t nmemb, FLAC__IOHandle handle);

/** Signature for the seek callback.
 *  The signature and semantics mostly match POSIX fseek() WITH ONE IMPORTANT
 *  EXCEPTION: the offset is a 64-bit type whereas fseek() is generally 'long'
 *  and 32-bits wide.
 *
 * \param  handle   The handle to the data source.
 * \param  offset   The new position, relative to \a whence
 * \param  whence   \c SEEK_SET, \c SEEK_CUR, or \c SEEK_END
 * \retval int
 *    \c 0 on success, \c -1 on error.
 */
typedef int (*FLAC__IOCallback_Seek) (FLAC__IOHandle handle, FLAC__int64 offset, int whence);

/** Signature for the tell callback.
 *  The signature and semantics mostly match POSIX ftell() WITH ONE IMPORTANT
 *  EXCEPTION: the offset is a 64-bit type whereas ftell() is generally 'long'
 *  and 32-bits wide.
 *
 * \param  handle   The handle to the data source.
 * \retval FLAC__int64
 *    The current position on success, \c -1 on error.
 */
typedef FLAC__int64 (*FLAC__IOCallback_Tell) (FLAC__IOHandle handle);

/** Signature for the EOF callback.
 *  The signature and semantics mostly match POSIX feof() but WATCHOUT:
 *  on many systems, feof() is a macro, so in this case a wrapper function
 *  must be provided instead.
 *
 * \param  handle   The handle to the data source.
 * \retval int
 *    \c 0 if not at end of file, nonzero if at end of file.
 */
typedef int (*FLAC__IOCallback_Eof) (FLAC__IOHandle handle);

/** Signature for the close callback.
 *  The signature and semantics match POSIX fclose() implementations
 *  and can generally be used interchangeably.
 *
 * \param  handle   The handle to the data source.
 * \retval int
 *    \c 0 on success, \c EOF on error.
 */
typedef int (*FLAC__IOCallback_Close) (FLAC__IOHandle handle);

/** A structure for holding a set of callbacks.
 *  Each FLAC interface that requires a FLAC__IOCallbacks structure will
 *  describe which of the callbacks are required.  The ones that are not
 *  required may be set to NULL.
 *
 *  If the seek requirement for an interface is optional, you can signify that
 *  a data source is not seekable by setting the \a seek field to \c NULL.
 */
typedef struct {
	FLAC__IOCallback_Read read;
	FLAC__IOCallback_Write write;
	FLAC__IOCallback_Seek seek;
	FLAC__IOCallback_Tell tell;
	FLAC__IOCallback_Eof eof;
	FLAC__IOCallback_Close close;
} FLAC__IOCallbacks;

/* \} */

#ifdef __cplusplus
}
#endif

#endif
