/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_ISAC_VAD_H_
#define MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_ISAC_VAD_H_

#include <stddef.h>

#include "modules/audio_coding/codecs/isac/main/source/structs.h"

void WebRtcIsac_InitPitchFilter(PitchFiltstr* pitchfiltdata);
void WebRtcIsac_InitPitchAnalysis(PitchAnalysisStruct* state);
void WebRtcIsac_InitPreFilterbank(PreFiltBankstr* prefiltdata);

double WebRtcIsac_LevDurb(double* a, double* k, double* r, size_t order);

/* The number of all-pass filter factors in an upper or lower channel*/
#define NUMBEROFCHANNELAPSECTIONS 2

/* The upper channel all-pass filter factors */
extern const float WebRtcIsac_kUpperApFactorsFloat[2];

/* The lower channel all-pass filter factors */
extern const float WebRtcIsac_kLowerApFactorsFloat[2];

void WebRtcIsac_AllPassFilter2Float(float* InOut,
                                    const float* APSectionFactors,
                                    int lengthInOut,
                                    int NumberOfSections,
                                    float* FilterState);
void WebRtcIsac_SplitAndFilterFloat(float* in,
                                    float* LP,
                                    float* HP,
                                    double* LP_la,
                                    double* HP_la,
                                    PreFiltBankstr* prefiltdata);

#endif  // MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_ISAC_VAD_H_
