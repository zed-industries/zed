/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_FILTER_FUNCTIONS_H_
#define MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_FILTER_FUNCTIONS_H_

#include <stddef.h>

#include "modules/audio_coding/codecs/isac/main/source/structs.h"

void WebRtcIsac_AutoCorr(double* r, const double* x, size_t N, size_t order);

void WebRtcIsac_WeightingFilter(const double* in,
                                double* weiout,
                                double* whiout,
                                WeightFiltstr* wfdata);

#endif  // MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_FILTER_FUNCTIONS_H_
