/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

/*
 * pitch_estimator.h
 *
 * Pitch functions
 *
 */

#ifndef MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_PITCH_ESTIMATOR_H_
#define MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_PITCH_ESTIMATOR_H_

#include <stddef.h>

#include "modules/audio_coding/codecs/isac/main/source/structs.h"

void WebRtcIsac_PitchAnalysis(
    const double* in, /* PITCH_FRAME_LEN samples */
    double* out,      /* PITCH_FRAME_LEN+QLOOKAHEAD samples */
    PitchAnalysisStruct* State,
    double* lags,
    double* gains);

#endif /* MODULES_AUDIO_CODING_CODECS_ISAC_MAIN_SOURCE_PITCH_ESTIMATOR_H_ */
