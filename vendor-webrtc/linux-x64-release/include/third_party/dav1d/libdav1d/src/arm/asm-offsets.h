/*
 * Copyright © 2021, VideoLAN and dav1d authors
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

#ifndef ARM_ASM_OFFSETS_H
#define ARM_ASM_OFFSETS_H

#include "config.h"

#define FGD_SEED                         0
#define FGD_AR_COEFF_LAG                 92
#define FGD_AR_COEFFS_Y                  96
#define FGD_AR_COEFFS_UV                 120
#define FGD_AR_COEFF_SHIFT               176
#define FGD_GRAIN_SCALE_SHIFT            184

#define FGD_SCALING_SHIFT                88
#define FGD_UV_MULT                      188
#define FGD_UV_LUMA_MULT                 196
#define FGD_UV_OFFSET                    204
#define FGD_CLIP_TO_RESTRICTED_RANGE     216

#if ARCH_AARCH64
#define RMVSF_IW8                        16
#define RMVSF_IH8                        20
#define RMVSF_MFMV_REF                   53
#define RMVSF_MFMV_REF2CUR               56
#define RMVSF_MFMV_REF2REF               68
#define RMVSF_N_MFMVS                    152
#define RMVSF_RP_REF                     168
#define RMVSF_RP_PROJ                    176
#define RMVSF_RP_STRIDE                  184
#define RMVSF_N_TILE_THREADS             200
#endif

#endif /* ARM_ASM_OFFSETS_H */
