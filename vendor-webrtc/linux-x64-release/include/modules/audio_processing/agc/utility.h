/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_UTILITY_H_
#define MODULES_AUDIO_PROCESSING_AGC_UTILITY_H_

namespace webrtc {

// TODO(turajs): Add description of function.
double Loudness2Db(double loudness);

double Linear2Loudness(double rms);

double Db2Loudness(double db);

double Dbfs2Loudness(double dbfs);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_UTILITY_H_
