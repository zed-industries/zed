/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _plbase64_h
#define _plbase64_h

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/*
 * PL_Base64Encode
 *
 * This routine encodes the data pointed to by the "src" parameter using the
 * base64 algorithm, and returns a pointer to the result.  If the "srclen"
 * parameter is not zero, it specifies the length of the source data.  If it
 * is zero, the source data is assumed to be null-terminated, and PL_strlen
 * is used to determine the source length.  If the "dest" parameter is not
 * null, it is assumed to point to a buffer of sufficient size (which may be
 * calculated: ((srclen + 2)/3)*4) into which the encoded data is placed
 * (without any termination).  If the "dest" parameter is null, a buffer is
 * allocated from the heap to hold the encoded data, and the result *will*
 * be terminated with an extra null character.  It is the caller's
 * responsibility to free the result when it is allocated.  A null is returned
 * if the allocation fails.
 *
 * NOTE: when calculating ((srclen + 2)/3)*4), first ensure that
 *     srclen <= (PR_UINT32_MAX/4) * 3
 * to avoid PRUint32 overflow.
 */

PR_EXTERN(char *)
PL_Base64Encode
(
    const char *src,
    PRUint32    srclen,
    char       *dest
);

/*
 * PL_Base64Decode
 *
 * This routine decodes the data pointed to by the "src" parameter using
 * the base64 algorithm, and returns a pointer to the result.  The source
 * may either include or exclude any trailing '=' characters.  If the
 * "srclen" parameter is not zero, it specifies the length of the source
 * data.  If it is zero, PL_strlen will be used to determine the source
 * length.  If the "dest" parameter is not null, it is assumed to point to
 * a buffer of sufficient size (which may be calculated: (srclen * 3)/4
 * when srclen includes the '=' characters) into which the decoded data
 * is placed (without any termination).  If the "dest" parameter is null,
 * a buffer is allocated from the heap to hold the decoded data, and the
 * result *will* be terminated with an extra null character.  It is the
 * caller's responsibility to free the result when it is allocated.  A null
 * is retuned if the allocation fails, or if the source is not well-coded.
 *
 * NOTE: when calculating (srclen * 3)/4, first ensure that
 *     srclen <= PR_UINT32_MAX/3
 * to avoid PRUint32 overflow.  Alternatively, calculate
 *     (srclen/4) * 3 + ((srclen%4) * 3)/4
 * which is equivalent but doesn't overflow for any value of srclen.
 */

PR_EXTERN(char *)
PL_Base64Decode
(
    const char *src,
    PRUint32    srclen,
    char       *dest
);

PR_END_EXTERN_C

#endif /* _plbase64_h */
