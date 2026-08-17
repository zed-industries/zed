/*
 * Copyright (c) 2020 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#ifndef VA_BACKEND_PROT_H
#define VA_BACKEND_PROT_H

#include <va/va_prot.h>

#ifdef __cplusplus
extern "C" {
#endif

/** \brief VTable version for VA/PROTECTION hooks. */
#define VA_DRIVER_VTABLE_PROT_VERSION 1

struct VADriverVTableProt {
    unsigned int            version;

    VAStatus
    (*vaCreateProtectedSession)(
        VADriverContextP ctx,
        VAConfigID config_id,
        VAProtectedSessionID *protected_session
    );

    VAStatus
    (*vaDestroyProtectedSession)(
        VADriverContextP ctx,
        VAProtectedSessionID protected_session
    );

    VAStatus
    (*vaAttachProtectedSession)(
        VADriverContextP ctx,
        VAContextID context,
        VAProtectedSessionID protected_session
    );

    VAStatus
    (*vaDetachProtectedSession)(
        VADriverContextP ctx,
        VAContextID context
    );

    VAStatus
    (*vaProtectedSessionExecute)(
        VADriverContextP ctx,
        VAProtectedSessionID protected_session,
        VABufferID buf_id
    );

    /** \brief Reserved bytes for future use, must be zero */
    unsigned long reserved[VA_PADDING_MEDIUM];
};

#ifdef __cplusplus
}
#endif

#endif /* VA_BACKEND_PROT_H */
