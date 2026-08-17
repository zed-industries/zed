/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_PITCH_FILTER_H_
#define MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_PITCH_FILTER_H_

#include "modules/audio_coding/codecs/isac/main/source/structs.h"

void WebRtcIsac_PitchfilterPre(double* indat,
                               double* outdat,
                               PitchFiltstr* pfp,
                               double* lags,
                               double* gains);

void WebRtcIsac_PitchfilterPost(double* indat,
                                double* outdat,
                                PitchFiltstr* pfp,
                                double* lags,
                                double* gains);

void WebRtcIsac_PitchfilterPre_la(double* indat,
                                  double* outdat,
                                  PitchFiltstr* pfp,
                                  double* lags,
                                  double* gains);

void WebRtcIsac_PitchfilterPre_gains(
    double* indat,
    double* outdat,
    double out_dG[][PITCH_FRAME_LEN + QLOOKAHEAD],
    PitchFiltstr* pfp,
    double* lags,
    double* gains);

#endif  // MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_PITCH_FILTER_H_
