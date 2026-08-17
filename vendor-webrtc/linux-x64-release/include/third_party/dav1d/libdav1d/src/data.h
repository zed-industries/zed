/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_DATA_H
#define DAV1D_SRC_DATA_H

#include "dav1d/data.h"

void dav1d_data_ref(Dav1dData *dst, const Dav1dData *src);

/**
 * Copy the source properties to the destination and increase the
 * user_data's reference count (if it's not NULL).
 */
void dav1d_data_props_copy(Dav1dDataProps *dst, const Dav1dDataProps *src);

void dav1d_data_props_set_defaults(Dav1dDataProps *props);

uint8_t *dav1d_data_create_internal(Dav1dData *buf, size_t sz);
int dav1d_data_wrap_internal(Dav1dData *buf, const uint8_t *ptr, size_t sz,
                             void (*free_callback)(const uint8_t *data,
                                                   void *user_data),
                             void *user_data);
int dav1d_data_wrap_user_data_internal(Dav1dData *buf,
                                       const uint8_t *user_data,
                                       void (*free_callback)(const uint8_t *user_data,
                                                             void *cookie),
                                       void *cookie);
void dav1d_data_unref_internal(Dav1dData *buf);
void dav1d_data_props_unref_internal(Dav1dDataProps *props);

#endif /* DAV1D_SRC_DATA_H */
