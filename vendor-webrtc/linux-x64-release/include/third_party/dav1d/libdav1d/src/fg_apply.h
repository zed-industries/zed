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

#ifndef DAV1D_SRC_FG_APPLY_H
#define DAV1D_SRC_FG_APPLY_H

#include "dav1d/picture.h"

#include "common/bitdepth.h"

#include "src/filmgrain.h"

#ifdef BITDEPTH
# define array_decl(type, name, sz) type name sz
#else
# define array_decl(type, name, sz) void *name
#endif

bitfn_decls(void dav1d_apply_grain,
            const Dav1dFilmGrainDSPContext *const dsp,
            Dav1dPicture *const out, const Dav1dPicture *const in);
bitfn_decls(void dav1d_prep_grain,
            const Dav1dFilmGrainDSPContext *const dsp,
            Dav1dPicture *const out, const Dav1dPicture *const in,
            array_decl(uint8_t, scaling, [3][SCALING_SIZE]),
            array_decl(entry, grain_lut, [3][GRAIN_HEIGHT+1][GRAIN_WIDTH]));
bitfn_decls(void dav1d_apply_grain_row,
            const Dav1dFilmGrainDSPContext *const dsp,
            Dav1dPicture *const out, const Dav1dPicture *const in,
            array_decl(const uint8_t, scaling, [3][SCALING_SIZE]),
            array_decl(const entry, grain_lut, [3][GRAIN_HEIGHT+1][GRAIN_WIDTH]),
            const int row);

#endif /* DAV1D_SRC_FG_APPLY_H */
