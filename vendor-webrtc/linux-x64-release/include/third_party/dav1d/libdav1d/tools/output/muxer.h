/*
 * Copyright © 2018, VideoLAN and dav1d authors
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

#ifndef DAV1D_OUTPUT_MUXER_H
#define DAV1D_OUTPUT_MUXER_H

#include "picture.h"

typedef struct MuxerPriv MuxerPriv;
typedef struct Muxer {
    int priv_data_size;
    const char *name;
    const char *extension;
    int (*write_header)(MuxerPriv *ctx, const char *filename,
                        const Dav1dPictureParameters *p, const unsigned fps[2]);
    int (*write_picture)(MuxerPriv *ctx, Dav1dPicture *p);
    void (*write_trailer)(MuxerPriv *ctx);
    /**
     * Verifies the muxed data (for example in the md5 muxer). Replaces write_trailer.
     *
     * @param  hash_string Muxer specific reference value.
     *
     * @return 0 on success.
     */
    int (*verify)(MuxerPriv *ctx, const char *hash_string);
} Muxer;

#endif /* DAV1D_OUTPUT_MUXER_H */
